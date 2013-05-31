extern mod luah_luajit2;
extern mod lualibh_luajit2;
extern mod lauxlibh_luajit2;

use luah_luajit2::*;
use lualibh_luajit2::*;
use lauxlibh_luajit2::*;

fn main() {
    let L = luaL_newstate();

    luaL_openlibs(L);

    lua_close(L);
}
