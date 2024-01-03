// 1. 点选
// 2. 框选
use bevy::{
    input::common_conditions::{input_just_released, input_pressed},
    prelude::*,
    render::primitives::Aabb,
};

use crate::{
    cursor::WorldTouchCursor, focus::find_entity_with_world_cursor,
    layer::Layer, states::ToolButton,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Selected>().add_systems(
            Update,
            (
                selected
                    .in_set(SelectedPlugin)
                    .run_if(in_state(ToolButton::Cursor))
                    .run_if(input_pressed(MouseButton::Left)),
                draw_selected.after(SelectedPlugin),
                cancel_selected_with_cursor
                    .in_set(SelectedPlugin)
                    .run_if(in_state(ToolButton::Cursor))
                    .run_if(input_just_released(MouseButton::Left)),
            ),
        );
    }
}

#[derive(Resource, Default)]
pub struct Selected(pub Vec<Entity>);

fn selected(
    node_query: Query<(Entity, &Aabb, &Layer)>,
    world_touch_cursor: Res<WorldTouchCursor>,
    mut selected: ResMut<Selected>,
) {
    if let Some((entity, _, _)) =
        find_entity_with_world_cursor(&node_query, world_touch_cursor)
    {
        *selected = Selected(vec![entity])
    }
}

fn cancel_selected_with_cursor(
    mut selected: ResMut<Selected>,
    node_query: Query<(Entity, &Aabb, &Layer)>,
    world_touch_cursor: Res<WorldTouchCursor>,
) {
    if let Some((entity, _, _)) =
        find_entity_with_world_cursor(&node_query, world_touch_cursor)
    {
        if selected.0.contains(&entity) {
            return;
        }
    }
    selected.0.clear();
}

fn draw_selected(
    mut gizmos: Gizmos,
    node_query: Query<(Entity, &Aabb, &Layer)>,
    selected: Res<Selected>,
) {
    selected.0.get(0).and_then(|entity| {
        let _ = node_query.get(*entity).and_then(|(_, aabb, _)| {
            gizmos.rect_2d(
                aabb.center.xy(),
                0.,
                aabb.half_extents.xy() * 2.,
                Color::RED,
            );
            Ok(())
        });
        Some(())
    });
}
