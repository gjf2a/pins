extern crate pins;

use pins::simple_avoid_robot;
use pins::config_robot;
use std::env;
use std::error::Error;

fn main() {
  match env::args().nth(1) {
    Some(arg) => {
      match config_robot::parse_file(&arg) {
        Ok(config) => {
          match simple_avoid_robot::SimpleAvoidRobot::from(&config) {
            Ok(mut robot) => robot.run(),
            Err(err) => println!("{}", err.description()),
          }
        },
        Err(err) => println!("{}", err.description()),
      }
    },
    None => println!("Usage: simple_robot_demo config_file"),
  }
}
