use std::panic::panic_any;

pub fn alarm() {
    panic_any("crash and burn!");
}