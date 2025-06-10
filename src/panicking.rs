use std::panic::panic_any;

#[allow(unused)]
pub fn alarm() {
    panic_any("crash and burn!");
}