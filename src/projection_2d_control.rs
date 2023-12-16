use bevy::{input::mouse::MouseWheel, prelude::*, window::PrimaryWindow};

use crate::states::AppState;

pub struct Projection2dControlPlugin;

impl Plugin for Projection2dControlPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<StateChangePositionBegin>()
        .init_resource::<CameraBeginTransform>()
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(AppState::MovingCamera),
            |q_windows: Query<&Window, With<PrimaryWindow>>,mut state_change_position_begin: ResMut<StateChangePositionBegin>| {
                let window = q_windows.single();
                if let Some(cursor_position) = window.cursor_position() {
                    *state_change_position_begin = StateChangePositionBegin(Some(cursor_position));
                }
            },
        )
        .add_systems(OnEnter(AppState::MovingCamera), set_camera_begin_transform)
        .add_systems(
            Update,
            update_camera.run_if(in_state(AppState::MovingCamera)),
        )
        .add_systems(
            Update,
            update_proj.run_if(in_state(AppState::Hovering)),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Default)]
struct CameraBeginTransform(Vec2);

#[derive(Default, Resource)]
struct StateChangePositionBegin(Option<Vec2>);

fn set_camera_begin_transform(
    camera_query: Query<&Transform, With<Camera>>,
    mut camera_transform_org: ResMut<CameraBeginTransform>,
) {
    *camera_transform_org = CameraBeginTransform(camera_query.single().translation.xy());
}

fn update_camera(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&OrthographicProjection, &mut Transform), With<Camera>>,
    camera_transform_org: Res<CameraBeginTransform>,
    state_change_position_begin: Res<StateChangePositionBegin>,
) {
    let mut window_point = Vec2::default();
    let (proj, mut transform) = camera_query.single_mut();
    let window = q_windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        window_point = cursor_position;
    }

    if let Some(cursor_org) = state_change_position_begin.0 {
        let new_xy =
            camera_transform_org.0 + (window_point - cursor_org) * Vec2::new(-1., 1.) * proj.scale;
        transform.translation.x = new_xy.x;
        transform.translation.y = new_xy.y;
    }
}

fn update_proj(
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut proj = camera_query.single_mut();
    // 缩放时不能有ctrl 按键按下
    if !keyboard_input.pressed(KeyCode::ControlLeft) {
        for event in mouse_wheel_events.read() {
            proj.scale += 0.01 * event.y
        }
    }
}
