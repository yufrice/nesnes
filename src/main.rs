use env_logger;
use std::env;

extern crate nesnes;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    nesnes::ui::run();
}
