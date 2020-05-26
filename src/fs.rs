use std::fs;

use crate::config;
use crate::cli::is_init;

pub fn init(is_standalone: bool) {
    let init = is_init("benl");
    match init {
        Ok(()) => {
            if is_standalone {
                println!("Creating eph data directory in: {}", config::EPH_DATA_DIR);
                match fs::create_dir(config::EPH_DATA_DIR) {
                    Ok(dir) => dir,
                    Err(_error) => {
                        println!("Looks like you already have a data directory.")
                    }
                };
            }
        }
        Err(e) => println!("error parsing header: {:?}", e),
    }
}