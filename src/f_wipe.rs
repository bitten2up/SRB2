use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut gamestate: gamestate_t;
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn I_UpdateNoBlit();
    fn I_FinishUpdate();
    fn I_ReadScreen(scr: *mut uint8_t);
    static mut pMasterPalette: *mut RGBA_t;
    static mut cv_timescale: consvar_t;
    fn V_DrawBlock(
        x: int32_t,
        y: int32_t,
        scrn: int32_t,
        width: int32_t,
        height: int32_t,
        src: *const uint8_t,
    );
    static mut vid: viddef_t;
    static mut screens: [*mut uint8_t; 5];
    fn V_DrawFill(x: int32_t, y: int32_t, w: int32_t, h: int32_t, c: int32_t);
    static mut cv_sleep: consvar_t;
    static mut fadecolormap: *mut lighttable_t;
    fn NetKeepAlive();
    fn R_GetTranslucencyTable(alphalevel: int32_t) -> *mut uint8_t;
    fn ST_runTitleCard();
    fn ST_drawWipeTitleCard();
    static mut st_overlay: boolean;
    fn W_CheckNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn W_LumpLength(lumpnum: lumpnum_t) -> size_t;
    fn W_CacheLumpNum(lump: lumpnum_t, tag: int32_t) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_ReallocAlign(
        ptr: *mut libc::c_void,
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn I_GetTime() -> tic_t;
    fn I_UpdateTime(timescale: fixed_t);
    fn I_Sleep(ms: uint32_t);
    fn I_OsPolling();
    fn M_Drawer();
    fn G_IsTitleCardAvailable() -> boolean;
    static mut moviemode: moviemode_t;
    fn M_SaveFrame();
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub type lumpnum_t = uint32_t;
pub type tic_t = uint32_t;
pub type gamestate_t = libc::c_uint;
pub const GS_WAITINGPLAYERS: gamestate_t = 13;
pub const GS_DEDICATEDSERVER: gamestate_t = 12;
pub const GS_CUTSCENE: gamestate_t = 11;
pub const GS_ENDING: gamestate_t = 10;
pub const GS_INTRO: gamestate_t = 9;
pub const GS_GAMEEND: gamestate_t = 8;
pub const GS_EVALUATION: gamestate_t = 7;
pub const GS_CREDITS: gamestate_t = 6;
pub const GS_TIMEATTACK: gamestate_t = 5;
pub const GS_TITLESCREEN: gamestate_t = 4;
pub const GS_CONTINUING: gamestate_t = 3;
pub const GS_INTERMISSION: gamestate_t = 2;
pub const GS_LEVEL: gamestate_t = 1;
pub const GS_NULL: gamestate_t = 0;
pub type fixed_t = int32_t;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type lighttable_t = uint8_t;
pub type wipestyle_t = libc::c_uint;
pub const WIPESTYLE_COLORMAP: wipestyle_t = 1;
pub const WIPESTYLE_NORMAL: wipestyle_t = 0;
pub type wipestyleflags_t = libc::c_uint;
pub const WSF_CROSSFADE: wipestyleflags_t = 8;
pub const WSF_TOWHITE: wipestyleflags_t = 4;
pub const WSF_FADEIN: wipestyleflags_t = 2;
pub const WSF_FADEOUT: wipestyleflags_t = 1;
pub type viddef_t = viddef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
pub type moviemode_t = libc::c_uint;
pub const MM_SCREENSHOT: moviemode_t = 3;
pub const MM_GIF: moviemode_t = 2;
pub const MM_APNG: moviemode_t = 1;
pub const MM_OFF: moviemode_t = 0;
pub type fademask_t = fademask_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fademask_s {
    pub mask: *mut uint8_t,
    pub width: uint16_t,
    pub height: uint16_t,
    pub size: size_t,
    pub xscale: fixed_t,
    pub yscale: fixed_t,
}
pub type consvar_t = consvar_s;
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
pub type CV_PossibleValue_t = CV_PossibleValue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CV_PossibleValue_s {
    pub value: int32_t,
    pub strvalue: *const libc::c_char,
}
pub const PU_STATIC: C2RustUnnamed_3 = 1;
pub const PU_CACHE: C2RustUnnamed_3 = 49;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_3 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_3 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_3 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_3 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_3 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_3 = 51;
pub const PU_LEVEL: C2RustUnnamed_3 = 50;
pub const PU_HWRCACHE: C2RustUnnamed_3 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_3 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_3 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_3 = 21;
pub const PU_HUDGFX: C2RustUnnamed_3 = 19;
pub const PU_SPRITE: C2RustUnnamed_3 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_3 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_3 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_3 = 15;
pub const PU_PATCH: C2RustUnnamed_3 = 14;
pub const PU_MUSIC: C2RustUnnamed_3 = 12;
pub const PU_SOUND: C2RustUnnamed_3 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_3 = 3;
pub const PU_LUA: C2RustUnnamed_3 = 2;
#[inline(always)]
unsafe extern "C" fn FixedDiv2(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return (a as int64_t * ((1 as libc::c_int) << 16 as libc::c_int) as int64_t
        / b as int64_t) as fixed_t;
}
#[inline(always)]
unsafe extern "C" fn FixedDiv(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    if abs(a) >> 16 as libc::c_int - 2 as libc::c_int >= abs(b) {
        return if a ^ b < 0 as libc::c_int {
            -(2147483647 as libc::c_int) - 1 as libc::c_int
        } else {
            2147483647 as libc::c_int
        };
    }
    return FixedDiv2(a, b);
}
#[no_mangle]
pub static mut wipedefs: [uint8_t; 28] = [
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    255 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    99 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
#[no_mangle]
pub static mut WipeInAction: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut WipeStageTitle: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut lastwipetic: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut wipestyle: wipestyle_t = WIPESTYLE_NORMAL;
#[no_mangle]
pub static mut wipestyleflags: wipestyleflags_t = WSF_CROSSFADE;
static mut wipe_scr_start: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut wipe_scr_end: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut wipe_scr: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut paldiv: fixed_t = 0 as libc::c_int;
unsafe extern "C" fn F_GetFadeMask(
    mut masknum: uint8_t,
    mut scrnnum: uint8_t,
) -> *mut fademask_t {
    let mut current_block: u64;
    static mut lumpname: [libc::c_char; 10] = unsafe {
        *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"FADEmmss\0\0")
    };
    static mut fm: fademask_t = {
        let mut init = fademask_s {
            mask: 0 as *const uint8_t as *mut uint8_t,
            width: 0 as libc::c_int as uint16_t,
            height: 0 as libc::c_int as uint16_t,
            size: 0 as libc::c_int as size_t,
            xscale: 0 as libc::c_int,
            yscale: 0 as libc::c_int,
        };
        init
    };
    let mut lumpnum: lumpnum_t = 0;
    let mut lump: *mut uint8_t = 0 as *mut uint8_t;
    let mut mask: *mut uint8_t = 0 as *mut uint8_t;
    let mut lsize: size_t = 0;
    let mut pcolor: *mut RGBA_t = 0 as *mut RGBA_t;
    if !(masknum as libc::c_int > 99 as libc::c_int
        || scrnnum as libc::c_int > 99 as libc::c_int)
    {
        sprintf(
            &mut *lumpname.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_char,
            b"%.2hu%.2hu\0" as *const u8 as *const libc::c_char,
            masknum as uint16_t as libc::c_int,
            scrnnum as uint16_t as libc::c_int,
        );
        lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
        if !(lumpnum == 4294967295 as libc::c_uint) {
            lump = W_CacheLumpNum(lumpnum, PU_CACHE as libc::c_int) as *mut uint8_t;
            lsize = W_LumpLength(lumpnum);
            match lsize {
                256000 => {
                    fm.width = 640 as libc::c_int as uint16_t;
                    fm.height = 400 as libc::c_int as uint16_t;
                    current_block = 4956146061682418353;
                }
                64000 => {
                    fm.width = 320 as libc::c_int as uint16_t;
                    fm.height = 200 as libc::c_int as uint16_t;
                    current_block = 4956146061682418353;
                }
                16000 => {
                    fm.width = 160 as libc::c_int as uint16_t;
                    fm.height = 100 as libc::c_int as uint16_t;
                    current_block = 4956146061682418353;
                }
                4000 => {
                    fm.width = 80 as libc::c_int as uint16_t;
                    fm.height = 50 as libc::c_int as uint16_t;
                    current_block = 4956146061682418353;
                }
                0 => {
                    current_block = 5546394164456084279;
                }
                _ => {
                    CONS_Alert(
                        CONS_WARNING,
                        b"Fade mask lump %s of incorrect size, ignored\n\0" as *const u8
                            as *const libc::c_char,
                        lumpname.as_mut_ptr(),
                    );
                    current_block = 5546394164456084279;
                }
            }
            match current_block {
                4956146061682418353 => {
                    if lsize != fm.size {
                        fm
                            .mask = Z_ReallocAlign(
                            fm.mask as *mut libc::c_void,
                            lsize,
                            PU_STATIC as libc::c_int,
                            0 as *mut libc::c_void,
                            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                                as int32_t,
                        ) as *mut uint8_t;
                    }
                    fm.size = lsize;
                    mask = fm.mask;
                    loop {
                        let fresh0 = lsize;
                        lsize = lsize.wrapping_sub(1);
                        if !(fresh0 != 0) {
                            break;
                        }
                        let fresh1 = lump;
                        lump = lump.offset(1);
                        pcolor = &mut *pMasterPalette.offset(*fresh1 as isize)
                            as *mut RGBA_t;
                        if wipestyle as libc::c_uint
                            == WIPESTYLE_COLORMAP as libc::c_int as libc::c_uint
                        {
                            let fresh2 = mask;
                            mask = mask.offset(1);
                            *fresh2 = ((*pcolor).s.red as libc::c_int / 8 as libc::c_int)
                                as uint8_t;
                        } else {
                            let fresh3 = mask;
                            mask = mask.offset(1);
                            *fresh3 = (FixedDiv(
                                ((*pcolor).s.red as libc::c_int + 1 as libc::c_int)
                                    << 16 as libc::c_int,
                                paldiv,
                            ) >> 16 as libc::c_int) as uint8_t;
                        }
                    }
                    fm
                        .xscale = FixedDiv(
                        vid.width << 16 as libc::c_int,
                        (fm.width as libc::c_int) << 16 as libc::c_int,
                    );
                    fm
                        .yscale = FixedDiv(
                        vid.height << 16 as libc::c_int,
                        (fm.height as libc::c_int) << 16 as libc::c_int,
                    );
                    return &mut fm;
                }
                _ => {}
            }
        }
    }
    if !(fm.mask).is_null() {
        Z_Free(fm.mask as *mut libc::c_void);
        fm.mask = 0 as *mut uint8_t;
        fm.size = 0 as libc::c_int as size_t;
    }
    return 0 as *mut fademask_t;
}
#[no_mangle]
pub unsafe extern "C" fn F_WipeStageTitle() {
    if WipeStageTitle != 0 && st_overlay != 0
        && wipestyle as libc::c_uint == WIPESTYLE_COLORMAP as libc::c_int as libc::c_uint
        && G_IsTitleCardAvailable() != 0
    {
        ST_runTitleCard();
        ST_drawWipeTitleCard();
    }
}
unsafe extern "C" fn F_DoWipe(mut fademask: *mut fademask_t) {
    let mut w: *mut uint8_t = wipe_scr;
    let mut s: *const uint8_t = wipe_scr_start;
    let mut e: *const uint8_t = wipe_scr_end;
    let mut w_base: *mut uint8_t = w;
    let mut s_base: *const uint8_t = s;
    let mut e_base: *const uint8_t = e;
    let mut transtbl: *mut uint8_t = 0 as *mut uint8_t;
    let mut mask: *const uint8_t = (*fademask).mask;
    let mut maskend: *const uint8_t = mask.offset((*fademask).size as isize);
    let mut draw_linestart: uint32_t = 0;
    let mut draw_rowstart: uint32_t = 0;
    let mut draw_lineend: uint32_t = 0;
    let mut draw_rowend: uint32_t = 0;
    let mut draw_linestogo: uint32_t = 0;
    let mut draw_rowstogo: uint32_t = 0;
    let mut scrxpos: *mut uint16_t = malloc(
        (((*fademask).width as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint16_t;
    let mut scrypos: *mut uint16_t = malloc(
        (((*fademask).height as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint16_t;
    let mut maskx: uint16_t = 0;
    let mut masky: uint16_t = 0;
    let mut relativepos: uint32_t = 0;
    *scrxpos.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    relativepos = 0 as libc::c_int as uint32_t;
    maskx = 1 as libc::c_int as uint16_t;
    while (maskx as libc::c_int) < (*fademask).width as libc::c_int {
        relativepos = relativepos.wrapping_add((*fademask).xscale as uint32_t);
        *scrxpos.offset(maskx as isize) = (relativepos >> 16 as libc::c_int) as uint16_t;
        maskx = maskx.wrapping_add(1);
        maskx;
    }
    *scrxpos.offset((*fademask).width as isize) = vid.width as uint16_t;
    *scrypos.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    relativepos = 0 as libc::c_int as uint32_t;
    masky = 1 as libc::c_int as uint16_t;
    while (masky as libc::c_int) < (*fademask).height as libc::c_int {
        relativepos = relativepos.wrapping_add((*fademask).yscale as uint32_t);
        *scrypos.offset(masky as isize) = (relativepos >> 16 as libc::c_int) as uint16_t;
        masky = masky.wrapping_add(1);
        masky;
    }
    *scrypos.offset((*fademask).height as isize) = vid.height as uint16_t;
    masky = 0 as libc::c_int as uint16_t;
    maskx = masky;
    loop {
        draw_rowstart = *scrxpos.offset(maskx as isize) as uint32_t;
        draw_rowend = *scrxpos.offset((maskx as libc::c_int + 1 as libc::c_int) as isize)
            as uint32_t;
        draw_linestart = *scrypos.offset(masky as isize) as uint32_t;
        draw_lineend = *scrypos
            .offset((masky as libc::c_int + 1 as libc::c_int) as isize) as uint32_t;
        relativepos = (draw_linestart * vid.width as uint32_t)
            .wrapping_add(draw_rowstart);
        draw_linestogo = draw_lineend.wrapping_sub(draw_linestart);
        if *mask as libc::c_int == 0 as libc::c_int {
            loop {
                let fresh4 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh4 != 0) {
                    break;
                }
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    w_base.offset(relativepos as isize) as *mut libc::c_void,
                    s_base.offset(relativepos as isize) as *const libc::c_void,
                    draw_rowend.wrapping_sub(draw_rowstart) as size_t,
                );
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        } else if *mask as libc::c_int >= 10 as libc::c_int {
            loop {
                let fresh5 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh5 != 0) {
                    break;
                }
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    w_base.offset(relativepos as isize) as *mut libc::c_void,
                    e_base.offset(relativepos as isize) as *const libc::c_void,
                    draw_rowend.wrapping_sub(draw_rowstart) as size_t,
                );
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        } else {
            transtbl = R_GetTranslucencyTable(
                9 as libc::c_int - *mask as libc::c_int + 1 as libc::c_int,
            );
            loop {
                let fresh6 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh6 != 0) {
                    break;
                }
                w = w_base.offset(relativepos as isize);
                s = s_base.offset(relativepos as isize);
                e = e_base.offset(relativepos as isize);
                draw_rowstogo = draw_rowend.wrapping_sub(draw_rowstart);
                loop {
                    let fresh7 = draw_rowstogo;
                    draw_rowstogo = draw_rowstogo.wrapping_sub(1);
                    if !(fresh7 != 0) {
                        break;
                    }
                    let fresh8 = e;
                    e = e.offset(1);
                    let fresh9 = s;
                    s = s.offset(1);
                    let fresh10 = w;
                    w = w.offset(1);
                    *fresh10 = *transtbl
                        .offset(
                            (((*fresh8 as libc::c_int) << 8 as libc::c_int)
                                + *fresh9 as libc::c_int) as isize,
                        );
                }
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        }
        maskx = maskx.wrapping_add(1);
        if maskx as libc::c_int >= (*fademask).width as libc::c_int {
            masky = masky.wrapping_add(1);
            masky;
            maskx = 0 as libc::c_int as uint16_t;
        }
        mask = mask.offset(1);
        if !(mask < maskend) {
            break;
        }
    }
    free(scrxpos as *mut libc::c_void);
    free(scrypos as *mut libc::c_void);
}
unsafe extern "C" fn F_DoColormapWipe(
    mut fademask: *mut fademask_t,
    mut colormap: *mut uint8_t,
) {
    let mut w: *mut uint8_t = wipe_scr;
    let mut s: *const uint8_t = wipe_scr_start;
    let mut e: *const uint8_t = wipe_scr_end;
    let mut w_base: *mut uint8_t = w;
    let mut s_base: *const uint8_t = s;
    let mut e_base: *const uint8_t = e;
    let mut transtbl: *mut uint8_t = 0 as *mut uint8_t;
    let mut mask: *const uint8_t = (*fademask).mask;
    let mut maskend: *const uint8_t = mask.offset((*fademask).size as isize);
    let mut draw_linestart: uint32_t = 0;
    let mut draw_rowstart: uint32_t = 0;
    let mut draw_lineend: uint32_t = 0;
    let mut draw_rowend: uint32_t = 0;
    let mut draw_linestogo: uint32_t = 0;
    let mut draw_rowstogo: uint32_t = 0;
    let mut scrxpos: *mut uint16_t = malloc(
        (((*fademask).width as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint16_t;
    let mut scrypos: *mut uint16_t = malloc(
        (((*fademask).height as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<uint16_t>() as libc::c_ulong),
    ) as *mut uint16_t;
    let mut maskx: uint16_t = 0;
    let mut masky: uint16_t = 0;
    let mut relativepos: uint32_t = 0;
    *scrxpos.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    relativepos = 0 as libc::c_int as uint32_t;
    maskx = 1 as libc::c_int as uint16_t;
    while (maskx as libc::c_int) < (*fademask).width as libc::c_int {
        relativepos = relativepos.wrapping_add((*fademask).xscale as uint32_t);
        *scrxpos.offset(maskx as isize) = (relativepos >> 16 as libc::c_int) as uint16_t;
        maskx = maskx.wrapping_add(1);
        maskx;
    }
    *scrxpos.offset((*fademask).width as isize) = vid.width as uint16_t;
    *scrypos.offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint16_t;
    relativepos = 0 as libc::c_int as uint32_t;
    masky = 1 as libc::c_int as uint16_t;
    while (masky as libc::c_int) < (*fademask).height as libc::c_int {
        relativepos = relativepos.wrapping_add((*fademask).yscale as uint32_t);
        *scrypos.offset(masky as isize) = (relativepos >> 16 as libc::c_int) as uint16_t;
        masky = masky.wrapping_add(1);
        masky;
    }
    *scrypos.offset((*fademask).height as isize) = vid.height as uint16_t;
    masky = 0 as libc::c_int as uint16_t;
    maskx = masky;
    loop {
        draw_rowstart = *scrxpos.offset(maskx as isize) as uint32_t;
        draw_rowend = *scrxpos.offset((maskx as libc::c_int + 1 as libc::c_int) as isize)
            as uint32_t;
        draw_linestart = *scrypos.offset(masky as isize) as uint32_t;
        draw_lineend = *scrypos
            .offset((masky as libc::c_int + 1 as libc::c_int) as isize) as uint32_t;
        relativepos = (draw_linestart * vid.width as uint32_t)
            .wrapping_add(draw_rowstart);
        draw_linestogo = draw_lineend.wrapping_sub(draw_linestart);
        if *mask as libc::c_int == 0 as libc::c_int {
            loop {
                let fresh11 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh11 != 0) {
                    break;
                }
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    w_base.offset(relativepos as isize) as *mut libc::c_void,
                    s_base.offset(relativepos as isize) as *const libc::c_void,
                    draw_rowend.wrapping_sub(draw_rowstart) as size_t,
                );
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        } else if *mask as libc::c_int >= 256 as libc::c_int / 8 as libc::c_int {
            loop {
                let fresh12 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh12 != 0) {
                    break;
                }
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    w_base.offset(relativepos as isize) as *mut libc::c_void,
                    e_base.offset(relativepos as isize) as *const libc::c_void,
                    draw_rowend.wrapping_sub(draw_rowstart) as size_t,
                );
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        } else {
            let mut nmask: libc::c_int = *mask as libc::c_int;
            if wipestyleflags as libc::c_uint & WSF_FADEIN as libc::c_int as libc::c_uint
                != 0
            {
                nmask = 256 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int - nmask;
            }
            transtbl = colormap.offset((nmask * 256 as libc::c_int) as isize);
            loop {
                let fresh13 = draw_linestogo;
                draw_linestogo = draw_linestogo.wrapping_sub(1);
                if !(fresh13 != 0) {
                    break;
                }
                w = w_base.offset(relativepos as isize);
                s = s_base.offset(relativepos as isize);
                e = e_base.offset(relativepos as isize);
                draw_rowstogo = draw_rowend.wrapping_sub(draw_rowstart);
                loop {
                    let fresh14 = draw_rowstogo;
                    draw_rowstogo = draw_rowstogo.wrapping_sub(1);
                    if !(fresh14 != 0) {
                        break;
                    }
                    let fresh15 = e;
                    e = e.offset(1);
                    let fresh16 = w;
                    w = w.offset(1);
                    *fresh16 = *transtbl.offset(*fresh15 as isize);
                }
                relativepos = relativepos.wrapping_add(vid.width as uint32_t);
            }
        }
        maskx = maskx.wrapping_add(1);
        if maskx as libc::c_int >= (*fademask).width as libc::c_int {
            masky = masky.wrapping_add(1);
            masky;
            maskx = 0 as libc::c_int as uint16_t;
        }
        mask = mask.offset(1);
        if !(mask < maskend) {
            break;
        }
    }
    free(scrxpos as *mut libc::c_void);
    free(scrypos as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn F_WipeStartScreen() {
    wipe_scr_start = screens[3 as libc::c_int as usize];
    I_ReadScreen(wipe_scr_start);
}
#[no_mangle]
pub unsafe extern "C" fn F_WipeEndScreen() {
    wipe_scr_end = screens[4 as libc::c_int as usize];
    I_ReadScreen(wipe_scr_end);
    V_DrawBlock(
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        vid.width,
        vid.height,
        wipe_scr_start,
    );
}
#[no_mangle]
pub unsafe extern "C" fn F_ShouldColormapFade() -> boolean {
    if wipestyleflags as libc::c_uint
        & (WSF_FADEIN as libc::c_int | WSF_FADEOUT as libc::c_int) as libc::c_uint != 0
        && wipestyleflags as libc::c_uint & WSF_CROSSFADE as libc::c_int as libc::c_uint
            == 0
    {
        return (gamestate as libc::c_uint == GS_LEVEL as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_TITLESCREEN as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_CONTINUING as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_CREDITS as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_EVALUATION as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_INTRO as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_ENDING as libc::c_int as libc::c_uint
            || gamestate as libc::c_uint == GS_TIMEATTACK as libc::c_int as libc::c_uint)
            as libc::c_int;
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_DecideWipeStyle() {
    wipestyle = WIPESTYLE_NORMAL;
    if F_ShouldColormapFade() != 0 {
        wipestyle = WIPESTYLE_COLORMAP;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_TryColormapFade(mut wipecolor: uint8_t) -> boolean {
    if F_ShouldColormapFade() != 0 {
        return true_0 as libc::c_int
    } else {
        V_DrawFill(
            0 as libc::c_int,
            0 as libc::c_int,
            320 as libc::c_int,
            200 as libc::c_int,
            wipecolor as int32_t,
        );
        return false_0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_RunWipe(mut wipetype: uint8_t, mut drawMenu: boolean) {
    let mut nowtime: tic_t = 0;
    let mut wipeframe: uint8_t = 0 as libc::c_int as uint8_t;
    let mut fmask: *mut fademask_t = 0 as *mut fademask_t;
    if paldiv == 0 {
        paldiv = FixedDiv(
            (257 as libc::c_int) << 16 as libc::c_int,
            (11 as libc::c_int) << 16 as libc::c_int,
        );
    }
    F_DecideWipeStyle();
    WipeInAction = true_0 as libc::c_int;
    wipe_scr = screens[0 as libc::c_int as usize];
    loop {
        let fresh17 = wipeframe;
        wipeframe = wipeframe.wrapping_add(1);
        fmask = F_GetFadeMask(wipetype, fresh17);
        if fmask.is_null() {
            break;
        }
        loop {
            nowtime = I_GetTime();
            if !(nowtime.wrapping_sub(lastwipetic as tic_t) == 0) {
                break;
            }
            I_Sleep(cv_sleep.value as uint32_t);
            I_UpdateTime(cv_timescale.value);
        }
        lastwipetic = nowtime as int32_t;
        if wipestyle as libc::c_uint == WIPESTYLE_COLORMAP as libc::c_int as libc::c_uint
        {
            let mut colormap: *mut uint8_t = fadecolormap;
            if wipestyleflags as libc::c_uint
                & WSF_TOWHITE as libc::c_int as libc::c_uint != 0
            {
                colormap = colormap
                    .offset(
                        (256 as libc::c_int / 8 as libc::c_int * 256 as libc::c_int)
                            as isize,
                    );
            }
            F_DoColormapWipe(fmask, colormap);
            F_WipeStageTitle();
        } else {
            F_DoWipe(fmask);
        }
        I_OsPolling();
        I_UpdateNoBlit();
        if drawMenu != 0 {
            M_Drawer();
        }
        I_FinishUpdate();
        if moviemode as u64 != 0 {
            M_SaveFrame();
        }
        NetKeepAlive();
    }
    WipeInAction = false_0 as libc::c_int;
    WipeStageTitle = false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_GetWipeLength(mut wipetype: uint8_t) -> tic_t {
    static mut lumpname: [libc::c_char; 10] = unsafe {
        *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"FADEmmss\0\0")
    };
    let mut lumpnum: lumpnum_t = 0;
    let mut wipeframe: uint8_t = 0;
    if wipetype as libc::c_int > 99 as libc::c_int {
        return 0 as libc::c_int as tic_t;
    }
    wipeframe = 0 as libc::c_int as uint8_t;
    while (wipeframe as libc::c_int) < 100 as libc::c_int {
        sprintf(
            &mut *lumpname.as_mut_ptr().offset(4 as libc::c_int as isize)
                as *mut libc::c_char,
            b"%.2hu%.2hu\0" as *const u8 as *const libc::c_char,
            wipetype as uint16_t as libc::c_int,
            wipeframe as uint16_t as libc::c_int,
        );
        lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
        if lumpnum == 4294967295 as libc::c_uint {
            wipeframe = wipeframe.wrapping_sub(1);
            return wipeframe as tic_t;
        }
        wipeframe = wipeframe.wrapping_add(1);
        wipeframe;
    }
    wipeframe = wipeframe.wrapping_sub(1);
    return wipeframe as tic_t;
}
#[no_mangle]
pub unsafe extern "C" fn F_WipeExists(mut wipetype: uint8_t) -> boolean {
    static mut lumpname: [libc::c_char; 10] = unsafe {
        *::core::mem::transmute::<&[u8; 10], &mut [libc::c_char; 10]>(b"FADEmm00\0\0")
    };
    let mut lumpnum: lumpnum_t = 0;
    if wipetype as libc::c_int > 99 as libc::c_int {
        return false_0 as libc::c_int;
    }
    sprintf(
        &mut *lumpname.as_mut_ptr().offset(4 as libc::c_int as isize)
            as *mut libc::c_char,
        b"%.2hu00\0" as *const u8 as *const libc::c_char,
        wipetype as uint16_t as libc::c_int,
    );
    lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
    return !(lumpnum == 4294967295 as libc::c_uint) as libc::c_int;
}
