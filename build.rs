/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate gcc;

fn main() {
    gcc::compile_library("libstb_image.a", &["src/stb_image.c"]);
}
