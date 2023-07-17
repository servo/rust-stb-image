#!/bin/bash
if test "x$BINDGEN" == "x"; then
    BINDGEN=bindgen
fi
$BINDGEN --no-layout-tests --allowlist-function "stbi_.*" -o src/stb_image.rs src/stb_image.c
