# picocolors_rs

[![crates-io](https://badgen.net/crates/v/picocolors_rs)](https://crates.io/crates/picocolors_rs)

```rust
use picocolors_rs::{bg_black, bold};

fn main() {
  println!("hello {}", bold(bg_black("world")));
}
```