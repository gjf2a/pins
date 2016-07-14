extern crate pins;
use std::env;
use pins::four;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::str::SplitWhitespace;

fn main() {
	let mut steps_per_sec = 400;
	let mut num_steps = 800;
        let mut content = String::new();
	let mut pin_nums = Vec::new();

	if let Some(config) = env::args().nth(1) {
		if config == "--help" {
			println!("Usage: multi_motor config_file num_steps steps_per_sec");
			std::process::exit(0);
		}
		let config_path = Path::new(&config);
		let mut config_file = File::open(&config_path).ok().unwrap();
		config_file.read_to_string(&mut content).unwrap();
		for line in content.lines() {
			pin_nums.push(make_pins(line.split_whitespace()));
		}
	} else {
		pin_nums.push(["67", "68", "44", "26"]);
	}

	if let Some(n) = env::args().nth(2) {
		num_steps = n.trim().parse::<u32>().ok().unwrap();
	}

	if let Some(sps) = env::args().nth(3) {
		steps_per_sec = sps.trim().parse::<u32>().ok().unwrap();
	}

	four::run_all(&pin_nums, num_steps, steps_per_sec);
}

fn make_pins(mut pins: SplitWhitespace) -> [&str; 4] {
	let mut result = ["0", "0", "0", "0"];
	for i in 0..4 {
		result[i] = pins.next().unwrap();
	}
	return result;
}
