pub fn run(){
  greetings("Hello", "AAA");
  let get_sum = add(5,5);
  println!("Sum: {}", get_sum);

  //Closure 
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("Closure Sum: {}", add_nums(3,3));

}

fn greetings(greet: &str, name: &str){
  println!("{greet}, {name}.", greet=greet, name=name);
}

fn add(n1 : i32, n2: i32) -> i32 {
  let result = n1 + n2;
  result
}