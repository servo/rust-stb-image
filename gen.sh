#!/bin/bash
if test "x$BINDGEN" == "x"; then
    BINDGEN=bindgen
fi
$BINDGEN -o src/stb_image.rs src/stb_image.c

