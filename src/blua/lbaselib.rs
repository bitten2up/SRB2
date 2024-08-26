use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    static mut stdout: *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_insert(L: *mut lua_State, idx: libc::c_int);
    fn lua_replace(L: *mut lua_State, idx: libc::c_int);
    fn lua_checkstack(L: *mut lua_State, sz: libc::c_int) -> libc::c_int;
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int);
    fn lua_isnumber(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_iscfunction(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    fn lua_rawequal(
        L: *mut lua_State,
        idx1: libc::c_int,
        idx2: libc::c_int,
    ) -> libc::c_int;
    fn lua_tonumber(L: *mut lua_State, idx: libc::c_int) -> lua_Number;
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_objlen(L: *mut lua_State, idx: libc::c_int) -> size_t;
    fn lua_tothread(L: *mut lua_State, idx: libc::c_int) -> *mut lua_State;
    fn lua_topointer(L: *mut lua_State, idx: libc::c_int) -> *const libc::c_void;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushlstring(L: *mut lua_State, s: *const libc::c_char, l: size_t);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        _: ...
    ) -> *const libc::c_char;
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_pushthread(L: *mut lua_State) -> libc::c_int;
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawget(L: *mut lua_State, idx: libc::c_int);
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_getfenv(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_setfenv(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_call(L: *mut lua_State, nargs: libc::c_int, nresults: libc::c_int);
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_yield(L: *mut lua_State, nresults: libc::c_int) -> libc::c_int;
    fn lua_resume(L: *mut lua_State, narg: libc::c_int) -> libc::c_int;
    fn lua_status(L: *mut lua_State) -> libc::c_int;
    fn lua_gc(L: *mut lua_State, what: libc::c_int, data: libc::c_int) -> libc::c_int;
    fn lua_error(L: *mut lua_State) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_concat(L: *mut lua_State, n: libc::c_int);
    fn lua_setlevel(from: *mut lua_State, to: *mut lua_State);
    fn lua_getstack(
        L: *mut lua_State,
        level: libc::c_int,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn lua_getinfo(
        L: *mut lua_State,
        what: *const libc::c_char,
        ar: *mut lua_Debug,
    ) -> libc::c_int;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_where(L: *mut lua_State, lvl: libc::c_int);
    fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn luaL_checktype(L: *mut lua_State, narg: libc::c_int, t: libc::c_int);
    fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
    fn luaL_checkany(L: *mut lua_State, narg: libc::c_int);
    fn luaL_optlstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_optnumber(
        L: *mut lua_State,
        nArg: libc::c_int,
        def: lua_Number,
    ) -> lua_Number;
    fn luaL_callmeta(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_getmetafield(
        L: *mut lua_State,
        obj: libc::c_int,
        e: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checkoption(
        L: *mut lua_State,
        narg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    fn LUA_LoadLump(wad: uint16_t, lump: uint16_t, noresults: boolean);
    static mut numwadfiles: uint16_t;
    static mut wadfiles: *mut *mut wadfile_t;
    fn W_CheckNumForFullNamePK3(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const true_0: C2RustUnnamed_0 = 1;
pub const false_0: C2RustUnnamed_0 = 0;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
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
    pub short_src: [libc::c_char; 60],
    pub i_ci: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
pub const RET_FOLDER: restype = 4;
pub type restype_t = restype;
pub type restype = libc::c_uint;
pub const RET_UNKNOWN: restype = 5;
pub const RET_PK3: restype = 3;
pub const RET_LUA: restype = 2;
pub const RET_SOC: restype = 1;
pub const RET_WAD: restype = 0;
pub type wadfile_t = wadfile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wadfile_s {
    pub filename: *mut libc::c_char,
    pub path: *mut libc::c_char,
    pub type_0: restype_t,
    pub lumpinfo: *mut lumpinfo_t,
    pub lumpcache: *mut *mut libc::c_void,
    pub patchcache: *mut *mut libc::c_void,
    pub numlumps: uint16_t,
    pub foldercount: uint16_t,
    pub handle: *mut FILE,
    pub filesize: uint32_t,
    pub md5sum: [uint8_t; 16],
    pub important: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lumpinfo_t {
    pub position: libc::c_ulong,
    pub disksize: libc::c_ulong,
    pub name: [libc::c_char; 9],
    pub hash: uint32_t,
    pub longname: *mut libc::c_char,
    pub fullname: *mut libc::c_char,
    pub diskpath: *mut libc::c_char,
    pub size: size_t,
    pub compression: compmethod,
}
pub type compmethod = libc::c_uint;
pub const CM_UNSUPPORTED: compmethod = 3;
pub const CM_LZF: compmethod = 2;
pub const CM_DEFLATE: compmethod = 1;
pub const CM_NOCOMPRESSION: compmethod = 0;
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    let mut i: libc::c_int = 0;
    lua_getfield(
        L,
        -(10002 as libc::c_int),
        b"tostring\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i <= n {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_pushvalue(L, i);
        lua_call(L, 1 as libc::c_int, 1 as libc::c_int);
        s = lua_tolstring(L, -(1 as libc::c_int), 0 as *mut size_t);
        if s.is_null() {
            return luaL_error(
                L,
                b"'tostring' must return a string to 'print'\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if i > 1 as libc::c_int {
            fputs(b"\t\0" as *const u8 as *const libc::c_char, stdout);
        }
        fputs(s, stdout);
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        i += 1;
        i;
    }
    fputs(b"\n\0" as *const u8 as *const libc::c_char, stdout);
    return 0 as libc::c_int;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> libc::c_int {
    let mut base: libc::c_int = luaL_optnumber(L, 2 as libc::c_int, 10 as libc::c_int);
    if base == 10 as libc::c_int {
        luaL_checkany(L, 1 as libc::c_int);
        if lua_isnumber(L, 1 as libc::c_int) != 0 {
            lua_pushnumber(L, lua_tonumber(L, 1 as libc::c_int));
            return 1 as libc::c_int;
        }
    } else {
        let mut s1: *const libc::c_char = luaL_checklstring(
            L,
            1 as libc::c_int,
            0 as *mut size_t,
        );
        let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut n: libc::c_ulong = 0;
        (2 as libc::c_int <= base && base <= 36 as libc::c_int
            || luaL_argerror(
                L,
                2 as libc::c_int,
                b"base out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        n = strtoul(s1, &mut s2, base);
        if s1 != s2 as *const libc::c_char {
            while *(*__ctype_b_loc())
                .offset(*s2 as libc::c_uchar as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                s2 = s2.offset(1);
                s2;
            }
            if *s2 as libc::c_int == '\0' as i32 {
                lua_pushnumber(L, n as lua_Number);
                return 1 as libc::c_int;
            }
        }
    }
    lua_pushnil(L);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> libc::c_int {
    let mut level: libc::c_int = luaL_optnumber(L, 2 as libc::c_int, 1 as libc::c_int);
    lua_settop(L, 1 as libc::c_int);
    if lua_isstring(L, 1 as libc::c_int) != 0 && level > 0 as libc::c_int {
        luaL_where(L, level);
        lua_pushvalue(L, 1 as libc::c_int);
        lua_concat(L, 2 as libc::c_int);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_getmetatable(L, 1 as libc::c_int) == 0 {
        lua_pushnil(L);
        return 1 as libc::c_int;
    }
    luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    (t == 0 as libc::c_int || t == 5 as libc::c_int
        || luaL_argerror(
            L,
            2 as libc::c_int,
            b"nil or table expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    if luaL_getmetafield(
        L,
        1 as libc::c_int,
        b"__metatable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        luaL_error(
            L,
            b"cannot change a protected metatable\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_settop(L, 2 as libc::c_int);
    lua_setmetatable(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn getfunc(mut L: *mut lua_State, mut opt: libc::c_int) {
    if lua_type(L, 1 as libc::c_int) == 6 as libc::c_int {
        lua_pushvalue(L, 1 as libc::c_int);
    } else {
        let mut ar: lua_Debug = lua_Debug {
            event: 0,
            name: 0 as *const libc::c_char,
            namewhat: 0 as *const libc::c_char,
            what: 0 as *const libc::c_char,
            source: 0 as *const libc::c_char,
            currentline: 0,
            nups: 0,
            linedefined: 0,
            lastlinedefined: 0,
            short_src: [0; 60],
            i_ci: 0,
        };
        let mut level: libc::c_int = if opt != 0 {
            luaL_optnumber(L, 1 as libc::c_int, 1 as libc::c_int)
        } else {
            luaL_checknumber(L, 1 as libc::c_int)
        };
        (level >= 0 as libc::c_int
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"level must be non-negative\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        if lua_getstack(L, level, &mut ar) == 0 as libc::c_int {
            luaL_argerror(
                L,
                1 as libc::c_int,
                b"invalid level\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_getinfo(L, b"f\0" as *const u8 as *const libc::c_char, &mut ar);
        if lua_type(L, -(1 as libc::c_int)) == 0 as libc::c_int {
            luaL_error(
                L,
                b"no function environment for tail call at level %d\0" as *const u8
                    as *const libc::c_char,
                level,
            );
        }
    };
}
unsafe extern "C" fn luaB_getfenv(mut L: *mut lua_State) -> libc::c_int {
    getfunc(L, 1 as libc::c_int);
    if lua_iscfunction(L, -(1 as libc::c_int)) != 0 {
        lua_pushvalue(L, -(10002 as libc::c_int));
    } else {
        lua_getfenv(L, -(1 as libc::c_int));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_setfenv(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 2 as libc::c_int, 5 as libc::c_int);
    getfunc(L, 0 as libc::c_int);
    lua_pushvalue(L, 2 as libc::c_int);
    if lua_isnumber(L, 1 as libc::c_int) != 0
        && lua_tonumber(L, 1 as libc::c_int) == 0 as libc::c_int
    {
        lua_pushthread(L);
        lua_insert(L, -(2 as libc::c_int));
        lua_setfenv(L, -(2 as libc::c_int));
        return 0 as libc::c_int;
    } else if lua_iscfunction(L, -(2 as libc::c_int)) != 0
        || lua_setfenv(L, -(2 as libc::c_int)) == 0 as libc::c_int
    {
        luaL_error(
            L,
            b"'setfenv' cannot change environment of given object\0" as *const u8
                as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_pushboolean(L, lua_rawequal(L, 1 as libc::c_int, 2 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_rawget(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    luaL_checkany(L, 2 as libc::c_int);
    luaL_checkany(L, 3 as libc::c_int);
    lua_settop(L, 3 as libc::c_int);
    lua_rawset(L, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_gcinfo(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, lua_gc(L, 3 as libc::c_int, 0 as libc::c_int));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> libc::c_int {
    static mut opts: [*const libc::c_char; 8] = [
        b"stop\0" as *const u8 as *const libc::c_char,
        b"restart\0" as *const u8 as *const libc::c_char,
        b"collect\0" as *const u8 as *const libc::c_char,
        b"count\0" as *const u8 as *const libc::c_char,
        b"step\0" as *const u8 as *const libc::c_char,
        b"setpause\0" as *const u8 as *const libc::c_char,
        b"setstepmul\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut optsnum: [libc::c_int; 7] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
    ];
    let mut o: libc::c_int = luaL_checkoption(
        L,
        1 as libc::c_int,
        b"collect\0" as *const u8 as *const libc::c_char,
        opts.as_ptr(),
    );
    let mut ex: libc::c_int = luaL_optnumber(L, 2 as libc::c_int, 0 as libc::c_int);
    let mut res: libc::c_int = lua_gc(L, optsnum[o as usize], ex);
    match optsnum[o as usize] {
        3 => {
            let mut b: libc::c_int = lua_gc(L, 4 as libc::c_int, 0 as libc::c_int);
            lua_pushnumber(L, res + b / 1024 as libc::c_int);
            return 1 as libc::c_int;
        }
        5 => {
            lua_pushboolean(L, res);
            return 1 as libc::c_int;
        }
        _ => {
            lua_pushnumber(L, res);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    lua_pushstring(L, lua_typename(L, lua_type(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    if lua_next(L, 1 as libc::c_int) != 0 {
        return 2 as libc::c_int
    } else {
        lua_pushnil(L);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    lua_pushvalue(L, -(10002 as libc::c_int) - 1 as libc::c_int);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushnil(L);
    return 3 as libc::c_int;
}
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 2 as libc::c_int);
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    i += 1;
    i;
    lua_pushnumber(L, i);
    lua_rawgeti(L, 1 as libc::c_int, i);
    return if lua_type(L, -(1 as libc::c_int)) == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    lua_pushvalue(L, -(10002 as libc::c_int) - 1 as libc::c_int);
    lua_pushvalue(L, 1 as libc::c_int);
    lua_pushnumber(L, 0 as libc::c_int);
    return 3 as libc::c_int;
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut fullfilename: [libc::c_char; 256] = [0; 256];
    let mut lumpnum: uint16_t = 0;
    let mut n: libc::c_int = lua_gettop(L);
    if !((**wadfiles.offset((numwadfiles as libc::c_int - 1 as libc::c_int) as isize))
        .type_0 as libc::c_uint == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset((numwadfiles as libc::c_int - 1 as libc::c_int) as isize))
            .type_0 as libc::c_uint == RET_FOLDER as libc::c_int as libc::c_uint)
    {
        luaL_error(
            L,
            b"dofile() only works with PK3 files\0" as *const u8 as *const libc::c_char,
        );
    }
    snprintf(
        fullfilename.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"Lua/%s\0" as *const u8 as *const libc::c_char,
        filename,
    );
    lumpnum = W_CheckNumForFullNamePK3(
        fullfilename.as_mut_ptr(),
        (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
        0 as libc::c_int as uint16_t,
    );
    if lumpnum as libc::c_int == 32767 as libc::c_int {
        luaL_error(
            L,
            b"can't find script '%s'\0" as *const u8 as *const libc::c_char,
            fullfilename.as_mut_ptr(),
        );
    }
    LUA_LoadLump(
        (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
        lumpnum,
        false_0 as libc::c_int,
    );
    return lua_gettop(L) - n;
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if lua_toboolean(L, 1 as libc::c_int) == 0 {
        return luaL_error(
            L,
            b"%s\0" as *const u8 as *const libc::c_char,
            luaL_optlstring(
                L,
                2 as libc::c_int,
                b"assertion failed!\0" as *const u8 as *const libc::c_char,
                0 as *mut size_t,
            ),
        );
    }
    return lua_gettop(L);
}
unsafe extern "C" fn luaB_unpack(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    luaL_checktype(L, 1 as libc::c_int, 5 as libc::c_int);
    i = luaL_optnumber(L, 2 as libc::c_int, 1 as libc::c_int);
    e = if lua_type(L, 3 as libc::c_int) <= 0 as libc::c_int {
        lua_objlen(L, 1 as libc::c_int) as libc::c_int
    } else {
        luaL_checknumber(L, 3 as libc::c_int)
    };
    if i > e {
        return 0 as libc::c_int;
    }
    n = e - i + 1 as libc::c_int;
    if n <= 0 as libc::c_int || lua_checkstack(L, n) == 0 {
        return luaL_error(
            L,
            b"too many results to unpack\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_rawgeti(L, 1 as libc::c_int, i);
    loop {
        let fresh0 = i;
        i = i + 1;
        if !(fresh0 < e) {
            break;
        }
        lua_rawgeti(L, 1 as libc::c_int, i);
    }
    return n;
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = lua_gettop(L);
    if lua_type(L, 1 as libc::c_int) == 4 as libc::c_int
        && *lua_tolstring(L, 1 as libc::c_int, 0 as *mut size_t) as libc::c_int
            == '#' as i32
    {
        lua_pushnumber(L, n - 1 as libc::c_int);
        return 1 as libc::c_int;
    } else {
        let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
        if i < 0 as libc::c_int {
            i = n + i;
        } else if i > n {
            i = n;
        }
        (1 as libc::c_int <= i
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"index out of range\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        return n - i;
    };
}
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    luaL_checkany(L, 1 as libc::c_int);
    status = lua_pcall(
        L,
        lua_gettop(L) - 1 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
    );
    lua_pushboolean(L, (status == 0 as libc::c_int) as libc::c_int);
    lua_insert(L, 1 as libc::c_int);
    return lua_gettop(L);
}
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    luaL_checkany(L, 2 as libc::c_int);
    lua_settop(L, 2 as libc::c_int);
    lua_insert(L, 1 as libc::c_int);
    status = lua_pcall(L, 0 as libc::c_int, -(1 as libc::c_int), 1 as libc::c_int);
    lua_pushboolean(L, (status == 0 as libc::c_int) as libc::c_int);
    lua_replace(L, 1 as libc::c_int);
    return lua_gettop(L);
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1 as libc::c_int);
    if luaL_callmeta(
        L,
        1 as libc::c_int,
        b"__tostring\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        return 1 as libc::c_int;
    }
    match lua_type(L, 1 as libc::c_int) {
        3 => {
            lua_pushstring(L, lua_tolstring(L, 1 as libc::c_int, 0 as *mut size_t));
        }
        4 => {
            lua_pushvalue(L, 1 as libc::c_int);
        }
        1 => {
            lua_pushstring(
                L,
                if lua_toboolean(L, 1 as libc::c_int) != 0 {
                    b"true\0" as *const u8 as *const libc::c_char
                } else {
                    b"false\0" as *const u8 as *const libc::c_char
                },
            );
        }
        0 => {
            lua_pushlstring(
                L,
                b"nil\0" as *const u8 as *const libc::c_char,
                (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    )
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        _ => {
            lua_pushfstring(
                L,
                b"%s: %p\0" as *const u8 as *const libc::c_char,
                lua_typename(L, lua_type(L, 1 as libc::c_int)),
                lua_topointer(L, 1 as libc::c_int),
            );
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_newproxy(mut L: *mut lua_State) -> libc::c_int {
    lua_settop(L, 1 as libc::c_int);
    lua_newuserdata(L, 0 as libc::c_int as size_t);
    if lua_toboolean(L, 1 as libc::c_int) == 0 as libc::c_int {
        return 1 as libc::c_int
    } else if lua_type(L, 1 as libc::c_int) == 1 as libc::c_int {
        lua_createtable(L, 0 as libc::c_int, 0 as libc::c_int);
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_pushboolean(L, 1 as libc::c_int);
        lua_rawset(L, -(10002 as libc::c_int) - 1 as libc::c_int);
    } else {
        let mut validproxy: libc::c_int = 0 as libc::c_int;
        if lua_getmetatable(L, 1 as libc::c_int) != 0 {
            lua_rawget(L, -(10002 as libc::c_int) - 1 as libc::c_int);
            validproxy = lua_toboolean(L, -(1 as libc::c_int));
            lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        }
        (validproxy != 0
            || luaL_argerror(
                L,
                1 as libc::c_int,
                b"boolean or proxy expected\0" as *const u8 as *const libc::c_char,
            ) != 0) as libc::c_int;
        lua_getmetatable(L, 1 as libc::c_int);
    }
    lua_setmetatable(L, 2 as libc::c_int);
    return 1 as libc::c_int;
}
static mut base_funcs: [luaL_Reg; 22] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"assert\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_assert as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"collectgarbage\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_collectgarbage
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"error\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_error as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"dofile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_dofile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gcinfo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_gcinfo as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getfenv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_getfenv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_getmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"next\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"pcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_pcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"print\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_print as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawequal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawequal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawget\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawget as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"rawset\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_rawset as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"select\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_select as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setfenv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_setfenv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setmetatable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_setmetatable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tonumber\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tonumber as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_tostring as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"unpack\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_unpack as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"xpcall\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_xpcall as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
static mut statnames: [*const libc::c_char; 4] = [
    b"running\0" as *const u8 as *const libc::c_char,
    b"suspended\0" as *const u8 as *const libc::c_char,
    b"normal\0" as *const u8 as *const libc::c_char,
    b"dead\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn costatus(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
) -> libc::c_int {
    if L == co {
        return 0 as libc::c_int;
    }
    match lua_status(co) {
        1 => return 1 as libc::c_int,
        0 => {
            let mut ar: lua_Debug = lua_Debug {
                event: 0,
                name: 0 as *const libc::c_char,
                namewhat: 0 as *const libc::c_char,
                what: 0 as *const libc::c_char,
                source: 0 as *const libc::c_char,
                currentline: 0,
                nups: 0,
                linedefined: 0,
                lastlinedefined: 0,
                short_src: [0; 60],
                i_ci: 0,
            };
            if lua_getstack(co, 0 as libc::c_int, &mut ar) > 0 as libc::c_int {
                return 2 as libc::c_int
            } else if lua_gettop(co) == 0 as libc::c_int {
                return 3 as libc::c_int
            } else {
                return 1 as libc::c_int
            }
        }
        _ => return 3 as libc::c_int,
    };
}
unsafe extern "C" fn luaB_costatus(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = lua_tothread(L, 1 as libc::c_int);
    (!co.is_null()
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"coroutine expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushstring(L, statnames[costatus(L, co) as usize]);
    return 1 as libc::c_int;
}
unsafe extern "C" fn auxresume(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
    mut narg: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = costatus(L, co);
    if lua_checkstack(co, narg) == 0 {
        luaL_error(
            L,
            b"too many arguments to resume\0" as *const u8 as *const libc::c_char,
        );
    }
    if status != 1 as libc::c_int {
        lua_pushfstring(
            L,
            b"cannot resume %s coroutine\0" as *const u8 as *const libc::c_char,
            statnames[status as usize],
        );
        return -(1 as libc::c_int);
    }
    lua_xmove(L, co, narg);
    lua_setlevel(L, co);
    status = lua_resume(co, narg);
    if status == 0 as libc::c_int || status == 1 as libc::c_int {
        let mut nres: libc::c_int = lua_gettop(co);
        if lua_checkstack(L, nres + 1 as libc::c_int) == 0 {
            luaL_error(
                L,
                b"too many results to resume\0" as *const u8 as *const libc::c_char,
            );
        }
        lua_xmove(co, L, nres);
        return nres;
    } else {
        lua_xmove(co, L, 1 as libc::c_int);
        return -(1 as libc::c_int);
    };
}
unsafe extern "C" fn luaB_coresume(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = lua_tothread(L, 1 as libc::c_int);
    let mut r: libc::c_int = 0;
    (!co.is_null()
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"coroutine expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    r = auxresume(L, co, lua_gettop(L) - 1 as libc::c_int);
    if r < 0 as libc::c_int {
        lua_pushboolean(L, 0 as libc::c_int);
        lua_insert(L, -(2 as libc::c_int));
        return 2 as libc::c_int;
    } else {
        lua_pushboolean(L, 1 as libc::c_int);
        lua_insert(L, -(r + 1 as libc::c_int));
        return r + 1 as libc::c_int;
    };
}
unsafe extern "C" fn luaB_auxwrap(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = lua_tothread(
        L,
        -(10002 as libc::c_int) - 1 as libc::c_int,
    );
    let mut r: libc::c_int = auxresume(L, co, lua_gettop(L));
    if r < 0 as libc::c_int {
        if lua_isstring(L, -(1 as libc::c_int)) != 0 {
            luaL_where(L, 1 as libc::c_int);
            lua_insert(L, -(2 as libc::c_int));
            lua_concat(L, 2 as libc::c_int);
        }
        lua_error(L);
    }
    return r;
}
unsafe extern "C" fn luaB_cocreate(mut L: *mut lua_State) -> libc::c_int {
    let mut NL: *mut lua_State = lua_newthread(L);
    (lua_type(L, 1 as libc::c_int) == 6 as libc::c_int
        && lua_iscfunction(L, 1 as libc::c_int) == 0
        || luaL_argerror(
            L,
            1 as libc::c_int,
            b"Lua function expected\0" as *const u8 as *const libc::c_char,
        ) != 0) as libc::c_int;
    lua_pushvalue(L, 1 as libc::c_int);
    lua_xmove(L, NL, 1 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_cowrap(mut L: *mut lua_State) -> libc::c_int {
    luaB_cocreate(L);
    lua_pushcclosure(
        L,
        Some(luaB_auxwrap as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        1 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn luaB_yield(mut L: *mut lua_State) -> libc::c_int {
    return lua_yield(L, lua_gettop(L));
}
unsafe extern "C" fn luaB_corunning(mut L: *mut lua_State) -> libc::c_int {
    if lua_pushthread(L) != 0 {
        lua_pushnil(L);
    }
    return 1 as libc::c_int;
}
static mut co_funcs: [luaL_Reg; 7] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"create\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cocreate as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"resume\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_coresume as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"running\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_corunning as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"status\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_costatus as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"wrap\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_cowrap as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"yield\0" as *const u8 as *const libc::c_char,
                func: Some(
                    luaB_yield as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
unsafe extern "C" fn auxopen(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut f: lua_CFunction,
    mut u: lua_CFunction,
) {
    lua_pushcclosure(L, u, 0 as libc::c_int);
    lua_pushcclosure(L, f, 1 as libc::c_int);
    lua_setfield(L, -(2 as libc::c_int), name);
}
unsafe extern "C" fn base_open(mut L: *mut lua_State) {
    lua_pushvalue(L, -(10002 as libc::c_int));
    lua_setfield(
        L,
        -(10002 as libc::c_int),
        b"_G\0" as *const u8 as *const libc::c_char,
    );
    luaL_register(L, b"_G\0" as *const u8 as *const libc::c_char, base_funcs.as_ptr());
    lua_pushlstring(
        L,
        b"Lua 5.1\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(10002 as libc::c_int),
        b"_VERSION\0" as *const u8 as *const libc::c_char,
    );
    auxopen(
        L,
        b"ipairs\0" as *const u8 as *const libc::c_char,
        Some(luaB_ipairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        Some(ipairsaux as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    auxopen(
        L,
        b"pairs\0" as *const u8 as *const libc::c_char,
        Some(luaB_pairs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        Some(luaB_next as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
    );
    lua_createtable(L, 0 as libc::c_int, 1 as libc::c_int);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_pushlstring(
        L,
        b"kv\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 3]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__mode\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(luaB_newproxy as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        1 as libc::c_int,
    );
    lua_setfield(
        L,
        -(10002 as libc::c_int),
        b"newproxy\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> libc::c_int {
    base_open(L);
    luaL_register(
        L,
        b"coroutine\0" as *const u8 as *const libc::c_char,
        co_funcs.as_ptr(),
    );
    return 2 as libc::c_int;
}
