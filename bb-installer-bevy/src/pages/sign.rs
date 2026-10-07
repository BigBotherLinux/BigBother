use crate::AppState;
use bevy::asset::RenderAssetUsages;
use bevy::feathers::controls::FeathersButton;
use bevy::feathers::theme::ThemedText;
use bevy::image::ToExtents;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat};
use bevy::ui::InteractionDisabled;
use bevy::ui_widgets::Activate;

pub struct SignPlugin;

const THICKNESS: f32 = 5.0;

const SIGN_SIZE: Vec2 = Vec2::new(400.0, 150.0);

const CANVAS: UVec2 = UVec2::new(400, 150);

const INK: Color = Color::srgb(0.1, 0.1, 0.12);

const MIN_SIGN_COUNT: u32 = 3000;

impl Plugin for SignPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MeshPickingPlugin>() {
            app.add_plugins(MeshPickingPlugin);
        }
        app.add_systems(OnEnter(AppState::Sign), spawn_sign)
            .add_systems(Update, unlock_continue.run_if(in_state(AppState::Sign)));
    }
}

#[derive(Component, FromTemplate)]
struct SignCanvas {
    image: Handle<Image>,
    last: Option<Vec2>,
    painted: u32,
}

#[derive(Component, FromTemplate)]
struct SignContinue;

fn spawn_sign(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let canvas = images.add(Image::new_fill(
        CANVAS.to_extents(),
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ));
    commands.spawn_scene(sign_label());
    commands.spawn_scene(sign_canvas(canvas));
}

fn unlock_continue(
    mut commands: Commands,
    signs: Query<&SignCanvas, Changed<SignCanvas>>,
    buttons: Query<Entity, (With<SignContinue>, With<InteractionDisabled>)>,
) {
    if signs.iter().any(|sign| sign.painted >= MIN_SIGN_COUNT) {
        for button in buttons.iter() {
            commands.entity(button).remove::<InteractionDisabled>();
        }
    }
}

/// UI overlay with the label. UI picks sort above meshes and nodes block
/// lower hits by default, so everything here must be `Pickable::IGNORE` or
/// the canvas never sees the pointer.
fn sign_label() -> impl Scene {
    bsn! {
        DespawnOnExit::<AppState>(AppState::Sign)
        Pickable::IGNORE
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        Children[
            (Pickable::IGNORE
            Node {
                width: {Val::Px(SIGN_SIZE.x)},
                height: {Val::Px(SIGN_SIZE.y)},
                justify_content: JustifyContent::Center,
            }
            Children[
                (Pickable::IGNORE
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Percent(100.0),
                    margin: UiRect::bottom(Val::Px(8.0)),
                }
                Text::new("Sign here")
                ThemedText),
                // Sits below the canvas, so it can stay pickable.
                (
                    @FeathersButton { @caption: bsn! { Text("Continue") ThemedText} }
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(100.0),
                        margin: UiRect::top(Val::Px(16.0)),
                        width: percent(30),
                    }
                    on(|_activate: On<Activate>, mut state: ResMut<NextState<AppState>>| {
                        state.set(AppState::Sign);
                    })
                    InteractionDisabled
                    SignContinue
                )
            ])
        ]
    }
}

/// Kept as its own root: a `Mesh2d` isn't laid out by UI, and parenting it to
/// a `Node` would put it behind that node for picking.
fn sign_canvas(canvas: Handle<Image>) -> impl Scene {
    bsn! {
        DespawnOnExit::<AppState>(AppState::Sign)
        Mesh2d(asset_value(Rectangle::from_size(SIGN_SIZE)))
        MeshMaterial2d::<ColorMaterial>(asset_value(ColorMaterial {
            // The texture is multiplied by `color`, so keep it white.
            color: Color::WHITE,
            texture: Some(canvas.clone()),
            ..default()
        }))
        SignCanvas { image: {canvas.clone()} }
        on(paint)
        on(end_stroke::<Release>)
        on(end_stroke::<Out>)
    }
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
    sign.painted += match sign.last {
        Some(prev) => stroke(&mut image, prev, point),
        None => dot(&mut image, point),
    };
    sign.last = Some(point);
}

fn end_stroke<E: Clone + Reflect + std::fmt::Debug>(
    event: On<Pointer<E>>,
    mut signs: Query<&mut SignCanvas>,
) {
    if let Ok(mut sign) = signs.get_mut(event.entity) {
        sign.last = None;
    }
}

fn dot(image: &mut Image, at: Vec2) -> u32 {
    let mut inked = 0;
    let r = THICKNESS / 2.0;
    let min = (at - r).floor().max(Vec2::ZERO).as_uvec2();
    let max = (at + r).ceil().min(CANVAS.as_vec2()).as_uvec2();

    for y in min.y..max.y {
        for x in min.x..max.x {
            if (Vec2::new(x as f32, y as f32) + 0.5).distance(at) > r {
                continue;
            }
            let blank = image
                .pixel_bytes(UVec3::new(x, y, 0))
                .is_ok_and(|px| px == [255; 4]);

            if blank && image.set_color_at(x, y, INK).is_ok() {
                inked += 1;
            }
        }
    }
    inked
}

fn stroke(image: &mut Image, from: Vec2, to: Vec2) -> u32 {
    let steps = (from.distance(to) * 2.0).ceil().max(1.0);
    let mut inked = 0;
    for i in 0..=steps as u32 {
        inked += dot(image, from.lerp(to, i as f32 / steps));
    }
    inked
}
