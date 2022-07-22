#![allow(dead_code)]

#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn main() {
  let name = String::from("Peter");
  let age = 27;
  let peter = Person { name ,age };

  println!("{:?}", peter);

  let point: Point = Point { x: 10.3, y: 0.4 };
  println!("point coords: ({}, {})", point.x, point.y);

  let bottom_right = Point { x: 5.2, ..point };

  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
  let Point { x: left_edge, y: top_edge } = point;

  println!("left edge: {} right edge: {}", left_edge, top_edge);

  let _rectangle = Rectangle {
    top_left: Point { x: left_edge, y: top_edge },
    bottom_right: bottom_right,
  };

  println!("Rectangle: {:?}", _rectangle);

  let _unit = Unit;

  let pair = Pair(1, 0.1);

  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  let rectangleArea = rect_area(
    Rectangle{
      top_left: Point { x: 0.0, y: 10.0 },
      bottom_right: Point { x: 20.0, y: 0.0 },
    }
  );
  println!("Area of rectangle want 200; got {}", rectangleArea);
}

fn rect_area(rect: Rectangle) -> f32 {
  let length = rect.bottom_right.x - rect.top_left.x;
  let width = rect.top_left.y - rect.bottom_right.y;
  length * width
}

fn square(point: Point, size: f32) -> Rectangle {
  Rectangle{
    top_left: Point { x: point.x, y: point.y },
    bottom_right: Point { x: point.x + size, y: point.y + size },
  }
}