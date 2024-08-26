use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtof(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_float;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strupr(n: *mut libc::c_char) -> libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn CONS_Debug(debugflags: int32_t, fmt: *const libc::c_char, _: ...);
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn M_GetToken(inputString: *const libc::c_char) -> *mut libc::c_char;
    fn M_UnGetToken();
    fn ASTBlendPaletteIndexes(
        background: uint8_t,
        foreground: uint8_t,
        style: libc::c_int,
        alpha: uint8_t,
    ) -> uint8_t;
    static mut texturememory: size_t;
    fn Picture_PNGDimensions(
        png: *mut uint8_t,
        width: *mut int32_t,
        height: *mut int32_t,
        topoffset: *mut int16_t,
        leftoffset: *mut int16_t,
        size: size_t,
    ) -> boolean;
    fn Picture_IsLumpPNG(d: *const uint8_t, s: size_t) -> boolean;
    fn Picture_Convert(
        informat: pictureformat_t,
        picture: *mut libc::c_void,
        outformat: pictureformat_t,
        insize: size_t,
        outsize: *mut size_t,
        inwidth: int32_t,
        inheight: int32_t,
        inleftoffset: int32_t,
        intopoffset: int32_t,
        flags: pictureflags_t,
    ) -> *mut libc::c_void;
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
    fn Picture_TextureToFlat(trickytex: size_t) -> *mut libc::c_void;
    static mut ds_flatheight: uint16_t;
    static mut ds_flatwidth: uint16_t;
    fn Picture_FlatConvert(
        informat: pictureformat_t,
        picture: *mut libc::c_void,
        outformat: pictureformat_t,
        insize: size_t,
        outsize: *mut size_t,
        inwidth: int16_t,
        inheight: int16_t,
        inleftoffset: int16_t,
        intopoffset: int16_t,
        flags: pictureflags_t,
    ) -> *mut libc::c_void;
    static mut nflatmask: uint32_t;
    static mut nflatxshift: uint32_t;
    static mut nflatyshift: uint32_t;
    static mut nflatshiftup: uint32_t;
    fn W_CacheLumpNumPwad(
        wad: uint16_t,
        lump: uint16_t,
        tag: int32_t,
    ) -> *mut libc::c_void;
    fn W_CheckNameForNumPwad(wad: uint16_t, lump: uint16_t) -> *const libc::c_char;
    fn W_LumpLengthPwad(wad: uint16_t, lump: uint16_t) -> size_t;
    fn W_ReadLumpHeaderPwad(
        wad: uint16_t,
        lump: uint16_t,
        dest: *mut libc::c_void,
        size: size_t,
        offset: size_t,
    ) -> size_t;
    fn W_IsLumpFolder(wad: uint16_t, lump: uint16_t) -> boolean;
    static mut numwadfiles: uint16_t;
    fn W_CheckNumForFolderStartPK3(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
    fn W_CheckNumForFolderEndPK3(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
    fn W_CheckNumForMarkerStartPwad(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
    fn W_CheckNumForNamePwad(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
    fn W_GetNumForName(name: *const libc::c_char) -> lumpnum_t;
    static mut wadfiles: *mut *mut wadfile_t;
    fn W_CacheLumpNum(lump: lumpnum_t, tag: int32_t) -> *mut libc::c_void;
    fn W_LumpLength(lumpnum: lumpnum_t) -> size_t;
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
    fn Z_SetUser(ptr: *mut libc::c_void, newuser: *mut *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type lumpnum_t = uint32_t;
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
pub type fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct post_t {
    pub topdelta: uint8_t,
    pub length: uint8_t,
}
pub type column_t = post_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct softwarepatch_t {
    pub width: int16_t,
    pub height: int16_t,
    pub leftoffset: int16_t,
    pub topoffset: int16_t,
    pub columnofs: [int32_t; 8],
}
pub type patchalphastyle = libc::c_uint;
pub const AST_FOG: patchalphastyle = 7;
pub const AST_OVERLAY: patchalphastyle = 6;
pub const AST_MODULATE: patchalphastyle = 5;
pub const AST_REVERSESUBTRACT: patchalphastyle = 4;
pub const AST_SUBTRACT: patchalphastyle = 3;
pub const AST_ADD: patchalphastyle = 2;
pub const AST_TRANSLUCENT: patchalphastyle = 1;
pub const AST_COPY: patchalphastyle = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const LEVELFLAT_TEXTURE: C2RustUnnamed_0 = 4;
pub const LEVELFLAT_PNG: C2RustUnnamed_0 = 3;
pub const LEVELFLAT_PATCH: C2RustUnnamed_0 = 2;
pub const LEVELFLAT_FLAT: C2RustUnnamed_0 = 1;
pub const LEVELFLAT_NONE: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct levelflat_t {
    pub name: [libc::c_char; 9],
    pub type_0: uint8_t,
    pub u: C2RustUnnamed_1,
    pub width: uint16_t,
    pub height: uint16_t,
    pub animseq: int32_t,
    pub numpics: int32_t,
    pub speed: int32_t,
    pub picture: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub flat: C2RustUnnamed_3,
    pub texture: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub num: int32_t,
    pub lastnum: int32_t,
    pub basenum: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub lumpnum: lumpnum_t,
    pub baselumpnum: lumpnum_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texpatch_t {
    pub originx: int16_t,
    pub originy: int16_t,
    pub wad: uint16_t,
    pub lump: uint16_t,
    pub flip: uint8_t,
    pub alpha: uint8_t,
    pub style: patchalphastyle,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TEXTURETYPE_FLAT: C2RustUnnamed_4 = 3;
pub const TEXTURETYPE_COMPOSITE: C2RustUnnamed_4 = 2;
pub const TEXTURETYPE_SINGLEPATCH: C2RustUnnamed_4 = 1;
pub const TEXTURETYPE_UNKNOWN: C2RustUnnamed_4 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_t {
    pub name: [libc::c_char; 8],
    pub hash: uint32_t,
    pub type_0: uint8_t,
    pub width: int16_t,
    pub height: int16_t,
    pub holes: boolean,
    pub flip: uint8_t,
    pub flat: *mut libc::c_void,
    pub patchcount: int16_t,
    pub patches: [texpatch_t; 0],
}
pub const PU_CACHE: C2RustUnnamed_6 = 49;
pub const PU_STATIC: C2RustUnnamed_6 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub name: [libc::c_char; 9],
    pub hash: uint32_t,
    pub id: int32_t,
}
pub const PU_LEVEL: C2RustUnnamed_6 = 50;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_6 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_6 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_6 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_6 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_6 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_6 = 51;
pub const PU_HWRCACHE: C2RustUnnamed_6 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_6 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_6 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_6 = 21;
pub const PU_HUDGFX: C2RustUnnamed_6 = 19;
pub const PU_SPRITE: C2RustUnnamed_6 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_6 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_6 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_6 = 15;
pub const PU_PATCH: C2RustUnnamed_6 = 14;
pub const PU_MUSIC: C2RustUnnamed_6 = 12;
pub const PU_SOUND: C2RustUnnamed_6 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_6 = 3;
pub const PU_LUA: C2RustUnnamed_6 = 2;
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
#[no_mangle]
pub static mut numtextures: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut textures: *mut *mut texture_t = 0 as *const *mut texture_t
    as *mut *mut texture_t;
#[no_mangle]
pub static mut texturecolumnofs: *mut *mut uint32_t = 0 as *const *mut uint32_t
    as *mut *mut uint32_t;
#[no_mangle]
pub static mut texturecache: *mut *mut uint8_t = 0 as *const *mut uint8_t
    as *mut *mut uint8_t;
#[no_mangle]
pub static mut texturewidth: *mut int32_t = 0 as *const int32_t as *mut int32_t;
#[no_mangle]
pub static mut textureheight: *mut fixed_t = 0 as *const fixed_t as *mut fixed_t;
#[no_mangle]
pub static mut texturetranslation: *mut int32_t = 0 as *const int32_t as *mut int32_t;
static mut tidcache: *mut C2RustUnnamed_5 = 0 as *const C2RustUnnamed_5
    as *mut C2RustUnnamed_5;
static mut tidcachelen: int32_t = 0 as libc::c_int;
#[inline]
unsafe extern "C" fn R_DrawColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut uint8_t,
    mut originPatch: *mut texpatch_t,
    mut cacheheight: int32_t,
    mut patchheight: int32_t,
) {
    let mut count: int32_t = 0;
    let mut position: int32_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut topdelta: int32_t = 0;
    let mut prevdelta: int32_t = -(1 as libc::c_int);
    let mut originy: int32_t = (*originPatch).originy as int32_t;
    while (*patch).topdelta as libc::c_int != 0xff as libc::c_int {
        topdelta = (*patch).topdelta as int32_t;
        if topdelta <= prevdelta {
            topdelta += prevdelta;
        }
        prevdelta = topdelta;
        source = (patch as *mut uint8_t).offset(3 as libc::c_int as isize);
        count = (*patch).length as int32_t;
        position = originy + topdelta;
        if position < 0 as libc::c_int {
            count += position;
            source = source.offset(-(position as isize));
            position = 0 as libc::c_int;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        if count > 0 as libc::c_int {
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                cache.offset(position as isize) as *mut libc::c_void,
                source as *const libc::c_void,
                count as size_t,
            );
        }
        patch = (patch as *mut uint8_t)
            .offset((*patch).length as libc::c_int as isize)
            .offset(4 as libc::c_int as isize) as *mut column_t;
    }
}
#[inline]
unsafe extern "C" fn R_DrawFlippedColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut uint8_t,
    mut originPatch: *mut texpatch_t,
    mut cacheheight: int32_t,
    mut patchheight: int32_t,
) {
    let mut count: int32_t = 0;
    let mut position: int32_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut topdelta: int32_t = 0;
    let mut prevdelta: int32_t = -(1 as libc::c_int);
    let mut originy: int32_t = (*originPatch).originy as int32_t;
    while (*patch).topdelta as libc::c_int != 0xff as libc::c_int {
        topdelta = (*patch).topdelta as int32_t;
        if topdelta <= prevdelta {
            topdelta += prevdelta;
        }
        prevdelta = topdelta;
        topdelta = patchheight - (*patch).length as libc::c_int - topdelta;
        source = (patch as *mut uint8_t)
            .offset(2 as libc::c_int as isize)
            .offset((*patch).length as libc::c_int as isize);
        count = (*patch).length as int32_t;
        position = originy + topdelta;
        if position < 0 as libc::c_int {
            count += position;
            source = source.offset(position as isize);
            position = 0 as libc::c_int;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        dest = cache.offset(position as isize);
        if count > 0 as libc::c_int {
            while dest < cache.offset(position as isize).offset(count as isize) {
                let fresh0 = dest;
                dest = dest.offset(1);
                *fresh0 = *source;
                source = source.offset(-1);
                source;
            }
        }
        patch = (patch as *mut uint8_t)
            .offset((*patch).length as libc::c_int as isize)
            .offset(4 as libc::c_int as isize) as *mut column_t;
    }
}
#[inline]
unsafe extern "C" fn R_DrawBlendColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut uint8_t,
    mut originPatch: *mut texpatch_t,
    mut cacheheight: int32_t,
    mut patchheight: int32_t,
) {
    let mut count: int32_t = 0;
    let mut position: int32_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut topdelta: int32_t = 0;
    let mut prevdelta: int32_t = -(1 as libc::c_int);
    let mut originy: int32_t = (*originPatch).originy as int32_t;
    while (*patch).topdelta as libc::c_int != 0xff as libc::c_int {
        topdelta = (*patch).topdelta as int32_t;
        if topdelta <= prevdelta {
            topdelta += prevdelta;
        }
        prevdelta = topdelta;
        source = (patch as *mut uint8_t).offset(3 as libc::c_int as isize);
        count = (*patch).length as int32_t;
        position = originy + topdelta;
        if position < 0 as libc::c_int {
            count += position;
            source = source.offset(-(position as isize));
            position = 0 as libc::c_int;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        dest = cache.offset(position as isize);
        if count > 0 as libc::c_int {
            while dest < cache.offset(position as isize).offset(count as isize) {
                if *source as libc::c_int != 0xff as libc::c_int {
                    *dest = ASTBlendPaletteIndexes(
                        *dest,
                        *source,
                        (*originPatch).style as libc::c_int,
                        (*originPatch).alpha,
                    );
                }
                source = source.offset(1);
                source;
                dest = dest.offset(1);
                dest;
            }
        }
        patch = (patch as *mut uint8_t)
            .offset((*patch).length as libc::c_int as isize)
            .offset(4 as libc::c_int as isize) as *mut column_t;
    }
}
#[inline]
unsafe extern "C" fn R_DrawBlendFlippedColumnInCache(
    mut patch: *mut column_t,
    mut cache: *mut uint8_t,
    mut originPatch: *mut texpatch_t,
    mut cacheheight: int32_t,
    mut patchheight: int32_t,
) {
    let mut count: int32_t = 0;
    let mut position: int32_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut topdelta: int32_t = 0;
    let mut prevdelta: int32_t = -(1 as libc::c_int);
    let mut originy: int32_t = (*originPatch).originy as int32_t;
    while (*patch).topdelta as libc::c_int != 0xff as libc::c_int {
        topdelta = (*patch).topdelta as int32_t;
        if topdelta <= prevdelta {
            topdelta += prevdelta;
        }
        prevdelta = topdelta;
        topdelta = patchheight - (*patch).length as libc::c_int - topdelta;
        source = (patch as *mut uint8_t)
            .offset(2 as libc::c_int as isize)
            .offset((*patch).length as libc::c_int as isize);
        count = (*patch).length as int32_t;
        position = originy + topdelta;
        if position < 0 as libc::c_int {
            count += position;
            source = source.offset(position as isize);
            position = 0 as libc::c_int;
        }
        if position + count > cacheheight {
            count = cacheheight - position;
        }
        dest = cache.offset(position as isize);
        if count > 0 as libc::c_int {
            while dest < cache.offset(position as isize).offset(count as isize) {
                if *source as libc::c_int != 0xff as libc::c_int {
                    *dest = ASTBlendPaletteIndexes(
                        *dest,
                        *source,
                        (*originPatch).style as libc::c_int,
                        (*originPatch).alpha,
                    );
                }
                source = source.offset(-1);
                source;
                dest = dest.offset(1);
                dest;
            }
        }
        patch = (patch as *mut uint8_t)
            .offset((*patch).length as libc::c_int as isize)
            .offset(4 as libc::c_int as isize) as *mut column_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateTexture(mut texnum: size_t) -> *mut uint8_t {
    let mut current_block: u64;
    let mut block: *mut uint8_t = 0 as *mut uint8_t;
    let mut blocktex: *mut uint8_t = 0 as *mut uint8_t;
    let mut texture: *mut texture_t = 0 as *mut texture_t;
    let mut patch: *mut texpatch_t = 0 as *mut texpatch_t;
    let mut realpatch: *mut softwarepatch_t = 0 as *mut softwarepatch_t;
    let mut pdata: *mut uint8_t = 0 as *mut uint8_t;
    let mut x: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut blocksize: size_t = 0;
    let mut patchcol: *mut column_t = 0 as *mut column_t;
    let mut colofs: *mut uint8_t = 0 as *mut uint8_t;
    let mut wadnum: uint16_t = 0;
    let mut lumpnum: lumpnum_t = 0;
    let mut lumplength: size_t = 0;
    texture = *textures.offset(texnum as isize);
    if (*texture).patchcount as libc::c_int == 1 as libc::c_int {
        let mut holey: boolean = false_0 as libc::c_int;
        patch = ((*texture).patches).as_mut_ptr();
        wadnum = (*patch).wad;
        lumpnum = (*patch).lump as lumpnum_t;
        lumplength = W_LumpLengthPwad(wadnum, lumpnum as uint16_t);
        pdata = W_CacheLumpNumPwad(wadnum, lumpnum as uint16_t, PU_CACHE as libc::c_int)
            as *mut uint8_t;
        realpatch = pdata as *mut softwarepatch_t;
        if Picture_IsLumpPNG(realpatch as *mut uint8_t, lumplength) != 0 {
            current_block = 1458872964303035757;
        } else if (*texture).type_0 as libc::c_int == TEXTURETYPE_FLAT as libc::c_int {
            current_block = 1458872964303035757;
        } else {
            if (*texture).width as libc::c_int > (*realpatch).width as libc::c_int
                || (*texture).height as libc::c_int > (*realpatch).height as libc::c_int
            {
                holey = true_0 as libc::c_int;
            }
            colofs = ((*realpatch).columnofs).as_mut_ptr() as *mut uint8_t;
            x = 0 as libc::c_int;
            while x < (*texture).width as libc::c_int && holey == 0 {
                let mut col: *mut column_t = (realpatch as *mut uint8_t)
                    .offset(
                        *(&mut *colofs.offset((x << 2 as libc::c_int) as isize)
                            as *mut uint8_t as *mut uint32_t) as int32_t as isize,
                    ) as *mut column_t;
                let mut topdelta: int32_t = 0;
                let mut prevdelta: int32_t = -(1 as libc::c_int);
                let mut y: int32_t = 0 as libc::c_int;
                while (*col).topdelta as libc::c_int != 0xff as libc::c_int {
                    topdelta = (*col).topdelta as int32_t;
                    if topdelta <= prevdelta {
                        topdelta += prevdelta;
                    }
                    prevdelta = topdelta;
                    if topdelta > y {
                        break;
                    }
                    y = topdelta + (*col).length as libc::c_int + 1 as libc::c_int;
                    col = (col as *mut uint8_t)
                        .offset((*col).length as libc::c_int as isize)
                        .offset(4 as libc::c_int as isize) as *mut column_t;
                }
                if y < (*texture).height as libc::c_int {
                    holey = true_0 as libc::c_int;
                }
                x += 1;
                x;
            }
            if holey != 0 {
                (*texture).holes = true_0 as libc::c_int;
                (*texture).flip = (*patch).flip;
                blocksize = lumplength;
                block = Z_CallocAlign(
                    blocksize,
                    PU_STATIC as libc::c_int,
                    &mut *texturecache.offset(texnum as isize) as *mut *mut uint8_t
                        as *mut libc::c_void,
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        as int32_t,
                ) as *mut uint8_t;
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    block as *mut libc::c_void,
                    realpatch as *const libc::c_void,
                    blocksize,
                );
                texturememory = texturememory.wrapping_add(blocksize);
                colofs = block.offset(8 as libc::c_int as isize);
                let ref mut fresh1 = *texturecolumnofs.offset(texnum as isize);
                *fresh1 = colofs as *mut uint32_t;
                blocktex = block;
                if (*patch).flip as libc::c_int & 1 as libc::c_int != 0 {
                    let mut realcolofs: *mut uint8_t = ((*realpatch).columnofs)
                        .as_mut_ptr() as *mut uint8_t;
                    x = 0 as libc::c_int;
                    while x < (*texture).width as libc::c_int {
                        *(&mut *colofs.offset((x << 2 as libc::c_int) as isize)
                            as *mut uint8_t
                            as *mut uint32_t) = *realcolofs
                            .offset(
                                ((*texture).width as libc::c_int - 1 as libc::c_int - x
                                    << 2 as libc::c_int) as isize,
                            ) as uint32_t;
                        x += 1;
                        x;
                    }
                }
                x = 0 as libc::c_int;
                while x < (*texture).width as libc::c_int {
                    *(&mut *colofs.offset((x << 2 as libc::c_int) as isize)
                        as *mut uint8_t
                        as *mut uint32_t) = (*(&mut *colofs
                        .offset((x << 2 as libc::c_int) as isize) as *mut uint8_t
                        as *mut uint32_t) as int32_t + 3 as libc::c_int) as uint32_t;
                    x += 1;
                    x;
                }
                current_block = 74003652056514909;
            } else {
                current_block = 1458872964303035757;
            }
        }
    } else {
        current_block = 1458872964303035757;
    }
    match current_block {
        1458872964303035757 => {
            (*texture).holes = false_0 as libc::c_int;
            (*texture).flip = 0 as libc::c_int as uint8_t;
            blocksize = ((*texture).width as libc::c_int * 4 as libc::c_int
                + (*texture).width as libc::c_int * (*texture).height as libc::c_int)
                as size_t;
            texturememory = texturememory.wrapping_add(blocksize);
            block = Z_MallocAlign(
                blocksize.wrapping_add(1 as libc::c_int as size_t),
                PU_STATIC as libc::c_int,
                &mut *texturecache.offset(texnum as isize) as *mut *mut uint8_t
                    as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            memset(
                block as *mut libc::c_void,
                255 as libc::c_int,
                blocksize.wrapping_add(1 as libc::c_int as size_t),
            );
            colofs = block;
            let ref mut fresh2 = *texturecolumnofs.offset(texnum as isize);
            *fresh2 = colofs as *mut uint32_t;
            blocktex = block
                .offset(((*texture).width as libc::c_int * 4 as libc::c_int) as isize);
            i = 0 as libc::c_int;
            patch = ((*texture).patches).as_mut_ptr();
            while i < (*texture).patchcount as libc::c_int {
                let mut dealloc: boolean = true_0 as libc::c_int;
                static mut ColumnDrawerPointer: Option::<
                    unsafe extern "C" fn(
                        *mut column_t,
                        *mut uint8_t,
                        *mut texpatch_t,
                        int32_t,
                        int32_t,
                    ) -> (),
                > = None;
                if (*patch).style as libc::c_uint
                    != AST_COPY as libc::c_int as libc::c_uint
                {
                    ColumnDrawerPointer = if (*patch).flip as libc::c_int
                        & 2 as libc::c_int != 0
                    {
                        Some(
                            R_DrawBlendFlippedColumnInCache
                                as unsafe extern "C" fn(
                                    *mut column_t,
                                    *mut uint8_t,
                                    *mut texpatch_t,
                                    int32_t,
                                    int32_t,
                                ) -> (),
                        )
                    } else {
                        Some(
                            R_DrawBlendColumnInCache
                                as unsafe extern "C" fn(
                                    *mut column_t,
                                    *mut uint8_t,
                                    *mut texpatch_t,
                                    int32_t,
                                    int32_t,
                                ) -> (),
                        )
                    };
                } else {
                    ColumnDrawerPointer = if (*patch).flip as libc::c_int
                        & 2 as libc::c_int != 0
                    {
                        Some(
                            R_DrawFlippedColumnInCache
                                as unsafe extern "C" fn(
                                    *mut column_t,
                                    *mut uint8_t,
                                    *mut texpatch_t,
                                    int32_t,
                                    int32_t,
                                ) -> (),
                        )
                    } else {
                        Some(
                            R_DrawColumnInCache
                                as unsafe extern "C" fn(
                                    *mut column_t,
                                    *mut uint8_t,
                                    *mut texpatch_t,
                                    int32_t,
                                    int32_t,
                                ) -> (),
                        )
                    };
                }
                wadnum = (*patch).wad;
                lumpnum = (*patch).lump as lumpnum_t;
                pdata = W_CacheLumpNumPwad(
                    wadnum,
                    lumpnum as uint16_t,
                    PU_CACHE as libc::c_int,
                ) as *mut uint8_t;
                lumplength = W_LumpLengthPwad(wadnum, lumpnum as uint16_t);
                realpatch = pdata as *mut softwarepatch_t;
                dealloc = true_0 as libc::c_int;
                if Picture_IsLumpPNG(realpatch as *mut uint8_t, lumplength) != 0 {
                    realpatch = Picture_PNGConvert(
                        realpatch as *mut uint8_t,
                        PICFMT_DOOMPATCH,
                        0 as *mut int32_t,
                        0 as *mut int32_t,
                        0 as *mut int16_t,
                        0 as *mut int16_t,
                        lumplength,
                        0 as *mut size_t,
                        0 as pictureflags_t,
                    ) as *mut softwarepatch_t;
                } else if (*texture).type_0 as libc::c_int
                    == TEXTURETYPE_FLAT as libc::c_int
                {
                    realpatch = Picture_Convert(
                        PICFMT_FLAT,
                        pdata as *mut libc::c_void,
                        PICFMT_DOOMPATCH,
                        0 as libc::c_int as size_t,
                        0 as *mut size_t,
                        (*texture).width as int32_t,
                        (*texture).height as int32_t,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as pictureflags_t,
                    ) as *mut softwarepatch_t;
                } else {
                    dealloc = false_0 as libc::c_int;
                }
                x1 = (*patch).originx as libc::c_int;
                width = (*realpatch).width as libc::c_int;
                height = (*realpatch).height as libc::c_int;
                x2 = x1 + width;
                if x1 > (*texture).width as libc::c_int || x2 < 0 as libc::c_int {
                    if dealloc != 0 {
                        Z_Free(realpatch as *mut libc::c_void);
                    }
                } else if (*patch).originy as libc::c_int
                    > (*texture).height as libc::c_int
                    || (*patch).originy as libc::c_int + height < 0 as libc::c_int
                {
                    if dealloc != 0 {
                        Z_Free(realpatch as *mut libc::c_void);
                    }
                } else {
                    if x1 < 0 as libc::c_int {
                        x = 0 as libc::c_int;
                    } else {
                        x = x1;
                    }
                    if x2 > (*texture).width as libc::c_int {
                        x2 = (*texture).width as libc::c_int;
                    }
                    while x < x2 {
                        if (*patch).flip as libc::c_int & 1 as libc::c_int != 0 {
                            patchcol = (realpatch as *mut uint8_t)
                                .offset(
                                    (*realpatch)
                                        .columnofs[(x1 + width - 1 as libc::c_int - x) as usize]
                                        as isize,
                                ) as *mut column_t;
                        } else {
                            patchcol = (realpatch as *mut uint8_t)
                                .offset((*realpatch).columnofs[(x - x1) as usize] as isize)
                                as *mut column_t;
                        }
                        *(&mut *colofs.offset((x << 2 as libc::c_int) as isize)
                            as *mut uint8_t
                            as *mut uint32_t) = (x * (*texture).height as libc::c_int
                            + (*texture).width as libc::c_int * 4 as libc::c_int)
                            as uint32_t;
                        ColumnDrawerPointer
                            .expect(
                                "non-null function pointer",
                            )(
                            patchcol,
                            block
                                .offset(
                                    *(&mut *colofs.offset((x << 2 as libc::c_int) as isize)
                                        as *mut uint8_t as *mut uint32_t) as int32_t as isize,
                                ),
                            patch,
                            (*texture).height as int32_t,
                            height,
                        );
                        x += 1;
                        x;
                    }
                    if dealloc != 0 {
                        Z_Free(realpatch as *mut libc::c_void);
                    }
                }
                i += 1;
                i;
                patch = patch.offset(1);
                patch;
            }
        }
        _ => {}
    }
    Z_ChangeTag(block as *mut libc::c_void, PU_CACHE as libc::c_int);
    return blocktex;
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateTextureAsFlat(mut texnum: size_t) -> *mut uint8_t {
    let mut texture: *mut texture_t = *textures.offset(texnum as isize);
    let mut converted: *mut uint8_t = 0 as *mut uint8_t;
    let mut size: size_t = ((*texture).width as libc::c_int
        * (*texture).height as libc::c_int) as size_t;
    if ((*texture).flat).is_null() {
        (*texture)
            .flat = Z_MallocAlign(
            size,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        );
        converted = Picture_TextureToFlat(texnum) as *mut uint8_t;
        M_Memcpy
            .expect(
                "non-null function pointer",
            )((*texture).flat, converted as *const libc::c_void, size);
        Z_Free(converted as *mut libc::c_void);
    }
    return (*texture).flat as *mut uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_GetTextureNum(mut texnum: int32_t) -> int32_t {
    if texnum < 0 as libc::c_int || texnum >= numtextures {
        return 0 as libc::c_int;
    }
    return *texturetranslation.offset(texnum as isize);
}
#[no_mangle]
pub unsafe extern "C" fn R_CheckTextureCache(mut tex: int32_t) {
    if (*texturecache.offset(tex as isize)).is_null() {
        R_GenerateTexture(tex as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GetColumn(
    mut tex: fixed_t,
    mut col: int32_t,
) -> *mut uint8_t {
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut width: int32_t = *texturewidth.offset(tex as isize);
    if width & width - 1 as libc::c_int != 0 {
        col = (col as uint32_t % width as uint32_t) as int32_t;
    } else {
        col &= width - 1 as libc::c_int;
    }
    data = *texturecache.offset(tex as isize);
    if data.is_null() {
        data = R_GenerateTexture(tex as size_t);
    }
    return data
        .offset(
            *(*texturecolumnofs.offset(tex as isize)).offset(col as isize) as int32_t
                as isize,
        );
}
#[no_mangle]
pub unsafe extern "C" fn R_GetFlat(mut flatlumpnum: lumpnum_t) -> *mut libc::c_void {
    return W_CacheLumpNum(flatlumpnum, PU_CACHE as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_GetLevelFlat(
    mut levelflat: *mut levelflat_t,
) -> *mut libc::c_void {
    let mut isleveltexture: boolean = ((*levelflat).type_0 as libc::c_int
        == LEVELFLAT_TEXTURE as libc::c_int) as libc::c_int;
    let mut texture: *mut texture_t = if isleveltexture != 0 {
        *textures.offset((*levelflat).u.texture.num as isize)
    } else {
        0 as *mut texture_t
    };
    let mut texturechanged: boolean = if isleveltexture != 0 {
        ((*levelflat).u.texture.num != (*levelflat).u.texture.lastnum) as libc::c_int
    } else {
        false_0 as libc::c_int
    };
    let mut flatdata: *mut uint8_t = 0 as *mut uint8_t;
    if isleveltexture != 0 && texturechanged == 0 {
        if !((*texture).flat).is_null() {
            flatdata = (*texture).flat as *mut uint8_t;
            ds_flatwidth = (*texture).width as uint16_t;
            ds_flatheight = (*texture).height as uint16_t;
            texturechanged = false_0 as libc::c_int;
        } else {
            texturechanged = true_0 as libc::c_int;
        }
    }
    if ((*levelflat).picture).is_null() || texturechanged != 0 {
        if isleveltexture != 0 {
            (*levelflat)
                .picture = R_GenerateTextureAsFlat((*levelflat).u.texture.num as size_t);
            (*levelflat).width = (*texture).width as uint16_t;
            ds_flatwidth = (*levelflat).width;
            (*levelflat).height = (*texture).height as uint16_t;
            ds_flatheight = (*levelflat).height;
        } else if (*levelflat).type_0 as libc::c_int == LEVELFLAT_PNG as libc::c_int {
            let mut pngwidth: int32_t = 0;
            let mut pngheight: int32_t = 0;
            (*levelflat)
                .picture = Picture_PNGConvert(
                W_CacheLumpNum((*levelflat).u.flat.lumpnum, PU_CACHE as libc::c_int)
                    as *const uint8_t,
                PICFMT_FLAT,
                &mut pngwidth,
                &mut pngheight,
                0 as *mut int16_t,
                0 as *mut int16_t,
                W_LumpLength((*levelflat).u.flat.lumpnum),
                0 as *mut size_t,
                0 as pictureflags_t,
            ) as *mut uint8_t;
            (*levelflat).width = pngwidth as uint16_t;
            (*levelflat).height = pngheight as uint16_t;
            ds_flatwidth = (*levelflat).width;
            ds_flatheight = (*levelflat).height;
        } else if (*levelflat).type_0 as libc::c_int == LEVELFLAT_PATCH as libc::c_int {
            let mut converted: *mut uint8_t = 0 as *mut uint8_t;
            let mut size: size_t = 0;
            let mut patch: *mut softwarepatch_t = W_CacheLumpNum(
                (*levelflat).u.flat.lumpnum,
                PU_CACHE as libc::c_int,
            ) as *mut softwarepatch_t;
            ds_flatwidth = (*patch).width as uint16_t;
            (*levelflat).width = ds_flatwidth;
            ds_flatheight = (*patch).height as uint16_t;
            (*levelflat).height = ds_flatheight;
            (*levelflat)
                .picture = Z_MallocAlign(
                ((*levelflat).width as libc::c_int * (*levelflat).height as libc::c_int)
                    as size_t,
                PU_LEVEL as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            converted = Picture_FlatConvert(
                PICFMT_DOOMPATCH,
                patch as *mut libc::c_void,
                PICFMT_FLAT,
                0 as libc::c_int as size_t,
                &mut size,
                (*levelflat).width as int16_t,
                (*levelflat).height as int16_t,
                (*patch).topoffset,
                (*patch).leftoffset,
                0 as pictureflags_t,
            ) as *mut uint8_t;
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                (*levelflat).picture as *mut libc::c_void,
                converted as *const libc::c_void,
                size,
            );
            Z_Free(converted as *mut libc::c_void);
        }
    } else {
        ds_flatwidth = (*levelflat).width;
        ds_flatheight = (*levelflat).height;
    }
    (*levelflat).u.texture.lastnum = (*levelflat).u.texture.num;
    if flatdata.is_null() {
        flatdata = (*levelflat).picture;
    }
    return flatdata as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn R_CheckPowersOfTwo() -> boolean {
    let mut wpow2: boolean = (ds_flatwidth as libc::c_int
        & ds_flatwidth as libc::c_int - 1 as libc::c_int == 0) as libc::c_int;
    let mut hpow2: boolean = (ds_flatheight as libc::c_int
        & ds_flatheight as libc::c_int - 1 as libc::c_int == 0) as libc::c_int;
    if ds_flatwidth as libc::c_int > 2048 as libc::c_int
        || ds_flatheight as libc::c_int > 2048 as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    return (ds_flatwidth as libc::c_int == ds_flatheight as libc::c_int && wpow2 != 0
        && hpow2 != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_CheckSolidColorFlat() -> boolean {
    return (ds_flatwidth as libc::c_int == 1 as libc::c_int
        && ds_flatheight as libc::c_int == 1 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_GetFlatSize(mut length: size_t) -> uint16_t {
    match length {
        4194304 => return 2048 as libc::c_int as uint16_t,
        1048576 => return 1024 as libc::c_int as uint16_t,
        262144 => return 512 as libc::c_int as uint16_t,
        65536 => return 256 as libc::c_int as uint16_t,
        16384 => return 128 as libc::c_int as uint16_t,
        1024 => return 32 as libc::c_int as uint16_t,
        256 => return 16 as libc::c_int as uint16_t,
        64 => return 8 as libc::c_int as uint16_t,
        16 => return 4 as libc::c_int as uint16_t,
        4 => return 2 as libc::c_int as uint16_t,
        1 => return 1 as libc::c_int as uint16_t,
        _ => return 64 as libc::c_int as uint16_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_GetFlatBits(mut size: int32_t) -> uint8_t {
    match size {
        2048 => return 11 as libc::c_int as uint8_t,
        1024 => return 10 as libc::c_int as uint8_t,
        512 => return 9 as libc::c_int as uint8_t,
        256 => return 8 as libc::c_int as uint8_t,
        128 => return 7 as libc::c_int as uint8_t,
        32 => return 5 as libc::c_int as uint8_t,
        16 => return 4 as libc::c_int as uint8_t,
        8 => return 3 as libc::c_int as uint8_t,
        4 => return 2 as libc::c_int as uint8_t,
        2 => return 1 as libc::c_int as uint8_t,
        1 => return 0 as libc::c_int as uint8_t,
        _ => return 6 as libc::c_int as uint8_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_SetFlatVars(mut length: size_t) {
    let mut size: uint16_t = R_GetFlatSize(length);
    let mut bits: uint8_t = R_GetFlatBits(size as int32_t);
    ds_flatheight = size;
    ds_flatwidth = ds_flatheight;
    if bits as libc::c_int == 0 as libc::c_int {
        return;
    }
    nflatshiftup = (16 as libc::c_int - bits as libc::c_int) as uint32_t;
    nflatxshift = (16 as libc::c_int as uint32_t).wrapping_add(nflatshiftup);
    nflatyshift = nflatxshift.wrapping_sub(bits as uint32_t);
    nflatmask = ((size as libc::c_int - 1 as libc::c_int) * size as libc::c_int)
        as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_FlushTextureCache() {
    let mut i: int32_t = 0;
    if numtextures != 0 {
        i = 0 as libc::c_int;
        while i < numtextures {
            Z_Free(*texturecache.offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
    }
}
unsafe extern "C" fn Rloadflats(mut i: int32_t, mut w: int32_t) -> int32_t {
    let mut j: uint16_t = 0;
    let mut texstart: uint16_t = 0;
    let mut texend: uint16_t = 0;
    let mut texture: *mut texture_t = 0 as *mut texture_t;
    let mut patch: *mut texpatch_t = 0 as *mut texpatch_t;
    let mut header: [uint8_t; 8] = [0; 8];
    if (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
        == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint
    {
        texstart = W_CheckNumForFolderStartPK3(
            b"flats/\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForFolderEndPK3(
            b"flats/\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            texstart,
        );
    } else {
        texstart = W_CheckNumForMarkerStartPwad(
            b"F_START\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForNamePwad(
            b"F_END\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            texstart,
        );
    }
    if !(texstart as libc::c_int == 32767 as libc::c_int
        || texend as libc::c_int == 32767 as libc::c_int)
    {
        let mut current_block_32: u64;
        j = 0 as libc::c_int as uint16_t;
        while (j as libc::c_int) < texend as libc::c_int - texstart as libc::c_int {
            let mut wadnum: uint16_t = w as uint16_t;
            let mut lumpnum: lumpnum_t = (texstart as libc::c_int + j as libc::c_int)
                as lumpnum_t;
            let mut lumplength: size_t = 0;
            let mut flatsize: size_t = 0;
            if (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
                == RET_PK3 as libc::c_int as libc::c_uint
                || (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
                    == RET_FOLDER as libc::c_int as libc::c_uint
            {
                if W_IsLumpFolder(wadnum, lumpnum as uint16_t) != 0 {
                    current_block_32 = 17216689946888361452;
                } else {
                    current_block_32 = 12800627514080957624;
                }
            } else {
                current_block_32 = 12800627514080957624;
            }
            match current_block_32 {
                12800627514080957624 => {
                    W_ReadLumpHeaderPwad(
                        wadnum,
                        lumpnum as uint16_t,
                        header.as_mut_ptr() as *mut libc::c_void,
                        ::core::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
                        0 as libc::c_int as size_t,
                    );
                    lumplength = W_LumpLengthPwad(wadnum, lumpnum as uint16_t);
                    flatsize = R_GetFlatSize(lumplength) as size_t;
                    let ref mut fresh3 = *textures.offset(i as isize);
                    *fresh3 = Z_CallocAlign(
                        (::core::mem::size_of::<texture_t>() as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<texpatch_t>() as libc::c_ulong,
                            ),
                        PU_STATIC as libc::c_int,
                        0 as *mut libc::c_void,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            as int32_t,
                    ) as *mut texture_t;
                    texture = *fresh3;
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*texture).name).as_mut_ptr() as *mut libc::c_void,
                        W_CheckNameForNumPwad(wadnum, lumpnum as uint16_t)
                            as *const libc::c_void,
                        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    );
                    (*texture)
                        .hash = quickncasehash(
                        ((*texture).name).as_mut_ptr(),
                        8 as libc::c_int as size_t,
                    );
                    if Picture_IsLumpPNG(header.as_mut_ptr(), lumplength) != 0 {
                        let mut flatlump: *mut uint8_t = W_CacheLumpNumPwad(
                            wadnum,
                            lumpnum as uint16_t,
                            PU_CACHE as libc::c_int,
                        ) as *mut uint8_t;
                        let mut width: int32_t = 0;
                        let mut height: int32_t = 0;
                        Picture_PNGDimensions(
                            flatlump,
                            &mut width,
                            &mut height,
                            0 as *mut int16_t,
                            0 as *mut int16_t,
                            lumplength,
                        );
                        (*texture).width = width as int16_t;
                        (*texture).height = height as int16_t;
                        Z_Free(flatlump as *mut libc::c_void);
                    } else {
                        (*texture).height = flatsize as int16_t;
                        (*texture).width = (*texture).height;
                    }
                    (*texture).type_0 = TEXTURETYPE_FLAT as libc::c_int as uint8_t;
                    (*texture).patchcount = 1 as libc::c_int as int16_t;
                    (*texture).holes = false_0 as libc::c_int;
                    (*texture).flip = 0 as libc::c_int as uint8_t;
                    patch = &mut *((*texture).patches)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut texpatch_t;
                    (*patch).originy = 0 as libc::c_int as int16_t;
                    (*patch).originx = (*patch).originy;
                    (*patch).wad = w as uint16_t;
                    (*patch)
                        .lump = (texstart as libc::c_int + j as libc::c_int) as uint16_t;
                    (*patch).flip = 0 as libc::c_int as uint8_t;
                    *texturewidth.offset(i as isize) = (*texture).width as int32_t;
                    *textureheight
                        .offset(
                            i as isize,
                        ) = ((*texture).height as libc::c_int) << 16 as libc::c_int;
                    i += 1;
                    i;
                }
                _ => {}
            }
            j = j.wrapping_add(1);
            j;
        }
    }
    return i;
}
unsafe extern "C" fn Rloadtextures(mut i: int32_t, mut w: int32_t) -> int32_t {
    let mut j: uint16_t = 0;
    let mut texstart: uint16_t = 0;
    let mut texend: uint16_t = 0;
    let mut texturesLumpPos: uint16_t = 0;
    let mut texture: *mut texture_t = 0 as *mut texture_t;
    let mut patch: *mut texpatch_t = 0 as *mut texpatch_t;
    let mut patchlump: softwarepatch_t = softwarepatch_t {
        width: 0,
        height: 0,
        leftoffset: 0,
        topoffset: 0,
        columnofs: [0; 8],
    };
    if (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
        == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint
    {
        texstart = W_CheckNumForFolderStartPK3(
            b"textures/\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForFolderEndPK3(
            b"textures/\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            texstart,
        );
        texturesLumpPos = W_CheckNumForNamePwad(
            b"TEXTURES\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        while texturesLumpPos as libc::c_int != 32767 as libc::c_int {
            R_ParseTEXTURESLump(w as uint16_t, texturesLumpPos, &mut i);
            texturesLumpPos = W_CheckNumForNamePwad(
                b"TEXTURES\0" as *const u8 as *const libc::c_char,
                w as uint16_t,
                (texturesLumpPos as libc::c_int + 1 as libc::c_int) as uint16_t,
            );
        }
    } else {
        texstart = W_CheckNumForMarkerStartPwad(
            b"TX_START\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForNamePwad(
            b"TX_END\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        texturesLumpPos = W_CheckNumForNamePwad(
            b"TEXTURES\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        if texturesLumpPos as libc::c_int != 32767 as libc::c_int {
            R_ParseTEXTURESLump(w as uint16_t, texturesLumpPos, &mut i);
        }
    }
    if !(texstart as libc::c_int == 32767 as libc::c_int
        || texend as libc::c_int == 32767 as libc::c_int)
    {
        let mut current_block_41: u64;
        j = 0 as libc::c_int as uint16_t;
        while (j as libc::c_int) < texend as libc::c_int - texstart as libc::c_int {
            let mut wadnum: uint16_t = w as uint16_t;
            let mut lumpnum: lumpnum_t = (texstart as libc::c_int + j as libc::c_int)
                as lumpnum_t;
            let mut lumplength: size_t = 0;
            if (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
                == RET_PK3 as libc::c_int as libc::c_uint
                || (**wadfiles.offset(w as isize)).type_0 as libc::c_uint
                    == RET_FOLDER as libc::c_int as libc::c_uint
            {
                if W_IsLumpFolder(wadnum, lumpnum as uint16_t) != 0 {
                    current_block_41 = 12349973810996921269;
                } else {
                    current_block_41 = 2370887241019905314;
                }
            } else {
                current_block_41 = 2370887241019905314;
            }
            match current_block_41 {
                2370887241019905314 => {
                    W_ReadLumpHeaderPwad(
                        wadnum,
                        lumpnum as uint16_t,
                        &mut patchlump as *mut softwarepatch_t as *mut libc::c_void,
                        8 as libc::c_int as size_t,
                        0 as libc::c_int as size_t,
                    );
                    lumplength = W_LumpLengthPwad(wadnum, lumpnum as uint16_t);
                    let ref mut fresh4 = *textures.offset(i as isize);
                    *fresh4 = Z_CallocAlign(
                        (::core::mem::size_of::<texture_t>() as libc::c_ulong)
                            .wrapping_add(
                                ::core::mem::size_of::<texpatch_t>() as libc::c_ulong,
                            ),
                        PU_STATIC as libc::c_int,
                        0 as *mut libc::c_void,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            as int32_t,
                    ) as *mut texture_t;
                    texture = *fresh4;
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*texture).name).as_mut_ptr() as *mut libc::c_void,
                        W_CheckNameForNumPwad(wadnum, lumpnum as uint16_t)
                            as *const libc::c_void,
                        ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                    );
                    (*texture)
                        .hash = quickncasehash(
                        ((*texture).name).as_mut_ptr(),
                        8 as libc::c_int as size_t,
                    );
                    if Picture_IsLumpPNG(
                        &mut patchlump as *mut softwarepatch_t as *mut uint8_t,
                        lumplength,
                    ) != 0
                    {
                        let mut png: *mut uint8_t = W_CacheLumpNumPwad(
                            wadnum,
                            lumpnum as uint16_t,
                            PU_CACHE as libc::c_int,
                        ) as *mut uint8_t;
                        let mut width: int32_t = 0;
                        let mut height: int32_t = 0;
                        Picture_PNGDimensions(
                            png,
                            &mut width,
                            &mut height,
                            0 as *mut int16_t,
                            0 as *mut int16_t,
                            lumplength,
                        );
                        (*texture).width = width as int16_t;
                        (*texture).height = height as int16_t;
                        Z_Free(png as *mut libc::c_void);
                    } else {
                        (*texture).width = patchlump.width;
                        (*texture).height = patchlump.height;
                    }
                    (*texture)
                        .type_0 = TEXTURETYPE_SINGLEPATCH as libc::c_int as uint8_t;
                    (*texture).patchcount = 1 as libc::c_int as int16_t;
                    (*texture).holes = false_0 as libc::c_int;
                    (*texture).flip = 0 as libc::c_int as uint8_t;
                    patch = &mut *((*texture).patches)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as *mut texpatch_t;
                    (*patch).originy = 0 as libc::c_int as int16_t;
                    (*patch).originx = (*patch).originy;
                    (*patch).wad = w as uint16_t;
                    (*patch)
                        .lump = (texstart as libc::c_int + j as libc::c_int) as uint16_t;
                    (*patch).flip = 0 as libc::c_int as uint8_t;
                    *texturewidth.offset(i as isize) = (*texture).width as int32_t;
                    *textureheight
                        .offset(
                            i as isize,
                        ) = ((*texture).height as libc::c_int) << 16 as libc::c_int;
                    i += 1;
                    i;
                }
                _ => {}
            }
            j = j.wrapping_add(1);
            j;
        }
    }
    return i;
}
unsafe extern "C" fn count_range(
    mut marker_start: *const libc::c_char,
    mut marker_end: *const libc::c_char,
    mut folder: *const libc::c_char,
    mut wadnum: uint16_t,
) -> int32_t {
    let mut j: uint16_t = 0;
    let mut texstart: uint16_t = 0;
    let mut texend: uint16_t = 0;
    let mut count: int32_t = 0 as libc::c_int;
    if (**wadfiles.offset(wadnum as isize)).type_0 as libc::c_uint
        == RET_PK3 as libc::c_int as libc::c_uint
        || (**wadfiles.offset(wadnum as isize)).type_0 as libc::c_uint
            == RET_FOLDER as libc::c_int as libc::c_uint
    {
        texstart = W_CheckNumForFolderStartPK3(
            folder,
            wadnum,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForFolderEndPK3(folder, wadnum, texstart);
    } else {
        texstart = W_CheckNumForMarkerStartPwad(
            marker_start,
            wadnum,
            0 as libc::c_int as uint16_t,
        );
        texend = W_CheckNumForNamePwad(marker_end, wadnum, texstart);
    }
    if texstart as libc::c_int != 32767 as libc::c_int
        && texend as libc::c_int != 32767 as libc::c_int
    {
        if (**wadfiles.offset(wadnum as isize)).type_0 as libc::c_uint
            == RET_PK3 as libc::c_int as libc::c_uint
            || (**wadfiles.offset(wadnum as isize)).type_0 as libc::c_uint
                == RET_FOLDER as libc::c_int as libc::c_uint
        {
            j = texstart;
            while (j as libc::c_int) < texend as libc::c_int {
                if W_IsLumpFolder(wadnum, j) == 0 {
                    count += 1;
                    count;
                }
                j = j.wrapping_add(1);
                j;
            }
        } else {
            count += texend as libc::c_int - texstart as libc::c_int;
        }
    }
    return count;
}
unsafe extern "C" fn R_CountTextures(mut wadnum: uint16_t) -> int32_t {
    let mut texturesLumpPos: uint16_t = 0;
    let mut count: int32_t = 0 as libc::c_int;
    count
        += count_range(
            b"F_START\0" as *const u8 as *const libc::c_char,
            b"F_END\0" as *const u8 as *const libc::c_char,
            b"flats/\0" as *const u8 as *const libc::c_char,
            wadnum,
        );
    texturesLumpPos = W_CheckNumForNamePwad(
        b"TEXTURES\0" as *const u8 as *const libc::c_char,
        wadnum,
        0 as libc::c_int as uint16_t,
    );
    while texturesLumpPos as libc::c_int != 32767 as libc::c_int {
        count += R_CountTexturesInTEXTURESLump(wadnum, texturesLumpPos);
        texturesLumpPos = W_CheckNumForNamePwad(
            b"TEXTURES\0" as *const u8 as *const libc::c_char,
            wadnum,
            (texturesLumpPos as libc::c_int + 1 as libc::c_int) as uint16_t,
        );
    }
    count
        += count_range(
            b"TX_START\0" as *const u8 as *const libc::c_char,
            b"TX_END\0" as *const u8 as *const libc::c_char,
            b"textures/\0" as *const u8 as *const libc::c_char,
            wadnum,
        );
    return count;
}
unsafe extern "C" fn recallocuser(
    mut user: *mut libc::c_void,
    mut old: size_t,
    mut new: size_t,
) {
    let mut p: *mut libc::c_char = Z_ReallocAlign(
        *(user as *mut *mut libc::c_void),
        new,
        PU_STATIC as libc::c_int,
        user,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    if new > old {
        memset(
            &mut *p.offset(old as isize) as *mut libc::c_char as *mut libc::c_void,
            0 as libc::c_int,
            new.wrapping_sub(old),
        );
    }
}
unsafe extern "C" fn R_AllocateTextures(mut add: int32_t) {
    let newtextures: int32_t = numtextures + add;
    let newsize: size_t = (newtextures as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    let oldsize: size_t = (numtextures as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    let mut i: int32_t = 0;
    recallocuser(
        &mut textures as *mut *mut *mut texture_t as *mut libc::c_void,
        oldsize,
        newsize,
    );
    recallocuser(
        &mut texturecolumnofs as *mut *mut *mut uint32_t as *mut libc::c_void,
        oldsize,
        newsize,
    );
    recallocuser(
        &mut texturecache as *mut *mut *mut uint8_t as *mut libc::c_void,
        oldsize,
        newsize,
    );
    recallocuser(
        &mut texturewidth as *mut *mut int32_t as *mut libc::c_void,
        oldsize,
        newsize,
    );
    recallocuser(
        &mut textureheight as *mut *mut fixed_t as *mut libc::c_void,
        oldsize,
        newsize,
    );
    Z_ReallocAlign(
        texturetranslation as *mut libc::c_void,
        ((newtextures + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<int32_t>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        &mut texturetranslation as *mut *mut int32_t as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    i = 0 as libc::c_int;
    while i < numtextures {
        Z_SetUser(
            *texturecache.offset(i as isize) as *mut libc::c_void,
            &mut *texturecache.offset(i as isize) as *mut *mut uint8_t
                as *mut *mut libc::c_void,
        );
        i += 1;
        i;
    }
    while i < newtextures {
        *texturetranslation.offset(i as isize) = i;
        i += 1;
        i;
    }
}
unsafe extern "C" fn R_DefineTextures(mut i: int32_t, mut w: uint16_t) -> int32_t {
    i = Rloadflats(i, w as int32_t);
    return Rloadtextures(i, w as int32_t);
}
unsafe extern "C" fn R_FinishLoadingTextures(mut add: int32_t) {
    numtextures += add;
}
#[no_mangle]
pub unsafe extern "C" fn R_LoadTextures() {
    let mut i: int32_t = 0;
    let mut w: int32_t = 0;
    let mut newtextures: int32_t = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < numwadfiles as libc::c_int {
        newtextures += R_CountTextures(w as uint16_t);
        w += 1;
        w;
    }
    if newtextures == 0 {
        I_Error(
            b"No textures detected in any WADs!\n\0" as *const u8 as *const libc::c_char,
        );
    }
    R_AllocateTextures(newtextures);
    i = 0 as libc::c_int;
    w = 0 as libc::c_int;
    while w < numwadfiles as libc::c_int {
        i = R_DefineTextures(i, w as uint16_t);
        w += 1;
        w;
    }
    R_FinishLoadingTextures(newtextures);
}
#[no_mangle]
pub unsafe extern "C" fn R_LoadTexturesPwad(mut wadnum: uint16_t) {
    let mut newtextures: int32_t = R_CountTextures(wadnum);
    R_AllocateTextures(newtextures);
    R_DefineTextures(numtextures, wadnum);
    R_FinishLoadingTextures(newtextures);
}
unsafe extern "C" fn R_ParsePatch(mut actuallyLoadPatch: boolean) -> *mut texpatch_t {
    let mut texturesToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut texturesTokenLength: size_t = 0;
    let mut endPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut patchName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut patchXPos: int16_t = 0;
    let mut patchYPos: int16_t = 0;
    let mut flip: uint8_t = 0 as libc::c_int as uint8_t;
    let mut alpha: uint8_t = 255 as libc::c_int as uint8_t;
    let mut style: patchalphastyle = AST_COPY;
    let mut resultPatch: *mut texpatch_t = 0 as *mut texpatch_t;
    let mut patchLumpNum: lumpnum_t = 0;
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where patch name should be\0"
                as *const u8 as *const libc::c_char,
        );
    }
    texturesTokenLength = strlen(texturesToken);
    if texturesTokenLength > 8 as libc::c_int as size_t {
        I_Error(
            b"Error parsing TEXTURES lump: Patch name \"%s\" exceeds 8 characters\0"
                as *const u8 as *const libc::c_char,
            texturesToken,
        );
    } else {
        if !patchName.is_null() {
            Z_Free(patchName as *mut libc::c_void);
        }
        patchName = Z_MallocAlign(
            texturesTokenLength
                .wrapping_add(1 as libc::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            patchName as *mut libc::c_void,
            texturesToken as *const libc::c_void,
            texturesTokenLength
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        );
        *patchName.offset(texturesTokenLength as isize) = '\0' as i32 as libc::c_char;
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where comma after \"%s\"'s patch name should be\0"
                as *const u8 as *const libc::c_char,
            patchName,
        );
    }
    if strcmp(texturesToken, b",\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected \",\" after %s's patch name, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            patchName,
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where patch \"%s\"'s x coordinate should be\0"
                as *const u8 as *const libc::c_char,
            patchName,
        );
    }
    endPos = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    patchXPos = strtol(texturesToken, &mut endPos, 10 as libc::c_int) as int16_t;
    if endPos == texturesToken || *endPos as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected an integer for patch \"%s\"'s x coordinate, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            patchName,
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where comma after patch \"%s\"'s x coordinate should be\0"
                as *const u8 as *const libc::c_char,
            patchName,
        );
    }
    if strcmp(texturesToken, b",\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected \",\" after patch \"%s\"'s x coordinate, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            patchName,
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where patch \"%s\"'s y coordinate should be\0"
                as *const u8 as *const libc::c_char,
            patchName,
        );
    }
    endPos = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    patchYPos = strtol(texturesToken, &mut endPos, 10 as libc::c_int) as int16_t;
    if endPos == texturesToken || *endPos as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected an integer for patch \"%s\"'s y coordinate, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            patchName,
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if !texturesToken.is_null() {
        if strcmp(texturesToken, b"{\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            Z_Free(texturesToken as *mut libc::c_void);
            texturesToken = M_GetToken(0 as *const libc::c_char);
            if texturesToken.is_null() {
                I_Error(
                    b"Error parsing TEXTURES lump: Unexpected end of file where patch \"%s\"'s parameters should be\0"
                        as *const u8 as *const libc::c_char,
                    patchName,
                );
            }
            while strcmp(texturesToken, b"}\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                if strcasecmp(
                    texturesToken,
                    b"ALPHA\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    Z_Free(texturesToken as *mut libc::c_void);
                    texturesToken = M_GetToken(0 as *const libc::c_char);
                    alpha = (255 as libc::c_int as libc::c_float
                        * strtof(texturesToken, 0 as *mut *mut libc::c_char)) as uint8_t;
                } else if strcasecmp(
                    texturesToken,
                    b"STYLE\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    Z_Free(texturesToken as *mut libc::c_void);
                    texturesToken = M_GetToken(0 as *const libc::c_char);
                    if strcasecmp(
                        texturesToken,
                        b"TRANSLUCENT\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        style = AST_TRANSLUCENT;
                    } else if strcasecmp(
                        texturesToken,
                        b"ADD\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        style = AST_ADD;
                    } else if strcasecmp(
                        texturesToken,
                        b"SUBTRACT\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        style = AST_SUBTRACT;
                    } else if strcasecmp(
                        texturesToken,
                        b"REVERSESUBTRACT\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        style = AST_REVERSESUBTRACT;
                    } else if strcasecmp(
                        texturesToken,
                        b"MODULATE\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        style = AST_MODULATE;
                    }
                } else if strcasecmp(
                    texturesToken,
                    b"FLIPX\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    flip = (flip as libc::c_int | 1 as libc::c_int) as uint8_t;
                } else if strcasecmp(
                    texturesToken,
                    b"FLIPY\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    flip = (flip as libc::c_int | 2 as libc::c_int) as uint8_t;
                }
                Z_Free(texturesToken as *mut libc::c_void);
                texturesToken = M_GetToken(0 as *const libc::c_char);
                if texturesToken.is_null() {
                    I_Error(
                        b"Error parsing TEXTURES lump: Unexpected end of file where patch \"%s\"'s parameters or right curly brace should be\0"
                            as *const u8 as *const libc::c_char,
                        patchName,
                    );
                }
            }
        } else {
            M_UnGetToken();
        }
        Z_Free(texturesToken as *mut libc::c_void);
    }
    if actuallyLoadPatch == true_0 as libc::c_int {
        patchLumpNum = W_GetNumForName(patchName);
        resultPatch = Z_MallocAlign(
            ::core::mem::size_of::<texpatch_t>() as libc::c_ulong,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut texpatch_t;
        (*resultPatch).originx = patchXPos;
        (*resultPatch).originy = patchYPos;
        (*resultPatch)
            .lump = (patchLumpNum & 65535 as libc::c_int as lumpnum_t) as uint16_t;
        (*resultPatch).wad = (patchLumpNum >> 16 as libc::c_int) as uint16_t;
        (*resultPatch).flip = flip;
        (*resultPatch).alpha = alpha;
        (*resultPatch).style = style;
        Z_Free(patchName as *mut libc::c_void);
        return resultPatch;
    } else {
        Z_Free(patchName as *mut libc::c_void);
        return 0 as *mut texpatch_t;
    };
}
unsafe extern "C" fn R_ParseTexture(mut actuallyLoadTexture: boolean) -> *mut texture_t {
    let mut texturesToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut texturesTokenLength: size_t = 0;
    let mut endPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newTextureWidth: int32_t = 0;
    let mut newTextureHeight: int32_t = 0;
    let mut resultTexture: *mut texture_t = 0 as *mut texture_t;
    let mut newPatch: *mut texpatch_t = 0 as *mut texpatch_t;
    let mut newTextureName: [libc::c_char; 9] = [0; 9];
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where texture name should be\0"
                as *const u8 as *const libc::c_char,
        );
    }
    texturesTokenLength = strlen(texturesToken);
    if texturesTokenLength > 8 as libc::c_int as size_t {
        I_Error(
            b"Error parsing TEXTURES lump: Texture name \"%s\" exceeds 8 characters\0"
                as *const u8 as *const libc::c_char,
            texturesToken,
        );
    } else {
        memset(
            &mut newTextureName as *mut [libc::c_char; 9] as *mut libc::c_void,
            0 as libc::c_int,
            9 as libc::c_int as libc::c_ulong,
        );
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            newTextureName.as_mut_ptr() as *mut libc::c_void,
            texturesToken as *const libc::c_void,
            texturesTokenLength,
        );
        strupr(newTextureName.as_mut_ptr());
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where comma after texture \"%s\"'s name should be\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
        );
    } else if strcmp(texturesToken, b",\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected \",\" after texture \"%s\"'s name, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where texture \"%s\"'s width should be\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
        );
    }
    endPos = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    newTextureWidth = strtol(texturesToken, &mut endPos, 10 as libc::c_int) as int32_t;
    if endPos == texturesToken || *endPos as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int || newTextureWidth < 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected a positive integer for texture \"%s\"'s width, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where comma after texture \"%s\"'s width should be\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
        );
    }
    if strcmp(texturesToken, b",\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected \",\" after texture \"%s\"'s width, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where texture \"%s\"'s height should be\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
        );
    }
    endPos = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    newTextureHeight = strtol(texturesToken, &mut endPos, 10 as libc::c_int) as int32_t;
    if endPos == texturesToken || *endPos as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int
        || newTextureHeight < 0 as libc::c_int
    {
        I_Error(
            b"Error parsing TEXTURES lump: Expected a positive integer for texture \"%s\"'s height, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    texturesToken = M_GetToken(0 as *const libc::c_char);
    if texturesToken.is_null() {
        I_Error(
            b"Error parsing TEXTURES lump: Unexpected end of file where open curly brace for texture \"%s\" should be\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
        );
    }
    if strcmp(texturesToken, b"{\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if actuallyLoadTexture != 0 {
            resultTexture = Z_CallocAlign(
                ::core::mem::size_of::<texture_t>() as libc::c_ulong,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut texture_t;
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                ((*resultTexture).name).as_mut_ptr() as *mut libc::c_void,
                newTextureName.as_mut_ptr() as *const libc::c_void,
                8 as libc::c_int as size_t,
            );
            (*resultTexture)
                .hash = quickncasehash(
                newTextureName.as_mut_ptr(),
                8 as libc::c_int as size_t,
            );
            (*resultTexture).width = newTextureWidth as int16_t;
            (*resultTexture).height = newTextureHeight as int16_t;
            (*resultTexture).type_0 = TEXTURETYPE_COMPOSITE as libc::c_int as uint8_t;
        }
        Z_Free(texturesToken as *mut libc::c_void);
        texturesToken = M_GetToken(0 as *const libc::c_char);
        if texturesToken.is_null() {
            I_Error(
                b"Error parsing TEXTURES lump: Unexpected end of file where patch definition for texture \"%s\" should be\0"
                    as *const u8 as *const libc::c_char,
                newTextureName.as_mut_ptr(),
            );
        }
        while strcmp(texturesToken, b"}\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            if strcasecmp(texturesToken, b"PATCH\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                Z_Free(texturesToken as *mut libc::c_void);
                if !resultTexture.is_null() {
                    newPatch = R_ParsePatch(true_0 as libc::c_int);
                    resultTexture = Z_ReallocAlign(
                        resultTexture as *mut libc::c_void,
                        (::core::mem::size_of::<texture_t>() as libc::c_ulong)
                            .wrapping_add(
                                (((*resultTexture).patchcount as libc::c_int
                                    + 1 as libc::c_int) as libc::c_ulong)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<texpatch_t>() as libc::c_ulong,
                                    ),
                            ),
                        PU_STATIC as libc::c_int,
                        0 as *mut libc::c_void,
                        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                            as int32_t,
                    ) as *mut texture_t;
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *((*resultTexture).patches)
                            .as_mut_ptr()
                            .offset((*resultTexture).patchcount as isize)
                            as *mut texpatch_t as *mut libc::c_void,
                        newPatch as *const libc::c_void,
                        ::core::mem::size_of::<texpatch_t>() as libc::c_ulong,
                    );
                    (*resultTexture).patchcount += 1;
                    (*resultTexture).patchcount;
                    Z_Free(newPatch as *mut libc::c_void);
                } else {
                    R_ParsePatch(false_0 as libc::c_int);
                }
            } else {
                I_Error(
                    b"Error parsing TEXTURES lump: Expected \"PATCH\" in texture \"%s\", got \"%s\"\0"
                        as *const u8 as *const libc::c_char,
                    newTextureName.as_mut_ptr(),
                    texturesToken,
                );
            }
            texturesToken = M_GetToken(0 as *const libc::c_char);
            if texturesToken.is_null() {
                I_Error(
                    b"Error parsing TEXTURES lump: Unexpected end of file where patch declaration or right curly brace for texture \"%s\" should be\0"
                        as *const u8 as *const libc::c_char,
                    newTextureName.as_mut_ptr(),
                );
            }
        }
        if !resultTexture.is_null()
            && (*resultTexture).patchcount as libc::c_int == 0 as libc::c_int
        {
            I_Error(
                b"Error parsing TEXTURES lump: Texture \"%s\" must have at least one patch\0"
                    as *const u8 as *const libc::c_char,
                newTextureName.as_mut_ptr(),
            );
        }
    } else {
        I_Error(
            b"Error parsing TEXTURES lump: Expected \"{\" for texture \"%s\", got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newTextureName.as_mut_ptr(),
            texturesToken,
        );
    }
    Z_Free(texturesToken as *mut libc::c_void);
    if actuallyLoadTexture != 0 {
        return resultTexture
    } else {
        return 0 as *mut texture_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_CountTexturesInTEXTURESLump(
    mut wadNum: uint16_t,
    mut lumpNum: uint16_t,
) -> libc::c_int {
    let mut texturesLump: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut texturesLumpLength: size_t = 0;
    let mut texturesText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numTexturesInLump: uint32_t = 0 as libc::c_int as uint32_t;
    let mut texturesToken: *mut libc::c_char = 0 as *mut libc::c_char;
    texturesLump = W_CacheLumpNumPwad(wadNum, lumpNum, PU_STATIC as libc::c_int)
        as *mut libc::c_char;
    if texturesLump.is_null() {
        return 0 as libc::c_int;
    }
    texturesLumpLength = W_LumpLengthPwad(wadNum, lumpNum);
    texturesText = Z_MallocAlign(
        texturesLumpLength
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    memmove(
        texturesText as *mut libc::c_void,
        texturesLump as *const libc::c_void,
        texturesLumpLength,
    );
    *texturesText.offset(texturesLumpLength as isize) = '\0' as i32 as libc::c_char;
    Z_Free(texturesLump as *mut libc::c_void);
    texturesToken = M_GetToken(texturesText);
    while !texturesToken.is_null() {
        if strcasecmp(
            texturesToken,
            b"WALLTEXTURE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcasecmp(
                texturesToken,
                b"TEXTURE\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            numTexturesInLump = numTexturesInLump.wrapping_add(1);
            numTexturesInLump;
            Z_Free(texturesToken as *mut libc::c_void);
            R_ParseTexture(false_0 as libc::c_int);
        } else {
            I_Error(
                b"Error parsing TEXTURES lump: Expected \"WALLTEXTURE\" or \"TEXTURE\", got \"%s\"\0"
                    as *const u8 as *const libc::c_char,
                texturesToken,
            );
        }
        texturesToken = M_GetToken(0 as *const libc::c_char);
    }
    Z_Free(texturesToken as *mut libc::c_void);
    Z_Free(texturesText as *mut libc::c_void);
    return numTexturesInLump as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_ParseTEXTURESLump(
    mut wadNum: uint16_t,
    mut lumpNum: uint16_t,
    mut texindex: *mut int32_t,
) {
    let mut texturesLump: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut texturesLumpLength: size_t = 0;
    let mut texturesText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut texturesToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newTexture: *mut texture_t = 0 as *mut texture_t;
    texturesLump = W_CacheLumpNumPwad(wadNum, lumpNum, PU_STATIC as libc::c_int)
        as *mut libc::c_char;
    if texturesLump.is_null() {
        return;
    }
    texturesLumpLength = W_LumpLengthPwad(wadNum, lumpNum);
    texturesText = Z_MallocAlign(
        texturesLumpLength
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    memmove(
        texturesText as *mut libc::c_void,
        texturesLump as *const libc::c_void,
        texturesLumpLength,
    );
    *texturesText.offset(texturesLumpLength as isize) = '\0' as i32 as libc::c_char;
    Z_Free(texturesLump as *mut libc::c_void);
    texturesToken = M_GetToken(texturesText);
    while !texturesToken.is_null() {
        if strcasecmp(
            texturesToken,
            b"WALLTEXTURE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcasecmp(
                texturesToken,
                b"TEXTURE\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            Z_Free(texturesToken as *mut libc::c_void);
            newTexture = R_ParseTexture(true_0 as libc::c_int);
            let ref mut fresh5 = *textures.offset(*texindex as isize);
            *fresh5 = newTexture;
            *texturewidth.offset(*texindex as isize) = (*newTexture).width as int32_t;
            *textureheight
                .offset(
                    *texindex as isize,
                ) = ((*newTexture).height as libc::c_int) << 16 as libc::c_int;
            *texindex += 1;
            *texindex;
        } else {
            I_Error(
                b"Error parsing TEXTURES lump: Expected \"WALLTEXTURE\" or \"TEXTURE\", got \"%s\"\0"
                    as *const u8 as *const libc::c_char,
                texturesToken,
            );
        }
        texturesToken = M_GetToken(0 as *const libc::c_char);
    }
    Z_Free(texturesToken as *mut libc::c_void);
    Z_Free(texturesText as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_GetFlatNumForName(
    mut name: *const libc::c_char,
) -> lumpnum_t {
    let mut i: int32_t = 0;
    let mut lump: lumpnum_t = 0;
    let mut start: lumpnum_t = 0;
    let mut end: lumpnum_t = 0;
    let mut current_block_3: u64;
    i = numwadfiles as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        match (**wadfiles.offset(i as isize)).type_0 as libc::c_uint {
            0 => {
                start = W_CheckNumForMarkerStartPwad(
                    b"F_START\0" as *const u8 as *const libc::c_char,
                    i as uint16_t,
                    0 as libc::c_int as uint16_t,
                ) as lumpnum_t;
                if start == 32767 as libc::c_int as lumpnum_t {
                    start = W_CheckNumForMarkerStartPwad(
                        b"FF_START\0" as *const u8 as *const libc::c_char,
                        i as uint16_t,
                        0 as libc::c_int as uint16_t,
                    ) as lumpnum_t;
                    if start == 32767 as libc::c_int as lumpnum_t {
                        current_block_3 = 8258075665625361029;
                    } else {
                        end = W_CheckNumForNamePwad(
                            b"FF_END\0" as *const u8 as *const libc::c_char,
                            i as uint16_t,
                            start as uint16_t,
                        ) as lumpnum_t;
                        if end == 32767 as libc::c_int as lumpnum_t {
                            current_block_3 = 8258075665625361029;
                        } else {
                            current_block_3 = 10599921512955367680;
                        }
                    }
                } else {
                    end = W_CheckNumForNamePwad(
                        b"F_END\0" as *const u8 as *const libc::c_char,
                        i as uint16_t,
                        start as uint16_t,
                    ) as lumpnum_t;
                    if end == 32767 as libc::c_int as lumpnum_t {
                        current_block_3 = 8258075665625361029;
                    } else {
                        current_block_3 = 10599921512955367680;
                    }
                }
            }
            3 | 4 => {
                start = W_CheckNumForFolderStartPK3(
                    b"Flats/\0" as *const u8 as *const libc::c_char,
                    i as uint16_t,
                    0 as libc::c_int as uint16_t,
                ) as lumpnum_t;
                if start == 32767 as libc::c_int as lumpnum_t {
                    current_block_3 = 8258075665625361029;
                } else {
                    end = W_CheckNumForFolderEndPK3(
                        b"Flats/\0" as *const u8 as *const libc::c_char,
                        i as uint16_t,
                        start as uint16_t,
                    ) as lumpnum_t;
                    if end == 32767 as libc::c_int as lumpnum_t {
                        current_block_3 = 8258075665625361029;
                    } else {
                        current_block_3 = 10599921512955367680;
                    }
                }
            }
            _ => {
                current_block_3 = 8258075665625361029;
            }
        }
        match current_block_3 {
            10599921512955367680 => {
                lump = W_CheckNumForNamePwad(name, i as uint16_t, start as uint16_t)
                    as lumpnum_t;
                if lump < end {
                    lump = lump.wrapping_add((i << 16 as libc::c_int) as lumpnum_t);
                    break;
                } else {
                    lump = 4294967295 as libc::c_uint;
                }
            }
            _ => {}
        }
        i -= 1;
        i;
    }
    return lump;
}
#[no_mangle]
pub unsafe extern "C" fn R_ClearTextureNumCache(mut btell: boolean) {
    if !tidcache.is_null() {
        Z_Free(tidcache as *mut libc::c_void);
    }
    tidcache = 0 as *mut C2RustUnnamed_5;
    if btell != 0 {
        CONS_Debug(
            0x400 as libc::c_int,
            b"Fun Fact: There are %d textures used in this map.\n\0" as *const u8
                as *const libc::c_char,
            tidcachelen,
        );
    }
    tidcachelen = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_CheckTextureNumForName(
    mut name: *const libc::c_char,
) -> int32_t {
    let mut i: int32_t = 0;
    let mut hash: uint32_t = 0;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        return 0 as libc::c_int;
    }
    hash = quickncasehash(name, 8 as libc::c_int as size_t);
    i = 0 as libc::c_int;
    while i < tidcachelen {
        if (*tidcache.offset(i as isize)).hash == hash
            && strncasecmp(
                ((*tidcache.offset(i as isize)).name).as_mut_ptr(),
                name,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            return (*tidcache.offset(i as isize)).id;
        }
        i += 1;
        i;
    }
    i = numtextures - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if (**textures.offset(i as isize)).hash == hash
            && strncasecmp(
                ((**textures.offset(i as isize)).name).as_mut_ptr(),
                name,
                8 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            tidcachelen += 1;
            tidcachelen;
            Z_ReallocAlign(
                tidcache as *mut libc::c_void,
                (tidcachelen as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong,
                    ),
                PU_STATIC as libc::c_int,
                &mut tidcache as *mut *mut C2RustUnnamed_5 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            );
            strncpy(
                ((*tidcache.offset((tidcachelen - 1 as libc::c_int) as isize)).name)
                    .as_mut_ptr(),
                name,
                8 as libc::c_int as libc::c_ulong,
            );
            (*tidcache.offset((tidcachelen - 1 as libc::c_int) as isize))
                .name[8 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            CONS_Debug(
                0x400 as libc::c_int,
                b"texture #%s: %s\n\0" as *const u8 as *const libc::c_char,
                sizeu1(tidcachelen as size_t),
                ((*tidcache.offset((tidcachelen - 1 as libc::c_int) as isize)).name)
                    .as_mut_ptr(),
            );
            (*tidcache.offset((tidcachelen - 1 as libc::c_int) as isize)).hash = hash;
            (*tidcache.offset((tidcachelen - 1 as libc::c_int) as isize)).id = i;
            return i;
        }
        i -= 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_CheckTextureNameForNum(
    mut num: int32_t,
) -> *const libc::c_char {
    if num > 0 as libc::c_int && num < numtextures {
        return ((**textures.offset(num as isize)).name).as_mut_ptr();
    }
    return b"-\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn R_TextureNameForNum(mut num: int32_t) -> *const libc::c_char {
    let mut result: *const libc::c_char = R_CheckTextureNameForNum(num);
    if strcmp(result, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        return b"REDWALL\0" as *const u8 as *const libc::c_char;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn R_TextureNumForName(mut name: *const libc::c_char) -> int32_t {
    let i: int32_t = R_CheckTextureNumForName(name);
    if i == -(1 as libc::c_int) {
        static mut redwall: int32_t = -(2 as libc::c_int);
        CONS_Debug(
            0x400 as libc::c_int,
            b"WARNING: R_TextureNumForName: %.8s not found\n\0" as *const u8
                as *const libc::c_char,
            name,
        );
        if redwall == -(2 as libc::c_int) {
            redwall = R_CheckTextureNumForName(
                b"REDWALL\0" as *const u8 as *const libc::c_char,
            );
        }
        if redwall != -(1 as libc::c_int) {
            return redwall;
        }
        return 1 as libc::c_int;
    }
    return i;
}
