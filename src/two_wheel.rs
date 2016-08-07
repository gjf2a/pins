use motor::MotorStates;
use motor::Direction;
use motor::Motor;
use std::io;
use config_robot;

pub struct TwoWheelBase {
  motors: MotorStates,
}

impl TwoWheelBase {
  pub fn from(config: &config_robot::RobotConfig) -> io::Result<TwoWheelBase> {
    if config.motors.len() == 2 {
      let mut twb = TwoWheelBase {motors: MotorStates::new(config.steps_per_sec)};
      twb.motors.add_motor(Motor::new(&config.motors[0]));
      twb.motors.add_motor(Motor::new(&config.motors[1]));
      Ok(twb)
    } else {
      config_robot::make_error_msg("Must have exactly two motors")
    }
  }

  pub fn forward(&mut self) {
    self.motors.set_dir(0, Direction::Forward);
    self.motors.set_dir(1, Direction::Forward);
    self.motors.run();
  }

  pub fn left(&mut self) {
    self.motors.set_dir(0, Direction::Reverse);
    self.motors.set_dir(1, Direction::Forward);
    self.motors.run();
  }

  pub fn right(&mut self) {
    self.motors.set_dir(0, Direction::Forward);
    self.motors.set_dir(1, Direction::Reverse);
    self.motors.run();
  }

  pub fn reverse(&mut self) {
    self.motors.set_dir(0, Direction::Reverse);
    self.motors.set_dir(1, Direction::Reverse);
    self.motors.run();
  }
}
