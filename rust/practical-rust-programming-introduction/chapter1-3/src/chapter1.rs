pub fn main() {
  let mut x = 5;
  println!("{}", x);
  x = 6;
  println!("{}", x);

  let source = vec![1, 2, 3, 4, 5];
  let result = source
    .into_iter()
    .filter(|n| n % 2 == 0)
    .map(|n| n.to_string())
    .collect::<Vec<String>>();

  let objective: Option<i32> = Some(1);
  match objective {
    Some(x) if x % 2 == 0 => println!("偶数 {}", x),
    Some(x) => println!("奇数, {}", x),
    None => println!("値なし")
  }

  let mut v = vec![];
  v.push(1);

  let dog = Dog{};
  let cat = Cat{};
  show_animal_data(dog);
  show_animal_data(cat);
}

trait Animal {
  fn lifespan(&self) -> u32;
  fn scientific_name(&self) -> String;
}

struct Dog;

impl Animal for Dog {
  fn lifespan(&self) -> u32 {
      13
  }

  fn scientific_name(&self) -> String {
      "Canis lupus familiaris".to_string()
  }
}

struct Cat;

impl Animal for Cat {
  fn lifespan(&self) -> u32 {
      16
  }

  fn scientific_name(&self) -> String {
      "Felis catus".to_string()
  }
}

fn show_animal_data<T: Animal>(animal: T) {
  println!("Lifespan: {} years", animal.lifespan());
  println!("Scientific name: {}", animal.scientific_name());
}
