use clap::{Parser, Subcommand};
use std::env;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
  version,
  about = "The CLI for when you can't decide what to do next."
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
  Drop {
    /// Space-separated list of tasks to remove
    tasks: Vec<String>,
  },

  /// Get list of all tasks
  List,

  /// Remove checked tasks from the list
  Clear,

  /// Get the path of the global list file
  Path,

  /// Change the path of the global list file (BROKEN)
  #[command(arg_required_else_help(true))]
  SetPath {
    /// Path to new global list file
    path: PathBuf,
  },
}

mod list;
use list::List;

fn main() -> std::io::Result<()> {
  let args = Cli::parse();

  let mut path = env::current_dir()?;
  path.push("todo");
  path.set_extension("txt"); // TODO changing from default path?

  let mut list = List::new(path);

  match args.command {
    None | Some(Commands::Pick) => {
      let picked = list.pick();
      match picked {
        Some(task) => println!("{}", task),
        None => println!("(No tasks to pick)"),
      };
    }
    Some(Commands::Add { tasks }) => {
      for task in tasks.iter() {
        list.add(task);
      }
    }
    Some(Commands::Drop { tasks }) => {
      for task in tasks.iter() {
        list.drop(task);
      }
    }
    Some(Commands::List) => println!("{}", list),
    Some(Commands::Clear) => list.clear(),
    Some(Commands::Path) => {
      println!("{}", list.path().display());
    }
    Some(Commands::SetPath { path }) => list.set_path(path),
  };

  Ok(())
}
