use config_robot;
use std::io;
use two_wheel::TwoWheelBase;

pub struct SimpleAvoidRobot {
  base: TwoWheelBase,
  front: u32,
  max_steps: Option<u32>,
}

impl SimpleAvoidRobot {
  pub fn from(config: &config_robot::RobotConfig) -> io::Result<SimpleAvoidRobot> {
    match TwoWheelBase::from(config) {
      Ok(base) => {
        if config.ir_sensors.len() == 1 {
          Ok(SimpleAvoidRobot {base: base, front: config.ir_sensors[0], max_steps: config.max_steps})
        } else {
          config_robot::make_error_msg("Must have exactly one IR sensor")
        }
      },
      Err(err) => Err(err),
    }
  }

  pub fn run(&self) {
    
  }
}
