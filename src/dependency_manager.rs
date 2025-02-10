use std::io::Write;
use std::fs;
use std::fs::File;
use std::path::Path;
use reqwest;
use file_diff::diff;
use crate::dependency::{Dependency, UpdateType};

pub enum UpdateResult {
    Updated,
    Failed,
    Ignored
}

pub enum AddResult {
    Added,
    Failed(String)
}

async fn add_remote_dependency(uri: &str, path: &str) -> Result<(), reqwest::Error> {
	let response = reqwest::get(uri).await?;
	let file_content = response.bytes().await?;
	let mut file = File::create(path).unwrap();

	file.write_all(&file_content).unwrap();

	Ok(())
}

fn add_local_dependency(uri: &str, path: &str) -> std::io::Result<()> {
	let target_file_path = Path::new(path);
	let target_directory_path = target_file_path.parent().unwrap();

	fs::create_dir_all(target_directory_path)?;
	fs::copy(uri, path)?;

    Ok(())
}

pub async fn add_dependency(uri: &str, path: &str, local: bool) -> AddResult {
    if local {
    	return match add_local_dependency(uri, path) {
    		Ok(_) => AddResult::Added,
    		Err(message) => AddResult::Failed(message.to_string())
    	};
    }

    match add_remote_dependency(uri, path).await {
    	Ok(_) => AddResult::Added,
    	Err(message) => AddResult::Failed(message.to_string())
    }
}

fn update_local_dependency(dependency: &Dependency) -> UpdateResult {
	let mut update_required = false;

	if let UpdateType::Always = dependency.update {
		update_required = true;
	} else if !diff(&dependency.uri, &dependency.path) {
		update_required = true;
	}

	if update_required {
		if let Ok(_) = add_local_dependency(&dependency.uri, &dependency.path) {
			return UpdateResult::Updated;
		} else {
			return UpdateResult::Failed;
		}
	}

	UpdateResult::Ignored
}

fn overwrite_dependency_file(temporary_file_path: &str, dependency_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
	fs::copy(&temporary_file_path, &dependency_file_path)?;
	fs::remove_file(&temporary_file_path)?;

	Ok(())
}

async fn update_remote_dependency(dependency: &Dependency) -> UpdateResult {
	let temporary_path = format!("{}.tmp", dependency.path);
	let mut update_required = false;

	if let Ok(_) = add_remote_dependency(&dependency.uri, &temporary_path).await {
		if let UpdateType::Always = dependency.update {
			update_required = true;
		} else if !diff(&temporary_path, &dependency.path) {
			update_required = true;
		}
	} else {
		return UpdateResult::Failed;
	}

	if update_required {
		if let Ok(_) = overwrite_dependency_file(&temporary_path, &dependency.path) {
			return UpdateResult::Updated;
		} else {
			return UpdateResult::Failed;
		}
	} else {
		fs::remove_file(&temporary_path).expect("Failed to remove the temporary file.");
	}

	UpdateResult::Ignored
}

pub async fn update_dependency(dependency: &Dependency) -> UpdateResult {
	if let UpdateType::Never = dependency.update {
		return UpdateResult::Ignored;
	}

	if dependency.local {
		return update_local_dependency(dependency);
	}

	update_remote_dependency(dependency).await
}

pub fn remove_dependency(dependency: &Dependency) -> Result<(), Box<dyn std::error::Error>> {
	fs::remove_file(&dependency.path)?;

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn adds_local_dependency() {
		let _result = add_dependency("./src/dependency_manager.rs", "./local_test.rs", true).await;

		assert!(matches!(AddResult::Added, _result));
		assert!(Path::new("./local_test.rs").exists());

		fs::remove_file("./local_test.rs").expect("Could not delete the test file");
	}

	#[tokio::test]
	async fn adds_remote_dependency() {
		let _result = add_dependency("https://www.example.com", "./remote_test.html", false).await;

		assert!(matches!(AddResult::Added, _result));
		assert!(Path::new("./remote_test.html").exists());

		fs::remove_file("./remote_test.html").expect("Could not delete the test file");
	}

	#[tokio::test]
	async fn updates_local_dependency() {
		File::create("./local_test_2.rs").expect("Failed to create a test file");

		let dependency = Dependency {
			name: String::from("local_test_2"),
			uri: String::from("./src/dependency_manager.rs"),
			path: String::from("./local_test_2.rs"),
			local: true,
			update: UpdateType::OnChange
		};
		let _result = update_dependency(&dependency).await;
		let file_content = fs::read_to_string("./local_test_2.rs").expect("Could not read the test file");

		assert!(matches!(UpdateResult::Updated, _result));
		assert!(file_content.len() > 0);

		fs::remove_file("./local_test_2.rs").expect("Could not delete the test file");
	}

	#[tokio::test]
	async fn updates_remote_dependency() {
		File::create("./remote_test_2.html").expect("Failed to create a test file");

		let dependency = Dependency {
			name: String::from("remote_test_2"),
			uri: String::from("https://www.example.com"),
			path: String::from("./remote_test_2.html"),
			local: false,
			update: UpdateType::OnChange
		};
		let _result = update_dependency(&dependency).await;
		let file_content = fs::read_to_string("./remote_test_2.html").expect("Could not read the test file");

		assert!(matches!(UpdateResult::Updated, _result));
		assert!(file_content.len() > 0);

		fs::remove_file("./remote_test_2.html").expect("Could not delete the test file");
	}

	#[test]
	fn removes_dependency() {
		File::create("./local_test_3.rs").expect("Failed to create a test file");

		let dependency = Dependency {
			name: String::from("local_test_3"),
			uri: String::from("./src/dependency_manager.rs"),
			path: String::from("./local_test_3.rs"),
			local: true,
			update: UpdateType::OnChange
		};

		remove_dependency(&dependency).expect("Failed to remove the dependency");

		assert!(!Path::new(&dependency.path).exists());
	}
}