// struct Color {
//   red: u8,
//   blue: u8,
//   green: u8
// }

// struct Color(u8,u8,u8);

struct Person {
  name: String,
  age: i8,
}

impl Person {
  fn new(name: &str, age: i8) -> Person {
    Person {
      name: name.to_string(),
      age: age,
    }
  }

  fn print_detail(&self) -> String{
    format!("Person: name={name}, age={age}.", name=self.name, age=self.age)
  }

  fn to_tuple(self)->(String, i8){
    (self.name, self.age)
  }

  fn set_name(&mut self, name: &str){
    self.name = name.to_string();
  }

  fn set_age(&mut self, age: i8){
    self.age = age;
  }
}

pub fn run() {
  // let mut c = Color {
  //   red: 255,
  //   green: 0,
  //   blue: 0
  // };

  // c.red = 200;

  // println!("Color RGB: {} {} {}", c.red, c.blue, c.green);

  // let mut c = Color(255,0,0);

  // c.0 = 200;
  // println!("Color RGB: {} {} {}", c.0, c.1, c.2);

  let mut p = Person::new("AAA", 20);
  println!("{}", p.print_detail());
  
  p.set_name("BBB");
  p.set_age(10);

  println!("{}", p.print_detail());
  
  println!("{:?}", p.to_tuple());
}
