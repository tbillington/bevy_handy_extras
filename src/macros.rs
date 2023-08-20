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

#[allow(unused)]
#[macro_export]
macro_rules! get {
    ($q:expr,$r:expr) => {
        match $q.get($r) {
            Ok(m) => m,
            _ => return,
        }
    };
}

#[allow(unused)]
#[macro_export]
macro_rules! get_mut {
    ($q:expr,$r:expr) => {
        match $q.get_mut($r) {
            Ok(m) => m,
            _ => return,
        }
    };
}

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
