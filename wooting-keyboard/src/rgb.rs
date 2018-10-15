use color::RGB8;

use wooting_rgb_sys::wooting_rgb_direct_set_key;
use wooting_rgb_sys::wooting_rgb_direct_reset_key;
use wooting_rgb_sys::wooting_rgb_reset;

pub use wooting_rgb_sys::WOOTING_RGB_COLS;
pub use wooting_rgb_sys::WOOTING_RGB_ROWS;

use ::assert_key_bounds;

/// Directly set and update 1 key on the keyboard
/// 
/// This function will directly change the color of 1 key on the keyboard. This will not influence the keyboard color array.
/// Use this function for simple applifications, like a notification. Use the array functions if you want to change the entire keyboard.
/// 
/// Returns true (1) if the color was set
pub fn direct_set_single_key(row: u8, column: u8, rgb : RGB8) -> bool {

    assert_key_bounds(row, column);
    
    unsafe {
        wooting_rgb_direct_set_key(row, column, rgb.r, rgb.g, rgb.b)
    }

}

#[test]
fn test_array_set_single_key() {
    direct_set_single_key(0, 0, RGB8::new(0, 0, 0));
}

/// Reset a single key
pub fn direct_reset_single_key(row: u8, column: u8) -> bool {

    assert_key_bounds(row, column);

    unsafe {
        wooting_rgb_direct_reset_key(row, column)
    }

}

#[test]
fn test_array_reset_single() {
    direct_reset_single_key(0, 0);
}

/// Reset all colors on keyboard to the original colors.
/// 
/// This function will restore all the colours to the colours that were originally on the keyboard. 
/// This function should always be called when you close the application.
///
/// Return true (1) if the keyboard is reset
pub fn reset() -> bool {
    unsafe {
        wooting_rgb_reset()
    }
}
