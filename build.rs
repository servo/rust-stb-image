/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

fn main() {
    let mut build = cc::Build::new();

    println!("cargo:rerun-if-changed=src/stb_image.c");

    build
        .cpp(true)
        .define("STB_IMAGE_IMPLEMENTATION", None)
        .file("src/stb_image.c");

    build.compile("libstb_image");
}
