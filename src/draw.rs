use bevy::{
    ecs::query::ReadOnlyWorldQuery,
    input::common_conditions::{
        input_just_pressed, input_just_released, input_pressed,
    },
    pbr::wireframe::Wireframe,
    prelude::*,
    sprite::{Material2dPlugin, Mesh2dHandle},
};
use bevy_prototype_lyon::prelude::*;

use crate::{
    chalk::ChalkMaterial,
    common::show_window_cursor,
    frame::FrameMaterial,
    states::{AppState, RunMode},
    toggle_component::{self, Toggle},
    touch_cursor::{TouchCursor, TouchCursorPlugin, WorldTouchCursor},
    ui::{FocusedTool, ToolButton},
};

pub struct DrawPlugin;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        let clear_condition = input_pressed(KeyCode::C);
        let draw_all_true = |world: &mut World| {
            world.insert_resource(GizmoConfig {
                aabb: AabbGizmoConfig {
                    draw_all: true,
                    ..default()
                },
                ..default()
            })
        };
        let draw_all_false = |world: &mut World| {
            world.insert_resource(GizmoConfig {
                aabb: AabbGizmoConfig {
                    draw_all: false,
                    ..default()
                },
                ..default()
            })
        };
        app.add_plugins((
            Material2dPlugin::<ChalkMaterial>::default(),
            Material2dPlugin::<FrameMaterial>::default(),
            TouchCursorPlugin,
        ))
        .add_systems(
            Update,
            toggle_component::toggle_component::<
                Handle<ChalkMaterial>,
                Handle<FrameMaterial>,
            >
                .run_if(
                    input_just_pressed(KeyCode::Tab)
                        .or_else(input_just_released(KeyCode::Tab)),
                ),
        )
        .add_systems(
            Update,
            draw_all_true.run_if(input_just_pressed(KeyCode::Tab)),
        )
        .add_systems(
            Update,
            draw_all_false.run_if(input_just_released(KeyCode::Tab)),
        )
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
    focused_line: Query<(Entity, &Line), With<Focused>>,
    mut commands: Commands,
) {
    if let Ok((focused_line, line)) = focused_line.get_single() {
        if line.0.len() == 0 {
            commands.entity(focused_line).despawn();
        } else {
            commands.entity(focused_line).remove::<Focused>();
        }
    }
}

fn spawn_focused_line(
    mut commands: Commands,
    mut materials: ResMut<Assets<ChalkMaterial>>,
    mut frame_materials: ResMut<Assets<FrameMaterial>>,
    mut current_layer: Local<i8>,
    touch_cursor: Res<TouchCursor>,
) {
    let mut stroke = Stroke::new(Color::RED, touch_cursor.size);
    stroke.options.line_join = LineJoin::Round;
    stroke.options.start_cap = LineCap::Round;
    stroke.options.end_cap = LineCap::Round;

    let toggle_material = Toggle(
        materials.add(ChalkMaterial {
            material_color: touch_cursor.color,
        }),
        frame_materials.add(FrameMaterial::default()),
    );

    commands.spawn((
        stroke,
        Path::default(),
        Mesh2dHandle::default(),
        SpatialBundle::default(),
        toggle_material.0.clone(),
        toggle_material,
        Focused,
        Line(vec![]),
        Wireframe,
        crate::layer::Layer::Foreground(*current_layer),
    ));
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
    touch_cursor: Res<TouchCursor>,
    mut commands: Commands,
) {
    let mut stroke = Stroke::new(Color::RED, touch_cursor.size);
    stroke.options.line_join = LineJoin::Round;
    stroke.options.start_cap = LineCap::Round;
    stroke.options.end_cap = LineCap::Round;
    for (id, line) in focused_line.iter() {
        commands.entity(id).insert((Path::from(line), stroke)); // Remove Aabb 以重新计算包围盒
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
