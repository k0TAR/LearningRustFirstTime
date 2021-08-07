//Primitive str : Immutable fixed-length.
//String : Growable, heap-allocated data structure.

pub fn run(){
  let mut hello = String::from("Hello ");
  println!("\"{text}\" : Length = {textLength}.", text=hello, textLength=hello.len());
  hello.push('\u{1F600}');
  hello.push_str(" World!");
  println!("\"{text}\" : Length = {textLength}.", text=hello, textLength=hello.len());
  println!("\"{text}\" : Capacity = {textCapacity}.", text=hello, textCapacity=hello.capacity());
  println!("Replace : \"{text}\".", text=hello.replace("Hello", "Hoi"));
  println!("\"{text}\" : Length = {textLength}.", text=hello, textLength=hello.len());

  for word in hello.split_whitespace() {
    println!("{}.", word);
  }

  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');

  assert_eq!(10, s.capacity());
  println!("{}", s);

}