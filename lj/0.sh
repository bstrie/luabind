#!/usr/bin/env sh

gcc -o c0 test0.c -I/usr/local/include/luajit-2.0 -lluajit-5.1
rustc -L . -o r0 test0.rs
