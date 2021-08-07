enum Movement {
  Up,
  Down,
  Right,
  Left
}

fn move_avatar(m : Movement){
  match m {
    Movement::Up => println!("UP"),
    Movement::Down => println!("Down"),
    Movement::Right => println!("Right"),
    Movement::Left => println!("Left"),
  }
}

pub fn run() {
  let avatar1 = Movement::Up;
  let avatar2 = Movement::Down;
  let avatar3 = Movement::Right;
  let avatar4 = Movement::Left;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}
