use config_robot;
use std::io;
use motor;
use motor::Motor;

pub struct SimpleAvoidRobot {
  left: Motor,
  right: Motor,
  front: u32,
  max_steps: Option<u32>,
}

impl SimpleAvoidRobot {
  pub fn run(self) {

  }
}

pub fn from(config: config_robot::RobotConfig, ir: u32) -> io::Result<SimpleAvoidRobot> {
  if config.motors.len() == 2 {
    Ok(SimpleAvoidRobot {left: motor::new(config.copy_motor(0), config.steps_per_sec),
                        right: motor::new(config.copy_motor(1), config.steps_per_sec),
                        front: ir,
                        max_steps: config.max_steps})
  } else {
    config_robot::make_error_msg("Must have exactly two motors")
  }
}
