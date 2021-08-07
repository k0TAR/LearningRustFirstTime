pub fn run() {
  let arr1 = [0, 1, 2];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2));

  let vec1 = vec![0, 1, 2];
  let vec2 = &vec1;

  println!("Values: {:?}", (&vec1, vec2));
}
