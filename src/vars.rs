pub fn run(){
  let name = "AAA";
  let mut age = 20;
  println!("Name is {name}. Age is {age}.", name=name, age=age);
  age = 10;
  println!("Name is {name}. Age is {age}.", name=name, age=age);

  const ID: i32 = 001;
  println!("ID: {}.", ID);

  let ( my_name, my_age ) = ("BBB", 30);
  println!("Name is {name}. Age is {age}.", name=my_name, age=my_age);

}