#![feature(old_io)]
use std::old_io;
use std::rand;
use std::cmp::Ordering;

fn main() {
  loop {
    println!("Guess the number game!!!");
    print!("Enter your guess: ");

    let input = old_io::stdin().read_line()
    .ok()
    .expect("Failed to read line");
    if input.trim() == "quit" { return; }
    let input_parse : Result<u32, _> = input.trim().parse();

    let num : u32 = match input_parse {
      Ok(num) => num,
      Err(e) => {
        println!("Enter a number you dumbass (1-100)");
        continue;
      }
    };

    let secret_number = (rand::random::<u32>() % 100) + 1;
    //  println!("You guessed {} and the secret number is {}", num, secret_number);

    match cmp(num, secret_number) {
      Ordering::Less => println!("Too small boy"),
      Ordering::Equal => {
        println!("Yes!! You were right");
        return;
        },
      Ordering::Greater => println!("Too large dude"),
    }
  }
}

fn cmp(a: u32, s: u32) -> Ordering {
  if a < s { Ordering::Less }
  else if a > s { Ordering::Greater }
  else { Ordering::Equal }
}
