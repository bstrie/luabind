#!/usr/bin/env sh

gcc -o c0 test0.c -I/usr/local/include -L/usr/local/lib -llua -lm -ldl
rustc -o r0 test0.rs
