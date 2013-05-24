/* automatically generated by rust-bindgen */

use std::libc::*;
pub type Struct_lua_State = c_void;
pub type lua_State = Struct_lua_State;
pub type lua_CFunction = *u8;
pub type lua_Reader = *u8;
pub type lua_Writer = *u8;
pub type lua_Alloc = *u8;
pub type lua_Number = c_double;
pub type lua_Integer = ptrdiff_t;
pub type lua_Unsigned = c_uint;
//pub type lua_Debug = Struct_lua_Debug;
pub type lua_Hook = *u8;
//pub struct Struct_lua_Debug {
//    pub event: c_int,
//    pub name: *c_schar,
//    pub namewhat: *c_schar,
//    pub what: *c_schar,
//    pub source: *c_schar,
//    pub currentline: c_int,
//    pub linedefined: c_int,
//    pub lastlinedefined: c_int,
//    pub nups: c_uchar,
//    pub nparams: c_uchar,
//    pub isvararg: c_schar,
//    pub istailcall: c_schar,
//    pub short_src: [c_schar, ..60u],
//    pub i_ci: *mut Struct_CallInfo,
//}
pub type Struct_CallInfo = c_void;
pub struct Struct_luaL_Reg {
    pub name: *c_schar,
    pub func: lua_CFunction,
}
pub type luaL_Reg = Struct_luaL_Reg;
pub struct Struct_luaL_Buffer {
    pub b: *mut c_schar,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub initb: [c_schar, ..8192u],
}
pub type luaL_Buffer = Struct_luaL_Buffer;
pub struct Struct_luaL_Stream {
    pub f: *mut FILE,
    pub closef: lua_CFunction,
}
pub type luaL_Stream = Struct_luaL_Stream;
#[link_args = "-llua"]
pub extern "C" {
    fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    fn lua_close(L: *mut lua_State);
    fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
    fn lua_version(L: *mut lua_State) -> *lua_Number;
    fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_gettop(L: *mut lua_State) -> c_int;
    fn lua_settop(L: *mut lua_State, idx: c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    fn lua_remove(L: *mut lua_State, idx: c_int);
    fn lua_insert(L: *mut lua_State, idx: c_int);
    fn lua_replace(L: *mut lua_State, idx: c_int);
    fn lua_copy(L: *mut lua_State, fromidx: c_int, toidx: c_int);
    fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);
    fn lua_isnumber(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_isstring(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_iscfunction(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_typename(L: *mut lua_State, tp: c_int) -> *c_schar;
    fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) ->
     lua_Number;
    fn lua_tointegerx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) ->
     lua_Integer;
    fn lua_tounsignedx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) ->
     lua_Unsigned;
    fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) ->
     *c_schar;
    fn lua_rawlen(L: *mut lua_State, idx: c_int) -> size_t;
    fn lua_tocfunction(L: *mut lua_State, idx: c_int) -> lua_CFunction;
    fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    fn lua_topointer(L: *mut lua_State, idx: c_int) -> *c_void;
    fn lua_arith(L: *mut lua_State, op: c_int);
    fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;
    fn lua_compare(L: *mut lua_State, idx1: c_int, idx2: c_int, op: c_int) ->
     c_int;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
    fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned);
    fn lua_pushlstring(L: *mut lua_State, s: *c_schar, l: size_t) -> *c_schar;
    fn lua_pushstring(L: *mut lua_State, s: *c_schar) -> *c_schar;
//    fn lua_pushvfstring(L: *mut lua_State, fmt: *c_schar,
//                        argp: *mut __va_list_tag) -> *c_schar;
    fn lua_pushfstring(L: *mut lua_State, fmt: *c_schar) -> *c_schar;
    fn lua_pushcclosure(L: *mut lua_State, _fn: lua_CFunction, n: c_int);
    fn lua_pushboolean(L: *mut lua_State, b: c_int);
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut c_void);
    fn lua_pushthread(L: *mut lua_State) -> c_int;
    fn lua_getglobal(L: *mut lua_State, var: *c_schar);
    fn lua_gettable(L: *mut lua_State, idx: c_int);
    fn lua_getfield(L: *mut lua_State, idx: c_int, k: *c_schar);
    fn lua_rawget(L: *mut lua_State, idx: c_int);
    fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: c_int);
    fn lua_rawgetp(L: *mut lua_State, idx: c_int, p: *c_void);
    fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut c_void;
    fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    fn lua_getuservalue(L: *mut lua_State, idx: c_int);
    fn lua_setglobal(L: *mut lua_State, var: *c_schar);
    fn lua_settable(L: *mut lua_State, idx: c_int);
    fn lua_setfield(L: *mut lua_State, idx: c_int, k: *c_schar);
    fn lua_rawset(L: *mut lua_State, idx: c_int);
    fn lua_rawseti(L: *mut lua_State, idx: c_int, n: c_int);
    fn lua_rawsetp(L: *mut lua_State, idx: c_int, p: *c_void);
    fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    fn lua_setuservalue(L: *mut lua_State, idx: c_int);
    fn lua_callk(L: *mut lua_State, nargs: c_int, nresults: c_int, ctx: c_int,
                 k: lua_CFunction);
    fn lua_getctx(L: *mut lua_State, ctx: *mut c_int) -> c_int;
    fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int,
                  errfunc: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    fn lua_load(L: *mut lua_State, reader: lua_Reader, dt: *mut c_void,
                chunkname: *c_schar, mode: *c_schar) -> c_int;
    fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut c_void) ->
     c_int;
    fn lua_yieldk(L: *mut lua_State, nresults: c_int, ctx: c_int,
                  k: lua_CFunction) -> c_int;
    fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: c_int) ->
     c_int;
    fn lua_status(L: *mut lua_State) -> c_int;
    fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
    fn lua_error(L: *mut lua_State) -> c_int;
    fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    fn lua_concat(L: *mut lua_State, n: c_int);
    fn lua_len(L: *mut lua_State, idx: c_int);
    fn lua_getallocf(L: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;
    fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut c_void);
//    fn lua_getstack(L: *mut lua_State, level: c_int, ar: *mut lua_Debug) ->
//     c_int;
//    fn lua_getinfo(L: *mut lua_State, what: *c_schar, ar: *mut lua_Debug) ->
//     c_int;
//    fn lua_getlocal(L: *mut lua_State, ar: *lua_Debug, n: c_int) -> *c_schar;
//    fn lua_setlocal(L: *mut lua_State, ar: *lua_Debug, n: c_int) -> *c_schar;
    fn lua_getupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) ->
     *c_schar;
    fn lua_setupvalue(L: *mut lua_State, funcindex: c_int, n: c_int) ->
     *c_schar;
    fn lua_upvalueid(L: *mut lua_State, fidx: c_int, n: c_int) -> *mut c_void;
    fn lua_upvaluejoin(L: *mut lua_State, fidx1: c_int, n1: c_int,
                       fidx2: c_int, n2: c_int);
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: c_int,
                   count: c_int) -> c_int;
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    fn lua_gethookmask(L: *mut lua_State) -> c_int;
    fn lua_gethookcount(L: *mut lua_State) -> c_int;
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number);
    fn luaL_getmetafield(L: *mut lua_State, obj: c_int, e: *c_schar) -> c_int;
    fn luaL_callmeta(L: *mut lua_State, obj: c_int, e: *c_schar) -> c_int;
    fn luaL_tolstring(L: *mut lua_State, idx: c_int, len: *mut size_t) ->
     *c_schar;
    fn luaL_argerror(L: *mut lua_State, numarg: c_int, extramsg: *c_schar) ->
     c_int;
    fn luaL_checklstring(L: *mut lua_State, numArg: c_int, l: *mut size_t) ->
     *c_schar;
    fn luaL_optlstring(L: *mut lua_State, numArg: c_int, def: *c_schar,
                       l: *mut size_t) -> *c_schar;
    fn luaL_checknumber(L: *mut lua_State, numArg: c_int) -> lua_Number;
    fn luaL_optnumber(L: *mut lua_State, nArg: c_int, def: lua_Number) ->
     lua_Number;
    fn luaL_checkinteger(L: *mut lua_State, numArg: c_int) -> lua_Integer;
    fn luaL_optinteger(L: *mut lua_State, nArg: c_int, def: lua_Integer) ->
     lua_Integer;
    fn luaL_checkunsigned(L: *mut lua_State, numArg: c_int) -> lua_Unsigned;
    fn luaL_optunsigned(L: *mut lua_State, numArg: c_int, def: lua_Unsigned)
     -> lua_Unsigned;
    fn luaL_checkstack(L: *mut lua_State, sz: c_int, msg: *c_schar);
    fn luaL_checktype(L: *mut lua_State, narg: c_int, t: c_int);
    fn luaL_checkany(L: *mut lua_State, narg: c_int);
    fn luaL_newmetatable(L: *mut lua_State, tname: *c_schar) -> c_int;
    fn luaL_setmetatable(L: *mut lua_State, tname: *c_schar);
    fn luaL_testudata(L: *mut lua_State, ud: c_int, tname: *c_schar) ->
     *mut c_void;
    fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *c_schar) ->
     *mut c_void;
    fn luaL_where(L: *mut lua_State, lvl: c_int);
    fn luaL_error(L: *mut lua_State, fmt: *c_schar) -> c_int;
    fn luaL_checkoption(L: *mut lua_State, narg: c_int, def: *c_schar,
                        lst: **c_schar) -> c_int;
    fn luaL_fileresult(L: *mut lua_State, stat: c_int, fname: *c_schar) ->
     c_int;
    fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int;
    fn luaL_ref(L: *mut lua_State, t: c_int) -> c_int;
    fn luaL_unref(L: *mut lua_State, t: c_int, _ref: c_int);
    fn luaL_loadfilex(L: *mut lua_State, filename: *c_schar, mode: *c_schar)
     -> c_int;
    fn luaL_loadbufferx(L: *mut lua_State, buff: *c_schar, sz: size_t,
                        name: *c_schar, mode: *c_schar) -> c_int;
    fn luaL_loadstring(L: *mut lua_State, s: *c_schar) -> c_int;
    fn luaL_newstate() -> *mut lua_State;
    fn luaL_len(L: *mut lua_State, idx: c_int) -> c_int;
    fn luaL_gsub(L: *mut lua_State, s: *c_schar, p: *c_schar, r: *c_schar) ->
     *c_schar;
    fn luaL_setfuncs(L: *mut lua_State, l: *luaL_Reg, nup: c_int);
    fn luaL_getsubtable(L: *mut lua_State, idx: c_int, fname: *c_schar) ->
     c_int;
    fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *c_schar,
                      level: c_int);
    fn luaL_requiref(L: *mut lua_State, modname: *c_schar,
                     openf: lua_CFunction, glb: c_int);
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut c_schar;
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *c_schar, l: size_t);
    fn luaL_addstring(B: *mut luaL_Buffer, s: *c_schar);
    fn luaL_addvalue(B: *mut luaL_Buffer);
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t);
    fn luaL_buffinitsize(L: *mut lua_State, B: *mut luaL_Buffer, sz: size_t)
     -> *mut c_schar;
}
