mod configuration_manager;
mod dependency_manager;
mod command;
mod dependency;
mod table;

use clap::Parser;
use command::{Arguments, Command};
use dependency_manager::UpdateResult;

#[tokio::main]
async fn main() {
    let arguments = Arguments::parse();
    let mut configuration = configuration_manager::open_configuration_file()
        .expect("The current directory does not contain a pastries.json file");

    match arguments.command {
        Command::Add { name, uri, path, local, update } => {
            if local {
                let error_message = format!("Failed to copy file {}", uri);

                dependency_manager::install_local_dependency(&uri, &path).expect(&error_message);
            } else {
                let error_message = format!("Failed to download file {}", uri);

                dependency_manager::install_remote_dependency(&uri, &path).await.expect(&error_message);
            }

            configuration.add_dependency(&name, &uri, &path, &local, &update);
            configuration_manager::update_configuration_file(configuration).expect("Failed to save the configuration file");

            println!("Added\n  Source: {}\n  File: {}", uri, path);
        },
        Command::Update { name } => {
            for dependency in configuration.dependencies {
                if name != "all" && dependency.name != name {
                    continue;
                }

                let result = dependency_manager::update_dependency(&dependency).await;

                match result {
                    UpdateResult::Updated => println!("Updated\n  Source: {}\n  File: {}", dependency.uri, dependency.path),
                    UpdateResult::Failed => println!("Failed\n  Source: {}\n  File: {}", dependency.uri, dependency.path),
                    UpdateResult::Ignored => ()
                };
            }
        },
        Command::Remove { name } => {
            if let Some(dependency) = configuration.remove_dependency(&name) {
                let error_message = format!("Failed to remove {}", name);

                dependency_manager::remove_dependency(&dependency).expect(&error_message);
                configuration_manager::update_configuration_file(configuration).expect("Failed to save the configuration file");

                println!("Removed dependency {}\nDeleted file {}", name, dependency.path);
            } else {
                println!("A dependency with a name {} does not exist.", name);
            }
        },
        Command::List => {
            let rows = configuration.dependencies.iter().map(|dependency| vec![dependency.name.as_str(), dependency.path.as_str()]).collect();

            table::draw(vec!["Name", "File path"], rows);
        }
    }
}
