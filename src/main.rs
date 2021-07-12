mod lib;

use lib::Stuff;

fn main() {

  let a = 4;
  let b = 2;

  println!("{} + {} = {}", a, b, Stuff::new(a, b).sum());
}
