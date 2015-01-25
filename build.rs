extern crate gcc;
use std::default::Default;

fn main() {
   gcc::compile_library("libstb-image.a", &Default::default(), &["src/stb_image.c"]);
}
