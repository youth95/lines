use bevy::{
    ecs::query::ReadOnlyWorldQuery, input::common_conditions::input_pressed,
    prelude::*, render::view::NoFrustumCulling, sprite::Material2dPlugin,
};
use bevy_prototype_lyon::prelude::*;

use crate::{
    chalk::ChalkMaterial,
    common::show_window_cursor,
    states::{AppState, RunMode},
    touch_cursor::{TouchCursor, TouchCursorPlugin, WorldTouchCursor},
    ui::{FocusedTool, ToolButton},
};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        let clear_condition = input_pressed(KeyCode::C);
        app.add_plugins((
            Material2dPlugin::<ChalkMaterial>::default(),
            TouchCursorPlugin,
        ))
        .add_systems(
            OnEnter(AppState::Hovering),
            remove_focused_line
                .run_if(resource_equals(FocusedTool(ToolButton::Pen))),
        )
        .add_systems(
            OnEnter(AppState::Drawing),
            spawn_focused_line
                .run_if(resource_equals(FocusedTool(ToolButton::Pen))),
        )
        .add_systems(Update, clear_with::<With<Path>>.run_if(clear_condition))
        .add_systems(
            Update,
            remove_line
                .run_if(resource_changed::<WorldTouchCursor>())
                .run_if(resource_equals(FocusedTool(ToolButton::Eraser)))
                .run_if(in_state(AppState::Drawing)),
        )
        .add_systems(
            Update,
            remove_last_with::<With<Path>>.run_if(undo_condition),
        )
        .add_systems(
            Update,
            (
                crate::layer::update_z_coordinate_based_on_layer,
                update_line,
                drawing,
            )
                .run_if(in_state(RunMode::Normal))
                .run_if(resource_equals(FocusedTool(ToolButton::Pen))),
        )
        .add_systems(
            Update,
            show_window_cursor.run_if(in_state(RunMode::Debug)),
        );
    }
}

#[derive(Component, Default)]
struct Focused;

#[derive(Component, Default)]
struct Line(Vec<Vec2>);

impl From<&Line> for Path {
    fn from(value: &Line) -> Self {
        let mut path_builder = PathBuilder::new();
        let Line(points) = value;

        if let Some((first, rest)) = points.split_first() {
            path_builder.move_to(*first);
            for point in rest {
                path_builder.line_to(*point);
            }
        }
        path_builder.build()
    }
}

fn remove_focused_line(
    focused_line: Query<Entity, With<Focused>>,
    mut commands: Commands,
) {
    if let Ok(focused_line) = focused_line.get_single() {
        commands.entity(focused_line).remove::<Focused>();
    }
}

fn spawn_focused_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<ChalkMaterial>>,
    mut current_layer: Local<i8>,
    touch_cursor: Res<TouchCursor>,
) {
    let mut stroke = Stroke::new(Color::RED, touch_cursor.size);
    stroke.options.line_join = LineJoin::Round;
    stroke.options.start_cap = LineCap::Round;
    stroke.options.end_cap = LineCap::Round;
    commands
        .spawn((
            stroke,
            ShapeBundle::default(),
            Focused,
            Line(vec![]),
            NoFrustumCulling, // 禁止视锥剔除 TODO: 可能有性能问题
            materials.add(ChalkMaterial {
                material_color: touch_cursor.color,
            }),
            crate::layer::Layer::Foreground(*current_layer),
        ))
        .remove::<Handle<ColorMaterial>>(); // 移除颜色材质
    *current_layer = (*current_layer + 1) % (i8::MAX - 1);
}

fn drawing(
    mut focused_line: Query<&mut Line, With<Focused>>,
    world_touch_cursor: Res<WorldTouchCursor>,
) {
    if world_touch_cursor.is_changed() {
        if let Ok(mut focused_line) = focused_line.get_single_mut() {
            let WorldTouchCursor(point) = *world_touch_cursor;
            let last = if let Some(last) = focused_line.0.iter().last() {
                last
            } else {
                &Vec2::ZERO
            };
            if last.distance(point) > 2. {
                focused_line.0.push(point);
            }
        }
    }
}

fn clear_with<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    query: Query<Entity, F>,
) {
    for id in query.iter() {
        commands.entity(id).despawn();
    }
}

fn remove_last_with<F: ReadOnlyWorldQuery>(
    mut commands: Commands,
    query: Query<Entity, F>,
) {
    query.iter().last().and_then(|id| {
        commands.entity(id).despawn();
        Some(())
    });
}

fn undo_condition(keyboard_input: Res<Input<KeyCode>>) -> bool {
    keyboard_input.just_pressed(KeyCode::Z)
        && keyboard_input.pressed(KeyCode::ControlLeft)
}

fn update_line(
    focused_line: Query<(Entity, &Line), Changed<Line>>,
    mut commands: Commands,
) {
    for (id, line) in focused_line.iter() {
        commands.entity(id).insert(Path::from(line));
    }
}

fn remove_line(
    world_touch_cursor: Res<WorldTouchCursor>,
    touch_cursor: Res<TouchCursor>,
    focused_line: Query<(Entity, &Line)>,
    mut commands: Commands,
) {
    for (id, line) in focused_line.iter() {
        if line
            .0
            .iter()
            .any(|p| p.distance(world_touch_cursor.0) <= touch_cursor.size)
        {
            commands.entity(id).despawn();
        }
    }
}

// fn show_mesh(mut q_mesh: Query<&Mesh2dHandle>, mut meshes: Assets<Mesh>) {
//     for mesh in q_mesh.iter_mut() {
//         if let Some(mut mesh) = meshes.get_mut(mesh.0.clone()) {
//             mesh.primitive_topology().set
//         }
//     }
// }
