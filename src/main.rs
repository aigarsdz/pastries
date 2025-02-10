mod configuration_manager;
mod dependency_manager;
mod command;
mod dependency;
mod table;

use clap::Parser;
use command::{Arguments, Command};
use colored::Colorize;
use dependency_manager::{AddResult, UpdateResult};

#[tokio::main]
async fn main() {
    let arguments = Arguments::parse();
    let mut configuration = configuration_manager::open_configuration_file()
        .expect("The current directory does not contain a pastries.json file");

    match arguments.command {
        Command::Add { name, uri, path, local, update } => {
            match dependency_manager::add_dependency(&uri, &path, local).await {
                AddResult::Added => {
                    configuration.add_dependency(&name, &uri, &path, &local, &update);
                    configuration_manager::update_configuration_file(configuration).expect("Failed to save the configuration file");

                    println!("\n{} {} {} {} from {}\n", "[Added]".green(), name, "to".bold(), path.italic(), uri.italic());
                },
                AddResult::Failed(error) => {
                    println!("\n{} {} {} {} from {}\n\n{}\n", "[Failed]".red(), name, "to".bold(), path.italic(), uri.italic(), error);
                }
            }
        },
        Command::Update { name } => {
            println!("");

            for dependency in configuration.dependencies {
                if name != "all" && dependency.name != name {
                    continue;
                }

                let result = dependency_manager::update_dependency(&dependency).await;

                match result {
                    UpdateResult::Updated => println!("{} {} {}", "[Updated]".green(), dependency.name, dependency.path.italic()),
                    UpdateResult::Failed => println!("{} {} {}", "[Failed]".red(), dependency.name, dependency.path.italic()),
                    UpdateResult::Ignored => println!("{} {} {}", "[Ignored]".cyan(), dependency.name, dependency.path.italic())
                };
            }

            println!("");
        },
        Command::Remove { name } => {
            if let Some(dependency) = configuration.remove_dependency(&name) {
                let error_message = format!("Failed to remove {}", name);

                dependency_manager::remove_dependency(&dependency).expect(&error_message);
                configuration_manager::update_configuration_file(configuration).expect("Failed to save the configuration file");

                println!("\n{} {} {}\n", "[Removed]".green(), name, dependency.path.italic());
            } else {
                println!("\nA dependency with a name {} does not exist.\n", name);
            }
        },
        Command::List => {
            let rows = configuration.dependencies.iter().map(
                |dependency| vec![dependency.name.as_str(), dependency.path.as_str(), dependency.uri.as_str()]
            ).collect();

            table::draw(vec!["Name", "File path", "Source"], rows);
        }
    }
}
