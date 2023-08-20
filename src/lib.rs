#[macro_use]
pub mod macros;
pub mod cleanup;
pub mod math;

use bevy_ecs::prelude::*;
pub use macros::*;

pub fn set_state<S: States>(next_state: S) -> impl FnMut(ResMut<NextState<S>>) {
    move |mut next_state_res: ResMut<NextState<S>>| {
        next_state_res.set(next_state.clone());
    }
}
