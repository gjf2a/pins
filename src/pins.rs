use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::io;
use std::error::Error;
use config_robot;

pub fn write_control(prefix: &str, filename: &str, value: &str) {
	let full_path = format!("{}/{}", prefix, filename);

	let mut file = match File::create(&full_path) {
		Err(why) => panic!("Couldn't create {}: {}", full_path, why),
		Ok(file) => file,
	};

	match file.write_all(value.as_bytes()) {
		Err(why) => panic!("Couldn't write to {}: {}", full_path, why),
		Ok(_) => {},
	}
}

pub fn setup_gpio(gpio_num: &str) {
	write_control("/sys/class/gpio", "export", &gpio_num);
}

pub fn takedown_gpio(gpio_num: &str) {
	write_control("/sys/class/gpio", "unexport", &gpio_num);
}

pub fn gpio_cmd(gpio_num: &str, gpio_cmd: &str, gpio_value: &str) {
	let gpio_path = format!("/sys/class/gpio/gpio{}", gpio_num);
	write_control(&gpio_path, gpio_cmd, gpio_value);
}

pub fn read_adc_voltage(ain_num: u32) -> io::Result<u32> {
  let adc_filename = format!("/sys/bus/iio/devices/iio:device{}", ain_num);
  match File::open(&Path::new(&adc_filename)) {
    Ok(mut adc_file) => {
      let mut content = String::new();
      match adc_file.read_to_string(&mut content) {
        Ok(_) => {
          match content.trim().parse::<u32>() {
            Ok(parsed) => Ok(parsed),
            Err(err) => config_robot::make_error_msg(err.description()),
          }
        },
        Err(err) => Err(err),
      }
    },
    Err(err) => Err(err)
  }
}
