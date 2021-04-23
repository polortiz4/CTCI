use std::collections::LinkedList;

// Solution to Excercice 3.6

pub enum AnimalKind {
  Dog,
  Cat,
}

pub struct Animal {
  pub kind: AnimalKind,
  pub name: String,
}
impl Animal {
  pub fn new(kind: AnimalKind, name: &str) -> Self {
    Animal {
      kind,
      name: name.to_string(),
    }
  }
}

struct AnimalNode {
  animal: Animal,
  id: usize,
}

pub struct AnimalQueue {
  dog_list: LinkedList<AnimalNode>,
  cat_list: LinkedList<AnimalNode>,
  next_id: usize,
}

impl AnimalQueue {
  /// Creates an new `AnimalQueue`
  ///
  /// ```
  /// use ctci::stacks_and_queues::animal_shelter::*;
  ///
  /// let a_queue = AnimalQueue::new();
  /// ```
  pub const fn new() -> Self {
    AnimalQueue {
      dog_list: LinkedList::new(),
      cat_list: LinkedList::new(),
      next_id: 0,
    }
  }

  /// Adds an animal to the corresponding queue
  ///
  /// ```
  /// use ctci::stacks_and_queues::animal_shelter::*;
  ///
  /// let mut a_queue = AnimalQueue::new();
  /// let animal = Animal::new(AnimalKind::Dog, "Cassius");
  /// a_queue.enqueue(animal);
  ///
  /// assert_eq!(a_queue.dequeueAny().unwrap().name, "Cassius");
  /// ```
  pub fn enqueue(&mut self, animal: Animal) {
    let node = AnimalNode {
      animal: animal,
      id: self.next_id,
    };
    match node.animal.kind {
      AnimalKind::Dog => {
        self.dog_list.push_back(node);
      }
      AnimalKind::Cat => {
        self.cat_list.push_back(node);
      }
    }
    self.next_id += 1;
  }

  /// Removes and returns the last added animal
  ///
  /// ```
  /// use ctci::stacks_and_queues::animal_shelter::*;
  ///
  /// let mut a_queue = AnimalQueue::new();
  /// let cat = Animal::new(AnimalKind::Cat, "catius");
  /// let dog = Animal::new(AnimalKind::Dog, "dogius");
  /// let mittens = Animal::new(AnimalKind::Cat, "mittens");
  ///
  /// a_queue.enqueue(cat);
  /// a_queue.enqueue(dog);
  /// a_queue.enqueue(mittens);
  ///
  /// assert_eq!(a_queue.dequeueAny().unwrap().name, "catius");
  /// assert_eq!(a_queue.dequeueAny().unwrap().name, "dogius");
  /// ```
  #[allow(non_snake_case)]
  pub fn dequeueAny(&mut self) -> Option<Animal> {
    if let Some(oldest_dog) = self.dog_list.front() {
      if let Some(oldest_cat) = self.cat_list.front() {
        if oldest_cat.id < oldest_dog.id {
          return Some(self.cat_list.pop_front().unwrap().animal);
        }
      }
      return Some(self.dog_list.pop_front().unwrap().animal);
    }
    None
  }

  /// Removes and returns the last Dog
  ///
  /// ```
  /// use ctci::stacks_and_queues::animal_shelter::*;
  ///
  /// let mut a_queue = AnimalQueue::new();
  /// let cat = Animal::new(AnimalKind::Cat, "catius");
  /// let dog = Animal::new(AnimalKind::Dog, "dogius");
  /// let mittens = Animal::new(AnimalKind::Cat, "mittens");
  ///
  /// a_queue.enqueue(cat);
  /// a_queue.enqueue(dog);
  /// a_queue.enqueue(mittens);
  ///
  /// assert_eq!(a_queue.dequeueDog().unwrap().name, "dogius");
  /// ```
  #[allow(non_snake_case)]
  pub fn dequeueDog(&mut self) -> Option<Animal> {
    match self.dog_list.pop_front() {
      Some(dog) => Some(dog.animal),
      None => None,
    }
  }

  /// Removes and returns the last Cat
  ///
  /// ```
  /// use ctci::stacks_and_queues::animal_shelter::*;
  ///
  /// let mut a_queue = AnimalQueue::new();
  /// let cat = Animal::new(AnimalKind::Cat, "catius");
  /// let dog = Animal::new(AnimalKind::Dog, "dogius");
  /// let mittens = Animal::new(AnimalKind::Cat, "mittens");
  ///
  /// a_queue.enqueue(cat);
  /// a_queue.enqueue(dog);
  /// a_queue.enqueue(mittens);
  ///
  /// assert_eq!(a_queue.dequeueCat().unwrap().name, "catius");
  /// ```
  #[allow(non_snake_case)]
  pub fn dequeueCat(&mut self) -> Option<Animal> {
    match self.cat_list.pop_front() {
      Some(cat) => Some(cat.animal),
      None => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_test() {
    let mut a_queue = AnimalQueue::new();
    let cat = Animal::new(AnimalKind::Cat, "catius");
    let dog = Animal::new(AnimalKind::Dog, "dogius");
    let mittens = Animal::new(AnimalKind::Cat, "mittens");
    a_queue.enqueue(cat);
    a_queue.enqueue(dog);
    a_queue.enqueue(mittens);
    assert_eq!(a_queue.dequeueAny().unwrap().name, "catius");
    assert_eq!(a_queue.dequeueAny().unwrap().name, "dogius");

    a_queue.enqueue(Animal::new(AnimalKind::Dog, "Nala"));
    a_queue.enqueue(Animal::new(AnimalKind::Dog, "Drogba"));
    a_queue.enqueue(Animal::new(AnimalKind::Cat, "snowball"));

    assert_eq!(a_queue.dequeueDog().unwrap().name, "Nala");
    assert_eq!(a_queue.dequeueCat().unwrap().name, "mittens");
    assert_eq!(a_queue.dequeueCat().unwrap().name, "snowball");
  }
}
