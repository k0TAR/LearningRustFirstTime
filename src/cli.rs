pub fn run() {
  let args: Vec<String> = std::env::args().collect();
  println!("Args: {:?}", args);

  let command = args[1].clone();
  let name = "AAA";
  let status = "100%";
  println!("Command: {}", command);

  if command == "hello" {
    println!("{} {}", command, name);
  } else if command == "status" {
    println!("Status: {}", status);
  } else {
    println!("Not valid command: {}", command);
  }

}
