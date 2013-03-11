#!/usr/bin/env sh

gcc -o c2 test2.c -I/usr/local/include -L/usr/local/lib -llua -lm -ldl
#rustc -L . -o r2 test2.rs
