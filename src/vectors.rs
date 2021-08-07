//Dynamic list

pub fn run(){
  let mut numbers: Vec<i32> = vec![0 , 1, 2, 3, 4];
  print_numbers(&numbers);

  numbers.push(5);
  numbers.push(6);

  print_numbers(&numbers);

  let slice: &[i32] = &numbers[1..3];
  println!("Slice : {:?}.", slice);

  for x in numbers.iter() {
    println!("{}", x);
  }

  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("Numbers mutated: {:?}", numbers);
}

fn print_numbers(numbers : &Vec<i32>){
  println!("{:?}", numbers); 
  //Vectors are heap allocated.
  println!("Array occupies {} bytes.", std::mem::size_of_val(numbers) );
}