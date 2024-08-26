use ::libc;
extern "C" {
    pub type lua_State;
    static mut skins: [skin_t; 32];
    static mut numskins: int32_t;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_remove(L: *mut lua_State, idx: libc::c_int);
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn luaL_checklstring(
        L: *mut lua_State,
        numArg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
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
    fn Lua_optoption(
        L: *mut lua_State,
        narg: libc::c_int,
        def: libc::c_int,
        list_ref: libc::c_int,
    ) -> libc::c_int;
    fn Lua_CreateFieldTable(
        L: *mut lua_State,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    fn LUA_PushUserdata(
        L: *mut lua_State,
        data: *mut libc::c_void,
        meta: *const libc::c_char,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type lumpnum_t = uint32_t;
pub type skinsound_t = libc::c_uint;
pub const NUMSKINSOUNDS: skinsound_t = 21;
pub const SKSJUMP: skinsound_t = 20;
pub const SKSGASP: skinsound_t = 19;
pub const SKSSKID: skinsound_t = 18;
pub const SKSZOOM: skinsound_t = 17;
pub const SKSSPNDSH: skinsound_t = 16;
pub const SKSTHOK: skinsound_t = 15;
pub const SKSPLVCT4: skinsound_t = 14;
pub const SKSPLVCT3: skinsound_t = 13;
pub const SKSPLVCT2: skinsound_t = 12;
pub const SKSPLVCT1: skinsound_t = 11;
pub const SKSPLDET4: skinsound_t = 10;
pub const SKSPLDET3: skinsound_t = 9;
pub const SKSPLDET2: skinsound_t = 8;
pub const SKSPLDET1: skinsound_t = 7;
pub const SKSPLPAN4: skinsound_t = 6;
pub const SKSPLPAN3: skinsound_t = 5;
pub const SKSPLPAN2: skinsound_t = 4;
pub const SKSPLPAN1: skinsound_t = 3;
pub const SKSPUDPUD: skinsound_t = 2;
pub const SKSPUTPUT: skinsound_t = 1;
pub const SKSSPIN: skinsound_t = 0;
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
pub type fixed_t = int32_t;
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
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
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
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = int32_t;
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
pub const numframes: spritesopt = 0;
pub type spritesopt = libc::c_uint;
pub const skin_sprites: skin = 35;
pub const skin_soundsid: skin = 34;
pub const skin_contangle: skin = 33;
pub const skin_contspeed: skin = 32;
pub const skin_highresscale: skin = 31;
pub const skin_prefoppositecolor: skin = 30;
pub const skin_supercolor: skin = 29;
pub const skin_prefcolor: skin = 28;
pub const skin_starttranscolor: skin = 27;
pub const skin_camerascale: skin = 26;
pub const skin_shieldscale: skin = 25;
pub const skin_spinheight: skin = 24;
pub const skin_height: skin = 23;
pub const skin_radius: skin = 22;
pub const skin_jumpfactor: skin = 21;
pub const skin_acceleration: skin = 20;
pub const skin_accelstart: skin = 19;
pub const skin_thrustfactor: skin = 18;
pub const skin_runspeed: skin = 17;
pub const skin_normalspeed: skin = 16;
pub const skin_maxdash: skin = 15;
pub const skin_mindash: skin = 14;
pub const skin_actionspd: skin = 13;
pub const skin_followitem: skin = 12;
pub const skin_revitem: skin = 11;
pub const skin_spinitem: skin = 10;
pub const skin_thokitem: skin = 9;
pub const skin_ability2: skin = 8;
pub const skin_ability: skin = 7;
pub const skin_supername: skin = 6;
pub const skin_hudname: skin = 5;
pub const skin_realname: skin = 4;
pub const skin_flags: skin = 3;
pub type skin = libc::c_uint;
pub const skin_wadnum: skin = 2;
pub const skin_name: skin = 1;
pub const skin_valid: skin = 0;
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
static mut skin_opt: [*const libc::c_char; 37] = [
    b"valid\0" as *const u8 as *const libc::c_char,
    b"name\0" as *const u8 as *const libc::c_char,
    b"wadnum\0" as *const u8 as *const libc::c_char,
    b"flags\0" as *const u8 as *const libc::c_char,
    b"realname\0" as *const u8 as *const libc::c_char,
    b"hudname\0" as *const u8 as *const libc::c_char,
    b"supername\0" as *const u8 as *const libc::c_char,
    b"ability\0" as *const u8 as *const libc::c_char,
    b"ability2\0" as *const u8 as *const libc::c_char,
    b"thokitem\0" as *const u8 as *const libc::c_char,
    b"spinitem\0" as *const u8 as *const libc::c_char,
    b"revitem\0" as *const u8 as *const libc::c_char,
    b"followitem\0" as *const u8 as *const libc::c_char,
    b"actionspd\0" as *const u8 as *const libc::c_char,
    b"mindash\0" as *const u8 as *const libc::c_char,
    b"maxdash\0" as *const u8 as *const libc::c_char,
    b"normalspeed\0" as *const u8 as *const libc::c_char,
    b"runspeed\0" as *const u8 as *const libc::c_char,
    b"thrustfactor\0" as *const u8 as *const libc::c_char,
    b"accelstart\0" as *const u8 as *const libc::c_char,
    b"acceleration\0" as *const u8 as *const libc::c_char,
    b"jumpfactor\0" as *const u8 as *const libc::c_char,
    b"radius\0" as *const u8 as *const libc::c_char,
    b"height\0" as *const u8 as *const libc::c_char,
    b"spinheight\0" as *const u8 as *const libc::c_char,
    b"shieldscale\0" as *const u8 as *const libc::c_char,
    b"camerascale\0" as *const u8 as *const libc::c_char,
    b"starttranscolor\0" as *const u8 as *const libc::c_char,
    b"prefcolor\0" as *const u8 as *const libc::c_char,
    b"supercolor\0" as *const u8 as *const libc::c_char,
    b"prefoppositecolor\0" as *const u8 as *const libc::c_char,
    b"highresscale\0" as *const u8 as *const libc::c_char,
    b"contspeed\0" as *const u8 as *const libc::c_char,
    b"contangle\0" as *const u8 as *const libc::c_char,
    b"soundsid\0" as *const u8 as *const libc::c_char,
    b"sprites\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut skin_fields_ref: libc::c_int = -(2 as libc::c_int);
unsafe extern "C" fn skin_get(mut L: *mut lua_State) -> libc::c_int {
    let mut skin: *mut skin_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"SKIN_T*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut skin_t);
    let mut field: skin = Lua_optoption(
        L,
        2 as libc::c_int,
        -(1 as libc::c_int),
        skin_fields_ref,
    ) as skin;
    match field as libc::c_uint {
        0 => {
            lua_pushboolean(
                L,
                (skin != 0 as *mut libc::c_void as *mut skin_t) as libc::c_int,
            );
        }
        1 => {
            lua_pushstring(L, ((*skin).name).as_mut_ptr());
        }
        2 => {
            return luaL_error(
                L,
                b"'skin_t' field '%s' is not implemented for Lua and cannot be accessed.\0"
                    as *const u8 as *const libc::c_char,
                skin_opt[field as usize],
            );
        }
        3 => {
            lua_pushnumber(L, (*skin).flags as lua_Number);
        }
        4 => {
            lua_pushstring(L, ((*skin).realname).as_mut_ptr());
        }
        5 => {
            lua_pushstring(L, ((*skin).hudname).as_mut_ptr());
        }
        6 => {
            lua_pushstring(L, ((*skin).supername).as_mut_ptr());
        }
        7 => {
            lua_pushnumber(L, (*skin).ability as lua_Number);
        }
        8 => {
            lua_pushnumber(L, (*skin).ability2 as lua_Number);
        }
        9 => {
            lua_pushnumber(L, (*skin).thokitem);
        }
        10 => {
            lua_pushnumber(L, (*skin).spinitem);
        }
        11 => {
            lua_pushnumber(L, (*skin).revitem);
        }
        12 => {
            lua_pushnumber(L, (*skin).followitem);
        }
        13 => {
            lua_pushnumber(L, (*skin).actionspd);
        }
        14 => {
            lua_pushnumber(L, (*skin).mindash);
        }
        15 => {
            lua_pushnumber(L, (*skin).maxdash);
        }
        16 => {
            lua_pushnumber(L, (*skin).normalspeed);
        }
        17 => {
            lua_pushnumber(L, (*skin).runspeed);
        }
        18 => {
            lua_pushnumber(L, (*skin).thrustfactor as lua_Number);
        }
        19 => {
            lua_pushnumber(L, (*skin).accelstart as lua_Number);
        }
        20 => {
            lua_pushnumber(L, (*skin).acceleration as lua_Number);
        }
        21 => {
            lua_pushnumber(L, (*skin).jumpfactor);
        }
        22 => {
            lua_pushnumber(L, (*skin).radius);
        }
        23 => {
            lua_pushnumber(L, (*skin).height);
        }
        24 => {
            lua_pushnumber(L, (*skin).spinheight);
        }
        25 => {
            lua_pushnumber(L, (*skin).shieldscale);
        }
        26 => {
            lua_pushnumber(L, (*skin).camerascale);
        }
        27 => {
            lua_pushnumber(L, (*skin).starttranscolor as lua_Number);
        }
        28 => {
            lua_pushnumber(L, (*skin).prefcolor as lua_Number);
        }
        29 => {
            lua_pushnumber(L, (*skin).supercolor as lua_Number);
        }
        30 => {
            lua_pushnumber(L, (*skin).prefoppositecolor as lua_Number);
        }
        31 => {
            lua_pushnumber(L, (*skin).highresscale);
        }
        32 => {
            lua_pushnumber(L, (*skin).contspeed as lua_Number);
        }
        33 => {
            lua_pushnumber(L, (*skin).contangle as lua_Number);
        }
        34 => {
            LUA_PushUserdata(
                L,
                ((*skin).soundsid).as_mut_ptr() as *mut libc::c_void,
                b"SKIN_T*SOUNDSID\0" as *const u8 as *const libc::c_char,
            );
        }
        35 => {
            LUA_PushUserdata(
                L,
                ((*skin).sprites).as_mut_ptr() as *mut libc::c_void,
                b"SKIN_T*SPRITES\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn skin_set(mut L: *mut lua_State) -> libc::c_int {
    return luaL_error(
        L,
        b"'skin_t' struct cannot be edited by Lua.\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn skin_num(mut L: *mut lua_State) -> libc::c_int {
    let mut skin: *mut skin_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"SKIN_T*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut skin_t);
    lua_pushnumber(
        L,
        skin.offset_from(skins.as_mut_ptr()) as libc::c_long as lua_Number,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_iterateSkins(mut L: *mut lua_State) -> libc::c_int {
    let mut i: int32_t = 0;
    if lua_gettop(L) < 2 as libc::c_int {
        lua_pushcclosure(
            L,
            Some(
                lib_iterateSkins as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    lua_settop(L, 2 as libc::c_int);
    lua_remove(L, 1 as libc::c_int);
    if !(lua_type(L, 1 as libc::c_int) == 0 as libc::c_int) {
        i = (*(luaL_checkudata(
            L,
            1 as libc::c_int,
            b"SKIN_T*\0" as *const u8 as *const libc::c_char,
        ) as *mut *mut skin_t))
            .offset_from(skins.as_mut_ptr()) as libc::c_long as int32_t
            + 1 as libc::c_int;
    } else {
        i = 0 as libc::c_int;
    }
    if i < numskins {
        LUA_PushUserdata(
            L,
            &mut *skins.as_mut_ptr().offset(i as isize) as *mut skin_t
                as *mut libc::c_void,
            b"SKIN_T*\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lib_getSkin(mut L: *mut lua_State) -> libc::c_int {
    let mut field: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: int32_t = 0;
    if lua_type(L, 2 as libc::c_int) == 3 as libc::c_int {
        i = luaL_checknumber(L, 2 as libc::c_int);
        if i < 0 as libc::c_int || i >= 32 as libc::c_int {
            return luaL_error(
                L,
                b"skins[] index %d out of range (0 - %d)\0" as *const u8
                    as *const libc::c_char,
                i,
                32 as libc::c_int - 1 as libc::c_int,
            );
        }
        if i >= numskins {
            return 0 as libc::c_int;
        }
        LUA_PushUserdata(
            L,
            &mut *skins.as_mut_ptr().offset(i as isize) as *mut skin_t
                as *mut libc::c_void,
            b"SKIN_T*\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    field = luaL_checklstring(L, 2 as libc::c_int, 0 as *mut size_t);
    if fastcmp(field, b"iterate\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushcclosure(
            L,
            Some(
                lib_iterateSkins as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
            ),
            0 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < numskins {
        if fastcmp((skins[i as usize].name).as_mut_ptr(), field) != 0 {
            LUA_PushUserdata(
                L,
                &mut *skins.as_mut_ptr().offset(i as isize) as *mut skin_t
                    as *mut libc::c_void,
                b"SKIN_T*\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn lib_numSkins(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, numskins);
    return 1 as libc::c_int;
}
unsafe extern "C" fn soundsid_get(mut L: *mut lua_State) -> libc::c_int {
    let mut soundsid: *mut sfxenum_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"SKIN_T*SOUNDSID\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut sfxenum_t);
    let mut i: skinsound_t = luaL_checknumber(L, 2 as libc::c_int) as skinsound_t;
    if i as libc::c_uint >= NUMSKINSOUNDS as libc::c_int as libc::c_uint {
        return luaL_error(
            L,
            b"'skinsound_t' cannot be %u\0" as *const u8 as *const libc::c_char,
            i as libc::c_uint,
        );
    }
    lua_pushnumber(L, *soundsid.offset(i as isize) as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn soundsid_num(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, NUMSKINSOUNDS as libc::c_int);
    return 1 as libc::c_int;
}
static mut sprites_opt: [*const libc::c_char; 2] = [
    b"numframes\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn lib_getSkinSprite(mut L: *mut lua_State) -> libc::c_int {
    let mut sksprites: *mut spritedef_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"SKIN_T*SPRITES\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut spritedef_t);
    let mut i: playersprite_t = luaL_checknumber(L, 2 as libc::c_int) as playersprite_t;
    if (i as libc::c_uint) < 0 as libc::c_int as libc::c_uint
        || i as libc::c_uint
            >= (NUMPLAYERSPRITES as libc::c_int * 2 as libc::c_int) as libc::c_uint
    {
        return luaL_error(
            L,
            b"'skin_t' field 'sprites' index %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i as libc::c_uint,
            NUMPLAYERSPRITES as libc::c_int * 2 as libc::c_int - 1 as libc::c_int,
        );
    }
    LUA_PushUserdata(
        L,
        &mut *sksprites.offset(i as isize) as *mut spritedef_t as *mut libc::c_void,
        b"SKIN_T*SPRITES[]\0" as *const u8 as *const libc::c_char,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_numSkinsSprites(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, NUMPLAYERSPRITES as libc::c_int * 2 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn sprite_get(mut L: *mut lua_State) -> libc::c_int {
    let mut sprite: *mut spritedef_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"SKIN_T*SPRITES[]\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut spritedef_t);
    let mut field: spritesopt = luaL_checkoption(
        L,
        2 as libc::c_int,
        0 as *const libc::c_char,
        sprites_opt.as_ptr(),
    ) as spritesopt;
    match field as libc::c_uint {
        0 => {
            lua_pushnumber(L, (*sprite).numframes as lua_Number);
        }
        _ => {}
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_SkinLib(mut L: *mut lua_State) -> libc::c_int {
    luaL_newmetatable(L, b"SKIN_T*\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(skin_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(skin_set as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__newindex\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(skin_num as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    skin_fields_ref = Lua_CreateFieldTable(L, skin_opt.as_ptr());
    luaL_newmetatable(L, b"SKIN_T*SOUNDSID\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(soundsid_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(soundsid_num as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    luaL_newmetatable(L, b"SKIN_T*SPRITES\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(lib_getSkinSprite as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(lib_numSkinsSprites as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    luaL_newmetatable(L, b"SKIN_T*SPRITES[]\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(sprite_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_newuserdata(L, 0 as libc::c_int as size_t);
    lua_createtable(L, 0 as libc::c_int, 2 as libc::c_int);
    lua_pushcclosure(
        L,
        Some(lib_getSkin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(lib_numSkins as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_setfield(
        L,
        -(10002 as libc::c_int),
        b"skins\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
