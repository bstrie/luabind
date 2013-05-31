#!/usr/bin/env sh

LD_LIBRARY_PATH=/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o luah_luajit2.rs ../luajit-2.0/src/lua.h
LD_LIBRARY_PATH=/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o lualibh_luajit2.rs ../luajit-2.0/src/lualib.h
LD_LIBRARY_PATH=/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rustwclang/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o lauxlibh_luajit2.rs ../luajit-2.0/src/lauxlib.h
