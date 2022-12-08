#[macro_export]
macro_rules! left {
    ( $ctx:expr) => {{
        tetra::input::is_key_down($ctx, tetra::input::Key::Left)
    }};
}

#[macro_export]
macro_rules! right {
    ( $ctx:expr) => {{
        tetra::input::is_key_down($ctx, tetra::input::Key::Right)
    }};
}

#[macro_export]
macro_rules! up {
    ( $ctx:expr) => {{
        tetra::input::is_key_down($ctx, tetra::input::Key::Up)
    }};
}

#[macro_export]
macro_rules! down {
    ( $ctx:expr) => {{
        tetra::input::is_key_down($ctx, tetra::input::Key::Down)
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
