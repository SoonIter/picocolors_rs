# picocolors

rust version of picocolors

[![crates-io](https://badgen.net/crates/v/picocolors_rs)](https://crates.io/crates/picocolors)

```rust
use picocolors::{bg_black, bold};

fn main() {
  println!("hello {}", bold(bg_black("world")));
}
```
