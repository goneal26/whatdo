use rand::seq::SliceRandom;
use std::fmt;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};
use std::path::PathBuf;

pub struct List {
  path: PathBuf,
}

impl List {
  // create new list file at path
  pub fn new(path: PathBuf) -> Self {
    Self { path }
  }

  // Pick and return a random unchecked task from the list, or None
  pub fn pick(&self) -> Option<String> {
    let fileres = OpenOptions::new().read(true).open(&self.path);

    let file = match fileres {
      Ok(file) => file,
      Err(_) => return None,
    };

    let reader = BufReader::new(file);

    // Collect all tasks
    let tasks: Vec<_> = reader
      .lines()
      .filter_map(|line| {
        let line = line.ok()?;
        if !line.trim().is_empty() {
          Some(line.trim().to_string())
        } else {
          None
        }
      })
      .collect();

    // Randomly select one task, if any
    tasks.choose(&mut rand::thread_rng()).cloned()
  }

  // add item to list
  pub fn add(&self, task: &str) {
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&self.path)
      .expect("(Unable to open list file)");

    writeln!(file, "{}", task).expect("(Unable to add task)");
  }

  // remove item from list
  pub fn remove(&self, task: &str) {
    let file = OpenOptions::new()
      .read(true)
      .open(&self.path)
      .expect("(Unable to open list file)");
    let reader = BufReader::new(file);

    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();
    let updated_tasks: Vec<_> =
      tasks.into_iter().filter(|t| !t.contains(task)).collect();

    fs::write(&self.path, updated_tasks.join("\n") + "\n")
      .expect("(Unable to remove task)");
  }

  // clear all items from list
  pub fn clear(&self) {
    fs::write(&self.path, "\n").expect("(Unable to clear list)");
  }

  // return the list's current file path
  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  // change list file path to given path
  // if no file at path, fail with error
  pub fn set_path(&mut self, path: PathBuf) {
    if !path.exists() {
      panic!("(No list file found at path {:?})", path);
    }
    self.path = path;
  }
}

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let fileresult = OpenOptions::new().read(true).open(&self.path);
    let file = match fileresult {
      Ok(file) => file,
      Err(_) => return write!(f, "(List is empty)"),
    };
    let reader = BufReader::new(file);
    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();
    let s = tasks.join("\n");

    write!(f, "{}", s)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Helper function to create a temporary List instance
  fn create_test_list() -> List {
    let t = tempfile::NamedTempFile::new().expect("Failed to create temp file");
    List::new(t.path().to_path_buf())
  }

  #[test]
  fn test_add_task() {
    let list = create_test_list();
    let task = "testtask";

    // Add task to the list
    list.add(task);

    // Verify the task is added to the file
    let content = fs::read_to_string(list.path()).expect("Unable to read file");
    assert!(content.contains("testtask"));
  }

  #[test]
  fn test_remove_task() {
    let list = create_test_list();
    let task = "task_to_remove";

    // Add a task, then remove it
    list.add(task);
    list.remove(task);

    // Verify the task is removed from the file
    let content = fs::read_to_string(list.path()).expect("Unable to read file");
    assert!(!content.contains("task_to_remove"));
  }

  #[test]
  fn test_clear_all_tasks() {
    let list = create_test_list();
    let task1 = "task1";
    let task2 = "task2";

    // Add tasks, then clear all tasks
    list.add(task1);
    list.add(task2);
    list.clear();

    // Verify all tasks are cleared
    let content = fs::read_to_string(list.path()).expect("Unable to read file");

    // it's okay if there's a little whitespace
    assert!(content.trim().is_empty());
  }

  #[test]
  fn test_clear_already_empty() {
    let list = create_test_list();

    list.clear();

    // Verify all tasks are cleared
    let content = fs::read_to_string(list.path()).expect("Unable to read file");

    // it's okay if there's a little whitespace
    assert!(content.trim().is_empty());
  }

  #[test]
  fn test_set_path() {
    let mut list = create_test_list();
    let new_temp_file =
      tempfile::NamedTempFile::new().expect("Failed to create new temp file");
    let new_path = new_temp_file.path().to_path_buf();

    // Change the path of the list
    list.set_path(new_path.clone());

    // Verify the path has been updated
    assert_eq!(list.path(), &new_path);
  }

  #[test]
  fn test_to_string() {
    let list = create_test_list();

    // Add tasks to the list
    list.add("task1");
    list.add("task2");

    // Get all tasks
    let all_tasks = list.to_string();
    let expected_all = "task1\ntask2";
    assert_eq!(all_tasks, expected_all);
  }

  #[test]
  fn test_to_string_when_empty() {
    let list = create_test_list();

    // Get all tasks
    let all_tasks = list.to_string();

    assert_eq!(all_tasks.trim(), String::from("(List is empty)"));
  }

  #[test]
  fn pick_task() {
    let list = create_test_list();

    list.add("task1");
    list.add("task2");
    list.add("task3");

    // Pick a random unchecked task
    let task = list.pick();

    // Assert the result is one of the unchecked tasks
    assert!(matches!(
      task.as_deref(),
      Some("task1") | Some("task2") | Some("task3")
    ));
  }

  #[test]
  fn pick_with_empty_list() {
    let list = create_test_list();
    let task = list.pick();

    assert!(task.is_none());
  }
}
