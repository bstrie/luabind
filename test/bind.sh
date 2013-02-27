#!/usr/bin/env sh

./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lua.rs ../lua-5.2.1/src/lua.h
./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lualib.rs ../lua-5.2.1/src/lualib.h
./bindgen -match lua -match lstate -I/media/linhaus/rust/llvm/x86_64-unknown-linux-gnu/Release+Asserts/lib/clang/3.2/include -o rust-lauxlib.rs ../lua-5.2.1/src/lauxlib.h
