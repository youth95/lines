// 1. 点选 包围盒 判定
// 2. 然后 二次 Mesh 内判定
use bevy::{prelude::*, render::primitives::Aabb};

use crate::{
    projection_2d_control::MainCamera, touch_cursor::WorldTouchCursor,
};

pub fn mesh_focus_system(
    node_query: Query<&Aabb>,
    camera_query: Query<&OrthographicProjection, With<MainCamera>>,
    world_touch_cursor: Res<WorldTouchCursor>,
    mut gizmos: Gizmos,
) {
    let WorldTouchCursor(Vec2 { x, y }) = *world_touch_cursor;
    for aabb in node_query.iter() {
        if (aabb.center.x - x).abs() <= aabb.half_extents.x
            && (aabb.center.y - y).abs() <= aabb.half_extents.y
        {
            gizmos.rect_2d(
                aabb.center.xy(),
                0.,
                aabb.half_extents.xy() * 2. * camera_query.single().scale,
                Color::BLUE,
            );
        }
    }
}
