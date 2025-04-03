#[cfg(debug_assertions)]
pub fn init_logger() {
    env_logger::init();
}

#[cfg(not(debug_assertions))]
pub fn init_logger() {
    // noop
}
