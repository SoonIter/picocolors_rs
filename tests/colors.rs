#[cfg(test)]
mod colors {
    macro_rules! make_test {
        ( $($name: ident),* ) => {
            $(
                #[test]
                fn $name() {
                  use picocolors_rs::$name;
                  let color: String = $name("I am a string.");
                  println!("{}",color);
                }
            )*
        }
    }

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
