/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
    .file("src/stb_image.c")
    .compile("libstb_image.a");

    let bindings = bindgen::Builder::default()
        .header("src/stb_image.c")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
