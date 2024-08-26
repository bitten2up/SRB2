use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn I_GetKey() -> int32_t;
    static mut capslock: boolean;
    static mut ctrldown: uint8_t;
    static mut shiftdown: uint8_t;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut cv_debug: int32_t;
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn I_OutputMsg(error: *const libc::c_char, _: ...);
    static mut gamestate: gamestate_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn COM_AddCommand(name: *const libc::c_char, func: com_func_t, flags: com_flags_t);
    fn COM_Argc() -> size_t;
    fn COM_Argv(arg: size_t) -> *const libc::c_char;
    fn COM_BufAddTextEx(btext: *const libc::c_char, flags: com_flags_t);
    fn COM_CompleteAlias(
        partial: *const libc::c_char,
        skips: int32_t,
    ) -> *const libc::c_char;
    fn COM_CompleteCommand(
        partial: *const libc::c_char,
        skips: int32_t,
    ) -> *const libc::c_char;
    fn CV_RegisterVar(variable: *mut consvar_t);
    fn CV_CompleteVar(partial: *mut libc::c_char, skips: int32_t) -> *const libc::c_char;
    static mut CV_Unsigned: [CV_PossibleValue_t; 0];
    static mut menuactive: boolean;
    static mut marathonmode: marathonmode_t;
    static mut metalrecording: boolean;
    static mut modeattacking: uint8_t;
    static mut viewwindowy: int32_t;
    static mut cv_consolechat: consvar_t;
    static mut dedicated: boolean;
    static mut splitscreen: boolean;
    static mut debugfile: *mut FILE;
    fn G_KeyNumToName(keynum: int32_t) -> *const libc::c_char;
    fn G_KeyNameToNum(keystr: *const libc::c_char) -> int32_t;
    static mut gamecontrol: [[int32_t; 2]; 43];
    static mut shiftxform: *mut libc::c_char;
    static mut chat_on: boolean;
    fn W_CacheLumpName(name: *const libc::c_char, tag: int32_t) -> *mut libc::c_void;
    fn W_UnlockCachedPatch(patch: *mut libc::c_void);
    fn W_CachePatchNum(lumpnum: lumpnum_t, tag: int32_t) -> *mut libc::c_void;
    fn W_GetNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn W_CheckNumForName(name: *const libc::c_char) -> lumpnum_t;
    static mut vid: viddef_t;
    static mut renderdeltatics: fixed_t;
    fn S_StartSound(origin: *const libc::c_void, sound_id: sfxenum_t);
    static mut cv_constextsize: consvar_t;
    fn GetPalette() -> *const libc::c_char;
    fn V_DrawCroppedPatch(
        x: fixed_t,
        y: fixed_t,
        pscale: fixed_t,
        vscale: fixed_t,
        scrn: int32_t,
        patch: *mut patch_t,
        colormap: *const uint8_t,
        sx: fixed_t,
        sy: fixed_t,
        w: fixed_t,
        h: fixed_t,
    );
    fn V_DrawFill(x: int32_t, y: int32_t, w: int32_t, h: int32_t, c: int32_t);
    fn V_DrawFadeConsBack(plines: int32_t);
    fn V_DrawCharacter(x: int32_t, y: int32_t, c: int32_t, lowercaseallowed: boolean);
    static mut rendermode: rendermode_t;
    fn I_FinishUpdate();
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_MallocAlign(
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn Z_StrDup(in_0: *const libc::c_char) -> *mut libc::c_char;
    static mut graphics_started: uint8_t;
    fn I_OsPolling();
    fn I_UpdateMouseGrab();
    fn I_ClipboardCopy(data: *const libc::c_char, size: size_t) -> int32_t;
    fn I_ClipboardPaste() -> *const libc::c_char;
    static mut refreshdirmenu: uint8_t;
    fn M_JumpWord(s: *const libc::c_char) -> libc::c_int;
    fn M_JumpWordReverse(line: *const libc::c_char, offset: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type lumpnum_t = uint32_t;
pub type tic_t = uint32_t;
pub type va_list = __builtin_va_list;
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
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
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
    pub revert: C2RustUnnamed_0,
    pub netid: uint16_t,
    pub changed: libc::c_char,
    pub next: *mut consvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub allocated: libc::c_char,
    pub v: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type viddef_t = viddef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed_2,
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
pub union C2RustUnnamed_2 {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
pub const render_none: rendermode_t = 3;
pub type rendermode_t = libc::c_uint;
pub const render_opengl: rendermode_t = 2;
pub const render_soft: rendermode_t = 1;
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
pub type fixed_t = int32_t;
pub type post_t = column_t;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct column_t {
    pub topdelta: uint8_t,
    pub length: uint8_t,
}
pub const PU_PATCH: C2RustUnnamed_6 = 14;
pub const CV_SAVE: C2RustUnnamed_3 = 1;
pub type sfxenum_t = libc::c_uint;
pub const NUMSFX: sfxenum_t = 3008;
pub const sfx_lastskinsoundslot: sfxenum_t = 3007;
pub const sfx_skinsoundslot0: sfxenum_t = 2336;
pub const sfx_lastfreeslot: sfxenum_t = 2335;
pub const sfx_freeslot0: sfxenum_t = 736;
pub const sfx_kc6e: sfxenum_t = 735;
pub const sfx_kc6d: sfxenum_t = 734;
pub const sfx_kc6c: sfxenum_t = 733;
pub const sfx_kc6b: sfxenum_t = 732;
pub const sfx_kc69: sfxenum_t = 731;
pub const sfx_kc68: sfxenum_t = 730;
pub const sfx_kc67: sfxenum_t = 729;
pub const sfx_kc66: sfxenum_t = 728;
pub const sfx_kc65: sfxenum_t = 727;
pub const sfx_kc64: sfxenum_t = 726;
pub const sfx_kc63: sfxenum_t = 725;
pub const sfx_kc62: sfxenum_t = 724;
pub const sfx_kc61: sfxenum_t = 723;
pub const sfx_kc60: sfxenum_t = 722;
pub const sfx_kc5f: sfxenum_t = 721;
pub const sfx_kc5e: sfxenum_t = 720;
pub const sfx_kc5d: sfxenum_t = 719;
pub const sfx_kc5c: sfxenum_t = 718;
pub const sfx_kc5b: sfxenum_t = 717;
pub const sfx_kc5a: sfxenum_t = 716;
pub const sfx_kc59: sfxenum_t = 715;
pub const sfx_kc58: sfxenum_t = 714;
pub const sfx_kc57: sfxenum_t = 713;
pub const sfx_kc56: sfxenum_t = 712;
pub const sfx_kc55: sfxenum_t = 711;
pub const sfx_kc54: sfxenum_t = 710;
pub const sfx_kc53: sfxenum_t = 709;
pub const sfx_kc52: sfxenum_t = 708;
pub const sfx_kc51: sfxenum_t = 707;
pub const sfx_kc50: sfxenum_t = 706;
pub const sfx_kc4f: sfxenum_t = 705;
pub const sfx_kc4e: sfxenum_t = 704;
pub const sfx_kc4d: sfxenum_t = 703;
pub const sfx_kc4c: sfxenum_t = 702;
pub const sfx_kc4b: sfxenum_t = 701;
pub const sfx_kc4a: sfxenum_t = 700;
pub const sfx_kc49: sfxenum_t = 699;
pub const sfx_kc48: sfxenum_t = 698;
pub const sfx_kc47: sfxenum_t = 697;
pub const sfx_kc46: sfxenum_t = 696;
pub const sfx_kc45: sfxenum_t = 695;
pub const sfx_kc44: sfxenum_t = 694;
pub const sfx_kc43: sfxenum_t = 693;
pub const sfx_kc42: sfxenum_t = 692;
pub const sfx_kc41: sfxenum_t = 691;
pub const sfx_kc40: sfxenum_t = 690;
pub const sfx_kc3f: sfxenum_t = 689;
pub const sfx_kc3e: sfxenum_t = 688;
pub const sfx_kc3d: sfxenum_t = 687;
pub const sfx_kc3c: sfxenum_t = 686;
pub const sfx_kc3b: sfxenum_t = 685;
pub const sfx_kc3a: sfxenum_t = 684;
pub const sfx_kc39: sfxenum_t = 683;
pub const sfx_kc38: sfxenum_t = 682;
pub const sfx_kc37: sfxenum_t = 681;
pub const sfx_kc36: sfxenum_t = 680;
pub const sfx_kc35: sfxenum_t = 679;
pub const sfx_kc34: sfxenum_t = 678;
pub const sfx_kc33: sfxenum_t = 677;
pub const sfx_kc32: sfxenum_t = 676;
pub const sfx_kc31: sfxenum_t = 675;
pub const sfx_kc30: sfxenum_t = 674;
pub const sfx_kc2f: sfxenum_t = 673;
pub const sfx_kc2e: sfxenum_t = 672;
pub const sfx_kc2d: sfxenum_t = 671;
pub const sfx_kc2c: sfxenum_t = 670;
pub const sfx_kc2b: sfxenum_t = 669;
pub const sfx_kc2a: sfxenum_t = 668;
pub const sfx_cdpcm9: sfxenum_t = 667;
pub const sfx_cdpcm8: sfxenum_t = 666;
pub const sfx_cdpcm7: sfxenum_t = 665;
pub const sfx_cdpcm6: sfxenum_t = 664;
pub const sfx_cdpcm5: sfxenum_t = 663;
pub const sfx_cdpcm4: sfxenum_t = 662;
pub const sfx_cdpcm3: sfxenum_t = 661;
pub const sfx_cdpcm2: sfxenum_t = 660;
pub const sfx_cdpcm1: sfxenum_t = 659;
pub const sfx_cdpcm0: sfxenum_t = 658;
pub const sfx_cdfm79: sfxenum_t = 657;
pub const sfx_cdfm78: sfxenum_t = 656;
pub const sfx_cdfm77: sfxenum_t = 655;
pub const sfx_cdfm76: sfxenum_t = 654;
pub const sfx_cdfm75: sfxenum_t = 653;
pub const sfx_cdfm74: sfxenum_t = 652;
pub const sfx_cdfm73: sfxenum_t = 651;
pub const sfx_cdfm72: sfxenum_t = 650;
pub const sfx_cdfm71: sfxenum_t = 649;
pub const sfx_cdfm70: sfxenum_t = 648;
pub const sfx_cdfm69: sfxenum_t = 647;
pub const sfx_cdfm68: sfxenum_t = 646;
pub const sfx_cdfm67: sfxenum_t = 645;
pub const sfx_cdfm66: sfxenum_t = 644;
pub const sfx_cdfm65: sfxenum_t = 643;
pub const sfx_cdfm64: sfxenum_t = 642;
pub const sfx_cdfm63: sfxenum_t = 641;
pub const sfx_cdfm62: sfxenum_t = 640;
pub const sfx_cdfm61: sfxenum_t = 639;
pub const sfx_cdfm60: sfxenum_t = 638;
pub const sfx_cdfm59: sfxenum_t = 637;
pub const sfx_cdfm58: sfxenum_t = 636;
pub const sfx_cdfm57: sfxenum_t = 635;
pub const sfx_cdfm56: sfxenum_t = 634;
pub const sfx_cdfm55: sfxenum_t = 633;
pub const sfx_cdfm54: sfxenum_t = 632;
pub const sfx_cdfm53: sfxenum_t = 631;
pub const sfx_cdfm52: sfxenum_t = 630;
pub const sfx_cdfm51: sfxenum_t = 629;
pub const sfx_cdfm50: sfxenum_t = 628;
pub const sfx_cdfm49: sfxenum_t = 627;
pub const sfx_cdfm48: sfxenum_t = 626;
pub const sfx_cdfm47: sfxenum_t = 625;
pub const sfx_cdfm46: sfxenum_t = 624;
pub const sfx_cdfm45: sfxenum_t = 623;
pub const sfx_cdfm44: sfxenum_t = 622;
pub const sfx_cdfm43: sfxenum_t = 621;
pub const sfx_cdfm42: sfxenum_t = 620;
pub const sfx_cdfm41: sfxenum_t = 619;
pub const sfx_cdfm40: sfxenum_t = 618;
pub const sfx_cdfm39: sfxenum_t = 617;
pub const sfx_cdfm38: sfxenum_t = 616;
pub const sfx_cdfm37: sfxenum_t = 615;
pub const sfx_cdfm36: sfxenum_t = 614;
pub const sfx_cdfm35: sfxenum_t = 613;
pub const sfx_cdfm34: sfxenum_t = 612;
pub const sfx_cdfm33: sfxenum_t = 611;
pub const sfx_cdfm32: sfxenum_t = 610;
pub const sfx_cdfm31: sfxenum_t = 609;
pub const sfx_cdfm30: sfxenum_t = 608;
pub const sfx_cdfm29: sfxenum_t = 607;
pub const sfx_cdfm28: sfxenum_t = 606;
pub const sfx_cdfm27: sfxenum_t = 605;
pub const sfx_cdfm26: sfxenum_t = 604;
pub const sfx_cdfm25: sfxenum_t = 603;
pub const sfx_cdfm24: sfxenum_t = 602;
pub const sfx_cdfm23: sfxenum_t = 601;
pub const sfx_cdfm22: sfxenum_t = 600;
pub const sfx_cdfm21: sfxenum_t = 599;
pub const sfx_cdfm20: sfxenum_t = 598;
pub const sfx_cdfm19: sfxenum_t = 597;
pub const sfx_cdfm18: sfxenum_t = 596;
pub const sfx_cdfm17: sfxenum_t = 595;
pub const sfx_cdfm16: sfxenum_t = 594;
pub const sfx_cdfm15: sfxenum_t = 593;
pub const sfx_cdfm14: sfxenum_t = 592;
pub const sfx_cdfm13: sfxenum_t = 591;
pub const sfx_cdfm12: sfxenum_t = 590;
pub const sfx_cdfm11: sfxenum_t = 589;
pub const sfx_cdfm10: sfxenum_t = 588;
pub const sfx_cdfm09: sfxenum_t = 587;
pub const sfx_cdfm08: sfxenum_t = 586;
pub const sfx_cdfm07: sfxenum_t = 585;
pub const sfx_cdfm06: sfxenum_t = 584;
pub const sfx_cdfm05: sfxenum_t = 583;
pub const sfx_cdfm04: sfxenum_t = 582;
pub const sfx_cdfm03: sfxenum_t = 581;
pub const sfx_cdfm02: sfxenum_t = 580;
pub const sfx_cdfm01: sfxenum_t = 579;
pub const sfx_cdfm00: sfxenum_t = 578;
pub const sfx_3db16: sfxenum_t = 577;
pub const sfx_3db14: sfxenum_t = 576;
pub const sfx_3db09: sfxenum_t = 575;
pub const sfx_3db06: sfxenum_t = 574;
pub const sfx_s3kdbl: sfxenum_t = 573;
pub const sfx_s3kdbs: sfxenum_t = 572;
pub const sfx_s3kdal: sfxenum_t = 571;
pub const sfx_s3kdas: sfxenum_t = 570;
pub const sfx_s3kd9l: sfxenum_t = 569;
pub const sfx_s3kd9s: sfxenum_t = 568;
pub const sfx_s3kd8l: sfxenum_t = 567;
pub const sfx_s3kd8s: sfxenum_t = 566;
pub const sfx_s3kd7l: sfxenum_t = 565;
pub const sfx_s3kd7s: sfxenum_t = 564;
pub const sfx_s3kd6l: sfxenum_t = 563;
pub const sfx_s3kd6s: sfxenum_t = 562;
pub const sfx_s3kd5l: sfxenum_t = 561;
pub const sfx_s3kd5s: sfxenum_t = 560;
pub const sfx_s3kd4l: sfxenum_t = 559;
pub const sfx_s3kd4s: sfxenum_t = 558;
pub const sfx_s3kd3l: sfxenum_t = 557;
pub const sfx_s3kd3s: sfxenum_t = 556;
pub const sfx_s3kd2l: sfxenum_t = 555;
pub const sfx_s3kd2s: sfxenum_t = 554;
pub const sfx_s3kd1l: sfxenum_t = 553;
pub const sfx_s3kd1s: sfxenum_t = 552;
pub const sfx_s3kd0l: sfxenum_t = 551;
pub const sfx_s3kd0s: sfxenum_t = 550;
pub const sfx_s3kcfl: sfxenum_t = 549;
pub const sfx_s3kcfs: sfxenum_t = 548;
pub const sfx_s3kcel: sfxenum_t = 547;
pub const sfx_s3kces: sfxenum_t = 546;
pub const sfx_s3kcdl: sfxenum_t = 545;
pub const sfx_s3kcds: sfxenum_t = 544;
pub const sfx_s3kccl: sfxenum_t = 543;
pub const sfx_s3kccs: sfxenum_t = 542;
pub const sfx_s3kcbl: sfxenum_t = 541;
pub const sfx_s3kcbs: sfxenum_t = 540;
pub const sfx_s3kcal: sfxenum_t = 539;
pub const sfx_s3kcas: sfxenum_t = 538;
pub const sfx_s3kc9l: sfxenum_t = 537;
pub const sfx_s3kc9s: sfxenum_t = 536;
pub const sfx_s3kc8l: sfxenum_t = 535;
pub const sfx_s3kc8s: sfxenum_t = 534;
pub const sfx_s3kc7l: sfxenum_t = 533;
pub const sfx_s3kc7s: sfxenum_t = 532;
pub const sfx_s3kc6l: sfxenum_t = 531;
pub const sfx_s3kc6s: sfxenum_t = 530;
pub const sfx_s3kc5l: sfxenum_t = 529;
pub const sfx_s3kc5s: sfxenum_t = 528;
pub const sfx_s3kc4l: sfxenum_t = 527;
pub const sfx_s3kc4s: sfxenum_t = 526;
pub const sfx_s3kc3l: sfxenum_t = 525;
pub const sfx_s3kc3s: sfxenum_t = 524;
pub const sfx_s3kc2l: sfxenum_t = 523;
pub const sfx_s3kc2s: sfxenum_t = 522;
pub const sfx_s3kc1l: sfxenum_t = 521;
pub const sfx_s3kc1s: sfxenum_t = 520;
pub const sfx_s3kc0l: sfxenum_t = 519;
pub const sfx_s3kc0s: sfxenum_t = 518;
pub const sfx_s3kbfl: sfxenum_t = 517;
pub const sfx_s3kbfs: sfxenum_t = 516;
pub const sfx_s3kbel: sfxenum_t = 515;
pub const sfx_s3kbes: sfxenum_t = 514;
pub const sfx_s3kbdl: sfxenum_t = 513;
pub const sfx_s3kbds: sfxenum_t = 512;
pub const sfx_s3kbcl: sfxenum_t = 511;
pub const sfx_s3kbcs: sfxenum_t = 510;
pub const sfx_s3kbb: sfxenum_t = 509;
pub const sfx_s3kba: sfxenum_t = 508;
pub const sfx_s3kb9: sfxenum_t = 507;
pub const sfx_s3kb8: sfxenum_t = 506;
pub const sfx_s3kb7: sfxenum_t = 505;
pub const sfx_s3kb6: sfxenum_t = 504;
pub const sfx_s3kb5: sfxenum_t = 503;
pub const sfx_s3kb4: sfxenum_t = 502;
pub const sfx_s3kb3: sfxenum_t = 501;
pub const sfx_s3kb2: sfxenum_t = 500;
pub const sfx_s3kb1: sfxenum_t = 499;
pub const sfx_s3kb0: sfxenum_t = 498;
pub const sfx_s3kaf: sfxenum_t = 497;
pub const sfx_s3kae: sfxenum_t = 496;
pub const sfx_s3kad: sfxenum_t = 495;
pub const sfx_s3kac: sfxenum_t = 494;
pub const sfx_s3kabf: sfxenum_t = 493;
pub const sfx_s3kabe: sfxenum_t = 492;
pub const sfx_s3kabd: sfxenum_t = 491;
pub const sfx_s3kabc: sfxenum_t = 490;
pub const sfx_s3kabb: sfxenum_t = 489;
pub const sfx_s3kaba: sfxenum_t = 488;
pub const sfx_s3kab9: sfxenum_t = 487;
pub const sfx_s3kab8: sfxenum_t = 486;
pub const sfx_s3kab7: sfxenum_t = 485;
pub const sfx_s3kab6: sfxenum_t = 484;
pub const sfx_s3kab5: sfxenum_t = 483;
pub const sfx_s3kab4: sfxenum_t = 482;
pub const sfx_s3kab3: sfxenum_t = 481;
pub const sfx_s3kab2: sfxenum_t = 480;
pub const sfx_s3kab1: sfxenum_t = 479;
pub const sfx_s3kab: sfxenum_t = 478;
pub const sfx_s3kaa: sfxenum_t = 477;
pub const sfx_s3ka9: sfxenum_t = 476;
pub const sfx_s3ka8: sfxenum_t = 475;
pub const sfx_s3ka7: sfxenum_t = 474;
pub const sfx_s3ka6: sfxenum_t = 473;
pub const sfx_s3ka5: sfxenum_t = 472;
pub const sfx_s3ka4: sfxenum_t = 471;
pub const sfx_s3ka3: sfxenum_t = 470;
pub const sfx_s3ka2: sfxenum_t = 469;
pub const sfx_s3ka1: sfxenum_t = 468;
pub const sfx_s3ka0: sfxenum_t = 467;
pub const sfx_s3k9f: sfxenum_t = 466;
pub const sfx_s3k9e: sfxenum_t = 465;
pub const sfx_s3k9d: sfxenum_t = 464;
pub const sfx_s3k9c: sfxenum_t = 463;
pub const sfx_s3k9b: sfxenum_t = 462;
pub const sfx_s3k9a: sfxenum_t = 461;
pub const sfx_s3k99: sfxenum_t = 460;
pub const sfx_s3k98: sfxenum_t = 459;
pub const sfx_s3k97: sfxenum_t = 458;
pub const sfx_s3k96: sfxenum_t = 457;
pub const sfx_s3k95: sfxenum_t = 456;
pub const sfx_s3k94: sfxenum_t = 455;
pub const sfx_s3k93: sfxenum_t = 454;
pub const sfx_s3k92: sfxenum_t = 453;
pub const sfx_s3k91: sfxenum_t = 452;
pub const sfx_s3k90: sfxenum_t = 451;
pub const sfx_s3k8f: sfxenum_t = 450;
pub const sfx_s3k8e: sfxenum_t = 449;
pub const sfx_s3k8d: sfxenum_t = 448;
pub const sfx_s3k8c: sfxenum_t = 447;
pub const sfx_s3k8b: sfxenum_t = 446;
pub const sfx_s3k8a: sfxenum_t = 445;
pub const sfx_s3k89: sfxenum_t = 444;
pub const sfx_s3k88: sfxenum_t = 443;
pub const sfx_s3k87: sfxenum_t = 442;
pub const sfx_s3k86: sfxenum_t = 441;
pub const sfx_s3k85: sfxenum_t = 440;
pub const sfx_s3k84: sfxenum_t = 439;
pub const sfx_s3k83: sfxenum_t = 438;
pub const sfx_s3k82: sfxenum_t = 437;
pub const sfx_s3k81: sfxenum_t = 436;
pub const sfx_s3k80: sfxenum_t = 435;
pub const sfx_s3k7f: sfxenum_t = 434;
pub const sfx_s3k7e: sfxenum_t = 433;
pub const sfx_s3k7d: sfxenum_t = 432;
pub const sfx_s3k7c: sfxenum_t = 431;
pub const sfx_s3k7b: sfxenum_t = 430;
pub const sfx_s3k7a: sfxenum_t = 429;
pub const sfx_s3k79: sfxenum_t = 428;
pub const sfx_s3k78: sfxenum_t = 427;
pub const sfx_s3k77: sfxenum_t = 426;
pub const sfx_s3k76: sfxenum_t = 425;
pub const sfx_s3k75: sfxenum_t = 424;
pub const sfx_s3k74: sfxenum_t = 423;
pub const sfx_s3k73: sfxenum_t = 422;
pub const sfx_s3k72: sfxenum_t = 421;
pub const sfx_s3k71: sfxenum_t = 420;
pub const sfx_s3k70: sfxenum_t = 419;
pub const sfx_s3k6f: sfxenum_t = 418;
pub const sfx_s3k6e: sfxenum_t = 417;
pub const sfx_s3k6d: sfxenum_t = 416;
pub const sfx_s3k6c: sfxenum_t = 415;
pub const sfx_s3k6b: sfxenum_t = 414;
pub const sfx_s3k6a: sfxenum_t = 413;
pub const sfx_s3k69: sfxenum_t = 412;
pub const sfx_s3k68: sfxenum_t = 411;
pub const sfx_s3k67: sfxenum_t = 410;
pub const sfx_s3k66: sfxenum_t = 409;
pub const sfx_s3k65: sfxenum_t = 408;
pub const sfx_s3k64: sfxenum_t = 407;
pub const sfx_s3k63: sfxenum_t = 406;
pub const sfx_s3k62: sfxenum_t = 405;
pub const sfx_s3k61: sfxenum_t = 404;
pub const sfx_s3k60: sfxenum_t = 403;
pub const sfx_s3k5f: sfxenum_t = 402;
pub const sfx_s3k5e: sfxenum_t = 401;
pub const sfx_s3k5d: sfxenum_t = 400;
pub const sfx_s3k5c: sfxenum_t = 399;
pub const sfx_s3k5b: sfxenum_t = 398;
pub const sfx_s3k5a: sfxenum_t = 397;
pub const sfx_s3k59: sfxenum_t = 396;
pub const sfx_s3k58: sfxenum_t = 395;
pub const sfx_s3k57: sfxenum_t = 394;
pub const sfx_s3k56: sfxenum_t = 393;
pub const sfx_s3k55: sfxenum_t = 392;
pub const sfx_s3k54: sfxenum_t = 391;
pub const sfx_s3k53: sfxenum_t = 390;
pub const sfx_s3k52: sfxenum_t = 389;
pub const sfx_s3k51: sfxenum_t = 388;
pub const sfx_s3k50: sfxenum_t = 387;
pub const sfx_s3k4f: sfxenum_t = 386;
pub const sfx_s3k4e: sfxenum_t = 385;
pub const sfx_s3k4d: sfxenum_t = 384;
pub const sfx_s3k4c: sfxenum_t = 383;
pub const sfx_s3k4b: sfxenum_t = 382;
pub const sfx_s3k4a: sfxenum_t = 381;
pub const sfx_s3k49: sfxenum_t = 380;
pub const sfx_s3k48: sfxenum_t = 379;
pub const sfx_s3k47: sfxenum_t = 378;
pub const sfx_s3k46: sfxenum_t = 377;
pub const sfx_s3k45: sfxenum_t = 376;
pub const sfx_s3k44: sfxenum_t = 375;
pub const sfx_s3k43: sfxenum_t = 374;
pub const sfx_s3k42: sfxenum_t = 373;
pub const sfx_s3k41: sfxenum_t = 372;
pub const sfx_s3k40: sfxenum_t = 371;
pub const sfx_s3k3f: sfxenum_t = 370;
pub const sfx_s3k3e: sfxenum_t = 369;
pub const sfx_s3k3d: sfxenum_t = 368;
pub const sfx_s3k3c: sfxenum_t = 367;
pub const sfx_s3k3b: sfxenum_t = 366;
pub const sfx_s3k3a: sfxenum_t = 365;
pub const sfx_s3k39: sfxenum_t = 364;
pub const sfx_s3k38: sfxenum_t = 363;
pub const sfx_s3k37: sfxenum_t = 362;
pub const sfx_s3k36: sfxenum_t = 361;
pub const sfx_s3k35: sfxenum_t = 360;
pub const sfx_s3k34: sfxenum_t = 359;
pub const sfx_s3k33: sfxenum_t = 358;
pub const sfx_s3k2b: sfxenum_t = 357;
pub const sfx_s260: sfxenum_t = 356;
pub const sfx_s25f: sfxenum_t = 355;
pub const sfx_s25e: sfxenum_t = 354;
pub const sfx_s25d: sfxenum_t = 353;
pub const sfx_s25c: sfxenum_t = 352;
pub const sfx_s25b: sfxenum_t = 351;
pub const sfx_s25a: sfxenum_t = 350;
pub const sfx_s259: sfxenum_t = 349;
pub const sfx_s258: sfxenum_t = 348;
pub const sfx_s257: sfxenum_t = 347;
pub const sfx_s256: sfxenum_t = 346;
pub const sfx_s255: sfxenum_t = 345;
pub const sfx_s254: sfxenum_t = 344;
pub const sfx_s253: sfxenum_t = 343;
pub const sfx_s252: sfxenum_t = 342;
pub const sfx_s251: sfxenum_t = 341;
pub const sfx_s250: sfxenum_t = 340;
pub const sfx_s24f: sfxenum_t = 339;
pub const sfx_s24e: sfxenum_t = 338;
pub const sfx_s24d: sfxenum_t = 337;
pub const sfx_s24c: sfxenum_t = 336;
pub const sfx_s24b: sfxenum_t = 335;
pub const sfx_s24a: sfxenum_t = 334;
pub const sfx_s249: sfxenum_t = 333;
pub const sfx_s248: sfxenum_t = 332;
pub const sfx_s247: sfxenum_t = 331;
pub const sfx_s246: sfxenum_t = 330;
pub const sfx_s245: sfxenum_t = 329;
pub const sfx_s244: sfxenum_t = 328;
pub const sfx_s243: sfxenum_t = 327;
pub const sfx_s242: sfxenum_t = 326;
pub const sfx_s241: sfxenum_t = 325;
pub const sfx_s240: sfxenum_t = 324;
pub const sfx_s23f: sfxenum_t = 323;
pub const sfx_s23e: sfxenum_t = 322;
pub const sfx_s23d: sfxenum_t = 321;
pub const sfx_s23c: sfxenum_t = 320;
pub const sfx_s23b: sfxenum_t = 319;
pub const sfx_s23a: sfxenum_t = 318;
pub const sfx_s239: sfxenum_t = 317;
pub const sfx_s238: sfxenum_t = 316;
pub const sfx_s237: sfxenum_t = 315;
pub const sfx_s236: sfxenum_t = 314;
pub const sfx_s235: sfxenum_t = 313;
pub const sfx_s234: sfxenum_t = 312;
pub const sfx_s233: sfxenum_t = 311;
pub const sfx_s232: sfxenum_t = 310;
pub const sfx_s231: sfxenum_t = 309;
pub const sfx_s230: sfxenum_t = 308;
pub const sfx_s22f: sfxenum_t = 307;
pub const sfx_s22e: sfxenum_t = 306;
pub const sfx_s22d: sfxenum_t = 305;
pub const sfx_s22c: sfxenum_t = 304;
pub const sfx_s22b: sfxenum_t = 303;
pub const sfx_s22a: sfxenum_t = 302;
pub const sfx_s229: sfxenum_t = 301;
pub const sfx_s228: sfxenum_t = 300;
pub const sfx_s227: sfxenum_t = 299;
pub const sfx_s226: sfxenum_t = 298;
pub const sfx_s225: sfxenum_t = 297;
pub const sfx_s224: sfxenum_t = 296;
pub const sfx_s223: sfxenum_t = 295;
pub const sfx_s222: sfxenum_t = 294;
pub const sfx_s221: sfxenum_t = 293;
pub const sfx_s220: sfxenum_t = 292;
pub const sfx_s1cf: sfxenum_t = 291;
pub const sfx_s1ce: sfxenum_t = 290;
pub const sfx_s1cd: sfxenum_t = 289;
pub const sfx_s1cc: sfxenum_t = 288;
pub const sfx_s1cb: sfxenum_t = 287;
pub const sfx_s1ca: sfxenum_t = 286;
pub const sfx_s1c9: sfxenum_t = 285;
pub const sfx_s1c8: sfxenum_t = 284;
pub const sfx_s1c7: sfxenum_t = 283;
pub const sfx_s1c6: sfxenum_t = 282;
pub const sfx_s1c5: sfxenum_t = 281;
pub const sfx_s1c4: sfxenum_t = 280;
pub const sfx_s1c3: sfxenum_t = 279;
pub const sfx_s1c2: sfxenum_t = 278;
pub const sfx_s1c1: sfxenum_t = 277;
pub const sfx_s1c0: sfxenum_t = 276;
pub const sfx_s1bf: sfxenum_t = 275;
pub const sfx_s1be: sfxenum_t = 274;
pub const sfx_s1bd: sfxenum_t = 273;
pub const sfx_s1bc: sfxenum_t = 272;
pub const sfx_s1bb: sfxenum_t = 271;
pub const sfx_s1ba: sfxenum_t = 270;
pub const sfx_s1b9: sfxenum_t = 269;
pub const sfx_s1b8: sfxenum_t = 268;
pub const sfx_s1b7: sfxenum_t = 267;
pub const sfx_s1b6: sfxenum_t = 266;
pub const sfx_s1b5: sfxenum_t = 265;
pub const sfx_s1b4: sfxenum_t = 264;
pub const sfx_s1b3: sfxenum_t = 263;
pub const sfx_s1b2: sfxenum_t = 262;
pub const sfx_s1b1: sfxenum_t = 261;
pub const sfx_s1b0: sfxenum_t = 260;
pub const sfx_s1af: sfxenum_t = 259;
pub const sfx_s1ae: sfxenum_t = 258;
pub const sfx_s1ad: sfxenum_t = 257;
pub const sfx_s1ac: sfxenum_t = 256;
pub const sfx_s1ab: sfxenum_t = 255;
pub const sfx_s1aa: sfxenum_t = 254;
pub const sfx_s1a9: sfxenum_t = 253;
pub const sfx_s1a8: sfxenum_t = 252;
pub const sfx_s1a7: sfxenum_t = 251;
pub const sfx_s1a6: sfxenum_t = 250;
pub const sfx_s1a5: sfxenum_t = 249;
pub const sfx_s1a4: sfxenum_t = 248;
pub const sfx_s1a3: sfxenum_t = 247;
pub const sfx_s1a2: sfxenum_t = 246;
pub const sfx_s1a1: sfxenum_t = 245;
pub const sfx_s1a0: sfxenum_t = 244;
pub const sfx_brakrx: sfxenum_t = 243;
pub const sfx_brakrl: sfxenum_t = 242;
pub const sfx_beelec: sfxenum_t = 241;
pub const sfx_bgxpld: sfxenum_t = 240;
pub const sfx_bexpld: sfxenum_t = 239;
pub const sfx_bewar4: sfxenum_t = 238;
pub const sfx_bewar3: sfxenum_t = 237;
pub const sfx_bewar2: sfxenum_t = 236;
pub const sfx_bewar1: sfxenum_t = 235;
pub const sfx_bestp2: sfxenum_t = 234;
pub const sfx_bestep: sfxenum_t = 233;
pub const sfx_beshot: sfxenum_t = 232;
pub const sfx_beragh: sfxenum_t = 231;
pub const sfx_beoutb: sfxenum_t = 230;
pub const sfx_belnch: sfxenum_t = 229;
pub const sfx_bejet1: sfxenum_t = 228;
pub const sfx_behurt: sfxenum_t = 227;
pub const sfx_begrnd: sfxenum_t = 226;
pub const sfx_begoop: sfxenum_t = 225;
pub const sfx_beflap: sfxenum_t = 224;
pub const sfx_befire: sfxenum_t = 223;
pub const sfx_befall: sfxenum_t = 222;
pub const sfx_beeyow: sfxenum_t = 221;
pub const sfx_bedie2: sfxenum_t = 220;
pub const sfx_bedie1: sfxenum_t = 219;
pub const sfx_bedeen: sfxenum_t = 218;
pub const sfx_becrsh: sfxenum_t = 217;
pub const sfx_bechrg: sfxenum_t = 216;
pub const sfx_bebomb: sfxenum_t = 215;
pub const sfx_thwomp: sfxenum_t = 214;
pub const sfx_marioa: sfxenum_t = 213;
pub const sfx_mario9: sfxenum_t = 212;
pub const sfx_mario8: sfxenum_t = 211;
pub const sfx_mario7: sfxenum_t = 210;
pub const sfx_mario6: sfxenum_t = 209;
pub const sfx_mario5: sfxenum_t = 208;
pub const sfx_mario4: sfxenum_t = 207;
pub const sfx_mario3: sfxenum_t = 206;
pub const sfx_mario2: sfxenum_t = 205;
pub const sfx_mario1: sfxenum_t = 204;
pub const sfx_koopfr: sfxenum_t = 203;
pub const sfx_ghosty: sfxenum_t = 202;
pub const sfx_pumpkn: sfxenum_t = 201;
pub const sfx_lntdie: sfxenum_t = 200;
pub const sfx_lntsit: sfxenum_t = 199;
pub const sfx_peww: sfxenum_t = 198;
pub const sfx_ngjump: sfxenum_t = 197;
pub const sfx_timeup: sfxenum_t = 196;
pub const sfx_prloop: sfxenum_t = 195;
pub const sfx_hidden: sfxenum_t = 194;
pub const sfx_hoop3: sfxenum_t = 193;
pub const sfx_hoop2: sfxenum_t = 192;
pub const sfx_hoop1: sfxenum_t = 191;
pub const sfx_ngskid: sfxenum_t = 190;
pub const sfx_nghurt: sfxenum_t = 189;
pub const sfx_ncspec: sfxenum_t = 188;
pub const sfx_drill2: sfxenum_t = 187;
pub const sfx_drill1: sfxenum_t = 186;
pub const sfx_nxdone: sfxenum_t = 185;
pub const sfx_ngdone: sfxenum_t = 184;
pub const sfx_nxitem: sfxenum_t = 183;
pub const sfx_ncitem: sfxenum_t = 182;
pub const sfx_ncchip: sfxenum_t = 181;
pub const sfx_nxbump: sfxenum_t = 180;
pub const sfx_nbmper: sfxenum_t = 179;
pub const sfx_xideya: sfxenum_t = 178;
pub const sfx_ideya: sfxenum_t = 177;
pub const sfx_addfil: sfxenum_t = 176;
pub const sfx_notadd: sfxenum_t = 175;
pub const sfx_adderr: sfxenum_t = 174;
pub const sfx_zelda: sfxenum_t = 173;
pub const sfx_wtrdng: sfxenum_t = 172;
pub const sfx_wepchg: sfxenum_t = 171;
pub const sfx_radio: sfxenum_t = 170;
pub const sfx_ptally: sfxenum_t = 169;
pub const sfx_oneup: sfxenum_t = 168;
pub const sfx_menu1: sfxenum_t = 167;
pub const sfx_flgcap: sfxenum_t = 166;
pub const sfx_emfind: sfxenum_t = 165;
pub const sfx_dwnind: sfxenum_t = 164;
pub const sfx_chchng: sfxenum_t = 163;
pub const sfx_antiri: sfxenum_t = 162;
pub const sfx_shattr: sfxenum_t = 161;
pub const sfx_iceb: sfxenum_t = 160;
pub const sfx_pscree: sfxenum_t = 159;
pub const sfx_lvfal1: sfxenum_t = 158;
pub const sfx_sprong: sfxenum_t = 157;
pub const sfx_bsnipe: sfxenum_t = 156;
pub const sfx_chuchu: sfxenum_t = 155;
pub const sfx_bowl: sfxenum_t = 154;
pub const sfx_vwre: sfxenum_t = 153;
pub const sfx_alart: sfxenum_t = 152;
pub const sfx_corkh: sfxenum_t = 151;
pub const sfx_corkp: sfxenum_t = 150;
pub const sfx_boingf: sfxenum_t = 149;
pub const sfx_mspogo: sfxenum_t = 148;
pub const sfx_mswarp: sfxenum_t = 147;
pub const sfx_shrpgo: sfxenum_t = 146;
pub const sfx_shrpsp: sfxenum_t = 145;
pub const sfx_wdjump: sfxenum_t = 144;
pub const sfx_turhit: sfxenum_t = 143;
pub const sfx_trpowr: sfxenum_t = 142;
pub const sfx_trfire: sfxenum_t = 141;
pub const sfx_token: sfxenum_t = 140;
pub const sfx_tink: sfxenum_t = 139;
pub const sfx_telept: sfxenum_t = 138;
pub const sfx_supert: sfxenum_t = 137;
pub const sfx_strpst: sfxenum_t = 136;
pub const sfx_statu2: sfxenum_t = 135;
pub const sfx_statu1: sfxenum_t = 134;
pub const sfx_spring: sfxenum_t = 133;
pub const sfx_spkdth: sfxenum_t = 132;
pub const sfx_spdpad: sfxenum_t = 131;
pub const sfx_shldls: sfxenum_t = 130;
pub const sfx_attrsg: sfxenum_t = 129;
pub const sfx_armasg: sfxenum_t = 128;
pub const sfx_elemsg: sfxenum_t = 127;
pub const sfx_frcssg: sfxenum_t = 126;
pub const sfx_forcsg: sfxenum_t = 125;
pub const sfx_wirlsg: sfxenum_t = 124;
pub const sfx_shield: sfxenum_t = 123;
pub const sfx_rlaunc: sfxenum_t = 122;
pub const sfx_rail2: sfxenum_t = 121;
pub const sfx_rail1: sfxenum_t = 120;
pub const sfx_pop: sfxenum_t = 119;
pub const sfx_pogo: sfxenum_t = 118;
pub const sfx_monton: sfxenum_t = 117;
pub const sfx_mixup: sfxenum_t = 116;
pub const sfx_mindig: sfxenum_t = 115;
pub const sfx_lvpass: sfxenum_t = 114;
pub const sfx_lose: sfxenum_t = 113;
pub const sfx_jshard: sfxenum_t = 112;
pub const sfx_jet: sfxenum_t = 111;
pub const sfx_itemup: sfxenum_t = 110;
pub const sfx_gravch: sfxenum_t = 109;
pub const sfx_gspray: sfxenum_t = 108;
pub const sfx_gloop: sfxenum_t = 107;
pub const sfx_ghit: sfxenum_t = 106;
pub const sfx_wepfir: sfxenum_t = 105;
pub const sfx_gbeep: sfxenum_t = 104;
pub const sfx_fizzle: sfxenum_t = 103;
pub const sfx_drown: sfxenum_t = 102;
pub const sfx_dmpain: sfxenum_t = 101;
pub const sfx_ding: sfxenum_t = 100;
pub const sfx_deton: sfxenum_t = 99;
pub const sfx_cybdth: sfxenum_t = 98;
pub const sfx_cgot: sfxenum_t = 97;
pub const sfx_cannon: sfxenum_t = 96;
pub const sfx_bnce2: sfxenum_t = 95;
pub const sfx_bnce1: sfxenum_t = 94;
pub const sfx_bkpoof: sfxenum_t = 93;
pub const sfx_appear: sfxenum_t = 92;
pub const sfx_rumble: sfxenum_t = 91;
pub const sfx_rumbam: sfxenum_t = 90;
pub const sfx_rocks4: sfxenum_t = 89;
pub const sfx_rocks3: sfxenum_t = 88;
pub const sfx_rocks2: sfxenum_t = 87;
pub const sfx_rocks1: sfxenum_t = 86;
pub const sfx_lavbub: sfxenum_t = 85;
pub const sfx_ambin2: sfxenum_t = 84;
pub const sfx_elevb3: sfxenum_t = 83;
pub const sfx_elevb2: sfxenum_t = 82;
pub const sfx_elevb1: sfxenum_t = 81;
pub const sfx_eleva3: sfxenum_t = 80;
pub const sfx_eleva2: sfxenum_t = 79;
pub const sfx_eleva1: sfxenum_t = 78;
pub const sfx_doord2: sfxenum_t = 77;
pub const sfx_doord1: sfxenum_t = 76;
pub const sfx_doorc2: sfxenum_t = 75;
pub const sfx_doorc1: sfxenum_t = 74;
pub const sfx_doorb1: sfxenum_t = 73;
pub const sfx_doora1: sfxenum_t = 72;
pub const sfx_wslap: sfxenum_t = 71;
pub const sfx_wdrip8: sfxenum_t = 70;
pub const sfx_wdrip7: sfxenum_t = 69;
pub const sfx_wdrip6: sfxenum_t = 68;
pub const sfx_wdrip5: sfxenum_t = 67;
pub const sfx_wdrip4: sfxenum_t = 66;
pub const sfx_wdrip3: sfxenum_t = 65;
pub const sfx_wdrip2: sfxenum_t = 64;
pub const sfx_wdrip1: sfxenum_t = 63;
pub const sfx_splish: sfxenum_t = 62;
pub const sfx_splash: sfxenum_t = 61;
pub const sfx_floush: sfxenum_t = 60;
pub const sfx_bubbl5: sfxenum_t = 59;
pub const sfx_bubbl4: sfxenum_t = 58;
pub const sfx_bubbl3: sfxenum_t = 57;
pub const sfx_bubbl2: sfxenum_t = 56;
pub const sfx_bubbl1: sfxenum_t = 55;
pub const sfx_amwtr8: sfxenum_t = 54;
pub const sfx_amwtr7: sfxenum_t = 53;
pub const sfx_amwtr6: sfxenum_t = 52;
pub const sfx_amwtr5: sfxenum_t = 51;
pub const sfx_amwtr4: sfxenum_t = 50;
pub const sfx_amwtr3: sfxenum_t = 49;
pub const sfx_amwtr2: sfxenum_t = 48;
pub const sfx_amwtr1: sfxenum_t = 47;
pub const sfx_athun2: sfxenum_t = 46;
pub const sfx_athun1: sfxenum_t = 45;
pub const sfx_litng4: sfxenum_t = 44;
pub const sfx_litng3: sfxenum_t = 43;
pub const sfx_litng2: sfxenum_t = 42;
pub const sfx_litng1: sfxenum_t = 41;
pub const sfx_rainin: sfxenum_t = 40;
pub const sfx_spsmsh: sfxenum_t = 39;
pub const sfx_ambmac: sfxenum_t = 38;
pub const sfx_wbreak: sfxenum_t = 37;
pub const sfx_steam2: sfxenum_t = 36;
pub const sfx_steam1: sfxenum_t = 35;
pub const sfx_pstop: sfxenum_t = 34;
pub const sfx_pstart: sfxenum_t = 33;
pub const sfx_mswing: sfxenum_t = 32;
pub const sfx_laser: sfxenum_t = 31;
pub const sfx_grind: sfxenum_t = 30;
pub const sfx_fire: sfxenum_t = 29;
pub const sfx_crumbl: sfxenum_t = 28;
pub const sfx_buzz4: sfxenum_t = 27;
pub const sfx_buzz3: sfxenum_t = 26;
pub const sfx_buzz2: sfxenum_t = 25;
pub const sfx_buzz1: sfxenum_t = 24;
pub const sfx_alarm: sfxenum_t = 23;
pub const sfx_ambint: sfxenum_t = 22;
pub const sfx_skid: sfxenum_t = 21;
pub const sfx_zoom: sfxenum_t = 20;
pub const sfx_thok: sfxenum_t = 19;
pub const sfx_spndsh: sfxenum_t = 18;
pub const sfx_spin: sfxenum_t = 17;
pub const sfx_putput: sfxenum_t = 16;
pub const sfx_pudpud: sfxenum_t = 15;
pub const sfx_jump: sfxenum_t = 14;
pub const sfx_gasp: sfxenum_t = 13;
pub const sfx_victr4: sfxenum_t = 12;
pub const sfx_victr3: sfxenum_t = 11;
pub const sfx_victr2: sfxenum_t = 10;
pub const sfx_victr1: sfxenum_t = 9;
pub const sfx_altow4: sfxenum_t = 8;
pub const sfx_altow3: sfxenum_t = 7;
pub const sfx_altow2: sfxenum_t = 6;
pub const sfx_altow1: sfxenum_t = 5;
pub const sfx_altdi4: sfxenum_t = 4;
pub const sfx_altdi3: sfxenum_t = 3;
pub const sfx_altdi2: sfxenum_t = 2;
pub const sfx_altdi1: sfxenum_t = 1;
pub const sfx_None: sfxenum_t = 0;
pub const PU_STATIC: C2RustUnnamed_6 = 1;
pub const CV_CALL: C2RustUnnamed_3 = 2;
pub const REFRESHDIR_ERROR: C2RustUnnamed_7 = 8;
pub const REFRESHDIR_WARNING: C2RustUnnamed_7 = 4;
pub type evtype_t = libc::c_uint;
pub const ev_joystick2: evtype_t = 6;
pub const ev_mouse2: evtype_t = 5;
pub const ev_joystick: evtype_t = 4;
pub const ev_mouse: evtype_t = 3;
pub const ev_console: evtype_t = 2;
pub const ev_keyup: evtype_t = 1;
pub const ev_keydown: evtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct event_t {
    pub type_0: evtype_t,
    pub key: int32_t,
    pub x: int32_t,
    pub y: int32_t,
    pub repeated: boolean,
}
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
pub type com_func_t = Option::<unsafe extern "C" fn() -> ()>;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CV_ALLOWLUA: C2RustUnnamed_3 = 4096;
pub const CV_CHEAT: C2RustUnnamed_3 = 2048;
pub const CV_HIDEN: C2RustUnnamed_3 = 1024;
pub const CV_NOSHOWHELP: C2RustUnnamed_3 = 512;
pub const CV_SHOWMODIFONETIME: C2RustUnnamed_3 = 256;
pub const CV_SHOWMODIF: C2RustUnnamed_3 = 128;
pub const CV_MODIFIED: C2RustUnnamed_3 = 64;
pub const CV_NOTINNET: C2RustUnnamed_3 = 32;
pub const CV_FLOAT: C2RustUnnamed_3 = 16;
pub const CV_NOINIT: C2RustUnnamed_3 = 8;
pub const CV_NETVAR: C2RustUnnamed_3 = 4;
pub const NUMINPUTS: C2RustUnnamed_4 = 484;
pub const PU_CACHE: C2RustUnnamed_6 = 49;
pub const GC_CONSOLE: C2RustUnnamed_5 = 33;
pub type marathonmode_t = libc::c_uint;
pub const MA_INGAME: marathonmode_t = 8;
pub const MA_NOCUTSCENES: marathonmode_t = 4;
pub const MA_INIT: marathonmode_t = 2;
pub const MA_RUNNING: marathonmode_t = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const KEY_2MOUSEWHEELDOWN: C2RustUnnamed_4 = 483;
pub const KEY_2MOUSEWHEELUP: C2RustUnnamed_4 = 482;
pub const KEY_MOUSEWHEELDOWN: C2RustUnnamed_4 = 481;
pub const KEY_MOUSEWHEELUP: C2RustUnnamed_4 = 480;
pub const KEY_DBL2HAT1: C2RustUnnamed_4 = 464;
pub const KEY_DBL2JOY1: C2RustUnnamed_4 = 432;
pub const KEY_DBL2MOUSE1: C2RustUnnamed_4 = 424;
pub const KEY_2HAT1: C2RustUnnamed_4 = 408;
pub const KEY_2JOY1: C2RustUnnamed_4 = 376;
pub const KEY_2MOUSE1: C2RustUnnamed_4 = 368;
pub const KEY_DBLHAT1: C2RustUnnamed_4 = 352;
pub const KEY_DBLJOY1: C2RustUnnamed_4 = 320;
pub const KEY_DBLMOUSE1: C2RustUnnamed_4 = 312;
pub const KEY_HAT1: C2RustUnnamed_4 = 296;
pub const KEY_JOY1: C2RustUnnamed_4 = 264;
pub const KEY_MOUSE1: C2RustUnnamed_4 = 256;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const NUM_GAMECONTROLS: C2RustUnnamed_5 = 43;
pub const GC_CUSTOM3: C2RustUnnamed_5 = 42;
pub const GC_CUSTOM2: C2RustUnnamed_5 = 41;
pub const GC_CUSTOM1: C2RustUnnamed_5 = 40;
pub const GC_VIEWPOINTPREV: C2RustUnnamed_5 = 39;
pub const GC_VIEWPOINTNEXT: C2RustUnnamed_5 = 38;
pub const GC_RECORDGIF: C2RustUnnamed_5 = 37;
pub const GC_SCREENSHOT: C2RustUnnamed_5 = 36;
pub const GC_SYSTEMMENU: C2RustUnnamed_5 = 35;
pub const GC_PAUSE: C2RustUnnamed_5 = 34;
pub const GC_JUMP: C2RustUnnamed_5 = 32;
pub const GC_SCORES: C2RustUnnamed_5 = 31;
pub const GC_TEAMKEY: C2RustUnnamed_5 = 30;
pub const GC_TALKKEY: C2RustUnnamed_5 = 29;
pub const GC_MOUSEAIMING: C2RustUnnamed_5 = 28;
pub const GC_CENTERVIEW: C2RustUnnamed_5 = 27;
pub const GC_LOOKDOWN: C2RustUnnamed_5 = 26;
pub const GC_LOOKUP: C2RustUnnamed_5 = 25;
pub const GC_CAMRESET: C2RustUnnamed_5 = 24;
pub const GC_CAMTOGGLE: C2RustUnnamed_5 = 23;
pub const GC_SPIN: C2RustUnnamed_5 = 22;
pub const GC_TOSSFLAG: C2RustUnnamed_5 = 21;
pub const GC_FIRENORMAL: C2RustUnnamed_5 = 20;
pub const GC_FIRE: C2RustUnnamed_5 = 19;
pub const GC_WEPSLOT10: C2RustUnnamed_5 = 18;
pub const GC_WEPSLOT9: C2RustUnnamed_5 = 17;
pub const GC_WEPSLOT8: C2RustUnnamed_5 = 16;
pub const GC_WEPSLOT7: C2RustUnnamed_5 = 15;
pub const GC_WEPSLOT6: C2RustUnnamed_5 = 14;
pub const GC_WEPSLOT5: C2RustUnnamed_5 = 13;
pub const GC_WEPSLOT4: C2RustUnnamed_5 = 12;
pub const GC_WEPSLOT3: C2RustUnnamed_5 = 11;
pub const GC_WEPSLOT2: C2RustUnnamed_5 = 10;
pub const GC_WEPSLOT1: C2RustUnnamed_5 = 9;
pub const GC_WEAPONPREV: C2RustUnnamed_5 = 8;
pub const GC_WEAPONNEXT: C2RustUnnamed_5 = 7;
pub const GC_TURNRIGHT: C2RustUnnamed_5 = 6;
pub const GC_TURNLEFT: C2RustUnnamed_5 = 5;
pub const GC_STRAFERIGHT: C2RustUnnamed_5 = 4;
pub const GC_STRAFELEFT: C2RustUnnamed_5 = 3;
pub const GC_BACKWARD: C2RustUnnamed_5 = 2;
pub const GC_FORWARD: C2RustUnnamed_5 = 1;
pub const GC_NULL: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_6 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_6 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_6 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_6 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_6 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_6 = 51;
pub const PU_LEVEL: C2RustUnnamed_6 = 50;
pub const PU_HWRCACHE: C2RustUnnamed_6 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_6 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_6 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_6 = 21;
pub const PU_HUDGFX: C2RustUnnamed_6 = 19;
pub const PU_SPRITE: C2RustUnnamed_6 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_6 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_6 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_6 = 15;
pub const PU_MUSIC: C2RustUnnamed_6 = 12;
pub const PU_SOUND: C2RustUnnamed_6 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_6 = 3;
pub const PU_LUA: C2RustUnnamed_6 = 2;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const REFRESHDIR_MAX: C2RustUnnamed_7 = 32;
pub const REFRESHDIR_NOTLOADED: C2RustUnnamed_7 = 16;
pub const REFRESHDIR_ADDFILE: C2RustUnnamed_7 = 2;
pub const REFRESHDIR_NORMAL: C2RustUnnamed_7 = 1;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return ((a as int64_t * b as int64_t) as uint64_t >> 16 as libc::c_int) as fixed_t;
}
#[inline(always)]
unsafe extern "C" fn FixedInt(mut a: fixed_t) -> fixed_t {
    return FixedMul(a, 1 as libc::c_int);
}
static mut con_started: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut con_startup: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut con_refresh: boolean = false_0 as libc::c_int;
static mut con_forcepic: boolean = true_0 as libc::c_int;
#[no_mangle]
pub static mut con_recalc: boolean = 0;
static mut con_tick: tic_t = 0;
static mut consoletoggle: boolean = 0;
static mut consoleready: boolean = 0;
#[no_mangle]
pub static mut con_destlines: int32_t = 0;
static mut con_curlines: int32_t = 0;
#[no_mangle]
pub static mut con_clipviewtop: int32_t = 0;
static mut con_hudlines: uint8_t = 0;
static mut con_hudtime: [uint32_t; 20] = [0; 20];
#[no_mangle]
pub static mut con_clearlines: int32_t = 0;
#[no_mangle]
pub static mut con_hudupdate: boolean = 0;
static mut con_line: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut con_cx: size_t = 0;
static mut con_cy: size_t = 0;
static mut con_totallines: size_t = 0;
static mut con_width: size_t = 0;
static mut con_scrollup: size_t = 0;
#[no_mangle]
pub static mut con_scalefactor: uint32_t = 0;
static mut inputlines: [[libc::c_char; 256]; 32] = [[0; 256]; 32];
static mut inputline: int32_t = 0;
static mut inputhist: int32_t = 0;
static mut input_cur: size_t = 0;
static mut input_sel: size_t = 0;
static mut input_len: size_t = 0;
static mut con_buffer: [libc::c_char; 16384] = [0; 16384];
static mut hudtime_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 99999999 as libc::c_int,
            strvalue: b"MAX\0" as *const u8 as *const libc::c_char,
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
static mut cons_hudtime: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_hudtime\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"5\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: hudtime_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut hudlines_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 20 as libc::c_int,
            strvalue: b"MAX\0" as *const u8 as *const libc::c_char,
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
static mut cons_hudlines: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_hudlines\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"5\0" as *const u8 as *const libc::c_char,
            flags: CV_CALL as libc::c_int | CV_SAVE as libc::c_int,
            PossibleValue: hudlines_cons_t.as_ptr() as *mut _,
            func: Some(CONS_hudlines_Change as unsafe extern "C" fn() -> ()),
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut speed_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 64 as libc::c_int,
            strvalue: b"MAX\0" as *const u8 as *const libc::c_char,
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
static mut cons_speed: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_speed\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"8\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: speed_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut cons_height: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_height\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"50\0" as *const u8 as *const libc::c_char,
            flags: CV_CALL as libc::c_int | CV_SAVE as libc::c_int,
            PossibleValue: CV_Unsigned.as_ptr() as *mut _,
            func: Some(CONS_height_Change as unsafe extern "C" fn() -> ()),
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut backpic_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"translucent\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"picture\0" as *const u8 as *const libc::c_char,
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
static mut cons_backpic: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_backpic\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"translucent\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: backpic_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut backcolor_cons_t: [CV_PossibleValue_t; 20] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"White\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"Black\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"Sepia\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 3 as libc::c_int,
            strvalue: b"Brown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 4 as libc::c_int,
            strvalue: b"Pink\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 5 as libc::c_int,
            strvalue: b"Raspberry\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 6 as libc::c_int,
            strvalue: b"Red\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 7 as libc::c_int,
            strvalue: b"Creamsicle\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 8 as libc::c_int,
            strvalue: b"Orange\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 9 as libc::c_int,
            strvalue: b"Gold\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 10 as libc::c_int,
            strvalue: b"Yellow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 11 as libc::c_int,
            strvalue: b"Emerald\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 12 as libc::c_int,
            strvalue: b"Green\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 13 as libc::c_int,
            strvalue: b"Cyan\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 14 as libc::c_int,
            strvalue: b"Steel\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 15 as libc::c_int,
            strvalue: b"Periwinkle\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 16 as libc::c_int,
            strvalue: b"Blue\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 17 as libc::c_int,
            strvalue: b"Purple\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 18 as libc::c_int,
            strvalue: b"Lavender\0" as *const u8 as *const libc::c_char,
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
pub static mut cons_backcolor: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"con_backcolor\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Green\0" as *const u8 as *const libc::c_char,
            flags: CV_CALL as libc::c_int | CV_SAVE as libc::c_int,
            PossibleValue: backcolor_cons_t.as_ptr() as *mut _,
            func: Some(CONS_backcolor_Change as unsafe extern "C" fn() -> ()),
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
unsafe extern "C" fn CONS_height_Change() {
    if con_destlines > 0 as libc::c_int && con_startup == 0 {
        CON_ChangeHeight();
    }
}
unsafe extern "C" fn CONS_hudlines_Change() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < con_hudlines as libc::c_int {
        con_hudtime[i as usize] = 0 as libc::c_int as uint32_t;
        i += 1;
        i;
    }
    con_hudlines = cons_hudlines.value as uint8_t;
    CONS_Printf(
        b"Number of console HUD lines is now %d\n\0" as *const u8 as *const libc::c_char,
        con_hudlines as libc::c_int,
    );
}
unsafe extern "C" fn CONS_Clear_f() {
    memset(
        con_buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16384 as libc::c_int as libc::c_ulong,
    );
    con_cx = 0 as libc::c_int as size_t;
    con_cy = con_totallines.wrapping_sub(1 as libc::c_int as size_t);
    con_line = &mut *con_buffer.as_mut_ptr().offset((con_cy * con_width) as isize)
        as *mut libc::c_char;
    con_scrollup = 0 as libc::c_int as size_t;
}
static mut bindtable: [*mut libc::c_char; 484] = [0 as *const libc::c_char
    as *mut libc::c_char; 484];
unsafe extern "C" fn CONS_Bind_f() {
    let mut na: size_t = 0;
    let mut key: int32_t = 0;
    na = COM_Argc();
    if na != 2 as libc::c_int as size_t && na != 3 as libc::c_int as size_t {
        CONS_Printf(
            b"bind <keyname> [<command>]: create shortcut keys to command(s)\n\0"
                as *const u8 as *const libc::c_char,
        );
        CONS_Printf(
            b"\x82%s\0" as *const u8 as *const libc::c_char,
            b"Bind table :\n\0" as *const u8 as *const libc::c_char,
        );
        na = 0 as libc::c_int as size_t;
        key = 0 as libc::c_int;
        while key < NUMINPUTS as libc::c_int {
            if !(bindtable[key as usize]).is_null() {
                CONS_Printf(
                    b"%s : \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    G_KeyNumToName(key),
                    bindtable[key as usize],
                );
                na = 1 as libc::c_int as size_t;
            }
            key += 1;
            key;
        }
        if na == 0 {
            CONS_Printf(b"(empty)\n\0" as *const u8 as *const libc::c_char);
        }
        return;
    }
    key = G_KeyNameToNum(COM_Argv(1 as libc::c_int as size_t));
    if key <= 0 as libc::c_int || key >= NUMINPUTS as libc::c_int {
        CONS_Alert(
            CONS_NOTICE,
            b"Invalid key name\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Z_Free(bindtable[key as usize] as *mut libc::c_void);
    bindtable[key as usize] = 0 as *mut libc::c_char;
    if na == 3 as libc::c_int as size_t {
        bindtable[key as usize] = Z_StrDup(COM_Argv(2 as libc::c_int as size_t));
    }
}
#[no_mangle]
pub static mut yellowmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut magentamap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut lgreenmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut bluemap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut graymap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut redmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut orangemap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut skymap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut purplemap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut aquamap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut peridotmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut azuremap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut brownmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut rosymap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut invertmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut consolebgmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut promptbgmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
static mut promptbgcolor: uint8_t = 255 as libc::c_int as uint8_t;
#[no_mangle]
pub unsafe extern "C" fn CON_SetupBackColormapEx(
    mut color: int32_t,
    mut prompt: boolean,
) {
    let mut i: uint16_t = 0;
    let mut palsum: uint16_t = 0;
    let mut j: uint8_t = 0;
    let mut palindex: uint8_t = 0;
    let mut pal: *mut uint8_t = W_CacheLumpName(GetPalette(), PU_CACHE as libc::c_int)
        as *mut uint8_t;
    let mut shift: int32_t = 6 as libc::c_int;
    if color == 2147483647 as libc::c_int {
        color = cons_backcolor.value;
    }
    shift = 6 as libc::c_int;
    match color {
        0 => {
            palindex = 15 as libc::c_int as uint8_t;
        }
        1 => {
            palindex = 31 as libc::c_int as uint8_t;
        }
        2 => {
            palindex = 251 as libc::c_int as uint8_t;
        }
        3 => {
            palindex = 239 as libc::c_int as uint8_t;
        }
        4 => {
            palindex = 215 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        5 => {
            palindex = 37 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        6 => {
            palindex = 47 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        7 => {
            palindex = 53 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        8 => {
            palindex = 63 as libc::c_int as uint8_t;
        }
        9 => {
            palindex = 56 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        10 => {
            palindex = 79 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        11 => {
            palindex = 119 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        12 => {
            palindex = 111 as libc::c_int as uint8_t;
        }
        13 => {
            palindex = 136 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        14 => {
            palindex = 175 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        15 => {
            palindex = 166 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        16 => {
            palindex = 159 as libc::c_int as uint8_t;
        }
        17 => {
            palindex = 187 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        18 => {
            palindex = 199 as libc::c_int as uint8_t;
            shift = 7 as libc::c_int;
        }
        _ => {
            palindex = 111 as libc::c_int as uint8_t;
        }
    }
    if prompt != 0 {
        if promptbgmap.is_null() {
            promptbgmap = Z_MallocAlign(
                256 as libc::c_int as size_t,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut uint8_t;
        }
        if color == promptbgcolor as libc::c_int {
            return
        } else {
            promptbgcolor = color as uint8_t;
        }
    } else if consolebgmap.is_null() {
        consolebgmap = Z_MallocAlign(
            256 as libc::c_int as size_t,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint8_t;
    }
    i = 0 as libc::c_int as uint16_t;
    j = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 768 as libc::c_int {
        palsum = (*pal.offset(i as isize) as libc::c_int
            + *pal.offset((i as libc::c_int + 1 as libc::c_int) as isize) as libc::c_int
            + *pal.offset((i as libc::c_int + 2 as libc::c_int) as isize) as libc::c_int
            >> shift) as uint16_t;
        if prompt != 0 {
            *promptbgmap
                .offset(
                    j as isize,
                ) = (palindex as libc::c_int - palsum as libc::c_int) as uint8_t;
        } else {
            *consolebgmap
                .offset(
                    j as isize,
                ) = (palindex as libc::c_int - palsum as libc::c_int) as uint8_t;
        }
        i = (i as libc::c_int + 3 as libc::c_int) as uint16_t;
        j = j.wrapping_add(1);
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CON_SetupBackColormap() {
    CON_SetupBackColormapEx(cons_backcolor.value, false_0 as libc::c_int);
    CON_SetupBackColormapEx(1 as libc::c_int, true_0 as libc::c_int);
}
unsafe extern "C" fn CONS_backcolor_Change() {
    CON_SetupBackColormapEx(cons_backcolor.value, false_0 as libc::c_int);
}
unsafe extern "C" fn CON_SetupColormaps() {
    let mut i: int32_t = 0;
    let mut memorysrc: *mut uint8_t = Z_MallocAlign(
        (256 as libc::c_int * 15 as libc::c_int) as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut uint8_t;
    magentamap = memorysrc;
    yellowmap = magentamap.offset(256 as libc::c_int as isize);
    lgreenmap = yellowmap.offset(256 as libc::c_int as isize);
    bluemap = lgreenmap.offset(256 as libc::c_int as isize);
    redmap = bluemap.offset(256 as libc::c_int as isize);
    graymap = redmap.offset(256 as libc::c_int as isize);
    orangemap = graymap.offset(256 as libc::c_int as isize);
    skymap = orangemap.offset(256 as libc::c_int as isize);
    purplemap = skymap.offset(256 as libc::c_int as isize);
    aquamap = purplemap.offset(256 as libc::c_int as isize);
    peridotmap = aquamap.offset(256 as libc::c_int as isize);
    azuremap = peridotmap.offset(256 as libc::c_int as isize);
    brownmap = azuremap.offset(256 as libc::c_int as isize);
    rosymap = brownmap.offset(256 as libc::c_int as isize);
    invertmap = rosymap.offset(256 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int * 15 as libc::c_int {
        *memorysrc = (i & 0xff as libc::c_int) as uint8_t;
        i += 1;
        i;
        memorysrc = memorysrc.offset(1);
        memorysrc;
    }
    *magentamap.offset(0 as libc::c_int as isize) = 177 as libc::c_int as uint8_t;
    *magentamap.offset(0x1 as libc::c_int as isize) = 177 as libc::c_int as uint8_t;
    *magentamap.offset(0x2 as libc::c_int as isize) = 178 as libc::c_int as uint8_t;
    *magentamap.offset(0x3 as libc::c_int as isize) = 178 as libc::c_int as uint8_t;
    *magentamap.offset(0x4 as libc::c_int as isize) = 178 as libc::c_int as uint8_t;
    *magentamap.offset(0x5 as libc::c_int as isize) = 180 as libc::c_int as uint8_t;
    *magentamap.offset(0x6 as libc::c_int as isize) = 180 as libc::c_int as uint8_t;
    *magentamap.offset(0x7 as libc::c_int as isize) = 180 as libc::c_int as uint8_t;
    *magentamap.offset(0x8 as libc::c_int as isize) = 182 as libc::c_int as uint8_t;
    *magentamap.offset(0x9 as libc::c_int as isize) = 182 as libc::c_int as uint8_t;
    *magentamap.offset(0xa as libc::c_int as isize) = 182 as libc::c_int as uint8_t;
    *magentamap.offset(0xb as libc::c_int as isize) = 182 as libc::c_int as uint8_t;
    *magentamap.offset(0xc as libc::c_int as isize) = 184 as libc::c_int as uint8_t;
    *magentamap.offset(0xd as libc::c_int as isize) = 184 as libc::c_int as uint8_t;
    *magentamap.offset(0xe as libc::c_int as isize) = 184 as libc::c_int as uint8_t;
    *magentamap.offset(0xf as libc::c_int as isize) = 185 as libc::c_int as uint8_t;
    *yellowmap.offset(0 as libc::c_int as isize) = 82 as libc::c_int as uint8_t;
    *yellowmap.offset(0x1 as libc::c_int as isize) = 82 as libc::c_int as uint8_t;
    *yellowmap.offset(0x2 as libc::c_int as isize) = 73 as libc::c_int as uint8_t;
    *yellowmap.offset(0x3 as libc::c_int as isize) = 73 as libc::c_int as uint8_t;
    *yellowmap.offset(0x4 as libc::c_int as isize) = 73 as libc::c_int as uint8_t;
    *yellowmap.offset(0x5 as libc::c_int as isize) = 74 as libc::c_int as uint8_t;
    *yellowmap.offset(0x6 as libc::c_int as isize) = 74 as libc::c_int as uint8_t;
    *yellowmap.offset(0x7 as libc::c_int as isize) = 74 as libc::c_int as uint8_t;
    *yellowmap.offset(0x8 as libc::c_int as isize) = 66 as libc::c_int as uint8_t;
    *yellowmap.offset(0x9 as libc::c_int as isize) = 66 as libc::c_int as uint8_t;
    *yellowmap.offset(0xa as libc::c_int as isize) = 66 as libc::c_int as uint8_t;
    *yellowmap.offset(0xb as libc::c_int as isize) = 66 as libc::c_int as uint8_t;
    *yellowmap.offset(0xc as libc::c_int as isize) = 67 as libc::c_int as uint8_t;
    *yellowmap.offset(0xd as libc::c_int as isize) = 67 as libc::c_int as uint8_t;
    *yellowmap.offset(0xe as libc::c_int as isize) = 67 as libc::c_int as uint8_t;
    *yellowmap.offset(0xf as libc::c_int as isize) = 68 as libc::c_int as uint8_t;
    *lgreenmap.offset(0 as libc::c_int as isize) = 96 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x1 as libc::c_int as isize) = 96 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x2 as libc::c_int as isize) = 98 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x3 as libc::c_int as isize) = 98 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x4 as libc::c_int as isize) = 98 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x5 as libc::c_int as isize) = 100 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x6 as libc::c_int as isize) = 100 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x7 as libc::c_int as isize) = 100 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x8 as libc::c_int as isize) = 103 as libc::c_int as uint8_t;
    *lgreenmap.offset(0x9 as libc::c_int as isize) = 103 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xa as libc::c_int as isize) = 103 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xb as libc::c_int as isize) = 103 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xc as libc::c_int as isize) = 105 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xd as libc::c_int as isize) = 105 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xe as libc::c_int as isize) = 105 as libc::c_int as uint8_t;
    *lgreenmap.offset(0xf as libc::c_int as isize) = 107 as libc::c_int as uint8_t;
    *bluemap.offset(0 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
    *bluemap.offset(0x1 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
    *bluemap.offset(0x2 as libc::c_int as isize) = 147 as libc::c_int as uint8_t;
    *bluemap.offset(0x3 as libc::c_int as isize) = 147 as libc::c_int as uint8_t;
    *bluemap.offset(0x4 as libc::c_int as isize) = 147 as libc::c_int as uint8_t;
    *bluemap.offset(0x5 as libc::c_int as isize) = 148 as libc::c_int as uint8_t;
    *bluemap.offset(0x6 as libc::c_int as isize) = 148 as libc::c_int as uint8_t;
    *bluemap.offset(0x7 as libc::c_int as isize) = 148 as libc::c_int as uint8_t;
    *bluemap.offset(0x8 as libc::c_int as isize) = 149 as libc::c_int as uint8_t;
    *bluemap.offset(0x9 as libc::c_int as isize) = 149 as libc::c_int as uint8_t;
    *bluemap.offset(0xa as libc::c_int as isize) = 149 as libc::c_int as uint8_t;
    *bluemap.offset(0xb as libc::c_int as isize) = 149 as libc::c_int as uint8_t;
    *bluemap.offset(0xc as libc::c_int as isize) = 150 as libc::c_int as uint8_t;
    *bluemap.offset(0xd as libc::c_int as isize) = 150 as libc::c_int as uint8_t;
    *bluemap.offset(0xe as libc::c_int as isize) = 150 as libc::c_int as uint8_t;
    *bluemap.offset(0xf as libc::c_int as isize) = 151 as libc::c_int as uint8_t;
    *redmap.offset(0 as libc::c_int as isize) = 32 as libc::c_int as uint8_t;
    *redmap.offset(0x1 as libc::c_int as isize) = 32 as libc::c_int as uint8_t;
    *redmap.offset(0x2 as libc::c_int as isize) = 33 as libc::c_int as uint8_t;
    *redmap.offset(0x3 as libc::c_int as isize) = 33 as libc::c_int as uint8_t;
    *redmap.offset(0x4 as libc::c_int as isize) = 33 as libc::c_int as uint8_t;
    *redmap.offset(0x5 as libc::c_int as isize) = 34 as libc::c_int as uint8_t;
    *redmap.offset(0x6 as libc::c_int as isize) = 34 as libc::c_int as uint8_t;
    *redmap.offset(0x7 as libc::c_int as isize) = 34 as libc::c_int as uint8_t;
    *redmap.offset(0x8 as libc::c_int as isize) = 35 as libc::c_int as uint8_t;
    *redmap.offset(0x9 as libc::c_int as isize) = 35 as libc::c_int as uint8_t;
    *redmap.offset(0xa as libc::c_int as isize) = 35 as libc::c_int as uint8_t;
    *redmap.offset(0xb as libc::c_int as isize) = 35 as libc::c_int as uint8_t;
    *redmap.offset(0xc as libc::c_int as isize) = 37 as libc::c_int as uint8_t;
    *redmap.offset(0xd as libc::c_int as isize) = 37 as libc::c_int as uint8_t;
    *redmap.offset(0xe as libc::c_int as isize) = 37 as libc::c_int as uint8_t;
    *redmap.offset(0xf as libc::c_int as isize) = 39 as libc::c_int as uint8_t;
    *graymap.offset(0 as libc::c_int as isize) = 8 as libc::c_int as uint8_t;
    *graymap.offset(0x1 as libc::c_int as isize) = 9 as libc::c_int as uint8_t;
    *graymap.offset(0x2 as libc::c_int as isize) = 10 as libc::c_int as uint8_t;
    *graymap.offset(0x3 as libc::c_int as isize) = 11 as libc::c_int as uint8_t;
    *graymap.offset(0x4 as libc::c_int as isize) = 12 as libc::c_int as uint8_t;
    *graymap.offset(0x5 as libc::c_int as isize) = 13 as libc::c_int as uint8_t;
    *graymap.offset(0x6 as libc::c_int as isize) = 14 as libc::c_int as uint8_t;
    *graymap.offset(0x7 as libc::c_int as isize) = 15 as libc::c_int as uint8_t;
    *graymap.offset(0x8 as libc::c_int as isize) = 16 as libc::c_int as uint8_t;
    *graymap.offset(0x9 as libc::c_int as isize) = 17 as libc::c_int as uint8_t;
    *graymap.offset(0xa as libc::c_int as isize) = 18 as libc::c_int as uint8_t;
    *graymap.offset(0xb as libc::c_int as isize) = 19 as libc::c_int as uint8_t;
    *graymap.offset(0xc as libc::c_int as isize) = 20 as libc::c_int as uint8_t;
    *graymap.offset(0xd as libc::c_int as isize) = 21 as libc::c_int as uint8_t;
    *graymap.offset(0xe as libc::c_int as isize) = 22 as libc::c_int as uint8_t;
    *graymap.offset(0xf as libc::c_int as isize) = 23 as libc::c_int as uint8_t;
    *orangemap.offset(0 as libc::c_int as isize) = 50 as libc::c_int as uint8_t;
    *orangemap.offset(0x1 as libc::c_int as isize) = 50 as libc::c_int as uint8_t;
    *orangemap.offset(0x2 as libc::c_int as isize) = 52 as libc::c_int as uint8_t;
    *orangemap.offset(0x3 as libc::c_int as isize) = 52 as libc::c_int as uint8_t;
    *orangemap.offset(0x4 as libc::c_int as isize) = 52 as libc::c_int as uint8_t;
    *orangemap.offset(0x5 as libc::c_int as isize) = 54 as libc::c_int as uint8_t;
    *orangemap.offset(0x6 as libc::c_int as isize) = 54 as libc::c_int as uint8_t;
    *orangemap.offset(0x7 as libc::c_int as isize) = 54 as libc::c_int as uint8_t;
    *orangemap.offset(0x8 as libc::c_int as isize) = 56 as libc::c_int as uint8_t;
    *orangemap.offset(0x9 as libc::c_int as isize) = 56 as libc::c_int as uint8_t;
    *orangemap.offset(0xa as libc::c_int as isize) = 56 as libc::c_int as uint8_t;
    *orangemap.offset(0xb as libc::c_int as isize) = 56 as libc::c_int as uint8_t;
    *orangemap.offset(0xc as libc::c_int as isize) = 59 as libc::c_int as uint8_t;
    *orangemap.offset(0xd as libc::c_int as isize) = 59 as libc::c_int as uint8_t;
    *orangemap.offset(0xe as libc::c_int as isize) = 59 as libc::c_int as uint8_t;
    *orangemap.offset(0xf as libc::c_int as isize) = 60 as libc::c_int as uint8_t;
    *skymap.offset(0 as libc::c_int as isize) = 129 as libc::c_int as uint8_t;
    *skymap.offset(0x1 as libc::c_int as isize) = 129 as libc::c_int as uint8_t;
    *skymap.offset(0x2 as libc::c_int as isize) = 130 as libc::c_int as uint8_t;
    *skymap.offset(0x3 as libc::c_int as isize) = 130 as libc::c_int as uint8_t;
    *skymap.offset(0x4 as libc::c_int as isize) = 130 as libc::c_int as uint8_t;
    *skymap.offset(0x5 as libc::c_int as isize) = 131 as libc::c_int as uint8_t;
    *skymap.offset(0x6 as libc::c_int as isize) = 131 as libc::c_int as uint8_t;
    *skymap.offset(0x7 as libc::c_int as isize) = 131 as libc::c_int as uint8_t;
    *skymap.offset(0x8 as libc::c_int as isize) = 133 as libc::c_int as uint8_t;
    *skymap.offset(0x9 as libc::c_int as isize) = 133 as libc::c_int as uint8_t;
    *skymap.offset(0xa as libc::c_int as isize) = 133 as libc::c_int as uint8_t;
    *skymap.offset(0xb as libc::c_int as isize) = 133 as libc::c_int as uint8_t;
    *skymap.offset(0xc as libc::c_int as isize) = 135 as libc::c_int as uint8_t;
    *skymap.offset(0xd as libc::c_int as isize) = 135 as libc::c_int as uint8_t;
    *skymap.offset(0xe as libc::c_int as isize) = 135 as libc::c_int as uint8_t;
    *skymap.offset(0xf as libc::c_int as isize) = 136 as libc::c_int as uint8_t;
    *purplemap.offset(0 as libc::c_int as isize) = 160 as libc::c_int as uint8_t;
    *purplemap.offset(0x1 as libc::c_int as isize) = 160 as libc::c_int as uint8_t;
    *purplemap.offset(0x2 as libc::c_int as isize) = 161 as libc::c_int as uint8_t;
    *purplemap.offset(0x3 as libc::c_int as isize) = 161 as libc::c_int as uint8_t;
    *purplemap.offset(0x4 as libc::c_int as isize) = 161 as libc::c_int as uint8_t;
    *purplemap.offset(0x5 as libc::c_int as isize) = 162 as libc::c_int as uint8_t;
    *purplemap.offset(0x6 as libc::c_int as isize) = 162 as libc::c_int as uint8_t;
    *purplemap.offset(0x7 as libc::c_int as isize) = 162 as libc::c_int as uint8_t;
    *purplemap.offset(0x8 as libc::c_int as isize) = 163 as libc::c_int as uint8_t;
    *purplemap.offset(0x9 as libc::c_int as isize) = 163 as libc::c_int as uint8_t;
    *purplemap.offset(0xa as libc::c_int as isize) = 163 as libc::c_int as uint8_t;
    *purplemap.offset(0xb as libc::c_int as isize) = 163 as libc::c_int as uint8_t;
    *purplemap.offset(0xc as libc::c_int as isize) = 164 as libc::c_int as uint8_t;
    *purplemap.offset(0xd as libc::c_int as isize) = 164 as libc::c_int as uint8_t;
    *purplemap.offset(0xe as libc::c_int as isize) = 164 as libc::c_int as uint8_t;
    *purplemap.offset(0xf as libc::c_int as isize) = 165 as libc::c_int as uint8_t;
    *aquamap.offset(0 as libc::c_int as isize) = 120 as libc::c_int as uint8_t;
    *aquamap.offset(0x1 as libc::c_int as isize) = 120 as libc::c_int as uint8_t;
    *aquamap.offset(0x2 as libc::c_int as isize) = 121 as libc::c_int as uint8_t;
    *aquamap.offset(0x3 as libc::c_int as isize) = 121 as libc::c_int as uint8_t;
    *aquamap.offset(0x4 as libc::c_int as isize) = 121 as libc::c_int as uint8_t;
    *aquamap.offset(0x5 as libc::c_int as isize) = 122 as libc::c_int as uint8_t;
    *aquamap.offset(0x6 as libc::c_int as isize) = 122 as libc::c_int as uint8_t;
    *aquamap.offset(0x7 as libc::c_int as isize) = 122 as libc::c_int as uint8_t;
    *aquamap.offset(0x8 as libc::c_int as isize) = 123 as libc::c_int as uint8_t;
    *aquamap.offset(0x9 as libc::c_int as isize) = 123 as libc::c_int as uint8_t;
    *aquamap.offset(0xa as libc::c_int as isize) = 123 as libc::c_int as uint8_t;
    *aquamap.offset(0xb as libc::c_int as isize) = 123 as libc::c_int as uint8_t;
    *aquamap.offset(0xc as libc::c_int as isize) = 124 as libc::c_int as uint8_t;
    *aquamap.offset(0xd as libc::c_int as isize) = 124 as libc::c_int as uint8_t;
    *aquamap.offset(0xe as libc::c_int as isize) = 124 as libc::c_int as uint8_t;
    *aquamap.offset(0xf as libc::c_int as isize) = 125 as libc::c_int as uint8_t;
    *peridotmap.offset(0 as libc::c_int as isize) = 73 as libc::c_int as uint8_t;
    *peridotmap.offset(0x1 as libc::c_int as isize) = 73 as libc::c_int as uint8_t;
    *peridotmap.offset(0x2 as libc::c_int as isize) = 188 as libc::c_int as uint8_t;
    *peridotmap.offset(0x3 as libc::c_int as isize) = 188 as libc::c_int as uint8_t;
    *peridotmap.offset(0x4 as libc::c_int as isize) = 188 as libc::c_int as uint8_t;
    *peridotmap.offset(0x5 as libc::c_int as isize) = 189 as libc::c_int as uint8_t;
    *peridotmap.offset(0x6 as libc::c_int as isize) = 189 as libc::c_int as uint8_t;
    *peridotmap.offset(0x7 as libc::c_int as isize) = 189 as libc::c_int as uint8_t;
    *peridotmap.offset(0x8 as libc::c_int as isize) = 190 as libc::c_int as uint8_t;
    *peridotmap.offset(0x9 as libc::c_int as isize) = 190 as libc::c_int as uint8_t;
    *peridotmap.offset(0xa as libc::c_int as isize) = 190 as libc::c_int as uint8_t;
    *peridotmap.offset(0xb as libc::c_int as isize) = 190 as libc::c_int as uint8_t;
    *peridotmap.offset(0xc as libc::c_int as isize) = 191 as libc::c_int as uint8_t;
    *peridotmap.offset(0xd as libc::c_int as isize) = 191 as libc::c_int as uint8_t;
    *peridotmap.offset(0xe as libc::c_int as isize) = 191 as libc::c_int as uint8_t;
    *peridotmap.offset(0xf as libc::c_int as isize) = 94 as libc::c_int as uint8_t;
    *azuremap.offset(0 as libc::c_int as isize) = 144 as libc::c_int as uint8_t;
    *azuremap.offset(0x1 as libc::c_int as isize) = 144 as libc::c_int as uint8_t;
    *azuremap.offset(0x2 as libc::c_int as isize) = 145 as libc::c_int as uint8_t;
    *azuremap.offset(0x3 as libc::c_int as isize) = 145 as libc::c_int as uint8_t;
    *azuremap.offset(0x4 as libc::c_int as isize) = 145 as libc::c_int as uint8_t;
    *azuremap.offset(0x5 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
    *azuremap.offset(0x6 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
    *azuremap.offset(0x7 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
    *azuremap.offset(0x8 as libc::c_int as isize) = 170 as libc::c_int as uint8_t;
    *azuremap.offset(0x9 as libc::c_int as isize) = 170 as libc::c_int as uint8_t;
    *azuremap.offset(0xa as libc::c_int as isize) = 170 as libc::c_int as uint8_t;
    *azuremap.offset(0xb as libc::c_int as isize) = 170 as libc::c_int as uint8_t;
    *azuremap.offset(0xc as libc::c_int as isize) = 171 as libc::c_int as uint8_t;
    *azuremap.offset(0xd as libc::c_int as isize) = 171 as libc::c_int as uint8_t;
    *azuremap.offset(0xe as libc::c_int as isize) = 171 as libc::c_int as uint8_t;
    *azuremap.offset(0xf as libc::c_int as isize) = 172 as libc::c_int as uint8_t;
    *brownmap.offset(0 as libc::c_int as isize) = 219 as libc::c_int as uint8_t;
    *brownmap.offset(0x1 as libc::c_int as isize) = 219 as libc::c_int as uint8_t;
    *brownmap.offset(0x2 as libc::c_int as isize) = 221 as libc::c_int as uint8_t;
    *brownmap.offset(0x3 as libc::c_int as isize) = 221 as libc::c_int as uint8_t;
    *brownmap.offset(0x4 as libc::c_int as isize) = 221 as libc::c_int as uint8_t;
    *brownmap.offset(0x5 as libc::c_int as isize) = 222 as libc::c_int as uint8_t;
    *brownmap.offset(0x6 as libc::c_int as isize) = 222 as libc::c_int as uint8_t;
    *brownmap.offset(0x7 as libc::c_int as isize) = 222 as libc::c_int as uint8_t;
    *brownmap.offset(0x8 as libc::c_int as isize) = 224 as libc::c_int as uint8_t;
    *brownmap.offset(0x9 as libc::c_int as isize) = 224 as libc::c_int as uint8_t;
    *brownmap.offset(0xa as libc::c_int as isize) = 224 as libc::c_int as uint8_t;
    *brownmap.offset(0xb as libc::c_int as isize) = 224 as libc::c_int as uint8_t;
    *brownmap.offset(0xc as libc::c_int as isize) = 227 as libc::c_int as uint8_t;
    *brownmap.offset(0xd as libc::c_int as isize) = 227 as libc::c_int as uint8_t;
    *brownmap.offset(0xe as libc::c_int as isize) = 227 as libc::c_int as uint8_t;
    *brownmap.offset(0xf as libc::c_int as isize) = 229 as libc::c_int as uint8_t;
    *rosymap.offset(0 as libc::c_int as isize) = 200 as libc::c_int as uint8_t;
    *rosymap.offset(0x1 as libc::c_int as isize) = 200 as libc::c_int as uint8_t;
    *rosymap.offset(0x2 as libc::c_int as isize) = 201 as libc::c_int as uint8_t;
    *rosymap.offset(0x3 as libc::c_int as isize) = 201 as libc::c_int as uint8_t;
    *rosymap.offset(0x4 as libc::c_int as isize) = 201 as libc::c_int as uint8_t;
    *rosymap.offset(0x5 as libc::c_int as isize) = 202 as libc::c_int as uint8_t;
    *rosymap.offset(0x6 as libc::c_int as isize) = 202 as libc::c_int as uint8_t;
    *rosymap.offset(0x7 as libc::c_int as isize) = 202 as libc::c_int as uint8_t;
    *rosymap.offset(0x8 as libc::c_int as isize) = 203 as libc::c_int as uint8_t;
    *rosymap.offset(0x9 as libc::c_int as isize) = 203 as libc::c_int as uint8_t;
    *rosymap.offset(0xa as libc::c_int as isize) = 203 as libc::c_int as uint8_t;
    *rosymap.offset(0xb as libc::c_int as isize) = 203 as libc::c_int as uint8_t;
    *rosymap.offset(0xc as libc::c_int as isize) = 204 as libc::c_int as uint8_t;
    *rosymap.offset(0xd as libc::c_int as isize) = 204 as libc::c_int as uint8_t;
    *rosymap.offset(0xe as libc::c_int as isize) = 204 as libc::c_int as uint8_t;
    *rosymap.offset(0xf as libc::c_int as isize) = 205 as libc::c_int as uint8_t;
    i = 0 as libc::c_int;
    while i <= 0x1f as libc::c_int {
        *invertmap.offset((0x1f as libc::c_int - i) as isize) = i as uint8_t;
        i += 1;
        i;
    }
    CON_SetupBackColormap();
}
#[no_mangle]
pub unsafe extern "C" fn CON_Init() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < NUMINPUTS as libc::c_int {
        bindtable[i as usize] = 0 as *mut libc::c_char;
        i += 1;
        i;
    }
    memset(
        con_buffer.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        16384 as libc::c_int as libc::c_ulong,
    );
    con_width = 0 as libc::c_int as size_t;
    CON_RecalcSize();
    CON_SetupColormaps();
    con_clipviewtop = -(1 as libc::c_int);
    con_hudlines = atoi(cons_hudlines.defaultvalue) as uint8_t;
    CON_InputInit();
    COM_AddCommand(
        b"cls\0" as *const u8 as *const libc::c_char,
        Some(CONS_Clear_f as unsafe extern "C" fn() -> ()),
        0 as com_flags_t,
    );
    con_destlines = vid.height;
    con_curlines = vid.height;
    if dedicated == 0 {
        con_started = true_0 as libc::c_int;
        con_startup = true_0 as libc::c_int;
        con_refresh = true_0 as libc::c_int;
        consoletoggle = false_0 as libc::c_int;
        CV_RegisterVar(&mut cons_hudtime);
        CV_RegisterVar(&mut cons_hudlines);
        CV_RegisterVar(&mut cons_speed);
        CV_RegisterVar(&mut cons_height);
        CV_RegisterVar(&mut cons_backpic);
        CV_RegisterVar(&mut cons_backcolor);
        COM_AddCommand(
            b"bind\0" as *const u8 as *const libc::c_char,
            Some(CONS_Bind_f as unsafe extern "C" fn() -> ()),
            0 as com_flags_t,
        );
    } else {
        con_started = true_0 as libc::c_int;
        con_startup = false_0 as libc::c_int;
        con_refresh = false_0 as libc::c_int;
        consoletoggle = true_0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn CON_StartRefresh() {
    if con_startup != 0 {
        con_refresh = true_0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CON_StopRefresh() {
    if con_startup != 0 {
        con_refresh = false_0 as libc::c_int;
    }
}
unsafe extern "C" fn CON_InputInit() {
    memset(
        inputlines.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[libc::c_char; 256]; 32]>() as libc::c_ulong,
    );
    inputline = 0 as libc::c_int;
    input_len = 0 as libc::c_int as size_t;
    input_sel = input_len;
    input_cur = input_sel;
}
unsafe extern "C" fn CON_RecalcSize() {
    let mut conw: size_t = 0;
    let mut oldcon_width: size_t = 0;
    let mut oldnumlines: size_t = 0;
    let mut i: size_t = 0;
    let mut oldcon_cy: size_t = 0;
    let mut tmp_buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    match cv_constextsize.value {
        256 => {
            con_scalefactor = 1 as libc::c_int as uint32_t;
        }
        512 => {
            con_scalefactor = vid.smalldupx as uint32_t;
        }
        768 => {
            con_scalefactor = vid.meddupx as uint32_t;
        }
        _ => {
            con_scalefactor = vid.dupx as uint32_t;
        }
    }
    con_recalc = false_0 as libc::c_int;
    if dedicated != 0 {
        conw = 1 as libc::c_int as size_t;
    } else {
        conw = ((vid.width >> 3 as libc::c_int) as uint32_t / con_scalefactor)
            .wrapping_sub(2 as libc::c_int as uint32_t) as size_t;
    }
    if con_curlines == vid.height {
        con_curlines = vid.height;
        con_destlines = vid.height;
    }
    if con_destlines > 0 as libc::c_int {
        CON_ChangeHeight();
        con_curlines = con_destlines;
    }
    if conw == con_width {
        return;
    }
    tmp_buffer = Z_MallocAlign(
        16384 as libc::c_int as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    string = Z_MallocAlign(
        16384 as libc::c_int as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    oldcon_width = con_width;
    oldnumlines = con_totallines;
    oldcon_cy = con_cy;
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        tmp_buffer as *mut libc::c_void,
        con_buffer.as_mut_ptr() as *const libc::c_void,
        16384 as libc::c_int as size_t,
    );
    if conw < 1 as libc::c_int as size_t {
        con_width = ((320 as libc::c_int >> 3 as libc::c_int) - 2 as libc::c_int)
            as size_t;
    } else {
        con_width = conw;
    }
    con_width = con_width.wrapping_add(11 as libc::c_int as size_t);
    con_totallines = 16384 as libc::c_int as size_t / con_width;
    memset(
        con_buffer.as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        16384 as libc::c_int as libc::c_ulong,
    );
    con_cx = 0 as libc::c_int as size_t;
    con_cy = con_totallines.wrapping_sub(1 as libc::c_int as size_t);
    con_line = &mut *con_buffer.as_mut_ptr().offset((con_cy * con_width) as isize)
        as *mut libc::c_char;
    con_scrollup = 0 as libc::c_int as size_t;
    if oldcon_width != 0 {
        i = oldcon_cy.wrapping_add(1 as libc::c_int as size_t);
        while i < oldcon_cy.wrapping_add(oldnumlines) {
            if *tmp_buffer.offset((i % oldnumlines * oldcon_width) as isize) != 0 {
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    string as *mut libc::c_void,
                    &mut *tmp_buffer.offset((i % oldnumlines * oldcon_width) as isize)
                        as *mut libc::c_char as *const libc::c_void,
                    oldcon_width,
                );
                conw = oldcon_width.wrapping_sub(1 as libc::c_int as size_t);
                while *string.offset(conw as isize) as libc::c_int == ' ' as i32
                    && conw != 0
                {
                    conw = conw.wrapping_sub(1);
                    conw;
                }
                *string
                    .offset(
                        conw.wrapping_add(1 as libc::c_int as size_t) as isize,
                    ) = '\n' as i32 as libc::c_char;
                *string
                    .offset(
                        conw.wrapping_add(2 as libc::c_int as size_t) as isize,
                    ) = '\0' as i32 as libc::c_char;
                CON_Print(string);
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    Z_Free(string as *mut libc::c_void);
    Z_Free(tmp_buffer as *mut libc::c_void);
}
unsafe extern "C" fn CON_ChangeHeight() {
    let mut minheight: int32_t = 0;
    minheight = (20 as libc::c_int as uint32_t * con_scalefactor) as int32_t;
    con_destlines = cons_height.value * vid.height / 100 as libc::c_int;
    if con_destlines < minheight {
        con_destlines = minheight;
    } else if con_destlines > vid.height {
        con_destlines = vid.height;
    }
    con_destlines &= !(0x3 as libc::c_int);
}
unsafe extern "C" fn CON_MoveConsole() {
    static mut fracmovement: fixed_t = 0 as libc::c_int;
    if cons_speed.value == 0 {
        con_curlines = con_destlines;
        return;
    }
    fracmovement += FixedMul(cons_speed.value * vid.fdupy, renderdeltatics);
    if con_curlines < con_destlines {
        con_curlines += FixedInt(fracmovement);
        if con_curlines > con_destlines {
            con_curlines = con_destlines;
        }
    } else {
        con_curlines -= FixedInt(fracmovement);
        if con_curlines < con_destlines {
            con_curlines = con_destlines;
        }
        if con_destlines == 0 as libc::c_int {
            con_tick = 0 as libc::c_int as tic_t;
        }
    }
    fracmovement %= (1 as libc::c_int) << 16 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CON_ClearHUD() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < con_hudlines as libc::c_int {
        con_hudtime[i as usize] = 0 as libc::c_int as uint32_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CON_ToggleOff() {
    if con_destlines == 0 {
        return;
    }
    con_destlines = 0 as libc::c_int;
    con_curlines = 0 as libc::c_int;
    CON_ClearHUD();
    con_forcepic = 0 as libc::c_int;
    con_clipviewtop = -(1 as libc::c_int);
    I_UpdateMouseGrab();
}
#[no_mangle]
pub unsafe extern "C" fn CON_Ready() -> boolean {
    let mut ready: boolean = 0;
    ready = consoleready;
    return ready;
}
#[no_mangle]
pub unsafe extern "C" fn CON_Ticker() {
    let mut i: int32_t = 0;
    let mut minheight: int32_t = 0;
    minheight = (20 as libc::c_int as uint32_t * con_scalefactor) as int32_t;
    con_tick = con_tick.wrapping_add(1);
    con_tick;
    con_tick &= 7 as libc::c_int as tic_t;
    if consoletoggle != 0 {
        consoletoggle = false_0 as libc::c_int;
        if con_destlines > 0 as libc::c_int {
            con_destlines = 0 as libc::c_int;
            CON_ClearHUD();
            I_UpdateMouseGrab();
        } else {
            CON_ChangeHeight();
        }
    }
    con_clipviewtop = -(1 as libc::c_int);
    if cons_backpic.value != 0 {
        if con_curlines > 0 as libc::c_int {
            con_clipviewtop = con_curlines - viewwindowy - 1 as libc::c_int
                - 10 as libc::c_int;
        }
        if con_clipviewtop < 0 as libc::c_int {
            con_clipviewtop = -(1 as libc::c_int);
        }
    }
    if con_destlines >= minheight {
        consoleready = true_0 as libc::c_int;
    } else {
        consoleready = false_0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < con_hudlines as libc::c_int {
        if con_hudtime[i as usize] != 0 {
            con_hudtime[i as usize] = (con_hudtime[i as usize]).wrapping_sub(1);
            con_hudtime[i as usize];
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn CON_InputClear() {
    memset(
        (inputlines[inputline as usize]).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    input_len = 0 as libc::c_int as size_t;
    input_sel = input_len;
    input_cur = input_sel;
}
unsafe extern "C" fn CON_InputSetString(mut c: *const libc::c_char) {
    memset(
        (inputlines[inputline as usize]).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    strcpy((inputlines[inputline as usize]).as_mut_ptr(), c);
    input_len = strlen(c);
    input_sel = input_len;
    input_cur = input_sel;
}
unsafe extern "C" fn CON_InputAddString(mut c: *const libc::c_char) {
    let mut csize: size_t = strlen(c);
    if input_len.wrapping_add(csize) > (256 as libc::c_int - 1 as libc::c_int) as size_t
    {
        return;
    }
    if input_cur != input_len {
        memmove(
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur.wrapping_add(csize) as isize) as *mut libc::c_char
                as *mut libc::c_void,
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur as isize) as *mut libc::c_char as *const libc::c_void,
            input_len.wrapping_sub(input_cur),
        );
    }
    memcpy(
        &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
            .as_mut_ptr()
            .offset(input_cur as isize) as *mut libc::c_char as *mut libc::c_void,
        c as *const libc::c_void,
        csize,
    );
    input_len = input_len.wrapping_add(csize);
    input_cur = input_cur.wrapping_add(csize);
    input_sel = input_cur;
}
unsafe extern "C" fn CON_InputDelSelection() {
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut len: size_t = 0;
    if input_cur == 0 {
        return;
    }
    if input_cur > input_sel {
        start = input_sel;
        end = input_cur;
    } else {
        start = input_cur;
        end = input_sel;
    }
    len = end.wrapping_sub(start);
    if end != input_len {
        memmove(
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(start as isize) as *mut libc::c_char as *mut libc::c_void,
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(end as isize) as *mut libc::c_char as *const libc::c_void,
            input_len.wrapping_sub(end),
        );
    }
    memset(
        &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
            .as_mut_ptr()
            .offset(input_len.wrapping_sub(len) as isize) as *mut libc::c_char
            as *mut libc::c_void,
        0 as libc::c_int,
        len,
    );
    input_len = input_len.wrapping_sub(len);
    input_cur = start;
    input_sel = input_cur;
}
unsafe extern "C" fn CON_InputAddChar(mut c: libc::c_char) {
    if input_len >= (256 as libc::c_int - 1 as libc::c_int) as size_t {
        return;
    }
    if input_cur != input_len {
        memmove(
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur.wrapping_add(1 as libc::c_int as size_t) as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur as isize) as *mut libc::c_char as *const libc::c_void,
            input_len.wrapping_sub(input_cur),
        );
    }
    let fresh0 = input_cur;
    input_cur = input_cur.wrapping_add(1);
    inputlines[inputline as usize][fresh0 as usize] = c;
    input_len = input_len.wrapping_add(1);
    inputlines[inputline
        as usize][input_len as usize] = 0 as libc::c_int as libc::c_char;
    input_sel = input_cur;
}
unsafe extern "C" fn CON_InputDelChar() {
    if input_cur == 0 {
        return;
    }
    if input_cur != input_len {
        memmove(
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as *mut libc::c_char as *mut libc::c_void,
            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                .as_mut_ptr()
                .offset(input_cur as isize) as *mut libc::c_char as *const libc::c_void,
            input_len.wrapping_sub(input_cur),
        );
    }
    input_len = input_len.wrapping_sub(1);
    inputlines[inputline
        as usize][input_len as usize] = 0 as libc::c_int as libc::c_char;
    input_cur = input_cur.wrapping_sub(1);
    input_sel = input_cur;
}
#[no_mangle]
pub unsafe extern "C" fn CON_Responder(mut ev: *mut event_t) -> boolean {
    static mut consdown: uint8_t = false_0 as libc::c_int as uint8_t;
    static mut completion: [libc::c_char; 80] = [0; 80];
    static mut skips: int32_t = 0;
    static mut com_skips: int32_t = 0;
    static mut var_skips: int32_t = 0;
    static mut alias_skips: int32_t = 0;
    let mut cmd: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: int32_t = 0;
    if chat_on != 0 {
        return false_0 as libc::c_int;
    }
    if (*ev).type_0 as libc::c_uint != ev_keydown as libc::c_int as libc::c_uint
        && (*ev).type_0 as libc::c_uint != ev_console as libc::c_int as libc::c_uint
    {
        if (*ev).key
            == gamecontrol[GC_CONSOLE as libc::c_int as usize][0 as libc::c_int as usize]
            || (*ev).key
                == gamecontrol[GC_CONSOLE as libc::c_int
                    as usize][1 as libc::c_int as usize]
        {
            consdown = false_0 as libc::c_int as uint8_t;
        }
        return false_0 as libc::c_int;
    }
    key = (*ev).key;
    if (*ev).type_0 as libc::c_uint != ev_console as libc::c_int as libc::c_uint {
        if modeattacking as libc::c_int != 0 || metalrecording != 0
            || marathonmode as libc::c_uint != 0
        {
            return false_0 as libc::c_int;
        }
        if key
            == gamecontrol[GC_CONSOLE as libc::c_int as usize][0 as libc::c_int as usize]
            || key
                == gamecontrol[GC_CONSOLE as libc::c_int
                    as usize][1 as libc::c_int as usize]
        {
            if consdown != 0 {
                return true_0 as libc::c_int;
            }
            consoletoggle = true_0 as libc::c_int;
            consdown = true_0 as libc::c_int as uint8_t;
            return true_0 as libc::c_int;
        }
        if consoleready == 0 && key < NUMINPUTS as libc::c_int {
            if menuactive == 0 && !(bindtable[key as usize]).is_null() {
                COM_BufAddTextEx(bindtable[key as usize], 0 as com_flags_t);
                COM_BufAddTextEx(
                    b"\n\0" as *const u8 as *const libc::c_char,
                    0 as com_flags_t,
                );
                return true_0 as libc::c_int;
            }
            return false_0 as libc::c_int;
        }
        if key == 27 as libc::c_int {
            consoletoggle = true_0 as libc::c_int;
            return true_0 as libc::c_int;
        }
    }
    if key == 0x80 as libc::c_int + 54 as libc::c_int
        || key == 0x80 as libc::c_int + 55 as libc::c_int
        || key == 0x80 as libc::c_int + 29 as libc::c_int
        || key == 0x80 as libc::c_int + 30 as libc::c_int
        || key == 0x80 as libc::c_int + 56 as libc::c_int
        || key == 0x80 as libc::c_int + 57 as libc::c_int
    {
        return true_0 as libc::c_int;
    }
    if key == 0x80 as libc::c_int + 105 as libc::c_int {
        if input_cur != 0 as libc::c_int as size_t {
            if ctrldown != 0 {
                input_cur = M_JumpWordReverse(
                    (inputlines[inputline as usize]).as_mut_ptr(),
                    input_cur as libc::c_int,
                ) as size_t;
            } else {
                input_cur = input_cur.wrapping_sub(1);
                input_cur;
            }
        }
        if shiftdown == 0 {
            input_sel = input_cur;
        }
        return true_0 as libc::c_int;
    } else if key == 0x80 as libc::c_int + 107 as libc::c_int {
        if input_cur < input_len {
            if ctrldown != 0 {
                input_cur = input_cur
                    .wrapping_add(
                        M_JumpWord(
                            &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                                .as_mut_ptr()
                                .offset(input_cur as isize),
                        ) as size_t,
                    );
            } else {
                input_cur = input_cur.wrapping_add(1);
                input_cur;
            }
        }
        if shiftdown == 0 {
            input_sel = input_cur;
        }
        return true_0 as libc::c_int;
    }
    if input_sel != input_cur {
        if key == 8 as libc::c_int || key == 0x80 as libc::c_int + 113 as libc::c_int {
            CON_InputDelSelection();
            return true_0 as libc::c_int;
        }
    } else if key == 8 as libc::c_int {
        if ctrldown != 0 {
            input_sel = M_JumpWordReverse(
                (inputlines[inputline as usize]).as_mut_ptr(),
                input_cur as libc::c_int,
            ) as size_t;
            CON_InputDelSelection();
        } else {
            CON_InputDelChar();
        }
        return true_0 as libc::c_int;
    } else if key == 0x80 as libc::c_int + 113 as libc::c_int {
        if input_cur == input_len {
            return true_0 as libc::c_int;
        }
        if ctrldown != 0 {
            input_sel = input_cur
                .wrapping_add(
                    M_JumpWord(
                        &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                            .as_mut_ptr()
                            .offset(input_cur as isize),
                    ) as size_t,
                );
            CON_InputDelSelection();
        } else {
            input_cur = input_cur.wrapping_add(1);
            input_cur;
            CON_InputDelChar();
        }
        return true_0 as libc::c_int;
    }
    if ctrldown != 0 {
        if key == 9 as libc::c_int {
            let mut i: size_t = 0;
            let mut len: size_t = 0;
            if completion[0 as libc::c_int as usize] == 0 {
                if input_len == 0 || input_len >= 40 as libc::c_int as size_t
                    || !(strchr(
                        (inputlines[inputline as usize]).as_mut_ptr(),
                        ' ' as i32,
                    ))
                        .is_null()
                {
                    return true_0 as libc::c_int;
                }
                strcpy(
                    completion.as_mut_ptr(),
                    (inputlines[inputline as usize]).as_mut_ptr(),
                );
            }
            len = strlen(completion.as_mut_ptr());
            CONS_Printf(b"\nCommands:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as size_t;
            cmd = COM_CompleteCommand(completion.as_mut_ptr(), i as int32_t);
            while !cmd.is_null() {
                CONS_Printf(
                    b"  \x83%s\x80%s\n\0" as *const u8 as *const libc::c_char,
                    completion.as_mut_ptr(),
                    cmd.offset(len as isize),
                );
                i = i.wrapping_add(1);
                cmd = COM_CompleteCommand(completion.as_mut_ptr(), i as int32_t);
            }
            if i == 0 as libc::c_int as size_t {
                CONS_Printf(b"  (none)\n\0" as *const u8 as *const libc::c_char);
            }
            CONS_Printf(b"Variables:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as size_t;
            cmd = CV_CompleteVar(completion.as_mut_ptr(), i as int32_t);
            while !cmd.is_null() {
                CONS_Printf(
                    b"  \x83%s\x80%s\n\0" as *const u8 as *const libc::c_char,
                    completion.as_mut_ptr(),
                    cmd.offset(len as isize),
                );
                i = i.wrapping_add(1);
                cmd = CV_CompleteVar(completion.as_mut_ptr(), i as int32_t);
            }
            if i == 0 as libc::c_int as size_t {
                CONS_Printf(b"  (none)\n\0" as *const u8 as *const libc::c_char);
            }
            CONS_Printf(b"Aliases:\n\0" as *const u8 as *const libc::c_char);
            i = 0 as libc::c_int as size_t;
            cmd = COM_CompleteAlias(completion.as_mut_ptr(), i as int32_t);
            while !cmd.is_null() {
                CONS_Printf(
                    b"  \x83%s\x80%s\n\0" as *const u8 as *const libc::c_char,
                    completion.as_mut_ptr(),
                    cmd.offset(len as isize),
                );
                i = i.wrapping_add(1);
                cmd = COM_CompleteAlias(completion.as_mut_ptr(), i as int32_t);
            }
            if i == 0 as libc::c_int as size_t {
                CONS_Printf(b"  (none)\n\0" as *const u8 as *const libc::c_char);
            }
            completion[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            return true_0 as libc::c_int;
        }
        if key == 0x80 as libc::c_int + 101 as libc::c_int {
            con_scrollup = con_totallines
                .wrapping_sub(
                    (con_curlines - 16 as libc::c_int >> 3 as libc::c_int) as size_t,
                );
            return true_0 as libc::c_int;
        } else if key == 0x80 as libc::c_int + 109 as libc::c_int {
            con_scrollup = 0 as libc::c_int as size_t;
            return true_0 as libc::c_int;
        }
        if key == 'x' as i32 || key == 'X' as i32 {
            if input_sel > input_cur {
                I_ClipboardCopy(
                    &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                        .as_mut_ptr()
                        .offset(input_cur as isize),
                    input_sel.wrapping_sub(input_cur),
                );
            } else {
                I_ClipboardCopy(
                    &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                        .as_mut_ptr()
                        .offset(input_sel as isize),
                    input_cur.wrapping_sub(input_sel),
                );
            }
            CON_InputDelSelection();
            completion[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            return true_0 as libc::c_int;
        } else if key == 'c' as i32 || key == 'C' as i32 {
            if input_sel > input_cur {
                I_ClipboardCopy(
                    &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                        .as_mut_ptr()
                        .offset(input_cur as isize),
                    input_sel.wrapping_sub(input_cur),
                );
            } else {
                I_ClipboardCopy(
                    &mut *(*inputlines.as_mut_ptr().offset(inputline as isize))
                        .as_mut_ptr()
                        .offset(input_sel as isize),
                    input_cur.wrapping_sub(input_sel),
                );
            }
            return true_0 as libc::c_int;
        } else if key == 'v' as i32 || key == 'V' as i32 {
            let mut paste: *const libc::c_char = I_ClipboardPaste();
            if input_sel != input_cur {
                CON_InputDelSelection();
            }
            if !paste.is_null() {
                CON_InputAddString(paste);
            }
            completion[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            return true_0 as libc::c_int;
        }
        if key == 'a' as i32 || key == 'A' as i32 {
            input_sel = 0 as libc::c_int as size_t;
            input_cur = input_len;
            return true_0 as libc::c_int;
        }
        return true_0 as libc::c_int;
    }
    if key == 9 as libc::c_int {
        if completion[0 as libc::c_int as usize] == 0 {
            if input_len == 0 || input_len >= 40 as libc::c_int as size_t
                || !(strchr((inputlines[inputline as usize]).as_mut_ptr(), ' ' as i32))
                    .is_null()
            {
                return true_0 as libc::c_int;
            }
            strcpy(
                completion.as_mut_ptr(),
                (inputlines[inputline as usize]).as_mut_ptr(),
            );
            skips = 0 as libc::c_int;
            com_skips = 0 as libc::c_int;
            var_skips = 0 as libc::c_int;
            alias_skips = 0 as libc::c_int;
        } else if shiftdown != 0 {
            if skips > 0 as libc::c_int {
                skips -= 1;
                skips;
            }
        } else {
            skips += 1;
            skips;
        }
        if skips <= com_skips {
            cmd = COM_CompleteCommand(completion.as_mut_ptr(), skips);
            if !cmd.is_null() && skips == com_skips {
                com_skips += 1;
                com_skips;
                var_skips += 1;
                var_skips;
                alias_skips += 1;
                alias_skips;
            }
        }
        if cmd.is_null() && skips <= var_skips {
            cmd = CV_CompleteVar(completion.as_mut_ptr(), skips - com_skips);
            if !cmd.is_null() && skips == var_skips {
                var_skips += 1;
                var_skips;
                alias_skips += 1;
                alias_skips;
            }
        }
        if cmd.is_null() && skips <= alias_skips {
            cmd = COM_CompleteAlias(completion.as_mut_ptr(), skips - var_skips);
            if !cmd.is_null() && skips == alias_skips {
                alias_skips += 1;
                alias_skips;
            }
        }
        if !cmd.is_null() {
            CON_InputSetString(va(b"%s \0" as *const u8 as *const libc::c_char, cmd));
        } else {
            skips -= 1;
            skips;
        }
        return true_0 as libc::c_int;
    }
    if key == 0x80 as libc::c_int + 103 as libc::c_int {
        if con_scrollup
            < con_totallines
                .wrapping_sub(
                    (con_curlines - 16 as libc::c_int >> 3 as libc::c_int) as size_t,
                )
        {
            con_scrollup = con_scrollup.wrapping_add(1);
            con_scrollup;
        }
        return true_0 as libc::c_int;
    } else if key == 0x80 as libc::c_int + 111 as libc::c_int {
        if con_scrollup > 0 as libc::c_int as size_t {
            con_scrollup = con_scrollup.wrapping_sub(1);
            con_scrollup;
        }
        return true_0 as libc::c_int;
    } else if key == 0x80 as libc::c_int + 101 as libc::c_int {
        input_cur = 0 as libc::c_int as size_t;
        if shiftdown == 0 {
            input_sel = input_cur;
        }
        return true_0 as libc::c_int;
    } else if key == 0x80 as libc::c_int + 109 as libc::c_int {
        input_cur = input_len;
        if shiftdown == 0 {
            input_sel = input_cur;
        }
        return true_0 as libc::c_int;
    }
    completion[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if key == 13 as libc::c_int {
        if input_len == 0 {
            return true_0 as libc::c_int;
        }
        COM_BufAddTextEx(
            (inputlines[inputline as usize]).as_mut_ptr(),
            0 as com_flags_t,
        );
        COM_BufAddTextEx(b"\n\0" as *const u8 as *const libc::c_char, 0 as com_flags_t);
        CONS_Printf(
            b"\x86%c\x80%s\n\0" as *const u8 as *const libc::c_char,
            '$' as i32,
            (inputlines[inputline as usize]).as_mut_ptr(),
        );
        inputline = inputline + 1 as libc::c_int & 31 as libc::c_int;
        inputhist = inputline;
        CON_InputClear();
        return true_0 as libc::c_int;
    }
    if key == 0x80 as libc::c_int + 102 as libc::c_int {
        loop {
            inputhist = inputhist - 1 as libc::c_int & 31 as libc::c_int;
            if !(inputhist != inputline
                && inputlines[inputhist as usize][0 as libc::c_int as usize] == 0)
            {
                break;
            }
        }
        if inputhist == inputline {
            inputhist = inputline + 1 as libc::c_int & 31 as libc::c_int;
        }
        CON_InputSetString((inputlines[inputhist as usize]).as_mut_ptr());
        return true_0 as libc::c_int;
    }
    if key == 0x80 as libc::c_int + 110 as libc::c_int {
        if inputhist == inputline {
            return true_0 as libc::c_int;
        }
        loop {
            inputhist = inputhist + 1 as libc::c_int & 31 as libc::c_int;
            if !(inputhist != inputline
                && inputlines[inputhist as usize][0 as libc::c_int as usize] == 0)
            {
                break;
            }
        }
        if inputhist == inputline {
            CON_InputClear();
        } else {
            CON_InputSetString((inputlines[inputhist as usize]).as_mut_ptr());
        }
        return true_0 as libc::c_int;
    }
    if key >= 0x80 as libc::c_int + 71 as libc::c_int
        && key <= 0x80 as libc::c_int + 83 as libc::c_int
    {
        let mut keypad_translation: [libc::c_char; 13] = [
            '7' as i32 as libc::c_char,
            '8' as i32 as libc::c_char,
            '9' as i32 as libc::c_char,
            '-' as i32 as libc::c_char,
            '4' as i32 as libc::c_char,
            '5' as i32 as libc::c_char,
            '6' as i32 as libc::c_char,
            '+' as i32 as libc::c_char,
            '1' as i32 as libc::c_char,
            '2' as i32 as libc::c_char,
            '3' as i32 as libc::c_char,
            '0' as i32 as libc::c_char,
            '.' as i32 as libc::c_char,
        ];
        key = keypad_translation[(key - (0x80 as libc::c_int + 71 as libc::c_int))
            as usize] as int32_t;
    } else if key == 0x80 as libc::c_int + 100 as libc::c_int {
        key = '/' as i32;
    }
    if key >= 'a' as i32 && key <= 'z' as i32 {
        if capslock ^ shiftdown as libc::c_int != 0 {
            key = *shiftxform.offset(key as isize) as int32_t;
        }
    } else if shiftdown != 0 {
        key = *shiftxform.offset(key as isize) as int32_t;
    }
    if key < 32 as libc::c_int || key > 127 as libc::c_int {
        return true_0 as libc::c_int;
    }
    if input_sel != input_cur {
        CON_InputDelSelection();
    }
    CON_InputAddChar(key as libc::c_char);
    return true_0 as libc::c_int;
}
unsafe extern "C" fn CON_Linefeed() {
    if con_hudlines != 0 {
        con_hudtime[(con_cy % con_hudlines as size_t)
            as usize] = (cons_hudtime.value * 35 as libc::c_int) as uint32_t;
    }
    con_cy = con_cy.wrapping_add(1);
    con_cy;
    con_cx = 0 as libc::c_int as size_t;
    con_line = &mut *con_buffer
        .as_mut_ptr()
        .offset((con_cy % con_totallines * con_width) as isize) as *mut libc::c_char;
    memset(con_line as *mut libc::c_void, ' ' as i32, con_width);
    con_hudupdate = true_0 as libc::c_int;
}
unsafe extern "C" fn CON_Print(mut msg: *mut libc::c_char) {
    let mut l: size_t = 0;
    let mut controlchars: int32_t = 0 as libc::c_int;
    let mut color: libc::c_char = -128i32 as libc::c_char;
    if msg.is_null() {
        return;
    }
    if *msg as libc::c_int == '\u{3}' as i32 {
        S_StartSound(0 as *const libc::c_void, sfx_radio);
    } else if *msg as libc::c_int == '\u{4}' as i32 {
        *msg = -126i32 as libc::c_char;
        S_StartSound(0 as *const libc::c_void, sfx_radio);
    }
    if *msg as libc::c_int & 0x80 as libc::c_int == 0 {
        let fresh1 = con_cx;
        con_cx = con_cx.wrapping_add(1);
        *con_line.offset(fresh1 as isize) = -128i32 as libc::c_char;
        controlchars = 1 as libc::c_int;
    }
    while *msg != 0 {
        while *msg as libc::c_int != 0 && *msg as libc::c_int <= ' ' as i32 {
            if *msg as libc::c_int & 0x80 as libc::c_int != 0 {
                let fresh2 = msg;
                msg = msg.offset(1);
                let fresh3 = con_cx;
                con_cx = con_cx.wrapping_add(1);
                let ref mut fresh4 = *con_line.offset(fresh3 as isize);
                *fresh4 = *fresh2;
                color = *fresh4;
                controlchars += 1;
                controlchars;
            } else {
                if *msg as libc::c_int == '\r' as i32 {
                    con_cy = con_cy.wrapping_sub(1);
                    con_cy;
                    CON_Linefeed();
                    color = -128i32 as libc::c_char;
                    controlchars = 0 as libc::c_int;
                } else if *msg as libc::c_int == '\n' as i32 {
                    CON_Linefeed();
                    let fresh5 = con_cx;
                    con_cx = con_cx.wrapping_add(1);
                    *con_line.offset(fresh5 as isize) = color;
                    controlchars = 1 as libc::c_int;
                } else if *msg as libc::c_int == ' ' as i32 {
                    let fresh6 = con_cx;
                    con_cx = con_cx.wrapping_add(1);
                    *con_line.offset(fresh6 as isize) = ' ' as i32 as libc::c_char;
                    if con_cx.wrapping_sub(controlchars as size_t)
                        >= con_width.wrapping_sub(11 as libc::c_int as size_t)
                    {
                        CON_Linefeed();
                        let fresh7 = con_cx;
                        con_cx = con_cx.wrapping_add(1);
                        *con_line.offset(fresh7 as isize) = color;
                        controlchars = 1 as libc::c_int;
                    }
                } else if *msg as libc::c_int == '\t' as i32 {
                    loop {
                        let fresh8 = con_cx;
                        con_cx = con_cx.wrapping_add(1);
                        *con_line.offset(fresh8 as isize) = ' ' as i32 as libc::c_char;
                        if !(con_cx.wrapping_sub(controlchars as size_t)
                            % 4 as libc::c_int as size_t != 0 as libc::c_int as size_t)
                        {
                            break;
                        }
                    }
                    if con_cx.wrapping_sub(controlchars as size_t)
                        >= con_width.wrapping_sub(11 as libc::c_int as size_t)
                    {
                        CON_Linefeed();
                        let fresh9 = con_cx;
                        con_cx = con_cx.wrapping_add(1);
                        *con_line.offset(fresh9 as isize) = color;
                        controlchars = 1 as libc::c_int;
                    }
                }
                msg = msg.offset(1);
                msg;
            }
        }
        if *msg as libc::c_int == '\0' as i32 {
            return;
        }
        l = 0 as libc::c_int as size_t;
        while l < con_width.wrapping_sub(11 as libc::c_int as size_t)
            && *msg.offset(l as isize) as libc::c_int > ' ' as i32
        {
            l = l.wrapping_add(1);
            l;
        }
        if con_cx.wrapping_sub(controlchars as size_t).wrapping_add(l)
            > con_width.wrapping_sub(11 as libc::c_int as size_t)
        {
            CON_Linefeed();
            let fresh10 = con_cx;
            con_cx = con_cx.wrapping_add(1);
            *con_line.offset(fresh10 as isize) = color;
            controlchars = 1 as libc::c_int;
        }
        while l > 0 as libc::c_int as size_t {
            let fresh11 = msg;
            msg = msg.offset(1);
            let fresh12 = con_cx;
            con_cx = con_cx.wrapping_add(1);
            *con_line.offset(fresh12 as isize) = *fresh11;
            l = l.wrapping_sub(1);
            l;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CON_LogMessage(mut msg: *const libc::c_char) {
    let mut txt: [libc::c_char; 8192] = [0; 8192];
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = msg;
    let mut e: *const libc::c_char = txt
        .as_mut_ptr()
        .offset(::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as isize)
        .offset(-(2 as libc::c_int as isize));
    t = txt.as_mut_ptr();
    while *p as libc::c_int != '\0' as i32 {
        if *p as libc::c_int == '\n' as i32 || *p as libc::c_int >= ' ' as i32 {
            let fresh13 = t;
            t = t.offset(1);
            *fresh13 = *p;
        }
        if t >= e as *mut libc::c_char {
            *t = '\0' as i32 as libc::c_char;
            I_OutputMsg(b"%s\0" as *const u8 as *const libc::c_char, txt.as_mut_ptr());
            t = txt.as_mut_ptr();
            memset(
                txt.as_mut_ptr() as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
            );
        }
        p = p.offset(1);
        p;
    }
    *t = '\0' as i32 as libc::c_char;
    I_OutputMsg(b"%s\0" as *const u8 as *const libc::c_char, txt.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CONS_Printf(mut fmt: *const libc::c_char, mut args: ...) {
    let mut argptr: ::core::ffi::VaListImpl;
    static mut txt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut refresh: boolean = 0;
    if txt.is_null() {
        txt = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    }
    argptr = args.clone();
    vsprintf(txt, fmt, argptr.as_va_list());
    if !debugfile.is_null() {
        fputs(txt, debugfile);
        fflush(debugfile);
    }
    if con_started != 0 {
        CON_Print(txt);
    }
    CON_LogMessage(txt);
    con_scrollup = 0 as libc::c_int as size_t;
    refresh = con_refresh;
    if refresh != 0 {
        CON_Drawer();
        I_FinishUpdate();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CONS_Alert(
    mut level: alerttype_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::core::ffi::VaListImpl;
    static mut txt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if txt.is_null() {
        txt = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    }
    argptr = args.clone();
    vsprintf(txt, fmt, argptr.as_va_list());
    match level as libc::c_uint {
        0 => {
            CONS_Printf(
                b"\x83%s\x80 \0" as *const u8 as *const libc::c_char,
                b"NOTICE:\0" as *const u8 as *const libc::c_char,
            );
        }
        1 => {
            refreshdirmenu = (refreshdirmenu as libc::c_int
                | REFRESHDIR_WARNING as libc::c_int) as uint8_t;
            CONS_Printf(
                b"\x82%s\x80 \0" as *const u8 as *const libc::c_char,
                b"WARNING:\0" as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            refreshdirmenu = (refreshdirmenu as libc::c_int
                | REFRESHDIR_ERROR as libc::c_int) as uint8_t;
            CONS_Printf(
                b"\x85%s\x80 \0" as *const u8 as *const libc::c_char,
                b"ERROR:\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    CONS_Printf(b"%s\0" as *const u8 as *const libc::c_char, txt);
}
#[no_mangle]
pub unsafe extern "C" fn CONS_Debug(
    mut debugflags: int32_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut argptr: ::core::ffi::VaListImpl;
    static mut txt: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    if cv_debug & debugflags != debugflags {
        return;
    }
    if txt.is_null() {
        txt = malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    }
    argptr = args.clone();
    vsprintf(txt, fmt, argptr.as_va_list());
    CONS_Printf(b"%s\0" as *const u8 as *const libc::c_char, txt);
}
#[no_mangle]
pub unsafe extern "C" fn CONS_Error(mut msg: *const libc::c_char) {
    CONS_Printf(b"\x82%s\0" as *const u8 as *const libc::c_char, msg);
    CONS_Printf(b"Press ENTER to continue\n\0" as *const u8 as *const libc::c_char);
    while I_GetKey() != 13 as libc::c_int {
        I_OsPolling();
    }
}
unsafe extern "C" fn CON_DrawInput() {
    let mut charwidth: int32_t = (con_scalefactor as int32_t) << 3 as libc::c_int;
    let mut p: *const libc::c_char = (inputlines[inputline as usize]).as_mut_ptr();
    let mut c: size_t = 0;
    let mut clen: size_t = 0;
    let mut cend: size_t = 0;
    let mut lellip: uint8_t = 0 as libc::c_int as uint8_t;
    let mut rellip: uint8_t = 0 as libc::c_int as uint8_t;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut i: int32_t = 0;
    y = (con_curlines as uint32_t)
        .wrapping_sub(12 as libc::c_int as uint32_t * con_scalefactor) as int32_t;
    x = charwidth * 2 as libc::c_int;
    clen = con_width.wrapping_sub(13 as libc::c_int as size_t);
    if input_len <= clen {
        c = 0 as libc::c_int as size_t;
        clen = input_len;
    } else {
        clen = clen.wrapping_sub(2 as libc::c_int as size_t);
        if input_cur <= clen / 2 as libc::c_int as size_t {
            c = 0 as libc::c_int as size_t;
            rellip = 1 as libc::c_int as uint8_t;
        } else {
            c = input_cur
                .wrapping_sub(clen / 2 as libc::c_int as size_t)
                .wrapping_add(2 as libc::c_int as size_t);
            x += charwidth * 2 as libc::c_int;
            lellip = 1 as libc::c_int as uint8_t;
            if c.wrapping_add(clen) >= input_len {
                c = input_len.wrapping_sub(clen);
            } else {
                clen = clen.wrapping_sub(2 as libc::c_int as size_t);
                rellip = 1 as libc::c_int as uint8_t;
            }
        }
    }
    if lellip != 0 {
        x -= charwidth * 3 as libc::c_int;
        if input_sel < c {
            V_DrawFill(
                x,
                y,
                charwidth * 3 as libc::c_int,
                (10 as libc::c_int as uint32_t * con_scalefactor) as int32_t,
                77 as libc::c_int | 0x40000000 as libc::c_int,
            );
        }
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            V_DrawCharacter(
                x,
                y,
                '.' as i32 | cv_constextsize.value | 0x6000 as libc::c_int
                    | 0x40000000 as libc::c_int,
                true_0 as libc::c_int,
            );
            i += 1;
            i;
            x += charwidth;
        }
    } else {
        V_DrawCharacter(
            x - charwidth,
            y,
            '$' as i32 | cv_constextsize.value | 0x6000 as libc::c_int
                | 0x40000000 as libc::c_int,
            true_0 as libc::c_int,
        );
    }
    cend = c.wrapping_add(clen);
    while c < cend {
        if input_sel > c && input_cur <= c || input_sel <= c && input_cur > c {
            V_DrawFill(
                x,
                y,
                charwidth,
                (10 as libc::c_int as uint32_t * con_scalefactor) as int32_t,
                77 as libc::c_int | 0x40000000 as libc::c_int,
            );
            V_DrawCharacter(
                x,
                y,
                *p.offset(c as isize) as libc::c_int | cv_constextsize.value
                    | 0x2000 as libc::c_int | 0x40000000 as libc::c_int,
                true_0 as libc::c_int,
            );
        } else {
            V_DrawCharacter(
                x,
                y,
                *p.offset(c as isize) as libc::c_int | cv_constextsize.value
                    | 0x40000000 as libc::c_int,
                true_0 as libc::c_int,
            );
        }
        if c == input_cur && con_tick >= 4 as libc::c_int as tic_t {
            V_DrawCharacter(
                x,
                (y as uint32_t)
                    .wrapping_add(con_scalefactor * 2 as libc::c_int as uint32_t)
                    as int32_t,
                '_' as i32 | cv_constextsize.value | 0x40000000 as libc::c_int,
                true_0 as libc::c_int,
            );
        }
        c = c.wrapping_add(1);
        c;
        x += charwidth;
    }
    if cend == input_cur && con_tick >= 4 as libc::c_int as tic_t {
        V_DrawCharacter(
            x,
            (y as uint32_t).wrapping_add(con_scalefactor * 2 as libc::c_int as uint32_t)
                as int32_t,
            '_' as i32 | cv_constextsize.value | 0x40000000 as libc::c_int,
            true_0 as libc::c_int,
        );
    }
    if rellip != 0 {
        if input_sel > cend {
            V_DrawFill(
                x,
                y,
                charwidth * 3 as libc::c_int,
                (10 as libc::c_int as uint32_t * con_scalefactor) as int32_t,
                77 as libc::c_int | 0x40000000 as libc::c_int,
            );
        }
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            V_DrawCharacter(
                x,
                y,
                '.' as i32 | cv_constextsize.value | 0x6000 as libc::c_int
                    | 0x40000000 as libc::c_int,
                true_0 as libc::c_int,
            );
            i += 1;
            i;
            x += charwidth;
        }
    }
}
unsafe extern "C" fn CON_DrawHudlines() {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: size_t = 0;
    let mut y: int32_t = 0;
    let mut charflags: int32_t = 0 as libc::c_int;
    let mut charwidth: int32_t = (8 as libc::c_int as uint32_t * con_scalefactor)
        as int32_t;
    let mut charheight: int32_t = (8 as libc::c_int as uint32_t * con_scalefactor)
        as int32_t;
    if con_hudlines == 0 {
        return;
    }
    if chat_on != 0
        && (cv_consolechat.value == 1 as libc::c_int || dedicated != 0
            || vid.width < 640 as libc::c_int || splitscreen != 0)
    {
        y = charheight;
    } else {
        y = 0 as libc::c_int;
    }
    i = con_cy.wrapping_sub(con_hudlines as size_t);
    while i <= con_cy {
        let mut c: size_t = 0;
        let mut x: int32_t = 0;
        if !((i as libc::c_int) < 0 as libc::c_int) {
            if !(con_hudtime[(i % con_hudlines as size_t) as usize]
                == 0 as libc::c_int as uint32_t)
            {
                p = &mut *con_buffer
                    .as_mut_ptr()
                    .offset((i % con_totallines * con_width) as isize)
                    as *mut libc::c_char as *mut uint8_t;
                c = 0 as libc::c_int as size_t;
                x = 0 as libc::c_int;
                while c < con_width {
                    while *p as libc::c_int & 0x80 as libc::c_int != 0 {
                        charflags = (*p as libc::c_int & 0x7f as libc::c_int)
                            << 12 as libc::c_int;
                        p = p.offset(1);
                        p;
                        c = c.wrapping_add(1);
                        c;
                    }
                    if c >= con_width {
                        break;
                    }
                    if !((*p as libc::c_int) < '\u{16}' as i32) {
                        V_DrawCharacter(
                            x,
                            y,
                            *p as int32_t | charflags | cv_constextsize.value
                                | 0x40000000 as libc::c_int,
                            true_0 as libc::c_int,
                        );
                    }
                    c = c.wrapping_add(1);
                    c;
                    x += charwidth;
                    p = p.offset(1);
                    p;
                }
                y += charheight;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    con_clearlines = y;
}
unsafe extern "C" fn CON_DrawBackpic() {
    let mut con_backpic: *mut patch_t = 0 as *mut patch_t;
    let mut piclump: lumpnum_t = 0;
    let mut x: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if con_startup != 0 {
        piclump = W_CheckNumForName(b"STARTUP\0" as *const u8 as *const libc::c_char);
    } else {
        piclump = W_CheckNumForName(b"CONSBACK\0" as *const u8 as *const libc::c_char);
    }
    if piclump == 4294967295 as libc::c_uint {
        piclump = W_GetNumForName(b"MISSING\0" as *const u8 as *const libc::c_char);
    }
    con_backpic = W_CachePatchNum(piclump, PU_PATCH as libc::c_int) as *mut patch_t;
    w = (*con_backpic).width as libc::c_int * vid.dupx;
    x = vid.width / 2 as libc::c_int - w / 2 as libc::c_int;
    h = con_curlines / vid.dupy;
    if x > 0 as libc::c_int {
        let mut column: *mut column_t = ((*con_backpic).columns)
            .offset(
                *((*con_backpic).columnofs).offset(0 as libc::c_int as isize) as isize,
            ) as *mut column_t;
        if (*column).topdelta == 0 {
            let mut source: *mut uint8_t = (column as *mut uint8_t)
                .offset(3 as libc::c_int as isize);
            let mut color: int32_t = *source.offset(0 as libc::c_int as isize)
                as libc::c_int | 0x40000000 as libc::c_int;
            V_DrawFill(0 as libc::c_int, 0 as libc::c_int, x, con_curlines, color);
            V_DrawFill(x + w, 0 as libc::c_int, vid.width - w, con_curlines, color);
        }
    }
    V_DrawCroppedPatch(
        x << 16 as libc::c_int,
        0 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0x40000000 as libc::c_int,
        con_backpic,
        0 as *const uint8_t,
        0 as libc::c_int,
        200 as libc::c_int - h << 16 as libc::c_int,
        (320 as libc::c_int) << 16 as libc::c_int,
        h << 16 as libc::c_int,
    );
    W_UnlockCachedPatch(con_backpic as *mut libc::c_void);
}
unsafe extern "C" fn CON_DrawConsole() {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: size_t = 0;
    let mut y: int32_t = 0;
    let mut charflags: int32_t = 0 as libc::c_int;
    let mut charwidth: int32_t = (con_scalefactor as int32_t) << 3 as libc::c_int;
    let mut charheight: int32_t = charwidth;
    let mut minheight: int32_t = (20 as libc::c_int as uint32_t * con_scalefactor)
        as int32_t;
    if con_curlines <= 0 as libc::c_int {
        return;
    }
    con_clearlines = con_curlines;
    con_hudupdate = true_0 as libc::c_int;
    if cons_backpic.value != 0 || con_forcepic != 0 {
        CON_DrawBackpic();
    } else if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
        V_DrawFadeConsBack(con_curlines);
    }
    if con_curlines >= minheight {
        i = con_cy.wrapping_sub(con_scrollup);
        i = i.wrapping_sub(1);
        i;
        i = i.wrapping_sub(((con_curlines - minheight) / charheight) as size_t);
        if rendermode as libc::c_uint == render_none as libc::c_int as libc::c_uint {
            return;
        }
        y = (con_curlines - minheight) % charheight;
        while y <= con_curlines - minheight {
            let mut x: int32_t = 0;
            let mut c: size_t = 0;
            p = &mut *con_buffer
                .as_mut_ptr()
                .offset(
                    ((if i > 0 as libc::c_int as size_t {
                        i
                    } else {
                        0 as libc::c_int as size_t
                    }) % con_totallines * con_width) as isize,
                ) as *mut libc::c_char as *mut uint8_t;
            c = 0 as libc::c_int as size_t;
            x = charwidth;
            while c < con_width {
                while *p as libc::c_int & 0x80 as libc::c_int != 0 {
                    charflags = (*p as libc::c_int & 0x7f as libc::c_int)
                        << 12 as libc::c_int;
                    p = p.offset(1);
                    p;
                    c = c.wrapping_add(1);
                    c;
                }
                if c >= con_width {
                    break;
                }
                V_DrawCharacter(
                    x,
                    y,
                    *p as int32_t | charflags | cv_constextsize.value
                        | 0x40000000 as libc::c_int,
                    true_0 as libc::c_int,
                );
                c = c.wrapping_add(1);
                c;
                x += charwidth;
                p = p.offset(1);
                p;
            }
            y += charheight;
            i = i.wrapping_add(1);
            i;
        }
    }
    if con_curlines >= minheight - charheight && con_startup == 0 {
        CON_DrawInput();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CON_Drawer() {
    if con_started == 0 || graphics_started == 0 {
        return;
    }
    if con_recalc != 0 {
        CON_RecalcSize();
        if con_curlines <= 0 as libc::c_int {
            CON_ClearHUD();
        }
    }
    if con_curlines != con_destlines {
        CON_MoveConsole();
    }
    if con_curlines > 0 as libc::c_int {
        CON_DrawConsole();
    } else if gamestate as libc::c_uint == GS_LEVEL as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_INTERMISSION as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_ENDING as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_CUTSCENE as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_CREDITS as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_EVALUATION as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint == GS_WAITINGPLAYERS as libc::c_int as libc::c_uint
    {
        CON_DrawHudlines();
    }
}
