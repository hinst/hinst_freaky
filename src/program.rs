extern crate rand;

use std::io;
use rand::Rng;
mod app;
mod game;

fn main() {
  println!("GLOBAL START");
  let mut text = String::new();
  io::stdin().read_line(&mut text).expect("Failed to read line");
  println!("Received text: '{}'", text);
  let random_number = rand::thread_rng().gen_range(0, 10);
  println!("random_number = {}", random_number);
  println!("GLOBAL EXIT");
}