#!/usr/bin/env sh

gcc -o c1 test1.c -I/usr/local/include -L/usr/local/lib -llua -lm -ldl
rustc -L . -o r1 test1.rs
