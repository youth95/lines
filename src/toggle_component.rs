use bevy::prelude::*;

#[derive(Component)]
pub struct Toggle<T: Bundle, T2: Bundle>(pub T, pub T2);

pub fn toggle_component<T: Bundle + Clone, T2: Bundle + Clone>(
    query: Query<(Entity, &Toggle<T, T2>)>,
    mut commands: Commands,
    mut is_bundle_0: Local<bool>,
) {
    for (entity, toggle) in query.iter() {
        if *is_bundle_0 {
            commands
                .entity(entity)
                .insert(toggle.0.clone())
                .remove::<T2>();
        } else {
            commands
                .entity(entity)
                .insert(toggle.1.clone())
                .remove::<T>();
        }
    }
    *is_bundle_0 = !*is_bundle_0;
}
