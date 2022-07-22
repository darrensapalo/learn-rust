use std::fmt;

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "( {}, {} )\n", self.0, self.1)?;
    write!(f, "( {}, {} )", self.2, self.3)
  }
}

fn reverse(m: Matrix) -> Matrix {
  Matrix(m.0, m.2, m.1, m.3)
}

fn main() {
  // let long_tuple = (
  //   1u8, 2u16, 3u32, 4u64,
  //   -1i8, -2i16, -3i32, -4i64,
  //   0.1f32, 0.2f64, 'a', true,
  // );

  // println!("long tuple first value: {}", long_tuple.0);
  // println!("long tuple second value: {}", long_tuple.1);

  // let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

  // println!("tuple of tuple: {:?}", tuple_of_tuples);
  // println!("one element tuple: {:?}", (9u8,).0);

  // let (mut a, b, c) = tuple_of_tuples.0;

  // println!("first value: {}", a);
  // a = 3;

  // println!("first value: {}", a);

  // let (mut d, e, f) = tuple_of_tuples.0;

  // println!("first value: {}", d);

  let m = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("Matrix: {:?}", m);
  println!("{}", m);

  println!("Transposing!");
  println!("{}", reverse(m));
}