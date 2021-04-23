use super::stack::Stack;
use super::stack::pour_stack_to;

// Solution to Excercice 3.5


/// Sorts items in a stack such that the small ones are at the top
/// 
/// # Examples
/// 
/// ```
/// use ctci::stacks_and_queues::Stack;
/// use ctci::stacks_and_queues::sort_stack::sort_stack;
/// 
/// let mut my_stack: Stack<i32> = Stack::new();
/// 
/// my_stack.push(1);
/// my_stack.push(3);
/// my_stack.push(2);
/// 
/// sort_stack(&mut my_stack);
/// 
/// assert_eq!(my_stack.pop(), Some(1));
/// assert_eq!(my_stack.pop(), Some(2));
/// assert_eq!(my_stack.pop(), Some(3));
/// ```
pub fn sort_stack<T: Ord>(stack: &mut Stack<T>){
  let mut helper_stack: Stack<T> = Stack::new();

  while let Some(item) = stack.pop(){
    while let Some(other_top) = helper_stack.pop(){
      if item < other_top {
        stack.push(other_top);
      }
      else{
        helper_stack.push(other_top);
        break;
      }
    }
    helper_stack.push(item);
  }
  pour_stack_to(&mut helper_stack, stack);
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_test(){
    let mut my_stack: Stack<i32> = Stack::new();

    my_stack.push(1);
    my_stack.push(3);
    my_stack.push(2);
    my_stack.push(10);
    my_stack.push(-1);
    my_stack.push(0);

    sort_stack(&mut my_stack);

    assert_eq!(my_stack.pop(), Some(-1));
    assert_eq!(my_stack.pop(), Some(0));
    assert_eq!(my_stack.pop(), Some(1));
    assert_eq!(my_stack.pop(), Some(2));
    assert_eq!(my_stack.pop(), Some(3));
    assert_eq!(my_stack.pop(), Some(10));
  }
}