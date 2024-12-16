use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

// list error handling
#[derive(Debug)]
pub struct DoListErr {
  err: String,
}

impl fmt::Display for DoListErr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "whatdo had error: {}", self.err)
  }
}

impl Error for DoListErr {}

// actual list type
#[derive(Serialize, Deserialize)]
pub struct DoList {
  list: Vec<String>,
  queue: VecDeque<String>,
}

impl DoList {
  // create new dolist
  pub fn new() -> Self {
    Self {
      list: Vec::new(),
      queue: VecDeque::new(),
    }
  }

  // add item to dolist
  pub fn add(&mut self, task: String) -> Result<(), DoListErr> {
    if self.list.contains(&task) {
      return Err(DoListErr {
        err: format!("item \"{task}\" already on the list"),
      });
    }

    self.list.push(task.clone());
    self.queue.push_back(task);
    Ok(())
  }

  // drop item from dolist and return it
  pub fn drop(&mut self, task: &str) -> Result<String, DoListErr> {
    if let Some(index) = self.list.iter().position(|s| *s == task) {
      let removed = self.list.remove(index);
      Ok(removed)
    } else {
      Err(DoListErr {
        err: format!("item \"{task}\" not removed (item not found)"),
      })
    }
  }

  // reshuffle dolist
  pub fn shuffle(&mut self) {
    // make copy of the hashset keys as a vec
    let mut copy: Vec<String> = Vec::from_iter((self.list).clone());

    // shuffle the slice
    let mut rng = thread_rng();
    copy.shuffle(&mut rng);

    // empty the current queue
    self.queue.clear();

    // add shuffled items to queue
    for item in &copy {
      self.queue.push_back(item.clone().clone());
    }
  }

  // Pick and return a random task from the list
  // if the list is empty, return None
  pub fn pick(&mut self) -> Option<String> {
    if self.list.is_empty() {
      return None;
    }

    if self.queue.is_empty() {
      self.shuffle();
    }

    self.queue.pop_front()
  }

  // clear all items from list
  pub fn clear(&mut self) {
    self.list.clear();
    self.queue.clear();
  }
}

// printing a DoList
impl fmt::Display for DoList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let list_string = self.list.clone().join("\n");
    write!(f, "{list_string}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add_and_pick() {
    let mut do_list = DoList::new();
    do_list.add("Task1".to_string()).unwrap();
    do_list.add("Task2".to_string()).unwrap();

    let first_pick = do_list.pick();
    let second_pick = do_list.pick();

    assert!(first_pick.is_some());
    assert!(second_pick.is_some());
    assert_ne!(first_pick, second_pick); // no duplicates due to queue
  }

  #[test]
  fn test_add_duplicates() {
    let mut do_list = DoList::new();
    assert!(do_list.add("Task1".to_string()).is_ok());
    let result = do_list.add("Task1".to_string());
    assert!(result.is_err());
    assert_eq!(
      format!("{}", result.unwrap_err()),
      "whatdo had error: item \"Task1\" already on the list"
    );
  }

  #[test]
  fn test_drop() {
    let mut do_list = DoList::new();
    do_list.add("Task1".to_string()).unwrap();
    let removed = do_list.drop("Task1").unwrap();
    assert_eq!(removed, "Task1".to_string());
  }

  #[test]
  fn test_shuffle_behavior() {
    let mut do_list = DoList::new();
    do_list.add("Task1".to_string()).unwrap();
    do_list.add("Task2".to_string()).unwrap();
    do_list.add("Task3".to_string()).unwrap();

    let mut picked = Vec::new();
    for _ in 0..3 {
      let task = do_list.pick().unwrap();
      assert!(!picked.contains(&task));
      picked.push(task);
    }

    // List has been fully traversed; now reshuffled
    assert!(do_list.pick().is_some());
  }

  #[test]
  fn test_clear() {
    let mut do_list = DoList::new();
    do_list.add("Task1".to_string()).unwrap();
    do_list.add("Task2".to_string()).unwrap();
    do_list.clear();

    assert!(do_list.pick().is_none());
  }

  #[test]
  fn test_pick_empty() {
    let mut do_list = DoList::new();
    assert!(do_list.pick().is_none());
  }

  #[test]
  fn test_display_format() {
    let mut do_list = DoList::new();
    do_list.add("Task1".to_string()).unwrap();
    do_list.add("Task2".to_string()).unwrap();
    do_list.add("Task3".to_string()).unwrap();

    // Use the `to_string` method to test the Display implementation
    let formatted = do_list.to_string();

    // HashSet does not guarantee order, so we need to verify the content
    let expected = "Task1\nTask2\nTask3";
    assert_eq!(
      formatted.lines().collect::<Vec<_>>(),
      expected.lines().collect::<Vec<_>>()
    );
  }
}
