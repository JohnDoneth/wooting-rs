
extern crate rgb as color;
extern crate wooting_rgb_sys;
extern crate wooting_analog_sys;

pub mod rgb;
pub mod analog;

pub const KEYBOARD_COLUMNS : u8 = wooting_rgb_sys::WOOTING_RGB_COLS as u8;
pub const KEYBOARD_ROWS : u8 = wooting_rgb_sys::WOOTING_RGB_ROWS as u8;

/// Returns true if a Wooting keyboard is connected
pub fn is_keyboard_connected() -> bool {
    use wooting_analog_sys::wooting_kbd_connected;

    unsafe { wooting_kbd_connected() }
}

/// Assert that the given row and column are within bounds
fn assert_key_bounds(row : u8, column: u8) {
    assert!(column < KEYBOARD_COLUMNS, "Key column out of bounds");
    assert!(row < KEYBOARD_ROWS, "Key row out of bounds");
}