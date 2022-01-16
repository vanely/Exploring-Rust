use rand::Rng;
use std::cmp::Ordering;
use std::io;
// run `cargo doc --open` to generate browser executable docs for crates being used


fn main() {
  println!("Guess the number.");
  println!("Please, enter your guess! >=D");

  // use rand crate to generate random number
  // can also use 1..=100 which is inclusive on the upperbound
  let secret_number = rand::thread_rng().gen_range(1..=50);
  // println!("Random number: {}", secret_num);

  let mut guess_string = String::new();
  io::stdin()
    .read_line(&mut guess_string)
    .expect("Failed to read line!");
  
  // convert number string "guess" to numeric value?\
  let mut guess_number: u32 = guess_string.trim().parse().expect("Unable to cast string to unsigned integer");

  // this comparison cmp function can return either of the below Ordering enum types.
  match guess_number.cmp(&secret_number) {
    Ordering::Less => println!("\nYour guess: {}, is too low.\nThe secret number was: {}", guess_number, secret_number),
    Ordering::Greater => println!("\nYour guess: {}, is too high.\nThe secret number was: {}", guess_number, secret_number),
    Ordering::Equal => println!("\nYour guess: {}, is CORRECT!!!", guess_number),
  }

  #[allow(unused_parens)] // inline modifications to linter
  while (guess_number != secret_number) {
    println!("\n");
    println!("Guess again! >>=D");
    let mut guess_string = String::new();

    io::stdin()
      .read_line(&mut guess_string)
      .expect("Failed to read line");

    guess_number = guess_string.trim().parse().expect("Unable to cast string to unsigned integer");
    if (guess_number < secret_number) {
      println!("Your guess: {}, is too low.\nThe secret number was: {}", guess_number, secret_number);
    } else if (guess_number > secret_number) {
      println!("Your guess: {}, is too high.\nThe secret number was: {}", guess_number, secret_number);
    } else {
      println!("Your guess: {}, is CORRECT!!!", guess_number);
    }
  }
}
