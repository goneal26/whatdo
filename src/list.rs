use rand::seq::SliceRandom;
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

    // Collect all unchecked tasks
    let tasks: Vec<_> = reader
      .lines()
      .filter_map(|line| {
        let line = line.ok()?;
        if line.starts_with("- [ ] ") {
          Some(line.trim_start_matches("- [ ] ").to_string())
        } else {
          None
        }
      })
      .collect();

    // Randomly select one unchecked task, if any
    tasks.choose(&mut rand::thread_rng()).cloned()
  }

  // add item to list
  pub fn add(&self, task: &str) {
    let mut file = OpenOptions::new()
      .create(true)
      .append(true)
      .open(&self.path)
      .expect("Unable to open list file");

    writeln!(file, "- [ ] {}", task).expect("Unable to add task");
  }

  // remove item from list
  pub fn remove(&self, task: &str) {
    let file = OpenOptions::new()
      .read(true)
      .open(&self.path)
      .expect("Unable to open list file");
    let reader = BufReader::new(file);

    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();
    let updated_tasks: Vec<_> =
      tasks.into_iter().filter(|t| !t.contains(task)).collect();

    fs::write(&self.path, updated_tasks.join("\n") + "\n")
      .expect("Unable to remove task");
  }

  // mark item as complete
  pub fn check(&self, task: &str) {
    self.modify_task(task, true);
  }

  // unmark item as complete
  pub fn uncheck(&self, task: &str) {
    self.modify_task(task, false);
  }

  // clear items from list
  // if all == true, clear all items,
  // otherwise only clear items marked complete
  pub fn clear(&self, all: bool) {
    let file = OpenOptions::new()
      .read(true)
      .open(&self.path)
      .expect("Unable to open list file");
    let reader = BufReader::new(file);

    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();
    let updated_tasks: Vec<_> = if all {
      vec![]
    } else {
      tasks
        .into_iter()
        .filter(|t| !t.starts_with("- [x] "))
        .collect()
    };

    fs::write(&self.path, updated_tasks.join("\n") + "\n")
      .expect("Unable to clear list");
  }

  // return the list's current file path
  pub fn get_path(&self) -> &PathBuf {
    &self.path
  }

  // change list file path to given path
  // if no file at path, fail with error
  pub fn set_path(&mut self, path: PathBuf) {
    if !path.exists() {
      panic!("No list file found at path {:?}", path);
    }
    self.path = path;
  }

  // Get the list as a string based on checked and unchecked flags
  pub fn get_list_as_string(&self, checked: bool, unchecked: bool) -> String {
    let fileresult = OpenOptions::new().read(true).open(&self.path);

    let file = match fileresult {
      Ok(file) => file,
      Err(_) => return String::from("(List is empty)"),
    };

    let reader = BufReader::new(file);

    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();

    let filtered_tasks: Vec<_> = tasks
      .into_iter()
      .filter(|task| {
        (checked && task.contains("- [x] "))
          || (unchecked && task.contains("- [ ] "))
      })
      .collect();

    filtered_tasks.join("\n")
  }

  // helper function to modify tasks
  fn modify_task(&self, task: &str, mark: bool) {
    let file = OpenOptions::new()
      .read(true)
      .open(&self.path)
      .expect("Unable to open list file");
    let reader = BufReader::new(file);

    let tasks: Vec<_> = reader.lines().collect::<Result<_>>().unwrap();
    let updated_tasks: Vec<_> = tasks
      .into_iter()
      .map(|t| {
        if t.contains(task) {
          if mark {
            format!("- [x] {}", task)
          } else {
            format!("- [ ] {}", task)
          }
        } else {
          t
        }
      })
      .collect();

    fs::write(&self.path, updated_tasks.join("\n") + "\n")
      .expect("Unable to write to list file");
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
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(content.contains("- [ ] testtask"));
  }

  #[test]
  fn test_remove_task() {
    let list = create_test_list();
    let task = "task_to_remove";

    // Add a task, then remove it
    list.add(task);
    list.remove(task);

    // Verify the task is removed from the file
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(!content.contains("- [ ] task_to_remove"));
  }

  #[test]
  fn test_check_task() {
    let list = create_test_list();
    let task = "task_to_check";

    // Add task, then check it
    list.add(task);
    list.check(task);

    // Verify the task is marked as checked
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(content.contains("- [x] task_to_check"));
  }

  #[test]
  fn test_uncheck_task() {
    let list = create_test_list();
    let task = "task_to_uncheck";

    // Add and check the task, then uncheck it
    list.add(task);
    list.check(task);
    list.uncheck(task);

    // Verify the task is unchecked
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(content.contains("- [ ] task_to_uncheck"));
  }

  #[test]
  fn test_clear_all_tasks() {
    let list = create_test_list();
    let task1 = "task1";
    let task2 = "task2";

    // Add tasks, then clear all tasks
    list.add(task1);
    list.add(task2);
    list.clear(true);

    // Verify all tasks are cleared
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");

    // it's okay if there's a little whitespace
    assert!(content.trim().is_empty());
  }

  #[test]
  fn test_clear_completed_tasks() {
    let list = create_test_list();
    let task1 = "task1";
    let task2 = "task2";

    // Add tasks and mark one as completed, then clear completed tasks
    list.add(task1);
    list.add(task2);
    list.check(task1);
    list.clear(false);

    // Verify that only the uncompleted task remains
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(content.contains("- [ ] task2"));
    assert!(!content.contains("- [x] task1"));
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
    assert_eq!(list.get_path(), &new_path);
  }

  #[test]
  fn test_modify_task() {
    let list = create_test_list();
    let task = "to_modify";

    // Add the task, modify it by checking, then modify it again by unchecking
    list.add(task);
    list.modify_task(task, true);
    list.modify_task(task, false);

    // Verify the task is first checked and then unchecked
    let content =
      fs::read_to_string(list.get_path()).expect("Unable to read file");
    assert!(content.contains("- [ ] to_modify"));
  }

  #[test]
  fn test_get_list_as_string_checked_and_unchecked() {
    let list = create_test_list();

    // Add tasks to the list
    list.add("task1");
    list.add("task2");
    list.check("task1");

    // Get all tasks (both checked and unchecked)
    let all_tasks = list.get_list_as_string(true, true);
    let expected_all = "- [x] task1\n- [ ] task2";
    assert_eq!(all_tasks, expected_all);

    // Get only unchecked tasks
    let unchecked_tasks = list.get_list_as_string(false, true);
    let expected_unchecked = "- [ ] task2";
    assert_eq!(unchecked_tasks, expected_unchecked);

    // Get only checked tasks
    let checked_tasks = list.get_list_as_string(true, false);
    let expected_checked = "- [x] task1";
    assert_eq!(checked_tasks, expected_checked);
  }

  #[test]
  fn test_get_list_as_string_only_checked() {
    let list = create_test_list();

    // Add tasks to the list
    list.add("task1");
    list.add("task2");
    list.check("task1");

    // Get only checked tasks
    let checked_tasks = list.get_list_as_string(true, false);
    let expected_checked = "- [x] task1";
    assert_eq!(checked_tasks, expected_checked);
  }

  #[test]
  fn test_get_list_as_string_only_unchecked() {
    let list = create_test_list();

    // Add tasks to the list
    list.add("task1");
    list.add("task2");

    // Get only unchecked tasks
    let unchecked_tasks = list.get_list_as_string(false, true);
    let expected_unchecked = "- [ ] task1\n- [ ] task2";
    assert_eq!(unchecked_tasks, expected_unchecked);
  }

  #[test]
  fn test_get_list_as_string_no_tasks() {
    let list = create_test_list();

    // Get all tasks from an empty list
    let empty_tasks = list.get_list_as_string(true, true);
    assert_eq!(empty_tasks, "(List is empty)");

    // Get unchecked tasks from an empty list
    let empty_unchecked = list.get_list_as_string(false, true);
    assert_eq!(empty_unchecked, "(List is empty)");

    // Get checked tasks from an empty list
    let empty_checked = list.get_list_as_string(true, false);
    assert_eq!(empty_checked, "(List is empty)");
  }

  #[test]
  fn test_get_list_as_string_no_checked_tasks() {
    let list = create_test_list();

    // Add tasks to the list
    list.add("task1");
    list.add("task2");

    // Get only checked tasks (none should be checked)
    let checked_tasks = list.get_list_as_string(true, false);
    assert_eq!(checked_tasks, "");
  }

  #[test]
  fn test_get_list_as_string_no_unchecked_tasks() {
    let list = create_test_list();

    // Add checked tasks to the list
    list.add("task1");
    list.add("task2");
    list.check("task1");
    list.check("task2");

    // Get only unchecked tasks (none should be unchecked)
    let unchecked_tasks = list.get_list_as_string(false, true);
    assert_eq!(unchecked_tasks, "");
  }

  #[test]
  fn pick_random_unchecked_task() {
    let list = create_test_list();

    list.add("task1");
    list.add("task2");
    list.add("task3");
    list.add("task4");

    list.check("task1");
    list.check("task3");

    // Pick a random unchecked task
    let task = list.pick();

    // Assert the result is one of the unchecked tasks
    assert!(matches!(task.as_deref(), Some("task2") | Some("task4")));
  }

  #[test]
  fn pick_with_no_unchecked_tasks() {
    let list = create_test_list();

    list.add("task1");
    list.add("task2");
    list.check("task1");
    list.check("task2");

    // Pick a random unchecked task
    let task = list.pick();

    // Assert the result is None
    assert!(task.is_none());
  }

  #[test]
  fn pick_with_empty_list() {
    let list = create_test_list();
    let task = list.pick();

    assert!(task.is_none());
  }
}
