pub struct Stuff {
  foo: usize,
  bar: usize,
}

impl Stuff {
  pub fn new(foo:usize, bar:usize) -> Self {
    Self {
      foo,
      bar,
    }
  }

  pub fn sum(&self) -> usize {
    self.foo + self.bar
  }
}