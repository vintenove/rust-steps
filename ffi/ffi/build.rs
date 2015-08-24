extern crate gcc;

fn main() {
    gcc::Config::new().file("src/ffi.c").compile("libAddTwo.a");
}