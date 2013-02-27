#!/usr/bin/env sh

LD_LIBRARY_PATH=/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lua.rs ../lua-5.2.1/src/lua.h
LD_LIBRARY_PATH=/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lualib.rs ../lua-5.2.1/src/lualib.h
LD_LIBRARY_PATH=/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/ ./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lauxlib.rs ../lua-5.2.1/src/lauxlib.h
