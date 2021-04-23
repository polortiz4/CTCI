use super::stack::Stack;
use super::stack::pour_stack_to;


// Solution to Excercise 3.4

pub struct Queue<T> {
  forward: Stack<T>,
  backward: Stack<T>,
}

impl<T> Queue<T> {
  fn flip_forward(&mut self) {
    pour_stack_to(&mut self.backward, &mut self.forward)
  }
  fn flip_backward(&mut self) {
    pour_stack_to(&mut self.forward, &mut self.backward)
  }
  pub fn enqueue(&mut self, item: T) {
    self.flip_backward();
    self.backward.push(item)
  }

  pub fn dequeue(&mut self) -> Option<T> {
    self.flip_forward();
    self.forward.pop()
  }

  pub const fn new() -> Self {
    Queue {
      forward: Stack::new(),
      backward: Stack::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let mut my_queue: Queue<i32> = Queue::new();
    my_queue.enqueue(3);
    my_queue.enqueue(4);
    assert_eq!(my_queue.dequeue().unwrap(), 3);
    assert_eq!(my_queue.dequeue().unwrap(), 4);
  }
}
