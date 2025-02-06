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

pub async fn install_remote_dependency(uri: &str, path: &str) -> Result<(), reqwest::Error> {
	let response = reqwest::get(uri).await?;
	let file_content = response.bytes().await?;
	let mut file = File::create(path).unwrap();

	file.write_all(&file_content).unwrap();

	Ok(())
}

pub fn install_local_dependency(uri: &str, path: &str) -> std::io::Result<()> {
	let target_file_path = Path::new(path);
	let target_directory_path = target_file_path.parent().unwrap();

	fs::create_dir_all(target_directory_path)?;
	fs::copy(uri, path)?;

    Ok(())
}

fn update_local_dependency(dependency: &Dependency) -> UpdateResult {
	let mut update_required = false;

	if let UpdateType::Always = dependency.update {
		update_required = true;
	} else if !diff(&dependency.uri, &dependency.path) {
		update_required = true;
	}

	if update_required {
		if let Ok(_) = install_local_dependency(&dependency.uri, &dependency.path) {
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

	if let Ok(_) = install_remote_dependency(&dependency.uri, &temporary_path).await {
		if let UpdateType::Always = dependency.update {
			update_required = true;
		} else if diff(&temporary_path, &dependency.path) {
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