use std::io;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::str::SplitWhitespace;

pub struct RobotConfig {
	motors: Vec<[String; 4]>,
	ir_sensors: Vec<u32>,
	steps_per_sec: u32,
	max_steps: Option<u32>,
}

pub fn parse_file(filename: &str) -> io::Result<RobotConfig> {
	let file_path = Path::new(&filename);
	let reader = File::open(&file_path).map(|f| BufReader::new(f));

	match reader {
		Ok(r) => {
			let mut result = Ok(RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None});
			for line in r.lines() {
				result = process_line(line, result);
			}
			result
		},
		Err(r) => Err(r),
	}
}

fn process_line(line: io::Result<String>, config: io::Result<RobotConfig>) -> io::Result<RobotConfig> {
	match line {
		Ok(line) => {
			let mut parts = line.split_whitespace();
			let keyword = parts.next();
			match keyword {
				Some(keyword) => process_contents(keyword, &mut parts, Ok(RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None})),
				None => config,
			}
		},
		Err(line) => Err(line),
	}
}

fn process_contents(keyword: &str, contents: &mut SplitWhitespace, config: io::Result<RobotConfig>) -> io::Result<RobotConfig> {
	match config {
		Ok(config) => {
			if keyword == "motor" {
				match array_from(contents) {
					Ok(contents) => {
						let mut updated = copy(config.motors);
						updated.push(contents);
						Ok(RobotConfig {motors: updated, ir_sensors: config.ir_sensors, steps_per_sec: config.steps_per_sec, max_steps: config.max_steps})
					},
					Err(err) => Err(err),
				}
			} else {
				Err(Error::new(ErrorKind::InvalidInput, format!("Unrecognized keyword: {}", keyword)))
			}
		},
		Err(err) => Err(err),
	}
}

fn array_from(contents: &mut SplitWhitespace) -> io::Result<[String; 4]> {
	let contents: Vec<&str> = contents.collect();
	if contents.len() == 4 {
		Ok([String::from(contents[0]), String::from(contents[1]),
		String::from(contents[2]), String::from(contents[3])])
	} else {
		Err(Error::new(ErrorKind::InvalidInput, format!("4 pins expected; received {}", contents.len())))
	}
}

fn copy(v: Vec<[String; 4]>) -> Vec<[String; 4]> {
	let mut result = Vec::new();
	for arr in v {
		result.push(arr);
	}
	result
}
