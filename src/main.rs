use clap::{Parser, Subcommand};
use std::env;
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
  /// (TODO NOT IMPLEMENTED YET)
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

  /// Change the path of the global list file (TODO NOT IMPLEMENTED YET)
  #[command(arg_required_else_help(true))]
  SetConfig {
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
  path.set_extension("md"); // TODO changing from default path?

  let mut list = List::new(path);

  match args.command {
    None | Some(Commands::Pick) => {
      let picked = list.pick();
      match picked {
        Some(task) => println!("Picked: {}", task),
        None => println!("No unfinished tasks to pick!"),
      };
    }
    Some(Commands::Add { tasks }) => {
      for task in tasks.iter() {
        list.add(task);
      }
    }
    Some(Commands::Remove { tasks }) => {
      for task in tasks.iter() {
        list.remove(task);
      }
    }
    Some(Commands::List { checked, unchecked }) => {
      if !checked && !unchecked {
        // i.e. no flags passed
        println!("{}", list.get_list_as_string(true, true)); // display full list
      } else {
        // filter for only checked/unchecked items
        println!("{}", list.get_list_as_string(checked, unchecked));
      }
    }
    Some(Commands::Check { tasks }) => {
      for task in tasks.iter() {
        list.check(task);
      }
    }
    Some(Commands::Uncheck { tasks }) => {
      for task in tasks.iter() {
        list.uncheck(task);
      }
    }
    Some(Commands::Clear { all }) => list.clear(all),
    Some(Commands::Config) => {
      println!("{}", list.get_path().display());
    }
    Some(Commands::SetConfig { path }) => list.set_path(path),
  };

  Ok(())
}
