use picocolors_rs::bg_black;
fn main() {
    println!("hello {}", bg_black(format!("123123 {}", "1111").as_str()));
}