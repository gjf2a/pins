use config_robot;
use pins;
use std::io;
use two_wheel::TwoWheelBase;

pub struct SimpleAvoidRobot {
  base: TwoWheelBase,
  front: u32,
  max_front_voltage: u32,
  max_steps: Option<u32>,
}

impl SimpleAvoidRobot {
  pub fn from(config: &config_robot::RobotConfig) -> io::Result<SimpleAvoidRobot> {
    match TwoWheelBase::from(config) {
      Ok(base) => {
        if config.ir_sensors.len() >= 1 {
          Ok(SimpleAvoidRobot {base: base, front: config.ir_sensors[0], max_front_voltage: 1000, max_steps: config.max_steps})
        } else {
          config_robot::make_error_msg("Must have at least one IR sensor")
        }
      },
      Err(err) => Err(err),
    }
  }

  pub fn run(&mut self) {
    let mut count = 0;
    loop {
      match self.max_steps {
        None => {},
        Some(step) => {
          if count > step {break;}
          else {count = count + 1;}
        },
      }

      match pins::read_adc_voltage(self.front) {
        Ok(ir_voltage) => {
          if ir_voltage > self.max_front_voltage {
            self.base.right();
          } else {
            self.base.forward();
          }
        },
        Err(_) => {},
      }
    }
  }
}
