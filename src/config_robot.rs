use std::io;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct RobotConfig {
	motors: Vec<[&'static str; 4]>,
	ir_sensors: Vec<u32>,
	steps_per_sec: u32,
	max_steps: Option<u32>, 
}

pub fn parse_file(filename: &str) -> io::Result<RobotConfig> {
	let file_path = Path::new(&filename);
	let reader = File::open(&file_path).map(|f| BufReader::new(f));
	
	match reader {
		Ok(r) => {
			let mut result = RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None};
			for line in r.lines() {
			}
			Result::Ok(result)
		},
		Err(r) => Result::Err(r),
	}
}
