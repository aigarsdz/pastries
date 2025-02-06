use std::env;
use std::io::BufReader;
use std::path::PathBuf;
use std::fs::File;
use crate::dependency::Configuration;

const CONFIGURATION_FILE_NAME: &str = "pastries.json";

pub fn open_configuration_file() -> Result<Configuration, Box<dyn std::error::Error>> {
	let mut current_directory = env::current_dir()?;

	current_directory.push(PathBuf::from(CONFIGURATION_FILE_NAME));

	let mut configuration = Configuration {
		dependencies: Vec::new()
	};

	if let Ok(file) = File::open(current_directory) {
		let reader = BufReader::new(file);

		configuration = serde_json::from_reader(reader)?;
	}

    Ok(configuration)
}

pub fn update_configuration_file(configuration: Configuration) -> Result<(), Box<dyn std::error::Error>> {
	let mut current_directory = env::current_dir()?;

	current_directory.push(PathBuf::from(CONFIGURATION_FILE_NAME));

	std::fs::write(current_directory, serde_json::to_string_pretty(&configuration).unwrap())?;

    Ok(())
}