extern mod luah_luajit2;
extern mod lualibh_luajit2;
extern mod lauxlibh_luajit2;

use luah_luajit2::*;
use lualibh_luajit2::*;
use lauxlibh_luajit2::*;

fn main() {
    let L = luaL_newstate();

    let ret1 = str::as_c_str("function foo (x,y) return x+y end",
                             |s| luaL_loadstring(L, s));
    let ret2 = lua_pcall(L, 0, -1, 0);
    match (ret1, ret2) {
        (0, 0) => (),
        _ => fail!()
    }

    str::as_c_str("foo", |s| lua_getfield(L, -10002, s));

    lua_pushinteger(L, 5);
    lua_pushinteger(L, 3);

    lua_call(L, 2, 1);
    let ret3 = lua_tointeger(L, -1);
    io::println(fmt!("Result: %?", ret3));
}
