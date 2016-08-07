use four;
use std::time::{Duration, Instant};

pub enum Direction {Forward, Reverse,}

pub struct MotorStates {
  motors: Vec<Motor>,
  sleep_time: Duration,
  next_pin: usize,
  target: Instant,
}

impl MotorStates {
  pub fn new(steps_per_sec: u32) -> MotorStates {
    MotorStates {motors: Vec::new(), sleep_time: four::find_sleep_time(steps_per_sec), next_pin: 0, target: Instant::now()}
  }

  pub fn add_motor(&mut self, m: Motor) {
    self.motors.push(m);
  }

  pub fn set_delay(&mut self, steps_per_sec: u32) {
    self.sleep_time = four::find_sleep_time(steps_per_sec);
  }

  pub fn num_motors(&self) -> usize {
    self.motors.len()
  }

  pub fn set_dir(&mut self, motor: usize, dir: Direction) {
    self.motors[motor].set_dir(dir);
  }

  pub fn run(&mut self) {
    set_all_at(&self.motors, self.next_pin, "1");
    self.target = four::wait_until(self.target, self.sleep_time);
    set_all_at(&self.motors, self.next_pin, "0");
    self.target = four::wait_until(self.target, self.sleep_time);
    self.next_pin = (self.next_pin + 1) % 4;
  }
}

fn set_all_at(pin_nums: &Vec<Motor>, pin_i: usize, value: &str) {
	for pins in pin_nums {
    super::pins::gpio_cmd(&pins.get_pins()[pin_i], "value", &value);
  }
}

pub struct Motor {
  for_pins: [String; 4],
  rev_pins: [String; 4],
  dir: Direction,
}

impl Drop for Motor {
  fn drop(&mut self) {
    for pin in self.for_pins.iter() {
      super::pins::takedown_gpio(pin)
    }
  }
}

impl Motor {
  pub fn new(pins: &[String; 4]) -> Motor {
    for pin in pins.iter() {
      four::setup_gpio(pin);
    }
    Motor {for_pins: copy_pins(pins), rev_pins: rev_pins(pins), dir: Direction::Forward}
  }

  pub fn get_pins(&self) -> &[String; 4] {
    match self.dir {
      Direction::Forward => &self.for_pins,
      Direction::Reverse => &self.rev_pins,
    }
  }

  pub fn set_dir(&mut self, new_dir: Direction) {
    self.dir = new_dir;
  }
}

fn rev_pins(pins: &[String; 4]) -> [String; 4] {
  [pins[3].clone(), pins[2].clone(), pins[1].clone(), pins[0].clone()]
}

pub fn copy_pins(pins: &[String; 4]) -> [String; 4] {
  [pins[0].clone(), pins[1].clone(), pins[2].clone(), pins[3].clone()]
}
