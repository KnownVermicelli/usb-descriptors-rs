#[macro_export]
macro_rules! upper {
    ($exp:expr) => {
        ($exp >> 8) as u8
    };
}

#[macro_export]
macro_rules! lower {
    ($exp:expr) => {
        ($exp & 0x00ff) as u8
    };
}
