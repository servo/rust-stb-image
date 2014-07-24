#!/bin/sh
./configure
make ./libstb-image.a
cp ./libstb-image.a $OUT_DIR
