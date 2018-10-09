
extern crate wooting;
extern crate rgb as color;

use color::RGB8;

use wooting::rgb;
use wooting::{KEYBOARD_ROWS, KEYBOARD_COLUMNS};

fn main() {

    for row in 0..KEYBOARD_ROWS {
        for column in 0..KEYBOARD_COLUMNS {
            rgb::direct_set_single_key(row, column, RGB8::new(255, 0, 0));

            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    }

    rgb::reset();

}