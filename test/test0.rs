extern mod luah_lua52;
extern mod lualibh_lua52;
extern mod lauxlibh_lua52;

use luah_lua52::*;
use lualibh_lua52::*;
use lauxlibh_lua52::*;

fn main() {
    let L = luaL_newstate();

    luaL_openlibs(L);

    lua_close(L);
}
