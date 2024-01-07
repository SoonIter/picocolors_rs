use picocolors_rs::{bg_black, bold};
fn main() {
  println!("hello {}", bold(bg_black("world")));
}
