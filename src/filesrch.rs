use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn rewinddir(__dirp: *mut DIR);
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn strupr(n: *mut libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut CV_YesNo: [CV_PossibleValue_t; 0];
    static mut numwadfiles: uint16_t;
    static mut wadfiles: *mut *mut wadfile_t;
    fn Addons_option_Onchange();
    fn nameonly(s: *mut libc::c_char);
    fn checkfilemd5(
        filename: *mut libc::c_char,
        wantedmd5sum: *const uint8_t,
    ) -> filestatus_t;
    static mut configfile: [libc::c_char; 512];
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_MallocAlign(
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn Z_CallocAlign(
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn Z_ReallocAlign(
        ptr: *mut libc::c_void,
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn Z_StrDup(in_0: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const CV_ALLOWLUA: C2RustUnnamed_0 = 4096;
pub const CV_CHEAT: C2RustUnnamed_0 = 2048;
pub const CV_HIDEN: C2RustUnnamed_0 = 1024;
pub const CV_NOSHOWHELP: C2RustUnnamed_0 = 512;
pub const CV_SHOWMODIFONETIME: C2RustUnnamed_0 = 256;
pub const CV_SHOWMODIF: C2RustUnnamed_0 = 128;
pub const CV_MODIFIED: C2RustUnnamed_0 = 64;
pub const CV_NOTINNET: C2RustUnnamed_0 = 32;
pub const CV_FLOAT: C2RustUnnamed_0 = 16;
pub const CV_NOINIT: C2RustUnnamed_0 = 8;
pub const CV_NETVAR: C2RustUnnamed_0 = 4;
pub const CV_CALL: C2RustUnnamed_0 = 2;
pub const CV_SAVE: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CV_PossibleValue_s {
    pub value: int32_t,
    pub strvalue: *const libc::c_char,
}
pub type CV_PossibleValue_t = CV_PossibleValue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct consvar_s {
    pub name: *const libc::c_char,
    pub defaultvalue: *const libc::c_char,
    pub flags: int32_t,
    pub PossibleValue: *mut CV_PossibleValue_t,
    pub func: Option::<unsafe extern "C" fn() -> ()>,
    pub value: int32_t,
    pub string: *const libc::c_char,
    pub zstring: *mut libc::c_char,
    pub revert: C2RustUnnamed_1,
    pub netid: uint16_t,
    pub changed: libc::c_char,
    pub next: *mut consvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub allocated: libc::c_char,
    pub v: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub string: *mut libc::c_char,
    pub const_munge: *const libc::c_char,
}
pub type consvar_t = consvar_s;
pub type compmethod = libc::c_uint;
pub const CM_UNSUPPORTED: compmethod = 3;
pub const CM_LZF: compmethod = 2;
pub const CM_DEFLATE: compmethod = 1;
pub const CM_NOCOMPRESSION: compmethod = 0;
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
pub type restype = libc::c_uint;
pub const RET_UNKNOWN: restype = 5;
pub const RET_FOLDER: restype = 4;
pub const RET_PK3: restype = 3;
pub const RET_LUA: restype = 2;
pub const RET_SOC: restype = 1;
pub const RET_WAD: restype = 0;
pub type restype_t = restype;
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
pub type wadfile_t = wadfile_s;
pub type filestatus_t = libc::c_uint;
pub const FS_MD5SUMBAD: filestatus_t = 6;
pub const FS_OPEN: filestatus_t = 5;
pub const FS_DOWNLOADING: filestatus_t = 4;
pub const FS_REQUESTED: filestatus_t = 3;
pub const FS_FOUND: filestatus_t = 2;
pub const FS_NOTFOUND: filestatus_t = 1;
pub const FS_NOTCHECKED: filestatus_t = 0;
pub const PU_STATIC: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const EXT_LOADED: C2RustUnnamed_3 = 128;
pub const NUM_EXT_TABLE: C2RustUnnamed_3 = 6;
pub const NUM_EXT: C2RustUnnamed_3 = 9;
pub const EXT_LUA: C2RustUnnamed_3 = 8;
pub const EXT_SOC: C2RustUnnamed_3 = 7;
pub const EXT_PK3: C2RustUnnamed_3 = 6;
pub const EXT_WAD: C2RustUnnamed_3 = 5;
pub const EXT_LOADSTART: C2RustUnnamed_3 = 5;
pub const EXT_CFG: C2RustUnnamed_3 = 4;
pub const EXT_TXT: C2RustUnnamed_3 = 3;
pub const EXT_START: C2RustUnnamed_3 = 3;
pub const EXT_NORESULTS: C2RustUnnamed_3 = 2;
pub const EXT_UP: C2RustUnnamed_3 = 1;
pub const EXT_FOLDER: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const DIR_STRING: C2RustUnnamed_4 = 2;
pub const DIR_LEN: C2RustUnnamed_4 = 1;
pub const DIR_TYPE: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_5 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_5 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_5 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_5 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_5 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_5 = 51;
pub const PU_LEVEL: C2RustUnnamed_5 = 50;
pub const PU_CACHE: C2RustUnnamed_5 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_5 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_5 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_5 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_5 = 21;
pub const PU_HUDGFX: C2RustUnnamed_5 = 19;
pub const PU_SPRITE: C2RustUnnamed_5 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_5 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_5 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_5 = 15;
pub const PU_PATCH: C2RustUnnamed_5 = 14;
pub const PU_MUSIC: C2RustUnnamed_5 = 12;
pub const PU_SOUND: C2RustUnnamed_5 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_5 = 3;
pub const PU_LUA: C2RustUnnamed_5 = 2;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn quickncasehash(
    mut p: *const libc::c_char,
    mut n: size_t,
) -> uint32_t {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut x: uint32_t = 5381 as libc::c_int as uint32_t;
    while i < n && *p.offset(i as isize) as libc::c_int != 0 {
        x = x * 33 as libc::c_int as uint32_t
            ^ ({
                let mut __res: libc::c_int = 0;
                if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        let mut __c: libc::c_int = *p.offset(i as isize) as libc::c_int;
                        __res = (if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        });
                    } else {
                        __res = tolower(*p.offset(i as isize) as libc::c_int);
                    }
                } else {
                    __res = *(*__ctype_tolower_loc())
                        .offset(*p.offset(i as isize) as libc::c_int as isize);
                }
                __res
            }) as uint32_t;
        i = i.wrapping_add(1);
        i;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn fopenfile(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut h: *mut FILE = fopen(path, mode);
    if !h.is_null() {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        let mut eno: libc::c_int = 0;
        if fstat(fileno(h), &mut st) == -(1 as libc::c_int) {
            eno = *__errno_location();
        } else if !(st.st_mode & 0o170000 as libc::c_int as __mode_t
            == 0o100000 as libc::c_int as __mode_t)
        {
            eno = 13 as libc::c_int;
        } else {
            return h
        }
        fclose(h);
        *__errno_location() = eno;
    }
    return 0 as *mut FILE;
}
static mut addons_cons_t: [CV_PossibleValue_t; 5] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Default\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"HOME\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"SRB2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 3 as libc::c_int,
            strvalue: b"CUSTOM\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut cv_addons_option: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"addons_option\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Default\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_CALL as libc::c_int,
            PossibleValue: addons_cons_t.as_ptr() as *mut _,
            func: Some(Addons_option_Onchange as unsafe extern "C" fn() -> ()),
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_1 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_2 {
                        string: 0 as *const libc::c_char as *mut libc::c_char,
                    },
                };
                init
            },
            netid: 0 as libc::c_uint as uint16_t,
            changed: 0 as libc::c_int as libc::c_char,
            next: 0 as *const consvar_s as *mut consvar_s,
        };
        init
    }
};
#[no_mangle]
pub static mut cv_addons_folder: consvar_t = {
    let mut init = consvar_s {
        name: b"addons_folder\0" as *const u8 as *const libc::c_char,
        defaultvalue: b"\0" as *const u8 as *const libc::c_char,
        flags: CV_SAVE as libc::c_int,
        PossibleValue: 0 as *const CV_PossibleValue_t as *mut CV_PossibleValue_t,
        func: None,
        value: 0 as libc::c_int,
        string: 0 as *const libc::c_char,
        zstring: 0 as *const libc::c_char as *mut libc::c_char,
        revert: {
            let mut init = C2RustUnnamed_1 {
                allocated: 0 as libc::c_int as libc::c_char,
                v: C2RustUnnamed_2 {
                    string: 0 as *const libc::c_char as *mut libc::c_char,
                },
            };
            init
        },
        netid: 0 as libc::c_uint as uint16_t,
        changed: 0 as libc::c_int as libc::c_char,
        next: 0 as *const consvar_s as *mut consvar_s,
    };
    init
};
static mut addons_md5_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Name\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"Contents\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut cv_addons_md5: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"addons_md5\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Name\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: addons_md5_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_1 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_2 {
                        string: 0 as *const libc::c_char as *mut libc::c_char,
                    },
                };
                init
            },
            netid: 0 as libc::c_uint as uint16_t,
            changed: 0 as libc::c_int as libc::c_char,
            next: 0 as *const consvar_s as *mut consvar_s,
        };
        init
    }
};
#[no_mangle]
pub static mut cv_addons_showall: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"addons_showall\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"No\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_YesNo.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_1 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_2 {
                        string: 0 as *const libc::c_char as *mut libc::c_char,
                    },
                };
                init
            },
            netid: 0 as libc::c_uint as uint16_t,
            changed: 0 as libc::c_int as libc::c_char,
            next: 0 as *const consvar_s as *mut consvar_s,
        };
        init
    }
};
#[no_mangle]
pub static mut cv_addons_search_case: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"addons_search_case\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"No\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_YesNo.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_1 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_2 {
                        string: 0 as *const libc::c_char as *mut libc::c_char,
                    },
                };
                init
            },
            netid: 0 as libc::c_uint as uint16_t,
            changed: 0 as libc::c_int as libc::c_char,
            next: 0 as *const consvar_s as *mut consvar_s,
        };
        init
    }
};
static mut addons_search_type_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Start\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"Anywhere\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut cv_addons_search_type: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"addons_search_type\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Anywhere\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: addons_search_type_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_1 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_2 {
                        string: 0 as *const libc::c_char as *mut libc::c_char,
                    },
                };
                init
            },
            netid: 0 as libc::c_uint as uint16_t,
            changed: 0 as libc::c_int as libc::c_char,
            next: 0 as *const consvar_s as *mut consvar_s,
        };
        init
    }
};
#[no_mangle]
pub static mut menupath: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut menupathindex: [size_t; 20] = [0; 20];
#[no_mangle]
pub static mut menudepthleft: size_t = 20 as libc::c_int as size_t;
#[no_mangle]
pub static mut menusearch: [libc::c_char; 33] = [0; 33];
#[no_mangle]
pub static mut dirmenu: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut coredirmenu: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub static mut sizedirmenu: size_t = 0;
#[no_mangle]
pub static mut sizecoredirmenu: size_t = 0;
#[no_mangle]
pub static mut dir_on: [size_t; 20] = [0; 20];
#[no_mangle]
pub static mut refreshdirmenu: uint8_t = 0 as libc::c_int as uint8_t;
#[no_mangle]
pub static mut refreshdirname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn filesearch(
    mut filename: *mut libc::c_char,
    mut startpath: *const libc::c_char,
    mut wantedmd5sum: *const uint8_t,
    mut completepath: boolean,
    mut maxsearchdepth: libc::c_int,
) -> filestatus_t {
    let mut retval: filestatus_t = FS_NOTFOUND;
    let mut dirhandle: *mut *mut DIR = 0 as *mut *mut DIR;
    let mut dent: *mut dirent = 0 as *mut dirent;
    let mut fsstat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut searchname: *mut libc::c_char = strdup(filename);
    let mut depthleft: libc::c_int = maxsearchdepth;
    let mut searchpath: [libc::c_char; 1024] = [0; 1024];
    let mut searchpathindex: *mut size_t = 0 as *mut size_t;
    dirhandle = malloc(
        (maxsearchdepth as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut DIR>() as libc::c_ulong),
    ) as *mut *mut DIR;
    searchpathindex = malloc(
        (maxsearchdepth as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    strcpy(searchpath.as_mut_ptr(), startpath);
    depthleft -= 1;
    *searchpathindex
        .offset(
            depthleft as isize,
        ) = (strlen(searchpath.as_mut_ptr()))
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    let ref mut fresh0 = *dirhandle.offset(depthleft as isize);
    *fresh0 = opendir(searchpath.as_mut_ptr());
    if (*dirhandle.offset(depthleft as isize)).is_null() {
        free(searchname as *mut libc::c_void);
        free(dirhandle as *mut libc::c_void);
        free(searchpathindex as *mut libc::c_void);
        return FS_NOTFOUND;
    }
    if searchpath[(*searchpathindex.offset(depthleft as isize))
        .wrapping_sub(2 as libc::c_int as size_t) as usize] as libc::c_int
        != (*::core::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize] as libc::c_int
    {
        searchpath[(*searchpathindex.offset(depthleft as isize))
            .wrapping_sub(1 as libc::c_int as size_t)
            as usize] = (*::core::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize];
        searchpath[*searchpathindex.offset(depthleft as isize)
            as usize] = 0 as libc::c_int as libc::c_char;
    } else {
        let ref mut fresh1 = *searchpathindex.offset(depthleft as isize);
        *fresh1 = (*fresh1).wrapping_sub(1);
        *fresh1;
    }
    while found == 0 && depthleft < maxsearchdepth {
        searchpath[*searchpathindex.offset(depthleft as isize)
            as usize] = 0 as libc::c_int as libc::c_char;
        dent = readdir(*dirhandle.offset(depthleft as isize));
        if dent.is_null() {
            let fresh2 = depthleft;
            depthleft = depthleft + 1;
            closedir(*dirhandle.offset(fresh2 as isize));
        } else {
            if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
                && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                    == '\0' as i32
                    || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                        == '.' as i32
                        && (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                            == '\0' as i32)
            {
                continue;
            }
            strcpy(
                &mut *searchpath
                    .as_mut_ptr()
                    .offset(*searchpathindex.offset(depthleft as isize) as isize),
                ((*dent).d_name).as_mut_ptr(),
            );
            if !(stat(searchpath.as_mut_ptr(), &mut fsstat) < 0 as libc::c_int) {
                if fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
                    == 0o40000 as libc::c_int as __mode_t && depthleft != 0
                {
                    depthleft -= 1;
                    *searchpathindex
                        .offset(
                            depthleft as isize,
                        ) = (strlen(searchpath.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let ref mut fresh3 = *dirhandle.offset(depthleft as isize);
                    *fresh3 = opendir(searchpath.as_mut_ptr());
                    if (*dirhandle.offset(depthleft as isize)).is_null() {
                        depthleft += 1;
                        depthleft;
                    }
                    searchpath[(*searchpathindex.offset(depthleft as isize))
                        .wrapping_sub(1 as libc::c_int as size_t)
                        as usize] = (*::core::mem::transmute::<
                        &[u8; 2],
                        &[libc::c_char; 2],
                    >(b"/\0"))[0 as libc::c_int as usize];
                    searchpath[*searchpathindex.offset(depthleft as isize)
                        as usize] = 0 as libc::c_int as libc::c_char;
                } else if strcasecmp(searchname, ((*dent).d_name).as_mut_ptr()) == 0 {
                    match checkfilemd5(searchpath.as_mut_ptr(), wantedmd5sum)
                        as libc::c_uint
                    {
                        2 => {
                            if completepath != 0 {
                                strcpy(filename, searchpath.as_mut_ptr());
                            } else {
                                strcpy(filename, ((*dent).d_name).as_mut_ptr());
                            }
                            retval = FS_FOUND;
                            found = 1 as libc::c_int;
                        }
                        6 => {
                            retval = FS_MD5SUMBAD;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    while depthleft < maxsearchdepth {
        let fresh4 = depthleft;
        depthleft = depthleft + 1;
        closedir(*dirhandle.offset(fresh4 as isize));
    }
    free(searchname as *mut libc::c_void);
    free(searchpathindex as *mut libc::c_void);
    free(dirhandle as *mut libc::c_void);
    return retval;
}
#[no_mangle]
pub static mut direrror: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn pathisdirectory(mut path: *const libc::c_char) -> int32_t {
    let mut fsstat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path, &mut fsstat) < 0 as libc::c_int {
        direrror = *__errno_location();
        return -(1 as libc::c_int);
    } else if fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
        == 0o40000 as libc::c_int as __mode_t
    {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn concatpaths(
    mut path: *const libc::c_char,
    mut startpath: *const libc::c_char,
) -> int32_t {
    let mut dirpath: [libc::c_char; 1024] = [0; 1024];
    let mut dirhandle: *mut DIR = 0 as *mut DIR;
    let mut stat_0: int32_t = 0;
    if !startpath.is_null() {
        let mut basepath: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            basepath.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%s/\0" as *const u8 as *const libc::c_char,
            startpath,
        );
        snprintf(
            dirpath.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%s%s\0" as *const u8 as *const libc::c_char,
            basepath.as_mut_ptr(),
            path,
        );
        stat_0 = samepaths(basepath.as_mut_ptr(), dirpath.as_mut_ptr());
        if stat_0 == 1 as libc::c_int {
            return 0 as libc::c_int
        } else if stat_0 < 0 as libc::c_int {
            return -(1 as libc::c_int)
        }
    } else {
        snprintf(
            dirpath.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            path,
        );
    }
    stat_0 = pathisdirectory(dirpath.as_mut_ptr());
    if stat_0 == 0 as libc::c_int {
        return 0 as libc::c_int
    } else if stat_0 < 0 as libc::c_int {
        if direrror == 2 as libc::c_int {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    dirhandle = opendir(dirpath.as_mut_ptr());
    if dirhandle.is_null() {
        return 0 as libc::c_int
    } else {
        closedir(dirhandle);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn samepaths(
    mut path1: *const libc::c_char,
    mut path2: *const libc::c_char,
) -> int32_t {
    let mut stat1: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut stat2: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path1, &mut stat1) < 0 as libc::c_int {
        direrror = *__errno_location();
        return -(1 as libc::c_int);
    }
    if stat(path2, &mut stat2) < 0 as libc::c_int {
        direrror = *__errno_location();
        return -(2 as libc::c_int);
    }
    if stat1.st_dev == stat2.st_dev {
        return (stat1.st_ino == stat2.st_ino) as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn initdirpath(
    mut dirpath: *mut libc::c_char,
    mut dirpathindex: *mut size_t,
    mut depthleft: libc::c_int,
) {
    *dirpathindex
        .offset(
            depthleft as isize,
        ) = (strlen(dirpath)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if *dirpath
        .offset(
            (*dirpathindex.offset(depthleft as isize))
                .wrapping_sub(2 as libc::c_int as size_t) as isize,
        ) as libc::c_int
        != (*::core::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize] as libc::c_int
    {
        *dirpath
            .offset(
                (*dirpathindex.offset(depthleft as isize))
                    .wrapping_sub(1 as libc::c_int as size_t) as isize,
            ) = (*::core::mem::transmute::<
            &[u8; 2],
            &[libc::c_char; 2],
        >(b"/\0"))[0 as libc::c_int as usize];
        *dirpath
            .offset(
                *dirpathindex.offset(depthleft as isize) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    } else {
        let ref mut fresh5 = *dirpathindex.offset(depthleft as isize);
        *fresh5 = (*fresh5).wrapping_sub(1);
        *fresh5;
    };
}
#[no_mangle]
pub unsafe extern "C" fn getdirectoryfiles(
    mut path: *const libc::c_char,
    mut nlmp: *mut uint16_t,
    mut nfolders: *mut uint16_t,
) -> *mut lumpinfo_t {
    let mut dirhandle: *mut *mut DIR = 0 as *mut *mut DIR;
    let mut dent: *mut dirent = 0 as *mut dirent;
    let mut fsstat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rootdir: libc::c_int = 48 as libc::c_int - 1 as libc::c_int;
    let mut depthleft: libc::c_int = rootdir;
    let mut dirpath: [libc::c_char; 1024] = [0; 1024];
    let mut dirpathindex: *mut size_t = 0 as *mut size_t;
    let mut lumpinfo: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut lump_p: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    let mut numlumps: uint16_t = 0 as libc::c_int as uint16_t;
    let mut failure: boolean = false_0 as libc::c_int;
    dirhandle = malloc(
        (48 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut DIR>() as libc::c_ulong),
    ) as *mut *mut DIR;
    dirpathindex = malloc(
        (48 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<size_t>() as libc::c_ulong),
    ) as *mut size_t;
    strlcpy(dirpath.as_mut_ptr(), path, 1024 as libc::c_int as libc::c_ulong);
    let ref mut fresh6 = *dirhandle.offset(depthleft as isize);
    *fresh6 = opendir(dirpath.as_mut_ptr());
    if (*dirhandle.offset(depthleft as isize)).is_null() {
        free(dirhandle as *mut libc::c_void);
        free(dirpathindex as *mut libc::c_void);
        return 0 as *mut lumpinfo_t;
    }
    initdirpath(dirpath.as_mut_ptr(), dirpathindex, depthleft);
    *nfolders = 0 as libc::c_int as uint16_t;
    while depthleft < 48 as libc::c_int {
        dirpath[*dirpathindex.offset(depthleft as isize)
            as usize] = 0 as libc::c_int as libc::c_char;
        dent = readdir(*dirhandle.offset(depthleft as isize));
        if dent.is_null() {
            if depthleft != rootdir {
                closedir(*dirhandle.offset(depthleft as isize));
            }
            depthleft += 1;
            depthleft;
        } else {
            if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
                && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                    == '\0' as i32
                    || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                        == '.' as i32
                        && (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                            == '\0' as i32)
            {
                continue;
            }
            strcpy(
                &mut *dirpath
                    .as_mut_ptr()
                    .offset(*dirpathindex.offset(depthleft as isize) as isize),
                ((*dent).d_name).as_mut_ptr(),
            );
            if !(stat(dirpath.as_mut_ptr(), &mut fsstat) < 0 as libc::c_int) {
                if fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
                    == 0o40000 as libc::c_int as __mode_t && depthleft != 0
                {
                    depthleft -= 1;
                    *dirpathindex
                        .offset(
                            depthleft as isize,
                        ) = (strlen(dirpath.as_mut_ptr()))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong);
                    let ref mut fresh7 = *dirhandle.offset(depthleft as isize);
                    *fresh7 = opendir(dirpath.as_mut_ptr());
                    if !(*dirhandle.offset(depthleft as isize)).is_null() {
                        *nfolders = (*nfolders).wrapping_add(1);
                        *nfolders;
                    } else {
                        depthleft += 1;
                        depthleft;
                    }
                    dirpath[(*dirpathindex.offset(depthleft as isize))
                        .wrapping_sub(1 as libc::c_int as size_t)
                        as usize] = '/' as i32 as libc::c_char;
                    dirpath[*dirpathindex.offset(depthleft as isize)
                        as usize] = 0 as libc::c_int as libc::c_char;
                } else {
                    numlumps = numlumps.wrapping_add(1);
                    numlumps;
                }
            }
            if !(numlumps as libc::c_int == 65535 as libc::c_int) {
                continue;
            }
            *nlmp = 65535 as libc::c_int as uint16_t;
            failure = true_0 as libc::c_int;
            break;
        }
    }
    if numlumps == 0 {
        *nlmp = 0 as libc::c_int as uint16_t;
        failure = true_0 as libc::c_int;
    }
    if failure != 0 {
        while depthleft < 48 as libc::c_int {
            let fresh8 = depthleft;
            depthleft = depthleft + 1;
            closedir(*dirhandle.offset(fresh8 as isize));
        }
        free(dirpathindex as *mut libc::c_void);
        free(dirhandle as *mut libc::c_void);
        return 0 as *mut lumpinfo_t;
    }
    rewinddir(*dirhandle.offset(rootdir as isize));
    depthleft = rootdir;
    strlcpy(dirpath.as_mut_ptr(), path, 1024 as libc::c_int as libc::c_ulong);
    initdirpath(dirpath.as_mut_ptr(), dirpathindex, depthleft);
    lumpinfo = Z_CallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lumpinfo_t>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut lumpinfo_t;
    lump_p = lumpinfo;
    while depthleft < 48 as libc::c_int {
        let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut trimname: *mut libc::c_char = 0 as *mut libc::c_char;
        dirpath[*dirpathindex.offset(depthleft as isize)
            as usize] = 0 as libc::c_int as libc::c_char;
        dent = readdir(*dirhandle.offset(depthleft as isize));
        if dent.is_null() {
            let fresh9 = depthleft;
            depthleft = depthleft + 1;
            closedir(*dirhandle.offset(fresh9 as isize));
        } else {
            if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
                && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                    == '\0' as i32
                    || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int
                        == '.' as i32
                        && (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                            == '\0' as i32)
            {
                continue;
            }
            strcpy(
                &mut *dirpath
                    .as_mut_ptr()
                    .offset(*dirpathindex.offset(depthleft as isize) as isize),
                ((*dent).d_name).as_mut_ptr(),
            );
            if stat(dirpath.as_mut_ptr(), &mut fsstat) < 0 as libc::c_int {
                continue;
            }
            if fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
                == 0o40000 as libc::c_int as __mode_t && depthleft != 0
            {
                depthleft -= 1;
                *dirpathindex
                    .offset(
                        depthleft as isize,
                    ) = (strlen(dirpath.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                let ref mut fresh10 = *dirhandle.offset(depthleft as isize);
                *fresh10 = opendir(dirpath.as_mut_ptr());
                if !(*dirhandle.offset(depthleft as isize)).is_null() {
                    dirpath[(*dirpathindex.offset(depthleft as isize))
                        .wrapping_sub(1 as libc::c_int as size_t)
                        as usize] = '/' as i32 as libc::c_char;
                    dirpath[*dirpathindex.offset(depthleft as isize)
                        as usize] = 0 as libc::c_int as libc::c_char;
                } else {
                    depthleft += 1;
                    depthleft;
                }
            } else {
                (*lump_p).diskpath = Z_StrDup(dirpath.as_mut_ptr());
                (*lump_p).compression = CM_NOCOMPRESSION;
                fullname = (*lump_p).diskpath;
                if !(strstr(fullname, path)).is_null() {
                    fullname = fullname
                        .offset(
                            (strlen(path))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                        );
                }
                trimname = strrchr(fullname, '/' as i32);
                if !trimname.is_null() {
                    trimname = trimname.offset(1);
                    trimname;
                } else {
                    trimname = fullname;
                }
                if *trimname.offset(0 as libc::c_int as isize) != 0 {
                    let mut dotpos: *mut libc::c_char = strrchr(trimname, '.' as i32);
                    if dotpos.is_null() {
                        dotpos = fullname.offset(strlen(fullname) as isize);
                    }
                    strncpy(
                        ((*lump_p).name).as_mut_ptr(),
                        trimname,
                        (if (8 as libc::c_int as libc::c_long)
                            < dotpos.offset_from(trimname) as libc::c_long
                        {
                            8 as libc::c_int as libc::c_long
                        } else {
                            dotpos.offset_from(trimname) as libc::c_long
                        }) as libc::c_ulong,
                    );
                    (*lump_p)
                        .longname = Z_CallocAlign(
                        (dotpos.offset_from(trimname) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as size_t,
                        PU_STATIC as libc::c_int,
                        0 as *mut libc::c_void,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            as int32_t,
                    ) as *mut libc::c_char;
                    strlcpy(
                        (*lump_p).longname,
                        trimname,
                        (dotpos.offset_from(trimname) as libc::c_long
                            + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                    );
                } else {
                    (*lump_p)
                        .longname = Z_CallocAlign(
                        1 as libc::c_int as size_t,
                        PU_STATIC as libc::c_int,
                        0 as *mut libc::c_void,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            as int32_t,
                    ) as *mut libc::c_char;
                }
                (*lump_p).fullname = Z_StrDup(fullname);
                (*lump_p)
                    .hash = quickncasehash(
                    ((*lump_p).name).as_mut_ptr(),
                    8 as libc::c_int as size_t,
                );
                lump_p = lump_p.offset(1);
                lump_p;
                i = i.wrapping_add(1);
                i;
                if !(i as libc::c_int > numlumps as libc::c_int
                    || i as libc::c_int == 65535 as libc::c_int - 1 as libc::c_int)
                {
                    continue;
                }
                while depthleft < 48 as libc::c_int {
                    let fresh11 = depthleft;
                    depthleft = depthleft + 1;
                    closedir(*dirhandle.offset(fresh11 as isize));
                }
                break;
            }
        }
    }
    free(dirpathindex as *mut libc::c_void);
    free(dirhandle as *mut libc::c_void);
    *nlmp = numlumps;
    return lumpinfo;
}
#[no_mangle]
pub static mut exttable: [[libc::c_char; 7]; 6] = unsafe {
    [
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.txt\0\0"),
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.cfg\0\0"),
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.wad\0\0"),
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.pk3\0\0"),
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.soc\0\0"),
        *::core::mem::transmute::<&[u8; 7], &mut [libc::c_char; 7]>(b"\x05.lua\0\0"),
    ]
};
static mut filenamebuf: *mut [libc::c_char; 512] = 0 as *const [libc::c_char; 512]
    as *mut [libc::c_char; 512];
unsafe extern "C" fn filemenucmp(
    mut haystack: *mut libc::c_char,
    mut needle: *mut libc::c_char,
) -> boolean {
    static mut localhaystack: [libc::c_char; 128] = [0; 128];
    strlcpy(localhaystack.as_mut_ptr(), haystack, 128 as libc::c_int as libc::c_ulong);
    if cv_addons_search_case.value == 0 {
        strupr(localhaystack.as_mut_ptr());
    }
    if cv_addons_search_type.value != 0 {
        return (strstr(localhaystack.as_mut_ptr(), needle) != 0 as *mut libc::c_char)
            as libc::c_int;
    }
    return (strncmp(
        localhaystack.as_mut_ptr(),
        needle,
        menusearch[0 as libc::c_int as usize] as libc::c_ulong,
    ) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn closefilemenu(mut validsize: boolean) {
    if !dirmenu.is_null() {
        if dirmenu != coredirmenu {
            if !(*dirmenu.offset(0 as libc::c_int as isize)).is_null()
                && *(*dirmenu.offset(0 as libc::c_int as isize))
                    .offset(DIR_TYPE as libc::c_int as isize) as uint8_t as libc::c_int
                    == EXT_NORESULTS as libc::c_int
            {
                Z_Free(*dirmenu.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                let ref mut fresh12 = *dirmenu.offset(0 as libc::c_int as isize);
                *fresh12 = 0 as *mut libc::c_char;
            }
            Z_Free(dirmenu as *mut libc::c_void);
        }
        dirmenu = 0 as *mut *mut libc::c_char;
        sizedirmenu = 0 as libc::c_int as size_t;
    }
    if !coredirmenu.is_null() {
        if validsize != 0 {
            while sizecoredirmenu > 0 as libc::c_int as size_t {
                Z_Free(
                    *coredirmenu
                        .offset(
                            sizecoredirmenu.wrapping_sub(1 as libc::c_int as size_t)
                                as isize,
                        ) as *mut libc::c_void,
                );
                let ref mut fresh13 = *coredirmenu
                    .offset(
                        sizecoredirmenu.wrapping_sub(1 as libc::c_int as size_t) as isize,
                    );
                *fresh13 = 0 as *mut libc::c_char;
                sizecoredirmenu = sizecoredirmenu.wrapping_sub(1);
                sizecoredirmenu;
            }
        } else {
            sizecoredirmenu = 0 as libc::c_int as size_t;
        }
        Z_Free(coredirmenu as *mut libc::c_void);
        coredirmenu = 0 as *mut *mut libc::c_char;
    }
    if !refreshdirname.is_null() {
        Z_Free(refreshdirname as *mut libc::c_void);
    }
    refreshdirname = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn searchfilemenu(mut tempname: *mut libc::c_char) {
    let mut i: size_t = 0;
    let mut first: size_t = 0;
    let mut localmenusearch: [libc::c_char; 32] = *::core::mem::transmute::<
        &[u8; 32],
        &mut [libc::c_char; 32],
    >(b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    if !dirmenu.is_null() {
        if dirmenu != coredirmenu {
            if !(*dirmenu.offset(0 as libc::c_int as isize)).is_null()
                && *(*dirmenu.offset(0 as libc::c_int as isize))
                    .offset(DIR_TYPE as libc::c_int as isize) as uint8_t as libc::c_int
                    == EXT_NORESULTS as libc::c_int
            {
                Z_Free(*dirmenu.offset(0 as libc::c_int as isize) as *mut libc::c_void);
                let ref mut fresh14 = *dirmenu.offset(0 as libc::c_int as isize);
                *fresh14 = 0 as *mut libc::c_char;
            }
        } else {
            dirmenu = 0 as *mut *mut libc::c_char;
        }
    }
    first = (if *(*coredirmenu.offset(0 as libc::c_int as isize))
        .offset(DIR_TYPE as libc::c_int as isize) as uint8_t as libc::c_int
        == EXT_UP as libc::c_int
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as size_t;
    if menusearch[0 as libc::c_int as usize] == 0 {
        if !dirmenu.is_null() {
            Z_Free(dirmenu as *mut libc::c_void);
        }
        dirmenu = coredirmenu;
        sizedirmenu = sizecoredirmenu;
        if !tempname.is_null() {
            i = first;
            while i < sizedirmenu {
                if strcmp(
                    (*dirmenu.offset(i as isize))
                        .offset(DIR_STRING as libc::c_int as isize),
                    tempname,
                ) == 0
                {
                    dir_on[menudepthleft as usize] = i;
                    break;
                } else {
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if i == sizedirmenu {
                dir_on[menudepthleft as usize] = first;
            }
            Z_Free(tempname as *mut libc::c_void);
        }
        return;
    }
    strcpy(
        localmenusearch.as_mut_ptr(),
        menusearch.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    if cv_addons_search_case.value == 0 {
        strupr(localmenusearch.as_mut_ptr());
    }
    sizedirmenu = 0 as libc::c_int as size_t;
    i = first;
    while i < sizecoredirmenu {
        if filemenucmp(
            (*coredirmenu.offset(i as isize)).offset(DIR_STRING as libc::c_int as isize),
            localmenusearch.as_mut_ptr(),
        ) != 0
        {
            sizedirmenu = sizedirmenu.wrapping_add(1);
            sizedirmenu;
        }
        i = i.wrapping_add(1);
        i;
    }
    if sizedirmenu == 0 {
        dirmenu = Z_ReallocAlign(
            dirmenu as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut *mut libc::c_char;
        if dirmenu.is_null()
            || {
                let ref mut fresh15 = *dirmenu.offset(0 as libc::c_int as isize);
                *fresh15 = Z_StrDup(
                    va(
                        b"%c\x0BNo results...\0" as *const u8 as *const libc::c_char,
                        EXT_NORESULTS as libc::c_int,
                    ),
                );
                (*fresh15).is_null()
            }
        {
            I_Error(
                b"searchfilemenu(): could not create \"No results...\".\0" as *const u8
                    as *const libc::c_char,
            );
        }
        sizedirmenu = 1 as libc::c_int as size_t;
        dir_on[menudepthleft as usize] = 0 as libc::c_int as size_t;
        if !tempname.is_null() {
            Z_Free(tempname as *mut libc::c_void);
        }
        return;
    }
    dirmenu = Z_ReallocAlign(
        dirmenu as *mut libc::c_void,
        sizedirmenu
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut *mut libc::c_char;
    if dirmenu.is_null() {
        I_Error(
            b"searchfilemenu(): could not reallocate dirmenu.\0" as *const u8
                as *const libc::c_char,
        );
    }
    sizedirmenu = 0 as libc::c_int as size_t;
    i = first;
    while i < sizecoredirmenu {
        if filemenucmp(
            (*coredirmenu.offset(i as isize)).offset(DIR_STRING as libc::c_int as isize),
            localmenusearch.as_mut_ptr(),
        ) != 0
        {
            if !tempname.is_null()
                && strcmp(
                    (*coredirmenu.offset(i as isize))
                        .offset(DIR_STRING as libc::c_int as isize),
                    tempname,
                ) == 0
            {
                dir_on[menudepthleft as usize] = sizedirmenu;
                Z_Free(tempname as *mut libc::c_void);
                tempname = 0 as *mut libc::c_char;
            }
            let fresh16 = sizedirmenu;
            sizedirmenu = sizedirmenu.wrapping_add(1);
            let ref mut fresh17 = *dirmenu.offset(fresh16 as isize);
            *fresh17 = *coredirmenu.offset(i as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    if !tempname.is_null() {
        dir_on[menudepthleft as usize] = 0 as libc::c_int as size_t;
        Z_Free(tempname as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn preparefilemenu(mut samedepth: boolean) -> boolean {
    let mut dirhandle: *mut DIR = 0 as *mut DIR;
    let mut dent: *mut dirent = 0 as *mut dirent;
    let mut fsstat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut folderpos: size_t = 0 as libc::c_int as size_t;
    let mut numfolders: size_t = 0 as libc::c_int as size_t;
    let mut tempname: *mut libc::c_char = 0 as *mut libc::c_char;
    if samedepth != 0 {
        if !dirmenu.is_null()
            && !(*dirmenu.offset(dir_on[menudepthleft as usize] as isize)).is_null()
        {
            tempname = Z_StrDup(
                (*dirmenu.offset(dir_on[menudepthleft as usize] as isize))
                    .offset(DIR_STRING as libc::c_int as isize),
            );
        }
    } else {
        menusearch[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        menusearch[0 as libc::c_int as usize] = menusearch[1 as libc::c_int as usize];
    }
    dirhandle = opendir(menupath.as_mut_ptr());
    if dirhandle.is_null() {
        closefilemenu(true_0 as libc::c_int);
        return false_0 as libc::c_int;
    }
    while sizecoredirmenu > 0 as libc::c_int as size_t {
        Z_Free(
            *coredirmenu
                .offset(
                    sizecoredirmenu.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) as *mut libc::c_void,
        );
        let ref mut fresh18 = *coredirmenu
            .offset(sizecoredirmenu.wrapping_sub(1 as libc::c_int as size_t) as isize);
        *fresh18 = 0 as *mut libc::c_char;
        sizecoredirmenu = sizecoredirmenu.wrapping_sub(1);
        sizecoredirmenu;
    }
    while true_0 as libc::c_int != 0 {
        menupath[menupathindex[menudepthleft as usize]
            as usize] = 0 as libc::c_int as libc::c_char;
        dent = readdir(dirhandle);
        if dent.is_null() {
            break;
        }
        if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
            && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
                || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
                    && (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32)
        {
            continue;
        }
        strcpy(
            &mut *menupath
                .as_mut_ptr()
                .offset(
                    *menupathindex.as_mut_ptr().offset(menudepthleft as isize) as isize,
                ),
            ((*dent).d_name).as_mut_ptr(),
        );
        if stat(menupath.as_mut_ptr(), &mut fsstat) < 0 as libc::c_int {
            continue;
        }
        if !(fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
            == 0o40000 as libc::c_int as __mode_t)
        {
            if cv_addons_showall.value == 0 {
                let mut len: size_t = (strlen(((*dent).d_name).as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                let mut ext: uint8_t = 0;
                ext = 0 as libc::c_int as uint8_t;
                while (ext as libc::c_int) < NUM_EXT_TABLE as libc::c_int {
                    if strcasecmp(
                        (exttable[ext as usize])
                            .as_mut_ptr()
                            .offset(1 as libc::c_int as isize),
                        ((*dent).d_name)
                            .as_mut_ptr()
                            .offset(len as isize)
                            .offset(
                                -(exttable[ext as usize][0 as libc::c_int as usize]
                                    as libc::c_int as isize),
                            ),
                    ) == 0
                    {
                        break;
                    }
                    ext = ext.wrapping_add(1);
                    ext;
                }
                if ext as libc::c_int == NUM_EXT_TABLE as libc::c_int {
                    continue;
                }
            }
        } else {
            numfolders = numfolders.wrapping_add(1);
            numfolders;
        }
        sizecoredirmenu = sizecoredirmenu.wrapping_add(1);
        sizecoredirmenu;
    }
    if sizecoredirmenu == 0 {
        closedir(dirhandle);
        closefilemenu(false_0 as libc::c_int);
        if !tempname.is_null() {
            Z_Free(tempname as *mut libc::c_void);
        }
        return false_0 as libc::c_int;
    }
    if menudepthleft != (20 as libc::c_int - 1 as libc::c_int) as size_t {
        sizecoredirmenu = sizecoredirmenu.wrapping_add(1);
        sizecoredirmenu;
        numfolders = numfolders.wrapping_add(1);
        numfolders;
        folderpos = folderpos.wrapping_add(1);
        folderpos;
    }
    if !dirmenu.is_null() && dirmenu == coredirmenu {
        dirmenu = 0 as *mut *mut libc::c_char;
    }
    coredirmenu = Z_ReallocAlign(
        coredirmenu as *mut libc::c_void,
        sizecoredirmenu
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut *mut libc::c_char;
    if coredirmenu.is_null() {
        closedir(dirhandle);
        I_Error(
            b"preparefilemenu(): could not reallocate coredirmenu.\0" as *const u8
                as *const libc::c_char,
        );
    }
    rewinddir(dirhandle);
    while pos.wrapping_add(folderpos) < sizecoredirmenu {
        menupath[menupathindex[menudepthleft as usize]
            as usize] = 0 as libc::c_int as libc::c_char;
        dent = readdir(dirhandle);
        if dent.is_null() {
            break;
        }
        if (*dent).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32
            && ((*dent).d_name[1 as libc::c_int as usize] as libc::c_int == '\0' as i32
                || (*dent).d_name[1 as libc::c_int as usize] as libc::c_int == '.' as i32
                    && (*dent).d_name[2 as libc::c_int as usize] as libc::c_int
                        == '\0' as i32)
        {
            continue;
        }
        strcpy(
            &mut *menupath
                .as_mut_ptr()
                .offset(
                    *menupathindex.as_mut_ptr().offset(menudepthleft as isize) as isize,
                ),
            ((*dent).d_name).as_mut_ptr(),
        );
        if stat(menupath.as_mut_ptr(), &mut fsstat) < 0 as libc::c_int {
            continue;
        }
        let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len_0: size_t = (strlen(((*dent).d_name).as_mut_ptr()))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        let mut ext_0: uint8_t = EXT_FOLDER as libc::c_int as uint8_t;
        let mut folder: uint8_t = 0;
        if !(fsstat.st_mode & 0o170000 as libc::c_int as __mode_t
            == 0o40000 as libc::c_int as __mode_t)
        {
            if !(numfolders.wrapping_add(pos) < sizecoredirmenu) {
                continue;
            }
            while (ext_0 as libc::c_int) < NUM_EXT_TABLE as libc::c_int {
                if strcasecmp(
                    (exttable[ext_0 as usize])
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                    ((*dent).d_name)
                        .as_mut_ptr()
                        .offset(len_0 as isize)
                        .offset(
                            -(exttable[ext_0 as usize][0 as libc::c_int as usize]
                                as libc::c_int as isize),
                        ),
                ) == 0
                {
                    break;
                }
                ext_0 = ext_0.wrapping_add(1);
                ext_0;
            }
            if ext_0 as libc::c_int == NUM_EXT_TABLE as libc::c_int
                && cv_addons_showall.value == 0
            {
                continue;
            }
            ext_0 = (ext_0 as libc::c_int + EXT_START as libc::c_int) as uint8_t;
            if ext_0 as libc::c_int >= EXT_LOADSTART as libc::c_int {
                let mut i: size_t = 0;
                if filenamebuf.is_null() {
                    filenamebuf = calloc(
                        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                            .wrapping_mul(512 as libc::c_int as libc::c_ulong),
                        numwadfiles as libc::c_ulong,
                    ) as *mut [libc::c_char; 512];
                }
                i = 0 as libc::c_int as size_t;
                while i < numwadfiles as size_t {
                    if (*filenamebuf.offset(i as isize))[0 as libc::c_int as usize] == 0
                    {
                        strncpy(
                            (*filenamebuf.offset(i as isize)).as_mut_ptr(),
                            (**wadfiles.offset(i as isize)).filename,
                            512 as libc::c_int as libc::c_ulong,
                        );
                        (*filenamebuf
                            .offset(
                                i as isize,
                            ))[(512 as libc::c_int - 1 as libc::c_int)
                            as usize] = '\0' as i32 as libc::c_char;
                        nameonly((*filenamebuf.offset(i as isize)).as_mut_ptr());
                    }
                    if !(strcmp(
                        ((*dent).d_name).as_mut_ptr(),
                        (*filenamebuf.offset(i as isize)).as_mut_ptr(),
                    ) != 0)
                    {
                        if !(cv_addons_md5.value != 0
                            && checkfilemd5(
                                menupath.as_mut_ptr(),
                                ((**wadfiles.offset(i as isize)).md5sum).as_mut_ptr(),
                            ) as u64 == 0)
                        {
                            ext_0 = (ext_0 as libc::c_int | EXT_LOADED as libc::c_int)
                                as uint8_t;
                        }
                    }
                    i = i.wrapping_add(1);
                    i;
                }
            } else if ext_0 as libc::c_int == EXT_TXT as libc::c_int {
                if strncmp(
                    ((*dent).d_name).as_mut_ptr(),
                    b"log-\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0
                    || strcmp(
                        ((*dent).d_name).as_mut_ptr(),
                        b"errorlog.txt\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    ext_0 = (ext_0 as libc::c_int | EXT_LOADED as libc::c_int)
                        as uint8_t;
                }
            }
            if strcmp(((*dent).d_name).as_mut_ptr(), configfile.as_mut_ptr()) == 0 {
                ext_0 = (ext_0 as libc::c_int | EXT_LOADED as libc::c_int) as uint8_t;
            }
            folder = 0 as libc::c_int as uint8_t;
        } else {
            folder = 1 as libc::c_int as uint8_t;
            len_0 = len_0.wrapping_add(folder as size_t);
        }
        if len_0 > 255 as libc::c_int as size_t {
            len_0 = 255 as libc::c_int as size_t;
        }
        temp = Z_MallocAlign(
            len_0
                .wrapping_add(DIR_STRING as libc::c_int as size_t)
                .wrapping_add(folder as size_t)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        if temp.is_null() {
            I_Error(
                b"preparefilemenu(): could not create file entry.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *temp.offset(DIR_TYPE as libc::c_int as isize) = ext_0 as libc::c_char;
        *temp.offset(DIR_LEN as libc::c_int as isize) = len_0 as uint8_t as libc::c_char;
        strlcpy(
            temp.offset(DIR_STRING as libc::c_int as isize),
            ((*dent).d_name).as_mut_ptr(),
            len_0,
        );
        if folder != 0 {
            strcpy(
                temp.offset(len_0 as isize),
                b"/\0" as *const u8 as *const libc::c_char,
            );
            let fresh19 = folderpos;
            folderpos = folderpos.wrapping_add(1);
            let ref mut fresh20 = *coredirmenu.offset(fresh19 as isize);
            *fresh20 = temp;
        } else {
            let fresh21 = pos;
            pos = pos.wrapping_add(1);
            let ref mut fresh22 = *coredirmenu
                .offset(numfolders.wrapping_add(fresh21) as isize);
            *fresh22 = temp;
        }
    }
    if !filenamebuf.is_null() {
        free(filenamebuf as *mut libc::c_void);
        filenamebuf = 0 as *mut [libc::c_char; 512];
    }
    closedir(dirhandle);
    if menudepthleft != (20 as libc::c_int - 1 as libc::c_int) as size_t
        && {
            let ref mut fresh23 = *coredirmenu.offset(0 as libc::c_int as isize);
            *fresh23 = Z_StrDup(
                va(
                    b"%c\x05UP...\0" as *const u8 as *const libc::c_char,
                    EXT_UP as libc::c_int,
                ),
            );
            (*fresh23).is_null()
        }
    {
        I_Error(
            b"preparefilemenu(): could not create \"UP...\".\0" as *const u8
                as *const libc::c_char,
        );
    }
    menupath[menupathindex[menudepthleft as usize]
        as usize] = 0 as libc::c_int as libc::c_char;
    sizecoredirmenu = numfolders.wrapping_add(pos);
    if sizecoredirmenu == 0 {
        dir_on[menudepthleft as usize] = 0 as libc::c_int as size_t;
        closefilemenu(false_0 as libc::c_int);
        return false_0 as libc::c_int;
    }
    searchfilemenu(tempname);
    return true_0 as libc::c_int;
}
