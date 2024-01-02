use bevy::ecs::query::ReadOnlyWorldQuery;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn show_window_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_windows.single_mut();
    window.cursor.visible = true;
}

pub fn hide_window_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_windows.single_mut();
    window.cursor.visible = false;
}

pub fn clear_with<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    query: Query<Entity, F>,
) {
    for id in query.iter() {
        commands.entity(id).despawn();
    }
}
