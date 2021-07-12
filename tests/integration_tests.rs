use ci_sandbox::Stuff;

#[test]
fn stuff() {

  const N : usize = 100_000_000;

  for i in 0..N {

    assert_eq!(Stuff::new(i, N - i).sum(), N);
  }
}