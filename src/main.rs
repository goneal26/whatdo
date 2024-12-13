use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
  version,
  about = "The TODO list CLI for when you can't decide what to do next."
)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Option<Commands>,
}

// possible subcommands
#[derive(Subcommand, Debug, Clone)]
enum Commands {
  /// Pick and display the name of a random task from the list.
  /// If no command is provided, this is the default behavior.
  Pick,

  /// Add tasks to the list
  #[command(arg_required_else_help(true))]
  Add {
    /// Space-separated list of tasks to add
    tasks: Vec<String>,
  },

  /// Remove tasks from the list
  #[command(arg_required_else_help(true))]
  Remove {
    /// Space-separated list of tasks to remove
    tasks: Vec<String>,
  },

  /// Get list of all tasks
  List {
    /// Filter for checked tasks
    #[clap(long, short, action)]
    checked: bool,

    /// Filter for unchecked tasks
    #[clap(long, short, action)]
    unchecked: bool,

    /// Display tasks as markdown list
    #[clap(long, short, action)]
    markdown: bool,
  },

  /// Mark tasks as complete
  #[command(arg_required_else_help(true))]
  Check {
    /// Space-separated list of tasks to mark as complete
    tasks: Vec<String>,
  },

  /// Unmark tasks as complete
  #[command(arg_required_else_help(true))]
  Uncheck {
    /// Space-separated list of tasks to unmark as complete
    tasks: Vec<String>,
  },

  /// Remove checked tasks from the list
  Clear {
    /// Remove ALL tasks from the list (checked AND unchecked)
    #[clap(long, action)]
    all: bool,
  },

  /// Get the path of the global list file
  Config, // TODO maybe change the names of these

  /// Change the path of the global list file
  #[command(arg_required_else_help(true))]
  SetConfig {
    /// Path to new global list file
    path: PathBuf,
  },
}

fn main() {
  let args = Cli::parse();

  match args.command {
    None => println!("picked task: ___"),
    Some(c) => println!("Subcommand: {:?}", c),
  }
}
