use config_robot;

// Create a data type which is a list of Motors.
// Each entry will have a Motor, an update-time, and a next-pin.
// There is some kind of loop to run them.
//
// Within the loop, each Motor's update-time is initialized based on its
// steps_per_sec.
//
// On each loop iteration, the current time is checked against each motor's
// update-time. Every motor which is past its time advances to its next-pin,
// updates its next-pin, and updates its update-time.

pub enum Direction {Forward, Reverse,}

pub struct Motor {
  for_pins: [String; 4],
  rev_pins: [String; 4],
  steps_per_sec: u32,
  dir: Direction,
}

pub fn new(pins: [String; 4], steps_per_sec: u32) -> Motor {
  Motor {for_pins: copy_pins(&pins), rev_pins: rev_pins(&pins), steps_per_sec: steps_per_sec, dir: Direction::Forward}
}

impl Motor {
  pub fn get_pins(&self) -> &[String; 4] {
    match self.dir {
      Direction::Forward => &self.for_pins,
      Direction::Reverse => &self.rev_pins,
    }
  }
}

fn rev_pins(pins: &[String; 4]) -> [String; 4] {
  [pins[3].clone(), pins[2].clone(), pins[1].clone(), pins[0].clone()]
}

pub fn copy_pins(pins: &[String; 4]) -> [String; 4] {
  [pins[0].clone(), pins[1].clone(), pins[2].clone(), pins[3].clone()]
}
