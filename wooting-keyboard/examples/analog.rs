extern crate wooting_keyboard;

use wooting_keyboard::analog;

fn main() {

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