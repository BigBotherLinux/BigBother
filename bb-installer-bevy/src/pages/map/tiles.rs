//! OpenStreetMap tiles draped on the globe.
//!
//! Each tile is its own curved patch of the unit sphere. Only the tiles around
//! the crosshair at one zoom level are wanted at a time, and a tile that has
//! not arrived yet is covered by its nearest loaded ancestor. The four tiles of
//! zoom 2 are always shown underneath, so the globe is never blank.
//!
//! The tile server's usage policy asks for an identifying User-Agent, at most
//! two connections, and visible attribution. See
//! <https://operations.osmfoundation.org/policies/tiles/>.

use std::f64::consts::PI;

use super::{FOV, View};
use crate::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task, futures::check_ready};
use bevy::window::PrimaryWindow;

const URL: &str = "https://tile.openstreetmap.org";
pub const MAX_ZOOM: u8 = 16;
/// Always loaded and always shown, under everything else.
const BASE_ZOOM: u8 = 2;
const TILE_PX: f32 = 256.0;
const MAX_IN_FLIGHT: usize = 2;
/// Tiles kept in memory. One is 256 KiB on the GPU.
const MAX_CACHED: usize = 600;
const RETRY_SECS: f32 = 10.0;
/// A zoom level has to be wanted this long before its tiles are fetched, so a
/// dive does not fetch every level on the way down.
const SETTLE_SECS: f32 = 0.3;
/// At most this many tiles out from the centre one, each way.
const MAX_REACH: i64 = 7;
/// How far each zoom level is lifted above the one below it, relative to the
/// camera's altitude, so the sharper tile always draws on top.
const LIFT: f32 = 5e-4;

/// Lat/lon to the unit sphere. Longitude 0 faces +Z, east is +X, north is +Y.
pub fn to_sphere(lat: f64, lon: f64) -> Vec3 {
    Vec3::new(
        (lat.cos() * lon.sin()) as f32,
        lat.sin() as f32,
        (lat.cos() * lon.cos()) as f32,
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct TileId {
    z: u8,
    x: u32,
    y: u32,
}

impl TileId {
    fn parent(self) -> Option<Self> {
        (self.z > 0).then(|| TileId {
            z: self.z - 1,
            x: self.x / 2,
            y: self.y / 2,
        })
    }

    fn url(self) -> String {
        format!("{URL}/{}/{}/{}.png", self.z, self.x, self.y)
    }
}

/// Web Mercator tile coordinates, fractional, at zoom `z`.
fn tile_coords(lat: f64, lon: f64, z: u8) -> (f64, f64) {
    let n = f64::from(1u32 << z);
    let x = (lon + PI) / (2.0 * PI) * n;
    let y = (1.0 - lat.tan().asinh() / PI) / 2.0 * n;
    (x, y)
}

/// Inverse of [`tile_coords`].
fn tile_lat_lon(x: f64, y: f64, z: u8) -> (f64, f64) {
    let n = f64::from(1u32 << z);
    let lon = x / n * 2.0 * PI - PI;
    let lat = (PI * (1.0 - 2.0 * y / n)).sinh().atan();
    (lat, lon)
}

struct Fetched {
    image: Image,
    mesh: Mesh,
    centre: Vec3,
}

struct Loaded {
    entity: Entity,
    centre: Vec3,
    last_used: u32,
}

#[derive(Resource)]
pub struct Tiles {
    agent: ureq::Agent,
    loaded: HashMap<TileId, Loaded>,
    in_flight: Vec<(TileId, Task<Option<Fetched>>)>,
    /// When each failed tile may be tried again, in `clock` seconds.
    failed: HashMap<TileId, f32>,
    shown: HashSet<TileId>,
    frame: u32,
    clock: f32,
    /// The zoom level being fetched and shown.
    zoom: u8,
    /// A finer level that has been wanted since `clock` time.
    pending: Option<(u8, f32)>,
}

impl Tiles {
    pub fn new(agent: ureq::Agent) -> Self {
        Self {
            agent,
            loaded: HashMap::default(),
            in_flight: Vec::new(),
            failed: HashMap::default(),
            shown: HashSet::default(),
            frame: 0,
            clock: 0.0,
            zoom: BASE_ZOOM,
            pending: None,
        }
    }

    /// Coarser levels take over right away, finer ones only once they have
    /// been wanted for `SETTLE_SECS`.
    fn settle_zoom(&mut self, wanted: u8) {
        if wanted <= self.zoom {
            self.zoom = wanted;
            self.pending = None;
            return;
        }
        match self.pending {
            Some((z, since)) if z == wanted => {
                if self.clock - since >= SETTLE_SECS {
                    self.zoom = wanted;
                    self.pending = None;
                }
            }
            _ => self.pending = Some((wanted, self.clock)),
        }
    }

    fn should_fetch(&self, id: TileId) -> bool {
        !self.loaded.contains_key(&id)
            && !self.in_flight.iter().any(|(t, _)| *t == id)
            && self.failed.get(&id).is_none_or(|&at| self.clock >= at)
    }

    fn fetch(&mut self, id: TileId) {
        let agent = self.agent.clone();
        let task = IoTaskPool::get().spawn(async move { fetch_tile(&agent, id) });
        self.in_flight.push((id, task));
    }
}

/// The zoom level whose tiles come out about `TILE_PX` on screen.
fn zoom_for(view: &View, screen_height: f32) -> u8 {
    let visible = 2.0 * view.altitude * (FOV / 2.0).tan();
    let world = std::f32::consts::TAU * (view.lat as f32).cos();
    let tiles_across = world * screen_height / (visible * TILE_PX);
    (tiles_across.log2().round() as i32).clamp(BASE_ZOOM as i32, MAX_ZOOM as i32) as u8
}

/// How big a tile of zoom `z` is on screen, in physical pixels.
fn tile_px(view: &View, z: u8, screen_height: f32) -> f32 {
    let visible = 2.0 * view.altitude * (FOV / 2.0).tan();
    let world = std::f32::consts::TAU * (view.lat as f32).cos();
    world / (1u32 << z) as f32 / visible * screen_height
}

/// Tiles of zoom `z` that cover the screen, nearest the crosshair first.
fn wanted_tiles(view: &View, z: u8, screen: Vec2) -> Vec<TileId> {
    let (fx, fy) = tile_coords(view.lat, view.lon, z);
    let size = tile_px(view, z, screen.y);
    let reach_x = ((screen.x / 2.0 / size).ceil() as i64 + 1).min(MAX_REACH);
    let reach_y = ((screen.y / 2.0 / size).ceil() as i64 + 1).min(MAX_REACH);
    let n = 1i64 << z;
    let (cx, cy) = (fx.floor() as i64, fy.floor() as i64);

    let mut tiles: Vec<(i64, TileId)> = Vec::new();
    let mut seen = HashSet::new();
    for dy in -reach_y..=reach_y {
        let y = cy + dy;
        if !(0..n).contains(&y) {
            continue;
        }
        for dx in -reach_x..=reach_x {
            let id = TileId {
                z,
                x: (cx + dx).rem_euclid(n) as u32,
                y: y as u32,
            };
            if seen.insert(id) {
                tiles.push((dx * dx + dy * dy, id));
            }
        }
    }
    tiles.sort_by_key(|(distance, _)| *distance);
    tiles.into_iter().map(|(_, id)| id).collect()
}

fn base_tiles() -> impl Iterator<Item = TileId> {
    let n = 1u32 << BASE_ZOOM;
    (0..n).flat_map(move |y| (0..n).map(move |x| TileId { z: BASE_ZOOM, x, y }))
}

/// Blocking; runs on the IO pool.
fn fetch_tile(agent: &ureq::Agent, id: TileId) -> Option<Fetched> {
    let bytes = match agent.get(id.url()).call() {
        Ok(mut response) => response.body_mut().read_to_vec(),
        Err(e) => Err(e),
    };
    let bytes = bytes.inspect_err(|e| warn!("Tile {id:?}: {e}")).ok()?;
    let image = Image::from_buffer(
        &bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::linear(),
        RenderAssetUsages::RENDER_WORLD,
    )
    .inspect_err(|e| warn!("Tile {id:?}: {e}"))
    .ok()?;
    let (mesh, centre) = tile_mesh(id);
    Some(Fetched {
        image,
        mesh,
        centre,
    })
}

/// A patch of the unit sphere covering one tile, relative to its centre so the
/// vertices keep their precision when zoomed far in.
///
/// Every level uses the same vertex spacing down to zoom 4, so the coarse and
/// fine patches lie on the same surface and only the lift separates them.
/// Finer tiles have more vertices than that, and so sit on or above the chords
/// of the coarser ones.
fn tile_mesh(id: TileId) -> (Mesh, Vec3) {
    let segments: u32 = (64 >> id.z).max(4);
    let (clat, clon) = tile_lat_lon(id.x as f64 + 0.5, id.y as f64 + 0.5, id.z);
    let centre = to_sphere(clat, clon);

    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    for j in 0..=segments {
        let v = j as f32 / segments as f32;
        for i in 0..=segments {
            let u = i as f32 / segments as f32;
            let (lat, lon) = tile_lat_lon(id.x as f64 + u as f64, id.y as f64 + v as f64, id.z);
            let p = to_sphere(lat, lon);
            positions.push((p - centre).to_array());
            normals.push(p.to_array());
            uvs.push([u, v]);
        }
    }

    // Seen from outside, u runs east (right) and v south (down).
    let row = segments + 1;
    let mut indices = Vec::new();
    for j in 0..segments {
        for i in 0..segments {
            let a = j * row + i;
            let (b, c, d) = (a + 1, a + row, a + row + 1);
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    let mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices));
    (mesh, centre)
}

#[allow(clippy::too_many_arguments)]
pub fn stream_tiles(
    mut commands: Commands,
    time: Res<Time>,
    view: Res<View>,
    window: Single<&Window, With<PrimaryWindow>>,
    tiles: Option<ResMut<Tiles>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut placed: Query<(&mut Transform, &mut Visibility)>,
) {
    let Some(mut tiles) = tiles else {
        return;
    };
    tiles.clock += time.delta_secs();
    tiles.frame += 1;
    let frame = tiles.frame;
    let screen = window.physical_size().as_vec2();

    // Arrivals.
    let mut arrived = Vec::new();
    tiles
        .in_flight
        .retain_mut(|(id, task)| match check_ready(task) {
            Some(result) => {
                arrived.push((*id, result));
                false
            }
            None => true,
        });
    for (id, result) in arrived {
        let Some(fetched) = result else {
            let retry = tiles.clock + RETRY_SECS;
            tiles.failed.insert(id, retry);
            continue;
        };
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(fetched.image)),
            unlit: true,
            cull_mode: None,
            ..default()
        });
        let entity = commands
            .spawn((
                Name::new("Map Tile"),
                DespawnOnExit(AppState::Map),
                Mesh3d(meshes.add(fetched.mesh)),
                MeshMaterial3d(material),
                Transform::from_translation(fetched.centre),
                Visibility::Hidden,
            ))
            .id();
        tiles.loaded.insert(
            id,
            Loaded {
                entity,
                centre: fetched.centre,
                last_used: frame,
            },
        );
    }

    tiles.settle_zoom(zoom_for(&view, screen.y));
    let wanted = wanted_tiles(&view, tiles.zoom, screen);

    // Requests: the base first, then from the crosshair outwards.
    for id in base_tiles().chain(wanted.iter().copied()) {
        if tiles.in_flight.len() >= MAX_IN_FLIGHT {
            break;
        }
        if tiles.should_fetch(id) {
            tiles.fetch(id);
        }
    }

    // What to draw: each wanted tile, or the closest ancestor we have.
    let mut shown: HashSet<TileId> = base_tiles()
        .filter(|id| tiles.loaded.contains_key(id))
        .collect();
    for id in wanted {
        let mut candidate = Some(id);
        while let Some(c) = candidate {
            if tiles.loaded.contains_key(&c) {
                shown.insert(c);
                break;
            }
            candidate = c.parent();
        }
    }

    for id in tiles.shown.difference(&shown) {
        if let Some(Ok((_, mut visibility))) =
            tiles.loaded.get(id).map(|l| placed.get_mut(l.entity))
        {
            *visibility = Visibility::Hidden;
        }
    }
    for id in &shown {
        let Some(loaded) = tiles.loaded.get_mut(id) else {
            continue;
        };
        loaded.last_used = frame;
        if let Ok((mut transform, mut visibility)) = placed.get_mut(loaded.entity) {
            let scale = 1.0 + LIFT * view.altitude * id.z as f32;
            transform.translation = loaded.centre * scale;
            transform.scale = Vec3::splat(scale);
            *visibility = Visibility::Visible;
        }
    }
    tiles.shown = shown;

    // Forget the least recently shown tiles.
    if tiles.loaded.len() > MAX_CACHED {
        let mut by_age: Vec<(u32, TileId)> = tiles
            .loaded
            .iter()
            .filter(|(id, _)| id.z != BASE_ZOOM && !tiles.shown.contains(*id))
            .map(|(id, l)| (l.last_used, *id))
            .collect();
        by_age.sort_unstable_by_key(|(age, _)| *age);
        let excess = tiles.loaded.len() - MAX_CACHED;
        for (_, id) in by_age.into_iter().take(excess) {
            if let Some(loaded) = tiles.loaded.remove(&id) {
                commands.entity(loaded.entity).despawn();
            }
        }
    }
}
