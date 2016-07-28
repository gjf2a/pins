use std::io;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::str::SplitWhitespace;

pub fn parse_file(filename: &str) -> io::Result<RobotConfig> {
	File::open(&Path::new(&filename)).map(|f| BufReader::new(f)).process_lines()
}

pub struct RobotConfig {
	motors: Vec<[String; 4]>,
	ir_sensors: Vec<u32>,
	steps_per_sec: u32,
	max_steps: Option<u32>,
}

impl RobotConfig {
	pub fn add_motor(self, contents: [String; 4]) -> RobotConfig {
		let mut updated = copy(self.motors);
		updated.push(contents);
		RobotConfig {motors: updated, ir_sensors: self.ir_sensors, steps_per_sec: self.steps_per_sec, max_steps: self.max_steps}
	}

	pub fn add_ir(self, ir: u32) -> RobotConfig {
		let mut updated = copy(self.ir_sensors);
		updated.push(ir);
		RobotConfig {motors: self.motors, ir_sensors: updated, steps_per_sec: self.steps_per_sec, max_steps: self.max_steps}
	}

	pub fn set_hz(self, hz: u32) -> RobotConfig {
		RobotConfig {motors: self.motors, ir_sensors: self.ir_sensors, steps_per_sec: hz, max_steps: self.max_steps}
	}

	pub fn set_max(self, max: u32) -> RobotConfig {
		RobotConfig {motors: self.motors, ir_sensors: self.ir_sensors, steps_per_sec: self.steps_per_sec, max_steps: Some(max)}
	}
}

trait LineProcessor {
	fn process_lines(self) -> io::Result<RobotConfig>;
}

impl LineProcessor for io::Result<BufReader<File>> {
	fn process_lines(self) -> io::Result<RobotConfig> {
		match self {
			Ok(r) => r.lines().fold(Ok(RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None}), process_line),
			Err(r) => Err(r),
		}
	}
}

fn process_line(config: io::Result<RobotConfig>, line: io::Result<String>) -> io::Result<RobotConfig> {
	match line {
		Ok(line) => {
			let mut parts = line.split_whitespace();
			match parts.next() {
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
			match keyword {
				"motor" => process_motor_spec(contents, config),
				"ir" => process_single_num(contents.next(), "Pin", |v| config.add_ir(v)),
				"hz" => process_single_num(contents.next(), "Cycles per second", |v| config.set_hz(v)),
				"max" => process_single_num(contents.next(), "Maximum steps", |v| config.set_max(v)),
				_ => make_error_msg(&format!("Unrecognized keyword: {}", keyword)),
			}
		},
		Err(err) => Err(err),
	}
}

fn process_motor_spec(contents: &mut SplitWhitespace, config: RobotConfig) -> io::Result<RobotConfig> {
	match array_from(contents) {
		Ok(contents) => Ok(config.add_motor(contents)),
		Err(err) => Err(err),
	}
}

fn process_single_num<F>(input: Option<&str>, failure: &'static str, generator: F) -> io::Result<RobotConfig>
	where F: FnOnce(u32) -> RobotConfig {
	match input {
		Some(input) => {
			match input.parse::<u32>() {
				Ok(value) => Ok(generator(value)),
				Err(err) => make_error_msg(err.description()),
			}
		},
		None => make_error_msg(&format!("{} not specified", failure)),
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
