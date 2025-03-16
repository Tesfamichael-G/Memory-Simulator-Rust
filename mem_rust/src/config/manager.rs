

use std::fs::File;
use std::{env, io};
use std::error::Error;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::data::Config;

use simulation_interface::trace::TraceType;


pub fn load_config(config_path: &str)  -> io::Result<Config>{

    // let config_dir = Path::new(config_path).parent().unwrap();
    let current_dir = env::current_dir()?;
    println!("Current working directory: {}", current_dir.display());

    // Print the path of the file being read
    println!("Config path provided: {}", config_path);

    // Check if the file exists
    let absolute_path = Path::new(config_path).canonicalize();
    match absolute_path {
        Ok(path) => println!("Resolved absolute path: {}", path.display()),
        Err(_) => println!("Error: Could not resolve absolute path."),
    }

    let mut file = File::open(config_path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let config: Config = serde_json::from_str(&data)?;

    // Handle case-insensitive and typo-tolerant interpretation
    let trace_type_str = config.trace_type.to_lowercase();
    let trace_type = match trace_type_str.as_str() {
        "memory" | "mem" => TraceType::Memory,
        "cpu" | "processor" => TraceType::CPU,
        _ => TraceType::Memory,
    };

    Ok(config)

}
