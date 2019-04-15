use std::env;
use env_logger;

extern crate nesnes;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    match nesnes::parser::parser("test0.nes") {
        Ok(arch) => nesnes::ui::run(arch),
        Err(err) => println!("{:?}", err),
    }
}
