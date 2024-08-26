use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut numskincolors: uint16_t;
    static mut skincolors: [skincolor_t; 1182];
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut netgame: boolean;
    static mut addedtogame: boolean;
    static mut multiplayer: boolean;
    static mut gametype: int16_t;
    static mut dedicated: boolean;
    static mut server: boolean;
    fn SendNetXCmd(id: netxcmd_t, param: *const libc::c_void, nparam: size_t);
    fn IsPlayerAdmin(playernum: int32_t) -> boolean;
    fn SendKick(playernum: uint8_t, msg: uint8_t);
    static mut cv_pointlimit: consvar_t;
    fn RegisterNetXCmd(
        id: netxcmd_t,
        cmd_f: Option::<unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> ()>,
    );
    static mut mapheaderinfo: [*mut mapheader_t; 1035];
    static mut consoleplayer: int32_t;
    static mut debugfile: *mut FILE;
    static mut cv_forceskin: consvar_t;
    static mut serverplayer: int32_t;
    static mut con_destlines: int32_t;
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
    fn Z_StrDup(in_0: *const libc::c_char) -> *mut libc::c_char;
    fn COM_Lua_f();
    static mut numskins: int32_t;
    static mut skins: [skin_t; 32];
    fn R_SkinAvailable(name: *const libc::c_char) -> int32_t;
    fn R_SkinUsable(playernum: int32_t, skinnum: int32_t) -> boolean;
    fn LUA_CVarChanged(cvar: *mut libc::c_void);
    static mut cv_nextmap: consvar_t;
    static mut cv_chooseskin: consvar_t;
    fn M_CanShowLevelInList(mapnum: int32_t, gt: int32_t) -> boolean;
    static mut cv_newgametype: consvar_t;
    static mut Color_cons_t: [CV_PossibleValue_t; 0];
    fn FIL_ReadFileTag(
        name: *const libc::c_char,
        buffer: *mut *mut uint8_t,
        tag: int32_t,
    ) -> size_t;
    fn axtoi(hexStg: *const libc::c_char) -> int32_t;
    fn M_Ftrim(_: libc::c_double) -> *const libc::c_char;
    static mut player_names: [[libc::c_char; 22]; 32];
    static mut cv_sideaxis: consvar_t;
    static mut cv_turnaxis: consvar_t;
    static mut cv_moveaxis: consvar_t;
    static mut cv_lookaxis: consvar_t;
    static mut cv_fireaxis: consvar_t;
    static mut cv_firenaxis: consvar_t;
    static mut cv_sideaxis2: consvar_t;
    static mut cv_turnaxis2: consvar_t;
    static mut cv_moveaxis2: consvar_t;
    static mut cv_lookaxis2: consvar_t;
    static mut cv_fireaxis2: consvar_t;
    static mut cv_firenaxis2: consvar_t;
    static mut demoplayback: boolean;
    fn HU_SetCEchoDuration(seconds: int32_t);
    fn HU_SetCEchoFlags(flags: int32_t);
    fn HU_DoCEcho(msg: *const libc::c_char);
    fn findfile(
        filename: *mut libc::c_char,
        wantedmd5sum: *const uint8_t,
        completepath: boolean,
    ) -> filestatus_t;
    fn D_CheckPathAllowed(
        path: *const libc::c_char,
        why: *const libc::c_char,
    ) -> boolean;
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
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
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type fixed_t = int32_t;
pub type mobjtype_t = mobj_type;
pub type mobj_type = libc::c_uint;
pub const NUMMOBJTYPES: mobj_type = 1163;
pub const MT_LASTFREESLOT: mobj_type = 1162;
pub const MT_FIRSTFREESLOT: mobj_type = 651;
pub const MT_RAY: mobj_type = 650;
pub const MT_NAMECHECK: mobj_type = 649;
pub const MT_YELLOWBRICKDEBRIS: mobj_type = 648;
pub const MT_BLUEBRICKDEBRIS: mobj_type = 647;
pub const MT_REDBRICKDEBRIS: mobj_type = 646;
pub const MT_WOODDEBRIS: mobj_type = 645;
pub const MT_BRICKDEBRIS: mobj_type = 644;
pub const MT_GFZDEBRIS: mobj_type = 643;
pub const MT_ROCKCRUMBLE16: mobj_type = 642;
pub const MT_ROCKCRUMBLE15: mobj_type = 641;
pub const MT_ROCKCRUMBLE14: mobj_type = 640;
pub const MT_ROCKCRUMBLE13: mobj_type = 639;
pub const MT_ROCKCRUMBLE12: mobj_type = 638;
pub const MT_ROCKCRUMBLE11: mobj_type = 637;
pub const MT_ROCKCRUMBLE10: mobj_type = 636;
pub const MT_ROCKCRUMBLE9: mobj_type = 635;
pub const MT_ROCKCRUMBLE8: mobj_type = 634;
pub const MT_ROCKCRUMBLE7: mobj_type = 633;
pub const MT_ROCKCRUMBLE6: mobj_type = 632;
pub const MT_ROCKCRUMBLE5: mobj_type = 631;
pub const MT_ROCKCRUMBLE4: mobj_type = 630;
pub const MT_ROCKCRUMBLE3: mobj_type = 629;
pub const MT_ROCKCRUMBLE2: mobj_type = 628;
pub const MT_ROCKCRUMBLE1: mobj_type = 627;
pub const MT_FALLINGROCK: mobj_type = 626;
pub const MT_ROCKSPAWNER: mobj_type = 625;
pub const MT_DUST: mobj_type = 624;
pub const MT_UWEXPLODE: mobj_type = 623;
pub const MT_EXPLODE: mobj_type = 622;
pub const MT_SPARK: mobj_type = 621;
pub const MT_SKYBOX: mobj_type = 620;
pub const MT_POLYSPAWN: mobj_type = 619;
pub const MT_POLYANCHOR: mobj_type = 618;
pub const MT_ANGLEMAN: mobj_type = 617;
pub const MT_OVERLAY: mobj_type = 616;
pub const MT_GHOST: mobj_type = 615;
pub const MT_PUSH: mobj_type = 614;
pub const MT_TUBEWAYPOINT: mobj_type = 613;
pub const MT_CRUMBLEOBJ: mobj_type = 612;
pub const MT_ALTVIEWMAN: mobj_type = 611;
pub const MT_TELEPORTMAN: mobj_type = 610;
pub const MT_HANGSTER: mobj_type = 609;
pub const MT_SPINBOBERT_FIRE2: mobj_type = 608;
pub const MT_SPINBOBERT_FIRE1: mobj_type = 607;
pub const MT_SPINBOBERT: mobj_type = 606;
pub const MT_CACOFIRE: mobj_type = 605;
pub const MT_CACOSHARD: mobj_type = 604;
pub const MT_CACOLANTERN: mobj_type = 603;
pub const MT_SMASHINGSPIKEBALL: mobj_type = 602;
pub const MT_BUGGLE: mobj_type = 601;
pub const MT_BUMBLEBORE: mobj_type = 600;
pub const MT_HIVEELEMENTAL: mobj_type = 599;
pub const MT_POPSHOT_TRAIL: mobj_type = 598;
pub const MT_POPSHOT: mobj_type = 597;
pub const MT_POPHAT: mobj_type = 596;
pub const MT_PENGUINATOR: mobj_type = 595;
pub const MT_SHLEEP: mobj_type = 594;
pub const MT_PIAN: mobj_type = 593;
pub const MT_NIGHTOPIANHELPER: mobj_type = 592;
pub const MT_IDEYAANCHOR: mobj_type = 591;
pub const MT_EGGCAPSULE: mobj_type = 590;
pub const MT_NIGHTSLINKFREEZE: mobj_type = 589;
pub const MT_NIGHTSEXTRATIME: mobj_type = 588;
pub const MT_NIGHTSHELPER: mobj_type = 587;
pub const MT_NIGHTSDRILLREFILL: mobj_type = 586;
pub const MT_NIGHTSSUPERLOOP: mobj_type = 585;
pub const MT_FLINGNIGHTSSTAR: mobj_type = 584;
pub const MT_NIGHTSSTAR: mobj_type = 583;
pub const MT_FLINGNIGHTSCHIP: mobj_type = 582;
pub const MT_NIGHTSCHIP: mobj_type = 581;
pub const MT_NIGHTSCORE: mobj_type = 580;
pub const MT_HOOPCENTER: mobj_type = 579;
pub const MT_HOOPCOLLIDE: mobj_type = 578;
pub const MT_HOOP: mobj_type = 577;
pub const MT_NIGHTSBUMPER: mobj_type = 576;
pub const MT_NIGHTSLOOPHELPER: mobj_type = 575;
pub const MT_NIGHTSPARKLE: mobj_type = 574;
pub const MT_NIGHTSDRONE_GOAL: mobj_type = 573;
pub const MT_NIGHTSDRONE_SPARKLING: mobj_type = 572;
pub const MT_NIGHTSDRONE_MAN: mobj_type = 571;
pub const MT_NIGHTSDRONE: mobj_type = 570;
pub const MT_AXISTRANSFERLINE: mobj_type = 569;
pub const MT_AXISTRANSFER: mobj_type = 568;
pub const MT_AXIS: mobj_type = 567;
pub const MT_TOAD: mobj_type = 566;
pub const MT_MARIOBUSH2: mobj_type = 565;
pub const MT_MARIOBUSH1: mobj_type = 564;
pub const MT_AXE: mobj_type = 563;
pub const MT_KOOPAFLAME: mobj_type = 562;
pub const MT_KOOPA: mobj_type = 561;
pub const MT_HAMMER: mobj_type = 560;
pub const MT_PUMATRAIL: mobj_type = 559;
pub const MT_PUMA: mobj_type = 558;
pub const MT_SHELL: mobj_type = 557;
pub const MT_FIREBALLTRAIL: mobj_type = 556;
pub const MT_FIREBALL: mobj_type = 555;
pub const MT_FIREFLOWER: mobj_type = 554;
pub const MT_BLUEGOOMBA: mobj_type = 553;
pub const MT_GOOMBA: mobj_type = 552;
pub const MT_FLINGCOIN: mobj_type = 551;
pub const MT_COIN: mobj_type = 550;
pub const MT_THROWNGRENADE: mobj_type = 549;
pub const MT_THROWNEXPLOSION: mobj_type = 548;
pub const MT_THROWNSCATTER: mobj_type = 547;
pub const MT_THROWNAUTOMATIC: mobj_type = 546;
pub const MT_THROWNINFINITY: mobj_type = 545;
pub const MT_THROWNBOUNCE: mobj_type = 544;
pub const MT_GRENADEPICKUP: mobj_type = 543;
pub const MT_SCATTERPICKUP: mobj_type = 542;
pub const MT_EXPLODEPICKUP: mobj_type = 541;
pub const MT_AUTOPICKUP: mobj_type = 540;
pub const MT_RAILPICKUP: mobj_type = 539;
pub const MT_BOUNCEPICKUP: mobj_type = 538;
pub const MT_GRENADERING: mobj_type = 537;
pub const MT_SCATTERRING: mobj_type = 536;
pub const MT_EXPLOSIONRING: mobj_type = 535;
pub const MT_AUTOMATICRING: mobj_type = 534;
pub const MT_INFINITYRING: mobj_type = 533;
pub const MT_RAILRING: mobj_type = 532;
pub const MT_BOUNCERING: mobj_type = 531;
pub const MT_REDRING: mobj_type = 530;
pub const MT_LHRT: mobj_type = 529;
pub const MT_CORK: mobj_type = 528;
pub const MT_AMBIENT: mobj_type = 527;
pub const MT_FINISHFLAG: mobj_type = 526;
pub const MT_GOTFLAG: mobj_type = 525;
pub const MT_TAG: mobj_type = 524;
pub const MT_LOCKONINF: mobj_type = 523;
pub const MT_LOCKON: mobj_type = 522;
pub const MT_GOTEMERALD: mobj_type = 521;
pub const MT_DROWNNUMBERS: mobj_type = 520;
pub const MT_SCORE: mobj_type = 519;
pub const MT_PARTICLEGEN: mobj_type = 518;
pub const MT_PARTICLE: mobj_type = 517;
pub const MT_TFOG: mobj_type = 516;
pub const MT_SPINDUST: mobj_type = 515;
pub const MT_WATERZAP: mobj_type = 514;
pub const MT_EXTRALARGEBUBBLE: mobj_type = 513;
pub const MT_MEDIUMBUBBLE: mobj_type = 512;
pub const MT_SMALLBUBBLE: mobj_type = 511;
pub const MT_SMOKE: mobj_type = 510;
pub const MT_LAVASPLISH: mobj_type = 509;
pub const MT_SPLISH: mobj_type = 508;
pub const MT_SNOWFLAKE: mobj_type = 507;
pub const MT_RAIN: mobj_type = 506;
pub const MT_SEED: mobj_type = 505;
pub const MT_SECRETFLICKY_02_CENTER: mobj_type = 504;
pub const MT_SECRETFLICKY_02: mobj_type = 503;
pub const MT_SECRETFLICKY_01_CENTER: mobj_type = 502;
pub const MT_SECRETFLICKY_01: mobj_type = 501;
pub const MT_FLICKY_16_CENTER: mobj_type = 500;
pub const MT_FLICKY_16: mobj_type = 499;
pub const MT_FLICKY_15_CENTER: mobj_type = 498;
pub const MT_FLICKY_15: mobj_type = 497;
pub const MT_FLICKY_14_CENTER: mobj_type = 496;
pub const MT_FLICKY_14: mobj_type = 495;
pub const MT_FLICKY_13_CENTER: mobj_type = 494;
pub const MT_FLICKY_13: mobj_type = 493;
pub const MT_FLICKY_12_CENTER: mobj_type = 492;
pub const MT_FLICKY_12: mobj_type = 491;
pub const MT_FLICKY_11_CENTER: mobj_type = 490;
pub const MT_FLICKY_11: mobj_type = 489;
pub const MT_FLICKY_10_CENTER: mobj_type = 488;
pub const MT_FLICKY_10: mobj_type = 487;
pub const MT_FLICKY_09_CENTER: mobj_type = 486;
pub const MT_FLICKY_09: mobj_type = 485;
pub const MT_FLICKY_08_CENTER: mobj_type = 484;
pub const MT_FLICKY_08: mobj_type = 483;
pub const MT_FLICKY_07_CENTER: mobj_type = 482;
pub const MT_FLICKY_07: mobj_type = 481;
pub const MT_FLICKY_06_CENTER: mobj_type = 480;
pub const MT_FLICKY_06: mobj_type = 479;
pub const MT_FLICKY_05_CENTER: mobj_type = 478;
pub const MT_FLICKY_05: mobj_type = 477;
pub const MT_FLICKY_04_CENTER: mobj_type = 476;
pub const MT_FLICKY_04: mobj_type = 475;
pub const MT_FLICKY_03_CENTER: mobj_type = 474;
pub const MT_FLICKY_03: mobj_type = 473;
pub const MT_FLICKY_02_CENTER: mobj_type = 472;
pub const MT_FLICKY_02: mobj_type = 471;
pub const MT_FLICKY_01_CENTER: mobj_type = 470;
pub const MT_FLICKY_01: mobj_type = 469;
pub const MT_SUPERSPARK: mobj_type = 468;
pub const MT_IVSP: mobj_type = 467;
pub const MT_THUNDERCOIN_SPARK: mobj_type = 466;
pub const MT_THUNDERCOIN_ORB: mobj_type = 465;
pub const MT_BUBBLEWRAP_ORB: mobj_type = 464;
pub const MT_FLAMEAURA_ORB: mobj_type = 463;
pub const MT_PITY_ORB: mobj_type = 462;
pub const MT_WHIRLWIND_ORB: mobj_type = 461;
pub const MT_ARMAGEDDON_ORB: mobj_type = 460;
pub const MT_FORCE_ORB: mobj_type = 459;
pub const MT_ATTRACT_ORB: mobj_type = 458;
pub const MT_ELEMENTAL_ORB: mobj_type = 457;
pub const MT_EGGSTATUE2: mobj_type = 456;
pub const MT_DBALL: mobj_type = 455;
pub const MT_PALMTREE_TOP: mobj_type = 454;
pub const MT_PALMTREE_TRUNK: mobj_type = 453;
pub const MT_BIG_PALMTREE_TOP: mobj_type = 452;
pub const MT_BIG_PALMTREE_TRUNK: mobj_type = 451;
pub const MT_BSZCLOVER: mobj_type = 450;
pub const MT_BSZSHRUB: mobj_type = 449;
pub const MT_BSZVINE_ORANGE: mobj_type = 448;
pub const MT_BSZVINE_YELLOW: mobj_type = 447;
pub const MT_BSZVINE_CYAN: mobj_type = 446;
pub const MT_BSZVINE_BLUE: mobj_type = 445;
pub const MT_BSZVINE_PURPLE: mobj_type = 444;
pub const MT_BSZVINE_RED: mobj_type = 443;
pub const MT_BSZBUSH_ORANGE: mobj_type = 442;
pub const MT_BSZBUSH_YELLOW: mobj_type = 441;
pub const MT_BSZBUSH_CYAN: mobj_type = 440;
pub const MT_BSZBUSH_BLUE: mobj_type = 439;
pub const MT_BSZBUSH_PURPLE: mobj_type = 438;
pub const MT_BSZBUSH_RED: mobj_type = 437;
pub const MT_BSZCLUSTER_ORANGE: mobj_type = 436;
pub const MT_BSZCLUSTER_YELLOW: mobj_type = 435;
pub const MT_BSZCLUSTER_CYAN: mobj_type = 434;
pub const MT_BSZCLUSTER_BLUE: mobj_type = 433;
pub const MT_BSZCLUSTER_PURPLE: mobj_type = 432;
pub const MT_BSZCLUSTER_RED: mobj_type = 431;
pub const MT_BSZTULIP_ORANGE: mobj_type = 430;
pub const MT_BSZTULIP_YELLOW: mobj_type = 429;
pub const MT_BSZTULIP_CYAN: mobj_type = 428;
pub const MT_BSZTULIP_BLUE: mobj_type = 427;
pub const MT_BSZTULIP_PURPLE: mobj_type = 426;
pub const MT_BSZTULIP_RED: mobj_type = 425;
pub const MT_BSZSHORTFLOWER_ORANGE: mobj_type = 424;
pub const MT_BSZSHORTFLOWER_YELLOW: mobj_type = 423;
pub const MT_BSZSHORTFLOWER_CYAN: mobj_type = 422;
pub const MT_BSZSHORTFLOWER_BLUE: mobj_type = 421;
pub const MT_BSZSHORTFLOWER_PURPLE: mobj_type = 420;
pub const MT_BSZSHORTFLOWER_RED: mobj_type = 419;
pub const MT_BSZFLOWER_ORANGE: mobj_type = 418;
pub const MT_BSZFLOWER_YELLOW: mobj_type = 417;
pub const MT_BSZFLOWER_CYAN: mobj_type = 416;
pub const MT_BSZFLOWER_BLUE: mobj_type = 415;
pub const MT_BSZFLOWER_PURPLE: mobj_type = 414;
pub const MT_BSZFLOWER_RED: mobj_type = 413;
pub const MT_BSZTALLFLOWER_ORANGE: mobj_type = 412;
pub const MT_BSZTALLFLOWER_YELLOW: mobj_type = 411;
pub const MT_BSZTALLFLOWER_CYAN: mobj_type = 410;
pub const MT_BSZTALLFLOWER_BLUE: mobj_type = 409;
pub const MT_BSZTALLFLOWER_PURPLE: mobj_type = 408;
pub const MT_BSZTALLFLOWER_RED: mobj_type = 407;
pub const MT_HHZSTALAGMITE_SHORT: mobj_type = 406;
pub const MT_HHZSTALAGMITE_TALL: mobj_type = 405;
pub const MT_HHZTENTACLE2: mobj_type = 404;
pub const MT_HHZTENTACLE1: mobj_type = 403;
pub const MT_HHZGRASS: mobj_type = 402;
pub const MT_HHZSHROOM: mobj_type = 401;
pub const MT_HHZTREE_PART: mobj_type = 400;
pub const MT_HHZTREE_TOP: mobj_type = 399;
pub const MT_JACKO3: mobj_type = 398;
pub const MT_JACKO2: mobj_type = 397;
pub const MT_JACKO1: mobj_type = 396;
pub const MT_CDLHRT: mobj_type = 395;
pub const MT_ROSY: mobj_type = 394;
pub const MT_FHZICE2: mobj_type = 393;
pub const MT_FHZICE1: mobj_type = 392;
pub const MT_XMASBUSH: mobj_type = 391;
pub const MT_XMASBERRYBUSH: mobj_type = 390;
pub const MT_XMASBLUEBERRYBUSH: mobj_type = 389;
pub const MT_MISTLETOE: mobj_type = 388;
pub const MT_HANGSTAR: mobj_type = 387;
pub const MT_LAMPPOST2: mobj_type = 386;
pub const MT_LAMPPOST1: mobj_type = 385;
pub const MT_SNOWMANHAT: mobj_type = 384;
pub const MT_SNOWMAN: mobj_type = 383;
pub const MT_CANDYCANE: mobj_type = 382;
pub const MT_XMASPOLE: mobj_type = 381;
pub const MT_STALAGMITE9: mobj_type = 380;
pub const MT_STALAGMITE8: mobj_type = 379;
pub const MT_STALAGMITE7: mobj_type = 378;
pub const MT_STALAGMITE6: mobj_type = 377;
pub const MT_STALAGMITE5: mobj_type = 376;
pub const MT_STALAGMITE4: mobj_type = 375;
pub const MT_STALAGMITE3: mobj_type = 374;
pub const MT_STALAGMITE2: mobj_type = 373;
pub const MT_STALAGMITE1: mobj_type = 372;
pub const MT_STALAGMITE0: mobj_type = 371;
pub const MT_BLUEGARGOYLE: mobj_type = 370;
pub const MT_GREENFLAME: mobj_type = 369;
pub const MT_TARGET: mobj_type = 368;
pub const MT_GLAREGOYLELONG: mobj_type = 367;
pub const MT_GLAREGOYLEDOWN: mobj_type = 366;
pub const MT_GLAREGOYLEUP: mobj_type = 365;
pub const MT_GLAREGOYLE: mobj_type = 364;
pub const MT_WALLVINE_SHORT: mobj_type = 363;
pub const MT_WALLVINE_LONG: mobj_type = 362;
pub const MT_TORCHFLOWER: mobj_type = 361;
pub const MT_JUNGLEPALM: mobj_type = 360;
pub const MT_BIGFERN: mobj_type = 359;
pub const MT_BIGFERNLEAF: mobj_type = 358;
pub const MT_ROLLOUTROCK: mobj_type = 357;
pub const MT_ROLLOUTSPAWN: mobj_type = 356;
pub const MT_LAVAFALLROCK: mobj_type = 355;
pub const MT_LAVAFALL_LAVA: mobj_type = 354;
pub const MT_LAVAFALL: mobj_type = 353;
pub const MT_FLAMEJETFLAMEB: mobj_type = 352;
pub const MT_FJSPINAXISB: mobj_type = 351;
pub const MT_FJSPINAXISA: mobj_type = 350;
pub const MT_FLAMEJETFLAME: mobj_type = 349;
pub const MT_VERTICALFLAMEJET: mobj_type = 348;
pub const MT_FLAMEJET: mobj_type = 347;
pub const MT_MINECARTSWITCHPOINT: mobj_type = 346;
pub const MT_TRAINSTEAMSPAWNER: mobj_type = 345;
pub const MT_TRAINDUSTSPAWNER: mobj_type = 344;
pub const MT_TRAINSEG: mobj_type = 343;
pub const MT_TRAINCAMEOSPAWNER: mobj_type = 342;
pub const MT_SALOONDOORCENTER: mobj_type = 341;
pub const MT_SALOONDOOR: mobj_type = 340;
pub const MT_MINECARTSPARK: mobj_type = 339;
pub const MT_MINECARTSIDEMARK: mobj_type = 338;
pub const MT_MINECARTENDSOLID: mobj_type = 337;
pub const MT_MINECARTEND: mobj_type = 336;
pub const MT_MINECARTSPAWNER: mobj_type = 335;
pub const MT_MINECARTSEG: mobj_type = 334;
pub const MT_MINECART: mobj_type = 333;
pub const MT_ARIDDUST: mobj_type = 332;
pub const MT_DUSTLAYER: mobj_type = 331;
pub const MT_DUSTDEVIL: mobj_type = 330;
pub const MT_PROXIMITYTNT: mobj_type = 329;
pub const MT_TNTBARREL: mobj_type = 328;
pub const MT_OILLAMP: mobj_type = 327;
pub const MT_ARIDSIGN_SHARPTURN: mobj_type = 326;
pub const MT_ARIDSIGN_CACTI: mobj_type = 325;
pub const MT_ARIDSIGN_CAUTION: mobj_type = 324;
pub const MT_CACTISMALLSEG: mobj_type = 323;
pub const MT_CACTITINYSEG: mobj_type = 322;
pub const MT_CACTI11: mobj_type = 321;
pub const MT_CACTI10: mobj_type = 320;
pub const MT_CACTI9: mobj_type = 319;
pub const MT_CACTI8: mobj_type = 318;
pub const MT_CACTI7: mobj_type = 317;
pub const MT_CACTI6: mobj_type = 316;
pub const MT_CACTI5: mobj_type = 315;
pub const MT_CACTI4: mobj_type = 314;
pub const MT_CACTI3: mobj_type = 313;
pub const MT_CACTI2: mobj_type = 312;
pub const MT_CACTI1: mobj_type = 311;
pub const MT_LITTLETUMBLEWEED: mobj_type = 310;
pub const MT_BIGTUMBLEWEED: mobj_type = 309;
pub const MT_BRAMBLES: mobj_type = 308;
pub const MT_SUSPICIOUSFACESTABBERSTATUE: mobj_type = 307;
pub const MT_FACESTABBERSTATUE: mobj_type = 306;
pub const MT_CRAWLASTATUE: mobj_type = 305;
pub const MT_WAVINGFLAGSEG2: mobj_type = 304;
pub const MT_WAVINGFLAGSEG1: mobj_type = 303;
pub const MT_WAVINGFLAG2: mobj_type = 302;
pub const MT_WAVINGFLAG1: mobj_type = 301;
pub const MT_FIRETORCH: mobj_type = 300;
pub const MT_FLAMEHOLDER: mobj_type = 299;
pub const MT_CANDLEPRICKET: mobj_type = 298;
pub const MT_CANDLE: mobj_type = 297;
pub const MT_CEZBUSH2: mobj_type = 296;
pub const MT_CEZBUSH1: mobj_type = 295;
pub const MT_PINETREE: mobj_type = 294;
pub const MT_CEZBANNER2: mobj_type = 293;
pub const MT_CEZBANNER1: mobj_type = 292;
pub const MT_CEZPOLE2: mobj_type = 291;
pub const MT_CEZPOLE1: mobj_type = 290;
pub const MT_CEZFLOWER: mobj_type = 289;
pub const MT_BIGFIREBAR: mobj_type = 288;
pub const MT_SMALLFIREBAR: mobj_type = 287;
pub const MT_REDSPRINGBALL: mobj_type = 286;
pub const MT_YELLOWSPRINGBALL: mobj_type = 285;
pub const MT_BIGGRABCHAIN: mobj_type = 284;
pub const MT_SMALLGRABCHAIN: mobj_type = 283;
pub const MT_BIGMACE: mobj_type = 282;
pub const MT_SMALLMACE: mobj_type = 281;
pub const MT_BIGMACECHAIN: mobj_type = 280;
pub const MT_SMALLMACECHAIN: mobj_type = 279;
pub const MT_CUSTOMMACEPOINT: mobj_type = 278;
pub const MT_FIREBARPOINT: mobj_type = 277;
pub const MT_HIDDEN_SLING: mobj_type = 276;
pub const MT_CHAINPOINT: mobj_type = 275;
pub const MT_SPRINGBALLPOINT: mobj_type = 274;
pub const MT_CHAINMACEPOINT: mobj_type = 273;
pub const MT_MACEPOINT: mobj_type = 272;
pub const MT_EGGSTATUE: mobj_type = 271;
pub const MT_FLAMEPARTICLE: mobj_type = 270;
pub const MT_FLAME: mobj_type = 269;
pub const MT_CHAIN: mobj_type = 268;
pub const MT_LIGHTBEAM: mobj_type = 267;
pub const MT_DSZ2STALAGMITE: mobj_type = 266;
pub const MT_DSZSTALAGMITE: mobj_type = 265;
pub const MT_ANIMALGAESEG: mobj_type = 264;
pub const MT_ANIMALGAETOP: mobj_type = 263;
pub const MT_KELP: mobj_type = 262;
pub const MT_BLUECRYSTAL: mobj_type = 261;
pub const MT_CORAL5: mobj_type = 260;
pub const MT_CORAL4: mobj_type = 259;
pub const MT_CORAL3: mobj_type = 258;
pub const MT_CORAL2: mobj_type = 257;
pub const MT_CORAL1: mobj_type = 256;
pub const MT_WATERDROP: mobj_type = 255;
pub const MT_WATERDRIP: mobj_type = 254;
pub const MT_SEAWEED: mobj_type = 253;
pub const MT_BIGGARGOYLE: mobj_type = 252;
pub const MT_GARGOYLE: mobj_type = 251;
pub const MT_ALARM: mobj_type = 250;
pub const MT_THZTREEBRANCH: mobj_type = 249;
pub const MT_THZTREE: mobj_type = 248;
pub const MT_THZFLOWER3: mobj_type = 247;
pub const MT_THZFLOWER2: mobj_type = 246;
pub const MT_THZFLOWER1: mobj_type = 245;
pub const MT_SPRINGTREE: mobj_type = 244;
pub const MT_BUSHREDTREE: mobj_type = 243;
pub const MT_BUSHTREE: mobj_type = 242;
pub const MT_POLYGONTREE: mobj_type = 241;
pub const MT_FHZPINKTREE: mobj_type = 240;
pub const MT_FHZTREE: mobj_type = 239;
pub const MT_CHECKERSUNSETTREE: mobj_type = 238;
pub const MT_CHECKERTREE: mobj_type = 237;
pub const MT_GFZCHERRYTREE: mobj_type = 236;
pub const MT_GFZBERRYTREE: mobj_type = 235;
pub const MT_GFZTREE: mobj_type = 234;
pub const MT_BUSH: mobj_type = 233;
pub const MT_BERRYBUSH: mobj_type = 232;
pub const MT_BLUEBERRYBUSH: mobj_type = 231;
pub const MT_GFZFLOWER3: mobj_type = 230;
pub const MT_GFZFLOWER2: mobj_type = 229;
pub const MT_GFZFLOWER1: mobj_type = 228;
pub const MT_TUTORIALFLOWERF: mobj_type = 227;
pub const MT_TUTORIALFLOWER: mobj_type = 226;
pub const MT_TUTORIALLEAF: mobj_type = 225;
pub const MT_TUTORIALPLANT: mobj_type = 224;
pub const MT_LETTER: mobj_type = 223;
pub const MT_DEMONFIRE: mobj_type = 222;
pub const MT_ARROW: mobj_type = 221;
pub const MT_CANNONBALLDECOR: mobj_type = 220;
pub const MT_CANNONBALL: mobj_type = 219;
pub const MT_TURRETLASER: mobj_type = 218;
pub const MT_JETTBULLET: mobj_type = 217;
pub const MT_MINE: mobj_type = 216;
pub const MT_ENERGYBALL: mobj_type = 215;
pub const MT_TORPEDO2: mobj_type = 214;
pub const MT_TORPEDO: mobj_type = 213;
pub const MT_LASER: mobj_type = 212;
pub const MT_ROCKET: mobj_type = 211;
pub const MT_THUNDERCOIN_ICON: mobj_type = 210;
pub const MT_BUBBLEWRAP_ICON: mobj_type = 209;
pub const MT_FLAMEAURA_ICON: mobj_type = 208;
pub const MT_SCORE10K_ICON: mobj_type = 207;
pub const MT_SCORE1K_ICON: mobj_type = 206;
pub const MT_RECYCLER_ICON: mobj_type = 205;
pub const MT_GRAVITY_ICON: mobj_type = 204;
pub const MT_MIXUP_ICON: mobj_type = 203;
pub const MT_EGGMAN_ICON: mobj_type = 202;
pub const MT_1UP_ICON: mobj_type = 201;
pub const MT_INVULN_ICON: mobj_type = 200;
pub const MT_SNEAKERS_ICON: mobj_type = 199;
pub const MT_ELEMENTAL_ICON: mobj_type = 198;
pub const MT_WHIRLWIND_ICON: mobj_type = 197;
pub const MT_ARMAGEDDON_ICON: mobj_type = 196;
pub const MT_FORCE_ICON: mobj_type = 195;
pub const MT_ATTRACT_ICON: mobj_type = 194;
pub const MT_PITY_ICON: mobj_type = 193;
pub const MT_RING_ICON: mobj_type = 192;
pub const MT_RING_BLUEBOX: mobj_type = 191;
pub const MT_RING_REDBOX: mobj_type = 190;
pub const MT_THUNDERCOIN_GOLDBOX: mobj_type = 189;
pub const MT_BUBBLEWRAP_GOLDBOX: mobj_type = 188;
pub const MT_FLAMEAURA_GOLDBOX: mobj_type = 187;
pub const MT_GRAVITY_GOLDBOX: mobj_type = 186;
pub const MT_EGGMAN_GOLDBOX: mobj_type = 185;
pub const MT_INVULN_GOLDBOX: mobj_type = 184;
pub const MT_SNEAKERS_GOLDBOX: mobj_type = 183;
pub const MT_ELEMENTAL_GOLDBOX: mobj_type = 182;
pub const MT_WHIRLWIND_GOLDBOX: mobj_type = 181;
pub const MT_ARMAGEDDON_GOLDBOX: mobj_type = 180;
pub const MT_FORCE_GOLDBOX: mobj_type = 179;
pub const MT_ATTRACT_GOLDBOX: mobj_type = 178;
pub const MT_PITY_GOLDBOX: mobj_type = 177;
pub const MT_THUNDERCOIN_BOX: mobj_type = 176;
pub const MT_BUBBLEWRAP_BOX: mobj_type = 175;
pub const MT_FLAMEAURA_BOX: mobj_type = 174;
pub const MT_SCORE10K_BOX: mobj_type = 173;
pub const MT_SCORE1K_BOX: mobj_type = 172;
pub const MT_RECYCLER_BOX: mobj_type = 171;
pub const MT_GRAVITY_BOX: mobj_type = 170;
pub const MT_MYSTERY_BOX: mobj_type = 169;
pub const MT_MIXUP_BOX: mobj_type = 168;
pub const MT_EGGMAN_BOX: mobj_type = 167;
pub const MT_1UP_BOX: mobj_type = 166;
pub const MT_INVULN_BOX: mobj_type = 165;
pub const MT_SNEAKERS_BOX: mobj_type = 164;
pub const MT_ELEMENTAL_BOX: mobj_type = 163;
pub const MT_WHIRLWIND_BOX: mobj_type = 162;
pub const MT_ARMAGEDDON_BOX: mobj_type = 161;
pub const MT_FORCE_BOX: mobj_type = 160;
pub const MT_ATTRACT_BOX: mobj_type = 159;
pub const MT_PITY_BOX: mobj_type = 158;
pub const MT_RING_BOX: mobj_type = 157;
pub const MT_BOXSPARKLE: mobj_type = 156;
pub const MT_CANNONLAUNCHER: mobj_type = 155;
pub const MT_BLASTEXECUTOR: mobj_type = 154;
pub const MT_BIGMINE: mobj_type = 153;
pub const MT_STARPOST: mobj_type = 152;
pub const MT_WALLSPIKEBASE: mobj_type = 151;
pub const MT_WALLSPIKE: mobj_type = 150;
pub const MT_SPIKE: mobj_type = 149;
pub const MT_SPINFIRE: mobj_type = 148;
pub const MT_SPIKEBALL: mobj_type = 147;
pub const MT_SIGN: mobj_type = 146;
pub const MT_BUBBLES: mobj_type = 145;
pub const MT_REDBOOSTER: mobj_type = 144;
pub const MT_YELLOWBOOSTER: mobj_type = 143;
pub const MT_BOOSTERROLLER: mobj_type = 142;
pub const MT_BOOSTERSEG: mobj_type = 141;
pub const MT_BLUEHORIZ: mobj_type = 140;
pub const MT_REDHORIZ: mobj_type = 139;
pub const MT_YELLOWHORIZ: mobj_type = 138;
pub const MT_BLUEDIAG: mobj_type = 137;
pub const MT_REDDIAG: mobj_type = 136;
pub const MT_YELLOWDIAG: mobj_type = 135;
pub const MT_BLUESPRING: mobj_type = 134;
pub const MT_REDSPRING: mobj_type = 133;
pub const MT_YELLOWSPRING: mobj_type = 132;
pub const MT_BALLOON: mobj_type = 131;
pub const MT_BUMPER: mobj_type = 130;
pub const MT_STEAM: mobj_type = 129;
pub const MT_FAN: mobj_type = 128;
pub const MT_FLINGEMERALD: mobj_type = 127;
pub const MT_EMERALDSPAWN: mobj_type = 126;
pub const MT_EMERHUNT: mobj_type = 125;
pub const MT_EMERALD7: mobj_type = 124;
pub const MT_EMERALD6: mobj_type = 123;
pub const MT_EMERALD5: mobj_type = 122;
pub const MT_EMERALD4: mobj_type = 121;
pub const MT_EMERALD3: mobj_type = 120;
pub const MT_EMERALD2: mobj_type = 119;
pub const MT_EMERALD1: mobj_type = 118;
pub const MT_EMBLEM: mobj_type = 117;
pub const MT_BLUEFLAG: mobj_type = 116;
pub const MT_REDFLAG: mobj_type = 115;
pub const MT_TOKEN: mobj_type = 114;
pub const MT_BLUETEAMRING: mobj_type = 113;
pub const MT_REDTEAMRING: mobj_type = 112;
pub const MT_BOMBSPHERE: mobj_type = 111;
pub const MT_FLINGBLUESPHERE: mobj_type = 110;
pub const MT_BLUESPHERE: mobj_type = 109;
pub const MT_FLINGRING: mobj_type = 108;
pub const MT_RING: mobj_type = 107;
pub const MT_MSGATHER: mobj_type = 106;
pub const MT_MSSHIELD_FRONT: mobj_type = 105;
pub const MT_METALSONIC_BATTLE: mobj_type = 104;
pub const MT_METALSONIC_RACE: mobj_type = 103;
pub const MT_CYBRAKDEMON_VILE_EXPLOSION: mobj_type = 102;
pub const MT_CYBRAKDEMON_NAPALM_FLAMES: mobj_type = 101;
pub const MT_CYBRAKDEMON_NAPALM_BOMB_SMALL: mobj_type = 100;
pub const MT_CYBRAKDEMON_NAPALM_BOMB_LARGE: mobj_type = 99;
pub const MT_CYBRAKDEMON_TARGET_DOT: mobj_type = 98;
pub const MT_CYBRAKDEMON_TARGET_RETICULE: mobj_type = 97;
pub const MT_CYBRAKDEMON_FLAMEREST: mobj_type = 96;
pub const MT_CYBRAKDEMON_FLAMESHOT: mobj_type = 95;
pub const MT_CYBRAKDEMON_MISSILE: mobj_type = 94;
pub const MT_CYBRAKDEMON_ELECTRIC_BARRIER: mobj_type = 93;
pub const MT_CYBRAKDEMON: mobj_type = 92;
pub const MT_BLACKEGGMAN_MISSILE: mobj_type = 91;
pub const MT_BLACKEGGMAN_GOOPFIRE: mobj_type = 90;
pub const MT_BLACKEGGMAN_HELPER: mobj_type = 89;
pub const MT_BLACKEGGMAN: mobj_type = 88;
pub const MT_FANGWAYPOINT: mobj_type = 87;
pub const MT_FSGNB: mobj_type = 86;
pub const MT_FSGNA: mobj_type = 85;
pub const MT_TNTDUST: mobj_type = 84;
pub const MT_FBOMB: mobj_type = 83;
pub const MT_PROJECTORLIGHT: mobj_type = 82;
pub const MT_VWREB: mobj_type = 81;
pub const MT_VWREF: mobj_type = 80;
pub const MT_BROKENROBOT: mobj_type = 79;
pub const MT_FANG: mobj_type = 78;
pub const MT_EGGROBO1JET: mobj_type = 77;
pub const MT_EGGROBO1: mobj_type = 76;
pub const MT_JETFLAME: mobj_type = 75;
pub const MT_EGGMOBILE4_MACE: mobj_type = 74;
pub const MT_EGGMOBILE4: mobj_type = 73;
pub const MT_SHOCKWAVE: mobj_type = 72;
pub const MT_FAKEMOBILE: mobj_type = 71;
pub const MT_EGGMOBILE3: mobj_type = 70;
pub const MT_GOOPTRAIL: mobj_type = 69;
pub const MT_GOOP: mobj_type = 68;
pub const MT_EGGMOBILE2_POGO: mobj_type = 67;
pub const MT_EGGMOBILE2: mobj_type = 66;
pub const MT_EGGMOBILE_FIRE: mobj_type = 65;
pub const MT_EGGMOBILE_TARGET: mobj_type = 64;
pub const MT_EGGMOBILE_BALL: mobj_type = 63;
pub const MT_JETFUME1: mobj_type = 62;
pub const MT_EGGMOBILE: mobj_type = 61;
pub const MT_BOSSJUNK: mobj_type = 60;
pub const MT_BOSS9GATHERPOINT: mobj_type = 59;
pub const MT_BOSS3WAYPOINT: mobj_type = 58;
pub const MT_EGGTRAP: mobj_type = 57;
pub const MT_BOSSFLYPOINT: mobj_type = 56;
pub const MT_SONIC3KBOSSEXPLODE: mobj_type = 55;
pub const MT_BOSSEXPLODE: mobj_type = 54;
pub const MT_DRAGONMINE: mobj_type = 53;
pub const MT_DRAGONTAIL: mobj_type = 52;
pub const MT_DRAGONWING: mobj_type = 51;
pub const MT_DRAGONBOMBER: mobj_type = 50;
pub const MT_PTERABYTE: mobj_type = 49;
pub const MT_PTERABYTEWAYPOINT: mobj_type = 48;
pub const MT_PTERABYTESPAWNER: mobj_type = 47;
pub const MT_PYREFLY_FIRE: mobj_type = 46;
pub const MT_PYREFLY: mobj_type = 45;
pub const MT_CANARIVORE_GAS: mobj_type = 44;
pub const MT_CANARIVORE: mobj_type = 43;
pub const MT_UNIBALL: mobj_type = 42;
pub const MT_UNIDUS: mobj_type = 41;
pub const MT_YELLOWSHELL: mobj_type = 40;
pub const MT_SPRINGSHELL: mobj_type = 39;
pub const MT_MINUSDIRT: mobj_type = 38;
pub const MT_MINUS: mobj_type = 37;
pub const MT_SNAPPER_HEAD: mobj_type = 36;
pub const MT_SNAPPER_LEG: mobj_type = 35;
pub const MT_GSNAPPER: mobj_type = 34;
pub const MT_EGGSHIELD: mobj_type = 33;
pub const MT_EGGGUARD: mobj_type = 32;
pub const MT_FACESTABBERSPEAR: mobj_type = 31;
pub const MT_FACESTABBER: mobj_type = 30;
pub const MT_ROBOHOOD: mobj_type = 29;
pub const MT_POINTYBALL: mobj_type = 28;
pub const MT_POINTY: mobj_type = 27;
pub const MT_VULTURE: mobj_type = 26;
pub const MT_SNAILER: mobj_type = 25;
pub const MT_JETJAW: mobj_type = 24;
pub const MT_BANPSPRING: mobj_type = 23;
pub const MT_BANPYURA: mobj_type = 22;
pub const MT_CRUSHCHAIN: mobj_type = 21;
pub const MT_CRUSHCLAW: mobj_type = 20;
pub const MT_CRUSHSTACEAN: mobj_type = 19;
pub const MT_SPINCUSHION: mobj_type = 18;
pub const MT_POPUPTURRET: mobj_type = 17;
pub const MT_TURRET: mobj_type = 16;
pub const MT_SKIM: mobj_type = 15;
pub const MT_DETON: mobj_type = 14;
pub const MT_CRAWLACOMMANDER: mobj_type = 13;
pub const MT_JETTGUNNER: mobj_type = 12;
pub const MT_JETTBOMBER: mobj_type = 11;
pub const MT_REDBUZZ: mobj_type = 10;
pub const MT_GOLDBUZZ: mobj_type = 9;
pub const MT_GFZFISH: mobj_type = 8;
pub const MT_REDCRAWLA: mobj_type = 7;
pub const MT_BLUECRAWLA: mobj_type = 6;
pub const MT_METALJETFUME: mobj_type = 5;
pub const MT_TAILSOVERLAY: mobj_type = 4;
pub const MT_PLAYER: mobj_type = 3;
pub const MT_THOK: mobj_type = 2;
pub const MT_UNKNOWN: mobj_type = 1;
pub const MT_NULL: mobj_type = 0;
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
pub struct nightsgrades_t {
    pub grade: [uint32_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct customoption_t {
    pub option: [libc::c_char; 32],
    pub value: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapheader_t {
    pub lvlttl: [libc::c_char; 22],
    pub subttl: [libc::c_char; 33],
    pub actnum: uint8_t,
    pub typeoflevel: uint32_t,
    pub nextlevel: int16_t,
    pub marathonnext: int16_t,
    pub keywords: [libc::c_char; 33],
    pub musname: [libc::c_char; 7],
    pub mustrack: uint16_t,
    pub muspos: uint32_t,
    pub forcecharacter: [libc::c_char; 17],
    pub weather: uint8_t,
    pub skynum: int16_t,
    pub skybox_scalex: int16_t,
    pub skybox_scaley: int16_t,
    pub skybox_scalez: int16_t,
    pub interscreen: [libc::c_char; 8],
    pub runsoc: [libc::c_char; 33],
    pub scriptname: [libc::c_char; 33],
    pub precutscenenum: uint8_t,
    pub cutscenenum: uint8_t,
    pub countdown: int16_t,
    pub palette: uint16_t,
    pub numlaps: uint8_t,
    pub unlockrequired: int8_t,
    pub levelselect: uint8_t,
    pub bonustype: int8_t,
    pub maxbonuslives: int8_t,
    pub levelflags: uint16_t,
    pub menuflags: uint8_t,
    pub selectheading: [libc::c_char; 22],
    pub startrings: uint16_t,
    pub sstimer: int32_t,
    pub ssspheres: uint32_t,
    pub gravity: fixed_t,
    pub ltzzpatch: [libc::c_char; 9],
    pub ltzztext: [libc::c_char; 9],
    pub ltactdiamond: [libc::c_char; 9],
    pub numFlickies: uint8_t,
    pub flickies: *mut mobjtype_t,
    pub numGradedMares: uint8_t,
    pub grades: *mut nightsgrades_t,
    pub musinterfadeout: uint32_t,
    pub musintername: [libc::c_char; 7],
    pub muspostbossname: [libc::c_char; 7],
    pub muspostbosstrack: uint16_t,
    pub muspostbosspos: uint32_t,
    pub muspostbossfadein: uint32_t,
    pub musforcereset: int8_t,
    pub numCustomOptions: uint8_t,
    pub customopts: *mut customoption_t,
}
pub type GameType = libc::c_uint;
pub const NUMGAMETYPES: GameType = 136;
pub const GT_LASTFREESLOT: GameType = 135;
pub const GT_FIRSTFREESLOT: GameType = 8;
pub const GT_CTF: GameType = 7;
pub const GT_HIDEANDSEEK: GameType = 6;
pub const GT_TAG: GameType = 5;
pub const GT_TEAMMATCH: GameType = 4;
pub const GT_MATCH: GameType = 3;
pub const GT_RACE: GameType = 2;
pub const GT_COMPETITION: GameType = 1;
pub const GT_COOP: GameType = 0;
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
pub type com_func_t = Option::<unsafe extern "C" fn() -> ()>;
pub type xcommand_t = xcommand_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xcommand_s {
    pub name: *const libc::c_char,
    pub next: *mut xcommand_s,
    pub function: com_func_t,
    pub flags: com_flags_t,
}
pub const PU_STATIC: C2RustUnnamed_3 = 1;
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
pub type cmdalias_t = cmdalias_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
pub type vsbuf_t = vsbuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vsbuf_s {
    pub allowoverflow: boolean,
    pub overflowed: boolean,
    pub data: *mut uint8_t,
    pub maxsize: size_t,
    pub cursize: size_t,
}
pub const CV_CALL: C2RustUnnamed_2 = 2;
pub const CV_MODIFIED: C2RustUnnamed_2 = 64;
pub const CV_SHOWMODIFONETIME: C2RustUnnamed_2 = 256;
pub const CV_SHOWMODIF: C2RustUnnamed_2 = 128;
pub const CV_FLOAT: C2RustUnnamed_2 = 16;
pub const CV_NOTINNET: C2RustUnnamed_2 = 32;
pub type netxcmd_t = libc::c_uint;
pub const MAXNETXCMD: netxcmd_t = 25;
pub const XD_LUAFILE: netxcmd_t = 24;
pub const XD_LUAVAR: netxcmd_t = 23;
pub const XD_LUACMD: netxcmd_t = 22;
pub const XD_DEMOTED: netxcmd_t = 21;
pub const XD_SUICIDE: netxcmd_t = 20;
pub const XD_SETMOTD: netxcmd_t = 19;
pub const XD_REQADDFOLDER: netxcmd_t = 18;
pub const XD_REQADDFILE: netxcmd_t = 17;
pub const XD_RUNSOC: netxcmd_t = 16;
pub const XD_RANDOMSEED: netxcmd_t = 15;
pub const XD_VERIFIED: netxcmd_t = 14;
pub const XD_CLEARSCORES: netxcmd_t = 13;
pub const XD_TEAMCHANGE: netxcmd_t = 12;
pub const XD_ADDPLAYER: netxcmd_t = 11;
pub const XD_PAUSE: netxcmd_t = 10;
pub const XD_ADDFOLDER: netxcmd_t = 9;
pub const XD_ADDFILE: netxcmd_t = 8;
pub const XD_EXITLEVEL: netxcmd_t = 7;
pub const XD_MAP: netxcmd_t = 6;
pub const XD_SAY: netxcmd_t = 5;
pub const XD_NETVAR: netxcmd_t = 4;
pub const XD_KICK: netxcmd_t = 3;
pub const XD_WEAPONPREF: netxcmd_t = 2;
pub const XD_NAMEANDCOLOR: netxcmd_t = 1;
pub const CV_NETVAR: C2RustUnnamed_2 = 4;
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
pub struct spriteinfo_t {
    pub pivot: [spriteframepivot_t; 64],
    pub available: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spriteframepivot_t {
    pub x: int32_t,
    pub y: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spritedef_t {
    pub numframes: size_t,
    pub spriteframes: *mut spriteframe_t,
}
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
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
}
pub const CV_SAVE: C2RustUnnamed_2 = 1;
pub const CV_ALLOWLUA: C2RustUnnamed_2 = 4096;
pub const CV_NOSHOWHELP: C2RustUnnamed_2 = 512;
pub const CV_CHEAT: C2RustUnnamed_2 = 2048;
pub const FS_NOTFOUND: filestatus_t = 1;
pub type filestatus_t = libc::c_uint;
pub const FS_MD5SUMBAD: filestatus_t = 6;
pub const FS_OPEN: filestatus_t = 5;
pub const FS_DOWNLOADING: filestatus_t = 4;
pub const FS_REQUESTED: filestatus_t = 3;
pub const FS_FOUND: filestatus_t = 2;
pub const FS_NOTCHECKED: filestatus_t = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const CV_HIDEN: C2RustUnnamed_2 = 1024;
pub const CV_NOINIT: C2RustUnnamed_2 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct old_demo_var {
    pub checksum: uint16_t,
    pub collides: boolean,
    pub cvar: *mut consvar_t,
    pub next: *mut old_demo_var_t,
}
pub type old_demo_var_t = old_demo_var;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_3 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_3 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_3 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_3 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_3 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_3 = 51;
pub const PU_LEVEL: C2RustUnnamed_3 = 50;
pub const PU_CACHE: C2RustUnnamed_3 = 49;
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn FixedToFloat(mut x: fixed_t) -> libc::c_float {
    return x as libc::c_float
        / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float;
}
static mut consvar_vars: *mut consvar_t = 0 as *const consvar_t as *mut consvar_t;
static mut consvar_number_of_netids: uint16_t = 0 as libc::c_int as uint16_t;
static mut consvar_old_demo_vars: *mut old_demo_var_t = 0 as *const old_demo_var_t
    as *mut old_demo_var_t;
static mut com_token: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut CV_OnOff: [CV_PossibleValue_t; 3] = [
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
            value: 0 as libc::c_int,
            strvalue: 0 as *const libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub static mut CV_YesNo: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"No\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"Yes\0" as *const u8 as *const libc::c_char,
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
pub static mut CV_Unsigned: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 999999999 as libc::c_int,
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
#[no_mangle]
pub static mut CV_Natural: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 999999999 as libc::c_int,
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
#[no_mangle]
pub static mut CV_TrueFalse: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"False\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"True\0" as *const u8 as *const libc::c_char,
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
static mut execversion_enabled: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut cv_execversion: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"execversion\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"25\0" as *const u8 as *const libc::c_char,
            flags: CV_CALL as libc::c_int,
            PossibleValue: CV_Unsigned.as_ptr() as *mut _,
            func: Some(CV_EnforceExecVersion as unsafe extern "C" fn() -> ()),
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
static mut joyaxis_default: boolean = false_0 as libc::c_int;
static mut joyaxis2_default: boolean = false_0 as libc::c_int;
static mut joyaxis_count: int32_t = 0 as libc::c_int;
static mut joyaxis2_count: int32_t = 0 as libc::c_int;
static mut com_wait: int32_t = 0;
static mut com_alias: *mut cmdalias_t = 0 as *const cmdalias_t as *mut cmdalias_t;
static mut com_text: vsbuf_t = vsbuf_s {
    allowoverflow: 0,
    overflowed: 0,
    data: 0 as *const uint8_t as *mut uint8_t,
    maxsize: 0,
    cursize: 0,
};
static mut com_flags: com_flags_t = 0 as com_flags_t;
unsafe extern "C" fn COM_Purge(
    mut s: *mut libc::c_char,
    mut np: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    n = strlen(s) as libc::c_int;
    t = s.offset(n as isize).offset(1 as libc::c_int as isize);
    p = s;
    loop {
        p = strchr(p, '\u{1b}' as i32);
        if p.is_null() {
            break;
        }
        memmove(
            p as *mut libc::c_void,
            &mut *p.offset(1 as libc::c_int as isize) as *mut libc::c_char
                as *const libc::c_void,
            (t.offset_from(p) as libc::c_long - 1 as libc::c_int as libc::c_long)
                as libc::c_ulong,
        );
        n -= 1;
        n;
    }
    if !np.is_null() {
        *np = n;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn COM_BufAddTextEx(
    mut ptext: *const libc::c_char,
    mut flags: com_flags_t,
) {
    let mut l: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    text = COM_Purge(Z_StrDup(ptext), &mut l);
    if (com_text.cursize)
        .wrapping_add(2 as libc::c_int as size_t)
        .wrapping_add(l as size_t) >= com_text.maxsize
    {
        CONS_Alert(
            CONS_WARNING,
            b"Command buffer full!\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    VS_WriteEx(&mut com_text, text as *const libc::c_void, l as size_t, flags);
    Z_Free(text as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn COM_BufInsertTextEx(
    mut ptext: *const libc::c_char,
    mut flags: com_flags_t,
) {
    let old_wait: int32_t = com_wait;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut templen: size_t = 0;
    templen = com_text.cursize;
    if templen != 0 {
        temp = M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            Z_MallocAlign(
                templen,
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ),
            com_text.data as *const libc::c_void,
            templen,
        ) as *mut libc::c_char;
        VS_Clear(&mut com_text);
    }
    com_wait = 0 as libc::c_int;
    COM_BufAddTextEx(ptext, flags);
    COM_BufExecute();
    com_wait += old_wait;
    if templen != 0 {
        VS_Write(&mut com_text, temp as *const libc::c_void, templen);
        Z_Free(temp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn COM_BufTicker() {
    if com_wait != 0 {
        com_wait -= 1;
        com_wait;
        return;
    }
    COM_BufExecute();
}
#[no_mangle]
pub unsafe extern "C" fn COM_BufExecute() {
    let mut i: size_t = 0;
    let mut ptext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 1024] = *::core::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut quotes: int32_t = 0;
    while com_text.cursize != 0 {
        ptext = com_text.data as *mut libc::c_char;
        quotes = 0 as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while i < com_text.cursize {
            if *ptext.offset(i as isize) as libc::c_int == '"' as i32 && quotes == 0
                && i > 0 as libc::c_int as size_t
                && *ptext.offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != ' ' as i32
            {
                break;
            }
            if *ptext.offset(i as isize) as libc::c_int == '"' as i32 {
                quotes += 1;
                quotes;
            }
            if quotes & 1 as libc::c_int == 0
                && *ptext.offset(i as isize) as libc::c_int == ';' as i32
            {
                break;
            }
            if *ptext.offset(i as isize) as libc::c_int == '\n' as i32
                || *ptext.offset(i as isize) as libc::c_int == '\r' as i32
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(line.as_mut_ptr() as *mut libc::c_void, ptext as *const libc::c_void, i);
        line[i as usize] = 0 as libc::c_int as libc::c_char;
        if i == com_text.cursize {
            com_text.cursize = 0 as libc::c_int as size_t;
        } else {
            i = i.wrapping_add(1);
            i;
            com_text.cursize = (com_text.cursize).wrapping_sub(i);
            memmove(
                ptext as *mut libc::c_void,
                ptext.offset(i as isize) as *const libc::c_void,
                com_text.cursize,
            );
        }
        COM_ExecuteString(line.as_mut_ptr());
        if !(com_wait != 0) {
            continue;
        }
        com_wait -= 1;
        com_wait;
        break;
    }
    com_flags = 0 as com_flags_t;
}
#[no_mangle]
pub unsafe extern "C" fn COM_ImmedExecute(mut ptext: *const libc::c_char) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    let mut line: [libc::c_char; 1024] = *::core::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut quotes: int32_t = 0;
    while i < strlen(ptext) {
        quotes = 0 as libc::c_int;
        j = 0 as libc::c_int as size_t;
        while i < strlen(ptext) {
            if *ptext.offset(i as isize) as libc::c_int == '"' as i32 && quotes == 0
                && i > 0 as libc::c_int as size_t
                && *ptext.offset(i.wrapping_sub(1 as libc::c_int as size_t) as isize)
                    as libc::c_int != ' ' as i32
            {
                return;
            }
            if *ptext.offset(i as isize) as libc::c_int == '"' as i32 {
                quotes += 1;
                quotes;
            }
            if quotes & 1 as libc::c_int == 0
                && *ptext.offset(i as isize) as libc::c_int == ';' as i32
                || *ptext.offset(i as isize) as libc::c_int == '\n' as i32
                || *ptext.offset(i as isize) as libc::c_int == '\r' as i32
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
            j = j.wrapping_add(1);
            j;
        }
        memcpy(
            line.as_mut_ptr() as *mut libc::c_void,
            ptext.offset(i.wrapping_sub(j) as isize) as *const libc::c_void,
            j,
        );
        line[j as usize] = 0 as libc::c_int as libc::c_char;
        COM_ExecuteString(line.as_mut_ptr());
        i = i.wrapping_add(1);
        i;
    }
}
static mut com_commands: *mut xcommand_t = 0 as *const xcommand_t as *mut xcommand_t;
static mut com_argc: size_t = 0;
static mut com_argv: [*mut libc::c_char; 80] = [0 as *const libc::c_char
    as *mut libc::c_char; 80];
static mut com_null_string: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
static mut com_args: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn COM_Init() {
    VS_Alloc(&mut com_text, ((32 as libc::c_int) << 10 as libc::c_int) as size_t);
    COM_AddCommand(
        b"alias\0" as *const u8 as *const libc::c_char,
        Some(COM_Alias_f as unsafe extern "C" fn() -> ()),
        0 as com_flags_t,
    );
    COM_AddCommand(
        b"echo\0" as *const u8 as *const libc::c_char,
        Some(COM_Echo_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"cecho\0" as *const u8 as *const libc::c_char,
        Some(COM_CEcho_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"cechoflags\0" as *const u8 as *const libc::c_char,
        Some(COM_CEchoFlags_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"cechoduration\0" as *const u8 as *const libc::c_char,
        Some(COM_CEchoDuration_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"exec\0" as *const u8 as *const libc::c_char,
        Some(COM_Exec_f as unsafe extern "C" fn() -> ()),
        0 as com_flags_t,
    );
    COM_AddCommand(
        b"wait\0" as *const u8 as *const libc::c_char,
        Some(COM_Wait_f as unsafe extern "C" fn() -> ()),
        0 as com_flags_t,
    );
    COM_AddCommand(
        b"help\0" as *const u8 as *const libc::c_char,
        Some(COM_Help_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"toggle\0" as *const u8 as *const libc::c_char,
        Some(COM_Toggle_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"add\0" as *const u8 as *const libc::c_char,
        Some(COM_Add_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    RegisterNetXCmd(
        XD_NETVAR,
        Some(Got_NetVar as unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn COM_Argc() -> size_t {
    return com_argc;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Argv(mut arg: size_t) -> *const libc::c_char {
    if arg >= com_argc || (arg as libc::c_int) < 0 as libc::c_int {
        return com_null_string;
    }
    return com_argv[arg as usize];
}
#[no_mangle]
pub unsafe extern "C" fn COM_Args() -> *mut libc::c_char {
    return com_args;
}
#[no_mangle]
pub unsafe extern "C" fn COM_CheckParm(mut check: *const libc::c_char) -> size_t {
    let mut i: size_t = 0;
    i = 1 as libc::c_int as size_t;
    while i < com_argc {
        if strcasecmp(check, com_argv[i as usize]) == 0 {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn COM_CheckPartialParm(mut check: *const libc::c_char) -> size_t {
    let mut len: libc::c_int = 0;
    let mut i: size_t = 0;
    len = strlen(check) as libc::c_int;
    i = 1 as libc::c_int as size_t;
    while i < com_argc {
        if strncasecmp(check, com_argv[i as usize], len as libc::c_ulong)
            == 0 as libc::c_int
        {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn COM_FirstOption() -> size_t {
    let mut i: size_t = 0;
    i = 1 as libc::c_int as size_t;
    while i < com_argc {
        if *(com_argv[i as usize]).offset(0 as libc::c_int as isize) as libc::c_int
            == '-' as i32
        {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn COM_TokenizeString(mut ptext: *mut libc::c_char) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < com_argc {
        Z_Free(com_argv[i as usize] as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
    com_argc = 0 as libc::c_int as size_t;
    com_args = 0 as *mut libc::c_char;
    while com_argc < 80 as libc::c_int as size_t {
        while *ptext as libc::c_int != '\0' as i32 && *ptext as libc::c_int <= ' ' as i32
            && *ptext as libc::c_int != '\n' as i32
        {
            if *ptext.offset(0 as libc::c_int as isize) as libc::c_int == '\u{1b}' as i32
            {
                com_flags = *ptext.offset(1 as libc::c_int as isize) as libc::c_uint
                    as com_flags_t;
                ptext = ptext.offset(2 as libc::c_int as isize);
            } else {
                ptext = ptext.offset(1);
                ptext;
            }
        }
        if *ptext as libc::c_int == '\n' as i32 || *ptext as libc::c_int == '\0' as i32 {
            break;
        }
        if com_argc == 1 as libc::c_int as size_t {
            com_args = ptext;
        }
        ptext = COM_Parse(ptext);
        if ptext.is_null() {
            break;
        }
        com_argv[com_argc as usize] = Z_StrDup(com_token.as_mut_ptr());
        com_argc = com_argc.wrapping_add(1);
        com_argc;
    }
}
#[no_mangle]
pub unsafe extern "C" fn COM_AddCommand(
    mut name: *const libc::c_char,
    mut func: com_func_t,
    mut flags: com_flags_t,
) {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    if *(CV_StringValue(name)).offset(0 as libc::c_int as isize) as libc::c_int
        != '\0' as i32
    {
        I_Error(b"%s is a variable name\n\0" as *const u8 as *const libc::c_char, name);
    }
    cmd = com_commands;
    while !cmd.is_null() {
        if strcasecmp(name, (*cmd).name) == 0 {
            if (*cmd).function != Some(COM_Lua_f as unsafe extern "C" fn() -> ()) {
                I_Error(
                    b"Command %s already exists\n\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            return;
        }
        cmd = (*cmd).next;
    }
    cmd = Z_MallocAlign(
        ::core::mem::size_of::<xcommand_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut xcommand_t;
    (*cmd).name = name;
    (*cmd).function = func;
    (*cmd).flags = flags;
    (*cmd).next = com_commands;
    com_commands = cmd;
}
#[no_mangle]
pub unsafe extern "C" fn COM_AddLuaCommand(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    if *(CV_StringValue(name)).offset(0 as libc::c_int as isize) as libc::c_int
        != '\0' as i32
    {
        return -(1 as libc::c_int);
    }
    cmd = com_commands;
    while !cmd.is_null() {
        if strcasecmp(name, (*cmd).name) == 0 {
            (*cmd).function = Some(COM_Lua_f as unsafe extern "C" fn() -> ());
            return 1 as libc::c_int;
        }
        cmd = (*cmd).next;
    }
    cmd = Z_MallocAlign(
        ::core::mem::size_of::<xcommand_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut xcommand_t;
    (*cmd).name = name;
    (*cmd).function = Some(COM_Lua_f as unsafe extern "C" fn() -> ());
    (*cmd).flags = COM_LUA;
    (*cmd).next = com_commands;
    com_commands = cmd;
    return 0 as libc::c_int;
}
unsafe extern "C" fn COM_Exists(mut com_name: *const libc::c_char) -> boolean {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    cmd = com_commands;
    while !cmd.is_null() {
        if strcasecmp(com_name, (*cmd).name) == 0 {
            return true_0 as libc::c_int;
        }
        cmd = (*cmd).next;
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn COM_CompleteCommand(
    mut partial: *const libc::c_char,
    mut skips: int32_t,
) -> *const libc::c_char {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    let mut len: size_t = 0;
    len = strlen(partial);
    if len == 0 {
        return 0 as *const libc::c_char;
    }
    cmd = com_commands;
    while !cmd.is_null() {
        if strncmp(partial, (*cmd).name, len) == 0 {
            let fresh0 = skips;
            skips = skips - 1;
            if fresh0 == 0 {
                return (*cmd).name;
            }
        }
        cmd = (*cmd).next;
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn COM_CompleteAlias(
    mut partial: *const libc::c_char,
    mut skips: int32_t,
) -> *const libc::c_char {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut len: size_t = 0;
    len = strlen(partial);
    if len == 0 {
        return 0 as *const libc::c_char;
    }
    a = com_alias;
    while !a.is_null() {
        if strncmp(partial, (*a).name, len) == 0 {
            let fresh1 = skips;
            skips = skips - 1;
            if fresh1 == 0 {
                return (*a).name;
            }
        }
        a = (*a).next;
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn COM_ExecuteString(mut ptext: *mut libc::c_char) {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    static mut recursion: int32_t = 0 as libc::c_int;
    COM_TokenizeString(ptext);
    if COM_Argc() == 0 as libc::c_int as size_t {
        return;
    }
    cmd = com_commands;
    while !cmd.is_null() {
        if strcasecmp(com_argv[0 as libc::c_int as usize], (*cmd).name) == 0 {
            if com_flags as libc::c_uint & COM_LUA as libc::c_int as libc::c_uint != 0
                && (*cmd).flags as libc::c_uint & COM_LUA as libc::c_int as libc::c_uint
                    == 0
            {
                CONS_Alert(
                    CONS_WARNING,
                    b"Command '%s' cannot be run from Lua.\n\0" as *const u8
                        as *const libc::c_char,
                    (*cmd).name,
                );
                return;
            }
            ((*cmd).function).expect("non-null function pointer")();
            return;
        }
        cmd = (*cmd).next;
    }
    a = com_alias;
    while !a.is_null() {
        if strcasecmp(com_argv[0 as libc::c_int as usize], (*a).name) == 0 {
            if recursion > 100 as libc::c_int {
                CONS_Alert(
                    CONS_WARNING,
                    b"Alias recursion cycle detected!\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                recursion += 1;
                recursion;
                COM_BufInsertTextEx((*a).value, com_flags);
                recursion -= 1;
                recursion;
            }
            return;
        }
        a = (*a).next;
    }
    if CV_Command() == 0 && (con_destlines != 0 || dedicated != 0) {
        CONS_Printf(
            b"Unknown command '%s'\n\0" as *const u8 as *const libc::c_char,
            COM_Argv(0 as libc::c_int as size_t),
        );
    }
}
unsafe extern "C" fn print_alias() {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    CONS_Printf(b"\x82Current alias commands:\n\0" as *const u8 as *const libc::c_char);
    a = com_alias;
    while !a.is_null() {
        CONS_Printf(
            b"%s : %s\0" as *const u8 as *const libc::c_char,
            (*a).name,
            (*a).value,
        );
        a = (*a).next;
    }
}
unsafe extern "C" fn add_alias(
    mut newname: *mut libc::c_char,
    mut newcmd: *mut libc::c_char,
) {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    a = com_alias;
    while !a.is_null() {
        if strcasecmp(newname, (*a).name) == 0 {
            Z_Free((*a).value as *mut libc::c_void);
            (*a).value = newcmd;
            return;
        }
        a = (*a).next;
    }
    a = Z_MallocAlign(
        ::core::mem::size_of::<cmdalias_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut cmdalias_t;
    (*a).next = com_alias;
    com_alias = a;
    (*a).name = newname;
    (*a).value = newcmd;
}
unsafe extern "C" fn COM_Alias_f() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut zcmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    let mut i: size_t = 0;
    let mut c: size_t = 0;
    if COM_Argc() < 3 as libc::c_int as size_t {
        CONS_Printf(
            b"alias <name> <command>: create a shortcut command that executes other command(s)\n\0"
                as *const u8 as *const libc::c_char,
        );
        print_alias();
        return;
    }
    name = Z_StrDup(COM_Argv(1 as libc::c_int as size_t));
    cmd[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    c = COM_Argc();
    i = 2 as libc::c_int as size_t;
    while i < c {
        strcat(cmd.as_mut_ptr(), COM_Argv(i));
        if i != c {
            strcat(cmd.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
    }
    strcat(cmd.as_mut_ptr(), b"\n\0" as *const u8 as *const libc::c_char);
    zcmd = Z_StrDup(cmd.as_mut_ptr());
    add_alias(name, zcmd);
}
unsafe extern "C" fn COM_Echo_f() {
    let mut i: size_t = 0;
    i = 1 as libc::c_int as size_t;
    while i < COM_Argc() {
        CONS_Printf(b"%s \0" as *const u8 as *const libc::c_char, COM_Argv(i));
        i = i.wrapping_add(1);
        i;
    }
    CONS_Printf(b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn COM_CEcho_f() {
    let mut i: size_t = 0;
    let mut cechotext: [libc::c_char; 1024] = *::core::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    i = 1 as libc::c_int as size_t;
    while i < COM_Argc() {
        strncat(
            cechotext.as_mut_ptr(),
            COM_Argv(i),
            (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        strncat(
            cechotext.as_mut_ptr(),
            b" \0" as *const u8 as *const libc::c_char,
            (::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        i = i.wrapping_add(1);
        i;
    }
    cechotext[(::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    HU_DoCEcho(cechotext.as_mut_ptr());
}
unsafe extern "C" fn COM_CEchoFlags_f() {
    if COM_Argc() > 1 as libc::c_int as size_t {
        let mut arg: *const libc::c_char = COM_Argv(1 as libc::c_int as size_t);
        if *arg.offset(0 as libc::c_int as isize) as libc::c_int != 0
            && *arg.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            && *arg.offset(1 as libc::c_int as isize) as libc::c_int != 0
            && *arg.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
        {
            HU_SetCEchoFlags(axtoi(arg.offset(2 as libc::c_int as isize)));
        } else {
            HU_SetCEchoFlags(atoi(arg));
        }
    } else {
        CONS_Printf(
            b"cechoflags <flags>: set CEcho flags, prepend with 0x to use hexadecimal\n\0"
                as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn COM_CEchoDuration_f() {
    if COM_Argc() > 1 as libc::c_int as size_t {
        HU_SetCEchoDuration(atoi(COM_Argv(1 as libc::c_int as size_t)));
    }
}
unsafe extern "C" fn COM_Exec_f() {
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut filename: [libc::c_char; 256] = [0; 256];
    if COM_Argc() < 2 as libc::c_int as size_t || COM_Argc() > 3 as libc::c_int as size_t
    {
        CONS_Printf(
            b"exec <filename>: run a script file\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if D_CheckPathAllowed(
        COM_Argv(1 as libc::c_int as size_t),
        b"tried to exec\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return;
    }
    FIL_ReadFileTag(
        COM_Argv(1 as libc::c_int as size_t),
        &mut buf,
        PU_STATIC as libc::c_int,
    );
    if buf.is_null() {
        strcpy(filename.as_mut_ptr(), COM_Argv(1 as libc::c_int as size_t));
        if findfile(filename.as_mut_ptr(), 0 as *const uint8_t, true_0 as libc::c_int)
            as libc::c_uint != FS_NOTFOUND as libc::c_int as libc::c_uint
        {
            FIL_ReadFileTag(filename.as_mut_ptr(), &mut buf, PU_STATIC as libc::c_int);
        }
        if buf.is_null() {
            if COM_CheckParm(b"-noerror\0" as *const u8 as *const libc::c_char) == 0 {
                CONS_Printf(
                    b"couldn't execute file %s\n\0" as *const u8 as *const libc::c_char,
                    COM_Argv(1 as libc::c_int as size_t),
                );
            }
            return;
        }
    }
    if COM_CheckParm(b"-silent\0" as *const u8 as *const libc::c_char) == 0 {
        CONS_Printf(
            b"executing %s\n\0" as *const u8 as *const libc::c_char,
            COM_Argv(1 as libc::c_int as size_t),
        );
    }
    COM_BufAddTextEx(buf as *mut libc::c_char, com_flags);
    COM_BufAddTextEx(b"\n\0" as *const u8 as *const libc::c_char, com_flags);
    Z_Free(buf as *mut libc::c_void);
}
unsafe extern "C" fn COM_Wait_f() {
    if COM_Argc() > 1 as libc::c_int as size_t {
        com_wait = atoi(COM_Argv(1 as libc::c_int as size_t));
    } else {
        com_wait = 1 as libc::c_int;
    };
}
unsafe extern "C" fn COM_Help_f() {
    let mut cmd: *mut xcommand_t = 0 as *mut xcommand_t;
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    let mut i: int32_t = 0 as libc::c_int;
    if COM_Argc() > 1 as libc::c_int as size_t {
        let mut help: *const libc::c_char = COM_Argv(1 as libc::c_int as size_t);
        cvar = CV_FindVar(help);
        if !cvar.is_null() {
            let mut floatmode: boolean = false_0 as libc::c_int;
            let mut cvalue: *const libc::c_char = 0 as *const libc::c_char;
            CONS_Printf(
                b"\x82Variable %s:\n\0" as *const u8 as *const libc::c_char,
                (*cvar).name,
            );
            CONS_Printf(b"  flags :\0" as *const u8 as *const libc::c_char);
            if (*cvar).flags & CV_SAVE as libc::c_int != 0 {
                CONS_Printf(b"AUTOSAVE \0" as *const u8 as *const libc::c_char);
            }
            if (*cvar).flags & CV_FLOAT as libc::c_int != 0 {
                CONS_Printf(b"FLOAT \0" as *const u8 as *const libc::c_char);
                floatmode = true_0 as libc::c_int;
            }
            if (*cvar).flags & CV_NETVAR as libc::c_int != 0 {
                CONS_Printf(b"NETVAR \0" as *const u8 as *const libc::c_char);
            }
            if (*cvar).flags & CV_CALL as libc::c_int != 0 {
                CONS_Printf(b"ACTION \0" as *const u8 as *const libc::c_char);
            }
            if (*cvar).flags & CV_CHEAT as libc::c_int != 0 {
                CONS_Printf(b"CHEAT \0" as *const u8 as *const libc::c_char);
            }
            CONS_Printf(b"\n\0" as *const u8 as *const libc::c_char);
            if !((*cvar).PossibleValue).is_null() {
                CONS_Printf(
                    b" Possible values:\n\0" as *const u8 as *const libc::c_char,
                );
                if (*cvar).PossibleValue == CV_YesNo.as_mut_ptr() {
                    CONS_Printf(
                        b"  Yes or No (On or Off, True or False, 1 or 0)\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*cvar).PossibleValue == CV_OnOff.as_mut_ptr() {
                    CONS_Printf(
                        b"  On or Off (Yes or No, True or False, 1 or 0)\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*cvar).PossibleValue == CV_TrueFalse.as_mut_ptr() {
                    CONS_Printf(
                        b"  True or False (On or Off, Yes or No, 1 or 0)\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*cvar).PossibleValue == Color_cons_t.as_mut_ptr() {
                    i = 1 as libc::c_int;
                    while i < numskincolors as libc::c_int {
                        if skincolors[i as usize].accessible != 0 {
                            CONS_Printf(
                                b"  %-2d : %s\n\0" as *const u8 as *const libc::c_char,
                                i,
                                (skincolors[i as usize].name).as_mut_ptr(),
                            );
                            if i == (*cvar).value {
                                cvalue = (skincolors[i as usize].name).as_mut_ptr();
                            }
                        }
                        i += 1;
                        i;
                    }
                } else {
                    if strcasecmp(
                        (*((*cvar).PossibleValue).offset(0 as libc::c_int as isize))
                            .strvalue,
                        b"MIN\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if floatmode != 0 {
                            let mut fu: libc::c_float = FixedToFloat(
                                (*((*cvar).PossibleValue).offset(0 as libc::c_int as isize))
                                    .value,
                            );
                            let mut ck: libc::c_float = FixedToFloat(
                                (*((*cvar).PossibleValue).offset(1 as libc::c_int as isize))
                                    .value,
                            );
                            CONS_Printf(
                                b"  range from %ld%s to %ld%s\n\0" as *const u8
                                    as *const libc::c_char,
                                fu as libc::c_long,
                                M_Ftrim(fu as libc::c_double),
                                ck as libc::c_long,
                                M_Ftrim(ck as libc::c_double),
                            );
                        } else {
                            CONS_Printf(
                                b"  range from %d to %d\n\0" as *const u8
                                    as *const libc::c_char,
                                (*((*cvar).PossibleValue).offset(0 as libc::c_int as isize))
                                    .value,
                                (*((*cvar).PossibleValue).offset(1 as libc::c_int as isize))
                                    .value,
                            );
                        }
                        i = 1 as libc::c_int + 1 as libc::c_int;
                    }
                    while !((*((*cvar).PossibleValue).offset(i as isize)).strvalue)
                        .is_null()
                    {
                        if floatmode != 0 {
                            CONS_Printf(
                                b"  %-2f : %s\n\0" as *const u8 as *const libc::c_char,
                                FixedToFloat(
                                    (*((*cvar).PossibleValue).offset(i as isize)).value,
                                ) as libc::c_double,
                                (*((*cvar).PossibleValue).offset(i as isize)).strvalue,
                            );
                        } else {
                            CONS_Printf(
                                b"  %-2d : %s\n\0" as *const u8 as *const libc::c_char,
                                (*((*cvar).PossibleValue).offset(i as isize)).value,
                                (*((*cvar).PossibleValue).offset(i as isize)).strvalue,
                            );
                        }
                        if (*((*cvar).PossibleValue).offset(i as isize)).value
                            == (*cvar).value
                        {
                            cvalue = (*((*cvar).PossibleValue).offset(i as isize))
                                .strvalue;
                        }
                        i += 1;
                        i;
                    }
                }
            }
            if !cvalue.is_null() {
                CONS_Printf(
                    b" Current value: %s\n\0" as *const u8 as *const libc::c_char,
                    cvalue,
                );
            } else if !((*cvar).string).is_null() {
                CONS_Printf(
                    b" Current value: %s\n\0" as *const u8 as *const libc::c_char,
                    (*cvar).string,
                );
            } else {
                CONS_Printf(
                    b" Current value: %d\n\0" as *const u8 as *const libc::c_char,
                    (*cvar).value,
                );
            }
            if !((*cvar).revert.v.string).is_null()
                && strcmp((*cvar).revert.v.string, (*cvar).string) != 0 as libc::c_int
            {
                CONS_Printf(
                    b" Value before netgame: %s\n\0" as *const u8 as *const libc::c_char,
                    (*cvar).revert.v.string,
                );
            }
        } else {
            cmd = com_commands;
            while !cmd.is_null() {
                if strcmp((*cmd).name, help) != 0 {
                    cmd = (*cmd).next;
                } else {
                    CONS_Printf(
                        b"\x82Command %s:\n\0" as *const u8 as *const libc::c_char,
                        (*cmd).name,
                    );
                    CONS_Printf(
                        b"  help is not available for commands\0" as *const u8
                            as *const libc::c_char,
                    );
                    CONS_Printf(
                        b"\x82\nCheck wiki.srb2.org for more or try typing <name> without arguments\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
            CONS_Printf(
                b"No exact match, searching...\n\0" as *const u8 as *const libc::c_char,
            );
            CONS_Printf(b"\x82Variables:\n\0" as *const u8 as *const libc::c_char);
            cvar = consvar_vars;
            while !cvar.is_null() {
                if !((*cvar).flags & CV_NOSHOWHELP as libc::c_int != 0
                    || (strstr((*cvar).name, help)).is_null())
                {
                    CONS_Printf(
                        b"%s \0" as *const u8 as *const libc::c_char,
                        (*cvar).name,
                    );
                    i += 1;
                    i;
                }
                cvar = (*cvar).next;
            }
            CONS_Printf(b"\x82\nCommands:\n\0" as *const u8 as *const libc::c_char);
            cmd = com_commands;
            while !cmd.is_null() {
                if !(strstr((*cmd).name, help)).is_null() {
                    CONS_Printf(
                        b"%s \0" as *const u8 as *const libc::c_char,
                        (*cmd).name,
                    );
                    i += 1;
                    i;
                }
                cmd = (*cmd).next;
            }
            CONS_Printf(
                b"\x82\nCheck wiki.srb2.org for more or type help <command or variable>\n\0"
                    as *const u8 as *const libc::c_char,
            );
            CONS_Debug(
                0x80 as libc::c_int,
                b"\x87Total : %d\n\0" as *const u8 as *const libc::c_char,
                i,
            );
        }
        return;
    } else {
        CONS_Printf(b"\x82Variables:\n\0" as *const u8 as *const libc::c_char);
        cvar = consvar_vars;
        while !cvar.is_null() {
            if !((*cvar).flags & CV_NOSHOWHELP as libc::c_int != 0) {
                CONS_Printf(b"%s \0" as *const u8 as *const libc::c_char, (*cvar).name);
                i += 1;
                i;
            }
            cvar = (*cvar).next;
        }
        CONS_Printf(b"\x82\nCommands:\n\0" as *const u8 as *const libc::c_char);
        cmd = com_commands;
        while !cmd.is_null() {
            CONS_Printf(b"%s \0" as *const u8 as *const libc::c_char, (*cmd).name);
            i += 1;
            i;
            cmd = (*cmd).next;
        }
        CONS_Printf(
            b"\x82\nCheck wiki.srb2.org for more or type help <command or variable>\n\0"
                as *const u8 as *const libc::c_char,
        );
        CONS_Debug(
            0x80 as libc::c_int,
            b"\x82Total : %d\n\0" as *const u8 as *const libc::c_char,
            i,
        );
    };
}
unsafe extern "C" fn COM_Toggle_f() {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    if COM_Argc() != 2 as libc::c_int as size_t {
        CONS_Printf(
            b"Toggle <cvar_name>: Toggle the value of a cvar\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    cvar = CV_FindVar(COM_Argv(1 as libc::c_int as size_t));
    if cvar.is_null() {
        CONS_Alert(
            CONS_NOTICE,
            b"%s is not a cvar\n\0" as *const u8 as *const libc::c_char,
            COM_Argv(1 as libc::c_int as size_t),
        );
        return;
    }
    if CV_Immutable(cvar) != 0 {
        return;
    }
    if !((*cvar).PossibleValue == CV_YesNo.as_mut_ptr()
        || (*cvar).PossibleValue == CV_OnOff.as_mut_ptr()
        || (*cvar).PossibleValue == CV_TrueFalse.as_mut_ptr())
    {
        CONS_Alert(
            CONS_NOTICE,
            b"%s is not a boolean value\n\0" as *const u8 as *const libc::c_char,
            COM_Argv(1 as libc::c_int as size_t),
        );
        return;
    }
    (*cvar).flags |= CV_SHOWMODIFONETIME as libc::c_int;
    CV_AddValue(cvar, 1 as libc::c_int);
}
unsafe extern "C" fn COM_Add_f() {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    if COM_Argc() != 3 as libc::c_int as size_t {
        CONS_Printf(
            b"Add <cvar_name> <value>: Add to the value of a cvar. Negative values work too!\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    cvar = CV_FindVar(COM_Argv(1 as libc::c_int as size_t));
    if cvar.is_null() {
        CONS_Alert(
            CONS_NOTICE,
            b"%s is not a cvar\n\0" as *const u8 as *const libc::c_char,
            COM_Argv(1 as libc::c_int as size_t),
        );
        return;
    }
    if CV_Immutable(cvar) != 0 {
        return;
    }
    if (*cvar).flags & CV_FLOAT as libc::c_int != 0 {
        let mut n: libc::c_float = (FixedToFloat((*cvar).value) as libc::c_double
            + atof(COM_Argv(2 as libc::c_int as size_t))) as libc::c_float;
        CV_Set(
            cvar,
            va(
                b"%ld%s\0" as *const u8 as *const libc::c_char,
                n as libc::c_long,
                M_Ftrim(n as libc::c_double),
            ),
        );
    } else {
        CV_AddValue(cvar, atoi(COM_Argv(2 as libc::c_int as size_t)));
    };
}
#[no_mangle]
pub unsafe extern "C" fn VS_Alloc(mut buf: *mut vsbuf_t, mut initsize: size_t) {
    if initsize < 256 as libc::c_int as size_t {
        initsize = 256 as libc::c_int as size_t;
    }
    (*buf)
        .data = Z_MallocAlign(
        initsize,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut uint8_t;
    (*buf).maxsize = initsize;
    (*buf).cursize = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn VS_Free(mut buf: *mut vsbuf_t) {
    (*buf).cursize = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn VS_Clear(mut buf: *mut vsbuf_t) {
    (*buf).cursize = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn VS_GetSpace(
    mut buf: *mut vsbuf_t,
    mut length: size_t,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*buf).cursize).wrapping_add(length) > (*buf).maxsize {
        if (*buf).allowoverflow == 0 {
            I_Error(b"overflow 111\0" as *const u8 as *const libc::c_char);
        }
        if length > (*buf).maxsize {
            I_Error(
                b"overflow l%s 112\0" as *const u8 as *const libc::c_char,
                sizeu1(length),
            );
        }
        (*buf).overflowed = true_0 as libc::c_int;
        CONS_Printf(b"VS buffer overflow\0" as *const u8 as *const libc::c_char);
        VS_Clear(buf);
    }
    data = ((*buf).data).offset((*buf).cursize as isize) as *mut libc::c_void;
    (*buf).cursize = ((*buf).cursize).wrapping_add(length);
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn VS_Write(
    mut buf: *mut vsbuf_t,
    mut data: *const libc::c_void,
    mut length: size_t,
) {
    M_Memcpy.expect("non-null function pointer")(VS_GetSpace(buf, length), data, length);
}
#[no_mangle]
pub unsafe extern "C" fn VS_WriteEx(
    mut buf: *mut vsbuf_t,
    mut data: *const libc::c_void,
    mut length: size_t,
    mut flags: com_flags_t,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = VS_GetSpace(buf, (2 as libc::c_int as size_t).wrapping_add(length))
        as *mut libc::c_char;
    *p.offset(0 as libc::c_int as isize) = '\u{1b}' as i32 as libc::c_char;
    *p.offset(1 as libc::c_int as isize) = flags as libc::c_char;
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        &mut *p.offset(2 as libc::c_int as isize) as *mut libc::c_char
            as *mut libc::c_void,
        data,
        length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn VS_Print(mut buf: *mut vsbuf_t, mut data: *const libc::c_char) {
    let mut len: size_t = 0;
    len = (strlen(data)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    if *((*buf).data)
        .offset(((*buf).cursize).wrapping_sub(1 as libc::c_int as size_t) as isize) != 0
    {
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            VS_GetSpace(buf, len) as *mut uint8_t as *mut libc::c_void,
            data as *const libc::c_void,
            len,
        );
    } else {
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            (VS_GetSpace(buf, len.wrapping_sub(1 as libc::c_int as size_t))
                as *mut uint8_t)
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
            data as *const libc::c_void,
            len,
        );
    };
}
static mut cv_null_string: *const libc::c_char = b"\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn CV_FindVar(mut name: *const libc::c_char) -> *mut consvar_t {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if strcasecmp(name, (*cvar).name) == 0 {
            return cvar;
        }
        cvar = (*cvar).next;
    }
    return 0 as *mut consvar_t;
}
#[inline]
unsafe extern "C" fn CV_ComputeOldDemoID(mut s: *const libc::c_char) -> uint16_t {
    let mut ret: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    static mut premiers: [uint16_t; 16] = [
        2 as libc::c_int as uint16_t,
        3 as libc::c_int as uint16_t,
        5 as libc::c_int as uint16_t,
        7 as libc::c_int as uint16_t,
        11 as libc::c_int as uint16_t,
        13 as libc::c_int as uint16_t,
        17 as libc::c_int as uint16_t,
        19 as libc::c_int as uint16_t,
        23 as libc::c_int as uint16_t,
        29 as libc::c_int as uint16_t,
        31 as libc::c_int as uint16_t,
        37 as libc::c_int as uint16_t,
        41 as libc::c_int as uint16_t,
        43 as libc::c_int as uint16_t,
        47 as libc::c_int as uint16_t,
        53 as libc::c_int as uint16_t,
    ];
    while *s != 0 {
        ret = (ret as libc::c_int
            + *s as libc::c_int * premiers[i as usize] as libc::c_int) as uint16_t;
        s = s.offset(1);
        s;
        i = ((i as libc::c_int + 1 as libc::c_int) % 16 as libc::c_int) as uint16_t;
    }
    return ret;
}
unsafe extern "C" fn CV_FindOldDemoVar(mut chk: uint16_t) -> *mut old_demo_var_t {
    let mut demovar: *mut old_demo_var_t = 0 as *mut old_demo_var_t;
    demovar = consvar_old_demo_vars;
    while !demovar.is_null() {
        if (*demovar).checksum as libc::c_int == chk as libc::c_int {
            if (*demovar).collides != 0 {
                CONS_Alert(
                    CONS_WARNING,
                    b"Old demo netvar id %hu is a collision\n\0" as *const u8
                        as *const libc::c_char,
                    chk as libc::c_int,
                );
                return 0 as *mut old_demo_var_t;
            }
            return demovar;
        }
        demovar = (*demovar).next;
    }
    return 0 as *mut old_demo_var_t;
}
unsafe extern "C" fn CV_FindNetVar(mut netid: uint16_t) -> *mut consvar_t {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    if netid as libc::c_int > consvar_number_of_netids as libc::c_int {
        return 0 as *mut consvar_t;
    }
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).netid as libc::c_int == netid as libc::c_int {
            return cvar;
        }
        cvar = (*cvar).next;
    }
    return 0 as *mut consvar_t;
}
unsafe extern "C" fn CV_RegisterOldDemoVar(mut variable: *mut consvar_t) {
    let mut demovar: *mut old_demo_var_t = 0 as *mut old_demo_var_t;
    let mut old_demo_id: uint16_t = 0;
    old_demo_id = CV_ComputeOldDemoID((*variable).name);
    demovar = CV_FindOldDemoVar(old_demo_id);
    if !demovar.is_null() {
        (*demovar).collides = true_0 as libc::c_int;
    } else {
        demovar = Z_CallocAlign(
            ::core::mem::size_of::<old_demo_var_t>() as libc::c_ulong,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut old_demo_var_t;
        (*demovar).checksum = old_demo_id;
        (*demovar).cvar = variable;
        (*demovar).next = consvar_old_demo_vars;
        consvar_old_demo_vars = demovar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn CV_RegisterVar(mut variable: *mut consvar_t) {
    if !(CV_FindVar((*variable).name)).is_null() {
        CONS_Printf(
            b"Variable %s is already defined\n\0" as *const u8 as *const libc::c_char,
            (*variable).name,
        );
        return;
    }
    if COM_Exists((*variable).name) != 0 {
        CONS_Printf(
            b"%s is a command name\n\0" as *const u8 as *const libc::c_char,
            (*variable).name,
        );
        return;
    }
    if (*variable).flags & CV_NETVAR as libc::c_int != 0 {
        if consvar_number_of_netids as libc::c_int == 65535 as libc::c_int {
            I_Error(b"Way too many netvars\0" as *const u8 as *const libc::c_char);
        }
        consvar_number_of_netids = consvar_number_of_netids.wrapping_add(1);
        (*variable).netid = consvar_number_of_netids;
        CV_RegisterOldDemoVar(variable);
    }
    if (*variable).flags & CV_HIDEN as libc::c_int == 0 {
        (*variable).next = consvar_vars;
        consvar_vars = variable;
    }
    (*variable).zstring = 0 as *mut libc::c_char;
    (*variable).string = (*variable).zstring;
    memset(
        &mut (*variable).revert as *mut C2RustUnnamed_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
    );
    (*variable).changed = 0 as libc::c_int as libc::c_char;
    if (*variable).flags & CV_NOINIT as libc::c_int != 0 {
        (*variable).flags &= !(CV_CALL as libc::c_int);
    }
    Setvalue(variable, (*variable).defaultvalue, false_0 as libc::c_int);
    if (*variable).flags & CV_NOINIT as libc::c_int != 0 {
        (*variable).flags |= CV_CALL as libc::c_int;
    }
    (*variable).flags &= !(CV_MODIFIED as libc::c_int);
}
unsafe extern "C" fn CV_StringValue(
    mut var_name: *const libc::c_char,
) -> *const libc::c_char {
    let mut var: *mut consvar_t = 0 as *mut consvar_t;
    var = CV_FindVar(var_name);
    if var.is_null() {
        return cv_null_string;
    }
    return (*var).string;
}
#[no_mangle]
pub unsafe extern "C" fn CV_CompleteVar(
    mut partial: *mut libc::c_char,
    mut skips: int32_t,
) -> *const libc::c_char {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    let mut len: size_t = 0;
    len = strlen(partial);
    if len == 0 {
        return 0 as *const libc::c_char;
    }
    cvar = consvar_vars;
    while !cvar.is_null() {
        if strncmp(partial, (*cvar).name, len) == 0 {
            let fresh2 = skips;
            skips = skips - 1;
            if fresh2 == 0 {
                return (*cvar).name;
            }
        }
        cvar = (*cvar).next;
    }
    return 0 as *const libc::c_char;
}
static mut serverloading: boolean = false_0 as libc::c_int;
unsafe extern "C" fn ReadNetVar(
    mut p: *mut *mut uint8_t,
    mut return_value: *mut *mut libc::c_char,
    mut return_stealth: *mut boolean,
) -> *mut consvar_t {
    let mut netid: uint16_t = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stealth: boolean = 0;
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    netid = ({
        let mut p_tmp: *mut uint16_t = *p as *mut libc::c_void as *mut uint16_t;
        let mut b: uint16_t = 0;
        memcpy(
            &mut b as *mut uint16_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    });
    val = *p as *mut libc::c_char;
    while ({
        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void as *mut libc::c_char;
        let mut b: libc::c_char = 0;
        memcpy(
            &mut b as *mut libc::c_char as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as libc::c_int != '\0' as i32
    {}
    stealth = ({
        let mut p_tmp: *mut uint8_t = *p as *mut libc::c_void as *mut uint8_t;
        let mut b: uint8_t = 0;
        memcpy(
            &mut b as *mut uint8_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as boolean;
    cvar = CV_FindNetVar(netid);
    if !cvar.is_null() {
        *return_value = val;
        *return_stealth = stealth;
        if !debugfile.is_null() {
            fputs(
                va(
                    b"Netvar received: %s [netid=%d] value %s\n\0" as *const u8
                        as *const libc::c_char,
                    (*cvar).name,
                    netid as libc::c_int,
                    val,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
    } else {
        CONS_Alert(
            CONS_WARNING,
            b"Netvar not found with netid %hu\n\0" as *const u8 as *const libc::c_char,
            netid as libc::c_int,
        );
    }
    return cvar;
}
unsafe extern "C" fn ReadOldDemoVar(
    mut p: *mut *mut uint8_t,
    mut return_value: *mut *mut libc::c_char,
    mut return_stealth: *mut boolean,
) -> *mut consvar_t {
    let mut id: uint16_t = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stealth: boolean = 0;
    let mut demovar: *mut old_demo_var_t = 0 as *mut old_demo_var_t;
    id = ({
        let mut p_tmp: *mut uint16_t = *p as *mut libc::c_void as *mut uint16_t;
        let mut b: uint16_t = 0;
        memcpy(
            &mut b as *mut uint16_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    });
    val = *p as *mut libc::c_char;
    while ({
        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void as *mut libc::c_char;
        let mut b: libc::c_char = 0;
        memcpy(
            &mut b as *mut libc::c_char as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as libc::c_int != '\0' as i32
    {}
    stealth = ({
        let mut p_tmp: *mut uint8_t = *p as *mut libc::c_void as *mut uint8_t;
        let mut b: uint8_t = 0;
        memcpy(
            &mut b as *mut uint8_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as boolean;
    demovar = CV_FindOldDemoVar(id);
    if !demovar.is_null() {
        *return_value = val;
        *return_stealth = stealth;
        return (*demovar).cvar;
    } else {
        CONS_Alert(
            CONS_WARNING,
            b"Netvar not found with old demo id %hu\n\0" as *const u8
                as *const libc::c_char,
            id as libc::c_int,
        );
        return 0 as *mut consvar_t;
    };
}
unsafe extern "C" fn ReadDemoVar(
    mut p: *mut *mut uint8_t,
    mut return_value: *mut *mut libc::c_char,
    mut return_stealth: *mut boolean,
) -> *mut consvar_t {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stealth: boolean = 0;
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    name = *p as *mut libc::c_char;
    while ({
        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void as *mut libc::c_char;
        let mut b: libc::c_char = 0;
        memcpy(
            &mut b as *mut libc::c_char as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as libc::c_int != '\0' as i32
    {}
    val = *p as *mut libc::c_char;
    while ({
        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void as *mut libc::c_char;
        let mut b: libc::c_char = 0;
        memcpy(
            &mut b as *mut libc::c_char as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as libc::c_int != '\0' as i32
    {}
    stealth = ({
        let mut p_tmp: *mut uint8_t = *p as *mut libc::c_void as *mut uint8_t;
        let mut b: uint8_t = 0;
        memcpy(
            &mut b as *mut uint8_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as boolean;
    cvar = CV_FindVar(name);
    if !cvar.is_null() {
        *return_value = val;
        *return_stealth = stealth;
    } else {
        CONS_Alert(
            CONS_WARNING,
            b"Netvar not found with name %s\n\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return cvar;
}
unsafe extern "C" fn Got_NetVar(mut p: *mut *mut uint8_t, mut playernum: int32_t) {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    let mut svalue: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stealth: boolean = 0;
    if playernum != serverplayer && IsPlayerAdmin(playernum) == 0 && serverloading == 0 {
        CONS_Alert(
            CONS_WARNING,
            b"Illegal netvar command received from %s\n\0" as *const u8
                as *const libc::c_char,
            (player_names[playernum as usize]).as_mut_ptr(),
        );
        if server != 0 {
            SendKick(
                playernum as uint8_t,
                (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
            );
        }
        return;
    }
    cvar = ReadNetVar(p, &mut svalue, &mut stealth);
    if !cvar.is_null() {
        Setvalue(cvar, svalue, stealth);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CV_SaveVars(mut p: *mut *mut uint8_t, mut in_demo: boolean) {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    let mut count_p: *mut uint8_t = *p;
    let mut count: uint16_t = 0 as libc::c_int as uint16_t;
    let mut p_tmp: *mut uint16_t = *p as *mut libc::c_void as *mut uint16_t;
    let tv: uint16_t = 0 as libc::c_int as uint16_t;
    memcpy(
        *p as *mut libc::c_void,
        &tv as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp = p_tmp.offset(1);
    p_tmp;
    *p = p_tmp as *mut libc::c_void as *mut uint8_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).flags & CV_NETVAR as libc::c_int != 0 && CV_IsSetToDefault(cvar) == 0
        {
            if in_demo != 0 {
                let mut tmp_i: size_t = 0;
                tmp_i = 0 as libc::c_int as size_t;
                while *((*cvar).name).offset(tmp_i as isize) as libc::c_int
                    != '\0' as i32
                {
                    let mut p_tmp_0: *mut libc::c_char = *p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_0: libc::c_char = *((*cvar).name).offset(tmp_i as isize);
                    memcpy(
                        *p as *mut libc::c_void,
                        &tv_0 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_0 = p_tmp_0.offset(1);
                    p_tmp_0;
                    *p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
                    tmp_i = tmp_i.wrapping_add(1);
                    tmp_i;
                }
                let mut p_tmp_1: *mut libc::c_char = *p as *mut libc::c_void
                    as *mut libc::c_char;
                let tv_1: libc::c_char = '\0' as i32 as libc::c_char;
                memcpy(
                    *p as *mut libc::c_void,
                    &tv_1 as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
                p_tmp_1 = p_tmp_1.offset(1);
                p_tmp_1;
                *p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
            } else {
                let mut p_tmp_2: *mut uint16_t = *p as *mut libc::c_void
                    as *mut uint16_t;
                let tv_2: uint16_t = (*cvar).netid;
                memcpy(
                    *p as *mut libc::c_void,
                    &tv_2 as *const uint16_t as *const libc::c_void,
                    ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
                );
                p_tmp_2 = p_tmp_2.offset(1);
                p_tmp_2;
                *p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
            }
            let mut tmp_i_0: size_t = 0;
            tmp_i_0 = 0 as libc::c_int as size_t;
            while *((*cvar).string).offset(tmp_i_0 as isize) as libc::c_int
                != '\0' as i32
            {
                let mut p_tmp_3: *mut libc::c_char = *p as *mut libc::c_void
                    as *mut libc::c_char;
                let tv_3: libc::c_char = *((*cvar).string).offset(tmp_i_0 as isize);
                memcpy(
                    *p as *mut libc::c_void,
                    &tv_3 as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
                p_tmp_3 = p_tmp_3.offset(1);
                p_tmp_3;
                *p = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
                tmp_i_0 = tmp_i_0.wrapping_add(1);
                tmp_i_0;
            }
            let mut p_tmp_4: *mut libc::c_char = *p as *mut libc::c_void
                as *mut libc::c_char;
            let tv_4: libc::c_char = '\0' as i32 as libc::c_char;
            memcpy(
                *p as *mut libc::c_void,
                &tv_4 as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            p_tmp_4 = p_tmp_4.offset(1);
            p_tmp_4;
            *p = p_tmp_4 as *mut libc::c_void as *mut uint8_t;
            let mut p_tmp_5: *mut uint8_t = *p as *mut libc::c_void as *mut uint8_t;
            let tv_5: uint8_t = false_0 as libc::c_int as uint8_t;
            memcpy(
                *p as *mut libc::c_void,
                &tv_5 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_5 = p_tmp_5.offset(1);
            p_tmp_5;
            *p = p_tmp_5 as *mut libc::c_void as *mut uint8_t;
            count = count.wrapping_add(1);
            count;
        }
        cvar = (*cvar).next;
    }
    let mut p_tmp_6: *mut uint16_t = count_p as *mut libc::c_void as *mut uint16_t;
    let tv_6: uint16_t = count;
    memcpy(
        count_p as *mut libc::c_void,
        &tv_6 as *const uint16_t as *const libc::c_void,
        ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
    );
    p_tmp_6 = p_tmp_6.offset(1);
    p_tmp_6;
    count_p = p_tmp_6 as *mut libc::c_void as *mut uint8_t;
}
unsafe extern "C" fn CV_LoadVars(
    mut p: *mut *mut uint8_t,
    mut got: Option::<
        unsafe extern "C" fn(
            *mut *mut uint8_t,
            *mut *mut libc::c_char,
            *mut boolean,
        ) -> *mut consvar_t,
    >,
) {
    let store: boolean = (server == 0 || demoplayback != 0) as libc::c_int;
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    let mut count: uint16_t = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stealth: boolean = 0;
    serverloading = true_0 as libc::c_int;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).flags & CV_NETVAR as libc::c_int != 0 {
            if store != 0 && ((*cvar).revert.v.string).is_null() {
                (*cvar).revert.v.const_munge = (*cvar).string;
                (*cvar)
                    .revert
                    .allocated = ((*cvar).zstring
                    != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int
                    as libc::c_char;
                (*cvar).zstring = 0 as *mut libc::c_char;
            }
            Setvalue(cvar, (*cvar).defaultvalue, true_0 as libc::c_int);
        }
        cvar = (*cvar).next;
    }
    count = ({
        let mut p_tmp: *mut uint16_t = *p as *mut libc::c_void as *mut uint16_t;
        let mut b: uint16_t = 0;
        memcpy(
            &mut b as *mut uint16_t as *mut libc::c_void,
            *p as *const libc::c_void,
            ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        *p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    });
    loop {
        let fresh3 = count;
        count = count.wrapping_sub(1);
        if !(fresh3 != 0) {
            break;
        }
        cvar = (Some(got.expect("non-null function pointer")))
            .expect("non-null function pointer")(p, &mut val, &mut stealth);
        if !cvar.is_null() {
            Setvalue(cvar, val, stealth);
        }
    }
    serverloading = false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CV_RevertNetVars() {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if !((*cvar).revert.v.string).is_null() {
            Setvalue(cvar, (*cvar).revert.v.string, false_0 as libc::c_int);
            if (*cvar).revert.allocated != 0 {
                Z_Free((*cvar).revert.v.string as *mut libc::c_void);
                (*cvar).revert.allocated = false_0 as libc::c_int as libc::c_char;
            }
            (*cvar).revert.v.string = 0 as *mut libc::c_char;
        }
        cvar = (*cvar).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CV_LoadNetVars(mut p: *mut *mut uint8_t) {
    CV_LoadVars(
        p,
        Some(
            ReadNetVar
                as unsafe extern "C" fn(
                    *mut *mut uint8_t,
                    *mut *mut libc::c_char,
                    *mut boolean,
                ) -> *mut consvar_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CV_LoadOldDemoVars(mut p: *mut *mut uint8_t) {
    CV_LoadVars(
        p,
        Some(
            ReadOldDemoVar
                as unsafe extern "C" fn(
                    *mut *mut uint8_t,
                    *mut *mut libc::c_char,
                    *mut boolean,
                ) -> *mut consvar_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CV_LoadDemoVars(mut p: *mut *mut uint8_t) {
    CV_LoadVars(
        p,
        Some(
            ReadDemoVar
                as unsafe extern "C" fn(
                    *mut *mut uint8_t,
                    *mut *mut libc::c_char,
                    *mut boolean,
                ) -> *mut consvar_t,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CV_ResetCheatNetVars() {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).flags & CV_CHEAT as libc::c_int != 0 {
            CV_SetCVar(cvar, (*cvar).defaultvalue, true_0 as libc::c_int);
        }
        cvar = (*cvar).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CV_IsSetToDefault(mut v: *mut consvar_t) -> boolean {
    return (strcmp((*v).defaultvalue, (*v).string) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CV_CheatsEnabled() -> uint8_t {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).flags & CV_CHEAT as libc::c_int != 0
            && strcmp((*cvar).defaultvalue, (*cvar).string) != 0
        {
            return 1 as libc::c_int as uint8_t;
        }
        cvar = (*cvar).next;
    }
    return 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn CV_SetCVar(
    mut var: *mut consvar_t,
    mut value: *const libc::c_char,
    mut stealth: boolean,
) {
    if var.is_null() || ((*var).string).is_null() || value.is_null()
        || strcasecmp((*var).string, value) == 0
    {
        return;
    }
    if (*var).flags & CV_NETVAR as libc::c_int != 0 {
        let mut buf: [uint8_t; 128] = [0; 128];
        let mut p: *mut uint8_t = buf.as_mut_ptr();
        if server == 0 && execversion_enabled != 0 {
            Setvalue(var, value, true_0 as libc::c_int);
            return;
        }
        if !(server != 0 || addedtogame != 0 && IsPlayerAdmin(consoleplayer) != 0) {
            CONS_Printf(
                b"Only the server or admin can change: %s %s\n\0" as *const u8
                    as *const libc::c_char,
                (*var).name,
                (*var).string,
            );
            return;
        }
        if var == &mut cv_forceskin as *mut consvar_t {
            let mut skin: int32_t = R_SkinAvailable(value);
            if strcasecmp(value, b"None\0" as *const u8 as *const libc::c_char) != 0
                && (skin == -(1 as libc::c_int)
                    || R_SkinUsable(-(1 as libc::c_int), skin) == 0)
            {
                CONS_Printf(
                    b"Please provide a valid skin name (\"None\" disables).\n\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
        }
        if netgame != 0 || multiplayer != 0 {
            let mut p_tmp: *mut uint16_t = p as *mut libc::c_void as *mut uint16_t;
            let tv: uint16_t = (*var).netid;
            memcpy(
                p as *mut libc::c_void,
                &tv as *const uint16_t as *const libc::c_void,
                ::core::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut uint8_t;
            let mut tmp_i: size_t = 0;
            tmp_i = 0 as libc::c_int as size_t;
            while *value.offset(tmp_i as isize) as libc::c_int != '\0' as i32 {
                let mut p_tmp_0: *mut libc::c_char = p as *mut libc::c_void
                    as *mut libc::c_char;
                let tv_0: libc::c_char = *value.offset(tmp_i as isize);
                memcpy(
                    p as *mut libc::c_void,
                    &tv_0 as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
                p_tmp_0 = p_tmp_0.offset(1);
                p_tmp_0;
                p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
                tmp_i = tmp_i.wrapping_add(1);
                tmp_i;
            }
            let mut p_tmp_1: *mut libc::c_char = p as *mut libc::c_void
                as *mut libc::c_char;
            let tv_1: libc::c_char = '\0' as i32 as libc::c_char;
            memcpy(
                p as *mut libc::c_void,
                &tv_1 as *const libc::c_char as *const libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            p_tmp_1 = p_tmp_1.offset(1);
            p_tmp_1;
            p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
            let mut p_tmp_2: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let tv_2: uint8_t = stealth as uint8_t;
            memcpy(
                p as *mut libc::c_void,
                &tv_2 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_2 = p_tmp_2.offset(1);
            p_tmp_2;
            p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
            SendNetXCmd(
                XD_NETVAR,
                buf.as_mut_ptr() as *const libc::c_void,
                p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
            );
        } else {
            Setvalue(var, value, stealth);
        }
    } else if (*var).flags & CV_NOTINNET as libc::c_int != 0 && netgame != 0 {
        CONS_Printf(
            b"This variable can't be changed while in netgame: %s %s\n\0" as *const u8
                as *const libc::c_char,
            (*var).name,
            (*var).string,
        );
        return;
    } else {
        Setvalue(var, value, stealth);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CV_StealthSet(
    mut var: *mut consvar_t,
    mut value: *const libc::c_char,
) {
    CV_SetCVar(var, value, true_0 as libc::c_int);
}
unsafe extern "C" fn CV_SetValueMaybeStealth(
    mut var: *mut consvar_t,
    mut value: int32_t,
    mut stealth: boolean,
) {
    let mut val: [libc::c_char; 17] = [0; 17];
    if var == &mut cv_forceskin as *mut consvar_t {
        let mut tmpskin: *const libc::c_char = 0 as *const libc::c_char;
        if value < 0 as libc::c_int || value >= numskins {
            tmpskin = b"None\0" as *const u8 as *const libc::c_char;
        } else {
            tmpskin = (skins[value as usize].name).as_mut_ptr();
        }
        strncpy(val.as_mut_ptr(), tmpskin, 16 as libc::c_int as libc::c_ulong);
    } else {
        sprintf(val.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, value);
    }
    CV_SetCVar(var, val.as_mut_ptr(), stealth);
}
#[no_mangle]
pub unsafe extern "C" fn CV_StealthSetValue(
    mut var: *mut consvar_t,
    mut value: int32_t,
) {
    CV_SetValueMaybeStealth(var, value, true_0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CV_Set(
    mut var: *mut consvar_t,
    mut value: *const libc::c_char,
) {
    CV_SetCVar(var, value, false_0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CV_SetValue(mut var: *mut consvar_t, mut value: int32_t) {
    CV_SetValueMaybeStealth(var, value, false_0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CV_AddValue(mut var: *mut consvar_t, mut increment: int32_t) {
    let mut newvalue: int32_t = 0;
    let mut max: int32_t = 0;
    if increment == 0 {
        return;
    }
    if var == &mut cv_pointlimit as *mut consvar_t
        && gametype as libc::c_int == GT_MATCH as libc::c_int
    {
        increment *= 50 as libc::c_int;
    }
    if var == &mut cv_forceskin as *mut consvar_t {
        let mut oldvalue: int32_t = (*var).value;
        newvalue = oldvalue;
        loop {
            newvalue += increment;
            if newvalue < -(1 as libc::c_int) {
                newvalue = numskins - 1 as libc::c_int;
            } else if newvalue >= numskins {
                newvalue = -(1 as libc::c_int);
            }
            if !(oldvalue != newvalue
                && R_SkinUsable(-(1 as libc::c_int), newvalue) == 0)
            {
                break;
            }
        }
    } else {
        newvalue = (*var).value + increment;
    }
    if !((*var).PossibleValue).is_null() {
        if var == &mut cv_nextmap as *mut consvar_t {
            let mut oldvalue_0: int32_t = (*var).value - 1 as libc::c_int;
            let mut gt: int32_t = 0;
            gt = cv_newgametype.value;
            newvalue = (*var).value - 1 as libc::c_int;
            loop {
                if increment > 0 as libc::c_int {
                    newvalue += 1;
                    newvalue;
                    if newvalue == 1035 as libc::c_int {
                        newvalue = 0 as libc::c_int;
                    }
                } else {
                    newvalue -= 1;
                    newvalue;
                    if newvalue == -(1 as libc::c_int) {
                        newvalue = 1035 as libc::c_int - 1 as libc::c_int;
                    }
                }
                if newvalue == oldvalue_0 {
                    gt = -(1 as libc::c_int);
                }
                (mapheaderinfo[newvalue as usize]).is_null();
                if !(newvalue != oldvalue_0 && M_CanShowLevelInList(newvalue, gt) == 0) {
                    break;
                }
            }
            (*var).value = newvalue + 1 as libc::c_int;
            ((*var).func).expect("non-null function pointer")();
            return;
        } else if !((*((*var).PossibleValue).offset(0 as libc::c_int as isize)).strvalue)
            .is_null()
            && strcmp(
                (*((*var).PossibleValue).offset(0 as libc::c_int as isize)).strvalue,
                b"MIN\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if newvalue
                < (*((*var).PossibleValue).offset(0 as libc::c_int as isize)).value
                || newvalue
                    > (*((*var).PossibleValue).offset(1 as libc::c_int as isize)).value
            {
                let mut currentindice: int32_t = -(1 as libc::c_int);
                let mut newindice: int32_t = 0;
                max = 1 as libc::c_int + 1 as libc::c_int;
                while !((*((*var).PossibleValue).offset(max as isize)).strvalue)
                    .is_null()
                {
                    if (*((*var).PossibleValue).offset(max as isize)).value == newvalue {
                        increment = 0 as libc::c_int;
                        currentindice = max;
                        break;
                    } else {
                        if (*((*var).PossibleValue).offset(max as isize)).value
                            == (*var).value
                        {
                            currentindice = max;
                        }
                        max += 1;
                        max;
                    }
                }
                if increment != 0 {
                    increment = if increment > 0 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        -(1 as libc::c_int)
                    };
                    if currentindice == -(1 as libc::c_int)
                        && max != 1 as libc::c_int + 1 as libc::c_int
                    {
                        newindice = (if increment > 0 as libc::c_int {
                            1 as libc::c_int
                        } else {
                            max
                        }) + increment;
                    } else {
                        newindice = currentindice + increment;
                    }
                    if newindice >= max || newindice <= 1 as libc::c_int {
                        if var == &mut cv_pointlimit as *mut consvar_t
                            && gametype as libc::c_int == GT_MATCH as libc::c_int
                            && increment > 0 as libc::c_int
                        {
                            CV_SetValue(var, 50 as libc::c_int);
                        } else {
                            newvalue = (*((*var).PossibleValue)
                                .offset(
                                    (if increment > 0 as libc::c_int {
                                        0 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }) as isize,
                                ))
                                .value;
                            CV_SetValue(var, newvalue);
                        }
                    } else {
                        CV_Set(
                            var,
                            (*((*var).PossibleValue).offset(newindice as isize)).strvalue,
                        );
                    }
                } else {
                    CV_Set(
                        var,
                        (*((*var).PossibleValue).offset(currentindice as isize)).strvalue,
                    );
                }
            } else {
                CV_SetValue(var, newvalue);
            }
        } else {
            let mut currentindice_0: int32_t = -(1 as libc::c_int);
            let mut newindice_0: int32_t = 0;
            max = 0 as libc::c_int;
            while !((*((*var).PossibleValue).offset(max as isize)).strvalue).is_null() {
                if (*((*var).PossibleValue).offset(max as isize)).value == (*var).value {
                    currentindice_0 = max;
                }
                max += 1;
                max;
            }
            if var == &mut cv_chooseskin as *mut consvar_t {
                newvalue = (*var).value - 1 as libc::c_int;
                loop {
                    if increment > 0 as libc::c_int {
                        newvalue += 1;
                        newvalue;
                        if newvalue == 32 as libc::c_int {
                            newvalue = 0 as libc::c_int;
                        }
                    } else if increment < 0 as libc::c_int {
                        newvalue -= 1;
                        newvalue;
                        if newvalue == -(1 as libc::c_int) {
                            newvalue = 32 as libc::c_int - 1 as libc::c_int;
                        }
                    }
                    if !((*((*var).PossibleValue).offset(newvalue as isize)).strvalue)
                        .is_null()
                    {
                        break;
                    }
                }
                (*var).value = newvalue + 1 as libc::c_int;
                (*var)
                    .string = (*((*var).PossibleValue).offset(newvalue as isize))
                    .strvalue;
                ((*var).func).expect("non-null function pointer")();
                return;
            }
            newindice_0 = (currentindice_0 + increment + max) % max;
            CV_Set(var, (*((*var).PossibleValue).offset(newindice_0 as isize)).strvalue);
        }
    } else {
        CV_SetValue(var, newvalue);
    }
    (*var).changed = 1 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn CV_InitFilterVar() {
    joyaxis2_default = true_0 as libc::c_int;
    joyaxis_default = joyaxis2_default;
    joyaxis2_count = 0 as libc::c_int;
    joyaxis_count = joyaxis2_count;
}
#[no_mangle]
pub unsafe extern "C" fn CV_ToggleExecVersion(mut enable: boolean) {
    execversion_enabled = enable;
}
unsafe extern "C" fn CV_EnforceExecVersion() {
    if execversion_enabled == 0 {
        CV_StealthSetValue(
            &mut cv_execversion,
            54 as libc::c_int + ((0 as libc::c_int) << 16 as libc::c_int),
        );
    }
}
unsafe extern "C" fn CV_FilterJoyAxisVars(
    mut v: *mut consvar_t,
    mut valstr: *const libc::c_char,
) -> boolean {
    if joyaxis_default != 0 {
        if strcasecmp((*v).name, b"joyaxis_turn\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"X-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis_count += 1;
                joyaxis_count;
            } else {
                joyaxis_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis_move\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"Y-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis_count += 1;
                joyaxis_count;
            } else {
                joyaxis_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis_side\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"Z-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis_count += 1;
                joyaxis_count;
            } else {
                joyaxis_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis_look\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"None\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis_count += 1;
                joyaxis_count;
            } else {
                joyaxis_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis_fire\0" as *const u8 as *const libc::c_char)
            == 0
            || strcasecmp(
                (*v).name,
                b"joyaxis_firenormal\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if joyaxis_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"None\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis_count += 1;
                joyaxis_count;
            } else {
                joyaxis_default = false_0 as libc::c_int;
            }
        }
        if joyaxis_count == 6 as libc::c_int {
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_turnaxis.name,
                    cv_turnaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_moveaxis.name,
                    cv_moveaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_sideaxis.name,
                    cv_sideaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_lookaxis.name,
                    cv_lookaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_fireaxis.name,
                    cv_fireaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_firenaxis.name,
                    cv_firenaxis.defaultvalue,
                ),
                0 as com_flags_t,
            );
            joyaxis_count += 1;
            joyaxis_count;
            return false_0 as libc::c_int;
        }
    }
    if joyaxis2_default != 0 {
        if strcasecmp((*v).name, b"joyaxis2_turn\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis2_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis2_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"X-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis2_count += 1;
                joyaxis2_count;
            } else {
                joyaxis2_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis2_move\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis2_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis2_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"Y-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis2_count += 1;
                joyaxis2_count;
            } else {
                joyaxis2_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis2_side\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis2_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis2_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"Z-Axis\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis2_count += 1;
                joyaxis2_count;
            } else {
                joyaxis2_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis2_look\0" as *const u8 as *const libc::c_char)
            == 0
        {
            if joyaxis2_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis2_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"None\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis2_count += 1;
                joyaxis2_count;
            } else {
                joyaxis2_default = false_0 as libc::c_int;
            }
        }
        if strcasecmp((*v).name, b"joyaxis2_fire\0" as *const u8 as *const libc::c_char)
            == 0
            || strcasecmp(
                (*v).name,
                b"joyaxis2_firenormal\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            if joyaxis2_count > 6 as libc::c_int {
                return false_0 as libc::c_int
            } else if joyaxis2_count == 6 as libc::c_int {
                return true_0 as libc::c_int
            }
            if strcasecmp(valstr, b"None\0" as *const u8 as *const libc::c_char) == 0 {
                joyaxis2_count += 1;
                joyaxis2_count;
            } else {
                joyaxis2_default = false_0 as libc::c_int;
            }
        }
        if joyaxis2_count == 6 as libc::c_int {
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_turnaxis2.name,
                    cv_turnaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_moveaxis2.name,
                    cv_moveaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_sideaxis2.name,
                    cv_sideaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_lookaxis2.name,
                    cv_lookaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_fireaxis2.name,
                    cv_fireaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            COM_BufInsertTextEx(
                va(
                    b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                    cv_firenaxis2.name,
                    cv_firenaxis2.defaultvalue,
                ),
                0 as com_flags_t,
            );
            joyaxis2_count += 1;
            joyaxis2_count;
            return false_0 as libc::c_int;
        }
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn CV_FilterVarByVersion(
    mut v: *mut consvar_t,
    mut valstr: *const libc::c_char,
) -> boolean {
    if (*v).flags & CV_SAVE as libc::c_int == 0 {
        return true_0 as libc::c_int;
    }
    if (cv_execversion.value & 0xffff as libc::c_int) < 26 as libc::c_int {
        if strcasecmp((*v).name, b"alwaysmlook\0" as *const u8 as *const libc::c_char)
            == 0
            || strcasecmp(
                (*v).name,
                b"alwaysmlook2\0" as *const u8 as *const libc::c_char,
            ) == 0
            || strcasecmp((*v).name, b"mousemove\0" as *const u8 as *const libc::c_char)
                == 0
            || strcasecmp((*v).name, b"mousemove2\0" as *const u8 as *const libc::c_char)
                == 0
        {
            return false_0 as libc::c_int;
        }
        if (strcasecmp((*v).name, b"mousesens\0" as *const u8 as *const libc::c_char)
            == 0
            || strcasecmp((*v).name, b"mousesens2\0" as *const u8 as *const libc::c_char)
                == 0
            || strcasecmp((*v).name, b"mouseysens\0" as *const u8 as *const libc::c_char)
                == 0
            || strcasecmp(
                (*v).name,
                b"mouseysens2\0" as *const u8 as *const libc::c_char,
            ) == 0) && atoi(valstr) == 35 as libc::c_int
        {
            return false_0 as libc::c_int;
        }
        if CV_FilterJoyAxisVars(v, valstr) == 0 {
            return false_0 as libc::c_int;
        }
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn CV_Command() -> boolean {
    let mut v: *mut consvar_t = 0 as *mut consvar_t;
    v = CV_FindVar(COM_Argv(0 as libc::c_int as size_t));
    if v.is_null() {
        return false_0 as libc::c_int;
    }
    if CV_Immutable(v) != 0 {
        CONS_Alert(
            CONS_WARNING,
            b"Variable '%s' cannot be changed from Lua.\n\0" as *const u8
                as *const libc::c_char,
            (*v).name,
        );
        return true_0 as libc::c_int;
    }
    if COM_Argc() == 1 as libc::c_int as size_t {
        CONS_Printf(
            b"\"%s\" is \"%s\" default is \"%s\"\n\0" as *const u8
                as *const libc::c_char,
            (*v).name,
            (*v).string,
            (*v).defaultvalue,
        );
        return true_0 as libc::c_int;
    }
    if (*v).flags & CV_SAVE as libc::c_int == 0
        || CV_FilterVarByVersion(v, COM_Argv(1 as libc::c_int as size_t)) != 0
    {
        CV_Set(v, COM_Argv(1 as libc::c_int as size_t));
        (*v).changed = 1 as libc::c_int as libc::c_char;
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CV_ClearChangedFlags() {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        (*cvar).changed = 0 as libc::c_int as libc::c_char;
        cvar = (*cvar).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CV_SaveVariables(mut f: *mut FILE) {
    let mut cvar: *mut consvar_t = 0 as *mut consvar_t;
    cvar = consvar_vars;
    while !cvar.is_null() {
        if (*cvar).flags & CV_SAVE as libc::c_int != 0 {
            let mut stringtowrite: [libc::c_char; 257] = [0; 257];
            let mut string: *const libc::c_char = 0 as *const libc::c_char;
            if !((*cvar).revert.v.string).is_null() {
                string = (*cvar).revert.v.string;
            } else {
                string = (*cvar).string;
            }
            if !((*cvar).PossibleValue).is_null()
                && !((*((*cvar).PossibleValue).offset(0 as libc::c_int as isize))
                    .strvalue)
                    .is_null()
                && strcasecmp(
                    (*((*cvar).PossibleValue).offset(0 as libc::c_int as isize))
                        .strvalue,
                    b"MIN\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                let mut which: libc::c_int = (strcasecmp(
                    string,
                    b"MAX\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int) as libc::c_int;
                if which != 0
                    || strcasecmp(string, b"MIN\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int
                {
                    let mut value: int32_t = (*((*cvar).PossibleValue)
                        .offset(which as isize))
                        .value;
                    if (*cvar).flags & CV_FLOAT as libc::c_int != 0 {
                        sprintf(
                            stringtowrite.as_mut_ptr(),
                            b"%f\0" as *const u8 as *const libc::c_char,
                            FixedToFloat(value) as libc::c_double,
                        );
                    } else {
                        sprintf(
                            stringtowrite.as_mut_ptr(),
                            b"%d\0" as *const u8 as *const libc::c_char,
                            value,
                        );
                    }
                    string = stringtowrite.as_mut_ptr();
                }
            }
            fprintf(
                f,
                b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                (*cvar).name,
                string,
            );
        }
        cvar = (*cvar).next;
    }
}
unsafe extern "C" fn CV_Immutable(mut var: *const consvar_t) -> boolean {
    if com_flags as libc::c_uint & COM_LUA as libc::c_int as libc::c_uint != 0 {
        if (*var).flags & CV_ALLOWLUA as libc::c_int == 0 {
            return true_0 as libc::c_int;
        }
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn COM_Parse(mut data: *mut libc::c_char) -> *mut libc::c_char {
    let mut c: libc::c_char = 0;
    let mut len: size_t = 0 as libc::c_int as size_t;
    com_token[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if data.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        loop {
            c = *data;
            if !(c as libc::c_int <= ' ' as i32) {
                break;
            }
            if c as libc::c_int == '\0' as i32 {
                return 0 as *mut libc::c_char;
            }
            data = data.offset(1);
            data;
        }
        if !(c as libc::c_int == '/' as i32
            && *data.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            break;
        }
        while *data as libc::c_int != 0 && *data as libc::c_int != '\n' as i32 {
            data = data.offset(1);
            data;
        }
    }
    if c as libc::c_int == '"' as i32 {
        data = data.offset(1);
        data;
        loop {
            let fresh4 = data;
            data = data.offset(1);
            c = *fresh4;
            if c as libc::c_int == '"' as i32 || c as libc::c_int == '\0' as i32 {
                com_token[len as usize] = 0 as libc::c_int as libc::c_char;
                return data;
            }
            if c as libc::c_int == '\u{1b}' as i32 {
                data = data.offset(1);
                data;
            } else {
                com_token[len as usize] = c;
                len = len.wrapping_add(1);
                len;
            }
        }
    }
    loop {
        if c as libc::c_int == '\u{1b}' as i32 {
            loop {
                data = data.offset(2 as libc::c_int as isize);
                c = *data;
                if !(c as libc::c_int == '\u{1b}' as i32) {
                    break;
                }
            }
        } else {
            com_token[len as usize] = c;
            data = data.offset(1);
            data;
            len = len.wrapping_add(1);
            len;
            c = *data;
        }
        if !(c as libc::c_int > 32 as libc::c_int) {
            break;
        }
    }
    com_token[len as usize] = 0 as libc::c_int as libc::c_char;
    return data;
}
