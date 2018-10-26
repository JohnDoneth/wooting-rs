extern crate wooting_keyboard;
extern crate sharedlib;

use wooting_keyboard::analog;

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

    let mut last_value = 0;

    println!("Press the 'Space' key!");

    loop {

        let current_value = analog::read_key(5, 6);

        if current_value != last_value {
            println!("Space Key Analog Reading: {}", current_value);
            last_value = current_value;
        }

        std::thread::sleep(std::time::Duration::from_millis(30));

    }

}