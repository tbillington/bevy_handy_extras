use bevy_ecs::prelude::*;
use bevy_hierarchy::DespawnRecursiveExt;

/// Despawn all entities that have a particular component.
///
/// ```rust
/// app.add_systems(OnExit(GameState::InGame), cleanup::<InGameTag>);
/// ```
pub fn cleanup<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
