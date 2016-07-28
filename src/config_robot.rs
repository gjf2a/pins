use std::io;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
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
	process_lines(File::open(&Path::new(&filename)).map(|f| BufReader::new(f)))
}

fn process_lines(reader: io::Result<BufReader<File>>) -> io::Result<RobotConfig> {
	match reader {
		Ok(r) => r.lines().fold(Ok(RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None}), process_line),
		Err(r) => Err(r),
	}
}

fn process_line(config: io::Result<RobotConfig>, line: io::Result<String>) -> io::Result<RobotConfig> {
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
			} else if keyword == "ir" {
				match contents.next() {
					Some(ir_str) => {
						let mut updated = copy(config.ir_sensors);
						match ir_str.parse::<u32>() {
							Ok(value) => {
								updated.push(value);
								Ok(RobotConfig {motors: config.motors, ir_sensors: updated, steps_per_sec: config.steps_per_sec, max_steps: config.max_steps})
							},
							Err(err) => make_error_msg(err.description()),
						}
					},
					None => make_error_msg("No pin specified"),
				}
			} else if keyword == "hz" {
				match contents.next() {
					Some(hz_str) => {
						match hz_str.parse::<u32>() {
							Ok(value) => Ok(RobotConfig {motors: config.motors, ir_sensors: config.ir_sensors, steps_per_sec: value, max_steps: config.max_steps}),
							Err(err) => make_error_msg(err.description()),
						}
					},
					None => make_error_msg("Cycles per second not specified"),
				}
			} else if keyword == "max" {
				match contents.next() {
					Some(max_str) => {
						match max_str.parse::<u32>() {
							Ok(value) => Ok(RobotConfig {motors: config.motors, ir_sensors: config.ir_sensors, steps_per_sec: config.steps_per_sec, max_steps: Some(value)}),
							Err(err) => make_error_msg(err.description()),
						}
					},
					None => make_error_msg("Maximum steps not specified"),
				}
			} else {
				make_error_msg(&format!("Unrecognized keyword: {}", keyword))
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
		make_error_msg(&format!("4 pins expected; received {}", contents.len()))
	}
}

fn copy<T>(v: Vec<T>) -> Vec<T> {
	let mut result = Vec::new();
	for arr in v {
		result.push(arr);
	}
	result
}

fn make_error_msg<T>(msg: &str) -> io::Result<T> {
	Err(io::Error::new(ErrorKind::InvalidInput, msg))
}
