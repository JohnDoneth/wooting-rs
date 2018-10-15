use wooting_analog_sys::wooting_read_analog;

use ::assert_key_bounds;

pub fn read_key(row: u8, column: u8) -> u8 {

    assert_key_bounds(row, column);

    unsafe { 
        wooting_read_analog(row, column) 
    }
}
