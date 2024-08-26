use ::libc;
extern "C" {
    pub type huddrawlist_s;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
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
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut gamestate: gamestate_t;
    static mut ultimatemode: uint8_t;
    static mut gameaction: gameaction_t;
    static mut botskin: uint8_t;
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Debug(debugflags: int32_t, fmt: *const libc::c_char, _: ...);
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut cv_debug: int32_t;
    static mut finesine: [fixed_t; 10240];
    static mut finecosine: *mut fixed_t;
    fn FixedAngle(fa: fixed_t) -> angle_t;
    static mut states: [state_t; 6735];
    static mut gamemap: int16_t;
    static mut maptol: uint32_t;
    static mut globalweather: uint8_t;
    static mut cursaveslot: int32_t;
    static mut marathonmode: marathonmode_t;
    static mut imcontinuing: boolean;
    static mut modeattacking: uint8_t;
    static mut numDemos: uint8_t;
    static mut demoDelayTime: uint32_t;
    static mut demoIdleTime: uint32_t;
    static mut netgame: boolean;
    static mut multiplayer: boolean;
    static mut gametype: int16_t;
    static mut splitscreen: boolean;
    static mut menuactive: boolean;
    static mut paused: uint8_t;
    static mut consoleplayer: int32_t;
    static mut displayplayer: int32_t;
    static mut secondarydisplayplayer: int32_t;
    static mut titlemap: int16_t;
    static mut hidetitlepics: boolean;
    static mut tutorialmode: boolean;
    static mut looptitle: boolean;
    static mut cutscenes: [*mut cutscene_t; 128];
    static mut textprompts: [*mut textprompt_t; 291];
    static mut mapheaderinfo: [*mut mapheader_t; 1035];
    static mut stagefailed: boolean;
    static mut emeralds: uint16_t;
    static mut introtoplay: uint8_t;
    static mut creditscutscene: uint8_t;
    static mut useBlackRock: uint8_t;
    static mut useContinues: uint8_t;
    static mut playerstarts: [*mut mapthing_t; 32];
    static mut wipegamestate: gamestate_t;
    static mut wipetypepre: int16_t;
    static mut wipetypepost: int16_t;
    static mut cv_playercolor: consvar_t;
    static mut cv_usemouse: consvar_t;
    static mut cv_rollingdemos: consvar_t;
    static mut cv_soundtest: consvar_t;
    static mut cv_sleep: consvar_t;
    fn Command_ExitGame_f();
    fn D_MapChange(
        pmapnum: int32_t,
        pgametype: int32_t,
        pultmode: boolean,
        presetplayers: boolean,
        pdelay: int32_t,
        pskipprecutscene: boolean,
        pfromlevelselect: boolean,
    );
    fn IsPlayerAdmin(playernum: int32_t) -> boolean;
    static mut server: boolean;
    static mut serverplayer: int32_t;
    fn W_CheckNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn D_StartTitle();
    fn W_CachePatchName(name: *const libc::c_char, tag: int32_t) -> *mut libc::c_void;
    fn W_CachePatchNum(lumpnum: lumpnum_t, tag: int32_t) -> *mut libc::c_void;
    fn W_UnlockCachedPatch(patch: *mut libc::c_void);
    static mut WipeInAction: boolean;
    static mut wipestyleflags: wipestyleflags_t;
    fn F_TryColormapFade(wipecolor: uint8_t) -> boolean;
    fn F_WipeStartScreen();
    fn F_WipeEndScreen();
    fn F_RunWipe(wipetype: uint8_t, drawMenu: boolean);
    static mut players: [player_t; 32];
    static mut nextmap: int16_t;
    fn G_DoLoadLevel(resetplayer: boolean);
    fn G_SaveGameData(data: *mut gamedata_t);
    fn G_CoopGametype() -> boolean;
    fn G_NextLevel();
    fn G_Continue();
    fn G_EndGame();
    static mut serverGamedata: *mut gamedata_t;
    static mut clientGamedata: *mut gamedata_t;
    fn M_UpdateUnlockablesAndExtraEmblems(data: *mut gamedata_t) -> uint8_t;
    fn M_SilentUpdateUnlockablesAndEmblems(data: *mut gamedata_t);
    fn G_SetGamestate(newstate: gamestate_t);
    static mut demofileoverride: demo_file_override_e;
    static mut titledemo: boolean;
    fn G_DoPlayDemo(defdemoname: *mut libc::c_char);
    static mut cv_timescale: consvar_t;
    static mut vid: viddef_t;
    fn R_GetTranslationColormap(
        skinnum: int32_t,
        color: skincolornum_t,
        flags: uint8_t,
    ) -> *mut uint8_t;
    fn P_ReturnThrustX(mo: *mut mobj_t, angle: angle_t, move_0: fixed_t) -> fixed_t;
    fn P_GetSkinSprite2(
        skin: *mut skin_t,
        spr2: uint8_t,
        player: *mut player_t,
    ) -> uint8_t;
    static mut deathmatchstarts: [*mut mapthing_t; 64];
    fn P_AllocMapHeader(i: int16_t);
    static mut skins: [skin_t; 32];
    fn Patch_Free(patch: *mut patch_t);
    fn P_AutoPause() -> boolean;
    fn P_SetPlayerMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
    static mut tmthing: *mut mobj_t;
    fn P_MapStart();
    fn P_LinedefExecute(tag: int16_t, actor: *mut mobj_t, caller: *mut sector_t);
    fn P_MapEnd();
    static mut thlist: [thinker_t; 0];
    fn P_RemoveThinkerDelayed(thinker: *mut thinker_t);
    static mut camera: camera_t;
    static mut rendertimefrac: fixed_t;
    fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t;
    static mut renderisnewtic: boolean;
    fn S_StartSound(origin: *const libc::c_void, sound_id: sfxenum_t);
    fn S_FadeOutStopMusic(ms: uint32_t) -> boolean;
    fn S_ChangeMusicEx(
        mmusic: *const libc::c_char,
        mflags: uint16_t,
        looping: boolean,
        position: uint32_t,
        prefadems: uint32_t,
        fadeinms: uint32_t,
    );
    fn S_StopSounds();
    fn I_FadeSong(
        target_volume: uint8_t,
        ms: uint32_t,
        callback: Option::<unsafe extern "C" fn() -> ()>,
    ) -> boolean;
    fn S_StopMusic();
    fn I_GetTime() -> tic_t;
    fn I_UpdateTime(timescale: fixed_t);
    static mut rendermode: rendermode_t;
    fn I_UpdateNoBlit();
    fn I_FinishUpdate();
    fn V_DrawStretchyFixedPatch(
        x: fixed_t,
        y: fixed_t,
        pscale: fixed_t,
        vscale: fixed_t,
        scrn: int32_t,
        patch: *mut patch_t,
        colormap: *const uint8_t,
    );
    fn V_DrawContinueIcon(
        x: int32_t,
        y: int32_t,
        flags: int32_t,
        skinnum: int32_t,
        skincolor: uint16_t,
    );
    fn V_DrawFill(x: int32_t, y: int32_t, w: int32_t, h: int32_t, c: int32_t);
    fn V_DrawFadeScreen(color: uint16_t, strength: uint8_t);
    fn V_DrawFadeFill(
        x: int32_t,
        y: int32_t,
        w: int32_t,
        h: int32_t,
        c: int32_t,
        color: uint16_t,
        strength: uint8_t,
    );
    fn V_DrawPromptBack(boxheight: int32_t, color: int32_t);
    fn V_DrawCharacter(x: int32_t, y: int32_t, c: int32_t, lowercaseallowed: boolean);
    fn V_DrawLevelTitle(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_WordWrap(
        x: int32_t,
        w: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn V_DrawString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_LevelNameWidth(string: *const libc::c_char) -> int32_t;
    fn V_DrawCreditString(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_CreditStringWidth(string: *const libc::c_char) -> int32_t;
    fn V_StringWidth(string: *const libc::c_char, option: int32_t) -> int32_t;
    fn V_DrawPatchFill(pat: *mut patch_t);
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_StrDup(in_0: *const libc::c_char) -> *mut libc::c_char;
    fn I_Sleep(ms: uint32_t);
    fn I_OsPolling();
    static mut menupres: [menupres_t; 58];
    static mut prevMenuId: uint32_t;
    static mut activeMenuId: uint32_t;
    fn M_SetMenuCurBackground(defaultname: *const libc::c_char);
    fn M_SetMenuCurFadeValue(defaultvalue: uint8_t);
    fn M_SetMenuCurTitlePics();
    fn M_Drawer();
    fn M_ClearMenus(callexitmenufunc: boolean);
    static mut MainDef: menu_t;
    static mut gamecontrol: [[int32_t; 2]; 43];
    static gcl_tutorial_used: [int32_t; 8];
    static gcl_movement: [int32_t; 4];
    static gcl_camera: [int32_t; 2];
    static gcl_movement_camera: [int32_t; 6];
    static gcl_jump: [int32_t; 1];
    static gcl_spin: [int32_t; 1];
    static gcl_jump_spin: [int32_t; 2];
    fn G_GetControlScheme(
        fromcontrols: *mut [int32_t; 2],
        gclist: *const int32_t,
        gclen: int32_t,
    ) -> int32_t;
    fn CON_ClearHUD();
    fn CON_ToggleOff();
    fn CON_Ready() -> boolean;
    fn M_RandomKey(a: int32_t) -> int32_t;
    fn M_RandomRange(a: int32_t, b: int32_t) -> int32_t;
    static mut moviemode: moviemode_t;
    fn M_SaveFrame();
    static mut stlivex: *mut patch_t;
    fn LUA_HUD_CreateDrawList() -> huddrawlist_h;
    fn LUA_HUD_ClearDrawList(list: huddrawlist_h);
    fn LUA_HUD_DestroyDrawList(list: huddrawlist_h);
    fn LUA_HUD_IsDrawListValid(list: huddrawlist_h) -> boolean;
    fn LUA_HUD_DrawList(list: huddrawlist_h);
    fn LUA_HookHUD(hook: libc::c_int, drawlist: huddrawlist_h);
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
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
pub type lumpnum_t = uint32_t;
pub type tic_t = uint32_t;
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
pub type gameaction_t = libc::c_uint;
pub const ga_continued: gameaction_t = 4;
pub const ga_startcont: gameaction_t = 3;
pub const ga_worlddone: gameaction_t = 2;
pub const ga_completed: gameaction_t = 1;
pub const ga_nothing: gameaction_t = 0;
pub const TITLEMAP_OFF: C2RustUnnamed_6 = 0;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type mtag_t = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taglist_t {
    pub tags: *mut mtag_t,
    pub count: uint16_t,
}
pub type fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector2_t {
    pub x: fixed_t,
    pub y: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector3_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapthing_t {
    pub x: int16_t,
    pub y: int16_t,
    pub angle: int16_t,
    pub pitch: int16_t,
    pub roll: int16_t,
    pub type_0: uint16_t,
    pub options: uint16_t,
    pub z: int16_t,
    pub extrainfo: uint8_t,
    pub tags: taglist_t,
    pub scale: fixed_t,
    pub spritexscale: fixed_t,
    pub spriteyscale: fixed_t,
    pub args: [int32_t; 10],
    pub stringargs: [*mut libc::c_char; 2],
    pub mobj: *mut mobj_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mobj_s {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub old_x: fixed_t,
    pub old_y: fixed_t,
    pub old_z: fixed_t,
    pub old_x2: fixed_t,
    pub old_y2: fixed_t,
    pub old_z2: fixed_t,
    pub snext: *mut mobj_s,
    pub sprev: *mut *mut mobj_s,
    pub angle: angle_t,
    pub pitch: angle_t,
    pub roll: angle_t,
    pub old_angle: angle_t,
    pub old_pitch: angle_t,
    pub old_roll: angle_t,
    pub old_angle2: angle_t,
    pub old_pitch2: angle_t,
    pub old_roll2: angle_t,
    pub spriteroll: angle_t,
    pub old_spriteroll: angle_t,
    pub old_spriteroll2: angle_t,
    pub sprite: spritenum_t,
    pub frame: uint32_t,
    pub sprite2: uint8_t,
    pub anim_duration: uint16_t,
    pub renderflags: uint32_t,
    pub blendmode: int32_t,
    pub spritexscale: fixed_t,
    pub spriteyscale: fixed_t,
    pub spritexoffset: fixed_t,
    pub spriteyoffset: fixed_t,
    pub old_spritexscale: fixed_t,
    pub old_spriteyscale: fixed_t,
    pub old_spritexoffset: fixed_t,
    pub old_spriteyoffset: fixed_t,
    pub floorspriteslope: *mut pslope_s,
    pub touching_sectorlist: *mut msecnode_s,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub floorrover: *mut ffloor_s,
    pub ceilingrover: *mut ffloor_s,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub pmomz: fixed_t,
    pub tics: int32_t,
    pub state: *mut state_t,
    pub flags: uint32_t,
    pub flags2: uint32_t,
    pub eflags: uint16_t,
    pub skin: *mut libc::c_void,
    pub color: uint16_t,
    pub drawonlyforplayer: *mut player_s,
    pub dontdrawforviewmobj: *mut mobj_s,
    pub bnext: *mut mobj_s,
    pub bprev: *mut *mut mobj_s,
    pub hnext: *mut mobj_s,
    pub hprev: *mut mobj_s,
    pub type_0: mobjtype_t,
    pub info: *const mobjinfo_t,
    pub health: int32_t,
    pub movedir: angle_t,
    pub movecount: int32_t,
    pub target: *mut mobj_s,
    pub reactiontime: int32_t,
    pub threshold: int32_t,
    pub player: *mut player_s,
    pub lastlook: int32_t,
    pub spawnpoint: *mut mapthing_t,
    pub tracer: *mut mobj_s,
    pub friction: fixed_t,
    pub movefactor: fixed_t,
    pub fuse: int32_t,
    pub watertop: fixed_t,
    pub waterbottom: fixed_t,
    pub mobjnum: uint32_t,
    pub scale: fixed_t,
    pub old_scale: fixed_t,
    pub old_scale2: fixed_t,
    pub destscale: fixed_t,
    pub scalespeed: fixed_t,
    pub extravalue1: int32_t,
    pub extravalue2: int32_t,
    pub cusval: int32_t,
    pub cvmem: int32_t,
    pub standingslope: *mut pslope_s,
    pub resetinterp: boolean,
    pub colorized: boolean,
    pub mirrored: boolean,
    pub shadowscale: fixed_t,
    pub dispoffset: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pslope_s {
    pub id: uint16_t,
    pub next: *mut pslope_s,
    pub o: vector3_t,
    pub normal: vector3_t,
    pub d: vector2_t,
    pub zdelta: fixed_t,
    pub zangle: angle_t,
    pub xydirection: angle_t,
    pub flags: uint8_t,
}
pub type angle_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_s {
    pub mo: *mut mobj_t,
    pub cmd: ticcmd_t,
    pub playerstate: playerstate_t,
    pub camerascale: fixed_t,
    pub shieldscale: fixed_t,
    pub viewz: fixed_t,
    pub viewheight: fixed_t,
    pub deltaviewheight: fixed_t,
    pub bob: fixed_t,
    pub viewrollangle: angle_t,
    pub angleturn: int16_t,
    pub oldrelangleturn: int16_t,
    pub aiming: angle_t,
    pub drawangle: angle_t,
    pub old_drawangle: angle_t,
    pub old_drawangle2: angle_t,
    pub rings: int16_t,
    pub spheres: int16_t,
    pub pity: int8_t,
    pub currentweapon: int32_t,
    pub ringweapons: int32_t,
    pub ammoremoval: uint16_t,
    pub ammoremovaltimer: tic_t,
    pub ammoremovalweapon: int32_t,
    pub powers: [uint16_t; 30],
    pub pflags: pflags_t,
    pub panim: panim_t,
    pub stronganim: uint8_t,
    pub flashcount: uint16_t,
    pub flashpal: uint16_t,
    pub skincolor: uint16_t,
    pub skin: int32_t,
    pub availabilities: uint32_t,
    pub score: uint32_t,
    pub recordscore: uint32_t,
    pub dashspeed: fixed_t,
    pub normalspeed: fixed_t,
    pub runspeed: fixed_t,
    pub thrustfactor: uint8_t,
    pub accelstart: uint8_t,
    pub acceleration: uint8_t,
    pub charability: uint8_t,
    pub charability2: uint8_t,
    pub charflags: uint32_t,
    pub thokitem: mobjtype_t,
    pub spinitem: mobjtype_t,
    pub revitem: mobjtype_t,
    pub followitem: mobjtype_t,
    pub followmobj: *mut mobj_t,
    pub actionspd: fixed_t,
    pub mindash: fixed_t,
    pub maxdash: fixed_t,
    pub jumpfactor: fixed_t,
    pub height: fixed_t,
    pub spinheight: fixed_t,
    pub lives: int8_t,
    pub continues: int8_t,
    pub xtralife: int8_t,
    pub gotcontinue: uint8_t,
    pub speed: fixed_t,
    pub secondjump: uint8_t,
    pub fly1: uint8_t,
    pub scoreadd: uint8_t,
    pub glidetime: tic_t,
    pub climbing: uint8_t,
    pub deadtimer: int32_t,
    pub exiting: tic_t,
    pub homing: uint8_t,
    pub dashmode: tic_t,
    pub skidtime: tic_t,
    pub cmomx: fixed_t,
    pub cmomy: fixed_t,
    pub rmomx: fixed_t,
    pub rmomy: fixed_t,
    pub numboxes: int16_t,
    pub totalring: int16_t,
    pub realtime: tic_t,
    pub laps: uint8_t,
    pub ctfteam: int32_t,
    pub gotflag: uint16_t,
    pub weapondelay: int32_t,
    pub tossdelay: int32_t,
    pub starpostx: int16_t,
    pub starposty: int16_t,
    pub starpostz: int16_t,
    pub starpostnum: int32_t,
    pub starposttime: tic_t,
    pub starpostangle: angle_t,
    pub starpostscale: fixed_t,
    pub angle_pos: angle_t,
    pub old_angle_pos: angle_t,
    pub axis1: *mut mobj_t,
    pub axis2: *mut mobj_t,
    pub bumpertime: tic_t,
    pub flyangle: int32_t,
    pub drilltimer: tic_t,
    pub linkcount: int32_t,
    pub linktimer: tic_t,
    pub anotherflyangle: int32_t,
    pub nightstime: tic_t,
    pub drillmeter: int32_t,
    pub drilldelay: uint8_t,
    pub bonustime: boolean,
    pub capsule: *mut mobj_t,
    pub drone: *mut mobj_t,
    pub oldscale: fixed_t,
    pub mare: uint8_t,
    pub marelap: uint8_t,
    pub marebonuslap: uint8_t,
    pub marebegunat: tic_t,
    pub startedtime: tic_t,
    pub finishedtime: tic_t,
    pub lapbegunat: tic_t,
    pub lapstartedtime: tic_t,
    pub finishedspheres: int16_t,
    pub finishedrings: int16_t,
    pub marescore: uint32_t,
    pub lastmarescore: uint32_t,
    pub totalmarescore: uint32_t,
    pub lastmare: uint8_t,
    pub lastmarelap: uint8_t,
    pub lastmarebonuslap: uint8_t,
    pub totalmarelap: uint8_t,
    pub totalmarebonuslap: uint8_t,
    pub maxlink: int32_t,
    pub texttimer: uint8_t,
    pub textvar: uint8_t,
    pub lastsidehit: int16_t,
    pub lastlinehit: int16_t,
    pub losstime: tic_t,
    pub timeshit: uint8_t,
    pub onconveyor: int32_t,
    pub awayviewmobj: *mut mobj_t,
    pub awayviewtics: int32_t,
    pub awayviewaiming: angle_t,
    pub spectator: boolean,
    pub outofcoop: boolean,
    pub removing: boolean,
    pub bot: uint8_t,
    pub botleader: *mut player_s,
    pub lastbuttons: uint16_t,
    pub botmem: botmem_t,
    pub blocked: boolean,
    pub jointime: tic_t,
    pub quittime: tic_t,
}
pub type botmem_t = botmem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct botmem_s {
    pub lastForward: boolean,
    pub lastBlocked: boolean,
    pub blocked: boolean,
    pub catchup_tics: uint8_t,
    pub thinkstate: uint8_t,
}
pub type mobj_t = mobj_s;
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
pub type panim_t = libc::c_uint;
pub const PA_RIDE: panim_t = 13;
pub const PA_ABILITY2: panim_t = 12;
pub const PA_ABILITY: panim_t = 11;
pub const PA_FALL: panim_t = 10;
pub const PA_SPRING: panim_t = 9;
pub const PA_JUMP: panim_t = 8;
pub const PA_ROLL: panim_t = 7;
pub const PA_PAIN: panim_t = 6;
pub const PA_DASH: panim_t = 5;
pub const PA_RUN: panim_t = 4;
pub const PA_WALK: panim_t = 3;
pub const PA_EDGE: panim_t = 2;
pub const PA_IDLE: panim_t = 1;
pub const PA_ETC: panim_t = 0;
pub type pflags_t = libc::c_uint;
pub const PF_FINISHED: pflags_t = 1073741824;
pub const PF_CANCARRY: pflags_t = 536870912;
pub const PF_FORCESTRAFE: pflags_t = 268435456;
pub const PF_TAGIT: pflags_t = 134217728;
pub const PF_GAMETYPEOVER: pflags_t = 67108864;
pub const PF_DRILLING: pflags_t = 33554432;
pub const PF_TRANSFERTOCLOSEST: pflags_t = 16777216;
pub const PF_SLIDING: pflags_t = 8388608;
pub const PF_BOUNCING: pflags_t = 4194304;
pub const PF_GLIDING: pflags_t = 2097152;
pub const PF_SHIELDABILITY: pflags_t = 1048576;
pub const PF_THOKKED: pflags_t = 524288;
pub const PF_STARTDASH: pflags_t = 262144;
pub const PF_SPINNING: pflags_t = 131072;
pub const PF_NOJUMPDAMAGE: pflags_t = 65536;
pub const PF_JUMPED: pflags_t = 32768;
pub const PF_STARTJUMP: pflags_t = 16384;
pub const PF_APPLYAUTOBRAKE: pflags_t = 8192;
pub const PF_FULLSTASIS: pflags_t = 6144;
pub const PF_JUMPSTASIS: pflags_t = 4096;
pub const PF_STASIS: pflags_t = 2048;
pub const PF_WPNDOWN: pflags_t = 1024;
pub const PF_JUMPDOWN: pflags_t = 512;
pub const PF_SPINDOWN: pflags_t = 256;
pub const PF_ATTACKDOWN: pflags_t = 128;
pub const PF_INVIS: pflags_t = 64;
pub const PF_NOCLIP: pflags_t = 32;
pub const PF_GODMODE: pflags_t = 16;
pub const PF_AUTOBRAKE: pflags_t = 8;
pub const PF_DIRECTIONCHAR: pflags_t = 4;
pub const PF_ANALOGMODE: pflags_t = 2;
pub const PF_FLIPCAM: pflags_t = 1;
pub type playerstate_t = libc::c_uint;
pub const PST_REBORN: playerstate_t = 2;
pub const PST_DEAD: playerstate_t = 1;
pub const PST_LIVE: playerstate_t = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct ticcmd_t {
    pub forwardmove: int8_t,
    pub sidemove: int8_t,
    pub angleturn: int16_t,
    pub aiming: int16_t,
    pub buttons: uint16_t,
    pub latency: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mobjinfo_t {
    pub doomednum: int32_t,
    pub spawnstate: statenum_t,
    pub spawnhealth: int32_t,
    pub seestate: statenum_t,
    pub seesound: sfxenum_t,
    pub reactiontime: int32_t,
    pub attacksound: sfxenum_t,
    pub painstate: statenum_t,
    pub painchance: int32_t,
    pub painsound: sfxenum_t,
    pub meleestate: statenum_t,
    pub missilestate: statenum_t,
    pub deathstate: statenum_t,
    pub xdeathstate: statenum_t,
    pub deathsound: sfxenum_t,
    pub speed: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub dispoffset: int32_t,
    pub mass: int32_t,
    pub damage: int32_t,
    pub activesound: sfxenum_t,
    pub flags: uint32_t,
    pub raisestate: statenum_t,
}
pub type statenum_t = state;
pub type state = libc::c_uint;
pub const NUMSTATES: state = 6735;
pub const S_LASTFREESLOT: state = 6734;
pub const S_FIRSTFREESLOT: state = 2639;
pub const S_NAMECHECK: state = 2638;
pub const S_YELLOWBRICKDEBRIS: state = 2637;
pub const S_BLUEBRICKDEBRIS: state = 2636;
pub const S_REDBRICKDEBRIS: state = 2635;
pub const S_WOODDEBRIS: state = 2634;
pub const S_BRICKDEBRIS: state = 2633;
pub const S_GFZDEBRIS: state = 2632;
pub const S_ROCKCRUMBLEP: state = 2631;
pub const S_ROCKCRUMBLEO: state = 2630;
pub const S_ROCKCRUMBLEN: state = 2629;
pub const S_ROCKCRUMBLEM: state = 2628;
pub const S_ROCKCRUMBLEL: state = 2627;
pub const S_ROCKCRUMBLEK: state = 2626;
pub const S_ROCKCRUMBLEJ: state = 2625;
pub const S_ROCKCRUMBLEI: state = 2624;
pub const S_ROCKCRUMBLEH: state = 2623;
pub const S_ROCKCRUMBLEG: state = 2622;
pub const S_ROCKCRUMBLEF: state = 2621;
pub const S_ROCKCRUMBLEE: state = 2620;
pub const S_ROCKCRUMBLED: state = 2619;
pub const S_ROCKCRUMBLEC: state = 2618;
pub const S_ROCKCRUMBLEB: state = 2617;
pub const S_ROCKCRUMBLEA: state = 2616;
pub const S_ROCKSPAWN: state = 2615;
pub const S_DUST4: state = 2614;
pub const S_DUST3: state = 2613;
pub const S_DUST2: state = 2612;
pub const S_DUST1: state = 2611;
pub const S_WPLD6: state = 2610;
pub const S_WPLD5: state = 2609;
pub const S_WPLD4: state = 2608;
pub const S_WPLD3: state = 2607;
pub const S_WPLD2: state = 2606;
pub const S_WPLD1: state = 2605;
pub const S_XPLD_EGGTRAP: state = 2604;
pub const S_XPLD6: state = 2603;
pub const S_XPLD5: state = 2602;
pub const S_XPLD4: state = 2601;
pub const S_XPLD3: state = 2600;
pub const S_XPLD2: state = 2599;
pub const S_XPLD1: state = 2598;
pub const S_XPLD_FLICKY: state = 2597;
pub const S_SPRK3: state = 2596;
pub const S_SPRK2: state = 2595;
pub const S_SPRK1: state = 2594;
pub const S_CRUMBLE2: state = 2593;
pub const S_CRUMBLE1: state = 2592;
pub const S_HANGSTER_RETURN3: state = 2591;
pub const S_HANGSTER_RETURN2: state = 2590;
pub const S_HANGSTER_RETURN1: state = 2589;
pub const S_HANGSTER_ARCUP3: state = 2588;
pub const S_HANGSTER_ARCUP2: state = 2587;
pub const S_HANGSTER_ARCUP1: state = 2586;
pub const S_HANGSTER_FLYREPEAT: state = 2585;
pub const S_HANGSTER_FLY4: state = 2584;
pub const S_HANGSTER_FLY3: state = 2583;
pub const S_HANGSTER_FLY2: state = 2582;
pub const S_HANGSTER_FLY1: state = 2581;
pub const S_HANGSTER_ARC3: state = 2580;
pub const S_HANGSTER_ARC2: state = 2579;
pub const S_HANGSTER_ARC1: state = 2578;
pub const S_HANGSTER_SWOOP2: state = 2577;
pub const S_HANGSTER_SWOOP1: state = 2576;
pub const S_HANGSTER_LOOK: state = 2575;
pub const S_SPINBOBERT_FIRE_TRAIL3: state = 2574;
pub const S_SPINBOBERT_FIRE_TRAIL2: state = 2573;
pub const S_SPINBOBERT_FIRE_TRAIL1: state = 2572;
pub const S_SPINBOBERT_FIRE_GHOST: state = 2571;
pub const S_SPINBOBERT_FIRE_MOVE: state = 2570;
pub const S_SPINBOBERT_MOVE_DOWN: state = 2569;
pub const S_SPINBOBERT_MOVE_FLIPDOWN: state = 2568;
pub const S_SPINBOBERT_MOVE_UP: state = 2567;
pub const S_SPINBOBERT_MOVE_FLIPUP: state = 2566;
pub const S_CACOFIRE_EXPLODE4: state = 2565;
pub const S_CACOFIRE_EXPLODE3: state = 2564;
pub const S_CACOFIRE_EXPLODE2: state = 2563;
pub const S_CACOFIRE_EXPLODE1: state = 2562;
pub const S_CACOFIRE3: state = 2561;
pub const S_CACOFIRE2: state = 2560;
pub const S_CACOFIRE1: state = 2559;
pub const S_CACOSHARD2_2: state = 2558;
pub const S_CACOSHARD2_1: state = 2557;
pub const S_CACOSHARD1_2: state = 2556;
pub const S_CACOSHARD1_1: state = 2555;
pub const S_CACOSHARD_RANDOMIZE: state = 2554;
pub const S_CACO_DIE_FALL: state = 2553;
pub const S_CACO_DIE_SHATTER: state = 2552;
pub const S_CACO_DIE_SCREAM: state = 2551;
pub const S_CACO_DIE_GIB2: state = 2550;
pub const S_CACO_DIE_GIB1: state = 2549;
pub const S_CACO_DIE_FLAGS: state = 2548;
pub const S_CACO_CLOSE: state = 2547;
pub const S_CACO_SHOOT2: state = 2546;
pub const S_CACO_SHOOT1: state = 2545;
pub const S_CACO_SHOOT_SOUND: state = 2544;
pub const S_CACO_PREPARE3: state = 2543;
pub const S_CACO_PREPARE2: state = 2542;
pub const S_CACO_PREPARE1: state = 2541;
pub const S_CACO_PREPARE_SOUND: state = 2540;
pub const S_CACO_RANDOM: state = 2539;
pub const S_CACO_CHASE_REPEAT: state = 2538;
pub const S_CACO_CHASE: state = 2537;
pub const S_CACO_ROAR: state = 2536;
pub const S_CACO_WAKE4: state = 2535;
pub const S_CACO_WAKE3: state = 2534;
pub const S_CACO_WAKE2: state = 2533;
pub const S_CACO_WAKE1: state = 2532;
pub const S_CACO_LOOK: state = 2531;
pub const S_SMASHSPIKE_RISE2: state = 2530;
pub const S_SMASHSPIKE_RISE1: state = 2529;
pub const S_SMASHSPIKE_STOMP2: state = 2528;
pub const S_SMASHSPIKE_STOMP1: state = 2527;
pub const S_SMASHSPIKE_FALL: state = 2526;
pub const S_SMASHSPIKE_EASE2: state = 2525;
pub const S_SMASHSPIKE_EASE1: state = 2524;
pub const S_SMASHSPIKE_FLOAT: state = 2523;
pub const S_BUGGLEFLY: state = 2522;
pub const S_BUGGLEIDLE: state = 2521;
pub const S_BUMBLEBORE_DIE: state = 2520;
pub const S_BUMBLEBORE_STUCK2: state = 2519;
pub const S_BUMBLEBORE_STUCK1: state = 2518;
pub const S_BUMBLEBORE_FALL2: state = 2517;
pub const S_BUMBLEBORE_FALL1: state = 2516;
pub const S_BUMBLEBORE_RAISE: state = 2515;
pub const S_BUMBLEBORE_FLY2: state = 2514;
pub const S_BUMBLEBORE_FLY1: state = 2513;
pub const S_BUMBLEBORE_LOOK2: state = 2512;
pub const S_BUMBLEBORE_LOOK1: state = 2511;
pub const S_BUMBLEBORE_SPAWN: state = 2510;
pub const S_HIVEELEMENTAL_DIE3: state = 2509;
pub const S_HIVEELEMENTAL_DIE2: state = 2508;
pub const S_HIVEELEMENTAL_DIE1: state = 2507;
pub const S_HIVEELEMENTAL_PAIN: state = 2506;
pub const S_HIVEELEMENTAL_DORMANT: state = 2505;
pub const S_HIVEELEMENTAL_SHOOT2: state = 2504;
pub const S_HIVEELEMENTAL_SHOOT1: state = 2503;
pub const S_HIVEELEMENTAL_PREPARE2: state = 2502;
pub const S_HIVEELEMENTAL_PREPARE1: state = 2501;
pub const S_HIVEELEMENTAL_LOOK: state = 2500;
pub const S_POPSHOT_TRAIL: state = 2499;
pub const S_POPSHOT: state = 2498;
pub const S_POPHAT_SHOOT4: state = 2497;
pub const S_POPHAT_SHOOT3: state = 2496;
pub const S_POPHAT_SHOOT2: state = 2495;
pub const S_POPHAT_SHOOT1: state = 2494;
pub const S_POPHAT_LOOK: state = 2493;
pub const S_PENGUINATOR_SLIDE5: state = 2492;
pub const S_PENGUINATOR_SLIDE4: state = 2491;
pub const S_PENGUINATOR_SLIDE3: state = 2490;
pub const S_PENGUINATOR_SLIDE2: state = 2489;
pub const S_PENGUINATOR_SLIDE1: state = 2488;
pub const S_PENGUINATOR_WADDLE4: state = 2487;
pub const S_PENGUINATOR_WADDLE3: state = 2486;
pub const S_PENGUINATOR_WADDLE2: state = 2485;
pub const S_PENGUINATOR_WADDLE1: state = 2484;
pub const S_PENGUINATOR_LOOK: state = 2483;
pub const S_SHLEEPBOUNCE3: state = 2482;
pub const S_SHLEEPBOUNCE2: state = 2481;
pub const S_SHLEEPBOUNCE1: state = 2480;
pub const S_SHLEEP4: state = 2479;
pub const S_SHLEEP3: state = 2478;
pub const S_SHLEEP2: state = 2477;
pub const S_SHLEEP1: state = 2476;
pub const S_PIAN_SING: state = 2475;
pub const S_PIAN_FLY3: state = 2474;
pub const S_PIAN_FLY2: state = 2473;
pub const S_PIAN_FLY1: state = 2472;
pub const S_PIAN_LOOK3: state = 2471;
pub const S_PIAN_LOOK2: state = 2470;
pub const S_PIAN_LOOK1: state = 2469;
pub const S_NIGHTOPIANHELPER9: state = 2468;
pub const S_NIGHTOPIANHELPER8: state = 2467;
pub const S_NIGHTOPIANHELPER7: state = 2466;
pub const S_NIGHTOPIANHELPER6: state = 2465;
pub const S_NIGHTOPIANHELPER5: state = 2464;
pub const S_NIGHTOPIANHELPER4: state = 2463;
pub const S_NIGHTOPIANHELPER3: state = 2462;
pub const S_NIGHTOPIANHELPER2: state = 2461;
pub const S_NIGHTOPIANHELPER1: state = 2460;
pub const S_ORBIDYA5: state = 2459;
pub const S_ORBIDYA4: state = 2458;
pub const S_ORBIDYA3: state = 2457;
pub const S_ORBIDYA2: state = 2456;
pub const S_ORBIDYA1: state = 2455;
pub const S_ORBITEM8: state = 2454;
pub const S_ORBITEM7: state = 2453;
pub const S_ORBITEM6: state = 2452;
pub const S_ORBITEM5: state = 2451;
pub const S_ORBITEM4: state = 2450;
pub const S_ORBITEM3: state = 2449;
pub const S_ORBITEM2: state = 2448;
pub const S_ORBITEM1: state = 2447;
pub const S_EGGCAPSULE: state = 2446;
pub const S_NIGHTSLINKFREEZE: state = 2445;
pub const S_NIGHTSEXTRATIME: state = 2444;
pub const S_NIGHTSHELPER: state = 2443;
pub const S_NIGHTSDRILLREFILL: state = 2442;
pub const S_NIGHTSSUPERLOOP: state = 2441;
pub const S_NIGHTSCORE100_2: state = 2440;
pub const S_NIGHTSCORE90_2: state = 2439;
pub const S_NIGHTSCORE80_2: state = 2438;
pub const S_NIGHTSCORE70_2: state = 2437;
pub const S_NIGHTSCORE60_2: state = 2436;
pub const S_NIGHTSCORE50_2: state = 2435;
pub const S_NIGHTSCORE40_2: state = 2434;
pub const S_NIGHTSCORE30_2: state = 2433;
pub const S_NIGHTSCORE20_2: state = 2432;
pub const S_NIGHTSCORE10_2: state = 2431;
pub const S_NIGHTSCORE100: state = 2430;
pub const S_NIGHTSCORE90: state = 2429;
pub const S_NIGHTSCORE80: state = 2428;
pub const S_NIGHTSCORE70: state = 2427;
pub const S_NIGHTSCORE60: state = 2426;
pub const S_NIGHTSCORE50: state = 2425;
pub const S_NIGHTSCORE40: state = 2424;
pub const S_NIGHTSCORE30: state = 2423;
pub const S_NIGHTSCORE20: state = 2422;
pub const S_NIGHTSCORE10: state = 2421;
pub const S_HOOP_XMASB: state = 2420;
pub const S_HOOP_XMASA: state = 2419;
pub const S_HOOP: state = 2418;
pub const S_NIGHTSBUMPER12: state = 2417;
pub const S_NIGHTSBUMPER11: state = 2416;
pub const S_NIGHTSBUMPER10: state = 2415;
pub const S_NIGHTSBUMPER9: state = 2414;
pub const S_NIGHTSBUMPER8: state = 2413;
pub const S_NIGHTSBUMPER7: state = 2412;
pub const S_NIGHTSBUMPER6: state = 2411;
pub const S_NIGHTSBUMPER5: state = 2410;
pub const S_NIGHTSBUMPER4: state = 2409;
pub const S_NIGHTSBUMPER3: state = 2408;
pub const S_NIGHTSBUMPER2: state = 2407;
pub const S_NIGHTSBUMPER1: state = 2406;
pub const S_NIGHTSLOOPHELPER: state = 2405;
pub const S_NIGHTSPARKLESUPER4: state = 2404;
pub const S_NIGHTSPARKLESUPER3: state = 2403;
pub const S_NIGHTSPARKLESUPER2: state = 2402;
pub const S_NIGHTSPARKLESUPER1: state = 2401;
pub const S_NIGHTSPARKLE4: state = 2400;
pub const S_NIGHTSPARKLE3: state = 2399;
pub const S_NIGHTSPARKLE2: state = 2398;
pub const S_NIGHTSPARKLE1: state = 2397;
pub const S_NIGHTSDRONE_GOAL4: state = 2396;
pub const S_NIGHTSDRONE_GOAL3: state = 2395;
pub const S_NIGHTSDRONE_GOAL2: state = 2394;
pub const S_NIGHTSDRONE_GOAL1: state = 2393;
pub const S_NIGHTSDRONE_SPARKLING16: state = 2392;
pub const S_NIGHTSDRONE_SPARKLING15: state = 2391;
pub const S_NIGHTSDRONE_SPARKLING14: state = 2390;
pub const S_NIGHTSDRONE_SPARKLING13: state = 2389;
pub const S_NIGHTSDRONE_SPARKLING12: state = 2388;
pub const S_NIGHTSDRONE_SPARKLING11: state = 2387;
pub const S_NIGHTSDRONE_SPARKLING10: state = 2386;
pub const S_NIGHTSDRONE_SPARKLING9: state = 2385;
pub const S_NIGHTSDRONE_SPARKLING8: state = 2384;
pub const S_NIGHTSDRONE_SPARKLING7: state = 2383;
pub const S_NIGHTSDRONE_SPARKLING6: state = 2382;
pub const S_NIGHTSDRONE_SPARKLING5: state = 2381;
pub const S_NIGHTSDRONE_SPARKLING4: state = 2380;
pub const S_NIGHTSDRONE_SPARKLING3: state = 2379;
pub const S_NIGHTSDRONE_SPARKLING2: state = 2378;
pub const S_NIGHTSDRONE_SPARKLING1: state = 2377;
pub const S_NIGHTSDRONE_MAN2: state = 2376;
pub const S_NIGHTSDRONE_MAN1: state = 2375;
pub const S_TOAD: state = 2374;
pub const S_MARIOBUSH2: state = 2373;
pub const S_MARIOBUSH1: state = 2372;
pub const S_AXE3: state = 2371;
pub const S_AXE2: state = 2370;
pub const S_AXE1: state = 2369;
pub const S_KOOPAFLAME3: state = 2368;
pub const S_KOOPAFLAME2: state = 2367;
pub const S_KOOPAFLAME1: state = 2366;
pub const S_KOOPA2: state = 2365;
pub const S_KOOPA1: state = 2364;
pub const S_HAMMER: state = 2363;
pub const S_PUMATRAIL4: state = 2362;
pub const S_PUMATRAIL3: state = 2361;
pub const S_PUMATRAIL2: state = 2360;
pub const S_PUMATRAIL1: state = 2359;
pub const S_PUMA_DOWN3: state = 2358;
pub const S_PUMA_DOWN2: state = 2357;
pub const S_PUMA_DOWN1: state = 2356;
pub const S_PUMA_UP3: state = 2355;
pub const S_PUMA_UP2: state = 2354;
pub const S_PUMA_UP1: state = 2353;
pub const S_PUMA_START2: state = 2352;
pub const S_PUMA_START1: state = 2351;
pub const S_SHELL: state = 2350;
pub const S_FIREBALLTRAIL2: state = 2349;
pub const S_FIREBALLTRAIL1: state = 2348;
pub const S_FIREBALL: state = 2347;
pub const S_FIREFLOWER4: state = 2346;
pub const S_FIREFLOWER3: state = 2345;
pub const S_FIREFLOWER2: state = 2344;
pub const S_FIREFLOWER1: state = 2343;
pub const S_BLUEGOOMBA_DEAD: state = 2342;
pub const S_BLUEGOOMBA9: state = 2341;
pub const S_BLUEGOOMBA8: state = 2340;
pub const S_BLUEGOOMBA7: state = 2339;
pub const S_BLUEGOOMBA6: state = 2338;
pub const S_BLUEGOOMBA5: state = 2337;
pub const S_BLUEGOOMBA4: state = 2336;
pub const S_BLUEGOOMBA3: state = 2335;
pub const S_BLUEGOOMBA2: state = 2334;
pub const S_BLUEGOOMBA1B: state = 2333;
pub const S_BLUEGOOMBA1: state = 2332;
pub const S_GOOMBA_DEAD: state = 2331;
pub const S_GOOMBA9: state = 2330;
pub const S_GOOMBA8: state = 2329;
pub const S_GOOMBA7: state = 2328;
pub const S_GOOMBA6: state = 2327;
pub const S_GOOMBA5: state = 2326;
pub const S_GOOMBA4: state = 2325;
pub const S_GOOMBA3: state = 2324;
pub const S_GOOMBA2: state = 2323;
pub const S_GOOMBA1B: state = 2322;
pub const S_GOOMBA1: state = 2321;
pub const S_COINSPARKLE4: state = 2320;
pub const S_COINSPARKLE3: state = 2319;
pub const S_COINSPARKLE2: state = 2318;
pub const S_COINSPARKLE1: state = 2317;
pub const S_COIN3: state = 2316;
pub const S_COIN2: state = 2315;
pub const S_COIN1: state = 2314;
pub const S_RINGEXPLODE: state = 2313;
pub const S_THROWNSCATTER: state = 2312;
pub const S_THROWNGRENADE18: state = 2311;
pub const S_THROWNGRENADE17: state = 2310;
pub const S_THROWNGRENADE16: state = 2309;
pub const S_THROWNGRENADE15: state = 2308;
pub const S_THROWNGRENADE14: state = 2307;
pub const S_THROWNGRENADE13: state = 2306;
pub const S_THROWNGRENADE12: state = 2305;
pub const S_THROWNGRENADE11: state = 2304;
pub const S_THROWNGRENADE10: state = 2303;
pub const S_THROWNGRENADE9: state = 2302;
pub const S_THROWNGRENADE8: state = 2301;
pub const S_THROWNGRENADE7: state = 2300;
pub const S_THROWNGRENADE6: state = 2299;
pub const S_THROWNGRENADE5: state = 2298;
pub const S_THROWNGRENADE4: state = 2297;
pub const S_THROWNGRENADE3: state = 2296;
pub const S_THROWNGRENADE2: state = 2295;
pub const S_THROWNGRENADE1: state = 2294;
pub const S_THROWNEXPLOSION7: state = 2293;
pub const S_THROWNEXPLOSION6: state = 2292;
pub const S_THROWNEXPLOSION5: state = 2291;
pub const S_THROWNEXPLOSION4: state = 2290;
pub const S_THROWNEXPLOSION3: state = 2289;
pub const S_THROWNEXPLOSION2: state = 2288;
pub const S_THROWNEXPLOSION1: state = 2287;
pub const S_THROWNAUTOMATIC7: state = 2286;
pub const S_THROWNAUTOMATIC6: state = 2285;
pub const S_THROWNAUTOMATIC5: state = 2284;
pub const S_THROWNAUTOMATIC4: state = 2283;
pub const S_THROWNAUTOMATIC3: state = 2282;
pub const S_THROWNAUTOMATIC2: state = 2281;
pub const S_THROWNAUTOMATIC1: state = 2280;
pub const S_THROWNINFINITY7: state = 2279;
pub const S_THROWNINFINITY6: state = 2278;
pub const S_THROWNINFINITY5: state = 2277;
pub const S_THROWNINFINITY4: state = 2276;
pub const S_THROWNINFINITY3: state = 2275;
pub const S_THROWNINFINITY2: state = 2274;
pub const S_THROWNINFINITY1: state = 2273;
pub const S_THROWNBOUNCE7: state = 2272;
pub const S_THROWNBOUNCE6: state = 2271;
pub const S_THROWNBOUNCE5: state = 2270;
pub const S_THROWNBOUNCE4: state = 2269;
pub const S_THROWNBOUNCE3: state = 2268;
pub const S_THROWNBOUNCE2: state = 2267;
pub const S_THROWNBOUNCE1: state = 2266;
pub const S_GRENADEPICKUPFADE8: state = 2265;
pub const S_GRENADEPICKUPFADE7: state = 2264;
pub const S_GRENADEPICKUPFADE6: state = 2263;
pub const S_GRENADEPICKUPFADE5: state = 2262;
pub const S_GRENADEPICKUPFADE4: state = 2261;
pub const S_GRENADEPICKUPFADE3: state = 2260;
pub const S_GRENADEPICKUPFADE2: state = 2259;
pub const S_GRENADEPICKUPFADE1: state = 2258;
pub const S_GRENADEPICKUP: state = 2257;
pub const S_SCATTERPICKUPFADE8: state = 2256;
pub const S_SCATTERPICKUPFADE7: state = 2255;
pub const S_SCATTERPICKUPFADE6: state = 2254;
pub const S_SCATTERPICKUPFADE5: state = 2253;
pub const S_SCATTERPICKUPFADE4: state = 2252;
pub const S_SCATTERPICKUPFADE3: state = 2251;
pub const S_SCATTERPICKUPFADE2: state = 2250;
pub const S_SCATTERPICKUPFADE1: state = 2249;
pub const S_SCATTERPICKUP: state = 2248;
pub const S_EXPLODEPICKUPFADE8: state = 2247;
pub const S_EXPLODEPICKUPFADE7: state = 2246;
pub const S_EXPLODEPICKUPFADE6: state = 2245;
pub const S_EXPLODEPICKUPFADE5: state = 2244;
pub const S_EXPLODEPICKUPFADE4: state = 2243;
pub const S_EXPLODEPICKUPFADE3: state = 2242;
pub const S_EXPLODEPICKUPFADE2: state = 2241;
pub const S_EXPLODEPICKUPFADE1: state = 2240;
pub const S_EXPLODEPICKUP: state = 2239;
pub const S_AUTOPICKUPFADE8: state = 2238;
pub const S_AUTOPICKUPFADE7: state = 2237;
pub const S_AUTOPICKUPFADE6: state = 2236;
pub const S_AUTOPICKUPFADE5: state = 2235;
pub const S_AUTOPICKUPFADE4: state = 2234;
pub const S_AUTOPICKUPFADE3: state = 2233;
pub const S_AUTOPICKUPFADE2: state = 2232;
pub const S_AUTOPICKUPFADE1: state = 2231;
pub const S_AUTOPICKUP: state = 2230;
pub const S_RAILPICKUPFADE8: state = 2229;
pub const S_RAILPICKUPFADE7: state = 2228;
pub const S_RAILPICKUPFADE6: state = 2227;
pub const S_RAILPICKUPFADE5: state = 2226;
pub const S_RAILPICKUPFADE4: state = 2225;
pub const S_RAILPICKUPFADE3: state = 2224;
pub const S_RAILPICKUPFADE2: state = 2223;
pub const S_RAILPICKUPFADE1: state = 2222;
pub const S_RAILPICKUP: state = 2221;
pub const S_BOUNCEPICKUPFADE8: state = 2220;
pub const S_BOUNCEPICKUPFADE7: state = 2219;
pub const S_BOUNCEPICKUPFADE6: state = 2218;
pub const S_BOUNCEPICKUPFADE5: state = 2217;
pub const S_BOUNCEPICKUPFADE4: state = 2216;
pub const S_BOUNCEPICKUPFADE3: state = 2215;
pub const S_BOUNCEPICKUPFADE2: state = 2214;
pub const S_BOUNCEPICKUPFADE1: state = 2213;
pub const S_BOUNCEPICKUP: state = 2212;
pub const S_GRENADERINGAMMO: state = 2211;
pub const S_SCATTERRINGAMMO: state = 2210;
pub const S_EXPLOSIONRINGAMMO: state = 2209;
pub const S_AUTOMATICRINGAMMO: state = 2208;
pub const S_INFINITYRINGAMMO: state = 2207;
pub const S_RAILRINGAMMO: state = 2206;
pub const S_BOUNCERINGAMMO: state = 2205;
pub const S_RRNG7: state = 2204;
pub const S_RRNG6: state = 2203;
pub const S_RRNG5: state = 2202;
pub const S_RRNG4: state = 2201;
pub const S_RRNG3: state = 2200;
pub const S_RRNG2: state = 2199;
pub const S_RRNG1: state = 2198;
pub const S_LHRT: state = 2197;
pub const S_CORK: state = 2196;
pub const S_FINISHFLAG: state = 2195;
pub const S_GOTFLAG: state = 2194;
pub const S_TTAG: state = 2193;
pub const S_LOCKONINF4: state = 2192;
pub const S_LOCKONINF3: state = 2191;
pub const S_LOCKONINF2: state = 2190;
pub const S_LOCKONINF1: state = 2189;
pub const S_LOCKON4: state = 2188;
pub const S_LOCKON3: state = 2187;
pub const S_LOCKON2: state = 2186;
pub const S_LOCKON1: state = 2185;
pub const S_FLIGHTINDICATOR: state = 2184;
pub const S_FIVE2: state = 2183;
pub const S_FOUR2: state = 2182;
pub const S_THREE2: state = 2181;
pub const S_TWO2: state = 2180;
pub const S_ONE2: state = 2179;
pub const S_ZERO2: state = 2178;
pub const S_FIVE1: state = 2177;
pub const S_FOUR1: state = 2176;
pub const S_THREE1: state = 2175;
pub const S_TWO1: state = 2174;
pub const S_ONE1: state = 2173;
pub const S_ZERO1: state = 2172;
pub const S_SCRL: state = 2171;
pub const S_SCRK: state = 2170;
pub const S_SCRJ: state = 2169;
pub const S_SCRI: state = 2168;
pub const S_SCRH: state = 2167;
pub const S_SCRG: state = 2166;
pub const S_SCRF: state = 2165;
pub const S_SCRE: state = 2164;
pub const S_SCRD: state = 2163;
pub const S_SCRC: state = 2162;
pub const S_SCRB: state = 2161;
pub const S_SCRA: state = 2160;
pub const S_PARTICLE: state = 2159;
pub const S_SEED: state = 2158;
pub const S_FOG14: state = 2157;
pub const S_FOG13: state = 2156;
pub const S_FOG12: state = 2155;
pub const S_FOG11: state = 2154;
pub const S_FOG10: state = 2153;
pub const S_FOG9: state = 2152;
pub const S_FOG8: state = 2151;
pub const S_FOG7: state = 2150;
pub const S_FOG6: state = 2149;
pub const S_FOG5: state = 2148;
pub const S_FOG4: state = 2147;
pub const S_FOG3: state = 2146;
pub const S_FOG2: state = 2145;
pub const S_FOG1: state = 2144;
pub const S_SPINDUST_FIRE4: state = 2143;
pub const S_SPINDUST_FIRE3: state = 2142;
pub const S_SPINDUST_FIRE2: state = 2141;
pub const S_SPINDUST_FIRE1: state = 2140;
pub const S_SPINDUST_BUBBLE4: state = 2139;
pub const S_SPINDUST_BUBBLE3: state = 2138;
pub const S_SPINDUST_BUBBLE2: state = 2137;
pub const S_SPINDUST_BUBBLE1: state = 2136;
pub const S_SPINDUST4: state = 2135;
pub const S_SPINDUST3: state = 2134;
pub const S_SPINDUST2: state = 2133;
pub const S_SPINDUST1: state = 2132;
pub const S_WATERZAP: state = 2131;
pub const S_POP1: state = 2130;
pub const S_EXTRALARGEBUBBLE: state = 2129;
pub const S_LARGEBUBBLE2: state = 2128;
pub const S_LARGEBUBBLE1: state = 2127;
pub const S_MEDIUMBUBBLE: state = 2126;
pub const S_SMALLBUBBLE: state = 2125;
pub const S_SMOKE5: state = 2124;
pub const S_SMOKE4: state = 2123;
pub const S_SMOKE3: state = 2122;
pub const S_SMOKE2: state = 2121;
pub const S_SMOKE1: state = 2120;
pub const S_SPLASH3: state = 2119;
pub const S_SPLASH2: state = 2118;
pub const S_SPLASH1: state = 2117;
pub const S_LAVASPLISH: state = 2116;
pub const S_SPLISH9: state = 2115;
pub const S_SPLISH8: state = 2114;
pub const S_SPLISH7: state = 2113;
pub const S_SPLISH6: state = 2112;
pub const S_SPLISH5: state = 2111;
pub const S_SPLISH4: state = 2110;
pub const S_SPLISH3: state = 2109;
pub const S_SPLISH2: state = 2108;
pub const S_SPLISH1: state = 2107;
pub const S_SNOW3: state = 2106;
pub const S_SNOW2: state = 2105;
pub const S_SNOW1: state = 2104;
pub const S_RAINRETURN: state = 2103;
pub const S_RAIN1: state = 2102;
pub const S_REDBOOSTERSEG_FACE: state = 2101;
pub const S_REDBOOSTERSEG_RIGHT: state = 2100;
pub const S_REDBOOSTERSEG_LEFT: state = 2099;
pub const S_REDBOOSTERROLLER: state = 2098;
pub const S_YELLOWBOOSTERSEG_FACE: state = 2097;
pub const S_YELLOWBOOSTERSEG_RIGHT: state = 2096;
pub const S_YELLOWBOOSTERSEG_LEFT: state = 2095;
pub const S_YELLOWBOOSTERROLLER: state = 2094;
pub const S_BOOSTERSOUND: state = 2093;
pub const S_BHORIZ8: state = 2092;
pub const S_BHORIZ7: state = 2091;
pub const S_BHORIZ6: state = 2090;
pub const S_BHORIZ5: state = 2089;
pub const S_BHORIZ4: state = 2088;
pub const S_BHORIZ3: state = 2087;
pub const S_BHORIZ2: state = 2086;
pub const S_BHORIZ1: state = 2085;
pub const S_RHORIZ8: state = 2084;
pub const S_RHORIZ7: state = 2083;
pub const S_RHORIZ6: state = 2082;
pub const S_RHORIZ5: state = 2081;
pub const S_RHORIZ4: state = 2080;
pub const S_RHORIZ3: state = 2079;
pub const S_RHORIZ2: state = 2078;
pub const S_RHORIZ1: state = 2077;
pub const S_YHORIZ8: state = 2076;
pub const S_YHORIZ7: state = 2075;
pub const S_YHORIZ6: state = 2074;
pub const S_YHORIZ5: state = 2073;
pub const S_YHORIZ4: state = 2072;
pub const S_YHORIZ3: state = 2071;
pub const S_YHORIZ2: state = 2070;
pub const S_YHORIZ1: state = 2069;
pub const S_BDIAG8: state = 2068;
pub const S_BDIAG7: state = 2067;
pub const S_BDIAG6: state = 2066;
pub const S_BDIAG5: state = 2065;
pub const S_BDIAG4: state = 2064;
pub const S_BDIAG3: state = 2063;
pub const S_BDIAG2: state = 2062;
pub const S_BDIAG1: state = 2061;
pub const S_RDIAG8: state = 2060;
pub const S_RDIAG7: state = 2059;
pub const S_RDIAG6: state = 2058;
pub const S_RDIAG5: state = 2057;
pub const S_RDIAG4: state = 2056;
pub const S_RDIAG3: state = 2055;
pub const S_RDIAG2: state = 2054;
pub const S_RDIAG1: state = 2053;
pub const S_YDIAG8: state = 2052;
pub const S_YDIAG7: state = 2051;
pub const S_YDIAG6: state = 2050;
pub const S_YDIAG5: state = 2049;
pub const S_YDIAG4: state = 2048;
pub const S_YDIAG3: state = 2047;
pub const S_YDIAG2: state = 2046;
pub const S_YDIAG1: state = 2045;
pub const S_BLUESPRING5: state = 2044;
pub const S_BLUESPRING4: state = 2043;
pub const S_BLUESPRING3: state = 2042;
pub const S_BLUESPRING2: state = 2041;
pub const S_BLUESPRING: state = 2040;
pub const S_REDSPRING5: state = 2039;
pub const S_REDSPRING4: state = 2038;
pub const S_REDSPRING3: state = 2037;
pub const S_REDSPRING2: state = 2036;
pub const S_REDSPRING: state = 2035;
pub const S_YELLOWSPRING5: state = 2034;
pub const S_YELLOWSPRING4: state = 2033;
pub const S_YELLOWSPRING3: state = 2032;
pub const S_YELLOWSPRING2: state = 2031;
pub const S_YELLOWSPRING: state = 2030;
pub const S_BALLOONPOP6: state = 2029;
pub const S_BALLOONPOP5: state = 2028;
pub const S_BALLOONPOP4: state = 2027;
pub const S_BALLOONPOP3: state = 2026;
pub const S_BALLOONPOP2: state = 2025;
pub const S_BALLOONPOP1: state = 2024;
pub const S_BALLOON: state = 2023;
pub const S_BUMPERHIT: state = 2022;
pub const S_BUMPER: state = 2021;
pub const S_STEAM8: state = 2020;
pub const S_STEAM7: state = 2019;
pub const S_STEAM6: state = 2018;
pub const S_STEAM5: state = 2017;
pub const S_STEAM4: state = 2016;
pub const S_STEAM3: state = 2015;
pub const S_STEAM2: state = 2014;
pub const S_STEAM1: state = 2013;
pub const S_FAN5: state = 2012;
pub const S_FAN4: state = 2011;
pub const S_FAN3: state = 2010;
pub const S_FAN2: state = 2009;
pub const S_FAN: state = 2008;
pub const S_SECRETFLICKY_02_CENTER: state = 2007;
pub const S_SECRETFLICKY_02_STAND: state = 2006;
pub const S_SECRETFLICKY_02_FLAP3: state = 2005;
pub const S_SECRETFLICKY_02_FLAP2: state = 2004;
pub const S_SECRETFLICKY_02_FLAP1: state = 2003;
pub const S_SECRETFLICKY_02_OUT: state = 2002;
pub const S_SECRETFLICKY_01_CENTER: state = 2001;
pub const S_SECRETFLICKY_01_STAND: state = 2000;
pub const S_SECRETFLICKY_01_DOWN: state = 1999;
pub const S_SECRETFLICKY_01_UP: state = 1998;
pub const S_SECRETFLICKY_01_HOP: state = 1997;
pub const S_SECRETFLICKY_01_AIM: state = 1996;
pub const S_SECRETFLICKY_01_OUT: state = 1995;
pub const S_FLICKY_16_CENTER: state = 1994;
pub const S_FLICKY_16_STAND: state = 1993;
pub const S_FLICKY_16_FLAP3: state = 1992;
pub const S_FLICKY_16_FLAP2: state = 1991;
pub const S_FLICKY_16_FLAP1: state = 1990;
pub const S_FLICKY_16_OUT: state = 1989;
pub const S_FLICKY_15_CENTER: state = 1988;
pub const S_FLICKY_15_STAND: state = 1987;
pub const S_FLICKY_15_DOWN: state = 1986;
pub const S_FLICKY_15_UP: state = 1985;
pub const S_FLICKY_15_HOP: state = 1984;
pub const S_FLICKY_15_AIM: state = 1983;
pub const S_FLICKY_15_OUT: state = 1982;
pub const S_FLICKY_14_CENTER: state = 1981;
pub const S_FLICKY_14_STAND: state = 1980;
pub const S_FLICKY_14_FLAP3: state = 1979;
pub const S_FLICKY_14_FLAP2: state = 1978;
pub const S_FLICKY_14_FLAP1: state = 1977;
pub const S_FLICKY_14_OUT: state = 1976;
pub const S_FLICKY_13_CENTER: state = 1975;
pub const S_FLICKY_13_STAND: state = 1974;
pub const S_FLICKY_13_DOWN: state = 1973;
pub const S_FLICKY_13_UP: state = 1972;
pub const S_FLICKY_13_HOP: state = 1971;
pub const S_FLICKY_13_AIM: state = 1970;
pub const S_FLICKY_13_OUT: state = 1969;
pub const S_FLICKY_12_CENTER: state = 1968;
pub const S_FLICKY_12_STAND: state = 1967;
pub const S_FLICKY_12_RUN3: state = 1966;
pub const S_FLICKY_12_RUN2: state = 1965;
pub const S_FLICKY_12_RUN1: state = 1964;
pub const S_FLICKY_12_AIM: state = 1963;
pub const S_FLICKY_12_OUT: state = 1962;
pub const S_FLICKY_11_CENTER: state = 1961;
pub const S_FLICKY_11_STAND: state = 1960;
pub const S_FLICKY_11_RUN3: state = 1959;
pub const S_FLICKY_11_RUN2: state = 1958;
pub const S_FLICKY_11_RUN1: state = 1957;
pub const S_FLICKY_11_AIM: state = 1956;
pub const S_FLICKY_11_OUT: state = 1955;
pub const S_FLICKY_10_CENTER: state = 1954;
pub const S_FLICKY_10_STAND: state = 1953;
pub const S_FLICKY_10_FLAP2: state = 1952;
pub const S_FLICKY_10_FLAP1: state = 1951;
pub const S_FLICKY_10_OUT: state = 1950;
pub const S_FLICKY_09_CENTER: state = 1949;
pub const S_FLICKY_09_STAND: state = 1948;
pub const S_FLICKY_09_DOWN: state = 1947;
pub const S_FLICKY_09_UP: state = 1946;
pub const S_FLICKY_09_HOP: state = 1945;
pub const S_FLICKY_09_AIM: state = 1944;
pub const S_FLICKY_09_OUT: state = 1943;
pub const S_FLICKY_08_CENTER: state = 1942;
pub const S_FLICKY_08_STAND: state = 1941;
pub const S_FLICKY_08_SWIM4: state = 1940;
pub const S_FLICKY_08_SWIM3: state = 1939;
pub const S_FLICKY_08_SWIM2: state = 1938;
pub const S_FLICKY_08_SWIM1: state = 1937;
pub const S_FLICKY_08_FLAP4: state = 1936;
pub const S_FLICKY_08_FLAP3: state = 1935;
pub const S_FLICKY_08_FLAP2: state = 1934;
pub const S_FLICKY_08_FLAP1: state = 1933;
pub const S_FLICKY_08_HOP: state = 1932;
pub const S_FLICKY_08_AIM: state = 1931;
pub const S_FLICKY_08_OUT: state = 1930;
pub const S_FLICKY_07_CENTER: state = 1929;
pub const S_FLICKY_07_STAND: state = 1928;
pub const S_FLICKY_07_SWIM3: state = 1927;
pub const S_FLICKY_07_SWIM2: state = 1926;
pub const S_FLICKY_07_SWIM1: state = 1925;
pub const S_FLICKY_07_DOWNR: state = 1924;
pub const S_FLICKY_07_UPR: state = 1923;
pub const S_FLICKY_07_HOPR: state = 1922;
pub const S_FLICKY_07_AIMR: state = 1921;
pub const S_FLICKY_07_DOWNL: state = 1920;
pub const S_FLICKY_07_UPL: state = 1919;
pub const S_FLICKY_07_HOPL: state = 1918;
pub const S_FLICKY_07_AIML: state = 1917;
pub const S_FLICKY_07_OUT: state = 1916;
pub const S_FLICKY_06_CENTER: state = 1915;
pub const S_FLICKY_06_STAND: state = 1914;
pub const S_FLICKY_06_DOWN: state = 1913;
pub const S_FLICKY_06_UP: state = 1912;
pub const S_FLICKY_06_HOP: state = 1911;
pub const S_FLICKY_06_AIM: state = 1910;
pub const S_FLICKY_06_OUT: state = 1909;
pub const S_FLICKY_05_CENTER: state = 1908;
pub const S_FLICKY_05_STAND: state = 1907;
pub const S_FLICKY_05_DOWN: state = 1906;
pub const S_FLICKY_05_UP: state = 1905;
pub const S_FLICKY_05_HOP: state = 1904;
pub const S_FLICKY_05_AIM: state = 1903;
pub const S_FLICKY_05_OUT: state = 1902;
pub const S_FLICKY_04_CENTER: state = 1901;
pub const S_FLICKY_04_STAND: state = 1900;
pub const S_FLICKY_04_SWIM4: state = 1899;
pub const S_FLICKY_04_SWIM3: state = 1898;
pub const S_FLICKY_04_SWIM2: state = 1897;
pub const S_FLICKY_04_SWIM1: state = 1896;
pub const S_FLICKY_04_DOWN: state = 1895;
pub const S_FLICKY_04_UP: state = 1894;
pub const S_FLICKY_04_HOP: state = 1893;
pub const S_FLICKY_04_AIM: state = 1892;
pub const S_FLICKY_04_OUT: state = 1891;
pub const S_FLICKY_03_CENTER: state = 1890;
pub const S_FLICKY_03_STAND: state = 1889;
pub const S_FLICKY_03_FLAP2: state = 1888;
pub const S_FLICKY_03_FLAP1: state = 1887;
pub const S_FLICKY_03_UP: state = 1886;
pub const S_FLICKY_03_HOP: state = 1885;
pub const S_FLICKY_03_AIM: state = 1884;
pub const S_FLICKY_03_OUT: state = 1883;
pub const S_FLICKY_02_CENTER: state = 1882;
pub const S_FLICKY_02_STAND: state = 1881;
pub const S_FLICKY_02_DOWN: state = 1880;
pub const S_FLICKY_02_UP: state = 1879;
pub const S_FLICKY_02_HOP: state = 1878;
pub const S_FLICKY_02_AIM: state = 1877;
pub const S_FLICKY_02_OUT: state = 1876;
pub const S_FLICKY_01_CENTER: state = 1875;
pub const S_FLICKY_01_STAND: state = 1874;
pub const S_FLICKY_01_FLAP3: state = 1873;
pub const S_FLICKY_01_FLAP2: state = 1872;
pub const S_FLICKY_01_FLAP1: state = 1871;
pub const S_FLICKY_01_OUT: state = 1870;
pub const S_FLICKY_BUBBLE: state = 1869;
pub const S_SSPK5: state = 1868;
pub const S_SSPK4: state = 1867;
pub const S_SSPK3: state = 1866;
pub const S_SSPK2: state = 1865;
pub const S_SSPK1: state = 1864;
pub const S_IVSP: state = 1863;
pub const S_THUNDERCOIN_SPARK: state = 1862;
pub const S_ZAPSB11: state = 1861;
pub const S_ZAPSB10: state = 1860;
pub const S_ZAPSB9: state = 1859;
pub const S_ZAPSB8: state = 1858;
pub const S_ZAPSB7: state = 1857;
pub const S_ZAPSB6: state = 1856;
pub const S_ZAPSB5: state = 1855;
pub const S_ZAPSB4: state = 1854;
pub const S_ZAPSB3: state = 1853;
pub const S_ZAPSB2: state = 1852;
pub const S_ZAPSB1: state = 1851;
pub const S_ZAPS16: state = 1850;
pub const S_ZAPS15: state = 1849;
pub const S_ZAPS14: state = 1848;
pub const S_ZAPS13: state = 1847;
pub const S_ZAPS12: state = 1846;
pub const S_ZAPS11: state = 1845;
pub const S_ZAPS10: state = 1844;
pub const S_ZAPS9: state = 1843;
pub const S_ZAPS8: state = 1842;
pub const S_ZAPS7: state = 1841;
pub const S_ZAPS6: state = 1840;
pub const S_ZAPS5: state = 1839;
pub const S_ZAPS4: state = 1838;
pub const S_ZAPS3: state = 1837;
pub const S_ZAPS2: state = 1836;
pub const S_ZAPS1: state = 1835;
pub const S_BUBSB6: state = 1834;
pub const S_BUBSB5: state = 1833;
pub const S_BUBSB4: state = 1832;
pub const S_BUBSB3: state = 1831;
pub const S_BUBSB2: state = 1830;
pub const S_BUBSB1: state = 1829;
pub const S_BUBS11: state = 1828;
pub const S_BUBS10: state = 1827;
pub const S_BUBS9: state = 1826;
pub const S_BUBS8: state = 1825;
pub const S_BUBS7: state = 1824;
pub const S_BUBS6: state = 1823;
pub const S_BUBS5: state = 1822;
pub const S_BUBS4: state = 1821;
pub const S_BUBS3: state = 1820;
pub const S_BUBS2: state = 1819;
pub const S_BUBS1: state = 1818;
pub const S_FIRSB10: state = 1817;
pub const S_FIRSB9: state = 1816;
pub const S_FIRSB8: state = 1815;
pub const S_FIRSB7: state = 1814;
pub const S_FIRSB6: state = 1813;
pub const S_FIRSB5: state = 1812;
pub const S_FIRSB4: state = 1811;
pub const S_FIRSB3: state = 1810;
pub const S_FIRSB2: state = 1809;
pub const S_FIRSB1: state = 1808;
pub const S_FIRS11: state = 1807;
pub const S_FIRS10: state = 1806;
pub const S_FIRS9: state = 1805;
pub const S_FIRS8: state = 1804;
pub const S_FIRS7: state = 1803;
pub const S_FIRS6: state = 1802;
pub const S_FIRS5: state = 1801;
pub const S_FIRS4: state = 1800;
pub const S_FIRS3: state = 1799;
pub const S_FIRS2: state = 1798;
pub const S_FIRS1: state = 1797;
pub const S_PITY12: state = 1796;
pub const S_PITY11: state = 1795;
pub const S_PITY10: state = 1794;
pub const S_PITY9: state = 1793;
pub const S_PITY8: state = 1792;
pub const S_PITY7: state = 1791;
pub const S_PITY6: state = 1790;
pub const S_PITY5: state = 1789;
pub const S_PITY4: state = 1788;
pub const S_PITY3: state = 1787;
pub const S_PITY2: state = 1786;
pub const S_PITY1: state = 1785;
pub const S_ELEMF10: state = 1784;
pub const S_ELEMF9: state = 1783;
pub const S_ELEMF8: state = 1782;
pub const S_ELEMF7: state = 1781;
pub const S_ELEMF6: state = 1780;
pub const S_ELEMF5: state = 1779;
pub const S_ELEMF4: state = 1778;
pub const S_ELEMF3: state = 1777;
pub const S_ELEMF2: state = 1776;
pub const S_ELEMF1: state = 1775;
pub const S_ELEM14: state = 1774;
pub const S_ELEM13: state = 1773;
pub const S_ELEM12: state = 1772;
pub const S_ELEM11: state = 1771;
pub const S_ELEM10: state = 1770;
pub const S_ELEM9: state = 1769;
pub const S_ELEM8: state = 1768;
pub const S_ELEM7: state = 1767;
pub const S_ELEM6: state = 1766;
pub const S_ELEM5: state = 1765;
pub const S_ELEM4: state = 1764;
pub const S_ELEM3: state = 1763;
pub const S_ELEM2: state = 1762;
pub const S_ELEM1: state = 1761;
pub const S_FORC21: state = 1760;
pub const S_FORC20: state = 1759;
pub const S_FORC19: state = 1758;
pub const S_FORC18: state = 1757;
pub const S_FORC17: state = 1756;
pub const S_FORC16: state = 1755;
pub const S_FORC15: state = 1754;
pub const S_FORC14: state = 1753;
pub const S_FORC13: state = 1752;
pub const S_FORC12: state = 1751;
pub const S_FORC11: state = 1750;
pub const S_FORC10: state = 1749;
pub const S_FORC9: state = 1748;
pub const S_FORC8: state = 1747;
pub const S_FORC7: state = 1746;
pub const S_FORC6: state = 1745;
pub const S_FORC5: state = 1744;
pub const S_FORC4: state = 1743;
pub const S_FORC3: state = 1742;
pub const S_FORC2: state = 1741;
pub const S_FORC1: state = 1740;
pub const S_MAGN13: state = 1739;
pub const S_MAGN12: state = 1738;
pub const S_MAGN11: state = 1737;
pub const S_MAGN10: state = 1736;
pub const S_MAGN9: state = 1735;
pub const S_MAGN8: state = 1734;
pub const S_MAGN7: state = 1733;
pub const S_MAGN6: state = 1732;
pub const S_MAGN5: state = 1731;
pub const S_MAGN4: state = 1730;
pub const S_MAGN3: state = 1729;
pub const S_MAGN2: state = 1728;
pub const S_MAGN1: state = 1727;
pub const S_WIND8: state = 1726;
pub const S_WIND7: state = 1725;
pub const S_WIND6: state = 1724;
pub const S_WIND5: state = 1723;
pub const S_WIND4: state = 1722;
pub const S_WIND3: state = 1721;
pub const S_WIND2: state = 1720;
pub const S_WIND1: state = 1719;
pub const S_ARMB32: state = 1718;
pub const S_ARMB31: state = 1717;
pub const S_ARMB30: state = 1716;
pub const S_ARMB29: state = 1715;
pub const S_ARMB28: state = 1714;
pub const S_ARMB27: state = 1713;
pub const S_ARMB26: state = 1712;
pub const S_ARMB25: state = 1711;
pub const S_ARMB24: state = 1710;
pub const S_ARMB23: state = 1709;
pub const S_ARMB22: state = 1708;
pub const S_ARMB21: state = 1707;
pub const S_ARMB20: state = 1706;
pub const S_ARMB19: state = 1705;
pub const S_ARMB18: state = 1704;
pub const S_ARMB17: state = 1703;
pub const S_ARMB16: state = 1702;
pub const S_ARMB15: state = 1701;
pub const S_ARMB14: state = 1700;
pub const S_ARMB13: state = 1699;
pub const S_ARMB12: state = 1698;
pub const S_ARMB11: state = 1697;
pub const S_ARMB10: state = 1696;
pub const S_ARMB9: state = 1695;
pub const S_ARMB8: state = 1694;
pub const S_ARMB7: state = 1693;
pub const S_ARMB6: state = 1692;
pub const S_ARMB5: state = 1691;
pub const S_ARMB4: state = 1690;
pub const S_ARMB3: state = 1689;
pub const S_ARMB2: state = 1688;
pub const S_ARMB1: state = 1687;
pub const S_ARMF32: state = 1686;
pub const S_ARMF31: state = 1685;
pub const S_ARMF30: state = 1684;
pub const S_ARMF29: state = 1683;
pub const S_ARMF28: state = 1682;
pub const S_ARMF27: state = 1681;
pub const S_ARMF26: state = 1680;
pub const S_ARMF25: state = 1679;
pub const S_ARMF24: state = 1678;
pub const S_ARMF23: state = 1677;
pub const S_ARMF22: state = 1676;
pub const S_ARMF21: state = 1675;
pub const S_ARMF20: state = 1674;
pub const S_ARMF19: state = 1673;
pub const S_ARMF18: state = 1672;
pub const S_ARMF17: state = 1671;
pub const S_ARMF16: state = 1670;
pub const S_ARMF15: state = 1669;
pub const S_ARMF14: state = 1668;
pub const S_ARMF13: state = 1667;
pub const S_ARMF12: state = 1666;
pub const S_ARMF11: state = 1665;
pub const S_ARMF10: state = 1664;
pub const S_ARMF9: state = 1663;
pub const S_ARMF8: state = 1662;
pub const S_ARMF7: state = 1661;
pub const S_ARMF6: state = 1660;
pub const S_ARMF5: state = 1659;
pub const S_ARMF4: state = 1658;
pub const S_ARMF3: state = 1657;
pub const S_ARMF2: state = 1656;
pub const S_ARMF1: state = 1655;
pub const S_ARMA16: state = 1654;
pub const S_ARMA15: state = 1653;
pub const S_ARMA14: state = 1652;
pub const S_ARMA13: state = 1651;
pub const S_ARMA12: state = 1650;
pub const S_ARMA11: state = 1649;
pub const S_ARMA10: state = 1648;
pub const S_ARMA9: state = 1647;
pub const S_ARMA8: state = 1646;
pub const S_ARMA7: state = 1645;
pub const S_ARMA6: state = 1644;
pub const S_ARMA5: state = 1643;
pub const S_ARMA4: state = 1642;
pub const S_ARMA3: state = 1641;
pub const S_ARMA2: state = 1640;
pub const S_ARMA1: state = 1639;
pub const S_EGGSTATUE2: state = 1638;
pub const S_DBALL6: state = 1637;
pub const S_DBALL5: state = 1636;
pub const S_DBALL4: state = 1635;
pub const S_DBALL3: state = 1634;
pub const S_DBALL2: state = 1633;
pub const S_DBALL1: state = 1632;
pub const S_PALMTREE_TOP: state = 1631;
pub const S_PALMTREE_TRUNK: state = 1630;
pub const S_BIG_PALMTREE_TOP: state = 1629;
pub const S_BIG_PALMTREE_TRUNK: state = 1628;
pub const S_BSZCLOVER: state = 1627;
pub const S_BSZSHRUB: state = 1626;
pub const S_BSZVINE_ORANGE: state = 1625;
pub const S_BSZVINE_YELLOW: state = 1624;
pub const S_BSZVINE_CYAN: state = 1623;
pub const S_BSZVINE_BLUE: state = 1622;
pub const S_BSZVINE_PURPLE: state = 1621;
pub const S_BSZVINE_RED: state = 1620;
pub const S_BSZBUSH_ORANGE: state = 1619;
pub const S_BSZBUSH_YELLOW: state = 1618;
pub const S_BSZBUSH_CYAN: state = 1617;
pub const S_BSZBUSH_BLUE: state = 1616;
pub const S_BSZBUSH_PURPLE: state = 1615;
pub const S_BSZBUSH_RED: state = 1614;
pub const S_BSZCLUSTER_ORANGE: state = 1613;
pub const S_BSZCLUSTER_YELLOW: state = 1612;
pub const S_BSZCLUSTER_CYAN: state = 1611;
pub const S_BSZCLUSTER_BLUE: state = 1610;
pub const S_BSZCLUSTER_PURPLE: state = 1609;
pub const S_BSZCLUSTER_RED: state = 1608;
pub const S_BSZTULIP_ORANGE: state = 1607;
pub const S_BSZTULIP_YELLOW: state = 1606;
pub const S_BSZTULIP_CYAN: state = 1605;
pub const S_BSZTULIP_BLUE: state = 1604;
pub const S_BSZTULIP_PURPLE: state = 1603;
pub const S_BSZTULIP_RED: state = 1602;
pub const S_BSZSHORTFLOWER_ORANGE: state = 1601;
pub const S_BSZSHORTFLOWER_YELLOW: state = 1600;
pub const S_BSZSHORTFLOWER_CYAN: state = 1599;
pub const S_BSZSHORTFLOWER_BLUE: state = 1598;
pub const S_BSZSHORTFLOWER_PURPLE: state = 1597;
pub const S_BSZSHORTFLOWER_RED: state = 1596;
pub const S_BSZFLOWER_ORANGE: state = 1595;
pub const S_BSZFLOWER_YELLOW: state = 1594;
pub const S_BSZFLOWER_CYAN: state = 1593;
pub const S_BSZFLOWER_BLUE: state = 1592;
pub const S_BSZFLOWER_PURPLE: state = 1591;
pub const S_BSZFLOWER_RED: state = 1590;
pub const S_BSZTALLFLOWER_ORANGE: state = 1589;
pub const S_BSZTALLFLOWER_YELLOW: state = 1588;
pub const S_BSZTALLFLOWER_CYAN: state = 1587;
pub const S_BSZTALLFLOWER_BLUE: state = 1586;
pub const S_BSZTALLFLOWER_PURPLE: state = 1585;
pub const S_BSZTALLFLOWER_RED: state = 1584;
pub const S_HHZSTALAGMITE_SHORT: state = 1583;
pub const S_HHZSTALAGMITE_TALL: state = 1582;
pub const S_HHZTENT2: state = 1581;
pub const S_HHZTENT1: state = 1580;
pub const S_HHZGRASS: state = 1579;
pub const S_HHZSHROOM_16: state = 1578;
pub const S_HHZSHROOM_15: state = 1577;
pub const S_HHZSHROOM_14: state = 1576;
pub const S_HHZSHROOM_13: state = 1575;
pub const S_HHZSHROOM_12: state = 1574;
pub const S_HHZSHROOM_11: state = 1573;
pub const S_HHZSHROOM_10: state = 1572;
pub const S_HHZSHROOM_9: state = 1571;
pub const S_HHZSHROOM_8: state = 1570;
pub const S_HHZSHROOM_7: state = 1569;
pub const S_HHZSHROOM_6: state = 1568;
pub const S_HHZSHROOM_5: state = 1567;
pub const S_HHZSHROOM_4: state = 1566;
pub const S_HHZSHROOM_3: state = 1565;
pub const S_HHZSHROOM_2: state = 1564;
pub const S_HHZSHROOM_1: state = 1563;
pub const S_HHZTREE_LEAF: state = 1562;
pub const S_HHZTREE_TRUNK: state = 1561;
pub const S_HHZTREE_TOP: state = 1560;
pub const S_JACKO3OVERLAY_4: state = 1559;
pub const S_JACKO3OVERLAY_3: state = 1558;
pub const S_JACKO3OVERLAY_2: state = 1557;
pub const S_JACKO3OVERLAY_1: state = 1556;
pub const S_JACKO3: state = 1555;
pub const S_JACKO2OVERLAY_4: state = 1554;
pub const S_JACKO2OVERLAY_3: state = 1553;
pub const S_JACKO2OVERLAY_2: state = 1552;
pub const S_JACKO2OVERLAY_1: state = 1551;
pub const S_JACKO2: state = 1550;
pub const S_JACKO1OVERLAY_4: state = 1549;
pub const S_JACKO1OVERLAY_3: state = 1548;
pub const S_JACKO1OVERLAY_2: state = 1547;
pub const S_JACKO1OVERLAY_1: state = 1546;
pub const S_JACKO1: state = 1545;
pub const S_ROSY_UNHAPPY: state = 1544;
pub const S_ROSY_STND: state = 1543;
pub const S_ROSY_PAIN: state = 1542;
pub const S_ROSY_HUG: state = 1541;
pub const S_ROSY_WALK: state = 1540;
pub const S_ROSY_JUMP: state = 1539;
pub const S_ROSY_IDLE4: state = 1538;
pub const S_ROSY_IDLE3: state = 1537;
pub const S_ROSY_IDLE2: state = 1536;
pub const S_ROSY_IDLE1: state = 1535;
pub const S_FHZICE2: state = 1534;
pub const S_FHZICE1: state = 1533;
pub const S_XMASBUSH: state = 1532;
pub const S_XMASBERRYBUSH: state = 1531;
pub const S_XMASBLUEBERRYBUSH: state = 1530;
pub const S_MISTLETOE: state = 1529;
pub const S_HANGSTAR: state = 1528;
pub const S_LAMPPOST2: state = 1527;
pub const S_LAMPPOST1: state = 1526;
pub const S_SNOWMANHAT: state = 1525;
pub const S_SNOWMAN: state = 1524;
pub const S_CANDYCANE: state = 1523;
pub const S_XMASPOLE: state = 1522;
pub const S_STG9: state = 1521;
pub const S_STG8: state = 1520;
pub const S_STG7: state = 1519;
pub const S_STG6: state = 1518;
pub const S_STG5: state = 1517;
pub const S_STG4: state = 1516;
pub const S_STG3: state = 1515;
pub const S_STG2: state = 1514;
pub const S_STG1: state = 1513;
pub const S_STG0: state = 1512;
pub const S_BLUEGARGOYLE: state = 1511;
pub const S_GREENFLAME: state = 1510;
pub const S_TARGET_ALLDONE: state = 1509;
pub const S_TARGET_RESPAWN: state = 1508;
pub const S_TARGET_HIT2: state = 1507;
pub const S_TARGET_HIT1: state = 1506;
pub const S_TARGET_IDLE: state = 1505;
pub const S_GLAREGOYLELONG_COOLDOWN: state = 1504;
pub const S_GLAREGOYLELONG_LOOP: state = 1503;
pub const S_GLAREGOYLELONG_FIRE: state = 1502;
pub const S_GLAREGOYLELONG_HOLD: state = 1501;
pub const S_GLAREGOYLELONG_BLINK: state = 1500;
pub const S_GLAREGOYLELONG_CHARGE: state = 1499;
pub const S_GLAREGOYLELONG: state = 1498;
pub const S_GLAREGOYLEDOWN_COOLDOWN: state = 1497;
pub const S_GLAREGOYLEDOWN_LOOP: state = 1496;
pub const S_GLAREGOYLEDOWN_FIRE: state = 1495;
pub const S_GLAREGOYLEDOWN_HOLD: state = 1494;
pub const S_GLAREGOYLEDOWN_BLINK: state = 1493;
pub const S_GLAREGOYLEDOWN_CHARGE: state = 1492;
pub const S_GLAREGOYLEDOWN: state = 1491;
pub const S_GLAREGOYLEUP_COOLDOWN: state = 1490;
pub const S_GLAREGOYLEUP_LOOP: state = 1489;
pub const S_GLAREGOYLEUP_FIRE: state = 1488;
pub const S_GLAREGOYLEUP_HOLD: state = 1487;
pub const S_GLAREGOYLEUP_BLINK: state = 1486;
pub const S_GLAREGOYLEUP_CHARGE: state = 1485;
pub const S_GLAREGOYLEUP: state = 1484;
pub const S_GLAREGOYLE_COOLDOWN: state = 1483;
pub const S_GLAREGOYLE_LOOP: state = 1482;
pub const S_GLAREGOYLE_FIRE: state = 1481;
pub const S_GLAREGOYLE_HOLD: state = 1480;
pub const S_GLAREGOYLE_BLINK: state = 1479;
pub const S_GLAREGOYLE_CHARGE: state = 1478;
pub const S_GLAREGOYLE: state = 1477;
pub const S_WALLVINE_SHORT: state = 1476;
pub const S_WALLVINE_LONG: state = 1475;
pub const S_TORCHFLOWER: state = 1474;
pub const S_JUNGLEPALM: state = 1473;
pub const S_BIGFERN2: state = 1472;
pub const S_BIGFERN1: state = 1471;
pub const S_BIGFERNLEAF: state = 1470;
pub const S_ROLLOUTROCK: state = 1469;
pub const S_ROLLOUTSPAWN: state = 1468;
pub const S_LAVAFALLROCK: state = 1467;
pub const S_LAVAFALL_LAVA3: state = 1466;
pub const S_LAVAFALL_LAVA2: state = 1465;
pub const S_LAVAFALL_LAVA1: state = 1464;
pub const S_LAVAFALL_SHOOT: state = 1463;
pub const S_LAVAFALL_TELL: state = 1462;
pub const S_LAVAFALL_DORMANT: state = 1461;
pub const S_FLAMEJETFLAMEB3: state = 1460;
pub const S_FLAMEJETFLAMEB2: state = 1459;
pub const S_FLAMEJETFLAMEB1: state = 1458;
pub const S_FJSPINAXISB2: state = 1457;
pub const S_FJSPINAXISB1: state = 1456;
pub const S_FJSPINAXISA2: state = 1455;
pub const S_FJSPINAXISA1: state = 1454;
pub const S_FLAMEJETFLAME9: state = 1453;
pub const S_FLAMEJETFLAME8: state = 1452;
pub const S_FLAMEJETFLAME7: state = 1451;
pub const S_FLAMEJETFLAME6: state = 1450;
pub const S_FLAMEJETFLAME5: state = 1449;
pub const S_FLAMEJETFLAME4: state = 1448;
pub const S_FLAMEJETFLAME3: state = 1447;
pub const S_FLAMEJETFLAME2: state = 1446;
pub const S_FLAMEJETFLAME1: state = 1445;
pub const S_FLAMEJETSTOP: state = 1444;
pub const S_FLAMEJETSTART: state = 1443;
pub const S_FLAMEJETSTND: state = 1442;
pub const S_TRAINSTEAM: state = 1441;
pub const S_TRAINDUST: state = 1440;
pub const S_TRAINPUFFMAKER: state = 1439;
pub const S_TRAINCAMEOSPAWNER_5: state = 1438;
pub const S_TRAINCAMEOSPAWNER_4: state = 1437;
pub const S_TRAINCAMEOSPAWNER_3: state = 1436;
pub const S_TRAINCAMEOSPAWNER_2: state = 1435;
pub const S_TRAINCAMEOSPAWNER_1: state = 1434;
pub const S_SALOONDOORCENTER: state = 1433;
pub const S_SALOONDOOR: state = 1432;
pub const S_MINECARTSPARK: state = 1431;
pub const S_MINECARTSIDEMARK2: state = 1430;
pub const S_MINECARTSIDEMARK1: state = 1429;
pub const S_MINECARTSEG_RIGHT: state = 1428;
pub const S_MINECARTSEG_LEFT: state = 1427;
pub const S_MINECARTSEG_BACK: state = 1426;
pub const S_MINECARTSEG_FRONT: state = 1425;
pub const S_MINECARTEND: state = 1424;
pub const S_MINECART_DTH1: state = 1423;
pub const S_MINECART_IDLE: state = 1422;
pub const S_ARIDDUST3: state = 1421;
pub const S_ARIDDUST2: state = 1420;
pub const S_ARIDDUST1: state = 1419;
pub const S_DUSTLAYER5: state = 1418;
pub const S_DUSTLAYER4: state = 1417;
pub const S_DUSTLAYER3: state = 1416;
pub const S_DUSTLAYER2: state = 1415;
pub const S_DUSTLAYER1: state = 1414;
pub const S_DUSTDEVIL: state = 1413;
pub const S_PROXIMITY_TNT_TRIGGER23: state = 1412;
pub const S_PROXIMITY_TNT_TRIGGER22: state = 1411;
pub const S_PROXIMITY_TNT_TRIGGER21: state = 1410;
pub const S_PROXIMITY_TNT_TRIGGER20: state = 1409;
pub const S_PROXIMITY_TNT_TRIGGER19: state = 1408;
pub const S_PROXIMITY_TNT_TRIGGER18: state = 1407;
pub const S_PROXIMITY_TNT_TRIGGER17: state = 1406;
pub const S_PROXIMITY_TNT_TRIGGER16: state = 1405;
pub const S_PROXIMITY_TNT_TRIGGER15: state = 1404;
pub const S_PROXIMITY_TNT_TRIGGER14: state = 1403;
pub const S_PROXIMITY_TNT_TRIGGER13: state = 1402;
pub const S_PROXIMITY_TNT_TRIGGER12: state = 1401;
pub const S_PROXIMITY_TNT_TRIGGER11: state = 1400;
pub const S_PROXIMITY_TNT_TRIGGER10: state = 1399;
pub const S_PROXIMITY_TNT_TRIGGER9: state = 1398;
pub const S_PROXIMITY_TNT_TRIGGER8: state = 1397;
pub const S_PROXIMITY_TNT_TRIGGER7: state = 1396;
pub const S_PROXIMITY_TNT_TRIGGER6: state = 1395;
pub const S_PROXIMITY_TNT_TRIGGER5: state = 1394;
pub const S_PROXIMITY_TNT_TRIGGER4: state = 1393;
pub const S_PROXIMITY_TNT_TRIGGER3: state = 1392;
pub const S_PROXIMITY_TNT_TRIGGER2: state = 1391;
pub const S_PROXIMITY_TNT_TRIGGER1: state = 1390;
pub const S_PROXIMITY_TNT: state = 1389;
pub const S_TNTBARREL_FLYING: state = 1388;
pub const S_TNTBARREL_EXPL7: state = 1387;
pub const S_TNTBARREL_EXPL6: state = 1386;
pub const S_TNTBARREL_EXPL5: state = 1385;
pub const S_TNTBARREL_EXPL4: state = 1384;
pub const S_TNTBARREL_EXPL3: state = 1383;
pub const S_TNTBARREL_EXPL2: state = 1382;
pub const S_TNTBARREL_EXPL1: state = 1381;
pub const S_TNTBARREL_STND1: state = 1380;
pub const S_OILLAMPFLARE: state = 1379;
pub const S_OILLAMP: state = 1378;
pub const S_ARIDSIGN_SHARPTURN: state = 1377;
pub const S_ARIDSIGN_CACTI: state = 1376;
pub const S_ARIDSIGN_CAUTION: state = 1375;
pub const S_CACTISMALLSEG: state = 1374;
pub const S_CACTITINYSEG: state = 1373;
pub const S_CACTI11: state = 1372;
pub const S_CACTI10: state = 1371;
pub const S_CACTI9: state = 1370;
pub const S_CACTI8: state = 1369;
pub const S_CACTI7: state = 1368;
pub const S_CACTI6: state = 1367;
pub const S_CACTI5: state = 1366;
pub const S_CACTI4: state = 1365;
pub const S_CACTI3: state = 1364;
pub const S_CACTI2: state = 1363;
pub const S_CACTI1: state = 1362;
pub const S_LITTLETUMBLEWEED_ROLL8: state = 1361;
pub const S_LITTLETUMBLEWEED_ROLL7: state = 1360;
pub const S_LITTLETUMBLEWEED_ROLL6: state = 1359;
pub const S_LITTLETUMBLEWEED_ROLL5: state = 1358;
pub const S_LITTLETUMBLEWEED_ROLL4: state = 1357;
pub const S_LITTLETUMBLEWEED_ROLL3: state = 1356;
pub const S_LITTLETUMBLEWEED_ROLL2: state = 1355;
pub const S_LITTLETUMBLEWEED_ROLL1: state = 1354;
pub const S_LITTLETUMBLEWEED: state = 1353;
pub const S_BIGTUMBLEWEED_ROLL8: state = 1352;
pub const S_BIGTUMBLEWEED_ROLL7: state = 1351;
pub const S_BIGTUMBLEWEED_ROLL6: state = 1350;
pub const S_BIGTUMBLEWEED_ROLL5: state = 1349;
pub const S_BIGTUMBLEWEED_ROLL4: state = 1348;
pub const S_BIGTUMBLEWEED_ROLL3: state = 1347;
pub const S_BIGTUMBLEWEED_ROLL2: state = 1346;
pub const S_BIGTUMBLEWEED_ROLL1: state = 1345;
pub const S_BIGTUMBLEWEED: state = 1344;
pub const S_BRAMBLES: state = 1343;
pub const S_SUSPICIOUSFACESTABBERSTATUE_BURST2: state = 1342;
pub const S_SUSPICIOUSFACESTABBERSTATUE_BURST1: state = 1341;
pub const S_SUSPICIOUSFACESTABBERSTATUE_WAIT: state = 1340;
pub const S_FACESTABBERSTATUE: state = 1339;
pub const S_CRAWLASTATUE: state = 1338;
pub const S_WAVINGFLAGSEG2: state = 1337;
pub const S_WAVINGFLAGSEG1: state = 1336;
pub const S_WAVINGFLAG: state = 1335;
pub const S_FIRETORCH: state = 1334;
pub const S_FLAMEHOLDER: state = 1333;
pub const S_CANDLEPRICKET: state = 1332;
pub const S_CANDLE: state = 1331;
pub const S_CEZBUSH2: state = 1330;
pub const S_CEZBUSH1: state = 1329;
pub const S_PINETREE: state = 1328;
pub const S_CEZBANNER2: state = 1327;
pub const S_CEZBANNER1: state = 1326;
pub const S_CEZPOLE: state = 1325;
pub const S_CEZFLOWER: state = 1324;
pub const S_BIGFIREBAR16: state = 1323;
pub const S_BIGFIREBAR15: state = 1322;
pub const S_BIGFIREBAR14: state = 1321;
pub const S_BIGFIREBAR13: state = 1320;
pub const S_BIGFIREBAR12: state = 1319;
pub const S_BIGFIREBAR11: state = 1318;
pub const S_BIGFIREBAR10: state = 1317;
pub const S_BIGFIREBAR9: state = 1316;
pub const S_BIGFIREBAR8: state = 1315;
pub const S_BIGFIREBAR7: state = 1314;
pub const S_BIGFIREBAR6: state = 1313;
pub const S_BIGFIREBAR5: state = 1312;
pub const S_BIGFIREBAR4: state = 1311;
pub const S_BIGFIREBAR3: state = 1310;
pub const S_BIGFIREBAR2: state = 1309;
pub const S_BIGFIREBAR1: state = 1308;
pub const S_SMALLFIREBAR16: state = 1307;
pub const S_SMALLFIREBAR15: state = 1306;
pub const S_SMALLFIREBAR14: state = 1305;
pub const S_SMALLFIREBAR13: state = 1304;
pub const S_SMALLFIREBAR12: state = 1303;
pub const S_SMALLFIREBAR11: state = 1302;
pub const S_SMALLFIREBAR10: state = 1301;
pub const S_SMALLFIREBAR9: state = 1300;
pub const S_SMALLFIREBAR8: state = 1299;
pub const S_SMALLFIREBAR7: state = 1298;
pub const S_SMALLFIREBAR6: state = 1297;
pub const S_SMALLFIREBAR5: state = 1296;
pub const S_SMALLFIREBAR4: state = 1295;
pub const S_SMALLFIREBAR3: state = 1294;
pub const S_SMALLFIREBAR2: state = 1293;
pub const S_SMALLFIREBAR1: state = 1292;
pub const S_REDSPRINGBALL5: state = 1291;
pub const S_REDSPRINGBALL4: state = 1290;
pub const S_REDSPRINGBALL3: state = 1289;
pub const S_REDSPRINGBALL2: state = 1288;
pub const S_REDSPRINGBALL: state = 1287;
pub const S_YELLOWSPRINGBALL5: state = 1286;
pub const S_YELLOWSPRINGBALL4: state = 1285;
pub const S_YELLOWSPRINGBALL3: state = 1284;
pub const S_YELLOWSPRINGBALL2: state = 1283;
pub const S_YELLOWSPRINGBALL: state = 1282;
pub const S_BIGGRABCHAIN: state = 1281;
pub const S_SMALLGRABCHAIN: state = 1280;
pub const S_BIGMACE: state = 1279;
pub const S_SMALLMACE: state = 1278;
pub const S_BIGMACECHAIN: state = 1277;
pub const S_SMALLMACECHAIN: state = 1276;
pub const S_SLING2: state = 1275;
pub const S_SLING1: state = 1274;
pub const S_EGGSTATUE1: state = 1273;
pub const S_FLAMEREST: state = 1272;
pub const S_FLAMEPARTICLE: state = 1271;
pub const S_FLAME: state = 1270;
pub const S_CEZCHAIN: state = 1269;
pub const S_LIGHTBEAM12: state = 1268;
pub const S_LIGHTBEAM11: state = 1267;
pub const S_LIGHTBEAM10: state = 1266;
pub const S_LIGHTBEAM9: state = 1265;
pub const S_LIGHTBEAM8: state = 1264;
pub const S_LIGHTBEAM7: state = 1263;
pub const S_LIGHTBEAM6: state = 1262;
pub const S_LIGHTBEAM5: state = 1261;
pub const S_LIGHTBEAM4: state = 1260;
pub const S_LIGHTBEAM3: state = 1259;
pub const S_LIGHTBEAM2: state = 1258;
pub const S_LIGHTBEAM1: state = 1257;
pub const S_DSZ2STALAGMITE: state = 1256;
pub const S_DSZSTALAGMITE: state = 1255;
pub const S_ANIMALGAESEG: state = 1254;
pub const S_ANIMALGAETOP2: state = 1253;
pub const S_ANIMALGAETOP1: state = 1252;
pub const S_KELP: state = 1251;
pub const S_BLUECRYSTAL1: state = 1250;
pub const S_CORAL5: state = 1249;
pub const S_CORAL4: state = 1248;
pub const S_CORAL3: state = 1247;
pub const S_CORAL2: state = 1246;
pub const S_CORAL1: state = 1245;
pub const S_DRIPC2: state = 1244;
pub const S_DRIPC1: state = 1243;
pub const S_DRIPB1: state = 1242;
pub const S_DRIPA4: state = 1241;
pub const S_DRIPA3: state = 1240;
pub const S_DRIPA2: state = 1239;
pub const S_DRIPA1: state = 1238;
pub const S_SEAWEED6: state = 1237;
pub const S_SEAWEED5: state = 1236;
pub const S_SEAWEED4: state = 1235;
pub const S_SEAWEED3: state = 1234;
pub const S_SEAWEED2: state = 1233;
pub const S_SEAWEED1: state = 1232;
pub const S_BIGGARGOYLE: state = 1231;
pub const S_GARGOYLE: state = 1230;
pub const S_ALARM1: state = 1229;
pub const S_THZTREEBRANCH13: state = 1228;
pub const S_THZTREEBRANCH12: state = 1227;
pub const S_THZTREEBRANCH11: state = 1226;
pub const S_THZTREEBRANCH10: state = 1225;
pub const S_THZTREEBRANCH9: state = 1224;
pub const S_THZTREEBRANCH8: state = 1223;
pub const S_THZTREEBRANCH7: state = 1222;
pub const S_THZTREEBRANCH6: state = 1221;
pub const S_THZTREEBRANCH5: state = 1220;
pub const S_THZTREEBRANCH4: state = 1219;
pub const S_THZTREEBRANCH3: state = 1218;
pub const S_THZTREEBRANCH2: state = 1217;
pub const S_THZTREEBRANCH1: state = 1216;
pub const S_THZTREE: state = 1215;
pub const S_THZFLOWERC: state = 1214;
pub const S_THZFLOWERB: state = 1213;
pub const S_THZFLOWERA: state = 1212;
pub const S_SPRINGTREE: state = 1211;
pub const S_BUSHREDTREE: state = 1210;
pub const S_BUSHTREE: state = 1209;
pub const S_POLYGONTREE: state = 1208;
pub const S_FHZPINKTREE: state = 1207;
pub const S_FHZTREE: state = 1206;
pub const S_CHECKERSUNSETTREE: state = 1205;
pub const S_CHECKERTREE: state = 1204;
pub const S_GFZCHERRYTREE: state = 1203;
pub const S_GFZBERRYTREE: state = 1202;
pub const S_GFZTREE: state = 1201;
pub const S_BUSH: state = 1200;
pub const S_BERRYBUSH: state = 1199;
pub const S_BLUEBERRYBUSH: state = 1198;
pub const S_GFZFLOWERC: state = 1197;
pub const S_GFZFLOWERB: state = 1196;
pub const S_GFZFLOWERA: state = 1195;
pub const S_TUTORIALFLOWERF16: state = 1194;
pub const S_TUTORIALFLOWERF15: state = 1193;
pub const S_TUTORIALFLOWERF14: state = 1192;
pub const S_TUTORIALFLOWERF13: state = 1191;
pub const S_TUTORIALFLOWERF12: state = 1190;
pub const S_TUTORIALFLOWERF11: state = 1189;
pub const S_TUTORIALFLOWERF10: state = 1188;
pub const S_TUTORIALFLOWERF9: state = 1187;
pub const S_TUTORIALFLOWERF8: state = 1186;
pub const S_TUTORIALFLOWERF7: state = 1185;
pub const S_TUTORIALFLOWERF6: state = 1184;
pub const S_TUTORIALFLOWERF5: state = 1183;
pub const S_TUTORIALFLOWERF4: state = 1182;
pub const S_TUTORIALFLOWERF3: state = 1181;
pub const S_TUTORIALFLOWERF2: state = 1180;
pub const S_TUTORIALFLOWERF1: state = 1179;
pub const S_TUTORIALFLOWER16: state = 1178;
pub const S_TUTORIALFLOWER15: state = 1177;
pub const S_TUTORIALFLOWER14: state = 1176;
pub const S_TUTORIALFLOWER13: state = 1175;
pub const S_TUTORIALFLOWER12: state = 1174;
pub const S_TUTORIALFLOWER11: state = 1173;
pub const S_TUTORIALFLOWER10: state = 1172;
pub const S_TUTORIALFLOWER9: state = 1171;
pub const S_TUTORIALFLOWER8: state = 1170;
pub const S_TUTORIALFLOWER7: state = 1169;
pub const S_TUTORIALFLOWER6: state = 1168;
pub const S_TUTORIALFLOWER5: state = 1167;
pub const S_TUTORIALFLOWER4: state = 1166;
pub const S_TUTORIALFLOWER3: state = 1165;
pub const S_TUTORIALFLOWER2: state = 1164;
pub const S_TUTORIALFLOWER1: state = 1163;
pub const S_TUTORIALLEAF16: state = 1162;
pub const S_TUTORIALLEAF15: state = 1161;
pub const S_TUTORIALLEAF14: state = 1160;
pub const S_TUTORIALLEAF13: state = 1159;
pub const S_TUTORIALLEAF12: state = 1158;
pub const S_TUTORIALLEAF11: state = 1157;
pub const S_TUTORIALLEAF10: state = 1156;
pub const S_TUTORIALLEAF9: state = 1155;
pub const S_TUTORIALLEAF8: state = 1154;
pub const S_TUTORIALLEAF7: state = 1153;
pub const S_TUTORIALLEAF6: state = 1152;
pub const S_TUTORIALLEAF5: state = 1151;
pub const S_TUTORIALLEAF4: state = 1150;
pub const S_TUTORIALLEAF3: state = 1149;
pub const S_TUTORIALLEAF2: state = 1148;
pub const S_TUTORIALLEAF1: state = 1147;
pub const S_LETTER: state = 1146;
pub const S_DEMONFIRE: state = 1145;
pub const S_ARROWBONK: state = 1144;
pub const S_ARROW: state = 1143;
pub const S_CANNONBALL1: state = 1142;
pub const S_TURRETLASEREXPLODE2: state = 1141;
pub const S_TURRETLASEREXPLODE1: state = 1140;
pub const S_TURRETLASER: state = 1139;
pub const S_JETBULLET2: state = 1138;
pub const S_JETBULLET1: state = 1137;
pub const S_MINE_BOOM4: state = 1136;
pub const S_MINE_BOOM3: state = 1135;
pub const S_MINE_BOOM2: state = 1134;
pub const S_MINE_BOOM1: state = 1133;
pub const S_MINE1: state = 1132;
pub const S_ENERGYBALL2: state = 1131;
pub const S_ENERGYBALL1: state = 1130;
pub const S_TORPEDO: state = 1129;
pub const S_LASERFLAME5: state = 1128;
pub const S_LASERFLAME4: state = 1127;
pub const S_LASERFLAME3: state = 1126;
pub const S_LASERFLAME2: state = 1125;
pub const S_LASERFLAME1: state = 1124;
pub const S_LASERFLASH: state = 1123;
pub const S_LASER2: state = 1122;
pub const S_LASER: state = 1121;
pub const S_ROCKET: state = 1120;
pub const S_THUNDERCOIN_ICON2: state = 1119;
pub const S_THUNDERCOIN_ICON1: state = 1118;
pub const S_BUBBLEWRAP_ICON2: state = 1117;
pub const S_BUBBLEWRAP_ICON1: state = 1116;
pub const S_FLAMEAURA_ICON2: state = 1115;
pub const S_FLAMEAURA_ICON1: state = 1114;
pub const S_SCORE10K_ICON2: state = 1113;
pub const S_SCORE10K_ICON1: state = 1112;
pub const S_SCORE1K_ICON2: state = 1111;
pub const S_SCORE1K_ICON1: state = 1110;
pub const S_RECYCLER_ICON2: state = 1109;
pub const S_RECYCLER_ICON1: state = 1108;
pub const S_GRAVITY_ICON2: state = 1107;
pub const S_GRAVITY_ICON1: state = 1106;
pub const S_MIXUP_ICON2: state = 1105;
pub const S_MIXUP_ICON1: state = 1104;
pub const S_EGGMAN_ICON2: state = 1103;
pub const S_EGGMAN_ICON1: state = 1102;
pub const S_1UP_ICON2: state = 1101;
pub const S_1UP_ICON1: state = 1100;
pub const S_INVULN_ICON2: state = 1099;
pub const S_INVULN_ICON1: state = 1098;
pub const S_SNEAKERS_ICON2: state = 1097;
pub const S_SNEAKERS_ICON1: state = 1096;
pub const S_ELEMENTAL_ICON2: state = 1095;
pub const S_ELEMENTAL_ICON1: state = 1094;
pub const S_WHIRLWIND_ICON2: state = 1093;
pub const S_WHIRLWIND_ICON1: state = 1092;
pub const S_ARMAGEDDON_ICON2: state = 1091;
pub const S_ARMAGEDDON_ICON1: state = 1090;
pub const S_FORCE_ICON2: state = 1089;
pub const S_FORCE_ICON1: state = 1088;
pub const S_ATTRACT_ICON2: state = 1087;
pub const S_ATTRACT_ICON1: state = 1086;
pub const S_PITY_ICON2: state = 1085;
pub const S_PITY_ICON1: state = 1084;
pub const S_RING_ICON2: state = 1083;
pub const S_RING_ICON1: state = 1082;
pub const S_BLUEBOX_POP2: state = 1081;
pub const S_BLUEBOX_POP1: state = 1080;
pub const S_RING_BLUEBOX2: state = 1079;
pub const S_RING_BLUEBOX1: state = 1078;
pub const S_REDBOX_POP2: state = 1077;
pub const S_REDBOX_POP1: state = 1076;
pub const S_RING_REDBOX2: state = 1075;
pub const S_RING_REDBOX1: state = 1074;
pub const S_THUNDERCOIN_GOLDBOX: state = 1073;
pub const S_BUBBLEWRAP_GOLDBOX: state = 1072;
pub const S_FLAMEAURA_GOLDBOX: state = 1071;
pub const S_GRAVITY_GOLDBOX: state = 1070;
pub const S_EGGMAN_GOLDBOX: state = 1069;
pub const S_INVULN_GOLDBOX: state = 1068;
pub const S_SNEAKERS_GOLDBOX: state = 1067;
pub const S_ELEMENTAL_GOLDBOX: state = 1066;
pub const S_WHIRLWIND_GOLDBOX: state = 1065;
pub const S_ARMAGEDDON_GOLDBOX: state = 1064;
pub const S_FORCE_GOLDBOX: state = 1063;
pub const S_ATTRACT_GOLDBOX: state = 1062;
pub const S_PITY_GOLDBOX: state = 1061;
pub const S_THUNDERCOIN_BOX: state = 1060;
pub const S_BUBBLEWRAP_BOX: state = 1059;
pub const S_FLAMEAURA_BOX: state = 1058;
pub const S_SCORE10K_BOX: state = 1057;
pub const S_SCORE1K_BOX: state = 1056;
pub const S_RECYCLER_BOX: state = 1055;
pub const S_GRAVITY_BOX: state = 1054;
pub const S_MIXUP_BOX: state = 1053;
pub const S_EGGMAN_BOX: state = 1052;
pub const S_1UP_BOX: state = 1051;
pub const S_INVULN_BOX: state = 1050;
pub const S_SNEAKERS_BOX: state = 1049;
pub const S_ELEMENTAL_BOX: state = 1048;
pub const S_WHIRLWIND_BOX: state = 1047;
pub const S_ARMAGEDDON_BOX: state = 1046;
pub const S_FORCE_BOX: state = 1045;
pub const S_ATTRACT_BOX: state = 1044;
pub const S_PITY_BOX: state = 1043;
pub const S_RING_BOX: state = 1042;
pub const S_MYSTERY_BOX: state = 1041;
pub const S_GOLDBOX_OFF7: state = 1040;
pub const S_GOLDBOX_OFF6: state = 1039;
pub const S_GOLDBOX_OFF5: state = 1038;
pub const S_GOLDBOX_OFF4: state = 1037;
pub const S_GOLDBOX_OFF3: state = 1036;
pub const S_GOLDBOX_OFF2: state = 1035;
pub const S_GOLDBOX_OFF1: state = 1034;
pub const S_GOLDBOX_FLICKER: state = 1033;
pub const S_BOX_POP2: state = 1032;
pub const S_BOX_POP1: state = 1031;
pub const S_BOX_FLICKER: state = 1030;
pub const S_BOXSPARKLE4: state = 1029;
pub const S_BOXSPARKLE3: state = 1028;
pub const S_BOXSPARKLE2: state = 1027;
pub const S_BOXSPARKLE1: state = 1026;
pub const S_CANNONLAUNCHER3: state = 1025;
pub const S_CANNONLAUNCHER2: state = 1024;
pub const S_CANNONLAUNCHER1: state = 1023;
pub const S_BIGMINE_BLAST5: state = 1022;
pub const S_BIGMINE_BLAST4: state = 1021;
pub const S_BIGMINE_BLAST3: state = 1020;
pub const S_BIGMINE_BLAST2: state = 1019;
pub const S_BIGMINE_BLAST1: state = 1018;
pub const S_BIGMINE_SET3: state = 1017;
pub const S_BIGMINE_SET2: state = 1016;
pub const S_BIGMINE_SET1: state = 1015;
pub const S_BIGMINE_ALERT3: state = 1014;
pub const S_BIGMINE_ALERT2: state = 1013;
pub const S_BIGMINE_ALERT1: state = 1012;
pub const S_BIGMINE_IDLE: state = 1011;
pub const S_STARPOST_ENDSPIN: state = 1010;
pub const S_STARPOST_SPIN: state = 1009;
pub const S_STARPOST_STARTSPIN: state = 1008;
pub const S_STARPOST_FLASH: state = 1007;
pub const S_STARPOST_IDLE: state = 1006;
pub const S_WALLSPIKED2: state = 1005;
pub const S_WALLSPIKED1: state = 1004;
pub const S_WALLSPIKEBASE: state = 1003;
pub const S_WALLSPIKE6: state = 1002;
pub const S_WALLSPIKE5: state = 1001;
pub const S_WALLSPIKE4: state = 1000;
pub const S_WALLSPIKE3: state = 999;
pub const S_WALLSPIKE2: state = 998;
pub const S_WALLSPIKE1: state = 997;
pub const S_SPIKED2: state = 996;
pub const S_SPIKED1: state = 995;
pub const S_SPIKE6: state = 994;
pub const S_SPIKE5: state = 993;
pub const S_SPIKE4: state = 992;
pub const S_SPIKE3: state = 991;
pub const S_SPIKE2: state = 990;
pub const S_SPIKE1: state = 989;
pub const S_TEAM_SPINFIRE6: state = 988;
pub const S_TEAM_SPINFIRE5: state = 987;
pub const S_TEAM_SPINFIRE4: state = 986;
pub const S_TEAM_SPINFIRE3: state = 985;
pub const S_TEAM_SPINFIRE2: state = 984;
pub const S_TEAM_SPINFIRE1: state = 983;
pub const S_SPINFIRE6: state = 982;
pub const S_SPINFIRE5: state = 981;
pub const S_SPINFIRE4: state = 980;
pub const S_SPINFIRE3: state = 979;
pub const S_SPINFIRE2: state = 978;
pub const S_SPINFIRE1: state = 977;
pub const S_SPIKEBALL8: state = 976;
pub const S_SPIKEBALL7: state = 975;
pub const S_SPIKEBALL6: state = 974;
pub const S_SPIKEBALL5: state = 973;
pub const S_SPIKEBALL4: state = 972;
pub const S_SPIKEBALL3: state = 971;
pub const S_SPIKEBALL2: state = 970;
pub const S_SPIKEBALL1: state = 969;
pub const S_CLEARSIGN: state = 968;
pub const S_EGGMANSIGN: state = 967;
pub const S_SIGNBOARD: state = 966;
pub const S_SIGNSTOP: state = 965;
pub const S_SIGNSLOW: state = 964;
pub const S_SIGNPLAYER: state = 963;
pub const S_SIGNSPIN6: state = 962;
pub const S_SIGNSPIN5: state = 961;
pub const S_SIGNSPIN4: state = 960;
pub const S_SIGNSPIN3: state = 959;
pub const S_SIGNSPIN2: state = 958;
pub const S_SIGNSPIN1: state = 957;
pub const S_SIGN: state = 956;
pub const S_BUBBLES4: state = 955;
pub const S_BUBBLES3: state = 954;
pub const S_BUBBLES2: state = 953;
pub const S_BUBBLES1: state = 952;
pub const S_SHRD3: state = 951;
pub const S_SHRD2: state = 950;
pub const S_SHRD1: state = 949;
pub const S_CEMG7: state = 948;
pub const S_CEMG6: state = 947;
pub const S_CEMG5: state = 946;
pub const S_CEMG4: state = 945;
pub const S_CEMG3: state = 944;
pub const S_CEMG2: state = 943;
pub const S_CEMG1: state = 942;
pub const S_EMBLEM26: state = 941;
pub const S_EMBLEM25: state = 940;
pub const S_EMBLEM24: state = 939;
pub const S_EMBLEM23: state = 938;
pub const S_EMBLEM22: state = 937;
pub const S_EMBLEM21: state = 936;
pub const S_EMBLEM20: state = 935;
pub const S_EMBLEM19: state = 934;
pub const S_EMBLEM18: state = 933;
pub const S_EMBLEM17: state = 932;
pub const S_EMBLEM16: state = 931;
pub const S_EMBLEM15: state = 930;
pub const S_EMBLEM14: state = 929;
pub const S_EMBLEM13: state = 928;
pub const S_EMBLEM12: state = 927;
pub const S_EMBLEM11: state = 926;
pub const S_EMBLEM10: state = 925;
pub const S_EMBLEM9: state = 924;
pub const S_EMBLEM8: state = 923;
pub const S_EMBLEM7: state = 922;
pub const S_EMBLEM6: state = 921;
pub const S_EMBLEM5: state = 920;
pub const S_EMBLEM4: state = 919;
pub const S_EMBLEM3: state = 918;
pub const S_EMBLEM2: state = 917;
pub const S_EMBLEM1: state = 916;
pub const S_BLUEFLAG: state = 915;
pub const S_REDFLAG: state = 914;
pub const S_TOKEN: state = 913;
pub const S_TEAMRING: state = 912;
pub const S_GRAVWELLRED: state = 911;
pub const S_GRAVWELLGREEN: state = 910;
pub const S_NIGHTSSTARXMAS: state = 909;
pub const S_NIGHTSSTAR: state = 908;
pub const S_NIGHTSCHIPBONUS: state = 907;
pub const S_NIGHTSCHIP: state = 906;
pub const S_BOMBSPHERE4: state = 905;
pub const S_BOMBSPHERE3: state = 904;
pub const S_BOMBSPHERE2: state = 903;
pub const S_BOMBSPHERE1: state = 902;
pub const S_BLUESPHERESPARK: state = 901;
pub const S_BLUESPHEREBONUS: state = 900;
pub const S_BLUESPHERE: state = 899;
pub const S_RING: state = 898;
pub const S_MSSHIELD_F2: state = 897;
pub const S_MSSHIELD_F1: state = 896;
pub const S_METALSONIC_FLEE2: state = 895;
pub const S_METALSONIC_FLEE1: state = 894;
pub const S_METALSONIC_DEATH4: state = 893;
pub const S_METALSONIC_DEATH3: state = 892;
pub const S_METALSONIC_DEATH2: state = 891;
pub const S_METALSONIC_DEATH1: state = 890;
pub const S_METALSONIC_PAIN: state = 889;
pub const S_METALSONIC_SHOOT: state = 888;
pub const S_METALSONIC_BADBOUNCE: state = 887;
pub const S_METALSONIC_BOUNCE: state = 886;
pub const S_METALSONIC_DASH: state = 885;
pub const S_METALSONIC_GATHER: state = 884;
pub const S_METALSONIC_RAISE: state = 883;
pub const S_METALSONIC_STUN: state = 882;
pub const S_METALSONIC_VECTOR: state = 881;
pub const S_METALSONIC_FLOAT: state = 880;
pub const S_METALSONIC_RACE: state = 879;
pub const S_CYBRAKDEMONVILEEXPLOSION3: state = 878;
pub const S_CYBRAKDEMONVILEEXPLOSION2: state = 877;
pub const S_CYBRAKDEMONVILEEXPLOSION1: state = 876;
pub const S_CYBRAKDEMONNAPALMFLAME_DIE: state = 875;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY6: state = 874;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY5: state = 873;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY4: state = 872;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY3: state = 871;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY2: state = 870;
pub const S_CYBRAKDEMONNAPALMFLAME_FLY1: state = 869;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL_DIE5: state = 868;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL_DIE4: state = 867;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL_DIE3: state = 866;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL_DIE2: state = 865;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL_DIE1: state = 864;
pub const S_CYBRAKDEMONNAPALMBOMBSMALL: state = 863;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_DIE4: state = 862;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_DIE3: state = 861;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_DIE2: state = 860;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_DIE1: state = 859;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_FLY4: state = 858;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_FLY3: state = 857;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_FLY2: state = 856;
pub const S_CYBRAKDEMONNAPALMBOMBLARGE_FLY1: state = 855;
pub const S_CYBRAKDEMONTARGETDOT: state = 854;
pub const S_CYBRAKDEMONTARGETRETICULE14: state = 853;
pub const S_CYBRAKDEMONTARGETRETICULE13: state = 852;
pub const S_CYBRAKDEMONTARGETRETICULE12: state = 851;
pub const S_CYBRAKDEMONTARGETRETICULE11: state = 850;
pub const S_CYBRAKDEMONTARGETRETICULE10: state = 849;
pub const S_CYBRAKDEMONTARGETRETICULE9: state = 848;
pub const S_CYBRAKDEMONTARGETRETICULE8: state = 847;
pub const S_CYBRAKDEMONTARGETRETICULE7: state = 846;
pub const S_CYBRAKDEMONTARGETRETICULE6: state = 845;
pub const S_CYBRAKDEMONTARGETRETICULE5: state = 844;
pub const S_CYBRAKDEMONTARGETRETICULE4: state = 843;
pub const S_CYBRAKDEMONTARGETRETICULE3: state = 842;
pub const S_CYBRAKDEMONTARGETRETICULE2: state = 841;
pub const S_CYBRAKDEMONTARGETRETICULE1: state = 840;
pub const S_CYBRAKDEMONELECTRICBARRIER_REVIVE3: state = 839;
pub const S_CYBRAKDEMONELECTRICBARRIER_REVIVE2: state = 838;
pub const S_CYBRAKDEMONELECTRICBARRIER_REVIVE1: state = 837;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMLOOP: state = 836;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMFAIL: state = 835;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM12: state = 834;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM11: state = 833;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM10: state = 832;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM9: state = 831;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM8: state = 830;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM7: state = 829;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM6: state = 828;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM5: state = 827;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM4: state = 826;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM3: state = 825;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM2: state = 824;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOM1: state = 823;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMCHOOSE: state = 822;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMSUCCESS: state = 821;
pub const S_CYBRAKDEMONELECTRICBARRIER_SPARK_RANDOMCHECK: state = 820;
pub const S_CYBRAKDEMONELECTRICBARRIER_DIE3: state = 819;
pub const S_CYBRAKDEMONELECTRICBARRIER_DIE2: state = 818;
pub const S_CYBRAKDEMONELECTRICBARRIER_DIE1: state = 817;
pub const S_CYBRAKDEMONELECTRICBARRIER24: state = 816;
pub const S_CYBRAKDEMONELECTRICBARRIER23: state = 815;
pub const S_CYBRAKDEMONELECTRICBARRIER22: state = 814;
pub const S_CYBRAKDEMONELECTRICBARRIER21: state = 813;
pub const S_CYBRAKDEMONELECTRICBARRIER20: state = 812;
pub const S_CYBRAKDEMONELECTRICBARRIER19: state = 811;
pub const S_CYBRAKDEMONELECTRICBARRIER18: state = 810;
pub const S_CYBRAKDEMONELECTRICBARRIER17: state = 809;
pub const S_CYBRAKDEMONELECTRICBARRIER16: state = 808;
pub const S_CYBRAKDEMONELECTRICBARRIER15: state = 807;
pub const S_CYBRAKDEMONELECTRICBARRIER14: state = 806;
pub const S_CYBRAKDEMONELECTRICBARRIER13: state = 805;
pub const S_CYBRAKDEMONELECTRICBARRIER12: state = 804;
pub const S_CYBRAKDEMONELECTRICBARRIER11: state = 803;
pub const S_CYBRAKDEMONELECTRICBARRIER10: state = 802;
pub const S_CYBRAKDEMONELECTRICBARRIER9: state = 801;
pub const S_CYBRAKDEMONELECTRICBARRIER8: state = 800;
pub const S_CYBRAKDEMONELECTRICBARRIER7: state = 799;
pub const S_CYBRAKDEMONELECTRICBARRIER6: state = 798;
pub const S_CYBRAKDEMONELECTRICBARRIER5: state = 797;
pub const S_CYBRAKDEMONELECTRICBARRIER4: state = 796;
pub const S_CYBRAKDEMONELECTRICBARRIER3: state = 795;
pub const S_CYBRAKDEMONELECTRICBARRIER2: state = 794;
pub const S_CYBRAKDEMONELECTRICBARRIER1: state = 793;
pub const S_CYBRAKDEMONELECTRICBARRIER_PLAYSOUND: state = 792;
pub const S_CYBRAKDEMONELECTRICBARRIER_INIT2: state = 791;
pub const S_CYBRAKDEMONELECTRICBARRIER_INIT1: state = 790;
pub const S_CYBRAKDEMONFLAMEREST: state = 789;
pub const S_CYBRAKDEMONFLAMESHOT_DIE: state = 788;
pub const S_CYBRAKDEMONFLAMESHOT_FLY3: state = 787;
pub const S_CYBRAKDEMONFLAMESHOT_FLY2: state = 786;
pub const S_CYBRAKDEMONFLAMESHOT_FLY1: state = 785;
pub const S_CYBRAKDEMONMISSILE_EXPLODE3: state = 784;
pub const S_CYBRAKDEMONMISSILE_EXPLODE2: state = 783;
pub const S_CYBRAKDEMONMISSILE_EXPLODE1: state = 782;
pub const S_CYBRAKDEMONMISSILE: state = 781;
pub const S_CYBRAKDEMON_INVINCIBLERIZE: state = 780;
pub const S_CYBRAKDEMON_DEINVINCIBLERIZE: state = 779;
pub const S_CYBRAKDEMON_DIE8: state = 778;
pub const S_CYBRAKDEMON_DIE7: state = 777;
pub const S_CYBRAKDEMON_DIE6: state = 776;
pub const S_CYBRAKDEMON_DIE5: state = 775;
pub const S_CYBRAKDEMON_DIE4: state = 774;
pub const S_CYBRAKDEMON_DIE3: state = 773;
pub const S_CYBRAKDEMON_DIE2: state = 772;
pub const S_CYBRAKDEMON_DIE1: state = 771;
pub const S_CYBRAKDEMON_PAIN3: state = 770;
pub const S_CYBRAKDEMON_PAIN2: state = 769;
pub const S_CYBRAKDEMON_PAIN1: state = 768;
pub const S_CYBRAKDEMON_FINISH_ATTACK2: state = 767;
pub const S_CYBRAKDEMON_FINISH_ATTACK1: state = 766;
pub const S_CYBRAKDEMON_NAPALM_ATTACK3: state = 765;
pub const S_CYBRAKDEMON_NAPALM_ATTACK2: state = 764;
pub const S_CYBRAKDEMON_NAPALM_ATTACK1: state = 763;
pub const S_CYBRAKDEMON_VILE_ATTACK6: state = 762;
pub const S_CYBRAKDEMON_VILE_ATTACK5: state = 761;
pub const S_CYBRAKDEMON_VILE_ATTACK4: state = 760;
pub const S_CYBRAKDEMON_VILE_ATTACK3: state = 759;
pub const S_CYBRAKDEMON_VILE_ATTACK2: state = 758;
pub const S_CYBRAKDEMON_VILE_ATTACK1: state = 757;
pub const S_CYBRAKDEMON_CHOOSE_ATTACK2: state = 756;
pub const S_CYBRAKDEMON_FLAME_ATTACK4: state = 755;
pub const S_CYBRAKDEMON_FLAME_ATTACK3: state = 754;
pub const S_CYBRAKDEMON_FLAME_ATTACK2: state = 753;
pub const S_CYBRAKDEMON_FLAME_ATTACK1: state = 752;
pub const S_CYBRAKDEMON_MISSILE_ATTACK6: state = 751;
pub const S_CYBRAKDEMON_MISSILE_ATTACK5: state = 750;
pub const S_CYBRAKDEMON_MISSILE_ATTACK4: state = 749;
pub const S_CYBRAKDEMON_MISSILE_ATTACK3: state = 748;
pub const S_CYBRAKDEMON_MISSILE_ATTACK2: state = 747;
pub const S_CYBRAKDEMON_MISSILE_ATTACK1: state = 746;
pub const S_CYBRAKDEMON_CHOOSE_ATTACK1: state = 745;
pub const S_CYBRAKDEMON_WALK6: state = 744;
pub const S_CYBRAKDEMON_WALK5: state = 743;
pub const S_CYBRAKDEMON_WALK4: state = 742;
pub const S_CYBRAKDEMON_WALK3: state = 741;
pub const S_CYBRAKDEMON_WALK2: state = 740;
pub const S_CYBRAKDEMON_WALK1: state = 739;
pub const S_CYBRAKDEMON_IDLE: state = 738;
pub const S_BLACKEGG_MISSILE: state = 737;
pub const S_BLACKEGG_GOOP7: state = 736;
pub const S_BLACKEGG_GOOP6: state = 735;
pub const S_BLACKEGG_GOOP5: state = 734;
pub const S_BLACKEGG_GOOP4: state = 733;
pub const S_BLACKEGG_GOOP3: state = 732;
pub const S_BLACKEGG_GOOP2: state = 731;
pub const S_BLACKEGG_GOOP1: state = 730;
pub const S_BLACKEGG_HELPER: state = 729;
pub const S_BLACKEGG_DESTROYPLAT3: state = 728;
pub const S_BLACKEGG_DESTROYPLAT2: state = 727;
pub const S_BLACKEGG_DESTROYPLAT1: state = 726;
pub const S_BLACKEGG_JUMP2: state = 725;
pub const S_BLACKEGG_JUMP1: state = 724;
pub const S_BLACKEGG_GOOP: state = 723;
pub const S_BLACKEGG_MISSILE3: state = 722;
pub const S_BLACKEGG_MISSILE2: state = 721;
pub const S_BLACKEGG_MISSILE1: state = 720;
pub const S_BLACKEGG_DIE5: state = 719;
pub const S_BLACKEGG_DIE4: state = 718;
pub const S_BLACKEGG_DIE3: state = 717;
pub const S_BLACKEGG_DIE2: state = 716;
pub const S_BLACKEGG_DIE1: state = 715;
pub const S_BLACKEGG_HITFACE4: state = 714;
pub const S_BLACKEGG_HITFACE3: state = 713;
pub const S_BLACKEGG_HITFACE2: state = 712;
pub const S_BLACKEGG_HITFACE1: state = 711;
pub const S_BLACKEGG_PAIN35: state = 710;
pub const S_BLACKEGG_PAIN34: state = 709;
pub const S_BLACKEGG_PAIN33: state = 708;
pub const S_BLACKEGG_PAIN32: state = 707;
pub const S_BLACKEGG_PAIN31: state = 706;
pub const S_BLACKEGG_PAIN30: state = 705;
pub const S_BLACKEGG_PAIN29: state = 704;
pub const S_BLACKEGG_PAIN28: state = 703;
pub const S_BLACKEGG_PAIN27: state = 702;
pub const S_BLACKEGG_PAIN26: state = 701;
pub const S_BLACKEGG_PAIN25: state = 700;
pub const S_BLACKEGG_PAIN24: state = 699;
pub const S_BLACKEGG_PAIN23: state = 698;
pub const S_BLACKEGG_PAIN22: state = 697;
pub const S_BLACKEGG_PAIN21: state = 696;
pub const S_BLACKEGG_PAIN20: state = 695;
pub const S_BLACKEGG_PAIN19: state = 694;
pub const S_BLACKEGG_PAIN18: state = 693;
pub const S_BLACKEGG_PAIN17: state = 692;
pub const S_BLACKEGG_PAIN16: state = 691;
pub const S_BLACKEGG_PAIN15: state = 690;
pub const S_BLACKEGG_PAIN14: state = 689;
pub const S_BLACKEGG_PAIN13: state = 688;
pub const S_BLACKEGG_PAIN12: state = 687;
pub const S_BLACKEGG_PAIN11: state = 686;
pub const S_BLACKEGG_PAIN10: state = 685;
pub const S_BLACKEGG_PAIN9: state = 684;
pub const S_BLACKEGG_PAIN8: state = 683;
pub const S_BLACKEGG_PAIN7: state = 682;
pub const S_BLACKEGG_PAIN6: state = 681;
pub const S_BLACKEGG_PAIN5: state = 680;
pub const S_BLACKEGG_PAIN4: state = 679;
pub const S_BLACKEGG_PAIN3: state = 678;
pub const S_BLACKEGG_PAIN2: state = 677;
pub const S_BLACKEGG_PAIN1: state = 676;
pub const S_BLACKEGG_SHOOT2: state = 675;
pub const S_BLACKEGG_SHOOT1: state = 674;
pub const S_BLACKEGG_WALK6: state = 673;
pub const S_BLACKEGG_WALK5: state = 672;
pub const S_BLACKEGG_WALK4: state = 671;
pub const S_BLACKEGG_WALK3: state = 670;
pub const S_BLACKEGG_WALK2: state = 669;
pub const S_BLACKEGG_WALK1: state = 668;
pub const S_BLACKEGG_STND2: state = 667;
pub const S_BLACKEGG_STND: state = 666;
pub const S_FSGND: state = 665;
pub const S_FSGNC: state = 664;
pub const S_FSGNB: state = 663;
pub const S_FSGNA: state = 662;
pub const S_TNTDUST_8: state = 661;
pub const S_TNTDUST_7: state = 660;
pub const S_TNTDUST_6: state = 659;
pub const S_TNTDUST_5: state = 658;
pub const S_TNTDUST_4: state = 657;
pub const S_TNTDUST_3: state = 656;
pub const S_TNTDUST_2: state = 655;
pub const S_TNTDUST_1: state = 654;
pub const S_FBOMB_EXPL6: state = 653;
pub const S_FBOMB_EXPL5: state = 652;
pub const S_FBOMB_EXPL4: state = 651;
pub const S_FBOMB_EXPL3: state = 650;
pub const S_FBOMB_EXPL2: state = 649;
pub const S_FBOMB_EXPL1: state = 648;
pub const S_FBOMB2: state = 647;
pub const S_FBOMB1: state = 646;
pub const S_PROJECTORLIGHT5: state = 645;
pub const S_PROJECTORLIGHT4: state = 644;
pub const S_PROJECTORLIGHT3: state = 643;
pub const S_PROJECTORLIGHT2: state = 642;
pub const S_PROJECTORLIGHT1: state = 641;
pub const S_VWREB: state = 640;
pub const S_VWREF: state = 639;
pub const S_ALART2: state = 638;
pub const S_ALART1: state = 637;
pub const S_BROKENROBOTF: state = 636;
pub const S_BROKENROBOTE: state = 635;
pub const S_BROKENROBOTD: state = 634;
pub const S_BROKENROBOTC: state = 633;
pub const S_BROKENROBOTB: state = 632;
pub const S_BROKENROBOTA: state = 631;
pub const S_BROKENROBOTRANDOM: state = 630;
pub const S_FANG_KO: state = 629;
pub const S_FANG_FLEEBOUNCE2: state = 628;
pub const S_FANG_FLEEBOUNCE1: state = 627;
pub const S_FANG_FLEEPATHING2: state = 626;
pub const S_FANG_FLEEPATHING1: state = 625;
pub const S_FANG_DIE8: state = 624;
pub const S_FANG_DIE7: state = 623;
pub const S_FANG_DIE6: state = 622;
pub const S_FANG_DIE5: state = 621;
pub const S_FANG_DIE4: state = 620;
pub const S_FANG_DIE3: state = 619;
pub const S_FANG_DIE2: state = 618;
pub const S_FANG_DIE1: state = 617;
pub const S_FANG_PINCHLOBSHOT4: state = 616;
pub const S_FANG_PINCHLOBSHOT3: state = 615;
pub const S_FANG_PINCHLOBSHOT2: state = 614;
pub const S_FANG_PINCHLOBSHOT1: state = 613;
pub const S_FANG_PINCHLOBSHOT0: state = 612;
pub const S_FANG_PINCHSKID2: state = 611;
pub const S_FANG_PINCHSKID1: state = 610;
pub const S_FANG_PINCHFALL2: state = 609;
pub const S_FANG_PINCHFALL1: state = 608;
pub const S_FANG_PINCHFALL0: state = 607;
pub const S_FANG_PINCHBOUNCE4: state = 606;
pub const S_FANG_PINCHBOUNCE3: state = 605;
pub const S_FANG_PINCHBOUNCE2: state = 604;
pub const S_FANG_PINCHBOUNCE1: state = 603;
pub const S_FANG_PINCHBOUNCE0: state = 602;
pub const S_FANG_PINCHPATHING: state = 601;
pub const S_FANG_PINCHPATHINGSTART2: state = 600;
pub const S_FANG_PINCHPATHINGSTART1: state = 599;
pub const S_FANG_WALLHIT: state = 598;
pub const S_FANG_WAIT2: state = 597;
pub const S_FANG_WAIT1: state = 596;
pub const S_FANG_LOBSHOT2: state = 595;
pub const S_FANG_LOBSHOT1: state = 594;
pub const S_FANG_LOBSHOT0: state = 593;
pub const S_FANG_FIREREPEAT: state = 592;
pub const S_FANG_FIRE4: state = 591;
pub const S_FANG_FIRE3: state = 590;
pub const S_FANG_FIRE2: state = 589;
pub const S_FANG_FIRE1: state = 588;
pub const S_FANG_FIRESTART2: state = 587;
pub const S_FANG_FIRESTART1: state = 586;
pub const S_FANG_CHOOSEATTACK: state = 585;
pub const S_FANG_SKID3: state = 584;
pub const S_FANG_SKID2: state = 583;
pub const S_FANG_SKID1: state = 582;
pub const S_FANG_PATHINGCONT3: state = 581;
pub const S_FANG_PATHINGCONT2: state = 580;
pub const S_FANG_PATHINGCONT1: state = 579;
pub const S_FANG_CHECKPATH2: state = 578;
pub const S_FANG_CHECKPATH1: state = 577;
pub const S_FANG_FALL2: state = 576;
pub const S_FANG_FALL1: state = 575;
pub const S_FANG_BOUNCE4: state = 574;
pub const S_FANG_BOUNCE3: state = 573;
pub const S_FANG_BOUNCE2: state = 572;
pub const S_FANG_BOUNCE1: state = 571;
pub const S_FANG_PATHING: state = 570;
pub const S_FANG_PATHINGSTART2: state = 569;
pub const S_FANG_PATHINGSTART1: state = 568;
pub const S_FANG_PAIN2: state = 567;
pub const S_FANG_PAIN1: state = 566;
pub const S_FANG_IDLE8: state = 565;
pub const S_FANG_IDLE7: state = 564;
pub const S_FANG_IDLE6: state = 563;
pub const S_FANG_IDLE5: state = 562;
pub const S_FANG_IDLE4: state = 561;
pub const S_FANG_IDLE3: state = 560;
pub const S_FANG_IDLE2: state = 559;
pub const S_FANG_IDLE1: state = 558;
pub const S_FANG_IDLE0: state = 557;
pub const S_FANG_CLONE4: state = 556;
pub const S_FANG_CLONE3: state = 555;
pub const S_FANG_CLONE2: state = 554;
pub const S_FANG_CLONE1: state = 553;
pub const S_FANG_INTRO12: state = 552;
pub const S_FANG_INTRO11: state = 551;
pub const S_FANG_INTRO10: state = 550;
pub const S_FANG_INTRO9: state = 549;
pub const S_FANG_INTRO8: state = 548;
pub const S_FANG_INTRO7: state = 547;
pub const S_FANG_INTRO6: state = 546;
pub const S_FANG_INTRO5: state = 545;
pub const S_FANG_INTRO4: state = 544;
pub const S_FANG_INTRO3: state = 543;
pub const S_FANG_INTRO2: state = 542;
pub const S_FANG_INTRO1: state = 541;
pub const S_FANG_INTRO0: state = 540;
pub const S_FANG_SETUP: state = 539;
pub const S_EGGROBOJET: state = 538;
pub const S_EGGROBO1_PISSED: state = 537;
pub const S_EGGROBO1_BSLAP2: state = 536;
pub const S_EGGROBO1_BSLAP1: state = 535;
pub const S_EGGROBO1_STND: state = 534;
pub const S_JETFLAME: state = 533;
pub const S_EGGMOBILE4_MACE_DIE3: state = 532;
pub const S_EGGMOBILE4_MACE_DIE2: state = 531;
pub const S_EGGMOBILE4_MACE_DIE1: state = 530;
pub const S_EGGMOBILE4_MACE: state = 529;
pub const S_EGGMOBILE4_FLEE2: state = 528;
pub const S_EGGMOBILE4_FLEE1: state = 527;
pub const S_EGGMOBILE4_DIE4: state = 526;
pub const S_EGGMOBILE4_DIE3: state = 525;
pub const S_EGGMOBILE4_DIE2: state = 524;
pub const S_EGGMOBILE4_DIE1: state = 523;
pub const S_EGGMOBILE4_PAIN2: state = 522;
pub const S_EGGMOBILE4_PAIN1: state = 521;
pub const S_EGGMOBILE4_RAISE2: state = 520;
pub const S_EGGMOBILE4_RAISE1: state = 519;
pub const S_EGGMOBILE4_RATK6: state = 518;
pub const S_EGGMOBILE4_RATK5: state = 517;
pub const S_EGGMOBILE4_RATK4: state = 516;
pub const S_EGGMOBILE4_RATK3: state = 515;
pub const S_EGGMOBILE4_RATK2: state = 514;
pub const S_EGGMOBILE4_RATK1: state = 513;
pub const S_EGGMOBILE4_LATK6: state = 512;
pub const S_EGGMOBILE4_LATK5: state = 511;
pub const S_EGGMOBILE4_LATK4: state = 510;
pub const S_EGGMOBILE4_LATK3: state = 509;
pub const S_EGGMOBILE4_LATK2: state = 508;
pub const S_EGGMOBILE4_LATK1: state = 507;
pub const S_EGGMOBILE4_STND: state = 506;
pub const S_SHOCKWAVE2: state = 505;
pub const S_SHOCKWAVE1: state = 504;
pub const S_BOSSSEBH2: state = 503;
pub const S_BOSSSEBH1: state = 502;
pub const S_FAKEMOBILE_DIE2: state = 501;
pub const S_FAKEMOBILE_DIE1: state = 500;
pub const S_FAKEMOBILE_ATK3D: state = 499;
pub const S_FAKEMOBILE_ATK3C: state = 498;
pub const S_FAKEMOBILE_ATK3B: state = 497;
pub const S_FAKEMOBILE_ATK3A: state = 496;
pub const S_FAKEMOBILE_ATK2: state = 495;
pub const S_FAKEMOBILE_ATK1: state = 494;
pub const S_FAKEMOBILE: state = 493;
pub const S_FAKEMOBILE_INIT: state = 492;
pub const S_EGGMOBILE3_FLEE2: state = 491;
pub const S_EGGMOBILE3_FLEE1: state = 490;
pub const S_EGGMOBILE3_DIE4: state = 489;
pub const S_EGGMOBILE3_DIE3: state = 488;
pub const S_EGGMOBILE3_DIE2: state = 487;
pub const S_EGGMOBILE3_DIE1: state = 486;
pub const S_EGGMOBILE3_PAIN2: state = 485;
pub const S_EGGMOBILE3_PAIN: state = 484;
pub const S_EGGMOBILE3_ROFL: state = 483;
pub const S_EGGMOBILE3_ATK5: state = 482;
pub const S_EGGMOBILE3_ATK4: state = 481;
pub const S_EGGMOBILE3_ATK3D: state = 480;
pub const S_EGGMOBILE3_ATK3C: state = 479;
pub const S_EGGMOBILE3_ATK3B: state = 478;
pub const S_EGGMOBILE3_ATK3A: state = 477;
pub const S_EGGMOBILE3_ATK2: state = 476;
pub const S_EGGMOBILE3_ATK1: state = 475;
pub const S_EGGMOBILE3_SHOCK: state = 474;
pub const S_EGGMOBILE3_STND: state = 473;
pub const S_GOOPTRAIL: state = 472;
pub const S_GOOP3: state = 471;
pub const S_GOOP2: state = 470;
pub const S_GOOP1: state = 469;
pub const S_BOSSSPIGOT: state = 468;
pub const S_BOSSTANK2: state = 467;
pub const S_BOSSTANK1: state = 466;
pub const S_EGGMOBILE2_FLEE2: state = 465;
pub const S_EGGMOBILE2_FLEE1: state = 464;
pub const S_EGGMOBILE2_DIE4: state = 463;
pub const S_EGGMOBILE2_DIE3: state = 462;
pub const S_EGGMOBILE2_DIE2: state = 461;
pub const S_EGGMOBILE2_DIE1: state = 460;
pub const S_EGGMOBILE2_PAIN2: state = 459;
pub const S_EGGMOBILE2_PAIN: state = 458;
pub const S_EGGMOBILE2_POGO7: state = 457;
pub const S_EGGMOBILE2_POGO6: state = 456;
pub const S_EGGMOBILE2_POGO5: state = 455;
pub const S_EGGMOBILE2_POGO4: state = 454;
pub const S_EGGMOBILE2_POGO3: state = 453;
pub const S_EGGMOBILE2_POGO2: state = 452;
pub const S_EGGMOBILE2_POGO1: state = 451;
pub const S_EGGMOBILE2_STND: state = 450;
pub const S_BOSSEGLZ2: state = 449;
pub const S_BOSSEGLZ1: state = 448;
pub const S_EGGMOBILE_TARGET: state = 447;
pub const S_EGGMOBILE_BALL: state = 446;
pub const S_EGGMOBILE_FLEE2: state = 445;
pub const S_EGGMOBILE_FLEE1: state = 444;
pub const S_EGGMOBILE_DIE4: state = 443;
pub const S_EGGMOBILE_DIE3: state = 442;
pub const S_EGGMOBILE_DIE2: state = 441;
pub const S_EGGMOBILE_DIE1: state = 440;
pub const S_EGGMOBILE_PAIN2: state = 439;
pub const S_EGGMOBILE_PAIN: state = 438;
pub const S_EGGMOBILE_PANIC15: state = 437;
pub const S_EGGMOBILE_PANIC14: state = 436;
pub const S_EGGMOBILE_PANIC13: state = 435;
pub const S_EGGMOBILE_PANIC12: state = 434;
pub const S_EGGMOBILE_PANIC11: state = 433;
pub const S_EGGMOBILE_PANIC10: state = 432;
pub const S_EGGMOBILE_PANIC9: state = 431;
pub const S_EGGMOBILE_PANIC8: state = 430;
pub const S_EGGMOBILE_PANIC7: state = 429;
pub const S_EGGMOBILE_PANIC6: state = 428;
pub const S_EGGMOBILE_PANIC5: state = 427;
pub const S_EGGMOBILE_PANIC4: state = 426;
pub const S_EGGMOBILE_PANIC3: state = 425;
pub const S_EGGMOBILE_PANIC2: state = 424;
pub const S_EGGMOBILE_PANIC1: state = 423;
pub const S_EGGMOBILE_RATK9: state = 422;
pub const S_EGGMOBILE_RATK8: state = 421;
pub const S_EGGMOBILE_RATK7: state = 420;
pub const S_EGGMOBILE_RATK6: state = 419;
pub const S_EGGMOBILE_RATK5: state = 418;
pub const S_EGGMOBILE_RATK4: state = 417;
pub const S_EGGMOBILE_RATK3: state = 416;
pub const S_EGGMOBILE_RATK2: state = 415;
pub const S_EGGMOBILE_RATK1: state = 414;
pub const S_EGGMOBILE_LATK9: state = 413;
pub const S_EGGMOBILE_LATK8: state = 412;
pub const S_EGGMOBILE_LATK7: state = 411;
pub const S_EGGMOBILE_LATK6: state = 410;
pub const S_EGGMOBILE_LATK5: state = 409;
pub const S_EGGMOBILE_LATK4: state = 408;
pub const S_EGGMOBILE_LATK3: state = 407;
pub const S_EGGMOBILE_LATK2: state = 406;
pub const S_EGGMOBILE_LATK1: state = 405;
pub const S_EGGMOBILE_ROFL: state = 404;
pub const S_EGGMOBILE_STND: state = 403;
pub const S_JETFUME1: state = 402;
pub const S_SONIC3KBOSSEXPLOSION6: state = 401;
pub const S_SONIC3KBOSSEXPLOSION5: state = 400;
pub const S_SONIC3KBOSSEXPLOSION4: state = 399;
pub const S_SONIC3KBOSSEXPLOSION3: state = 398;
pub const S_SONIC3KBOSSEXPLOSION2: state = 397;
pub const S_SONIC3KBOSSEXPLOSION1: state = 396;
pub const S_BOSSEXPLODE: state = 395;
pub const S_DRAGONMINE_FASTLOOP: state = 394;
pub const S_DRAGONMINE_FASTFLASH2: state = 393;
pub const S_DRAGONMINE_FASTFLASH1: state = 392;
pub const S_DRAGONMINE_SLOWLOOP: state = 391;
pub const S_DRAGONMINE_SLOWFLASH2: state = 390;
pub const S_DRAGONMINE_SLOWFLASH1: state = 389;
pub const S_DRAGONMINE_LAND2: state = 388;
pub const S_DRAGONMINE_LAND1: state = 387;
pub const S_DRAGONMINE: state = 386;
pub const S_DRAGONTAIL_RELOAD: state = 385;
pub const S_DRAGONTAIL_EMPTYLOOP: state = 384;
pub const S_DRAGONTAIL_EMPTY: state = 383;
pub const S_DRAGONTAIL_LOADED: state = 382;
pub const S_DRAGONWING4: state = 381;
pub const S_DRAGONWING3: state = 380;
pub const S_DRAGONWING2: state = 379;
pub const S_DRAGONWING1: state = 378;
pub const S_DRAGONBOMBER: state = 377;
pub const S_PTERABYTE_SWOOPUP: state = 376;
pub const S_PTERABYTE_SWOOPDOWN: state = 375;
pub const S_PTERABYTE_FLY4: state = 374;
pub const S_PTERABYTE_FLY3: state = 373;
pub const S_PTERABYTE_FLY2: state = 372;
pub const S_PTERABYTE_FLY1: state = 371;
pub const S_PTERABYTEWAYPOINT: state = 370;
pub const S_PTERABYTESPAWNER: state = 369;
pub const S_PYREFIRE2: state = 368;
pub const S_PYREFIRE1: state = 367;
pub const S_PYREFLY_BURN: state = 366;
pub const S_PYREFLY_FLY: state = 365;
pub const S_CANARIVOREGAS_8: state = 364;
pub const S_CANARIVOREGAS_7: state = 363;
pub const S_CANARIVOREGAS_6: state = 362;
pub const S_CANARIVOREGAS_5: state = 361;
pub const S_CANARIVOREGAS_4: state = 360;
pub const S_CANARIVOREGAS_3: state = 359;
pub const S_CANARIVOREGAS_2: state = 358;
pub const S_CANARIVOREGAS_1: state = 357;
pub const S_CANARIVORE_CLOSE2: state = 356;
pub const S_CANARIVORE_CLOSE1: state = 355;
pub const S_CANARIVORE_GASREPEAT: state = 354;
pub const S_CANARIVORE_GAS5: state = 353;
pub const S_CANARIVORE_GAS4: state = 352;
pub const S_CANARIVORE_GAS3: state = 351;
pub const S_CANARIVORE_GAS2: state = 350;
pub const S_CANARIVORE_GAS1: state = 349;
pub const S_CANARIVORE_AWAKEN3: state = 348;
pub const S_CANARIVORE_AWAKEN2: state = 347;
pub const S_CANARIVORE_AWAKEN1: state = 346;
pub const S_CANARIVORE_LOOK: state = 345;
pub const S_UNIDUS_BALL: state = 344;
pub const S_UNIDUS_RUN: state = 343;
pub const S_UNIDUS_STND: state = 342;
pub const S_YSHELL_SPRING4: state = 341;
pub const S_YSHELL_SPRING3: state = 340;
pub const S_YSHELL_SPRING2: state = 339;
pub const S_YSHELL_SPRING1: state = 338;
pub const S_YSHELL_RUN4: state = 337;
pub const S_YSHELL_RUN3: state = 336;
pub const S_YSHELL_RUN2: state = 335;
pub const S_YSHELL_RUN1: state = 334;
pub const S_YSHELL_STND: state = 333;
pub const S_SSHELL_SPRING4: state = 332;
pub const S_SSHELL_SPRING3: state = 331;
pub const S_SSHELL_SPRING2: state = 330;
pub const S_SSHELL_SPRING1: state = 329;
pub const S_SSHELL_RUN4: state = 328;
pub const S_SSHELL_RUN3: state = 327;
pub const S_SSHELL_RUN2: state = 326;
pub const S_SSHELL_RUN1: state = 325;
pub const S_SSHELL_STND: state = 324;
pub const S_MINUSDIRT7: state = 323;
pub const S_MINUSDIRT6: state = 322;
pub const S_MINUSDIRT5: state = 321;
pub const S_MINUSDIRT4: state = 320;
pub const S_MINUSDIRT3: state = 319;
pub const S_MINUSDIRT2: state = 318;
pub const S_MINUSDIRT1: state = 317;
pub const S_MINUS_AERIAL4: state = 316;
pub const S_MINUS_AERIAL3: state = 315;
pub const S_MINUS_AERIAL2: state = 314;
pub const S_MINUS_AERIAL1: state = 313;
pub const S_MINUS_POPUP: state = 312;
pub const S_MINUS_BURST5: state = 311;
pub const S_MINUS_BURST4: state = 310;
pub const S_MINUS_BURST3: state = 309;
pub const S_MINUS_BURST2: state = 308;
pub const S_MINUS_BURST1: state = 307;
pub const S_MINUS_BURST0: state = 306;
pub const S_MINUS_DIGGING4: state = 305;
pub const S_MINUS_DIGGING3: state = 304;
pub const S_MINUS_DIGGING2: state = 303;
pub const S_MINUS_DIGGING1: state = 302;
pub const S_MINUS_STND: state = 301;
pub const S_MINUS_INIT: state = 300;
pub const S_SNAPPER_HEAD: state = 299;
pub const S_SNAPPER_LEGRAISE: state = 298;
pub const S_SNAPPER_LEG: state = 297;
pub const S_SNAPPER_XPLD: state = 296;
pub const S_GSNAPPER4: state = 295;
pub const S_GSNAPPER3: state = 294;
pub const S_GSNAPPER2: state = 293;
pub const S_GSNAPPER1: state = 292;
pub const S_GSNAPPER_STND: state = 291;
pub const S_SNAPPER_SPAWN2: state = 290;
pub const S_SNAPPER_SPAWN: state = 289;
pub const S_EGGSHIELDBREAK: state = 288;
pub const S_EGGSHIELD: state = 287;
pub const S_EGGGUARD_RUN4: state = 286;
pub const S_EGGGUARD_RUN3: state = 285;
pub const S_EGGGUARD_RUN2: state = 284;
pub const S_EGGGUARD_RUN1: state = 283;
pub const S_EGGGUARD_MAD3: state = 282;
pub const S_EGGGUARD_MAD2: state = 281;
pub const S_EGGGUARD_MAD1: state = 280;
pub const S_EGGGUARD_WALK4: state = 279;
pub const S_EGGGUARD_WALK3: state = 278;
pub const S_EGGGUARD_WALK2: state = 277;
pub const S_EGGGUARD_WALK1: state = 276;
pub const S_EGGGUARD_STND: state = 275;
pub const S_FACESTABBERSPEAR: state = 274;
pub const S_FACESTABBER_DIE3: state = 273;
pub const S_FACESTABBER_DIE2: state = 272;
pub const S_FACESTABBER_DIE1: state = 271;
pub const S_FACESTABBER_PAIN: state = 270;
pub const S_FACESTABBER_CHARGE4: state = 269;
pub const S_FACESTABBER_CHARGE3: state = 268;
pub const S_FACESTABBER_CHARGE2: state = 267;
pub const S_FACESTABBER_CHARGE1: state = 266;
pub const S_FACESTABBER_STND6: state = 265;
pub const S_FACESTABBER_STND5: state = 264;
pub const S_FACESTABBER_STND4: state = 263;
pub const S_FACESTABBER_STND3: state = 262;
pub const S_FACESTABBER_STND2: state = 261;
pub const S_FACESTABBER_STND1: state = 260;
pub const S_ROBOHOOD_JUMP3: state = 259;
pub const S_ROBOHOOD_JUMP2: state = 258;
pub const S_ROBOHOOD_JUMP1: state = 257;
pub const S_ROBOHOOD_FIRE2: state = 256;
pub const S_ROBOHOOD_FIRE1: state = 255;
pub const S_ROBOHOOD_STAND: state = 254;
pub const S_ROBOHOOD_LOOK: state = 253;
pub const S_POINTYBALL1: state = 252;
pub const S_POINTY1: state = 251;
pub const S_VULTURE_STUNNED: state = 250;
pub const S_VULTURE_ZOOM2: state = 249;
pub const S_VULTURE_ZOOM1: state = 248;
pub const S_VULTURE_DRIFT: state = 247;
pub const S_VULTURE_STND: state = 246;
pub const S_SNAILER_FLICKY: state = 245;
pub const S_SNAILER1: state = 244;
pub const S_JETJAW_SOUND: state = 243;
pub const S_JETJAW_CHOMP16: state = 242;
pub const S_JETJAW_CHOMP15: state = 241;
pub const S_JETJAW_CHOMP14: state = 240;
pub const S_JETJAW_CHOMP13: state = 239;
pub const S_JETJAW_CHOMP12: state = 238;
pub const S_JETJAW_CHOMP11: state = 237;
pub const S_JETJAW_CHOMP10: state = 236;
pub const S_JETJAW_CHOMP9: state = 235;
pub const S_JETJAW_CHOMP8: state = 234;
pub const S_JETJAW_CHOMP7: state = 233;
pub const S_JETJAW_CHOMP6: state = 232;
pub const S_JETJAW_CHOMP5: state = 231;
pub const S_JETJAW_CHOMP4: state = 230;
pub const S_JETJAW_CHOMP3: state = 229;
pub const S_JETJAW_CHOMP2: state = 228;
pub const S_JETJAW_CHOMP1: state = 227;
pub const S_JETJAW_ROAM8: state = 226;
pub const S_JETJAW_ROAM7: state = 225;
pub const S_JETJAW_ROAM6: state = 224;
pub const S_JETJAW_ROAM5: state = 223;
pub const S_JETJAW_ROAM4: state = 222;
pub const S_JETJAW_ROAM3: state = 221;
pub const S_JETJAW_ROAM2: state = 220;
pub const S_JETJAW_ROAM1: state = 219;
pub const S_CDIAG8: state = 218;
pub const S_CDIAG7: state = 217;
pub const S_CDIAG6: state = 216;
pub const S_CDIAG5: state = 215;
pub const S_CDIAG4: state = 214;
pub const S_CDIAG3: state = 213;
pub const S_CDIAG2: state = 212;
pub const S_CDIAG1: state = 211;
pub const S_BANPYURA_ROAMPAUSE: state = 210;
pub const S_BANPYURA_ROAM4: state = 209;
pub const S_BANPYURA_ROAM3: state = 208;
pub const S_BANPYURA_ROAM2: state = 207;
pub const S_BANPYURA_ROAM1: state = 206;
pub const S_CRUSHCHAIN: state = 205;
pub const S_CRUSHCLAW_WAIT: state = 204;
pub const S_CRUSHCLAW_IN: state = 203;
pub const S_CRUSHCLAW_STAY: state = 202;
pub const S_CRUSHCLAW_OUT: state = 201;
pub const S_CRUSHCLAW_AIM: state = 200;
pub const S_CRUSHSTACEAN_PUNCH2: state = 199;
pub const S_CRUSHSTACEAN_PUNCH1: state = 198;
pub const S_CRUSHSTACEAN_ROAMPAUSE: state = 197;
pub const S_CRUSHSTACEAN_ROAM4: state = 196;
pub const S_CRUSHSTACEAN_ROAM3: state = 195;
pub const S_CRUSHSTACEAN_ROAM2: state = 194;
pub const S_CRUSHSTACEAN_ROAM1: state = 193;
pub const S_SPINCUSHION_STOP4: state = 192;
pub const S_SPINCUSHION_STOP3: state = 191;
pub const S_SPINCUSHION_STOP2: state = 190;
pub const S_SPINCUSHION_STOP1: state = 189;
pub const S_SPINCUSHION_SPIN4: state = 188;
pub const S_SPINCUSHION_SPIN3: state = 187;
pub const S_SPINCUSHION_SPIN2: state = 186;
pub const S_SPINCUSHION_SPIN1: state = 185;
pub const S_SPINCUSHION_AIM5: state = 184;
pub const S_SPINCUSHION_AIM4: state = 183;
pub const S_SPINCUSHION_AIM3: state = 182;
pub const S_SPINCUSHION_AIM2: state = 181;
pub const S_SPINCUSHION_AIM1: state = 180;
pub const S_SPINCUSHION_CHASE4: state = 179;
pub const S_SPINCUSHION_CHASE3: state = 178;
pub const S_SPINCUSHION_CHASE2: state = 177;
pub const S_SPINCUSHION_CHASE1: state = 176;
pub const S_SPINCUSHION_LOOK: state = 175;
pub const S_TURRETPOPDOWN8: state = 174;
pub const S_TURRETPOPDOWN7: state = 173;
pub const S_TURRETPOPDOWN6: state = 172;
pub const S_TURRETPOPDOWN5: state = 171;
pub const S_TURRETPOPDOWN4: state = 170;
pub const S_TURRETPOPDOWN3: state = 169;
pub const S_TURRETPOPDOWN2: state = 168;
pub const S_TURRETPOPDOWN1: state = 167;
pub const S_TURRETSHOOT: state = 166;
pub const S_TURRETPOPUP8: state = 165;
pub const S_TURRETPOPUP7: state = 164;
pub const S_TURRETPOPUP6: state = 163;
pub const S_TURRETPOPUP5: state = 162;
pub const S_TURRETPOPUP4: state = 161;
pub const S_TURRETPOPUP3: state = 160;
pub const S_TURRETPOPUP2: state = 159;
pub const S_TURRETPOPUP1: state = 158;
pub const S_TURRETSEE: state = 157;
pub const S_TURRETLOOK: state = 156;
pub const S_TURRETSHOCK9: state = 155;
pub const S_TURRETSHOCK8: state = 154;
pub const S_TURRETSHOCK7: state = 153;
pub const S_TURRETSHOCK6: state = 152;
pub const S_TURRETSHOCK5: state = 151;
pub const S_TURRETSHOCK4: state = 150;
pub const S_TURRETSHOCK3: state = 149;
pub const S_TURRETSHOCK2: state = 148;
pub const S_TURRETSHOCK1: state = 147;
pub const S_TURRETFIRE: state = 146;
pub const S_TURRET: state = 145;
pub const S_SKIM4: state = 144;
pub const S_SKIM3: state = 143;
pub const S_SKIM2: state = 142;
pub const S_SKIM1: state = 141;
pub const S_DETON15: state = 140;
pub const S_DETON14: state = 139;
pub const S_DETON13: state = 138;
pub const S_DETON12: state = 137;
pub const S_DETON11: state = 136;
pub const S_DETON10: state = 135;
pub const S_DETON9: state = 134;
pub const S_DETON8: state = 133;
pub const S_DETON7: state = 132;
pub const S_DETON6: state = 131;
pub const S_DETON5: state = 130;
pub const S_DETON4: state = 129;
pub const S_DETON3: state = 128;
pub const S_DETON2: state = 127;
pub const S_DETON1: state = 126;
pub const S_CCOMMAND4: state = 125;
pub const S_CCOMMAND3: state = 124;
pub const S_CCOMMAND2: state = 123;
pub const S_CCOMMAND1: state = 122;
pub const S_JETGSHOOT2: state = 121;
pub const S_JETGSHOOT1: state = 120;
pub const S_JETGZOOM2: state = 119;
pub const S_JETGZOOM1: state = 118;
pub const S_JETGLOOK2: state = 117;
pub const S_JETGLOOK1: state = 116;
pub const S_JETBZOOM2: state = 115;
pub const S_JETBZOOM1: state = 114;
pub const S_JETBLOOK2: state = 113;
pub const S_JETBLOOK1: state = 112;
pub const S_RBUZZFLY2: state = 111;
pub const S_RBUZZFLY1: state = 110;
pub const S_RBUZZLOOK2: state = 109;
pub const S_RBUZZLOOK1: state = 108;
pub const S_BUZZFLY2: state = 107;
pub const S_BUZZFLY1: state = 106;
pub const S_BUZZLOOK2: state = 105;
pub const S_BUZZLOOK1: state = 104;
pub const S_FISH4: state = 103;
pub const S_FISH3: state = 102;
pub const S_FISH2: state = 101;
pub const S_FISH1: state = 100;
pub const S_SPOS_RUN6: state = 99;
pub const S_SPOS_RUN5: state = 98;
pub const S_SPOS_RUN4: state = 97;
pub const S_SPOS_RUN3: state = 96;
pub const S_SPOS_RUN2: state = 95;
pub const S_SPOS_RUN1: state = 94;
pub const S_SPOS_STND: state = 93;
pub const S_POSS_RUN6: state = 92;
pub const S_POSS_RUN5: state = 91;
pub const S_POSS_RUN4: state = 90;
pub const S_POSS_RUN3: state = 89;
pub const S_POSS_RUN2: state = 88;
pub const S_POSS_RUN1: state = 87;
pub const S_POSS_STND: state = 86;
pub const S_JETFUMEFLASH: state = 85;
pub const S_TAILSOVERLAY_DASH: state = 84;
pub const S_TAILSOVERLAY_EDGE: state = 83;
pub const S_TAILSOVERLAY_GASP: state = 82;
pub const S_TAILSOVERLAY_PAIN: state = 81;
pub const S_TAILSOVERLAY_TIRE: state = 80;
pub const S_TAILSOVERLAY_FLY: state = 79;
pub const S_TAILSOVERLAY_RUN: state = 78;
pub const S_TAILSOVERLAY_MINUS60DEGREES: state = 77;
pub const S_TAILSOVERLAY_MINUS30DEGREES: state = 76;
pub const S_TAILSOVERLAY_PLUS60DEGREES: state = 75;
pub const S_TAILSOVERLAY_PLUS30DEGREES: state = 74;
pub const S_TAILSOVERLAY_0DEGREES: state = 73;
pub const S_TAILSOVERLAY_STAND: state = 72;
pub const S_PLAY_NIGHTS_ATTACK: state = 71;
pub const S_PLAY_NIGHTS_PULL: state = 70;
pub const S_PLAY_NIGHTS_STUN: state = 69;
pub const S_PLAY_NIGHTS_DRILL: state = 68;
pub const S_PLAY_NIGHTS_FLY: state = 67;
pub const S_PLAY_NIGHTS_FLOAT: state = 66;
pub const S_PLAY_NIGHTS_STAND: state = 65;
pub const S_PLAY_NIGHTS_TRANS6: state = 64;
pub const S_PLAY_NIGHTS_TRANS5: state = 63;
pub const S_PLAY_NIGHTS_TRANS4: state = 62;
pub const S_PLAY_NIGHTS_TRANS3: state = 61;
pub const S_PLAY_NIGHTS_TRANS2: state = 60;
pub const S_PLAY_NIGHTS_TRANS1: state = 59;
pub const S_PLAY_SIGN: state = 58;
pub const S_PLAY_ICON3: state = 57;
pub const S_PLAY_ICON2: state = 56;
pub const S_PLAY_ICON1: state = 55;
pub const S_PLAY_BOX2: state = 54;
pub const S_PLAY_BOX1: state = 53;
pub const S_OBJPLACE_DUMMY: state = 52;
pub const S_PLAY_SUPER_TRANS6: state = 51;
pub const S_PLAY_SUPER_TRANS5: state = 50;
pub const S_PLAY_SUPER_TRANS4: state = 49;
pub const S_PLAY_SUPER_TRANS3: state = 48;
pub const S_PLAY_SUPER_TRANS2: state = 47;
pub const S_PLAY_SUPER_TRANS1: state = 46;
pub const S_PLAY_MELEE_LANDING: state = 45;
pub const S_PLAY_MELEE_FINISH: state = 44;
pub const S_PLAY_MELEE: state = 43;
pub const S_PLAY_TWINSPIN: state = 42;
pub const S_PLAY_FIRE_FINISH: state = 41;
pub const S_PLAY_FIRE: state = 40;
pub const S_PLAY_BOUNCE_LANDING: state = 39;
pub const S_PLAY_BOUNCE: state = 38;
pub const S_PLAY_FLOAT_RUN: state = 37;
pub const S_PLAY_FLOAT: state = 36;
pub const S_PLAY_CLIMB: state = 35;
pub const S_PLAY_CLING: state = 34;
pub const S_PLAY_GLIDE_LANDING: state = 33;
pub const S_PLAY_GLIDE: state = 32;
pub const S_PLAY_FLY_TIRED: state = 31;
pub const S_PLAY_SWIM: state = 30;
pub const S_PLAY_FLY: state = 29;
pub const S_PLAY_SPINDASH: state = 28;
pub const S_PLAY_RIDE: state = 27;
pub const S_PLAY_EDGE: state = 26;
pub const S_PLAY_FALL: state = 25;
pub const S_PLAY_SPRING: state = 24;
pub const S_PLAY_JUMP: state = 23;
pub const S_PLAY_GASP: state = 22;
pub const S_PLAY_ROLL: state = 21;
pub const S_PLAY_DRWN: state = 20;
pub const S_PLAY_DEAD: state = 19;
pub const S_PLAY_STUN: state = 18;
pub const S_PLAY_PAIN: state = 17;
pub const S_PLAY_DASH: state = 16;
pub const S_PLAY_RUN: state = 15;
pub const S_PLAY_SKID: state = 14;
pub const S_PLAY_WALK: state = 13;
pub const S_PLAY_WAIT: state = 12;
pub const S_PLAY_STND: state = 11;
pub const S_THOK: state = 10;
pub const S_RAISESTATE: state = 9;
pub const S_XDEATHSTATE: state = 8;
pub const S_DEATHSTATE: state = 7;
pub const S_MISSILESTATE: state = 6;
pub const S_MELEESTATE: state = 5;
pub const S_SEESTATE: state = 4;
pub const S_SPAWNSTATE: state = 3;
pub const S_INVISIBLE: state = 2;
pub const S_UNKNOWN: state = 1;
pub const S_NULL: state = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state_t {
    pub sprite: spritenum_t,
    pub frame: uint32_t,
    pub tics: int32_t,
    pub action: actionf_t,
    pub var1: int32_t,
    pub var2: int32_t,
    pub nextstate: statenum_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union actionf_t {
    pub acv: actionf_v,
    pub acp1: actionf_p1,
}
pub type actionf_p1 = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type actionf_v = Option::<unsafe extern "C" fn() -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ffloor_s {
    pub topheight: *mut fixed_t,
    pub toppic: *mut int32_t,
    pub toplightlevel: *mut int16_t,
    pub topxoffs: *mut fixed_t,
    pub topyoffs: *mut fixed_t,
    pub topangle: *mut angle_t,
    pub bottomheight: *mut fixed_t,
    pub bottompic: *mut int32_t,
    pub bottomxoffs: *mut fixed_t,
    pub bottomyoffs: *mut fixed_t,
    pub bottomangle: *mut angle_t,
    pub t_slope: *mut *mut pslope_s,
    pub b_slope: *mut *mut pslope_s,
    pub secnum: size_t,
    pub fofflags: ffloortype_e,
    pub master: *mut line_s,
    pub target: *mut sector_s,
    pub next: *mut ffloor_s,
    pub prev: *mut ffloor_s,
    pub lastlight: int32_t,
    pub alpha: int32_t,
    pub blend: uint8_t,
    pub norender: tic_t,
    pub bustflags: ffloorbustflags_e,
    pub busttype: uint8_t,
    pub busttag: int16_t,
    pub sinkspeed: fixed_t,
    pub friction: fixed_t,
    pub bouncestrength: fixed_t,
    pub spawnflags: ffloortype_e,
    pub spawnalpha: int32_t,
    pub fadingdata: *mut libc::c_void,
}
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
pub type ffloorbustflags_e = libc::c_uint;
pub const FB_ONLYBOTTOM: ffloorbustflags_e = 4;
pub const FB_EXECUTOR: ffloorbustflags_e = 2;
pub const FB_PUSHABLES: ffloorbustflags_e = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sector_s {
    pub floorheight: fixed_t,
    pub ceilingheight: fixed_t,
    pub floorpic: int32_t,
    pub ceilingpic: int32_t,
    pub lightlevel: int16_t,
    pub special: int16_t,
    pub tags: taglist_t,
    pub soundorg: degenmobj_t,
    pub validcount: size_t,
    pub thinglist: *mut mobj_t,
    pub floordata: *mut libc::c_void,
    pub ceilingdata: *mut libc::c_void,
    pub lightingdata: *mut libc::c_void,
    pub fadecolormapdata: *mut libc::c_void,
    pub floorxoffset: fixed_t,
    pub flooryoffset: fixed_t,
    pub ceilingxoffset: fixed_t,
    pub ceilingyoffset: fixed_t,
    pub floorangle: angle_t,
    pub ceilingangle: angle_t,
    pub heightsec: int32_t,
    pub camsec: int32_t,
    pub floorlightlevel: int16_t,
    pub ceilinglightlevel: int16_t,
    pub floorlightabsolute: boolean,
    pub ceilinglightabsolute: boolean,
    pub floorlightsec: int32_t,
    pub ceilinglightsec: int32_t,
    pub crumblestate: int32_t,
    pub touching_thinglist: *mut msecnode_s,
    pub linecount: size_t,
    pub lines: *mut *mut line_s,
    pub ffloors: *mut ffloor_t,
    pub attached: *mut size_t,
    pub attachedsolid: *mut boolean,
    pub numattached: size_t,
    pub maxattached: size_t,
    pub lightlist: *mut lightlist_t,
    pub numlights: int32_t,
    pub moved: boolean,
    pub extra_colormap: *mut extracolormap_t,
    pub colormap_protected: boolean,
    pub gravity: fixed_t,
    pub gravityptr: *mut fixed_t,
    pub flags: sectorflags_t,
    pub specialflags: sectorspecialflags_t,
    pub damagetype: uint8_t,
    pub triggertag: mtag_t,
    pub triggerer: uint8_t,
    pub friction: fixed_t,
    pub cullheight: *mut line_s,
    pub floorspeed: fixed_t,
    pub ceilspeed: fixed_t,
    pub preciplist: *mut precipmobj_t,
    pub touching_preciplist: *mut mprecipsecnode_s,
    pub f_slope: *mut pslope_t,
    pub c_slope: *mut pslope_t,
    pub hasslope: boolean,
    pub spawn_lightlevel: int16_t,
    pub spawn_extra_colormap: *mut extracolormap_t,
}
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
pub type pslope_t = pslope_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mprecipsecnode_s {
    pub m_sector: *mut sector_t,
    pub m_thing: *mut precipmobj_s,
    pub m_sectorlist_prev: *mut mprecipsecnode_s,
    pub m_sectorlist_next: *mut mprecipsecnode_s,
    pub m_thinglist_prev: *mut mprecipsecnode_s,
    pub m_thinglist_next: *mut mprecipsecnode_s,
    pub visited: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct precipmobj_s {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub old_x: fixed_t,
    pub old_y: fixed_t,
    pub old_z: fixed_t,
    pub old_x2: fixed_t,
    pub old_y2: fixed_t,
    pub old_z2: fixed_t,
    pub snext: *mut precipmobj_s,
    pub sprev: *mut *mut precipmobj_s,
    pub angle: angle_t,
    pub pitch: angle_t,
    pub roll: angle_t,
    pub old_angle: angle_t,
    pub old_pitch: angle_t,
    pub old_roll: angle_t,
    pub old_angle2: angle_t,
    pub old_pitch2: angle_t,
    pub old_roll2: angle_t,
    pub spriteroll: angle_t,
    pub old_spriteroll: angle_t,
    pub old_spriteroll2: angle_t,
    pub sprite: spritenum_t,
    pub frame: uint32_t,
    pub sprite2: uint8_t,
    pub anim_duration: uint16_t,
    pub renderflags: uint32_t,
    pub blendmode: int32_t,
    pub spritexscale: fixed_t,
    pub spriteyscale: fixed_t,
    pub spritexoffset: fixed_t,
    pub spriteyoffset: fixed_t,
    pub old_spritexscale: fixed_t,
    pub old_spriteyscale: fixed_t,
    pub old_spritexoffset: fixed_t,
    pub old_spriteyoffset: fixed_t,
    pub floorspriteslope: *mut pslope_s,
    pub touching_sectorlist: *mut mprecipsecnode_s,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub floorrover: *mut ffloor_s,
    pub ceilingrover: *mut ffloor_s,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
    pub precipflags: fixed_t,
    pub tics: int32_t,
    pub state: *mut state_t,
    pub flags: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subsector_s {
    pub sector: *mut sector_t,
    pub numlines: int16_t,
    pub firstline: uint32_t,
    pub polyList: *mut polyobj_s,
    pub validcount: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyobj_s {
    pub link: mdllistitem_t,
    pub id: int32_t,
    pub first: int32_t,
    pub next: int32_t,
    pub parent: int32_t,
    pub segCount: size_t,
    pub numSegsAlloc: size_t,
    pub segs: *mut *mut seg_s,
    pub numVertices: size_t,
    pub numVerticesAlloc: size_t,
    pub origVerts: *mut vertex_t,
    pub tmpVerts: *mut vertex_t,
    pub vertices: *mut *mut vertex_t,
    pub numLines: size_t,
    pub numLinesAlloc: size_t,
    pub lines: *mut *mut line_s,
    pub spawnSpot: degenmobj_t,
    pub centerPt: vertex_t,
    pub zdist: fixed_t,
    pub angle: angle_t,
    pub attached: uint8_t,
    pub blockbox: [fixed_t; 4],
    pub linked: uint8_t,
    pub validcount: size_t,
    pub damage: int32_t,
    pub thrust: fixed_t,
    pub flags: int32_t,
    pub thinker: *mut thinker_t,
    pub isBad: uint8_t,
    pub translucency: int32_t,
    pub triggertag: int16_t,
    pub visplane: *mut visplane_s,
    pub spawnflags: int32_t,
    pub spawntrans: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visplane_s {
    pub next: *mut visplane_s,
    pub height: fixed_t,
    pub viewx: fixed_t,
    pub viewy: fixed_t,
    pub viewz: fixed_t,
    pub viewangle: angle_t,
    pub plangle: angle_t,
    pub picnum: int32_t,
    pub lightlevel: int32_t,
    pub minx: int32_t,
    pub maxx: int32_t,
    pub extra_colormap: *mut extracolormap_t,
    pub padtopstart: uint16_t,
    pub top: [uint16_t; 1920],
    pub padtopend: uint16_t,
    pub padbottomstart: uint16_t,
    pub bottom: [uint16_t; 1920],
    pub padbottomend: uint16_t,
    pub high: int32_t,
    pub low: int32_t,
    pub xoffs: fixed_t,
    pub yoffs: fixed_t,
    pub ffloor: *mut ffloor_s,
    pub polyobj: *mut polyobj_t,
    pub slope: *mut pslope_t,
}
pub type polyobj_t = polyobj_s;
pub type thinker_t = thinker_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinker_s {
    pub prev: *mut thinker_s,
    pub next: *mut thinker_s,
    pub function: think_t,
    pub references: int32_t,
}
pub type think_t = actionf_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vertex_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub floorzset: boolean,
    pub ceilingzset: boolean,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct degenmobj_t {
    pub thinker: thinker_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_s {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub angle: angle_t,
    pub flags: int16_t,
    pub special: int16_t,
    pub tags: taglist_t,
    pub args: [int32_t; 10],
    pub stringargs: [*mut libc::c_char; 2],
    pub sidenum: [uint16_t; 2],
    pub alpha: fixed_t,
    pub blendmode: uint8_t,
    pub executordelay: int32_t,
    pub bbox: [fixed_t; 4],
    pub slopetype: slopetype_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub validcount: size_t,
    pub polyobj: *mut polyobj_t,
    pub callcount: int16_t,
}
pub type sector_t = sector_s;
pub type slopetype_t = libc::c_uint;
pub const ST_NEGATIVE: slopetype_t = 3;
pub const ST_POSITIVE: slopetype_t = 2;
pub const ST_VERTICAL: slopetype_t = 1;
pub const ST_HORIZONTAL: slopetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct seg_s {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,
    pub side: int32_t,
    pub offset: fixed_t,
    pub angle: angle_t,
    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
    pub length: fixed_t,
    pub numlights: size_t,
    pub rlights: *mut r_lightlist_t,
    pub polyseg: *mut polyobj_t,
    pub dontrenderme: boolean,
    pub glseg: boolean,
}
pub type r_lightlist_t = r_lightlist_s;
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
pub type line_t = line_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct side_t {
    pub textureoffset: fixed_t,
    pub rowoffset: fixed_t,
    pub offsetx_top: fixed_t,
    pub offsetx_mid: fixed_t,
    pub offsetx_bot: fixed_t,
    pub offsety_top: fixed_t,
    pub offsety_mid: fixed_t,
    pub offsety_bot: fixed_t,
    pub toptexture: int32_t,
    pub bottomtexture: int32_t,
    pub midtexture: int32_t,
    pub line: *mut line_t,
    pub sector: *mut sector_t,
    pub special: int16_t,
    pub repeatcnt: int16_t,
    pub colormap_data: *mut extracolormap_t,
}
pub type mdllistitem_t = mdllistitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdllistitem_s {
    pub next: *mut mdllistitem_s,
    pub prev: *mut *mut mdllistitem_s,
}
pub type precipmobj_t = precipmobj_s;
pub type sectorspecialflags_t = libc::c_uint;
pub const SSF_GRAVITYOVERRIDE: sectorspecialflags_t = 1048576;
pub const SSF_JUMPFLIP: sectorspecialflags_t = 524288;
pub const SSF_ROPEHANG: sectorspecialflags_t = 262144;
pub const SSF_FINISHLINE: sectorspecialflags_t = 131072;
pub const SSF_ZOOMTUBEEND: sectorspecialflags_t = 65536;
pub const SSF_ZOOMTUBESTART: sectorspecialflags_t = 32768;
pub const SSF_FORCESPIN: sectorspecialflags_t = 16384;
pub const SSF_SUPERTRANSFORM: sectorspecialflags_t = 8192;
pub const SSF_FAN: sectorspecialflags_t = 4096;
pub const SSF_BLUETEAMBASE: sectorspecialflags_t = 2048;
pub const SSF_REDTEAMBASE: sectorspecialflags_t = 1024;
pub const SSF_RETURNFLAG: sectorspecialflags_t = 512;
pub const SSF_SPECIALSTAGEPIT: sectorspecialflags_t = 256;
pub const SSF_EXIT: sectorspecialflags_t = 128;
pub const SSF_STARPOSTACTIVATOR: sectorspecialflags_t = 64;
pub const SSF_SPEEDPAD: sectorspecialflags_t = 32;
pub const SSF_CONVEYOR: sectorspecialflags_t = 16;
pub const SSF_WINDCURRENT: sectorspecialflags_t = 8;
pub const SSF_NOSTEPDOWN: sectorspecialflags_t = 4;
pub const SSF_DOUBLESTEPUP: sectorspecialflags_t = 2;
pub const SSF_OUTERSPACE: sectorspecialflags_t = 1;
pub type sectorflags_t = libc::c_uint;
pub const MSF_NOCLIPCAMERA: sectorflags_t = 512;
pub const MSF_HEATWAVE: sectorflags_t = 256;
pub const MSF_GRAVITYFLIP: sectorflags_t = 128;
pub const MSF_INVERTPRECIP: sectorflags_t = 64;
pub const MSF_TRIGGERLINE_MOBJ: sectorflags_t = 32;
pub const MSF_TRIGGERLINE_PLANE: sectorflags_t = 16;
pub const MSF_TRIGGERSPECIAL_HEADBUMP: sectorflags_t = 8;
pub const MSF_TRIGGERSPECIAL_TOUCH: sectorflags_t = 4;
pub const MSF_FLIPSPECIAL_BOTH: sectorflags_t = 3;
pub const MSF_FLIPSPECIAL_CEILING: sectorflags_t = 2;
pub const MSF_FLIPSPECIAL_FLOOR: sectorflags_t = 1;
pub type lightlist_t = lightlist_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightlist_s {
    pub height: fixed_t,
    pub lightlevel: *mut int16_t,
    pub extra_colormap: *mut *mut extracolormap_t,
    pub flags: int32_t,
    pub caster: *mut ffloor_t,
    pub slope: *mut pslope_s,
}
pub type ffloor_t = ffloor_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msecnode_s {
    pub m_sector: *mut sector_t,
    pub m_thing: *mut mobj_s,
    pub m_sectorlist_prev: *mut msecnode_s,
    pub m_sectorlist_next: *mut msecnode_s,
    pub m_thinglist_prev: *mut msecnode_s,
    pub m_thinglist_next: *mut msecnode_s,
    pub visited: boolean,
}
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const BT_CUSTOM3: C2RustUnnamed_0 = 32768;
pub const BT_CUSTOM2: C2RustUnnamed_0 = 16384;
pub const BT_CUSTOM1: C2RustUnnamed_0 = 8192;
pub const BT_FIRENORMAL: C2RustUnnamed_0 = 4096;
pub const BT_JUMP: C2RustUnnamed_0 = 2048;
pub const BT_TOSSFLAG: C2RustUnnamed_0 = 1024;
pub const BT_CAMRIGHT: C2RustUnnamed_0 = 512;
pub const BT_CAMLEFT: C2RustUnnamed_0 = 256;
pub const BT_SPIN: C2RustUnnamed_0 = 128;
pub const BT_ATTACK: C2RustUnnamed_0 = 64;
pub const BT_WEAPONPREV: C2RustUnnamed_0 = 32;
pub const BT_WEAPONNEXT: C2RustUnnamed_0 = 16;
pub const BT_WEAPONMASK: C2RustUnnamed_0 = 15;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUMPOWERS: C2RustUnnamed_1 = 30;
pub const pw_strong: C2RustUnnamed_1 = 29;
pub const pw_ignorelatch: C2RustUnnamed_1 = 28;
pub const pw_justlaunched: C2RustUnnamed_1 = 27;
pub const pw_dye: C2RustUnnamed_1 = 26;
pub const pw_nocontrol: C2RustUnnamed_1 = 25;
pub const pw_nights_linkfreeze: C2RustUnnamed_1 = 24;
pub const pw_nights_helper: C2RustUnnamed_1 = 23;
pub const pw_nights_superloop: C2RustUnnamed_1 = 22;
pub const pw_emeralds: C2RustUnnamed_1 = 21;
pub const pw_railring: C2RustUnnamed_1 = 20;
pub const pw_explosionring: C2RustUnnamed_1 = 19;
pub const pw_grenadering: C2RustUnnamed_1 = 18;
pub const pw_scatterring: C2RustUnnamed_1 = 17;
pub const pw_bouncering: C2RustUnnamed_1 = 16;
pub const pw_automaticring: C2RustUnnamed_1 = 15;
pub const pw_infinityring: C2RustUnnamed_1 = 14;
pub const pw_gravityboots: C2RustUnnamed_1 = 13;
pub const pw_super: C2RustUnnamed_1 = 12;
pub const pw_noautobrake: C2RustUnnamed_1 = 11;
pub const pw_justsprung: C2RustUnnamed_1 = 10;
pub const pw_pushing: C2RustUnnamed_1 = 9;
pub const pw_extralife: C2RustUnnamed_1 = 8;
pub const pw_spacetime: C2RustUnnamed_1 = 7;
pub const pw_underwater: C2RustUnnamed_1 = 6;
pub const pw_tailsfly: C2RustUnnamed_1 = 5;
pub const pw_carry: C2RustUnnamed_1 = 4;
pub const pw_shield: C2RustUnnamed_1 = 3;
pub const pw_flashing: C2RustUnnamed_1 = 2;
pub const pw_sneakers: C2RustUnnamed_1 = 1;
pub const pw_invulnerability: C2RustUnnamed_1 = 0;
pub type player_t = player_s;
pub type marathonmode_t = libc::c_uint;
pub const MA_INGAME: marathonmode_t = 8;
pub const MA_NOCUTSCENES: marathonmode_t = 4;
pub const MA_INIT: marathonmode_t = 2;
pub const MA_RUNNING: marathonmode_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scene_t {
    pub numpics: uint8_t,
    pub picname: [[libc::c_char; 8]; 8],
    pub pichires: [uint8_t; 8],
    pub text: *mut libc::c_char,
    pub xcoord: [uint16_t; 8],
    pub ycoord: [uint16_t; 8],
    pub picduration: [uint16_t; 8],
    pub musicloop: uint8_t,
    pub textxpos: uint16_t,
    pub textypos: uint16_t,
    pub musswitch: [libc::c_char; 7],
    pub musswitchflags: uint16_t,
    pub musswitchposition: uint32_t,
    pub fadecolor: uint8_t,
    pub fadeinid: uint8_t,
    pub fadeoutid: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cutscene_t {
    pub scene: [scene_t; 128],
    pub numscenes: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textpage_t {
    pub numpics: uint8_t,
    pub picmode: uint8_t,
    pub pictoloop: uint8_t,
    pub pictostart: uint8_t,
    pub picname: [[libc::c_char; 8]; 8],
    pub pichires: [uint8_t; 8],
    pub xcoord: [uint16_t; 8],
    pub ycoord: [uint16_t; 8],
    pub picduration: [uint16_t; 8],
    pub musswitch: [libc::c_char; 7],
    pub musswitchflags: uint16_t,
    pub musicloop: uint8_t,
    pub tag: [libc::c_char; 33],
    pub name: [libc::c_char; 34],
    pub iconname: [libc::c_char; 8],
    pub rightside: boolean,
    pub iconflip: boolean,
    pub hidehud: uint8_t,
    pub lines: uint8_t,
    pub backcolor: int32_t,
    pub align: uint8_t,
    pub verticalalign: uint8_t,
    pub textspeed: uint8_t,
    pub textsfx: sfxenum_t,
    pub nextprompt: uint8_t,
    pub nextpage: uint8_t,
    pub nexttag: [libc::c_char; 33],
    pub timetonext: int32_t,
    pub text: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textprompt_t {
    pub page: [textpage_t; 128],
    pub numpages: int32_t,
}
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
    pub revert: C2RustUnnamed_2,
    pub netid: uint16_t,
    pub changed: libc::c_char,
    pub next: *mut consvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub allocated: libc::c_char,
    pub v: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub string: *mut libc::c_char,
    pub const_munge: *const libc::c_char,
}
pub type consvar_t = consvar_s;
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
pub const KEY_HAT1: C2RustUnnamed_11 = 296;
pub const KEY_JOY1: C2RustUnnamed_11 = 264;
pub const KEY_MOUSE1: C2RustUnnamed_11 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gamedata_t {
    pub loaded: boolean,
    pub achieved: [boolean; 128],
    pub collected: [boolean; 512],
    pub extraCollected: [boolean; 48],
    pub unlocked: [boolean; 80],
    pub mainrecords: [*mut recorddata_t; 1035],
    pub nightsrecords: [*mut nightsdata_t; 1035],
    pub mapvisited: [uint8_t; 1035],
    pub timesBeaten: uint32_t,
    pub timesBeatenWithEmeralds: uint32_t,
    pub timesBeatenUltimate: uint32_t,
    pub totalplaytime: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nightsdata_t {
    pub nummares: uint8_t,
    pub score: [uint32_t; 9],
    pub grade: [uint8_t; 9],
    pub time: [tic_t; 9],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recorddata_t {
    pub time: tic_t,
    pub score: uint32_t,
    pub rings: uint16_t,
}
pub const TC_BLINK: C2RustUnnamed_8 = -124;
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
pub const TC_ALLWHITE: C2RustUnnamed_8 = -126;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_9 = 15;
pub type wipestyleflags_t = libc::c_uint;
pub const WSF_CROSSFADE: wipestyleflags_t = 8;
pub const WSF_TOWHITE: wipestyleflags_t = 4;
pub const WSF_FADEIN: wipestyleflags_t = 2;
pub const WSF_FADEOUT: wipestyleflags_t = 1;
pub const render_none: rendermode_t = 3;
pub type rendermode_t = libc::c_uint;
pub const render_opengl: rendermode_t = 2;
pub const render_soft: rendermode_t = 1;
pub type moviemode_t = libc::c_uint;
pub const MM_SCREENSHOT: moviemode_t = 3;
pub const MM_GIF: moviemode_t = 2;
pub const MM_APNG: moviemode_t = 1;
pub const MM_OFF: moviemode_t = 0;
pub type demo_file_override_e = libc::c_uint;
pub const DFILE_OVERRIDE_SKIP: demo_file_override_e = 2;
pub const DFILE_OVERRIDE_LOAD: demo_file_override_e = 1;
pub const DFILE_OVERRIDE_NONE: demo_file_override_e = 0;
pub type camera_t = camera_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camera_s {
    pub chase: boolean,
    pub aiming: angle_t,
    pub viewheight: fixed_t,
    pub startangle: angle_t,
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub reset: boolean,
    pub angle: angle_t,
    pub subsector: *mut subsector_s,
    pub floorz: fixed_t,
    pub ceilingz: fixed_t,
    pub radius: fixed_t,
    pub height: fixed_t,
    pub relativex: fixed_t,
    pub momx: fixed_t,
    pub momy: fixed_t,
    pub momz: fixed_t,
}
pub const THINK_MOBJ: C2RustUnnamed_7 = 2;
pub const gcs_platform: C2RustUnnamed_12 = 2;
pub const gcs_fps: C2RustUnnamed_12 = 1;
pub const gcs_custom: C2RustUnnamed_12 = 0;
pub type huddrawlist_h = *mut huddrawlist_s;
pub const hudhook_title: C2RustUnnamed_13 = 2;
pub const TTMODE_USER: ttmode_enum = 3;
pub const TC_RAINBOW: C2RustUnnamed_8 = -125;
pub type viddef_t = viddef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed_4,
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
pub union C2RustUnnamed_4 {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
pub const TTMODE_ALACROIX: ttmode_enum = 2;
pub const TTMODE_NONE: ttmode_enum = 0;
pub const TTMODE_OLD: ttmode_enum = 1;
pub type ttmode_enum = libc::c_uint;
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
pub const TC_DEFAULT: C2RustUnnamed_8 = -122;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub x: uint32_t,
    pub patch: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menupres_t {
    pub bgname: [libc::c_char; 8],
    pub fadestrength: int8_t,
    pub bgcolor: int32_t,
    pub titlescrollxspeed: int32_t,
    pub titlescrollyspeed: int32_t,
    pub bghide: boolean,
    pub hidetitlepics: int8_t,
    pub ttmode: ttmode_enum,
    pub ttscale: uint8_t,
    pub ttname: [libc::c_char; 9],
    pub ttx: int16_t,
    pub tty: int16_t,
    pub ttloop: int16_t,
    pub tttics: uint16_t,
    pub musname: [libc::c_char; 7],
    pub mustrack: uint16_t,
    pub muslooping: boolean,
    pub musstop: boolean,
    pub musignore: boolean,
    pub enterbubble: boolean,
    pub exitbubble: boolean,
    pub entertag: int32_t,
    pub exittag: int32_t,
    pub enterwipe: int16_t,
    pub exitwipe: int16_t,
}
pub const MN_MAIN: C2RustUnnamed_10 = 1;
pub type subsector_t = subsector_s;
pub const TITLEMAP_LOADING: C2RustUnnamed_6 = 1;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const TITLEMAP_RUNNING: C2RustUnnamed_6 = 2;
pub type menu_t = menu_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menu_s {
    pub menuid: uint32_t,
    pub menutitlepic: *const libc::c_char,
    pub numitems: int16_t,
    pub prevMenu: *mut menu_s,
    pub menuitems: *mut menuitem_t,
    pub drawroutine: Option::<unsafe extern "C" fn() -> ()>,
    pub x: int16_t,
    pub y: int16_t,
    pub lastOn: int16_t,
    pub quitroutine: Option::<unsafe extern "C" fn() -> boolean>,
}
pub type menuitem_t = menuitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuitem_s {
    pub status: uint16_t,
    pub patch: *const libc::c_char,
    pub text: *const libc::c_char,
    pub itemaction: *mut libc::c_void,
    pub alphaKey: uint16_t,
}
pub type C2RustUnnamed_7 = libc::c_uint;
pub const NUM_THINKERLISTS: C2RustUnnamed_7 = 5;
pub const THINK_PRECIP: C2RustUnnamed_7 = 4;
pub const THINK_DYNSLOPE: C2RustUnnamed_7 = 3;
pub const THINK_MAIN: C2RustUnnamed_7 = 1;
pub const THINK_POLYOBJ: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_int;
pub const TC_DASHMODE: C2RustUnnamed_8 = -123;
pub const TC_METALSONIC: C2RustUnnamed_8 = -127;
pub const TC_BOSS: C2RustUnnamed_8 = -128;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_9 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_9 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_9 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_9 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_9 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_9 = 51;
pub const PU_LEVEL: C2RustUnnamed_9 = 50;
pub const PU_CACHE: C2RustUnnamed_9 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_9 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_9 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_9 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_9 = 21;
pub const PU_HUDGFX: C2RustUnnamed_9 = 19;
pub const PU_SPRITE: C2RustUnnamed_9 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_9 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_9 = 16;
pub const PU_PATCH: C2RustUnnamed_9 = 14;
pub const PU_MUSIC: C2RustUnnamed_9 = 12;
pub const PU_SOUND: C2RustUnnamed_9 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_9 = 3;
pub const PU_LUA: C2RustUnnamed_9 = 2;
pub const PU_STATIC: C2RustUnnamed_9 = 1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const NUMMENUTYPES: C2RustUnnamed_10 = 58;
pub const MN_SPECIAL: C2RustUnnamed_10 = 57;
pub const MN_AD_MAIN: C2RustUnnamed_10 = 56;
pub const MN_SR_SOUNDTEST: C2RustUnnamed_10 = 55;
pub const MN_SR_PLAYER: C2RustUnnamed_10 = 54;
pub const MN_SR_EMBLEMHINT: C2RustUnnamed_10 = 53;
pub const MN_SR_UNLOCKCHECKLIST: C2RustUnnamed_10 = 52;
pub const MN_SR_LEVELSELECT: C2RustUnnamed_10 = 51;
pub const MN_SR_PANDORA: C2RustUnnamed_10 = 50;
pub const MN_SR_MAIN: C2RustUnnamed_10 = 49;
pub const MN_OP_ERASEDATA: C2RustUnnamed_10 = 48;
pub const MN_OP_SCREENSHOTS: C2RustUnnamed_10 = 47;
pub const MN_OP_ADDONS: C2RustUnnamed_10 = 46;
pub const MN_OP_DATA: C2RustUnnamed_10 = 45;
pub const MN_OP_MONITORTOGGLE: C2RustUnnamed_10 = 44;
pub const MN_OP_SERVER: C2RustUnnamed_10 = 43;
pub const MN_OP_SOUND: C2RustUnnamed_10 = 42;
pub const MN_OP_OPENGL_LIGHTING: C2RustUnnamed_10 = 41;
pub const MN_OP_OPENGL: C2RustUnnamed_10 = 40;
pub const MN_OP_COLOR: C2RustUnnamed_10 = 39;
pub const MN_OP_VIDEOMODE: C2RustUnnamed_10 = 38;
pub const MN_OP_VIDEO: C2RustUnnamed_10 = 37;
pub const MN_OP_PLAYSTYLE: C2RustUnnamed_10 = 36;
pub const MN_OP_P2CAMERA: C2RustUnnamed_10 = 35;
pub const MN_OP_P2JOYSTICK: C2RustUnnamed_10 = 34;
pub const MN_OP_P2MOUSE: C2RustUnnamed_10 = 33;
pub const MN_OP_P2CONTROLS: C2RustUnnamed_10 = 32;
pub const MN_OP_P1CAMERA: C2RustUnnamed_10 = 31;
pub const MN_OP_JOYSTICKSET: C2RustUnnamed_10 = 30;
pub const MN_OP_P1JOYSTICK: C2RustUnnamed_10 = 29;
pub const MN_OP_P1MOUSE: C2RustUnnamed_10 = 28;
pub const MN_OP_CHANGECONTROLS: C2RustUnnamed_10 = 27;
pub const MN_OP_P1CONTROLS: C2RustUnnamed_10 = 26;
pub const MN_OP_MAIN: C2RustUnnamed_10 = 25;
pub const MN_MP_SERVER_OPTIONS: C2RustUnnamed_10 = 24;
pub const MN_MP_PLAYERSETUP: C2RustUnnamed_10 = 23;
pub const MN_MP_ROOM: C2RustUnnamed_10 = 22;
pub const MN_MP_CONNECT: C2RustUnnamed_10 = 21;
pub const MN_MP_SERVER: C2RustUnnamed_10 = 20;
pub const MN_MP_SPLITSCREEN: C2RustUnnamed_10 = 19;
pub const MN_MP_MAIN: C2RustUnnamed_10 = 18;
pub const MN_SP_MARATHON: C2RustUnnamed_10 = 17;
pub const MN_SP_NIGHTS_GHOST: C2RustUnnamed_10 = 16;
pub const MN_SP_NIGHTS_REPLAY: C2RustUnnamed_10 = 15;
pub const MN_SP_NIGHTS_GUESTREPLAY: C2RustUnnamed_10 = 14;
pub const MN_SP_NIGHTS_LEVELSELECT: C2RustUnnamed_10 = 13;
pub const MN_SP_NIGHTSATTACK: C2RustUnnamed_10 = 12;
pub const MN_SP_GHOST: C2RustUnnamed_10 = 11;
pub const MN_SP_REPLAY: C2RustUnnamed_10 = 10;
pub const MN_SP_GUESTREPLAY: C2RustUnnamed_10 = 9;
pub const MN_SP_TIMEATTACK_LEVELSELECT: C2RustUnnamed_10 = 8;
pub const MN_SP_TIMEATTACK: C2RustUnnamed_10 = 7;
pub const MN_SP_LEVELSTATS: C2RustUnnamed_10 = 6;
pub const MN_SP_LEVELSELECT: C2RustUnnamed_10 = 5;
pub const MN_SP_PLAYER: C2RustUnnamed_10 = 4;
pub const MN_SP_LOAD: C2RustUnnamed_10 = 3;
pub const MN_SP_MAIN: C2RustUnnamed_10 = 2;
pub const MN_NONE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const NUMINPUTS: C2RustUnnamed_11 = 484;
pub const KEY_2MOUSEWHEELDOWN: C2RustUnnamed_11 = 483;
pub const KEY_2MOUSEWHEELUP: C2RustUnnamed_11 = 482;
pub const KEY_MOUSEWHEELDOWN: C2RustUnnamed_11 = 481;
pub const KEY_MOUSEWHEELUP: C2RustUnnamed_11 = 480;
pub const KEY_DBL2HAT1: C2RustUnnamed_11 = 464;
pub const KEY_DBL2JOY1: C2RustUnnamed_11 = 432;
pub const KEY_DBL2MOUSE1: C2RustUnnamed_11 = 424;
pub const KEY_2HAT1: C2RustUnnamed_11 = 408;
pub const KEY_2JOY1: C2RustUnnamed_11 = 376;
pub const KEY_2MOUSE1: C2RustUnnamed_11 = 368;
pub const KEY_DBLHAT1: C2RustUnnamed_11 = 352;
pub const KEY_DBLJOY1: C2RustUnnamed_11 = 320;
pub const KEY_DBLMOUSE1: C2RustUnnamed_11 = 312;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const num_gamecontrolschemes: C2RustUnnamed_12 = 3;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const hudhook_MAX: C2RustUnnamed_13 = 5;
pub const hudhook_intermission: C2RustUnnamed_13 = 4;
pub const hudhook_titlecard: C2RustUnnamed_13 = 3;
pub const hudhook_scores: C2RustUnnamed_13 = 1;
pub const hudhook_game: C2RustUnnamed_13 = 0;
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
#[inline(always)]
unsafe extern "C" fn FixedInt(mut a: fixed_t) -> fixed_t {
    return FixedMul(a, 1 as libc::c_int);
}
#[inline(always)]
unsafe extern "C" fn FixedDiv2(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return (a as int64_t * ((1 as libc::c_int) << 16 as libc::c_int) as int64_t
        / b as int64_t) as fixed_t;
}
#[inline(always)]
unsafe extern "C" fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return ((a as int64_t * b as int64_t) as uint64_t >> 16 as libc::c_int) as fixed_t;
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
#[no_mangle]
pub static mut finalecount: int32_t = 0;
#[no_mangle]
pub static mut titlescrollxspeed: int32_t = 20 as libc::c_int;
#[no_mangle]
pub static mut titlescrollyspeed: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut titlemapinaction: uint8_t = TITLEMAP_OFF as libc::c_int as uint8_t;
static mut timetonext: int32_t = 0;
static mut continuetime: int32_t = 0;
static mut animtimer: tic_t = 0;
static mut skullAnimCounter: int16_t = 0;
static mut deplete: int32_t = 0;
static mut stoptimer: tic_t = 0;
static mut keypressed: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut titlemapcameraref: *mut mobj_t = 0 as *const mobj_t as *mut mobj_t;
#[no_mangle]
pub static mut curbgname: [libc::c_char; 9] = [0; 9];
#[no_mangle]
pub static mut curfadevalue: int8_t = 0;
#[no_mangle]
pub static mut curbgcolor: int32_t = 0;
#[no_mangle]
pub static mut curbgxspeed: int32_t = 0;
#[no_mangle]
pub static mut curbgyspeed: int32_t = 0;
#[no_mangle]
pub static mut curbghide: boolean = 0;
#[no_mangle]
pub static mut hidetitlemap: boolean = 0;
static mut curbgx: fixed_t = 0 as libc::c_int;
static mut curbgy: fixed_t = 0 as libc::c_int;
static mut curDemo: uint8_t = 0 as libc::c_int as uint8_t;
static mut demoDelayLeft: uint32_t = 0;
static mut demoIdleLeft: uint32_t = 0;
#[no_mangle]
pub static mut ttmode: ttmode_enum = TTMODE_OLD;
#[no_mangle]
pub static mut ttscale: uint8_t = 1 as libc::c_int as uint8_t;
#[no_mangle]
pub static mut ttname: [libc::c_char; 9] = [0; 9];
#[no_mangle]
pub static mut ttx: int16_t = 0 as libc::c_int as int16_t;
#[no_mangle]
pub static mut tty: int16_t = 0 as libc::c_int as int16_t;
#[no_mangle]
pub static mut ttloop: int16_t = -(1 as libc::c_int) as int16_t;
#[no_mangle]
pub static mut tttics: uint16_t = 1 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut curhidepics: boolean = 0;
#[no_mangle]
pub static mut curttmode: ttmode_enum = TTMODE_NONE;
#[no_mangle]
pub static mut curttscale: uint8_t = 0;
#[no_mangle]
pub static mut curttname: [libc::c_char; 9] = [0; 9];
#[no_mangle]
pub static mut curttx: int16_t = 0;
#[no_mangle]
pub static mut curtty: int16_t = 0;
#[no_mangle]
pub static mut curttloop: int16_t = 0;
#[no_mangle]
pub static mut curtttics: uint16_t = 0;
static mut ttbanner: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttwing: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttsonic: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttswave1: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttswave2: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttswip1: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttsprep1: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttsprep2: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop1: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop2: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop3: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop4: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop5: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop6: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut ttspop7: *mut patch_t = 0 as *const patch_t as *mut patch_t;
static mut testttscale: int8_t = 0 as libc::c_int as int8_t;
static mut activettscale: int8_t = 0 as libc::c_int as int8_t;
#[no_mangle]
pub static mut ttavailable: [boolean; 6] = [0; 6];
#[no_mangle]
pub static mut ttloaded: [boolean; 6] = [0; 6];
static mut ttribb: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsont: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttrobo: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttwot: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttembl: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttrbtx: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsoib: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsoif: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsoba: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsobk: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttsodh: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttaib: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttaif: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttaba: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttabk: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttabt: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut tttaft: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttknib: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttknif: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttknba: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttknbk: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut ttkndh: [[*mut patch_t; 30]; 6] = [[0 as *const patch_t
    as *mut patch_t; 30]; 6];
static mut sonic_blink: boolean = false_0 as libc::c_int;
static mut sonic_blink_twice: boolean = false_0 as libc::c_int;
static mut sonic_blinked_already: boolean = false_0 as libc::c_int;
static mut sonic_idle_start: int32_t = 0 as libc::c_int;
static mut sonic_idle_end: int32_t = 0 as libc::c_int;
static mut tails_blink: boolean = false_0 as libc::c_int;
static mut tails_blink_twice: boolean = false_0 as libc::c_int;
static mut tails_blinked_already: boolean = false_0 as libc::c_int;
static mut tails_idle_start: int32_t = 0 as libc::c_int;
static mut tails_idle_end: int32_t = 0 as libc::c_int;
static mut knux_blink: boolean = false_0 as libc::c_int;
static mut knux_blink_twice: boolean = false_0 as libc::c_int;
static mut knux_blinked_already: boolean = false_0 as libc::c_int;
static mut knux_idle_start: int32_t = 0 as libc::c_int;
static mut knux_idle_end: int32_t = 0 as libc::c_int;
static mut ttuser: [*mut patch_t; 100] = [0 as *const patch_t as *mut patch_t; 100];
static mut ttuser_count: int32_t = 0 as libc::c_int;
static mut goodending: boolean = 0;
static mut endbrdr: [*mut patch_t; 2] = [0 as *const patch_t as *mut patch_t; 2];
static mut endbgsp: [*mut patch_t; 3] = [0 as *const patch_t as *mut patch_t; 3];
static mut endegrk: [*mut patch_t; 2] = [0 as *const patch_t as *mut patch_t; 2];
static mut endfwrk: [*mut patch_t; 3] = [0 as *const patch_t as *mut patch_t; 3];
static mut endspkl: [*mut patch_t; 3] = [0 as *const patch_t as *mut patch_t; 3];
static mut endglow: [*mut patch_t; 2] = [0 as *const patch_t as *mut patch_t; 2];
static mut endxpld: [*mut patch_t; 4] = [0 as *const patch_t as *mut patch_t; 4];
static mut endescp: [*mut patch_t; 5] = [0 as *const patch_t as *mut patch_t; 5];
static mut sparkloffs: [[int32_t; 2]; 3] = [[0; 2]; 3];
static mut sparklloop: int32_t = 0;
#[no_mangle]
pub static mut promptactive: boolean = false_0 as libc::c_int;
static mut promptmo: *mut mobj_t = 0 as *const mobj_t as *mut mobj_t;
static mut promptpostexectag: int16_t = 0;
static mut promptblockcontrols: boolean = 0;
static mut promptpagetext: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut callpromptnum: int32_t = 2147483647 as libc::c_int;
static mut callpagenum: int32_t = 2147483647 as libc::c_int;
static mut callplayer: int32_t = 2147483647 as libc::c_int;
static mut cutscene_basetext: *const libc::c_char = 0 as *const libc::c_char;
static mut cutscene_disptext: [libc::c_char; 1024] = [0; 1024];
static mut cutscene_baseptr: int32_t = 0 as libc::c_int;
static mut cutscene_writeptr: int32_t = 0 as libc::c_int;
static mut cutscene_textcount: int32_t = 0 as libc::c_int;
static mut cutscene_textspeed: int32_t = 0 as libc::c_int;
static mut cutscene_boostspeed: uint8_t = 0 as libc::c_int as uint8_t;
#[no_mangle]
pub static mut stjrintro: [libc::c_char; 9] = unsafe {
    *::core::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"STJRI000\0")
};
static mut luahuddrawlist_title: huddrawlist_h = 0 as *const huddrawlist_s
    as *mut huddrawlist_s;
unsafe extern "C" fn F_WriteText() -> uint8_t {
    let mut numtowrite: int32_t = 1 as libc::c_int;
    let mut c: *const libc::c_char = 0 as *const libc::c_char;
    if cutscene_boostspeed != 0 {
        numtowrite = 8 as libc::c_int;
    } else {
        cutscene_textcount -= 1;
        if cutscene_textcount >= 0 as libc::c_int {
            return 1 as libc::c_int as uint8_t;
        }
        if cutscene_textspeed < 7 as libc::c_int {
            numtowrite = 8 as libc::c_int - cutscene_textspeed;
        }
    }
    while numtowrite > 0 as libc::c_int {
        c = &*cutscene_basetext.offset(cutscene_baseptr as isize) as *const libc::c_char;
        if c.is_null() || *c == 0 || *c as libc::c_int == '#' as i32 {
            return 0 as libc::c_int as uint8_t;
        }
        if *c as uint8_t as libc::c_int >= 0xa0 as libc::c_int
            && *c as uint8_t as libc::c_int <= 0xaf as libc::c_int
        {
            cutscene_textspeed = *c as uint8_t as libc::c_int - 0xa0 as libc::c_int;
        } else if *c as uint8_t as libc::c_int >= 0xb0 as libc::c_int
            && *c as uint8_t as libc::c_int
                <= 0xb0 as libc::c_int + 35 as libc::c_int - 1 as libc::c_int
        {
            cutscene_textcount = *c as uint8_t as libc::c_int - 0xaf as libc::c_int;
            numtowrite = 0 as libc::c_int;
        } else {
            let fresh0 = cutscene_writeptr;
            cutscene_writeptr = cutscene_writeptr + 1;
            cutscene_disptext[fresh0 as usize] = *c;
            if (*c as uint8_t as libc::c_int) < 0x80 as libc::c_int {
                numtowrite -= 1;
                numtowrite;
            }
        }
        cutscene_baseptr += 1;
        cutscene_baseptr;
    }
    if cutscene_textcount < 0 as libc::c_int {
        cutscene_textcount = 0 as libc::c_int;
        if cutscene_textspeed > 7 as libc::c_int {
            cutscene_textcount = cutscene_textspeed - 7 as libc::c_int;
        }
    }
    return 1 as libc::c_int as uint8_t;
}
unsafe extern "C" fn F_NewCutscene(mut basetext: *const libc::c_char) {
    cutscene_basetext = basetext;
    memset(
        cutscene_disptext.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    );
    cutscene_baseptr = 0 as libc::c_int;
    cutscene_writeptr = cutscene_baseptr;
    cutscene_textspeed = 9 as libc::c_int;
    cutscene_textcount = 35 as libc::c_int / 2 as libc::c_int;
}
#[no_mangle]
pub static mut intro_scenenum: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut intro_curtime: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut introtext: [*const libc::c_char; 17] = [0 as *const libc::c_char; 17];
static mut introscenetime: [tic_t; 17] = [
    (5 as libc::c_int * 35 as libc::c_int) as tic_t,
    (11 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (15 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (14 as libc::c_int * 35 as libc::c_int) as tic_t,
    (18 as libc::c_int * 35 as libc::c_int) as tic_t,
    (19 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (19 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 4 as libc::c_int)
        as tic_t,
    (10 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (16 as libc::c_int * 35 as libc::c_int) as tic_t,
    (16 as libc::c_int * 35 as libc::c_int) as tic_t,
    (16 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (17 as libc::c_int * 35 as libc::c_int) as tic_t,
    (7 as libc::c_int * 35 as libc::c_int) as tic_t,
    (8 as libc::c_int * 35 as libc::c_int) as tic_t,
    (18 as libc::c_int * 35 as libc::c_int + 35 as libc::c_int / 2 as libc::c_int)
        as tic_t,
    (16 as libc::c_int * 35 as libc::c_int) as tic_t,
    (25 as libc::c_int * 35 as libc::c_int) as tic_t,
];
#[no_mangle]
pub unsafe extern "C" fn F_StartIntro() {
    S_StopMusic();
    S_StopSounds();
    if introtoplay != 0 {
        if (cutscenes[(introtoplay as libc::c_int - 1 as libc::c_int) as usize])
            .is_null()
        {
            D_StartTitle();
        } else {
            F_StartCustomCutscene(
                introtoplay as libc::c_int - 1 as libc::c_int,
                false_0 as libc::c_int,
                false_0 as libc::c_int,
                false_0 as libc::c_int,
            );
        }
        return;
    }
    introtext[0 as libc::c_int as usize] = b" #\0" as *const u8 as *const libc::c_char;
    introtext[1 as libc::c_int
        as usize] = b"Two months had passed since Dr. Eggman\ntried to take over the world using his\nRing Satellite.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[2 as libc::c_int
        as usize] = b"As it was about to drain the rings\naway from the planet, Sonic burst into\nthe control room and for what he thought\nwould be the last time,\xB4 defeated\nDr. Eggman.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[3 as libc::c_int
        as usize] = b"\nWhat Sonic, Tails, and Knuckles had\nnot anticipated was that Eggman would\nreturn,\xB8 bringing an all new threat.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[4 as libc::c_int
        as usize] = b"\xA8About every five years, a strange asteroid\nhovers around the planet.\xBF It suddenly\nappears from nowhere, circles around, and\n\xB6- just as mysteriously as it arrives -\xB6\nvanishes after only one week.\xBF\nNo one knows why it appears, or how.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[5 as libc::c_int
        as usize] = b"\xA7\"Curses!\"\xA9\xBA Eggman yelled. \xA7\"That hedgehog\nand his ridiculous friends will pay\ndearly for this!\"\xA9\xC8 Just then his scanner\nblipped as the Black Rock made its\nappearance from nowhere.\xBF Eggman looked at\nthe screen, and just shrugged it off.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[6 as libc::c_int
        as usize] = b"It was hours later\nthat he had an\nidea. \xBF\xA7\"The Black\nRock has a large\namount of energy\nwithin it\xAC...\xA7\xBF\nIf I can somehow\nharness this,\xB8 I\ncan turn it into\nthe ultimate\nbattle station\xAC...\xA7\xBF\nAnd every last\nperson will be\nbegging for mercy,\xB8\xA8\nincluding Sonic!\"\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[7 as libc::c_int
        as usize] = b"\xA8\nBefore beginning his scheme,\nEggman decided to give Sonic\na reunion party...\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[8 as libc::c_int
        as usize] = b"\xA5\"PRE-\xB6PARING-\xB6TO-\xB4FIRE-\xB6IN-\xB615-\xB6SECONDS!\"\xA8\xB8\nhis targeting system crackled\nrobotically down the com-link. \xBF\xA7\"Good!\"\xA8\xB8\nEggman sat back in his eggmobile and\nbegan to count down as he saw the\nGreenflower mountain on the monitor.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[9 as libc::c_int
        as usize] = b"\xA5\"10...\xD29...\xD28...\"\xA8\xD2\nMeanwhile, Sonic was tearing across the\nzones. Everything became a blur as he\nran up slopes, skimmed over water,\nand catapulted himself off rocks with\nhis phenomenal speed.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[10 as libc::c_int
        as usize] = b"\xA5\"6...\xD25...\xD24...\"\xA8\xD2\nSonic knew he was getting closer to the\nzone, and pushed himself harder.\xB4 Finally,\nthe mountain appeared on the horizon.\xD2\xD2\n\xA5\"3...\xD22...\xD21...\xD2Zero.\"\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[11 as libc::c_int
        as usize] = b"Greenflower Mountain was no more.\xC4\nSonic arrived just in time to see what\nlittle of the 'ruins' were left.\nThe natural beauty of the zone\nhad been obliterated.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[12 as libc::c_int
        as usize] = b"\xA7\"You're not\nquite as gone\nas we thought,\nhuh?\xBF Are you\ngoing to tell\nus your plan as\nusual or will I\n\xA8\xB4'have to work\nit out'\xA7 or\nsomething?\"\xD2\xD2\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[13 as libc::c_int
        as usize] = b"\"We'll see\xAA...\xA7\xBF let's give you a quick warm\nup, Sonic!\xA6\xC4 JETTYSYNS!\xA7\xBD Open fire!\"\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[14 as libc::c_int
        as usize] = b"Eggman took this\nas his cue and\nblasted off,\nleaving Sonic\nand Tails behind.\xB6\nTails looked at\nthe once-perfect\nmountainside\nwith a grim face\nand sighed.\xC6\n\xA7\"Now\xB6 what do we\ndo?\",\xA9 he asked.\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[15 as libc::c_int
        as usize] = b"\xA7\"Easy!\xBF We go\nfind Eggman\nand stop his\nlatest\ninsane plan.\xBF\nJust like\nwe've always\ndone,\xBA right?\xD2\n\n\xAE...\xA9\xD2\n\n\"Tails, what\n\xAA*ARE*\xA9 you\ndoing?\"\n#\0"
        as *const u8 as *const libc::c_char;
    introtext[16 as libc::c_int
        as usize] = b"\xA8\"I'm just finding what mission obje\xAC\xB1...\xBF\n\xA6a-\xB8ha!\xBF Here it is!\xA8\xBF This will only give us\nthe robot's primary objective.\xBF It says\xAC\xB1...\"\n\xD2\xA3\x83* LOCATE  AND  RETRIEVE:  CHAOS  EMERALDS *\xBF\n*  CLOSEST  LOCATION:  GREENFLOWER  ZONE  *\x80\n\xA9\xD2\xD2\"All right, then\xAF... \xD2\xD2\xA7let's go!\"\n#\0"
        as *const u8 as *const libc::c_char;
    G_SetGamestate(GS_INTRO);
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    F_NewCutscene(introtext[0 as libc::c_int as usize]);
    intro_scenenum = 0 as libc::c_int;
    stoptimer = 0 as libc::c_int as tic_t;
    skullAnimCounter = stoptimer as int16_t;
    animtimer = skullAnimCounter as tic_t;
    finalecount = animtimer as int32_t;
    timetonext = introscenetime[intro_scenenum as usize] as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn F_IntroDrawer() {
    let mut highres: boolean = true_0 as libc::c_int;
    let mut cx: int32_t = 8 as libc::c_int;
    let mut cy: int32_t = 128 as libc::c_int;
    let mut background: *mut patch_t = 0 as *mut patch_t;
    let mut bgxoffs: int32_t = 0 as libc::c_int;
    let mut patch: *mut libc::c_void = 0 as *mut libc::c_void;
    match intro_scenenum {
        0 => {
            bgxoffs = 28 as libc::c_int;
        }
        1 => {
            background = W_CachePatchName(
                b"INTRO1\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        2 => {
            background = W_CachePatchName(
                b"INTRO2\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        3 => {
            background = W_CachePatchName(
                b"INTRO3\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        4 => {
            background = W_CachePatchName(
                b"INTRO4\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        5 => {
            if intro_curtime >= 5 as libc::c_int * 35 as libc::c_int {
                background = W_CachePatchName(
                    b"RADAR\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"DRAT\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
        }
        6 => {
            background = W_CachePatchName(
                b"INTRO6\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            cx = 180 as libc::c_int;
            cy = 8 as libc::c_int;
        }
        7 => {
            if intro_curtime
                >= 7 as libc::c_int * 35 as libc::c_int
                    + 35 as libc::c_int / 7 as libc::c_int * 2 as libc::c_int
            {
                background = W_CachePatchName(
                    b"SGRASS5\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else if intro_curtime
                >= 7 as libc::c_int * 35 as libc::c_int
                    + 35 as libc::c_int / 7 as libc::c_int
            {
                background = W_CachePatchName(
                    b"SGRASS4\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else if intro_curtime >= 7 as libc::c_int * 35 as libc::c_int {
                background = W_CachePatchName(
                    b"SGRASS3\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else if intro_curtime >= 6 as libc::c_int * 35 as libc::c_int {
                background = W_CachePatchName(
                    b"SGRASS2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"SGRASS1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
        }
        8 => {
            background = W_CachePatchName(
                b"WATCHING\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        9 => {
            background = W_CachePatchName(
                b"ZOOMING\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        11 => {
            background = W_CachePatchName(
                b"INTRO5\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        12 => {
            background = W_CachePatchName(
                b"REVENGE\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            cx = 208 as libc::c_int;
            cy = 8 as libc::c_int;
        }
        13 => {
            background = W_CachePatchName(
                b"CONFRONT\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            cy += 48 as libc::c_int;
        }
        14 => {
            background = W_CachePatchName(
                b"TAILSSAD\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            bgxoffs = 144 as libc::c_int;
            cx = 8 as libc::c_int;
            cy = 8 as libc::c_int;
        }
        15 => {
            if intro_curtime >= 7 as libc::c_int * 35 as libc::c_int {
                background = W_CachePatchName(
                    b"SONICDO2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"SONICDO1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
            cx = 224 as libc::c_int;
            cy = 8 as libc::c_int;
        }
        16 => {
            background = W_CachePatchName(
                b"INTRO7\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        10 | _ => {}
    }
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    if !background.is_null() {
        if highres != 0 {
            V_DrawStretchyFixedPatch(
                bgxoffs << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                background,
                0 as *const uint8_t,
            );
        } else {
            V_DrawStretchyFixedPatch(
                bgxoffs * ((1 as libc::c_int) << 16 as libc::c_int),
                (0 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                background,
                0 as *const uint8_t,
            );
        }
    } else if intro_scenenum == 0 as libc::c_int {
        if intro_curtime > 1 as libc::c_int
            && intro_curtime < introscenetime[intro_scenenum as usize] as int32_t
        {
            V_DrawFill(
                0 as libc::c_int,
                0 as libc::c_int,
                320 as libc::c_int,
                200 as libc::c_int,
                31 as libc::c_int,
            );
            if intro_curtime < 35 as libc::c_int - 5 as libc::c_int {
                sprintf(
                    stjrintro.as_mut_ptr(),
                    b"STJRI%03u\0" as *const u8 as *const libc::c_char,
                    intro_curtime - 1 as libc::c_int,
                );
            } else if intro_curtime >= 35 as libc::c_int - 6 as libc::c_int
                && intro_curtime
                    < 2 as libc::c_int * 35 as libc::c_int - 20 as libc::c_int
            {
                return
            } else if intro_curtime
                == 2 as libc::c_int * 35 as libc::c_int - 19 as libc::c_int
            {
                strncpy(
                    stjrintro.as_mut_ptr(),
                    b"STJRI029\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as libc::c_ulong,
                );
                background = W_CachePatchName(
                    stjrintro.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                V_DrawStretchyFixedPatch(
                    bgxoffs << 16 as libc::c_int,
                    (84 as libc::c_int) << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    background,
                    0 as *const uint8_t,
                );
            }
            if WipeInAction == 0 {
                background = W_CachePatchName(
                    stjrintro.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                V_DrawStretchyFixedPatch(
                    bgxoffs << 16 as libc::c_int,
                    (84 as libc::c_int) << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    background,
                    0 as *const uint8_t,
                );
            }
        }
    } else if intro_scenenum == 10 as libc::c_int {
        if timetonext > 5 as libc::c_int * 35 as libc::c_int
            && timetonext < 6 as libc::c_int * 35 as libc::c_int
        {
            if finalecount & 3 as libc::c_int == 0 {
                background = W_CachePatchName(
                    b"BRITEGG1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"DARKEGG1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
            V_DrawStretchyFixedPatch(
                (0 as libc::c_int) << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                background,
                0 as *const uint8_t,
            );
        } else if timetonext > 3 as libc::c_int * 35 as libc::c_int
            && timetonext < 4 as libc::c_int * 35 as libc::c_int
        {
            if finalecount & 3 as libc::c_int == 0 {
                background = W_CachePatchName(
                    b"BRITEGG2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"DARKEGG2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
            V_DrawStretchyFixedPatch(
                (0 as libc::c_int) << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                background,
                0 as *const uint8_t,
            );
        } else if timetonext > 1 as libc::c_int * 35 as libc::c_int
            && timetonext < 2 as libc::c_int * 35 as libc::c_int
        {
            if finalecount & 3 as libc::c_int == 0 {
                background = W_CachePatchName(
                    b"BRITEGG3\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            } else {
                background = W_CachePatchName(
                    b"DARKEGG3\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
            }
            V_DrawStretchyFixedPatch(
                (0 as libc::c_int) << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                background,
                0 as *const uint8_t,
            );
        } else {
            let mut sonicdelay: tic_t = (if 0 as libc::c_int
                > timetonext - 16 as libc::c_int * 35 as libc::c_int
            {
                0 as libc::c_int
            } else {
                timetonext - 16 as libc::c_int * 35 as libc::c_int
            }) as tic_t;
            let mut tailsdelay: tic_t = (if 0 as libc::c_int
                > timetonext - (9 as libc::c_int * 35 as libc::c_int >> 1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                timetonext - (9 as libc::c_int * 35 as libc::c_int >> 1 as libc::c_int)
            }) as tic_t;
            let mut knucklesdelay: tic_t = (if 0 as libc::c_int
                > timetonext - (5 as libc::c_int * 35 as libc::c_int >> 1 as libc::c_int)
            {
                0 as libc::c_int
            } else {
                timetonext - (5 as libc::c_int * 35 as libc::c_int >> 1 as libc::c_int)
            }) as tic_t;
            let mut sonicx: int32_t = ((timetonext >> 2 as libc::c_int) as tic_t)
                .wrapping_add(
                    (if sonicdelay < (35 as libc::c_int >> 1 as libc::c_int) as tic_t {
                        sonicdelay
                    } else {
                        (35 as libc::c_int >> 1 as libc::c_int) as tic_t
                    }) * sonicdelay,
                ) as int32_t;
            let mut tailsx: int32_t = (32 as libc::c_int as tic_t)
                .wrapping_add(
                    (if tailsdelay < (35 as libc::c_int >> 1 as libc::c_int) as tic_t {
                        tailsdelay
                    } else {
                        (35 as libc::c_int >> 1 as libc::c_int) as tic_t
                    }) * tailsdelay,
                ) as int32_t;
            let mut knucklesx: int32_t = (96 as libc::c_int as tic_t)
                .wrapping_add(
                    (if knucklesdelay < (35 as libc::c_int >> 1 as libc::c_int) as tic_t
                    {
                        knucklesdelay
                    } else {
                        (35 as libc::c_int >> 1 as libc::c_int) as tic_t
                    }) * knucklesdelay,
                ) as int32_t;
            let mut tailsy: int32_t = 12 as libc::c_int
                + P_ReturnThrustX(
                    0 as *mut mobj_t,
                    (finalecount * 0x10000000 as libc::c_int) as angle_t,
                    2 as libc::c_int,
                );
            let mut knucklesy: int32_t = 48 as libc::c_int
                - (timetonext >> 3 as libc::c_int);
            let mut skyx: int32_t = 0;
            let mut grassx: int32_t = 0;
            if timetonext >= 0 as libc::c_int && timetonext < 18 as libc::c_int {
                deplete -= 16 as libc::c_int;
            } else {
                stoptimer = finalecount as tic_t;
                deplete = 96 as libc::c_int;
            }
            skyx = (2 as libc::c_int as tic_t * stoptimer % 320 as libc::c_int as tic_t)
                as int32_t;
            grassx = (16 as libc::c_int as tic_t * stoptimer
                % 320 as libc::c_int as tic_t) as int32_t;
            sonicx += deplete;
            tailsx += sonicx;
            knucklesx += sonicx;
            sonicx
                += P_ReturnThrustX(
                    0 as *mut mobj_t,
                    (finalecount * 0x71c71c7 as libc::c_int) as angle_t,
                    3 as libc::c_int,
                );
            patch = W_CachePatchName(
                b"INTROSKY\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            );
            V_DrawStretchyFixedPatch(
                skyx << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                patch as *mut patch_t,
                0 as *const uint8_t,
            );
            V_DrawStretchyFixedPatch(
                (skyx - 320 as libc::c_int) << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                patch as *mut patch_t,
                0 as *const uint8_t,
            );
            W_UnlockCachedPatch(patch);
            patch = W_CachePatchName(
                b"INTROGRS\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            );
            V_DrawStretchyFixedPatch(
                grassx << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                patch as *mut patch_t,
                0 as *const uint8_t,
            );
            V_DrawStretchyFixedPatch(
                (grassx - 320 as libc::c_int) << 16 as libc::c_int,
                (0 as libc::c_int) << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                patch as *mut patch_t,
                0 as *const uint8_t,
            );
            W_UnlockCachedPatch(patch);
            if finalecount & 1 as libc::c_int != 0 {
                patch = W_CachePatchName(
                    b"RUN2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    sonicx << 16 as libc::c_int,
                    (54 as libc::c_int) << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
                if finalecount & 2 as libc::c_int != 0 {
                    patch = W_CachePatchName(
                        b"PEELOUT4\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        (sonicx - 8 as libc::c_int) << 16 as libc::c_int,
                        (92 as libc::c_int) << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                    patch = W_CachePatchName(
                        b"HELICOP2\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        tailsx << 16 as libc::c_int,
                        tailsy << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                } else {
                    patch = W_CachePatchName(
                        b"PEELOUT2\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        (sonicx - 8 as libc::c_int) << 16 as libc::c_int,
                        (92 as libc::c_int) << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                    patch = W_CachePatchName(
                        b"HELICOP1\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        tailsx << 16 as libc::c_int,
                        tailsy << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                }
                patch = W_CachePatchName(
                    b"FLY2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    tailsx << 16 as libc::c_int,
                    tailsy << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
                patch = W_CachePatchName(
                    b"GLIDE2\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    knucklesx << 16 as libc::c_int,
                    knucklesy << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
            } else {
                patch = W_CachePatchName(
                    b"RUN1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    sonicx << 16 as libc::c_int,
                    (54 as libc::c_int) << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
                if finalecount & 2 as libc::c_int != 0 {
                    patch = W_CachePatchName(
                        b"PEELOUT3\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        (sonicx - 8 as libc::c_int) << 16 as libc::c_int,
                        (92 as libc::c_int) << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                    patch = W_CachePatchName(
                        b"HELICOP2\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        tailsx << 16 as libc::c_int,
                        tailsy << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                } else {
                    patch = W_CachePatchName(
                        b"PEELOUT1\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        (sonicx - 8 as libc::c_int) << 16 as libc::c_int,
                        (92 as libc::c_int) << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                    patch = W_CachePatchName(
                        b"HELICOP1\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    );
                    V_DrawStretchyFixedPatch(
                        tailsx << 16 as libc::c_int,
                        tailsy << 16 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                        0 as libc::c_int,
                        patch as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    W_UnlockCachedPatch(patch);
                }
                patch = W_CachePatchName(
                    b"FLY1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    tailsx << 16 as libc::c_int,
                    tailsy << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
                patch = W_CachePatchName(
                    b"GLIDE1\0" as *const u8 as *const libc::c_char,
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                );
                V_DrawStretchyFixedPatch(
                    knucklesx << 16 as libc::c_int,
                    knucklesy << 16 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                    0 as libc::c_int,
                    patch as *mut patch_t,
                    0 as *const uint8_t,
                );
                W_UnlockCachedPatch(patch);
            }
            V_DrawFill(
                -(80 as libc::c_int),
                0 as libc::c_int,
                80 as libc::c_int,
                256 as libc::c_int,
                31 as libc::c_int,
            );
            V_DrawFill(
                320 as libc::c_int,
                0 as libc::c_int,
                80 as libc::c_int,
                256 as libc::c_int,
                31 as libc::c_int,
            );
        }
    }
    W_UnlockCachedPatch(background as *mut libc::c_void);
    if intro_scenenum == 4 as libc::c_int {
        if intro_curtime > 1 as libc::c_int {
            let mut worktics: int32_t = intro_curtime - 1 as libc::c_int;
            let mut scale: int32_t = (1 as libc::c_int) << 16 as libc::c_int;
            let mut rockpat: *mut patch_t = 0 as *mut patch_t;
            let mut colormap: *mut uint8_t = 0 as *mut uint8_t;
            let mut glow: *mut patch_t = 0 as *mut patch_t;
            let mut trans: int32_t = 0 as libc::c_int;
            let mut x: int32_t = ((320 as libc::c_int - 64 as libc::c_int)
                << 16 as libc::c_int)
                - intro_curtime * ((1 as libc::c_int) << 16 as libc::c_int)
                    / 3 as libc::c_int;
            let mut y: int32_t = (24 as libc::c_int) << 16 as libc::c_int;
            if worktics < 5 as libc::c_int {
                scale = worktics << 16 as libc::c_int - 2 as libc::c_int;
                x
                    += 30 as libc::c_int
                        * (((1 as libc::c_int) << 16 as libc::c_int) - scale);
                y
                    += 30 as libc::c_int
                        * (((1 as libc::c_int) << 16 as libc::c_int) - scale);
            }
            rockpat = W_CachePatchName(
                va(
                    b"ROID00%.2d\0" as *const u8 as *const libc::c_char,
                    34 as libc::c_int - worktics % 35 as libc::c_int,
                ),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            glow = W_CachePatchName(
                va(
                    b"ENDGLOW%.1d\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + (worktics & 1 as libc::c_int),
                ),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            if worktics >= 5 as libc::c_int {
                trans = worktics - 5 as libc::c_int >> 1 as libc::c_int;
            }
            if trans < 10 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    x,
                    y,
                    scale,
                    scale,
                    trans << 16 as libc::c_int,
                    glow,
                    0 as *const uint8_t,
                );
            }
            trans = 15 as libc::c_int - worktics;
            if trans < 0 as libc::c_int {
                trans = -trans;
            }
            if finalecount < 15 as libc::c_int {
                colormap = R_GetTranslationColormap(
                    TC_ALLWHITE as libc::c_int,
                    SKINCOLOR_NONE,
                    1 as libc::c_int as uint8_t,
                );
            }
            V_DrawStretchyFixedPatch(
                x,
                y,
                scale,
                scale,
                0 as libc::c_int,
                rockpat,
                colormap,
            );
            if trans < 10 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    x,
                    y,
                    scale,
                    scale,
                    trans << 16 as libc::c_int,
                    rockpat,
                    R_GetTranslationColormap(
                        TC_BLINK as libc::c_int,
                        SKINCOLOR_AQUA,
                        1 as libc::c_int as uint8_t,
                    ),
                );
            }
        }
    } else if intro_scenenum == 1 as libc::c_int
        && intro_curtime < 5 as libc::c_int * 35 as libc::c_int
    {
        let mut trans_0: int32_t = intro_curtime + 10 as libc::c_int
            - 5 as libc::c_int * 35 as libc::c_int;
        if trans_0 < 0 as libc::c_int {
            trans_0 = 0 as libc::c_int;
        }
        V_DrawRightAlignedString(
            320 as libc::c_int - 4 as libc::c_int,
            200 as libc::c_int - 12 as libc::c_int,
            0x800000 as libc::c_int | trans_0 << 16 as libc::c_int,
            b"\x86Press \x82ENTER\x86 to skip...\0" as *const u8 as *const libc::c_char,
        );
    }
    V_DrawString(cx, cy, 0x800000 as libc::c_int, cutscene_disptext.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn F_IntroTicker() {
    finalecount += 1;
    finalecount;
    timetonext -= 1;
    timetonext;
    F_WriteText();
    if keypressed != 0 {
        keypressed = false_0 as libc::c_int;
    }
    wipestyleflags = WSF_CROSSFADE;
    if timetonext <= 0 as libc::c_int {
        if intro_scenenum == 0 as libc::c_int {
            if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
                wipestyleflags = WSF_FADEOUT;
                F_WipeStartScreen();
                F_TryColormapFade(31 as libc::c_int as uint8_t);
                F_IntroDrawer();
                F_WipeEndScreen();
                F_RunWipe(99 as libc::c_int as uint8_t, true_0 as libc::c_int);
            }
            S_ChangeMusicEx(
                b"_intro\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint16_t,
                false_0 as libc::c_int,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            );
        } else if intro_scenenum == 10 as libc::c_int {
            if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
                wipestyleflags = (WSF_FADEOUT as libc::c_int
                    | WSF_TOWHITE as libc::c_int) as wipestyleflags_t;
                F_WipeStartScreen();
                F_TryColormapFade(0 as libc::c_int as uint8_t);
                F_IntroDrawer();
                F_WipeEndScreen();
                F_RunWipe(99 as libc::c_int as uint8_t, true_0 as libc::c_int);
            }
        } else if intro_scenenum == 16 as libc::c_int {
            if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
                wipestyleflags = WSF_FADEOUT;
                F_WipeStartScreen();
                F_TryColormapFade(31 as libc::c_int as uint8_t);
                F_IntroDrawer();
                F_WipeEndScreen();
                F_RunWipe(99 as libc::c_int as uint8_t, true_0 as libc::c_int);
            }
            let mut nowtime: tic_t = 0;
            let mut quittime: tic_t = 0;
            let mut lasttime: tic_t = 0;
            lasttime = I_GetTime();
            nowtime = lasttime;
            quittime = nowtime
                .wrapping_add(
                    (35 as libc::c_int * 1 as libc::c_int * 2 as libc::c_int) as tic_t,
                );
            while quittime > nowtime {
                loop {
                    nowtime = I_GetTime();
                    if !(nowtime.wrapping_sub(lasttime) == 0) {
                        break;
                    }
                    I_Sleep(cv_sleep.value as uint32_t);
                    I_UpdateTime(cv_timescale.value);
                }
                lasttime = nowtime;
                I_OsPolling();
                I_UpdateNoBlit();
                M_Drawer();
                I_FinishUpdate();
                if moviemode as u64 != 0 {
                    M_SaveFrame();
                }
            }
            D_StartTitle();
            wipegamestate = GS_INTRO;
            return;
        }
        intro_scenenum += 1;
        F_NewCutscene(introtext[intro_scenenum as usize]);
        timetonext = introscenetime[intro_scenenum as usize] as int32_t;
        F_WipeStartScreen();
        wipegamestate = 4294967295 as gamestate_t;
        stoptimer = 0 as libc::c_int as tic_t;
        animtimer = stoptimer;
    }
    intro_curtime = (introscenetime[intro_scenenum as usize])
        .wrapping_sub(timetonext as tic_t) as int32_t;
    if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
        if intro_scenenum == 0 as libc::c_int
            && intro_curtime == 2 as libc::c_int * 35 as libc::c_int - 19 as libc::c_int
        {
            S_ChangeMusicEx(
                b"_stjr\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as uint16_t,
                false_0 as libc::c_int,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            );
            wipestyleflags = WSF_FADEIN;
            F_WipeStartScreen();
            F_TryColormapFade(31 as libc::c_int as uint8_t);
            F_IntroDrawer();
            F_WipeEndScreen();
            F_RunWipe(99 as libc::c_int as uint8_t, true_0 as libc::c_int);
        } else if intro_scenenum == 5 as libc::c_int
            && intro_curtime == 5 as libc::c_int * 35 as libc::c_int
            || intro_scenenum == 7 as libc::c_int
                && intro_curtime == 6 as libc::c_int * 35 as libc::c_int
            || intro_scenenum == 15 as libc::c_int
                && intro_curtime == 7 as libc::c_int * 35 as libc::c_int
        {
            F_WipeStartScreen();
            V_DrawFill(
                0 as libc::c_int,
                0 as libc::c_int,
                320 as libc::c_int,
                200 as libc::c_int,
                31 as libc::c_int,
            );
            F_IntroDrawer();
            F_WipeEndScreen();
            F_RunWipe(99 as libc::c_int as uint8_t, true_0 as libc::c_int);
        }
    }
    if animtimer != 0 {
        animtimer = animtimer.wrapping_sub(1);
        animtimer;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_IntroResponder(mut event: *mut event_t) -> boolean {
    let mut key: int32_t = (*event).key;
    match key {
        256 => {
            key = 13 as libc::c_int;
        }
        257 => {
            key = 8 as libc::c_int;
        }
        264 | 266 => {
            key = 13 as libc::c_int;
        }
        267 => {
            key = 'n' as i32;
        }
        265 => {
            key = 8 as libc::c_int;
        }
        296 => {
            key = 0x80 as libc::c_int + 102 as libc::c_int;
        }
        297 => {
            key = 0x80 as libc::c_int + 110 as libc::c_int;
        }
        298 => {
            key = 0x80 as libc::c_int + 105 as libc::c_int;
        }
        299 => {
            key = 0x80 as libc::c_int + 107 as libc::c_int;
        }
        _ => {}
    }
    if (*event).type_0 as libc::c_uint != ev_keydown as libc::c_int as libc::c_uint
        && key != 301 as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    if key != 27 as libc::c_int && key != 13 as libc::c_int && key != 32 as libc::c_int
        && key != 8 as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    if keypressed != 0 {
        return false_0 as libc::c_int;
    }
    keypressed = true_0 as libc::c_int;
    return true_0 as libc::c_int;
}
static mut credits: [*const libc::c_char; 177] = [
    b"\x01Sonic Robo Blast 2\0" as *const u8 as *const libc::c_char,
    b"\x01Credits\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Game Design\0" as *const u8 as *const libc::c_char,
    b"Sonic Team Junior\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"Johnny \"Sonikku\" Wallbank\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Programming\0" as *const u8 as *const libc::c_char,
    b"\"altaluna\"\0" as *const u8 as *const libc::c_char,
    b"Alam \"GBC\" Arias\0" as *const u8 as *const libc::c_char,
    b"Logan \"GBA\" Arias\0" as *const u8 as *const libc::c_char,
    b"Zolton \"Zippy_Zolton\" Auburn\0" as *const u8 as *const libc::c_char,
    b"Colette \"fickleheart\" Bordelon\0" as *const u8 as *const libc::c_char,
    b"Andrew \"orospakr\" Clunis\0" as *const u8 as *const libc::c_char,
    b"Sally \"TehRealSalt\" Cochenour\0" as *const u8 as *const libc::c_char,
    b"Gregor \"Oogaland\" Dick\0" as *const u8 as *const libc::c_char,
    b"Callum Dickinson\0" as *const u8 as *const libc::c_char,
    b"Scott \"Graue\" Feeney\0" as *const u8 as *const libc::c_char,
    b"Victor \"SteelT\" Fuentes\0" as *const u8 as *const libc::c_char,
    b"Nathan \"Jazz\" Giroux\0" as *const u8 as *const libc::c_char,
    b"\"Golden\"\0" as *const u8 as *const libc::c_char,
    b"Vivian \"toaster\" Grannell\0" as *const u8 as *const libc::c_char,
    b"Julio \"Chaos Zero 64\" Guir\0" as *const u8 as *const libc::c_char,
    b"\"Hanicef\"\0" as *const u8 as *const libc::c_char,
    b"\"Hannu_Hanhi\"\0" as *const u8 as *const libc::c_char,
    b"Kepa \"Nev3r\" Iceta\0" as *const u8 as *const libc::c_char,
    b"Thomas \"Shadow Hog\" Igoe\0" as *const u8 as *const libc::c_char,
    b"Iestyn \"Monster Iestyn\" Jealous\0" as *const u8 as *const libc::c_char,
    b"\"Kaito Sinclaire\"\0" as *const u8 as *const libc::c_char,
    b"\"Kalaron\"\0" as *const u8 as *const libc::c_char,
    b"\"katsy\"\0" as *const u8 as *const libc::c_char,
    b"Ronald \"Furyhunter\" Kinard\0" as *const u8 as *const libc::c_char,
    b"\"Lat'\"\0" as *const u8 as *const libc::c_char,
    b"\"LZA\"\0" as *const u8 as *const libc::c_char,
    b"Matthew \"Shuffle\" Marsalko\0" as *const u8 as *const libc::c_char,
    b"Steven \"StroggOnMeth\" McGranahan\0" as *const u8 as *const libc::c_char,
    b"\"Morph\"\0" as *const u8 as *const libc::c_char,
    b"Louis-Antoine \"LJ Sonic\" de Moulins\0" as *const u8 as *const libc::c_char,
    b"John \"JTE\" Muniz\0" as *const u8 as *const libc::c_char,
    b"Colin \"Sonict\" Pfaff\0" as *const u8 as *const libc::c_char,
    b"James \"james\" Robert Roman\0" as *const u8 as *const libc::c_char,
    b"Sean \"Sryder13\" Ryder\0" as *const u8 as *const libc::c_char,
    b"Ehab \"Wolfy\" Saeed\0" as *const u8 as *const libc::c_char,
    b"Tasos \"tatokis\" Sahanidis\0" as *const u8 as *const libc::c_char,
    b"Riku \"Ors\" Salminen\0" as *const u8 as *const libc::c_char,
    b"Jonas \"MascaraSnake\" Sauer\0" as *const u8 as *const libc::c_char,
    b"Wessel \"sphere\" Smit\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"\"VelocitOni\"\0" as *const u8 as *const libc::c_char,
    b"Ikaro \"Tatsuru\" Vinhas\0" as *const u8 as *const libc::c_char,
    b"Ben \"Cue\" Woodford\0" as *const u8 as *const libc::c_char,
    b"Lachlan \"Lach\" Wright\0" as *const u8 as *const libc::c_char,
    b"Marco \"mazmazz\" Zafra\0" as *const u8 as *const libc::c_char,
    b"\"Zwip-Zwap Zapony\"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Art\0" as *const u8 as *const libc::c_char,
    b"Victor \"VAdaPEGA\" Ara\x1Fjo\0" as *const u8 as *const libc::c_char,
    b"\"Arrietty\"\0" as *const u8 as *const libc::c_char,
    b"Ryan \"Blaze Hedgehog\" Bloom\0" as *const u8 as *const libc::c_char,
    b"Graeme P. \"SuperPhanto\" Caldwell\0" as *const u8 as *const libc::c_char,
    b"\"ChrispyPixels\"\0" as *const u8 as *const libc::c_char,
    b"Paul \"Boinciel\" Clempson\0" as *const u8 as *const libc::c_char,
    b"Sally \"TehRealSalt\" Cochenour\0" as *const u8 as *const libc::c_char,
    b"\"Dave Lite\"\0" as *const u8 as *const libc::c_char,
    b"Desmond \"Blade\" DesJardins\0" as *const u8 as *const libc::c_char,
    b"Sherman \"CoatRack\" DesJardins\0" as *const u8 as *const libc::c_char,
    b"\"DirkTheHusky\"\0" as *const u8 as *const libc::c_char,
    b"Jesse \"Jeck Jims\" Emerick\0" as *const u8 as *const libc::c_char,
    b"\"Fighter_Builder\"\0" as *const u8 as *const libc::c_char,
    b"Buddy \"KinkaJoy\" Fischer\0" as *const u8 as *const libc::c_char,
    b"Vivian \"toaster\" Grannell\0" as *const u8 as *const libc::c_char,
    b"James \"SwitchKaze\" Hale\0" as *const u8 as *const libc::c_char,
    b"James \"SeventhSentinel\" Hall\0" as *const u8 as *const libc::c_char,
    b"Kepa \"Nev3r\" Iceta\0" as *const u8 as *const libc::c_char,
    b"Iestyn \"Monster Iestyn\" Jealous\0" as *const u8 as *const libc::c_char,
    b"William \"GuyWithThePie\" Kloppenberg\0" as *const u8 as *const libc::c_char,
    b"Alice \"Alacroix\" de Lemos\0" as *const u8 as *const libc::c_char,
    b"Logan \"Hyperchaotix\" McCloud\0" as *const u8 as *const libc::c_char,
    b"Alexander \"DrTapeworm\" Moench-Ford\0" as *const u8 as *const libc::c_char,
    b"Andrew \"Senku Niola\" Moran\0" as *const u8 as *const libc::c_char,
    b"\"MotorRoach\"\0" as *const u8 as *const libc::c_char,
    b"Phillip \"TelosTurntable\" Robinson\0" as *const u8 as *const libc::c_char,
    b"\"Scizor300\"\0" as *const u8 as *const libc::c_char,
    b"Wessel \"sphere\" Smit\0" as *const u8 as *const libc::c_char,
    b"David \"Instant Sonic\" Spencer Jr.\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"Daniel \"Inazuma\" Trinh\0" as *const u8 as *const libc::c_char,
    b"\"VelocitOni\"\0" as *const u8 as *const libc::c_char,
    b"Jarrett \"JEV3\" Voight\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Music and Sound\0" as *const u8 as *const libc::c_char,
    b"\x01Production\0" as *const u8 as *const libc::c_char,
    b"Victor \"VAdaPEGA\" Ara\x1Fjo\0" as *const u8 as *const libc::c_char,
    b"Malcolm \"RedXVI\" Brown\0" as *const u8 as *const libc::c_char,
    b"Dave \"DemonTomatoDave\" Bulmer\0" as *const u8 as *const libc::c_char,
    b"Paul \"Boinciel\" Clempson\0" as *const u8 as *const libc::c_char,
    b"\"Cyan Helkaraxe\"\0" as *const u8 as *const libc::c_char,
    b"Claire \"clairebun\" Ellis\0" as *const u8 as *const libc::c_char,
    b"James \"SeventhSentinel\" Hall\0" as *const u8 as *const libc::c_char,
    b"Kepa \"Nev3r\" Iceta\0" as *const u8 as *const libc::c_char,
    b"Iestyn \"Monster Iestyn\" Jealous\0" as *const u8 as *const libc::c_char,
    b"Jarel \"Arrow\" Jones\0" as *const u8 as *const libc::c_char,
    b"Alexander \"DrTapeworm\" Moench-Ford\0" as *const u8 as *const libc::c_char,
    b"Stefan \"Stuf\" Rimalia\0" as *const u8 as *const libc::c_char,
    b"Shane Mychal Sexton\0" as *const u8 as *const libc::c_char,
    b"Dave \"Big Wave Dave\" Spencer\0" as *const u8 as *const libc::c_char,
    b"David \"instantSonic\" Spencer\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Level Design\0" as *const u8 as *const libc::c_char,
    b"Colette \"fickleheart\" Bordelon\0" as *const u8 as *const libc::c_char,
    b"Hank \"FuriousFox\" Brannock\0" as *const u8 as *const libc::c_char,
    b"Matthew \"Fawfulfan\" Chapman\0" as *const u8 as *const libc::c_char,
    b"Paul \"Boinciel\" Clempson\0" as *const u8 as *const libc::c_char,
    b"Sally \"TehRealSalt\" Cochenour\0" as *const u8 as *const libc::c_char,
    b"Desmond \"Blade\" DesJardins\0" as *const u8 as *const libc::c_char,
    b"Sherman \"CoatRack\" DesJardins\0" as *const u8 as *const libc::c_char,
    b"Ben \"Mystic\" Geyer\0" as *const u8 as *const libc::c_char,
    b"Nathan \"Jazz\" Giroux\0" as *const u8 as *const libc::c_char,
    b"Vivian \"toaster\" Grannell\0" as *const u8 as *const libc::c_char,
    b"James \"SeventhSentinel\" Hall\0" as *const u8 as *const libc::c_char,
    b"Kepa \"Nev3r\" Iceta\0" as *const u8 as *const libc::c_char,
    b"Thomas \"Shadow Hog\" Igoe\0" as *const u8 as *const libc::c_char,
    b"\"Kaito Sinclaire\"\0" as *const u8 as *const libc::c_char,
    b"Alexander \"DrTapeworm\" Moench-Ford\0" as *const u8 as *const libc::c_char,
    b"\"Revan\"\0" as *const u8 as *const libc::c_char,
    b"Anna \"QueenDelta\" Sandlin\0" as *const u8 as *const libc::c_char,
    b"Wessel \"sphere\" Smit\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"Rob Tisdell\0" as *const u8 as *const libc::c_char,
    b"\"Torgo\"\0" as *const u8 as *const libc::c_char,
    b"Jarrett \"JEV3\" Voight\0" as *const u8 as *const libc::c_char,
    b"Johnny \"Sonikku\" Wallbank\0" as *const u8 as *const libc::c_char,
    b"Marco \"mazmazz\" Zafra\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Boss Design\0" as *const u8 as *const libc::c_char,
    b"Ben \"Mystic\" Geyer\0" as *const u8 as *const libc::c_char,
    b"Vivian \"toaster\" Grannell\0" as *const u8 as *const libc::c_char,
    b"Thomas \"Shadow Hog\" Igoe\0" as *const u8 as *const libc::c_char,
    b"John \"JTE\" Muniz\0" as *const u8 as *const libc::c_char,
    b"Samuel \"Prime 2.0\" Peters\0" as *const u8 as *const libc::c_char,
    b"\"SSNTails\"\0" as *const u8 as *const libc::c_char,
    b"Johnny \"Sonikku\" Wallbank\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Testing\0" as *const u8 as *const libc::c_char,
    b"Discord Community Testers\0" as *const u8 as *const libc::c_char,
    b"Hank \"FuriousFox\" Brannock\0" as *const u8 as *const libc::c_char,
    b"Cody \"Playah\" Koester\0" as *const u8 as *const libc::c_char,
    b"Skye \"OmegaVelocity\" Meredith\0" as *const u8 as *const libc::c_char,
    b"Stephen \"HEDGESMFG\" Moellering\0" as *const u8 as *const libc::c_char,
    b"Rosalie \"ST218\" Molina\0" as *const u8 as *const libc::c_char,
    b"Samuel \"Prime 2.0\" Peters\0" as *const u8 as *const libc::c_char,
    b"Colin \"Sonict\" Pfaff\0" as *const u8 as *const libc::c_char,
    b"Bill \"Tets\" Reed\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Special Thanks\0" as *const u8 as *const libc::c_char,
    b"id Software\0" as *const u8 as *const libc::c_char,
    b"Doom Legacy Project\0" as *const u8 as *const libc::c_char,
    b"FreeDoom Project\0" as *const u8 as *const libc::c_char,
    b"Kart Krew\0" as *const u8 as *const libc::c_char,
    b"Alex \"MistaED\" Fuller\0" as *const u8 as *const libc::c_char,
    b"Howard Drossin\0" as *const u8 as *const libc::c_char,
    b"Pascal \"CodeImp\" vd Heiden\0" as *const u8 as *const libc::c_char,
    b"Randi Heit (<!>)\0" as *const u8 as *const libc::c_char,
    b"Simon \"sirjuddington\" Judd\0" as *const u8 as *const libc::c_char,
    b"SRB2 Community Contributors\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Produced By\0" as *const u8 as *const libc::c_char,
    b"Sonic Team Junior\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Published By\0" as *const u8 as *const libc::c_char,
    b"A 28K dialup modem\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\x01Thank you       \0" as *const u8 as *const libc::c_char,
    b"\x01for playing!       \0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut credits_pics: [C2RustUnnamed_5; 13] = [
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT01\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (271 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT02\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT03\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (316 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT04\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT05\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (399 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT06\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT07\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (302 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT08\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT09\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (250 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 8 as libc::c_int as uint32_t,
            patch: b"CREDIT11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: (320 as libc::c_int - 8 as libc::c_int
                - (279 as libc::c_int >> 1 as libc::c_int)) as uint32_t,
            patch: b"CREDIT12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            x: 0 as libc::c_int as uint32_t,
            patch: 0 as *const libc::c_char,
        };
        init
    },
];
static mut credits_height: uint32_t = 0 as libc::c_int as uint32_t;
static mut credits_numpics: uint8_t = 0;
#[no_mangle]
pub unsafe extern "C" fn F_StartCredits() {
    G_SetGamestate(GS_CREDITS);
    M_ClearMenus(true_0 as libc::c_int);
    if creditscutscene != 0 {
        F_StartCustomCutscene(
            creditscutscene as libc::c_int - 1 as libc::c_int,
            false_0 as libc::c_int,
            false_0 as libc::c_int,
            false_0 as libc::c_int,
        );
        return;
    }
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    S_StopMusic();
    S_StopSounds();
    S_ChangeMusicEx(
        b"_creds\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as uint16_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
    );
    finalecount = 0 as libc::c_int;
    animtimer = 0 as libc::c_int as tic_t;
    timetonext = 2 as libc::c_int * 35 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_CreditDrawer() {
    let mut i: uint16_t = 0;
    let mut zagpos: int16_t = (((timetonext - finalecount) as tic_t)
        .wrapping_sub(animtimer) % 32 as libc::c_int as tic_t) as int16_t;
    let mut y: fixed_t = (((80 as libc::c_int) << 16 as libc::c_int) as tic_t)
        .wrapping_sub(animtimer << 16 as libc::c_int >> 1 as libc::c_int) as fixed_t;
    let mut colornum: uint8_t = 0;
    let mut colormap: *const uint8_t = 0 as *const uint8_t;
    if players[consoleplayer as usize].skincolor != 0 {
        colornum = players[consoleplayer as usize].skincolor as uint8_t;
    } else {
        colornum = cv_playercolor.value as uint8_t;
    }
    colormap = R_GetTranslationColormap(
        TC_DEFAULT as libc::c_int,
        colornum as skincolornum_t,
        1 as libc::c_int as uint8_t,
    );
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    V_DrawStretchyFixedPatch(
        -(16 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
        (zagpos as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0x4000000 as libc::c_int,
        W_CachePatchName(
            b"LTZIGZAG\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t,
        colormap,
    );
    V_DrawStretchyFixedPatch(
        -(16 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
        (zagpos as libc::c_int - 320 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0x4000000 as libc::c_int,
        W_CachePatchName(
            b"LTZIGZAG\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t,
        colormap,
    );
    V_DrawStretchyFixedPatch(
        (320 as libc::c_int + 16 as libc::c_int)
            * ((1 as libc::c_int) << 16 as libc::c_int),
        (zagpos as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0x8000000 as libc::c_int | 0x800000 as libc::c_int,
        W_CachePatchName(
            b"LTZIGZAG\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t,
        colormap,
    );
    V_DrawStretchyFixedPatch(
        (320 as libc::c_int + 16 as libc::c_int)
            * ((1 as libc::c_int) << 16 as libc::c_int),
        (zagpos as libc::c_int - 320 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0x8000000 as libc::c_int | 0x800000 as libc::c_int,
        W_CachePatchName(
            b"LTZIGZAG\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t,
        colormap,
    );
    i = 0 as libc::c_int as uint16_t;
    while !(credits_pics[i as usize].patch).is_null() {
        V_DrawStretchyFixedPatch(
            (credits_pics[i as usize].x << 16 as libc::c_int) as fixed_t,
            (((280 as libc::c_int) << 16 as libc::c_int) as uint32_t)
                .wrapping_add(
                    (i as uint32_t * credits_height << 16 as libc::c_int)
                        / credits_numpics as uint32_t,
                )
                .wrapping_sub(
                    4 as libc::c_int as tic_t * (animtimer << 16 as libc::c_int)
                        / 5 as libc::c_int as tic_t,
                ) as fixed_t,
            (1 as libc::c_int) << 16 as libc::c_int >> 1 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int >> 1 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchName(
                credits_pics[i as usize].patch,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t,
            0 as *const uint8_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    V_DrawFadeScreen(0xff00 as libc::c_int as uint16_t, 16 as libc::c_int as uint8_t);
    i = 0 as libc::c_int as uint16_t;
    while !(credits[i as usize]).is_null() {
        match *(credits[i as usize]).offset(0 as libc::c_int as isize) as libc::c_int {
            0 => {
                y += (80 as libc::c_int) << 16 as libc::c_int;
            }
            1 => {
                if y >> 16 as libc::c_int > -(20 as libc::c_int) {
                    V_DrawCreditString(
                        160 as libc::c_int
                            - (V_CreditStringWidth(
                                &*(*credits.as_mut_ptr().offset(i as isize))
                                    .offset(1 as libc::c_int as isize),
                            ) >> 1 as libc::c_int) << 16 as libc::c_int,
                        y,
                        0 as libc::c_int,
                        &*(*credits.as_mut_ptr().offset(i as isize))
                            .offset(1 as libc::c_int as isize),
                    );
                }
                y += (30 as libc::c_int) << 16 as libc::c_int;
            }
            2 => {
                if y >> 16 as libc::c_int > -(10 as libc::c_int) {
                    V_DrawStringAtFixed(
                        320 as libc::c_int
                            - V_StringWidth(
                                &*(*credits.as_mut_ptr().offset(i as isize))
                                    .offset(1 as libc::c_int as isize),
                                0x800000 as libc::c_int | 0x2000 as libc::c_int,
                            ) << 16 as libc::c_int >> 1 as libc::c_int,
                        y,
                        0x800000 as libc::c_int | 0x2000 as libc::c_int,
                        &*(*credits.as_mut_ptr().offset(i as isize))
                            .offset(1 as libc::c_int as isize),
                    );
                }
                y += (12 as libc::c_int) << 16 as libc::c_int;
            }
            _ => {
                if y >> 16 as libc::c_int > -(10 as libc::c_int) {
                    V_DrawStringAtFixed(
                        (32 as libc::c_int) << 16 as libc::c_int,
                        y,
                        0x800000 as libc::c_int,
                        credits[i as usize],
                    );
                }
                y += (12 as libc::c_int) << 16 as libc::c_int;
            }
        }
        if FixedMul(y, vid.dupy) > vid.height {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_CreditTicker() {
    let mut i: uint16_t = 0;
    let mut y: fixed_t = (((80 as libc::c_int) << 16 as libc::c_int) as tic_t)
        .wrapping_sub(animtimer << 16 as libc::c_int >> 1 as libc::c_int) as fixed_t;
    if credits_height == 0 as libc::c_int as uint32_t {
        i = 0 as libc::c_int as uint16_t;
        while !(credits[i as usize]).is_null() {
            match *(credits[i as usize]).offset(0 as libc::c_int as isize) as libc::c_int
            {
                0 => {
                    credits_height = credits_height
                        .wrapping_add(80 as libc::c_int as uint32_t);
                }
                1 => {
                    credits_height = credits_height
                        .wrapping_add(30 as libc::c_int as uint32_t);
                }
                _ => {
                    credits_height = credits_height
                        .wrapping_add(12 as libc::c_int as uint32_t);
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        credits_height = 131 as libc::c_int as uint32_t * credits_height
            / 80 as libc::c_int as uint32_t;
    }
    i = 0 as libc::c_int as uint16_t;
    while !(credits[i as usize]).is_null() {
        match *(credits[i as usize]).offset(0 as libc::c_int as isize) as libc::c_int {
            0 => {
                y += (80 as libc::c_int) << 16 as libc::c_int;
            }
            1 => {
                y += (30 as libc::c_int) << 16 as libc::c_int;
            }
            _ => {
                y += (12 as libc::c_int) << 16 as libc::c_int;
            }
        }
        if FixedMul(y, vid.dupy) > vid.height {
            break;
        }
        i = i.wrapping_add(1);
        i;
    }
    if (credits[i as usize]).is_null() && y <= (120 as libc::c_int) << 16 as libc::c_int
        && finalecount == 0
    {
        timetonext = 5 as libc::c_int * 35 as libc::c_int + 1 as libc::c_int;
        finalecount = 5 as libc::c_int * 35 as libc::c_int;
    }
    if timetonext != 0 {
        timetonext -= 1;
        timetonext;
    } else {
        animtimer = animtimer.wrapping_add(1);
        animtimer;
    }
    if finalecount != 0
        && {
            finalecount -= 1;
            finalecount == 0 as libc::c_int
        }
    {
        F_StartGameEvaluation();
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_CreditResponder(mut event: *mut event_t) -> boolean {
    let mut key: int32_t = (*event).key;
    match key {
        256 => {
            key = 13 as libc::c_int;
        }
        257 => {
            key = 8 as libc::c_int;
        }
        264 | 266 => {
            key = 13 as libc::c_int;
        }
        267 => {
            key = 'n' as i32;
        }
        265 => {
            key = 8 as libc::c_int;
        }
        296 => {
            key = 0x80 as libc::c_int + 102 as libc::c_int;
        }
        297 => {
            key = 0x80 as libc::c_int + 110 as libc::c_int;
        }
        298 => {
            key = 0x80 as libc::c_int + 105 as libc::c_int;
        }
        299 => {
            key = 0x80 as libc::c_int + 107 as libc::c_int;
        }
        _ => {}
    }
    if (*serverGamedata).timesBeaten == 0 && !(netgame != 0 || multiplayer != 0)
        && cv_debug == 0
    {
        return false_0 as libc::c_int;
    }
    if (*event).type_0 as libc::c_uint != ev_keydown as libc::c_int as libc::c_uint {
        return false_0 as libc::c_int;
    }
    if key != 27 as libc::c_int && key != 13 as libc::c_int && key != 32 as libc::c_int
        && key != 8 as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    if keypressed != 0 {
        return true_0 as libc::c_int;
    }
    keypressed = true_0 as libc::c_int;
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_StartGameEvaluation() {
    if cursaveslot == -(1 as libc::c_int) {
        S_FadeOutStopMusic((2 as libc::c_int * 1000 as libc::c_int) as uint32_t);
        F_StartGameEnd();
        return;
    }
    S_FadeOutStopMusic((5 as libc::c_int * 1000 as libc::c_int) as uint32_t);
    G_SetGamestate(GS_EVALUATION);
    M_ClearMenus(true_0 as libc::c_int);
    goodending = (emeralds as libc::c_int
        & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
        == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int) as libc::c_int;
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    finalecount = -(1 as libc::c_int);
    sparklloop = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_GameEvaluationDrawer() {
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut i: int32_t = 0;
    let mut fa: angle_t = 0;
    let mut eemeralds_cur: int32_t = 0;
    let mut patchname: [libc::c_char; 7] = *::core::mem::transmute::<
        &[u8; 7],
        &mut [libc::c_char; 7],
    >(b"CEMGx0\0");
    let mut endingtext: *const libc::c_char = 0 as *const libc::c_char;
    if marathonmode as u64 != 0 {
        endingtext = b"THANKS FOR THE RUN!\0" as *const u8 as *const libc::c_char;
    } else if goodending != 0 {
        endingtext = b"CONGRATULATIONS!\0" as *const u8 as *const libc::c_char;
    } else {
        endingtext = b"TRY AGAIN...\0" as *const u8 as *const libc::c_char;
    }
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    if finalecount > 0 as libc::c_int && useBlackRock as libc::c_int != 0 {
        let mut scale: int32_t = (1 as libc::c_int) << 16 as libc::c_int;
        let mut rockpat: *mut patch_t = 0 as *mut patch_t;
        let mut colormap: [*mut uint8_t; 2] = [0 as *mut uint8_t, 0 as *mut uint8_t];
        let mut glow: *mut patch_t = 0 as *mut patch_t;
        let mut trans: int32_t = 0 as libc::c_int;
        x = ((320 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int
            + 11 as libc::c_int) << 16 as libc::c_int;
        y = ((200 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int
            + 12 as libc::c_int) << 16 as libc::c_int;
        if finalecount < 5 as libc::c_int {
            scale = finalecount << 16 as libc::c_int - 2 as libc::c_int;
            x += 30 as libc::c_int * (((1 as libc::c_int) << 16 as libc::c_int) - scale);
            y += 30 as libc::c_int * (((1 as libc::c_int) << 16 as libc::c_int) - scale);
        }
        if goodending != 0 {
            rockpat = W_CachePatchName(
                va(
                    b"ROID00%.2d\0" as *const u8 as *const libc::c_char,
                    34 as libc::c_int - finalecount % 35 as libc::c_int,
                ),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            glow = W_CachePatchName(
                va(
                    b"ENDGLOW%.1d\0" as *const u8 as *const libc::c_char,
                    2 as libc::c_int + (finalecount & 1 as libc::c_int),
                ),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            x -= (1 as libc::c_int) << 16 as libc::c_int;
        } else {
            rockpat = W_CachePatchName(
                b"ROID0000\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            glow = W_CachePatchName(
                va(
                    b"ENDGLOW%.1d\0" as *const u8 as *const libc::c_char,
                    finalecount & 1 as libc::c_int,
                ),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        if finalecount >= 5 as libc::c_int {
            trans = finalecount - 5 as libc::c_int >> 1 as libc::c_int;
        }
        if trans < 10 as libc::c_int {
            V_DrawStretchyFixedPatch(
                x,
                y,
                scale,
                scale,
                trans << 16 as libc::c_int,
                glow,
                0 as *const uint8_t,
            );
        }
        trans = 15 as libc::c_int - finalecount;
        if trans < 0 as libc::c_int {
            trans = -trans;
        }
        if finalecount < 15 as libc::c_int {
            colormap[0 as libc::c_int
                as usize] = R_GetTranslationColormap(
                TC_ALLWHITE as libc::c_int,
                SKINCOLOR_NONE,
                1 as libc::c_int as uint8_t,
            );
        }
        V_DrawStretchyFixedPatch(
            x,
            y,
            scale,
            scale,
            0 as libc::c_int,
            rockpat,
            colormap[0 as libc::c_int as usize],
        );
        if trans < 10 as libc::c_int {
            colormap[1 as libc::c_int
                as usize] = R_GetTranslationColormap(
                TC_BLINK as libc::c_int,
                SKINCOLOR_AQUA,
                1 as libc::c_int as uint8_t,
            );
            V_DrawStretchyFixedPatch(
                x,
                y,
                scale,
                scale,
                trans << 16 as libc::c_int,
                rockpat,
                colormap[1 as libc::c_int as usize],
            );
        }
        if goodending != 0 {
            let mut j: int32_t = if sparklloop & 1 as libc::c_int != 0 {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            };
            if j > finalecount / 7 as libc::c_int {
                j = finalecount / 7 as libc::c_int;
            }
            while j != 0 {
                if j > 1 as libc::c_int || sparklloop >= 2 as libc::c_int {
                    V_DrawStretchyFixedPatch(
                        x
                            + sparkloffs[(j - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                        y
                            + sparkloffs[(j - 1 as libc::c_int)
                                as usize][1 as libc::c_int as usize],
                        (1 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        0 as libc::c_int,
                        W_CachePatchName(
                            va(
                                b"ENDSPKL%.1d\0" as *const u8 as *const libc::c_char,
                                j
                                    - (if sparklloop & 1 as libc::c_int != 0 {
                                        0 as libc::c_int
                                    } else {
                                        1 as libc::c_int
                                    }),
                            ),
                            PU_PATCH_LOWPRIORITY as libc::c_int,
                        ) as *mut patch_t,
                        R_GetTranslationColormap(
                            TC_DEFAULT as libc::c_int,
                            SKINCOLOR_AQUA,
                            1 as libc::c_int as uint8_t,
                        ),
                    );
                }
                j -= 1;
                j;
            }
        } else {
            let mut eggrock: *mut patch_t = W_CachePatchName(
                b"ENDEGRK5\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            V_DrawStretchyFixedPatch(
                x,
                y,
                scale,
                scale,
                0 as libc::c_int,
                eggrock,
                colormap[0 as libc::c_int as usize],
            );
            if trans < 10 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    x,
                    y,
                    scale,
                    scale,
                    trans << 16 as libc::c_int,
                    eggrock,
                    colormap[1 as libc::c_int as usize],
                );
            } else if sparklloop != 0 {
                V_DrawStretchyFixedPatch(
                    x,
                    y,
                    scale,
                    scale,
                    10 as libc::c_int - sparklloop << 16 as libc::c_int,
                    W_CachePatchName(
                        b"ENDEGRK0\0" as *const u8 as *const libc::c_char,
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    ) as *mut patch_t,
                    colormap[1 as libc::c_int as usize],
                );
            }
        }
    }
    eemeralds_cur = (finalecount % 360 as libc::c_int) << 16 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        fa = FixedAngle(eemeralds_cur) >> 19 as libc::c_int
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        x = ((320 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + 60 as libc::c_int
                * (*finecosine.offset(fa as isize)
                    >> 16 as libc::c_int - 16 as libc::c_int);
        y = ((200 as libc::c_int + 16 as libc::c_int)
            << 16 as libc::c_int - 1 as libc::c_int)
            + 60 as libc::c_int
                * (finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int);
        eemeralds_cur += ((360 as libc::c_int) << 16 as libc::c_int) / 7 as libc::c_int;
        patchname[4 as libc::c_int
            as usize] = ('A' as i32 + i as libc::c_char as libc::c_int) as libc::c_char;
        V_DrawStretchyFixedPatch(
            x,
            y,
            (1 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            if emeralds as libc::c_int & (1 as libc::c_int) << i != 0 {
                0 as libc::c_int
            } else {
                0x80000 as libc::c_int
            },
            W_CachePatchName(patchname.as_mut_ptr(), PU_PATCH_LOWPRIORITY as libc::c_int)
                as *mut patch_t,
            0 as *const uint8_t,
        );
        i += 1;
        i;
    }
    V_DrawCreditString(
        320 as libc::c_int - V_CreditStringWidth(endingtext)
            << 16 as libc::c_int - 1 as libc::c_int,
        (200 as libc::c_int - 100 as libc::c_int)
            << 16 as libc::c_int - 1 as libc::c_int,
        0 as libc::c_int,
        endingtext,
    );
    if marathonmode as u64 != 0 {
        let mut rtatext: *const libc::c_char = 0 as *const libc::c_char;
        let mut cuttext: *const libc::c_char = 0 as *const libc::c_char;
        rtatext = if marathonmode as libc::c_uint
            & MA_INGAME as libc::c_int as libc::c_uint != 0
        {
            b"In-game timer\0" as *const u8 as *const libc::c_char
        } else {
            b"RTA timer\0" as *const u8 as *const libc::c_char
        };
        cuttext = if marathonmode as libc::c_uint
            & MA_NOCUTSCENES as libc::c_int as libc::c_uint != 0
        {
            b"\0" as *const u8 as *const libc::c_char
        } else {
            b" w/ cutscenes\0" as *const u8 as *const libc::c_char
        };
        if botskin != 0 {
            endingtext = va(
                b"%s & %s, %s%s\0" as *const u8 as *const libc::c_char,
                (skins[players[consoleplayer as usize].skin as usize].realname)
                    .as_mut_ptr(),
                (skins[(botskin as libc::c_int - 1 as libc::c_int) as usize].realname)
                    .as_mut_ptr(),
                rtatext,
                cuttext,
            );
        } else {
            endingtext = va(
                b"%s, %s%s\0" as *const u8 as *const libc::c_char,
                (skins[players[consoleplayer as usize].skin as usize].realname)
                    .as_mut_ptr(),
                rtatext,
                cuttext,
            );
        }
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            182 as libc::c_int,
            0x2000000 as libc::c_int
                | (if ultimatemode as libc::c_int != 0 {
                    0x5000 as libc::c_int
                } else {
                    0x2000 as libc::c_int
                }),
            endingtext,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_GameEvaluationTicker() {
    finalecount += 1;
    if finalecount > 10 as libc::c_int * 35 as libc::c_int {
        F_StartGameEnd();
        return;
    }
    if !(useBlackRock == 0) {
        if goodending == 0 {
            if sparklloop != 0 {
                sparklloop -= 1;
                sparklloop;
            }
            if finalecount == 5 as libc::c_int * 35 as libc::c_int / 2 as libc::c_int
                || finalecount == 7 as libc::c_int * 35 as libc::c_int / 2 as libc::c_int
                || finalecount
                    == 7 as libc::c_int * 35 as libc::c_int / 2 as libc::c_int
                        + 5 as libc::c_int
            {
                S_StartSound(0 as *const libc::c_void, sfx_s3k5c);
                sparklloop = 10 as libc::c_int;
            }
        } else {
            sparklloop += 1;
            if sparklloop == 7 as libc::c_int {
                let mut workingangle: angle_t = FixedAngle(
                    M_RandomKey(360 as libc::c_int) << 16 as libc::c_int,
                ) >> 19 as libc::c_int;
                let mut workingradius: fixed_t = M_RandomKey(26 as libc::c_int);
                sparkloffs[2 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize] = sparkloffs[1 as libc::c_int
                    as usize][0 as libc::c_int as usize];
                sparkloffs[2 as libc::c_int
                    as usize][1 as libc::c_int
                    as usize] = sparkloffs[1 as libc::c_int
                    as usize][1 as libc::c_int as usize];
                sparkloffs[1 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize] = sparkloffs[0 as libc::c_int
                    as usize][0 as libc::c_int as usize];
                sparkloffs[1 as libc::c_int
                    as usize][1 as libc::c_int
                    as usize] = sparkloffs[0 as libc::c_int
                    as usize][1 as libc::c_int as usize];
                sparkloffs[0 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize] = ((30 as libc::c_int) << 16 as libc::c_int)
                    + workingradius
                        * (*finecosine.offset(workingangle as isize)
                            >> 16 as libc::c_int - 16 as libc::c_int);
                sparkloffs[0 as libc::c_int
                    as usize][1 as libc::c_int
                    as usize] = ((30 as libc::c_int) << 16 as libc::c_int)
                    + workingradius
                        * (finesine[workingangle as usize]
                            >> 16 as libc::c_int - 16 as libc::c_int);
                sparklloop = 0 as libc::c_int;
            }
        }
    }
    if G_CoopGametype() != 0 && stagefailed == 0
        && finalecount == 5 as libc::c_int * 35 as libc::c_int
    {
        (*serverGamedata).timesBeaten = ((*serverGamedata).timesBeaten).wrapping_add(1);
        (*serverGamedata).timesBeaten;
        (*clientGamedata).timesBeaten = ((*clientGamedata).timesBeaten).wrapping_add(1);
        (*clientGamedata).timesBeaten;
        if emeralds as libc::c_int
            & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
            == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int
        {
            (*serverGamedata)
                .timesBeatenWithEmeralds = ((*serverGamedata).timesBeatenWithEmeralds)
                .wrapping_add(1);
            (*serverGamedata).timesBeatenWithEmeralds;
            (*clientGamedata)
                .timesBeatenWithEmeralds = ((*clientGamedata).timesBeatenWithEmeralds)
                .wrapping_add(1);
            (*clientGamedata).timesBeatenWithEmeralds;
        }
        if ultimatemode != 0 {
            (*serverGamedata)
                .timesBeatenUltimate = ((*serverGamedata).timesBeatenUltimate)
                .wrapping_add(1);
            (*serverGamedata).timesBeatenUltimate;
            (*clientGamedata)
                .timesBeatenUltimate = ((*clientGamedata).timesBeatenUltimate)
                .wrapping_add(1);
            (*clientGamedata).timesBeatenUltimate;
        }
        M_SilentUpdateUnlockablesAndEmblems(serverGamedata);
        if M_UpdateUnlockablesAndExtraEmblems(clientGamedata) != 0 {
            S_StartSound(0 as *const libc::c_void, sfx_s3k68);
        }
        G_SaveGameData(clientGamedata);
    }
}
unsafe extern "C" fn F_CacheEnding() {
    endbrdr[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDBRDR1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endegrk[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDEGRK0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endegrk[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDEGRK1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endglow[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDGLOW0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endglow[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDGLOW1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endbgsp[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDBGSP0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endbgsp[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDBGSP1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endbgsp[2 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDBGSP2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endspkl[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDSPKL0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endspkl[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDSPKL1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endspkl[2 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDSPKL2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endxpld[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDXPLD0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endxpld[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDXPLD1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endxpld[2 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDXPLD2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endxpld[3 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDXPLD3\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endescp[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDESCP0\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endescp[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDESCP1\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endescp[2 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDESCP2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endescp[3 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDESCP3\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endescp[4 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDESCP4\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    goodending = (emeralds as libc::c_int
        & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
        == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int) as libc::c_int;
    if goodending != 0 {
        let mut skinnum: uint8_t = players[consoleplayer as usize].skin as uint8_t;
        let mut sprdef: *mut spritedef_t = 0 as *mut spritedef_t;
        let mut sprframe: *mut spriteframe_t = 0 as *mut spriteframe_t;
        if skins[skinnum as usize].sprites[SPR2_XTRA as libc::c_int as usize].numframes
            > (3 as libc::c_int + 2 as libc::c_int) as size_t
        {
            sprdef = &mut *((*skins.as_mut_ptr().offset(skinnum as isize)).sprites)
                .as_mut_ptr()
                .offset(SPR2_XTRA as libc::c_int as isize) as *mut spritedef_t;
            sprframe = &mut *((*sprdef).spriteframes).offset(3 as libc::c_int as isize)
                as *mut spriteframe_t;
            endfwrk[0 as libc::c_int
                as usize] = W_CachePatchNum(
                (*sprframe).lumppat[0 as libc::c_int as usize],
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            sprframe = &mut *((*sprdef).spriteframes)
                .offset((3 as libc::c_int + 1 as libc::c_int) as isize)
                as *mut spriteframe_t;
            endfwrk[1 as libc::c_int
                as usize] = W_CachePatchNum(
                (*sprframe).lumppat[0 as libc::c_int as usize],
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            sprframe = &mut *((*sprdef).spriteframes)
                .offset((3 as libc::c_int + 2 as libc::c_int) as isize)
                as *mut spriteframe_t;
            endfwrk[2 as libc::c_int
                as usize] = W_CachePatchNum(
                (*sprframe).lumppat[0 as libc::c_int as usize],
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        } else {
            endfwrk[0 as libc::c_int
                as usize] = W_CachePatchName(
                b"ENDFWRK3\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            endfwrk[1 as libc::c_int
                as usize] = W_CachePatchName(
                b"ENDFWRK4\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            endfwrk[2 as libc::c_int
                as usize] = W_CachePatchName(
                b"ENDFWRK5\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        endbrdr[0 as libc::c_int
            as usize] = W_CachePatchName(
            b"ENDBRDR2\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
    } else {
        endfwrk[0 as libc::c_int
            as usize] = W_CachePatchName(
            b"ENDFWRK0\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        endfwrk[1 as libc::c_int
            as usize] = W_CachePatchName(
            b"ENDFWRK1\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        endfwrk[2 as libc::c_int
            as usize] = W_CachePatchName(
            b"ENDFWRK2\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        endbrdr[0 as libc::c_int
            as usize] = W_CachePatchName(
            b"ENDBRDR0\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
    };
}
unsafe extern "C" fn F_CacheGoodEnding() {
    endegrk[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDEGRK2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endegrk[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDEGRK3\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endglow[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDGLOW2\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endglow[1 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDGLOW3\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    endxpld[0 as libc::c_int
        as usize] = W_CachePatchName(
        b"ENDEGRK4\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
}
#[no_mangle]
pub unsafe extern "C" fn F_StartEnding() {
    G_SetGamestate(GS_ENDING);
    wipetypepost = 32767 as libc::c_int as int16_t;
    M_ClearMenus(true_0 as libc::c_int);
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    S_StopMusic();
    S_StopSounds();
    finalecount = -(10 as libc::c_int);
    memset(
        sparkloffs.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    );
    sparklloop = 0 as libc::c_int;
    F_CacheEnding();
}
#[no_mangle]
pub unsafe extern "C" fn F_EndingTicker() {
    finalecount += 1;
    if finalecount > 14 as libc::c_int * 35 as libc::c_int {
        F_StartCredits();
        wipetypepre = 32767 as libc::c_int as int16_t;
        return;
    }
    if finalecount == -(8 as libc::c_int) {
        S_ChangeMusicEx(
            if goodending != 0 {
                b"_endg\0" as *const u8 as *const libc::c_char
            } else {
                b"_endb\0" as *const u8 as *const libc::c_char
            },
            0 as libc::c_int as uint16_t,
            false_0 as libc::c_int,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
    if goodending != 0 && finalecount == 6 as libc::c_int * 35 as libc::c_int {
        F_CacheGoodEnding();
    }
    sparklloop += 1;
    if sparklloop == 15 as libc::c_int {
        let mut workingangle: angle_t = FixedAngle(
            M_RandomRange(-(170 as libc::c_int), 80 as libc::c_int) << 16 as libc::c_int,
        ) >> 19 as libc::c_int;
        let mut workingradius: fixed_t = M_RandomKey(26 as libc::c_int);
        sparkloffs[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = ((30 as libc::c_int) << 16 as libc::c_int)
            + workingradius
                * (*finecosine.offset(workingangle as isize)
                    >> 16 as libc::c_int - 16 as libc::c_int);
        sparkloffs[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = ((30 as libc::c_int) << 16 as libc::c_int)
            + workingradius
                * (finesine[workingangle as usize]
                    >> 16 as libc::c_int - 16 as libc::c_int);
        sparklloop = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_EndingDrawer() {
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut parallaxticker: int32_t = 0;
    let mut rockpat: *mut patch_t = 0 as *mut patch_t;
    if goodending == 0 || finalecount < 6 as libc::c_int * 35 as libc::c_int {
        rockpat = W_CachePatchName(
            b"ROID0000\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
    } else {
        rockpat = W_CachePatchName(
            va(
                b"ROID00%.2d\0" as *const u8 as *const libc::c_char,
                34 as libc::c_int
                    - (finalecount - 6 as libc::c_int * 35 as libc::c_int)
                        % 35 as libc::c_int,
            ),
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
    }
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    parallaxticker = finalecount - 6 as libc::c_int * 35 as libc::c_int;
    x = -((parallaxticker * 20 as libc::c_int) << 16 as libc::c_int)
        / (6 as libc::c_int * 35 as libc::c_int);
    y = ((parallaxticker * 7 as libc::c_int) << 16 as libc::c_int)
        / (6 as libc::c_int * 35 as libc::c_int);
    i = ((320 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int + 11 as libc::c_int)
        << 16 as libc::c_int;
    j = ((200 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int + 12 as libc::c_int)
        << 16 as libc::c_int;
    if !(finalecount <= -(10 as libc::c_int)) {
        if finalecount < 0 as libc::c_int {
            V_DrawFadeFill(
                24 as libc::c_int,
                24 as libc::c_int,
                320 as libc::c_int - 48 as libc::c_int,
                200 as libc::c_int - 48 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int as uint16_t,
                (10 as libc::c_int + finalecount) as uint8_t,
            );
        } else if finalecount <= 20 as libc::c_int {
            V_DrawFill(
                24 as libc::c_int,
                24 as libc::c_int,
                320 as libc::c_int - 48 as libc::c_int,
                200 as libc::c_int - 48 as libc::c_int,
                0 as libc::c_int,
            );
            if finalecount != 0 && finalecount < 20 as libc::c_int {
                let mut trans: int32_t = 10 as libc::c_int - finalecount;
                if trans < 0 as libc::c_int {
                    trans = -trans;
                    V_DrawStretchyFixedPatch(
                        320 as libc::c_int / 2 as libc::c_int
                            * ((1 as libc::c_int) << 16 as libc::c_int),
                        (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        0 as libc::c_int,
                        endbrdr[0 as libc::c_int as usize],
                        0 as *const uint8_t,
                    );
                }
                V_DrawStretchyFixedPatch(
                    320 as libc::c_int / 2 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int),
                    (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    trans << 16 as libc::c_int,
                    endbrdr[1 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
            } else if finalecount == 20 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    320 as libc::c_int / 2 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int),
                    (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endbrdr[0 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
            }
        } else if goodending != 0
            && (parallaxticker == -(2 as libc::c_int) || parallaxticker == 0)
        {
            V_DrawFill(
                24 as libc::c_int,
                24 as libc::c_int,
                320 as libc::c_int - 48 as libc::c_int,
                200 as libc::c_int - 48 as libc::c_int,
                0 as libc::c_int,
            );
            V_DrawStretchyFixedPatch(
                x + i,
                y + j,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                endegrk[0 as libc::c_int as usize],
                R_GetTranslationColormap(
                    TC_BLINK as libc::c_int,
                    SKINCOLOR_BLACK,
                    1 as libc::c_int as uint8_t,
                ),
            );
        } else if goodending != 0 && parallaxticker == -(1 as libc::c_int) {
            V_DrawStretchyFixedPatch(
                x + i,
                y + j,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                rockpat,
                R_GetTranslationColormap(
                    TC_ALLWHITE as libc::c_int,
                    SKINCOLOR_NONE,
                    1 as libc::c_int as uint8_t,
                ),
            );
            V_DrawStretchyFixedPatch(
                320 as libc::c_int / 2 as libc::c_int
                    * ((1 as libc::c_int) << 16 as libc::c_int),
                (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                endbrdr[1 as libc::c_int as usize],
                0 as *const uint8_t,
            );
        } else {
            let mut doexplosions: boolean = false_0 as libc::c_int;
            let mut borderstuff: boolean = false_0 as libc::c_int;
            let mut tweakx: int32_t = 0 as libc::c_int;
            let mut tweaky: int32_t = 0 as libc::c_int;
            if parallaxticker < 75 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    -(x / 10 as libc::c_int),
                    -(y / 10 as libc::c_int),
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endbgsp[0 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
                V_DrawStretchyFixedPatch(
                    -(x / 5 as libc::c_int),
                    -(y / 5 as libc::c_int),
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endbgsp[1 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
                V_DrawStretchyFixedPatch(
                    0 as libc::c_int,
                    -(y / 2 as libc::c_int),
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endbgsp[2 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
                V_DrawStretchyFixedPatch(
                    ((200 as libc::c_int) << 16 as libc::c_int)
                        + (finalecount << 16 as libc::c_int - 2 as libc::c_int),
                    ((100 as libc::c_int) << 16 as libc::c_int)
                        + (finalecount << 16 as libc::c_int - 2 as libc::c_int),
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endescp[4 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
                if parallaxticker > -(19 as libc::c_int) {
                    let mut trans_0: int32_t = -parallaxticker >> 1 as libc::c_int;
                    if trans_0 < 0 as libc::c_int {
                        trans_0 = 0 as libc::c_int;
                    }
                    V_DrawStretchyFixedPatch(
                        ((200 as libc::c_int) << 16 as libc::c_int)
                            + (finalecount << 16 as libc::c_int - 2 as libc::c_int),
                        ((100 as libc::c_int) << 16 as libc::c_int)
                            + (finalecount << 16 as libc::c_int - 2 as libc::c_int),
                        (1 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        trans_0 << 16 as libc::c_int,
                        endescp[(finalecount / 2 as libc::c_int & 3 as libc::c_int)
                            as usize],
                        0 as *const uint8_t,
                    );
                }
                if goodending != 0 && parallaxticker > 0 as libc::c_int {
                    let mut scale: int32_t = ((1 as libc::c_int) << 16 as libc::c_int)
                        + ((parallaxticker - 10 as libc::c_int) << 7 as libc::c_int);
                    let mut trans_1: int32_t = parallaxticker >> 2 as libc::c_int;
                    let mut colormap: *mut uint8_t = R_GetTranslationColormap(
                        TC_RAINBOW as libc::c_int,
                        SKINCOLOR_JET,
                        1 as libc::c_int as uint8_t,
                    );
                    if parallaxticker < 10 as libc::c_int {
                        tweakx = parallaxticker << 16 as libc::c_int;
                        tweaky = (7 as libc::c_int * parallaxticker
                            << 16 as libc::c_int - 2 as libc::c_int) / 5 as libc::c_int;
                    } else {
                        tweakx = (10 as libc::c_int) << 16 as libc::c_int;
                        tweaky = (7 as libc::c_int)
                            << 16 as libc::c_int - 1 as libc::c_int;
                    }
                    i += tweakx;
                    j -= tweaky;
                    x <<= 1 as libc::c_int;
                    y <<= 1 as libc::c_int;
                    V_DrawStretchyFixedPatch(
                        i - x,
                        j - y,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        0 as libc::c_int,
                        endegrk[0 as libc::c_int as usize],
                        colormap,
                    );
                    if trans_1 < 10 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            i - x,
                            j - y,
                            (1 as libc::c_int) << 16 as libc::c_int,
                            (1 as libc::c_int) << 16 as libc::c_int,
                            trans_1 << 16 as libc::c_int,
                            endegrk[0 as libc::c_int as usize],
                            0 as *const uint8_t,
                        );
                    }
                    V_DrawStretchyFixedPatch(
                        30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + i
                            - 2 as libc::c_int * x,
                        30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + j
                            - 2 as libc::c_int * y
                            - ((7 as libc::c_int) << 16 as libc::c_int)
                                / 2 as libc::c_int,
                        scale,
                        scale,
                        0 as libc::c_int,
                        endegrk[1 as libc::c_int as usize],
                        colormap,
                    );
                    if trans_1 < 10 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            30 as libc::c_int
                                * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + i
                                - 2 as libc::c_int * x,
                            30 as libc::c_int
                                * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + j
                                - 2 as libc::c_int * y,
                            scale,
                            scale,
                            trans_1 << 16 as libc::c_int,
                            endegrk[1 as libc::c_int as usize],
                            0 as *const uint8_t,
                        );
                    }
                    scale += (parallaxticker - 10 as libc::c_int) << 7 as libc::c_int;
                    V_DrawStretchyFixedPatch(
                        30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + i
                            - x / 2 as libc::c_int,
                        30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + j
                            - y / 2 as libc::c_int
                            - ((7 as libc::c_int) << 16 as libc::c_int)
                                / 2 as libc::c_int,
                        scale,
                        scale,
                        0 as libc::c_int,
                        endxpld[0 as libc::c_int as usize],
                        colormap,
                    );
                    if trans_1 < 10 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            30 as libc::c_int
                                * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + i
                                - x / 2 as libc::c_int,
                            30 as libc::c_int
                                * (((1 as libc::c_int) << 16 as libc::c_int) - scale) + j
                                - y / 2 as libc::c_int,
                            scale,
                            scale,
                            trans_1 << 16 as libc::c_int,
                            endxpld[0 as libc::c_int as usize],
                            0 as *const uint8_t,
                        );
                    }
                }
            } else if goodending != 0 {
                tweakx = (10 as libc::c_int) << 16 as libc::c_int;
                tweaky = (7 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int;
                i += tweakx;
                j += tweaky;
                x <<= 1 as libc::c_int;
                y <<= 1 as libc::c_int;
            }
            if goodending != 0 && parallaxticker > 0 as libc::c_int {
                i -= 3 as libc::c_int + (tweakx << 1 as libc::c_int);
                j += tweaky << 2 as libc::c_int;
            }
            if parallaxticker <= 70 as libc::c_int {
                let mut trans_2: int32_t = 0;
                let mut scale_0: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
                let mut colormap_0: [*mut uint8_t; 2] = [
                    0 as *mut uint8_t,
                    0 as *mut uint8_t,
                ];
                x += i;
                y += j;
                if parallaxticker > 66 as libc::c_int {
                    scale_0 = 70 as libc::c_int - parallaxticker
                        << 16 as libc::c_int - 2 as libc::c_int;
                    x
                        += 30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale_0);
                    y
                        += 30 as libc::c_int
                            * (((1 as libc::c_int) << 16 as libc::c_int) - scale_0);
                } else if !(parallaxticker > 60 as libc::c_int
                    || goodending != 0 && parallaxticker > 0 as libc::c_int)
                {
                    doexplosions = true_0 as libc::c_int;
                    if sparklloop == 0 {
                        x
                            += if sparkloffs[0 as libc::c_int
                                as usize][0 as libc::c_int as usize]
                                < (30 as libc::c_int) << 16 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                -((1 as libc::c_int) << 16 as libc::c_int)
                            };
                        y
                            += if sparkloffs[0 as libc::c_int
                                as usize][1 as libc::c_int as usize]
                                < (30 as libc::c_int) << 16 as libc::c_int
                            {
                                (1 as libc::c_int) << 16 as libc::c_int
                            } else {
                                -((1 as libc::c_int) << 16 as libc::c_int)
                            };
                    }
                }
                if goodending != 0 && finalecount > 6 as libc::c_int * 35 as libc::c_int
                {
                    parallaxticker -= 40 as libc::c_int;
                }
                if (-parallaxticker / 4 as libc::c_int) < 5 as libc::c_int {
                    trans_2 = -parallaxticker / 4 as libc::c_int + 5 as libc::c_int;
                    if trans_2 < 0 as libc::c_int {
                        trans_2 = 0 as libc::c_int;
                    }
                    V_DrawStretchyFixedPatch(
                        x,
                        y,
                        scale_0,
                        scale_0,
                        trans_2 << 16 as libc::c_int,
                        endglow[(if finalecount & 1 as libc::c_int != 0 {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }) as usize],
                        0 as *const uint8_t,
                    );
                }
                if goodending != 0 && finalecount > 6 as libc::c_int * 35 as libc::c_int
                {
                    if finalecount
                        < 6 as libc::c_int * 35 as libc::c_int + 10 as libc::c_int
                    {
                        V_DrawFadeFill(
                            24 as libc::c_int,
                            24 as libc::c_int,
                            320 as libc::c_int - 48 as libc::c_int,
                            200 as libc::c_int - 48 as libc::c_int,
                            0 as libc::c_int,
                            0 as libc::c_int as uint16_t,
                            (6 as libc::c_int * 35 as libc::c_int + 10 as libc::c_int
                                - finalecount) as uint8_t,
                        );
                    }
                    parallaxticker -= 30 as libc::c_int;
                }
                if parallaxticker / 2 as libc::c_int > -(15 as libc::c_int) {
                    colormap_0[0 as libc::c_int
                        as usize] = R_GetTranslationColormap(
                        TC_ALLWHITE as libc::c_int,
                        SKINCOLOR_NONE,
                        1 as libc::c_int as uint8_t,
                    );
                }
                V_DrawStretchyFixedPatch(
                    x,
                    y,
                    scale_0,
                    scale_0,
                    0 as libc::c_int,
                    rockpat,
                    colormap_0[0 as libc::c_int as usize],
                );
                if parallaxticker / 2 as libc::c_int > -(25 as libc::c_int) {
                    trans_2 = parallaxticker / 2 as libc::c_int + 15 as libc::c_int;
                    if trans_2 < 0 as libc::c_int {
                        trans_2 = -trans_2;
                    }
                    if trans_2 < 10 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            x,
                            y,
                            scale_0,
                            scale_0,
                            trans_2 << 16 as libc::c_int,
                            rockpat,
                            R_GetTranslationColormap(
                                TC_BLINK as libc::c_int,
                                SKINCOLOR_AQUA,
                                1 as libc::c_int as uint8_t,
                            ),
                        );
                    }
                }
                if goodending != 0 && finalecount > 6 as libc::c_int * 35 as libc::c_int
                {
                    if finalecount
                        < 6 as libc::c_int * 35 as libc::c_int + 10 as libc::c_int
                    {
                        V_DrawStretchyFixedPatch(
                            x,
                            y,
                            scale_0,
                            scale_0,
                            (finalecount - 6 as libc::c_int * 35 as libc::c_int)
                                << 16 as libc::c_int,
                            rockpat,
                            R_GetTranslationColormap(
                                TC_BLINK as libc::c_int,
                                SKINCOLOR_BLACK,
                                1 as libc::c_int as uint8_t,
                            ),
                        );
                    }
                } else {
                    if (-parallaxticker / 2 as libc::c_int) < -(5 as libc::c_int) {
                        colormap_0[1 as libc::c_int
                            as usize] = R_GetTranslationColormap(
                            TC_ALLWHITE as libc::c_int,
                            SKINCOLOR_NONE,
                            1 as libc::c_int as uint8_t,
                        );
                    }
                    V_DrawStretchyFixedPatch(
                        x,
                        y,
                        scale_0,
                        scale_0,
                        0 as libc::c_int,
                        endegrk[0 as libc::c_int as usize],
                        colormap_0[1 as libc::c_int as usize],
                    );
                    if (-parallaxticker / 2 as libc::c_int) < 5 as libc::c_int {
                        trans_2 = -parallaxticker / 2 as libc::c_int + 5 as libc::c_int;
                        if trans_2 < 0 as libc::c_int {
                            trans_2 = -trans_2;
                        }
                        if trans_2 < 10 as libc::c_int {
                            V_DrawStretchyFixedPatch(
                                x,
                                y,
                                scale_0,
                                scale_0,
                                trans_2 << 16 as libc::c_int,
                                endegrk[1 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        }
                    }
                }
            } else {
                let mut scale_1: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
                let mut frame: int32_t = 0;
                let mut colormap_1: *mut uint8_t = 0 as *mut uint8_t;
                parallaxticker -= 70 as libc::c_int;
                x
                    += ((320 as libc::c_int - 3 as libc::c_int)
                        << 16 as libc::c_int - 1 as libc::c_int) - tweakx;
                y
                    += ((200 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
                        + tweaky;
                borderstuff = true_0 as libc::c_int;
                if parallaxticker < 5 as libc::c_int {
                    scale_1 = (parallaxticker << 16 as libc::c_int) / 4 as libc::c_int;
                    V_DrawFadeFill(
                        24 as libc::c_int,
                        24 as libc::c_int,
                        320 as libc::c_int - 48 as libc::c_int,
                        200 as libc::c_int - 48 as libc::c_int,
                        0 as libc::c_int,
                        31 as libc::c_int as uint16_t,
                        (parallaxticker * 2 as libc::c_int) as uint8_t,
                    );
                } else {
                    scale_1 += (parallaxticker - 4 as libc::c_int) << 5 as libc::c_int;
                }
                if goodending != 0 {
                    colormap_1 = R_GetTranslationColormap(
                        players[consoleplayer as usize].skin,
                        players[consoleplayer as usize].skincolor as skincolornum_t,
                        1 as libc::c_int as uint8_t,
                    );
                }
                frame = (if parallaxticker & 1 as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) + parallaxticker / 35 as libc::c_int;
                if frame < 3 as libc::c_int {
                    V_DrawStretchyFixedPatch(
                        x,
                        y,
                        scale_1,
                        scale_1,
                        0 as libc::c_int,
                        endfwrk[frame as usize],
                        colormap_1,
                    );
                }
            }
            if sparklloop >= 3 as libc::c_int && doexplosions != 0 {
                let mut boomtime: int32_t = parallaxticker - sparklloop;
                x = (((320 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int
                    + 11 as libc::c_int) << 16 as libc::c_int)
                    - ((boomtime * 20 as libc::c_int) << 16 as libc::c_int)
                        / (6 as libc::c_int * 35 as libc::c_int);
                y = (((200 as libc::c_int - 82 as libc::c_int) / 2 as libc::c_int
                    + 12 as libc::c_int) << 16 as libc::c_int)
                    + ((boomtime * 7 as libc::c_int) << 16 as libc::c_int)
                        / (6 as libc::c_int * 35 as libc::c_int);
                V_DrawStretchyFixedPatch(
                    x + sparkloffs[0 as libc::c_int as usize][0 as libc::c_int as usize],
                    y + sparkloffs[0 as libc::c_int as usize][1 as libc::c_int as usize],
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    0 as libc::c_int,
                    endxpld[(sparklloop / 4 as libc::c_int) as usize],
                    0 as *const uint8_t,
                );
            }
            if finalecount < 30 as libc::c_int {
                V_DrawFadeFill(
                    24 as libc::c_int,
                    24 as libc::c_int,
                    320 as libc::c_int - 48 as libc::c_int,
                    200 as libc::c_int - 48 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int as uint16_t,
                    (30 as libc::c_int - finalecount) as uint8_t,
                );
            }
            let mut trans_3: int32_t = 0 as libc::c_int;
            if borderstuff != 0 {
                trans_3 = 10 as libc::c_int * parallaxticker
                    / (3 as libc::c_int * 35 as libc::c_int);
            }
            if trans_3 < 10 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    320 as libc::c_int / 2 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int),
                    (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    trans_3 << 16 as libc::c_int,
                    endbrdr[0 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
            }
            if borderstuff != 0 && parallaxticker < 11 as libc::c_int {
                V_DrawStretchyFixedPatch(
                    320 as libc::c_int / 2 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int),
                    (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (parallaxticker - 1 as libc::c_int) << 16 as libc::c_int,
                    endbrdr[1 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
            } else if goodending != 0
                && finalecount > 6 as libc::c_int * 35 as libc::c_int
                && finalecount < 6 as libc::c_int * 35 as libc::c_int + 10 as libc::c_int
            {
                V_DrawStretchyFixedPatch(
                    320 as libc::c_int / 2 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int),
                    (200 as libc::c_int / 2 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (finalecount - 6 as libc::c_int * 35 as libc::c_int)
                        << 16 as libc::c_int,
                    endbrdr[1 as libc::c_int as usize],
                    0 as *const uint8_t,
                );
            }
            if goodending != 0 && finalecount >= 35 as libc::c_int
                && finalecount < 6 as libc::c_int * 35 as libc::c_int
            {
                let mut workingtime: int32_t = finalecount - 35 as libc::c_int;
                let mut radius: fixed_t = vid.width / vid.dupx
                    * (6 as libc::c_int * 35 as libc::c_int - 35 as libc::c_int
                        - workingtime)
                    / (6 as libc::c_int * 35 as libc::c_int - 35 as libc::c_int);
                let mut fa: angle_t = 0;
                let mut eemeralds_cur: [int32_t; 4] = [0; 4];
                let mut patchname: [libc::c_char; 7] = *::core::mem::transmute::<
                    &[u8; 7],
                    &mut [libc::c_char; 7],
                >(b"CEMGx0\0");
                radius <<= 16 as libc::c_int;
                i = 0 as libc::c_int;
                while i < 4 as libc::c_int {
                    if i == 1 as libc::c_int {
                        workingtime -= sparklloop;
                    } else if i != 0 {
                        workingtime -= 15 as libc::c_int;
                    }
                    eemeralds_cur[i
                        as usize] = (workingtime % 360 as libc::c_int)
                        << 16 as libc::c_int;
                    i += 1;
                    i;
                }
                i = 0 as libc::c_int;
                while i < 7 as libc::c_int {
                    let mut colormap_2: *mut uint8_t = 0 as *mut uint8_t;
                    let mut col: skincolornum_t = SKINCOLOR_GREEN;
                    match i {
                        1 => {
                            col = SKINCOLOR_MAGENTA;
                        }
                        2 => {
                            col = SKINCOLOR_BLUE;
                        }
                        3 => {
                            col = SKINCOLOR_SKY;
                        }
                        4 => {
                            col = SKINCOLOR_ORANGE;
                        }
                        5 => {
                            col = SKINCOLOR_RED;
                        }
                        6 => {
                            col = SKINCOLOR_GREY;
                        }
                        0 | _ => {}
                    }
                    colormap_2 = R_GetTranslationColormap(
                        TC_DEFAULT as libc::c_int,
                        col,
                        1 as libc::c_int as uint8_t,
                    );
                    j = if sparklloop & 1 as libc::c_int != 0 {
                        2 as libc::c_int
                    } else {
                        3 as libc::c_int
                    };
                    while j != 0 {
                        fa = FixedAngle(eemeralds_cur[j as usize]) >> 19 as libc::c_int
                            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
                        x = ((320 as libc::c_int)
                            << 16 as libc::c_int - 1 as libc::c_int)
                            + FixedMul(
                                *finecosine.offset(fa as isize)
                                    >> 16 as libc::c_int - 16 as libc::c_int,
                                radius,
                            );
                        y = ((200 as libc::c_int)
                            << 16 as libc::c_int - 1 as libc::c_int)
                            + FixedMul(
                                finesine[fa as usize]
                                    >> 16 as libc::c_int - 16 as libc::c_int,
                                radius,
                            );
                        eemeralds_cur[j as usize]
                            += ((360 as libc::c_int) << 16 as libc::c_int)
                                / 7 as libc::c_int;
                        V_DrawStretchyFixedPatch(
                            x,
                            y,
                            (1 as libc::c_int) << 16 as libc::c_int,
                            (1 as libc::c_int) << 16 as libc::c_int,
                            0 as libc::c_int,
                            endspkl[(j
                                - (if sparklloop & 1 as libc::c_int != 0 {
                                    0 as libc::c_int
                                } else {
                                    1 as libc::c_int
                                })) as usize],
                            colormap_2,
                        );
                        j -= 1;
                        j;
                    }
                    i += 1;
                    i;
                }
                i = 0 as libc::c_int;
                while i < 7 as libc::c_int {
                    fa = FixedAngle(eemeralds_cur[0 as libc::c_int as usize])
                        >> 19 as libc::c_int
                        & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
                    x = ((320 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
                        + FixedMul(
                            *finecosine.offset(fa as isize)
                                >> 16 as libc::c_int - 16 as libc::c_int,
                            radius,
                        );
                    y = ((200 as libc::c_int + 16 as libc::c_int)
                        << 16 as libc::c_int - 1 as libc::c_int)
                        + FixedMul(
                            finesine[fa as usize]
                                >> 16 as libc::c_int - 16 as libc::c_int,
                            radius,
                        );
                    eemeralds_cur[0 as libc::c_int as usize]
                        += ((360 as libc::c_int) << 16 as libc::c_int)
                            / 7 as libc::c_int;
                    patchname[4 as libc::c_int
                        as usize] = ('A' as i32 + i as libc::c_char as libc::c_int)
                        as libc::c_char;
                    V_DrawStretchyFixedPatch(
                        x,
                        y,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        (1 as libc::c_int) << 16 as libc::c_int,
                        0 as libc::c_int,
                        W_CachePatchName(
                            patchname.as_mut_ptr(),
                            PU_PATCH_LOWPRIORITY as libc::c_int,
                        ) as *mut patch_t,
                        0 as *const uint8_t,
                    );
                    i += 1;
                    i;
                }
            }
        }
    }
    if cv_soundtest.value == 413 as libc::c_int {
        let mut trans_4: int32_t = 0 as libc::c_int;
        let mut donttouch: boolean = false_0 as libc::c_int;
        let mut str: *const libc::c_char = 0 as *const libc::c_char;
        if goodending != 0 {
            str = va(
                b"[S] %s: Engage.\0" as *const u8 as *const libc::c_char,
                (skins[players[consoleplayer as usize].skin as usize].realname)
                    .as_mut_ptr(),
            );
        } else {
            str = b"[S] Eggman: Abscond.\0" as *const u8 as *const libc::c_char;
        }
        if finalecount < 10 as libc::c_int {
            trans_4 = (10 as libc::c_int - finalecount) / 2 as libc::c_int;
        } else if finalecount > 14 as libc::c_int * 35 as libc::c_int - 20 as libc::c_int
        {
            trans_4 = 10 as libc::c_int
                + (finalecount - 14 as libc::c_int * 35 as libc::c_int)
                    / 2 as libc::c_int;
            donttouch = true_0 as libc::c_int;
        }
        if trans_4 < 10 as libc::c_int {
            V_DrawCenteredString(
                320 as libc::c_int / 2 as libc::c_int,
                8 as libc::c_int,
                0x800000 as libc::c_int | trans_4 << 16 as libc::c_int,
                str,
            );
            V_DrawCharacter(
                32 as libc::c_int,
                200 as libc::c_int - 16 as libc::c_int,
                '>' as i32 | trans_4 << 16 as libc::c_int,
                false_0 as libc::c_int,
            );
            V_DrawString(
                40 as libc::c_int,
                (if finalecount
                    == 14 as libc::c_int * 35 as libc::c_int
                        - (20 as libc::c_int + 35 as libc::c_int)
                {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) + 200 as libc::c_int - 16 as libc::c_int,
                (if (*serverGamedata).timesBeaten != 0
                    || finalecount
                        >= 14 as libc::c_int * 35 as libc::c_int - 35 as libc::c_int
                {
                    0x9000 as libc::c_int
                } else {
                    0x4000 as libc::c_int
                }) | trans_4 << 16 as libc::c_int,
                b" [S] ===>\0" as *const u8 as *const libc::c_char,
            );
        }
        if finalecount
            > 14 as libc::c_int * 35 as libc::c_int
                - (20 as libc::c_int + 2 as libc::c_int * 35 as libc::c_int)
        {
            let mut trans2: int32_t = abs(
                5 as libc::c_int
                    * (*finecosine
                        .offset(
                            (FixedAngle(
                                (finalecount * 5 as libc::c_int) << 16 as libc::c_int,
                            ) >> 19 as libc::c_int
                                & (8192 as libc::c_int - 1 as libc::c_int) as angle_t)
                                as isize,
                        ) >> 16 as libc::c_int - 16 as libc::c_int) >> 16 as libc::c_int,
            ) + 2 as libc::c_int;
            if donttouch == 0 {
                trans_4 = 10 as libc::c_int
                    + (14 as libc::c_int * 35 as libc::c_int
                        - (20 as libc::c_int + 2 as libc::c_int * 35 as libc::c_int))
                    - finalecount;
                if trans_4 > trans2 {
                    trans2 = trans_4;
                }
            } else {
                trans2 += 2 as libc::c_int * trans_4;
            }
            if trans2 < 10 as libc::c_int {
                V_DrawCharacter(
                    26 as libc::c_int,
                    200 as libc::c_int - 33 as libc::c_int,
                    '\u{1c}' as i32 | trans2 << 16 as libc::c_int,
                    false_0 as libc::c_int,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_StartGameEnd() {
    G_SetGamestate(GS_GAMEEND);
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    S_StopSounds();
    M_ClearMenus(true_0 as libc::c_int);
    timetonext = 35 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_GameEndDrawer() {}
#[no_mangle]
pub unsafe extern "C" fn F_GameEndTicker() {
    if timetonext > 0 as libc::c_int {
        timetonext -= 1;
        timetonext;
    } else {
        D_StartTitle();
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_InitMenuPresValues() {
    curbgx = 0 as libc::c_int;
    curbgy = 0 as libc::c_int;
    prevMenuId = 0 as libc::c_int as uint32_t;
    activeMenuId = MainDef.menuid;
    strncpy(
        curbgname.as_mut_ptr(),
        b"TITLESKY\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    );
    curfadevalue = 16 as libc::c_int as int8_t;
    curbgcolor = -(1 as libc::c_int);
    curbgxspeed = if gamestate as libc::c_uint
        == GS_TIMEATTACK as libc::c_int as libc::c_uint
    {
        0 as libc::c_int
    } else {
        titlescrollxspeed
    };
    curbgyspeed = if gamestate as libc::c_uint
        == GS_TIMEATTACK as libc::c_int as libc::c_uint
    {
        18 as libc::c_int
    } else {
        titlescrollyspeed
    };
    curbghide = if gamestate as libc::c_uint
        == GS_TIMEATTACK as libc::c_int as libc::c_uint
    {
        false_0 as libc::c_int
    } else {
        true_0 as libc::c_int
    };
    curhidepics = hidetitlepics;
    curttmode = ttmode;
    curttscale = ttscale;
    strncpy(
        curttname.as_mut_ptr(),
        ttname.as_mut_ptr(),
        9 as libc::c_int as libc::c_ulong,
    );
    curttx = ttx;
    curtty = tty;
    curttloop = ttloop;
    curtttics = tttics;
    M_SetMenuCurBackground(
        if gamestate as libc::c_uint == GS_TIMEATTACK as libc::c_int as libc::c_uint {
            b"RECATTBG\0" as *const u8 as *const libc::c_char
        } else {
            b"TITLESKY\0" as *const u8 as *const libc::c_char
        },
    );
    M_SetMenuCurFadeValue(16 as libc::c_int as uint8_t);
    M_SetMenuCurTitlePics();
    LUA_HUD_DestroyDrawList(luahuddrawlist_title);
    luahuddrawlist_title = LUA_HUD_CreateDrawList();
}
#[no_mangle]
pub unsafe extern "C" fn F_SkyScroll(mut patchname: *const libc::c_char) {
    let mut x: int32_t = 0;
    let mut basey: int32_t = 0 as libc::c_int;
    let mut dupz: int32_t = if vid.dupx < vid.dupy { vid.dupx } else { vid.dupy };
    let mut pat: *mut patch_t = 0 as *mut patch_t;
    if rendermode as libc::c_uint == render_none as libc::c_int as libc::c_uint {
        return;
    }
    if patchname.is_null() || *patchname.offset(0 as libc::c_int as isize) == 0 {
        V_DrawFill(
            0 as libc::c_int,
            0 as libc::c_int,
            vid.width,
            vid.height,
            31 as libc::c_int,
        );
        return;
    }
    pat = W_CachePatchName(patchname, PU_PATCH_LOWPRIORITY as libc::c_int)
        as *mut patch_t;
    if curbgxspeed == 0 && curbgyspeed == 0 {
        V_DrawPatchFill(pat);
        W_UnlockCachedPatch(pat as *mut libc::c_void);
        return;
    }
    curbgx %= (*pat).width as libc::c_int * 16 as libc::c_int;
    curbgy %= (*pat).height as libc::c_int * 16 as libc::c_int;
    x = (curbgx * dupz
        + FixedInt(
            (rendertimefrac - ((1 as libc::c_int) << 16 as libc::c_int)) * curbgxspeed
                * dupz,
        )) / 16 as libc::c_int;
    basey = (curbgy * dupz
        + FixedInt(
            (rendertimefrac - ((1 as libc::c_int) << 16 as libc::c_int)) * curbgyspeed
                * dupz,
        )) / 16 as libc::c_int;
    if x > 0 as libc::c_int {
        x -= (*pat).width as libc::c_int * dupz;
    }
    if basey > 0 as libc::c_int {
        basey -= (*pat).height as libc::c_int * dupz;
    }
    while x < vid.width {
        let mut y: int32_t = basey;
        while y < vid.height {
            V_DrawStretchyFixedPatch(
                x * ((1 as libc::c_int) << 16 as libc::c_int),
                y << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0x40000000 as libc::c_int,
                pat,
                0 as *const uint8_t,
            );
            y += (*pat).height as libc::c_int * dupz;
        }
        x += (*pat).width as libc::c_int * dupz;
    }
    W_UnlockCachedPatch(pat as *mut libc::c_void);
}
unsafe extern "C" fn F_CacheTitleScreen() {
    match curttmode as libc::c_uint {
        1 | 0 => {
            ttbanner = W_CachePatchName(
                b"TTBANNER\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttwing = W_CachePatchName(
                b"TTWING\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsonic = W_CachePatchName(
                b"TTSONIC\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttswave1 = W_CachePatchName(
                b"TTSWAVE1\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttswave2 = W_CachePatchName(
                b"TTSWAVE2\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttswip1 = W_CachePatchName(
                b"TTSWIP1\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsprep1 = W_CachePatchName(
                b"TTSPREP1\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsprep2 = W_CachePatchName(
                b"TTSPREP2\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop1 = W_CachePatchName(
                b"TTSPOP1\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop2 = W_CachePatchName(
                b"TTSPOP2\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop3 = W_CachePatchName(
                b"TTSPOP3\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop4 = W_CachePatchName(
                b"TTSPOP4\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop5 = W_CachePatchName(
                b"TTSPOP5\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop6 = W_CachePatchName(
                b"TTSPOP6\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttspop7 = W_CachePatchName(
                b"TTSPOP7\0" as *const u8 as *const libc::c_char,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
        }
        3 => {
            let mut i: uint16_t = 0;
            let mut lumpnum: lumpnum_t = 0;
            let mut lumpname: [libc::c_char; 9] = [0; 9];
            lumpnum = W_CheckNumForName(curttname.as_mut_ptr());
            if lumpnum != 4294967295 as libc::c_uint {
                ttuser[0 as libc::c_int
                    as usize] = W_CachePatchName(
                    curttname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                ttuser[(if (1 as libc::c_int) < 100 as libc::c_int - 1 as libc::c_int {
                    1 as libc::c_int
                } else {
                    100 as libc::c_int - 1 as libc::c_int
                }) as usize] = 0 as *mut patch_t;
            } else if strlen(curttname.as_mut_ptr()) <= 6 as libc::c_int as libc::c_ulong
            {
                let mut cnt: fixed_t = strlen(curttname.as_mut_ptr()) as fixed_t;
                strncpy(
                    lumpname.as_mut_ptr(),
                    curttname.as_mut_ptr(),
                    7 as libc::c_int as libc::c_ulong,
                );
                i = 0 as libc::c_int as uint16_t;
                while (i as libc::c_int) < 100 as libc::c_int - 1 as libc::c_int {
                    sprintf(
                        &mut *lumpname.as_mut_ptr().offset(cnt as isize)
                            as *mut libc::c_char,
                        b"%.2hu\0" as *const u8 as *const libc::c_char,
                        (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                    );
                    lumpname[8 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_char;
                    lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                    if !(lumpnum != 4294967295 as libc::c_uint) {
                        break;
                    }
                    ttuser[i
                        as usize] = W_CachePatchName(
                        lumpname.as_mut_ptr(),
                        PU_PATCH_LOWPRIORITY as libc::c_int,
                    ) as *mut patch_t;
                    i = i.wrapping_add(1);
                    i;
                }
                ttuser[(if (i as libc::c_int) < 100 as libc::c_int - 1 as libc::c_int {
                    i as libc::c_int
                } else {
                    100 as libc::c_int - 1 as libc::c_int
                }) as usize] = 0 as *mut patch_t;
            } else {
                ttuser[0 as libc::c_int as usize] = 0 as *mut patch_t;
            }
        }
        2 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_StartTitleScreen() {
    if menupres[MN_MAIN as libc::c_int as usize].musname[0 as libc::c_int as usize] != 0
    {
        S_ChangeMusicEx(
            (menupres[MN_MAIN as libc::c_int as usize].musname).as_mut_ptr(),
            menupres[MN_MAIN as libc::c_int as usize].mustrack,
            menupres[MN_MAIN as libc::c_int as usize].muslooping,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    } else {
        S_ChangeMusicEx(
            b"_title\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as uint16_t,
            looptitle,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
    if gamestate as libc::c_uint != GS_TITLESCREEN as libc::c_int as libc::c_uint
        && gamestate as libc::c_uint != GS_WAITINGPLAYERS as libc::c_int as libc::c_uint
    {
        knux_idle_end = 0 as libc::c_int;
        knux_idle_start = knux_idle_end;
        knux_blink_twice = knux_idle_start;
        knux_blink = knux_blink_twice;
        tails_idle_end = knux_blink;
        tails_idle_start = tails_idle_end;
        tails_blink_twice = tails_idle_start;
        tails_blink = tails_blink_twice;
        sonic_idle_end = tails_blink;
        sonic_idle_start = sonic_idle_end;
        sonic_blink_twice = sonic_idle_start;
        sonic_blink = sonic_blink_twice;
        activettscale = sonic_blink as int8_t;
        testttscale = activettscale;
        ttloaded[5 as libc::c_int as usize] = testttscale as boolean;
        ttloaded[4 as libc::c_int as usize] = ttloaded[5 as libc::c_int as usize];
        ttloaded[3 as libc::c_int as usize] = ttloaded[4 as libc::c_int as usize];
        ttloaded[2 as libc::c_int as usize] = ttloaded[3 as libc::c_int as usize];
        ttloaded[1 as libc::c_int as usize] = ttloaded[2 as libc::c_int as usize];
        ttloaded[0 as libc::c_int as usize] = ttloaded[1 as libc::c_int as usize];
        ttuser_count = ttloaded[0 as libc::c_int as usize];
        knux_blinked_already = 1 as libc::c_int;
        tails_blinked_already = knux_blinked_already;
        sonic_blinked_already = tails_blinked_already;
        if curttmode as libc::c_uint == TTMODE_ALACROIX as libc::c_int as libc::c_uint {
            finalecount = -(3 as libc::c_int);
        } else {
            finalecount = 0 as libc::c_int;
        }
        wipetypepost = menupres[MN_MAIN as libc::c_int as usize].enterwipe;
    } else {
        wipegamestate = GS_TITLESCREEN;
    }
    if titlemap != 0 {
        let mut startpos: *mut mapthing_t = 0 as *mut mapthing_t;
        let mut prevwipegamestate: gamestate_t = wipegamestate;
        titlemapinaction = TITLEMAP_LOADING as libc::c_int as uint8_t;
        titlemapcameraref = 0 as *mut mobj_t;
        gamemap = titlemap;
        if (mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .is_null()
        {
            P_AllocMapHeader((gamemap as libc::c_int - 1 as libc::c_int) as int16_t);
        }
        maptol = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .typeoflevel;
        globalweather = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int)
            as usize])
            .weather;
        G_DoLoadLevel(true_0 as libc::c_int);
        if titlemap == 0 {
            return;
        }
        players[displayplayer as usize].playerstate = PST_DEAD;
        if !(playerstarts[0 as libc::c_int as usize]).is_null() {
            startpos = playerstarts[0 as libc::c_int as usize];
        } else if !(deathmatchstarts[0 as libc::c_int as usize]).is_null() {
            startpos = deathmatchstarts[0 as libc::c_int as usize];
        } else {
            startpos = 0 as *mut mapthing_t;
        }
        if !startpos.is_null() {
            camera.x = ((*startpos).x as libc::c_int) << 16 as libc::c_int;
            camera.y = ((*startpos).y as libc::c_int) << 16 as libc::c_int;
            camera.subsector = R_PointInSubsector(camera.x, camera.y);
            camera
                .z = (*(*camera.subsector).sector).floorheight
                + (((*startpos).z as libc::c_int) << 16 as libc::c_int);
            camera
                .angle = ((*startpos).angle as libc::c_int % 360 as libc::c_int
                * 0xb60b61 as libc::c_int) as angle_t;
            camera.aiming = 0 as libc::c_int as angle_t;
        } else {
            camera.aiming = 0 as libc::c_int as angle_t;
            camera.angle = camera.aiming;
            camera.z = camera.angle as fixed_t;
            camera.y = camera.z;
            camera.x = camera.y;
            camera.subsector = 0 as *mut subsector_s;
        }
        camera.chase = true_0 as libc::c_int;
        camera.height = 0 as libc::c_int;
        if menupres[MN_MAIN as libc::c_int as usize].entertag != 0 {
            P_LinedefExecute(
                menupres[MN_MAIN as libc::c_int as usize].entertag as int16_t,
                players[displayplayer as usize].mo,
                0 as *mut sector_t,
            );
        }
        wipegamestate = prevwipegamestate;
    } else {
        titlemapinaction = TITLEMAP_OFF as libc::c_int as uint8_t;
        gamemap = 1 as libc::c_int as int16_t;
        if (mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .is_null()
        {
            P_AllocMapHeader((gamemap as libc::c_int - 1 as libc::c_int) as int16_t);
        }
        CON_ClearHUD();
    }
    G_SetGamestate(GS_TITLESCREEN);
    skullAnimCounter = 0 as libc::c_int as int16_t;
    animtimer = skullAnimCounter as tic_t;
    demoDelayLeft = demoDelayTime;
    demoIdleLeft = demoIdleTime;
    F_CacheTitleScreen();
}
unsafe extern "C" fn F_UnloadAlacroixGraphics(mut oldttscale: int8_t) {
    let mut i: uint16_t = 0;
    oldttscale -= 1;
    oldttscale;
    i = 0 as libc::c_int as uint16_t;
    while (i as libc::c_int) < 30 as libc::c_int {
        if !(ttembl[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttembl[oldttscale as usize][i as usize]);
            ttembl[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttribb[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttribb[oldttscale as usize][i as usize]);
            ttribb[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsont[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsont[oldttscale as usize][i as usize]);
            ttsont[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttrobo[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttrobo[oldttscale as usize][i as usize]);
            ttrobo[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttwot[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttwot[oldttscale as usize][i as usize]);
            tttwot[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttrbtx[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttrbtx[oldttscale as usize][i as usize]);
            ttrbtx[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsoib[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsoib[oldttscale as usize][i as usize]);
            ttsoib[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsoif[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsoif[oldttscale as usize][i as usize]);
            ttsoif[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsoba[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsoba[oldttscale as usize][i as usize]);
            ttsoba[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsobk[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsobk[oldttscale as usize][i as usize]);
            ttsobk[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttsodh[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttsodh[oldttscale as usize][i as usize]);
            ttsodh[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttaib[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttaib[oldttscale as usize][i as usize]);
            tttaib[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttaif[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttaif[oldttscale as usize][i as usize]);
            tttaif[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttaba[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttaba[oldttscale as usize][i as usize]);
            tttaba[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttabk[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttabk[oldttscale as usize][i as usize]);
            tttabk[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttabt[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttabt[oldttscale as usize][i as usize]);
            tttabt[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(tttaft[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(tttaft[oldttscale as usize][i as usize]);
            tttaft[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttknib[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttknib[oldttscale as usize][i as usize]);
            ttknib[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttknif[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttknif[oldttscale as usize][i as usize]);
            ttknif[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttknba[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttknba[oldttscale as usize][i as usize]);
            ttknba[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttknbk[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttknbk[oldttscale as usize][i as usize]);
            ttknbk[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        if !(ttkndh[oldttscale as usize][i as usize]).is_null() {
            Patch_Free(ttkndh[oldttscale as usize][i as usize]);
            ttkndh[oldttscale as usize][i as usize] = 0 as *mut patch_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    ttloaded[oldttscale as usize] = false_0 as libc::c_int;
}
unsafe extern "C" fn F_LoadAlacroixGraphics(mut newttscale: int8_t) {
    let mut i: uint16_t = 0;
    let mut j: uint16_t = 0;
    let mut lumpnum: lumpnum_t = 0;
    let mut lumpname: [libc::c_char; 9] = [0; 9];
    let mut names: [[libc::c_char; 5]; 22] = [
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"EMBL\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"RIBB\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SONT\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"ROBO\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TWOT\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"RBTX\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SOIB\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SOIF\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SOBA\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SOBK\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"SODH\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TAIB\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TAIF\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TABA\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TABK\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TABT\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"TAFT\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"KNIB\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"KNIF\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"KNBA\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"KNBK\0"),
        *::core::mem::transmute::<&[u8; 5], &mut [libc::c_char; 5]>(b"KNDH\0"),
    ];
    let mut lumpnames: [[libc::c_char; 7]; 22] = [[0; 7]; 22];
    newttscale -= 1;
    newttscale;
    if ttloaded[newttscale as usize] == 0 {
        j = 0 as libc::c_int as uint16_t;
        while (j as libc::c_int) < 22 as libc::c_int {
            sprintf(
                &mut *(*lumpnames.as_mut_ptr().offset(j as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_char,
                b"T%.1hu%s\0" as *const u8 as *const libc::c_char,
                (newttscale as uint8_t as libc::c_int + 1 as libc::c_int) as uint16_t
                    as libc::c_int,
                (names[j as usize]).as_mut_ptr(),
            );
            j = j.wrapping_add(1);
            j;
        }
        lumpnum = W_CheckNumForName((lumpnames[0 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttembl[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[0 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttembl[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[0 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt: fixed_t = strlen(
                (lumpnames[0 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[0 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttembl[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttembl[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttembl[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[1 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttribb[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[1 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttribb[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[1 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_0: fixed_t = strlen(
                (lumpnames[1 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[1 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_0 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttribb[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttribb[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttribb[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[2 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttsont[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[2 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsont[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[2 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_1: fixed_t = strlen(
                (lumpnames[2 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[2 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_1 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsont[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsont[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsont[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[3 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttrobo[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[3 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttrobo[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[3 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_2: fixed_t = strlen(
                (lumpnames[3 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[3 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_2 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttrobo[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttrobo[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttrobo[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[4 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            tttwot[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[4 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttwot[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[4 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_3: fixed_t = strlen(
                (lumpnames[4 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[4 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_3 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttwot[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttwot[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttwot[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[5 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttrbtx[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[5 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttrbtx[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[5 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_4: fixed_t = strlen(
                (lumpnames[5 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[5 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_4 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttrbtx[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttrbtx[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttrbtx[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[6 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttsoib[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[6 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsoib[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[6 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_5: fixed_t = strlen(
                (lumpnames[6 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[6 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_5 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsoib[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsoib[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsoib[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[7 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttsoif[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[7 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsoif[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[7 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_6: fixed_t = strlen(
                (lumpnames[7 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[7 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_6 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsoif[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsoif[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsoif[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[8 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttsoba[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[8 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsoba[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[8 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_7: fixed_t = strlen(
                (lumpnames[8 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[8 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_7 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsoba[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsoba[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsoba[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName((lumpnames[9 as libc::c_int as usize]).as_mut_ptr());
        if lumpnum != 4294967295 as libc::c_uint {
            ttsobk[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[9 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsobk[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[9 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_8: fixed_t = strlen(
                (lumpnames[9 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[9 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_8 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsobk[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsobk[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsobk[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[10 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttsodh[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[10 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttsodh[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[10 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_9: fixed_t = strlen(
                (lumpnames[10 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[10 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_9 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttsodh[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttsodh[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttsodh[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[11 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttaib[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[11 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttaib[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[11 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_10: fixed_t = strlen(
                (lumpnames[11 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[11 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_10 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttaib[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttaib[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttaib[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[12 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttaif[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[12 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttaif[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[12 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_11: fixed_t = strlen(
                (lumpnames[12 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[12 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_11 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttaif[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttaif[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttaif[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[13 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttaba[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[13 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttaba[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[13 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_12: fixed_t = strlen(
                (lumpnames[13 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[13 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_12 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttaba[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttaba[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttaba[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[14 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttabk[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[14 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttabk[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[14 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_13: fixed_t = strlen(
                (lumpnames[14 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[14 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_13 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttabk[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttabk[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttabk[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[15 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttabt[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[15 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttabt[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[15 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_14: fixed_t = strlen(
                (lumpnames[15 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[15 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_14 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttabt[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttabt[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttabt[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[16 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            tttaft[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[16 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            tttaft[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[16 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_15: fixed_t = strlen(
                (lumpnames[16 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[16 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_15 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                tttaft[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            tttaft[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            tttaft[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[17 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttknib[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[17 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttknib[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[17 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_16: fixed_t = strlen(
                (lumpnames[17 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[17 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_16 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttknib[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttknib[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttknib[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[18 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttknif[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[18 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttknif[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[18 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_17: fixed_t = strlen(
                (lumpnames[18 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[18 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_17 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttknif[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttknif[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttknif[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[19 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttknba[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[19 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttknba[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[19 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_18: fixed_t = strlen(
                (lumpnames[19 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[19 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_18 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttknba[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttknba[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttknba[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[20 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttknbk[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[20 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttknbk[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[20 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_19: fixed_t = strlen(
                (lumpnames[20 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[20 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_19 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttknbk[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttknbk[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttknbk[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        lumpnum = W_CheckNumForName(
            (lumpnames[21 as libc::c_int as usize]).as_mut_ptr(),
        );
        if lumpnum != 4294967295 as libc::c_uint {
            ttkndh[newttscale
                as usize][0 as libc::c_int
                as usize] = W_CachePatchName(
                (lumpnames[21 as libc::c_int as usize]).as_mut_ptr(),
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) as *mut patch_t;
            ttkndh[newttscale
                as usize][(if (1 as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                1 as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else if strlen((lumpnames[21 as libc::c_int as usize]).as_mut_ptr())
            <= 6 as libc::c_int as libc::c_ulong
        {
            let mut cnt_20: fixed_t = strlen(
                (lumpnames[21 as libc::c_int as usize]).as_mut_ptr(),
            ) as fixed_t;
            strncpy(
                lumpname.as_mut_ptr(),
                (lumpnames[21 as libc::c_int as usize]).as_mut_ptr(),
                7 as libc::c_int as libc::c_ulong,
            );
            i = 0 as libc::c_int as uint16_t;
            while (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                sprintf(
                    &mut *lumpname.as_mut_ptr().offset(cnt_20 as isize)
                        as *mut libc::c_char,
                    b"%.2hu\0" as *const u8 as *const libc::c_char,
                    (i as libc::c_int + 1 as libc::c_int) as uint16_t as libc::c_int,
                );
                lumpname[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
                lumpnum = W_CheckNumForName(lumpname.as_mut_ptr());
                if !(lumpnum != 4294967295 as libc::c_uint) {
                    break;
                }
                ttkndh[newttscale
                    as usize][i
                    as usize] = W_CachePatchName(
                    lumpname.as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t;
                i = i.wrapping_add(1);
                i;
            }
            ttkndh[newttscale
                as usize][(if (i as libc::c_int) < 30 as libc::c_int - 1 as libc::c_int {
                i as libc::c_int
            } else {
                30 as libc::c_int - 1 as libc::c_int
            }) as usize] = 0 as *mut patch_t;
        } else {
            ttkndh[newttscale as usize][0 as libc::c_int as usize] = 0 as *mut patch_t;
        }
        ttloaded[newttscale as usize] = true_0 as libc::c_int;
    }
}
unsafe extern "C" fn F_FigureActiveTtScale() {
    let mut newttscale: int8_t = (if 1 as libc::c_int
        > (if (6 as libc::c_int) < vid.dupx { 6 as libc::c_int } else { vid.dupx })
    {
        1 as libc::c_int
    } else if (6 as libc::c_int) < vid.dupx {
        6 as libc::c_int
    } else {
        vid.dupx
    }) as int8_t;
    let mut oldttscale: int8_t = activettscale;
    if newttscale as libc::c_int == testttscale as libc::c_int {
        return;
    }
    if oldttscale as libc::c_int > 0 as libc::c_int {
        F_UnloadAlacroixGraphics(oldttscale);
    }
    testttscale = newttscale;
    while newttscale as libc::c_int >= 1 as libc::c_int {
        if ttavailable[(newttscale as libc::c_int - 1 as libc::c_int) as usize] != 0 {
            break;
        }
        newttscale -= 1;
        newttscale;
    }
    while newttscale as libc::c_int <= 6 as libc::c_int {
        if ttavailable[(newttscale as libc::c_int - 1 as libc::c_int) as usize] != 0 {
            break;
        }
        newttscale += 1;
        newttscale;
    }
    activettscale = (if newttscale as libc::c_int >= 1 as libc::c_int
        && newttscale as libc::c_int <= 6 as libc::c_int
    {
        newttscale as libc::c_int
    } else {
        0 as libc::c_int
    }) as int8_t;
    if activettscale as libc::c_int > 0 as libc::c_int {
        F_LoadAlacroixGraphics(activettscale);
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_TitleScreenDrawer() {
    let mut hidepics: boolean = 0;
    let mut sc: fixed_t = ((1 as libc::c_int) << 16 as libc::c_int)
        / (if 1 as libc::c_int > curttscale as libc::c_int {
            1 as libc::c_int
        } else {
            curttscale as libc::c_int
        });
    let mut whitefade: int32_t = 0 as libc::c_int;
    let mut whitecol: [*mut uint8_t; 2] = [0 as *mut uint8_t, 0 as *mut uint8_t];
    if modeattacking != 0 {
        return;
    }
    if curbgcolor >= 0 as libc::c_int {
        V_DrawFill(
            0 as libc::c_int,
            0 as libc::c_int,
            320 as libc::c_int,
            200 as libc::c_int,
            curbgcolor,
        );
    } else if curbghide == 0 || titlemapinaction == 0
        || gamestate as libc::c_uint == GS_WAITINGPLAYERS as libc::c_int as libc::c_uint
    {
        F_SkyScroll(curbgname.as_mut_ptr());
    }
    if gamestate as libc::c_uint != GS_TITLESCREEN as libc::c_int as libc::c_uint
        && gamestate as libc::c_uint != GS_WAITINGPLAYERS as libc::c_int as libc::c_uint
    {
        return;
    }
    if ttwing.is_null()
        && (curttmode as libc::c_uint == TTMODE_OLD as libc::c_int as libc::c_uint
            || curttmode as libc::c_uint == TTMODE_NONE as libc::c_int as libc::c_uint)
    {
        return;
    }
    hidepics = curhidepics;
    if !(hidepics != 0) {
        let mut current_block_458: u64;
        match curttmode as libc::c_uint {
            1 | 0 => {
                V_DrawStretchyFixedPatch(
                    (30 as libc::c_int) << 16 as libc::c_int,
                    (14 as libc::c_int) << 16 as libc::c_int,
                    sc,
                    sc,
                    0 as libc::c_int,
                    ttwing,
                    0 as *const uint8_t,
                );
                if finalecount < 57 as libc::c_int {
                    if finalecount == 35 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (115 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop1,
                            0 as *const uint8_t,
                        );
                    } else if finalecount == 36 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (114 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop2,
                            0 as *const uint8_t,
                        );
                    } else if finalecount == 37 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (113 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop3,
                            0 as *const uint8_t,
                        );
                    } else if finalecount == 38 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (112 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop4,
                            0 as *const uint8_t,
                        );
                    } else if finalecount == 39 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (111 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop5,
                            0 as *const uint8_t,
                        );
                    } else if finalecount == 40 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (110 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop6,
                            0 as *const uint8_t,
                        );
                    } else if finalecount >= 41 as libc::c_int
                        && finalecount <= 44 as libc::c_int
                    {
                        V_DrawStretchyFixedPatch(
                            (109 as libc::c_int) << 16 as libc::c_int,
                            (15 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttspop7,
                            0 as *const uint8_t,
                        );
                    } else if finalecount >= 45 as libc::c_int
                        && finalecount <= 48 as libc::c_int
                    {
                        V_DrawStretchyFixedPatch(
                            (108 as libc::c_int) << 16 as libc::c_int,
                            (12 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttsprep1,
                            0 as *const uint8_t,
                        );
                    } else if finalecount >= 49 as libc::c_int
                        && finalecount <= 52 as libc::c_int
                    {
                        V_DrawStretchyFixedPatch(
                            (107 as libc::c_int) << 16 as libc::c_int,
                            (9 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttsprep2,
                            0 as *const uint8_t,
                        );
                    } else if finalecount >= 53 as libc::c_int
                        && finalecount <= 56 as libc::c_int
                    {
                        V_DrawStretchyFixedPatch(
                            (106 as libc::c_int) << 16 as libc::c_int,
                            (6 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttswip1,
                            0 as *const uint8_t,
                        );
                    }
                    V_DrawStretchyFixedPatch(
                        (93 as libc::c_int) << 16 as libc::c_int,
                        (106 as libc::c_int) << 16 as libc::c_int,
                        sc,
                        sc,
                        0 as libc::c_int,
                        ttsonic,
                        0 as *const uint8_t,
                    );
                } else {
                    V_DrawStretchyFixedPatch(
                        (93 as libc::c_int) << 16 as libc::c_int,
                        (106 as libc::c_int) << 16 as libc::c_int,
                        sc,
                        sc,
                        0 as libc::c_int,
                        ttsonic,
                        0 as *const uint8_t,
                    );
                    if finalecount / 5 as libc::c_int & 1 as libc::c_int != 0 {
                        V_DrawStretchyFixedPatch(
                            (100 as libc::c_int) << 16 as libc::c_int,
                            (3 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttswave1,
                            0 as *const uint8_t,
                        );
                    } else {
                        V_DrawStretchyFixedPatch(
                            (100 as libc::c_int) << 16 as libc::c_int,
                            (3 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttswave2,
                            0 as *const uint8_t,
                        );
                    }
                }
                V_DrawStretchyFixedPatch(
                    (48 as libc::c_int) << 16 as libc::c_int,
                    (142 as libc::c_int) << 16 as libc::c_int,
                    sc,
                    sc,
                    0 as libc::c_int,
                    ttbanner,
                    0 as *const uint8_t,
                );
            }
            2 => {
                F_FigureActiveTtScale();
                if !(activettscale == 0) {
                    sc = ((1 as libc::c_int) << 16 as libc::c_int)
                        / activettscale as libc::c_int;
                    if finalecount <= 29 as libc::c_int {
                        V_DrawFill(
                            0 as libc::c_int,
                            0 as libc::c_int,
                            320 as libc::c_int,
                            200 as libc::c_int,
                            31 as libc::c_int,
                        );
                    } else {
                        if finalecount > 29 as libc::c_int
                            && finalecount < 35 as libc::c_int
                        {
                            whitefade = 9 as libc::c_int;
                            V_DrawFadeScreen(
                                0 as libc::c_int as uint16_t,
                                whitefade as uint8_t,
                            );
                        } else if finalecount > 34 as libc::c_int
                            && 44 as libc::c_int - finalecount > 0 as libc::c_int
                            && 44 as libc::c_int - finalecount < 10 as libc::c_int
                        {
                            V_DrawFadeScreen(
                                0 as libc::c_int as uint16_t,
                                (44 as libc::c_int - finalecount) as uint8_t,
                            );
                        }
                        if 39 as libc::c_int - finalecount > 0 as libc::c_int {
                            whitefade = 9 as libc::c_int
                                - (39 as libc::c_int - finalecount) << 16 as libc::c_int;
                            whitecol[0 as libc::c_int
                                as usize] = R_GetTranslationColormap(
                                TC_RAINBOW as libc::c_int,
                                SKINCOLOR_SUPERGOLD3,
                                1 as libc::c_int as uint8_t,
                            );
                            whitecol[1 as libc::c_int
                                as usize] = R_GetTranslationColormap(
                                TC_ALLWHITE as libc::c_int,
                                SKINCOLOR_NONE,
                                1 as libc::c_int as uint8_t,
                            );
                        }
                    }
                    V_DrawStretchyFixedPatch(
                        (40 as libc::c_int) << 16 as libc::c_int,
                        (20 as libc::c_int) << 16 as libc::c_int,
                        sc,
                        sc,
                        0 as libc::c_int,
                        ttembl[(activettscale as libc::c_int - 1 as libc::c_int)
                            as usize][0 as libc::c_int as usize],
                        0 as *const uint8_t,
                    );
                    if !(whitecol[0 as libc::c_int as usize]).is_null() {
                        V_DrawStretchyFixedPatch(
                            (40 as libc::c_int) << 16 as libc::c_int,
                            (20 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            whitefade,
                            ttembl[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                            whitecol[0 as libc::c_int as usize],
                        );
                        V_DrawStretchyFixedPatch(
                            (40 as libc::c_int) << 16 as libc::c_int,
                            (20 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0x50000 as libc::c_int
                                + (whitefade / 2 as libc::c_int & 0xf0000 as libc::c_int),
                            ttembl[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                            whitecol[1 as libc::c_int as usize],
                        );
                    }
                    if finalecount <= 29 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (39 as libc::c_int) << 16 as libc::c_int,
                            (88 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttribb[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][(if (if 0 as libc::c_int > finalecount {
                                0 as libc::c_int
                            } else {
                                finalecount
                            }) < 24 as libc::c_int
                            {
                                if 0 as libc::c_int > finalecount {
                                    0 as libc::c_int
                                } else {
                                    finalecount
                                }
                            } else {
                                24 as libc::c_int
                            }) as usize],
                            0 as *const uint8_t,
                        );
                        V_DrawFadeScreen(
                            0xff00 as libc::c_int as uint16_t,
                            12 as libc::c_int as uint8_t,
                        );
                        if finalecount >= 0 as libc::c_int {
                            V_DrawStretchyFixedPatch(
                                (89 as libc::c_int) << 16 as libc::c_int,
                                (92 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                ttsont[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][(if finalecount < 28 as libc::c_int {
                                    finalecount
                                } else {
                                    28 as libc::c_int
                                }) as usize],
                                0 as *const uint8_t,
                            );
                        }
                        if finalecount > 9 as libc::c_int {
                            let mut fadeval: int32_t = 0 as libc::c_int;
                            if finalecount < 30 as libc::c_int {
                                let mut fadecounter: uint8_t = (30 as libc::c_int
                                    - finalecount) as uint8_t;
                                match fadecounter as libc::c_int {
                                    20 | 19 => {
                                        fadeval = 0x90000 as libc::c_int;
                                    }
                                    18 | 17 => {
                                        fadeval = 0x80000 as libc::c_int;
                                    }
                                    16 | 15 => {
                                        fadeval = 0x70000 as libc::c_int;
                                    }
                                    14 | 13 => {
                                        fadeval = 0x60000 as libc::c_int;
                                    }
                                    12 | 11 => {
                                        fadeval = 0x50000 as libc::c_int;
                                    }
                                    10 | 9 => {
                                        fadeval = 0x40000 as libc::c_int;
                                    }
                                    8 | 7 => {
                                        fadeval = 0x30000 as libc::c_int;
                                    }
                                    6 | 5 => {
                                        fadeval = 0x20000 as libc::c_int;
                                    }
                                    4 | 3 => {
                                        fadeval = 0x10000 as libc::c_int;
                                    }
                                    _ => {}
                                }
                            }
                            V_DrawStretchyFixedPatch(
                                (79 as libc::c_int) << 16 as libc::c_int,
                                (132 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                fadeval,
                                ttrobo[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                            if finalecount > 15 as libc::c_int {
                                V_DrawStretchyFixedPatch(
                                    (106 as libc::c_int) << 16 as libc::c_int,
                                    (118 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    fadeval,
                                    tttwot[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][(if (finalecount - 16 as libc::c_int)
                                        < 15 as libc::c_int
                                    {
                                        finalecount - 16 as libc::c_int
                                    } else {
                                        15 as libc::c_int
                                    }) as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount
                        >= 41 as libc::c_int + 44 as libc::c_int + 70 as libc::c_int
                    {
                        if knux_idle_start == 0
                            || finalecount - knux_idle_start >= knux_idle_end
                        {
                            if knux_blink != 0 {
                                knux_blink = false_0 as libc::c_int;
                                knux_blinked_already = true_0 as libc::c_int;
                            } else if knux_blinked_already != 0 {
                                knux_blinked_already = false_0 as libc::c_int;
                            } else {
                                knux_blink = (M_RandomKey(100 as libc::c_int)
                                    % 2 as libc::c_int == 0) as libc::c_int;
                                knux_blink_twice = if knux_blink != 0 {
                                    (M_RandomKey(100 as libc::c_int) % 5 as libc::c_int == 0)
                                        as libc::c_int
                                } else {
                                    false_0 as libc::c_int
                                };
                            }
                            knux_idle_start = finalecount;
                        }
                        knux_idle_end = if knux_blink != 0 {
                            if knux_blink_twice != 0 {
                                17 as libc::c_int
                            } else {
                                7 as libc::c_int
                            }
                        } else {
                            46 as libc::c_int
                        };
                    }
                    if finalecount
                        >= 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                    {
                        if tails_idle_start == 0
                            || finalecount - tails_idle_start >= tails_idle_end
                        {
                            if tails_blink != 0 {
                                tails_blink = false_0 as libc::c_int;
                                tails_blinked_already = true_0 as libc::c_int;
                            } else if tails_blinked_already != 0 {
                                tails_blinked_already = false_0 as libc::c_int;
                            } else {
                                tails_blink = (M_RandomKey(100 as libc::c_int)
                                    % 3 as libc::c_int == 0) as libc::c_int;
                                tails_blink_twice = if tails_blink != 0 {
                                    (M_RandomKey(100 as libc::c_int) % 5 as libc::c_int == 0)
                                        as libc::c_int
                                } else {
                                    false_0 as libc::c_int
                                };
                            }
                            tails_idle_start = finalecount;
                        }
                        tails_idle_end = if tails_blink != 0 {
                            if tails_blink_twice != 0 {
                                17 as libc::c_int
                            } else {
                                7 as libc::c_int
                            }
                        } else {
                            30 as libc::c_int
                        };
                    }
                    if finalecount
                        >= 41 as libc::c_int + 0 as libc::c_int + 57 as libc::c_int
                    {
                        if sonic_idle_start == 0
                            || finalecount - sonic_idle_start >= sonic_idle_end
                        {
                            if sonic_blink != 0 {
                                sonic_blink = false_0 as libc::c_int;
                                sonic_blinked_already = true_0 as libc::c_int;
                            } else if sonic_blinked_already != 0 {
                                sonic_blinked_already = false_0 as libc::c_int;
                            } else {
                                sonic_blink = (M_RandomKey(100 as libc::c_int)
                                    % 3 as libc::c_int == 0) as libc::c_int;
                                sonic_blink_twice = if sonic_blink != 0 {
                                    (M_RandomKey(100 as libc::c_int) % 5 as libc::c_int == 0)
                                        as libc::c_int
                                } else {
                                    false_0 as libc::c_int
                                };
                            }
                            sonic_idle_start = finalecount;
                        }
                        sonic_idle_end = if sonic_blink != 0 {
                            if sonic_blink_twice != 0 {
                                17 as libc::c_int
                            } else {
                                7 as libc::c_int
                            }
                        } else {
                            25 as libc::c_int
                        };
                    }
                    if finalecount >= 41 as libc::c_int + 27 as libc::c_int {
                        if finalecount
                            >= 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                        {
                            let mut taftcount: int8_t = ((finalecount
                                - (41 as libc::c_int + 27 as libc::c_int
                                    + 60 as libc::c_int)) % 41 as libc::c_int) as int8_t;
                            if taftcount as libc::c_int >= 0 as libc::c_int
                                && (taftcount as libc::c_int) < 5 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 5 as libc::c_int
                                && (taftcount as libc::c_int) < 9 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 9 as libc::c_int
                                && (taftcount as libc::c_int) < 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 12 as libc::c_int
                                && (taftcount as libc::c_int) < 14 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 14 as libc::c_int
                                && (taftcount as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 17 as libc::c_int
                                && (taftcount as libc::c_int) < 21 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 21 as libc::c_int
                                && (taftcount as libc::c_int) < 24 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 24 as libc::c_int
                                && (taftcount as libc::c_int) < 25 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 25 as libc::c_int
                                && (taftcount as libc::c_int) < 28 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 28 as libc::c_int
                                && (taftcount as libc::c_int) < 31 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 31 as libc::c_int
                                && (taftcount as libc::c_int) < 35 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if taftcount as libc::c_int >= 35 as libc::c_int
                                && (taftcount as libc::c_int) < 41 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabt[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 27 as libc::c_int {
                        if finalecount
                            >= 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                        {
                            let mut tabtcount: int8_t = ((finalecount
                                - (41 as libc::c_int + 27 as libc::c_int
                                    + 60 as libc::c_int)) % 41 as libc::c_int) as int8_t;
                            if tabtcount as libc::c_int >= 0 as libc::c_int
                                && (tabtcount as libc::c_int) < 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 6 as libc::c_int
                                && (tabtcount as libc::c_int) < 11 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 11 as libc::c_int
                                && (tabtcount as libc::c_int) < 15 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 15 as libc::c_int
                                && (tabtcount as libc::c_int) < 18 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 18 as libc::c_int
                                && (tabtcount as libc::c_int) < 19 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 19 as libc::c_int
                                && (tabtcount as libc::c_int) < 22 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 22 as libc::c_int
                                && (tabtcount as libc::c_int) < 27 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 27 as libc::c_int
                                && (tabtcount as libc::c_int) < 30 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 30 as libc::c_int
                                && (tabtcount as libc::c_int) < 31 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 31 as libc::c_int
                                && (tabtcount as libc::c_int) < 34 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 34 as libc::c_int
                                && (tabtcount as libc::c_int) < 37 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if tabtcount as libc::c_int >= 37 as libc::c_int
                                && (tabtcount as libc::c_int) < 41 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaft[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 44 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 44 as libc::c_int + 70 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 0 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 6 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 10 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 13 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 13 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 15 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 15 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 18 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 18 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 22 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 22 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 28 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 28 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 32 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 32 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 35 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 35 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 40 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 40 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 41 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 41 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 44 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 44 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 50 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][12 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 50 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 56 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][13 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 56 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 57 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][14 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 57 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 60 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][15 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 60 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 63 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][16 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 63 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 67 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][17 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 67 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 70 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][18 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else if knux_blink == 0 {
                            V_DrawStretchyFixedPatch(
                                (167 as libc::c_int) << 16 as libc::c_int,
                                (7 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                ttknba[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        } else {
                            let mut idlecount: int8_t = (finalecount - knux_idle_start)
                                as int8_t;
                            if idlecount as libc::c_int >= 0 as libc::c_int
                                && (idlecount as libc::c_int) < 2 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 2 as libc::c_int
                                && (idlecount as libc::c_int) < 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 6 as libc::c_int
                                && (idlecount as libc::c_int) < 7 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 7 as libc::c_int
                                && (idlecount as libc::c_int) < 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknba[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 10 as libc::c_int
                                && (idlecount as libc::c_int) < 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 12 as libc::c_int
                                && (idlecount as libc::c_int) < 16 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount as libc::c_int >= 16 as libc::c_int
                                && (idlecount as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknbk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 27 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 0 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 6 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 10 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 12 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 16 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 16 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 22 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 22 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 23 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 23 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 26 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 26 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 30 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 30 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 35 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 35 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 41 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 41 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 43 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 43 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 47 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 47 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 51 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][12 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 51 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 53 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][13 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 53 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 56 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][14 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 56 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][15 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else if tails_blink == 0 {
                            V_DrawStretchyFixedPatch(
                                (35 as libc::c_int) << 16 as libc::c_int,
                                (19 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                tttaba[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        } else {
                            let mut idlecount_0: int8_t = (finalecount
                                - tails_idle_start) as int8_t;
                            if idlecount_0 as libc::c_int >= 0 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 2 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 2 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 6 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 7 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 7 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaba[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 10 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 12 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 16 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_0 as libc::c_int >= 16 as libc::c_int
                                && (idlecount_0 as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttabk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 0 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 0 as libc::c_int + 57 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 0 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 6 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 11 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 11 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 14 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 14 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 18 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 18 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 19 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 19 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 27 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 27 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 31 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 33 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 36 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 36 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 40 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 40 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 44 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 44 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 47 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 47 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 49 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][12 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 49 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 50 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][13 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 50 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 53 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][14 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 53 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 57 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoib[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][15 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else if sonic_blink == 0 {
                            V_DrawStretchyFixedPatch(
                                (89 as libc::c_int) << 16 as libc::c_int,
                                (13 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                ttsoba[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        } else {
                            let mut idlecount_1: int8_t = (finalecount
                                - sonic_idle_start) as int8_t;
                            if idlecount_1 as libc::c_int >= 0 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 2 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 2 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 6 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 6 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 7 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 7 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoba[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 10 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 12 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 16 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_1 as libc::c_int >= 16 as libc::c_int
                                && (idlecount_1 as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsobk[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount > 29 as libc::c_int {
                        V_DrawStretchyFixedPatch(
                            (39 as libc::c_int) << 16 as libc::c_int,
                            (93 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttrbtx[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                            0 as *const uint8_t,
                        );
                    }
                    if !(whitecol[0 as libc::c_int as usize]).is_null() {
                        V_DrawStretchyFixedPatch(
                            (39 as libc::c_int) << 16 as libc::c_int,
                            (93 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            whitefade,
                            ttrbtx[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                            whitecol[0 as libc::c_int as usize],
                        );
                        V_DrawStretchyFixedPatch(
                            (39 as libc::c_int) << 16 as libc::c_int,
                            (93 as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0x50000 as libc::c_int
                                + (whitefade / 2 as libc::c_int & 0xf0000 as libc::c_int),
                            ttrbtx[(activettscale as libc::c_int - 1 as libc::c_int)
                                as usize][0 as libc::c_int as usize],
                            whitecol[1 as libc::c_int as usize],
                        );
                    }
                    if finalecount >= 41 as libc::c_int + 44 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 44 as libc::c_int + 70 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 22 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 28 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 28 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 32 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 44 as libc::c_int + 32 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 44 as libc::c_int + 35 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttknif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else if knux_blink == 0 {
                            let mut idlecount_2: int8_t = (finalecount - knux_idle_start)
                                as int8_t;
                            if idlecount_2 as libc::c_int >= 0 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 5 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 5 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 10 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 10 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 13 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 13 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 14 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 14 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 17 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 21 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 21 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 27 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 27 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 32 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 32 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 34 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 34 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 37 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 37 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 39 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 39 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 42 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_2 as libc::c_int >= 42 as libc::c_int
                                && (idlecount_2 as libc::c_int) < 46 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (167 as libc::c_int) << 16 as libc::c_int,
                                    (7 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][12 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else {
                            V_DrawStretchyFixedPatch(
                                (167 as libc::c_int) << 16 as libc::c_int,
                                (7 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                ttkndh[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 27 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 27 as libc::c_int + 60 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 26 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 30 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 30 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 35 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 35 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 41 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 41 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 43 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 43 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 47 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 27 as libc::c_int + 47 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 27 as libc::c_int + 51 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (35 as libc::c_int) << 16 as libc::c_int,
                                    (19 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    tttaif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][12 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        }
                    }
                    if finalecount >= 41 as libc::c_int + 0 as libc::c_int {
                        if finalecount
                            < 41 as libc::c_int + 0 as libc::c_int + 57 as libc::c_int
                        {
                            if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 19 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 27 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 27 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 31 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 31 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 33 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 33 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 36 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 36 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 40 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][9 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 40 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 44 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][10 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 44 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 47 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][11 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if finalecount
                                >= 41 as libc::c_int + 0 as libc::c_int + 53 as libc::c_int
                                && finalecount
                                    < 41 as libc::c_int + 0 as libc::c_int + 57 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsoif[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][15 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else if sonic_blink == 0 {
                            let mut idlecount_3: int8_t = (finalecount
                                - sonic_idle_start) as int8_t;
                            if idlecount_3 as libc::c_int >= 0 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 5 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][0 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 5 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 8 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][1 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 8 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 9 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][2 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 9 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 12 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][3 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 12 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 17 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][4 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 17 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 19 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][5 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 19 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 21 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][6 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 21 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 22 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][7 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            } else if idlecount_3 as libc::c_int >= 22 as libc::c_int
                                && (idlecount_3 as libc::c_int) < 25 as libc::c_int
                            {
                                V_DrawStretchyFixedPatch(
                                    (89 as libc::c_int) << 16 as libc::c_int,
                                    (13 as libc::c_int) << 16 as libc::c_int,
                                    sc,
                                    sc,
                                    0 as libc::c_int,
                                    ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                        as usize][8 as libc::c_int as usize],
                                    0 as *const uint8_t,
                                );
                            }
                        } else {
                            V_DrawStretchyFixedPatch(
                                (89 as libc::c_int) << 16 as libc::c_int,
                                (13 as libc::c_int) << 16 as libc::c_int,
                                sc,
                                sc,
                                0 as libc::c_int,
                                ttsodh[(activettscale as libc::c_int - 1 as libc::c_int)
                                    as usize][0 as libc::c_int as usize],
                                0 as *const uint8_t,
                            );
                        }
                    }
                }
            }
            3 => {
                if (ttuser[(if 0 as libc::c_int > ttuser_count {
                    0 as libc::c_int
                } else {
                    ttuser_count
                }) as usize])
                    .is_null()
                {
                    if curttloop as libc::c_int > -(1 as libc::c_int)
                        && !(ttuser[curttloop as usize]).is_null()
                    {
                        ttuser_count = curttloop as int32_t;
                        current_block_458 = 6835922529576182797;
                    } else if !(ttuser[(if 0 as libc::c_int
                        > ttuser_count - 1 as libc::c_int
                    {
                        0 as libc::c_int
                    } else {
                        ttuser_count - 1 as libc::c_int
                    }) as usize])
                        .is_null()
                    {
                        ttuser_count = if 0 as libc::c_int
                            > ttuser_count - 1 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            ttuser_count - 1 as libc::c_int
                        };
                        current_block_458 = 6835922529576182797;
                    } else {
                        current_block_458 = 5996944888495321466;
                    }
                } else {
                    current_block_458 = 6835922529576182797;
                }
                match current_block_458 {
                    5996944888495321466 => {}
                    _ => {
                        V_DrawStretchyFixedPatch(
                            (curttx as libc::c_int) << 16 as libc::c_int,
                            (curtty as libc::c_int) << 16 as libc::c_int,
                            sc,
                            sc,
                            0 as libc::c_int,
                            ttuser[ttuser_count as usize],
                            0 as *const uint8_t,
                        );
                        if finalecount
                            % (if 1 as libc::c_int > curtttics as libc::c_int {
                                1 as libc::c_int
                            } else {
                                curtttics as libc::c_int
                            }) == 0
                        {
                            ttuser_count += 1;
                            ttuser_count;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if LUA_HUD_IsDrawListValid(luahuddrawlist_title) == 0 {
        LUA_HUD_DestroyDrawList(luahuddrawlist_title);
        luahuddrawlist_title = LUA_HUD_CreateDrawList();
    }
    if renderisnewtic != 0 {
        LUA_HUD_ClearDrawList(luahuddrawlist_title);
        LUA_HookHUD(hudhook_title as libc::c_int, luahuddrawlist_title);
    }
    LUA_HUD_DrawList(luahuddrawlist_title);
}
#[no_mangle]
pub unsafe extern "C" fn F_MenuPresTicker() {
    curbgx += curbgxspeed;
    curbgy += curbgyspeed;
}
#[no_mangle]
pub unsafe extern "C" fn F_TitleScreenTicker(mut run: boolean) {
    if run != 0 {
        finalecount += 1;
        finalecount;
    }
    if gameaction as libc::c_uint != ga_nothing as libc::c_int as libc::c_uint
        || gamestate as libc::c_uint != GS_TITLESCREEN as libc::c_int as libc::c_uint
    {
        return;
    }
    if titlemapinaction != 0 {
        let mut th: *mut thinker_t = 0 as *mut thinker_t;
        let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
        let mut cameraref: *mut mobj_t = 0 as *mut mobj_t;
        if titlemapcameraref.is_null()
            || (*titlemapcameraref).type_0 as libc::c_uint
                != MT_ALTVIEWMAN as libc::c_int as libc::c_uint
        {
            th = (*thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)).next;
            while th
                != &mut *thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)
                    as *mut thinker_t
            {
                if !((*th).function.acp1
                    == ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut thinker_t) -> ()>,
                        actionf_p1,
                    >(
                        Some(
                            P_RemoveThinkerDelayed
                                as unsafe extern "C" fn(*mut thinker_t) -> (),
                        ),
                    ))
                {
                    mo2 = th as *mut mobj_t;
                    if !mo2.is_null() {
                        if !((*mo2).type_0 as libc::c_uint
                            != MT_ALTVIEWMAN as libc::c_int as libc::c_uint)
                        {
                            titlemapcameraref = mo2;
                            cameraref = titlemapcameraref;
                            break;
                        }
                    }
                }
                th = (*th).next;
            }
        } else {
            cameraref = titlemapcameraref;
        }
        if !cameraref.is_null() {
            camera.x = (*cameraref).x;
            camera.y = (*cameraref).y;
            camera.z = (*cameraref).z;
            camera.angle = (*cameraref).angle;
            camera.aiming = (*cameraref).cusval as angle_t;
            camera.subsector = (*cameraref).subsector;
        } else {
            camera
                .angle = (camera.angle)
                .wrapping_add(
                    (titlescrollxspeed * 0xb60b61 as libc::c_int / 64 as libc::c_int)
                        as angle_t,
                );
        }
    }
    if cv_rollingdemos.value == 0 || numDemos == 0 {
        return;
    }
    if demoDelayLeft != 0 {
        demoDelayLeft = demoDelayLeft.wrapping_sub(1);
        demoDelayLeft;
        return;
    }
    if menuactive != 0 || CON_Ready() != 0 {
        demoIdleLeft = demoIdleTime;
        return;
    }
    demoIdleLeft = demoIdleLeft.wrapping_sub(1);
    if demoIdleLeft == 0 {
        let mut dname: [libc::c_char; 9] = [0; 9];
        let mut l: lumpnum_t = 0;
        demoIdleLeft = demoIdleTime;
        if curDemo as libc::c_int == numDemos as libc::c_int {
            curDemo = 0 as libc::c_int as uint8_t;
            F_StartIntro();
            return;
        }
        curDemo = curDemo.wrapping_add(1);
        snprintf(
            dname.as_mut_ptr(),
            9 as libc::c_int as libc::c_ulong,
            b"DEMO_%03u\0" as *const u8 as *const libc::c_char,
            curDemo as libc::c_int,
        );
        l = W_CheckNumForName(dname.as_mut_ptr());
        if l == 4294967295 as libc::c_uint {
            CONS_Alert(
                CONS_ERROR,
                b"Demo lump \"%s\" doesn't exist\n\0" as *const u8
                    as *const libc::c_char,
                dname.as_mut_ptr(),
            );
            F_StartIntro();
            return;
        }
        titledemo = true_0 as libc::c_int;
        demofileoverride = DFILE_OVERRIDE_NONE;
        G_DoPlayDemo(dname.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_TitleDemoTicker() {
    keypressed = false_0 as libc::c_int;
}
static mut contskins: [*mut skin_t; 2] = [0 as *const skin_t as *mut skin_t; 2];
static mut cont_spr2: [[uint8_t; 6]; 2] = [[0; 6]; 2];
static mut contcolormaps: [*mut uint8_t; 2] = [0 as *const uint8_t as *mut uint8_t; 2];
#[no_mangle]
pub unsafe extern "C" fn F_StartContinue() {
    if multiplayer == 0
        && (ultimatemode as libc::c_int != 0
            || useContinues as libc::c_int != 0 && marathonmode as u64 == 0
            || modeattacking == 0 && !(cursaveslot > 0 as libc::c_int))
        && players[consoleplayer as usize].continues as libc::c_int <= 0 as libc::c_int
    {
        Command_ExitGame_f();
        return;
    }
    wipestyleflags = WSF_FADEOUT;
    G_SetGamestate(GS_CONTINUING);
    gameaction = ga_nothing;
    keypressed = false_0 as libc::c_int;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    M_ClearMenus(true_0 as libc::c_int);
    S_ChangeMusicEx(
        b"_conti\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as uint16_t,
        false_0 as libc::c_int,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
        0 as libc::c_int as uint32_t,
    );
    S_StopSounds();
    contskins[0 as libc::c_int
        as usize] = &mut *skins
        .as_mut_ptr()
        .offset((*players.as_mut_ptr().offset(consoleplayer as isize)).skin as isize)
        as *mut skin_t;
    cont_spr2[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = P_GetSkinSprite2(
        contskins[0 as libc::c_int as usize],
        SPR2_CNT1 as libc::c_int as uint8_t,
        0 as *mut player_t,
    );
    cont_spr2[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize] = ((*contskins[0 as libc::c_int as usize]).contangle as libc::c_int
        & 7 as libc::c_int) as uint8_t;
    contcolormaps[0 as libc::c_int
        as usize] = R_GetTranslationColormap(
        players[consoleplayer as usize].skin,
        players[consoleplayer as usize].skincolor as skincolornum_t,
        1 as libc::c_int as uint8_t,
    );
    cont_spr2[0 as libc::c_int
        as usize][4 as libc::c_int
        as usize] = (*contskins[0 as libc::c_int as usize])
        .sprites[cont_spr2[0 as libc::c_int as usize][0 as libc::c_int as usize]
            as usize]
        .numframes as uint8_t;
    cont_spr2[0 as libc::c_int
        as usize][5 as libc::c_int
        as usize] = (if 1 as libc::c_int
        > (*contskins[0 as libc::c_int as usize]).contspeed as libc::c_int
    {
        1 as libc::c_int
    } else {
        (*contskins[0 as libc::c_int as usize]).contspeed as libc::c_int
    }) as uint8_t;
    if botskin != 0 {
        let mut secondplaya: int32_t = 0;
        if secondarydisplayplayer != consoleplayer {
            secondplaya = secondarydisplayplayer;
        } else {
            secondplaya = 1 as libc::c_int;
        }
        contskins[1 as libc::c_int
            as usize] = &mut *skins
            .as_mut_ptr()
            .offset((*players.as_mut_ptr().offset(secondplaya as isize)).skin as isize)
            as *mut skin_t;
        cont_spr2[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = P_GetSkinSprite2(
            contskins[1 as libc::c_int as usize],
            SPR2_CNT4 as libc::c_int as uint8_t,
            0 as *mut player_t,
        );
        cont_spr2[1 as libc::c_int
            as usize][2 as libc::c_int
            as usize] = ((*contskins[1 as libc::c_int as usize]).contangle as libc::c_int
            >> 3 as libc::c_int & 7 as libc::c_int) as uint8_t;
        contcolormaps[1 as libc::c_int
            as usize] = R_GetTranslationColormap(
            players[secondplaya as usize].skin,
            players[secondplaya as usize].skincolor as skincolornum_t,
            1 as libc::c_int as uint8_t,
        );
        cont_spr2[1 as libc::c_int
            as usize][4 as libc::c_int
            as usize] = (*contskins[1 as libc::c_int as usize])
            .sprites[cont_spr2[1 as libc::c_int as usize][0 as libc::c_int as usize]
                as usize]
            .numframes as uint8_t;
        if cont_spr2[1 as libc::c_int as usize][0 as libc::c_int as usize] as libc::c_int
            == SPR2_CNT4 as libc::c_int
        {
            cont_spr2[1 as libc::c_int
                as usize][5 as libc::c_int as usize] = 4 as libc::c_int as uint8_t;
        } else {
            cont_spr2[1 as libc::c_int
                as usize][5 as libc::c_int
                as usize] = (if 1 as libc::c_int
                > (*contskins[1 as libc::c_int as usize]).contspeed as libc::c_int
            {
                1 as libc::c_int
            } else {
                (*contskins[1 as libc::c_int as usize]).contspeed as libc::c_int
            }) as uint8_t;
        }
    } else {
        contskins[1 as libc::c_int as usize] = 0 as *mut skin_t;
        contcolormaps[1 as libc::c_int as usize] = 0 as *mut uint8_t;
        cont_spr2[1 as libc::c_int
            as usize][5 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        cont_spr2[1 as libc::c_int
            as usize][4 as libc::c_int
            as usize] = cont_spr2[1 as libc::c_int as usize][5 as libc::c_int as usize];
        cont_spr2[1 as libc::c_int
            as usize][2 as libc::c_int
            as usize] = cont_spr2[1 as libc::c_int as usize][4 as libc::c_int as usize];
        cont_spr2[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = cont_spr2[1 as libc::c_int as usize][2 as libc::c_int as usize];
    }
    cont_spr2[1 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    cont_spr2[1 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = cont_spr2[1 as libc::c_int as usize][3 as libc::c_int as usize];
    cont_spr2[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = cont_spr2[1 as libc::c_int as usize][1 as libc::c_int as usize];
    cont_spr2[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = cont_spr2[0 as libc::c_int as usize][3 as libc::c_int as usize];
    timetonext = 11 as libc::c_int * 35 as libc::c_int + 11 as libc::c_int;
    continuetime = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_ContinueDrawer() {
    let mut sprdef: *mut spritedef_t = 0 as *mut spritedef_t;
    let mut sprframe: *mut spriteframe_t = 0 as *mut spriteframe_t;
    let mut patch: *mut patch_t = 0 as *mut patch_t;
    let mut i: int32_t = 0;
    let mut x: int32_t = 320 as libc::c_int >> 1 as libc::c_int;
    let mut ncontinues: int32_t = players[consoleplayer as usize].continues as int32_t;
    let mut numbuf: [libc::c_char; 9] = *::core::mem::transmute::<
        &[u8; 9],
        &mut [libc::c_char; 9],
    >(b"CONTNUM*\0");
    let mut timeleft: tic_t = (timetonext / 35 as libc::c_int) as tic_t;
    let mut offsx: int32_t = 0 as libc::c_int;
    let mut offsy: int32_t = 0 as libc::c_int;
    let mut lift: [int32_t; 2] = [0 as libc::c_int, 0 as libc::c_int];
    if continuetime >= 3 as libc::c_int * 35 as libc::c_int {
        V_DrawFill(
            0 as libc::c_int,
            0 as libc::c_int,
            320 as libc::c_int,
            200 as libc::c_int,
            0 as libc::c_int,
        );
        return;
    }
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    if timetonext >= 11 as libc::c_int * 35 as libc::c_int + 10 as libc::c_int {
        return;
    }
    V_DrawLevelTitle(
        x
            - (V_LevelNameWidth(b"Continue?\0" as *const u8 as *const libc::c_char)
                >> 1 as libc::c_int),
        16 as libc::c_int,
        0 as libc::c_int,
        b"Continue?\0" as *const u8 as *const libc::c_char,
    );
    patch = W_CachePatchName(
        b"CONTSTAR\0" as *const u8 as *const libc::c_char,
        PU_PATCH_LOWPRIORITY as libc::c_int,
    ) as *mut patch_t;
    V_DrawStretchyFixedPatch(
        (x - 32 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
        (160 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0 as libc::c_int,
        patch,
        0 as *const uint8_t,
    );
    V_DrawStretchyFixedPatch(
        (x + 32 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
        (160 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0 as libc::c_int,
        patch,
        0 as *const uint8_t,
    );
    if timeleft > 9 as libc::c_int as tic_t {
        numbuf[7 as libc::c_int as usize] = '1' as i32 as libc::c_char;
        V_DrawStretchyFixedPatch(
            (x - 10 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
            (160 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchName(numbuf.as_mut_ptr(), PU_PATCH_LOWPRIORITY as libc::c_int)
                as *mut patch_t,
            0 as *const uint8_t,
        );
        numbuf[7 as libc::c_int as usize] = '0' as i32 as libc::c_char;
        V_DrawStretchyFixedPatch(
            (x + 10 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
            (160 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchName(numbuf.as_mut_ptr(), PU_PATCH_LOWPRIORITY as libc::c_int)
                as *mut patch_t,
            0 as *const uint8_t,
        );
    } else {
        numbuf[7 as libc::c_int
            as usize] = ('0' as i32 as tic_t).wrapping_add(timeleft) as libc::c_char;
        V_DrawStretchyFixedPatch(
            x * ((1 as libc::c_int) << 16 as libc::c_int),
            (160 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            (1 as libc::c_int) << 16 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchName(numbuf.as_mut_ptr(), PU_PATCH_LOWPRIORITY as libc::c_int)
                as *mut patch_t,
            0 as *const uint8_t,
        );
    }
    if multiplayer == 0
        && (ultimatemode as libc::c_int != 0
            || useContinues as libc::c_int != 0 && marathonmode as u64 == 0
            || modeattacking == 0 && !(cursaveslot > 0 as libc::c_int))
    {
        if ncontinues > 10 as libc::c_int {
            if continuetime & 1 as libc::c_int == 0 || continuetime > 17 as libc::c_int {
                V_DrawContinueIcon(
                    x,
                    68 as libc::c_int,
                    0 as libc::c_int,
                    players[consoleplayer as usize].skin,
                    players[consoleplayer as usize].skincolor,
                );
            }
            V_DrawStretchyFixedPatch(
                (x + 12 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
                (66 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                stlivex,
                0 as *const uint8_t,
            );
            V_DrawRightAlignedString(
                x + 38 as libc::c_int,
                64 as libc::c_int,
                0 as libc::c_int,
                va(
                    b"%d\0" as *const u8 as *const libc::c_char,
                    if imcontinuing != 0 {
                        ncontinues - 1 as libc::c_int
                    } else {
                        ncontinues
                    },
                ),
            );
        } else {
            x += ncontinues / 2 as libc::c_int * 30 as libc::c_int;
            if ncontinues & 1 as libc::c_int == 0 {
                x -= 15 as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < ncontinues {
                if !(i == ncontinues / 2 as libc::c_int
                    && (continuetime & 1 as libc::c_int != 0
                        || continuetime > 17 as libc::c_int))
                {
                    V_DrawContinueIcon(
                        x - i * 30 as libc::c_int,
                        68 as libc::c_int,
                        0 as libc::c_int,
                        players[consoleplayer as usize].skin,
                        players[consoleplayer as usize].skincolor,
                    );
                }
                i += 1;
                i;
            }
            x = 320 as libc::c_int >> 1 as libc::c_int;
        }
    }
    V_DrawStretchyFixedPatch(
        x * ((1 as libc::c_int) << 16 as libc::c_int),
        (140 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        (1 as libc::c_int) << 16 as libc::c_int,
        0 as libc::c_int,
        W_CachePatchName(
            b"CONTSPOT\0" as *const u8 as *const libc::c_char,
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t,
        0 as *const uint8_t,
    );
    if continuetime != 0 {
        let mut w: int32_t = if continuetime < 28 as libc::c_int {
            continuetime
        } else {
            28 as libc::c_int
        };
        let mut brightness: int32_t = continuetime >> 1 as libc::c_int
            & 7 as libc::c_int;
        if brightness > 3 as libc::c_int {
            brightness = 8 as libc::c_int - brightness;
        }
        V_DrawFadeFill(
            x - w,
            0 as libc::c_int,
            w << 1 as libc::c_int,
            140 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int as uint16_t,
            (3 as libc::c_int + brightness) as uint8_t,
        );
    }
    if !(contskins[1 as libc::c_int as usize]).is_null() {
        if continuetime > 15 as libc::c_int {
            let mut work: angle_t = FixedAngle(
                10 as libc::c_int * (continuetime - 15 as libc::c_int)
                    << 16 as libc::c_int,
            ) >> 19 as libc::c_int;
            offsy = (finesine[work as usize] >> 16 as libc::c_int - 16 as libc::c_int)
                << 1 as libc::c_int;
            offsx = 27 as libc::c_int
                * (*finecosine.offset(work as isize)
                    >> 16 as libc::c_int - 16 as libc::c_int) >> 1 as libc::c_int;
        } else {
            offsx = (27 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int;
        }
        lift[1 as libc::c_int as usize] = continuetime - 10 as libc::c_int;
        if lift[1 as libc::c_int as usize] < 0 as libc::c_int {
            lift[1 as libc::c_int as usize] = 0 as libc::c_int;
        } else if lift[1 as libc::c_int as usize] > 35 as libc::c_int + 5 as libc::c_int
        {
            lift[1 as libc::c_int as usize] = 35 as libc::c_int + 5 as libc::c_int;
        }
    }
    lift[0 as libc::c_int as usize] = continuetime - 5 as libc::c_int;
    if lift[0 as libc::c_int as usize] < 0 as libc::c_int {
        lift[0 as libc::c_int as usize] = 0 as libc::c_int;
    } else if lift[0 as libc::c_int as usize] > 35 as libc::c_int + 5 as libc::c_int {
        lift[0 as libc::c_int as usize] = 35 as libc::c_int + 5 as libc::c_int;
    }
    if offsy < 0 as libc::c_int {
        sprdef = &mut *((**contskins.as_mut_ptr().offset(0 as libc::c_int as isize))
            .sprites)
            .as_mut_ptr()
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as isize,
            ) as *mut spritedef_t;
        sprframe = &mut *((*sprdef).spriteframes)
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as isize,
            ) as *mut spriteframe_t;
        patch = W_CachePatchNum(
            (*sprframe)
                .lumppat[cont_spr2[0 as libc::c_int as usize][2 as libc::c_int as usize]
                as usize],
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        V_DrawStretchyFixedPatch(
            ((320 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int) - offsx,
            (140 as libc::c_int - lift[0 as libc::c_int as usize] << 16 as libc::c_int)
                - offsy,
            (*contskins[0 as libc::c_int as usize]).highresscale,
            (*contskins[0 as libc::c_int as usize]).highresscale,
            if (*sprframe).flip as libc::c_int
                & (1 as libc::c_int)
                    << cont_spr2[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        as libc::c_int != 0
            {
                0x800000 as libc::c_int
            } else {
                0 as libc::c_int
            },
            patch,
            contcolormaps[0 as libc::c_int as usize],
        );
    }
    if !(contskins[1 as libc::c_int as usize]).is_null() {
        sprdef = &mut *((**contskins.as_mut_ptr().offset(1 as libc::c_int as isize))
            .sprites)
            .as_mut_ptr()
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as isize,
            ) as *mut spritedef_t;
        sprframe = &mut *((*sprdef).spriteframes)
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(1 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as isize,
            ) as *mut spriteframe_t;
        patch = W_CachePatchNum(
            (*sprframe)
                .lumppat[cont_spr2[1 as libc::c_int as usize][2 as libc::c_int as usize]
                as usize],
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        V_DrawStretchyFixedPatch(
            ((320 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int) + offsx,
            (140 as libc::c_int - lift[1 as libc::c_int as usize] << 16 as libc::c_int)
                + offsy,
            (*contskins[1 as libc::c_int as usize]).highresscale,
            (*contskins[1 as libc::c_int as usize]).highresscale,
            if (*sprframe).flip as libc::c_int
                & (1 as libc::c_int)
                    << cont_spr2[1 as libc::c_int as usize][2 as libc::c_int as usize]
                        as libc::c_int != 0
            {
                0x800000 as libc::c_int
            } else {
                0 as libc::c_int
            },
            patch,
            contcolormaps[1 as libc::c_int as usize],
        );
    }
    if offsy >= 0 as libc::c_int {
        sprdef = &mut *((**contskins.as_mut_ptr().offset(0 as libc::c_int as isize))
            .sprites)
            .as_mut_ptr()
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as isize,
            ) as *mut spritedef_t;
        sprframe = &mut *((*sprdef).spriteframes)
            .offset(
                *(*cont_spr2.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as isize,
            ) as *mut spriteframe_t;
        patch = W_CachePatchNum(
            (*sprframe)
                .lumppat[cont_spr2[0 as libc::c_int as usize][2 as libc::c_int as usize]
                as usize],
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        V_DrawStretchyFixedPatch(
            ((320 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int) - offsx,
            (140 as libc::c_int - lift[0 as libc::c_int as usize] << 16 as libc::c_int)
                - offsy,
            (*contskins[0 as libc::c_int as usize]).highresscale,
            (*contskins[0 as libc::c_int as usize]).highresscale,
            if (*sprframe).flip as libc::c_int
                & (1 as libc::c_int)
                    << cont_spr2[0 as libc::c_int as usize][2 as libc::c_int as usize]
                        as libc::c_int != 0
            {
                0x800000 as libc::c_int
            } else {
                0 as libc::c_int
            },
            patch,
            contcolormaps[0 as libc::c_int as usize],
        );
    }
    if timetonext > 11 as libc::c_int * 35 as libc::c_int {
        V_DrawFadeScreen(
            31 as libc::c_int as uint16_t,
            (timetonext - 11 as libc::c_int * 35 as libc::c_int) as uint8_t,
        );
    }
    if continuetime > 3 as libc::c_int * 35 as libc::c_int - 10 as libc::c_int {
        V_DrawFadeScreen(
            0 as libc::c_int as uint16_t,
            (continuetime - (3 as libc::c_int * 35 as libc::c_int - 10 as libc::c_int))
                as uint8_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_ContinueTicker() {
    if imcontinuing == 0 {
        if timetonext > 0 as libc::c_int {
            timetonext -= 1;
            if timetonext == 0 {
                Command_ExitGame_f();
                return;
            }
        }
    } else {
        continuetime += 1;
        if continuetime == 3 as libc::c_int * 35 as libc::c_int {
            G_Continue();
            return;
        }
        if continuetime > 5 as libc::c_int
            && (continuetime & 1 as libc::c_int != 0 || continuetime > 35 as libc::c_int)
            && {
                cont_spr2[0 as libc::c_int
                    as usize][2 as libc::c_int
                    as usize] = (cont_spr2[0 as libc::c_int
                    as usize][2 as libc::c_int as usize])
                    .wrapping_add(1);
                cont_spr2[0 as libc::c_int as usize][2 as libc::c_int as usize]
                    as libc::c_int >= 8 as libc::c_int
            }
        {
            cont_spr2[0 as libc::c_int
                as usize][2 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        }
        if continuetime > 10 as libc::c_int
            && (continuetime & 1 as libc::c_int == 0
                || continuetime > 35 as libc::c_int + 5 as libc::c_int)
            && {
                cont_spr2[1 as libc::c_int
                    as usize][2 as libc::c_int
                    as usize] = (cont_spr2[1 as libc::c_int
                    as usize][2 as libc::c_int as usize])
                    .wrapping_add(1);
                cont_spr2[1 as libc::c_int as usize][2 as libc::c_int as usize]
                    as libc::c_int >= 8 as libc::c_int
            }
        {
            cont_spr2[1 as libc::c_int
                as usize][2 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        }
        if continuetime == 3 as libc::c_int * 35 as libc::c_int - 10 as libc::c_int {
            S_StartSound(0 as *const libc::c_void, sfx_cdfm56);
        } else if continuetime == 5 as libc::c_int {
            cont_spr2[0 as libc::c_int
                as usize][0 as libc::c_int
                as usize] = P_GetSkinSprite2(
                contskins[0 as libc::c_int as usize],
                SPR2_CNT2 as libc::c_int as uint8_t,
                0 as *mut player_t,
            );
            cont_spr2[0 as libc::c_int
                as usize][4 as libc::c_int
                as usize] = (*contskins[0 as libc::c_int as usize])
                .sprites[cont_spr2[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as usize]
                .numframes as uint8_t;
            cont_spr2[0 as libc::c_int
                as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
            cont_spr2[0 as libc::c_int
                as usize][1 as libc::c_int
                as usize] = cont_spr2[0 as libc::c_int
                as usize][3 as libc::c_int as usize];
            cont_spr2[0 as libc::c_int
                as usize][5 as libc::c_int as usize] = 2 as libc::c_int as uint8_t;
        } else if continuetime == 35 as libc::c_int {
            cont_spr2[0 as libc::c_int
                as usize][0 as libc::c_int
                as usize] = P_GetSkinSprite2(
                contskins[0 as libc::c_int as usize],
                SPR2_CNT3 as libc::c_int as uint8_t,
                0 as *mut player_t,
            );
            cont_spr2[0 as libc::c_int
                as usize][4 as libc::c_int
                as usize] = (*contskins[0 as libc::c_int as usize])
                .sprites[cont_spr2[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    as usize]
                .numframes as uint8_t;
            cont_spr2[0 as libc::c_int
                as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
            cont_spr2[0 as libc::c_int
                as usize][1 as libc::c_int
                as usize] = cont_spr2[0 as libc::c_int
                as usize][3 as libc::c_int as usize];
        } else if !(contskins[1 as libc::c_int as usize]).is_null() {
            if continuetime == 10 as libc::c_int {
                cont_spr2[1 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize] = P_GetSkinSprite2(
                    contskins[1 as libc::c_int as usize],
                    SPR2_CNT2 as libc::c_int as uint8_t,
                    0 as *mut player_t,
                );
                cont_spr2[1 as libc::c_int
                    as usize][4 as libc::c_int
                    as usize] = (*contskins[1 as libc::c_int as usize])
                    .sprites[cont_spr2[1 as libc::c_int
                        as usize][0 as libc::c_int as usize] as usize]
                    .numframes as uint8_t;
                cont_spr2[1 as libc::c_int
                    as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
                cont_spr2[1 as libc::c_int
                    as usize][1 as libc::c_int
                    as usize] = cont_spr2[1 as libc::c_int
                    as usize][3 as libc::c_int as usize];
                cont_spr2[1 as libc::c_int
                    as usize][5 as libc::c_int as usize] = 2 as libc::c_int as uint8_t;
            } else if continuetime == 35 as libc::c_int + 5 as libc::c_int {
                cont_spr2[1 as libc::c_int
                    as usize][0 as libc::c_int
                    as usize] = P_GetSkinSprite2(
                    contskins[1 as libc::c_int as usize],
                    SPR2_CNT3 as libc::c_int as uint8_t,
                    0 as *mut player_t,
                );
                cont_spr2[1 as libc::c_int
                    as usize][4 as libc::c_int
                    as usize] = (*contskins[1 as libc::c_int as usize])
                    .sprites[cont_spr2[1 as libc::c_int
                        as usize][0 as libc::c_int as usize] as usize]
                    .numframes as uint8_t;
                cont_spr2[1 as libc::c_int
                    as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
                cont_spr2[1 as libc::c_int
                    as usize][1 as libc::c_int
                    as usize] = cont_spr2[1 as libc::c_int
                    as usize][3 as libc::c_int as usize];
            }
        }
    }
    cont_spr2[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (cont_spr2[0 as libc::c_int as usize][3 as libc::c_int as usize])
        .wrapping_add(1);
    if cont_spr2[0 as libc::c_int as usize][3 as libc::c_int as usize] as libc::c_int
        >= cont_spr2[0 as libc::c_int as usize][5 as libc::c_int as usize] as libc::c_int
    {
        cont_spr2[0 as libc::c_int
            as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        cont_spr2[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = (cont_spr2[0 as libc::c_int as usize][1 as libc::c_int as usize])
            .wrapping_add(1);
        if cont_spr2[0 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int
            >= cont_spr2[0 as libc::c_int as usize][4 as libc::c_int as usize]
                as libc::c_int
        {
            cont_spr2[0 as libc::c_int
                as usize][1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        }
    }
    if !(contskins[1 as libc::c_int as usize]).is_null()
        && {
            cont_spr2[1 as libc::c_int
                as usize][3 as libc::c_int
                as usize] = (cont_spr2[1 as libc::c_int
                as usize][3 as libc::c_int as usize])
                .wrapping_add(1);
            cont_spr2[1 as libc::c_int as usize][3 as libc::c_int as usize]
                as libc::c_int
                >= cont_spr2[1 as libc::c_int as usize][5 as libc::c_int as usize]
                    as libc::c_int
        }
    {
        cont_spr2[1 as libc::c_int
            as usize][3 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        cont_spr2[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = (cont_spr2[1 as libc::c_int as usize][1 as libc::c_int as usize])
            .wrapping_add(1);
        if cont_spr2[1 as libc::c_int as usize][1 as libc::c_int as usize] as libc::c_int
            >= cont_spr2[1 as libc::c_int as usize][4 as libc::c_int as usize]
                as libc::c_int
        {
            cont_spr2[1 as libc::c_int
                as usize][1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_ContinueResponder(mut event: *mut event_t) -> boolean {
    let mut key: int32_t = (*event).key;
    if keypressed != 0 {
        return true_0 as libc::c_int;
    }
    if timetonext >= 21 as libc::c_int * 35 as libc::c_int / 2 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if (*event).type_0 as libc::c_uint != ev_keydown as libc::c_int as libc::c_uint {
        return false_0 as libc::c_int;
    }
    match key {
        13 | 32 | 256 | 264 | 266 => {}
        _ => return false_0 as libc::c_int,
    }
    keypressed = true_0 as libc::c_int;
    imcontinuing = true_0 as libc::c_int;
    S_StartSound(0 as *const libc::c_void, sfx_kc6b);
    I_FadeSong(
        0 as libc::c_int as uint8_t,
        1000 as libc::c_int as uint32_t,
        Some(S_StopMusic as unsafe extern "C" fn() -> ()),
    );
    return true_0 as libc::c_int;
}
static mut scenenum: int32_t = 0;
static mut cutnum: int32_t = 0;
static mut picxpos: int32_t = 0;
static mut picypos: int32_t = 0;
static mut picnum: int32_t = 0;
static mut pictime: int32_t = 0;
static mut picmode: int32_t = 0;
static mut numpics: int32_t = 0;
static mut pictoloop: int32_t = 0;
static mut textxpos: int32_t = 0;
static mut textypos: int32_t = 0;
static mut cutsceneover: boolean = false_0 as libc::c_int;
static mut runningprecutscene: boolean = false_0 as libc::c_int;
static mut precutresetplayer: boolean = false_0 as libc::c_int;
static mut precutFLS: boolean = false_0 as libc::c_int;
unsafe extern "C" fn F_AdvanceToNextScene() {
    if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
        F_WipeStartScreen();
        if (*cutscenes[cutnum as usize]).scene[scenenum as usize].fadecolor != 0 {
            V_DrawFill(
                0 as libc::c_int,
                0 as libc::c_int,
                320 as libc::c_int,
                200 as libc::c_int,
                (*cutscenes[cutnum as usize]).scene[scenenum as usize].fadecolor
                    as int32_t,
            );
            F_WipeEndScreen();
            F_RunWipe(
                (*cutscenes[cutnum as usize]).scene[scenenum as usize].fadeinid,
                true_0 as libc::c_int,
            );
            F_WipeStartScreen();
        }
    }
    if scenenum + 1 as libc::c_int >= (*cutscenes[cutnum as usize]).numscenes {
        F_EndCutScene();
        return;
    }
    scenenum += 1;
    scenenum;
    timetonext = 0 as libc::c_int;
    stoptimer = 0 as libc::c_int as tic_t;
    picnum = 0 as libc::c_int;
    picxpos = (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .xcoord[picnum as usize] as int32_t;
    picypos = (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .ycoord[picnum as usize] as int32_t;
    if (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .musswitch[0 as libc::c_int as usize] != 0
    {
        S_ChangeMusicEx(
            ((*cutscenes[cutnum as usize]).scene[scenenum as usize].musswitch)
                .as_mut_ptr(),
            (*cutscenes[cutnum as usize]).scene[scenenum as usize].musswitchflags,
            (*cutscenes[cutnum as usize]).scene[scenenum as usize].musicloop as boolean,
            (*cutscenes[cutnum as usize]).scene[scenenum as usize].musswitchposition,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
    F_NewCutscene((*cutscenes[cutnum as usize]).scene[scenenum as usize].text);
    picnum = 0 as libc::c_int;
    picxpos = (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .xcoord[picnum as usize] as int32_t;
    picypos = (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .ycoord[picnum as usize] as int32_t;
    textxpos = (*cutscenes[cutnum as usize]).scene[scenenum as usize].textxpos
        as int32_t;
    textypos = (*cutscenes[cutnum as usize]).scene[scenenum as usize].textypos
        as int32_t;
    pictime = (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .picduration[picnum as usize] as int32_t;
    animtimer = pictime as tic_t;
    if rendermode as libc::c_uint != render_none as libc::c_int as libc::c_uint {
        F_CutsceneDrawer();
        F_WipeEndScreen();
        F_RunWipe(
            (*cutscenes[cutnum as usize]).scene[scenenum as usize].fadeoutid,
            true_0 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_EndCutScene() {
    cutsceneover = true_0 as libc::c_int;
    if runningprecutscene != 0 {
        if server != 0 {
            D_MapChange(
                gamemap as int32_t,
                gametype as int32_t,
                ultimatemode as boolean,
                precutresetplayer,
                0 as libc::c_int,
                true_0 as libc::c_int,
                precutFLS,
            );
        }
    } else if cutnum == creditscutscene as libc::c_int - 1 as libc::c_int {
        F_StartGameEvaluation();
    } else if cutnum == introtoplay as libc::c_int - 1 as libc::c_int {
        D_StartTitle();
    } else if (nextmap as libc::c_int) < 1100 as libc::c_int - 1 as libc::c_int {
        G_NextLevel();
    } else {
        G_EndGame();
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_StartCustomCutscene(
    mut cutscenenum: int32_t,
    mut precutscene: boolean,
    mut resetplayer: boolean,
    mut FLS: boolean,
) {
    if (cutscenes[cutscenenum as usize]).is_null() {
        return;
    }
    G_SetGamestate(GS_CUTSCENE);
    if wipegamestate as libc::c_uint == GS_CUTSCENE as libc::c_int as libc::c_uint {
        wipegamestate = 4294967295 as gamestate_t;
    }
    gameaction = ga_nothing;
    paused = false_0 as libc::c_int as uint8_t;
    CON_ToggleOff();
    F_NewCutscene(
        (*cutscenes[cutscenenum as usize]).scene[0 as libc::c_int as usize].text,
    );
    cutsceneover = false_0 as libc::c_int;
    runningprecutscene = precutscene;
    precutresetplayer = resetplayer;
    precutFLS = FLS;
    picnum = 0 as libc::c_int;
    scenenum = picnum;
    cutnum = cutscenenum;
    picxpos = (*cutscenes[cutnum as usize])
        .scene[0 as libc::c_int as usize]
        .xcoord[0 as libc::c_int as usize] as int32_t;
    picypos = (*cutscenes[cutnum as usize])
        .scene[0 as libc::c_int as usize]
        .ycoord[0 as libc::c_int as usize] as int32_t;
    textxpos = (*cutscenes[cutnum as usize]).scene[0 as libc::c_int as usize].textxpos
        as int32_t;
    textypos = (*cutscenes[cutnum as usize]).scene[0 as libc::c_int as usize].textypos
        as int32_t;
    pictime = (*cutscenes[cutnum as usize])
        .scene[0 as libc::c_int as usize]
        .picduration[0 as libc::c_int as usize] as int32_t;
    keypressed = false_0 as libc::c_int;
    finalecount = 0 as libc::c_int;
    timetonext = 0 as libc::c_int;
    animtimer = (*cutscenes[cutnum as usize])
        .scene[0 as libc::c_int as usize]
        .picduration[0 as libc::c_int as usize] as tic_t;
    stoptimer = 0 as libc::c_int as tic_t;
    if (*cutscenes[cutnum as usize])
        .scene[0 as libc::c_int as usize]
        .musswitch[0 as libc::c_int as usize] != 0
    {
        S_ChangeMusicEx(
            ((*cutscenes[cutnum as usize]).scene[0 as libc::c_int as usize].musswitch)
                .as_mut_ptr(),
            (*cutscenes[cutnum as usize])
                .scene[0 as libc::c_int as usize]
                .musswitchflags,
            (*cutscenes[cutnum as usize]).scene[0 as libc::c_int as usize].musicloop
                as boolean,
            (*cutscenes[cutnum as usize]).scene[scenenum as usize].musswitchposition,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    } else {
        S_StopMusic();
    }
    S_StopSounds();
}
#[no_mangle]
pub unsafe extern "C" fn F_CutsceneDrawer() {
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    if (*cutscenes[cutnum as usize])
        .scene[scenenum as usize]
        .picname[picnum as usize][0 as libc::c_int as usize] as libc::c_int
        != '\0' as i32
    {
        if (*cutscenes[cutnum as usize])
            .scene[scenenum as usize]
            .pichires[picnum as usize] != 0
        {
            V_DrawStretchyFixedPatch(
                picxpos << 16 as libc::c_int,
                picypos << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                W_CachePatchName(
                    ((*cutscenes[cutnum as usize])
                        .scene[scenenum as usize]
                        .picname[picnum as usize])
                        .as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t,
                0 as *const uint8_t,
            );
        } else {
            V_DrawStretchyFixedPatch(
                picxpos * ((1 as libc::c_int) << 16 as libc::c_int),
                picypos << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                W_CachePatchName(
                    ((*cutscenes[cutnum as usize])
                        .scene[scenenum as usize]
                        .picname[picnum as usize])
                        .as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t,
                0 as *const uint8_t,
            );
        }
    }
    V_DrawString(
        textxpos,
        textypos,
        0x800000 as libc::c_int,
        cutscene_disptext.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn F_CutsceneTicker() {
    let mut i: int32_t = 0;
    if cutsceneover != 0 {
        return;
    }
    finalecount += 1;
    finalecount;
    cutscene_boostspeed = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(netgame != 0 && i != serverplayer && IsPlayerAdmin(i) == 0) {
            if players[i as usize].cmd.buttons as libc::c_int & BT_SPIN as libc::c_int
                != 0
            {
                keypressed = false_0 as libc::c_int;
                cutscene_boostspeed = 1 as libc::c_int as uint8_t;
                if timetonext != 0 {
                    timetonext = 2 as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    if animtimer != 0 {
        animtimer = animtimer.wrapping_sub(1);
        animtimer;
        if animtimer <= 0 as libc::c_int as tic_t {
            if picnum < 7 as libc::c_int
                && (*cutscenes[cutnum as usize])
                    .scene[scenenum as usize]
                    .picname[(picnum + 1 as libc::c_int)
                    as usize][0 as libc::c_int as usize] as libc::c_int != '\0' as i32
            {
                picnum += 1;
                picnum;
                picxpos = (*cutscenes[cutnum as usize])
                    .scene[scenenum as usize]
                    .xcoord[picnum as usize] as int32_t;
                picypos = (*cutscenes[cutnum as usize])
                    .scene[scenenum as usize]
                    .ycoord[picnum as usize] as int32_t;
                pictime = (*cutscenes[cutnum as usize])
                    .scene[scenenum as usize]
                    .picduration[picnum as usize] as int32_t;
                animtimer = pictime as tic_t;
            } else {
                timetonext = 2 as libc::c_int;
            }
        }
    }
    if timetonext != 0 {
        timetonext -= 1;
        timetonext;
    }
    stoptimer = stoptimer.wrapping_add(1);
    if stoptimer > 2 as libc::c_int as tic_t && timetonext == 1 as libc::c_int {
        F_AdvanceToNextScene();
    } else if timetonext == 0 && F_WriteText() == 0 {
        timetonext = 5 as libc::c_int * 35 as libc::c_int + 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_CutsceneResponder(mut event: *mut event_t) -> boolean {
    if cutnum == introtoplay as libc::c_int - 1 as libc::c_int {
        return F_IntroResponder(event);
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn F_GetPageTextGeometry(
    mut pagelines: *mut uint8_t,
    mut rightside: *mut boolean,
    mut boxh: *mut int32_t,
    mut texth: *mut int32_t,
    mut texty: *mut int32_t,
    mut namey: *mut int32_t,
    mut chevrony: *mut int32_t,
    mut textx: *mut int32_t,
    mut textr: *mut int32_t,
) {
    let mut iconlump: lumpnum_t = W_CheckNumForName(
        ((*textprompts[cutnum as usize]).page[scenenum as usize].iconname).as_mut_ptr(),
    );
    *pagelines = (if (*textprompts[cutnum as usize]).page[scenenum as usize].lines
        as libc::c_int != 0
    {
        (*textprompts[cutnum as usize]).page[scenenum as usize].lines as libc::c_int
    } else {
        4 as libc::c_int
    }) as uint8_t;
    *rightside = (iconlump != 4294967295 as libc::c_uint
        && (*textprompts[cutnum as usize]).page[scenenum as usize].rightside != 0)
        as libc::c_int;
    *boxh = *pagelines as libc::c_int * 2 as libc::c_int;
    *texth = if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .name[0 as libc::c_int as usize] as libc::c_int != 0
    {
        (*pagelines as libc::c_int - 1 as libc::c_int) * 2 as libc::c_int
    } else {
        *pagelines as libc::c_int * 2 as libc::c_int
    };
    *texty = 200 as libc::c_int
        - (*texth * 4 as libc::c_int + *texth / 2 as libc::c_int * 4 as libc::c_int);
    *namey = 200 as libc::c_int
        - (*boxh * 4 as libc::c_int + *boxh / 2 as libc::c_int * 4 as libc::c_int);
    *chevrony = 200 as libc::c_int
        - (1 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int
            + 1 as libc::c_int * 2 as libc::c_int / 2 as libc::c_int * 4 as libc::c_int);
    *textx = if iconlump != 4294967295 as libc::c_uint && *rightside == 0 {
        *boxh * 4 as libc::c_int + *boxh / 2 as libc::c_int * 4 as libc::c_int
            + 4 as libc::c_int
    } else {
        4 as libc::c_int
    };
    *textr = if *rightside != 0 {
        320 as libc::c_int
            - (*boxh * 4 as libc::c_int + *boxh / 2 as libc::c_int * 4 as libc::c_int
                + 4 as libc::c_int)
    } else {
        320 as libc::c_int - 4 as libc::c_int
    };
}
unsafe extern "C" fn F_GetPromptHideHudBound() -> fixed_t {
    let mut pagelines: uint8_t = 0;
    let mut rightside: boolean = 0;
    let mut boxh: int32_t = 0;
    let mut texth: int32_t = 0;
    let mut texty: int32_t = 0;
    let mut namey: int32_t = 0;
    let mut chevrony: int32_t = 0;
    let mut textx: int32_t = 0;
    let mut textr: int32_t = 0;
    if cutnum == 2147483647 as libc::c_int || scenenum == 2147483647 as libc::c_int
        || (textprompts[cutnum as usize]).is_null()
        || scenenum >= (*textprompts[cutnum as usize]).numpages
        || (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud == 0
        || splitscreen != 0
            && (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud
                as libc::c_int != 2 as libc::c_int
    {
        return 0 as libc::c_int
    } else if (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud
        as libc::c_int == 2 as libc::c_int
    {
        return 200 as libc::c_int
    }
    F_GetPageTextGeometry(
        &mut pagelines,
        &mut rightside,
        &mut boxh,
        &mut texth,
        &mut texty,
        &mut namey,
        &mut chevrony,
        &mut textx,
        &mut textr,
    );
    boxh *= vid.dupy;
    boxh = boxh * 4 as libc::c_int + boxh / 2 as libc::c_int * 5 as libc::c_int;
    return 0 as libc::c_int - boxh;
}
#[no_mangle]
pub unsafe extern "C" fn F_GetPromptHideHudAll() -> boolean {
    if cutnum == 2147483647 as libc::c_int || scenenum == 2147483647 as libc::c_int
        || (textprompts[cutnum as usize]).is_null()
        || scenenum >= (*textprompts[cutnum as usize]).numpages
        || (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud == 0
        || splitscreen != 0
            && (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud
                as libc::c_int != 2 as libc::c_int
    {
        return false_0 as libc::c_int
    } else if (*textprompts[cutnum as usize]).page[scenenum as usize].hidehud
        as libc::c_int == 2 as libc::c_int
    {
        return true_0 as libc::c_int
    } else {
        return false_0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_GetPromptHideHud(mut y: fixed_t) -> boolean {
    let mut ybound: int32_t = 0;
    let mut fromtop: boolean = 0;
    let mut ytest: fixed_t = 0;
    if promptactive == 0 {
        return false_0 as libc::c_int;
    }
    ybound = F_GetPromptHideHudBound();
    fromtop = (ybound >= 0 as libc::c_int) as libc::c_int;
    ytest = if fromtop != 0 { ybound } else { 200 as libc::c_int + ybound };
    return if fromtop != 0 {
        (y < ytest) as libc::c_int
    } else {
        (y >= ytest) as libc::c_int
    };
}
unsafe extern "C" fn F_PreparePageText(mut pagetext: *mut libc::c_char) {
    let mut pagelines: uint8_t = 0;
    let mut rightside: boolean = 0;
    let mut boxh: int32_t = 0;
    let mut texth: int32_t = 0;
    let mut texty: int32_t = 0;
    let mut namey: int32_t = 0;
    let mut chevrony: int32_t = 0;
    let mut textx: int32_t = 0;
    let mut textr: int32_t = 0;
    F_GetPageTextGeometry(
        &mut pagelines,
        &mut rightside,
        &mut boxh,
        &mut texth,
        &mut texty,
        &mut namey,
        &mut chevrony,
        &mut textx,
        &mut textr,
    );
    if !promptpagetext.is_null() {
        Z_Free(promptpagetext as *mut libc::c_void);
    }
    promptpagetext = if !pagetext.is_null()
        && *pagetext.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        V_WordWrap(textx, textr, 0 as libc::c_int, pagetext)
    } else {
        Z_StrDup(b"\0" as *const u8 as *const libc::c_char)
    };
    F_NewCutscene(promptpagetext);
    cutscene_textspeed = if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .textspeed as libc::c_int != 0
    {
        (*textprompts[cutnum as usize]).page[scenenum as usize].textspeed as libc::c_int
    } else {
        35 as libc::c_int / 5 as libc::c_int
    };
    cutscene_textcount = 0 as libc::c_int;
    cutscene_boostspeed = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn F_AdvanceToNextPage() {
    let mut nextprompt: int32_t = if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .nextprompt as libc::c_int != 0
    {
        (*textprompts[cutnum as usize]).page[scenenum as usize].nextprompt as libc::c_int
            - 1 as libc::c_int
    } else {
        2147483647 as libc::c_int
    };
    let mut nextpage: int32_t = if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .nextpage as libc::c_int != 0
    {
        (*textprompts[cutnum as usize]).page[scenenum as usize].nextpage as libc::c_int
            - 1 as libc::c_int
    } else {
        2147483647 as libc::c_int
    };
    let mut oldcutnum: int32_t = cutnum;
    if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .nexttag[0 as libc::c_int as usize] != 0
    {
        F_GetPromptPageByNamedTag(
            ((*textprompts[cutnum as usize]).page[scenenum as usize].nexttag)
                .as_mut_ptr(),
            &mut nextprompt,
            &mut nextpage,
        );
    }
    if nextprompt != 2147483647 as libc::c_int {
        if nextprompt
            <= 201 as libc::c_int
                + 6 as libc::c_int * 5 as libc::c_int * 3 as libc::c_int
            && !(textprompts[nextprompt as usize]).is_null()
        {
            cutnum = nextprompt;
        } else {
            cutnum = 2147483647 as libc::c_int;
        }
    }
    if nextpage != 2147483647 as libc::c_int {
        if cutnum != 2147483647 as libc::c_int {
            scenenum = nextpage;
            if scenenum >= 128 as libc::c_int
                || scenenum > (*textprompts[cutnum as usize]).numpages - 1 as libc::c_int
            {
                scenenum = 2147483647 as libc::c_int;
            }
        }
    } else if cutnum != oldcutnum {
        scenenum = 0 as libc::c_int;
    } else if (scenenum + 1 as libc::c_int) < 128 as libc::c_int
        && scenenum < (*textprompts[cutnum as usize]).numpages - 1 as libc::c_int
    {
        scenenum += 1;
        scenenum;
    } else {
        scenenum = 2147483647 as libc::c_int;
    }
    if cutnum == 2147483647 as libc::c_int || scenenum == 2147483647 as libc::c_int {
        F_EndTextPrompt(false_0 as libc::c_int, false_0 as libc::c_int);
    } else {
        timetonext = if (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .timetonext != 0
        {
            (*textprompts[cutnum as usize]).page[scenenum as usize].timetonext
        } else {
            35 as libc::c_int / 10 as libc::c_int
        };
        F_PreparePageText((*textprompts[cutnum as usize]).page[scenenum as usize].text);
        picnum = (*textprompts[cutnum as usize]).page[scenenum as usize].pictostart
            as int32_t;
        numpics = (*textprompts[cutnum as usize]).page[scenenum as usize].numpics
            as int32_t;
        picmode = (*textprompts[cutnum as usize]).page[scenenum as usize].picmode
            as int32_t;
        pictoloop = if (*textprompts[cutnum as usize]).page[scenenum as usize].pictoloop
            as libc::c_int > 0 as libc::c_int
        {
            (*textprompts[cutnum as usize]).page[scenenum as usize].pictoloop
                as libc::c_int - 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        picxpos = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .xcoord[picnum as usize] as int32_t;
        picypos = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .ycoord[picnum as usize] as int32_t;
        pictime = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .picduration[picnum as usize] as int32_t;
        animtimer = pictime as tic_t;
        if (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .musswitch[0 as libc::c_int as usize] != 0
        {
            S_ChangeMusicEx(
                ((*textprompts[cutnum as usize]).page[scenenum as usize].musswitch)
                    .as_mut_ptr(),
                (*textprompts[cutnum as usize]).page[scenenum as usize].musswitchflags,
                (*textprompts[cutnum as usize]).page[scenenum as usize].musicloop
                    as boolean,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn F_EndTextPrompt(mut forceexec: boolean, mut noexec: boolean) {
    let mut promptwasactive: boolean = promptactive;
    promptactive = false_0 as libc::c_int;
    callplayer = 2147483647 as libc::c_int;
    callpagenum = callplayer;
    callpromptnum = callpagenum;
    if promptwasactive != 0 {
        if !promptmo.is_null() && !((*promptmo).player).is_null()
            && promptblockcontrols != 0
        {
            (*promptmo).reactiontime = 35 as libc::c_int / 4 as libc::c_int;
        }
    }
    if (promptwasactive != 0 || forceexec != 0) && noexec == 0
        && promptpostexectag as libc::c_int != 0
    {
        if !tmthing.is_null() {
            P_LinedefExecute(promptpostexectag, promptmo, 0 as *mut sector_t);
        } else {
            P_MapStart();
            P_LinedefExecute(promptpostexectag, promptmo, 0 as *mut sector_t);
            P_MapEnd();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_StartTextPrompt(
    mut promptnum: int32_t,
    mut pagenum: int32_t,
    mut mo: *mut mobj_t,
    mut postexectag: uint16_t,
    mut blockcontrols: boolean,
    mut freezerealtime: boolean,
) {
    let mut i: int32_t = 0;
    if promptactive != 0 && splitscreen != 0 && promptnum == callpromptnum
        && pagenum == callpagenum
    {
        return;
    }
    if netgame != 0 {
        F_EndTextPrompt(true_0 as libc::c_int, false_0 as libc::c_int);
        return;
    }
    keypressed = false_0 as libc::c_int;
    finalecount = 0 as libc::c_int;
    timetonext = 0 as libc::c_int;
    animtimer = 0 as libc::c_int as tic_t;
    stoptimer = 0 as libc::c_int as tic_t;
    skullAnimCounter = 0 as libc::c_int as int16_t;
    promptmo = mo;
    promptpostexectag = postexectag as int16_t;
    promptblockcontrols = blockcontrols;
    callpromptnum = promptnum;
    callpagenum = pagenum;
    cutnum = if promptnum
        < 201 as libc::c_int + 6 as libc::c_int * 5 as libc::c_int * 3 as libc::c_int
        && !(textprompts[promptnum as usize]).is_null()
    {
        promptnum
    } else {
        2147483647 as libc::c_int
    };
    scenenum = if cutnum != 2147483647 as libc::c_int && pagenum < 128 as libc::c_int
        && pagenum <= (*textprompts[cutnum as usize]).numpages - 1 as libc::c_int
    {
        pagenum
    } else {
        2147483647 as libc::c_int
    };
    promptactive = (cutnum != 2147483647 as libc::c_int
        && scenenum != 2147483647 as libc::c_int) as libc::c_int;
    if promptactive != 0 {
        timetonext = if (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .timetonext != 0
        {
            (*textprompts[cutnum as usize]).page[scenenum as usize].timetonext
        } else {
            35 as libc::c_int / 10 as libc::c_int
        };
        F_PreparePageText((*textprompts[cutnum as usize]).page[scenenum as usize].text);
        picnum = (*textprompts[cutnum as usize]).page[scenenum as usize].pictostart
            as int32_t;
        numpics = (*textprompts[cutnum as usize]).page[scenenum as usize].numpics
            as int32_t;
        picmode = (*textprompts[cutnum as usize]).page[scenenum as usize].picmode
            as int32_t;
        pictoloop = if (*textprompts[cutnum as usize]).page[scenenum as usize].pictoloop
            as libc::c_int > 0 as libc::c_int
        {
            (*textprompts[cutnum as usize]).page[scenenum as usize].pictoloop
                as libc::c_int - 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        picxpos = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .xcoord[picnum as usize] as int32_t;
        picypos = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .ycoord[picnum as usize] as int32_t;
        pictime = (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .picduration[picnum as usize] as int32_t;
        animtimer = pictime as tic_t;
        if (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .musswitch[0 as libc::c_int as usize] != 0
        {
            S_ChangeMusicEx(
                ((*textprompts[cutnum as usize]).page[scenenum as usize].musswitch)
                    .as_mut_ptr(),
                (*textprompts[cutnum as usize]).page[scenenum as usize].musswitchflags,
                (*textprompts[cutnum as usize]).page[scenenum as usize].musicloop
                    as boolean,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            );
        }
        if promptblockcontrols != 0 && !mo.is_null() && !((*mo).player).is_null() {
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if players[i as usize].mo == mo {
                    callplayer = i;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
        }
    } else {
        F_EndTextPrompt(true_0 as libc::c_int, false_0 as libc::c_int);
    };
}
unsafe extern "C" fn F_GetTextPromptTutorialTag(
    mut tag: *mut libc::c_char,
    mut length: int32_t,
) -> boolean {
    let mut gcs: int32_t = gcs_custom as libc::c_int;
    let mut suffixed: boolean = true_0 as libc::c_int;
    if tag.is_null() || *tag.offset(0 as libc::c_int as isize) == 0 || tutorialmode == 0
    {
        return false_0 as libc::c_int;
    }
    if strncmp(
        tag,
        b"TAM\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_movement.as_ptr(),
            4 as libc::c_int,
        );
    } else if strncmp(
        tag,
        b"TAC\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_movement.as_ptr(),
            4 as libc::c_int,
        );
        if gcs == gcs_custom as libc::c_int {
            gcs = G_GetControlScheme(
                gamecontrol.as_mut_ptr(),
                gcl_camera.as_ptr(),
                2 as libc::c_int,
            );
        }
        if gcs == gcs_fps as libc::c_int && cv_usemouse.value == 0 {
            gcs = gcs_platform as libc::c_int;
        }
    } else if strncmp(
        tag,
        b"TAD\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_movement_camera.as_ptr(),
            6 as libc::c_int,
        );
    } else if strncmp(
        tag,
        b"TAJ\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_jump.as_ptr(),
            1 as libc::c_int,
        );
    } else if strncmp(
        tag,
        b"TAS\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_spin.as_ptr(),
            1 as libc::c_int,
        );
    } else if strncmp(
        tag,
        b"TAA\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_jump.as_ptr(),
            1 as libc::c_int,
        );
    } else if strncmp(
        tag,
        b"TAW\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_jump_spin.as_ptr(),
            2 as libc::c_int,
        );
    } else {
        gcs = G_GetControlScheme(
            gamecontrol.as_mut_ptr(),
            gcl_tutorial_used.as_ptr(),
            8 as libc::c_int,
        );
    }
    match gcs {
        1 => {
            suffixed = false_0 as libc::c_int;
        }
        2 => {
            strncat(
                tag,
                b"PLATFORM\0" as *const u8 as *const libc::c_char,
                length as libc::c_ulong,
            );
        }
        _ => {
            strncat(
                tag,
                b"CUSTOM\0" as *const u8 as *const libc::c_char,
                length as libc::c_ulong,
            );
        }
    }
    return suffixed;
}
#[no_mangle]
pub unsafe extern "C" fn F_GetPromptPageByNamedTag(
    mut tag: *const libc::c_char,
    mut promptnum: *mut int32_t,
    mut pagenum: *mut int32_t,
) {
    let mut nosuffixpromptnum: int32_t = 2147483647 as libc::c_int;
    let mut nosuffixpagenum: int32_t = 2147483647 as libc::c_int;
    let mut tutorialpromptnum: int32_t = if tutorialmode != 0 {
        201 as libc::c_int - 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut suffixed: boolean = false_0 as libc::c_int;
    let mut found: boolean = false_0 as libc::c_int;
    let mut suffixedtag: [libc::c_char; 33] = [0; 33];
    *pagenum = 2147483647 as libc::c_int;
    *promptnum = *pagenum;
    if tag.is_null() || *tag.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    strncpy(suffixedtag.as_mut_ptr(), tag, 33 as libc::c_int as libc::c_ulong);
    suffixedtag[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if tutorialmode != 0 {
        suffixed = F_GetTextPromptTutorialTag(
            suffixedtag.as_mut_ptr(),
            33 as libc::c_int,
        );
    }
    *promptnum = 0 as libc::c_int + tutorialpromptnum;
    while *promptnum
        < 201 as libc::c_int + 6 as libc::c_int * 5 as libc::c_int * 3 as libc::c_int
    {
        if !(textprompts[*promptnum as usize]).is_null() {
            *pagenum = 0 as libc::c_int;
            while *pagenum < (*textprompts[*promptnum as usize]).numpages
                && *pagenum < 128 as libc::c_int
            {
                if suffixed != 0
                    && fastcmp(
                        suffixedtag.as_mut_ptr(),
                        ((*textprompts[*promptnum as usize]).page[*pagenum as usize].tag)
                            .as_mut_ptr(),
                    ) != 0
                {
                    found = true_0 as libc::c_int;
                    break;
                } else {
                    if nosuffixpromptnum == 2147483647 as libc::c_int
                        && nosuffixpagenum == 2147483647 as libc::c_int
                        && fastcmp(
                            tag,
                            ((*textprompts[*promptnum as usize])
                                .page[*pagenum as usize]
                                .tag)
                                .as_mut_ptr(),
                        ) != 0
                    {
                        if suffixed != 0 {
                            nosuffixpromptnum = *promptnum;
                            nosuffixpagenum = *pagenum;
                        } else {
                            found = true_0 as libc::c_int;
                            break;
                        }
                    }
                    *pagenum += 1;
                    *pagenum;
                }
            }
            if found != 0 {
                break;
            }
        }
        *promptnum += 1;
        *promptnum;
    }
    if suffixed != 0 && found == 0 && nosuffixpromptnum != 2147483647 as libc::c_int
        && nosuffixpagenum != 2147483647 as libc::c_int
    {
        found = true_0 as libc::c_int;
        *promptnum = nosuffixpromptnum;
        *pagenum = nosuffixpagenum;
    }
    if found == 0 {
        CONS_Debug(
            0x80 as libc::c_int,
            b"Text prompt: Can't find a page with named tag %s or suffixed tag %s\n\0"
                as *const u8 as *const libc::c_char,
            tag,
            suffixedtag.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_TextPromptDrawer() {
    let mut iconlump: lumpnum_t = 0;
    let mut pagelines: uint8_t = 0;
    let mut rightside: boolean = 0;
    let mut boxh: int32_t = 0;
    let mut texth: int32_t = 0;
    let mut texty: int32_t = 0;
    let mut namey: int32_t = 0;
    let mut chevrony: int32_t = 0;
    let mut textx: int32_t = 0;
    let mut textr: int32_t = 0;
    let mut patch: *mut patch_t = 0 as *mut patch_t;
    if promptactive == 0 {
        return;
    }
    iconlump = W_CheckNumForName(
        ((*textprompts[cutnum as usize]).page[scenenum as usize].iconname).as_mut_ptr(),
    );
    F_GetPageTextGeometry(
        &mut pagelines,
        &mut rightside,
        &mut boxh,
        &mut texth,
        &mut texty,
        &mut namey,
        &mut chevrony,
        &mut textx,
        &mut textr,
    );
    if picnum >= 0 as libc::c_int && picnum < numpics
        && (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .picname[picnum as usize][0 as libc::c_int as usize] as libc::c_int
            != '\0' as i32
    {
        if (*textprompts[cutnum as usize])
            .page[scenenum as usize]
            .pichires[picnum as usize] != 0
        {
            V_DrawStretchyFixedPatch(
                picxpos << 16 as libc::c_int,
                picypos << 16 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
                0 as libc::c_int,
                W_CachePatchName(
                    ((*textprompts[cutnum as usize])
                        .page[scenenum as usize]
                        .picname[picnum as usize])
                        .as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t,
                0 as *const uint8_t,
            );
        } else {
            V_DrawStretchyFixedPatch(
                picxpos * ((1 as libc::c_int) << 16 as libc::c_int),
                picypos << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                (1 as libc::c_int) << 16 as libc::c_int,
                0 as libc::c_int,
                W_CachePatchName(
                    ((*textprompts[cutnum as usize])
                        .page[scenenum as usize]
                        .picname[picnum as usize])
                        .as_mut_ptr(),
                    PU_PATCH_LOWPRIORITY as libc::c_int,
                ) as *mut patch_t,
                0 as *const uint8_t,
            );
        }
    }
    V_DrawPromptBack(
        boxh,
        (*textprompts[cutnum as usize]).page[scenenum as usize].backcolor,
    );
    if iconlump != 4294967295 as libc::c_uint {
        let mut iconx: int32_t = 0;
        let mut icony: int32_t = 0;
        let mut scale: int32_t = 0;
        let mut scaledsize: int32_t = 0;
        patch = W_CachePatchName(
            ((*textprompts[cutnum as usize]).page[scenenum as usize].iconname)
                .as_mut_ptr(),
            PU_PATCH_LOWPRIORITY as libc::c_int,
        ) as *mut patch_t;
        if (*patch).width as libc::c_int > (*patch).height as libc::c_int {
            scale = FixedDiv(
                boxh * 4 as libc::c_int + boxh / 2 as libc::c_int * 4 as libc::c_int
                    - 4 as libc::c_int,
                (*patch).width as fixed_t,
            );
            scaledsize = FixedMul((*patch).height as fixed_t, scale);
            iconx = (if rightside != 0 {
                320 as libc::c_int
                    - (boxh * 4 as libc::c_int
                        + boxh / 2 as libc::c_int * 4 as libc::c_int)
            } else {
                4 as libc::c_int
            }) << 16 as libc::c_int;
            icony = ((namey - 4 as libc::c_int) << 16 as libc::c_int)
                + FixedDiv(
                    200 as libc::c_int - namey + 4 as libc::c_int - scaledsize,
                    2 as libc::c_int,
                );
        } else if (*patch).height as libc::c_int > (*patch).width as libc::c_int {
            scale = FixedDiv(
                boxh * 4 as libc::c_int + boxh / 2 as libc::c_int * 4 as libc::c_int
                    - 4 as libc::c_int,
                (*patch).height as fixed_t,
            );
            scaledsize = FixedMul((*patch).width as fixed_t, scale);
            iconx = (if rightside != 0 {
                320 as libc::c_int
                    - (boxh * 4 as libc::c_int
                        + boxh / 2 as libc::c_int * 4 as libc::c_int)
            } else {
                4 as libc::c_int
            }) << 16 as libc::c_int;
            icony = namey << 16 as libc::c_int;
            iconx
                += FixedDiv(
                    FixedMul((*patch).height as fixed_t, scale) - scaledsize,
                    2 as libc::c_int,
                );
        } else {
            scale = FixedDiv(
                boxh * 4 as libc::c_int + boxh / 2 as libc::c_int * 4 as libc::c_int
                    - 4 as libc::c_int,
                (*patch).width as fixed_t,
            );
            iconx = (if rightside != 0 {
                320 as libc::c_int
                    - (boxh * 4 as libc::c_int
                        + boxh / 2 as libc::c_int * 4 as libc::c_int)
            } else {
                4 as libc::c_int
            }) << 16 as libc::c_int;
            icony = namey << 16 as libc::c_int;
        }
        if (*textprompts[cutnum as usize]).page[scenenum as usize].iconflip != 0 {
            iconx += FixedMul((*patch).width as fixed_t, scale) << 16 as libc::c_int;
        }
        V_DrawStretchyFixedPatch(
            iconx,
            icony,
            scale,
            scale,
            0x2000000 as libc::c_int
                | (if (*textprompts[cutnum as usize]).page[scenenum as usize].iconflip
                    != 0
                {
                    0x800000 as libc::c_int
                } else {
                    0 as libc::c_int
                }),
            patch,
            0 as *const uint8_t,
        );
        W_UnlockCachedPatch(patch as *mut libc::c_void);
    }
    V_DrawString(
        textx,
        texty,
        0x2000000 as libc::c_int | 0x800000 as libc::c_int,
        cutscene_disptext.as_mut_ptr(),
    );
    if (*textprompts[cutnum as usize])
        .page[scenenum as usize]
        .name[0 as libc::c_int as usize] != 0
    {
        V_DrawString(
            textx,
            namey,
            0x2000000 as libc::c_int | 0x800000 as libc::c_int,
            ((*textprompts[cutnum as usize]).page[scenenum as usize].name).as_mut_ptr(),
        );
    }
    if promptblockcontrols != 0 && timetonext == 0 {
        V_DrawString(
            textr - 8 as libc::c_int,
            chevrony + skullAnimCounter as libc::c_int / 5 as libc::c_int,
            0x2000000 as libc::c_int | 0x2000 as libc::c_int,
            b"\x1B\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_TextPromptTicker() {
    let mut i: int32_t = 0;
    if promptactive == 0 || paused as libc::c_int != 0 || P_AutoPause() != 0 {
        return;
    }
    finalecount += 1;
    finalecount;
    cutscene_boostspeed = 0 as libc::c_int as uint8_t;
    skullAnimCounter -= 1;
    if skullAnimCounter as libc::c_int <= 0 as libc::c_int {
        skullAnimCounter = 8 as libc::c_int as int16_t;
    }
    if (*textprompts[cutnum as usize]).page[scenenum as usize].timetonext != 0 {
        if promptblockcontrols != 0 {
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if !(netgame != 0 && i != serverplayer && IsPlayerAdmin(i) == 0) {
                    if splitscreen != 0 {
                        if i == consoleplayer || i == secondarydisplayplayer {
                            players[i as usize]
                                .powers[pw_nocontrol as libc::c_int
                                as usize] = 1 as libc::c_int as uint16_t;
                            if !(players[i as usize].mo).is_null() {
                                if (*players[i as usize].mo).state
                                    == states
                                        .as_mut_ptr()
                                        .offset(S_PLAY_STND as libc::c_int as isize)
                                    && (*players[i as usize].mo).tics != -(1 as libc::c_int)
                                {
                                    (*players[i as usize].mo).tics += 1;
                                    (*players[i as usize].mo).tics;
                                } else if (*players[i as usize].mo).state
                                    == states
                                        .as_mut_ptr()
                                        .offset(S_PLAY_WAIT as libc::c_int as isize)
                                {
                                    P_SetPlayerMobjState(players[i as usize].mo, S_PLAY_STND);
                                }
                            }
                        }
                    } else if i == consoleplayer {
                        players[i as usize]
                            .powers[pw_nocontrol as libc::c_int
                            as usize] = 1 as libc::c_int as uint16_t;
                        if !(players[i as usize].mo).is_null() {
                            if (*players[i as usize].mo).state
                                == states
                                    .as_mut_ptr()
                                    .offset(S_PLAY_STND as libc::c_int as isize)
                                && (*players[i as usize].mo).tics != -(1 as libc::c_int)
                            {
                                (*players[i as usize].mo).tics += 1;
                                (*players[i as usize].mo).tics;
                            } else if (*players[i as usize].mo).state
                                == states
                                    .as_mut_ptr()
                                    .offset(S_PLAY_WAIT as libc::c_int as isize)
                            {
                                P_SetPlayerMobjState(players[i as usize].mo, S_PLAY_STND);
                            }
                        }
                    }
                    if splitscreen == 0 {
                        break;
                    }
                }
                i += 1;
                i;
            }
        }
        if timetonext >= 1 as libc::c_int {
            timetonext -= 1;
            timetonext;
        }
        if timetonext == 0 {
            F_AdvanceToNextPage();
        }
        F_WriteText();
    } else {
        if promptblockcontrols != 0 {
            let mut current_block_57: u64;
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if !(netgame != 0 && i != serverplayer && IsPlayerAdmin(i) == 0) {
                    if splitscreen != 0 {
                        if i == consoleplayer || i == secondarydisplayplayer {
                            players[i as usize]
                                .powers[pw_nocontrol as libc::c_int
                                as usize] = 1 as libc::c_int as uint16_t;
                            if callplayer == consoleplayer
                                || callplayer == secondarydisplayplayer
                            {
                                if i != callplayer {
                                    current_block_57 = 2873832966593178012;
                                } else {
                                    current_block_57 = 13321564401369230990;
                                }
                            } else if i != consoleplayer {
                                current_block_57 = 2873832966593178012;
                            } else {
                                current_block_57 = 13321564401369230990;
                            }
                        } else {
                            current_block_57 = 2873832966593178012;
                        }
                    } else if i == consoleplayer {
                        players[i as usize]
                            .powers[pw_nocontrol as libc::c_int
                            as usize] = 1 as libc::c_int as uint16_t;
                        if !(players[i as usize].mo).is_null() {
                            if (*players[i as usize].mo).state
                                == states
                                    .as_mut_ptr()
                                    .offset(S_PLAY_STND as libc::c_int as isize)
                                && (*players[i as usize].mo).tics != -(1 as libc::c_int)
                            {
                                (*players[i as usize].mo).tics += 1;
                                (*players[i as usize].mo).tics;
                            } else if (*players[i as usize].mo).state
                                == states
                                    .as_mut_ptr()
                                    .offset(S_PLAY_WAIT as libc::c_int as isize)
                            {
                                P_SetPlayerMobjState(players[i as usize].mo, S_PLAY_STND);
                            }
                        }
                        current_block_57 = 13321564401369230990;
                    } else {
                        current_block_57 = 2873832966593178012;
                    }
                    match current_block_57 {
                        2873832966593178012 => {}
                        _ => {
                            if players[i as usize].cmd.buttons as libc::c_int
                                & BT_SPIN as libc::c_int != 0
                                || players[i as usize].cmd.buttons as libc::c_int
                                    & BT_JUMP as libc::c_int != 0
                            {
                                if timetonext > 1 as libc::c_int {
                                    timetonext -= 1;
                                    timetonext;
                                } else if cutscene_baseptr != 0 {
                                    cutscene_boostspeed = 1 as libc::c_int as uint8_t;
                                }
                                if keypressed != 0 {
                                    if splitscreen == 0 {
                                        break;
                                    }
                                    current_block_57 = 2873832966593178012;
                                } else {
                                    if timetonext == 0 {
                                        F_AdvanceToNextPage();
                                        if promptactive != 0 {
                                            S_StartSound(0 as *const libc::c_void, sfx_menu1);
                                        }
                                    }
                                    keypressed = true_0 as libc::c_int;
                                    current_block_57 = 15970011996474399071;
                                }
                            } else {
                                if players[i as usize].cmd.buttons as libc::c_int
                                    & BT_SPIN as libc::c_int == 0
                                    && players[i as usize].cmd.buttons as libc::c_int
                                        & BT_JUMP as libc::c_int == 0
                                {
                                    keypressed = false_0 as libc::c_int;
                                }
                                current_block_57 = 15970011996474399071;
                            }
                            match current_block_57 {
                                2873832966593178012 => {}
                                _ => {
                                    if splitscreen == 0 {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
                i;
            }
        }
        if scenenum >= 128 as libc::c_int
            || ((*textprompts[cutnum as usize]).page[scenenum as usize].text).is_null()
            || *((*textprompts[cutnum as usize]).page[scenenum as usize].text)
                .offset(0 as libc::c_int as isize) == 0 || F_WriteText() == 0
        {
            timetonext = (promptblockcontrols == 0) as libc::c_int;
        }
    }
    if picnum >= 0 as libc::c_int && picnum < numpics {
        if animtimer <= 0 as libc::c_int as tic_t {
            let mut persistanimtimer: boolean = false_0 as libc::c_int;
            if picnum < numpics - 1 as libc::c_int
                && (*textprompts[cutnum as usize])
                    .page[scenenum as usize]
                    .picname[(picnum + 1 as libc::c_int)
                    as usize][0 as libc::c_int as usize] as libc::c_int != '\0' as i32
            {
                picnum += 1;
                picnum;
            } else if picmode == 1 as libc::c_int {
                picnum = pictoloop;
            } else if picmode == 2 as libc::c_int {
                picnum = -(1 as libc::c_int);
            } else {
                persistanimtimer = true_0 as libc::c_int;
            }
            if persistanimtimer == 0 && picnum >= 0 as libc::c_int {
                picxpos = (*textprompts[cutnum as usize])
                    .page[scenenum as usize]
                    .xcoord[picnum as usize] as int32_t;
                picypos = (*textprompts[cutnum as usize])
                    .page[scenenum as usize]
                    .ycoord[picnum as usize] as int32_t;
                pictime = (*textprompts[cutnum as usize])
                    .page[scenenum as usize]
                    .picduration[picnum as usize] as int32_t;
                animtimer = pictime as tic_t;
            }
        } else {
            animtimer = animtimer.wrapping_sub(1);
            animtimer;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_StartWaitingPlayers() {
    wipegamestate = GS_TITLESCREEN;
    finalecount = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn F_WaitingPlayersTicker() {
    if paused != 0 {
        return;
    }
    finalecount += 1;
    finalecount;
    if finalecount == 2 as libc::c_int {
        S_ChangeMusicEx(
            b"_CHSEL\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as uint16_t,
            true_0 as libc::c_int,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn F_WaitingPlayersDrawer() {
    let mut waittext1: *const libc::c_char = b"You will join\0" as *const u8
        as *const libc::c_char;
    let mut waittext2: *const libc::c_char = b"next level...\0" as *const u8
        as *const libc::c_char;
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    V_DrawCreditString(
        160 as libc::c_int - (V_CreditStringWidth(waittext1) >> 1 as libc::c_int)
            << 16 as libc::c_int,
        (48 as libc::c_int) << 16 as libc::c_int,
        0 as libc::c_int,
        waittext1,
    );
    V_DrawCreditString(
        160 as libc::c_int - (V_CreditStringWidth(waittext2) >> 1 as libc::c_int)
            << 16 as libc::c_int,
        (64 as libc::c_int) << 16 as libc::c_int,
        0 as libc::c_int,
        waittext2,
    );
}
unsafe extern "C" fn run_static_initializers() {
    credits_numpics = (::core::mem::size_of::<[C2RustUnnamed_5; 13]>() as libc::c_ulong)
        .wrapping_div(::core::mem::size_of::<C2RustUnnamed_5>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
