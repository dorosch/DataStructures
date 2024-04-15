use std::ops::Deref;


/// A generic stack data structure.
///
/// This struct represents a stack, which is a Last-In-First-Out 
/// (LIFO) data structure. Elements are added and removed from 
/// the top of the stack. This implementation uses a Vec<T> 
/// internally to store the elements.
/// 
/// # Example
/// 
/// ```
/// let mut stack = Stack::<i32>::new();
/// stack.push(7);
/// stack.push(32);
/// 
/// // item: 32, item: 7
/// for item in stack.iter() {
///   println!("item: {item}");
/// }
/// 
/// assert!(!stack.is_empty());
/// assert_eq!(stack.peek(), &32);
/// assert_eq!(stack.pop(), Some(32));
/// assert_eq!(stack.len(), 1);
/// ```
pub struct Stack<T> {
  items: Vec<T>
}

impl<T> Default for Stack<T> {
  /// Creates a new instance of `Stack` with default values.
  /// 
  /// # Example
  /// 
  /// ```
  /// let stack = Stack::<i32>::default();
  /// ```
  fn default() -> Self {
    Self::new()
  }
}

impl<T> Stack<T> {
  /// Creates a new empty instance of `Stack`.
  pub fn new() -> Self {
    Self { items: Vec::new() }
  }

  /// Pushes a value onto the top of the stack.
  pub fn push(&mut self, value: T) {
    self.items.push(value);
  }

  /// Removes the top value from the stack and returns it, 
  /// or `None` if the stack is empty.
  pub fn pop(&mut self) -> Option<T> {
    self.items.pop()
  }

  /// Returns a reference to the top value of the stack, 
  /// or `None` if the stack is empty.
  pub fn peek(&self) -> Option<&T> {
    self.items.last()
  }

  /// Checks if the stack is empty.
  pub fn is_empty(&self) -> bool {
    self.items.is_empty()
  }

  /// Returns an iterator over the elements of the stack.
  pub fn iter(&self) -> StackIterator<T> {
    StackIterator {
        items: &self.items,
        index: self.items.len(),
    }
  }
}

/// An iterator over the elements of a `Stack`.
pub struct StackIterator<'a, T> {
  items: &'a Vec<T>,
  index: usize,
}

impl<'a, T> Iterator for StackIterator<'a, T> {
  type Item = &'a T;

  /// Advances the iterator and returns the next element, 
  /// or `None` if the iterator is exhausted.
  ///
  /// This method advances the iterator to the next element 
  /// and returns a reference to it. If the iterator is exhausted, 
  /// it returns `None`.
  fn next(&mut self) -> Option<Self::Item> {
      if self.index > 0 {
          self.index -= 1;
          Some(&self.items[self.index])
      } else {
          None
      }
  }
}

impl<T> Deref for Stack<T> {
  type Target = Vec<T>;

  /// Returns a reference to the internal vector of the stack.
  ///
  /// This method allows you to access the internal vector of the stack
  /// through a reference. It is useful when you need to directly manipulate
  /// the underlying vector.
  fn deref(&self) -> &Self::Target {
      &self.items
  }
}


#[cfg(test)]
mod tests {
  use super::Stack;

  #[test]
  fn empty_stack() {
    assert!(Stack::<i32>::new().is_empty());
  }

  #[test]
  fn push() {
    let mut stack = Stack::<i32>::new();
    stack.push(42);
    stack.push(314);

    assert_eq!(stack.len(), 2);
  }

  #[test]
  fn pop() {
    let mut stack = Stack::<i32>::new();
    stack.push(42);
    stack.push(314);

    assert_eq!(stack.pop(), Some(314));
    assert_eq!(stack.pop(), Some(42));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn peek() {
    let mut stack = Stack::<i32>::new();
    stack.push(42);
    stack.push(314);

    assert_eq!(stack.peek(), Some(&314));
    assert_eq!(stack.len(), 2);
  }

  #[test]
  fn is_empty() {
    let mut stack = Stack::<i32>::new();
    stack.push(42);

    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
  }

  #[test]
  fn iter() {
    let mut stack = Stack::<i32>::new();
    stack.push(42);
    stack.push(314);

    for (index, item) in stack.iter().enumerate() {
      match index {
        0 => assert_eq!(item, &314),
        1 => assert_eq!(item, &42),
        _ => (),
      }
    }
  }
}
