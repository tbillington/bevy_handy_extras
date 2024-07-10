#[macro_export]
macro_rules! some_continue {
    ($q:expr) => {
        match $q {
            Some(m) => m,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! ok_continue {
    ($q:expr) => {
        match $q {
            Ok(m) => m,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! get_continue {
    ($q:expr, $r:expr) => {
        match $q.get($r) {
            Ok(m) => m,
            _ => continue,
        }
    };
}

#[macro_export]
macro_rules! get_mut_continue {
    ($q:expr, $r:expr) => {
        match $q.get_mut($r) {
            Ok(m) => m,
            _ => continue,
        }
    };
}

/// To run a system from within another system
/// ```rust
/// fn foo(mut commands: Commands) {
///     commands.add(run_system!(my_system));
/// }
/// ```
#[macro_export]
macro_rules! run_system {
    ($s:ident) => {
        |world: &mut World| {
            use bevy::ecs::system::RunSystemOnce;
            world.run_system_once($s);
        }
    };
}

// TODO: variants that throw in debug but return in release?

/// Unwraps an [`Option`], returning on [`Option::None`]
#[allow(unused)]
#[macro_export]
macro_rules! some {
    ($q:expr) => {
        match $q {
            Some(m) => m,
            _ => return,
        }
    };
}

/// Unwraps an [`Result`], returning on [`Result::Err`]
#[allow(unused)]
#[macro_export]
macro_rules! ok {
    ($q:expr) => {
        match $q {
            Ok(m) => m,
            _ => return,
        }
    };
}

/// Shortcut to get components from a query by [`bevy_ecs::Entity`].
///
/// ```rust
/// fn my_sys(transform: Query<&Transform>) {
///     let my_entity: Entity = ...;
///     let window = get!(transform, my_entity);
/// }
/// ```
#[allow(unused)]
#[macro_export]
macro_rules! get {
    ($q:expr, $r:expr) => {
        match $q.get($r) {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! get_else {
    ($q:expr, $r:expr, $e:expr) => {
        match $q.get($r) {
            Ok(m) => m,
            _ => return $e,
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! get_mut {
    ($q:expr, $r:expr) => {
        match $q.get_mut($r) {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! get_mut_else {
    ($q:expr, $r:expr, $e:expr) => {
        match $q.get_mut($r) {
            Ok(m) => m,
            _ => return $e,
        }
    };
}

/// Shortcut to get components from a single entity query by [`bevy_ecs::Entity`].
///
/// ```rust
/// fn my_sys(window: Query<&Window, With<PrimaryWindow>>) {
///     let window = get_single!(window);
/// }
/// ```
#[allow(unused)]
#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}

/// Clears the buffer then writes formatted data in.
#[allow(unused)]
#[macro_export]
macro_rules! write_cleared {
    ($dst:expr, $($arg:tt)*) => {
        let d = $dst;
        d.clear();
        let _ = d.write_fmt(core::format_args!($($arg)*));
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! pub_prelude {
    ($($m:ident),+) => {
        $(
            #[allow(unused)]
            pub mod $m {
                pub use crate::$m::prelude::*;
            }
        )+
    };
}

#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! pub_crate_prelude {
    ($($m:ident),+) => {
        $(
            #[allow(unused)]
            pub(crate) mod $m {
                pub(crate) use crate::$m::prelude::*;
            }
        )+
    };
}
