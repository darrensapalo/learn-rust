use std::mem;

fn analyze_slice(slice: &[i32]) {
  println!("first element of the slice: {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

fn main() {
  let xs: [i32; 5] = [1, 2, 3, 4, 5];

  let ys: [i32; 500] = [0; 500];

  println!("xs at 0: {}", xs[0]);
  println!("ys {}, {}, {}, {}", ys[0], ys[1], ys[2], ys[3]);

  println!("number of elements of an array: {}", xs.len());

  println!("array occupies {} bytes", mem::size_of_val(&ys));

  analyze_slice(&xs);

  println!("slice of an array");
  analyze_slice(&ys[1 .. 4]);

  let empty_array: [u32; 0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]);

  // What values do the arrays have if I don't
  // initialize it with zeroes?
}