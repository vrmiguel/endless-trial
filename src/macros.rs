#[macro_export]
macro_rules! left {
    ( $ctx:expr) => {{
        input::is_key_down($ctx, Key::Left)
    }};
}

#[macro_export]
macro_rules! right {
    ( $ctx:expr) => {{
        input::is_key_down($ctx, Key::Right)
    }};
}

#[macro_export]
macro_rules! up {
    ( $ctx:expr) => {{
        input::is_key_down($ctx, Key::Up)
    }};
}

#[macro_export]
macro_rules! down {
    ( $ctx:expr) => {{
        input::is_key_down($ctx, Key::Down)
    }};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($($arg:tt)*) => (println!($($arg)*));
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}
