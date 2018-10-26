extern crate wooting_keyboard;
extern crate rgb as color;
extern crate sharedlib;

use color::RGB8;

use wooting_keyboard::rgb;
use wooting_keyboard::{KEYBOARD_ROWS, KEYBOARD_COLUMNS};

use sharedlib::{Lib, Func, Symbol};

fn run_visualizer() {
    unsafe {
        let path_to_lib = "visualizer.dll";
        let lib = Lib::new(path_to_lib).expect("could not find visualizer.dll");
        let visualizer_init_symbol: Func<extern "C" fn()> = lib.find_func("visualizer_init").unwrap();
        let visualizer_init = visualizer_init_symbol.get();
        visualizer_init();
    }
}

fn main() {

    run_visualizer();

    for row in 0..KEYBOARD_ROWS {
        for column in 0..KEYBOARD_COLUMNS {
            rgb::direct_set_single_key(row, column, RGB8::new(0, 255, 255));

            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    }

    rgb::reset();

}