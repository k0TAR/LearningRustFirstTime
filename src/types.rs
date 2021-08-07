/*
Primitive Types
Integers: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
Floats: f32, f64
bool
char
Tuples
Arrays
*/
pub fn run () {
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);
  let is_active: bool = true;
  let face = '\u{1F600}';
  println!("{:?}.", (is_active, face));
}