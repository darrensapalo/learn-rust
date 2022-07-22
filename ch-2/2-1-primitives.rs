
fn main() {
  
  let logicial: bool = true;

  let a_float = 1.0f64;
  let an_integer = 5i32;

  println!("float: {}", a_float);
  println!("int: {}", an_integer);

  let defaultFloat = 1.3;
  let defaultInteger = 25;

  println!("default float: {}", defaultFloat);
  println!("default int: {}", defaultInteger);

  let mut inferred_type = 2;
  inferred_type = 4294967296i64;

  println!("number: {}", inferred_type);

  let mut mutable = 12;
  mutable = 21;

  let underscoredNumbers = 1_00_000_0_000u32;
  println!("a million: {}", underscoredNumbers);


  let (a, b) = reverse((25, false));
  println!("reversed: ({}, {})", a, b);

}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (integer, boolean) = pair;

  (boolean, integer)
}