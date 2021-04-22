#[macro_export]
macro_rules! left {
    ( $ctx:expr) => {
        {
            input::is_key_down($ctx, Key::Left)
        }
    };
}

#[macro_export]
macro_rules! right {
    ( $ctx:expr) => {
        {
            input::is_key_down($ctx, Key::Right)
        }
    };
}

#[macro_export]
macro_rules! up {
    ( $ctx:expr) => {
        {
            input::is_key_down($ctx, Key::Up)
        }
    };
}

#[macro_export]
macro_rules! down {
    ( $ctx:expr) => {
        {
            input::is_key_down($ctx, Key::Down)
        }
    };
}

#[macro_export]
macro_rules! deg_to_rad {
    ( $angle:expr) => {
        {
            input::is_key_down($ctx, Key::Down)
        }
    };
}