use std::io;

struct RobotConfig {
	motors: Vec<[&str; 4]>,
	ir_sensors: Vec<u32>,
	steps_per_sec: u32,
	max_steps: Option<u32>, 
}

pub fn parse_file(filename: &str) -> io::Result<RobotConfig> {
	
}
