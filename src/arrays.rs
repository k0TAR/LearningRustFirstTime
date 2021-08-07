//Fixed list

pub fn run(){
  let numbers: [i32; 5] = [0 , 1, 2, 3, 4];
  println!("{:?}", numbers); 
  //Arrays are stack allocated.
  println!("Array occupies {} bytes.", std::mem::size_of_val(&numbers) );

  let slice: &[i32] = &numbers[1..3];
  println!("Slice : {:?}.", slice);
}