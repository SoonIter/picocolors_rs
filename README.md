# picocolors

rust port of [picocolors](https://github.com/alexeyraspopov/picocolors)

[![crates-io](https://badgen.net/crates/v/picocolors_rs)](https://crates.io/crates/picocolors)

```rust
use picocolors::{bg_black, bold, formatter};
fn main() {
  println!("hello {}", bold(bg_black("world")));

  let custom_blue = formatter("\x1b[34m", "\x1b[39m");
  println!("hello {}", custom_blue("world"));
}
```
