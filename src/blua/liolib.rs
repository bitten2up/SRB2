use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type lua_State;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn tmpfile() -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn clearerr(__stream: *mut FILE);
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_replace(L: *mut lua_State, idx: libc::c_int);
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
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
    fn lua_tocfunction(L: *mut lua_State, idx: libc::c_int) -> lua_CFunction;
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
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
    fn lua_gettable(L: *mut lua_State, idx: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_getfenv(L: *mut lua_State, idx: libc::c_int);
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_setfenv(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn luaL_argerror(
        L: *mut lua_State,
        numarg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
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
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char);
    fn luaL_checktype(L: *mut lua_State, narg: libc::c_int, t: libc::c_int);
    fn luaL_checkany(L: *mut lua_State, narg: libc::c_int);
    fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;
    fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn luaL_checkoption(
        L: *mut lua_State,
        narg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer);
    fn luaL_prepbuffer(B: *mut luaL_Buffer) -> *mut libc::c_char;
    fn luaL_pushresult(B: *mut luaL_Buffer);
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn I_mkdir(dirname: *const libc::c_char, unixright: int32_t) -> int32_t;
    static mut server: boolean;
    static mut serverplayer: int32_t;
    static mut player_names: [[libc::c_char; 22]; 32];
    fn SendKick(playernum: uint8_t, msg: uint8_t);
    static mut luafiletransfers: *mut luafiletransfer_t;
    static mut waitingforluafilecommand: boolean;
    static mut luafiledir: [libc::c_char; 272];
    fn AddLuaFileTransfer(filename: *const libc::c_char, mode: *const libc::c_char);
    fn SV_PrepareSendLuaFile();
    fn RemoveLuaFileTransfer();
    fn CL_PrepareDownloadLuaFile();
    static mut gL: *mut lua_State;
    fn LUA_GetErrorMessage(L: *mut lua_State) -> libc::c_int;
    fn LUA_Call(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errorhandlerindex: libc::c_int,
    ) -> libc::c_int;
    fn I_GetTime() -> tic_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub p: *mut libc::c_char,
    pub lvl: libc::c_int,
    pub L: *mut lua_State,
    pub buffer: [libc::c_char; 8192],
}
pub type tic_t = uint32_t;
pub type boolean = int32_t;
pub type luafiletransfer_t = luafiletransfer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luafiletransfer_s {
    pub filename: *mut libc::c_char,
    pub realfilename: *mut libc::c_char,
    pub mode: [libc::c_char; 4],
    pub id: int32_t,
    pub ongoing: boolean,
    pub nodestatus: [luafiletransfernodestatus_t; 127],
    pub nodetimeouts: [tic_t; 127],
    pub next: *mut luafiletransfer_s,
}
pub type luafiletransfernodestatus_t = libc::c_uint;
pub const LFTNS_SENT: luafiletransfernodestatus_t = 4;
pub const LFTNS_SENDING: luafiletransfernodestatus_t = 3;
pub const LFTNS_ASKED: luafiletransfernodestatus_t = 2;
pub const LFTNS_WAITING: luafiletransfernodestatus_t = 1;
pub const LFTNS_NONE: luafiletransfernodestatus_t = 0;
pub const false_0: C2RustUnnamed = 0;
pub const true_0: C2RustUnnamed = 1;
pub type C2RustUnnamed = libc::c_uint;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
static mut whitelist: [*const libc::c_char; 7] = [
    b".bmp\0" as *const u8 as *const libc::c_char,
    b".cfg\0" as *const u8 as *const libc::c_char,
    b".csv\0" as *const u8 as *const libc::c_char,
    b".dat\0" as *const u8 as *const libc::c_char,
    b".png\0" as *const u8 as *const libc::c_char,
    b".sav2\0" as *const u8 as *const libc::c_char,
    b".txt\0" as *const u8 as *const libc::c_char,
];
static mut numwrittenbytes: int64_t = 0 as libc::c_int as int64_t;
static mut numopenedfiles: int64_t = 0 as libc::c_int as int64_t;
unsafe extern "C" fn pushresult(
    mut L: *mut lua_State,
    mut i: libc::c_int,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut en: libc::c_int = *__errno_location();
    if i != 0 {
        lua_pushboolean(L, 1 as libc::c_int);
        return 1 as libc::c_int;
    } else {
        lua_pushnil(L);
        if !filename.is_null() {
            lua_pushfstring(
                L,
                b"%s: %s\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(en),
            );
        } else {
            lua_pushfstring(
                L,
                b"%s\0" as *const u8 as *const libc::c_char,
                strerror(en),
            );
        }
        lua_pushnumber(L, en);
        return 3 as libc::c_int;
    };
}
unsafe extern "C" fn io_type(mut L: *mut lua_State) -> libc::c_int {
    let mut ud: *mut libc::c_void = 0 as *mut libc::c_void;
    luaL_checkany(L, 1 as libc::c_int);
    ud = lua_touserdata(L, 1 as libc::c_int);
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"FILE*\0" as *const u8 as *const libc::c_char,
    );
    if ud.is_null() || lua_getmetatable(L, 1 as libc::c_int) == 0
        || lua_rawequal(L, -(2 as libc::c_int), -(1 as libc::c_int)) == 0
    {
        lua_pushnil(L);
    } else if (*(ud as *mut *mut FILE)).is_null() {
        lua_pushlstring(
            L,
            b"closed file\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        lua_pushlstring(
            L,
            b"file\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn tofile(mut L: *mut lua_State) -> *mut FILE {
    let mut f: *mut *mut FILE = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"FILE*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut FILE;
    if (*f).is_null() {
        luaL_error(
            L,
            b"attempt to use a closed file\0" as *const u8 as *const libc::c_char,
        );
    }
    return *f;
}
unsafe extern "C" fn newfile(mut L: *mut lua_State) -> *mut *mut FILE {
    let mut pf: *mut *mut FILE = lua_newuserdata(
        L,
        ::core::mem::size_of::<*mut FILE>() as libc::c_ulong,
    ) as *mut *mut FILE;
    *pf = 0 as *mut FILE;
    lua_getfield(
        L,
        -(10000 as libc::c_int),
        b"FILE*\0" as *const u8 as *const libc::c_char,
    );
    lua_setmetatable(L, -(2 as libc::c_int));
    return pf;
}
unsafe extern "C" fn io_noclose(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnil(L);
    lua_pushlstring(
        L,
        b"cannot close standard file\0" as *const u8 as *const libc::c_char,
        (::core::mem::size_of::<[libc::c_char; 27]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    return 2 as libc::c_int;
}
unsafe extern "C" fn io_fclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut *mut FILE = luaL_checkudata(
        L,
        1 as libc::c_int,
        b"FILE*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut FILE;
    let mut ok: libc::c_int = (fclose(*p) == 0 as libc::c_int) as libc::c_int;
    *p = 0 as *mut FILE;
    return pushresult(L, ok, 0 as *const libc::c_char);
}
unsafe extern "C" fn aux_close(mut L: *mut lua_State) -> libc::c_int {
    lua_getfenv(L, 1 as libc::c_int);
    lua_getfield(
        L,
        -(1 as libc::c_int),
        b"__close\0" as *const u8 as *const libc::c_char,
    );
    return (lua_tocfunction(L, -(1 as libc::c_int)))
        .expect("non-null function pointer")(L);
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 1 as libc::c_int) == -(1 as libc::c_int) {
        lua_rawgeti(L, -(10001 as libc::c_int), 2 as libc::c_int);
    }
    tofile(L);
    return aux_close(L);
}
unsafe extern "C" fn io_gc(mut L: *mut lua_State) -> libc::c_int {
    let mut f: *mut FILE = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"FILE*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut FILE);
    if !f.is_null() {
        aux_close(L);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn io_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut f: *mut FILE = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"FILE*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut FILE);
    if f.is_null() {
        lua_pushlstring(
            L,
            b"file (closed)\0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        lua_pushfstring(L, b"file (%p)\0" as *const u8 as *const libc::c_char, f);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MakePathDirs(mut path: *mut libc::c_char) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = path;
    while *c != 0 {
        if *c as libc::c_int == '/' as i32 || *c as libc::c_int == '\\' as i32 {
            let mut sep: libc::c_char = *c;
            *c = '\0' as i32 as libc::c_char;
            I_mkdir(path, 0o755 as libc::c_int);
            *c = sep;
        }
        c = c.offset(1);
        c;
    }
}
unsafe extern "C" fn CheckFileName(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut length: libc::c_int = strlen(filename) as libc::c_int;
    let mut pass: boolean = false_0 as libc::c_int;
    let mut i: size_t = 0;
    if !(strchr(filename, '\\' as i32)).is_null() {
        luaL_error(
            L,
            b"access denied to %s: \\ is not allowed, use / instead\0" as *const u8
                as *const libc::c_char,
            filename,
        );
        return pushresult(L, 0 as libc::c_int, filename);
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if strcasecmp(
            &*filename
                .offset(
                    (length as libc::c_ulong)
                        .wrapping_sub(
                            (strlen
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> libc::c_ulong)(
                                *whitelist.as_mut_ptr().offset(i as isize),
                            ),
                        ) as isize,
                ),
            whitelist[i as usize],
        ) == 0
        {
            pass = true_0 as libc::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    if !(strstr(filename, b"./\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(filename, b"..\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strchr(filename, ':' as i32)).is_null()
        || *filename.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || pass == 0
    {
        luaL_error(
            L,
            b"access denied to %s\0" as *const u8 as *const libc::c_char,
            filename,
        );
        return pushresult(L, 0 as libc::c_int, filename);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2 as libc::c_int,
        b"r\0" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut checkresult: libc::c_int = 0;
    checkresult = CheckFileName(L, filename);
    if checkresult != 0 {
        return checkresult;
    }
    luaL_checktype(L, 3 as libc::c_int, 6 as libc::c_int);
    if !(!(strchr(mode, 'r' as i32)).is_null() || !(strchr(mode, '+' as i32)).is_null())
    {
        luaL_error(
            L,
            b"open() is only for reading, use openlocal() for writing\0" as *const u8
                as *const libc::c_char,
        );
    }
    AddLuaFileTransfer(filename, mode);
    return 0 as libc::c_int;
}
unsafe extern "C" fn io_openlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut pf: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut filename: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2 as libc::c_int,
        b"r\0" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut realfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filetransfer: *mut luafiletransfer_t = 0 as *mut luafiletransfer_t;
    let mut checkresult: libc::c_int = 0;
    checkresult = CheckFileName(L, filename);
    if checkresult != 0 {
        return checkresult;
    }
    realfilename = va(
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        luafiledir.as_mut_ptr(),
        filename,
    );
    if server == 0
        && strncasecmp(
            filename,
            b"client/\0" as *const u8 as *const libc::c_char,
            strlen(b"client/\0" as *const u8 as *const libc::c_char),
        ) != 0
    {
        I_Error(
            b"Access denied to %s\nClients can only access files stored in luafiles/client/\n\0"
                as *const u8 as *const libc::c_char,
            filename,
        );
    }
    filetransfer = luafiletransfers;
    while !filetransfer.is_null() {
        if strcasecmp((*filetransfer).filename, filename) == 0 {
            I_Error(
                b"Access denied to %s\nFiles can't be opened while being downloaded\n\0"
                    as *const u8 as *const libc::c_char,
                filename,
            );
        }
        filetransfer = (*filetransfer).next;
    }
    if server == 0
        && (!(strchr(mode, 'w' as i32)).is_null()
            || !(strchr(mode, 'a' as i32)).is_null()
            || !(strchr(mode, '+' as i32)).is_null())
    {
        if numopenedfiles
            >= ((I_GetTime() / (60 as libc::c_int * 35 as libc::c_int) as tic_t)
                .wrapping_add(1 as libc::c_int as tic_t) * 50 as libc::c_int as tic_t)
                as int64_t
        {
            I_Error(
                b"Access denied to %s\nFile opening rate exceeded\n\0" as *const u8
                    as *const libc::c_char,
                filename,
            );
        }
        numopenedfiles += 1;
        numopenedfiles;
    }
    MakePathDirs(realfilename);
    pf = newfile(L);
    *pf = fopen(realfilename, mode);
    return if (*pf).is_null() {
        pushresult(L, 0 as libc::c_int, filename)
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn Got_LuaFile(mut cp: *mut *mut uint8_t, mut playernum: int32_t) {
    let mut pf: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut success: uint8_t = ({
        let mut p_tmp: *mut uint8_t = *cp as *mut libc::c_void as *mut uint8_t;
        let mut b: uint8_t = 0;
        memcpy(
            &mut b as *mut uint8_t as *mut libc::c_void,
            *cp as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *cp = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    });
    if playernum != serverplayer {
        CONS_Alert(
            CONS_WARNING,
            b"Illegal luafile command received from %s\n\0" as *const u8
                as *const libc::c_char,
            (player_names[playernum as usize]).as_mut_ptr(),
        );
        if server != 0 {
            SendKick(playernum as uint8_t, 2 as libc::c_int as uint8_t);
        }
        return;
    }
    if luafiletransfers.is_null() {
        I_Error(b"No Lua file transfer\n\0" as *const u8 as *const libc::c_char);
    }
    lua_settop(gL, 0 as libc::c_int);
    lua_pushcclosure(
        gL,
        Some(LUA_GetErrorMessage as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_pushfstring(
        gL,
        b"file_callback_%d\0" as *const u8 as *const libc::c_char,
        (*luafiletransfers).id,
    );
    lua_gettable(gL, -(10000 as libc::c_int));
    if success != 0 {
        let mut mode: [libc::c_char; 4] = [0; 4];
        strcpy(mode.as_mut_ptr(), ((*luafiletransfers).mode).as_mut_ptr());
        if (strchr(mode.as_mut_ptr(), 'b' as i32)).is_null() {
            strcat(mode.as_mut_ptr(), b"b\0" as *const u8 as *const libc::c_char);
        }
        pf = newfile(gL);
        *pf = fopen((*luafiletransfers).realfilename, mode.as_mut_ptr());
        if (*pf).is_null() {
            I_Error(
                b"Can't open file \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*luafiletransfers).realfilename,
            );
        }
    } else {
        lua_pushnil(gL);
    }
    lua_pushstring(gL, (*luafiletransfers).filename);
    LUA_Call(gL, 2 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    lua_settop(gL, 0 as libc::c_int);
    if success != 0 {
        if !(*pf).is_null() {
            fclose(*pf);
            *pf = 0 as *mut FILE;
        }
        if server == 0 {
            remove((*luafiletransfers).realfilename);
        }
    }
    RemoveLuaFileTransfer();
    if waitingforluafilecommand != 0 {
        waitingforluafilecommand = false_0 as libc::c_int;
        CL_PrepareDownloadLuaFile();
    }
    if server != 0 && !luafiletransfers.is_null() {
        SV_PrepareSendLuaFile();
    }
}
#[no_mangle]
pub unsafe extern "C" fn StoreLuaFileCallback(mut id: int32_t) {
    lua_pushfstring(gL, b"file_callback_%d\0" as *const u8 as *const libc::c_char, id);
    lua_pushvalue(gL, 3 as libc::c_int);
    lua_settable(gL, -(10000 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn RemoveLuaFileCallback(mut id: int32_t) {
    lua_pushfstring(gL, b"file_callback_%d\0" as *const u8 as *const libc::c_char, id);
    lua_pushnil(gL);
    lua_settable(gL, -(10000 as libc::c_int));
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State) -> libc::c_int {
    let mut pf: *mut *mut FILE = newfile(L);
    *pf = tmpfile();
    return if (*pf).is_null() {
        pushresult(L, 0 as libc::c_int, 0 as *const libc::c_char)
    } else {
        1 as libc::c_int
    };
}
unsafe extern "C" fn aux_lines(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut toclose: libc::c_int,
) {
    lua_pushvalue(L, idx);
    lua_pushboolean(L, toclose);
    lua_pushcclosure(
        L,
        Some(io_readline as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        2 as libc::c_int,
    );
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State) -> libc::c_int {
    tofile(L);
    aux_lines(L, 1 as libc::c_int, 0 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_number(
    mut L: *mut lua_State,
    mut f: *mut FILE,
) -> libc::c_int {
    let mut d: lua_Number = 0;
    if fscanf(f, b"%d\0" as *const u8 as *const libc::c_char, &mut d as *mut lua_Number)
        == 1 as libc::c_int
    {
        lua_pushnumber(L, d);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State, mut f: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = getc(f);
    ungetc(c, f);
    lua_pushlstring(L, 0 as *const libc::c_char, 0 as libc::c_int as size_t);
    return (c != -(1 as libc::c_int)) as libc::c_int;
}
unsafe extern "C" fn read_line(mut L: *mut lua_State, mut f: *mut FILE) -> libc::c_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        p: 0 as *mut libc::c_char,
        lvl: 0,
        L: 0 as *mut lua_State,
        buffer: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        let mut l: size_t = 0;
        let mut p: *mut libc::c_char = luaL_prepbuffer(&mut b);
        if (fgets(p, 8192 as libc::c_int, f)).is_null() {
            luaL_pushresult(&mut b);
            return (lua_objlen(L, -(1 as libc::c_int)) > 0 as libc::c_int as size_t)
                as libc::c_int;
        }
        l = strlen(p);
        if l == 0 as libc::c_int as size_t
            || *p.offset(l.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int != '\n' as i32
        {
            b.p = (b.p).offset(l as isize);
        } else {
            b.p = (b.p).offset(l.wrapping_sub(1 as libc::c_int as size_t) as isize);
            luaL_pushresult(&mut b);
            return 1 as libc::c_int;
        }
    };
}
unsafe extern "C" fn read_chars(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut n: size_t,
) -> libc::c_int {
    let mut rlen: size_t = 0;
    let mut nr: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        p: 0 as *mut libc::c_char,
        lvl: 0,
        L: 0 as *mut lua_State,
        buffer: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    rlen = 8192 as libc::c_int as size_t;
    loop {
        let mut p: *mut libc::c_char = luaL_prepbuffer(&mut b);
        if rlen > n {
            rlen = n;
        }
        nr = fread(
            p as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            rlen,
            f,
        );
        b.p = (b.p).offset(nr as isize);
        n = n.wrapping_sub(nr);
        if !(n > 0 as libc::c_int as size_t && nr == rlen) {
            break;
        }
    }
    luaL_pushresult(&mut b);
    return (n == 0 as libc::c_int as size_t
        || lua_objlen(L, -(1 as libc::c_int)) > 0 as libc::c_int as size_t)
        as libc::c_int;
}
unsafe extern "C" fn g_read(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut nargs: libc::c_int = lua_gettop(L) - 1 as libc::c_int;
    let mut success: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    clearerr(f);
    if nargs == 0 as libc::c_int {
        success = read_line(L, f);
        n = first + 1 as libc::c_int;
    } else {
        luaL_checkstack(
            L,
            nargs + 20 as libc::c_int,
            b"too many arguments\0" as *const u8 as *const libc::c_char,
        );
        success = 1 as libc::c_int;
        n = first;
        loop {
            let fresh0 = nargs;
            nargs = nargs - 1;
            if !(fresh0 != 0 && success != 0) {
                break;
            }
            if lua_type(L, n) == 3 as libc::c_int {
                let mut l: size_t = lua_tonumber(L, n) as size_t;
                success = if l == 0 as libc::c_int as size_t {
                    test_eof(L, f)
                } else {
                    read_chars(L, f, l)
                };
            } else {
                let mut p: *const libc::c_char = lua_tolstring(L, n, 0 as *mut size_t);
                (!p.is_null()
                    && *p.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32
                    || luaL_argerror(
                        L,
                        n,
                        b"invalid option\0" as *const u8 as *const libc::c_char,
                    ) != 0) as libc::c_int;
                match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                    110 => {
                        success = read_number(L, f);
                    }
                    108 => {
                        success = read_line(L, f);
                    }
                    97 => {
                        read_chars(L, f, !(0 as libc::c_int as size_t));
                        success = 1 as libc::c_int;
                    }
                    _ => {
                        return luaL_argerror(
                            L,
                            n,
                            b"invalid format\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            n += 1;
            n;
        }
    }
    if ferror(f) != 0 {
        return pushresult(L, 0 as libc::c_int, 0 as *const libc::c_char);
    }
    if success == 0 {
        lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
        lua_pushnil(L);
    }
    return n - first;
}
unsafe extern "C" fn f_read(mut L: *mut lua_State) -> libc::c_int {
    return g_read(L, tofile(L), 2 as libc::c_int);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State) -> libc::c_int {
    let mut f: *mut FILE = *(lua_touserdata(
        L,
        -(10002 as libc::c_int) - 1 as libc::c_int,
    ) as *mut *mut FILE);
    let mut sucess: libc::c_int = 0;
    if f.is_null() {
        luaL_error(L, b"file is already closed\0" as *const u8 as *const libc::c_char);
    }
    sucess = read_line(L, f);
    if ferror(f) != 0 {
        return luaL_error(
            L,
            b"%s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if sucess != 0 {
        return 1 as libc::c_int
    } else {
        if lua_toboolean(L, -(10002 as libc::c_int) - 2 as libc::c_int) != 0 {
            lua_settop(L, 0 as libc::c_int);
            lua_pushvalue(L, -(10002 as libc::c_int) - 1 as libc::c_int);
            aux_close(L);
        }
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn g_write(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut arg: libc::c_int,
) -> libc::c_int {
    let mut nargs: libc::c_int = lua_gettop(L) - 1 as libc::c_int;
    let mut status: libc::c_int = 1 as libc::c_int;
    loop {
        let fresh1 = nargs;
        nargs = nargs - 1;
        if !(fresh1 != 0) {
            break;
        }
        if lua_type(L, arg) == 3 as libc::c_int {
            status = (status != 0
                && fprintf(
                    f,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    lua_tonumber(L, arg),
                ) > 0 as libc::c_int) as libc::c_int;
        } else {
            let mut l: size_t = 0;
            let mut s: *const libc::c_char = luaL_checklstring(L, arg, &mut l);
            if server == 0 {
                if (numwrittenbytes as size_t).wrapping_add(l)
                    > ((I_GetTime() / (60 as libc::c_int * 35 as libc::c_int) as tic_t)
                        .wrapping_add(1 as libc::c_int as tic_t)
                        * (10 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                            as tic_t) as size_t
                {
                    luaL_error(
                        L,
                        b"file write rate limit exceeded; changes have been discarded\0"
                            as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    numwrittenbytes = (numwrittenbytes as size_t).wrapping_add(l)
                        as int64_t as int64_t;
                }
            }
            status = (status != 0
                && fwrite(
                    s as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    l,
                    f,
                ) == l) as libc::c_int;
        }
        arg += 1;
        arg;
    }
    return pushresult(L, status, 0 as *const libc::c_char);
}
unsafe extern "C" fn f_write(mut L: *mut lua_State) -> libc::c_int {
    return g_write(L, tofile(L), 2 as libc::c_int);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = [
        0 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
    ];
    static mut modenames: [*const libc::c_char; 4] = [
        b"set\0" as *const u8 as *const libc::c_char,
        b"cur\0" as *const u8 as *const libc::c_char,
        b"end\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int = luaL_checkoption(
        L,
        2 as libc::c_int,
        b"cur\0" as *const u8 as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut offset: libc::c_long = luaL_optnumber(L, 3 as libc::c_int, 0 as libc::c_int)
        as libc::c_long;
    op = fseek(f, offset, mode[op as usize]);
    if op != 0 {
        return pushresult(L, 0 as libc::c_int, 0 as *const libc::c_char)
    } else {
        lua_pushnumber(L, ftell(f) as lua_Number);
        return 1 as libc::c_int;
    };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = [
        2 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    ];
    static mut modenames: [*const libc::c_char; 4] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"full\0" as *const u8 as *const libc::c_char,
        b"line\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int = luaL_checkoption(
        L,
        2 as libc::c_int,
        0 as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut sz: lua_Number = luaL_optnumber(L, 3 as libc::c_int, 8192 as libc::c_int);
    let mut res: libc::c_int = setvbuf(
        f,
        0 as *mut libc::c_char,
        mode[op as usize],
        sz as size_t,
    );
    return pushresult(
        L,
        (res == 0 as libc::c_int) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State) -> libc::c_int {
    return pushresult(
        L,
        (fflush(tofile(L)) == 0 as libc::c_int) as libc::c_int,
        0 as *const libc::c_char,
    );
}
static mut iolib: [luaL_Reg; 6] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_close as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"open\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_open as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"openlocal\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_openlocal as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tmpfile\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_tmpfile as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"type\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_type as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
static mut flib: [luaL_Reg; 10] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"close\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_close as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"flush\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_flush as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"lines\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_lines as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"read\0" as *const u8 as *const libc::c_char,
                func: Some(f_read as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"seek\0" as *const u8 as *const libc::c_char,
                func: Some(f_seek as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setvbuf\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_setvbuf as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"write\0" as *const u8 as *const libc::c_char,
                func: Some(
                    f_write as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__gc\0" as *const u8 as *const libc::c_char,
                func: Some(io_gc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"__tostring\0" as *const u8 as *const libc::c_char,
                func: Some(
                    io_tostring as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
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
unsafe extern "C" fn createmeta(mut L: *mut lua_State) {
    luaL_newmetatable(L, b"FILE*\0" as *const u8 as *const libc::c_char);
    lua_pushvalue(L, -(1 as libc::c_int));
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    luaL_register(L, 0 as *const libc::c_char, flib.as_ptr());
}
unsafe extern "C" fn createstdfile(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut k: libc::c_int,
    mut fname: *const libc::c_char,
) {
    let ref mut fresh2 = *newfile(L);
    *fresh2 = f;
    if k > 0 as libc::c_int {
        lua_pushvalue(L, -(1 as libc::c_int));
        lua_rawseti(L, -(10001 as libc::c_int), k);
    }
    lua_pushvalue(L, -(2 as libc::c_int));
    lua_setfenv(L, -(2 as libc::c_int));
    lua_setfield(L, -(3 as libc::c_int), fname);
}
unsafe extern "C" fn newfenv(mut L: *mut lua_State, mut cls: lua_CFunction) {
    lua_createtable(L, 0 as libc::c_int, 1 as libc::c_int);
    lua_pushcclosure(L, cls, 0 as libc::c_int);
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__close\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State) -> libc::c_int {
    createmeta(L);
    newfenv(L, Some(io_fclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int));
    lua_replace(L, -(10001 as libc::c_int));
    luaL_register(L, b"io\0" as *const u8 as *const libc::c_char, iolib.as_ptr());
    newfenv(L, Some(io_noclose as unsafe extern "C" fn(*mut lua_State) -> libc::c_int));
    createstdfile(
        L,
        stdin,
        1 as libc::c_int,
        b"stdin\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stdout,
        2 as libc::c_int,
        b"stdout\0" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stderr,
        0 as libc::c_int,
        b"stderr\0" as *const u8 as *const libc::c_char,
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    return 1 as libc::c_int;
}
