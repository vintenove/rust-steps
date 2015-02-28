extern crate phrases;

use phrases::english;
use phrases::japanese;

fn main() {
  println!("Hello in English: {}", english::hello());
  println!("Hello in Japanese: {}", english::hello());
  println!("Goodbye in English: {}", japanese::goodbye());
  println!("Goodbye in Japanese: {}", japanese::goodbye());
}
