use std::env;
use env_logger;

extern crate nesnes;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    nesnes::ui::run();
}
