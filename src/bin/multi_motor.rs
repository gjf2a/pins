extern crate pins;
use std::env;
use pins::four;

fn main() {
        let config_arg = env::args().nth(1);
        let rate_arg = env::args().nth(2);
        let n_arg = env::args().nth(3);
        if let Some(config) = config_arg {
                // open the file
                // look for config info
                // 
                if let Some(rate) = rate_arg {
                        if let Some(num_steps) = n_arg {
                                        
                                std::process::exit(0);
                        }
                }
        }
        println!("Usage: multi_motor config_file steps_per_sec num_steps");
}
