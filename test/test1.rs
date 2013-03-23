extern mod luah_lua52;
extern mod lualibh_lua52;
extern mod lauxlibh_lua52;

use luah_lua52::*;
use lualibh_lua52::*;
use lauxlibh_lua52::*;

fn main() {
    let L = luaL_newstate();

    let ret1 = str::as_c_str("function foo (x,y) return x+y end",
                             |s| luaL_loadstring(L, s));
    let ret2 = lua_pcallk(L, 0, -1, 0, 0, ptr::null());
    match (ret1, ret2) {
        (0, 0) => (),
        _ => fail!()
    }

    str::as_c_str("foo", |s| lua_getglobal(L, s));

    lua_pushinteger(L, 5);
    lua_pushinteger(L, 3);

    lua_callk(L, 2, 1, 0, ptr::null());
    let ret3 = lua_tointegerx(L, -1, ptr::null());
    io::println(fmt!("Result: %?", ret3));
}
