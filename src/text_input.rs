use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    common::clear_with, cursor::WorldTouchCursor,
    double_click::on_double_click, states::ToolButton,
};

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (enter_text_input, spawn_text_cursor)
                    .run_if(in_state(ToolButton::Cursor))
                    .run_if(on_double_click),
                (exit_text_input, clear_with::<With<TextCursor>>)
                    .before(enter_text_input)
                    .run_if(in_state(ToolButton::Cursor))
                    .run_if(is_ime_enabled)
                    .run_if(input_just_pressed(MouseButton::Left)),
            ),
        );
    }
}

fn is_ime_enabled(windows: Query<&Window>) -> bool {
    windows.single().ime_enabled
}

fn enter_text_input(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.ime_position = window.cursor_position().unwrap();
    window.ime_enabled = true;
}

fn exit_text_input(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.ime_enabled = false;
}
#[derive(Component)]
struct TextCursor;

fn spawn_text_cursor(
    mut commands: Commands,
    world_cursor: Res<WorldTouchCursor>,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(1.0, 16.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                world_cursor.0.x,
                world_cursor.0.y,
                2.,
            )),
            ..default()
        })
        .insert(TextCursor);
}
