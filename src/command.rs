use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Command {
    #[command(arg_required_else_help = true)]
    Add {
        #[arg(long)]
        name: String,

        #[arg(long)]
        uri: String,

        #[arg(long)]
        path: String,

        #[arg(long)]
        local: bool,

        #[arg(long, default_value="on_change")]
        update: String
    },
    Update {
        #[arg(default_value="all")]
        name: String
    },
    #[command(arg_required_else_help = true)]
    Remove {
        name: String
    },
    List
}

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command
}