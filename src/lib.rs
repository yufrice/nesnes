#![feature(test)]
extern crate test;

#[macro_use]
extern crate auto_enums;
extern crate env_logger;
extern crate log;
extern crate sdl2;

pub mod arch;
pub mod parser;
pub mod ui;

const SPRITE_SIDE: usize = 8;
const SPRITE: usize = SPRITE_SIDE * SPRITE_SIDE;
const PATTERN_LENGTH: usize = 0x200;

const DISPLAY_SIZE: usize = 184_320;
const DISPLAY_SPRITE_WIDTH: usize = 32;
const DISPLAY_HEIGHT: usize = 240;
const DISPLAY_WIDTH: usize = 256;
