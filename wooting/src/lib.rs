
extern crate rgb as color;
extern crate wooting_rgb_sys;

pub mod rgb;
pub mod analog;

pub const KEYBOARD_COLUMNS : u8 = wooting_rgb_sys::WOOTING_RGB_COLS as u8;
pub const KEYBOARD_ROWS : u8 = wooting_rgb_sys::WOOTING_RGB_ROWS as u8;
