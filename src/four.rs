use std::time::{Duration, Instant};

pub fn run_all(pin_nums: &Vec<[&str; 4]>, num_steps: u32, steps_per_sec: u32) {
	setup_gpios(pin_nums);
	go_n_steps(num_steps, pin_nums, find_sleep_time(steps_per_sec));
	takedown_gpios(pin_nums);
}

pub fn do_all<F>(pin_nums: &Vec<[&str; 4]>, todo: F) 
	where F : Fn(&str) {
        for pins in pin_nums {
                for pin in pins.into_iter() {
			todo(pin);
		}
	}
}

pub fn setup_gpios(pin_nums: &Vec<[&str; 4]>) {
	do_all(pin_nums, |pin: &str| {
		super::pins::setup_gpio(&pin);
        	super::pins::gpio_cmd(&pin, "direction", "out");
	})
}

pub fn takedown_gpios(pin_nums: &Vec<[&str; 4]>) {
	do_all(pin_nums, |pin: &str| super::pins::takedown_gpio(&pin))
}

pub fn set_all_at(pin_nums: &Vec<[&str; 4]>, pin_i: usize, value: &str) {
	for pins in pin_nums {
        	super::pins::gpio_cmd(pins[pin_i], "value", &value);
        }
}

pub fn go_n_steps(n: u32, pin_nums: &Vec<[&str; 4]>, sleep_time: Duration) {
	let mut target: Instant = Instant::now() + sleep_time;
	for step in 0..n {
		let pin_i = step as usize % 4;
		set_all_at(pin_nums, pin_i, "1");
		target = wait_until(target, sleep_time);
		set_all_at(pin_nums, pin_i, "0");
		target = wait_until(target, sleep_time);
	}
}

pub fn wait_until(target: Instant, sleep_time: Duration) -> Instant {
	while Instant::now() < target {}
	Instant::now() + sleep_time
}

pub fn find_sleep_time(steps_per_sec: u32) -> Duration {
	let nano = 1000000000;
	Duration::new(0, nano / (2 * steps_per_sec))
}
