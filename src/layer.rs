use bevy::prelude::*;

#[derive(Component)]
pub enum Layer {
    // Background(i8),
    Foreground(i8),
}

pub fn update_z_coordinate_based_on_layer(
    mut query: Query<(&mut Transform, &Layer), Changed<Layer>>,
) {
    for (mut transform, layer) in query.iter_mut() {
        transform.translation.z = match layer {
            // Layer::Background(order_in_layer) => -1. + *order_in_layer as f32 / 1000.,
            Layer::Foreground(order_in_layer) => 0. + *order_in_layer as f32 / 1000.,
        }
    }
}
