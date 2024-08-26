use ::libc;
use core::arch::asm;
extern "C" {
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut numskincolors: uint16_t;
    static mut skincolors: [skincolor_t; 1182];
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    static mut colfuncs: [Option::<unsafe extern "C" fn() -> ()>; 9];
    static mut vid: viddef_t;
    static mut colormaps: *mut lighttable_t;
    fn ASTBlendPixel(
        background: RGBA_t,
        foreground: RGBA_t,
        style: libc::c_int,
        alpha: uint8_t,
    ) -> uint32_t;
    static mut centerx: int32_t;
    static mut centery: int32_t;
    static mut centeryfrac: fixed_t;
    static mut fovtan: fixed_t;
    static mut planezlight: *mut *mut lighttable_t;
    static mut skins: [skin_t; 32];
    static mut screens: [*mut uint8_t; 5];
    fn InitColorLUT(lut: *mut colorlookup_t, palette: *mut RGBA_t, makecolors: boolean);
    fn GetColorLUT(
        lut: *mut colorlookup_t,
        r: uint8_t,
        g: uint8_t,
        b: uint8_t,
    ) -> uint8_t;
    static mut pLocalPalette: *mut RGBA_t;
    static mut pMasterPalette: *mut RGBA_t;
    fn W_GetNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn W_ReadLump(lump: lumpnum_t, dest: *mut libc::c_void);
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
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
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
pub type lumpnum_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skincolor_s {
    pub name: [libc::c_char; 33],
    pub ramp: [uint8_t; 16],
    pub invcolor: uint16_t,
    pub invshade: uint8_t,
    pub chatcolor: uint16_t,
    pub accessible: boolean,
}
pub type skincolor_t = skincolor_s;
pub type skincolornum_t = libc::c_uint;
pub const NUMSUPERCOLORS: skincolornum_t = 9;
pub const MAXSKINCOLORS: skincolornum_t = 1182;
pub const SKINCOLOR_LASTFREESLOT: skincolornum_t = 1181;
pub const SKINCOLOR_FIRSTFREESLOT: skincolornum_t = 158;
pub const SKINCOLOR_SUPERTAN5: skincolornum_t = 157;
pub const SKINCOLOR_SUPERTAN4: skincolornum_t = 156;
pub const SKINCOLOR_SUPERTAN3: skincolornum_t = 155;
pub const SKINCOLOR_SUPERTAN2: skincolornum_t = 154;
pub const SKINCOLOR_SUPERTAN1: skincolornum_t = 153;
pub const SKINCOLOR_SUPERRUST5: skincolornum_t = 152;
pub const SKINCOLOR_SUPERRUST4: skincolornum_t = 151;
pub const SKINCOLOR_SUPERRUST3: skincolornum_t = 150;
pub const SKINCOLOR_SUPERRUST2: skincolornum_t = 149;
pub const SKINCOLOR_SUPERRUST1: skincolornum_t = 148;
pub const SKINCOLOR_SUPERPURPLE5: skincolornum_t = 147;
pub const SKINCOLOR_SUPERPURPLE4: skincolornum_t = 146;
pub const SKINCOLOR_SUPERPURPLE3: skincolornum_t = 145;
pub const SKINCOLOR_SUPERPURPLE2: skincolornum_t = 144;
pub const SKINCOLOR_SUPERPURPLE1: skincolornum_t = 143;
pub const SKINCOLOR_SUPERSKY5: skincolornum_t = 142;
pub const SKINCOLOR_SUPERSKY4: skincolornum_t = 141;
pub const SKINCOLOR_SUPERSKY3: skincolornum_t = 140;
pub const SKINCOLOR_SUPERSKY2: skincolornum_t = 139;
pub const SKINCOLOR_SUPERSKY1: skincolornum_t = 138;
pub const SKINCOLOR_SUPERPERIDOT5: skincolornum_t = 137;
pub const SKINCOLOR_SUPERPERIDOT4: skincolornum_t = 136;
pub const SKINCOLOR_SUPERPERIDOT3: skincolornum_t = 135;
pub const SKINCOLOR_SUPERPERIDOT2: skincolornum_t = 134;
pub const SKINCOLOR_SUPERPERIDOT1: skincolornum_t = 133;
pub const SKINCOLOR_SUPERGOLD5: skincolornum_t = 132;
pub const SKINCOLOR_SUPERGOLD4: skincolornum_t = 131;
pub const SKINCOLOR_SUPERGOLD3: skincolornum_t = 130;
pub const SKINCOLOR_SUPERGOLD2: skincolornum_t = 129;
pub const SKINCOLOR_SUPERGOLD1: skincolornum_t = 128;
pub const SKINCOLOR_SUPERORANGE5: skincolornum_t = 127;
pub const SKINCOLOR_SUPERORANGE4: skincolornum_t = 126;
pub const SKINCOLOR_SUPERORANGE3: skincolornum_t = 125;
pub const SKINCOLOR_SUPERORANGE2: skincolornum_t = 124;
pub const SKINCOLOR_SUPERORANGE1: skincolornum_t = 123;
pub const SKINCOLOR_SUPERRED5: skincolornum_t = 122;
pub const SKINCOLOR_SUPERRED4: skincolornum_t = 121;
pub const SKINCOLOR_SUPERRED3: skincolornum_t = 120;
pub const SKINCOLOR_SUPERRED2: skincolornum_t = 119;
pub const SKINCOLOR_SUPERRED1: skincolornum_t = 118;
pub const SKINCOLOR_SUPERSILVER5: skincolornum_t = 117;
pub const SKINCOLOR_SUPERSILVER4: skincolornum_t = 116;
pub const SKINCOLOR_SUPERSILVER3: skincolornum_t = 115;
pub const SKINCOLOR_SUPERSILVER2: skincolornum_t = 114;
pub const SKINCOLOR_SUPERSILVER1: skincolornum_t = 113;
pub const FIRSTSUPERCOLOR: skincolornum_t = 113;
pub const SKINCOLOR_VOLCANIC: skincolornum_t = 112;
pub const SKINCOLOR_SANGRIA: skincolornum_t = 111;
pub const SKINCOLOR_FANCY: skincolornum_t = 110;
pub const SKINCOLOR_ROSY: skincolornum_t = 109;
pub const SKINCOLOR_TAFFY: skincolornum_t = 108;
pub const SKINCOLOR_RASPBERRY: skincolornum_t = 107;
pub const SKINCOLOR_PLUM: skincolornum_t = 106;
pub const SKINCOLOR_EVENTIDE: skincolornum_t = 105;
pub const SKINCOLOR_MAUVE: skincolornum_t = 104;
pub const SKINCOLOR_LILAC: skincolornum_t = 103;
pub const SKINCOLOR_ROYAL: skincolornum_t = 102;
pub const SKINCOLOR_VIOLET: skincolornum_t = 101;
pub const SKINCOLOR_NEON: skincolornum_t = 100;
pub const SKINCOLOR_MAGENTA: skincolornum_t = 99;
pub const SKINCOLOR_SIBERITE: skincolornum_t = 98;
pub const SKINCOLOR_BUBBLEGUM: skincolornum_t = 97;
pub const SKINCOLOR_FUCHSIA: skincolornum_t = 96;
pub const SKINCOLOR_NOBLE: skincolornum_t = 95;
pub const SKINCOLOR_PURPLE: skincolornum_t = 94;
pub const SKINCOLOR_PASTEL: skincolornum_t = 93;
pub const SKINCOLOR_MAJESTY: skincolornum_t = 92;
pub const SKINCOLOR_DUSK: skincolornum_t = 91;
pub const SKINCOLOR_VAPOR: skincolornum_t = 90;
pub const SKINCOLOR_GALAXY: skincolornum_t = 89;
pub const SKINCOLOR_MIDNIGHT: skincolornum_t = 88;
pub const SKINCOLOR_COBALT: skincolornum_t = 87;
pub const SKINCOLOR_BLUE: skincolornum_t = 86;
pub const SKINCOLOR_CORNFLOWER: skincolornum_t = 85;
pub const SKINCOLOR_ARCTIC: skincolornum_t = 84;
pub const SKINCOLOR_SAPPHIRE: skincolornum_t = 83;
pub const SKINCOLOR_DAYBREAK: skincolornum_t = 82;
pub const SKINCOLOR_ICY: skincolornum_t = 81;
pub const SKINCOLOR_DREAM: skincolornum_t = 80;
pub const SKINCOLOR_CERULEAN: skincolornum_t = 79;
pub const SKINCOLOR_MARINE: skincolornum_t = 78;
pub const SKINCOLOR_SKY: skincolornum_t = 77;
pub const SKINCOLOR_AQUAMARINE: skincolornum_t = 76;
pub const SKINCOLOR_TURQUOISE: skincolornum_t = 75;
pub const SKINCOLOR_CYAN: skincolornum_t = 74;
pub const SKINCOLOR_WAVE: skincolornum_t = 73;
pub const SKINCOLOR_OCEAN: skincolornum_t = 72;
pub const SKINCOLOR_TEAL: skincolornum_t = 71;
pub const SKINCOLOR_AQUA: skincolornum_t = 70;
pub const SKINCOLOR_BOTTLE: skincolornum_t = 69;
pub const SKINCOLOR_ISLAND: skincolornum_t = 68;
pub const SKINCOLOR_SEAFOAM: skincolornum_t = 67;
pub const SKINCOLOR_EMERALD: skincolornum_t = 66;
pub const SKINCOLOR_MASTER: skincolornum_t = 65;
pub const SKINCOLOR_MINT: skincolornum_t = 64;
pub const SKINCOLOR_JADE: skincolornum_t = 63;
pub const SKINCOLOR_SHAMROCK: skincolornum_t = 62;
pub const SKINCOLOR_FOREST: skincolornum_t = 61;
pub const SKINCOLOR_GREEN: skincolornum_t = 60;
pub const SKINCOLOR_CHARTREUSE: skincolornum_t = 59;
pub const SKINCOLOR_HEADLIGHT: skincolornum_t = 58;
pub const SKINCOLOR_APPLE: skincolornum_t = 57;
pub const SKINCOLOR_PERIDOT: skincolornum_t = 56;
pub const SKINCOLOR_LIME: skincolornum_t = 55;
pub const SKINCOLOR_LEMON: skincolornum_t = 54;
pub const SKINCOLOR_PEAR: skincolornum_t = 53;
pub const SKINCOLOR_OLIVE: skincolornum_t = 52;
pub const SKINCOLOR_YELLOW: skincolornum_t = 51;
pub const SKINCOLOR_GOLDENROD: skincolornum_t = 50;
pub const SKINCOLOR_SANDY: skincolornum_t = 49;
pub const SKINCOLOR_GOLD: skincolornum_t = 48;
pub const SKINCOLOR_TOPAZ: skincolornum_t = 47;
pub const SKINCOLOR_TANGERINE: skincolornum_t = 46;
pub const SKINCOLOR_RUST: skincolornum_t = 45;
pub const SKINCOLOR_ORANGE: skincolornum_t = 44;
pub const SKINCOLOR_APRICOT: skincolornum_t = 43;
pub const SKINCOLOR_COPPER: skincolornum_t = 42;
pub const SKINCOLOR_SUNSET: skincolornum_t = 41;
pub const SKINCOLOR_FOUNDATION: skincolornum_t = 40;
pub const SKINCOLOR_QUAIL: skincolornum_t = 39;
pub const SKINCOLOR_PEACHY: skincolornum_t = 38;
pub const SKINCOLOR_KETCHUP: skincolornum_t = 37;
pub const SKINCOLOR_GARNET: skincolornum_t = 36;
pub const SKINCOLOR_FLAME: skincolornum_t = 35;
pub const SKINCOLOR_CRIMSON: skincolornum_t = 34;
pub const SKINCOLOR_RED: skincolornum_t = 33;
pub const SKINCOLOR_PEPPER: skincolornum_t = 32;
pub const SKINCOLOR_SALMON: skincolornum_t = 31;
pub const SKINCOLOR_CHERRY: skincolornum_t = 30;
pub const SKINCOLOR_RUBY: skincolornum_t = 29;
pub const SKINCOLOR_LAVENDER: skincolornum_t = 28;
pub const SKINCOLOR_EGGPLANT: skincolornum_t = 27;
pub const SKINCOLOR_AZURE: skincolornum_t = 26;
pub const SKINCOLOR_MOSS: skincolornum_t = 25;
pub const SKINCOLOR_ROSEBUSH: skincolornum_t = 24;
pub const SKINCOLOR_BEIGE: skincolornum_t = 23;
pub const SKINCOLOR_TAN: skincolornum_t = 22;
pub const SKINCOLOR_ECRU: skincolornum_t = 21;
pub const SKINCOLOR_SEPIA: skincolornum_t = 20;
pub const SKINCOLOR_BRONZE: skincolornum_t = 19;
pub const SKINCOLOR_BOULDER: skincolornum_t = 18;
pub const SKINCOLOR_BROWN: skincolornum_t = 17;
pub const SKINCOLOR_LATTE: skincolornum_t = 16;
pub const SKINCOLOR_YOGURT: skincolornum_t = 15;
pub const SKINCOLOR_ROSEWOOD: skincolornum_t = 14;
pub const SKINCOLOR_PINK: skincolornum_t = 13;
pub const SKINCOLOR_BLUEBELL: skincolornum_t = 12;
pub const SKINCOLOR_MOONSTONE: skincolornum_t = 11;
pub const SKINCOLOR_SLATE: skincolornum_t = 10;
pub const SKINCOLOR_AETHER: skincolornum_t = 9;
pub const SKINCOLOR_BLACK: skincolornum_t = 8;
pub const SKINCOLOR_JET: skincolornum_t = 7;
pub const SKINCOLOR_CARBON: skincolornum_t = 6;
pub const SKINCOLOR_SILVER: skincolornum_t = 5;
pub const SKINCOLOR_GREY: skincolornum_t = 4;
pub const SKINCOLOR_CLOUDY: skincolornum_t = 3;
pub const SKINCOLOR_BONE: skincolornum_t = 2;
pub const SKINCOLOR_WHITE: skincolornum_t = 1;
pub const SKINCOLOR_NONE: skincolornum_t = 0;
pub type fixed_t = int32_t;
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
pub type ffloortype_e = libc::c_uint;
pub const FOF_SPLAT: ffloortype_e = 1073741824;
pub const FOF_BOUNCY: ffloortype_e = 536870912;
pub const FOF_COLORMAPONLY: ffloortype_e = 268435456;
pub const FOF_RIPPLE: ffloortype_e = 134217728;
pub const FOF_INTANGIBLEFLATS: ffloortype_e = 100663296;
pub const FOF_REVERSEPLATFORM: ffloortype_e = 67108864;
pub const FOF_PLATFORM: ffloortype_e = 33554432;
pub const FOF_QUICKSAND: ffloortype_e = 16777216;
pub const FOF_BUSTUP: ffloortype_e = 8388608;
pub const FOF_MARIO: ffloortype_e = 4194304;
pub const FOF_GOOWATER: ffloortype_e = 2097152;
pub const FOF_CRUMBLE: ffloortype_e = 1048576;
pub const FOF_NORETURN: ffloortype_e = 524288;
pub const FOF_FLOATBOB: ffloortype_e = 262144;
pub const FOF_DOUBLESHADOW: ffloortype_e = 131072;
pub const FOF_INVERTSIDES: ffloortype_e = 65536;
pub const FOF_ALLSIDES: ffloortype_e = 32768;
pub const FOF_INVERTPLANES: ffloortype_e = 16384;
pub const FOF_FOG: ffloortype_e = 8192;
pub const FOF_TRANSLUCENT: ffloortype_e = 4096;
pub const FOF_EXTRA: ffloortype_e = 2048;
pub const FOF_BOTHPLANES: ffloortype_e = 1024;
pub const FOF_CUTSPRITES: ffloortype_e = 512;
pub const FOF_CUTLEVEL: ffloortype_e = 384;
pub const FOF_CUTEXTRA: ffloortype_e = 256;
pub const FOF_CUTSOLIDS: ffloortype_e = 128;
pub const FOF_NOSHADE: ffloortype_e = 64;
pub const FOF_SWIMMABLE: ffloortype_e = 32;
pub const FOF_RENDERALL: ffloortype_e = 24;
pub const FOF_RENDERPLANES: ffloortype_e = 16;
pub const FOF_RENDERSIDES: ffloortype_e = 8;
pub const FOF_SOLID: ffloortype_e = 6;
pub const FOF_BLOCKOTHERS: ffloortype_e = 4;
pub const FOF_BLOCKPLAYER: ffloortype_e = 2;
pub const FOF_EXISTS: ffloortype_e = 1;
pub type extracolormap_t = extracolormap_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct extracolormap_s {
    pub fadestart: uint8_t,
    pub fadeend: uint8_t,
    pub flags: uint8_t,
    pub rgba: int32_t,
    pub fadergba: int32_t,
    pub colormap: *mut lighttable_t,
    pub next: *mut extracolormap_s,
    pub prev: *mut extracolormap_s,
}
pub type lighttable_t = uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct r_lightlist_s {
    pub height: fixed_t,
    pub heightstep: fixed_t,
    pub botheight: fixed_t,
    pub botheightstep: fixed_t,
    pub startheight: fixed_t,
    pub lightlevel: int16_t,
    pub extra_colormap: *mut extracolormap_t,
    pub rcolormap: *mut lighttable_t,
    pub flags: ffloortype_e,
    pub lightnum: int32_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUMTRANSMAPS: C2RustUnnamed_0 = 10;
pub const tr_trans90: C2RustUnnamed_0 = 9;
pub const tr_trans80: C2RustUnnamed_0 = 8;
pub const tr_trans70: C2RustUnnamed_0 = 7;
pub const tr_trans60: C2RustUnnamed_0 = 6;
pub const tr_trans50: C2RustUnnamed_0 = 5;
pub const tr_trans40: C2RustUnnamed_0 = 4;
pub const tr_trans30: C2RustUnnamed_0 = 3;
pub const tr_trans20: C2RustUnnamed_0 = 2;
pub const tr_trans10: C2RustUnnamed_0 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CV_PossibleValue_s {
    pub value: int32_t,
    pub strvalue: *const libc::c_char,
}
pub type CV_PossibleValue_t = CV_PossibleValue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed_1,
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
pub union C2RustUnnamed_1 {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
pub type viddef_t = viddef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
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
pub struct floatv3_t {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float,
}
pub type C2RustUnnamed_2 = libc::c_int;
pub const TC_DEFAULT: C2RustUnnamed_2 = -122;
pub const TC_DASHMODE: C2RustUnnamed_2 = -123;
pub const TC_BLINK: C2RustUnnamed_2 = -124;
pub const TC_RAINBOW: C2RustUnnamed_2 = -125;
pub const TC_ALLWHITE: C2RustUnnamed_2 = -126;
pub const TC_METALSONIC: C2RustUnnamed_2 = -127;
pub const TC_BOSS: C2RustUnnamed_2 = -128;
pub const PU_STATIC: C2RustUnnamed_4 = 1;
pub const PU_LEVEL: C2RustUnnamed_4 = 50;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUMBLENDMAPS: C2RustUnnamed_3 = 4;
pub const blendtab_modulate: C2RustUnnamed_3 = 3;
pub const blendtab_reversesubtract: C2RustUnnamed_3 = 2;
pub const blendtab_subtract: C2RustUnnamed_3 = 1;
pub const blendtab_add: C2RustUnnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colorlookup_t {
    pub init: boolean,
    pub palette: [RGBA_t; 256],
    pub table: [uint16_t; 65535],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct libdivide_u32_t {
    pub magic: uint32_t,
    pub more: uint8_t,
}
pub const LIBDIVIDE_ADD_MARKER: C2RustUnnamed_5 = 64;
pub const LIBDIVIDE_32_SHIFT_MASK: C2RustUnnamed_5 = 31;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_4 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_4 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_4 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_4 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_4 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_4 = 51;
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
pub type C2RustUnnamed_5 = libc::c_uint;
pub const LIBDIVIDE_NEGATIVE_DIVISOR: C2RustUnnamed_5 = 128;
pub const LIBDIVIDE_64_SHIFT_MASK: C2RustUnnamed_5 = 63;
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
unsafe extern "C" fn FloatToFixed(mut f: libc::c_float) -> fixed_t {
    return (f * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float) as fixed_t;
}
#[inline(always)]
unsafe extern "C" fn FixedToFloat(mut x: fixed_t) -> libc::c_float {
    return x as libc::c_float
        / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float;
}
#[inline]
unsafe extern "C" fn libdivide_mullhi_u32(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    let mut xl: uint64_t = x as uint64_t;
    let mut yl: uint64_t = y as uint64_t;
    let mut rl: uint64_t = xl * yl;
    return (rl >> 32 as libc::c_int) as uint32_t;
}
#[inline]
unsafe extern "C" fn libdivide_count_leading_zeros32(mut val: uint32_t) -> int32_t {
    return val.leading_zeros() as i32;
}
#[inline]
unsafe extern "C" fn libdivide_64_div_32_to_32(
    mut u1: uint32_t,
    mut u0: uint32_t,
    mut v: uint32_t,
    mut r: *mut uint32_t,
) -> uint32_t {
    let mut result: uint32_t = 0;
    asm!(
        "divl {2}", inlateout("ax") u0 => result, inlateout("dx") u1 => * r,
        inlateout(reg) v => _, options(preserves_flags, pure, readonly, att_syntax)
    );
    return result;
}
#[inline]
unsafe extern "C" fn libdivide_internal_u32_gen(
    mut d: uint32_t,
    mut branchfree: libc::c_int,
) -> libdivide_u32_t {
    let mut result: libdivide_u32_t = libdivide_u32_t {
        magic: 0,
        more: 0,
    };
    let mut floor_log_2_d: uint32_t = 0;
    if d == 0 as libc::c_int as uint32_t {
        I_Error(
            b"libdivide.h:%d: %s(): Error: %s\n\0" as *const u8 as *const libc::c_char,
            594 as libc::c_int,
            (*::core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"libdivide_internal_u32_gen\0"))
                .as_ptr(),
            b"divider must be != 0\0" as *const u8 as *const libc::c_char,
        );
    }
    floor_log_2_d = (31 as libc::c_int - libdivide_count_leading_zeros32(d)) as uint32_t;
    if d & d.wrapping_sub(1 as libc::c_int as uint32_t) == 0 as libc::c_int as uint32_t {
        result.magic = 0 as libc::c_int as uint32_t;
        result
            .more = floor_log_2_d
            .wrapping_sub((branchfree != 0 as libc::c_int) as libc::c_int as uint32_t)
            as uint8_t;
    } else {
        let mut more: uint8_t = 0;
        let mut rem: uint32_t = 0;
        let mut proposed_m: uint32_t = 0;
        let mut e: uint32_t = 0;
        proposed_m = libdivide_64_div_32_to_32(
            (1 as libc::c_uint) << floor_log_2_d,
            0 as libc::c_int as uint32_t,
            d,
            &mut rem,
        );
        e = d.wrapping_sub(rem);
        if branchfree == 0 && e < (1 as libc::c_uint) << floor_log_2_d {
            more = floor_log_2_d as uint8_t;
        } else {
            let twice_rem: uint32_t = rem.wrapping_add(rem);
            proposed_m = proposed_m.wrapping_add(proposed_m);
            if twice_rem >= d || twice_rem < rem {
                proposed_m = proposed_m.wrapping_add(1 as libc::c_int as uint32_t);
            }
            more = (floor_log_2_d | LIBDIVIDE_ADD_MARKER as libc::c_int as uint32_t)
                as uint8_t;
        }
        result.magic = (1 as libc::c_int as uint32_t).wrapping_add(proposed_m);
        result.more = more;
    }
    return result;
}
unsafe extern "C" fn libdivide_u32_gen(mut d: uint32_t) -> libdivide_u32_t {
    return libdivide_internal_u32_gen(d, 0 as libc::c_int);
}
unsafe extern "C" fn libdivide_u32_do(
    mut numer: uint32_t,
    mut denom: *const libdivide_u32_t,
) -> uint32_t {
    let mut more: uint8_t = (*denom).more;
    if (*denom).magic == 0 {
        return numer >> more as libc::c_int
    } else {
        let mut q: uint32_t = libdivide_mullhi_u32((*denom).magic, numer);
        if more as libc::c_int & LIBDIVIDE_ADD_MARKER as libc::c_int != 0 {
            let mut t: uint32_t = (numer.wrapping_sub(q) >> 1 as libc::c_int)
                .wrapping_add(q);
            return t >> (more as libc::c_int & LIBDIVIDE_32_SHIFT_MASK as libc::c_int);
        } else {
            return q >> more as libc::c_int
        }
    };
}
#[no_mangle]
pub static mut viewwidth: int32_t = 0;
#[no_mangle]
pub static mut scaledviewwidth: int32_t = 0;
#[no_mangle]
pub static mut viewheight: int32_t = 0;
#[no_mangle]
pub static mut viewwindowx: int32_t = 0;
#[no_mangle]
pub static mut viewwindowy: int32_t = 0;
#[no_mangle]
pub static mut ylookup: [*mut uint8_t; 4800] = [0 as *const uint8_t
    as *mut uint8_t; 4800];
#[no_mangle]
pub static mut ylookup1: [*mut uint8_t; 4800] = [0 as *const uint8_t
    as *mut uint8_t; 4800];
#[no_mangle]
pub static mut ylookup2: [*mut uint8_t; 4800] = [0 as *const uint8_t
    as *mut uint8_t; 4800];
#[no_mangle]
pub static mut columnofs: [int32_t; 7680] = [0; 7680];
#[no_mangle]
pub static mut topleft: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut dc_colormap: *mut lighttable_t = 0 as *const lighttable_t
    as *mut lighttable_t;
#[no_mangle]
pub static mut dc_x: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut dc_yl: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut dc_yh: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut dc_iscale: fixed_t = 0;
#[no_mangle]
pub static mut dc_texturemid: fixed_t = 0;
#[no_mangle]
pub static mut dc_hires: uint8_t = 0;
#[no_mangle]
pub static mut dc_source: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut transtables: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut blendtables: [*mut uint8_t; 4] = [0 as *const uint8_t as *mut uint8_t; 4];
#[no_mangle]
pub static mut dc_transmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut dc_translation: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut dc_lightlist: *mut r_lightlist_s = 0 as *const r_lightlist_s
    as *mut r_lightlist_s;
#[no_mangle]
pub static mut dc_numlights: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut dc_maxlights: int32_t = 0;
#[no_mangle]
pub static mut dc_texheight: int32_t = 0;
#[no_mangle]
pub static mut ds_y: int32_t = 0;
#[no_mangle]
pub static mut ds_x1: int32_t = 0;
#[no_mangle]
pub static mut ds_x2: int32_t = 0;
#[no_mangle]
pub static mut ds_colormap: *mut lighttable_t = 0 as *const lighttable_t
    as *mut lighttable_t;
#[no_mangle]
pub static mut ds_translation: *mut lighttable_t = 0 as *const lighttable_t
    as *mut lighttable_t;
#[no_mangle]
pub static mut ds_xfrac: fixed_t = 0;
#[no_mangle]
pub static mut ds_yfrac: fixed_t = 0;
#[no_mangle]
pub static mut ds_xstep: fixed_t = 0;
#[no_mangle]
pub static mut ds_ystep: fixed_t = 0;
#[no_mangle]
pub static mut ds_waterofs: int32_t = 0;
#[no_mangle]
pub static mut ds_bgofs: int32_t = 0;
#[no_mangle]
pub static mut ds_flatwidth: uint16_t = 0;
#[no_mangle]
pub static mut ds_flatheight: uint16_t = 0;
#[no_mangle]
pub static mut ds_powersoftwo: boolean = 0;
#[no_mangle]
pub static mut ds_solidcolor: boolean = 0;
#[no_mangle]
pub static mut ds_source: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut ds_transmap: *mut uint8_t = 0 as *const uint8_t as *mut uint8_t;
#[no_mangle]
pub static mut ds_su: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut ds_sv: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut ds_sz: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut ds_sup: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut ds_svp: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut ds_szp: *mut floatv3_t = 0 as *const floatv3_t as *mut floatv3_t;
#[no_mangle]
pub static mut focallengthf: libc::c_float = 0.;
#[no_mangle]
pub static mut zeroheight: libc::c_float = 0.;
#[no_mangle]
pub static mut nflatxshift: uint32_t = 0;
#[no_mangle]
pub static mut nflatyshift: uint32_t = 0;
#[no_mangle]
pub static mut nflatshiftup: uint32_t = 0;
#[no_mangle]
pub static mut nflatmask: uint32_t = 0;
static mut translationtablecache: [*mut *mut uint8_t; 39] = [
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
    0 as *const *mut uint8_t as *mut *mut uint8_t,
];
#[no_mangle]
pub static mut skincolor_modified: [uint8_t; 1182] = [0; 1182];
unsafe extern "C" fn SkinToCacheIndex(mut skinnum: int32_t) -> int32_t {
    match skinnum {
        -122 => return 32 as libc::c_int,
        -128 => return 32 as libc::c_int + 1 as libc::c_int,
        -127 => return 32 as libc::c_int + 2 as libc::c_int,
        -126 => return 32 as libc::c_int + 3 as libc::c_int,
        -125 => return 32 as libc::c_int + 4 as libc::c_int,
        -124 => return 32 as libc::c_int + 5 as libc::c_int,
        -123 => return 32 as libc::c_int + 6 as libc::c_int,
        _ => {}
    }
    return skinnum;
}
unsafe extern "C" fn CacheIndexToSkin(mut ttc: int32_t) -> int32_t {
    match ttc {
        32 => return TC_DEFAULT as libc::c_int,
        33 => return TC_BOSS as libc::c_int,
        34 => return TC_METALSONIC as libc::c_int,
        35 => return TC_ALLWHITE as libc::c_int,
        36 => return TC_RAINBOW as libc::c_int,
        37 => return TC_BLINK as libc::c_int,
        38 => return TC_DASHMODE as libc::c_int,
        _ => {}
    }
    return ttc;
}
#[no_mangle]
pub static mut Color_cons_t: [CV_PossibleValue_t; 1183] = [CV_PossibleValue_s {
    value: 0,
    strvalue: 0 as *const libc::c_char,
}; 1183];
#[no_mangle]
pub unsafe extern "C" fn R_InitTranslucencyTables() {
    transtables = Z_MallocAlign(
        (9 as libc::c_int * 0x10000 as libc::c_int) as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        16 as libc::c_int,
    ) as *mut uint8_t;
    W_ReadLump(
        W_GetNumForName(b"TRANS10\0" as *const u8 as *const libc::c_char),
        transtables as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS20\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x10000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS30\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x20000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS40\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x30000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS50\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x40000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS60\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x50000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS70\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x60000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS80\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x70000 as libc::c_int as isize) as *mut libc::c_void,
    );
    W_ReadLump(
        W_GetNumForName(b"TRANS90\0" as *const u8 as *const libc::c_char),
        transtables.offset(0x80000 as libc::c_int as isize) as *mut libc::c_void,
    );
    R_GenerateBlendTables();
}
static mut transtab_lut: colorlookup_t = colorlookup_t {
    init: 0,
    palette: [FColorRGBA { rgba: 0 }; 256],
    table: [0; 65535],
};
unsafe extern "C" fn BlendTab_Translucent(
    mut table: *mut uint8_t,
    mut style: libc::c_int,
    mut blendamt: uint8_t,
) {
    let mut bg: int16_t = 0;
    let mut fg: int16_t = 0;
    if table.is_null() {
        I_Error(
            b"BlendTab_Translucent: input table was NULL!\0" as *const u8
                as *const libc::c_char,
        );
    }
    bg = 0 as libc::c_int as int16_t;
    while (bg as libc::c_int) < 0xff as libc::c_int {
        fg = 0 as libc::c_int as int16_t;
        while (fg as libc::c_int) < 0xff as libc::c_int {
            let mut backrgba: RGBA_t = *pMasterPalette
                .offset((bg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut frontrgba: RGBA_t = *pMasterPalette
                .offset((fg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut result: RGBA_t = FColorRGBA { rgba: 0 };
            result
                .rgba = ASTBlendPixel(
                backrgba,
                frontrgba,
                style,
                0xff as libc::c_int as uint8_t,
            );
            result
                .rgba = ASTBlendPixel(
                result,
                frontrgba,
                AST_TRANSLUCENT as libc::c_int,
                blendamt,
            );
            *table
                .offset(
                    (bg as libc::c_int * 0x100 as libc::c_int + fg as libc::c_int)
                        as isize,
                ) = GetColorLUT(
                &mut transtab_lut,
                result.s.red,
                result.s.green,
                result.s.blue,
            );
            fg += 1;
            fg;
        }
        bg += 1;
        bg;
    }
}
unsafe extern "C" fn BlendTab_Subtractive(
    mut table: *mut uint8_t,
    mut style: libc::c_int,
    mut blendamt: uint8_t,
) {
    let mut bg: int16_t = 0;
    let mut fg: int16_t = 0;
    if table.is_null() {
        I_Error(
            b"BlendTab_Subtractive: input table was NULL!\0" as *const u8
                as *const libc::c_char,
        );
    }
    if blendamt as libc::c_int == 0xff as libc::c_int {
        memset(
            table as *mut libc::c_void,
            GetColorLUT(
                &mut transtab_lut,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as uint8_t,
            ) as libc::c_int,
            0x10000 as libc::c_int as libc::c_ulong,
        );
        return;
    }
    bg = 0 as libc::c_int as int16_t;
    while (bg as libc::c_int) < 0xff as libc::c_int {
        fg = 0 as libc::c_int as int16_t;
        while (fg as libc::c_int) < 0xff as libc::c_int {
            let mut backrgba: RGBA_t = *pMasterPalette
                .offset((bg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut frontrgba: RGBA_t = *pMasterPalette
                .offset((fg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut result: RGBA_t = FColorRGBA { rgba: 0 };
            result
                .rgba = ASTBlendPixel(
                backrgba,
                frontrgba,
                style,
                0xff as libc::c_int as uint8_t,
            );
            result
                .s
                .red = (if 0 as libc::c_int
                > result.s.red as libc::c_int - blendamt as libc::c_int
            {
                0 as libc::c_int
            } else {
                result.s.red as libc::c_int - blendamt as libc::c_int
            }) as uint8_t;
            result
                .s
                .green = (if 0 as libc::c_int
                > result.s.green as libc::c_int - blendamt as libc::c_int
            {
                0 as libc::c_int
            } else {
                result.s.green as libc::c_int - blendamt as libc::c_int
            }) as uint8_t;
            result
                .s
                .blue = (if 0 as libc::c_int
                > result.s.blue as libc::c_int - blendamt as libc::c_int
            {
                0 as libc::c_int
            } else {
                result.s.blue as libc::c_int - blendamt as libc::c_int
            }) as uint8_t;
            *table
                .offset(
                    (bg as libc::c_int * 0x100 as libc::c_int + fg as libc::c_int)
                        as isize,
                ) = GetColorLUT(
                &mut transtab_lut,
                result.s.red,
                result.s.green,
                result.s.blue,
            );
            fg += 1;
            fg;
        }
        bg += 1;
        bg;
    }
}
unsafe extern "C" fn BlendTab_Modulative(mut table: *mut uint8_t) {
    let mut bg: int16_t = 0;
    let mut fg: int16_t = 0;
    if table.is_null() {
        I_Error(
            b"BlendTab_Modulative: input table was NULL!\0" as *const u8
                as *const libc::c_char,
        );
    }
    bg = 0 as libc::c_int as int16_t;
    while (bg as libc::c_int) < 0xff as libc::c_int {
        fg = 0 as libc::c_int as int16_t;
        while (fg as libc::c_int) < 0xff as libc::c_int {
            let mut backrgba: RGBA_t = *pMasterPalette
                .offset((bg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut frontrgba: RGBA_t = *pMasterPalette
                .offset((fg as libc::c_int & 0xff as libc::c_int) as isize);
            let mut result: RGBA_t = FColorRGBA { rgba: 0 };
            result
                .rgba = ASTBlendPixel(
                backrgba,
                frontrgba,
                AST_MODULATE as libc::c_int,
                0 as libc::c_int as uint8_t,
            );
            *table
                .offset(
                    (bg as libc::c_int * 0x100 as libc::c_int + fg as libc::c_int)
                        as isize,
                ) = GetColorLUT(
                &mut transtab_lut,
                result.s.red,
                result.s.green,
                result.s.blue,
            );
            fg += 1;
            fg;
        }
        bg += 1;
        bg;
    }
}
static mut BlendTab_Count: [int32_t; 4] = [
    9 as libc::c_int + 1 as libc::c_int,
    9 as libc::c_int + 1 as libc::c_int,
    9 as libc::c_int + 1 as libc::c_int,
    1 as libc::c_int,
];
static mut BlendTab_FromStyle: [int32_t; 7] = [
    0 as libc::c_int,
    0 as libc::c_int,
    blendtab_add as libc::c_int,
    blendtab_subtract as libc::c_int,
    blendtab_reversesubtract as libc::c_int,
    blendtab_modulate as libc::c_int,
    0 as libc::c_int,
];
unsafe extern "C" fn BlendTab_GenerateMaps(
    mut tab: int32_t,
    mut style: int32_t,
    mut genfunc: Option::<unsafe extern "C" fn(*mut uint8_t, libc::c_int, uint8_t) -> ()>,
) {
    let mut i: int32_t = 0 as libc::c_int;
    let mut num: int32_t = BlendTab_Count[tab as usize];
    let amtmul: libc::c_float = 256.0f32
        / (9 as libc::c_int + 1 as libc::c_int) as libc::c_float;
    while i < num {
        let offs: size_t = (0x10000 as libc::c_int * i) as size_t;
        let alpha: uint16_t = (if (amtmul * i as libc::c_float)
            < 0xff as libc::c_int as libc::c_float
        {
            amtmul * i as libc::c_float
        } else {
            0xff as libc::c_int as libc::c_float
        }) as uint16_t;
        genfunc
            .expect(
                "non-null function pointer",
            )(
            (blendtables[tab as usize]).offset(offs as isize),
            style,
            alpha as uint8_t,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateBlendTables() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < NUMBLENDMAPS as libc::c_int {
        blendtables[i
            as usize] = Z_MallocAlign(
            (BlendTab_Count[i as usize] * 0x10000 as libc::c_int) as size_t,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            16 as libc::c_int,
        ) as *mut uint8_t;
        i += 1;
        i;
    }
    InitColorLUT(&mut transtab_lut, pMasterPalette, false_0 as libc::c_int);
    BlendTab_GenerateMaps(
        blendtab_add as libc::c_int,
        AST_ADD as libc::c_int,
        Some(
            BlendTab_Translucent
                as unsafe extern "C" fn(*mut uint8_t, libc::c_int, uint8_t) -> (),
        ),
    );
    BlendTab_GenerateMaps(
        blendtab_subtract as libc::c_int,
        AST_SUBTRACT as libc::c_int,
        Some(
            BlendTab_Subtractive
                as unsafe extern "C" fn(*mut uint8_t, libc::c_int, uint8_t) -> (),
        ),
    );
    BlendTab_GenerateMaps(
        blendtab_reversesubtract as libc::c_int,
        AST_REVERSESUBTRACT as libc::c_int,
        Some(
            BlendTab_Translucent
                as unsafe extern "C" fn(*mut uint8_t, libc::c_int, uint8_t) -> (),
        ),
    );
    BlendTab_Modulative(blendtables[blendtab_modulate as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn R_GetTranslucencyTable(
    mut alphalevel: int32_t,
) -> *mut uint8_t {
    return transtables
        .offset(
            ((if (if (alphalevel - 1 as libc::c_int)
                < NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
            {
                alphalevel - 1 as libc::c_int
            } else {
                NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
            }) > 0 as libc::c_int
            {
                (if (alphalevel - 1 as libc::c_int)
                    < NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
                {
                    alphalevel - 1 as libc::c_int
                } else {
                    NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
                })
            } else {
                0 as libc::c_int
            }) << 16 as libc::c_int) as isize,
        );
}
#[no_mangle]
pub unsafe extern "C" fn R_GetBlendTable(
    mut style: libc::c_int,
    mut alphalevel: int32_t,
) -> *mut uint8_t {
    let mut offs: size_t = 0;
    if style <= AST_COPY as libc::c_int || style >= AST_OVERLAY as libc::c_int {
        return 0 as *mut uint8_t;
    }
    offs = ((if (if alphalevel
        < BlendTab_Count[BlendTab_FromStyle[style as usize] as usize] - 1 as libc::c_int
    {
        alphalevel
    } else {
        BlendTab_Count[BlendTab_FromStyle[style as usize] as usize] - 1 as libc::c_int
    }) > 0 as libc::c_int
    {
        (if alphalevel
            < BlendTab_Count[BlendTab_FromStyle[style as usize] as usize]
                - 1 as libc::c_int
        {
            alphalevel
        } else {
            BlendTab_Count[BlendTab_FromStyle[style as usize] as usize]
                - 1 as libc::c_int
        })
    } else {
        0 as libc::c_int
    }) << 16 as libc::c_int) as size_t;
    match style {
        2 => {
            return (blendtables[blendtab_add as libc::c_int as usize])
                .offset(offs as isize);
        }
        3 => {
            return (blendtables[blendtab_subtract as libc::c_int as usize])
                .offset(offs as isize);
        }
        4 => {
            return (blendtables[blendtab_reversesubtract as libc::c_int as usize])
                .offset(offs as isize);
        }
        5 => return blendtables[blendtab_modulate as libc::c_int as usize],
        _ => {}
    }
    alphalevel -= 1;
    if alphalevel >= 0 as libc::c_int {
        return transtables
            .offset(
                ((if (if alphalevel < NUMTRANSMAPS as libc::c_int - 2 as libc::c_int {
                    alphalevel
                } else {
                    NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
                }) > 0 as libc::c_int
                {
                    (if alphalevel < NUMTRANSMAPS as libc::c_int - 2 as libc::c_int {
                        alphalevel
                    } else {
                        NUMTRANSMAPS as libc::c_int - 2 as libc::c_int
                    })
                } else {
                    0 as libc::c_int
                }) << 16 as libc::c_int) as isize,
            )
    } else {
        return 0 as *mut uint8_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_BlendLevelVisible(
    mut blendmode: int32_t,
    mut alphalevel: int32_t,
) -> boolean {
    if blendmode <= AST_COPY as libc::c_int || blendmode == AST_SUBTRACT as libc::c_int
        || blendmode == AST_MODULATE as libc::c_int
        || blendmode >= AST_OVERLAY as libc::c_int
    {
        return true_0 as libc::c_int;
    }
    return (alphalevel < BlendTab_Count[BlendTab_FromStyle[blendmode as usize] as usize])
        as libc::c_int;
}
unsafe extern "C" fn R_RainbowColormap(
    mut dest_colormap: *mut uint8_t,
    mut skincolor: uint16_t,
) {
    let mut i: int32_t = 0;
    let mut color: RGBA_t = FColorRGBA { rgba: 0 };
    let mut brightness: uint8_t = 0;
    let mut j: int32_t = 0;
    let mut colorbrightnesses: [uint8_t; 16] = [0; 16];
    let mut brightdif: uint16_t = 0;
    let mut temp: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        color = *pLocalPalette
            .offset(
                (skincolors[skincolor as usize].ramp[i as usize] as libc::c_int
                    & 0xff as libc::c_int) as isize,
            );
        colorbrightnesses[i
            as usize] = ((1063 as libc::c_int * color.s.red as uint16_t as libc::c_int
            / 5000 as libc::c_int
            + 3576 as libc::c_int * color.s.green as uint16_t as libc::c_int
                / 5000 as libc::c_int
            + 361 as libc::c_int * color.s.blue as uint16_t as libc::c_int
                / 5000 as libc::c_int) / 3 as libc::c_int) as uint8_t;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if i == 0 as libc::c_int || i == 31 as libc::c_int {
            *dest_colormap.offset(i as isize) = i as uint8_t;
        } else {
            color = *pLocalPalette.offset((i & 0xff as libc::c_int) as isize);
            brightness = ((1063 as libc::c_int * color.s.red as uint16_t as libc::c_int
                / 5000 as libc::c_int
                + 3576 as libc::c_int * color.s.green as uint16_t as libc::c_int
                    / 5000 as libc::c_int
                + 361 as libc::c_int * color.s.blue as uint16_t as libc::c_int
                    / 5000 as libc::c_int) / 3 as libc::c_int) as uint8_t;
            brightdif = 256 as libc::c_int as uint16_t;
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                temp = abs(
                    brightness as int16_t as libc::c_int
                        - colorbrightnesses[j as usize] as int16_t as libc::c_int,
                );
                if temp < brightdif as libc::c_int {
                    brightdif = temp as uint16_t;
                    *dest_colormap
                        .offset(
                            i as isize,
                        ) = skincolors[skincolor as usize].ramp[j as usize];
                }
                j += 1;
                j;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn R_GenerateTranslationColormap(
    mut dest_colormap: *mut uint8_t,
    mut skinnum: int32_t,
    mut color: uint16_t,
) {
    let mut i: int32_t = 0;
    let mut starttranscolor: int32_t = 0;
    let mut skinramplength: int32_t = 0;
    if skinnum < TC_DEFAULT as libc::c_int {
        match skinnum {
            -126 => {
                memset(
                    dest_colormap as *mut libc::c_void,
                    0 as libc::c_int,
                    (256 as libc::c_int as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as libc::c_ulong),
                );
                return;
            }
            -125 => {
                if color as libc::c_int >= numskincolors as libc::c_int {
                    I_Error(
                        b"Invalid skin color #%hu.\0" as *const u8
                            as *const libc::c_char,
                        color as libc::c_int,
                    );
                }
                if color as libc::c_int != SKINCOLOR_NONE as libc::c_int {
                    R_RainbowColormap(dest_colormap, color);
                    return;
                }
            }
            -124 => {
                if color as libc::c_int >= numskincolors as libc::c_int {
                    I_Error(
                        b"Invalid skin color #%hu.\0" as *const u8
                            as *const libc::c_char,
                        color as libc::c_int,
                    );
                }
                if color as libc::c_int != SKINCOLOR_NONE as libc::c_int {
                    memset(
                        dest_colormap as *mut libc::c_void,
                        skincolors[color as usize].ramp[3 as libc::c_int as usize]
                            as libc::c_int,
                        (256 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                            ),
                    );
                    return;
                }
            }
            _ => {}
        }
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *dest_colormap.offset(i as isize) = i as uint8_t;
            i += 1;
            i;
        }
        if skinnum == TC_BOSS as libc::c_int {
            let mut originalColormap: *mut uint8_t = R_GetTranslationColormap(
                TC_DEFAULT as libc::c_int,
                color as skincolornum_t,
                1 as libc::c_int as uint8_t,
            );
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                *dest_colormap
                    .offset(
                        (96 as libc::c_int + i) as isize,
                    ) = *originalColormap.offset((96 as libc::c_int + i) as isize);
                *dest_colormap.offset((31 as libc::c_int - i) as isize) = i as uint8_t;
                i += 1;
                i;
            }
        } else if skinnum == TC_METALSONIC as libc::c_int {
            i = 0 as libc::c_int;
            while i < 6 as libc::c_int {
                *dest_colormap
                    .offset(
                        skincolors[SKINCOLOR_BLUE as libc::c_int as usize]
                            .ramp[(12 as libc::c_int - i) as usize] as isize,
                    ) = skincolors[SKINCOLOR_BLUE as libc::c_int as usize]
                    .ramp[i as usize];
                i += 1;
                i;
            }
            let ref mut fresh0 = *dest_colormap.offset(254 as libc::c_int as isize);
            *fresh0 = 0 as libc::c_int as uint8_t;
            let ref mut fresh1 = *dest_colormap.offset(253 as libc::c_int as isize);
            *fresh1 = *fresh0;
            *dest_colormap.offset(159 as libc::c_int as isize) = *fresh1;
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                *dest_colormap
                    .offset(
                        (96 as libc::c_int + i) as isize,
                    ) = *dest_colormap
                    .offset(
                        skincolors[SKINCOLOR_COBALT as libc::c_int as usize]
                            .ramp[i as usize] as isize,
                    );
                i += 1;
                i;
            }
        } else if skinnum == TC_DASHMODE as libc::c_int {
            let ref mut fresh2 = *dest_colormap.offset(97 as libc::c_int as isize);
            *fresh2 = 48 as libc::c_int as uint8_t;
            *dest_colormap.offset(96 as libc::c_int as isize) = *fresh2;
            *dest_colormap
                .offset(98 as libc::c_int as isize) = 49 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(99 as libc::c_int as isize) = 51 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(100 as libc::c_int as isize) = 52 as libc::c_int as uint8_t;
            let ref mut fresh3 = *dest_colormap.offset(102 as libc::c_int as isize);
            *fresh3 = 54 as libc::c_int as uint8_t;
            *dest_colormap.offset(101 as libc::c_int as isize) = *fresh3;
            *dest_colormap
                .offset(103 as libc::c_int as isize) = 34 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(104 as libc::c_int as isize) = 37 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(105 as libc::c_int as isize) = 39 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(106 as libc::c_int as isize) = 41 as libc::c_int as uint8_t;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                *dest_colormap
                    .offset(
                        (107 as libc::c_int + i) as isize,
                    ) = (43 as libc::c_int + i) as uint8_t;
                i += 1;
                i;
            }
            *dest_colormap
                .offset(32 as libc::c_int as isize) = 146 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(33 as libc::c_int as isize) = 147 as libc::c_int as uint8_t;
            let ref mut fresh4 = *dest_colormap.offset(35 as libc::c_int as isize);
            *fresh4 = 170 as libc::c_int as uint8_t;
            *dest_colormap.offset(34 as libc::c_int as isize) = *fresh4;
            *dest_colormap
                .offset(36 as libc::c_int as isize) = 171 as libc::c_int as uint8_t;
            let ref mut fresh5 = *dest_colormap.offset(38 as libc::c_int as isize);
            *fresh5 = 172 as libc::c_int as uint8_t;
            *dest_colormap.offset(37 as libc::c_int as isize) = *fresh5;
            let ref mut fresh6 = *dest_colormap.offset(41 as libc::c_int as isize);
            *fresh6 = 173 as libc::c_int as uint8_t;
            let ref mut fresh7 = *dest_colormap.offset(40 as libc::c_int as isize);
            *fresh7 = *fresh6;
            *dest_colormap.offset(39 as libc::c_int as isize) = *fresh7;
            let ref mut fresh8 = *dest_colormap.offset(44 as libc::c_int as isize);
            *fresh8 = 174 as libc::c_int as uint8_t;
            let ref mut fresh9 = *dest_colormap.offset(43 as libc::c_int as isize);
            *fresh9 = *fresh8;
            *dest_colormap.offset(42 as libc::c_int as isize) = *fresh9;
            let ref mut fresh10 = *dest_colormap.offset(47 as libc::c_int as isize);
            *fresh10 = 175 as libc::c_int as uint8_t;
            let ref mut fresh11 = *dest_colormap.offset(46 as libc::c_int as isize);
            *fresh11 = *fresh10;
            *dest_colormap.offset(45 as libc::c_int as isize) = *fresh11;
            *dest_colormap
                .offset(71 as libc::c_int as isize) = 139 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(170 as libc::c_int as isize) = 52 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(171 as libc::c_int as isize) = 54 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(172 as libc::c_int as isize) = 56 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(173 as libc::c_int as isize) = 42 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(174 as libc::c_int as isize) = 45 as libc::c_int as uint8_t;
            *dest_colormap
                .offset(175 as libc::c_int as isize) = 47 as libc::c_int as uint8_t;
        }
        return;
    } else if color as libc::c_int == SKINCOLOR_NONE as libc::c_int {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *dest_colormap.offset(i as isize) = i as uint8_t;
            i += 1;
            i;
        }
        return;
    }
    if color as libc::c_int >= numskincolors as libc::c_int {
        I_Error(
            b"Invalid skin color #%hu.\0" as *const u8 as *const libc::c_char,
            color as libc::c_int,
        );
    }
    if skinnum < 0 as libc::c_int && skinnum > TC_DEFAULT as libc::c_int {
        I_Error(
            b"Invalid translation colormap index %d.\0" as *const u8
                as *const libc::c_char,
            skinnum,
        );
    }
    starttranscolor = if skinnum != TC_DEFAULT as libc::c_int {
        skins[skinnum as usize].starttranscolor as libc::c_int
    } else {
        96 as libc::c_int
    };
    if starttranscolor >= 256 as libc::c_int {
        I_Error(
            b"Invalid startcolor #%d.\0" as *const u8 as *const libc::c_char,
            starttranscolor,
        );
    }
    i = 0 as libc::c_int;
    while i < starttranscolor {
        *dest_colormap.offset(i as isize) = i as uint8_t;
        i += 1;
        i;
    }
    i = starttranscolor + 16 as libc::c_int;
    if i < 256 as libc::c_int {
        i = i as uint8_t as int32_t;
        while i < 256 as libc::c_int {
            *dest_colormap.offset(i as isize) = i as uint8_t;
            i += 1;
            i;
        }
        skinramplength = 16 as libc::c_int;
    } else {
        skinramplength = i - 256 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < skinramplength {
        *dest_colormap
            .offset(
                (starttranscolor + i) as isize,
            ) = skincolors[color as usize].ramp[i as usize];
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GetTranslationColormap(
    mut skinnum: int32_t,
    mut color: skincolornum_t,
    mut flags: uint8_t,
) -> *mut uint8_t {
    let mut ret: *mut uint8_t = 0 as *mut uint8_t;
    let mut skintableindex: int32_t = SkinToCacheIndex(skinnum);
    let mut i: int32_t = 0;
    if flags as libc::c_int & 1 as libc::c_int != 0 {
        if (translationtablecache[skintableindex as usize]).is_null() {
            translationtablecache[skintableindex
                as usize] = Z_CallocAlign(
                (MAXSKINCOLORS as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut *mut uint8_t>() as libc::c_ulong,
                    ),
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut *mut uint8_t;
        }
        ret = *(translationtablecache[skintableindex as usize]).offset(color as isize);
        if skincolor_modified[color as usize] != 0 {
            i = 0 as libc::c_int;
            while i
                < (::core::mem::size_of::<[*mut *mut uint8_t; 39]>() as libc::c_ulong)
                    .wrapping_div(
                        ::core::mem::size_of::<*mut *mut uint8_t>() as libc::c_ulong,
                    ) as int32_t
            {
                if !(translationtablecache[i as usize]).is_null()
                    && !(*(translationtablecache[i as usize]).offset(color as isize))
                        .is_null()
                {
                    R_GenerateTranslationColormap(
                        *(translationtablecache[i as usize]).offset(color as isize),
                        CacheIndexToSkin(i),
                        color as uint16_t,
                    );
                }
                i += 1;
                i;
            }
            skincolor_modified[color as usize] = false_0 as libc::c_int as uint8_t;
        }
    } else {
        ret = 0 as *mut uint8_t;
    }
    if ret.is_null() {
        ret = Z_MallocAlign(
            256 as libc::c_int as size_t,
            if flags as libc::c_int & 1 as libc::c_int != 0 {
                PU_LEVEL as libc::c_int
            } else {
                PU_STATIC as libc::c_int
            },
            0 as *mut libc::c_void,
            8 as libc::c_int,
        ) as *mut uint8_t;
        R_GenerateTranslationColormap(ret, skinnum, color as uint16_t);
        if flags as libc::c_int & 1 as libc::c_int != 0 {
            let ref mut fresh12 = *(translationtablecache[skintableindex as usize])
                .offset(color as isize);
            *fresh12 = ret;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn R_FlushTranslationColormapCache() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i
        < (::core::mem::size_of::<[*mut *mut uint8_t; 39]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*mut *mut uint8_t>() as libc::c_ulong)
            as int32_t
    {
        if !(translationtablecache[i as usize]).is_null() {
            memset(
                translationtablecache[i as usize] as *mut libc::c_void,
                0 as libc::c_int,
                (MAXSKINCOLORS as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut *mut uint8_t>() as libc::c_ulong,
                    ),
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GetColorByName(mut name: *const libc::c_char) -> uint16_t {
    let mut color: uint16_t = atoi(name) as uint16_t;
    if color as libc::c_int > 0 as libc::c_int
        && (color as libc::c_int) < numskincolors as libc::c_int
    {
        return color;
    }
    color = 1 as libc::c_int as uint16_t;
    while (color as libc::c_int) < numskincolors as libc::c_int {
        if strcasecmp((skincolors[color as usize].name).as_mut_ptr(), name) == 0 {
            return color;
        }
        color = color.wrapping_add(1);
        color;
    }
    return SKINCOLOR_NONE as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_GetSuperColorByName(
    mut name: *const libc::c_char,
) -> uint16_t {
    let mut i: uint16_t = 0;
    let mut color: uint16_t = SKINCOLOR_NONE as libc::c_int as uint16_t;
    let mut realname: *mut libc::c_char = Z_MallocAlign(
        (32 as libc::c_int + 1 as libc::c_int) as size_t,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    snprintf(
        realname,
        (32 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
        b"Super %s 1\0" as *const u8 as *const libc::c_char,
        name,
    );
    i = 1 as libc::c_int as uint16_t;
    while (i as libc::c_int) < numskincolors as libc::c_int {
        if strcasecmp((skincolors[i as usize].name).as_mut_ptr(), realname) == 0 {
            color = i;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    Z_Free(realname as *mut libc::c_void);
    return color;
}
#[no_mangle]
pub unsafe extern "C" fn R_InitViewBuffer(mut width: int32_t, mut height: int32_t) {
    let mut i: int32_t = 0;
    let mut bytesperpixel: int32_t = vid.bpp;
    if width > 1920 as libc::c_int {
        width = 1920 as libc::c_int;
    }
    if height > 1200 as libc::c_int {
        height = 1200 as libc::c_int;
    }
    if bytesperpixel < 1 as libc::c_int || bytesperpixel > 4 as libc::c_int {
        I_Error(
            b"R_InitViewBuffer: wrong bytesperpixel value %d\n\0" as *const u8
                as *const libc::c_char,
            bytesperpixel,
        );
    }
    viewwindowx = vid.width - width >> 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < width {
        columnofs[i as usize] = (viewwindowx + i) * bytesperpixel;
        i += 1;
        i;
    }
    if width == vid.width {
        viewwindowy = 0 as libc::c_int;
    } else {
        viewwindowy = vid.height - height >> 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < height {
        ylookup1[i
            as usize] = (screens[0 as libc::c_int as usize])
            .offset(((i + viewwindowy) * vid.width * bytesperpixel) as isize);
        ylookup[i as usize] = ylookup1[i as usize];
        ylookup2[i
            as usize] = (screens[0 as libc::c_int as usize])
            .offset(
                ((i + (vid.height >> 1 as libc::c_int)) * vid.width * bytesperpixel)
                    as isize,
            );
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut viewborderlump: [lumpnum_t; 8] = [0; 8];
#[no_mangle]
pub unsafe extern "C" fn R_InitViewBorder() {
    viewborderlump[0 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_t\0" as *const u8 as *const libc::c_char);
    viewborderlump[1 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_b\0" as *const u8 as *const libc::c_char);
    viewborderlump[2 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_l\0" as *const u8 as *const libc::c_char);
    viewborderlump[3 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_r\0" as *const u8 as *const libc::c_char);
    viewborderlump[4 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_tl\0" as *const u8 as *const libc::c_char);
    viewborderlump[6 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_bl\0" as *const u8 as *const libc::c_char);
    viewborderlump[5 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_tr\0" as *const u8 as *const libc::c_char);
    viewborderlump[7 as libc::c_int
        as usize] = W_GetNumForName(b"brdr_br\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn R_VideoErase(mut ofs: size_t, mut count: int32_t) {
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        (screens[0 as libc::c_int as usize]).offset(ofs as isize) as *mut libc::c_void,
        (screens[1 as libc::c_int as usize]).offset(ofs as isize) as *const libc::c_void,
        count as size_t,
    );
}
static mut tiltlighting: [int32_t; 1920] = [0; 1920];
unsafe extern "C" fn R_CalcTiltedLighting(mut start: fixed_t, mut end: fixed_t) {
    let mut left: int32_t = ds_x1;
    let mut right: int32_t = ds_x2;
    let mut step: fixed_t = (end - start) / (ds_x2 - ds_x1 + 1 as libc::c_int);
    let mut i: int32_t = 0;
    i = left;
    while i <= right {
        start += step;
        tiltlighting[i as usize] = start >> 16 as libc::c_int;
        if tiltlighting[i as usize] < 0 as libc::c_int {
            tiltlighting[i as usize] = 0 as libc::c_int;
        } else if tiltlighting[i as usize] >= 48 as libc::c_int {
            tiltlighting[i as usize] = 48 as libc::c_int - 1 as libc::c_int;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    count += 1;
    count;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    let mut source: *const uint8_t = dc_source;
    let mut colormap: *const lighttable_t = dc_colormap;
    let mut heightmask: int32_t = dc_texheight - 1 as libc::c_int;
    if dc_texheight & heightmask != 0 {
        heightmask += 1;
        heightmask;
        heightmask <<= 16 as libc::c_int;
        if frac < 0 as libc::c_int {
            loop {
                frac += heightmask;
                if !(frac < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            while frac >= heightmask {
                frac -= heightmask;
            }
        }
        loop {
            *dest = *colormap
                .offset(*source.offset((frac >> 16 as libc::c_int) as isize) as isize);
            dest = dest.offset(vid.width as isize);
            if fracstep > 0x7fffffff as libc::c_int - frac {
                frac += fracstep - heightmask;
            } else {
                frac += fracstep;
            }
            while frac >= heightmask {
                frac -= heightmask;
            }
            count -= 1;
            if !(count != 0) {
                break;
            }
        }
    } else {
        loop {
            count -= 2 as libc::c_int;
            if !(count >= 0 as libc::c_int) {
                break;
            }
            *dest = *colormap
                .offset(
                    *source.offset((frac >> 16 as libc::c_int & heightmask) as isize)
                        as isize,
                );
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            *dest = *colormap
                .offset(
                    *source.offset((frac >> 16 as libc::c_int & heightmask) as isize)
                        as isize,
                );
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
        }
        if count & 1 as libc::c_int != 0 {
            *dest = *colormap
                .offset(
                    *source.offset((frac >> 16 as libc::c_int & heightmask) as isize)
                        as isize,
                );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Draw2sMultiPatchColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    count += 1;
    count;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    let mut source: *const uint8_t = dc_source;
    let mut colormap: *const lighttable_t = dc_colormap;
    let mut heightmask: int32_t = dc_texheight - 1 as libc::c_int;
    let mut val: uint8_t = 0;
    if dc_texheight & heightmask != 0 {
        heightmask += 1;
        heightmask;
        heightmask <<= 16 as libc::c_int;
        if frac < 0 as libc::c_int {
            loop {
                frac += heightmask;
                if !(frac < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            while frac >= heightmask {
                frac -= heightmask;
            }
        }
        loop {
            val = *source.offset((frac >> 16 as libc::c_int) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
            dest = dest.offset(vid.width as isize);
            if fracstep > 0x7fffffff as libc::c_int - frac {
                frac += fracstep - heightmask;
            } else {
                frac += fracstep;
            }
            while frac >= heightmask {
                frac -= heightmask;
            }
            count -= 1;
            if !(count != 0) {
                break;
            }
        }
    } else {
        loop {
            count -= 2 as libc::c_int;
            if !(count >= 0 as libc::c_int) {
                break;
            }
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
        }
        if count & 1 as libc::c_int != 0 {
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Draw2sMultiPatchTranslucentColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    count += 1;
    count;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    let mut source: *const uint8_t = dc_source;
    let mut transmap: *const uint8_t = dc_transmap;
    let mut colormap: *const lighttable_t = dc_colormap;
    let mut heightmask: int32_t = dc_texheight - 1 as libc::c_int;
    let mut val: uint8_t = 0;
    if dc_texheight & heightmask != 0 {
        heightmask += 1;
        heightmask;
        heightmask <<= 16 as libc::c_int;
        if frac < 0 as libc::c_int {
            loop {
                frac += heightmask;
                if !(frac < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            while frac >= heightmask {
                frac -= heightmask;
            }
        }
        loop {
            val = *source.offset((frac >> 16 as libc::c_int) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *transmap
                    .offset(
                        ((*colormap.offset(val as isize) as libc::c_int)
                            << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
            dest = dest.offset(vid.width as isize);
            if fracstep > 0x7fffffff as libc::c_int - frac {
                frac += fracstep - heightmask;
            } else {
                frac += fracstep;
            }
            while frac >= heightmask {
                frac -= heightmask;
            }
            count -= 1;
            if !(count != 0) {
                break;
            }
        }
    } else {
        loop {
            count -= 2 as libc::c_int;
            if !(count >= 0 as libc::c_int) {
                break;
            }
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *transmap
                    .offset(
                        ((*colormap.offset(val as isize) as libc::c_int)
                            << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *transmap
                    .offset(
                        ((*colormap.offset(val as isize) as libc::c_int)
                            << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
        }
        if count & 1 as libc::c_int != 0 {
            val = *source.offset((frac >> 16 as libc::c_int & heightmask) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *transmap
                    .offset(
                        ((*colormap.offset(val as isize) as libc::c_int)
                            << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawShadeColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    if dc_yl < 0 as libc::c_int || dc_x >= vid.width {
        return;
    }
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    loop {
        *dest = *colormaps
            .offset(
                (((*dc_source.offset((frac >> 16 as libc::c_int) as isize)
                    as libc::c_int) << 8 as libc::c_int) + *dest as libc::c_int) as isize,
            );
        dest = dest.offset(vid.width as isize);
        frac += fracstep;
        let fresh13 = count;
        count = count - 1;
        if !(fresh13 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl + 1 as libc::c_int;
    if count <= 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    let mut source: *const uint8_t = dc_source;
    let mut transmap: *const uint8_t = dc_transmap;
    let mut colormap: *const lighttable_t = dc_colormap;
    let mut heightmask: int32_t = dc_texheight - 1 as libc::c_int;
    if dc_texheight & heightmask != 0 {
        heightmask += 1;
        heightmask;
        heightmask <<= 16 as libc::c_int;
        if frac < 0 as libc::c_int {
            loop {
                frac += heightmask;
                if !(frac < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            while frac >= heightmask {
                frac -= heightmask;
            }
        }
        loop {
            *dest = *transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source.offset((frac >> 16 as libc::c_int) as isize) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            if frac >= heightmask {
                frac -= heightmask;
            }
            count -= 1;
            if !(count != 0) {
                break;
            }
        }
    } else {
        loop {
            count -= 2 as libc::c_int;
            if !(count >= 0 as libc::c_int) {
                break;
            }
            *dest = *transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            *dest = *transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
        }
        if count & 1 as libc::c_int != 0 {
            *dest = *transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawDropShadowColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    count = dc_yh - dc_yl + 1 as libc::c_int;
    if count <= 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    let mut transmap_offset: *const uint8_t = dc_transmap
        .offset(
            ((*dc_colormap.offset(31 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int) as isize,
        );
    loop {
        count -= 2 as libc::c_int;
        if !(count >= 0 as libc::c_int) {
            break;
        }
        *dest = *transmap_offset.offset(*dest as libc::c_int as isize);
        dest = dest.offset(vid.width as isize);
        *dest = *transmap_offset.offset(*dest as libc::c_int as isize);
        dest = dest.offset(vid.width as isize);
    }
    if count & 1 as libc::c_int != 0 {
        *dest = *transmap_offset.offset(*dest as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslatedTranslucentColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl + 1 as libc::c_int;
    if count <= 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    let mut heightmask: int32_t = dc_texheight - 1 as libc::c_int;
    if dc_texheight & heightmask != 0 {
        heightmask += 1;
        heightmask;
        heightmask <<= 16 as libc::c_int;
        if frac < 0 as libc::c_int {
            loop {
                frac += heightmask;
                if !(frac < 0 as libc::c_int) {
                    break;
                }
            }
        } else {
            while frac >= heightmask {
                frac -= heightmask;
            }
        }
        loop {
            *dest = *dc_transmap
                .offset(
                    ((*dc_colormap
                        .offset(
                            *dc_translation
                                .offset(
                                    *dc_source.offset((frac >> 16 as libc::c_int) as isize)
                                        as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            if frac >= heightmask {
                frac -= heightmask;
            }
            count -= 1;
            if !(count != 0) {
                break;
            }
        }
    } else {
        loop {
            count -= 2 as libc::c_int;
            if !(count >= 0 as libc::c_int) {
                break;
            }
            *dest = *dc_transmap
                .offset(
                    ((*dc_colormap
                        .offset(
                            *dc_translation
                                .offset(
                                    *dc_source
                                        .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                        as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
            *dest = *dc_transmap
                .offset(
                    ((*dc_colormap
                        .offset(
                            *dc_translation
                                .offset(
                                    *dc_source
                                        .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                        as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(vid.width as isize);
            frac += fracstep;
        }
        if count & 1 as libc::c_int != 0 {
            *dest = *dc_transmap
                .offset(
                    ((*dc_colormap
                        .offset(
                            *dc_translation
                                .offset(
                                    *dc_source
                                        .offset((frac >> 16 as libc::c_int & heightmask) as isize)
                                        as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslatedColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut frac: fixed_t = 0;
    let mut fracstep: fixed_t = 0;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    fracstep = dc_iscale;
    frac = (dc_texturemid
        + FixedMul((dc_yl << 16 as libc::c_int) - centeryfrac, fracstep))
        * (dc_hires == 0) as libc::c_int;
    loop {
        *dest = *dc_colormap
            .offset(
                *dc_translation
                    .offset(
                        *dc_source.offset((frac >> 16 as libc::c_int) as isize) as isize,
                    ) as isize,
            );
        dest = dest.offset(vid.width as isize);
        frac += fracstep;
        let fresh14 = count;
        count = count - 1;
        if !(fresh14 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpan_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    if dest.offset(8 as libc::c_int as isize) > deststop as *mut uint8_t {
        return;
    }
    while count >= 8 as libc::c_int as size_t {
        *dest
            .offset(
                0 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                1 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                2 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                3 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                4 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                5 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                6 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                7 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh15 = count;
        count = count.wrapping_sub(1);
        if !(fresh15 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        let fresh16 = dest;
        dest = dest.offset(1);
        *fresh16 = *colormap
            .offset(
                *source
                    .offset(
                        (yposition as uint32_t >> nflatyshift & nflatmask
                            | xposition as uint32_t >> nflatxshift) as isize,
                    ) as isize,
            );
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh17 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh17 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            *dest = *colormap
                .offset(
                    *source
                        .offset(
                            (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                        ) as isize,
                );
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh18 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh18 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            *dest = *colormap
                .offset(
                    *source
                        .offset(
                            (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                        ) as isize,
                );
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh19 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh19 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                *dest = *colormap
                    .offset(
                        *source
                            .offset(
                                (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                            ) as isize,
                    );
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedTranslucentSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh20 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh20 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset(
                                    (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh21 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh21 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset(
                                    (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh22 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh22 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *source
                                    .offset(
                                        (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                    ) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedWaterSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut dsrc: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    dsrc = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh23 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh23 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let fresh24 = dsrc;
            dsrc = dsrc.offset(1);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset(
                                    (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*fresh24 as libc::c_int as isize);
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh25 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh25 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let fresh26 = dsrc;
            dsrc = dsrc.offset(1);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset(
                                    (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                ) as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*fresh26 as libc::c_int as isize);
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh27 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh27 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                let fresh28 = dsrc;
                dsrc = dsrc.offset(1);
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *source
                                    .offset(
                                        (v >> nflatyshift & nflatmask | u >> nflatxshift) as isize,
                                    ) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh28 as libc::c_int as isize);
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedSplat_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint8_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh29 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh29 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh30 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh30 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh31 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh31 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                val = *source
                    .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
                if val as libc::c_int != 255 as libc::c_int {
                    *dest = *colormap.offset(val as isize);
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSplat_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    while count >= 8 as libc::c_int as size_t {
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(0 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(1 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(2 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(3 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(4 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(5 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(6 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val &= 0x3fffff as libc::c_int as uint32_t;
        val = *source.offset(val as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest.offset(7 as libc::c_int as isize) = *colormap.offset(val as isize);
        }
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh32 = count;
        count = count.wrapping_sub(1);
        if !(fresh32 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest = *colormap.offset(val as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentSplat_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    while count >= 8 as libc::c_int as size_t {
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    0 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    1 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(1 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    2 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(2 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    3 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(3 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    4 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(4 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    5 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(5 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    6 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(6 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest
                .offset(
                    7 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest.offset(7 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh33 = count;
        count = count.wrapping_sub(1);
        if !(fresh33 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawFloorSprite_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    while count >= 8 as libc::c_int as size_t {
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    0 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    1 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    2 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    3 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    4 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    5 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    6 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        val = *source.offset(val as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    7 as libc::c_int as isize,
                ) = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh34 = count;
        count = count.wrapping_sub(1);
        if !(fresh34 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentFloorSprite_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    while count >= 8 as libc::c_int as size_t {
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    0 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    1 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(1 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    2 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(2 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    3 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(3 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    4 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(4 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    5 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(5 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    6 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(6 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest
                .offset(
                    7 as libc::c_int as isize,
                ) = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest.offset(7 as libc::c_int as isize) as libc::c_int as isize);
        }
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh35 = count;
        count = count.wrapping_sub(1);
        if !(fresh35 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        val = *source
            .offset(
                (yposition as uint32_t >> nflatyshift & nflatmask
                    | xposition as uint32_t >> nflatxshift) as isize,
            ) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedFloorSprite_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint16_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *colormap
                    .offset(
                        *translation
                            .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                            as isize,
                    );
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *colormap
                    .offset(
                        *translation
                            .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                            as isize,
                    );
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                val = *source
                    .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
                if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                    *dest = *colormap
                        .offset(
                            *translation
                                .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                as isize,
                        );
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedTranslucentFloorSprite_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint16_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *translation
                                    .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            val = *source
                .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *translation
                                    .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                val = *source
                    .offset((v >> nflatyshift & nflatmask | u >> nflatxshift) as isize);
                if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                    *dest = *ds_transmap
                        .offset(
                            ((*colormap
                                .offset(
                                    *translation
                                        .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                        as isize,
                                ) as libc::c_int) << 8 as libc::c_int) as isize,
                        )
                        .offset(*dest as libc::c_int as isize);
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentSpan_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    xposition <<= nflatshiftup;
    yposition <<= nflatshiftup;
    xstep <<= nflatshiftup;
    ystep <<= nflatshiftup;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    while count >= 8 as libc::c_int as size_t {
        *dest
            .offset(
                0 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(0 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                1 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(1 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                2 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(2 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                3 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(3 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                4 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(4 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                5 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(5 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                6 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(6 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        *dest
            .offset(
                7 as libc::c_int as isize,
            ) = *ds_transmap
            .offset(
                ((*colormap
                    .offset(
                        *source
                            .offset(
                                (yposition as uint32_t >> nflatyshift & nflatmask
                                    | xposition as uint32_t >> nflatxshift) as isize,
                            ) as isize,
                    ) as libc::c_int) << 8 as libc::c_int) as isize,
            )
            .offset(*dest.offset(7 as libc::c_int as isize) as libc::c_int as isize);
        xposition += xstep;
        yposition += ystep;
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh36 = count;
        count = count.wrapping_sub(1);
        if !(fresh36 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        val = yposition as uint32_t >> nflatyshift & nflatmask
            | xposition as uint32_t >> nflatxshift;
        *dest = *ds_transmap
            .offset(
                ((*colormap.offset(*source.offset(val as isize) as isize) as libc::c_int)
                    << 8 as libc::c_int) as isize,
            )
            .offset(*dest as libc::c_int as isize);
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawWaterSpan_8() {
    let mut xposition: uint32_t = 0;
    let mut yposition: uint32_t = 0;
    let mut xstep: uint32_t = 0;
    let mut ystep: uint32_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut dsrc: *mut uint8_t = 0 as *mut uint8_t;
    let mut count: size_t = 0;
    xposition = (ds_xfrac << nflatshiftup) as uint32_t;
    yposition = (ds_yfrac + ds_waterofs << nflatshiftup) as uint32_t;
    xstep = (ds_xstep << nflatshiftup) as uint32_t;
    ystep = (ds_ystep << nflatshiftup) as uint32_t;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    dsrc = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    count = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    while count >= 8 as libc::c_int as size_t {
        let fresh37 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                0 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh37 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh38 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                1 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh38 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh39 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                2 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh39 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh40 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                3 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh40 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh41 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                4 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh41 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh42 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                5 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh42 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh43 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                6 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh43 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        let fresh44 = dsrc;
        dsrc = dsrc.offset(1);
        *dest
            .offset(
                7 as libc::c_int as isize,
            ) = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh44 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
        dest = dest.offset(8 as libc::c_int as isize);
        count = count.wrapping_sub(8 as libc::c_int as size_t);
    }
    loop {
        let fresh45 = count;
        count = count.wrapping_sub(1);
        if !(fresh45 != 0) {
            break;
        }
        let fresh46 = dsrc;
        dsrc = dsrc.offset(1);
        let fresh47 = dest;
        dest = dest.offset(1);
        *fresh47 = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source
                            .offset(
                                (yposition >> nflatyshift & nflatmask
                                    | xposition >> nflatxshift) as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh46 as libc::c_int as isize) as isize,
            );
        xposition = xposition.wrapping_add(xstep);
        yposition = yposition.wrapping_add(ystep);
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawFogSpan_8() {
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut count: size_t = 0;
    colormap = ds_colormap;
    dest = &mut *topleft.offset((ds_y * vid.width + ds_x1) as isize) as *mut uint8_t;
    count = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    while count >= 4 as libc::c_int as size_t {
        *dest
            .offset(
                0 as libc::c_int as isize,
            ) = *colormap.offset(*dest.offset(0 as libc::c_int as isize) as isize);
        *dest
            .offset(
                1 as libc::c_int as isize,
            ) = *colormap.offset(*dest.offset(1 as libc::c_int as isize) as isize);
        *dest
            .offset(
                2 as libc::c_int as isize,
            ) = *colormap.offset(*dest.offset(2 as libc::c_int as isize) as isize);
        *dest
            .offset(
                3 as libc::c_int as isize,
            ) = *colormap.offset(*dest.offset(3 as libc::c_int as isize) as isize);
        dest = dest.offset(4 as libc::c_int as isize);
        count = count.wrapping_sub(4 as libc::c_int as size_t);
    }
    loop {
        let fresh48 = count;
        count = count.wrapping_sub(1);
        if !(fresh48 != 0) {
            break;
        }
        *dest = *colormap.offset(*dest as isize);
        dest = dest.offset(1);
        dest;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedFogSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut iz: libc::c_double = ((*ds_szp).z
        + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    loop {
        let fresh49 = ds_x1;
        ds_x1 = ds_x1 + 1;
        let mut colormap: *mut uint8_t = (*planezlight
            .offset(tiltlighting[fresh49 as usize] as isize))
            .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
        *dest = *colormap.offset(*dest as isize);
        dest = dest.offset(1);
        dest;
        width -= 1;
        if !(width >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSolidColorSpan_8() {
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut source: uint8_t = *ds_colormap
        .offset(*ds_source.offset(0 as libc::c_int as isize) as isize);
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    memset(dest as *mut libc::c_void, source as libc::c_int, count);
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTransSolidColorSpan_8() {
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut source: uint8_t = *ds_colormap
        .offset(*ds_source.offset(0 as libc::c_int as isize) as isize);
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    loop {
        let fresh50 = count;
        count = count.wrapping_sub(1);
        if !(fresh50 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        *dest = *ds_transmap
            .offset(((source as libc::c_int) << 8 as libc::c_int) as isize)
            .offset(*dest as libc::c_int as isize);
        dest = dest.offset(1);
        dest;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedSolidColorSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut source: uint8_t = *ds_source.offset(0 as libc::c_int as isize);
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut iz: libc::c_double = ((*ds_szp).z
        + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    loop {
        let fresh51 = ds_x1;
        ds_x1 = ds_x1 + 1;
        let mut colormap: *mut uint8_t = (*planezlight
            .offset(tiltlighting[fresh51 as usize] as isize))
            .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
        let fresh52 = dest;
        dest = dest.offset(1);
        *fresh52 = *colormap.offset(source as isize);
        width -= 1;
        if !(width >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedTransSolidColorSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut source: uint8_t = *ds_source.offset(0 as libc::c_int as isize);
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut iz: libc::c_double = ((*ds_szp).z
        + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    loop {
        let fresh53 = ds_x1;
        ds_x1 = ds_x1 + 1;
        let mut colormap: *mut uint8_t = (*planezlight
            .offset(tiltlighting[fresh53 as usize] as isize))
            .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
        *dest = *ds_transmap
            .offset(
                ((*colormap.offset(source as isize) as libc::c_int) << 8 as libc::c_int)
                    as isize,
            )
            .offset(*dest as libc::c_int as isize);
        dest = dest.offset(1);
        dest;
        width -= 1;
        if !(width >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawWaterSolidColorSpan_8() {
    let mut source: uint8_t = *ds_source.offset(0 as libc::c_int as isize);
    let mut colormap: *mut uint8_t = ds_colormap;
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut dsrc: *mut uint8_t = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    loop {
        let fresh54 = count;
        count = count.wrapping_sub(1);
        if !(fresh54 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        let fresh55 = dsrc;
        dsrc = dsrc.offset(1);
        *dest = *colormap
            .offset(
                *ds_transmap
                    .offset(((source as libc::c_int) << 8 as libc::c_int) as isize)
                    .offset(*fresh55 as libc::c_int as isize) as isize,
            );
        dest = dest.offset(1);
        dest;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedWaterSolidColorSpan_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut source: uint8_t = *ds_source.offset(0 as libc::c_int as isize);
    let mut dest: *mut uint8_t = (ylookup[ds_y as usize])
        .offset(columnofs[ds_x1 as usize] as isize);
    let mut dsrc: *mut uint8_t = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    let mut iz: libc::c_double = ((*ds_szp).z
        + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    loop {
        let fresh56 = ds_x1;
        ds_x1 = ds_x1 + 1;
        let mut colormap: *mut uint8_t = (*planezlight
            .offset(tiltlighting[fresh56 as usize] as isize))
            .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
        let fresh57 = dsrc;
        dsrc = dsrc.offset(1);
        let fresh58 = dest;
        dest = dest.offset(1);
        *fresh58 = *ds_transmap
            .offset(
                ((*colormap.offset(source as isize) as libc::c_int) << 8 as libc::c_int)
                    as isize,
            )
            .offset(*fresh57 as libc::c_int as isize);
        width -= 1;
        if !(width >= 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawFogColumn_8() {
    let mut count: int32_t = 0;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    dest = &mut *topleft.offset((dc_yl * vid.width + dc_x) as isize) as *mut uint8_t;
    loop {
        *dest = *dc_colormap.offset(*dest as isize);
        dest = dest.offset(vid.width as isize);
        let fresh59 = count;
        count = count - 1;
        if !(fresh59 != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawColumnShadowed_8() {
    let mut count: int32_t = 0;
    let mut realyh: int32_t = 0;
    let mut i: int32_t = 0;
    let mut height: int32_t = 0;
    let mut bheight: int32_t = 0 as libc::c_int;
    let mut solid: int32_t = 0 as libc::c_int;
    realyh = dc_yh;
    count = dc_yh - dc_yl;
    if count < 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < dc_numlights {
        solid = ((*dc_lightlist.offset(i as isize)).flags as libc::c_uint
            & FOF_CUTSOLIDS as libc::c_int as libc::c_uint) as int32_t;
        height = (*dc_lightlist.offset(i as isize)).height >> 12 as libc::c_int;
        if solid != 0 {
            bheight = (*dc_lightlist.offset(i as isize)).botheight >> 12 as libc::c_int;
            if bheight < height {
                let mut temp: int32_t = height;
                height = bheight;
                bheight = temp;
            }
        }
        if height <= dc_yl {
            dc_colormap = (*dc_lightlist.offset(i as isize)).rcolormap;
            if solid != 0 && dc_yl < bheight {
                dc_yl = bheight;
            }
        } else {
            dc_yh = height;
            if dc_yh > realyh {
                dc_yh = realyh;
            }
            (colfuncs[0 as libc::c_int as usize]).expect("non-null function pointer")();
            if solid != 0 {
                dc_yl = bheight;
            } else {
                dc_yl = dc_yh + 1 as libc::c_int;
            }
            dc_colormap = (*dc_lightlist.offset(i as isize)).rcolormap;
        }
        i += 1;
        i;
    }
    dc_yh = realyh;
    if dc_yl <= realyh {
        (colfuncs[0 as libc::c_int as usize]).expect("non-null function pointer")();
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpan_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    if dest.offset(8 as libc::c_int as isize) > deststop as *mut uint8_t {
        return;
    }
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh60 = count;
        count = count.wrapping_sub(1);
        if !(fresh60 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        let fresh61 = dest;
        dest = dest.offset(1);
        *fresh61 = *colormap
            .offset(
                *source.offset((y * ds_flatwidth as libc::c_int + x) as isize) as isize,
            );
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedSpan_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh62 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh62 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            *dest = *colormap
                .offset(
                    *source.offset((y * ds_flatwidth as libc::c_int + x) as isize)
                        as isize,
                );
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh63 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh63 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            *dest = *colormap
                .offset(
                    *source.offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize)
                        as isize,
                );
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh64 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh64 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                *dest = *colormap
                    .offset(
                        *source
                            .offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize)
                            as isize,
                    );
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedTranslucentSpan_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh65 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh65 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((y * ds_flatwidth as libc::c_int + x) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh66 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh66 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh67 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh67 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *source
                                    .offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedSplat_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint8_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh68 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh68 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh69 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh69 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize);
            if val as libc::c_int != 255 as libc::c_int {
                *dest = *colormap.offset(val as isize);
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh70 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh70 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                val = *source.offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize);
                if val as libc::c_int != 255 as libc::c_int {
                    *dest = *colormap.offset(val as isize);
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSplat_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh71 = count;
        count = count.wrapping_sub(1);
        if !(fresh71 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest = *colormap.offset(val as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentSplat_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh72 = count;
        count = count.wrapping_sub(1);
        if !(fresh72 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize) as uint32_t;
        if val != 255 as libc::c_int as uint32_t {
            *dest = *ds_transmap
                .offset(
                    ((*colormap.offset(val as isize) as libc::c_int) << 8 as libc::c_int)
                        as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawFloorSprite_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh73 = count;
        count = count.wrapping_sub(1);
        if !(fresh73 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest = *colormap
                .offset(
                    *translation.offset((val & 0xff as libc::c_int as uint32_t) as isize)
                        as isize,
                );
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentFloorSprite_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh74 = count;
        count = count.wrapping_sub(1);
        if !(fresh74 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize) as uint32_t;
        if val & 0xff00 as libc::c_int as uint32_t != 0 {
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *translation
                                .offset((val & 0xff as libc::c_int as uint32_t) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*dest as libc::c_int as isize);
        }
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedFloorSprite_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint16_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *colormap
                    .offset(
                        *translation
                            .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                            as isize,
                    );
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *colormap
                    .offset(
                        *translation
                            .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                            as isize,
                    );
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                val = *source.offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize);
                if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                    *dest = *colormap
                        .offset(
                            *translation
                                .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                as isize,
                        );
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedTranslucentFloorSprite_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint16_t = 0 as *mut uint16_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut translation: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut val: uint16_t = 0;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    source = ds_source as *mut uint16_t;
    colormap = ds_colormap;
    translation = ds_translation;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y * ds_flatwidth as libc::c_int + x) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *translation
                                    .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            val = *source.offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize);
            if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *translation
                                    .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*dest as libc::c_int as isize);
            }
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                val = *source.offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize);
                if val as libc::c_int & 0xff00 as libc::c_int != 0 {
                    *dest = *ds_transmap
                        .offset(
                            ((*colormap
                                .offset(
                                    *translation
                                        .offset((val as libc::c_int & 0xff as libc::c_int) as isize)
                                        as isize,
                                ) as libc::c_int) << 8 as libc::c_int) as isize,
                        )
                        .offset(*dest as libc::c_int as isize);
                }
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTranslucentSpan_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    let mut val: uint32_t = 0;
    xposition = ds_xfrac;
    yposition = ds_yfrac;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh75 = count;
        count = count.wrapping_sub(1);
        if !(fresh75 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        val = (y * ds_flatwidth as libc::c_int + x) as uint32_t;
        *dest = *ds_transmap
            .offset(
                ((*colormap.offset(*source.offset(val as isize) as isize) as libc::c_int)
                    << 8 as libc::c_int) as isize,
            )
            .offset(*dest as libc::c_int as isize);
        dest = dest.offset(1);
        dest;
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawWaterSpan_NPO2_8() {
    let mut xposition: fixed_t = 0;
    let mut yposition: fixed_t = 0;
    let mut xstep: fixed_t = 0;
    let mut ystep: fixed_t = 0;
    let mut x: fixed_t = 0;
    let mut y: fixed_t = 0;
    let mut fixedwidth: fixed_t = 0;
    let mut fixedheight: fixed_t = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut dsrc: *mut uint8_t = 0 as *mut uint8_t;
    let mut deststop: *const uint8_t = (screens[0 as libc::c_int as usize])
        .offset((vid.rowbytes * vid.height as size_t) as isize);
    let mut count: size_t = (ds_x2 - ds_x1 + 1 as libc::c_int) as size_t;
    xposition = ds_xfrac;
    yposition = ds_yfrac + ds_waterofs;
    xstep = ds_xstep;
    ystep = ds_ystep;
    source = ds_source;
    colormap = ds_colormap;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    dsrc = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    fixedwidth = (ds_flatwidth as libc::c_int) << 16 as libc::c_int;
    fixedheight = (ds_flatheight as libc::c_int) << 16 as libc::c_int;
    if xposition < 0 as libc::c_int {
        xposition = (fixedwidth as uint32_t)
            .wrapping_sub((fixedwidth - xposition) as uint32_t % fixedwidth as uint32_t)
            as fixed_t;
    } else if xposition >= fixedwidth {
        xposition %= fixedwidth;
    }
    if yposition < 0 as libc::c_int {
        yposition = (fixedheight as uint32_t)
            .wrapping_sub(
                (fixedheight - yposition) as uint32_t % fixedheight as uint32_t,
            ) as fixed_t;
    } else if yposition >= fixedheight {
        yposition %= fixedheight;
    }
    loop {
        let fresh76 = count;
        count = count.wrapping_sub(1);
        if !(fresh76 != 0 && dest <= deststop as *mut uint8_t) {
            break;
        }
        if xstep < 0 as libc::c_int {
            while xposition < 0 as libc::c_int {
                xposition += fixedwidth;
            }
        } else {
            while xposition >= fixedwidth {
                xposition -= fixedwidth;
            }
        }
        if ystep < 0 as libc::c_int {
            while yposition < 0 as libc::c_int {
                yposition += fixedheight;
            }
        } else {
            while yposition >= fixedheight {
                yposition -= fixedheight;
            }
        }
        x = xposition >> 16 as libc::c_int;
        y = yposition >> 16 as libc::c_int;
        let fresh77 = dsrc;
        dsrc = dsrc.offset(1);
        let fresh78 = dest;
        dest = dest.offset(1);
        *fresh78 = *colormap
            .offset(
                *ds_transmap
                    .offset(
                        ((*source.offset((y * ds_flatwidth as libc::c_int + x) as isize)
                            as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh77 as libc::c_int as isize) as isize,
            );
        xposition += xstep;
        yposition += ystep;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTiltedWaterSpan_NPO2_8() {
    let mut width: libc::c_int = ds_x2 - ds_x1;
    let mut iz: libc::c_double = 0.;
    let mut uz: libc::c_double = 0.;
    let mut vz: libc::c_double = 0.;
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut i: libc::c_int = 0;
    let mut source: *mut uint8_t = 0 as *mut uint8_t;
    let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
    let mut dest: *mut uint8_t = 0 as *mut uint8_t;
    let mut dsrc: *mut uint8_t = 0 as *mut uint8_t;
    let mut startz: libc::c_double = 0.;
    let mut startu: libc::c_double = 0.;
    let mut startv: libc::c_double = 0.;
    let mut izstep: libc::c_double = 0.;
    let mut uzstep: libc::c_double = 0.;
    let mut vzstep: libc::c_double = 0.;
    let mut endz: libc::c_double = 0.;
    let mut endu: libc::c_double = 0.;
    let mut endv: libc::c_double = 0.;
    let mut stepu: uint32_t = 0;
    let mut stepv: uint32_t = 0;
    let mut x_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatwidth as uint32_t);
    let mut y_divider: libdivide_u32_t = libdivide_u32_gen(ds_flatheight as uint32_t);
    iz = ((*ds_szp).z + (*ds_szp).y * (centery - ds_y) as libc::c_float
        + (*ds_szp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    let mut planelightfloat: libc::c_float = (320 as libc::c_int * 320 as libc::c_int
        / vid.width) as libc::c_float / zeroheight / 21.0f32 * FixedToFloat(fovtan);
    let mut lightstart: libc::c_float = 0.;
    let mut lightend: libc::c_float = 0.;
    lightend = ((iz + ((*ds_szp).x * width as libc::c_float) as libc::c_double)
        * planelightfloat as libc::c_double) as libc::c_float;
    lightstart = (iz * planelightfloat as libc::c_double) as libc::c_float;
    R_CalcTiltedLighting(FloatToFixed(lightstart), FloatToFixed(lightend));
    uz = ((*ds_sup).z + (*ds_sup).y * (centery - ds_y) as libc::c_float
        + (*ds_sup).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    vz = ((*ds_svp).z + (*ds_svp).y * (centery - ds_y) as libc::c_float
        + (*ds_svp).x * (ds_x1 - centerx) as libc::c_float) as libc::c_double;
    dest = (ylookup[ds_y as usize]).offset(columnofs[ds_x1 as usize] as isize);
    dsrc = (screens[1 as libc::c_int as usize])
        .offset(((ds_y + ds_bgofs) * vid.width) as isize)
        .offset(ds_x1 as isize);
    source = ds_source;
    startz = 1.0f32 as libc::c_double / iz;
    startu = uz * startz;
    startv = vz * startz;
    izstep = ((*ds_szp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    uzstep = ((*ds_sup).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    vzstep = ((*ds_svp).x * 16 as libc::c_int as libc::c_float) as libc::c_double;
    width += 1;
    width;
    while width >= 16 as libc::c_int {
        iz += izstep;
        uz += uzstep;
        vz += vzstep;
        endz = 1.0f32 as libc::c_double / iz;
        endu = uz * endz;
        endv = vz * endz;
        stepu = ((endu - startu) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        stepv = ((endv - startv) * 0.0625f32 as libc::c_double) as int64_t as uint32_t;
        u = startu as int64_t as uint32_t;
        v = startv as int64_t as uint32_t;
        i = 16 as libc::c_int - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            let fresh79 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh79 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x < 0 as libc::c_int {
                x = (x as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x = (x as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y < 0 as libc::c_int {
                y = (y as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y = (y as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            let fresh80 = dsrc;
            dsrc = dsrc.offset(1);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((y * ds_flatwidth as libc::c_int + x) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*fresh80 as libc::c_int as isize);
            dest = dest.offset(1);
            dest;
            u = u.wrapping_add(stepu);
            v = v.wrapping_add(stepv);
            i -= 1;
            i;
        }
        startu = endu;
        startv = endv;
        width -= 16 as libc::c_int;
    }
    if width > 0 as libc::c_int {
        if width == 1 as libc::c_int {
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            let fresh81 = ds_x1;
            ds_x1 = ds_x1 + 1;
            colormap = (*planezlight.offset(tiltlighting[fresh81 as usize] as isize))
                .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
            let mut x_0: fixed_t = u as fixed_t >> 16 as libc::c_int;
            let mut y_0: fixed_t = v as fixed_t >> 16 as libc::c_int;
            if x_0 < 0 as libc::c_int {
                x_0 = (x_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-x_0 - 1 as libc::c_int) as uint32_t,
                            &mut x_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                x_0 = (x_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(x_0 as uint32_t, &mut x_divider)
                            * ds_flatwidth as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            if y_0 < 0 as libc::c_int {
                y_0 = (y_0 as uint32_t)
                    .wrapping_add(
                        (libdivide_u32_do(
                            (-y_0 - 1 as libc::c_int) as uint32_t,
                            &mut y_divider,
                        ))
                            .wrapping_add(1 as libc::c_int as uint32_t)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            } else {
                y_0 = (y_0 as uint32_t)
                    .wrapping_sub(
                        libdivide_u32_do(y_0 as uint32_t, &mut y_divider)
                            * ds_flatheight as uint32_t,
                    ) as fixed_t as fixed_t;
            }
            let fresh82 = dsrc;
            dsrc = dsrc.offset(1);
            *dest = *ds_transmap
                .offset(
                    ((*colormap
                        .offset(
                            *source
                                .offset((y_0 * ds_flatwidth as libc::c_int + x_0) as isize)
                                as isize,
                        ) as libc::c_int) << 8 as libc::c_int) as isize,
                )
                .offset(*fresh82 as libc::c_int as isize);
        } else {
            let mut left: libc::c_double = width as libc::c_double;
            iz += (*ds_szp).x as libc::c_double * left;
            uz += (*ds_sup).x as libc::c_double * left;
            vz += (*ds_svp).x as libc::c_double * left;
            endz = 1.0f32 as libc::c_double / iz;
            endu = uz * endz;
            endv = vz * endz;
            left = 1.0f32 as libc::c_double / left;
            stepu = ((endu - startu) * left) as int64_t as uint32_t;
            stepv = ((endv - startv) * left) as int64_t as uint32_t;
            u = startu as int64_t as uint32_t;
            v = startv as int64_t as uint32_t;
            while width != 0 as libc::c_int {
                let fresh83 = ds_x1;
                ds_x1 = ds_x1 + 1;
                colormap = (*planezlight.offset(tiltlighting[fresh83 as usize] as isize))
                    .offset(ds_colormap.offset_from(colormaps) as libc::c_long as isize);
                let mut x_1: fixed_t = u as fixed_t >> 16 as libc::c_int;
                let mut y_1: fixed_t = v as fixed_t >> 16 as libc::c_int;
                if x_1 < 0 as libc::c_int {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-x_1 - 1 as libc::c_int) as uint32_t,
                                &mut x_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    x_1 = (x_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(x_1 as uint32_t, &mut x_divider)
                                * ds_flatwidth as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                if y_1 < 0 as libc::c_int {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_add(
                            (libdivide_u32_do(
                                (-y_1 - 1 as libc::c_int) as uint32_t,
                                &mut y_divider,
                            ))
                                .wrapping_add(1 as libc::c_int as uint32_t)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                } else {
                    y_1 = (y_1 as uint32_t)
                        .wrapping_sub(
                            libdivide_u32_do(y_1 as uint32_t, &mut y_divider)
                                * ds_flatheight as uint32_t,
                        ) as fixed_t as fixed_t;
                }
                let fresh84 = dsrc;
                dsrc = dsrc.offset(1);
                *dest = *ds_transmap
                    .offset(
                        ((*colormap
                            .offset(
                                *source
                                    .offset((y_1 * ds_flatwidth as libc::c_int + x_1) as isize)
                                    as isize,
                            ) as libc::c_int) << 8 as libc::c_int) as isize,
                    )
                    .offset(*fresh84 as libc::c_int as isize);
                dest = dest.offset(1);
                dest;
                u = u.wrapping_add(stepu);
                v = v.wrapping_add(stepv);
                width -= 1;
                width;
            }
        }
    }
}
