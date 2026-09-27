use bb_installer_bevy::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::image::ToExtents;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};

pub struct SignPlugin;

/// Stroke width, in canvas pixels.
const THICKNESS: f32 = 5.0;

/// Size of the sign quad in world units.
const SIGN_SIZE: Vec2 = Vec2::new(400.0, 150.0);

/// Resolution of the texture we paint into. Higher = finer strokes, more RAM.
const CANVAS: UVec2 = UVec2::new(400, 150);

const INK: Color = Color::srgb(0.1, 0.1, 0.12);

impl Plugin for SignPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Sign), spawn_sign);
    }
}

/// Holds the image we draw into plus the previous point of the current stroke,
/// so we can join samples into a continuous line instead of dotting.
#[derive(Component)]
struct SignCanvas {
    image: Handle<Image>,
    last: Option<Vec2>,
}

fn spawn_sign(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // A blank white RGBA canvas. MAIN_WORLD keeps the pixel data on the CPU so
    // we can keep mutating it after upload. (Use `&[0, 0, 0, 0]` instead for a
    // transparent canvas that only shows the ink.)
    let canvas = images.add(Image::new_fill(
        CANVAS.to_extents(),
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));

    commands
        .spawn((
            DespawnOnExit(AppState::Sign),
            Mesh2d(meshes.add(Rectangle::from_size(SIGN_SIZE))),
            MeshMaterial2d(materials.add(ColorMaterial {
                // The texture is multiplied by `color`, so keep it white.
                color: Color::WHITE,
                texture: Some(canvas.clone()),
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, 0.0),
            SignCanvas {
                image: canvas,
                last: None,
            },
        ))
        .observe(paint)
        .observe(end_stroke);
}

/// Runs for every pointer move over the sign; paints while the left button is held.
fn paint(
    drag: On<Pointer<Move>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut signs: Query<(&GlobalTransform, &mut SignCanvas)>,
    mut images: ResMut<Assets<Image>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    let Ok((transform, mut sign)) = signs.get_mut(drag.entity) else {
        return;
    };
    // World-space hit point from MeshPickingPlugin -> the mesh's local space.
    let Some(world) = drag.hit.position else {
        return;
    };
    let local = transform
        .affine()
        .inverse()
        .transform_point3(world)
        .truncate();

    // Local space is centred on the quad and Y is up; texels start top-left.
    let uv = local / SIGN_SIZE + Vec2::splat(0.5);
    let point = Vec2::new(uv.x, 1.0 - uv.y) * CANVAS.as_vec2();

    let Some(mut image) = images.get_mut(&sign.image) else {
        return;
    };
    match sign.last {
        Some(prev) => stroke(&mut image, prev, point),
        None => dot(&mut image, point),
    }
    sign.last = Some(point);
}

/// Releasing (or leaving the sign) breaks the line, so the next stroke starts fresh.
fn end_stroke(out: On<Pointer<Release>>, mut signs: Query<&mut SignCanvas>) {
    if let Ok(mut sign) = signs.get_mut(out.entity) {
        sign.last = None;
    }
}

/// Stamps a filled circle of `THICKNESS` diameter centred on `at`.
fn dot(image: &mut Image, at: Vec2) {
    let r = THICKNESS / 2.0;
    let min = (at - r).floor().max(Vec2::ZERO).as_uvec2();
    let max = (at + r).ceil().min(CANVAS.as_vec2()).as_uvec2();

    for y in min.y..max.y {
        for x in min.x..max.x {
            // +0.5 to measure from the texel centre.
            if (Vec2::new(x as f32, y as f32) + 0.5).distance(at) <= r {
                let _ = image.set_color_at(x, y, INK);
            }
        }
    }
}

/// Walks from `from` to `to` in half-pixel steps, stamping as it goes. Cheap and
/// good enough for a mouse; a real brush would use a distance-field instead.
fn stroke(image: &mut Image, from: Vec2, to: Vec2) {
    let steps = (from.distance(to) * 2.0).ceil().max(1.0);
    for i in 0..=steps as u32 {
        dot(image, from.lerp(to, i as f32 / steps));
    }
}
