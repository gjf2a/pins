use std::io;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::io::ErrorKind;
use std::fs::File;
use std::str::SplitWhitespace;
use four;

pub fn parse_file(filename: &str) -> io::Result<RobotConfig> {
	File::open(&Path::new(&filename)).map(|f| BufReader::new(f)).process_lines()
}

pub struct RobotConfig {
	pub motors: Vec<[String; 4]>,
	pub ir_sensors: Vec<u32>,
	pub steps_per_sec: u32,
	pub max_steps: Option<u32>,
}

impl RobotConfig {
	pub fn copy_motor(&self, i: usize) -> [String; 4] {
		let ref arr = self.motors[i];
		[arr[0].clone(), arr[1].clone(), arr[2].clone(), arr[3].clone()]
	}

	pub fn motor_loop(&self) {
		match self.max_steps {
			Some(max) => four::run_all(&self.motors, max, self.steps_per_sec),
			None => panic!("Indefinite run not implemented"),
		}
	}

	pub fn motor_ir_loop(&self, max_forward_voltage: u32) {
		match self.max_steps {
			Some(max) => {
				four::setup_gpios(&self.motors);

				four::takedown_gpios(&self.motors);
			},
			None => panic!("Indefinite run not implemented"),
		}
	}

	pub fn add_motor(&mut self, contents: [String; 4]) {
		self.motors.push(contents);
	}

	pub fn add_ir(&mut self, ir: u32) {
		self.ir_sensors.push(ir);
	}

	pub fn set_hz(&mut self, hz: u32) {
		self.steps_per_sec = hz;
	}

	pub fn set_max(&mut self, max: u32) {
		self.max_steps = Some(max)
	}
}

trait LineProcessor {
	fn process_lines(self) -> io::Result<RobotConfig>;
}

impl LineProcessor for io::Result<BufReader<File>> {
	fn process_lines(self) -> io::Result<RobotConfig> {
		match self {
			Ok(r) => {
				let mut config = RobotConfig {motors: Vec::new(), ir_sensors: Vec::new(), steps_per_sec: 100, max_steps: None};
				let result = r.lines().fold(Ok(()), |r, s| process_line(&mut config, s, r));
				match result {Ok(()) => Ok(config), Err(err) => make_error_msg(err.description()),}
			},
			Err(r) => Err(r),
		}
	}
}

fn process_line(config: &mut RobotConfig, line: io::Result<String>, result: io::Result<()>) -> io::Result<()> {
	match line {
		Ok(line) => {
			let mut parts = line.split_whitespace();
			match parts.next() {
				Some(keyword) => process_contents(keyword, &mut parts, config),
				None => Ok(()),
			}
		},
		Err(line) => Err(line),
	}
}

fn process_contents(keyword: &str, contents: &mut SplitWhitespace, config: &mut RobotConfig) -> io::Result<()> {
	match keyword {
		"motor" => process_motor_spec(contents, config),
		"ir" => process_single_num(contents, "Pin", |v| config.add_ir(v)),
		"hz" => process_single_num(contents, "Cycles per second", |v| config.set_hz(v)),
		"max" => process_single_num(contents, "Maximum steps", |v| config.set_max(v)),
		_ => make_error_msg(&format!("Unrecognized keyword: {}", keyword))
	}
}

fn process_single_num<F: FnMut(u32)>(contents: &mut SplitWhitespace, err_msg: &'static str, mut updater: F) -> io::Result<()> {
	match contents.next() {
		Some(input) => {
			match input.parse::<u32>() {
				Ok(value) => {updater(value); Ok(())},
				Err(err) => make_error_msg(err.description()),
			}
		},
		None => make_error_msg(&format!("{} not specified", err_msg)),
	}
}

fn process_motor_spec(contents: &mut SplitWhitespace, config: &mut RobotConfig) -> io::Result<()> {
	match array_from(contents) {
		Ok(contents) => {config.add_motor(contents); Ok(())},
		Err(err) => Err(err),
	}
}

fn array_from(contents: &mut SplitWhitespace) -> io::Result<[String; 4]> {
	let nums: Vec<String> = contents.map(|s| String::from(s)).collect();
	if nums.len() == 4 {
		Ok([nums[0].clone(), nums[1].clone(), nums[2].clone(), nums[3].clone()])
	} else {
		make_error_msg(&format!("Parse error on pin numbers"))
	}
}

pub fn make_error_msg<T>(msg: &str) -> io::Result<T> {
	Err(io::Error::new(ErrorKind::InvalidInput, msg))
}
