#!/usr/bin/env sh

gcc -o c1 test1.c -I/usr/local/include/luajit-2.0 -lluajit-5.1
rustc -L . -o r1 test1.rs
