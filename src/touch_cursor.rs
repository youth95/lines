use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    window::PrimaryWindow,
};

use crate::projection_2d_control::MainCamera;

pub struct TouchCursorPlugin;

impl Plugin for TouchCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<TouchCursorUiMaterial>::default())
            .init_resource::<TouchCursor>()
            .register_type::<TouchCursor>()
            .init_resource::<WorldTouchCursor>()
            .register_type::<WorldTouchCursor>()
            .add_systems(Startup, setup_touch_cursor)
            .add_systems(
                Update,
                (update_touch_cursor, update_world_torch_cursor),
            );
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TouchCursor {
    pub color: Color,
    pub size: f32,
}

#[derive(Resource, Default, Reflect, Debug)]
#[reflect(Resource)]
pub struct WorldTouchCursor(pub Vec2);

impl Default for TouchCursor {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            size: 16.0,
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct TouchCursorUiMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for TouchCursorUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_shader.wgsl".into()
    }
}

fn update_touch_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_proj: Query<&OrthographicProjection, With<MainCamera>>,
    mut touch_cursor: ResMut<TouchCursor>,
    mut ui_materials: ResMut<Assets<TouchCursorUiMaterial>>,
    mut q_cursor: Query<(
        &mut Style,
        &mut Transform,
        &mut Handle<TouchCursorUiMaterial>,
    )>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let window = q_windows.single_mut();
    if let Some(cursor_position) = window.cursor_position() {
        let (mut style, mut transform, mut material) = q_cursor.single_mut();
        style.left = Val::Px(cursor_position.x - touch_cursor.size / 2.0);
        style.top = Val::Px(cursor_position.y - touch_cursor.size / 2.0);
        transform.scale.x = 1. / q_proj.single().scale;
        transform.scale.y = 1. / q_proj.single().scale;
        style.width = Val::Px(touch_cursor.size);
        style.height = Val::Px(touch_cursor.size);
        *material = ui_materials.add(TouchCursorUiMaterial {
            color: touch_cursor.color.into(),
        })
    }

    if keyboard_input.pressed(KeyCode::BracketLeft) {
        touch_cursor.size -= 0.5;
    }

    if keyboard_input.pressed(KeyCode::BracketRight) {
        touch_cursor.size += 0.5;
    }
}

fn setup_touch_cursor(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<TouchCursorUiMaterial>>,
    touch_cursor: Res<TouchCursor>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(MaterialNodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Px(touch_cursor.size),
                    height: Val::Px(touch_cursor.size),
                    ..default()
                },
                material: ui_materials.add(TouchCursorUiMaterial {
                    color: Color::WHITE.into(),
                }),
                ..default()
            });
        });
}

fn update_world_torch_cursor(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut world_touch_cursor: ResMut<WorldTouchCursor>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (camera, camera_transform) = camera_query.single();
    for event in cursor_moved_events.read() {
        if let Some(point) =
            camera.viewport_to_world_2d(camera_transform, event.position)
        {
            if world_touch_cursor.0 != point {
                *world_touch_cursor = WorldTouchCursor(point);
            }
        }
    }
}
