/* automatically generated by rust-bindgen */

// Generated with: bindgen 0.52.0

pub const LUA_LDIR: &'static [u8; 7usize] = b"!\\lua\\\0";
pub const LUA_CDIR: &'static [u8; 3usize] = b"!\\\0";
pub const LUA_PATH_DEFAULT: &'static [u8; 38usize] =
    b".\\?.lua;!\\lua\\?.lua;!\\lua\\?\\init.lua;\0";
pub const LUA_CPATH_DEFAULT: &'static [u8; 30usize] = b".\\?.dll;!\\?.dll;!\\loadall.dll\0";
pub const LUA_PATH: &'static [u8; 9usize] = b"LUA_PATH\0";
pub const LUA_CPATH: &'static [u8; 10usize] = b"LUA_CPATH\0";
pub const LUA_INIT: &'static [u8; 9usize] = b"LUA_INIT\0";
pub const LUA_DIRSEP: &'static [u8; 2usize] = b"\\\0";
pub const LUA_PATHSEP: &'static [u8; 2usize] = b";\0";
pub const LUA_PATH_MARK: &'static [u8; 2usize] = b"?\0";
pub const LUA_EXECDIR: &'static [u8; 2usize] = b"!\0";
pub const LUA_IGMARK: &'static [u8; 2usize] = b"-\0";
pub const LUA_PATH_CONFIG: &'static [u8; 11usize] = b"\\\n;\n?\n!\n-\n\0";
pub const LUAI_MAXSTACK: u32 = 65500;
pub const LUAI_MAXCSTACK: u32 = 8000;
pub const LUAI_GCPAUSE: u32 = 200;
pub const LUAI_GCMUL: u32 = 200;
pub const LUA_MAXCAPTURES: u32 = 32;
pub const LUA_IDSIZE: u32 = 60;
pub const LUA_NUMBER_SCAN: &'static [u8; 4usize] = b"%lf\0";
pub const LUA_NUMBER_FMT: &'static [u8; 6usize] = b"%.14g\0";
pub const LUAI_MAXNUMBER2STR: u32 = 32;
pub const LUA_INTFRMLEN: &'static [u8; 2usize] = b"l\0";
pub const LUA_VERSION: &'static [u8; 8usize] = b"Lua 5.1\0";
pub const LUA_RELEASE: &'static [u8; 10usize] = b"Lua 5.1.4\0";
pub const LUA_VERSION_NUM: u32 = 501;
pub const LUA_COPYRIGHT: &'static [u8; 41usize] = b"Copyright (C) 1994-2008 Lua.org, PUC-Rio\0";
pub const LUA_AUTHORS: &'static [u8; 49usize] =
    b"R. Ierusalimschy, L. H. de Figueiredo & W. Celes\0";
pub const LUA_SIGNATURE: &'static [u8; 5usize] = b"\x1BLua\0";
pub const LUA_MULTRET: i32 = -1;
pub const LUA_REGISTRYINDEX: i32 = -10000;
pub const LUA_ENVIRONINDEX: i32 = -10001;
pub const LUA_GLOBALSINDEX: i32 = -10002;
pub const LUA_OK: u32 = 0;
pub const LUA_YIELD: u32 = 1;
pub const LUA_ERRRUN: u32 = 2;
pub const LUA_ERRSYNTAX: u32 = 3;
pub const LUA_ERRMEM: u32 = 4;
pub const LUA_ERRERR: u32 = 5;
pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: u32 = 0;
pub const LUA_TBOOLEAN: u32 = 1;
pub const LUA_TLIGHTUSERDATA: u32 = 2;
pub const LUA_TNUMBER: u32 = 3;
pub const LUA_TSTRING: u32 = 4;
pub const LUA_TTABLE: u32 = 5;
pub const LUA_TFUNCTION: u32 = 6;
pub const LUA_TUSERDATA: u32 = 7;
pub const LUA_TTHREAD: u32 = 8;
pub const LUA_MINSTACK: u32 = 20;
pub const LUA_GCSTOP: u32 = 0;
pub const LUA_GCRESTART: u32 = 1;
pub const LUA_GCCOLLECT: u32 = 2;
pub const LUA_GCCOUNT: u32 = 3;
pub const LUA_GCCOUNTB: u32 = 4;
pub const LUA_GCSTEP: u32 = 5;
pub const LUA_GCSETPAUSE: u32 = 6;
pub const LUA_GCSETSTEPMUL: u32 = 7;
pub const LUA_GCISRUNNING: u32 = 9;
pub const LUA_HOOKCALL: u32 = 0;
pub const LUA_HOOKRET: u32 = 1;
pub const LUA_HOOKLINE: u32 = 2;
pub const LUA_HOOKCOUNT: u32 = 3;
pub const LUA_HOOKTAILRET: u32 = 4;
pub const LUA_MASKCALL: u32 = 1;
pub const LUA_MASKRET: u32 = 2;
pub const LUA_MASKLINE: u32 = 4;
pub const LUA_MASKCOUNT: u32 = 8;
pub const LUA_FILEHANDLE: &'static [u8; 6usize] = b"FILE*\0";
pub const LUA_COLIBNAME: &'static [u8; 10usize] = b"coroutine\0";
pub const LUA_MATHLIBNAME: &'static [u8; 5usize] = b"math\0";
pub const LUA_STRLIBNAME: &'static [u8; 7usize] = b"string\0";
pub const LUA_TABLIBNAME: &'static [u8; 6usize] = b"table\0";
pub const LUA_IOLIBNAME: &'static [u8; 3usize] = b"io\0";
pub const LUA_OSLIBNAME: &'static [u8; 3usize] = b"os\0";
pub const LUA_LOADLIBNAME: &'static [u8; 8usize] = b"package\0";
pub const LUA_DBLIBNAME: &'static [u8; 6usize] = b"debug\0";
pub const LUA_BITLIBNAME: &'static [u8; 4usize] = b"bit\0";
pub const LUA_JITLIBNAME: &'static [u8; 4usize] = b"jit\0";
pub const LUA_FFILIBNAME: &'static [u8; 4usize] = b"ffi\0";
pub const LUA_ERRFILE: u32 = 6;
pub const LUA_NOREF: i32 = -2;
pub const LUA_REFNIL: i32 = -1;
pub const LUAJIT_VERSION: &'static [u8; 19usize] = b"LuaJIT 2.1.0-beta3\0";
pub const LUAJIT_VERSION_NUM: u32 = 20100;
pub const LUAJIT_COPYRIGHT: &'static [u8; 34usize] = b"Copyright (C) 2005-2017 Mike Pall\0";
pub const LUAJIT_URL: &'static [u8; 19usize] = b"http://luajit.org/\0";
pub const LUAJIT_MODE_MASK: u32 = 255;
pub const LUAJIT_MODE_OFF: u32 = 0;
pub const LUAJIT_MODE_ON: u32 = 256;
pub const LUAJIT_MODE_FLUSH: u32 = 512;
pub type va_list = __builtin_va_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// <https://www.lua.org/manual/5.1/manual.html#lua_State >
pub struct lua_State {
    _unused: [u8; 0],
}
/// <https://www.lua.org/manual/5.1/manual.html#lua_CFunction >
pub type lua_CFunction =
    ::core::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> libc::c_int>;
/// <https://www.lua.org/manual/5.1/manual.html#lua_Reader >
pub type lua_Reader = ::core::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        ud: *mut libc::c_void,
        sz: *mut usize,
    ) -> *const libc::c_char,
>;
/// <https://www.lua.org/manual/5.1/manual.html#lua_Writer >
pub type lua_Writer = ::core::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const libc::c_void,
        sz: usize,
        ud: *mut libc::c_void,
    ) -> libc::c_int,
>;
/// <https://www.lua.org/manual/5.1/manual.html#lua_Alloc >
pub type lua_Alloc = ::core::option::Option<
    unsafe extern "C" fn(
        ud: *mut libc::c_void,
        ptr: *mut libc::c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut libc::c_void,
>;
/// <https://www.lua.org/manual/5.1/manual.html#lua_Number >
pub type lua_Number = f64;
/// <https://www.lua.org/manual/5.1/manual.html#lua_Integer >
pub type lua_Integer = isize;
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_newstate>
    pub fn lua_newstate(f: lua_Alloc, ud: *mut libc::c_void) -> *mut lua_State;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_close>
    pub fn lua_close(L: *mut lua_State);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_newthread>
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_atpanic>
    pub fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gettop>
    pub fn lua_gettop(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_settop>
    pub fn lua_settop(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushvalue>
    pub fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_remove>
    pub fn lua_remove(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_insert>
    pub fn lua_insert(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_replace>
    pub fn lua_replace(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_checkstack>
    pub fn lua_checkstack(L: *mut lua_State, sz: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_xmove>
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_isnumber>
    pub fn lua_isnumber(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_isstring>
    pub fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_iscfunction>
    pub fn lua_iscfunction(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_isuserdata>
    pub fn lua_isuserdata(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_type>
    pub fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_typename>
    pub fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_equal>
    pub fn lua_equal(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_rawequal>
    pub fn lua_rawequal(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_lessthan>
    pub fn lua_lessthan(L: *mut lua_State, idx1: libc::c_int, idx2: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tonumber>
    pub fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tointeger>
    pub fn lua_tointeger(L: *mut lua_State, idx: libc::c_int) -> lua_Integer;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_toboolean>
    pub fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tolstring>
    pub fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut usize,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_objlen>
    pub fn lua_objlen(L: *mut lua_State, idx: libc::c_int) -> usize;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tocfunction>
    pub fn lua_tocfunction(L: *mut lua_State, idx: libc::c_int) -> lua_CFunction;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_touserdata>
    pub fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tothread>
    pub fn lua_tothread(L: *mut lua_State, idx: libc::c_int) -> *mut lua_State;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_topointer>
    pub fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushnil>
    pub fn lua_pushnil(L: *mut lua_State);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushnumber>
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushinteger>
    pub fn lua_pushinteger(L: *mut lua_State, n: lua_Integer);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushlstring>
    pub fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: usize);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushstring>
    pub fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushvfstring>
    pub fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: va_list,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushfstring>
    pub fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...)
        -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure>
    pub fn lua_pushcclosure(L: *mut lua_State, fn_: lua_CFunction, n: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushboolean>
    pub fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushlightuserdata>
    pub fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pushthread>
    pub fn lua_pushthread(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gettable>
    pub fn lua_gettable(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getfield>
    pub fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_rawget>
    pub fn lua_rawget(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_rawgeti>
    pub fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_createtable>
    pub fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_newuserdata>
    pub fn lua_newuserdata(L: *mut lua_State, sz: usize) -> *mut libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getmetatable>
    pub fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getfenv>
    pub fn lua_getfenv(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_settable>
    pub fn lua_settable(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setfield>
    pub fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_rawset>
    pub fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_rawseti>
    pub fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setmetatable>
    pub fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setfenv>
    pub fn lua_setfenv(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_call>
    pub fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_pcall>
    pub fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_cpcall>
    pub fn lua_cpcall(L: *mut lua_State, func: lua_CFunction, ud: *mut libc::c_void)
        -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_load>
    pub fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_dump>
    pub fn lua_dump(L: *mut lua_State, writer: lua_Writer, data: *mut libc::c_void) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_yield>
    pub fn lua_yield(L: *mut lua_State, nresults: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_resume>
    pub fn lua_resume(L: *mut lua_State, narg: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_status>
    pub fn lua_status(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gc>
    pub fn lua_gc(L: *mut lua_State, what: libc::c_int, data: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_error>
    pub fn lua_error(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_next>
    pub fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_concat>
    pub fn lua_concat(L: *mut lua_State, n: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getallocf>
    pub fn lua_getallocf(L: *mut lua_State, ud: *mut *mut libc::c_void) -> lua_Alloc;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setallocf>
    pub fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut libc::c_void);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setlevel>
    pub fn lua_setlevel(from: *mut lua_State, to: *mut lua_State);
}
/// <https://www.lua.org/manual/5.1/manual.html#lua_Hook >
pub type lua_Hook =
    ::core::option::Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getstack>
    pub fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getinfo>
    pub fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getlocal>
    pub fn lua_getlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setlocal>
    pub fn lua_setlocal(
        L: *mut lua_State,
        ar: *const lua_Debug,
        n: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_getupvalue>
    pub fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_setupvalue>
    pub fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_sethook>
    pub fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gethook>
    pub fn lua_gethook(L: *mut lua_State) -> lua_Hook;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gethookmask>
    pub fn lua_gethookmask(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_gethookcount>
    pub fn lua_gethookcount(L: *mut lua_State) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_upvalueid>
    pub fn lua_upvalueid(L: *mut lua_State, idx: libc::c_int, n: libc::c_int) -> *mut libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_upvaluejoin>
    pub fn lua_upvaluejoin(
        L: *mut lua_State,
        idx1: libc::c_int,
        n1: libc::c_int,
        idx2: libc::c_int,
        n2: libc::c_int,
    );
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_loadx>
    pub fn lua_loadx(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_version>
    pub fn lua_version(L: *mut lua_State) -> *const lua_Number;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_copy>
    pub fn lua_copy(L: *mut lua_State, fromidx: libc::c_int, toidx: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tonumberx>
    pub fn lua_tonumberx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Number;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_tointegerx>
    pub fn lua_tointegerx(
        L: *mut lua_State,
        idx: libc::c_int,
        isnum: *mut libc::c_int,
    ) -> lua_Integer;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#lua_isyieldable>
    pub fn lua_isyieldable(L: *mut lua_State) -> libc::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
/// <https://www.lua.org/manual/5.1/manual.html#lua_Debug >
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub short_src: [libc::c_char; 60usize],
    pub i_ci: libc::c_int,
}
#[test]
fn bindgen_test_layout_lua_Debug() {
    assert_eq!(
        ::core::mem::size_of::<lua_Debug>(),
        120usize,
        concat!("Size of: ", stringify!(lua_Debug))
    );
    assert_eq!(
        ::core::mem::align_of::<lua_Debug>(),
        8usize,
        concat!("Alignment of ", stringify!(lua_Debug))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).event as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).namewhat as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(namewhat)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).what as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(what)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).source as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).currentline as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(currentline)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).nups as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(nups)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).linedefined as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(linedefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).lastlinedefined as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(lastlinedefined)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).short_src as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(short_src)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<lua_Debug>())).i_ci as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(i_ci)
        )
    );
}
impl ::core::fmt::Debug for lua_Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write ! ( f , "lua_Debug {{ event: {:?}, name: {:?}, namewhat: {:?}, what: {:?}, source: {:?}, currentline: {:?}, nups: {:?}, linedefined: {:?}, lastlinedefined: {:?}, short_src: [...], i_ci: {:?} }}" , self . event , self . name , self . namewhat , self . what , self . source , self . currentline , self . nups , self . linedefined , self . lastlinedefined , self . i_ci )
    }
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_openlibs>
    pub fn luaL_openlibs(L: *mut lua_State);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// <https://www.lua.org/manual/5.1/manual.html#luaL_Reg >
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[test]
fn bindgen_test_layout_luaL_Reg() {
    assert_eq!(
        ::core::mem::size_of::<luaL_Reg>(),
        16usize,
        concat!("Size of: ", stringify!(luaL_Reg))
    );
    assert_eq!(
        ::core::mem::align_of::<luaL_Reg>(),
        8usize,
        concat!("Alignment of ", stringify!(luaL_Reg))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Reg>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Reg),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Reg>())).func as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Reg),
            "::",
            stringify!(func)
        )
    );
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_openlib>
    pub fn luaL_openlib(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
        nup: libc::c_int,
    );
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_register>
    pub fn luaL_register(L: *mut lua_State, libname: *const libc::c_char, l: *const luaL_Reg);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_getmetafield>
    pub fn luaL_getmetafield(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_callmeta>
    pub fn luaL_callmeta(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_typerror>
    pub fn luaL_typerror(
        L: *mut lua_State,
        narg: libc::c_int,
        tname: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_argerror>
    pub fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checklstring>
    pub fn luaL_checklstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        l: *mut usize,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_optlstring>
    pub fn luaL_optlstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        def: *const libc::c_char,
        l: *mut usize,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checknumber>
    pub fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_optnumber>
    pub fn luaL_optnumber(L: *mut lua_State, nArg: libc::c_int, def: lua_Number) -> lua_Number;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checkinteger>
    pub fn luaL_checkinteger(L: *mut lua_State, numArg: libc::c_int) -> lua_Integer;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_optinteger>
    pub fn luaL_optinteger(L: *mut lua_State, nArg: libc::c_int, def: lua_Integer) -> lua_Integer;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checkstack>
    pub fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checktype>
    pub fn luaL_checktype(L: *mut lua_State, narg: libc::c_int, t: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checkany>
    pub fn luaL_checkany(L: *mut lua_State, narg: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_newmetatable>
    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checkudata>
    pub fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_where>
    pub fn luaL_where(L: *mut lua_State, lvl: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_error>
    pub fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_checkoption>
    pub fn luaL_checkoption(
        L: *mut lua_State,
        narg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_ref>
    pub fn luaL_ref(L: *mut lua_State, t: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_unref>
    pub fn luaL_unref(L: *mut lua_State, t: libc::c_int, ref_: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_loadfile>
    pub fn luaL_loadfile(L: *mut lua_State, filename: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_loadbuffer>
    pub fn luaL_loadbuffer(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: usize,
        name: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_loadstring>
    pub fn luaL_loadstring(L: *mut lua_State, s: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_newstate>
    pub fn luaL_newstate() -> *mut lua_State;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_gsub>
    pub fn luaL_gsub(
        L: *mut lua_State,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_findtable>
    pub fn luaL_findtable(
        L: *mut lua_State,
        idx: libc::c_int,
        fname: *const libc::c_char,
        szhint: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_fileresult>
    pub fn luaL_fileresult(
        L: *mut lua_State,
        stat: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_execresult>
    pub fn luaL_execresult(L: *mut lua_State, stat: libc::c_int) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_loadfilex>
    pub fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_loadbufferx>
    pub fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: usize,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_traceback>
    pub fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: libc::c_int,
    );
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_setfuncs>
    pub fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_pushmodule>
    pub fn luaL_pushmodule(L: *mut lua_State, modname: *const libc::c_char, sizehint: libc::c_int);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_testudata>
    pub fn luaL_testudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_setmetatable>
    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const libc::c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
/// <https://www.lua.org/manual/5.1/manual.html#luaL_Buffer >
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub lvl: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 512usize],
}
#[test]
fn bindgen_test_layout_luaL_Buffer() {
    assert_eq!(
        ::core::mem::size_of::<luaL_Buffer>(),
        536usize,
        concat!("Size of: ", stringify!(luaL_Buffer))
    );
    assert_eq!(
        ::core::mem::align_of::<luaL_Buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(luaL_Buffer))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Buffer>())).p as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Buffer>())).lvl as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(lvl)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Buffer>())).L as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(L)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<luaL_Buffer>())).buffer as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(buffer)
        )
    );
}
impl ::core::fmt::Debug for luaL_Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(
            f,
            "luaL_Buffer {{ p: {:?}, lvl: {:?}, L: {:?}, buffer: [...] }}",
            self.p, self.lvl, self.L
        )
    }
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_buffinit>
    pub fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_prepbuffer>
    pub fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char;
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_addlstring>
    pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const libc::c_char, l: usize);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_addstring>
    pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const libc::c_char);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_addvalue>
    pub fn luaL_addvalue(B: *mut luaL_Buffer);
}
extern "C" {
    /// <https://www.lua.org/manual/5.1/manual.html#luaL_pushresult>
    pub fn luaL_pushresult(B: *mut luaL_Buffer);
}
pub const LUAJIT_MODE_ENGINE: _bindgen_ty_1 = 0;
pub const LUAJIT_MODE_DEBUG: _bindgen_ty_1 = 1;
pub const LUAJIT_MODE_FUNC: _bindgen_ty_1 = 2;
pub const LUAJIT_MODE_ALLFUNC: _bindgen_ty_1 = 3;
pub const LUAJIT_MODE_ALLSUBFUNC: _bindgen_ty_1 = 4;
pub const LUAJIT_MODE_TRACE: _bindgen_ty_1 = 5;
pub const LUAJIT_MODE_WRAPCFUNC: _bindgen_ty_1 = 16;
pub const LUAJIT_MODE_MAX: _bindgen_ty_1 = 17;
pub type _bindgen_ty_1 = i32;
extern "C" {
    /// <https://luajit.org/ext_c_api.html>
    pub fn luaJIT_setmode(L: *mut lua_State, idx: libc::c_int, mode: libc::c_int) -> libc::c_int;
}
pub type luaJIT_profile_callback = ::core::option::Option<
    unsafe extern "C" fn(
        data: *mut libc::c_void,
        L: *mut lua_State,
        samples: libc::c_int,
        vmstate: libc::c_int,
    ),
>;
extern "C" {
    /// <https://luajit.org/ext_c_api.html>
    pub fn luaJIT_profile_start(
        L: *mut lua_State,
        mode: *const libc::c_char,
        cb: luaJIT_profile_callback,
        data: *mut libc::c_void,
    );
}
extern "C" {
    /// <https://luajit.org/ext_c_api.html>
    pub fn luaJIT_profile_stop(L: *mut lua_State);
}
extern "C" {
    /// <https://luajit.org/ext_c_api.html>
    pub fn luaJIT_profile_dumpstack(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        depth: libc::c_int,
        len: *mut usize,
    ) -> *const libc::c_char;
}
extern "C" {
    /// <https://luajit.org/ext_c_api.html>
    pub fn luaJIT_version_2_1_0_beta3();
}
pub type __builtin_va_list = *mut libc::c_char;
