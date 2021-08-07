pub fn run() {
  //let mut count = 1;
  // loop {
  //   count += 1;
  //   println!("Number : {}", count);

  //   if count == 20 {
  //     break;
  //   }
  // }

  // while count <= 30 {
  //   fizz_buzz_print(&count);
  //   count += 1;
  // }

  for x in 1..31 {
    fizz_buzz_print(&x);
  }
}

fn fizz_buzz_print(count : &i32){
  if count % 3 == 0 {
    print!("Fizz");
  } 
  if count % 5 == 0 {
    print!("Buzz");
  }
  if count % 3 == 0 || count % 5 == 0 {
    println!();
  } else {
    println!("{}", count);
  }
}