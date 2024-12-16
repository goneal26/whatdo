#![warn(clippy::pedantic)]
use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
extern crate dirs;

// clap boilerplate
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
  /// Pick and display the name of a random task from the queue.
  Pick,

  /// Add tasks to the queue
  #[command(arg_required_else_help(true))]
  Add {
    /// Space-separated list of tasks to add
    tasks: Vec<String>,
  },

  /// Remove tasks from the queue by name
  #[command(arg_required_else_help(true))]
  Drop {
    /// Space-separated list of tasks to remove
    tasks: Vec<String>,
  },

  /// Return the list of all tasks
  List,

  /// Clear all tasks from the queue
  Clear,

  /// Get the path to the TOML file containing your list/queue
  Path,

  /// Reshuffle the queue
  /// This is done automatically after you've gone through every task
  Shuffle,
}

// data structure for storing tasks/hobbies
mod dolist;
use dolist::DoList;

// get the path to the config file
// return None if not found
fn get_path() -> Option<PathBuf> {
  // should be something like '~/.config/'
  let base_dir = dirs::config_local_dir()?;

  let config_path = base_dir.join("whatdo").join("list.toml");

  // should be something like '~/.config/whatdo/list.toml'
  Some(config_path)
}

// deserialize list data stored in toml file
fn load_dolist() -> Result<DoList, String> {
  let Some(path) = get_path() else {
    return Err("whatdo had error: list file path not found".to_string());
  };

  // fetch file contents into toml string
  let Ok(data) = fs::read_to_string(&path) else {
    return Err("whatdo had error: failed to read list file".to_string());
  };

  let Ok(list) = toml::from_str(data.as_ref()) else {
    return Err(
      "whatdo had error: failed to deserialize list data".to_string(),
    );
  };

  Ok(list)
}

// serialize list data and write to toml file
fn store_dolist(list: &DoList) -> Result<(), String> {
  let Ok(data) = toml::to_string_pretty(list) else {
    return Err("whatdo had error: failed to serialize list data".to_string());
  };

  let Some(path) = get_path() else {
    return Err("whatdo had error: list file path not found".to_string());
  };

  // create config file if does not exist
  if let Some(parent) = path.parent() {
    if fs::create_dir_all(parent).is_err() {
      return Err(
        "whatdo had error: failed to create list parent directory".to_string(),
      );
    }
  }

  // write to file
  match fs::write(&path, data) {
    Ok(()) => Ok(()),
    Err(_) => Err("whatdo had error: failed to write list to file".to_string()),
  }
}

fn main() {
  let args = Cli::parse();

  // load list from file
  let mut list = match load_dolist() {
    Ok(list) => list,
    Err(error) => {
      eprintln!("{error}\nCreating new file...");
      DoList::new()
    }
  };

  match args.command {
    None | Some(Commands::Pick) => {
      let picked = list.pick();
      match picked {
        Some(task) => println!("{task}"),
        None => eprintln!("whatdo had error: list is empty"),
      };
    }
    Some(Commands::Add { tasks }) => {
      for task in &tasks {
        match list.add(task.to_string()) {
          Ok(()) => {}
          Err(error) => eprintln!("{error}"),
        }
      }
    }
    Some(Commands::Drop { tasks }) => {
      for task in &tasks {
        match list.drop(task) {
          Ok(removed_task) => println!("whatdo: removed \"{removed_task}\""),
          Err(error) => eprintln!("{error}"),
        }
      }
    }
    Some(Commands::List) => println!("{list}"),
    Some(Commands::Clear) => {
      list.clear();
      println!("whatdo: cleared all items");
    }
    Some(Commands::Path) => {
      match get_path() {
        Some(path) => println!("{}", path.display()),
        None => eprintln!("whatdo had error: list file path not found"),
      };
    }
    Some(Commands::Shuffle) => {
      list.shuffle();
      println!("whatdo: reshuffled queue");
    }
  };

  match store_dolist(&list) {
    Ok(()) => {}
    Err(msg) => panic!("{msg}"),
  }
}
