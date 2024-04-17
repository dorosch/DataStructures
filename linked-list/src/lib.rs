use std::fmt::Display;

/// A singly linked list implementation.
/// 
/// This data structure represents a singly linked list, where each element
/// (node) contains a value and a reference to the next element in the list.
/// The list starts with a head node, and each node points to the next node
/// until the end of the list is reached (where the next node is None).
/// 
/// # Example
///
/// ```
/// let mut list: LinkedList<i32> = LinkedList::new();
/// assert!(list.is_empty());
///
/// // Append some elements to the list
/// list.append(1);
/// list.append(2);
/// list.append(3);
/// assert_eq!(list.len(), 3);
///
/// // Prepend an element to the list
/// list.prepend(0);
/// assert_eq!(list.len(), 4);
/// ```
pub struct LinkedList<T: Display> {
  head: Option<Box<Node<T>>>
}

/// Represents a node in a singly linked list.
struct Node<T> {
  /// The value stored in the node.
  value: T,
  /// Pointer to the next node in the list.
  next: Option<Box<Node<T>>>
}

impl<T: Display> LinkedList<T> {
  /// Creates a new empty linked list.
  fn new() -> Self {
    Self { head: None }
  }

  /// Checks if the linked list is empty.
  fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  /// Returns the number of elements in the linked list.
  fn len(&self) -> usize {
    let mut size: usize = 0;
    let mut current = &self.head;

    while let Some(node) = current {
      size += 1;
      current = &node.next;
    }

    size
  }

  /// Inserts a new element at the beginning of the linked list.
  fn prepend(&mut self, value: T) {
    self.head = Some(
      Box::new(Node {
        value,
        next: self.head.take()
      }
    ));
  }

  /// Appends a new element at the end of the linked list.
  fn append(&mut self, value: T) {
    if self.head.is_none() {
      self.prepend(value);
      return;
    }

    let mut current = self.head.as_mut();

    while let Some(node) = current {
      if node.next.is_none() {
        node.next = Some(
          Box::new(Node {
            value,
            next: None
          })
        );
        break;
      }

      current = node.next.as_mut();
    }
  }
}

impl<T: Display> Display for LinkedList<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut current = &self.head;

    write!(f, "[")?;

    while let Some(node) = current {
      write!(f, "{}", node.value)?;
      current = &node.next;
    }

    write!(f, "]")
  }
}


#[cfg(test)]
mod tests {
  use super::LinkedList;

  #[test]
  fn empty_list() {
    assert!(LinkedList::<i32>::new().is_empty());
  }

  #[test]
  fn len() {
    let mut list = LinkedList::<i32>::new();

    assert_eq!(list.len(), 0);
    list.prepend(32);
    assert_eq!(list.len(), 1);
    list.prepend(64);
    assert_eq!(list.len(), 2);
  }

  #[test]
  fn prepend() {
    let mut list = LinkedList::<i32>::new();
    list.prepend(32);

    assert!(!list.is_empty());
  }

  #[test]
  fn append() {
    let mut list = LinkedList::<i32>::new();
    list.append(32);
    list.append(64);

    assert!(!list.is_empty());
    assert_eq!(list.len(), 2);
  }
}
