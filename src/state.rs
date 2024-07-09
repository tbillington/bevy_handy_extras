use bevy_ecs::prelude::*;
use bevy_state::prelude::*;

pub fn set_state<S: States + Clone + bevy_state::state::FreelyMutableState>(
    state: S,
) -> impl FnMut(ResMut<NextState<S>>) {
    move |mut next_state: ResMut<NextState<S>>| {
        next_state.set(state.clone());
    }
}
