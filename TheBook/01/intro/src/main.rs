fn main() {
  println!("Hello, my name is Vanely");

  // primitives types in Rust
  // u8, i8, u16, i16, u32, i32, u64 i64
  // f32, f64
  // bool

  // variables are inherently immutable
  let n = 5; // would not be able to reassign this variable
  let mut x = 10; // this var is mutable
  let a: i8 = 10;
  let b: u8 = 15;
  let c: i16 = 20;
  let d: u16 = 25;
  let e: i32 = 30;
  let f: u32 = 35;
  let g: i64 = 8837472;
  let h: u64 = 645688468;
  let mut i: f32 = 20.2;
  let mut j: f64 = 12.63826382;

  // booleans
  let mut k: bool = true;
  let l = false; // rust allows both, dynamic and static typing.

  // chars
  let m = 'a';
  let mut n: char = 'b';

  //--------------------------------------------------

  // tuples (type is similar to an interface)
  let tup: (f32, i16, char) = (3.8, 25, 'z');
  let tup_two: (i8, bool, f64) = (10, false, 6.625978);

  // tuples can be nested
  let tup_three = (25, tup_two);

  // these curly brace placeholders arr like string formatting in python
  println!("{} {}", tup.0, tup.2);

  // {:?} is a debug flag, can be used to output entire tuples
  // {:#?} is a prettyfied debug flag, seperates values into their own lines
  println!("{:?}\n {:#?}", tup, tup_three);

  // tuples can also be destructored, like in JS
  let (a, b, c) = tup;

  // if we want to ignore a value in a parameter space,
  // instead of passing a variable to cosume the value,
  // we can place an underscore on the parameter space.
  let (_, y, _) = tup_two;

  if (y) {
    println!("Truthy value");
  } else {
    println!("Falsy value");
  }

  // tuple values can also be accessed by index like an object.
  let t1 = tup.0; // value is 3.8
  let t2 = tup_two.2;

  //--------------------------------------------------

  // arrays in rust must contain a single type.
  let arr = [1, 2, 3, 4, 5, 6]; // once defined, cannot changes size

  // values within an array can be accessed by index with brackets
  let a1 = arr[4]; // value is 5
}

