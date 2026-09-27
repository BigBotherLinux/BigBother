//! World map: Natural Earth country outlines, drawn as line meshes.
//!
//! The SVG is baked into the binary, so there is no asset path to get wrong
//! under Nix, and no loading state to wait on.

use bb_installer_bevy::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const MAP_SVG: &str = include_str!("world_map.svg");

/// The exporter's viewBox. Coordinates are already projected, so all we do is
/// flip y (SVG points down, Bevy up), recentre, and scale.
const VIEW_W: f32 = 800.0;
const VIEW_H: f32 = 387.0;
const SCALE: f32 = 2.0;

const LAND: Color = Color::srgb(0.30, 0.30, 0.38);
const HOVER: Color = Color::srgb(0.75, 0.25, 0.25);
const PICKED: Color = Color::srgb(0.95, 0.15, 0.15);

/// ADM0_A3 of the country the user picked.
#[derive(Resource, Default)]
pub struct Selected(pub Option<String>);

#[derive(Component)]
pub struct Country {
    pub code: String,
    /// Rings in world space, kept for hit testing.
    rings: Vec<Vec<Vec2>>,
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>()
            .add_systems(OnEnter(AppState::Map), spawn_map)
            .add_systems(Update, highlight.run_if(in_state(AppState::Map)));
    }
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Every element is `<path d="..." [fill-rule="..."] id="XXX"/>`, regular
    // enough that an XML parser would be the more fragile choice.
    for element in MAP_SVG.split("<path ").skip(1) {
        let Some((d, _)) = element.strip_prefix("d=\"").and_then(|r| r.split_once('"')) else {
            continue;
        };
        let Some((_, rest)) = element.rsplit_once("id=\"") else {
            continue;
        };
        let Some((code, _)) = rest.split_once('"') else {
            continue;
        };

        let mut rings: Vec<Vec<Vec2>> = Vec::new();
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        // Only absolute `M`, bare coordinate pairs (implicit lineto) and `Z`
        // appear, so splitting on 'M' gives exactly the rings.
        for ring in d.split('M').skip(1) {
            let nums: Vec<f32> = ring
                .replace('Z', " ")
                .split_whitespace()
                .filter_map(|t| t.parse().ok())
                .collect();
            if nums.len() < 6 {
                continue; // fewer than three points is not an area
            }

            let base = positions.len() as u32;
            let (pairs, _odd) = nums.as_chunks::<2>();
            let points: Vec<Vec2> = pairs
                .iter()
                .map(|&[x, y]| Vec2::new((x - VIEW_W / 2.0) * SCALE, (VIEW_H / 2.0 - y) * SCALE))
                .collect();

            for (i, p) in points.iter().enumerate() {
                positions.push([p.x, p.y, 0.0]);
                indices.push(base + i as u32);
                indices.push(base + (i as u32 + 1) % points.len() as u32); // wraps closed
            }
            rings.push(points);
        }

        if rings.is_empty() {
            continue;
        }

        commands.spawn((
            DespawnOnExit(AppState::Map),
            // RENDER_WORLD is enough: nothing raycasts this, so the CPU copy
            // can be dropped after upload.
            Mesh2d(
                meshes.add(
                    Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::RENDER_WORLD)
                        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
                        .with_inserted_indices(Indices::U32(indices)),
                ),
            ),
            MeshMaterial2d(materials.add(LAND)),
            Country {
                code: code.to_string(),
                rings,
            },
        ));
    }
}

fn highlight(
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    countries: Query<(&Country, &MeshMaterial2d<ColorMaterial>)>,
    mut selected: ResMut<Selected>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cursor = windows.single().ok().and_then(|w| w.cursor_position());
    let point = match (cursor, cameras.single().ok()) {
        (Some(c), Some((camera, transform))) => camera.viewport_to_world_2d(transform, c).ok(),
        _ => None,
    };

    for (country, material) in &countries {
        // Even-odd ray crossing over every ring at once: the same rule the
        // SVG's fill-rule asks for, which makes holes free. A point inside
        // both South Africa and Lesotho crosses two rings and comes out even.
        // Winding order is meaningless in this file, so parity is all we have.
        let mut hovered = false;
        if let Some(p) = point {
            for ring in &country.rings {
                for i in 0..ring.len() {
                    let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
                    if (a.y > p.y) != (b.y > p.y)
                        && p.x < a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x)
                    {
                        hovered = !hovered;
                    }
                }
            }
        }

        if hovered && mouse.just_pressed(MouseButton::Left) {
            selected.0 = Some(country.code.clone());
            info!("selected {}", country.code);
        }

        if let Some(mut material) = materials.get_mut(&material.0) {
            material.color = if hovered {
                HOVER
            } else if selected.0.as_deref() == Some(country.code.as_str()) {
                PICKED
            } else {
                LAND
            };
        }
    }
}
