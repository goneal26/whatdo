use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashSet, VecDeque};
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
pub struct DoList {
  list: HashSet<String>,
  shuffle_queue: VecDeque<String>,
}

impl DoList {
  // create new dolist
  pub fn new() -> Self {
    Self {
      list: HashSet::new(),
      shuffle_queue: VecDeque::new(),
    }
  }

  // add item to dolist
  pub fn add(&mut self, task: String) -> Result<(), DoListErr> {
    if !self.list.insert(task.clone()) {
      return Err(DoListErr {
        err: format!("item \"{}\" already on the list", task),
      });
    }

    self.shuffle_queue.push_back(task);
    Ok(())
  }

  // drop item from dolist and return it
  pub fn drop(&mut self, task: String) -> Option<String> {
    if !self.list.remove(&task) {
      return None;
    }

    Some(task)
  }

  // reshuffle dolist
  fn shuffle(&mut self) {
    // make copy of the hashset keys as a vec
    let mut copy: Vec<String> = Vec::from_iter((&self.list).clone());

    // shuffle the slice
    let mut rng = thread_rng();
    copy.shuffle(&mut rng);

    // add shuffled items to queue
    for item in copy.iter() {
      self.shuffle_queue.push_back(item.clone().clone());
    }
  }

  // Pick and return a random task from the list
  // if the list is empty, return None
  pub fn pick(&mut self) -> Option<String> {
    if self.list.is_empty() {
      return None;
    }

    if self.shuffle_queue.is_empty() {
      self.shuffle();
    }

    self.shuffle_queue.pop_front()
  }

  // clear all items from list
  pub fn clear(&mut self) {
    self.list.clear();
    self.shuffle_queue.clear();
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
    assert_ne!(first_pick, second_pick); // no duplicates due to shuffle_queue
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
    let removed = do_list.drop("Task1".to_string());
    assert_eq!(removed, Some("Task1".to_string()));

    // Verify it's no longer in the list
    assert_eq!(do_list.drop("Task1".to_string()), None);
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
}
