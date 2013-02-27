extern mod rust_lua;
extern mod rust_lualib;
extern mod rust_lauxlib;

use rust_lua::*;
use rust_lualib::*;
use rust_lauxlib::*;

fn main() {
    let L = luaL_newstate();

    luaL_openlibs(L);

    lua_close(L);
}
