use picocolors::{bg_black, bold, formatter};
fn main() {
  println!("hello {}", bold(bg_black("world")));

  let my_custom_blue = formatter("\x1b[34m", "\x1b[39m");
  println!("hello {}", my_custom_blue("world"));
}
