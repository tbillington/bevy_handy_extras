use bevy_ecs::prelude::*;
use bevy_state::prelude::*;

/// Sets a state, saves writing a whole system just to do this.
///
/// ```rust
/// app.add_systems(Update, set_state(GameState::GameOver).run_if(on_event::<PlayerDied>()));
/// ```
pub fn set_state<S: States + Clone + bevy_state::state::FreelyMutableState>(
    state: S,
) -> impl FnMut(ResMut<NextState<S>>) {
    move |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    }
}
