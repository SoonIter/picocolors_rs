use picocolors_rs::{bg_black, bold};
fn main() {
    println!(
        "hello {}",
        bold(&bg_black(format!("123123 {}", "1111").as_str()))
    );
}
