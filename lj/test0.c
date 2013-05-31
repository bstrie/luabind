#include <lua.h>
#include <lauxlib.h>

int main(void) {
    lua_State *L;

    L = luaL_newstate();

    luaL_openlibs(L);

    lua_close(L);

    return 0;
}
