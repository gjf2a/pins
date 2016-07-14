extern crate pins;
use std::env;

fn main() {
        let first = env::args().nth(1);
        let second = env::args().nth(2);
        if let Some(pin) = first {
                if let Some(value) = second {
                        println!("{} {}", pin, value);
                        pins::pins::setup_gpio(&pin);
                        std::process::exit(0);
                }
        }
        println!("Usage: gpio_demo pin value")
}
