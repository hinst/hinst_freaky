extern crate rand;

use std::io;
use rand::Rng;

mod game;

use game::app::*;

fn main() {
  println!("GLOBAL START");
  let mut app = App::default();
}