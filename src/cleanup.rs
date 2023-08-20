use bevy_ecs::prelude::*;
use bevy_hierarchy::DespawnRecursiveExt;

pub fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
