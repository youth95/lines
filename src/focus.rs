// 1. 点选 包围盒 判定
// 2. TODO: 然后 二次 Mesh 内判定
use bevy::{
    input::common_conditions::input_pressed, prelude::*,
    render::primitives::Aabb,
};

use crate::{
    cursor::WorldTouchCursor, layer::Layer, selected::Selected,
    states::ToolButton,
};

#[derive(Resource, Default)]
pub struct HoveredMesh(pub Option<Entity>);

pub struct MeshFocusPlugin;

impl Plugin for MeshFocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredMesh>().add_systems(
            Update,
            (mesh_focus_system, draw_focus)
                .run_if(in_state(ToolButton::Cursor))
                .run_if(not(input_pressed(MouseButton::Left))),
        );
    }
}

pub fn find_entity_with_world_cursor<'a>(
    node_query: &'a Query<(Entity, &Aabb, &Layer)>,
    world_touch_cursor: Res<WorldTouchCursor>,
) -> Option<(Entity, &'a i8, &'a Aabb)> {
    node_query
        .iter()
        .filter_map(|(entity, aabb, Layer::Foreground(layer))| {
            let WorldTouchCursor(Vec2 { x, y }) = *world_touch_cursor;
            if (aabb.center.x - x).abs() <= aabb.half_extents.x
                && (aabb.center.y - y).abs() <= aabb.half_extents.y
            {
                Some((entity, layer, aabb))
            } else {
                None
            }
        })
        .max_by(|(_, l1, _), (_, l2, _)| l1.cmp(l2))
}

fn mesh_focus_system(
    node_query: Query<(Entity, &Aabb, &Layer)>,
    world_touch_cursor: Res<WorldTouchCursor>,
    selected: Res<Selected>,
    mut hovered_mesh: ResMut<HoveredMesh>,
) {
    let entity = find_entity_with_world_cursor(&node_query, world_touch_cursor)
        .and_then(|(entity, _, _)| Some(entity));
    if let (Some(a), Some(b)) = (selected.0.get(0), entity) {
        if *a == b {
            *hovered_mesh = HoveredMesh(None);
            return;
        }
    }
    *hovered_mesh = HoveredMesh(entity);
}

fn draw_focus(
    mut gizmos: Gizmos,
    node_query: Query<(Entity, &Aabb, &Layer)>,
    hovered_mesh: Res<HoveredMesh>,
) {
    hovered_mesh.0.and_then(|entity| {
        let _ = node_query.get(entity).and_then(|(_, aabb, _)| {
            gizmos.rect_2d(
                aabb.center.xy(),
                0.,
                aabb.half_extents.xy() * 2.,
                Color::BLUE,
            );
            Ok(())
        });
        Some(())
    });
}
