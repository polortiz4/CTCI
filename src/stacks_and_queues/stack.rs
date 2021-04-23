pub struct Stack<T> {
  vector: Vec<T>,
}

impl<T> Stack<T> {
  pub fn pop(&mut self) -> Option<T> {
    self.vector.pop()
  }

  pub fn push(&mut self, item: T) {
    self.vector.push(item)
  }

  pub fn peek<'a>(&'a self) -> Option<&'a T> {
    self.vector.last()
  }

  pub fn is_empty(&self) -> bool {
    self.vector.is_empty()
  }

  /// Creates an empty Stack
  /// 
  /// # Examples
  /// 
  /// ```
  /// use ctci::stacks_and_queues::Stack;
  /// 
  /// let mut stack: Stack<i32> = Stack::new();
  /// ```
  pub const fn new() -> Self {
    Stack { vector: Vec::new() }
  }
}

/// pops all elements of from_stack and pushes them to to_stack
/// 
/// # Examples
/// 
/// ```
/// use ctci::stacks_and_queues::Stack;
/// use ctci::stacks_and_queues::pour_stack_to;
/// 
/// let mut from_stack: Stack<i32> = Stack::new();
/// let mut to_stack: Stack<i32> = Stack::new();
/// 
/// from_stack.push(1);
/// from_stack.push(2);
/// 
/// to_stack.push(3);
/// 
/// pour_stack_to(&mut from_stack, &mut to_stack);
/// 
/// assert_eq!(to_stack.pop(), Some(1));
/// assert_eq!(to_stack.pop(), Some(2));
/// assert_eq!(to_stack.pop(), Some(3));
/// ```
pub fn pour_stack_to<T>(from_stack: &mut Stack<T>, to_stack: &mut Stack<T>) {
  loop {
    if let Some(item) = from_stack.pop() {
      to_stack.push(item);
    } else {
      break;
    }
  }
}

