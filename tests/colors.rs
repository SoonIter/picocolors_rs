macro_rules! make_test {
    ( $($name: ident),* ) => {
        $(
            #[test]
            fn $name() {
              use picocolors::$name;
              let color1: String = $name("I am a string.");
              let color2: String = $name("I am a string.".to_string());
              assert_eq!(color1, color2);
              println!("{}", color1);
            }
        )*
    }
}
#[cfg(test)]
mod colors {
  make_test! {
   reset,
   bold,
   dim,
   italic,
   underline,
   inverse,
   hidden,
   strikethrough,
   black,
   red,
   green,
   yellow,
   blue,
   magenta,
   cyan,
   white,
   gray,
   bg_black,
   bg_red,
   bg_green,
   bg_yellow,
   bg_blue,
   bg_magenta,
   bg_cyan,
   bg_white
  }
}
