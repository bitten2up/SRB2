use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut vid: viddef_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    static mut CV_OnOff: [CV_PossibleValue_t; 0];
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_MallocAlign(
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
    static mut screens: [*mut uint8_t; 5];
    static mut pLocalPalette: *mut RGBA_t;
    static mut pMasterPalette: *mut RGBA_t;
    static mut rendermode: rendermode_t;
    fn I_ReadScreen(scr: *mut uint8_t);
    fn I_GetPrecisePrecision() -> uint64_t;
    fn I_GetPreciseTime() -> precise_t;
    static mut cv_screenshot_colorprofile: consvar_t;
    static mut st_palette: int32_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct byteColor_t {
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
    pub alpha: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union FColorRGBA {
    pub rgba: uint32_t,
    pub s: byteColor_t,
}
pub type RGBA_t = FColorRGBA;
pub type precise_t = uint64_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed_3,
    pub recalc: int32_t,
    pub direct: *mut uint8_t,
    pub dupx: int32_t,
    pub dupy: int32_t,
    pub fdupx: int32_t,
    pub fdupy: int32_t,
    pub bpp: int32_t,
    pub baseratio: int32_t,
    pub WndParent: *mut libc::c_void,
    pub smalldupx: uint8_t,
    pub smalldupy: uint8_t,
    pub meddupx: uint8_t,
    pub meddupy: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
pub type viddef_t = viddef_s;
pub const PU_STATIC: C2RustUnnamed_4 = 1;
pub const render_soft: rendermode_t = 1;
pub type rendermode_t = libc::c_uint;
pub const render_none: rendermode_t = 3;
pub const render_opengl: rendermode_t = 2;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_4 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_4 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_4 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_4 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_4 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_4 = 51;
pub const PU_LEVEL: C2RustUnnamed_4 = 50;
pub const PU_CACHE: C2RustUnnamed_4 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_4 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_4 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_4 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_4 = 21;
pub const PU_HUDGFX: C2RustUnnamed_4 = 19;
pub const PU_SPRITE: C2RustUnnamed_4 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_4 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_4 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_4 = 15;
pub const PU_PATCH: C2RustUnnamed_4 = 14;
pub const PU_MUSIC: C2RustUnnamed_4 = 12;
pub const PU_SOUND: C2RustUnnamed_4 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_4 = 3;
pub const PU_LUA: C2RustUnnamed_4 = 2;
#[no_mangle]
pub static mut gif_dynamicdelay_cons_t: [CV_PossibleValue_t; 4] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Off\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"On\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"Accurate, experimental\0" as *const u8 as *const libc::c_char,
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
pub static mut cv_gif_optimize: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"gif_optimize\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"On\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_OnOff.as_ptr() as *mut _,
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
pub static mut cv_gif_downscale: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"gif_downscale\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"On\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_OnOff.as_ptr() as *mut _,
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
pub static mut cv_gif_dynamicdelay: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"gif_dynamicdelay\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"On\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: gif_dynamicdelay_cons_t.as_ptr() as *mut _,
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
pub static mut cv_gif_localcolortable: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"gif_localcolortable\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"On\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_OnOff.as_ptr() as *mut _,
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
static mut gif_optimize: boolean = false_0 as libc::c_int;
static mut gif_downscale: boolean = false_0 as libc::c_int;
static mut gif_dynamicdelay: uint8_t = 0 as libc::c_int as uint8_t;
static mut gif_localcolortable: boolean = false_0 as libc::c_int;
static mut gif_colorprofile: boolean = false_0 as libc::c_int;
static mut gif_headerpalette: *mut RGBA_t = 0 as *const RGBA_t as *mut RGBA_t;
static mut gif_framepalette: *mut RGBA_t = 0 as *const RGBA_t as *mut RGBA_t;
static mut gif_out: *mut FILE = 0 as *const FILE as *mut FILE;
static mut gif_frames: int32_t = 0 as libc::c_int;
static mut gif_prevframetime: precise_t = 0 as libc::c_int as precise_t;
static mut gif_delayus: uint32_t = 0 as libc::c_int as uint32_t;
static mut gif_writeover: uint8_t = 0 as libc::c_int as uint8_t;
unsafe extern "C" fn GIF_optimizecmprow(
    mut dst: *const uint8_t,
    mut src: *const uint8_t,
    mut row: int32_t,
    mut last: *mut int32_t,
    mut left: *mut int32_t,
    mut right: *mut int32_t,
) -> uint8_t {
    let mut dp: *const uint8_t = dst.offset((vid.width * row) as isize);
    let mut sp: *const uint8_t = src.offset((vid.width * row) as isize);
    let mut dtmp: *const uint8_t = 0 as *const uint8_t;
    let mut stmp: *const uint8_t = 0 as *const uint8_t;
    let mut doleft: uint8_t = 1 as libc::c_int as uint8_t;
    let mut doright: uint8_t = 1 as libc::c_int as uint8_t;
    let mut i: int32_t = 0 as libc::c_int;
    if memcmp(
        sp as *const libc::c_void,
        dp as *const libc::c_void,
        vid.width as libc::c_ulong,
    ) == 0
    {
        return 0 as libc::c_int as uint8_t;
    }
    *last = row;
    i = 0 as libc::c_int;
    if *left == 0 as libc::c_int {
        doleft = 0 as libc::c_int as uint8_t;
    } else if *left > 0 as libc::c_int {
        if memcmp(
            sp as *const libc::c_void,
            dp as *const libc::c_void,
            *left as libc::c_ulong,
        ) == 0
        {
            doleft = 0 as libc::c_int as uint8_t;
        }
    }
    while doleft != 0 {
        dtmp = dp.offset(i as isize);
        stmp = sp.offset(i as isize);
        if *dtmp as libc::c_int != *stmp as libc::c_int {
            doleft = 0 as libc::c_int as uint8_t;
            *left = i;
        }
        i += 1;
        i;
    }
    i = vid.width - 1 as libc::c_int;
    if *right == vid.width - 1 as libc::c_int {
        doright = 0 as libc::c_int as uint8_t;
    } else if *right >= 0 as libc::c_int {
        dtmp = dp.offset(*right as isize).offset(1 as libc::c_int as isize);
        stmp = sp.offset(*right as isize).offset(1 as libc::c_int as isize);
        if memcmp(
            stmp as *const libc::c_void,
            dtmp as *const libc::c_void,
            (vid.width - (*right + 1 as libc::c_int)) as libc::c_ulong,
        ) == 0
        {
            doright = 0 as libc::c_int as uint8_t;
        }
    }
    while doright != 0 {
        dtmp = dp.offset(i as isize);
        stmp = sp.offset(i as isize);
        if *dtmp as libc::c_int != *stmp as libc::c_int {
            doright = 0 as libc::c_int as uint8_t;
            *right = i;
        }
        i -= 1;
        i;
    }
    return 1 as libc::c_int as uint8_t;
}
unsafe extern "C" fn GIF_optimizeregion(
    mut dst: *const uint8_t,
    mut src: *const uint8_t,
    mut x: *mut int32_t,
    mut y: *mut int32_t,
    mut w: *mut int32_t,
    mut h: *mut int32_t,
) {
    let mut st: int32_t = 0 as libc::c_int;
    let mut sb: int32_t = vid.height - 1 as libc::c_int;
    let mut firstchg_t: int32_t = -(1 as libc::c_int);
    let mut firstchg_b: int32_t = -(1 as libc::c_int);
    let mut lastchg_t: int32_t = -(1 as libc::c_int);
    let mut lastchg_b: int32_t = -(1 as libc::c_int);
    let mut lmpix: int32_t = -(1 as libc::c_int);
    let mut rmpix: int32_t = -(1 as libc::c_int);
    let mut stopt: uint8_t = 0 as libc::c_int as uint8_t;
    let mut stopb: uint8_t = 0 as libc::c_int as uint8_t;
    while (stopt == 0 || stopb == 0) && st < sb {
        if stopt == 0 {
            let fresh0 = st;
            st = st + 1;
            if GIF_optimizecmprow(
                dst,
                src,
                fresh0,
                &mut lastchg_t,
                &mut lmpix,
                &mut rmpix,
            ) as libc::c_int != 0 && lmpix == 0 as libc::c_int
                && rmpix == vid.width - 1 as libc::c_int
            {
                stopt = 1 as libc::c_int as uint8_t;
            }
            if firstchg_t < 0 as libc::c_int && lastchg_t >= 0 as libc::c_int {
                firstchg_t = lastchg_t;
            }
        }
        if stopb == 0 {
            let fresh1 = sb;
            sb = sb - 1;
            if GIF_optimizecmprow(
                dst,
                src,
                fresh1,
                &mut lastchg_b,
                &mut lmpix,
                &mut rmpix,
            ) as libc::c_int != 0 && lmpix == 0 as libc::c_int
                && rmpix == vid.width - 1 as libc::c_int
            {
                stopb = 1 as libc::c_int as uint8_t;
            }
            if firstchg_b < 0 as libc::c_int && lastchg_b >= 0 as libc::c_int {
                firstchg_b = lastchg_b;
            }
        }
    }
    if lmpix < 0 as libc::c_int {
        *y = 0 as libc::c_int;
        *x = *y;
        *h = 1 as libc::c_int;
        *w = *h;
        return;
    }
    *x = lmpix;
    *y = if firstchg_t < 0 as libc::c_int && lastchg_b >= 0 as libc::c_int {
        lastchg_b
    } else {
        firstchg_t
    };
    *w = rmpix + 1 as libc::c_int;
    *h = (if firstchg_b < 0 as libc::c_int && lastchg_t >= 0 as libc::c_int {
        lastchg_t
    } else {
        firstchg_b
    }) + 1 as libc::c_int;
    *w -= *x;
    *h -= *y;
}
static mut gifbwr_buf: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut gifbwr_cur: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut gifbwr_bufsize: uint8_t = 0 as libc::c_int as uint8_t;
static mut gifbwr_bits_buf: uint32_t = 0 as libc::c_int as uint32_t;
static mut gifbwr_bits_num: int32_t = 0 as libc::c_int;
static mut gifbwr_bits_min: uint8_t = 9 as libc::c_int as uint8_t;
unsafe extern "C" fn GIF_bwrflush() {
    if gifbwr_bits_num > 0 as libc::c_int {
        let mut p_tmp: *mut uint8_t = gifbwr_cur as *mut libc::c_void as *mut uint8_t;
        let tv: uint8_t = (gifbwr_bits_buf & 0xff as libc::c_int as uint32_t) as uint8_t;
        memcpy(
            gifbwr_cur as *mut libc::c_void,
            &tv as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        gifbwr_cur = p_tmp as *mut libc::c_void as *mut uint8_t;
        gifbwr_bufsize = gifbwr_bufsize.wrapping_add(1);
        gifbwr_bufsize;
    }
    gifbwr_bits_num = 0 as libc::c_int;
    gifbwr_bits_buf = gifbwr_bits_num as uint32_t;
}
unsafe extern "C" fn GIF_bwrwrite(mut idata: uint32_t) {
    gifbwr_bits_buf |= idata << gifbwr_bits_num;
    gifbwr_bits_num += gifbwr_bits_min as libc::c_int;
    while gifbwr_bits_num >= 8 as libc::c_int {
        let mut p_tmp: *mut uint8_t = gifbwr_cur as *mut libc::c_void as *mut uint8_t;
        let tv: uint8_t = (gifbwr_bits_buf & 0xff as libc::c_int as uint32_t) as uint8_t;
        memcpy(
            gifbwr_cur as *mut libc::c_void,
            &tv as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        gifbwr_cur = p_tmp as *mut libc::c_void as *mut uint8_t;
        gifbwr_bits_buf >>= 8 as libc::c_int;
        gifbwr_bits_num -= 8 as libc::c_int;
        gifbwr_bufsize = gifbwr_bufsize.wrapping_add(1);
        gifbwr_bufsize;
    }
}
static mut scrbuf_pos: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut scrbuf_linebegin: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut scrbuf_lineend: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut scrbuf_writeend: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut scrbuf_downscaleamt: int16_t = 1 as libc::c_int as int16_t;
static mut giflzw_workingCode: uint16_t = 0;
static mut giflzw_nextCodeToAssign: uint16_t = 0;
static mut giflzw_hashTable: *mut uint32_t = 0 as *const uint32_t as *mut uint32_t;
unsafe extern "C" fn GIF_prepareLZW() {
    gifbwr_bits_min = 9 as libc::c_int as uint8_t;
    giflzw_nextCodeToAssign = 0x102 as libc::c_int as uint16_t;
    if giflzw_hashTable.is_null() {
        giflzw_hashTable = Z_MallocAlign(
            (16384 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint32_t;
    }
    memset(
        giflzw_hashTable as *mut libc::c_void,
        0 as libc::c_int,
        (16384 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
    );
}
unsafe extern "C" fn GIF_searchHash(
    mut key: uint32_t,
    mut pOutput: *mut uint32_t,
) -> libc::c_char {
    let mut entry: uint32_t = 0;
    let mut position: uint32_t = key >> 6 as libc::c_int
        & 0x3fff as libc::c_int as uint32_t;
    while *giflzw_hashTable.offset(position as isize) != 0 as libc::c_int as uint32_t {
        entry = *giflzw_hashTable.offset(position as isize);
        if entry >> 12 as libc::c_int == key {
            *pOutput = entry & 0xfff as libc::c_int as uint32_t;
            return 1 as libc::c_int as libc::c_char;
        }
        position = position.wrapping_add(1 as libc::c_int as uint32_t)
            & 0x3fff as libc::c_int as uint32_t;
    }
    return 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn GIF_addHash(mut key: uint32_t, mut value: uint32_t) {
    let mut position: uint32_t = key >> 6 as libc::c_int
        & 0x3fff as libc::c_int as uint32_t;
    loop {
        if *giflzw_hashTable.offset(position as isize) == 0 as libc::c_int as uint32_t {
            *giflzw_hashTable
                .offset(
                    position as isize,
                ) = key << 12 as libc::c_int | value & 0xfff as libc::c_int as uint32_t;
            return;
        }
        position = position.wrapping_add(1 as libc::c_int as uint32_t)
            & 0x3fff as libc::c_int as uint32_t;
    };
}
unsafe extern "C" fn GIF_feedByte(mut pbyte: uint8_t) {
    let mut key: uint32_t = 0;
    let mut hashOutput: uint32_t = 0 as libc::c_int as uint32_t;
    if giflzw_workingCode as libc::c_int == 65535 as libc::c_int {
        giflzw_workingCode = pbyte as uint16_t;
        return;
    }
    key = ((giflzw_workingCode as libc::c_int) << 8 as libc::c_int
        | pbyte as libc::c_int) as uint32_t;
    if 0 as libc::c_int == GIF_searchHash(key, &mut hashOutput) as libc::c_int {
        if giflzw_nextCodeToAssign as libc::c_int
            > (1 as libc::c_int) << gifbwr_bits_min as libc::c_int
        {
            gifbwr_bits_min = gifbwr_bits_min.wrapping_add(1);
            gifbwr_bits_min;
        }
        GIF_bwrwrite(giflzw_workingCode as uint32_t);
        GIF_addHash(key, giflzw_nextCodeToAssign as uint32_t);
        giflzw_nextCodeToAssign = giflzw_nextCodeToAssign.wrapping_add(1);
        giflzw_nextCodeToAssign;
        giflzw_workingCode = pbyte as uint16_t;
        return;
    }
    giflzw_workingCode = hashOutput as uint16_t;
}
unsafe extern "C" fn GIF_lzw() {
    while scrbuf_pos <= scrbuf_writeend {
        GIF_feedByte(*scrbuf_pos);
        if giflzw_nextCodeToAssign as libc::c_int >= 4096 as libc::c_int {
            GIF_bwrwrite(0x100 as libc::c_int as uint32_t);
            GIF_prepareLZW();
        }
        scrbuf_pos = scrbuf_pos.offset(scrbuf_downscaleamt as libc::c_int as isize);
        if scrbuf_pos >= scrbuf_lineend {
            scrbuf_lineend = scrbuf_lineend
                .offset((vid.width * scrbuf_downscaleamt as libc::c_int) as isize);
            scrbuf_linebegin = scrbuf_linebegin
                .offset((vid.width * scrbuf_downscaleamt as libc::c_int) as isize);
            scrbuf_pos = scrbuf_linebegin;
        }
        if gifbwr_bufsize as libc::c_int >= 248 as libc::c_int {
            break;
        }
    }
    if scrbuf_pos > scrbuf_writeend {
        let fresh2 = giflzw_nextCodeToAssign;
        giflzw_nextCodeToAssign = giflzw_nextCodeToAssign.wrapping_add(1);
        if fresh2 as libc::c_int > (1 as libc::c_int) << gifbwr_bits_min as libc::c_int {
            gifbwr_bits_min = gifbwr_bits_min.wrapping_add(1);
            gifbwr_bits_min;
        }
        GIF_bwrwrite(giflzw_workingCode as uint32_t);
        let fresh3 = giflzw_nextCodeToAssign;
        giflzw_nextCodeToAssign = giflzw_nextCodeToAssign.wrapping_add(1);
        if fresh3 as libc::c_int > (1 as libc::c_int) << gifbwr_bits_min as libc::c_int {
            gifbwr_bits_min = gifbwr_bits_min.wrapping_add(1);
            gifbwr_bits_min;
        }
        GIF_bwrwrite(0x101 as libc::c_int as uint32_t);
        GIF_bwrflush();
        gif_writeover = 1 as libc::c_int as uint8_t;
    }
}
#[no_mangle]
pub static mut gifhead_base: [uint8_t; 6] = [
    0x47 as libc::c_int as uint8_t,
    0x49 as libc::c_int as uint8_t,
    0x46 as libc::c_int as uint8_t,
    0x38 as libc::c_int as uint8_t,
    0x39 as libc::c_int as uint8_t,
    0x61 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut gifhead_nsid: [uint8_t; 19] = [
    0x21 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xb as libc::c_int as uint8_t,
    0x4e as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x54 as libc::c_int as uint8_t,
    0x53 as libc::c_int as uint8_t,
    0x43 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x50 as libc::c_int as uint8_t,
    0x45 as libc::c_int as uint8_t,
    0x32 as libc::c_int as uint8_t,
    0x2e as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x1 as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0xff as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
unsafe extern "C" fn GIF_getpalette(mut palnum: size_t) -> *mut RGBA_t {
    return if gif_colorprofile != 0 {
        &mut *pLocalPalette.offset((palnum * 256 as libc::c_int as size_t) as isize)
            as *mut RGBA_t
    } else {
        &mut *pMasterPalette.offset((palnum * 256 as libc::c_int as size_t) as isize)
            as *mut RGBA_t
    };
}
unsafe extern "C" fn GIF_palwrite(
    mut p: *mut uint8_t,
    mut pal: *mut RGBA_t,
) -> *mut uint8_t {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv: uint8_t = (*pal.offset(i as isize)).s.red;
        memcpy(
            p as *mut libc::c_void,
            &tv as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        p = p_tmp as *mut libc::c_void as *mut uint8_t;
        let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_0: uint8_t = (*pal.offset(i as isize)).s.green;
        memcpy(
            p as *mut libc::c_void,
            &tv_0 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_0 = p_tmp_0.offset(1);
        p_tmp_0;
        p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
        let mut p_tmp_1: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_1: uint8_t = (*pal.offset(i as isize)).s.blue;
        memcpy(
            p as *mut libc::c_void,
            &tv_1 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_1 = p_tmp_1.offset(1);
        p_tmp_1;
        p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
        i += 1;
        i;
    }
    return p;
}
unsafe extern "C" fn GIF_headwrite() {
    let mut gifhead: *mut uint8_t = Z_MallocAlign(
        800 as libc::c_int as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut uint8_t;
    let mut p: *mut uint8_t = gifhead;
    let mut rwidth: uint16_t = 0;
    let mut rheight: uint16_t = 0;
    if gif_out.is_null() {
        return;
    }
    memcpy(
        p as *mut libc::c_void,
        gifhead_base.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong as isize);
    if gif_downscale != 0 {
        scrbuf_downscaleamt = vid.dupx as int16_t;
        rwidth = (vid.width / scrbuf_downscaleamt as libc::c_int) as uint16_t;
        rheight = (vid.height / scrbuf_downscaleamt as libc::c_int) as uint16_t;
    } else {
        scrbuf_downscaleamt = 1 as libc::c_int as int16_t;
        rwidth = vid.width as uint16_t;
        rheight = vid.height as uint16_t;
    }
    let mut p_tmp: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv: uint16_t = rwidth;
    memcpy(
        p as *mut libc::c_void,
        &tv as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp = p_tmp.offset(1);
    p_tmp;
    p = p_tmp as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_0: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv_0: uint16_t = rheight;
    memcpy(
        p as *mut libc::c_void,
        &tv_0 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_0 = p_tmp_0.offset(1);
    p_tmp_0;
    p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_1: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_1: uint8_t = 0xf7 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_1 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_1 = p_tmp_1.offset(1);
    p_tmp_1;
    p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_2: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_2: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_2 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_2 = p_tmp_2.offset(1);
    p_tmp_2;
    p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_3: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_3: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_3 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_3 = p_tmp_3.offset(1);
    p_tmp_3;
    p = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
    p = GIF_palwrite(p, gif_headerpalette);
    memcpy(
        p as *mut libc::c_void,
        gifhead_nsid.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint8_t; 19]>() as libc::c_ulong,
    );
    p = p.offset(::core::mem::size_of::<[uint8_t; 19]>() as libc::c_ulong as isize);
    fwrite(
        gifhead as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        800 as libc::c_int as libc::c_ulong,
        gif_out,
    );
    Z_Free(gifhead as *mut libc::c_void);
}
#[no_mangle]
pub static mut gifframe_gchead: [uint8_t; 4] = [
    0x21 as libc::c_int as uint8_t,
    0xf9 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
    0x4 as libc::c_int as uint8_t,
];
static mut gifframe_data: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut gifframe_size: size_t = 8192 as libc::c_int as size_t;
unsafe extern "C" fn GIF_framewrite() {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut movie_screen: *mut uint8_t = screens[2 as libc::c_int as usize];
    let mut blitx: int32_t = 0;
    let mut blity: int32_t = 0;
    let mut blitw: int32_t = 0;
    let mut blith: int32_t = 0;
    let mut palchanged: boolean = 0;
    if gifframe_data.is_null() {
        gifframe_data = Z_MallocAlign(
            gifframe_size,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint8_t;
    }
    p = gifframe_data;
    if gif_out.is_null() {
        return;
    }
    if gif_localcolortable != 0 {
        gif_framepalette = GIF_getpalette(
            (if st_palette > 0 as libc::c_int { st_palette } else { 0 as libc::c_int })
                as size_t,
        );
        palchanged = memcmp(
            gif_headerpalette as *const libc::c_void,
            gif_framepalette as *const libc::c_void,
            (::core::mem::size_of::<RGBA_t>() as libc::c_ulong)
                .wrapping_mul(256 as libc::c_int as libc::c_ulong),
        );
    } else {
        palchanged = false_0 as libc::c_int;
    }
    if gif_optimize != 0 && gif_frames > 0 as libc::c_int && palchanged == 0 {
        let mut cur_screen: *mut uint8_t = screens[0 as libc::c_int as usize];
        GIF_optimizeregion(
            cur_screen,
            movie_screen,
            &mut blitx,
            &mut blity,
            &mut blitw,
            &mut blith,
        );
        if rendermode as libc::c_uint == render_soft as libc::c_int as libc::c_uint {
            I_ReadScreen(movie_screen);
        }
    } else {
        blity = 0 as libc::c_int;
        blitx = blity;
        blitw = vid.width;
        blith = vid.height;
        if gif_frames == 0 as libc::c_int
            && rendermode as libc::c_uint == render_soft as libc::c_int as libc::c_uint
        {
            I_ReadScreen(movie_screen);
        }
        movie_screen = screens[0 as libc::c_int as usize];
    }
    let mut delay: uint16_t = 0 as libc::c_int as uint16_t;
    let mut startline: int32_t = 0;
    if gif_dynamicdelay as libc::c_int == 2 as libc::c_int as uint8_t as libc::c_int {
        let mut mingifdelay: uint16_t = 10 as libc::c_int as uint16_t;
        gif_delayus = (gif_delayus as uint64_t)
            .wrapping_add(
                (I_GetPreciseTime()).wrapping_sub(gif_prevframetime)
                    / (I_GetPrecisePrecision() / 1000000 as libc::c_int as uint64_t),
            ) as uint32_t as uint32_t;
        if gif_delayus / 1000 as libc::c_int as uint32_t >= mingifdelay as uint32_t {
            let mut frames: libc::c_int = (gif_delayus / 1000 as libc::c_int as uint32_t
                / mingifdelay as uint32_t) as libc::c_int;
            delay = frames as uint16_t;
            gif_delayus = gif_delayus
                .wrapping_sub(
                    (frames * (mingifdelay as libc::c_int * 1000 as libc::c_int))
                        as uint32_t,
                );
        }
    } else if gif_dynamicdelay as libc::c_int
        == 1 as libc::c_int as uint8_t as libc::c_int
    {
        let mut delayf: libc::c_float = ceil(
            (100.0f32 / (35 as libc::c_int * 1 as libc::c_int) as libc::c_float)
                as libc::c_double,
        ) as libc::c_float;
        delay = ((I_GetPreciseTime()).wrapping_sub(gif_prevframetime) as uint16_t
            as uint64_t / (I_GetPrecisePrecision() / 1000000 as libc::c_int as uint64_t)
            / 10 as libc::c_int as uint64_t / 1000 as libc::c_int as uint64_t)
            as uint16_t;
        if (delay as libc::c_int) < delayf as uint16_t as libc::c_int {
            delay = delayf as uint16_t;
        }
    } else {
        let mut d1: libc::c_int = (100.0f32
            / (35 as libc::c_int * 1 as libc::c_int) as libc::c_float
            * (gif_frames + 1 as libc::c_int) as libc::c_float) as libc::c_int;
        let mut d2: libc::c_int = (100.0f32
            / (35 as libc::c_int * 1 as libc::c_int) as libc::c_float
            * gif_frames as libc::c_float) as libc::c_int;
        delay = (d1 - d2) as uint16_t;
    }
    memcpy(
        p as *mut libc::c_void,
        gifframe_gchead.as_ptr() as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    );
    p = p.offset(4 as libc::c_int as isize);
    let mut p_tmp: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv: uint16_t = delay;
    memcpy(
        p as *mut libc::c_void,
        &tv as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp = p_tmp.offset(1);
    p_tmp;
    p = p_tmp as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_0: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_0 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_0 = p_tmp_0.offset(1);
    p_tmp_0;
    p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_1: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_1: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_1 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_1 = p_tmp_1.offset(1);
    p_tmp_1;
    p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
    if scrbuf_downscaleamt as libc::c_int > 1 as libc::c_int {
        blitx -= blitx % scrbuf_downscaleamt as libc::c_int;
        blity -= blity % scrbuf_downscaleamt as libc::c_int;
        blitw = (blitw + (scrbuf_downscaleamt as libc::c_int - 1 as libc::c_int))
            / scrbuf_downscaleamt as libc::c_int * scrbuf_downscaleamt as libc::c_int;
        blith = (blith + (scrbuf_downscaleamt as libc::c_int - 1 as libc::c_int))
            / scrbuf_downscaleamt as libc::c_int * scrbuf_downscaleamt as libc::c_int;
    }
    let mut p_tmp_2: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_2: uint8_t = 0x2c as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_2 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_2 = p_tmp_2.offset(1);
    p_tmp_2;
    p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_3: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv_3: uint16_t = (blitx / scrbuf_downscaleamt as libc::c_int) as uint16_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_3 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_3 = p_tmp_3.offset(1);
    p_tmp_3;
    p = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_4: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv_4: uint16_t = (blity / scrbuf_downscaleamt as libc::c_int) as uint16_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_4 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_4 = p_tmp_4.offset(1);
    p_tmp_4;
    p = p_tmp_4 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_5: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv_5: uint16_t = (blitw / scrbuf_downscaleamt as libc::c_int) as uint16_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_5 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_5 = p_tmp_5.offset(1);
    p_tmp_5;
    p = p_tmp_5 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_6: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
    let tv_6: uint16_t = (blith / scrbuf_downscaleamt as libc::c_int) as uint16_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_6 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_6 = p_tmp_6.offset(1);
    p_tmp_6;
    p = p_tmp_6 as *mut libc::c_void as *mut uint8_t;
    if gif_localcolortable == 0 {
        let mut p_tmp_7: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_7: uint8_t = 0 as libc::c_int as uint8_t;
        memcpy(
            p as *mut libc::c_void,
            &tv_7 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_7 = p_tmp_7.offset(1);
        p_tmp_7;
        p = p_tmp_7 as *mut libc::c_void as *mut uint8_t;
    } else if palchanged != 0 {
        let mut p_tmp_8: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_8: uint8_t = 0x87 as libc::c_int as uint8_t;
        memcpy(
            p as *mut libc::c_void,
            &tv_8 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_8 = p_tmp_8.offset(1);
        p_tmp_8;
        p = p_tmp_8 as *mut libc::c_void as *mut uint8_t;
        p = GIF_palwrite(p, gif_framepalette);
    } else {
        let mut p_tmp_9: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_9: uint8_t = 0 as libc::c_int as uint8_t;
        memcpy(
            p as *mut libc::c_void,
            &tv_9 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_9 = p_tmp_9.offset(1);
        p_tmp_9;
        p = p_tmp_9 as *mut libc::c_void as *mut uint8_t;
    }
    scrbuf_pos = movie_screen
        .offset(blitx as isize)
        .offset((blity * vid.width) as isize);
    scrbuf_writeend = scrbuf_pos
        .offset((blitw - 1 as libc::c_int) as isize)
        .offset(((blith - 1 as libc::c_int) * vid.width) as isize);
    if gifbwr_buf.is_null() {
        gifbwr_buf = Z_MallocAlign(
            256 as libc::c_int as size_t,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint8_t;
    }
    gifbwr_cur = gifbwr_buf;
    GIF_prepareLZW();
    giflzw_workingCode = 65535 as libc::c_int as uint16_t;
    let mut p_tmp_10: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_10: uint8_t = (gifbwr_bits_min as libc::c_int - 1 as libc::c_int) as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_10 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_10 = p_tmp_10.offset(1);
    p_tmp_10;
    p = p_tmp_10 as *mut libc::c_void as *mut uint8_t;
    startline = (scrbuf_pos.offset_from(movie_screen) as libc::c_long
        / vid.width as libc::c_long) as int32_t;
    scrbuf_linebegin = movie_screen
        .offset((startline * vid.width) as isize)
        .offset(blitx as isize);
    scrbuf_lineend = scrbuf_linebegin.offset(blitw as isize);
    GIF_bwrwrite(0x100 as libc::c_int as uint32_t);
    gif_writeover = 0 as libc::c_int as uint8_t;
    while gif_writeover == 0 {
        GIF_lzw();
        if (p.offset_from(gifframe_data) as libc::c_long as size_t)
            .wrapping_add(gifbwr_bufsize as size_t)
            .wrapping_add(1 as libc::c_int as size_t) >= gifframe_size
        {
            let mut temppos: int32_t = p.offset_from(gifframe_data) as libc::c_long
                as int32_t;
            gifframe_size = gifframe_size * 2 as libc::c_int as size_t;
            gifframe_data = Z_ReallocAlign(
                gifframe_data as *mut libc::c_void,
                gifframe_size,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
            p = gifframe_data.offset(temppos as isize);
        }
        gifbwr_cur = gifbwr_buf;
        let mut p_tmp_11: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv_11: uint8_t = gifbwr_bufsize;
        memcpy(
            p as *mut libc::c_void,
            &tv_11 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_11 = p_tmp_11.offset(1);
        p_tmp_11;
        p = p_tmp_11 as *mut libc::c_void as *mut uint8_t;
        memcpy(
            p as *mut libc::c_void,
            gifbwr_cur as *const libc::c_void,
            gifbwr_bufsize as libc::c_ulong,
        );
        p = p.offset(gifbwr_bufsize as libc::c_int as isize);
        gifbwr_bufsize = 0 as libc::c_int as uint8_t;
        gifbwr_cur = gifbwr_buf;
    }
    let mut p_tmp_12: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_12: uint8_t = 0 as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_12 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_12 = p_tmp_12.offset(1);
    p_tmp_12;
    p = p_tmp_12 as *mut libc::c_void as *mut uint8_t;
    fwrite(
        gifframe_data as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        p.offset_from(gifframe_data) as libc::c_long as libc::c_ulong,
        gif_out,
    );
    gif_frames += 1;
    gif_frames;
    gif_prevframetime = I_GetPreciseTime();
}
#[no_mangle]
pub unsafe extern "C" fn GIF_open(mut filename: *const libc::c_char) -> int32_t {
    gif_out = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    if gif_out.is_null() {
        return 0 as libc::c_int;
    }
    gif_optimize = (cv_gif_optimize.value != 0) as libc::c_int;
    gif_downscale = (cv_gif_downscale.value != 0) as libc::c_int;
    gif_dynamicdelay = cv_gif_dynamicdelay.value as uint8_t;
    gif_localcolortable = (cv_gif_localcolortable.value != 0) as libc::c_int;
    gif_colorprofile = (cv_screenshot_colorprofile.value != 0) as libc::c_int;
    gif_headerpalette = GIF_getpalette(0 as libc::c_int as size_t);
    GIF_headwrite();
    gif_frames = 0 as libc::c_int;
    gif_prevframetime = I_GetPreciseTime();
    gif_delayus = 0 as libc::c_int as uint32_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GIF_frame() {
    GIF_framewrite();
}
#[no_mangle]
pub unsafe extern "C" fn GIF_close() -> int32_t {
    if gif_out.is_null() {
        return 0 as libc::c_int;
    }
    fwrite(
        b";\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        gif_out,
    );
    fclose(gif_out);
    gif_out = 0 as *mut FILE;
    if !gifbwr_buf.is_null() {
        Z_Free(gifbwr_buf as *mut libc::c_void);
    }
    gifbwr_cur = 0 as *mut uint8_t;
    gifbwr_buf = gifbwr_cur;
    if !gifframe_data.is_null() {
        Z_Free(gifframe_data as *mut libc::c_void);
    }
    gifframe_data = 0 as *mut uint8_t;
    if !giflzw_hashTable.is_null() {
        Z_Free(giflzw_hashTable as *mut libc::c_void);
    }
    giflzw_hashTable = 0 as *mut uint32_t;
    CONS_Printf(
        b"Animated gif closed; wrote %d frames\n\0" as *const u8 as *const libc::c_char,
        gif_frames,
    );
    return 1 as libc::c_int;
}
