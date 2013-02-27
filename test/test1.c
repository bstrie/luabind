#include <stdio.h>
#include <stdlib.h>
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

int main()
{
    lua_State *L = luaL_newstate();

    //if (luaL_dostring(L, "function foo (x,y) return x+y end")) exit(1);
    if ((luaL_loadstring(L, "function foo (x,y) return x+y end") || lua_pcallk(L, 0, -1, 0, 0, 0))) exit(1);

    lua_getglobal(L, "foo");

    lua_pushinteger(L, 5);
    lua_pushinteger(L, 3);

    //lua_call(L, 2, 1);
    lua_callk(L, 2, 1, 0, 0);
    //printf("Result: %d\n", lua_tointeger(L, -1));
    printf("Result: %d\n", lua_tointegerx(L, -1, 0));

    return 0;
}
