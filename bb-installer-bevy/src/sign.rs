use crate::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::image::ToExtents;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};

pub struct SignPlugin;

const THICKNESS: f32 = 5.0;

const SIGN_SIZE: Vec2 = Vec2::new(400.0, 150.0);

const CANVAS: UVec2 = UVec2::new(400, 150);

const INK: Color = Color::srgb(0.1, 0.1, 0.12);

impl Plugin for SignPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MeshPickingPlugin>() {
            app.add_plugins(MeshPickingPlugin);
        }
        app.add_systems(OnEnter(AppState::Sign), spawn_sign);
    }
}

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

fn end_stroke(out: On<Pointer<Release>>, mut signs: Query<&mut SignCanvas>) {
    if let Ok(mut sign) = signs.get_mut(out.entity) {
        sign.last = None;
    }
}

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

fn stroke(image: &mut Image, from: Vec2, to: Vec2) {
    let steps = (from.distance(to) * 2.0).ceil().max(1.0);
    for i in 0..=steps as u32 {
        dot(image, from.lerp(to, i as f32 / steps));
    }
}
