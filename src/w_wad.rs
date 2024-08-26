use ::libc;
extern "C" {
    pub type internal_state;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
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
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn lzf_decompress(
        in_data: *const libc::c_void,
        in_len: size_t,
        out_data: *mut libc::c_void,
        out_len: size_t,
    ) -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strupr(n: *mut libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn sizeu2(num: size_t) -> *mut libc::c_char;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn CONS_Debug(debugflags: int32_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn DEH_LoadDehackedLumpPwad(wad: uint16_t, lump: uint16_t, mainfile: boolean);
    static mut modifiedgame: boolean;
    static mut mainwads: uint16_t;
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
    fn Z_ChangeTag(ptr: *mut libc::c_void, tag: int32_t);
    fn Z_StrDup(in_0: *const libc::c_char) -> *mut libc::c_char;
    static mut refreshdirmenu: uint8_t;
    static mut refreshdirname: *mut libc::c_char;
    static mut dirmenu: *mut *mut libc::c_char;
    fn getdirectoryfiles(
        path: *const libc::c_char,
        nlmp: *mut uint16_t,
        nfolders: *mut uint16_t,
    ) -> *mut lumpinfo_t;
    fn samepaths(path1: *const libc::c_char, path2: *const libc::c_char) -> int32_t;
    fn concatpaths(path: *const libc::c_char, startpath: *const libc::c_char) -> int32_t;
    fn pathisdirectory(path: *const libc::c_char) -> int32_t;
    static mut direrror: libc::c_int;
    fn Patch_Create(
        source: *mut softwarepatch_t,
        srcsize: size_t,
        dest: *mut libc::c_void,
    ) -> *mut patch_t;
    fn Picture_PNGConvert(
        png: *const uint8_t,
        outformat: pictureformat_t,
        w: *mut int32_t,
        h: *mut int32_t,
        topoffset: *mut int16_t,
        leftoffset: *mut int16_t,
        insize: size_t,
        outsize: *mut size_t,
        flags: pictureflags_t,
    ) -> *mut libc::c_void;
    fn Picture_IsLumpPNG(d: *const uint8_t, s: size_t) -> boolean;
    fn LUA_LoadLump(wad: uint16_t, lump: uint16_t, noresults: boolean);
    fn nameonly(s: *mut libc::c_char);
    fn findfile(
        filename: *mut libc::c_char,
        wantedmd5sum: *const uint8_t,
        completepath: boolean,
    ) -> filestatus_t;
    static mut srb2home: [libc::c_char; 256];
    static mut srb2path: [libc::c_char; 256];
    fn I_GetTime() -> tic_t;
    fn md5_stream(stream: *mut FILE, resblock: *mut libc::c_void) -> libc::c_int;
    fn M_FileError(handle: *mut FILE) -> *const libc::c_char;
    fn M_IsStringEmpty(s: *const libc::c_char) -> boolean;
    fn G_SetGameModified(silent: boolean);
}
pub type size_t = libc::c_ulong;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type lumpnum_t = uint32_t;
pub type tic_t = uint32_t;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct filelump_t {
    pub filepos: uint32_t,
    pub size: uint32_t,
    pub name: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wadinfo_t {
    pub identification: [libc::c_char; 4],
    pub numlumps: uint32_t,
    pub infotableofs: uint32_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct virtlump_t {
    pub name: [libc::c_char; 9],
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct virtres_t {
    pub numlumps: size_t,
    pub vlumps: *mut virtlump_t,
}
pub const PU_LEVEL: C2RustUnnamed_1 = 50;
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
pub const PU_STATIC: C2RustUnnamed_1 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addfilelist_t {
    pub files: *mut *mut libc::c_char,
    pub numfiles: size_t,
}
pub type filestatus_t = libc::c_uint;
pub const FS_MD5SUMBAD: filestatus_t = 6;
pub const FS_OPEN: filestatus_t = 5;
pub const FS_DOWNLOADING: filestatus_t = 4;
pub const FS_REQUESTED: filestatus_t = 3;
pub const FS_FOUND: filestatus_t = 2;
pub const FS_NOTFOUND: filestatus_t = 1;
pub const FS_NOTCHECKED: filestatus_t = 0;
pub type lumpnum_cache_t = lumpnum_cache_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lumpnum_cache_s {
    pub lumpname: [libc::c_char; 32],
    pub lumpnum: lumpnum_t,
}
pub type zlentry_t = zlentry_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zlentry_s {
    pub signature: [libc::c_char; 4],
    pub versionneeded: uint16_t,
    pub flags: uint16_t,
    pub compression: uint16_t,
    pub modtime: uint16_t,
    pub moddate: uint16_t,
    pub CRC32: uint32_t,
    pub compsize: uint32_t,
    pub size: uint32_t,
    pub namelen: uint16_t,
    pub xtralen: uint16_t,
}
pub type zentry_t = zentry_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zentry_s {
    pub signature: [libc::c_char; 4],
    pub version: uint16_t,
    pub versionneeded: uint16_t,
    pub flags: uint16_t,
    pub compression: uint16_t,
    pub modtime: uint16_t,
    pub moddate: uint16_t,
    pub CRC32: uint32_t,
    pub compsize: uint32_t,
    pub size: uint32_t,
    pub namelen: uint16_t,
    pub xtralen: uint16_t,
    pub commlen: uint16_t,
    pub diskstart: uint16_t,
    pub attrint: uint16_t,
    pub attrext: uint32_t,
    pub offset: uint32_t,
}
pub type zend_t = zend_s;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct zend_s {
    pub signature: [libc::c_char; 4],
    pub diskpos: uint16_t,
    pub cdirdisk: uint16_t,
    pub diskentries: uint16_t,
    pub entries: uint16_t,
    pub cdirsize: uint32_t,
    pub cdiroffset: uint32_t,
    pub commentlen: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lumpchecklist_t {
    pub name: *const libc::c_char,
    pub len: size_t,
}
pub const REFRESHDIR_MAX: C2RustUnnamed_2 = 32;
pub const REFRESHDIR_ADDFILE: C2RustUnnamed_2 = 2;
pub const REFRESHDIR_NORMAL: C2RustUnnamed_2 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct patch_t {
    pub width: int16_t,
    pub height: int16_t,
    pub leftoffset: int16_t,
    pub topoffset: int16_t,
    pub columnofs: *mut int32_t,
    pub columns: *mut uint8_t,
    pub hardware: *mut libc::c_void,
    pub flats: [*mut libc::c_void; 4],
    pub rotated: *mut rotsprite_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct softwarepatch_t {
    pub width: int16_t,
    pub height: int16_t,
    pub leftoffset: int16_t,
    pub topoffset: int16_t,
    pub columnofs: [int32_t; 8],
}
pub type pictureflags_t = libc::c_uint;
pub const PICFLAGS_YFLIP: pictureflags_t = 2;
pub const PICFLAGS_XFLIP: pictureflags_t = 1;
pub type pictureformat_t = libc::c_uint;
pub const PICFMT_DOOMPATCH32: pictureformat_t = 10;
pub const PICFMT_FLAT32: pictureformat_t = 9;
pub const PICFMT_PATCH32: pictureformat_t = 8;
pub const PICFMT_DOOMPATCH16: pictureformat_t = 7;
pub const PICFMT_FLAT16: pictureformat_t = 6;
pub const PICFMT_PATCH16: pictureformat_t = 5;
pub const PICFMT_PNG: pictureformat_t = 4;
pub const PICFMT_DOOMPATCH: pictureformat_t = 3;
pub const PICFMT_FLAT: pictureformat_t = 2;
pub const PICFMT_PATCH: pictureformat_t = 1;
pub const PICFMT_NONE: pictureformat_t = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_1 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_1 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_1 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_1 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_1 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_1 = 51;
pub const PU_CACHE: C2RustUnnamed_1 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_1 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_1 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_1 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_1 = 21;
pub const PU_HUDGFX: C2RustUnnamed_1 = 19;
pub const PU_SPRITE: C2RustUnnamed_1 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_1 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_1 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_1 = 15;
pub const PU_PATCH: C2RustUnnamed_1 = 14;
pub const PU_MUSIC: C2RustUnnamed_1 = 12;
pub const PU_SOUND: C2RustUnnamed_1 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_1 = 3;
pub const PU_LUA: C2RustUnnamed_1 = 2;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const REFRESHDIR_NOTLOADED: C2RustUnnamed_2 = 16;
pub const REFRESHDIR_ERROR: C2RustUnnamed_2 = 8;
pub const REFRESHDIR_WARNING: C2RustUnnamed_2 = 4;
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
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline(always)]
unsafe extern "C" fn fastcmp(
    mut s: *const libc::c_char,
    mut c: *const libc::c_char,
) -> boolean {
    while *s as libc::c_int != 0 && *s as libc::c_int == *c as libc::c_int {
        s = s.offset(1);
        s;
        c = c.offset(1);
        c;
    }
    return (*s as libc::c_int == *c as libc::c_int) as libc::c_int;
}
static mut lumpnumcache: [lumpnum_cache_t; 64] = [lumpnum_cache_s {
    lumpname: [0; 32],
    lumpnum: 0,
}; 64];
static mut lumpnumcacheindex: uint16_t = 0 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut numwadfiles: uint16_t = 0;
#[no_mangle]
pub static mut wadfiles: *mut *mut wadfile_t = 0 as *const *mut wadfile_t
    as *mut *mut wadfile_t;
#[no_mangle]
pub unsafe extern "C" fn W_Shutdown() {
    loop {
        let fresh0 = numwadfiles;
        numwadfiles = numwadfiles.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        let mut wad: *mut wadfile_t = *wadfiles.offset(numwadfiles as isize);
        if !((*wad).handle).is_null() {
            fclose((*wad).handle);
        }
        Z_Free((*wad).filename as *mut libc::c_void);
        if !((*wad).path).is_null() {
            Z_Free((*wad).path as *mut libc::c_void);
        }
        loop {
            let fresh1 = (*wad).numlumps;
            (*wad).numlumps = ((*wad).numlumps).wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            if !((*((*wad).lumpinfo).offset((*wad).numlumps as isize)).diskpath)
                .is_null()
            {
                Z_Free(
                    (*((*wad).lumpinfo).offset((*wad).numlumps as isize)).diskpath
                        as *mut libc::c_void,
                );
            }
            Z_Free(
                (*((*wad).lumpinfo).offset((*wad).numlumps as isize)).longname
                    as *mut libc::c_void,
            );
            Z_Free(
                (*((*wad).lumpinfo).offset((*wad).numlumps as isize)).fullname
                    as *mut libc::c_void,
            );
        }
        Z_Free((*wad).lumpinfo as *mut libc::c_void);
        Z_Free(wad as *mut libc::c_void);
    }
    Z_Free(wadfiles as *mut libc::c_void);
}
static mut filenamebuf: [libc::c_char; 512] = [0; 512];
#[no_mangle]
pub unsafe extern "C" fn W_OpenWadFile(
    mut filename: *mut *const libc::c_char,
    mut useerrors: boolean,
) -> *mut FILE {
    let mut handle: *mut FILE = 0 as *mut FILE;
    if filenamebuf.as_mut_ptr() != *filename as *mut libc::c_char {
        strncpy(
            filenamebuf.as_mut_ptr(),
            *filename,
            512 as libc::c_int as libc::c_ulong,
        );
        filenamebuf[(512 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
        *filename = filenamebuf.as_mut_ptr();
    }
    handle = fopen(*filename, b"rb\0" as *const u8 as *const libc::c_char);
    if handle.is_null() {
        nameonly(filenamebuf.as_mut_ptr());
        if findfile(filenamebuf.as_mut_ptr(), 0 as *const uint8_t, true_0 as libc::c_int)
            as u64 != 0
        {
            handle = fopen(*filename, b"rb\0" as *const u8 as *const libc::c_char);
            if handle.is_null() {
                if useerrors != 0 {
                    CONS_Alert(
                        CONS_ERROR,
                        b"Can't open %s\n\0" as *const u8 as *const libc::c_char,
                        *filename,
                    );
                }
                return 0 as *mut FILE;
            }
        } else {
            if useerrors != 0 {
                CONS_Alert(
                    CONS_ERROR,
                    b"File %s not found.\n\0" as *const u8 as *const libc::c_char,
                    *filename,
                );
            }
            return 0 as *mut FILE;
        }
    }
    return handle;
}
unsafe extern "C" fn W_LoadDehackedLumpsPK3(
    mut wadnum: uint16_t,
    mut mainfile: boolean,
) {
    let mut posStart: uint16_t = 0;
    let mut posEnd: uint16_t = 0;
    posStart = W_CheckNumForFullNamePK3(
        b"Init.lua\0" as *const u8 as *const libc::c_char,
        wadnum,
        0 as libc::c_int as uint16_t,
    );
    if posStart as libc::c_int != 32767 as libc::c_int {
        LUA_LoadLump(wadnum, posStart, true_0 as libc::c_int);
    } else {
        posStart = W_CheckNumForFolderStartPK3(
            b"Lua/\0" as *const u8 as *const libc::c_char,
            wadnum,
            0 as libc::c_int as uint16_t,
        );
        if posStart as libc::c_int != 32767 as libc::c_int {
            posEnd = W_CheckNumForFolderEndPK3(
                b"Lua/\0" as *const u8 as *const libc::c_char,
                wadnum,
                posStart,
            );
            while (posStart as libc::c_int) < posEnd as libc::c_int {
                LUA_LoadLump(wadnum, posStart, true_0 as libc::c_int);
                posStart = posStart.wrapping_add(1);
                posStart;
            }
        }
    }
    posStart = W_CheckNumForFolderStartPK3(
        b"SOC/\0" as *const u8 as *const libc::c_char,
        wadnum,
        0 as libc::c_int as uint16_t,
    );
    if posStart as libc::c_int != 32767 as libc::c_int {
        posEnd = W_CheckNumForFolderEndPK3(
            b"SOC/\0" as *const u8 as *const libc::c_char,
            wadnum,
            posStart,
        );
        while (posStart as libc::c_int) < posEnd as libc::c_int {
            let mut lump_p: *mut lumpinfo_t = &mut *((**wadfiles.offset(wadnum as isize))
                .lumpinfo)
                .offset(posStart as isize) as *mut lumpinfo_t;
            let mut length: size_t = (strlen(
                (**wadfiles.offset(wadnum as isize)).filename,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*lump_p).fullname));
            let mut name: *mut libc::c_char = malloc(
                length.wrapping_add(1 as libc::c_int as size_t),
            ) as *mut libc::c_char;
            sprintf(
                name,
                b"%s|%s\0" as *const u8 as *const libc::c_char,
                (**wadfiles.offset(wadnum as isize)).filename,
                (*lump_p).fullname,
            );
            *name.offset(length as isize) = '\0' as i32 as libc::c_char;
            CONS_Printf(
                b"Loading SOC from %s\n\0" as *const u8 as *const libc::c_char,
                name,
            );
            DEH_LoadDehackedLumpPwad(wadnum, posStart, mainfile);
            free(name as *mut libc::c_void);
            posStart = posStart.wrapping_add(1);
            posStart;
        }
    }
}
unsafe extern "C" fn W_LoadDehackedLumps(mut wadnum: uint16_t, mut mainfile: boolean) {
    let mut lump: uint16_t = 0;
    let mut lump_p: *mut lumpinfo_t = (**wadfiles.offset(wadnum as isize)).lumpinfo;
    lump = 0 as libc::c_int as uint16_t;
    while (lump as libc::c_int)
        < (**wadfiles.offset(wadnum as isize)).numlumps as libc::c_int
    {
        if memcmp(
            ((*lump_p).name).as_mut_ptr() as *const libc::c_void,
            b"LUA_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            LUA_LoadLump(wadnum, lump, true_0 as libc::c_int);
        }
        lump = lump.wrapping_add(1);
        lump;
        lump_p = lump_p.offset(1);
        lump_p;
    }
    let mut lump_p_0: *mut lumpinfo_t = (**wadfiles.offset(wadnum as isize)).lumpinfo;
    lump = 0 as libc::c_int as uint16_t;
    while (lump as libc::c_int)
        < (**wadfiles.offset(wadnum as isize)).numlumps as libc::c_int
    {
        if memcmp(
            ((*lump_p_0).name).as_mut_ptr() as *const libc::c_void,
            b"SOC_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            let mut length: size_t = (strlen(
                (**wadfiles.offset(wadnum as isize)).filename,
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(strlen((*lump_p_0).fullname));
            let mut name: *mut libc::c_char = malloc(
                length.wrapping_add(1 as libc::c_int as size_t),
            ) as *mut libc::c_char;
            sprintf(
                name,
                b"%s|%s\0" as *const u8 as *const libc::c_char,
                (**wadfiles.offset(wadnum as isize)).filename,
                (*lump_p_0).fullname,
            );
            *name.offset(length as isize) = '\0' as i32 as libc::c_char;
            CONS_Printf(
                b"Loading SOC from %s\n\0" as *const u8 as *const libc::c_char,
                name,
            );
            DEH_LoadDehackedLumpPwad(wadnum, lump, mainfile);
            free(name as *mut libc::c_void);
        } else if memcmp(
            ((*lump_p_0).name).as_mut_ptr() as *const libc::c_void,
            b"MAINCFG\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            CONS_Printf(
                b"Loading main config from %s\n\0" as *const u8 as *const libc::c_char,
                (**wadfiles.offset(wadnum as isize)).filename,
            );
            DEH_LoadDehackedLumpPwad(wadnum, lump, mainfile);
        } else if memcmp(
            ((*lump_p_0).name).as_mut_ptr() as *const libc::c_void,
            b"OBJCTCFG\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            CONS_Printf(
                b"Loading object config from %s\n\0" as *const u8 as *const libc::c_char,
                (**wadfiles.offset(wadnum as isize)).filename,
            );
            DEH_LoadDehackedLumpPwad(wadnum, lump, mainfile);
        }
        lump = lump.wrapping_add(1);
        lump;
        lump_p_0 = lump_p_0.offset(1);
        lump_p_0;
    }
}
unsafe extern "C" fn W_MakeFileMD5(
    mut filename: *const libc::c_char,
    mut resblock: *mut libc::c_void,
) -> int32_t {
    let mut fhandle: *mut FILE = 0 as *mut FILE;
    fhandle = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if !fhandle.is_null() {
        let mut t: tic_t = I_GetTime();
        CONS_Debug(
            0x400 as libc::c_int,
            b"Making MD5 for %s\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
        if md5_stream(fhandle, resblock) == 1 as libc::c_int {
            fclose(fhandle);
            return 1 as libc::c_int;
        }
        CONS_Debug(
            0x400 as libc::c_int,
            b"MD5 calc for %s took %f seconds\n\0" as *const u8 as *const libc::c_char,
            filename,
            ((I_GetTime()).wrapping_sub(t) as libc::c_float
                / (35 as libc::c_int * 1 as libc::c_int) as libc::c_float)
                as libc::c_double,
        );
        fclose(fhandle);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn W_InvalidateLumpnumCache() {
    memset(
        lumpnumcache.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[lumpnum_cache_t; 64]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn ResourceFileDetect(mut filename: *const libc::c_char) -> restype_t {
    if strcasecmp(
        &*filename
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(filename))
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ),
        b".pk3\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return RET_PK3;
    }
    if strcasecmp(
        &*filename
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(filename))
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ),
        b".soc\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return RET_SOC;
    }
    if strcasecmp(
        &*filename
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(filename))
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ),
        b".lua\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return RET_LUA;
    }
    return RET_WAD;
}
unsafe extern "C" fn ResGetLumpsStandalone(
    mut handle: *mut FILE,
    mut numlumps: *mut uint16_t,
    mut lumpname: *const libc::c_char,
) -> *mut lumpinfo_t {
    let mut lumpinfo: *mut lumpinfo_t = Z_CallocAlign(
        ::core::mem::size_of::<lumpinfo_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut lumpinfo_t;
    (*lumpinfo).position = 0 as libc::c_int as libc::c_ulong;
    fseek(handle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    (*lumpinfo).size = ftell(handle) as size_t;
    fseek(handle, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
    strcpy(((*lumpinfo).name).as_mut_ptr(), lumpname);
    (*lumpinfo).hash = quickncasehash(lumpname, 8 as libc::c_int as size_t);
    (*lumpinfo)
        .longname = Z_MallocAlign(
        (9 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    strcpy((*lumpinfo).longname, lumpname);
    *((*lumpinfo).longname)
        .offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    (*lumpinfo)
        .fullname = Z_MallocAlign(
        (9 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    strcpy((*lumpinfo).fullname, lumpname);
    *((*lumpinfo).fullname)
        .offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    *numlumps = 1 as libc::c_int as uint16_t;
    return lumpinfo;
}
unsafe extern "C" fn ResGetLumpsWad(
    mut handle: *mut FILE,
    mut nlmp: *mut uint16_t,
    mut filename: *const libc::c_char,
) -> *mut lumpinfo_t {
    let mut numlumps: uint16_t = *nlmp;
    let mut lumpinfo: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut i: size_t = 0;
    let mut compressed: int32_t = 0 as libc::c_int;
    let mut header: wadinfo_t = wadinfo_t {
        identification: [0; 4],
        numlumps: 0,
        infotableofs: 0,
    };
    let mut lump_p: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut fileinfo: *mut filelump_t = 0 as *mut filelump_t;
    let mut fileinfov: *mut libc::c_void = 0 as *mut libc::c_void;
    if fread(
        &mut header as *mut wadinfo_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<wadinfo_t>() as libc::c_ulong,
        handle,
    ) < ::core::mem::size_of::<wadinfo_t>() as libc::c_ulong
    {
        CONS_Alert(
            CONS_ERROR,
            b"Can't read wad header because %s\n\0" as *const u8 as *const libc::c_char,
            M_FileError(handle),
        );
        return 0 as *mut lumpinfo_t;
    }
    if memcmp(
        (header.identification).as_mut_ptr() as *const libc::c_void,
        b"ZWAD\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        compressed = 1 as libc::c_int;
    } else if memcmp(
        (header.identification).as_mut_ptr() as *const libc::c_void,
        b"IWAD\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && memcmp(
            (header.identification).as_mut_ptr() as *const libc::c_void,
            b"PWAD\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        && memcmp(
            (header.identification).as_mut_ptr() as *const libc::c_void,
            b"SDLL\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        CONS_Alert(
            CONS_ERROR,
            b"Invalid WAD header\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut lumpinfo_t;
    }
    header.numlumps = header.numlumps as int32_t as uint32_t;
    header.infotableofs = header.infotableofs as int32_t as uint32_t;
    i = (header.numlumps as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<filelump_t>() as libc::c_ulong);
    fileinfo = malloc(i) as *mut filelump_t;
    fileinfov = fileinfo as *mut libc::c_void;
    if fseek(handle, header.infotableofs as libc::c_long, 0 as libc::c_int)
        == -(1 as libc::c_int)
        || fread(
            fileinfo as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            i,
            handle,
        ) < i
    {
        CONS_Alert(
            CONS_ERROR,
            b"Corrupt wadfile directory (%s)\n\0" as *const u8 as *const libc::c_char,
            M_FileError(handle),
        );
        free(fileinfov);
        return 0 as *mut lumpinfo_t;
    }
    numlumps = header.numlumps as uint16_t;
    lumpinfo = Z_MallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lumpinfo_t>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut lumpinfo_t;
    lump_p = lumpinfo;
    i = 0 as libc::c_int as size_t;
    while i < numlumps as size_t {
        (*lump_p).position = (*fileinfo).filepos as int32_t as libc::c_ulong;
        (*lump_p).disksize = (*fileinfo).size as int32_t as libc::c_ulong;
        (*lump_p).size = (*lump_p).disksize;
        (*lump_p).diskpath = 0 as *mut libc::c_char;
        if compressed != 0 {
            let mut realsize: uint32_t = 0 as libc::c_int as uint32_t;
            if fseek(handle, (*lump_p).position as libc::c_long, 0 as libc::c_int)
                == -(1 as libc::c_int)
                || fread(
                    &mut realsize as *mut uint32_t as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                    handle,
                ) < ::core::mem::size_of::<uint32_t>() as libc::c_ulong
            {
                I_Error(
                    b"corrupt compressed file: %s; maybe %s\0" as *const u8
                        as *const libc::c_char,
                    filename,
                    M_FileError(handle),
                );
            }
            realsize = realsize as int32_t as uint32_t;
            if realsize != 0 as libc::c_int as uint32_t {
                (*lump_p).size = realsize as size_t;
                (*lump_p).compression = CM_LZF;
            } else {
                (*lump_p)
                    .size = ((*lump_p).size).wrapping_sub(4 as libc::c_int as size_t);
                (*lump_p).compression = CM_NOCOMPRESSION;
            }
            (*lump_p)
                .position = ((*lump_p).position)
                .wrapping_add(4 as libc::c_int as libc::c_ulong);
            (*lump_p)
                .disksize = ((*lump_p).disksize)
                .wrapping_sub(4 as libc::c_int as libc::c_ulong);
        } else {
            (*lump_p).compression = CM_NOCOMPRESSION;
        }
        memset(
            ((*lump_p).name).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            9 as libc::c_int as libc::c_ulong,
        );
        strncpy(
            ((*lump_p).name).as_mut_ptr(),
            ((*fileinfo).name).as_mut_ptr(),
            8 as libc::c_int as libc::c_ulong,
        );
        (*lump_p)
            .hash = quickncasehash(
            ((*lump_p).name).as_mut_ptr(),
            8 as libc::c_int as size_t,
        );
        (*lump_p)
            .longname = Z_MallocAlign(
            (9 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        strncpy(
            (*lump_p).longname,
            ((*fileinfo).name).as_mut_ptr(),
            8 as libc::c_int as libc::c_ulong,
        );
        *((*lump_p).longname)
            .offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        (*lump_p)
            .fullname = Z_MallocAlign(
            (9 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        strncpy(
            (*lump_p).fullname,
            ((*fileinfo).name).as_mut_ptr(),
            8 as libc::c_int as libc::c_ulong,
        );
        *((*lump_p).fullname)
            .offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        i = i.wrapping_add(1);
        i;
        lump_p = lump_p.offset(1);
        lump_p;
        fileinfo = fileinfo.offset(1);
        fileinfo;
    }
    free(fileinfov);
    *nlmp = numlumps;
    return lumpinfo;
}
unsafe extern "C" fn ResFindSignature(
    mut handle: *mut FILE,
    mut endPat: *mut libc::c_char,
    mut startpos: uint32_t,
) -> boolean {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    fseek(handle, startpos as libc::c_long, 0 as libc::c_int);
    s = endPat;
    loop {
        c = fgetc(handle);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if *s as libc::c_int != c && s > endPat {
            s = endPat;
        }
        if *s as libc::c_int == c {
            s = s.offset(1);
            s;
            if *s as libc::c_int == 0 as libc::c_int {
                return true_0 as libc::c_int;
            }
        }
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn ResGetLumpsZip(
    mut handle: *mut FILE,
    mut nlmp: *mut uint16_t,
) -> *mut lumpinfo_t {
    let mut zend: zend_t = zend_s {
        signature: [0; 4],
        diskpos: 0,
        cdirdisk: 0,
        diskentries: 0,
        entries: 0,
        cdirsize: 0,
        cdiroffset: 0,
        commentlen: 0,
    };
    let mut zentry: zentry_t = zentry_s {
        signature: [0; 4],
        version: 0,
        versionneeded: 0,
        flags: 0,
        compression: 0,
        modtime: 0,
        moddate: 0,
        CRC32: 0,
        compsize: 0,
        size: 0,
        namelen: 0,
        xtralen: 0,
        commlen: 0,
        diskstart: 0,
        attrint: 0,
        attrext: 0,
        offset: 0,
    };
    let mut zlentry: zlentry_t = zlentry_s {
        signature: [0; 4],
        versionneeded: 0,
        flags: 0,
        compression: 0,
        modtime: 0,
        moddate: 0,
        CRC32: 0,
        compsize: 0,
        size: 0,
        namelen: 0,
        xtralen: 0,
    };
    let mut numlumps: uint16_t = *nlmp;
    let mut lumpinfo: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut lump_p: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut i: size_t = 0;
    let mut pat_central: [libc::c_char; 5] = [
        0x50 as libc::c_int as libc::c_char,
        0x4b as libc::c_int as libc::c_char,
        0x1 as libc::c_int as libc::c_char,
        0x2 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut pat_end: [libc::c_char; 5] = [
        0x50 as libc::c_int as libc::c_char,
        0x4b as libc::c_int as libc::c_char,
        0x5 as libc::c_int as libc::c_char,
        0x6 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    fseek(handle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    if ResFindSignature(
        handle,
        pat_end.as_mut_ptr(),
        (if 0 as libc::c_int as libc::c_long
            > ftell(handle) - (22 as libc::c_int + 65536 as libc::c_int) as libc::c_long
        {
            0 as libc::c_int as libc::c_long
        } else {
            ftell(handle) - (22 as libc::c_int + 65536 as libc::c_int) as libc::c_long
        }) as uint32_t,
    ) == 0
    {
        CONS_Alert(
            CONS_ERROR,
            b"Missing central directory\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut lumpinfo_t;
    }
    fseek(handle, -(4 as libc::c_int) as libc::c_long, 1 as libc::c_int);
    if fread(
        &mut zend as *mut zend_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<zend_t>() as libc::c_ulong,
        handle,
    ) < ::core::mem::size_of::<zend_t>() as libc::c_ulong
    {
        CONS_Alert(
            CONS_ERROR,
            b"Corrupt central directory (%s)\n\0" as *const u8 as *const libc::c_char,
            M_FileError(handle),
        );
        return 0 as *mut lumpinfo_t;
    }
    numlumps = zend.entries;
    lumpinfo = Z_MallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<lumpinfo_t>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut lumpinfo_t;
    lump_p = lumpinfo;
    fseek(handle, zend.cdiroffset as libc::c_long, 0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < numlumps as size_t {
        let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut trimname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dotpos: *mut libc::c_char = 0 as *mut libc::c_char;
        if fread(
            &mut zentry as *mut zentry_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<zentry_t>() as libc::c_ulong,
            handle,
        ) < ::core::mem::size_of::<zentry_t>() as libc::c_ulong
        {
            CONS_Alert(
                CONS_ERROR,
                b"Failed to read central directory (%s)\n\0" as *const u8
                    as *const libc::c_char,
                M_FileError(handle),
            );
            Z_Free(lumpinfo as *mut libc::c_void);
            return 0 as *mut lumpinfo_t;
        }
        if memcmp(
            (zentry.signature).as_mut_ptr() as *const libc::c_void,
            pat_central.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            CONS_Alert(
                CONS_ERROR,
                b"Central directory is corrupt\n\0" as *const u8 as *const libc::c_char,
            );
            Z_Free(lumpinfo as *mut libc::c_void);
            return 0 as *mut lumpinfo_t;
        }
        (*lump_p).position = zentry.offset as libc::c_ulong;
        (*lump_p).disksize = zentry.compsize as libc::c_ulong;
        (*lump_p).diskpath = 0 as *mut libc::c_char;
        (*lump_p).size = zentry.size as size_t;
        fullname = malloc(
            (zentry.namelen as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        ) as *mut libc::c_char;
        if fgets(fullname, zentry.namelen as libc::c_int + 1 as libc::c_int, handle)
            != fullname
        {
            CONS_Alert(
                CONS_ERROR,
                b"Unable to read lumpname (%s)\n\0" as *const u8 as *const libc::c_char,
                M_FileError(handle),
            );
            Z_Free(lumpinfo as *mut libc::c_void);
            free(fullname as *mut libc::c_void);
            return 0 as *mut lumpinfo_t;
        }
        trimname = strrchr(fullname, '/' as i32);
        if !trimname.is_null() {
            trimname = trimname.offset(1);
            trimname;
        } else {
            trimname = fullname;
        }
        dotpos = strrchr(trimname, '.' as i32);
        if dotpos.is_null() {
            dotpos = fullname.offset(strlen(fullname) as isize);
        }
        memset(
            ((*lump_p).name).as_mut_ptr() as *mut libc::c_void,
            '\0' as i32,
            9 as libc::c_int as libc::c_ulong,
        );
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
            .hash = quickncasehash(
            ((*lump_p).name).as_mut_ptr(),
            8 as libc::c_int as size_t,
        );
        (*lump_p)
            .longname = Z_CallocAlign(
            (dotpos.offset_from(trimname) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as size_t,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        strlcpy(
            (*lump_p).longname,
            trimname,
            (dotpos.offset_from(trimname) as libc::c_long
                + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
        );
        (*lump_p)
            .fullname = Z_CallocAlign(
            (zentry.namelen as libc::c_int + 1 as libc::c_int) as size_t,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        strncpy((*lump_p).fullname, fullname, zentry.namelen as libc::c_ulong);
        match zentry.compression as libc::c_int {
            0 => {
                (*lump_p).compression = CM_NOCOMPRESSION;
            }
            8 => {
                (*lump_p).compression = CM_DEFLATE;
            }
            14 => {
                (*lump_p).compression = CM_LZF;
            }
            _ => {
                CONS_Alert(
                    CONS_WARNING,
                    b"%s: Unsupported compression method\n\0" as *const u8
                        as *const libc::c_char,
                    fullname,
                );
                (*lump_p).compression = CM_UNSUPPORTED;
            }
        }
        free(fullname as *mut libc::c_void);
        if fseek(
            handle,
            (zentry.xtralen as libc::c_int + zentry.commlen as libc::c_int)
                as libc::c_long,
            1 as libc::c_int,
        ) != 0 as libc::c_int
        {
            CONS_Alert(
                CONS_ERROR,
                b"Central directory is corrupt\n\0" as *const u8 as *const libc::c_char,
            );
            Z_Free(lumpinfo as *mut libc::c_void);
            return 0 as *mut lumpinfo_t;
        }
        i = i.wrapping_add(1);
        i;
        lump_p = lump_p.offset(1);
        lump_p;
    }
    i = 0 as libc::c_int as size_t;
    lump_p = lumpinfo;
    while i < numlumps as size_t {
        if fseek(handle, (*lump_p).position as libc::c_long, 0 as libc::c_int)
            != 0 as libc::c_int
            || fread(
                &mut zlentry as *mut zlentry_t as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<zlentry_t>() as libc::c_ulong,
                handle,
            ) < ::core::mem::size_of::<zlentry_t>() as libc::c_ulong
        {
            CONS_Alert(
                CONS_ERROR,
                b"Local headers for lump %s are corrupt\n\0" as *const u8
                    as *const libc::c_char,
                (*lump_p).fullname,
            );
            Z_Free(lumpinfo as *mut libc::c_void);
            return 0 as *mut lumpinfo_t;
        }
        (*lump_p)
            .position = ((*lump_p).position)
            .wrapping_add(
                (::core::mem::size_of::<zlentry_t>() as libc::c_ulong)
                    .wrapping_add(zlentry.namelen as libc::c_ulong)
                    .wrapping_add(zlentry.xtralen as libc::c_ulong),
            );
        i = i.wrapping_add(1);
        i;
        lump_p = lump_p.offset(1);
        lump_p;
    }
    *nlmp = numlumps;
    return lumpinfo;
}
unsafe extern "C" fn CheckPathsNotEqual(
    mut path1: *const libc::c_char,
    mut path2: *const libc::c_char,
) -> int32_t {
    let mut stat: int32_t = samepaths(path1, path2);
    if stat == 1 as libc::c_int {
        return 0 as libc::c_int
    } else if stat < 0 as libc::c_int {
        return -(1 as libc::c_int)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn W_IsPathToFolderValid(
    mut path: *const libc::c_char,
) -> int32_t {
    let mut stat: int32_t = 0;
    let mut p: *const libc::c_char = path
        .offset((strlen(path)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    while *p as libc::c_int == '\\' as i32 || *p as libc::c_int == '/' as i32
        || *p as libc::c_int == ':' as i32
    {
        p = p.offset(-1);
        p;
        if p < path {
            return 0 as libc::c_int;
        }
    }
    stat = pathisdirectory(path);
    if stat == 0 as libc::c_int {
        return 0 as libc::c_int
    } else if stat < 0 as libc::c_int {
        if direrror == 2 as libc::c_int {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    stat = CheckPathsNotEqual(path, srb2home.as_mut_ptr());
    if stat != 1 as libc::c_int {
        return stat;
    }
    stat = CheckPathsNotEqual(path, srb2path.as_mut_ptr());
    if stat != 1 as libc::c_int {
        return stat;
    }
    stat = CheckPathsNotEqual(path, b".\0" as *const u8 as *const libc::c_char);
    if stat != 1 as libc::c_int {
        return stat;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn CheckConcatFolderPath(
    mut startpath: *const libc::c_char,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    if concatpaths(path, startpath) == 1 as libc::c_int {
        let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
        if !startpath.is_null() {
            let mut len: size_t = (strlen(startpath))
                .wrapping_add(strlen(path))
                .wrapping_add(strlen(b"/\0" as *const u8 as *const libc::c_char))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            fn_0 = Z_MallocAlign(
                len,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut libc::c_char;
            snprintf(
                fn_0,
                len,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                startpath,
                path,
            );
        } else {
            fn_0 = Z_StrDup(path);
        }
        return fn_0;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn W_GetFullFolderPath(
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    let mut fn_0: *mut libc::c_char = CheckConcatFolderPath(
        0 as *const libc::c_char,
        path,
    );
    if !fn_0.is_null() {
        return fn_0;
    }
    fn_0 = CheckConcatFolderPath(srb2home.as_mut_ptr(), path);
    if !fn_0.is_null() {
        return fn_0;
    }
    fn_0 = CheckConcatFolderPath(srb2path.as_mut_ptr(), path);
    if !fn_0.is_null() {
        return fn_0;
    }
    fn_0 = CheckConcatFolderPath(b".\0" as *const u8 as *const libc::c_char, path);
    if !fn_0.is_null() {
        return fn_0;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn ResGetLumpsFolder(
    mut path: *const libc::c_char,
    mut nlmp: *mut uint16_t,
    mut nfolders: *mut uint16_t,
) -> *mut lumpinfo_t {
    return getdirectoryfiles(path, nlmp, nfolders);
}
unsafe extern "C" fn W_InitFileError(
    mut filename: *const libc::c_char,
    mut exitworthy: boolean,
) -> uint16_t {
    if exitworthy != 0 {
        I_Error(
            b"%s was not found or not valid.\nCheck the log for more details.\n\0"
                as *const u8 as *const libc::c_char,
            filename,
        );
    } else {
        CONS_Printf(
            b"Errors occurred while loading %s; not added.\n\0" as *const u8
                as *const libc::c_char,
            filename,
        );
    }
    return 32767 as libc::c_int as uint16_t;
}
unsafe extern "C" fn W_ReadFileShaders(mut wadfile: *mut wadfile_t) {}
#[no_mangle]
pub unsafe extern "C" fn W_InitFile(
    mut filename: *const libc::c_char,
    mut mainfile: boolean,
    mut startup: boolean,
) -> uint16_t {
    let mut handle: *mut FILE = 0 as *mut FILE;
    let mut lumpinfo: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut wadfile: *mut wadfile_t = 0 as *mut wadfile_t;
    let mut type_0: restype_t = RET_WAD;
    let mut numlumps: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: size_t = 0;
    let mut md5sum: [uint8_t; 16] = [0; 16];
    let mut important: libc::c_int = 0;
    if refreshdirmenu as libc::c_int & REFRESHDIR_ADDFILE as libc::c_int == 0 {
        refreshdirmenu = (REFRESHDIR_NORMAL as libc::c_int
            | REFRESHDIR_ADDFILE as libc::c_int) as uint8_t;
    }
    if !refreshdirname.is_null() {
        Z_Free(refreshdirname as *mut libc::c_void);
    }
    if !dirmenu.is_null() {
        refreshdirname = Z_StrDup(filename);
        nameonly(refreshdirname);
    } else {
        refreshdirname = 0 as *mut libc::c_char;
    }
    if numwadfiles as libc::c_int >= 65535 as libc::c_int {
        CONS_Alert(
            CONS_ERROR,
            b"Maximum wad files reached\n\0" as *const u8 as *const libc::c_char,
        );
        refreshdirmenu = (refreshdirmenu as libc::c_int | REFRESHDIR_MAX as libc::c_int)
            as uint8_t;
        return W_InitFileError(filename, startup);
    }
    handle = W_OpenWadFile(&mut filename, true_0 as libc::c_int);
    if handle.is_null() {
        return W_InitFileError(filename, startup);
    }
    important = W_VerifyNMUSlumps(filename, startup);
    if important == -(1 as libc::c_int) {
        fclose(handle);
        return 32767 as libc::c_int as uint16_t;
    }
    important = (important == 0) as libc::c_int;
    W_MakeFileMD5(filename, md5sum.as_mut_ptr() as *mut libc::c_void);
    i = 0 as libc::c_int as size_t;
    while i < numwadfiles as size_t {
        if !((**wadfiles.offset(i as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint)
        {
            if memcmp(
                ((**wadfiles.offset(i as isize)).md5sum).as_mut_ptr()
                    as *const libc::c_void,
                md5sum.as_mut_ptr() as *const libc::c_void,
                16 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                CONS_Alert(
                    CONS_ERROR,
                    b"%s is already loaded\n\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                if !handle.is_null() {
                    fclose(handle);
                }
                return W_InitFileError(filename, false_0 as libc::c_int);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    type_0 = ResourceFileDetect(filename);
    match type_0 as libc::c_uint {
        1 => {
            lumpinfo = ResGetLumpsStandalone(
                handle,
                &mut numlumps,
                b"OBJCTCFG\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            lumpinfo = ResGetLumpsStandalone(
                handle,
                &mut numlumps,
                b"LUA_INIT\0" as *const u8 as *const libc::c_char,
            );
        }
        3 => {
            lumpinfo = ResGetLumpsZip(handle, &mut numlumps);
        }
        0 => {
            lumpinfo = ResGetLumpsWad(handle, &mut numlumps, filename);
        }
        _ => {
            CONS_Alert(
                CONS_ERROR,
                b"Unsupported file format\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if lumpinfo.is_null() {
        fclose(handle);
        return W_InitFileError(filename, startup);
    }
    if important != 0 && mainfile == 0 {
        modifiedgame = true_0 as libc::c_int;
    }
    wadfile = Z_MallocAlign(
        ::core::mem::size_of::<wadfile_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut wadfile_t;
    (*wadfile).filename = Z_StrDup(filename);
    (*wadfile).path = 0 as *mut libc::c_char;
    (*wadfile).type_0 = type_0;
    (*wadfile).handle = handle;
    (*wadfile).numlumps = numlumps;
    (*wadfile).foldercount = 0 as libc::c_int as uint16_t;
    (*wadfile).lumpinfo = lumpinfo;
    (*wadfile).important = important;
    fseek(handle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    (*wadfile).filesize = ftell(handle) as libc::c_uint;
    (*wadfile).type_0 = type_0;
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        &mut (*wadfile).md5sum as *mut [uint8_t; 16] as *mut libc::c_void,
        &mut md5sum as *mut [uint8_t; 16] as *const libc::c_void,
        16 as libc::c_int as size_t,
    );
    Z_CallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        &mut (*wadfile).lumpcache as *mut *mut *mut libc::c_void as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    Z_CallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        &mut (*wadfile).patchcache as *mut *mut *mut libc::c_void as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    CONS_Printf(
        b"Added file %s (%u lumps)\n\0" as *const u8 as *const libc::c_char,
        filename,
        numlumps as libc::c_int,
    );
    wadfiles = Z_ReallocAlign(
        wadfiles as *mut libc::c_void,
        (::core::mem::size_of::<*mut wadfile_t>() as libc::c_ulong)
            .wrapping_mul(
                (numwadfiles as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            ),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut *mut wadfile_t;
    let ref mut fresh2 = *wadfiles.offset(numwadfiles as isize);
    *fresh2 = wadfile;
    numwadfiles = numwadfiles.wrapping_add(1);
    numwadfiles;
    W_ReadFileShaders(wadfile);
    match (*wadfile).type_0 as libc::c_uint {
        0 => {
            W_LoadDehackedLumps(
                (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
                mainfile,
            );
        }
        3 => {
            W_LoadDehackedLumpsPK3(
                (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
                mainfile,
            );
        }
        1 => {
            CONS_Printf(
                b"Loading SOC from %s\n\0" as *const u8 as *const libc::c_char,
                (*wadfile).filename,
            );
            DEH_LoadDehackedLumpPwad(
                (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
                0 as libc::c_int as uint16_t,
                mainfile,
            );
        }
        2 => {
            LUA_LoadLump(
                (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
                0 as libc::c_int as uint16_t,
                true_0 as libc::c_int,
            );
        }
        _ => {}
    }
    W_InvalidateLumpnumCache();
    return (*wadfile).numlumps;
}
#[no_mangle]
pub unsafe extern "C" fn W_InitFolder(
    mut path: *const libc::c_char,
    mut mainfile: boolean,
    mut startup: boolean,
) -> uint16_t {
    let mut lumpinfo: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut wadfile: *mut wadfile_t = 0 as *mut wadfile_t;
    let mut numlumps: uint16_t = 0 as libc::c_int as uint16_t;
    let mut foldercount: uint16_t = 0;
    let mut i: size_t = 0;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fullpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut important: libc::c_int = 0;
    let mut stat: int32_t = 0;
    if refreshdirmenu as libc::c_int & REFRESHDIR_ADDFILE as libc::c_int == 0 {
        refreshdirmenu = (REFRESHDIR_NORMAL as libc::c_int
            | REFRESHDIR_ADDFILE as libc::c_int) as uint8_t;
    }
    if !refreshdirname.is_null() {
        Z_Free(refreshdirname as *mut libc::c_void);
    }
    if !dirmenu.is_null() {
        refreshdirname = Z_StrDup(path);
    } else {
        refreshdirname = 0 as *mut libc::c_char;
    }
    if numwadfiles as libc::c_int >= 65535 as libc::c_int {
        CONS_Alert(
            CONS_ERROR,
            b"Maximum wad files reached\n\0" as *const u8 as *const libc::c_char,
        );
        refreshdirmenu = (refreshdirmenu as libc::c_int | REFRESHDIR_MAX as libc::c_int)
            as uint8_t;
        return W_InitFileError(path, startup);
    }
    important = 1 as libc::c_int;
    p = path
        .offset((strlen(path)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    while *p as libc::c_int == '\\' as i32 || *p as libc::c_int == '/' as i32
        || *p as libc::c_int == ':' as i32
    {
        p = p.offset(-1);
        p;
        if p < path {
            CONS_Alert(
                CONS_ERROR,
                b"Path %s is invalid\n\0" as *const u8 as *const libc::c_char,
                path,
            );
            return W_InitFileError(path, startup);
        }
    }
    p = p.offset(1);
    p;
    i = (p.offset_from(path) as libc::c_long + 1 as libc::c_int as libc::c_long)
        as size_t;
    fn_0 = Z_MallocAlign(
        i,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    strlcpy(fn_0, path, i);
    if M_IsStringEmpty(fn_0) != 0 {
        CONS_Alert(
            CONS_ERROR,
            b"Folder name is empty\n\0" as *const u8 as *const libc::c_char,
        );
        Z_Free(fn_0 as *mut libc::c_void);
        if startup != 0 {
            return W_InitFileError(
                b"A folder\0" as *const u8 as *const libc::c_char,
                true_0 as libc::c_int,
            )
        } else {
            return W_InitFileError(
                b"a folder\0" as *const u8 as *const libc::c_char,
                false_0 as libc::c_int,
            )
        }
    }
    stat = W_IsPathToFolderValid(fn_0);
    if stat != 1 as libc::c_int {
        if stat == 0 as libc::c_int {
            CONS_Alert(
                CONS_ERROR,
                b"Path %s is invalid\n\0" as *const u8 as *const libc::c_char,
                fn_0,
            );
        } else if stat < 0 as libc::c_int {
            CONS_Alert(
                CONS_ERROR,
                b"Could not stat %s: %s\n\0" as *const u8 as *const libc::c_char,
                fn_0,
                strerror(direrror),
            );
        }
        Z_Free(fn_0 as *mut libc::c_void);
        return W_InitFileError(path, startup);
    }
    fullpath = W_GetFullFolderPath(fn_0);
    if fullpath.is_null() {
        CONS_Alert(
            CONS_ERROR,
            b"Path %s is invalid\n\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
        Z_Free(fn_0 as *mut libc::c_void);
        return W_InitFileError(path, startup);
    }
    i = 0 as libc::c_int as size_t;
    while i < numwadfiles as size_t {
        if !((**wadfiles.offset(i as isize)).type_0 as libc::c_uint
            != RET_FOLDER as libc::c_int as libc::c_uint)
        {
            if samepaths((**wadfiles.offset(i as isize)).path, fullpath)
                > 0 as libc::c_int
            {
                CONS_Alert(
                    CONS_ERROR,
                    b"%s is already loaded\n\0" as *const u8 as *const libc::c_char,
                    path,
                );
                Z_Free(fn_0 as *mut libc::c_void);
                Z_Free(fullpath as *mut libc::c_void);
                return W_InitFileError(path, false_0 as libc::c_int);
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    lumpinfo = ResGetLumpsFolder(fullpath, &mut numlumps, &mut foldercount);
    if lumpinfo.is_null() {
        if numlumps == 0 {
            CONS_Alert(
                CONS_ERROR,
                b"Folder %s is empty\n\0" as *const u8 as *const libc::c_char,
                path,
            );
        } else if numlumps as libc::c_int == 65535 as libc::c_int {
            CONS_Alert(
                CONS_ERROR,
                b"Folder %s contains too many files\n\0" as *const u8
                    as *const libc::c_char,
                path,
            );
        } else {
            CONS_Alert(
                CONS_ERROR,
                b"Unknown error enumerating files from folder %s\n\0" as *const u8
                    as *const libc::c_char,
                path,
            );
        }
        Z_Free(fn_0 as *mut libc::c_void);
        Z_Free(fullpath as *mut libc::c_void);
        return W_InitFileError(path, startup);
    }
    if important != 0 && mainfile == 0 {
        G_SetGameModified(true_0 as libc::c_int);
    }
    wadfile = Z_MallocAlign(
        ::core::mem::size_of::<wadfile_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut wadfile_t;
    (*wadfile).filename = fn_0;
    (*wadfile).path = fullpath;
    (*wadfile).type_0 = RET_FOLDER;
    (*wadfile).handle = 0 as *mut FILE;
    (*wadfile).numlumps = numlumps;
    (*wadfile).foldercount = foldercount;
    (*wadfile).lumpinfo = lumpinfo;
    (*wadfile).important = important;
    (*wadfile).filesize = 0 as libc::c_int as uint32_t;
    memset(
        ((*wadfile).md5sum).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    Z_CallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        &mut (*wadfile).lumpcache as *mut *mut *mut libc::c_void as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    Z_CallocAlign(
        (numlumps as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        &mut (*wadfile).patchcache as *mut *mut *mut libc::c_void as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    CONS_Printf(
        b"Added folder %s (%u files, %u folders)\n\0" as *const u8
            as *const libc::c_char,
        fn_0,
        numlumps as libc::c_int,
        foldercount as libc::c_int,
    );
    wadfiles = Z_ReallocAlign(
        wadfiles as *mut libc::c_void,
        (::core::mem::size_of::<*mut wadfile_t>() as libc::c_ulong)
            .wrapping_mul(
                (numwadfiles as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            ),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut *mut wadfile_t;
    let ref mut fresh3 = *wadfiles.offset(numwadfiles as isize);
    *fresh3 = wadfile;
    numwadfiles = numwadfiles.wrapping_add(1);
    numwadfiles;
    W_ReadFileShaders(wadfile);
    W_LoadDehackedLumpsPK3(
        (numwadfiles as libc::c_int - 1 as libc::c_int) as uint16_t,
        mainfile,
    );
    W_InvalidateLumpnumCache();
    return (*wadfile).numlumps;
}
#[no_mangle]
pub unsafe extern "C" fn W_InitMultipleFiles(mut list: *mut addfilelist_t) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*list).numfiles {
        let mut fn_0: *const libc::c_char = *((*list).files).offset(i as isize);
        let mut pathsep: libc::c_char = *fn_0
            .offset(
                (strlen(fn_0)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        let mut mainfile: boolean = ((numwadfiles as libc::c_int)
            < mainwads as libc::c_int) as libc::c_int;
        if pathsep as libc::c_int == '\\' as i32 || pathsep as libc::c_int == '/' as i32
        {
            W_InitFolder(fn_0, mainfile, true_0 as libc::c_int);
        } else {
            W_InitFile(fn_0, mainfile, true_0 as libc::c_int);
        }
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn TestValidLump(mut wad: uint16_t, mut lump: uint16_t) -> boolean {
    if (*wadfiles.offset(wad as isize)).is_null() {
        return false_0 as libc::c_int;
    }
    if lump as libc::c_int >= (**wadfiles.offset(wad as isize)).numlumps as libc::c_int {
        return false_0 as libc::c_int;
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNameForNumPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
) -> *const libc::c_char {
    if lump as libc::c_int >= (**wadfiles.offset(wad as isize)).numlumps as libc::c_int
        || TestValidLump(wad, 0 as libc::c_int as uint16_t) == 0
    {
        return 0 as *const libc::c_char;
    }
    return ((*((**wadfiles.offset(wad as isize)).lumpinfo).offset(lump as isize)).name)
        .as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNameForNum(
    mut lumpnum: lumpnum_t,
) -> *const libc::c_char {
    return W_CheckNameForNumPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForNamePwad(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut i: uint16_t = 0;
    static mut uname: [libc::c_char; 9] = [0; 9];
    let mut hash: uint32_t = 0;
    if TestValidLump(wad, 0 as libc::c_int as uint16_t) == 0 {
        return 32767 as libc::c_int as uint16_t;
    }
    strlcpy(
        uname.as_mut_ptr(),
        name,
        ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
    );
    strupr(uname.as_mut_ptr());
    hash = quickncasehash(uname.as_mut_ptr(), 8 as libc::c_int as size_t);
    if (startlump as libc::c_int)
        < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int
    {
        let mut lump_p: *mut lumpinfo_t = ((**wadfiles.offset(wad as isize)).lumpinfo)
            .offset(startlump as libc::c_int as isize);
        i = startlump;
        while (i as libc::c_int)
            < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int
        {
            if (*lump_p).hash == hash
                && strncmp(
                    ((*lump_p).name).as_mut_ptr(),
                    uname.as_mut_ptr(),
                    (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) == 0
            {
                return i;
            }
            i = i.wrapping_add(1);
            i;
            lump_p = lump_p.offset(1);
            lump_p;
        }
    }
    return 32767 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForLongNamePwad(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut i: uint16_t = 0;
    static mut uname: [libc::c_char; 257] = [0; 257];
    if TestValidLump(wad, 0 as libc::c_int as uint16_t) == 0 {
        return 32767 as libc::c_int as uint16_t;
    }
    strlcpy(
        uname.as_mut_ptr(),
        name,
        ::core::mem::size_of::<[libc::c_char; 257]>() as libc::c_ulong,
    );
    strupr(uname.as_mut_ptr());
    if (startlump as libc::c_int)
        < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int
    {
        let mut lump_p: *mut lumpinfo_t = ((**wadfiles.offset(wad as isize)).lumpinfo)
            .offset(startlump as libc::c_int as isize);
        i = startlump;
        while (i as libc::c_int)
            < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int
        {
            if strcmp((*lump_p).longname, uname.as_mut_ptr()) == 0 {
                return i;
            }
            i = i.wrapping_add(1);
            i;
            lump_p = lump_p.offset(1);
            lump_p;
        }
    }
    return 32767 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForMarkerStartPwad(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut marker: uint16_t = 0;
    marker = W_CheckNumForNamePwad(name, wad, startlump);
    if marker as libc::c_int != 32767 as libc::c_int {
        marker = marker.wrapping_add(1);
        marker;
    }
    return marker;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForFolderStartPK3(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut name_length: size_t = 0;
    let mut i: int32_t = 0;
    let mut lump_p: *mut lumpinfo_t = ((**wadfiles.offset(wad as isize)).lumpinfo)
        .offset(startlump as libc::c_int as isize);
    name_length = strlen(name);
    i = startlump as int32_t;
    while i < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int {
        if strncasecmp(name, (*lump_p).fullname, name_length) == 0 as libc::c_int {
            if strlen((*lump_p).fullname) == name_length {
                i += 1;
                i;
            }
            break;
        } else {
            i += 1;
            i;
            lump_p = lump_p.offset(1);
            lump_p;
        }
    }
    return i as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForFolderEndPK3(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut i: int32_t = 0;
    let mut lump_p: *mut lumpinfo_t = ((**wadfiles.offset(wad as isize)).lumpinfo)
        .offset(startlump as libc::c_int as isize);
    i = startlump as int32_t;
    while i < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int {
        if strncasecmp(name, (*lump_p).fullname, strlen(name)) != 0 {
            break;
        }
        i += 1;
        i;
        lump_p = lump_p.offset(1);
        lump_p;
    }
    return i as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForFullNamePK3(
    mut name: *const libc::c_char,
    mut wad: uint16_t,
    mut startlump: uint16_t,
) -> uint16_t {
    let mut i: int32_t = 0;
    let mut lump_p: *mut lumpinfo_t = ((**wadfiles.offset(wad as isize)).lumpinfo)
        .offset(startlump as libc::c_int as isize);
    i = startlump as int32_t;
    while i < (**wadfiles.offset(wad as isize)).numlumps as libc::c_int {
        if strncasecmp(name, (*lump_p).fullname, strlen(name)) == 0 {
            return i as uint16_t;
        }
        i += 1;
        i;
        lump_p = lump_p.offset(1);
        lump_p;
    }
    return 32767 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForName(mut name: *const libc::c_char) -> lumpnum_t {
    let mut i: int32_t = 0;
    let mut check: lumpnum_t = 32767 as libc::c_int as lumpnum_t;
    if *name == 0 {
        return 4294967295 as libc::c_uint;
    }
    i = lumpnumcacheindex as libc::c_int + 64 as libc::c_int;
    while i > lumpnumcacheindex as libc::c_int {
        if lumpnumcache[(i & 64 as libc::c_int - 1 as libc::c_int) as usize]
            .lumpname[8 as libc::c_int as usize] == 0
            && strncmp(
                (lumpnumcache[(i & 64 as libc::c_int - 1 as libc::c_int) as usize]
                    .lumpname)
                    .as_mut_ptr(),
                name,
                8 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            lumpnumcacheindex = (i & 64 as libc::c_int - 1 as libc::c_int) as uint16_t;
            return lumpnumcache[lumpnumcacheindex as usize].lumpnum;
        }
        i -= 1;
        i;
    }
    i = numwadfiles as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        check = W_CheckNumForNamePwad(name, i as uint16_t, 0 as libc::c_int as uint16_t)
            as lumpnum_t;
        if check != 32767 as libc::c_int as lumpnum_t {
            break;
        }
        i -= 1;
        i;
    }
    if check == 32767 as libc::c_int as lumpnum_t {
        return 4294967295 as libc::c_uint
    } else {
        lumpnumcacheindex = (lumpnumcacheindex as libc::c_int + 1 as libc::c_int
            & 64 as libc::c_int - 1 as libc::c_int) as uint16_t;
        memset(
            (lumpnumcache[lumpnumcacheindex as usize].lumpname).as_mut_ptr()
                as *mut libc::c_void,
            '\0' as i32,
            32 as libc::c_int as libc::c_ulong,
        );
        strncpy(
            (lumpnumcache[lumpnumcacheindex as usize].lumpname).as_mut_ptr(),
            name,
            8 as libc::c_int as libc::c_ulong,
        );
        lumpnumcache[lumpnumcacheindex as usize]
            .lumpnum = ((i << 16 as libc::c_int) as lumpnum_t).wrapping_add(check);
        return lumpnumcache[lumpnumcacheindex as usize].lumpnum;
    };
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForLongName(
    mut name: *const libc::c_char,
) -> lumpnum_t {
    let mut i: int32_t = 0;
    let mut check: lumpnum_t = 32767 as libc::c_int as lumpnum_t;
    if *name == 0 {
        return 4294967295 as libc::c_uint;
    }
    i = lumpnumcacheindex as libc::c_int + 64 as libc::c_int;
    while i > lumpnumcacheindex as libc::c_int {
        if strcmp(
            (lumpnumcache[(i & 64 as libc::c_int - 1 as libc::c_int) as usize].lumpname)
                .as_mut_ptr(),
            name,
        ) == 0 as libc::c_int
        {
            lumpnumcacheindex = (i & 64 as libc::c_int - 1 as libc::c_int) as uint16_t;
            return lumpnumcache[lumpnumcacheindex as usize].lumpnum;
        }
        i -= 1;
        i;
    }
    i = numwadfiles as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        check = W_CheckNumForLongNamePwad(
            name,
            i as uint16_t,
            0 as libc::c_int as uint16_t,
        ) as lumpnum_t;
        if check != 32767 as libc::c_int as lumpnum_t {
            break;
        }
        i -= 1;
        i;
    }
    if check == 32767 as libc::c_int as lumpnum_t {
        return 4294967295 as libc::c_uint
    } else {
        if strlen(name) < 32 as libc::c_int as libc::c_ulong {
            lumpnumcacheindex = (lumpnumcacheindex as libc::c_int + 1 as libc::c_int
                & 64 as libc::c_int - 1 as libc::c_int) as uint16_t;
            memset(
                (lumpnumcache[lumpnumcacheindex as usize].lumpname).as_mut_ptr()
                    as *mut libc::c_void,
                '\0' as i32,
                32 as libc::c_int as libc::c_ulong,
            );
            strlcpy(
                (lumpnumcache[lumpnumcacheindex as usize].lumpname).as_mut_ptr(),
                name,
                32 as libc::c_int as libc::c_ulong,
            );
            lumpnumcache[lumpnumcacheindex as usize]
                .lumpnum = ((i << 16 as libc::c_int) as lumpnum_t).wrapping_add(check);
        }
        return ((i << 16 as libc::c_int) as lumpnum_t).wrapping_add(check);
    };
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForMap(mut name: *const libc::c_char) -> lumpnum_t {
    let mut hash: uint32_t = quickncasehash(name, 8 as libc::c_int as size_t);
    let mut lumpNum: uint16_t = 0;
    let mut end: uint16_t = 0;
    let mut i: uint32_t = 0;
    let mut p: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    i = (numwadfiles as libc::c_int - 1 as libc::c_int) as uint32_t;
    while i < numwadfiles as uint32_t {
        if (**wadfiles.offset(i as isize)).type_0 as libc::c_uint
            == RET_WAD as libc::c_int as libc::c_uint
        {
            lumpNum = 0 as libc::c_int as uint16_t;
            while (lumpNum as libc::c_int)
                < (**wadfiles.offset(i as isize)).numlumps as libc::c_int
            {
                p = ((**wadfiles.offset(i as isize)).lumpinfo)
                    .offset(lumpNum as libc::c_int as isize);
                if (*p).hash == hash
                    && strncmp(
                        name,
                        ((*p).name).as_mut_ptr(),
                        8 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    return (i << 16 as libc::c_int).wrapping_add(lumpNum as uint32_t);
                }
                lumpNum = lumpNum.wrapping_add(1);
                lumpNum;
            }
        } else if (**wadfiles.offset(i as isize)).type_0 as libc::c_uint
            == RET_PK3 as libc::c_int as libc::c_uint
            || (**wadfiles.offset(i as isize)).type_0 as libc::c_uint
                == RET_FOLDER as libc::c_int as libc::c_uint
        {
            lumpNum = W_CheckNumForFolderStartPK3(
                b"maps/\0" as *const u8 as *const libc::c_char,
                i as uint16_t,
                0 as libc::c_int as uint16_t,
            );
            if lumpNum as libc::c_int != 32767 as libc::c_int {
                end = W_CheckNumForFolderEndPK3(
                    b"maps/\0" as *const u8 as *const libc::c_char,
                    i as uint16_t,
                    lumpNum,
                );
                while (lumpNum as libc::c_int) < end as libc::c_int {
                    p = ((**wadfiles.offset(i as isize)).lumpinfo)
                        .offset(lumpNum as libc::c_int as isize);
                    if (*p).hash == hash
                        && strncasecmp(
                            name,
                            ((*p).name).as_mut_ptr(),
                            8 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        let mut extension: *const libc::c_char = strrchr(
                            (*p).fullname,
                            '.' as i32,
                        );
                        if !(!extension.is_null()
                            && strcasecmp(
                                extension,
                                b".wad\0" as *const u8 as *const libc::c_char,
                            ) != 0)
                        {
                            return (i << 16 as libc::c_int)
                                .wrapping_add(lumpNum as uint32_t);
                        }
                    }
                    lumpNum = lumpNum.wrapping_add(1);
                    lumpNum;
                }
            }
        }
        i = i.wrapping_sub(1);
        i;
    }
    return 4294967295 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn W_GetNumForName(mut name: *const libc::c_char) -> lumpnum_t {
    let mut i: lumpnum_t = 0;
    i = W_CheckNumForName(name);
    if i == 4294967295 as libc::c_uint {
        I_Error(
            b"W_GetNumForName: %s not found!\n\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn W_GetNumForLongName(
    mut name: *const libc::c_char,
) -> lumpnum_t {
    let mut i: lumpnum_t = 0;
    i = W_CheckNumForLongName(name);
    if i == 4294967295 as libc::c_uint {
        I_Error(
            b"W_GetNumForLongName: %s not found!\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn W_CheckNumForNameInBlock(
    mut name: *const libc::c_char,
    mut blockstart: *const libc::c_char,
    mut blockend: *const libc::c_char,
) -> lumpnum_t {
    let mut i: int32_t = 0;
    let mut bsid: lumpnum_t = 0;
    let mut beid: lumpnum_t = 0;
    let mut check: lumpnum_t = 32767 as libc::c_int as lumpnum_t;
    i = numwadfiles as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (**wadfiles.offset(i as isize)).type_0 as libc::c_uint
            == RET_WAD as libc::c_int as libc::c_uint
        {
            bsid = W_CheckNumForNamePwad(
                blockstart,
                i as uint16_t,
                0 as libc::c_int as uint16_t,
            ) as lumpnum_t;
            if !(bsid == 32767 as libc::c_int as lumpnum_t) {
                beid = W_CheckNumForNamePwad(
                    blockend,
                    i as uint16_t,
                    0 as libc::c_int as uint16_t,
                ) as lumpnum_t;
                if !(beid == 32767 as libc::c_int as lumpnum_t) {
                    check = W_CheckNumForNamePwad(name, i as uint16_t, bsid as uint16_t)
                        as lumpnum_t;
                    if check < beid {
                        return ((i << 16 as libc::c_int) as lumpnum_t)
                            .wrapping_add(check);
                    }
                }
            }
        }
        i -= 1;
        i;
    }
    return 4294967295 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn W_LumpExists(mut name: *const libc::c_char) -> uint8_t {
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    i = numwadfiles as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut lump_p: *mut lumpinfo_t = (**wadfiles.offset(i as isize)).lumpinfo;
        j = 0 as libc::c_int;
        while j < (**wadfiles.offset(i as isize)).numlumps as libc::c_int {
            if fastcmp((*lump_p).longname, name) != 0 {
                return true_0 as libc::c_int as uint8_t;
            }
            j += 1;
            j;
            lump_p = lump_p.offset(1);
            lump_p;
        }
        i -= 1;
        i;
    }
    return false_0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn W_LumpLengthPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
) -> size_t {
    let mut l: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    if TestValidLump(wad, lump) == 0 {
        return 0 as libc::c_int as size_t;
    }
    l = ((**wadfiles.offset(wad as isize)).lumpinfo)
        .offset(lump as libc::c_int as isize);
    if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
        == RET_FOLDER as libc::c_int as libc::c_uint
    {
        let mut stat: int32_t = pathisdirectory((*l).diskpath);
        if stat < 0 as libc::c_int {
            if direrror == 2 as libc::c_int {
                I_Error(
                    b"W_LumpLengthPwad: file %s doesn't exist\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                );
            } else {
                I_Error(
                    b"W_LumpLengthPwad: could not stat %s: %s\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                    strerror(direrror),
                );
            }
        } else if stat == 1 as libc::c_int {
            return 0 as libc::c_int as size_t
        } else {
            let mut handle: *mut FILE = fopen(
                (*l).diskpath,
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if handle.is_null() {
                I_Error(
                    b"W_LumpLengthPwad: could not open file %s\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                );
            }
            fseek(handle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
            (*l).disksize = ftell(handle) as libc::c_ulong;
            (*l).size = (*l).disksize;
            fclose(handle);
        }
    }
    return (*l).size;
}
#[no_mangle]
pub unsafe extern "C" fn W_LumpLength(mut lumpnum: lumpnum_t) -> size_t {
    return W_LumpLengthPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_IsLumpWad(mut lumpnum: lumpnum_t) -> boolean {
    if (**wadfiles.offset((lumpnum >> 16 as libc::c_int) as uint16_t as isize)).type_0
        as libc::c_uint == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset((lumpnum >> 16 as libc::c_int) as uint16_t as isize))
            .type_0 as libc::c_uint == RET_FOLDER as libc::c_int as libc::c_uint
    {
        let mut lumpfullName: *const libc::c_char = (*((**wadfiles
            .offset((lumpnum >> 16 as libc::c_int) as uint16_t as isize))
            .lumpinfo)
            .offset(
                (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t as libc::c_int
                    as isize,
            ))
            .fullname;
        if strlen(lumpfullName) < 4 as libc::c_int as libc::c_ulong {
            return false_0 as libc::c_int;
        }
        return (strncasecmp(
            lumpfullName
                .offset(strlen(lumpfullName) as isize)
                .offset(-(4 as libc::c_int as isize)),
            b".wad\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0) as libc::c_int;
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn W_IsLumpFolder(
    mut wad: uint16_t,
    mut lump: uint16_t,
) -> boolean {
    if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
        == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint
    {
        let mut name: *const libc::c_char = (*((**wadfiles.offset(wad as isize))
            .lumpinfo)
            .offset(lump as isize))
            .fullname;
        return (*name
            .offset(
                (strlen(name)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '/' as i32) as libc::c_int;
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zerr(mut ret: libc::c_int) {
    CONS_Printf(b"zpipe: \0" as *const u8 as *const libc::c_char);
    match ret {
        -1 => {
            if ferror(stdin) != 0 {
                CONS_Printf(
                    b"error reading stdin\n\0" as *const u8 as *const libc::c_char,
                );
            }
            if ferror(stdout) != 0 {
                CONS_Printf(
                    b"error writing stdout\n\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        -2 => {
            CONS_Printf(
                b"invalid compression level\n\0" as *const u8 as *const libc::c_char,
            );
        }
        -3 => {
            CONS_Printf(
                b"invalid or incomplete deflate data\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        -4 => {
            CONS_Printf(b"out of memory\n\0" as *const u8 as *const libc::c_char);
        }
        -6 => {
            CONS_Printf(
                b"zlib version mismatch!\n\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn W_ReadLumpHeaderPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut dest: *mut libc::c_void,
    mut size: size_t,
    mut offset: size_t,
) -> size_t {
    let mut lumpsize: size_t = 0;
    let mut bytesread: size_t = 0;
    let mut l: *mut lumpinfo_t = 0 as *mut lumpinfo_t;
    let mut handle: *mut FILE = 0 as *mut FILE;
    if TestValidLump(wad, lump) == 0 {
        return 0 as libc::c_int as size_t;
    }
    l = ((**wadfiles.offset(wad as isize)).lumpinfo)
        .offset(lump as libc::c_int as isize);
    if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
        == RET_FOLDER as libc::c_int as libc::c_uint
    {
        let mut stat: int32_t = pathisdirectory((*l).diskpath);
        if stat < 0 as libc::c_int {
            if direrror == 2 as libc::c_int {
                I_Error(
                    b"W_ReadLumpHeaderPwad: file %s doesn't exist\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                );
            } else {
                I_Error(
                    b"W_ReadLumpHeaderPwad: could not stat %s: %s\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                    strerror(direrror),
                );
            }
        } else if stat == 1 as libc::c_int {
            return 0 as libc::c_int as size_t
        } else {
            handle = fopen((*l).diskpath, b"rb\0" as *const u8 as *const libc::c_char);
            if handle.is_null() {
                I_Error(
                    b"W_ReadLumpHeaderPwad: could not open file %s\0" as *const u8
                        as *const libc::c_char,
                    (*l).diskpath,
                );
            }
            fseek(handle, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
            (*l).disksize = ftell(handle) as libc::c_ulong;
            (*l).size = (*l).disksize;
        }
    }
    lumpsize = (*((**wadfiles.offset(wad as isize)).lumpinfo).offset(lump as isize))
        .size;
    if lumpsize == 0 || lumpsize < offset {
        if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint
        {
            fclose(handle);
        }
        return 0 as libc::c_int as size_t;
    }
    if size == 0 || size.wrapping_add(offset) > lumpsize {
        size = lumpsize.wrapping_sub(offset);
    }
    if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
        != RET_FOLDER as libc::c_int as libc::c_uint
    {
        handle = (**wadfiles.offset(wad as isize)).handle;
    }
    fseek(
        handle,
        ((*l).position).wrapping_add(offset) as libc::c_long,
        0 as libc::c_int,
    );
    match (*((**wadfiles.offset(wad as isize)).lumpinfo).offset(lump as isize))
        .compression as libc::c_uint
    {
        0 => {
            bytesread = fread(dest, 1 as libc::c_int as libc::c_ulong, size, handle);
            if (**wadfiles.offset(wad as isize)).type_0 as libc::c_uint
                == RET_FOLDER as libc::c_int as libc::c_uint
            {
                fclose(handle);
            }
            return bytesread;
        }
        2 => {
            let mut rawData: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut decData: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut retval: size_t = 0;
            rawData = Z_MallocAlign(
                (*l).disksize,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut libc::c_char;
            decData = Z_MallocAlign(
                (*l).size,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut libc::c_char;
            if fread(
                rawData as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                (*l).disksize,
                handle,
            ) < (*l).disksize
            {
                I_Error(
                    b"wad %d, lump %d: cannot read compressed data\0" as *const u8
                        as *const libc::c_char,
                    wad as libc::c_int,
                    lump as libc::c_int,
                );
            }
            retval = lzf_decompress(
                rawData as *const libc::c_void,
                (*l).disksize,
                decData as *mut libc::c_void,
                (*l).size,
            );
            if retval == 0 as libc::c_int as size_t {
                if *__errno_location() == 7 as libc::c_int {
                    I_Error(
                        b"wad %d, lump %d: compressed data too big (bigger than %s)\0"
                            as *const u8 as *const libc::c_char,
                        wad as libc::c_int,
                        lump as libc::c_int,
                        sizeu1((*l).size),
                    );
                } else if *__errno_location() == 22 as libc::c_int {
                    I_Error(
                        b"wad %d, lump %d: invalid compressed data\0" as *const u8
                            as *const libc::c_char,
                        wad as libc::c_int,
                        lump as libc::c_int,
                    );
                }
            }
            if retval != (*l).size {
                I_Error(
                    b"wad %d, lump %d: decompressed to wrong number of bytes (expected %s, got %s)\0"
                        as *const u8 as *const libc::c_char,
                    wad as libc::c_int,
                    lump as libc::c_int,
                    sizeu1((*l).size),
                    sizeu2(retval),
                );
            }
            if decData.is_null() {
                return 0 as libc::c_int as size_t;
            }
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(dest, decData.offset(offset as isize) as *const libc::c_void, size);
            Z_Free(rawData as *mut libc::c_void);
            Z_Free(decData as *mut libc::c_void);
            return size;
        }
        1 => {
            let mut rawData_0: *mut uint8_t = 0 as *mut uint8_t;
            let mut decData_0: *mut uint8_t = 0 as *mut uint8_t;
            let mut zErr: libc::c_int = 0;
            let mut strm: z_stream = z_stream_s {
                next_in: 0 as *mut Bytef,
                avail_in: 0,
                total_in: 0,
                next_out: 0 as *mut Bytef,
                avail_out: 0,
                total_out: 0,
                msg: 0 as *mut libc::c_char,
                state: 0 as *mut internal_state,
                zalloc: None,
                zfree: None,
                opaque: 0 as *mut libc::c_void,
                data_type: 0,
                adler: 0,
                reserved: 0,
            };
            let mut rawSize: libc::c_ulong = (*l).disksize;
            let mut decSize: libc::c_ulong = (*l).size;
            rawData_0 = Z_MallocAlign(
                rawSize,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            decData_0 = Z_MallocAlign(
                decSize,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            if fread(
                rawData_0 as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                rawSize,
                handle,
            ) < rawSize
            {
                I_Error(
                    b"wad %d, lump %d: cannot read compressed data\0" as *const u8
                        as *const libc::c_char,
                    wad as libc::c_int,
                    lump as libc::c_int,
                );
            }
            strm.zalloc = None;
            strm.zfree = None;
            strm.opaque = 0 as voidpf;
            strm.avail_in = rawSize as uInt;
            strm.total_in = strm.avail_in as uLong;
            strm.avail_out = decSize as uInt;
            strm.total_out = strm.avail_out as uLong;
            strm.next_in = rawData_0;
            strm.next_out = decData_0;
            zErr = inflateInit2_(
                &mut strm,
                -(15 as libc::c_int),
                b"1.2.13\0" as *const u8 as *const libc::c_char,
                ::core::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if zErr == 0 as libc::c_int {
                zErr = inflate(&mut strm, 4 as libc::c_int);
                if zErr == 1 as libc::c_int {
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(dest, decData_0 as *const libc::c_void, size);
                } else {
                    size = 0 as libc::c_int as size_t;
                    zerr(zErr);
                }
                inflateEnd(&mut strm);
            } else {
                size = 0 as libc::c_int as size_t;
                zerr(zErr);
            }
            Z_Free(rawData_0 as *mut libc::c_void);
            Z_Free(decData_0 as *mut libc::c_void);
            return size;
        }
        _ => {
            I_Error(
                b"wad %d, lump %d: unsupported compression type!\0" as *const u8
                    as *const libc::c_char,
                wad as libc::c_int,
                lump as libc::c_int,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn W_ReadLumpHeader(
    mut lumpnum: lumpnum_t,
    mut dest: *mut libc::c_void,
    mut size: size_t,
    mut offset: size_t,
) -> size_t {
    return W_ReadLumpHeaderPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        dest,
        size,
        offset,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_ReadLump(
    mut lumpnum: lumpnum_t,
    mut dest: *mut libc::c_void,
) {
    W_ReadLumpHeaderPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        dest,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_ReadLumpPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut dest: *mut libc::c_void,
) {
    W_ReadLumpHeaderPwad(
        wad,
        lump,
        dest,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheLumpNumPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut lumpcache: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if TestValidLump(wad, lump) == 0 {
        return 0 as *mut libc::c_void;
    }
    lumpcache = (**wadfiles.offset(wad as isize)).lumpcache;
    if (*lumpcache.offset(lump as isize)).is_null() {
        let mut ptr: *mut libc::c_void = Z_MallocAlign(
            W_LumpLengthPwad(wad, lump),
            tag,
            &mut *lumpcache.offset(lump as isize) as *mut *mut libc::c_void
                as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        );
        W_ReadLumpHeaderPwad(
            wad,
            lump,
            ptr,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
    } else {
        Z_ChangeTag(*lumpcache.offset(lump as isize), tag);
    }
    return *lumpcache.offset(lump as isize);
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheLumpNum(
    mut lumpnum: lumpnum_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    return W_CacheLumpNumPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        tag,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheLumpNumForce(
    mut lumpnum: lumpnum_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut wad: uint16_t = 0;
    let mut lump: uint16_t = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    wad = (lumpnum >> 16 as libc::c_int) as uint16_t;
    lump = (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t;
    if TestValidLump(wad, lump) == 0 {
        return 0 as *mut libc::c_void;
    }
    ptr = Z_MallocAlign(
        W_LumpLengthPwad(wad, lump),
        tag,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    W_ReadLumpHeaderPwad(
        wad,
        lump,
        ptr,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
    );
    return ptr;
}
unsafe extern "C" fn W_IsLumpCachedPWAD(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut ptr: *mut libc::c_void,
) -> boolean {
    let mut lcache: *mut libc::c_void = 0 as *mut libc::c_void;
    if TestValidLump(wad, lump) == 0 {
        return false_0 as libc::c_int;
    }
    lcache = *((**wadfiles.offset(wad as isize)).lumpcache).offset(lump as isize);
    if !ptr.is_null() {
        if ptr == lcache {
            return true_0 as libc::c_int;
        }
    } else if !lcache.is_null() {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn W_IsLumpCached(
    mut lumpnum: lumpnum_t,
    mut ptr: *mut libc::c_void,
) -> boolean {
    return W_IsLumpCachedPWAD(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        ptr,
    );
}
unsafe extern "C" fn W_IsPatchCachedPWAD(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut ptr: *mut libc::c_void,
) -> boolean {
    let mut lcache: *mut libc::c_void = 0 as *mut libc::c_void;
    if TestValidLump(wad, lump) == 0 {
        return false_0 as libc::c_int;
    }
    lcache = *((**wadfiles.offset(wad as isize)).patchcache).offset(lump as isize);
    if !ptr.is_null() {
        if ptr == lcache {
            return true_0 as libc::c_int;
        }
    } else if !lcache.is_null() {
        return true_0 as libc::c_int
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn W_IsPatchCached(
    mut lumpnum: lumpnum_t,
    mut ptr: *mut libc::c_void,
) -> boolean {
    return W_IsPatchCachedPWAD(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        ptr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheLumpName(
    mut name: *const libc::c_char,
    mut tag: int32_t,
) -> *mut libc::c_void {
    return W_CacheLumpNum(W_GetNumForName(name), tag);
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheSoftwarePatchNumPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut lumpcache: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    if TestValidLump(wad, lump) == 0 {
        return 0 as *mut libc::c_void;
    }
    lumpcache = (**wadfiles.offset(wad as isize)).patchcache;
    if (*lumpcache.offset(lump as isize)).is_null() {
        let mut len: size_t = W_LumpLengthPwad(wad, lump);
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut dest: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut lumpdata: *mut libc::c_void = Z_MallocAlign(
            len,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        );
        W_ReadLumpHeaderPwad(
            wad,
            lump,
            lumpdata,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
        );
        ptr = lumpdata;
        if Picture_IsLumpPNG(lumpdata as *mut uint8_t, len) != 0 {
            ptr = Picture_PNGConvert(
                lumpdata as *mut uint8_t,
                PICFMT_DOOMPATCH,
                0 as *mut int32_t,
                0 as *mut int32_t,
                0 as *mut int16_t,
                0 as *mut int16_t,
                len,
                &mut len,
                0 as pictureflags_t,
            );
        }
        dest = Z_CallocAlign(
            ::core::mem::size_of::<patch_t>() as libc::c_ulong,
            tag,
            &mut *lumpcache.offset(lump as isize) as *mut *mut libc::c_void
                as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        );
        Patch_Create(ptr as *mut softwarepatch_t, len, dest);
        Z_Free(ptr);
    } else {
        Z_ChangeTag(*lumpcache.offset(lump as isize), tag);
    }
    return *lumpcache.offset(lump as isize);
}
#[no_mangle]
pub unsafe extern "C" fn W_CacheSoftwarePatchNum(
    mut lumpnum: lumpnum_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    return W_CacheSoftwarePatchNumPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        tag,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_CachePatchNumPwad(
    mut wad: uint16_t,
    mut lump: uint16_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut patch: *mut patch_t = 0 as *mut patch_t;
    if TestValidLump(wad, lump) == 0 {
        return 0 as *mut libc::c_void;
    }
    patch = W_CacheSoftwarePatchNumPwad(wad, lump, tag) as *mut patch_t;
    return patch as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn W_CachePatchNum(
    mut lumpnum: lumpnum_t,
    mut tag: int32_t,
) -> *mut libc::c_void {
    return W_CachePatchNumPwad(
        (lumpnum >> 16 as libc::c_int) as uint16_t,
        (lumpnum & 0xffff as libc::c_int as lumpnum_t) as uint16_t,
        tag,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_UnlockCachedPatch(mut patch: *mut libc::c_void) {
    if patch.is_null() {
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn W_CachePatchName(
    mut name: *const libc::c_char,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut num: lumpnum_t = 0;
    num = W_CheckNumForName(name);
    if num == 4294967295 as libc::c_uint {
        return W_CachePatchNum(
            W_GetNumForName(b"MISSING\0" as *const u8 as *const libc::c_char),
            tag,
        );
    }
    return W_CachePatchNum(num, tag);
}
#[no_mangle]
pub unsafe extern "C" fn W_CachePatchLongName(
    mut name: *const libc::c_char,
    mut tag: int32_t,
) -> *mut libc::c_void {
    let mut num: lumpnum_t = 0;
    num = W_CheckNumForLongName(name);
    if num == 4294967295 as libc::c_uint {
        return W_CachePatchNum(
            W_GetNumForLongName(b"MISSING\0" as *const u8 as *const libc::c_char),
            tag,
        );
    }
    return W_CachePatchNum(num, tag);
}
unsafe extern "C" fn PrintMD5String(
    mut md5: *const uint8_t,
    mut buf: *mut libc::c_char,
) {
    snprintf(
        buf,
        (2 as libc::c_int * 16 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        b"%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x\0"
            as *const u8 as *const libc::c_char,
        *md5.offset(0 as libc::c_int as isize) as libc::c_int,
        *md5.offset(1 as libc::c_int as isize) as libc::c_int,
        *md5.offset(2 as libc::c_int as isize) as libc::c_int,
        *md5.offset(3 as libc::c_int as isize) as libc::c_int,
        *md5.offset(4 as libc::c_int as isize) as libc::c_int,
        *md5.offset(5 as libc::c_int as isize) as libc::c_int,
        *md5.offset(6 as libc::c_int as isize) as libc::c_int,
        *md5.offset(7 as libc::c_int as isize) as libc::c_int,
        *md5.offset(8 as libc::c_int as isize) as libc::c_int,
        *md5.offset(9 as libc::c_int as isize) as libc::c_int,
        *md5.offset(10 as libc::c_int as isize) as libc::c_int,
        *md5.offset(11 as libc::c_int as isize) as libc::c_int,
        *md5.offset(12 as libc::c_int as isize) as libc::c_int,
        *md5.offset(13 as libc::c_int as isize) as libc::c_int,
        *md5.offset(14 as libc::c_int as isize) as libc::c_int,
        *md5.offset(15 as libc::c_int as isize) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn W_VerifyFileMD5(
    mut wadfilenum: uint16_t,
    mut matchmd5: *const libc::c_char,
) {
    let mut realmd5: [uint8_t; 16] = [0; 16];
    let mut ix: int32_t = 0;
    ix = 0 as libc::c_int;
    while ix < 2 as libc::c_int * 16 as libc::c_int {
        let mut n: int32_t = 0;
        let mut c: int32_t = *matchmd5.offset(ix as isize) as int32_t;
        if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            n = c - '0' as i32;
        } else if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
            & _ISupper as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            n = c - 'A' as i32 + 10 as libc::c_int;
        } else {
            n = c - 'a' as i32 + 10 as libc::c_int;
        }
        if ix & 1 as libc::c_int != 0 {
            realmd5[(ix >> 1 as libc::c_int)
                as usize] = (realmd5[(ix >> 1 as libc::c_int) as usize] as libc::c_int
                + n) as uint8_t;
        } else {
            realmd5[(ix >> 1 as libc::c_int)
                as usize] = (n << 4 as libc::c_int) as uint8_t;
        }
        ix += 1;
        ix;
    }
    if memcmp(
        realmd5.as_mut_ptr() as *const libc::c_void,
        ((**wadfiles.offset(wadfilenum as isize)).md5sum).as_mut_ptr()
            as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        let mut actualmd5text: [libc::c_char; 33] = [0; 33];
        PrintMD5String(
            ((**wadfiles.offset(wadfilenum as isize)).md5sum).as_mut_ptr(),
            actualmd5text.as_mut_ptr(),
        );
        I_Error(
            b"File is old, is corrupt or has been modified:\n%s\nFound MD5: %s\nWanted MD5: %s\n\0"
                as *const u8 as *const libc::c_char,
            (**wadfiles.offset(wadfilenum as isize)).filename,
            actualmd5text.as_mut_ptr(),
            matchmd5,
        );
    }
}
unsafe extern "C" fn W_VerifyName(
    mut name: *const libc::c_char,
    mut checklist: *mut lumpchecklist_t,
    mut status: boolean,
) -> libc::c_int {
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while (*checklist.offset(j as isize)).len != 0
        && !((*checklist.offset(j as isize)).name).is_null()
    {
        if (strncasecmp(
            name,
            (*checklist.offset(j as isize)).name,
            (*checklist.offset(j as isize)).len,
        ) != false_0 as libc::c_int) as libc::c_int == status
        {
            return true_0 as libc::c_int;
        }
        j = j.wrapping_add(1);
        j;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn W_VerifyWAD(
    mut fp: *mut FILE,
    mut checklist: *mut lumpchecklist_t,
    mut status: boolean,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut header: wadinfo_t = wadinfo_t {
        identification: [0; 4],
        numlumps: 0,
        infotableofs: 0,
    };
    let mut lumpinfo: filelump_t = filelump_t {
        filepos: 0,
        size: 0,
        name: [0; 8],
    };
    if fread(
        &mut header as *mut wadinfo_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<wadinfo_t>() as libc::c_ulong,
        fp,
    ) == ::core::mem::size_of::<wadinfo_t>() as libc::c_ulong
        && header.numlumps < 32767 as libc::c_int as uint32_t
        && strncmp(
            (header.identification).as_mut_ptr(),
            b"ZWAD\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        && strncmp(
            (header.identification).as_mut_ptr(),
            b"IWAD\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        && strncmp(
            (header.identification).as_mut_ptr(),
            b"PWAD\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        && strncmp(
            (header.identification).as_mut_ptr(),
            b"SDLL\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
    {
        return true_0 as libc::c_int;
    }
    header.numlumps = header.numlumps as int32_t as uint32_t;
    header.infotableofs = header.infotableofs as int32_t as uint32_t;
    if fseek(fp, header.infotableofs as libc::c_long, 0 as libc::c_int)
        == -(1 as libc::c_int)
    {
        return true_0 as libc::c_int;
    }
    i = 0 as libc::c_int as size_t;
    while i < header.numlumps as size_t {
        if fread(
            &mut lumpinfo as *mut filelump_t as *mut libc::c_void,
            ::core::mem::size_of::<filelump_t>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            return true_0 as libc::c_int;
        }
        lumpinfo.filepos = lumpinfo.filepos as int32_t as uint32_t;
        lumpinfo.size = lumpinfo.size as int32_t as uint32_t;
        if !(lumpinfo.size == 0 as libc::c_int as uint32_t) {
            if W_VerifyName((lumpinfo.name).as_mut_ptr(), checklist, status) == 0 {
                return false_0 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return true_0 as libc::c_int;
}
static mut folderblacklist: [lumpchecklist_t; 8] = [
    {
        let mut init = lumpchecklist_t {
            name: b"Lua/\0" as *const u8 as *const libc::c_char,
            len: 4 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"SOC/\0" as *const u8 as *const libc::c_char,
            len: 4 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"Sprites/\0" as *const u8 as *const libc::c_char,
            len: 8 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"Textures/\0" as *const u8 as *const libc::c_char,
            len: 9 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"Patches/\0" as *const u8 as *const libc::c_char,
            len: 8 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"Flats/\0" as *const u8 as *const libc::c_char,
            len: 6 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: b"Fades/\0" as *const u8 as *const libc::c_char,
            len: 6 as libc::c_int as size_t,
        };
        init
    },
    {
        let mut init = lumpchecklist_t {
            name: 0 as *const libc::c_char,
            len: 0 as libc::c_int as size_t,
        };
        init
    },
];
unsafe extern "C" fn W_VerifyPK3(
    mut fp: *mut FILE,
    mut checklist: *mut lumpchecklist_t,
    mut status: boolean,
) -> libc::c_int {
    let mut verified: libc::c_int = true_0 as libc::c_int;
    let mut zend: zend_t = zend_s {
        signature: [0; 4],
        diskpos: 0,
        cdirdisk: 0,
        diskentries: 0,
        entries: 0,
        cdirsize: 0,
        cdiroffset: 0,
        commentlen: 0,
    };
    let mut zentry: zentry_t = zentry_s {
        signature: [0; 4],
        version: 0,
        versionneeded: 0,
        flags: 0,
        compression: 0,
        modtime: 0,
        moddate: 0,
        CRC32: 0,
        compsize: 0,
        size: 0,
        namelen: 0,
        xtralen: 0,
        commlen: 0,
        diskstart: 0,
        attrint: 0,
        attrext: 0,
        offset: 0,
    };
    let mut zlentry: zlentry_t = zlentry_s {
        signature: [0; 4],
        versionneeded: 0,
        flags: 0,
        compression: 0,
        modtime: 0,
        moddate: 0,
        CRC32: 0,
        compsize: 0,
        size: 0,
        namelen: 0,
        xtralen: 0,
    };
    let mut file_size: libc::c_long = 0;
    let mut data_size: libc::c_long = 0;
    let mut old_position: libc::c_long = 0;
    let mut numlumps: uint16_t = 0;
    let mut i: size_t = 0;
    let mut pat_central: [libc::c_char; 5] = [
        0x50 as libc::c_int as libc::c_char,
        0x4b as libc::c_int as libc::c_char,
        0x1 as libc::c_int as libc::c_char,
        0x2 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut pat_end: [libc::c_char; 5] = [
        0x50 as libc::c_int as libc::c_char,
        0x4b as libc::c_int as libc::c_char,
        0x5 as libc::c_int as libc::c_char,
        0x6 as libc::c_int as libc::c_char,
        0 as libc::c_int as libc::c_char,
    ];
    let mut lumpname: [libc::c_char; 9] = [0; 9];
    fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    file_size = ftell(fp);
    if ResFindSignature(
        fp,
        pat_end.as_mut_ptr(),
        (if 0 as libc::c_int as libc::c_long
            > ftell(fp) - (22 as libc::c_int + 65536 as libc::c_int) as libc::c_long
        {
            0 as libc::c_int as libc::c_long
        } else {
            ftell(fp) - (22 as libc::c_int + 65536 as libc::c_int) as libc::c_long
        }) as uint32_t,
    ) == 0
    {
        return true_0 as libc::c_int;
    }
    fseek(fp, -(4 as libc::c_int) as libc::c_long, 1 as libc::c_int);
    if fread(
        &mut zend as *mut zend_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<zend_t>() as libc::c_ulong,
        fp,
    ) < ::core::mem::size_of::<zend_t>() as libc::c_ulong
    {
        return true_0 as libc::c_int;
    }
    data_size = ::core::mem::size_of::<zend_t>() as libc::c_ulong as libc::c_long;
    numlumps = zend.entries;
    fseek(fp, zend.cdiroffset as libc::c_long, 0 as libc::c_int);
    i = 0 as libc::c_int as size_t;
    while i < numlumps as size_t {
        let mut fullname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut trimname: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut dotpos: *mut libc::c_char = 0 as *mut libc::c_char;
        if fread(
            &mut zentry as *mut zentry_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<zentry_t>() as libc::c_ulong,
            fp,
        ) < ::core::mem::size_of::<zentry_t>() as libc::c_ulong
        {
            return true_0 as libc::c_int;
        }
        if memcmp(
            (zentry.signature).as_mut_ptr() as *const libc::c_void,
            pat_central.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) != 0
        {
            return true_0 as libc::c_int;
        }
        if verified == true_0 as libc::c_int {
            fullname = malloc(
                (zentry.namelen as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            ) as *mut libc::c_char;
            if fgets(fullname, zentry.namelen as libc::c_int + 1 as libc::c_int, fp)
                != fullname
            {
                return true_0 as libc::c_int;
            }
            trimname = strrchr(fullname, '/' as i32);
            if !trimname.is_null() {
                trimname = trimname.offset(1);
                trimname;
            } else {
                trimname = fullname;
            }
            if *trimname != 0 {
                dotpos = strrchr(trimname, '.' as i32);
                if dotpos.is_null() {
                    dotpos = fullname.offset(strlen(fullname) as isize);
                }
                memset(
                    lumpname.as_mut_ptr() as *mut libc::c_void,
                    '\0' as i32,
                    9 as libc::c_int as libc::c_ulong,
                );
                strncpy(
                    lumpname.as_mut_ptr(),
                    trimname,
                    (if (8 as libc::c_int as libc::c_long)
                        < dotpos.offset_from(trimname) as libc::c_long
                    {
                        8 as libc::c_int as libc::c_long
                    } else {
                        dotpos.offset_from(trimname) as libc::c_long
                    }) as libc::c_ulong,
                );
                if W_VerifyName(lumpname.as_mut_ptr(), checklist, status) == 0 {
                    verified = false_0 as libc::c_int;
                } else if W_VerifyName(fullname, folderblacklist.as_mut_ptr(), status)
                    != 0
                {
                    verified = false_0 as libc::c_int;
                }
            }
            free(fullname as *mut libc::c_void);
            if fseek(
                fp,
                (zentry.xtralen as libc::c_int + zentry.commlen as libc::c_int)
                    as libc::c_long,
                1 as libc::c_int,
            ) != 0 as libc::c_int
            {
                return true_0 as libc::c_int;
            }
        } else if fseek(
            fp,
            (zentry.namelen as libc::c_int + zentry.xtralen as libc::c_int
                + zentry.commlen as libc::c_int) as libc::c_long,
            1 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return true_0 as libc::c_int
        }
        data_size = (data_size as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<zentry_t>() as libc::c_ulong)
                    .wrapping_add(zentry.namelen as libc::c_ulong)
                    .wrapping_add(zentry.xtralen as libc::c_ulong)
                    .wrapping_add(zentry.commlen as libc::c_ulong),
            ) as libc::c_long as libc::c_long;
        old_position = ftell(fp);
        if fseek(fp, zentry.offset as libc::c_long, 0 as libc::c_int) != 0 as libc::c_int
        {
            return true_0 as libc::c_int;
        }
        if fread(
            &mut zlentry as *mut zlentry_t as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<zlentry_t>() as libc::c_ulong,
            fp,
        ) < ::core::mem::size_of::<zlentry_t>() as libc::c_ulong
        {
            return true_0 as libc::c_int;
        }
        data_size = (data_size as libc::c_ulong)
            .wrapping_add(
                (::core::mem::size_of::<zlentry_t>() as libc::c_ulong)
                    .wrapping_add(zlentry.namelen as libc::c_ulong)
                    .wrapping_add(zlentry.xtralen as libc::c_ulong)
                    .wrapping_add(zlentry.compsize as libc::c_ulong),
            ) as libc::c_long as libc::c_long;
        fseek(fp, old_position, 0 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
    if data_size < file_size {
        let mut error: *const libc::c_char = b"ZIP file has holes (%ld extra bytes)\n\0"
            as *const u8 as *const libc::c_char;
        CONS_Alert(CONS_ERROR, error, file_size - data_size);
        return -(1 as libc::c_int);
    } else if data_size > file_size {
        let mut error_0: *const libc::c_char = b"Reported size of ZIP file contents exceeds file size (%ld extra bytes)\n\0"
            as *const u8 as *const libc::c_char;
        CONS_Alert(CONS_ERROR, error_0, data_size - file_size);
        return -(1 as libc::c_int);
    } else {
        return verified
    };
}
unsafe extern "C" fn W_VerifyFile(
    mut filename: *const libc::c_char,
    mut checklist: *mut lumpchecklist_t,
    mut status: boolean,
) -> libc::c_int {
    let mut handle: *mut FILE = 0 as *mut FILE;
    let mut goodfile: libc::c_int = false_0 as libc::c_int;
    if checklist.is_null() {
        I_Error(
            b"No checklist for %s\n\0" as *const u8 as *const libc::c_char,
            filename,
        );
    }
    handle = W_OpenWadFile(&mut filename, false_0 as libc::c_int);
    if handle.is_null() {
        return -(1 as libc::c_int);
    }
    if strcasecmp(
        &*filename
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(filename))
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ),
        b".pk3\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        goodfile = W_VerifyPK3(handle, checklist, status);
    } else if strcasecmp(
        &*filename
            .offset(
                ((strlen
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                    ) -> libc::c_ulong)(filename))
                    .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
            ),
        b".soc\0" as *const u8 as *const libc::c_char,
    ) != 0
        && strcasecmp(
            &*filename
                .offset(
                    ((strlen
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                        ) -> libc::c_ulong)(filename))
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong) as isize,
                ),
            b".lua\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        goodfile = W_VerifyWAD(handle, checklist, status);
    }
    fclose(handle);
    return goodfile;
}
#[no_mangle]
pub unsafe extern "C" fn W_VerifyNMUSlumps(
    mut filename: *const libc::c_char,
    mut exit_on_error: boolean,
) -> libc::c_int {
    let mut NMUSlist: [lumpchecklist_t; 71] = [
        {
            let mut init = lumpchecklist_t {
                name: b"D_\0" as *const u8 as *const libc::c_char,
                len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"O_\0" as *const u8 as *const libc::c_char,
                len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"DS\0" as *const u8 as *const libc::c_char,
                len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"ENDOOM\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"PLAYPAL\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"PAL\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"COLORMAP\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CLM\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"TRANS\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CONSBACK\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SAVE\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"BLACXLVL\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"GAMEDONE\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CONT\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"STNONEX\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"ULTIMATE\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SLCT\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"LSSTATIC\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"BLANKLV\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CRFNT\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NTFNT\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NTFNO\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"LTFNT\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"TTL\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"STCFN\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"TNYFN\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"STLIVE\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CROSHAI\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"INTERSC\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SPECTILE\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"STT\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"YB_\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"RESULT\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"RACE\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SRB2BACK\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"M_\0" as *const u8 as *const libc::c_char,
                len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"LT\0" as *const u8 as *const libc::c_char,
                len: 2 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"HOMING\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"HOMITM\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CHARFG\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CHARBG\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"RECATK\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"RECCLOCK\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NTSATK\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NTSSONC\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SLID\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CONT\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"MINICAPS\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"BLUESTAT\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"BYELSTAT\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"ORNGSTAT\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"REDSTAT\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"YELSTAT\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NBRACKET\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NGHTLINK\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NGT\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NARROW\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NREDAR\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NSS\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NBON\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NRNG\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NHUD\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"CAPS\0" as *const u8 as *const libc::c_char,
                len: 4 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"DRILL\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"GRADE\0" as *const u8 as *const libc::c_char,
                len: 5 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"MINUS5\0" as *const u8 as *const libc::c_char,
                len: 6 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"NGRTIMER\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"MUSICDEF\0" as *const u8 as *const libc::c_char,
                len: 8 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SHADERS\0" as *const u8 as *const libc::c_char,
                len: 7 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: b"SH_\0" as *const u8 as *const libc::c_char,
                len: 3 as libc::c_int as size_t,
            };
            init
        },
        {
            let mut init = lumpchecklist_t {
                name: 0 as *const libc::c_char,
                len: 0 as libc::c_int as size_t,
            };
            init
        },
    ];
    let mut status: libc::c_int = W_VerifyFile(
        filename,
        NMUSlist.as_mut_ptr(),
        false_0 as libc::c_int,
    );
    if status == -(1 as libc::c_int) {
        W_InitFileError(filename, exit_on_error);
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn vres_GetMap(mut lumpnum: lumpnum_t) -> *mut virtres_t {
    let mut i: uint32_t = 0;
    let mut vres: *mut virtres_t = 0 as *mut virtres_t;
    let mut vlumps: *mut virtlump_t = 0 as *mut virtlump_t;
    let mut numlumps: size_t = 0 as libc::c_int as size_t;
    if W_IsLumpWad(lumpnum) != 0 {
        let mut wadData: *mut uint8_t = W_CacheLumpNum(lumpnum, PU_LEVEL as libc::c_int)
            as *mut uint8_t;
        let mut fileinfo: *mut filelump_t = wadData
            .offset((*(wadData as *mut wadinfo_t)).infotableofs as isize)
            as *mut filelump_t;
        numlumps = (*(wadData as *mut wadinfo_t)).numlumps as size_t;
        vlumps = Z_MallocAlign(
            (::core::mem::size_of::<virtlump_t>() as libc::c_ulong)
                .wrapping_mul(numlumps),
            PU_LEVEL as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut virtlump_t;
        i = 0 as libc::c_int as uint32_t;
        while (i as size_t) < numlumps {
            (*vlumps.offset(i as isize))
                .size = (*fileinfo.offset(i as isize)).size as size_t;
            memcpy(
                ((*vlumps.offset(i as isize)).name).as_mut_ptr() as *mut libc::c_void,
                ((*fileinfo.offset(i as isize)).name).as_mut_ptr()
                    as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            (*vlumps.offset(i as isize))
                .name[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            let ref mut fresh4 = (*vlumps.offset(i as isize)).data;
            *fresh4 = Z_MallocAlign(
                (*vlumps.offset(i as isize)).size,
                PU_LEVEL as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            memcpy(
                (*vlumps.offset(i as isize)).data as *mut libc::c_void,
                wadData.offset((*fileinfo.offset(i as isize)).filepos as isize)
                    as *const libc::c_void,
                (*vlumps.offset(i as isize)).size,
            );
            i = i.wrapping_add(1);
            i;
        }
        Z_Free(wadData as *mut libc::c_void);
    } else {
        let mut lumppos: lumpnum_t = lumpnum.wrapping_add(1 as libc::c_int as lumpnum_t);
        i = (lumppos & 0xffff as libc::c_int as lumpnum_t) as uint16_t as uint32_t;
        while i
            < (**wadfiles.offset((lumpnum >> 16 as libc::c_int) as uint16_t as isize))
                .numlumps as uint32_t
        {
            if memcmp(
                W_CheckNameForNum(lumppos) as *const libc::c_void,
                b"MAP\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
            lumppos = lumppos.wrapping_add(1);
            lumppos;
            numlumps = numlumps.wrapping_add(1);
            numlumps;
        }
        numlumps = numlumps.wrapping_add(1);
        numlumps;
        vlumps = Z_MallocAlign(
            (::core::mem::size_of::<virtlump_t>() as libc::c_ulong)
                .wrapping_mul(numlumps),
            PU_LEVEL as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut virtlump_t;
        i = 0 as libc::c_int as uint32_t;
        while (i as size_t) < numlumps {
            (*vlumps.offset(i as isize)).size = W_LumpLength(lumpnum);
            memcpy(
                ((*vlumps.offset(i as isize)).name).as_mut_ptr() as *mut libc::c_void,
                W_CheckNameForNum(lumpnum) as *const libc::c_void,
                8 as libc::c_int as libc::c_ulong,
            );
            (*vlumps.offset(i as isize))
                .name[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            let ref mut fresh5 = (*vlumps.offset(i as isize)).data;
            *fresh5 = W_CacheLumpNum(lumpnum, PU_LEVEL as libc::c_int) as *mut uint8_t;
            i = i.wrapping_add(1);
            i;
            lumpnum = lumpnum.wrapping_add(1);
            lumpnum;
        }
    }
    vres = Z_MallocAlign(
        ::core::mem::size_of::<virtres_t>() as libc::c_ulong,
        PU_LEVEL as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut virtres_t;
    (*vres).vlumps = vlumps;
    (*vres).numlumps = numlumps;
    return vres;
}
#[no_mangle]
pub unsafe extern "C" fn vres_Free(mut vres: *mut virtres_t) {
    loop {
        let fresh6 = (*vres).numlumps;
        (*vres).numlumps = ((*vres).numlumps).wrapping_sub(1);
        if !(fresh6 != 0) {
            break;
        }
        Z_Free(
            (*((*vres).vlumps).offset((*vres).numlumps as isize)).data
                as *mut libc::c_void,
        );
    }
    Z_Free((*vres).vlumps as *mut libc::c_void);
    Z_Free(vres as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn vres_Find(
    mut vres: *const virtres_t,
    mut name: *const libc::c_char,
) -> *mut virtlump_t {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while (i as size_t) < (*vres).numlumps {
        if fastcmp(name, ((*((*vres).vlumps).offset(i as isize)).name).as_mut_ptr()) != 0
        {
            return &mut *((*vres).vlumps).offset(i as isize) as *mut virtlump_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut virtlump_t;
}
