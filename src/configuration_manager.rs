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

#[cfg(test)]
mod tests {
	use serial_test::serial;
	use std::{fs, path::Path};
	use super::*;
	use crate::dependency::{Dependency, UpdateType};

	fn get_configuration_with_one_dependency() -> Configuration {
		Configuration {
			dependencies: vec![
				Dependency {
					name: String::from("test"),
					uri: String::from("./src/configuration_manager.rs"),
					path: String::from("test.rs"),
					local: true,
					update: UpdateType::OnChange
				}
			]
		}
	}

	fn get_configuration_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
		let mut file_path = env::current_dir()?;

		file_path.push(PathBuf::from(CONFIGURATION_FILE_NAME));

		Ok(file_path)
	}

	fn create_configuration_file() -> Result<(), Box<dyn std::error::Error>> {
		let configuration = get_configuration_with_one_dependency();
		let configuration_file_path = get_configuration_file_path()?;

		std::fs::write(configuration_file_path, serde_json::to_string_pretty(&configuration).unwrap())?;

    	Ok(())
	}

	fn delete_configuration_file() -> Result<(), Box<dyn std::error::Error>> {
		let configuration_file_path = get_configuration_file_path()?;

		fs::remove_file(configuration_file_path)?;

		Ok(())
	}

	#[test]
	#[serial]
	fn returns_default_configuration_if_file_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
		let configuration = open_configuration_file()?;

		assert_eq!(0, configuration.dependencies.len());

		Ok(())
	}

	#[test]
	#[serial]
	fn returns_configuration_from_file() -> Result<(), Box<dyn std::error::Error>> {
		create_configuration_file()?;

		let configuration = open_configuration_file()?;

		assert_eq!(1, configuration.dependencies.len());
		assert_eq!("test", &configuration.dependencies[0].name);

		delete_configuration_file()?;
		Ok(())
	}

	#[test]
	#[serial]
	fn creates_file_during_update_if_it_does_not_exist() -> Result<(), Box<dyn std::error::Error>> {
		let configuration = get_configuration_with_one_dependency();
		let configuration_file_path = get_configuration_file_path()?;

		update_configuration_file(configuration)?;

		assert!(Path::new(&configuration_file_path).exists());

		delete_configuration_file()?;
		Ok(())
	}

	#[test]
	#[serial]
	fn updates_configuration_from_file() -> Result<(), Box<dyn std::error::Error>> {
		create_configuration_file()?;

		let mut configuration = open_configuration_file()?;

		configuration.dependencies[0].name = String::from("updated");

		update_configuration_file(configuration)?;

		let updated_configuration = open_configuration_file()?;

		assert_eq!("updated", &updated_configuration.dependencies[0].name);

		delete_configuration_file()?;
		Ok(())
	}
}