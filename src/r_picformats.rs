use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type png_info_def;
    pub type png_struct_def;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strupr(n: *mut libc::c_char) -> libc::c_int;
    fn strlwr(n: *mut libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    static mut sprnames: [[libc::c_char; 5]; 908];
    static mut spr2names: [[libc::c_char; 5]; 128];
    fn NearestPaletteColor(
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
        palette: *mut RGBA_t,
    ) -> uint8_t;
    static mut spriteinfo: [spriteinfo_t; 907];
    fn Patch_Create(
        source: *mut softwarepatch_t,
        srcsize: size_t,
        dest: *mut libc::c_void,
    ) -> *mut patch_t;
    static mut textures: *mut *mut texture_t;
    fn R_CheckTextureCache(tex: int32_t);
    fn R_GetColumn(tex: fixed_t, col: int32_t) -> *mut uint8_t;
    static mut numtextures: int32_t;
    static mut skins: [skin_t; 32];
    fn R_SkinAvailable(name: *const libc::c_char) -> int32_t;
    fn InitColorLUT(lut: *mut colorlookup_t, palette: *mut RGBA_t, makecolors: boolean);
    fn GetColorLUT(
        lut: *mut colorlookup_t,
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
    ) -> uint8_t;
    static mut pLocalPalette: *mut RGBA_t;
    static mut pMasterPalette: *mut RGBA_t;
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
    static mut wadfiles: *mut *mut wadfile_t;
    fn W_LumpLengthPwad(wad: uint16_t, lump: uint16_t) -> size_t;
    fn W_CacheLumpNumPwad(
        wad: uint16_t,
        lump: uint16_t,
        tag: int32_t,
    ) -> *mut libc::c_void;
    fn png_create_read_struct(
        user_png_ver: png_const_charp,
        error_ptr: png_voidp,
        error_fn: png_error_ptr,
        warn_fn: png_error_ptr,
    ) -> png_structp;
    fn png_set_longjmp_fn(
        png_ptr: png_structrp,
        longjmp_fn: png_longjmp_ptr,
        jmp_buf_size: size_t,
    ) -> *mut jmp_buf;
    fn png_create_info_struct(png_ptr: png_const_structrp) -> png_infop;
    fn png_read_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_set_palette_to_rgb(png_ptr: png_structrp);
    fn png_set_tRNS_to_alpha(png_ptr: png_structrp);
    fn png_set_gray_to_rgb(png_ptr: png_structrp);
    fn png_set_add_alpha(png_ptr: png_structrp, filler: png_uint_32, flags: libc::c_int);
    fn png_set_strip_16(png_ptr: png_structrp);
    fn png_read_update_info(png_ptr: png_structrp, info_ptr: png_inforp);
    fn png_read_image(png_ptr: png_structrp, image: png_bytepp);
    fn png_destroy_read_struct(
        png_ptr_ptr: png_structpp,
        info_ptr_ptr: png_infopp,
        end_info_ptr_ptr: png_infopp,
    );
    fn png_set_read_fn(
        png_ptr: png_structrp,
        io_ptr: png_voidp,
        read_data_fn: png_rw_ptr,
    );
    fn png_get_io_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_set_read_user_chunk_fn(
        png_ptr: png_structrp,
        user_chunk_ptr: png_voidp,
        read_user_chunk_fn: png_user_chunk_ptr,
    );
    fn png_get_user_chunk_ptr(png_ptr: png_const_structrp) -> png_voidp;
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_get_valid(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        flag: png_uint_32,
    ) -> png_uint_32;
    fn png_get_rowbytes(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> size_t;
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut libc::c_int,
        color_type: *mut libc::c_int,
        interlace_method: *mut libc::c_int,
        compression_method: *mut libc::c_int,
        filter_method: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_get_PLTE(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        palette: *mut png_colorp,
        num_palette: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_get_tRNS(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        trans_alpha: *mut png_bytep,
        num_trans: *mut libc::c_int,
        trans_color: *mut png_color_16p,
    ) -> png_uint_32;
    fn png_set_keep_unknown_chunks(
        png_ptr: png_structrp,
        keep: libc::c_int,
        chunk_list: png_const_bytep,
        num_chunks: libc::c_int,
    );
    fn png_set_user_limits(
        png_ptr: png_structrp,
        user_width_max: png_uint_32,
        user_height_max: png_uint_32,
    );
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type fixed_t = int32_t;
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
pub type spritenum_t = sprite;
pub type sprite = libc::c_uint;
pub const NUMSPRITES: sprite = 907;
pub const SPR_LASTFREESLOT: sprite = 906;
pub const SPR_FIRSTFREESLOT: sprite = 395;
pub const SPR_GWLR: sprite = 394;
pub const SPR_GWLG: sprite = 393;
pub const SPR_BRIY: sprite = 392;
pub const SPR_BRIB: sprite = 391;
pub const SPR_BRIR: sprite = 390;
pub const SPR_WDDB: sprite = 389;
pub const SPR_BRIC: sprite = 388;
pub const SPR_GFZD: sprite = 387;
pub const SPR_ROIP: sprite = 386;
pub const SPR_ROIO: sprite = 385;
pub const SPR_ROIN: sprite = 384;
pub const SPR_ROIM: sprite = 383;
pub const SPR_ROIL: sprite = 382;
pub const SPR_ROIK: sprite = 381;
pub const SPR_ROIJ: sprite = 380;
pub const SPR_ROII: sprite = 379;
pub const SPR_ROIH: sprite = 378;
pub const SPR_ROIG: sprite = 377;
pub const SPR_ROIF: sprite = 376;
pub const SPR_ROIE: sprite = 375;
pub const SPR_ROID: sprite = 374;
pub const SPR_ROIC: sprite = 373;
pub const SPR_ROIB: sprite = 372;
pub const SPR_ROIA: sprite = 371;
pub const SPR_BMNB: sprite = 370;
pub const SPR_BOM4: sprite = 369;
pub const SPR_BOM3: sprite = 368;
pub const SPR_BOM2: sprite = 367;
pub const SPR_BOM1: sprite = 366;
pub const SPR_SPRK: sprite = 365;
pub const SPR_HBAT: sprite = 364;
pub const SPR_SBSK: sprite = 363;
pub const SPR_SBFL: sprite = 362;
pub const SPR_SBOB: sprite = 361;
pub const SPR_BAL2: sprite = 360;
pub const SPR_CACO: sprite = 359;
pub const SPR_HMCE: sprite = 358;
pub const SPR_FMCE: sprite = 357;
pub const SPR_BBUZ: sprite = 356;
pub const SPR_BUMB: sprite = 355;
pub const SPR_HIVE: sprite = 354;
pub const SPR_POPH: sprite = 353;
pub const SPR_PENG: sprite = 352;
pub const SPR_SHLP: sprite = 351;
pub const SPR_NTPN: sprite = 350;
pub const SPR_IDYA: sprite = 349;
pub const SPR_CAPS: sprite = 348;
pub const SPR_NPRU: sprite = 347;
pub const SPR_NSCR: sprite = 346;
pub const SPR_HOOP: sprite = 345;
pub const SPR_NBMP: sprite = 344;
pub const SPR_NSPK: sprite = 343;
pub const SPR_NDRN: sprite = 342;
pub const SPR_TOAD: sprite = 341;
pub const SPR_MUS2: sprite = 340;
pub const SPR_MUS1: sprite = 339;
pub const SPR_MAXE: sprite = 338;
pub const SPR_BFLM: sprite = 337;
pub const SPR_KOOP: sprite = 336;
pub const SPR_HAMM: sprite = 335;
pub const SPR_PUMA: sprite = 334;
pub const SPR_SHLL: sprite = 333;
pub const SPR_FBLL: sprite = 332;
pub const SPR_FFWR: sprite = 331;
pub const SPR_BGOM: sprite = 330;
pub const SPR_GOOM: sprite = 329;
pub const SPR_CPRK: sprite = 328;
pub const SPR_COIN: sprite = 327;
pub const SPR_TSCR: sprite = 326;
pub const SPR_TGRE: sprite = 325;
pub const SPR_TAUT: sprite = 324;
pub const SPR_PIKG: sprite = 323;
pub const SPR_PIKS: sprite = 322;
pub const SPR_PIKE: sprite = 321;
pub const SPR_PIKA: sprite = 320;
pub const SPR_PIKR: sprite = 319;
pub const SPR_PIKB: sprite = 318;
pub const SPR_RNGG: sprite = 317;
pub const SPR_RNGS: sprite = 316;
pub const SPR_RNGE: sprite = 315;
pub const SPR_RNGA: sprite = 314;
pub const SPR_RNGI: sprite = 313;
pub const SPR_RNGR: sprite = 312;
pub const SPR_RNGB: sprite = 311;
pub const SPR_RRNG: sprite = 310;
pub const SPR_LHRT: sprite = 309;
pub const SPR_CORK: sprite = 308;
pub const SPR_FNSF: sprite = 307;
pub const SPR_GFLG: sprite = 306;
pub const SPR_TTAG: sprite = 305;
pub const SPR_LCKN: sprite = 304;
pub const SPR_FLII: sprite = 303;
pub const SPR_DRWN: sprite = 302;
pub const SPR_SCOR: sprite = 301;
pub const SPR_PRTL: sprite = 300;
pub const SPR_SEED: sprite = 299;
pub const SPR_TFOG: sprite = 298;
pub const SPR_FPRT: sprite = 297;
pub const SPR_DUST: sprite = 296;
pub const SPR_WZAP: sprite = 295;
pub const SPR_BUBL: sprite = 294;
pub const SPR_SMOK: sprite = 293;
pub const SPR_SPLA: sprite = 292;
pub const SPR_LSPL: sprite = 291;
pub const SPR_SPLH: sprite = 290;
pub const SPR_SNO1: sprite = 289;
pub const SPR_RAIN: sprite = 288;
pub const SPR_BSTR: sprite = 287;
pub const SPR_BSTY: sprite = 286;
pub const SPR_SSWB: sprite = 285;
pub const SPR_SSWR: sprite = 284;
pub const SPR_SSWY: sprite = 283;
pub const SPR_BSPR: sprite = 282;
pub const SPR_RSPR: sprite = 281;
pub const SPR_YSPR: sprite = 280;
pub const SPR_SPRB: sprite = 279;
pub const SPR_SPRR: sprite = 278;
pub const SPR_SPRY: sprite = 277;
pub const SPR_BLON: sprite = 276;
pub const SPR_BUMP: sprite = 275;
pub const SPR_STEM: sprite = 274;
pub const SPR_FANS: sprite = 273;
pub const SPR_FS02: sprite = 272;
pub const SPR_FS01: sprite = 271;
pub const SPR_FL16: sprite = 270;
pub const SPR_FL15: sprite = 269;
pub const SPR_FL14: sprite = 268;
pub const SPR_FL13: sprite = 267;
pub const SPR_FL12: sprite = 266;
pub const SPR_FL11: sprite = 265;
pub const SPR_FL10: sprite = 264;
pub const SPR_FL09: sprite = 263;
pub const SPR_FL08: sprite = 262;
pub const SPR_FL07: sprite = 261;
pub const SPR_FL06: sprite = 260;
pub const SPR_FL05: sprite = 259;
pub const SPR_FL04: sprite = 258;
pub const SPR_FL03: sprite = 257;
pub const SPR_FL02: sprite = 256;
pub const SPR_FL01: sprite = 255;
pub const SPR_FBUB: sprite = 254;
pub const SPR_GOAL: sprite = 253;
pub const SPR_SSPK: sprite = 252;
pub const SPR_IVSP: sprite = 251;
pub const SPR_ZAPS: sprite = 250;
pub const SPR_BUBS: sprite = 249;
pub const SPR_FIRS: sprite = 248;
pub const SPR_PITY: sprite = 247;
pub const SPR_FORC: sprite = 246;
pub const SPR_ELEM: sprite = 245;
pub const SPR_MAGN: sprite = 244;
pub const SPR_WIND: sprite = 243;
pub const SPR_ARMB: sprite = 242;
pub const SPR_ARMF: sprite = 241;
pub const SPR_ARMA: sprite = 240;
pub const SPR_DBAL: sprite = 239;
pub const SPR_STLG: sprite = 238;
pub const SPR_BSZ8: sprite = 237;
pub const SPR_BSZ7: sprite = 236;
pub const SPR_BSZ6: sprite = 235;
pub const SPR_BSZ5: sprite = 234;
pub const SPR_BST6: sprite = 233;
pub const SPR_BST5: sprite = 232;
pub const SPR_BST4: sprite = 231;
pub const SPR_BST3: sprite = 230;
pub const SPR_BST2: sprite = 229;
pub const SPR_BST1: sprite = 228;
pub const SPR_BSZ3: sprite = 227;
pub const SPR_BSZ2: sprite = 226;
pub const SPR_BSZ1: sprite = 225;
pub const SPR_CFLM: sprite = 224;
pub const SPR_RCRY: sprite = 223;
pub const SPR_BGAR: sprite = 222;
pub const SPR_HHZM: sprite = 221;
pub const SPR_SHRM: sprite = 220;
pub const SPR_HHPL: sprite = 219;
pub const SPR_PUMK: sprite = 218;
pub const SPR_ROSY: sprite = 217;
pub const SPR_FHZI: sprite = 216;
pub const SPR_XMS6: sprite = 215;
pub const SPR_XMS5: sprite = 214;
pub const SPR_XMS4: sprite = 213;
pub const SPR_XMS3: sprite = 212;
pub const SPR_XMS2: sprite = 211;
pub const SPR_XMS1: sprite = 210;
pub const SPR_WVIN: sprite = 209;
pub const SPR_TFLO: sprite = 208;
pub const SPR_JPLA: sprite = 207;
pub const SPR_LFAL: sprite = 206;
pub const SPR_DFLM: sprite = 205;
pub const SPR_FLME: sprite = 204;
pub const SPR_STEA: sprite = 203;
pub const SPR_TRAI: sprite = 202;
pub const SPR_TRAE: sprite = 201;
pub const SPR_SALD: sprite = 200;
pub const SPR_MCSP: sprite = 199;
pub const SPR_MCRT: sprite = 198;
pub const SPR_ADST: sprite = 197;
pub const SPR_TAZD: sprite = 196;
pub const SPR_REMT: sprite = 195;
pub const SPR_BARR: sprite = 194;
pub const SPR_OILF: sprite = 193;
pub const SPR_OILL: sprite = 192;
pub const SPR_WWS3: sprite = 191;
pub const SPR_WWS2: sprite = 190;
pub const SPR_WWSG: sprite = 189;
pub const SPR_CACT: sprite = 188;
pub const SPR_STBL: sprite = 187;
pub const SPR_BTBL: sprite = 186;
pub const SPR_CABR: sprite = 185;
pub const SPR_CBBS: sprite = 184;
pub const SPR_CSTA: sprite = 183;
pub const SPR_CFLG: sprite = 182;
pub const SPR_CTRC: sprite = 181;
pub const SPR_FLMH: sprite = 180;
pub const SPR_CNDL: sprite = 179;
pub const SPR_CEZB: sprite = 178;
pub const SPR_PINE: sprite = 177;
pub const SPR_BANR: sprite = 176;
pub const SPR_BFBR: sprite = 175;
pub const SPR_SFBR: sprite = 174;
pub const SPR_RSPB: sprite = 173;
pub const SPR_YSPB: sprite = 172;
pub const SPR_BMCE: sprite = 171;
pub const SPR_SMCE: sprite = 170;
pub const SPR_BMCH: sprite = 169;
pub const SPR_SMCH: sprite = 168;
pub const SPR_ESTA: sprite = 167;
pub const SPR_FLAM: sprite = 166;
pub const SPR_CHAN: sprite = 165;
pub const SPR_LIBE: sprite = 164;
pub const SPR_DSTG: sprite = 163;
pub const SPR_ALGB: sprite = 162;
pub const SPR_ALGA: sprite = 161;
pub const SPR_KELP: sprite = 160;
pub const SPR_BCRY: sprite = 159;
pub const SPR_CORL: sprite = 158;
pub const SPR_DRIP: sprite = 157;
pub const SPR_SEWE: sprite = 156;
pub const SPR_GARG: sprite = 155;
pub const SPR_ALRM: sprite = 154;
pub const SPR_THZT: sprite = 153;
pub const SPR_FWR6: sprite = 152;
pub const SPR_FWR5: sprite = 151;
pub const SPR_THZP: sprite = 150;
pub const SPR_TRE6: sprite = 149;
pub const SPR_TRE5: sprite = 148;
pub const SPR_TRE4: sprite = 147;
pub const SPR_TRE3: sprite = 146;
pub const SPR_TRE2: sprite = 145;
pub const SPR_TRE1: sprite = 144;
pub const SPR_BUS3: sprite = 143;
pub const SPR_BUS2: sprite = 142;
pub const SPR_BUS1: sprite = 141;
pub const SPR_FWR4: sprite = 140;
pub const SPR_FWR3: sprite = 139;
pub const SPR_FWR2: sprite = 138;
pub const SPR_FWR1: sprite = 137;
pub const SPR_TUPF: sprite = 136;
pub const SPR_TUPL: sprite = 135;
pub const SPR_LETR: sprite = 134;
pub const SPR_CFIR: sprite = 133;
pub const SPR_AROW: sprite = 132;
pub const SPR_CBLL: sprite = 131;
pub const SPR_TRLS: sprite = 130;
pub const SPR_JBUL: sprite = 129;
pub const SPR_MINE: sprite = 128;
pub const SPR_ENRG: sprite = 127;
pub const SPR_TORP: sprite = 126;
pub const SPR_LASF: sprite = 125;
pub const SPR_LASR: sprite = 124;
pub const SPR_MISL: sprite = 123;
pub const SPR_TVZP: sprite = 122;
pub const SPR_TVBB: sprite = 121;
pub const SPR_TVFL: sprite = 120;
pub const SPR_TVTK: sprite = 119;
pub const SPR_TV1K: sprite = 118;
pub const SPR_TVRC: sprite = 117;
pub const SPR_TVGV: sprite = 116;
pub const SPR_TVMY: sprite = 115;
pub const SPR_TVMX: sprite = 114;
pub const SPR_TVEG: sprite = 113;
pub const SPR_TV1P: sprite = 112;
pub const SPR_TV1U: sprite = 111;
pub const SPR_TVIV: sprite = 110;
pub const SPR_TVSS: sprite = 109;
pub const SPR_TVEL: sprite = 108;
pub const SPR_TVWW: sprite = 107;
pub const SPR_TVAR: sprite = 106;
pub const SPR_TVFO: sprite = 105;
pub const SPR_TVAT: sprite = 104;
pub const SPR_TVPI: sprite = 103;
pub const SPR_TVRI: sprite = 102;
pub const SPR_TBRI: sprite = 101;
pub const SPR_TRRI: sprite = 100;
pub const SPR_XLTV: sprite = 99;
pub const SPR_MSTV: sprite = 98;
pub const SPR_PUMI: sprite = 97;
pub const SPR_BMNE: sprite = 96;
pub const SPR_STPT: sprite = 95;
pub const SPR_WSPB: sprite = 94;
pub const SPR_WSPK: sprite = 93;
pub const SPR_USPK: sprite = 92;
pub const SPR_TFLM: sprite = 91;
pub const SPR_SFLM: sprite = 90;
pub const SPR_SPIK: sprite = 89;
pub const SPR_SIGN: sprite = 88;
pub const SPR_BBLS: sprite = 87;
pub const SPR_SHRD: sprite = 86;
pub const SPR_CEMG: sprite = 85;
pub const SPR_EMBM: sprite = 84;
pub const SPR_NSTR: sprite = 83;
pub const SPR_NCHP: sprite = 82;
pub const SPR_SPHR: sprite = 81;
pub const SPR_BFLG: sprite = 80;
pub const SPR_RFLG: sprite = 79;
pub const SPR_TOKE: sprite = 78;
pub const SPR_TRNG: sprite = 77;
pub const SPR_RING: sprite = 76;
pub const SPR_MSCB: sprite = 75;
pub const SPR_MSCF: sprite = 74;
pub const SPR_METL: sprite = 73;
pub const SPR_MNPL: sprite = 72;
pub const SPR_NPLM: sprite = 71;
pub const SPR_TARG: sprite = 70;
pub const SPR_ELEC: sprite = 69;
pub const SPR_RCKT: sprite = 68;
pub const SPR_EGGT: sprite = 67;
pub const SPR_BMSL: sprite = 66;
pub const SPR_BGOO: sprite = 65;
pub const SPR_BRAK: sprite = 64;
pub const SPR_EGGR: sprite = 63;
pub const SPR_BARD: sprite = 62;
pub const SPR_BARX: sprite = 61;
pub const SPR_FSGN: sprite = 60;
pub const SPR_FBOM: sprite = 59;
pub const SPR_PROJ: sprite = 58;
pub const SPR_VWRE: sprite = 57;
pub const SPR_WHAT: sprite = 56;
pub const SPR_BRKN: sprite = 55;
pub const SPR_FANG: sprite = 54;
pub const SPR_EGR1: sprite = 53;
pub const SPR_EFIR: sprite = 52;
pub const SPR_EGGP: sprite = 51;
pub const SPR_SHCK: sprite = 50;
pub const SPR_FAKE: sprite = 49;
pub const SPR_SEBH: sprite = 48;
pub const SPR_EGGO: sprite = 47;
pub const SPR_GOOP: sprite = 46;
pub const SPR_TANK: sprite = 45;
pub const SPR_EGGN: sprite = 44;
pub const SPR_EGLZ: sprite = 43;
pub const SPR_EGGM: sprite = 42;
pub const SPR_JETF: sprite = 41;
pub const SPR_DRAB: sprite = 40;
pub const SPR_PTER: sprite = 39;
pub const SPR_PYRE: sprite = 38;
pub const SPR_CANG: sprite = 37;
pub const SPR_CANA: sprite = 36;
pub const SPR_UNID: sprite = 35;
pub const SPR_SSHL: sprite = 34;
pub const SPR_MNUD: sprite = 33;
pub const SPR_MNUS: sprite = 32;
pub const SPR_GSNH: sprite = 31;
pub const SPR_GSNL: sprite = 30;
pub const SPR_GSNP: sprite = 29;
pub const SPR_ESHI: sprite = 28;
pub const SPR_SPSH: sprite = 27;
pub const SPR_STAB: sprite = 26;
pub const SPR_CBFS: sprite = 25;
pub const SPR_ARCH: sprite = 24;
pub const SPR_PNTY: sprite = 23;
pub const SPR_VLTR: sprite = 22;
pub const SPR_SNLR: sprite = 21;
pub const SPR_JJAW: sprite = 20;
pub const SPR_CSPR: sprite = 19;
pub const SPR_CR2B: sprite = 18;
pub const SPR_CRAB: sprite = 17;
pub const SPR_SHRP: sprite = 16;
pub const SPR_TURR: sprite = 15;
pub const SPR_TRET: sprite = 14;
pub const SPR_SKIM: sprite = 13;
pub const SPR_DETN: sprite = 12;
pub const SPR_CCOM: sprite = 11;
pub const SPR_JETG: sprite = 10;
pub const SPR_JETB: sprite = 9;
pub const SPR_RBUZ: sprite = 8;
pub const SPR_BUZZ: sprite = 7;
pub const SPR_FISH: sprite = 6;
pub const SPR_SPOS: sprite = 5;
pub const SPR_POSS: sprite = 4;
pub const SPR_PLAY: sprite = 3;
pub const SPR_THOK: sprite = 2;
pub const SPR_UNKN: sprite = 1;
pub const SPR_NULL: sprite = 0;
pub type playersprite = libc::c_uint;
pub const NUMPLAYERSPRITES: playersprite = 128;
pub const SPR2_LASTFREESLOT: playersprite = 127;
pub const SPR2_FIRSTFREESLOT: playersprite = 60;
pub const SPR2_XTRA: playersprite = 59;
pub const SPR2_LIFE: playersprite = 58;
pub const SPR2_SIGN: playersprite = 57;
pub const SPR2_CNT4: playersprite = 56;
pub const SPR2_CNT3: playersprite = 55;
pub const SPR2_CNT2: playersprite = 54;
pub const SPR2_CNT1: playersprite = 53;
pub const SPR2_TALC: playersprite = 52;
pub const SPR2_TALB: playersprite = 51;
pub const SPR2_TALA: playersprite = 50;
pub const SPR2_TAL9: playersprite = 49;
pub const SPR2_TAL8: playersprite = 48;
pub const SPR2_TAL7: playersprite = 47;
pub const SPR2_TAL6: playersprite = 46;
pub const SPR2_TAL5: playersprite = 45;
pub const SPR2_TAL4: playersprite = 44;
pub const SPR2_TAL3: playersprite = 43;
pub const SPR2_TAL2: playersprite = 42;
pub const SPR2_TAL1: playersprite = 41;
pub const SPR2_TAL0: playersprite = 40;
pub const SPR2_NATK: playersprite = 39;
pub const SPR2_NPUL: playersprite = 38;
pub const SPR2_NSTN: playersprite = 37;
pub const SPR2_NDRL: playersprite = 36;
pub const SPR2_NFLY: playersprite = 35;
pub const SPR2_NFLT: playersprite = 34;
pub const SPR2_NSTD: playersprite = 33;
pub const SPR2_TRNS: playersprite = 32;
pub const SPR2_MLEL: playersprite = 31;
pub const SPR2_MLEE: playersprite = 30;
pub const SPR2_TWIN: playersprite = 29;
pub const SPR2_FIRE: playersprite = 28;
pub const SPR2_BNCE: playersprite = 27;
pub const SPR2_FRUN: playersprite = 26;
pub const SPR2_FLT: playersprite = 25;
pub const SPR2_CLMB: playersprite = 24;
pub const SPR2_CLNG: playersprite = 23;
pub const SPR2_LAND: playersprite = 22;
pub const SPR2_GLID: playersprite = 21;
pub const SPR2_TIRE: playersprite = 20;
pub const SPR2_SWIM: playersprite = 19;
pub const SPR2_FLY: playersprite = 18;
pub const SPR2_SPIN: playersprite = 17;
pub const SPR2_RIDE: playersprite = 16;
pub const SPR2_EDGE: playersprite = 15;
pub const SPR2_FALL: playersprite = 14;
pub const SPR2_SPNG: playersprite = 13;
pub const SPR2_JUMP: playersprite = 12;
pub const SPR2_GASP: playersprite = 11;
pub const SPR2_ROLL: playersprite = 10;
pub const SPR2_DRWN: playersprite = 9;
pub const SPR2_DEAD: playersprite = 8;
pub const SPR2_STUN: playersprite = 7;
pub const SPR2_PAIN: playersprite = 6;
pub const SPR2_DASH: playersprite = 5;
pub const SPR2_RUN: playersprite = 4;
pub const SPR2_SKID: playersprite = 3;
pub const SPR2_WALK: playersprite = 2;
pub const SPR2_WAIT: playersprite = 1;
pub const SPR2_STND: playersprite = 0;
pub type playersprite_t = playersprite;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct post_t {
    pub topdelta: uint8_t,
    pub length: uint8_t,
}
pub type column_t = post_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spriteframe_t {
    pub rotate: uint8_t,
    pub lumppat: [lumpnum_t; 16],
    pub lumpid: [size_t; 16],
    pub flip: uint16_t,
    pub rotated: [[*mut rotsprite_t; 16]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spritedef_t {
    pub numframes: size_t,
    pub spriteframes: *mut spriteframe_t,
}
pub type skinflags_t = libc::c_uint;
pub const SF_NOSHIELDABILITY: skinflags_t = 524288;
pub const SF_CANBUSTWALLS: skinflags_t = 262144;
pub const SF_NOSUPERJUMPBOOST: skinflags_t = 131072;
pub const SF_NOSUPERSPRITES: skinflags_t = 65536;
pub const SF_NONIGHTSSUPER: skinflags_t = 32768;
pub const SF_NONIGHTSROTATION: skinflags_t = 16384;
pub const SF_MULTIABILITY: skinflags_t = 8192;
pub const SF_FASTEDGE: skinflags_t = 4096;
pub const SF_DASHMODE: skinflags_t = 2048;
pub const SF_MACHINE: skinflags_t = 1024;
pub const SF_MARIODAMAGE: skinflags_t = 768;
pub const SF_STOMPDAMAGE: skinflags_t = 512;
pub const SF_NOJUMPDAMAGE: skinflags_t = 256;
pub const SF_NOJUMPSPIN: skinflags_t = 128;
pub const SF_RUNONWATER: skinflags_t = 64;
pub const SF_NOSPEEDADJUST: skinflags_t = 32;
pub const SF_NOSKID: skinflags_t = 16;
pub const SF_HIRES: skinflags_t = 8;
pub const SF_NOSPINDASHDUST: skinflags_t = 4;
pub const SF_NOSUPERSPIN: skinflags_t = 2;
pub const SF_SUPER: skinflags_t = 1;
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
pub type pictureflags_t = libc::c_uint;
pub const PICFLAGS_YFLIP: pictureflags_t = 2;
pub const PICFLAGS_XFLIP: pictureflags_t = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PICDEPTH_32BPP: C2RustUnnamed_0 = 32;
pub const PICDEPTH_16BPP: C2RustUnnamed_0 = 16;
pub const PICDEPTH_8BPP: C2RustUnnamed_0 = 8;
pub const PICDEPTH_NONE: C2RustUnnamed_0 = 0;
pub const PU_STATIC: C2RustUnnamed_1 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spriteframepivot_t {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spriteinfo_t {
    pub pivot: [spriteframepivot_t; 64],
    pub available: boolean,
}
pub type png_uint_32 = libc::c_uint;
pub type png_bytep = *mut png_byte;
pub type png_byte = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorlookup_t {
    pub init: boolean,
    pub palette: [RGBA_t; 256],
    pub table: [uint16_t; 65535],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_chunk_t {
    pub name: [libc::c_char; 4],
    pub data: *mut libc::c_void,
    pub size: size_t,
}
pub type png_infopp = *mut *mut png_info;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_structp = *mut png_struct;
pub type png_struct = png_struct_def;
pub type png_structpp = *mut *mut png_struct;
pub type png_bytepp = *mut *mut png_byte;
pub type png_structrp = *mut png_struct;
pub type png_const_inforp = *const png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_color_16p = *mut png_color_16;
pub type png_color_16 = png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: png_byte,
    pub red: png_uint_16,
    pub green: png_uint_16,
    pub blue: png_uint_16,
    pub gray: png_uint_16,
}
pub type png_uint_16 = libc::c_ushort;
pub type png_colorp = *mut png_color;
pub type png_color = png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: png_byte,
    pub green: png_byte,
    pub blue: png_byte,
}
pub type png_const_bytep = *const png_byte;
pub type png_unknown_chunkp = *mut png_unknown_chunk;
pub type png_unknown_chunk = png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [png_byte; 5],
    pub data: *mut png_byte,
    pub size: size_t,
    pub location: png_byte,
}
pub type png_voidp = *mut libc::c_void;
pub type png_user_chunk_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_unknown_chunkp) -> libc::c_int,
>;
pub type png_size_t = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_io_t {
    pub buffer: *const uint8_t,
    pub size: uint32_t,
    pub position: uint32_t,
}
pub type png_const_charp = *const libc::c_char;
pub type png_rw_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_bytep, size_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type png_longjmp_ptr = Option::<
    unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> (),
>;
pub type png_error_ptr = Option::<
    unsafe extern "C" fn(png_structp, png_const_charp) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skin_t {
    pub name: [libc::c_char; 17],
    pub wadnum: uint16_t,
    pub flags: skinflags_t,
    pub realname: [libc::c_char; 17],
    pub hudname: [libc::c_char; 17],
    pub supername: [libc::c_char; 23],
    pub ability: uint8_t,
    pub ability2: uint8_t,
    pub thokitem: int32_t,
    pub spinitem: int32_t,
    pub revitem: int32_t,
    pub followitem: int32_t,
    pub actionspd: fixed_t,
    pub mindash: fixed_t,
    pub maxdash: fixed_t,
    pub normalspeed: fixed_t,
    pub runspeed: fixed_t,
    pub thrustfactor: uint8_t,
    pub accelstart: uint8_t,
    pub acceleration: uint8_t,
    pub jumpfactor: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub spinheight: fixed_t,
    pub shieldscale: fixed_t,
    pub camerascale: fixed_t,
    pub starttranscolor: uint8_t,
    pub prefcolor: uint16_t,
    pub supercolor: uint16_t,
    pub prefoppositecolor: uint16_t,
    pub highresscale: fixed_t,
    pub contspeed: uint8_t,
    pub contangle: uint8_t,
    pub soundsid: [sfxenum_t; 21],
    pub sprites: [spritedef_t; 256],
    pub sprinfo: [spriteinfo_t; 256],
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
pub type restype_t = restype;
pub type restype = libc::c_uint;
pub const RET_UNKNOWN: restype = 5;
pub const RET_FOLDER: restype = 4;
pub const RET_PK3: restype = 3;
pub const RET_LUA: restype = 2;
pub const RET_SOC: restype = 1;
pub const RET_WAD: restype = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_1 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_1 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_1 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_1 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_1 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_1 = 51;
pub const PU_LEVEL: C2RustUnnamed_1 = 50;
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn R_Char2Frame(mut cn: libc::c_char) -> uint8_t {
    if cn as libc::c_int >= 'A' as i32 && cn as libc::c_int <= 'Z' as i32 {
        return (cn as libc::c_int - 'A' as i32) as uint8_t;
    }
    if cn as libc::c_int >= '0' as i32 && cn as libc::c_int <= '9' as i32 {
        return (cn as libc::c_int - '0' as i32 + 26 as libc::c_int) as uint8_t;
    }
    if cn as libc::c_int >= 'a' as i32 && cn as libc::c_int <= 'z' as i32 {
        return (cn as libc::c_int - 'a' as i32 + 36 as libc::c_int) as uint8_t;
    }
    if cn as libc::c_int == '!' as i32 {
        return 62 as libc::c_int as uint8_t;
    }
    if cn as libc::c_int == '@' as i32 {
        return 63 as libc::c_int as uint8_t;
    }
    return 255 as libc::c_int as uint8_t;
}
static mut imgbuf: [libc::c_uchar; 67108864] = [0; 67108864];
static mut png_colorlookup: colorlookup_t = colorlookup_t {
    init: 0,
    palette: [FColorRGBA { rgba: 0 }; 256],
    table: [0; 65535],
};
#[no_mangle]
pub unsafe extern "C" fn Picture_Convert(
    mut informat: pictureformat_t,
    mut picture: *mut libc::c_void,
    mut outformat: pictureformat_t,
    mut insize: size_t,
    mut outsize: *mut size_t,
    mut inwidth: int32_t,
    mut inheight: int32_t,
    mut inleftoffset: int32_t,
    mut intopoffset: int32_t,
    mut flags: pictureflags_t,
) -> *mut libc::c_void {
    if informat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_Convert: input format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if outformat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_Convert: output format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if informat as libc::c_uint == outformat as libc::c_uint {
        I_Error(
            b"Picture_Convert: input and output formats were the same!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if Picture_IsPatchFormat(outformat) != 0 {
        return Picture_PatchConvert(
            informat,
            picture,
            outformat,
            insize,
            outsize,
            inwidth as int16_t,
            inheight as int16_t,
            inleftoffset as int16_t,
            intopoffset as int16_t,
            flags,
        )
    } else if Picture_IsFlatFormat(outformat) != 0 {
        return Picture_FlatConvert(
            informat,
            picture,
            outformat,
            insize,
            outsize,
            inwidth as int16_t,
            inheight as int16_t,
            inleftoffset as int16_t,
            intopoffset as int16_t,
            flags,
        )
    } else {
        I_Error(
            b"Picture_Convert: unsupported input format!\0" as *const u8
                as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Picture_PatchConvert(
    mut informat: pictureformat_t,
    mut picture: *mut libc::c_void,
    mut outformat: pictureformat_t,
    mut insize: size_t,
    mut outsize: *mut size_t,
    mut inwidth: int16_t,
    mut inheight: int16_t,
    mut inleftoffset: int16_t,
    mut intopoffset: int16_t,
    mut flags: pictureflags_t,
) -> *mut libc::c_void {
    let mut x: int16_t = 0;
    let mut y: int16_t = 0;
    let mut img: *mut uint8_t = 0 as *mut uint8_t;
    let mut imgptr: *mut uint8_t = imgbuf.as_mut_ptr();
    let mut colpointers: *mut uint8_t = 0 as *mut uint8_t;
    let mut startofspan: *mut uint8_t = 0 as *mut uint8_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut inpatch: *mut patch_t = 0 as *mut patch_t;
    let mut inbpp: int32_t = Picture_FormatBPP(informat);
    if informat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_PatchConvert: input format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if outformat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_PatchConvert: output format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if informat as libc::c_uint == outformat as libc::c_uint {
        I_Error(
            b"Picture_PatchConvert: input and output formats were the same!\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if inbpp == PICDEPTH_NONE as libc::c_int {
        I_Error(
            b"Picture_PatchConvert: unknown input bits per pixel?!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if Picture_FormatBPP(outformat) == PICDEPTH_NONE as libc::c_int {
        I_Error(
            b"Picture_PatchConvert: unknown output bits per pixel?!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if Picture_IsPatchFormat(informat) != 0 {
        inpatch = picture as *mut patch_t;
        if Picture_IsDoomPatchFormat(informat) != 0 {
            let mut doompatch: *mut softwarepatch_t = picture as *mut softwarepatch_t;
            inwidth = (*doompatch).width;
            inheight = (*doompatch).height;
            inleftoffset = (*doompatch).leftoffset;
            intopoffset = (*doompatch).topoffset;
        } else {
            inwidth = (*inpatch).width;
            inheight = (*inpatch).height;
            inleftoffset = (*inpatch).leftoffset;
            intopoffset = (*inpatch).topoffset;
        }
    }
    let mut p_tmp: *mut int16_t = imgptr as *mut libc::c_void as *mut int16_t;
    let tv: int16_t = inwidth;
    memcpy(
        imgptr as *mut libc::c_void,
        &tv as *const int16_t as *const libc::c_void,
        ::core::mem::size_of::<int16_t>() as libc::c_ulong,
    );
    p_tmp = p_tmp.offset(1);
    p_tmp;
    imgptr = p_tmp as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_0: *mut int16_t = imgptr as *mut libc::c_void as *mut int16_t;
    let tv_0: int16_t = inheight;
    memcpy(
        imgptr as *mut libc::c_void,
        &tv_0 as *const int16_t as *const libc::c_void,
        ::core::mem::size_of::<int16_t>() as libc::c_ulong,
    );
    p_tmp_0 = p_tmp_0.offset(1);
    p_tmp_0;
    imgptr = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_1: *mut int16_t = imgptr as *mut libc::c_void as *mut int16_t;
    let tv_1: int16_t = inleftoffset;
    memcpy(
        imgptr as *mut libc::c_void,
        &tv_1 as *const int16_t as *const libc::c_void,
        ::core::mem::size_of::<int16_t>() as libc::c_ulong,
    );
    p_tmp_1 = p_tmp_1.offset(1);
    p_tmp_1;
    imgptr = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
    let mut p_tmp_2: *mut int16_t = imgptr as *mut libc::c_void as *mut int16_t;
    let tv_2: int16_t = intopoffset;
    memcpy(
        imgptr as *mut libc::c_void,
        &tv_2 as *const int16_t as *const libc::c_void,
        ::core::mem::size_of::<int16_t>() as libc::c_ulong,
    );
    p_tmp_2 = p_tmp_2.offset(1);
    p_tmp_2;
    imgptr = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
    colpointers = imgptr;
    imgptr = imgptr.offset((inwidth as libc::c_int * 4 as libc::c_int) as isize);
    x = 0 as libc::c_int as int16_t;
    while (x as libc::c_int) < inwidth as libc::c_int {
        let mut lastStartY: libc::c_int = 0 as libc::c_int;
        let mut spanSize: libc::c_int = 0 as libc::c_int;
        startofspan = 0 as *mut uint8_t;
        let mut p_tmp_3: *mut int32_t = colpointers as *mut libc::c_void as *mut int32_t;
        let tv_3: int32_t = imgptr.offset_from(imgbuf.as_mut_ptr()) as libc::c_long
            as int32_t;
        memcpy(
            colpointers as *mut libc::c_void,
            &tv_3 as *const int32_t as *const libc::c_void,
            ::core::mem::size_of::<int32_t>() as libc::c_ulong,
        );
        p_tmp_3 = p_tmp_3.offset(1);
        p_tmp_3;
        colpointers = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
        y = 0 as libc::c_int as int16_t;
        while (y as libc::c_int) < inheight as libc::c_int {
            let mut input: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut opaque: boolean = false_0 as libc::c_int;
            if Picture_IsPatchFormat(informat) != 0 {
                input = Picture_GetPatchPixel(
                    inpatch,
                    informat,
                    x as int32_t,
                    y as int32_t,
                    flags,
                );
            } else if Picture_IsFlatFormat(informat) != 0 {
                let mut offs: size_t = (y as libc::c_int * inwidth as libc::c_int
                    + x as libc::c_int) as size_t;
                match informat as libc::c_uint {
                    9 => {
                        input = (picture as *mut uint32_t).offset(offs as isize)
                            as *mut libc::c_void;
                    }
                    6 => {
                        input = (picture as *mut uint16_t).offset(offs as isize)
                            as *mut libc::c_void;
                    }
                    2 => {
                        input = (picture as *mut uint8_t).offset(offs as isize)
                            as *mut libc::c_void;
                    }
                    _ => {
                        I_Error(
                            b"Picture_PatchConvert: unsupported flat input format!\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
            } else {
                I_Error(
                    b"Picture_PatchConvert: unsupported input format!\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !input.is_null() {
                let mut alpha: uint8_t = 0xff as libc::c_int as uint8_t;
                if inbpp == PICDEPTH_32BPP as libc::c_int {
                    let mut px: RGBA_t = *(input as *mut RGBA_t);
                    alpha = px.s.alpha;
                } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                    let mut px_0: uint16_t = *(input as *mut uint16_t);
                    alpha = ((px_0 as libc::c_int & 0xff00 as libc::c_int)
                        >> 8 as libc::c_int) as uint8_t;
                } else if inbpp == PICDEPTH_8BPP as libc::c_int {
                    let mut px_1: uint8_t = *(input as *mut uint8_t);
                    if px_1 as libc::c_int == 255 as libc::c_int {
                        alpha = 0 as libc::c_int as uint8_t;
                    }
                }
                opaque = (alpha as libc::c_int > 1 as libc::c_int) as libc::c_int;
            }
            if opaque == 0 {
                if !startofspan.is_null() {
                    let mut p_tmp_4: *mut uint8_t = imgptr as *mut libc::c_void
                        as *mut uint8_t;
                    let tv_4: uint8_t = 0 as libc::c_int as uint8_t;
                    memcpy(
                        imgptr as *mut libc::c_void,
                        &tv_4 as *const uint8_t as *const libc::c_void,
                        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                    );
                    p_tmp_4 = p_tmp_4.offset(1);
                    p_tmp_4;
                    imgptr = p_tmp_4 as *mut libc::c_void as *mut uint8_t;
                }
                startofspan = 0 as *mut uint8_t;
            } else {
                if startofspan.is_null() || spanSize == 255 as libc::c_int {
                    let mut writeY: libc::c_int = y as libc::c_int;
                    if !startofspan.is_null() {
                        let mut p_tmp_5: *mut uint8_t = imgptr as *mut libc::c_void
                            as *mut uint8_t;
                        let tv_5: uint8_t = 0 as libc::c_int as uint8_t;
                        memcpy(
                            imgptr as *mut libc::c_void,
                            &tv_5 as *const uint8_t as *const libc::c_void,
                            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                        );
                        p_tmp_5 = p_tmp_5.offset(1);
                        p_tmp_5;
                        imgptr = p_tmp_5 as *mut libc::c_void as *mut uint8_t;
                    }
                    if y as libc::c_int > 254 as libc::c_int {
                        if lastStartY < 254 as libc::c_int {
                            let mut p_tmp_6: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_6: uint8_t = 254 as libc::c_int as uint8_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_6 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_6 = p_tmp_6.offset(1);
                            p_tmp_6;
                            imgptr = p_tmp_6 as *mut libc::c_void as *mut uint8_t;
                            let mut p_tmp_7: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_7: uint8_t = 0 as libc::c_int as uint8_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_7 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_7 = p_tmp_7.offset(1);
                            p_tmp_7;
                            imgptr = p_tmp_7 as *mut libc::c_void as *mut uint8_t;
                            imgptr = imgptr.offset(2 as libc::c_int as isize);
                            lastStartY = 254 as libc::c_int;
                        }
                        writeY = y as libc::c_int - lastStartY;
                        while writeY > 254 as libc::c_int {
                            let mut p_tmp_8: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_8: uint8_t = 254 as libc::c_int as uint8_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_8 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_8 = p_tmp_8.offset(1);
                            p_tmp_8;
                            imgptr = p_tmp_8 as *mut libc::c_void as *mut uint8_t;
                            let mut p_tmp_9: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_9: uint8_t = 0 as libc::c_int as uint8_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_9 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_9 = p_tmp_9.offset(1);
                            p_tmp_9;
                            imgptr = p_tmp_9 as *mut libc::c_void as *mut uint8_t;
                            imgptr = imgptr.offset(2 as libc::c_int as isize);
                            writeY -= 254 as libc::c_int;
                        }
                    }
                    startofspan = imgptr;
                    let mut p_tmp_10: *mut uint8_t = imgptr as *mut libc::c_void
                        as *mut uint8_t;
                    let tv_10: uint8_t = writeY as uint8_t;
                    memcpy(
                        imgptr as *mut libc::c_void,
                        &tv_10 as *const uint8_t as *const libc::c_void,
                        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                    );
                    p_tmp_10 = p_tmp_10.offset(1);
                    p_tmp_10;
                    imgptr = p_tmp_10 as *mut libc::c_void as *mut uint8_t;
                    imgptr = imgptr.offset(2 as libc::c_int as isize);
                    spanSize = 0 as libc::c_int;
                    lastStartY = y as libc::c_int;
                }
                match outformat as libc::c_uint {
                    8 | 10 => {
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut out: RGBA_t = *(input as *mut RGBA_t);
                            let mut p_tmp_11: *mut uint32_t = imgptr as *mut libc::c_void
                                as *mut uint32_t;
                            let tv_11: uint32_t = out.rgba;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_11 as *const uint32_t as *const libc::c_void,
                                ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                            );
                            p_tmp_11 = p_tmp_11.offset(1);
                            p_tmp_11;
                            imgptr = p_tmp_11 as *mut libc::c_void as *mut uint8_t;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            let mut out_0: RGBA_t = *pMasterPalette
                                .offset(
                                    (*(input as *mut uint16_t) as libc::c_int
                                        & 0xff as libc::c_int) as isize,
                                );
                            let mut p_tmp_12: *mut uint32_t = imgptr as *mut libc::c_void
                                as *mut uint32_t;
                            let tv_12: uint32_t = out_0.rgba;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_12 as *const uint32_t as *const libc::c_void,
                                ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                            );
                            p_tmp_12 = p_tmp_12.offset(1);
                            p_tmp_12;
                            imgptr = p_tmp_12 as *mut libc::c_void as *mut uint8_t;
                        } else {
                            let mut out_1: RGBA_t = *pMasterPalette
                                .offset(
                                    (*(input as *mut uint8_t) as libc::c_int
                                        & 0xff as libc::c_int) as isize,
                                );
                            let mut p_tmp_13: *mut uint32_t = imgptr as *mut libc::c_void
                                as *mut uint32_t;
                            let tv_13: uint32_t = out_1.rgba;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_13 as *const uint32_t as *const libc::c_void,
                                ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                            );
                            p_tmp_13 = p_tmp_13.offset(1);
                            p_tmp_13;
                            imgptr = p_tmp_13 as *mut libc::c_void as *mut uint8_t;
                        }
                    }
                    5 | 7 => {
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut in_0: RGBA_t = *(input as *mut RGBA_t);
                            let mut out_2: uint8_t = NearestPaletteColor(
                                in_0.s.red,
                                in_0.s.green,
                                in_0.s.blue,
                                0 as *mut RGBA_t,
                            );
                            let mut p_tmp_14: *mut uint16_t = imgptr as *mut libc::c_void
                                as *mut uint16_t;
                            let tv_14: uint16_t = (0xff00 as libc::c_int
                                | out_2 as libc::c_int) as uint16_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_14 as *const uint16_t as *const libc::c_void,
                                ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                            );
                            p_tmp_14 = p_tmp_14.offset(1);
                            p_tmp_14;
                            imgptr = p_tmp_14 as *mut libc::c_void as *mut uint8_t;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            let mut p_tmp_15: *mut uint16_t = imgptr as *mut libc::c_void
                                as *mut uint16_t;
                            let tv_15: uint16_t = *(input as *mut uint16_t);
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_15 as *const uint16_t as *const libc::c_void,
                                ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                            );
                            p_tmp_15 = p_tmp_15.offset(1);
                            p_tmp_15;
                            imgptr = p_tmp_15 as *mut libc::c_void as *mut uint8_t;
                        } else {
                            let mut p_tmp_16: *mut uint16_t = imgptr as *mut libc::c_void
                                as *mut uint16_t;
                            let tv_16: uint16_t = (0xff00 as libc::c_int
                                | *(input as *mut uint8_t) as libc::c_int) as uint16_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_16 as *const uint16_t as *const libc::c_void,
                                ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                            );
                            p_tmp_16 = p_tmp_16.offset(1);
                            p_tmp_16;
                            imgptr = p_tmp_16 as *mut libc::c_void as *mut uint8_t;
                        }
                    }
                    _ => {
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut in_1: RGBA_t = *(input as *mut RGBA_t);
                            let mut out_3: uint8_t = NearestPaletteColor(
                                in_1.s.red,
                                in_1.s.green,
                                in_1.s.blue,
                                0 as *mut RGBA_t,
                            );
                            let mut p_tmp_17: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_17: uint8_t = out_3;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_17 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_17 = p_tmp_17.offset(1);
                            p_tmp_17;
                            imgptr = p_tmp_17 as *mut libc::c_void as *mut uint8_t;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            let mut out_4: uint16_t = *(input as *mut uint16_t);
                            let mut p_tmp_18: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_18: uint8_t = (out_4 as libc::c_int
                                & 0xff as libc::c_int) as uint8_t;
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_18 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_18 = p_tmp_18.offset(1);
                            p_tmp_18;
                            imgptr = p_tmp_18 as *mut libc::c_void as *mut uint8_t;
                        } else {
                            let mut p_tmp_19: *mut uint8_t = imgptr as *mut libc::c_void
                                as *mut uint8_t;
                            let tv_19: uint8_t = *(input as *mut uint8_t);
                            memcpy(
                                imgptr as *mut libc::c_void,
                                &tv_19 as *const uint8_t as *const libc::c_void,
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            );
                            p_tmp_19 = p_tmp_19.offset(1);
                            p_tmp_19;
                            imgptr = p_tmp_19 as *mut libc::c_void as *mut uint8_t;
                        }
                    }
                }
                spanSize += 1;
                spanSize;
                *startofspan.offset(1 as libc::c_int as isize) = spanSize as uint8_t;
            }
            y += 1;
            y;
        }
        if !startofspan.is_null() {
            let mut p_tmp_20: *mut uint8_t = imgptr as *mut libc::c_void as *mut uint8_t;
            let tv_20: uint8_t = 0 as libc::c_int as uint8_t;
            memcpy(
                imgptr as *mut libc::c_void,
                &tv_20 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_20 = p_tmp_20.offset(1);
            p_tmp_20;
            imgptr = p_tmp_20 as *mut libc::c_void as *mut uint8_t;
        }
        let mut p_tmp_21: *mut uint8_t = imgptr as *mut libc::c_void as *mut uint8_t;
        let tv_21: uint8_t = 0xff as libc::c_int as uint8_t;
        memcpy(
            imgptr as *mut libc::c_void,
            &tv_21 as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp_21 = p_tmp_21.offset(1);
        p_tmp_21;
        imgptr = p_tmp_21 as *mut libc::c_void as *mut uint8_t;
        x += 1;
        x;
    }
    size = imgptr.offset_from(imgbuf.as_mut_ptr()) as libc::c_long as size_t;
    img = Z_MallocAlign(
        size,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut uint8_t;
    memcpy(img as *mut libc::c_void, imgbuf.as_mut_ptr() as *const libc::c_void, size);
    if Picture_IsInternalPatchFormat(outformat) != 0 {
        let mut converted: *mut patch_t = Patch_Create(
            img as *mut softwarepatch_t,
            size,
            0 as *mut libc::c_void,
        );
        Z_Free(img as *mut libc::c_void);
        if !outsize.is_null() {
            *outsize = ::core::mem::size_of::<patch_t>() as libc::c_ulong;
        }
        return converted as *mut libc::c_void;
    } else {
        if !outsize.is_null() {
            *outsize = size;
        }
        return img as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Picture_FlatConvert(
    mut informat: pictureformat_t,
    mut picture: *mut libc::c_void,
    mut outformat: pictureformat_t,
    mut insize: size_t,
    mut outsize: *mut size_t,
    mut inwidth: int16_t,
    mut inheight: int16_t,
    mut inleftoffset: int16_t,
    mut intopoffset: int16_t,
    mut flags: pictureflags_t,
) -> *mut libc::c_void {
    let mut outflat: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut inpatch: *mut patch_t = 0 as *mut patch_t;
    let mut inbpp: int32_t = Picture_FormatBPP(informat);
    let mut outbpp: int32_t = Picture_FormatBPP(outformat);
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut size: size_t = 0;
    if informat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_FlatConvert: input format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if outformat as libc::c_uint == PICFMT_NONE as libc::c_int as libc::c_uint {
        I_Error(
            b"Picture_FlatConvert: output format was PICFMT_NONE!\0" as *const u8
                as *const libc::c_char,
        );
    } else if informat as libc::c_uint == outformat as libc::c_uint {
        I_Error(
            b"Picture_FlatConvert: input and output formats were the same!\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if inbpp == PICDEPTH_NONE as libc::c_int {
        I_Error(
            b"Picture_FlatConvert: unknown input bits per pixel?!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if outbpp == PICDEPTH_NONE as libc::c_int {
        I_Error(
            b"Picture_FlatConvert: unknown output bits per pixel?!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if Picture_IsPatchFormat(informat) != 0 {
        inpatch = picture as *mut patch_t;
        if Picture_IsDoomPatchFormat(informat) != 0 {
            let mut doompatch: *mut softwarepatch_t = picture as *mut softwarepatch_t;
            inwidth = (*doompatch).width;
            inheight = (*doompatch).height;
        } else {
            inwidth = (*inpatch).width;
            inheight = (*inpatch).height;
        }
    }
    size = (inwidth as libc::c_int * inheight as libc::c_int
        * (outbpp / 8 as libc::c_int)) as size_t;
    outflat = Z_CallocAlign(
        size,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    if !outsize.is_null() {
        *outsize = size;
    }
    if outbpp == PICDEPTH_8BPP as libc::c_int {
        memset(outflat, 255 as libc::c_int, size);
    }
    y = 0 as libc::c_int;
    while y < inheight as libc::c_int {
        x = 0 as libc::c_int;
        while x < inwidth as libc::c_int {
            let mut input: *mut libc::c_void = 0 as *mut libc::c_void;
            let mut offs: size_t = (y * inwidth as libc::c_int + x) as size_t;
            if Picture_IsPatchFormat(informat) != 0 {
                input = Picture_GetPatchPixel(inpatch, informat, x, y, flags);
            } else if Picture_IsFlatFormat(informat) != 0 {
                input = (picture as *mut uint8_t)
                    .offset((offs * (inbpp / 8 as libc::c_int) as size_t) as isize)
                    as *mut libc::c_void;
            } else {
                I_Error(
                    b"Picture_FlatConvert: unsupported input format!\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !input.is_null() {
                match outformat as libc::c_uint {
                    9 => {
                        let mut f32: *mut uint32_t = outflat as *mut uint32_t;
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut out: RGBA_t = *(input as *mut RGBA_t);
                            *f32.offset(offs as isize) = out.rgba;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            let mut out_0: RGBA_t = *pMasterPalette
                                .offset(
                                    (*(input as *mut uint16_t) as libc::c_int
                                        & 0xff as libc::c_int) as isize,
                                );
                            *f32.offset(offs as isize) = out_0.rgba;
                        } else {
                            let mut out_1: RGBA_t = *pMasterPalette
                                .offset(
                                    (*(input as *mut uint8_t) as libc::c_int
                                        & 0xff as libc::c_int) as isize,
                                );
                            *f32.offset(offs as isize) = out_1.rgba;
                        }
                    }
                    6 => {
                        let mut f16: *mut uint16_t = outflat as *mut uint16_t;
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut in_0: RGBA_t = *(input as *mut RGBA_t);
                            let mut out_2: uint8_t = NearestPaletteColor(
                                in_0.s.red,
                                in_0.s.green,
                                in_0.s.blue,
                                0 as *mut RGBA_t,
                            );
                            *f16
                                .offset(
                                    offs as isize,
                                ) = (0xff00 as libc::c_int | out_2 as libc::c_int)
                                as uint16_t;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            *f16.offset(offs as isize) = *(input as *mut uint16_t);
                        } else {
                            *f16
                                .offset(
                                    offs as isize,
                                ) = (0xff00 as libc::c_int
                                | *(input as *mut uint8_t) as libc::c_int) as uint16_t;
                        }
                    }
                    2 => {
                        let mut f8: *mut uint8_t = outflat as *mut uint8_t;
                        if inbpp == PICDEPTH_32BPP as libc::c_int {
                            let mut in_1: RGBA_t = *(input as *mut RGBA_t);
                            let mut out_3: uint8_t = NearestPaletteColor(
                                in_1.s.red,
                                in_1.s.green,
                                in_1.s.blue,
                                0 as *mut RGBA_t,
                            );
                            *f8.offset(offs as isize) = out_3;
                        } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                            let mut out_4: uint16_t = *(input as *mut uint16_t);
                            *f8
                                .offset(
                                    offs as isize,
                                ) = (out_4 as libc::c_int & 0xff as libc::c_int) as uint8_t;
                        } else {
                            *f8.offset(offs as isize) = *(input as *mut uint8_t);
                        }
                    }
                    _ => {
                        I_Error(
                            b"Picture_FlatConvert: unsupported output format!\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            x += 1;
            x;
        }
        y += 1;
        y;
    }
    return outflat;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_GetPatchPixel(
    mut patch: *mut patch_t,
    mut informat: pictureformat_t,
    mut x: int32_t,
    mut y: int32_t,
    mut flags: pictureflags_t,
) -> *mut libc::c_void {
    let mut ofs: fixed_t = 0;
    let mut column: *mut column_t = 0 as *mut column_t;
    let mut inbpp: int32_t = Picture_FormatBPP(informat);
    let mut doompatch: *mut softwarepatch_t = patch as *mut softwarepatch_t;
    let mut isdoompatch: boolean = Picture_IsDoomPatchFormat(informat);
    let mut width: int16_t = 0;
    if patch.is_null() {
        I_Error(
            b"Picture_GetPatchPixel: patch == NULL\0" as *const u8 as *const libc::c_char,
        );
    }
    width = (if isdoompatch != 0 {
        (*doompatch).width as libc::c_int
    } else {
        (*patch).width as libc::c_int
    }) as int16_t;
    if x >= 0 as libc::c_int && x < width as libc::c_int {
        let mut colx: int32_t = if flags as libc::c_uint
            & PICFLAGS_XFLIP as libc::c_int as libc::c_uint != 0
        {
            width as libc::c_int - 1 as libc::c_int - x
        } else {
            x
        };
        let mut topdelta: int32_t = 0;
        let mut prevdelta: int32_t = -(1 as libc::c_int);
        let mut colofs: int32_t = if isdoompatch != 0 {
            (*doompatch).columnofs[colx as usize]
        } else {
            *((*patch).columnofs).offset(colx as isize)
        };
        if isdoompatch != 0 {
            column = (doompatch as *mut uint8_t).offset(colofs as isize)
                as *mut column_t;
        } else {
            column = ((*patch).columns).offset(colofs as isize) as *mut column_t;
        }
        while (*column).topdelta as libc::c_int != 0xff as libc::c_int {
            let mut s8: *mut uint8_t = 0 as *mut uint8_t;
            let mut s16: *mut uint16_t = 0 as *mut uint16_t;
            let mut s32: *mut uint32_t = 0 as *mut uint32_t;
            topdelta = (*column).topdelta as int32_t;
            if topdelta <= prevdelta {
                topdelta += prevdelta;
            }
            prevdelta = topdelta;
            ofs = y - topdelta;
            if y >= topdelta && ofs < (*column).length as libc::c_int {
                s8 = (column as *mut uint8_t).offset(3 as libc::c_int as isize);
                match inbpp {
                    32 => {
                        s32 = s8 as *mut uint32_t;
                        return &mut *s32.offset(ofs as isize) as *mut uint32_t
                            as *mut libc::c_void;
                    }
                    16 => {
                        s16 = s8 as *mut uint16_t;
                        return &mut *s16.offset(ofs as isize) as *mut uint16_t
                            as *mut libc::c_void;
                    }
                    _ => {
                        return &mut *s8.offset(ofs as isize) as *mut uint8_t
                            as *mut libc::c_void;
                    }
                }
            }
            if inbpp == PICDEPTH_32BPP as libc::c_int {
                column = (column as *mut uint32_t)
                    .offset((*column).length as libc::c_int as isize) as *mut column_t;
            } else if inbpp == PICDEPTH_16BPP as libc::c_int {
                column = (column as *mut uint16_t)
                    .offset((*column).length as libc::c_int as isize) as *mut column_t;
            } else {
                column = (column as *mut uint8_t)
                    .offset((*column).length as libc::c_int as isize) as *mut column_t;
            }
            column = (column as *mut uint8_t).offset(4 as libc::c_int as isize)
                as *mut column_t;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_FormatBPP(mut format: pictureformat_t) -> int32_t {
    let mut bpp: int32_t = PICDEPTH_NONE as libc::c_int;
    match format as libc::c_uint {
        8 | 9 | 10 | 4 => {
            bpp = PICDEPTH_32BPP as libc::c_int;
        }
        5 | 6 | 7 => {
            bpp = PICDEPTH_16BPP as libc::c_int;
        }
        1 | 2 | 3 => {
            bpp = PICDEPTH_8BPP as libc::c_int;
        }
        _ => {}
    }
    return bpp;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_IsPatchFormat(mut format: pictureformat_t) -> boolean {
    return (Picture_IsInternalPatchFormat(format) != 0
        || Picture_IsDoomPatchFormat(format) != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_IsInternalPatchFormat(
    mut format: pictureformat_t,
) -> boolean {
    match format as libc::c_uint {
        1 | 5 | 8 => return true_0 as libc::c_int,
        _ => return false_0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Picture_IsDoomPatchFormat(
    mut format: pictureformat_t,
) -> boolean {
    match format as libc::c_uint {
        3 | 7 | 10 => return true_0 as libc::c_int,
        _ => return false_0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn Picture_IsFlatFormat(mut format: pictureformat_t) -> boolean {
    return (format as libc::c_uint == PICFMT_FLAT as libc::c_int as libc::c_uint
        || format as libc::c_uint == PICFMT_FLAT16 as libc::c_int as libc::c_uint
        || format as libc::c_uint == PICFMT_FLAT32 as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_CheckIfDoomPatch(
    mut patch: *mut softwarepatch_t,
    mut size: size_t,
) -> boolean {
    let mut width: int16_t = 0;
    let mut height: int16_t = 0;
    let mut result: boolean = 0;
    if size < 13 as libc::c_int as size_t {
        return false_0 as libc::c_int;
    }
    width = (*patch).width;
    height = (*patch).height;
    result = (height as libc::c_int > 0 as libc::c_int
        && height as libc::c_int <= 16384 as libc::c_int
        && width as libc::c_int > 0 as libc::c_int
        && width as libc::c_int <= 16384 as libc::c_int) as libc::c_int;
    if result != 0 {
        let mut x: int16_t = 0;
        x = 0 as libc::c_int as int16_t;
        while (x as libc::c_int) < width as libc::c_int {
            let mut ofs: uint32_t = (*patch).columnofs[x as usize] as uint32_t;
            if ofs
                < (width as uint32_t * 4 as libc::c_int as uint32_t)
                    .wrapping_add(8 as libc::c_int as uint32_t)
                || ofs >= size as uint32_t
            {
                result = false_0 as libc::c_int;
                break;
            } else {
                x += 1;
                x;
            }
        }
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_TextureToFlat(
    mut trickytex: size_t,
) -> *mut libc::c_void {
    let mut texture: *mut texture_t = 0 as *mut texture_t;
    let mut tex: size_t = 0;
    let mut converted: *mut uint8_t = 0 as *mut uint8_t;
    let mut flatsize: size_t = 0;
    let mut col: fixed_t = 0;
    let mut ofs: fixed_t = 0;
    let mut column: *mut column_t = 0 as *mut column_t;
    let mut desttop: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *mut uint8_t = 0 as *mut uint8_t;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    if trickytex >= numtextures as libc::c_uint as size_t {
        I_Error(
            b"Picture_TextureToFlat: invalid texture number!\0" as *const u8
                as *const libc::c_char,
        );
    }
    tex = trickytex;
    texture = *textures.offset(tex as isize);
    R_CheckTextureCache(tex as int32_t);
    flatsize = ((*texture).width as libc::c_int * (*texture).height as libc::c_int)
        as size_t;
    converted = Z_MallocAlign(
        flatsize,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut uint8_t;
    memset(converted as *mut libc::c_void, 255 as libc::c_int, flatsize);
    desttop = converted;
    deststop = desttop.offset(flatsize as isize);
    col = 0 as libc::c_int;
    while col < (*texture).width as libc::c_int {
        if (*texture).holes == 0 {
            column = R_GetColumn(tex as fixed_t, col) as *mut column_t;
            source = column as *mut uint8_t;
            dest = desttop;
            ofs = 0 as libc::c_int;
            while dest < deststop && ofs < (*texture).height as libc::c_int {
                if *source.offset(ofs as isize) as libc::c_int != 255 as libc::c_int {
                    *dest = *source.offset(ofs as isize);
                }
                dest = dest.offset((*texture).width as libc::c_int as isize);
                ofs += 1;
                ofs;
            }
        } else {
            let mut topdelta: int32_t = 0;
            let mut prevdelta: int32_t = -(1 as libc::c_int);
            column = (R_GetColumn(tex as fixed_t, col))
                .offset(-(3 as libc::c_int as isize)) as *mut column_t;
            while (*column).topdelta as libc::c_int != 0xff as libc::c_int {
                topdelta = (*column).topdelta as int32_t;
                if topdelta <= prevdelta {
                    topdelta += prevdelta;
                }
                prevdelta = topdelta;
                dest = desttop
                    .offset((topdelta * (*texture).width as libc::c_int) as isize);
                source = (column as *mut uint8_t).offset(3 as libc::c_int as isize);
                ofs = 0 as libc::c_int;
                while dest < deststop && ofs < (*column).length as libc::c_int {
                    if *source.offset(ofs as isize) as libc::c_int != 255 as libc::c_int
                    {
                        *dest = *source.offset(ofs as isize);
                    }
                    dest = dest.offset((*texture).width as libc::c_int as isize);
                    ofs += 1;
                    ofs;
                }
                column = (column as *mut uint8_t)
                    .offset((*column).length as libc::c_int as isize)
                    .offset(4 as libc::c_int as isize) as *mut column_t;
            }
        }
        col += 1;
        col;
        desttop = desttop.offset(1);
        desttop;
    }
    return converted as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_IsLumpPNG(
    mut d: *const uint8_t,
    mut s: size_t,
) -> boolean {
    if s < 67 as libc::c_int as size_t {
        return false_0 as libc::c_int;
    }
    return (memcmp(
        &*d.offset(0 as libc::c_int as isize) as *const uint8_t as *const libc::c_void,
        b"\x89PNG\r\n\x1A\n\0" as *const u8 as *const libc::c_char
            as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn PNG_IOReader(
    mut png_ptr: png_structp,
    mut data: png_bytep,
    mut length: png_size_t,
) {
    let mut f: *mut png_io_t = png_get_io_ptr(png_ptr as *const png_struct)
        as *mut png_io_t;
    if length > ((*f).size).wrapping_sub((*f).position) as png_size_t {
        png_error(
            png_ptr as *const png_struct,
            b"PNG_IOReader: buffer overrun\0" as *const u8 as *const libc::c_char,
        );
    }
    memcpy(
        data as *mut libc::c_void,
        ((*f).buffer).offset((*f).position as isize) as *const libc::c_void,
        length,
    );
    (*f)
        .position = ((*f).position as png_size_t).wrapping_add(length) as uint32_t
        as uint32_t;
}
static mut chunkname: *mut png_byte = 0 as *const png_byte as *mut png_byte;
static mut chunk: png_chunk_t = png_chunk_t {
    name: [0; 4],
    data: 0 as *const libc::c_void as *mut libc::c_void,
    size: 0,
};
unsafe extern "C" fn PNG_ChunkReader(
    mut png_ptr: png_structp,
    mut chonk: png_unknown_chunkp,
) -> libc::c_int {
    if memcmp(
        ((*chonk).name).as_mut_ptr() as *const libc::c_void,
        chunkname as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        memcpy(
            (chunk.name).as_mut_ptr() as *mut libc::c_void,
            ((*chonk).name).as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        chunk.size = (*chonk).size;
        chunk
            .data = Z_MallocAlign(
            chunk.size,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        );
        memcpy(chunk.data, (*chonk).data as *const libc::c_void, chunk.size);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn PNG_error(mut PNG: png_structp, mut pngtext: png_const_charp) {
    CONS_Debug(
        0x8 as libc::c_int,
        b"libpng error at %p: %s\0" as *const u8 as *const libc::c_char,
        PNG,
        pngtext,
    );
}
unsafe extern "C" fn PNG_warn(mut PNG: png_structp, mut pngtext: png_const_charp) {
    CONS_Debug(
        0x8 as libc::c_int,
        b"libpng warning at %p: %s\0" as *const u8 as *const libc::c_char,
        PNG,
        pngtext,
    );
}
static mut grAb_chunk: [png_byte; 5] = [
    'g' as i32 as png_byte,
    'r' as i32 as png_byte,
    'A' as i32 as png_byte,
    'b' as i32 as png_byte,
    '\0' as i32 as png_byte,
];
unsafe extern "C" fn PNG_Read(
    mut png: *const uint8_t,
    mut w: *mut int32_t,
    mut h: *mut int32_t,
    mut topoffset: *mut int16_t,
    mut leftoffset: *mut int16_t,
    mut use_palette: *mut boolean,
    mut size: size_t,
) -> *mut png_bytep {
    let mut png_ptr: png_structp = 0 as *mut png_struct;
    let mut png_info_ptr: png_infop = 0 as *mut png_info;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut y: png_uint_32 = 0;
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut palette_size: libc::c_int = 0;
    let mut trans: png_bytep = 0 as png_bytep;
    let mut num_trans: libc::c_int = 0 as libc::c_int;
    let mut png_io: png_io_t = png_io_t {
        buffer: 0 as *const uint8_t,
        size: 0,
        position: 0,
    };
    let mut row_pointers: *mut png_bytep = 0 as *mut png_bytep;
    let mut user_chunk_ptr: *mut png_voidp = 0 as *mut png_voidp;
    png_ptr = png_create_read_struct(
        b"1.6.43\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        Some(PNG_error as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
        Some(PNG_warn as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
    );
    if png_ptr.is_null() {
        I_Error(
            b"PNG_Read: Couldn't initialize libpng!\0" as *const u8
                as *const libc::c_char,
        );
    }
    png_info_ptr = png_create_info_struct(png_ptr as *const png_struct);
    if png_info_ptr.is_null() {
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
        I_Error(
            b"PNG_Read: libpng couldn't allocate memory!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _setjmp(
        (*png_set_longjmp_fn(
            png_ptr,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !>,
                png_longjmp_ptr,
            >(
                Some(
                    longjmp as unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                ),
            ),
            ::core::mem::size_of::<jmp_buf>() as libc::c_ulong,
        ))
            .as_mut_ptr(),
    ) != 0
    {
        png_destroy_read_struct(&mut png_ptr, &mut png_info_ptr, 0 as png_infopp);
        I_Error(b"PNG_Read: libpng load error!\0" as *const u8 as *const libc::c_char);
    }
    png_io.buffer = png;
    png_io.size = size as uint32_t;
    png_io.position = 0 as libc::c_int as uint32_t;
    png_set_read_fn(
        png_ptr,
        &mut png_io as *mut png_io_t as png_voidp,
        Some(
            PNG_IOReader
                as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
        ),
    );
    memset(
        &mut chunk as *mut png_chunk_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<png_chunk_t>() as libc::c_ulong,
    );
    chunkname = grAb_chunk.as_mut_ptr();
    user_chunk_ptr = png_get_user_chunk_ptr(png_ptr as *const png_struct)
        as *mut png_voidp;
    png_set_read_user_chunk_fn(
        png_ptr,
        user_chunk_ptr as png_voidp,
        Some(
            PNG_ChunkReader
                as unsafe extern "C" fn(png_structp, png_unknown_chunkp) -> libc::c_int,
        ),
    );
    png_set_keep_unknown_chunks(
        png_ptr,
        2 as libc::c_int,
        chunkname as png_const_bytep,
        1 as libc::c_int,
    );
    png_set_user_limits(
        png_ptr,
        2048 as libc::c_int as png_uint_32,
        2048 as libc::c_int as png_uint_32,
    );
    png_read_info(png_ptr, png_info_ptr);
    png_get_IHDR(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    );
    if bit_depth == 16 as libc::c_int {
        png_set_strip_16(png_ptr);
    }
    palette = 0 as png_colorp;
    *use_palette = false_0 as libc::c_int;
    if color_type == 0 as libc::c_int || color_type == 4 as libc::c_int {
        png_set_gray_to_rgb(png_ptr);
    } else if color_type == 2 as libc::c_int | 1 as libc::c_int {
        let mut usepal: boolean = false_0 as libc::c_int;
        if png_get_PLTE(
            png_ptr as *const png_struct,
            png_info_ptr,
            &mut palette,
            &mut palette_size,
        ) != 0
        {
            if palette_size == 256 as libc::c_int && !pMasterPalette.is_null() {
                let mut pal: png_colorp = palette;
                let mut i: int32_t = 0;
                usepal = true_0 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 256 as libc::c_int {
                    let mut curpal: *mut byteColor_t = &mut (*pMasterPalette
                        .offset(i as isize))
                        .s;
                    if (*pal).red as libc::c_int != (*curpal).red as libc::c_int
                        || (*pal).green as libc::c_int != (*curpal).green as libc::c_int
                        || (*pal).blue as libc::c_int != (*curpal).blue as libc::c_int
                    {
                        usepal = false_0 as libc::c_int;
                        break;
                    } else {
                        pal = pal.offset(1);
                        pal;
                        i += 1;
                        i;
                    }
                }
            }
        }
        if usepal != 0 {
            let mut result: png_uint_32 = png_get_tRNS(
                png_ptr as *const png_struct,
                png_info_ptr,
                &mut trans,
                &mut num_trans,
                0 as *mut png_color_16p,
            );
            if result & 0x10 as libc::c_uint != 0 && num_trans > 0 as libc::c_int
                && !trans.is_null()
            {
                let mut i_0: int32_t = 0;
                i_0 = 0 as libc::c_int;
                while i_0 < num_trans {
                    if (*trans.offset(i_0 as isize) as libc::c_int) < 0xff as libc::c_int
                    {
                        usepal = false_0 as libc::c_int;
                        break;
                    } else {
                        i_0 += 1;
                        i_0;
                    }
                }
            }
        }
        if usepal != 0 {
            *use_palette = true_0 as libc::c_int;
        } else {
            png_set_palette_to_rgb(png_ptr);
        }
    }
    if png_get_valid(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
        0x10 as libc::c_uint,
    ) != 0
    {
        png_set_tRNS_to_alpha(png_ptr);
    } else if color_type != 2 as libc::c_int | 4 as libc::c_int
        && color_type != 4 as libc::c_int
    {
        png_set_add_alpha(png_ptr, 0xff as libc::c_int as png_uint_32, 1 as libc::c_int);
    }
    png_read_update_info(png_ptr, png_info_ptr);
    row_pointers = malloc(
        (::core::mem::size_of::<png_bytep>() as libc::c_ulong)
            .wrapping_mul(height as libc::c_ulong),
    ) as *mut png_bytep;
    y = 0 as libc::c_int as png_uint_32;
    while y < height {
        let ref mut fresh0 = *row_pointers.offset(y as isize);
        *fresh0 = malloc(
            png_get_rowbytes(
                png_ptr as *const png_struct,
                png_info_ptr as *const png_info,
            ),
        ) as *mut png_byte;
        y = y.wrapping_add(1);
        y;
    }
    png_read_image(png_ptr, row_pointers);
    if (!topoffset.is_null() || !leftoffset.is_null()) && !(chunk.data).is_null() {
        let mut offsets: *mut int32_t = chunk.data as *mut int32_t;
        if !leftoffset.is_null() {
            *leftoffset = ((*offsets >> 24 as libc::c_int & 0xff as libc::c_int
                | *offsets << 8 as libc::c_int & 0xff0000 as libc::c_int
                | *offsets >> 8 as libc::c_int & 0xff00 as libc::c_int) as libc::c_uint
                | (*offsets << 24 as libc::c_int) as libc::c_uint
                    & 0xff000000 as libc::c_uint) as int16_t;
        }
        offsets = offsets.offset(1);
        offsets;
        if !topoffset.is_null() {
            *topoffset = ((*offsets >> 24 as libc::c_int & 0xff as libc::c_int
                | *offsets << 8 as libc::c_int & 0xff0000 as libc::c_int
                | *offsets >> 8 as libc::c_int & 0xff00 as libc::c_int) as libc::c_uint
                | (*offsets << 24 as libc::c_int) as libc::c_uint
                    & 0xff000000 as libc::c_uint) as int16_t;
        }
    }
    png_destroy_read_struct(&mut png_ptr, &mut png_info_ptr, 0 as png_infopp);
    if !(chunk.data).is_null() {
        Z_Free(chunk.data);
    }
    *w = width as int32_t;
    *h = height as int32_t;
    return row_pointers;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_PNGConvert(
    mut png: *const uint8_t,
    mut outformat: pictureformat_t,
    mut w: *mut int32_t,
    mut h: *mut int32_t,
    mut topoffset: *mut int16_t,
    mut leftoffset: *mut int16_t,
    mut insize: size_t,
    mut outsize: *mut size_t,
    mut flags: pictureflags_t,
) -> *mut libc::c_void {
    let mut flat: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut outbpp: int32_t = 0;
    let mut flatsize: size_t = 0;
    let mut x: png_uint_32 = 0;
    let mut y: png_uint_32 = 0;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut palette: boolean = false_0 as libc::c_int;
    let mut row_pointers: *mut png_bytep = 0 as *mut png_bytep;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut pngwidth: int32_t = 0;
    let mut pngheight: int32_t = 0;
    let mut loffs: int16_t = 0 as libc::c_int as int16_t;
    let mut toffs: int16_t = 0 as libc::c_int as int16_t;
    if png.is_null() {
        I_Error(
            b"Picture_PNGConvert: picture was NULL!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if w.is_null() {
        w = &mut pngwidth;
    }
    if h.is_null() {
        h = &mut pngheight;
    }
    if topoffset.is_null() {
        topoffset = &mut toffs;
    }
    if leftoffset.is_null() {
        leftoffset = &mut loffs;
    }
    row_pointers = PNG_Read(png, w, h, topoffset, leftoffset, &mut palette, insize);
    width = *w as png_uint_32;
    height = *h as png_uint_32;
    if row_pointers.is_null() {
        I_Error(
            b"Picture_PNGConvert: row_pointers was NULL!\0" as *const u8
                as *const libc::c_char,
        );
    }
    outbpp = Picture_FormatBPP(outformat);
    if Picture_IsPatchFormat(outformat) != 0 {
        if outbpp == PICDEPTH_8BPP as libc::c_int {
            outbpp = PICDEPTH_16BPP as libc::c_int;
        }
    }
    if outbpp == PICDEPTH_NONE as libc::c_int {
        I_Error(
            b"Picture_PNGConvert: unknown output bits per pixel?!\0" as *const u8
                as *const libc::c_char,
        );
    }
    flatsize = (width * height * (outbpp / 8 as libc::c_int) as png_uint_32) as size_t;
    if !outsize.is_null() {
        *outsize = flatsize;
    }
    flat = Z_CallocAlign(
        flatsize,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    );
    if outbpp == PICDEPTH_8BPP as libc::c_int {
        memset(flat, 255 as libc::c_int, (width * height) as libc::c_ulong);
    }
    if outbpp != PICDEPTH_32BPP as libc::c_int {
        InitColorLUT(&mut png_colorlookup, pMasterPalette, false_0 as libc::c_int);
    }
    if outbpp == PICDEPTH_32BPP as libc::c_int {
        let mut out: RGBA_t = FColorRGBA { rgba: 0 };
        let mut outflat: *mut uint32_t = flat as *mut uint32_t;
        if palette != 0 {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    out = *pLocalPalette
                        .offset(
                            (*row.offset(x as isize) as libc::c_int
                                & 0xff as libc::c_int) as isize,
                        );
                    *outflat.offset((y * width).wrapping_add(x) as isize) = out.rgba;
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        } else {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    let mut px: png_bytep = &mut *row
                        .offset((x * 4 as libc::c_int as png_uint_32) as isize)
                        as *mut png_byte;
                    if *px.offset(3 as libc::c_int as isize) != 0 {
                        out.s.red = *px.offset(0 as libc::c_int as isize);
                        out.s.green = *px.offset(1 as libc::c_int as isize);
                        out.s.blue = *px.offset(2 as libc::c_int as isize);
                        out.s.alpha = *px.offset(3 as libc::c_int as isize);
                        *outflat.offset((y * width).wrapping_add(x) as isize) = out.rgba;
                    } else {
                        *outflat
                            .offset(
                                (y * width).wrapping_add(x) as isize,
                            ) = 0 as libc::c_int as uint32_t;
                    }
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        }
    } else if outbpp == PICDEPTH_16BPP as libc::c_int {
        let mut outflat_0: *mut uint16_t = flat as *mut uint16_t;
        if palette != 0 {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    *outflat_0
                        .offset(
                            (y * width).wrapping_add(x) as isize,
                        ) = ((0xff as libc::c_int) << 8 as libc::c_int
                        | *row.offset(x as isize) as libc::c_int) as uint16_t;
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        } else {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    let mut px_0: png_bytep = &mut *row
                        .offset((x * 4 as libc::c_int as png_uint_32) as isize)
                        as *mut png_byte;
                    let mut red: uint8_t = *px_0.offset(0 as libc::c_int as isize);
                    let mut green: uint8_t = *px_0.offset(1 as libc::c_int as isize);
                    let mut blue: uint8_t = *px_0.offset(2 as libc::c_int as isize);
                    let mut alpha: uint8_t = *px_0.offset(3 as libc::c_int as isize);
                    if alpha != 0 {
                        let mut palidx: uint8_t = GetColorLUT(
                            &mut png_colorlookup,
                            red,
                            green,
                            blue,
                        );
                        *outflat_0
                            .offset(
                                (y * width).wrapping_add(x) as isize,
                            ) = ((0xff as libc::c_int) << 8 as libc::c_int
                            | palidx as libc::c_int) as uint16_t;
                    } else {
                        *outflat_0
                            .offset(
                                (y * width).wrapping_add(x) as isize,
                            ) = 0 as libc::c_int as uint16_t;
                    }
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        }
    } else {
        let mut outflat_1: *mut uint8_t = flat as *mut uint8_t;
        if palette != 0 {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    *outflat_1
                        .offset(
                            (y * width).wrapping_add(x) as isize,
                        ) = *row.offset(x as isize);
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        } else {
            y = 0 as libc::c_int as png_uint_32;
            while y < height {
                row = *row_pointers.offset(y as isize);
                x = 0 as libc::c_int as png_uint_32;
                while x < width {
                    let mut px_1: png_bytep = &mut *row
                        .offset((x * 4 as libc::c_int as png_uint_32) as isize)
                        as *mut png_byte;
                    let mut red_0: uint8_t = *px_1.offset(0 as libc::c_int as isize);
                    let mut green_0: uint8_t = *px_1.offset(1 as libc::c_int as isize);
                    let mut blue_0: uint8_t = *px_1.offset(2 as libc::c_int as isize);
                    let mut alpha_0: uint8_t = *px_1.offset(3 as libc::c_int as isize);
                    if alpha_0 != 0 {
                        let mut palidx_0: uint8_t = GetColorLUT(
                            &mut png_colorlookup,
                            red_0,
                            green_0,
                            blue_0,
                        );
                        *outflat_1
                            .offset((y * width).wrapping_add(x) as isize) = palidx_0;
                    }
                    x = x.wrapping_add(1);
                    x;
                }
                y = y.wrapping_add(1);
                y;
            }
        }
    }
    y = 0 as libc::c_int as png_uint_32;
    while y < height {
        free(*row_pointers.offset(y as isize) as *mut libc::c_void);
        y = y.wrapping_add(1);
        y;
    }
    free(row_pointers as *mut libc::c_void);
    if Picture_IsPatchFormat(outformat) != 0 {
        let mut converted: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut informat: pictureformat_t = PICFMT_NONE;
        match outbpp {
            32 => {
                informat = PICFMT_FLAT32;
            }
            16 => {
                informat = PICFMT_FLAT16;
            }
            _ => {
                informat = PICFMT_FLAT;
            }
        }
        converted = Picture_PatchConvert(
            informat,
            flat,
            outformat,
            insize,
            outsize,
            width as int16_t,
            height as int16_t,
            *leftoffset,
            *topoffset,
            flags,
        );
        Z_Free(flat);
        return converted;
    }
    return flat;
}
#[no_mangle]
pub unsafe extern "C" fn Picture_PNGDimensions(
    mut png: *mut uint8_t,
    mut width: *mut int32_t,
    mut height: *mut int32_t,
    mut topoffset: *mut int16_t,
    mut leftoffset: *mut int16_t,
    mut size: size_t,
) -> boolean {
    let mut png_ptr: png_structp = 0 as *mut png_struct;
    let mut png_info_ptr: png_infop = 0 as *mut png_info;
    let mut w: png_uint_32 = 0;
    let mut h: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut png_io: png_io_t = png_io_t {
        buffer: 0 as *const uint8_t,
        size: 0,
        position: 0,
    };
    let mut user_chunk_ptr: *mut png_voidp = 0 as *mut png_voidp;
    png_ptr = png_create_read_struct(
        b"1.6.43\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        Some(PNG_error as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
        Some(PNG_warn as unsafe extern "C" fn(png_structp, png_const_charp) -> ()),
    );
    if png_ptr.is_null() {
        I_Error(
            b"Picture_PNGDimensions: Couldn't initialize libpng!\0" as *const u8
                as *const libc::c_char,
        );
    }
    png_info_ptr = png_create_info_struct(png_ptr as *const png_struct);
    if png_info_ptr.is_null() {
        png_destroy_read_struct(&mut png_ptr, 0 as png_infopp, 0 as png_infopp);
        I_Error(
            b"Picture_PNGDimensions: libpng couldn't allocate memory!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if _setjmp(
        (*png_set_longjmp_fn(
            png_ptr,
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !>,
                png_longjmp_ptr,
            >(
                Some(
                    longjmp as unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                ),
            ),
            ::core::mem::size_of::<jmp_buf>() as libc::c_ulong,
        ))
            .as_mut_ptr(),
    ) != 0
    {
        png_destroy_read_struct(&mut png_ptr, &mut png_info_ptr, 0 as png_infopp);
        I_Error(
            b"Picture_PNGDimensions: libpng load error!\0" as *const u8
                as *const libc::c_char,
        );
    }
    png_io.buffer = png;
    png_io.size = size as uint32_t;
    png_io.position = 0 as libc::c_int as uint32_t;
    png_set_read_fn(
        png_ptr,
        &mut png_io as *mut png_io_t as png_voidp,
        Some(
            PNG_IOReader
                as unsafe extern "C" fn(png_structp, png_bytep, png_size_t) -> (),
        ),
    );
    memset(
        &mut chunk as *mut png_chunk_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<png_chunk_t>() as libc::c_ulong,
    );
    chunkname = grAb_chunk.as_mut_ptr();
    user_chunk_ptr = png_get_user_chunk_ptr(png_ptr as *const png_struct)
        as *mut png_voidp;
    png_set_read_user_chunk_fn(
        png_ptr,
        user_chunk_ptr as png_voidp,
        Some(
            PNG_ChunkReader
                as unsafe extern "C" fn(png_structp, png_unknown_chunkp) -> libc::c_int,
        ),
    );
    png_set_keep_unknown_chunks(
        png_ptr,
        2 as libc::c_int,
        chunkname as png_const_bytep,
        1 as libc::c_int,
    );
    png_set_user_limits(
        png_ptr,
        2048 as libc::c_int as png_uint_32,
        2048 as libc::c_int as png_uint_32,
    );
    png_read_info(png_ptr, png_info_ptr);
    png_get_IHDR(
        png_ptr as *const png_struct,
        png_info_ptr as *const png_info,
        &mut w,
        &mut h,
        &mut bit_depth,
        &mut color_type,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    );
    if (!topoffset.is_null() || !leftoffset.is_null()) && !(chunk.data).is_null() {
        let mut offsets: *mut int32_t = chunk.data as *mut int32_t;
        if !leftoffset.is_null() {
            *leftoffset = ((*offsets >> 24 as libc::c_int & 0xff as libc::c_int
                | *offsets << 8 as libc::c_int & 0xff0000 as libc::c_int
                | *offsets >> 8 as libc::c_int & 0xff00 as libc::c_int) as libc::c_uint
                | (*offsets << 24 as libc::c_int) as libc::c_uint
                    & 0xff000000 as libc::c_uint) as int16_t;
        }
        offsets = offsets.offset(1);
        offsets;
        if !topoffset.is_null() {
            *topoffset = ((*offsets >> 24 as libc::c_int & 0xff as libc::c_int
                | *offsets << 8 as libc::c_int & 0xff0000 as libc::c_int
                | *offsets >> 8 as libc::c_int & 0xff00 as libc::c_int) as libc::c_uint
                | (*offsets << 24 as libc::c_int) as libc::c_uint
                    & 0xff000000 as libc::c_uint) as int16_t;
        }
    }
    png_destroy_read_struct(&mut png_ptr, &mut png_info_ptr, 0 as png_infopp);
    if !(chunk.data).is_null() {
        Z_Free(chunk.data);
    }
    *width = w as int32_t;
    *height = h as int32_t;
    return true_0 as libc::c_int;
}
unsafe extern "C" fn R_ParseSpriteInfoFrame(mut info: *mut spriteinfo_t) {
    let mut sprinfoToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sprinfoTokenLength: size_t = 0;
    let mut frameChar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut frameFrame: uint8_t = 0xff as libc::c_int as uint8_t;
    let mut frameXPivot: int16_t = 0 as libc::c_int as int16_t;
    let mut frameYPivot: int16_t = 0 as libc::c_int as int16_t;
    sprinfoToken = M_GetToken(0 as *const libc::c_char);
    if sprinfoToken.is_null() {
        I_Error(
            b"Error parsing SPRTINFO lump: Unexpected end of file where sprite frame should be\0"
                as *const u8 as *const libc::c_char,
        );
    }
    sprinfoTokenLength = strlen(sprinfoToken);
    if sprinfoTokenLength != 1 as libc::c_int as size_t {
        I_Error(
            b"Error parsing SPRTINFO lump: Invalid frame \"%s\"\0" as *const u8
                as *const libc::c_char,
            sprinfoToken,
        );
    } else {
        frameChar = sprinfoToken;
    }
    frameFrame = R_Char2Frame(*frameChar.offset(0 as libc::c_int as isize));
    Z_Free(sprinfoToken as *mut libc::c_void);
    sprinfoToken = M_GetToken(0 as *const libc::c_char);
    if sprinfoToken.is_null() {
        I_Error(
            b"Error parsing SPRTINFO lump: Missing sprite info\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        if strcmp(sprinfoToken, b"{\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            Z_Free(sprinfoToken as *mut libc::c_void);
            sprinfoToken = M_GetToken(0 as *const libc::c_char);
            if sprinfoToken.is_null() {
                I_Error(
                    b"Error parsing SPRTINFO lump: Unexpected end of file where sprite info should be\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            while strcmp(sprinfoToken, b"}\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                if strcasecmp(
                    sprinfoToken,
                    b"XPIVOT\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    Z_Free(sprinfoToken as *mut libc::c_void);
                    sprinfoToken = M_GetToken(0 as *const libc::c_char);
                    frameXPivot = atoi(sprinfoToken) as int16_t;
                } else if strcasecmp(
                    sprinfoToken,
                    b"YPIVOT\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    Z_Free(sprinfoToken as *mut libc::c_void);
                    sprinfoToken = M_GetToken(0 as *const libc::c_char);
                    frameYPivot = atoi(sprinfoToken) as int16_t;
                } else if strcasecmp(
                    sprinfoToken,
                    b"ROTAXIS\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    Z_Free(sprinfoToken as *mut libc::c_void);
                    sprinfoToken = M_GetToken(0 as *const libc::c_char);
                }
                Z_Free(sprinfoToken as *mut libc::c_void);
                sprinfoToken = M_GetToken(0 as *const libc::c_char);
                if sprinfoToken.is_null() {
                    I_Error(
                        b"Error parsing SPRTINFO lump: Unexpected end of file where sprite info or right curly brace should be\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        Z_Free(sprinfoToken as *mut libc::c_void);
    }
    (*info).pivot[frameFrame as usize].x = frameXPivot as int32_t;
    (*info).pivot[frameFrame as usize].y = frameYPivot as int32_t;
}
unsafe extern "C" fn R_ParseSpriteInfo(mut spr2: boolean) {
    let mut info: *mut spriteinfo_t = 0 as *mut spriteinfo_t;
    let mut sprinfoToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sprinfoTokenLength: size_t = 0;
    let mut newSpriteName: [libc::c_char; 5] = [0; 5];
    let mut sprnum: spritenum_t = NUMSPRITES;
    let mut spr2num: playersprite_t = NUMPLAYERSPRITES;
    let mut i: int32_t = 0;
    let mut skinnumbers: [int32_t; 32] = [0; 32];
    let mut foundskins: int32_t = 0 as libc::c_int;
    sprinfoToken = M_GetToken(0 as *const libc::c_char);
    if sprinfoToken.is_null() {
        I_Error(
            b"Error parsing SPRTINFO lump: Unexpected end of file where sprite name should be\0"
                as *const u8 as *const libc::c_char,
        );
    }
    sprinfoTokenLength = strlen(sprinfoToken);
    if sprinfoTokenLength != 4 as libc::c_int as size_t {
        I_Error(
            b"Error parsing SPRTINFO lump: Sprite name \"%s\" isn't 4 characters long\0"
                as *const u8 as *const libc::c_char,
            sprinfoToken,
        );
    } else {
        memset(
            &mut newSpriteName as *mut [libc::c_char; 5] as *mut libc::c_void,
            0 as libc::c_int,
            5 as libc::c_int as libc::c_ulong,
        );
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            newSpriteName.as_mut_ptr() as *mut libc::c_void,
            sprinfoToken as *const libc::c_void,
            sprinfoTokenLength,
        );
        strupr(newSpriteName.as_mut_ptr());
    }
    Z_Free(sprinfoToken as *mut libc::c_void);
    if spr2 == 0 {
        i = 0 as libc::c_int;
        while i <= NUMSPRITES as libc::c_int {
            if i == NUMSPRITES as libc::c_int {
                I_Error(
                    b"Error parsing SPRTINFO lump: Unknown sprite name \"%s\"\0"
                        as *const u8 as *const libc::c_char,
                    newSpriteName.as_mut_ptr(),
                );
            }
            if memcmp(
                newSpriteName.as_mut_ptr() as *const libc::c_void,
                (sprnames[i as usize]).as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                sprnum = i as spritenum_t;
                break;
            } else {
                i += 1;
                i;
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i <= NUMPLAYERSPRITES as libc::c_int {
            if i == NUMPLAYERSPRITES as libc::c_int {
                I_Error(
                    b"Error parsing SPRTINFO lump: Unknown sprite2 name \"%s\"\0"
                        as *const u8 as *const libc::c_char,
                    newSpriteName.as_mut_ptr(),
                );
            }
            if memcmp(
                newSpriteName.as_mut_ptr() as *const libc::c_void,
                (spr2names[i as usize]).as_mut_ptr() as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                spr2num = i as playersprite_t;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    info = Z_CallocAlign(
        ::core::mem::size_of::<spriteinfo_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut spriteinfo_t;
    (*info).available = true_0 as libc::c_int;
    sprinfoToken = M_GetToken(0 as *const libc::c_char);
    if sprinfoToken.is_null() {
        I_Error(
            b"Error parsing SPRTINFO lump: Unexpected end of file where open curly brace for sprite \"%s\" should be\0"
                as *const u8 as *const libc::c_char,
            newSpriteName.as_mut_ptr(),
        );
    }
    if strcmp(sprinfoToken, b"{\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        Z_Free(sprinfoToken as *mut libc::c_void);
        sprinfoToken = M_GetToken(0 as *const libc::c_char);
        if sprinfoToken.is_null() {
            I_Error(
                b"Error parsing SPRTINFO lump: Unexpected end of file where definition for sprite \"%s\" should be\0"
                    as *const u8 as *const libc::c_char,
                newSpriteName.as_mut_ptr(),
            );
        }
        while strcmp(sprinfoToken, b"}\0" as *const u8 as *const libc::c_char)
            != 0 as libc::c_int
        {
            if strcasecmp(sprinfoToken, b"SKIN\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let mut skinnum: int32_t = 0;
                let mut skinName: *mut libc::c_char = 0 as *mut libc::c_char;
                if spr2 == 0 {
                    I_Error(
                        b"Error parsing SPRTINFO lump: \"SKIN\" token found outside of a sprite2 definition\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                Z_Free(sprinfoToken as *mut libc::c_void);
                sprinfoToken = M_GetToken(0 as *const libc::c_char);
                if sprinfoToken.is_null() {
                    I_Error(
                        b"Error parsing SPRTINFO lump: Unexpected end of file where skin frame should be\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                sprinfoTokenLength = strlen(sprinfoToken);
                skinName = Z_MallocAlign(
                    sprinfoTokenLength
                        .wrapping_add(1 as libc::c_int as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                    PU_STATIC as libc::c_int,
                    0 as *mut libc::c_void,
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        as int32_t,
                ) as *mut libc::c_char;
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    skinName as *mut libc::c_void,
                    sprinfoToken as *const libc::c_void,
                    sprinfoTokenLength
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        ),
                );
                *skinName
                    .offset(sprinfoTokenLength as isize) = '\0' as i32 as libc::c_char;
                strlwr(skinName);
                Z_Free(sprinfoToken as *mut libc::c_void);
                skinnum = R_SkinAvailable(skinName);
                if skinnum == -(1 as libc::c_int) {
                    I_Error(
                        b"Error parsing SPRTINFO lump: Unknown skin \"%s\"\0"
                            as *const u8 as *const libc::c_char,
                        skinName,
                    );
                }
                skinnumbers[foundskins as usize] = skinnum;
                foundskins += 1;
                foundskins;
            } else if strcasecmp(
                sprinfoToken,
                b"FRAME\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                R_ParseSpriteInfoFrame(info);
                Z_Free(sprinfoToken as *mut libc::c_void);
                if spr2 != 0 {
                    if foundskins == 0 {
                        I_Error(
                            b"Error parsing SPRTINFO lump: No skins specified in this sprite2 definition\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    i = 0 as libc::c_int;
                    while i < foundskins {
                        let mut skinnum_0: size_t = skinnumbers[i as usize] as size_t;
                        let mut skin: *mut skin_t = &mut *skins
                            .as_mut_ptr()
                            .offset(skinnum_0 as isize) as *mut skin_t;
                        let mut sprinfo: *mut spriteinfo_t = ((*skin).sprinfo)
                            .as_mut_ptr();
                        M_Memcpy
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *sprinfo.offset(spr2num as isize) as *mut spriteinfo_t
                                as *mut libc::c_void,
                            info as *const libc::c_void,
                            ::core::mem::size_of::<spriteinfo_t>() as libc::c_ulong,
                        );
                        i += 1;
                        i;
                    }
                } else {
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *spriteinfo.as_mut_ptr().offset(sprnum as isize)
                            as *mut spriteinfo_t as *mut libc::c_void,
                        info as *const libc::c_void,
                        ::core::mem::size_of::<spriteinfo_t>() as libc::c_ulong,
                    );
                }
            } else {
                I_Error(
                    b"Error parsing SPRTINFO lump: Unknown keyword \"%s\" in sprite %s\0"
                        as *const u8 as *const libc::c_char,
                    sprinfoToken,
                    newSpriteName.as_mut_ptr(),
                );
            }
            sprinfoToken = M_GetToken(0 as *const libc::c_char);
            if sprinfoToken.is_null() {
                I_Error(
                    b"Error parsing SPRTINFO lump: Unexpected end of file where sprite info or right curly brace for sprite \"%s\" should be\0"
                        as *const u8 as *const libc::c_char,
                    newSpriteName.as_mut_ptr(),
                );
            }
        }
    } else {
        I_Error(
            b"Error parsing SPRTINFO lump: Expected \"{\" for sprite \"%s\", got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            newSpriteName.as_mut_ptr(),
            sprinfoToken,
        );
    }
    Z_Free(sprinfoToken as *mut libc::c_void);
    Z_Free(info as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_ParseSPRTINFOLump(
    mut wadNum: uint16_t,
    mut lumpNum: uint16_t,
) {
    let mut sprinfoLump: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sprinfoLumpLength: size_t = 0;
    let mut sprinfoText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sprinfoToken: *mut libc::c_char = 0 as *mut libc::c_char;
    sprinfoLump = W_CacheLumpNumPwad(wadNum, lumpNum, PU_STATIC as libc::c_int)
        as *mut libc::c_char;
    if sprinfoLump.is_null() {
        return;
    }
    sprinfoLumpLength = W_LumpLengthPwad(wadNum, lumpNum);
    sprinfoText = Z_MallocAlign(
        sprinfoLumpLength
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    memmove(
        sprinfoText as *mut libc::c_void,
        sprinfoLump as *const libc::c_void,
        sprinfoLumpLength,
    );
    *sprinfoText.offset(sprinfoLumpLength as isize) = '\0' as i32 as libc::c_char;
    Z_Free(sprinfoLump as *mut libc::c_void);
    sprinfoToken = M_GetToken(sprinfoText);
    while !sprinfoToken.is_null() {
        if strcasecmp(sprinfoToken, b"SPRITE\0" as *const u8 as *const libc::c_char) == 0
        {
            R_ParseSpriteInfo(false_0 as libc::c_int);
        } else if strcasecmp(
            sprinfoToken,
            b"SPRITE2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            R_ParseSpriteInfo(true_0 as libc::c_int);
        } else {
            I_Error(
                b"Error parsing SPRTINFO lump: Unknown keyword \"%s\"\0" as *const u8
                    as *const libc::c_char,
                sprinfoToken,
            );
        }
        Z_Free(sprinfoToken as *mut libc::c_void);
        sprinfoToken = M_GetToken(0 as *const libc::c_char);
    }
    Z_Free(sprinfoText as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_LoadSpriteInfoLumps(
    mut wadnum: uint16_t,
    mut numlumps: uint16_t,
) {
    let mut lumpinfo: *mut lumpinfo_t = (**wadfiles.offset(wadnum as isize)).lumpinfo;
    let mut i: uint16_t = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < numlumps as libc::c_int {
        name = ((*lumpinfo).name).as_mut_ptr();
        if memcmp(
            name as *const libc::c_void,
            b"SPRTINFO\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
            || memcmp(
                name as *const libc::c_void,
                b"SPR_\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            R_ParseSPRTINFOLump(wadnum, i);
        }
        i = i.wrapping_add(1);
        i;
        lumpinfo = lumpinfo.offset(1);
        lumpinfo;
    }
}
