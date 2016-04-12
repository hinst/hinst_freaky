use std::io;

fn main() {
  println!("GLOBAL START...");
  let mut text = String::new();
  io::stdin().read_line(&mut text).expect("Failed to read line");
  println!("Received text: '{}'", text);
  println!("GLOBAL EXIT")
}