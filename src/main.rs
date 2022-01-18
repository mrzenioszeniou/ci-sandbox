mod lib;

use lib::Stuff;

fn main() {

  let a = 4;
  let b = 3;

  println!("{} + {} == {}", a, b, Stuff::new(a, b).sum());
}
