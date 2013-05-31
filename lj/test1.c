#include <stdio.h>
#include <stdlib.h>
#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

int main()
{
    lua_State *L = luaL_newstate();

    //if (luaL_dostring(L, "function foo (x,y) return x+y end")) exit(1);
    if ((luaL_loadstring(L, "function foo (x,y) return x+y end") || lua_pcall(L, 0, -1, 0))) exit(1);

    //lua_getglobal(L, "foo");
    lua_getfield(L, -10002, "foo");

    lua_pushinteger(L, 5);
    lua_pushinteger(L, 3);

    //lua_call(L, 2, 1);
    lua_call(L, 2, 1);
    //printf("Result: %d\n", lua_tointeger(L, -1));
    printf("Result: %d\n", lua_tointeger(L, -1));

    return 0;
}
