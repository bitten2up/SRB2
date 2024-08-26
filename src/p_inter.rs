use ::libc;
extern "C" {
    pub type visplane_s;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn CONS_Debug(debugflags: int32_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    static mut gameaction: gameaction_t;
    static mut ultimatemode: uint8_t;
    static mut gamestate: gamestate_t;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut finesine: [fixed_t; 10240];
    static mut finecosine: *mut fixed_t;
    fn I_Tactile(Type: FFType, Effect: *const JoyFF_t);
    fn I_Tactile2(Type: FFType, Effect: *const JoyFF_t);
    fn FixedAngle(fa: fixed_t) -> angle_t;
    fn FixedSqrt(x: fixed_t) -> fixed_t;
    fn FixedHypot(x: fixed_t, y: fixed_t) -> fixed_t;
    static mut S_sfx: [sfxinfo_t; 0];
    fn A_Scream();
    fn A_Boss5ExtraRepeat();
    static mut states: [state_t; 6735];
    fn P_SpawnParaloop(
        x: fixed_t,
        y: fixed_t,
        z: fixed_t,
        radius: fixed_t,
        number: int32_t,
        type_0: mobjtype_t,
        nstate: statenum_t,
        rotangle: angle_t,
        spawncenter: boolean,
    );
    fn P_SetScale(mobj: *mut mobj_t, newscale: fixed_t);
    static mut emeraldspawndelay: uint16_t;
    fn S_StartCaption(sfx_id: sfxenum_t, cnum: int32_t, lifespan: uint16_t);
    fn S_StartSound(origin: *const libc::c_void, sound_id: sfxenum_t);
    fn S_StopSound(origin: *mut libc::c_void);
    static mut music_stack_noposition: boolean;
    static mut music_stack_fadeout: uint32_t;
    fn S_ChangeMusicEx(
        mmusic: *const libc::c_char,
        mflags: uint16_t,
        looping: boolean,
        position: uint32_t,
        prefadems: uint32_t,
        fadeinms: uint32_t,
    );
    fn S_StopMusic();
    fn S_FadeMusicFromVolume(
        target_volume: uint8_t,
        source_volume: int16_t,
        ms: uint32_t,
    ) -> boolean;
    static mut mobjinfo: [mobjinfo_t; 1163];
    static mut automapactive: boolean;
    fn AM_Stop();
    static mut player_names: [[libc::c_char; 22]; 32];
    static mut players: [player_t; 32];
    static mut playeringame: [boolean; 32];
    static mut rw_maximums: [int16_t; 7];
    static mut localaiming: int32_t;
    static mut localaiming2: int32_t;
    fn G_SaveGameData(data: *mut gamedata_t);
    fn G_SaveGameOver(slot: uint32_t, modifylives: boolean);
    fn G_IsSpecialStage(mapnum: int32_t) -> boolean;
    fn G_GametypeUsesLives() -> boolean;
    fn G_GametypeUsesCoopLives() -> boolean;
    fn G_GametypeUsesCoopStarposts() -> boolean;
    fn G_GametypeHasTeams() -> boolean;
    fn G_PlatformGametype() -> boolean;
    fn G_CoopGametype() -> boolean;
    fn G_TagGametype() -> boolean;
    static mut invulntics: uint16_t;
    static mut flashingtics: uint16_t;
    static mut underwatertics: uint16_t;
    static mut nightslinktics: uint16_t;
    static mut useContinues: uint8_t;
    static mut shareEmblems: uint8_t;
    static mut hunt1: *mut mobj_t;
    static mut hunt2: *mut mobj_t;
    static mut hunt3: *mut mobj_t;
    static mut countdown: uint32_t;
    static mut countdown2: uint32_t;
    static mut hidetime: tic_t;
    static mut cv_itemrespawntime: consvar_t;
    static mut cv_itemrespawn: consvar_t;
    static mut cv_flagtime: consvar_t;
    static mut cv_friendlyfire: consvar_t;
    static mut cv_pointlimit: consvar_t;
    static mut cv_timelimit: consvar_t;
    static mut timelimitintics: uint32_t;
    static mut cv_hazardlog: consvar_t;
    static mut cv_killingdead: consvar_t;
    static mut cv_coopstarposts: consvar_t;
    static mut cv_cooplives: consvar_t;
    static mut cv_overtime: consvar_t;
    fn D_SendExitLevel(cheat: boolean);
    static mut server: boolean;
    fn D_NumPlayers() -> int32_t;
    static mut serverplayer: int32_t;
    static mut demoplayback: boolean;
    static mut demorecording: boolean;
    static mut redscore: uint32_t;
    static mut objectplacing: boolean;
    static mut clientGamedata: *mut gamedata_t;
    static mut serverGamedata: *mut gamedata_t;
    static mut emblemlocations: [emblem_t; 512];
    static mut numemblems: int32_t;
    fn M_UpdateUnlockablesAndExtraEmblems(data: *mut gamedata_t) -> uint8_t;
    fn M_SilentUpdateUnlockablesAndEmblems(data: *mut gamedata_t);
    fn M_EmblemSkinNum(emblem: *mut emblem_t) -> int32_t;
    static mut bluescore: uint32_t;
    static mut tokenlist: uint32_t;
    static mut token: uint32_t;
    static mut emeralds: uint16_t;
    static mut stagefailed: boolean;
    static mut mapheaderinfo: [*mut mapheader_t; 1035];
    static mut quake: quake;
    static mut bflagpoint: *mut mapthing_t;
    static mut rflagpoint: *mut mapthing_t;
    static mut blueflag: *mut mobj_t;
    static mut redflag: *mut mobj_t;
    static mut secondarydisplayplayer: int32_t;
    static mut displayplayer: int32_t;
    static mut consoleplayer: int32_t;
    static mut circuitmap: boolean;
    static mut splitscreen: boolean;
    static mut gametyperules: uint32_t;
    static mut gametype: int16_t;
    static mut multiplayer: boolean;
    static mut netgame: boolean;
    static mut modeattacking: uint8_t;
    static mut metalrecording: boolean;
    static mut usedCheats: boolean;
    static mut modifiedgame: boolean;
    static mut numgameovers: uint8_t;
    static mut marathonmode: marathonmode_t;
    static mut cursaveslot: int32_t;
    static mut maptol: uint32_t;
    static mut gamemap: int16_t;
    fn G_GhostAddHit(victim: *mut mobj_t);
    fn G_GhostAddColor(color: ghostcolor_t);
    fn G_StopMetalRecording(kill: boolean) -> !;
    fn P_RandomFixed() -> fixed_t;
    fn P_RandomByte() -> uint8_t;
    fn P_RandomKey(a: int32_t) -> int32_t;
    fn P_RandomRange(a: int32_t, b: int32_t) -> int32_t;
    fn P_SetTarget2(mo: *mut *mut mobj_t, target: *mut mobj_t) -> *mut mobj_t;
    fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t;
    static mut thlist: [thinker_t; 0];
    fn P_AddPlayerScore(player: *mut player_t, amount: uint32_t);
    fn P_StealPlayerScore(player: *mut player_t, amount: uint32_t);
    fn P_PlayerInPain(player: *mut player_t) -> boolean;
    fn P_DoPlayerPain(
        player: *mut player_t,
        source: *mut mobj_t,
        inflictor: *mut mobj_t,
    );
    fn P_ResetPlayer(player: *mut player_t);
    fn P_PlayerCanDamage(player: *mut player_t, thing: *mut mobj_t) -> boolean;
    fn P_IsLocalPlayer(player: *mut player_t) -> boolean;
    fn P_SetPlayerAngle(player: *mut player_t, angle: angle_t);
    fn P_IsObjectOnGround(mo: *mut mobj_t) -> boolean;
    fn P_SetObjectMomZ(mo: *mut mobj_t, value: fixed_t, relative: boolean);
    fn P_RestoreMusic(player: *mut player_t);
    fn P_SwitchShield(player: *mut player_t, shieldtype: uint16_t);
    fn P_GivePlayerRings(player: *mut player_t, num_rings: int32_t);
    fn P_GivePlayerSpheres(player: *mut player_t, num_spheres: int32_t);
    fn P_GivePlayerLives(player: *mut player_t, numlives: int32_t);
    fn P_GiveCoopLives(player: *mut player_t, numlives: int32_t, sound: boolean);
    fn P_GiveEmerald(spawnObj: boolean);
    fn P_DoBubbleBounce(player: *mut player_t);
    fn P_DoAbilityBounce(player: *mut player_t, changemomz: boolean);
    fn P_TwinSpinRejuvenate(player: *mut player_t, type_0: mobjtype_t);
    fn P_BlackOw(player: *mut player_t);
    fn P_NightserizePlayer(player: *mut player_t, ptime: int32_t);
    fn P_InstaThrust(mo: *mut mobj_t, angle: angle_t, move_0: fixed_t);
    fn P_ReturnThrustX(mo: *mut mobj_t, angle: angle_t, move_0: fixed_t) -> fixed_t;
    fn P_ReturnThrustY(mo: *mut mobj_t, angle: angle_t, move_0: fixed_t) -> fixed_t;
    fn P_PlayLivesJingle(player: *mut player_t);
    fn P_GetLives(player: *mut player_t) -> boolean;
    fn P_SpectatorJoinGame(player: *mut player_t) -> boolean;
    fn P_PlayJingle(player: *mut player_t, jingletype: jingletype_t);
    fn P_SpawnMobj(
        x: fixed_t,
        y: fixed_t,
        z: fixed_t,
        type_0: mobjtype_t,
    ) -> *mut mobj_t;
    fn P_RemoveMobj(th: *mut mobj_t);
    fn P_MobjWasRemoved(th: *mut mobj_t) -> boolean;
    fn P_SetPlayerMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
    fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
    fn P_SpawnMobjFromMobj(
        mobj: *mut mobj_t,
        xofs: fixed_t,
        yofs: fixed_t,
        zofs: fixed_t,
        type_0: mobjtype_t,
    ) -> *mut mobj_t;
    fn P_MobjFlip(mobj: *mut mobj_t) -> int8_t;
    static mut var1: int32_t;
    static mut var2: int32_t;
    fn P_UnsetThingPosition(thing: *mut mobj_t);
    fn P_SetThingPosition(thing: *mut mobj_t);
    fn P_RemoveThinkerDelayed(thinker: *mut thinker_t);
    fn P_LinedefExecute(tag: int16_t, actor: *mut mobj_t, caller: *mut sector_t);
    static mut leveltime: tic_t;
    fn EV_DoElevator(tag: mtag_t, line: *mut line_t, elevtype: elevator_e);
    fn P_PlayerTouchingSectorSpecialFlag(
        player: *mut player_t,
        flag: sectorspecialflags_t,
    ) -> *mut sector_t;
    fn R_PointToAngle2(
        px2: fixed_t,
        py2: fixed_t,
        px1: fixed_t,
        py1: fixed_t,
    ) -> angle_t;
    fn P_SwitchSpheresBonusMode(bonustime: boolean);
    fn HU_SetCEchoFlags(flags: int32_t);
    fn HU_SetCEchoDuration(seconds: int32_t);
    fn HU_DoCEcho(msg: *const libc::c_char);
    fn LUA_HookTouchSpecial(special: *mut mobj_t, toucher: *mut mobj_t) -> libc::c_int;
    fn LUA_HookShouldDamage(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: int32_t,
        damagetype: uint8_t,
    ) -> libc::c_int;
    fn LUA_HookMobjDamage(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: int32_t,
        damagetype: uint8_t,
    ) -> libc::c_int;
    fn LUA_HookMobjDeath(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damagetype: uint8_t,
    ) -> libc::c_int;
    fn LUA_HookHurtMsg(
        _: *mut player_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damagetype: uint8_t,
    ) -> libc::c_int;
    fn M_CountBits(num: uint32_t, size: uint8_t) -> uint8_t;
    fn F_StartTextPrompt(
        promptnum: int32_t,
        pagenum: int32_t,
        mo: *mut mobj_t,
        postexectag: uint16_t,
        blockcontrols: boolean,
        freezerealtime: boolean,
    );
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUMSUPERCOLORS: C2RustUnnamed_0 = 9;
pub const MAXSKINCOLORS: C2RustUnnamed_0 = 1182;
pub const SKINCOLOR_LASTFREESLOT: C2RustUnnamed_0 = 1181;
pub const SKINCOLOR_FIRSTFREESLOT: C2RustUnnamed_0 = 158;
pub const SKINCOLOR_SUPERTAN5: C2RustUnnamed_0 = 157;
pub const SKINCOLOR_SUPERTAN4: C2RustUnnamed_0 = 156;
pub const SKINCOLOR_SUPERTAN3: C2RustUnnamed_0 = 155;
pub const SKINCOLOR_SUPERTAN2: C2RustUnnamed_0 = 154;
pub const SKINCOLOR_SUPERTAN1: C2RustUnnamed_0 = 153;
pub const SKINCOLOR_SUPERRUST5: C2RustUnnamed_0 = 152;
pub const SKINCOLOR_SUPERRUST4: C2RustUnnamed_0 = 151;
pub const SKINCOLOR_SUPERRUST3: C2RustUnnamed_0 = 150;
pub const SKINCOLOR_SUPERRUST2: C2RustUnnamed_0 = 149;
pub const SKINCOLOR_SUPERRUST1: C2RustUnnamed_0 = 148;
pub const SKINCOLOR_SUPERPURPLE5: C2RustUnnamed_0 = 147;
pub const SKINCOLOR_SUPERPURPLE4: C2RustUnnamed_0 = 146;
pub const SKINCOLOR_SUPERPURPLE3: C2RustUnnamed_0 = 145;
pub const SKINCOLOR_SUPERPURPLE2: C2RustUnnamed_0 = 144;
pub const SKINCOLOR_SUPERPURPLE1: C2RustUnnamed_0 = 143;
pub const SKINCOLOR_SUPERSKY5: C2RustUnnamed_0 = 142;
pub const SKINCOLOR_SUPERSKY4: C2RustUnnamed_0 = 141;
pub const SKINCOLOR_SUPERSKY3: C2RustUnnamed_0 = 140;
pub const SKINCOLOR_SUPERSKY2: C2RustUnnamed_0 = 139;
pub const SKINCOLOR_SUPERSKY1: C2RustUnnamed_0 = 138;
pub const SKINCOLOR_SUPERPERIDOT5: C2RustUnnamed_0 = 137;
pub const SKINCOLOR_SUPERPERIDOT4: C2RustUnnamed_0 = 136;
pub const SKINCOLOR_SUPERPERIDOT3: C2RustUnnamed_0 = 135;
pub const SKINCOLOR_SUPERPERIDOT2: C2RustUnnamed_0 = 134;
pub const SKINCOLOR_SUPERPERIDOT1: C2RustUnnamed_0 = 133;
pub const SKINCOLOR_SUPERGOLD5: C2RustUnnamed_0 = 132;
pub const SKINCOLOR_SUPERGOLD4: C2RustUnnamed_0 = 131;
pub const SKINCOLOR_SUPERGOLD3: C2RustUnnamed_0 = 130;
pub const SKINCOLOR_SUPERGOLD2: C2RustUnnamed_0 = 129;
pub const SKINCOLOR_SUPERGOLD1: C2RustUnnamed_0 = 128;
pub const SKINCOLOR_SUPERORANGE5: C2RustUnnamed_0 = 127;
pub const SKINCOLOR_SUPERORANGE4: C2RustUnnamed_0 = 126;
pub const SKINCOLOR_SUPERORANGE3: C2RustUnnamed_0 = 125;
pub const SKINCOLOR_SUPERORANGE2: C2RustUnnamed_0 = 124;
pub const SKINCOLOR_SUPERORANGE1: C2RustUnnamed_0 = 123;
pub const SKINCOLOR_SUPERRED5: C2RustUnnamed_0 = 122;
pub const SKINCOLOR_SUPERRED4: C2RustUnnamed_0 = 121;
pub const SKINCOLOR_SUPERRED3: C2RustUnnamed_0 = 120;
pub const SKINCOLOR_SUPERRED2: C2RustUnnamed_0 = 119;
pub const SKINCOLOR_SUPERRED1: C2RustUnnamed_0 = 118;
pub const SKINCOLOR_SUPERSILVER5: C2RustUnnamed_0 = 117;
pub const SKINCOLOR_SUPERSILVER4: C2RustUnnamed_0 = 116;
pub const SKINCOLOR_SUPERSILVER3: C2RustUnnamed_0 = 115;
pub const SKINCOLOR_SUPERSILVER2: C2RustUnnamed_0 = 114;
pub const SKINCOLOR_SUPERSILVER1: C2RustUnnamed_0 = 113;
pub const FIRSTSUPERCOLOR: C2RustUnnamed_0 = 113;
pub const SKINCOLOR_VOLCANIC: C2RustUnnamed_0 = 112;
pub const SKINCOLOR_SANGRIA: C2RustUnnamed_0 = 111;
pub const SKINCOLOR_FANCY: C2RustUnnamed_0 = 110;
pub const SKINCOLOR_ROSY: C2RustUnnamed_0 = 109;
pub const SKINCOLOR_TAFFY: C2RustUnnamed_0 = 108;
pub const SKINCOLOR_RASPBERRY: C2RustUnnamed_0 = 107;
pub const SKINCOLOR_PLUM: C2RustUnnamed_0 = 106;
pub const SKINCOLOR_EVENTIDE: C2RustUnnamed_0 = 105;
pub const SKINCOLOR_MAUVE: C2RustUnnamed_0 = 104;
pub const SKINCOLOR_LILAC: C2RustUnnamed_0 = 103;
pub const SKINCOLOR_ROYAL: C2RustUnnamed_0 = 102;
pub const SKINCOLOR_VIOLET: C2RustUnnamed_0 = 101;
pub const SKINCOLOR_NEON: C2RustUnnamed_0 = 100;
pub const SKINCOLOR_MAGENTA: C2RustUnnamed_0 = 99;
pub const SKINCOLOR_SIBERITE: C2RustUnnamed_0 = 98;
pub const SKINCOLOR_BUBBLEGUM: C2RustUnnamed_0 = 97;
pub const SKINCOLOR_FUCHSIA: C2RustUnnamed_0 = 96;
pub const SKINCOLOR_NOBLE: C2RustUnnamed_0 = 95;
pub const SKINCOLOR_PURPLE: C2RustUnnamed_0 = 94;
pub const SKINCOLOR_PASTEL: C2RustUnnamed_0 = 93;
pub const SKINCOLOR_MAJESTY: C2RustUnnamed_0 = 92;
pub const SKINCOLOR_DUSK: C2RustUnnamed_0 = 91;
pub const SKINCOLOR_VAPOR: C2RustUnnamed_0 = 90;
pub const SKINCOLOR_GALAXY: C2RustUnnamed_0 = 89;
pub const SKINCOLOR_MIDNIGHT: C2RustUnnamed_0 = 88;
pub const SKINCOLOR_COBALT: C2RustUnnamed_0 = 87;
pub const SKINCOLOR_BLUE: C2RustUnnamed_0 = 86;
pub const SKINCOLOR_CORNFLOWER: C2RustUnnamed_0 = 85;
pub const SKINCOLOR_ARCTIC: C2RustUnnamed_0 = 84;
pub const SKINCOLOR_SAPPHIRE: C2RustUnnamed_0 = 83;
pub const SKINCOLOR_DAYBREAK: C2RustUnnamed_0 = 82;
pub const SKINCOLOR_ICY: C2RustUnnamed_0 = 81;
pub const SKINCOLOR_DREAM: C2RustUnnamed_0 = 80;
pub const SKINCOLOR_CERULEAN: C2RustUnnamed_0 = 79;
pub const SKINCOLOR_MARINE: C2RustUnnamed_0 = 78;
pub const SKINCOLOR_SKY: C2RustUnnamed_0 = 77;
pub const SKINCOLOR_AQUAMARINE: C2RustUnnamed_0 = 76;
pub const SKINCOLOR_TURQUOISE: C2RustUnnamed_0 = 75;
pub const SKINCOLOR_CYAN: C2RustUnnamed_0 = 74;
pub const SKINCOLOR_WAVE: C2RustUnnamed_0 = 73;
pub const SKINCOLOR_OCEAN: C2RustUnnamed_0 = 72;
pub const SKINCOLOR_TEAL: C2RustUnnamed_0 = 71;
pub const SKINCOLOR_AQUA: C2RustUnnamed_0 = 70;
pub const SKINCOLOR_BOTTLE: C2RustUnnamed_0 = 69;
pub const SKINCOLOR_ISLAND: C2RustUnnamed_0 = 68;
pub const SKINCOLOR_SEAFOAM: C2RustUnnamed_0 = 67;
pub const SKINCOLOR_EMERALD: C2RustUnnamed_0 = 66;
pub const SKINCOLOR_MASTER: C2RustUnnamed_0 = 65;
pub const SKINCOLOR_MINT: C2RustUnnamed_0 = 64;
pub const SKINCOLOR_JADE: C2RustUnnamed_0 = 63;
pub const SKINCOLOR_SHAMROCK: C2RustUnnamed_0 = 62;
pub const SKINCOLOR_FOREST: C2RustUnnamed_0 = 61;
pub const SKINCOLOR_GREEN: C2RustUnnamed_0 = 60;
pub const SKINCOLOR_CHARTREUSE: C2RustUnnamed_0 = 59;
pub const SKINCOLOR_HEADLIGHT: C2RustUnnamed_0 = 58;
pub const SKINCOLOR_APPLE: C2RustUnnamed_0 = 57;
pub const SKINCOLOR_PERIDOT: C2RustUnnamed_0 = 56;
pub const SKINCOLOR_LIME: C2RustUnnamed_0 = 55;
pub const SKINCOLOR_LEMON: C2RustUnnamed_0 = 54;
pub const SKINCOLOR_PEAR: C2RustUnnamed_0 = 53;
pub const SKINCOLOR_OLIVE: C2RustUnnamed_0 = 52;
pub const SKINCOLOR_YELLOW: C2RustUnnamed_0 = 51;
pub const SKINCOLOR_GOLDENROD: C2RustUnnamed_0 = 50;
pub const SKINCOLOR_SANDY: C2RustUnnamed_0 = 49;
pub const SKINCOLOR_GOLD: C2RustUnnamed_0 = 48;
pub const SKINCOLOR_TOPAZ: C2RustUnnamed_0 = 47;
pub const SKINCOLOR_TANGERINE: C2RustUnnamed_0 = 46;
pub const SKINCOLOR_RUST: C2RustUnnamed_0 = 45;
pub const SKINCOLOR_ORANGE: C2RustUnnamed_0 = 44;
pub const SKINCOLOR_APRICOT: C2RustUnnamed_0 = 43;
pub const SKINCOLOR_COPPER: C2RustUnnamed_0 = 42;
pub const SKINCOLOR_SUNSET: C2RustUnnamed_0 = 41;
pub const SKINCOLOR_FOUNDATION: C2RustUnnamed_0 = 40;
pub const SKINCOLOR_QUAIL: C2RustUnnamed_0 = 39;
pub const SKINCOLOR_PEACHY: C2RustUnnamed_0 = 38;
pub const SKINCOLOR_KETCHUP: C2RustUnnamed_0 = 37;
pub const SKINCOLOR_GARNET: C2RustUnnamed_0 = 36;
pub const SKINCOLOR_FLAME: C2RustUnnamed_0 = 35;
pub const SKINCOLOR_CRIMSON: C2RustUnnamed_0 = 34;
pub const SKINCOLOR_RED: C2RustUnnamed_0 = 33;
pub const SKINCOLOR_PEPPER: C2RustUnnamed_0 = 32;
pub const SKINCOLOR_SALMON: C2RustUnnamed_0 = 31;
pub const SKINCOLOR_CHERRY: C2RustUnnamed_0 = 30;
pub const SKINCOLOR_RUBY: C2RustUnnamed_0 = 29;
pub const SKINCOLOR_LAVENDER: C2RustUnnamed_0 = 28;
pub const SKINCOLOR_EGGPLANT: C2RustUnnamed_0 = 27;
pub const SKINCOLOR_AZURE: C2RustUnnamed_0 = 26;
pub const SKINCOLOR_MOSS: C2RustUnnamed_0 = 25;
pub const SKINCOLOR_ROSEBUSH: C2RustUnnamed_0 = 24;
pub const SKINCOLOR_BEIGE: C2RustUnnamed_0 = 23;
pub const SKINCOLOR_TAN: C2RustUnnamed_0 = 22;
pub const SKINCOLOR_ECRU: C2RustUnnamed_0 = 21;
pub const SKINCOLOR_SEPIA: C2RustUnnamed_0 = 20;
pub const SKINCOLOR_BRONZE: C2RustUnnamed_0 = 19;
pub const SKINCOLOR_BOULDER: C2RustUnnamed_0 = 18;
pub const SKINCOLOR_BROWN: C2RustUnnamed_0 = 17;
pub const SKINCOLOR_LATTE: C2RustUnnamed_0 = 16;
pub const SKINCOLOR_YOGURT: C2RustUnnamed_0 = 15;
pub const SKINCOLOR_ROSEWOOD: C2RustUnnamed_0 = 14;
pub const SKINCOLOR_PINK: C2RustUnnamed_0 = 13;
pub const SKINCOLOR_BLUEBELL: C2RustUnnamed_0 = 12;
pub const SKINCOLOR_MOONSTONE: C2RustUnnamed_0 = 11;
pub const SKINCOLOR_SLATE: C2RustUnnamed_0 = 10;
pub const SKINCOLOR_AETHER: C2RustUnnamed_0 = 9;
pub const SKINCOLOR_BLACK: C2RustUnnamed_0 = 8;
pub const SKINCOLOR_JET: C2RustUnnamed_0 = 7;
pub const SKINCOLOR_CARBON: C2RustUnnamed_0 = 6;
pub const SKINCOLOR_SILVER: C2RustUnnamed_0 = 5;
pub const SKINCOLOR_GREY: C2RustUnnamed_0 = 4;
pub const SKINCOLOR_CLOUDY: C2RustUnnamed_0 = 3;
pub const SKINCOLOR_BONE: C2RustUnnamed_0 = 2;
pub const SKINCOLOR_WHITE: C2RustUnnamed_0 = 1;
pub const SKINCOLOR_NONE: C2RustUnnamed_0 = 0;
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
#[repr(C, packed)]
pub struct ticcmd_t {
    pub forwardmove: int8_t,
    pub sidemove: int8_t,
    pub angleturn: int16_t,
    pub aiming: int16_t,
    pub buttons: uint16_t,
    pub latency: uint8_t,
}
pub type FFType = libc::c_int;
pub const NumberofForces: FFType = 7;
pub const SawtoothDownForce: FFType = 6;
pub const SawtoothUpForce: FFType = 5;
pub const TriangleForce: FFType = 4;
pub const SineForce: FFType = 3;
pub const SquareForce: FFType = 2;
pub const RampForce: FFType = 1;
pub const ConstantForce: FFType = 0;
pub const EvilForce: FFType = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JoyFF_s {
    pub ForceX: int32_t,
    pub ForceY: int32_t,
    pub Duration: uint32_t,
    pub Gain: int32_t,
    pub Magnitude: int32_t,
    pub Start: int32_t,
    pub End: int32_t,
    pub Offset: int32_t,
    pub Phase: uint32_t,
    pub Period: uint32_t,
}
pub type JoyFF_t = JoyFF_s;
pub type angle_t = uint32_t;
pub type actionf_v = Option::<unsafe extern "C" fn() -> ()>;
pub type actionf_p1 = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union actionf_t {
    pub acv: actionf_v,
    pub acp1: actionf_p1,
}
pub type think_t = actionf_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinker_s {
    pub prev: *mut thinker_s,
    pub next: *mut thinker_s,
    pub function: think_t,
    pub references: int32_t,
}
pub type thinker_t = thinker_s;
pub type mtag_t = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct taglist_t {
    pub tags: *mut mtag_t,
    pub count: uint16_t,
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
pub type polyobj_t = polyobj_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfxinfo_struct {
    pub name: *const libc::c_char,
    pub singularity: boolean,
    pub priority: int32_t,
    pub pitch: int32_t,
    pub volume: int32_t,
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub skinsound: int32_t,
    pub usefulness: int32_t,
    pub lumpnum: lumpnum_t,
    pub caption: [libc::c_char; 32],
}
pub type sfxinfo_t = sfxinfo_struct;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MF_RUNSPAWNFUNC: C2RustUnnamed_1 = 536870912;
pub const MF_GRENADEBOUNCE: C2RustUnnamed_1 = 268435456;
pub const MF_NOCLIPTHING: C2RustUnnamed_1 = 134217728;
pub const MF_NIGHTSITEM: C2RustUnnamed_1 = 67108864;
pub const MF_STICKY: C2RustUnnamed_1 = 33554432;
pub const MF_PAIN: C2RustUnnamed_1 = 16777216;
pub const MF_SCENERY: C2RustUnnamed_1 = 8388608;
pub const MF_ENEMY: C2RustUnnamed_1 = 4194304;
pub const MF_NOCLIPHEIGHT: C2RustUnnamed_1 = 2097152;
pub const MF_FIRE: C2RustUnnamed_1 = 1048576;
pub const MF_NOTHINK: C2RustUnnamed_1 = 524288;
pub const MF_MONITOR: C2RustUnnamed_1 = 262144;
pub const MF_BOUNCE: C2RustUnnamed_1 = 131072;
pub const MF_SPRING: C2RustUnnamed_1 = 65536;
pub const MF_MISSILE: C2RustUnnamed_1 = 32768;
pub const MF_BOXICON: C2RustUnnamed_1 = 16384;
pub const MF_FLOAT: C2RustUnnamed_1 = 8192;
pub const MF_NOCLIP: C2RustUnnamed_1 = 4096;
pub const MF_SLIDEME: C2RustUnnamed_1 = 2048;
pub const MF_AMBIENT: C2RustUnnamed_1 = 1024;
pub const MF_NOGRAVITY: C2RustUnnamed_1 = 512;
pub const MF_SPAWNCEILING: C2RustUnnamed_1 = 256;
pub const MF_BOSS: C2RustUnnamed_1 = 128;
pub const MF_PUSHABLE: C2RustUnnamed_1 = 64;
pub const MF_PAPERCOLLISION: C2RustUnnamed_1 = 32;
pub const MF_NOBLOCKMAP: C2RustUnnamed_1 = 16;
pub const MF_NOSECTOR: C2RustUnnamed_1 = 8;
pub const MF_SHOOTABLE: C2RustUnnamed_1 = 4;
pub const MF_SOLID: C2RustUnnamed_1 = 2;
pub const MF_SPECIAL: C2RustUnnamed_1 = 1;
pub type mobjflag2_t = libc::c_uint;
pub const MF2_SPLAT: mobjflag2_t = 1073741824;
pub const MF2_SHIELD: mobjflag2_t = 536870912;
pub const MF2_LINKDRAW: mobjflag2_t = 268435456;
pub const MF2_AMBUSH: mobjflag2_t = 134217728;
pub const MF2_BOSSDEAD: mobjflag2_t = 67108864;
pub const MF2_BOSSFLEE: mobjflag2_t = 33554432;
pub const MF2_BOSSNOTRAP: mobjflag2_t = 16777216;
pub const MF2_FRET: mobjflag2_t = 8388608;
pub const MF2_SKULLFLY: mobjflag2_t = 4194304;
pub const MF2_OBJECTFLIP: mobjflag2_t = 2097152;
pub const MF2_STRONGBOX: mobjflag2_t = 1048576;
pub const MF2_SHADOW: mobjflag2_t = 524288;
pub const MF2_SUPERFIRE: mobjflag2_t = 262144;
pub const MF2_FIRING: mobjflag2_t = 131072;
pub const MF2_JUSTATTACKED: mobjflag2_t = 65536;
pub const MF2_NIGHTSPULL: mobjflag2_t = 32768;
pub const MF2_DEBRIS: mobjflag2_t = 16384;
pub const MF2_INFLOAT: mobjflag2_t = 8192;
pub const MF2_INVERTAIMABLE: mobjflag2_t = 4096;
pub const MF2_CLASSICPUSH: mobjflag2_t = 2048;
pub const MF2_SLIDEPUSH: mobjflag2_t = 1024;
pub const MF2_BEYONDTHEGRAVE: mobjflag2_t = 512;
pub const MF2_SCATTER: mobjflag2_t = 256;
pub const MF2_EXPLOSION: mobjflag2_t = 128;
pub const MF2_BOUNCERING: mobjflag2_t = 64;
pub const MF2_RAILRING: mobjflag2_t = 32;
pub const MF2_AUTOMATIC: mobjflag2_t = 16;
pub const MF2_DONTDRAW: mobjflag2_t = 8;
pub const MF2_DONTRESPAWN: mobjflag2_t = 4;
pub const MF2_TWOD: mobjflag2_t = 2;
pub const MF2_AXIS: mobjflag2_t = 1;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MFE_REVERSESUPER: C2RustUnnamed_2 = 12288;
pub const MFE_FORCENOSUPER: C2RustUnnamed_2 = 8192;
pub const MFE_FORCESUPER: C2RustUnnamed_2 = 4096;
pub const MFE_TRACERANGLE: C2RustUnnamed_2 = 2048;
pub const MFE_APPLYPMOMZ: C2RustUnnamed_2 = 1024;
pub const MFE_SPRUNG: C2RustUnnamed_2 = 512;
pub const MFE_PUSHED: C2RustUnnamed_2 = 256;
pub const MFE_TOUCHLAVA: C2RustUnnamed_2 = 128;
pub const MFE_GOOWATER: C2RustUnnamed_2 = 64;
pub const MFE_VERTICALFLIP: C2RustUnnamed_2 = 32;
pub const MFE_JUSTSTEPPEDDOWN: C2RustUnnamed_2 = 16;
pub const MFE_UNDERWATER: C2RustUnnamed_2 = 8;
pub const MFE_TOUCHWATER: C2RustUnnamed_2 = 4;
pub const MFE_JUSTHITFLOOR: C2RustUnnamed_2 = 2;
pub const MFE_ONGROUND: C2RustUnnamed_2 = 1;
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
    pub revert: C2RustUnnamed_3,
    pub netid: uint16_t,
    pub changed: libc::c_char,
    pub next: *mut consvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub allocated: libc::c_char,
    pub v: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub string: *mut libc::c_char,
    pub const_munge: *const libc::c_char,
}
pub type consvar_t = consvar_s;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const SF_NOSHIELDABILITY: C2RustUnnamed_5 = 524288;
pub const SF_CANBUSTWALLS: C2RustUnnamed_5 = 262144;
pub const SF_NOSUPERJUMPBOOST: C2RustUnnamed_5 = 131072;
pub const SF_NOSUPERSPRITES: C2RustUnnamed_5 = 65536;
pub const SF_NONIGHTSSUPER: C2RustUnnamed_5 = 32768;
pub const SF_NONIGHTSROTATION: C2RustUnnamed_5 = 16384;
pub const SF_MULTIABILITY: C2RustUnnamed_5 = 8192;
pub const SF_FASTEDGE: C2RustUnnamed_5 = 4096;
pub const SF_DASHMODE: C2RustUnnamed_5 = 2048;
pub const SF_MACHINE: C2RustUnnamed_5 = 1024;
pub const SF_MARIODAMAGE: C2RustUnnamed_5 = 768;
pub const SF_STOMPDAMAGE: C2RustUnnamed_5 = 512;
pub const SF_NOJUMPDAMAGE: C2RustUnnamed_5 = 256;
pub const SF_NOJUMPSPIN: C2RustUnnamed_5 = 128;
pub const SF_RUNONWATER: C2RustUnnamed_5 = 64;
pub const SF_NOSPEEDADJUST: C2RustUnnamed_5 = 32;
pub const SF_NOSKID: C2RustUnnamed_5 = 16;
pub const SF_HIRES: C2RustUnnamed_5 = 8;
pub const SF_NOSPINDASHDUST: C2RustUnnamed_5 = 4;
pub const SF_NOSUPERSPIN: C2RustUnnamed_5 = 2;
pub const SF_SUPER: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const CA_TWINSPIN: C2RustUnnamed_6 = 15;
pub const CA_BOUNCE: C2RustUnnamed_6 = 14;
pub const CA_JUMPTHOK: C2RustUnnamed_6 = 13;
pub const CA_AIRDRILL: C2RustUnnamed_6 = 12;
pub const CA_JUMPBOOST: C2RustUnnamed_6 = 11;
pub const CA_FALLSWITCH: C2RustUnnamed_6 = 10;
pub const CA_TELEKINESIS: C2RustUnnamed_6 = 9;
pub const CA_SLOWFALL: C2RustUnnamed_6 = 8;
pub const CA_FLOAT: C2RustUnnamed_6 = 7;
pub const CA_DOUBLEJUMP: C2RustUnnamed_6 = 6;
pub const CA_SWIM: C2RustUnnamed_6 = 5;
pub const CA_HOMINGTHOK: C2RustUnnamed_6 = 4;
pub const CA_GLIDEANDCLIMB: C2RustUnnamed_6 = 3;
pub const CA_FLY: C2RustUnnamed_6 = 2;
pub const CA_THOK: C2RustUnnamed_6 = 1;
pub const CA_NONE: C2RustUnnamed_6 = 0;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const CA2_MELEE: C2RustUnnamed_7 = 3;
pub const CA2_GUNSLINGER: C2RustUnnamed_7 = 2;
pub const CA2_SPINDASH: C2RustUnnamed_7 = 1;
pub const CA2_NONE: C2RustUnnamed_7 = 0;
pub type C2RustUnnamed_8 = libc::c_int;
pub const SH_NOSTACK: C2RustUnnamed_8 = -513;
pub const SH_STACK: C2RustUnnamed_8 = 512;
pub const SH_FIREFLOWER: C2RustUnnamed_8 = 512;
pub const SH_FORCEHP: C2RustUnnamed_8 = 255;
pub const SH_FORCE: C2RustUnnamed_8 = 256;
pub const SH_THUNDERCOIN: C2RustUnnamed_8 = 4098;
pub const SH_BUBBLEWRAP: C2RustUnnamed_8 = 2049;
pub const SH_FLAMEAURA: C2RustUnnamed_8 = 1025;
pub const SH_ELEMENTAL: C2RustUnnamed_8 = 3073;
pub const SH_ATTRACT: C2RustUnnamed_8 = 4097;
pub const SH_PINK: C2RustUnnamed_8 = 4;
pub const SH_ARMAGEDDON: C2RustUnnamed_8 = 3;
pub const SH_WHIRLWIND: C2RustUnnamed_8 = 2;
pub const SH_PITY: C2RustUnnamed_8 = 1;
pub const SH_PROTECTSPIKE: C2RustUnnamed_8 = 8192;
pub const SH_PROTECTELECTRIC: C2RustUnnamed_8 = 4096;
pub const SH_PROTECTWATER: C2RustUnnamed_8 = 2048;
pub const SH_PROTECTFIRE: C2RustUnnamed_8 = 1024;
pub const SH_NONE: C2RustUnnamed_8 = 0;
pub type C2RustUnnamed_9 = libc::c_uint;
pub const CR_FAN: C2RustUnnamed_9 = 13;
pub const CR_DUSTDEVIL: C2RustUnnamed_9 = 12;
pub const CR_PTERABYTE: C2RustUnnamed_9 = 11;
pub const CR_ROLLOUT: C2RustUnnamed_9 = 10;
pub const CR_MINECART: C2RustUnnamed_9 = 9;
pub const CR_MACESPIN: C2RustUnnamed_9 = 8;
pub const CR_ROPEHANG: C2RustUnnamed_9 = 7;
pub const CR_ZOOMTUBE: C2RustUnnamed_9 = 6;
pub const CR_BRAKGOOP: C2RustUnnamed_9 = 5;
pub const CR_NIGHTSFALL: C2RustUnnamed_9 = 4;
pub const CR_NIGHTSMODE: C2RustUnnamed_9 = 3;
pub const CR_PLAYER: C2RustUnnamed_9 = 2;
pub const CR_GENERIC: C2RustUnnamed_9 = 1;
pub const CR_NONE: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const STR_METAL: C2RustUnnamed_10 = 4224;
pub const STR_BOUNCE: C2RustUnnamed_10 = 521;
pub const STR_MELEE: C2RustUnnamed_10 = 6979;
pub const STR_TWINSPIN: C2RustUnnamed_10 = 7967;
pub const STR_GLIDE: C2RustUnnamed_10 = 3;
pub const STR_FLY: C2RustUnnamed_10 = 17;
pub const STR_BUST: C2RustUnnamed_10 = 1792;
pub const STR_ATTACK: C2RustUnnamed_10 = 30;
pub const STR_SPIKE: C2RustUnnamed_10 = 4096;
pub const STR_SPRING: C2RustUnnamed_10 = 2048;
pub const STR_CEILING: C2RustUnnamed_10 = 1024;
pub const STR_FLOOR: C2RustUnnamed_10 = 512;
pub const STR_WALL: C2RustUnnamed_10 = 256;
pub const STR_DASH: C2RustUnnamed_10 = 128;
pub const STR_HEAVY: C2RustUnnamed_10 = 64;
pub const STR_GUARD: C2RustUnnamed_10 = 32;
pub const STR_UPPER: C2RustUnnamed_10 = 16;
pub const STR_STOMP: C2RustUnnamed_10 = 8;
pub const STR_TAIL: C2RustUnnamed_10 = 4;
pub const STR_PUNCH: C2RustUnnamed_10 = 2;
pub const STR_ANIM: C2RustUnnamed_10 = 1;
pub const STR_NONE: C2RustUnnamed_10 = 0;
pub type powertype_t = libc::c_uint;
pub const NUMPOWERS: powertype_t = 30;
pub const pw_strong: powertype_t = 29;
pub const pw_ignorelatch: powertype_t = 28;
pub const pw_justlaunched: powertype_t = 27;
pub const pw_dye: powertype_t = 26;
pub const pw_nocontrol: powertype_t = 25;
pub const pw_nights_linkfreeze: powertype_t = 24;
pub const pw_nights_helper: powertype_t = 23;
pub const pw_nights_superloop: powertype_t = 22;
pub const pw_emeralds: powertype_t = 21;
pub const pw_railring: powertype_t = 20;
pub const pw_explosionring: powertype_t = 19;
pub const pw_grenadering: powertype_t = 18;
pub const pw_scatterring: powertype_t = 17;
pub const pw_bouncering: powertype_t = 16;
pub const pw_automaticring: powertype_t = 15;
pub const pw_infinityring: powertype_t = 14;
pub const pw_gravityboots: powertype_t = 13;
pub const pw_super: powertype_t = 12;
pub const pw_noautobrake: powertype_t = 11;
pub const pw_justsprung: powertype_t = 10;
pub const pw_pushing: powertype_t = 9;
pub const pw_extralife: powertype_t = 8;
pub const pw_spacetime: powertype_t = 7;
pub const pw_underwater: powertype_t = 6;
pub const pw_tailsfly: powertype_t = 5;
pub const pw_carry: powertype_t = 4;
pub const pw_shield: powertype_t = 3;
pub const pw_flashing: powertype_t = 2;
pub const pw_sneakers: powertype_t = 1;
pub const pw_invulnerability: powertype_t = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const RW_RAIL: C2RustUnnamed_11 = 32;
pub const RW_EXPLODE: C2RustUnnamed_11 = 16;
pub const RW_GRENADE: C2RustUnnamed_11 = 8;
pub const RW_SCATTER: C2RustUnnamed_11 = 4;
pub const RW_BOUNCE: C2RustUnnamed_11 = 2;
pub const RW_AUTO: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const BOT_MPAI: C2RustUnnamed_12 = 3;
pub const BOT_2PHUMAN: C2RustUnnamed_12 = 2;
pub const BOT_2PAI: C2RustUnnamed_12 = 1;
pub const BOT_NONE: C2RustUnnamed_12 = 0;
pub type player_t = player_s;
pub type marathonmode_t = libc::c_uint;
pub const MA_INGAME: marathonmode_t = 8;
pub const MA_NOCUTSCENES: marathonmode_t = 4;
pub const MA_INIT: marathonmode_t = 2;
pub const MA_RUNNING: marathonmode_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mappoint_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quake {
    pub x: fixed_t,
    pub y: fixed_t,
    pub z: fixed_t,
    pub time: uint16_t,
    pub epicenter: *mut mappoint_t,
    pub radius: fixed_t,
    pub intensity: fixed_t,
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
pub type GameTypeRules = libc::c_int;
pub const GTR_CUTSCENES: GameTypeRules = -2147483648;
pub const GTR_NOTITLECARD: GameTypeRules = 1073741824;
pub const GTR_ALLOWEXIT: GameTypeRules = 536870912;
pub const GTR_SPAWNENEMIES: GameTypeRules = 268435456;
pub const GTR_SPAWNINVUL: GameTypeRules = 134217728;
pub const GTR_DEATHMATCHSTARTS: GameTypeRules = 67108864;
pub const GTR_NOSPECTATORSPAWN: GameTypeRules = 33554432;
pub const GTR_DEATHPENALTY: GameTypeRules = 16777216;
pub const GTR_PITYSHIELD: GameTypeRules = 8388608;
pub const GTR_RESPAWNDELAY: GameTypeRules = 4194304;
pub const GTR_BLINDFOLDED: GameTypeRules = 2097152;
pub const GTR_HIDEFROZEN: GameTypeRules = 1048576;
pub const GTR_STARTCOUNTDOWN: GameTypeRules = 524288;
pub const GTR_FRIENDLYFIRE: GameTypeRules = 262144;
pub const GTR_HURTMESSAGES: GameTypeRules = 131072;
pub const GTR_OVERTIME: GameTypeRules = 65536;
pub const GTR_TIMELIMIT: GameTypeRules = 32768;
pub const GTR_POINTLIMIT: GameTypeRules = 16384;
pub const GTR_TAG: GameTypeRules = 8192;
pub const GTR_RACE: GameTypeRules = 4096;
pub const GTR_EMERALDHUNT: GameTypeRules = 2048;
pub const GTR_EMERALDTOKENS: GameTypeRules = 1024;
pub const GTR_SPECIALSTAGES: GameTypeRules = 512;
pub const GTR_FRIENDLY: GameTypeRules = 256;
pub const GTR_TEAMFLAGS: GameTypeRules = 128;
pub const GTR_POWERSTONES: GameTypeRules = 64;
pub const GTR_FIRSTPERSON: GameTypeRules = 32;
pub const GTR_TEAMS: GameTypeRules = 16;
pub const GTR_LIVES: GameTypeRules = 8;
pub const GTR_SPECTATORS: GameTypeRules = 4;
pub const GTR_RINGSLINGER: GameTypeRules = 2;
pub const GTR_CAMPAIGN: GameTypeRules = 1;
pub type TypeOfLevel = libc::c_uint;
pub const TOL_XMAS: TypeOfLevel = 4096;
pub const TOL_ERZ3: TypeOfLevel = 2048;
pub const TOL_NIGHTS: TypeOfLevel = 1024;
pub const TOL_MARIO: TypeOfLevel = 512;
pub const TOL_2D: TypeOfLevel = 256;
pub const TOL_CTF: TypeOfLevel = 64;
pub const TOL_TAG: TypeOfLevel = 32;
pub const TOL_MATCH: TypeOfLevel = 16;
pub const TOL_RACE: TypeOfLevel = 8;
pub const TOL_COMPETITION: TypeOfLevel = 4;
pub const TOL_COOP: TypeOfLevel = 2;
pub const TOL_SP: TypeOfLevel = 1;
pub type ghostcolor_t = libc::c_uint;
pub const GHC_RETURNSKIN: ghostcolor_t = 5;
pub const GHC_NIGHTSSKIN: ghostcolor_t = 4;
pub const GHC_INVINCIBLE: ghostcolor_t = 3;
pub const GHC_FIREFLOWER: ghostcolor_t = 2;
pub const GHC_SUPER: ghostcolor_t = 1;
pub const GHC_NORMAL: ghostcolor_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct emblem_t {
    pub type_0: uint8_t,
    pub tag: int16_t,
    pub level: int16_t,
    pub sprite: uint8_t,
    pub color: uint16_t,
    pub var: int32_t,
    pub stringVar: *mut libc::c_char,
    pub hint: [libc::c_char; 110],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct recorddata_t {
    pub time: tic_t,
    pub score: uint32_t,
    pub rings: uint16_t,
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
pub type C2RustUnnamed_13 = libc::c_uint;
pub const NUM_THINKERLISTS: C2RustUnnamed_13 = 5;
pub const THINK_PRECIP: C2RustUnnamed_13 = 4;
pub const THINK_DYNSLOPE: C2RustUnnamed_13 = 3;
pub const THINK_MOBJ: C2RustUnnamed_13 = 2;
pub const THINK_MAIN: C2RustUnnamed_13 = 1;
pub const THINK_POLYOBJ: C2RustUnnamed_13 = 0;
pub type jingletype_t = libc::c_uint;
pub const NUMJINGLES: jingletype_t = 12;
pub const JT_SSTIMEOUT: jingletype_t = 11;
pub const JT_NIGHTSTIMEOUT: jingletype_t = 10;
pub const JT_GOVER: jingletype_t = 9;
pub const JT_SUPER: jingletype_t = 8;
pub const JT_DROWN: jingletype_t = 7;
pub const JT_MINV: jingletype_t = 6;
pub const JT_INV: jingletype_t = 5;
pub const JT_SHOES: jingletype_t = 4;
pub const JT_1UP: jingletype_t = 3;
pub const JT_MASTER: jingletype_t = 2;
pub const JT_OTHER: jingletype_t = 1;
pub const JT_NONE: jingletype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BasicFF_s {
    pub ForceX: int32_t,
    pub ForceY: int32_t,
    pub player: *const player_t,
    pub Duration: uint32_t,
    pub Gain: int32_t,
    pub Magnitude: int32_t,
}
pub type BasicFF_t = BasicFF_s;
pub type elevator_e = libc::c_uint;
pub const bridgeFall: elevator_e = 5;
pub const elevateHighest: elevator_e = 4;
pub const elevateBounce: elevator_e = 3;
pub const elevateContinuous: elevator_e = 2;
pub const elevateDown: elevator_e = 1;
pub const elevateUp: elevator_e = 0;
#[inline(always)]
unsafe extern "C" fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return ((a as int64_t * b as int64_t) as uint64_t >> 16 as libc::c_int) as fixed_t;
}
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
pub unsafe extern "C" fn P_ForceFeed(
    mut player: *const player_t,
    mut attack: int32_t,
    mut fade: int32_t,
    mut duration: tic_t,
    mut period: int32_t,
) {
    let mut Basicfeed: BasicFF_t = BasicFF_s {
        ForceX: 0,
        ForceY: 0,
        player: 0 as *const player_t,
        Duration: 0,
        Gain: 0,
        Magnitude: 0,
    };
    if player.is_null() {
        return;
    }
    Basicfeed
        .Duration = (duration as libc::c_long
        * (100 as libc::c_long / 35 as libc::c_int as libc::c_long)) as uint32_t;
    Basicfeed.ForceY = 1 as libc::c_int;
    Basicfeed.ForceX = Basicfeed.ForceY;
    Basicfeed.Gain = 25000 as libc::c_int;
    Basicfeed.Magnitude = period * 10 as libc::c_int;
    Basicfeed.player = player;
    P_RampConstant(&mut Basicfeed, attack, fade);
}
#[no_mangle]
pub unsafe extern "C" fn P_ForceConstant(mut FFInfo: *const BasicFF_t) {
    let mut ConstantQuake: JoyFF_t = JoyFF_s {
        ForceX: 0,
        ForceY: 0,
        Duration: 0,
        Gain: 0,
        Magnitude: 0,
        Start: 0,
        End: 0,
        Offset: 0,
        Phase: 0,
        Period: 0,
    };
    if FFInfo.is_null() || ((*FFInfo).player).is_null() {
        return;
    }
    ConstantQuake.ForceX = (*FFInfo).ForceX;
    ConstantQuake.ForceY = (*FFInfo).ForceY;
    ConstantQuake.Duration = (*FFInfo).Duration;
    ConstantQuake.Gain = (*FFInfo).Gain;
    ConstantQuake.Magnitude = (*FFInfo).Magnitude;
    if (*FFInfo).player
        == &mut *players.as_mut_ptr().offset(consoleplayer as isize) as *mut player_t
            as *const player_t
    {
        I_Tactile(ConstantForce, &mut ConstantQuake);
    } else if splitscreen != 0
        && (*FFInfo).player
            == &mut *players.as_mut_ptr().offset(secondarydisplayplayer as isize)
                as *mut player_t as *const player_t
    {
        I_Tactile2(ConstantForce, &mut ConstantQuake);
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RampConstant(
    mut FFInfo: *const BasicFF_t,
    mut Start: int32_t,
    mut End: int32_t,
) {
    let mut RampQuake: JoyFF_t = JoyFF_s {
        ForceX: 0,
        ForceY: 0,
        Duration: 0,
        Gain: 0,
        Magnitude: 0,
        Start: 0,
        End: 0,
        Offset: 0,
        Phase: 0,
        Period: 0,
    };
    if FFInfo.is_null() || ((*FFInfo).player).is_null() {
        return;
    }
    RampQuake.ForceX = (*FFInfo).ForceX;
    RampQuake.ForceY = (*FFInfo).ForceY;
    RampQuake.Duration = (*FFInfo).Duration;
    RampQuake.Gain = (*FFInfo).Gain;
    RampQuake.Magnitude = (*FFInfo).Magnitude;
    RampQuake.Start = Start;
    RampQuake.End = End;
    if (*FFInfo).player
        == &mut *players.as_mut_ptr().offset(consoleplayer as isize) as *mut player_t
            as *const player_t
    {
        I_Tactile(ConstantForce, &mut RampQuake);
    } else if splitscreen != 0
        && (*FFInfo).player
            == &mut *players.as_mut_ptr().offset(secondarydisplayplayer as isize)
                as *mut player_t as *const player_t
    {
        I_Tactile2(ConstantForce, &mut RampQuake);
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_ClearStarPost(mut postnum: int32_t) {
    let mut th: *mut thinker_t = 0 as *mut thinker_t;
    let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
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
                    P_RemoveThinkerDelayed as unsafe extern "C" fn(*mut thinker_t) -> (),
                ),
            ))
        {
            mo2 = th as *mut mobj_t;
            if !((*mo2).type_0 as libc::c_uint
                != MT_STARPOST as libc::c_int as libc::c_uint)
            {
                if !((*mo2).health > postnum) {
                    P_SetMobjState(mo2, (*(*mo2).info).seestate);
                }
            }
        }
        th = (*th).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_ResetStarposts() {
    let mut th: *mut thinker_t = 0 as *mut thinker_t;
    let mut post: *mut mobj_t = 0 as *mut mobj_t;
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
                    P_RemoveThinkerDelayed as unsafe extern "C" fn(*mut thinker_t) -> (),
                ),
            ))
        {
            post = th as *mut mobj_t;
            if !((*post).type_0 as libc::c_uint
                != MT_STARPOST as libc::c_int as libc::c_uint)
            {
                P_SetMobjState(post, (*(*post).info).spawnstate);
            }
        }
        th = (*th).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_CanPickupItem(
    mut player: *mut player_t,
    mut weapon: boolean,
) -> boolean {
    if ((*player).mo).is_null() || (*(*player).mo).health <= 0 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if (*player).bot as libc::c_int != 0
        && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
    {
        if weapon != 0 {
            return false_0 as libc::c_int;
        }
        return P_CanPickupItem(
            &mut *players.as_mut_ptr().offset(consoleplayer as isize),
            true_0 as libc::c_int,
        );
    }
    if (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int
        > flashingtics as libc::c_int / 4 as libc::c_int * 3 as libc::c_int
        && ((*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int)
            < 65535 as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_CanPickupEmblem(
    mut player: *mut player_t,
    mut emblemID: int32_t,
) -> boolean {
    let mut emblem: *mut emblem_t = 0 as *mut emblem_t;
    if emblemID < 0 as libc::c_int || emblemID >= numemblems {
        return false_0 as libc::c_int;
    }
    emblem = &mut *emblemlocations.as_mut_ptr().offset(emblemID as isize)
        as *mut emblem_t;
    if demoplayback != 0 {
        return false_0 as libc::c_int;
    }
    if (*player).bot as libc::c_int != 0
        && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
    {
        return false_0 as libc::c_int;
    }
    if (*emblem).type_0 as libc::c_int == 1 as libc::c_int {
        let mut skinnum: int32_t = M_EmblemSkinNum(emblem);
        if (*player).skin != skinnum {
            return false_0 as libc::c_int;
        }
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_EmblemWasCollected(mut emblemID: int32_t) -> boolean {
    if emblemID < 0 as libc::c_int || emblemID >= numemblems {
        return true_0 as libc::c_int;
    }
    if shareEmblems as libc::c_int != 0
        && (*serverGamedata).collected[emblemID as usize] == 0
    {
        return false_0 as libc::c_int;
    }
    return (*clientGamedata).collected[emblemID as usize];
}
#[no_mangle]
pub unsafe extern "C" fn P_DoNightsScore(mut player: *mut player_t) {
    let mut dummymo: *mut mobj_t = 0 as *mut mobj_t;
    if (*player).exiting != 0 {
        return;
    }
    dummymo = P_SpawnMobj(
        (*(*player).mo).x,
        (*(*player).mo).y,
        (*(*player).mo).z + (*(*player).mo).height / 2 as libc::c_int,
        MT_NIGHTSCORE,
    );
    if (*player).bot as libc::c_int != 0
        && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
    {
        player = &mut *players.as_mut_ptr().offset(consoleplayer as isize)
            as *mut player_t;
    }
    if G_IsSpecialStage(gamemap as int32_t) != 0 {
        let mut i: int32_t = 0;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0 {
                players[i as usize].linkcount += 1;
                if players[i as usize].linkcount > players[i as usize].maxlink {
                    players[i as usize].maxlink = players[i as usize].linkcount;
                }
                players[i as usize].linktimer = nightslinktics as tic_t;
            }
            i += 1;
            i;
        }
    } else {
        (*player).linkcount += 1;
        if (*player).linkcount > (*player).maxlink {
            (*player).maxlink = (*player).linkcount;
        }
        (*player).linktimer = nightslinktics as tic_t;
    }
    P_AddPlayerScore(
        player,
        ((if (*player).linkcount < 10 as libc::c_int {
            (*player).linkcount
        } else {
            10 as libc::c_int
        })
            * (if (*player).bonustime != 0 {
                20 as libc::c_int
            } else {
                10 as libc::c_int
            })) as uint32_t,
    );
    P_SetMobjState(
        dummymo,
        (if (*player).bonustime != 0 {
            (*(*dummymo).info).xdeathstate as libc::c_uint
        } else {
            (*(*dummymo).info).spawnstate as libc::c_uint
        })
            .wrapping_add(
                (if (*player).linkcount < 10 as libc::c_int {
                    (*player).linkcount
                } else {
                    10 as libc::c_int
                }) as libc::c_uint,
            )
            .wrapping_sub(1 as libc::c_int as libc::c_uint) as statenum_t,
    );
    (*dummymo).momz = (1 as libc::c_int) << 16 as libc::c_int;
    (*dummymo).fuse = 3 as libc::c_int * 35 as libc::c_int;
    (*dummymo)
        .scalespeed = ((1 as libc::c_int) << 16 as libc::c_int) / 25 as libc::c_int;
    (*dummymo).destscale = 2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
    (*dummymo).extravalue1 = (*player).linkcount - 1 as libc::c_int;
    (*dummymo)
        .extravalue2 = if (*player).linkcount - 1 as libc::c_int >= 300 as libc::c_int {
        if (*player).linkcount - 1 as libc::c_int >= 600 as libc::c_int {
            2 as libc::c_int
        } else {
            1 as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_DoMatchSuper(mut player: *mut player_t) {
    let mut match_emeralds: uint16_t = (*player)
        .powers[pw_emeralds as libc::c_int as usize];
    let mut doteams: boolean = false_0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if G_GametypeHasTeams() != 0 {
        doteams = true_0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if players[i as usize].ctfteam == (*player).ctfteam {
                match_emeralds = (match_emeralds as libc::c_int
                    | players[i as usize].powers[pw_emeralds as libc::c_int as usize]
                        as libc::c_int) as uint16_t;
            }
            i += 1;
            i;
        }
    }
    if !(match_emeralds as libc::c_int
        & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
        == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
            | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
    {
        return;
    }
    emeraldspawndelay = (invulntics as libc::c_int + 1 as libc::c_int) as uint16_t;
    (*player).powers[pw_emeralds as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    (*player).powers[pw_invulnerability as libc::c_int as usize] = emeraldspawndelay;
    (*player).powers[pw_sneakers as libc::c_int as usize] = emeraldspawndelay;
    if P_IsLocalPlayer(player) != 0
        && (*player).powers[pw_super as libc::c_int as usize] == 0
    {
        S_StopMusic();
        if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
            G_GhostAddColor(GHC_INVINCIBLE);
        }
        strlcpy(
            ((*S_sfx.as_mut_ptr().offset(sfx_None as libc::c_int as isize)).caption)
                .as_mut_ptr(),
            b"Invincibility\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as libc::c_ulong,
        );
        S_StartCaption(
            sfx_None,
            -(1 as libc::c_int),
            (*player).powers[pw_invulnerability as libc::c_int as usize],
        );
        S_ChangeMusicEx(
            if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                b"_minv\0" as *const u8 as *const libc::c_char
            } else {
                b"_inv\0" as *const u8 as *const libc::c_char
            },
            0 as libc::c_int as uint16_t,
            false_0 as libc::c_int,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
            0 as libc::c_int as uint32_t,
        );
    }
    P_StealPlayerScore(player, 50 as libc::c_int as uint32_t);
    if doteams != 0 {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0
                && players[i as usize].ctfteam == (*player).ctfteam
                && players[i as usize].powers[pw_emeralds as libc::c_int as usize]
                    as libc::c_int != 0 as libc::c_int
            {
                players[i as usize]
                    .powers[pw_emeralds as libc::c_int
                    as usize] = 0 as libc::c_int as uint16_t;
                (*player)
                    .powers[pw_invulnerability as libc::c_int
                    as usize] = (invulntics as libc::c_int + 1 as libc::c_int)
                    as uint16_t;
                (*player)
                    .powers[pw_sneakers as libc::c_int
                    as usize] = (*player)
                    .powers[pw_invulnerability as libc::c_int as usize];
                if P_IsLocalPlayer(player) != 0
                    && (*player).powers[pw_super as libc::c_int as usize] == 0
                {
                    S_StopMusic();
                    if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                        G_GhostAddColor(GHC_INVINCIBLE);
                    }
                    strlcpy(
                        ((*S_sfx.as_mut_ptr().offset(sfx_None as libc::c_int as isize))
                            .caption)
                            .as_mut_ptr(),
                        b"Invincibility\0" as *const u8 as *const libc::c_char,
                        14 as libc::c_int as libc::c_ulong,
                    );
                    S_StartCaption(
                        sfx_None,
                        -(1 as libc::c_int),
                        (*player).powers[pw_invulnerability as libc::c_int as usize],
                    );
                    S_ChangeMusicEx(
                        if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                            b"_minv\0" as *const u8 as *const libc::c_char
                        } else {
                            b"_inv\0" as *const u8 as *const libc::c_char
                        },
                        0 as libc::c_int as uint16_t,
                        false_0 as libc::c_int,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                        0 as libc::c_int as uint32_t,
                    );
                }
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_TouchSpecialThing(
    mut special: *mut mobj_t,
    mut toucher: *mut mobj_t,
    mut heightcheck: boolean,
) {
    let mut player: *mut player_t = 0 as *mut player_t;
    let mut i: int32_t = 0;
    let mut elementalpierce: uint8_t = 0;
    if objectplacing != 0 {
        return;
    }
    if (*toucher).health <= 0 as libc::c_int {
        return;
    }
    if (*special).health <= 0 as libc::c_int {
        return;
    }
    if heightcheck != 0 {
        if (*special).type_0 as libc::c_uint
            == MT_FLINGEMERALD as libc::c_int as libc::c_uint
        {
            if (*toucher).z > (*special).z + (*special).height {
                return;
            }
            if (*special).z - (*special).height > (*toucher).z + (*toucher).height {
                return;
            }
        } else {
            if (*toucher).momz < 0 as libc::c_int {
                if (*toucher).z + (*toucher).momz > (*special).z + (*special).height {
                    return;
                }
            } else if (*toucher).z > (*special).z + (*special).height {
                return
            }
            if (*toucher).momz > 0 as libc::c_int {
                if (*toucher).z + (*toucher).height + (*toucher).momz < (*special).z {
                    return;
                }
            } else if (*toucher).z + (*toucher).height < (*special).z {
                return
            }
        }
    }
    player = (*toucher).player;
    if (*player).spectator != 0 {
        return;
    }
    if (*special).flags & (MF_ENEMY as libc::c_int | MF_BOSS as libc::c_int) as uint32_t
        != 0 && (*special).flags2 & MF2_FRET as libc::c_int as uint32_t != 0
    {
        return;
    }
    if LUA_HookTouchSpecial(special, toucher) != 0 || P_MobjWasRemoved(special) != 0 {
        return;
    }
    elementalpierce = (if ((*player).powers[pw_shield as libc::c_int as usize]
        as libc::c_int & SH_NOSTACK as libc::c_int == SH_ELEMENTAL as libc::c_int
        || (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_NOSTACK as libc::c_int == SH_BUBBLEWRAP as libc::c_int)
        && (*player).pflags as libc::c_uint
            & PF_SHIELDABILITY as libc::c_int as libc::c_uint != 0
    {
        if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_NOSTACK as libc::c_int == SH_ELEMENTAL as libc::c_int
        {
            1 as libc::c_int
        } else {
            2 as libc::c_int
        }
    } else {
        0 as libc::c_int
    }) as uint8_t;
    if (*special).flags & (MF_ENEMY as libc::c_int | MF_BOSS as libc::c_int) as uint32_t
        != 0 && (*special).flags & MF_MISSILE as libc::c_int as uint32_t == 0
    {
        match (*special).type_0 as libc::c_uint {
            88 => {
                P_DamageMobj(
                    toucher,
                    special,
                    special,
                    1 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                );
                return;
            }
            153 => {
                (*special).momx = (*toucher).momx / 3 as libc::c_int;
                (*special).momy = (*toucher).momy / 3 as libc::c_int;
                (*special).momz = (*toucher).momz / 3 as libc::c_int;
                (*toucher).momx /= -(8 as libc::c_int);
                (*toucher).momy /= -(8 as libc::c_int);
                (*toucher).momz /= -(8 as libc::c_int);
                (*special).flags &= !(MF_SPECIAL as libc::c_int) as uint32_t;
                if (*(*special).info).activesound as u64 != 0 {
                    S_StartSound(
                        special as *const libc::c_void,
                        (*(*special).info).activesound,
                    );
                }
                P_SetTarget2(&mut (*special).tracer, toucher);
                (*player).homing = 0 as libc::c_int as uint8_t;
                return;
            }
            34 => {
                if elementalpierce == 0
                    && (*toucher).z < (*special).z + (*special).height
                    && (*toucher).z + (*toucher).height > (*special).z
                    && P_DamageMobj(
                        toucher,
                        special,
                        special,
                        1 as libc::c_int,
                        4 as libc::c_int as uint8_t,
                    ) != 0
                {
                    return;
                }
            }
            18 => {
                if P_MobjFlip(toucher) as libc::c_int
                    * ((*toucher).z
                        - ((*special).z + (*special).height / 2 as libc::c_int))
                    > 0 as libc::c_int
                {
                    if (*player).pflags as libc::c_uint
                        & PF_BOUNCING as libc::c_int as libc::c_uint != 0
                    {
                        (*toucher).momz = -(*toucher).momz;
                        P_DoAbilityBounce(player, false_0 as libc::c_int);
                        return;
                    } else if P_DamageMobj(
                        toucher,
                        special,
                        special,
                        1 as libc::c_int,
                        4 as libc::c_int as uint8_t,
                    ) != 0
                    {
                        return
                    }
                }
            }
            78 => {
                if (*player).powers[pw_flashing as libc::c_int as usize] == 0
                    && !((*player).charability as libc::c_int
                        == CA_TWINSPIN as libc::c_int
                        && (*player).panim as libc::c_uint
                            == PA_ABILITY as libc::c_int as libc::c_uint)
                    && !((*player).charability2 as libc::c_int
                        == CA2_MELEE as libc::c_int
                        && (*player).panim as libc::c_uint
                            == PA_ABILITY2 as libc::c_int as libc::c_uint)
                {
                    if ((*special).state
                        == &mut *states
                            .as_mut_ptr()
                            .offset(S_FANG_BOUNCE3 as libc::c_int as isize)
                            as *mut state_t
                        || (*special).state
                            == &mut *states
                                .as_mut_ptr()
                                .offset(S_FANG_BOUNCE4 as libc::c_int as isize)
                                as *mut state_t
                        || (*special).state
                            == &mut *states
                                .as_mut_ptr()
                                .offset(S_FANG_PINCHBOUNCE3 as libc::c_int as isize)
                                as *mut state_t
                        || (*special).state
                            == &mut *states
                                .as_mut_ptr()
                                .offset(S_FANG_PINCHBOUNCE4 as libc::c_int as isize)
                                as *mut state_t)
                        && P_MobjFlip(special) as libc::c_int
                            * ((*special).z + (*special).height / 2 as libc::c_int
                                - ((*toucher).z + (*toucher).height / 2 as libc::c_int))
                            > (*toucher).height / 2 as libc::c_int
                    {
                        P_DamageMobj(
                            toucher,
                            special,
                            special,
                            1 as libc::c_int,
                            0 as libc::c_int as uint8_t,
                        );
                        P_SetTarget2(&mut (*special).tracer, toucher);
                        if (*special).state
                            == &mut *states
                                .as_mut_ptr()
                                .offset(S_FANG_PINCHBOUNCE3 as libc::c_int as isize)
                                as *mut state_t
                            || (*special).state
                                == &mut *states
                                    .as_mut_ptr()
                                    .offset(S_FANG_PINCHBOUNCE4 as libc::c_int as isize)
                                    as *mut state_t
                        {
                            P_SetMobjState(special, S_FANG_PINCHPATHINGSTART2);
                        } else {
                            var2 = 4 as libc::c_int;
                            var1 = var2;
                            A_Boss5ExtraRepeat(special);
                            P_SetMobjState(special, S_FANG_PATHINGCONT2);
                        }
                        if (*special).eflags as libc::c_int
                            & MFE_VERTICALFLIP as libc::c_int != 0
                        {
                            (*special).z = (*toucher).z - (*special).height;
                        } else {
                            (*special).z = (*toucher).z + (*toucher).height;
                        }
                        return;
                    }
                }
            }
            45 => {
                if (*special).extravalue2 == 2 as libc::c_int
                    && P_DamageMobj(
                        (*player).mo,
                        special,
                        special,
                        1 as libc::c_int,
                        2 as libc::c_int as uint8_t,
                    ) != 0
                {
                    return;
                }
            }
            _ => {}
        }
        if P_PlayerCanDamage(player, special) != 0 {
            if (*special).type_0 as libc::c_uint
                == MT_PTERABYTE as libc::c_int as libc::c_uint
                && (*special).target == (*player).mo
                && (*special).extravalue1 == 1 as libc::c_int
            {
                return;
            }
            if P_MobjFlip(toucher) as libc::c_int * (*toucher).momz < 0 as libc::c_int
                && elementalpierce as libc::c_int != 1 as libc::c_int
                && (*player).powers[pw_strong as libc::c_int as usize] as libc::c_int
                    & STR_HEAVY as libc::c_int == 0
            {
                let mut setmomz: fixed_t = -(*toucher).momz;
                if elementalpierce as libc::c_int == 2 as libc::c_int {
                    P_DoBubbleBounce(player);
                }
                (*toucher).momz = setmomz;
                if elementalpierce as libc::c_int == 2 as libc::c_int {
                    let mut underwater: boolean = (*toucher).eflags as libc::c_int
                        & MFE_UNDERWATER as libc::c_int;
                    if underwater != 0 {
                        (*toucher).momz /= 2 as libc::c_int;
                    }
                    (*toucher).momz
                        -= (*toucher).momz
                            / (if underwater != 0 {
                                8 as libc::c_int
                            } else {
                                4 as libc::c_int
                            });
                }
            }
            if (*player).pflags as libc::c_uint
                & PF_BOUNCING as libc::c_int as libc::c_uint != 0
            {
                P_DoAbilityBounce(player, false_0 as libc::c_int);
            }
            if (*(*special).info).spawnhealth > 1 as libc::c_int {
                (*toucher).momx = -(*toucher).momx;
                (*toucher).momy = -(*toucher).momy;
                if (*player).charability as libc::c_int == CA_FLY as libc::c_int
                    && (*player).panim as libc::c_uint
                        == PA_ABILITY as libc::c_int as libc::c_uint
                {
                    (*toucher).momz = -(*toucher).momz / 2 as libc::c_int;
                } else if (*player).pflags as libc::c_uint
                    & PF_GLIDING as libc::c_int as libc::c_uint != 0
                    && P_IsObjectOnGround(toucher) == 0
                {
                    (*player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*player).pflags as libc::c_uint
                            & !(PF_GLIDING as libc::c_int | PF_JUMPED as libc::c_int
                                | PF_NOJUMPDAMAGE as libc::c_int) as libc::c_uint,
                    );
                    P_SetPlayerMobjState(toucher, S_PLAY_FALL);
                    (*toucher).momz
                        += P_MobjFlip(toucher) as libc::c_int
                            * ((*player).speed >> 3 as libc::c_int);
                    (*toucher)
                        .momx = 7 as libc::c_int * (*toucher).momx >> 3 as libc::c_int;
                    (*toucher)
                        .momy = 7 as libc::c_int * (*toucher).momy >> 3 as libc::c_int;
                } else if (*player).powers[pw_strong as libc::c_int as usize]
                    as libc::c_int & STR_DASH as libc::c_int != 0
                    && (*player).panim as libc::c_uint
                        == PA_DASH as libc::c_int as libc::c_uint
                {
                    P_DoPlayerPain(player, special, special);
                }
            }
            P_DamageMobj(
                special,
                toucher,
                toucher,
                1 as libc::c_int,
                0 as libc::c_int as uint8_t,
            );
            if (*player).charability as libc::c_int == CA_TWINSPIN as libc::c_int
                && (*player).panim as libc::c_uint
                    == PA_ABILITY as libc::c_int as libc::c_uint
            {
                P_TwinSpinRejuvenate(player, (*player).thokitem);
            }
        } else {
            if (*special).type_0 as libc::c_uint
                == MT_PTERABYTE as libc::c_int as libc::c_uint
                && (*special).target == (*player).mo
            {
                return;
            }
            P_DamageMobj(
                toucher,
                special,
                special,
                1 as libc::c_int,
                0 as libc::c_int as uint8_t,
            );
        }
        return;
    } else if (*special).flags & MF_FIRE as libc::c_int as uint32_t != 0 {
        P_DamageMobj(
            toucher,
            special,
            special,
            1 as libc::c_int,
            2 as libc::c_int as uint8_t,
        );
        return;
    } else {
        let mut current_block_856: u64;
        match (*special).type_0 as libc::c_uint {
            112 => {
                if (*player).ctfteam != 1 as libc::c_int {
                    return;
                }
                current_block_856 = 16677036429817807717;
            }
            113 => {
                current_block_856 = 16677036429817807717;
            }
            107 | 108 | 550 | 551 | 583 => {
                current_block_856 = 4166128649646877775;
            }
            109 | 110 | 581 | 582 => {
                if P_CanPickupItem(player, false_0 as libc::c_int) == 0
                    && (*special).flags2 & MF2_NIGHTSPULL as libc::c_int as uint32_t == 0
                {
                    return;
                }
                (*special).momz = 0 as libc::c_int;
                (*special).momy = (*special).momz;
                (*special).momx = (*special).momy;
                P_GivePlayerSpheres(player, 1 as libc::c_int);
                if (*special).type_0 as libc::c_uint
                    == MT_BLUESPHERE as libc::c_int as libc::c_uint
                    || (*special).type_0 as libc::c_uint
                        == MT_FLINGBLUESPHERE as libc::c_int as libc::c_uint
                {
                    (*special)
                        .destscale = (if (*player)
                        .powers[pw_carry as libc::c_int as usize] as libc::c_int
                        == CR_NIGHTSMODE as libc::c_int
                    {
                        4 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) * (*special).scale;
                    if states[(*(*special).info).deathstate as usize].tics
                        > 0 as libc::c_int
                    {
                        (*special)
                            .scalespeed = FixedDiv(
                            FixedDiv((*special).destscale, (*special).scale),
                            states[(*(*special).info).deathstate as usize].tics
                                << 16 as libc::c_int,
                        );
                    } else {
                        (*special)
                            .scalespeed = 4 as libc::c_int
                            * ((1 as libc::c_int) << 16 as libc::c_int)
                            / 5 as libc::c_int;
                    }
                }
                if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
                    P_DoNightsScore(player);
                }
                current_block_856 = 14894058611842117342;
            }
            111 => {
                if P_CanPickupItem(player, false_0 as libc::c_int) == 0
                    && (*special).flags2 & MF2_NIGHTSPULL as libc::c_int as uint32_t == 0
                {
                    return;
                }
                (*special).momz = 0 as libc::c_int;
                (*special).momy = (*special).momz;
                (*special).momx = (*special).momy;
                P_DamageMobj(
                    toucher,
                    special,
                    special,
                    1 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                );
                current_block_856 = 14894058611842117342;
            }
            540 | 538 | 542 | 543 | 541 | 539 => {
                if P_CanPickupItem(player, true_0 as libc::c_int) == 0 {
                    return;
                }
                if (*(*special).info).mass >= pw_infinityring as libc::c_int
                    && (*(*special).info).mass <= pw_railring as libc::c_int
                {
                    let mut pindex: int32_t = (*(*special).info).mass
                        - pw_infinityring as libc::c_int;
                    (*player)
                        .powers[(*(*special).info).mass
                        as usize] = ((*player).powers[(*(*special).info).mass as usize]
                        as libc::c_int
                        + (*special).reactiontime as uint16_t as libc::c_int)
                        as uint16_t;
                    (*player).ringweapons
                        |= (1 as libc::c_int) << pindex - 1 as libc::c_int;
                    if (*player).powers[(*(*special).info).mass as usize] as libc::c_int
                        > rw_maximums[pindex as usize] as libc::c_int
                    {
                        (*player)
                            .powers[(*(*special).info).mass
                            as usize] = rw_maximums[pindex as usize] as uint16_t;
                    }
                }
                current_block_856 = 14894058611842117342;
            }
            533 | 534 | 531 | 536 | 537 | 535 | 532 => {
                if P_CanPickupItem(player, true_0 as libc::c_int) == 0 {
                    return;
                }
                if (*(*special).info).mass >= pw_infinityring as libc::c_int
                    && (*(*special).info).mass <= pw_railring as libc::c_int
                {
                    let mut pindex_0: int32_t = (*(*special).info).mass
                        - pw_infinityring as libc::c_int;
                    (*player)
                        .powers[(*(*special).info).mass
                        as usize] = ((*player).powers[(*(*special).info).mass as usize]
                        as libc::c_int + (*special).health as uint16_t as libc::c_int)
                        as uint16_t;
                    if (*player).powers[(*(*special).info).mass as usize] as libc::c_int
                        > rw_maximums[pindex_0 as usize] as libc::c_int
                    {
                        (*player)
                            .powers[(*(*special).info).mass
                            as usize] = rw_maximums[pindex_0 as usize] as uint16_t;
                    }
                }
                current_block_856 = 14894058611842117342;
            }
            114 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                P_AddPlayerScore(player, 1000 as libc::c_int as uint32_t);
                if gametyperules & GTR_SPECIALSTAGES as libc::c_int as uint32_t == 0
                    || modeattacking as libc::c_int != 0
                {
                    S_StartSound(toucher as *const libc::c_void, sfx_chchng);
                } else {
                    tokenlist = tokenlist.wrapping_add((*special).health as uint32_t);
                    if emeralds as libc::c_int
                        & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                            | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                            | 64 as libc::c_int)
                        == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                            | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                            | 64 as libc::c_int
                    {
                        if multiplayer == 0
                            && (ultimatemode as libc::c_int != 0
                                || useContinues as libc::c_int != 0
                                    && marathonmode as u64 == 0
                                || modeattacking == 0 && !(cursaveslot > 0 as libc::c_int))
                        {
                            (*player)
                                .continues = ((*player).continues as libc::c_int
                                + 1 as libc::c_int) as int8_t;
                            (*player).gotcontinue = true_0 as libc::c_int as uint8_t;
                            if P_IsLocalPlayer(player) != 0 {
                                S_StartSound(0 as *const libc::c_void, sfx_s3kac);
                            } else {
                                S_StartSound(toucher as *const libc::c_void, sfx_chchng);
                            }
                        } else {
                            P_GiveCoopLives(
                                player,
                                1 as libc::c_int,
                                true_0 as libc::c_int,
                            );
                            S_StartSound(toucher as *const libc::c_void, sfx_chchng);
                        }
                    } else {
                        token = token.wrapping_add(1);
                        token;
                        S_StartSound(toucher as *const libc::c_void, sfx_token);
                    }
                }
                current_block_856 = 14894058611842117342;
            }
            125 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if hunt1 == special {
                    hunt1 = 0 as *mut mobj_t;
                } else if hunt2 == special {
                    hunt2 = 0 as *mut mobj_t;
                } else if hunt3 == special {
                    hunt3 = 0 as *mut mobj_t;
                }
                if hunt1.is_null() && hunt2.is_null() && hunt3.is_null() {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if !(playeringame[i as usize] == 0
                            || players[i as usize].spectator != 0)
                        {
                            players[i as usize]
                                .exiting = (14 as libc::c_int * 35 as libc::c_int
                                / 5 as libc::c_int + 1 as libc::c_int) as tic_t;
                        }
                        i += 1;
                        i;
                    }
                }
                current_block_856 = 14894058611842117342;
            }
            118 | 119 | 120 | 121 | 122 | 123 | 124 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if (*special).threshold != 0 {
                    (*player)
                        .powers[pw_emeralds as libc::c_int
                        as usize] = ((*player)
                        .powers[pw_emeralds as libc::c_int as usize] as libc::c_int
                        | (*(*special).info).speed) as uint16_t;
                    P_DoMatchSuper(player);
                } else {
                    emeralds = (emeralds as libc::c_int | (*(*special).info).speed)
                        as uint16_t;
                    stagefailed = false_0 as libc::c_int;
                }
                if !((*special).target).is_null()
                    && (*(*special).target).type_0 as libc::c_uint
                        == MT_EMERALDSPAWN as libc::c_int as libc::c_uint
                {
                    if !((*(*special).target).target).is_null() {
                        P_SetTarget2(&mut (*(*special).target).target, 0 as *mut mobj_t);
                    }
                    (*(*special).target).threshold = 0 as libc::c_int;
                    P_SetTarget2(&mut (*special).target, 0 as *mut mobj_t);
                }
                current_block_856 = 14894058611842117342;
            }
            127 => {
                if P_CanPickupItem(player, true_0 as libc::c_int) == 0
                    || (*player).tossdelay != 0
                {
                    return;
                }
                (*player)
                    .powers[pw_emeralds as libc::c_int
                    as usize] = ((*player).powers[pw_emeralds as libc::c_int as usize]
                    as libc::c_int | (*special).threshold) as uint16_t;
                P_DoMatchSuper(player);
                current_block_856 = 14894058611842117342;
            }
            117 => {
                let toucherIsServer: boolean = (player.offset_from(players.as_mut_ptr())
                    as libc::c_long == serverplayer as libc::c_long) as libc::c_int;
                let consoleIsServer: boolean = (consoleplayer == serverplayer)
                    as libc::c_int;
                let mut prevCollected: boolean = false_0 as libc::c_int;
                if (*special).flags2 & MF2_NIGHTSPULL as libc::c_int as uint32_t != 0
                    && toucher == (*special).tracer
                {
                    P_SetTarget2(&mut (*special).tracer, 0 as *mut mobj_t);
                    (*special).flags2 &= !(MF2_NIGHTSPULL as libc::c_int) as uint32_t;
                    (*special).movefactor = 0 as libc::c_int;
                    (*special).momz = 0 as libc::c_int;
                    (*special).momy = (*special).momz;
                    (*special).momx = (*special).momy;
                }
                if P_CanPickupEmblem(player, (*special).health - 1 as libc::c_int) == 0 {
                    return;
                }
                prevCollected = P_EmblemWasCollected(
                    (*special).health - 1 as libc::c_int,
                );
                if toucherIsServer != 0 || shareEmblems as libc::c_int != 0 {
                    (*serverGamedata)
                        .collected[((*special).health - 1 as libc::c_int)
                        as usize] = true_0 as libc::c_int;
                    M_SilentUpdateUnlockablesAndEmblems(serverGamedata);
                }
                if P_IsLocalPlayer(player) != 0
                    || consoleIsServer != 0 && shareEmblems as libc::c_int != 0
                {
                    (*clientGamedata)
                        .collected[((*special).health - 1 as libc::c_int)
                        as usize] = true_0 as libc::c_int;
                    M_UpdateUnlockablesAndExtraEmblems(clientGamedata);
                    G_SaveGameData(clientGamedata);
                }
                if netgame != 0 {
                    let mut spark: *mut mobj_t = P_SpawnMobjFromMobj(
                        special,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        0 as libc::c_int,
                        MT_SPARK,
                    );
                    if prevCollected == false_0 as libc::c_int
                        && P_EmblemWasCollected((*special).health - 1 as libc::c_int)
                            == true_0 as libc::c_int
                    {
                        S_StartSound(
                            (if shareEmblems as libc::c_int != 0 {
                                0 as *mut mobj_t
                            } else {
                                special
                            }) as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    } else {
                        (*spark).flags2 |= MF2_DONTDRAW as libc::c_int as uint32_t;
                    }
                    return;
                } else if !(prevCollected == false_0 as libc::c_int
                    && P_EmblemWasCollected((*special).health - 1 as libc::c_int)
                        == true_0 as libc::c_int)
                {
                    return
                }
                current_block_856 = 14894058611842117342;
            }
            115 | 116 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int
                    != 0 || (*player).tossdelay != 0
                {
                    return;
                }
                if ((*special).spawnpoint).is_null() {
                    return;
                }
                if (*special).fuse == 1 as libc::c_int {
                    return;
                }
                let mut flagteam: uint8_t = (if (*special).type_0 as libc::c_uint
                    == MT_REDFLAG as libc::c_int as libc::c_uint
                {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                }) as uint8_t;
                let mut specialflag: sectorspecialflags_t = (if (*special).type_0
                    as libc::c_uint == MT_REDFLAG as libc::c_int as libc::c_uint
                {
                    SSF_REDTEAMBASE as libc::c_int
                } else {
                    SSF_BLUETEAMBASE as libc::c_int
                }) as sectorspecialflags_t;
                let mut flagtext: *const libc::c_char = 0 as *const libc::c_char;
                let mut flagcolor: libc::c_char = 0;
                let mut plname: [libc::c_char; 25] = [0; 25];
                if (*special).type_0 as libc::c_uint
                    == MT_REDFLAG as libc::c_int as libc::c_uint
                {
                    flagtext = b"Red flag\0" as *const u8 as *const libc::c_char;
                    flagcolor = -123i32 as libc::c_char;
                } else {
                    flagtext = b"Blue flag\0" as *const u8 as *const libc::c_char;
                    flagcolor = -124i32 as libc::c_char;
                }
                snprintf(
                    plname.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
                    b"%s%s%s\0" as *const u8 as *const libc::c_char,
                    if (*player).ctfteam != 0 {
                        if (*player).ctfteam == 1 as libc::c_int {
                            b"\x85\0" as *const u8 as *const libc::c_char
                        } else {
                            b"\x84\0" as *const u8 as *const libc::c_char
                        }
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                    (player_names[player.offset_from(players.as_mut_ptr())
                        as libc::c_long as usize])
                        .as_mut_ptr(),
                    if (*player).ctfteam != 0 {
                        b"\x80\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\0" as *const u8 as *const libc::c_char
                    },
                );
                if (*player).ctfteam == flagteam as libc::c_int {
                    if (*special).x >> 16 as libc::c_int
                        != (*(*special).spawnpoint).x as libc::c_int
                        || (*special).y >> 16 as libc::c_int
                            != (*(*special).spawnpoint).y as libc::c_int
                    {
                        (*special).fuse = 1 as libc::c_int;
                        (*special).flags2 |= MF2_JUSTATTACKED as libc::c_int as uint32_t;
                        if (P_PlayerTouchingSectorSpecialFlag(player, specialflag))
                            .is_null()
                        {
                            CONS_Printf(
                                b"%s returned the %c%s%c to base.\n\0" as *const u8
                                    as *const libc::c_char,
                                plname.as_mut_ptr(),
                                flagcolor as libc::c_int,
                                flagtext,
                                0x80 as libc::c_int,
                            );
                        }
                    }
                    current_block_856 = 13613992618864203028;
                } else if (*player).ctfteam != 0 {
                    let mut flagflag: uint16_t = (if (*special).type_0 as libc::c_uint
                        == MT_REDFLAG as libc::c_int as libc::c_uint
                    {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }) as uint16_t;
                    let mut flagmobj: *mut *mut mobj_t = if (*special).type_0
                        as libc::c_uint == MT_REDFLAG as libc::c_int as libc::c_uint
                    {
                        &mut redflag
                    } else {
                        &mut blueflag
                    };
                    if (*player).powers[pw_super as libc::c_int as usize] != 0 {
                        return;
                    }
                    (*player)
                        .gotflag = ((*player).gotflag as libc::c_int
                        | flagflag as libc::c_int) as uint16_t;
                    CONS_Printf(
                        b"%s picked up the %c%s%c!\n\0" as *const u8
                            as *const libc::c_char,
                        plname.as_mut_ptr(),
                        flagcolor as libc::c_int,
                        flagtext,
                        0x80 as libc::c_int,
                    );
                    *flagmobj = 0 as *mut mobj_t;
                    current_block_856 = 14894058611842117342;
                } else {
                    current_block_856 = 13613992618864203028;
                }
                match current_block_856 {
                    14894058611842117342 => {}
                    _ => return,
                }
            }
            570 => {
                let mut spec: boolean = G_IsSpecialStage(gamemap as int32_t);
                let mut cangiveemmy: boolean = false_0 as libc::c_int;
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if (*player).exiting != 0 {
                    return;
                }
                if (*player).bonustime != 0 {
                    if spec != 0 {
                        if !(!((*toucher).tracer).is_null()
                            && (*(*toucher).tracer).type_0 as libc::c_uint
                                == MT_GOTEMERALD as libc::c_int as libc::c_uint)
                        {
                            i = 0 as libc::c_int;
                            while i < 32 as libc::c_int {
                                if playeringame[i as usize] != 0
                                    && players[i as usize].playerstate as libc::c_uint
                                        == PST_LIVE as libc::c_int as libc::c_uint
                                    && !((*players[i as usize].mo).tracer).is_null()
                                    && (*(*players[i as usize].mo).tracer).type_0
                                        as libc::c_uint
                                        == MT_GOTEMERALD as libc::c_int as libc::c_uint
                                {
                                    return;
                                }
                                i += 1;
                                i;
                            }
                        }
                        cangiveemmy = true_0 as libc::c_int;
                    } else {
                        S_StartSound(
                            toucher as *const libc::c_void,
                            (*(*special).info).activesound,
                        );
                    }
                } else {
                    if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                        == CR_NIGHTSMODE as libc::c_int
                    {
                        return;
                    }
                    S_StartSound(toucher as *const libc::c_void, sfx_supert);
                }
                P_SwitchSpheresBonusMode(false_0 as libc::c_int);
                if !(netgame != 0 || multiplayer != 0)
                    && !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    P_SetTarget2(&mut (*special).tracer, toucher);
                }
                P_SetTarget2(&mut (*player).drone, special);
                P_NightserizePlayer(player, (*special).health);
                if spec == 0 {
                    if !((*toucher).tracer).is_null() {
                        let mut orbittarget: *mut mobj_t = if !((*special).target)
                            .is_null()
                        {
                            (*special).target
                        } else {
                            special
                        };
                        let mut hnext: *mut mobj_t = (*orbittarget).hnext;
                        let mut anchorpoint: *mut mobj_t = 0 as *mut mobj_t;
                        let mut anchorpoint2: *mut mobj_t = 0 as *mut mobj_t;
                        let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
                        let mut th: *mut thinker_t = 0 as *mut thinker_t;
                        th = (*thlist
                            .as_mut_ptr()
                            .offset(THINK_MOBJ as libc::c_int as isize))
                            .next;
                        while th
                            != &mut *thlist
                                .as_mut_ptr()
                                .offset(THINK_MOBJ as libc::c_int as isize)
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
                                if !((*mo2).type_0 as libc::c_uint
                                    != MT_IDEYAANCHOR as libc::c_int as libc::c_uint)
                                {
                                    if (*mo2).health == (*(*toucher).tracer).health {
                                        anchorpoint = mo2;
                                    } else if !((*(*toucher).tracer).hnext).is_null()
                                        && (*mo2).health == (*(*(*toucher).tracer).hnext).health
                                    {
                                        anchorpoint2 = mo2;
                                    }
                                    if ((*(*toucher).tracer).hnext).is_null()
                                        && !anchorpoint.is_null()
                                        || !((*(*toucher).tracer).hnext).is_null()
                                            && !anchorpoint.is_null() && !anchorpoint2.is_null()
                                    {
                                        break;
                                    }
                                }
                            }
                            th = (*th).next;
                        }
                        if !anchorpoint.is_null() {
                            (*(*toucher).tracer).flags
                                |= MF_GRENADEBOUNCE as libc::c_int as uint32_t;
                            (*(*toucher).tracer)
                                .threshold = (8 as libc::c_int) << 20 as libc::c_int;
                        }
                        if !anchorpoint2.is_null() {
                            (*(*(*toucher).tracer).hnext).flags
                                |= MF_GRENADEBOUNCE as libc::c_int as uint32_t;
                            (*(*(*toucher).tracer).hnext)
                                .threshold = (8 as libc::c_int) << 20 as libc::c_int;
                        }
                        P_SetTarget2(&mut (*orbittarget).hnext, (*toucher).tracer);
                        if ((*(*orbittarget).hnext).hnext).is_null() {
                            P_SetTarget2(&mut (*(*orbittarget).hnext).hnext, hnext);
                        } else {
                            P_SetTarget2(
                                &mut (*(*(*orbittarget).hnext).hnext).target,
                                if !anchorpoint2.is_null() {
                                    anchorpoint2
                                } else {
                                    orbittarget
                                },
                            );
                        }
                        P_SetTarget2(
                            &mut (*(*orbittarget).hnext).target,
                            if !anchorpoint.is_null() {
                                anchorpoint
                            } else {
                                orbittarget
                            },
                        );
                        P_SetTarget2(&mut (*toucher).tracer, 0 as *mut mobj_t);
                        if !hnext.is_null() {
                            (*(*orbittarget).hnext)
                                .extravalue1 = ((*hnext).extravalue1
                                - 72 as libc::c_int * 0xb60b61 as libc::c_int) as angle_t
                                as int32_t;
                            if (*(*orbittarget).hnext).extravalue1 > (*hnext).extravalue1
                            {
                                (*(*orbittarget).hnext).extravalue1
                                    -= 72 as libc::c_int * 0xb60b61 as libc::c_int
                                        / (*(*orbittarget).hnext).extravalue1;
                            }
                        }
                    }
                    if (*player).exiting != 0 {
                        let mut hnext_0: *mut mobj_t = if !((*special).target).is_null()
                        {
                            (*special).target
                        } else {
                            special
                        };
                        loop {
                            hnext_0 = (*hnext_0).hnext;
                            if hnext_0.is_null() {
                                break;
                            }
                            (*hnext_0).flags
                                &= !(MF_GRENADEBOUNCE as libc::c_int) as uint32_t;
                            (*hnext_0).threshold = 0 as libc::c_int;
                            P_SetTarget2(&mut (*hnext_0).target, toucher);
                        }
                    }
                    return;
                }
                if cangiveemmy == 0 {
                    return;
                }
                if (*player).exiting != 0 {
                    P_GiveEmerald(false_0 as libc::c_int);
                } else if !((*(*player).mo).tracer).is_null()
                    && (*player).mare as libc::c_int != 0
                {
                    P_KillMobj(
                        (*toucher).tracer,
                        0 as *mut mobj_t,
                        0 as *mut mobj_t,
                        0 as libc::c_int as uint8_t,
                    );
                    S_StartSound(0 as *const libc::c_void, sfx_ghosty);
                    (*special).flags2 |= MF2_DONTDRAW as libc::c_int as uint32_t;
                }
                return;
            }
            575 => {
                if (*special).fuse < (*toucher).fuse - 35 as libc::c_int {
                    let mut th_0: *mut thinker_t = 0 as *mut thinker_t;
                    let mut mo2_0: *mut mobj_t = 0 as *mut mobj_t;
                    let mut count: int32_t = 0;
                    let mut x: fixed_t = 0;
                    let mut y: fixed_t = 0;
                    let mut z: fixed_t = 0;
                    let mut gatherradius: fixed_t = 0;
                    let mut d: angle_t = 0;
                    let mut sparklestate: statenum_t = S_NULL;
                    if (*special).target != toucher {
                        return;
                    }
                    x = (*special).x >> 16 as libc::c_int;
                    y = (*special).y >> 16 as libc::c_int;
                    z = (*special).z >> 16 as libc::c_int;
                    count = 1 as libc::c_int;
                    th_0 = (*thlist
                        .as_mut_ptr()
                        .offset(THINK_MOBJ as libc::c_int as isize))
                        .next;
                    while th_0
                        != &mut *thlist
                            .as_mut_ptr()
                            .offset(THINK_MOBJ as libc::c_int as isize) as *mut thinker_t
                    {
                        if !((*th_0).function.acp1
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
                            mo2_0 = th_0 as *mut mobj_t;
                            if !(mo2_0 == special) {
                                if !((*mo2_0).target != toucher) {
                                    if (*mo2_0).type_0 as libc::c_uint
                                        == MT_NIGHTSPARKLE as libc::c_int as libc::c_uint
                                    {
                                        (*mo2_0).tics = 1 as libc::c_int;
                                    } else if (*mo2_0).type_0 as libc::c_uint
                                        == MT_NIGHTSLOOPHELPER as libc::c_int as libc::c_uint
                                    {
                                        if (*mo2_0).fuse >= (*special).fuse {
                                            count += 1;
                                            count;
                                            x += (*mo2_0).x >> 16 as libc::c_int;
                                            y += (*mo2_0).y >> 16 as libc::c_int;
                                            z += (*mo2_0).z >> 16 as libc::c_int;
                                        }
                                        P_RemoveMobj(mo2_0);
                                    }
                                }
                            }
                        }
                        th_0 = (*th_0).next;
                    }
                    x = x / count << 16 as libc::c_int;
                    y = y / count << 16 as libc::c_int;
                    z = z / count << 16 as libc::c_int;
                    gatherradius = P_AproxDistance(
                        P_AproxDistance((*special).x - x, (*special).y - y),
                        (*special).z - z,
                    );
                    P_RemoveMobj(special);
                    if (*player).powers[pw_nights_superloop as libc::c_int as usize] != 0
                    {
                        gatherradius *= 2 as libc::c_int;
                        sparklestate = mobjinfo[MT_NIGHTSPARKLE as libc::c_int as usize]
                            .seestate;
                    }
                    if gatherradius
                        < 30 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                    {
                        return;
                    }
                    d = 0 as libc::c_int as angle_t;
                    while d < 16 as libc::c_int as angle_t {
                        P_SpawnParaloop(
                            x,
                            y,
                            z,
                            gatherradius,
                            16 as libc::c_int,
                            MT_NIGHTSPARKLE,
                            sparklestate,
                            d * 0x10000000 as libc::c_int as angle_t,
                            false_0 as libc::c_int,
                        );
                        d = d.wrapping_add(1);
                        d;
                    }
                    S_StartSound(toucher as *const libc::c_void, sfx_prloop);
                    let mut current_block_397: u64;
                    th_0 = (*thlist
                        .as_mut_ptr()
                        .offset(THINK_MOBJ as libc::c_int as isize))
                        .next;
                    while th_0
                        != &mut *thlist
                            .as_mut_ptr()
                            .offset(THINK_MOBJ as libc::c_int as isize) as *mut thinker_t
                    {
                        if !((*th_0).function.acp1
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
                            mo2_0 = th_0 as *mut mobj_t;
                            if !(P_AproxDistance(
                                P_AproxDistance((*mo2_0).x - x, (*mo2_0).y - y),
                                (*mo2_0).z - z,
                            ) > gatherradius)
                            {
                                if (*mo2_0).flags & MF_SHOOTABLE as libc::c_int as uint32_t
                                    != 0
                                {
                                    P_DamageMobj(
                                        mo2_0,
                                        toucher,
                                        toucher,
                                        1 as libc::c_int,
                                        0 as libc::c_int as uint8_t,
                                    );
                                } else {
                                    if (*mo2_0).flags & MF_NIGHTSITEM as libc::c_int as uint32_t
                                        != 0
                                    {
                                        if (*mo2_0).flags2
                                            & MF2_STRONGBOX as libc::c_int as uint32_t != 0
                                            && (*player).bonustime == 0
                                        {
                                            current_block_397 = 309319537768397308;
                                        } else if (*mo2_0).flags
                                            & MF_SPECIAL as libc::c_int as uint32_t == 0
                                            && (*mo2_0).health != 0
                                        {
                                            (*mo2_0).flags2
                                                &= !(MF2_DONTDRAW as libc::c_int) as uint32_t;
                                            (*mo2_0).flags |= MF_SPECIAL as libc::c_int as uint32_t;
                                            (*mo2_0).flags
                                                &= !(MF_NIGHTSITEM as libc::c_int) as uint32_t;
                                            S_StartSound(toucher as *const libc::c_void, sfx_hidden);
                                            current_block_397 = 309319537768397308;
                                        } else {
                                            current_block_397 = 11216321167622260211;
                                        }
                                    } else {
                                        current_block_397 = 11216321167622260211;
                                    }
                                    match current_block_397 {
                                        309319537768397308 => {}
                                        _ => {
                                            if (*mo2_0).type_0 as libc::c_uint
                                                == MT_RING as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_COIN as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_BLUESPHERE as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_BOMBSPHERE as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_NIGHTSCHIP as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_NIGHTSSTAR as libc::c_int as libc::c_uint
                                                || (*mo2_0).type_0 as libc::c_uint
                                                    == MT_EMBLEM as libc::c_int as libc::c_uint
                                                    && (*mo2_0).reactiontime & 1 as libc::c_int != 0
                                                    && P_CanPickupEmblem(
                                                        player,
                                                        (*mo2_0).health - 1 as libc::c_int,
                                                    ) != 0
                                                    && P_EmblemWasCollected((*mo2_0).health - 1 as libc::c_int)
                                                        == 0
                                            {
                                                (*mo2_0).flags
                                                    |= (MF_NOCLIP as libc::c_int
                                                        | MF_NOCLIPHEIGHT as libc::c_int) as uint32_t;
                                                (*mo2_0).flags2
                                                    |= MF2_NIGHTSPULL as libc::c_int as uint32_t;
                                                (*mo2_0).movefactor = 0 as libc::c_int;
                                                P_SetTarget2(&mut (*mo2_0).tracer, toucher);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        th_0 = (*th_0).next;
                    }
                }
                return;
            }
            590 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                    == CR_NIGHTSMODE as libc::c_int && ((*toucher).target).is_null()
                {
                    return;
                }
                if !((*toucher).tracer).is_null()
                    && (*(*toucher).tracer).health > 0 as libc::c_int
                {
                    return;
                }
                if (*player).mare as libc::c_int != (*special).threshold {
                    return;
                }
                if (*special).reactiontime > 0 as libc::c_int {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) != 0 && (*player).exiting == 0 {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && &mut *players.as_mut_ptr().offset(i as isize)
                                as *mut player_t != player
                            && players[i as usize].spheres as libc::c_int
                                > 0 as libc::c_int
                        {
                            (*player)
                                .spheres = ((*player).spheres as libc::c_int
                                + players[i as usize].spheres as libc::c_int) as int16_t;
                            players[i as usize].spheres = 0 as libc::c_int as int16_t;
                        }
                        i += 1;
                        i;
                    }
                }
                if (*player).spheres as libc::c_int <= 0 as libc::c_int
                    || (*player).exiting != 0
                {
                    return;
                }
                P_SetTarget2(&mut (*player).capsule, special);
                (*special)
                    .reactiontime = (player.offset_from(players.as_mut_ptr())
                    as libc::c_long + 1 as libc::c_int as libc::c_long) as int32_t;
                P_SetTarget2(&mut (*special).target, 0 as *mut mobj_t);
                (*player).texttimer = 0 as libc::c_int as uint8_t;
                return;
            }
            576 => {
                if (*player).exiting != 0 {
                    return;
                }
                if (*player).bumpertime
                    <= (35 as libc::c_int / 2 as libc::c_int - 5 as libc::c_int) as tic_t
                {
                    S_StartSound(
                        toucher as *const libc::c_void,
                        (*(*special).info).seesound,
                    );
                    if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                        == CR_NIGHTSMODE as libc::c_int
                    {
                        (*player)
                            .bumpertime = (35 as libc::c_int / 2 as libc::c_int)
                            as tic_t;
                        if (*special).threshold > 0 as libc::c_int {
                            (*player)
                                .flyangle = (*special).threshold * 30 as libc::c_int
                                - 1 as libc::c_int;
                        } else {
                            (*player).flyangle = (*special).threshold;
                        }
                        (*player)
                            .speed = FixedMul(
                            (*(*special).info).speed,
                            (*special).scale,
                        );
                        P_SetTarget2(&mut (*(*player).mo).hnext, special);
                    } else {
                        let mut fa: angle_t = 0;
                        let mut xspeed: fixed_t = 0;
                        let mut yspeed: fixed_t = 0;
                        let speed: fixed_t = FixedMul(
                            FixedDiv(
                                (*(*special).info).speed
                                    * ((1 as libc::c_int) << 16 as libc::c_int),
                                75 as libc::c_int
                                    * ((1 as libc::c_int) << 16 as libc::c_int),
                            ),
                            FixedSqrt(FixedMul((*toucher).scale, (*special).scale)),
                        );
                        (*player)
                            .bumpertime = (35 as libc::c_int / 2 as libc::c_int)
                            as tic_t;
                        P_UnsetThingPosition(toucher);
                        (*toucher).x = (*special).x;
                        (*toucher).y = (*special).y;
                        P_SetThingPosition(toucher);
                        (*toucher)
                            .z = (*special).z + (*special).height / 4 as libc::c_int;
                        if (*special).threshold > 0 as libc::c_int {
                            fa = FixedAngle(
                                ((*special).threshold * 30 as libc::c_int
                                    - 1 as libc::c_int)
                                    * ((1 as libc::c_int) << 16 as libc::c_int),
                            ) >> 19 as libc::c_int
                                & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
                        } else {
                            fa = 0 as libc::c_int as angle_t;
                        }
                        xspeed = FixedMul(
                            *finecosine.offset(fa as isize)
                                >> 16 as libc::c_int - 16 as libc::c_int,
                            speed,
                        );
                        yspeed = FixedMul(
                            finesine[fa as usize]
                                >> 16 as libc::c_int - 16 as libc::c_int,
                            speed,
                        );
                        P_InstaThrust(
                            toucher,
                            (*special).angle,
                            xspeed / 10 as libc::c_int,
                        );
                        (*toucher).momz = yspeed / 11 as libc::c_int;
                        (*toucher).angle = (*special).angle;
                        P_SetPlayerAngle(player, (*toucher).angle);
                        P_ResetPlayer(player);
                        P_SetPlayerMobjState(toucher, S_PLAY_FALL);
                    }
                }
                return;
            }
            585 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    || !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) == 0 {
                    (*player)
                        .powers[pw_nights_superloop as libc::c_int
                        as usize] = (*(*special).info).speed as uint16_t;
                } else {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize]
                                .powers[pw_nights_superloop as libc::c_int
                                as usize] = (*(*special).info).speed as uint16_t;
                        }
                        i += 1;
                        i;
                    }
                    if (*(*special).info).deathsound as libc::c_uint
                        != sfx_None as libc::c_int as libc::c_uint
                    {
                        S_StartSound(
                            0 as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                }
                if player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t || G_IsSpecialStage(gamemap as int32_t) != 0
                {
                    HU_SetCEchoFlags(0x10000000 as libc::c_int);
                    HU_SetCEchoDuration(4 as libc::c_int);
                    HU_DoCEcho(
                        b"\\\\\\\\\\\\\\\\Super Paraloop\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_856 = 14894058611842117342;
            }
            586 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    || !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) == 0 {
                    (*player).drillmeter = (*(*special).info).speed;
                } else {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize].drillmeter = (*(*special).info).speed;
                        }
                        i += 1;
                        i;
                    }
                    if (*(*special).info).deathsound as libc::c_uint
                        != sfx_None as libc::c_int as libc::c_uint
                    {
                        S_StartSound(
                            0 as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                }
                if player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t || G_IsSpecialStage(gamemap as int32_t) != 0
                {
                    HU_SetCEchoFlags(0x10000000 as libc::c_int);
                    HU_SetCEchoDuration(4 as libc::c_int);
                    HU_DoCEcho(
                        b"\\\\\\\\\\\\\\\\Drill Refill\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_856 = 14894058611842117342;
            }
            587 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    || !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) == 0 {
                    let mut flickyobj: *mut mobj_t = P_SpawnMobj(
                        (*toucher).x,
                        (*toucher).y,
                        (*toucher).z + (*(*toucher).info).height,
                        MT_NIGHTOPIANHELPER,
                    );
                    P_SetTarget2(&mut (*flickyobj).target, toucher);
                    (*player)
                        .powers[pw_nights_helper as libc::c_int
                        as usize] = (*(*special).info).speed as uint16_t;
                } else {
                    let mut flickyobj_0: *mut mobj_t = 0 as *mut mobj_t;
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && !(players[i as usize].mo).is_null()
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize]
                                .powers[pw_nights_helper as libc::c_int
                                as usize] = (*(*special).info).speed as uint16_t;
                            flickyobj_0 = P_SpawnMobj(
                                (*players[i as usize].mo).x,
                                (*players[i as usize].mo).y,
                                (*players[i as usize].mo).z
                                    + (*(*players[i as usize].mo).info).height,
                                MT_NIGHTOPIANHELPER,
                            );
                            P_SetTarget2(
                                &mut (*flickyobj_0).target,
                                players[i as usize].mo,
                            );
                        }
                        i += 1;
                        i;
                    }
                    if (*(*special).info).deathsound as libc::c_uint
                        != sfx_None as libc::c_int as libc::c_uint
                    {
                        S_StartSound(
                            0 as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                }
                if player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t || G_IsSpecialStage(gamemap as int32_t) != 0
                {
                    HU_SetCEchoFlags(0x10000000 as libc::c_int);
                    HU_SetCEchoDuration(4 as libc::c_int);
                    HU_DoCEcho(
                        b"\\\\\\\\\\\\\\\\Nightopian Helper\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_856 = 14894058611842117342;
            }
            588 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    || !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) == 0 {
                    (*player)
                        .nightstime = ((*player).nightstime)
                        .wrapping_add((*(*special).info).speed as tic_t);
                    (*player)
                        .startedtime = ((*player).startedtime)
                        .wrapping_add((*(*special).info).speed as tic_t);
                    (*player)
                        .lapstartedtime = ((*player).lapstartedtime)
                        .wrapping_add((*(*special).info).speed as tic_t);
                    P_RestoreMusic(player);
                } else {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize]
                                .nightstime = (players[i as usize].nightstime)
                                .wrapping_add((*(*special).info).speed as tic_t);
                            players[i as usize]
                                .startedtime = (players[i as usize].startedtime)
                                .wrapping_add((*(*special).info).speed as tic_t);
                            players[i as usize]
                                .lapstartedtime = (players[i as usize].lapstartedtime)
                                .wrapping_add((*(*special).info).speed as tic_t);
                            P_RestoreMusic(
                                &mut *players.as_mut_ptr().offset(i as isize),
                            );
                        }
                        i += 1;
                        i;
                    }
                    if (*(*special).info).deathsound as libc::c_uint
                        != sfx_None as libc::c_int as libc::c_uint
                    {
                        S_StartSound(
                            0 as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                }
                if player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t || G_IsSpecialStage(gamemap as int32_t) != 0
                {
                    HU_SetCEchoFlags(0x10000000 as libc::c_int);
                    HU_SetCEchoDuration(4 as libc::c_int);
                    HU_DoCEcho(
                        b"\\\\\\\\\\\\\\\\Extra Time\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_856 = 14894058611842117342;
            }
            589 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    || !((*player).powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int)
                {
                    return;
                }
                if G_IsSpecialStage(gamemap as int32_t) == 0 {
                    (*player)
                        .powers[pw_nights_linkfreeze as libc::c_int
                        as usize] = (*(*special).info).speed as uint16_t;
                    (*player).linktimer = nightslinktics as tic_t;
                } else {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize]
                                .powers[pw_nights_linkfreeze as libc::c_int
                                as usize] = (players[i as usize]
                                .powers[pw_nights_linkfreeze as libc::c_int as usize]
                                as libc::c_int
                                + (*(*special).info).speed as uint16_t as libc::c_int)
                                as uint16_t;
                            players[i as usize].linktimer = nightslinktics as tic_t;
                        }
                        i += 1;
                        i;
                    }
                    if (*(*special).info).deathsound as libc::c_uint
                        != sfx_None as libc::c_int as libc::c_uint
                    {
                        S_StartSound(
                            0 as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                }
                if player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t || G_IsSpecialStage(gamemap as int32_t) != 0
                {
                    HU_SetCEchoFlags(0x10000000 as libc::c_int);
                    HU_SetCEchoDuration(4 as libc::c_int);
                    HU_DoCEcho(
                        b"\\\\\\\\\\\\\\\\Link Freeze\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block_856 = 14894058611842117342;
            }
            578 => {
                while !((*special).hprev).is_null() {
                    special = (*special).hprev;
                }
                i = 0 as libc::c_int;
                while (*special).type_0 as libc::c_uint
                    == MT_HOOP as libc::c_int as libc::c_uint
                {
                    (*special).fuse = 11 as libc::c_int;
                    (*special).movedir = i as angle_t;
                    (*special).extravalue1 = (*(*special).target).extravalue1;
                    (*special).extravalue2 = (*(*special).target).extravalue2;
                    (*(*special).target).threshold = 4242 as libc::c_int;
                    i += 1;
                    i;
                    special = (*special).hnext;
                }
                let mut hnext_1: *mut mobj_t = 0 as *mut mobj_t;
                while !special.is_null() {
                    hnext_1 = (*special).hnext;
                    P_RemoveMobj(special);
                    special = hnext_1;
                }
                P_DoNightsScore(player);
                if G_IsSpecialStage(gamemap as int32_t) != 0 {
                    i = 0 as libc::c_int;
                    while i < 32 as libc::c_int {
                        if playeringame[i as usize] != 0
                            && players[i as usize]
                                .powers[pw_carry as libc::c_int as usize] as libc::c_int
                                == CR_NIGHTSMODE as libc::c_int
                        {
                            players[i as usize].drillmeter
                                += 35 as libc::c_int / 2 as libc::c_int;
                        }
                        i += 1;
                        i;
                    }
                } else if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    players[consoleplayer as usize].drillmeter
                        += 35 as libc::c_int / 2 as libc::c_int;
                } else {
                    (*player).drillmeter += 35 as libc::c_int / 2 as libc::c_int;
                }
                if (*player).linkcount <= 5 as libc::c_int {
                    S_StartSound(toucher as *const libc::c_void, sfx_hoop1);
                } else if (*player).linkcount <= 10 as libc::c_int {
                    S_StartSound(toucher as *const libc::c_void, sfx_hoop2);
                } else {
                    S_StartSound(toucher as *const libc::c_void, sfx_hoop3);
                }
                return;
            }
            557 => {
                let mut bounceon: boolean = (P_MobjFlip(toucher) as libc::c_int
                    * ((*toucher).z
                        - ((*special).z + (*special).height / 2 as libc::c_int))
                    > 0 as libc::c_int
                    && P_MobjFlip(toucher) as libc::c_int * (*toucher).momz
                        < 0 as libc::c_int) as libc::c_int;
                if (*special).threshold == 35 as libc::c_int {
                    if bounceon != 0 {
                        (*special).momy = 0 as libc::c_int;
                        (*special).momx = (*special).momy;
                        S_StartSound(toucher as *const libc::c_void, sfx_mario2);
                        P_SetTarget2(&mut (*special).target, 0 as *mut mobj_t);
                        (*special).threshold = 35 as libc::c_int - 1 as libc::c_int;
                        (*toucher).momz = -(*toucher).momz;
                    } else {
                        P_DamageMobj(
                            toucher,
                            special,
                            (*special).target,
                            1 as libc::c_int,
                            0 as libc::c_int as uint8_t,
                        );
                    }
                } else if (*special).threshold == 0 as libc::c_int {
                    (*special)
                        .movedir = (if (*special).movedir == 1 as libc::c_int as angle_t
                    {
                        -(1 as libc::c_int)
                    } else {
                        1 as libc::c_int
                    }) as angle_t;
                    P_InstaThrust(
                        special,
                        (*toucher).angle,
                        (*(*special).info).speed * (*special).scale,
                    );
                    S_StartSound(toucher as *const libc::c_void, sfx_mario2);
                    P_SetTarget2(&mut (*special).target, toucher);
                    (*special)
                        .threshold = 3 as libc::c_int * 35 as libc::c_int
                        / 2 as libc::c_int;
                    if bounceon != 0 {
                        (*toucher).momz = -(*toucher).momz;
                    }
                }
                return;
            }
            563 => {
                let mut th_1: *mut thinker_t = 0 as *mut thinker_t;
                let mut mo2_1: *mut mobj_t = 0 as *mut mobj_t;
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                if !((*special).spawnpoint).is_null() {
                    EV_DoElevator(
                        (*(*special).spawnpoint).args[0 as libc::c_int as usize]
                            as mtag_t,
                        0 as *mut line_t,
                        bridgeFall,
                    );
                }
                th_1 = (*thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize))
                    .next;
                while th_1
                    != &mut *thlist
                        .as_mut_ptr()
                        .offset(THINK_MOBJ as libc::c_int as isize) as *mut thinker_t
                {
                    if !((*th_1).function.acp1
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
                        mo2_1 = th_1 as *mut mobj_t;
                        if !((*mo2_1).type_0 as libc::c_uint
                            != MT_KOOPA as libc::c_int as libc::c_uint)
                        {
                            (*mo2_1)
                                .momz = 5 as libc::c_int
                                * ((1 as libc::c_int) << 16 as libc::c_int);
                            break;
                        }
                    }
                    th_1 = (*th_1).next;
                }
                current_block_856 = 14894058611842117342;
            }
            223 => {
                if (*special).health != 0 && (*player).bot == 0 {
                    F_StartTextPrompt(
                        199 as libc::c_int,
                        0 as libc::c_int,
                        toucher,
                        0 as libc::c_int as uint16_t,
                        true_0 as libc::c_int,
                        false_0 as libc::c_int,
                    );
                    (*special).health = 0 as libc::c_int;
                    if ultimatemode as libc::c_int != 0
                        && ((*player).continues as libc::c_int) < 99 as libc::c_int
                    {
                        (*player).continues += 1;
                        (*player).continues;
                    }
                }
                return;
            }
            554 => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                S_StartSound(toucher as *const libc::c_void, sfx_mario3);
                (*player)
                    .powers[pw_shield as libc::c_int
                    as usize] = ((*player).powers[pw_shield as libc::c_int as usize]
                    as libc::c_int & SH_NOSTACK as libc::c_int
                    | SH_FIREFLOWER as libc::c_int) as uint16_t;
                if !((*player).powers[pw_super as libc::c_int as usize] as libc::c_int
                    != 0
                    || maptol & TOL_MARIO as libc::c_int as uint32_t != 0
                        && (*player).powers[pw_invulnerability as libc::c_int as usize]
                            as libc::c_int != 0)
                {
                    (*(*player).mo).color = SKINCOLOR_WHITE as libc::c_int as uint16_t;
                    G_GhostAddColor(GHC_FIREFLOWER);
                }
                current_block_856 = 14894058611842117342;
            }
            152 => {
                P_TouchStarPost(
                    special,
                    player,
                    (!((*special).spawnpoint).is_null()
                        && (*(*special).spawnpoint).args[1 as libc::c_int as usize] != 0)
                        as libc::c_int,
                );
                return;
            }
            71 => {
                let mut touchx: fixed_t = 0;
                let mut touchy: fixed_t = 0;
                let mut touchspeed: fixed_t = 0;
                let mut angle: angle_t = 0;
                if P_AproxDistance(
                    (*toucher).x - (*special).x,
                    (*toucher).y - (*special).y,
                )
                    > P_AproxDistance(
                        (*toucher).x - (*toucher).momx - (*special).x,
                        (*toucher).y - (*toucher).momy - (*special).y,
                    )
                {
                    touchx = (*toucher).x + (*toucher).momx;
                    touchy = (*toucher).y + (*toucher).momy;
                } else {
                    touchx = (*toucher).x;
                    touchy = (*toucher).y;
                }
                angle = R_PointToAngle2((*special).x, (*special).y, touchx, touchy);
                touchspeed = P_AproxDistance((*toucher).momx, (*toucher).momy);
                (*toucher).momx = P_ReturnThrustX(special, angle, touchspeed);
                (*toucher).momy = P_ReturnThrustY(special, angle, touchspeed);
                (*toucher).momz = -(*toucher).momz;
                if (*player).pflags as libc::c_uint
                    & PF_GLIDING as libc::c_int as libc::c_uint != 0
                    && P_IsObjectOnGround(toucher) == 0
                {
                    (*player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*player).pflags as libc::c_uint
                            & !(PF_GLIDING as libc::c_int | PF_JUMPED as libc::c_int
                                | PF_NOJUMPDAMAGE as libc::c_int) as libc::c_uint,
                    );
                    P_SetPlayerMobjState(toucher, S_PLAY_FALL);
                    (*toucher).momz
                        += P_MobjFlip(toucher) as libc::c_int
                            * ((*player).speed >> 3 as libc::c_int);
                    (*toucher)
                        .momx = 7 as libc::c_int * (*toucher).momx >> 3 as libc::c_int;
                    (*toucher)
                        .momy = 7 as libc::c_int * (*toucher).momy >> 3 as libc::c_int;
                }
                (*player).homing = 0 as libc::c_int as uint8_t;
                S_StartSound(
                    toucher as *const libc::c_void,
                    (*(*special).info).painsound,
                );
                return;
            }
            90 => {
                if (*player).powers[pw_flashing as libc::c_int as usize] == 0
                    && (*player).powers[pw_ignorelatch as libc::c_int as usize]
                        as libc::c_int & (1 as libc::c_int) << 15 as libc::c_int == 0
                {
                    (*toucher).momx = 0 as libc::c_int;
                    (*toucher).momy = 0 as libc::c_int;
                    if (*toucher).momz != 0 as libc::c_int {
                        (*special).momz = (*toucher).momz;
                    }
                    (*player)
                        .powers[pw_carry as libc::c_int
                        as usize] = CR_BRAKGOOP as libc::c_int as uint16_t;
                    P_SetTarget2(&mut (*toucher).tracer, special);
                    P_ResetPlayer(player);
                    if !((*special).target).is_null()
                        && (*(*special).target).state
                            == &mut *states
                                .as_mut_ptr()
                                .offset(S_BLACKEGG_SHOOT1 as libc::c_int as isize)
                                as *mut state_t
                    {
                        if (*(*special).target).health <= 2 as libc::c_int
                            && P_RandomFixed()
                                < ((1 as libc::c_int) << 16 as libc::c_int)
                                    / 2 as libc::c_int
                        {
                            P_SetMobjState(
                                (*special).target,
                                (*(*(*special).target).info).missilestate,
                            );
                        } else {
                            P_SetMobjState(
                                (*special).target,
                                (*(*(*special).target).info).raisestate,
                            );
                        }
                    }
                }
                return;
            }
            33 => {
                let mut angle_0: angle_t = (R_PointToAngle2(
                    (*special).x,
                    (*special).y,
                    (*toucher).x,
                    (*toucher).y,
                ))
                    .wrapping_sub((*special).angle);
                let mut touchspeed_0: fixed_t = P_AproxDistance(
                    (*toucher).momx,
                    (*toucher).momy,
                );
                if touchspeed_0 < (*special).scale {
                    touchspeed_0 = (*special).scale;
                }
                if !(angle_0 > 0x40000000 as libc::c_int as angle_t
                    && angle_0 < 0xc0000000 as libc::c_uint)
                {
                    (*toucher)
                        .momx = P_ReturnThrustX(special, (*special).angle, touchspeed_0);
                    (*toucher)
                        .momy = P_ReturnThrustY(special, (*special).angle, touchspeed_0);
                    (*toucher).momz = -(*toucher).momz;
                    if (*player).pflags as libc::c_uint
                        & PF_GLIDING as libc::c_int as libc::c_uint != 0
                        && P_IsObjectOnGround(toucher) == 0
                    {
                        (*player)
                            .pflags = ::core::mem::transmute::<
                            libc::c_uint,
                            pflags_t,
                        >(
                            (*player).pflags as libc::c_uint
                                & !(PF_GLIDING as libc::c_int | PF_JUMPED as libc::c_int
                                    | PF_NOJUMPDAMAGE as libc::c_int) as libc::c_uint,
                        );
                        P_SetPlayerMobjState(toucher, S_PLAY_FALL);
                        (*toucher).momz
                            += P_MobjFlip(toucher) as libc::c_int
                                * ((*player).speed >> 3 as libc::c_int);
                        (*toucher)
                            .momx = 7 as libc::c_int * (*toucher).momx
                            >> 3 as libc::c_int;
                        (*toucher)
                            .momy = 7 as libc::c_int * (*toucher).momy
                            >> 3 as libc::c_int;
                    }
                    (*player).homing = 0 as libc::c_int as uint8_t;
                    S_StartSound(
                        toucher as *const libc::c_void,
                        (*(*special).info).painsound,
                    );
                    if !((*special).target).is_null() {
                        (*(*special).target)
                            .extravalue1 = -(*(*(*special).target).info).speed;
                    }
                    return;
                } else {
                    (*toucher).momx = -(*toucher).momx / 2 as libc::c_int;
                    (*toucher).momy = -(*toucher).momy / 2 as libc::c_int;
                    (*toucher).momz = -(*toucher).momz;
                }
                current_block_856 = 14894058611842117342;
            }
            76 => {
                if (*special).state
                    == &mut *states
                        .as_mut_ptr()
                        .offset((*(*special).info).deathstate as isize) as *mut state_t
                {
                    return;
                }
                if P_PlayerInPain(player) != 0 {
                    return;
                }
                P_SetMobjState(special, (*(*special).info).meleestate);
                (*special).angle = (*special).movedir;
                (*special).momy = 0 as libc::c_int;
                (*special).momx = (*special).momy;
                P_SetPlayerMobjState(toucher, S_PLAY_STUN);
                (*player)
                    .pflags = ::core::mem::transmute::<
                    libc::c_uint,
                    pflags_t,
                >(
                    (*player).pflags as libc::c_uint
                        & !(PF_APPLYAUTOBRAKE as libc::c_int) as libc::c_uint,
                );
                P_ResetPlayer(player);
                (*player)
                    .drawangle = ((*special).angle)
                    .wrapping_add(0x80000000 as libc::c_uint);
                P_InstaThrust(
                    toucher,
                    (*special).angle,
                    FixedMul(
                        3 as libc::c_int * (*(*special).info).speed,
                        (*special).scale / 2 as libc::c_int,
                    ),
                );
                (*toucher).z += P_MobjFlip(toucher) as libc::c_int;
                if (*toucher).eflags as libc::c_int & MFE_UNDERWATER as libc::c_int != 0
                {
                    P_SetObjectMomZ(
                        toucher,
                        FixedDiv(
                            10511 as libc::c_int
                                * ((1 as libc::c_int) << 16 as libc::c_int),
                            2600 as libc::c_int
                                * ((1 as libc::c_int) << 16 as libc::c_int),
                        ),
                        false_0 as libc::c_int,
                    );
                } else {
                    P_SetObjectMomZ(
                        toucher,
                        FixedDiv(
                            69 as libc::c_int
                                * ((1 as libc::c_int) << 16 as libc::c_int),
                            10 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        ),
                        false_0 as libc::c_int,
                    );
                }
                if P_IsLocalPlayer(player) != 0 {
                    quake
                        .intensity = 9 as libc::c_int
                        * ((1 as libc::c_int) << 16 as libc::c_int);
                    quake.time = (35 as libc::c_int / 2 as libc::c_int) as uint16_t;
                    quake.epicenter = 0 as *mut mappoint_t;
                }
                S_StartSound(
                    toucher as *const libc::c_void,
                    (*(*special).info).attacksound,
                );
                return;
            }
            309 | 310 => {
                if (*toucher).momx != 0 || (*toucher).momy != 0 {
                    (*special).momx = (*toucher).momx;
                    (*special).momy = (*toucher).momy;
                    (*special)
                        .momz = P_AproxDistance((*toucher).momx, (*toucher).momy)
                        / 4 as libc::c_int;
                    if (*toucher).momz > 0 as libc::c_int {
                        (*special).momz += (*toucher).momz / 8 as libc::c_int;
                    }
                    P_SetMobjState(special, (*(*special).info).seestate);
                }
                return;
            }
            283 | 284 => {
                let mut macespin: boolean = false_0 as libc::c_int;
                if P_MobjFlip(toucher) as libc::c_int * (*toucher).momz
                    > 0 as libc::c_int
                    || (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                        != 0
                {
                    return;
                }
                if (*toucher).z > (*special).z + (*special).height / 2 as libc::c_int {
                    return;
                }
                if ((*toucher).z + (*toucher).height / 2 as libc::c_int) < (*special).z {
                    return;
                }
                if (*player).powers[pw_flashing as libc::c_int as usize] != 0 {
                    return;
                }
                if !((*special).tracer).is_null()
                    && (*(*special).tracer).flags2
                        & MF2_STRONGBOX as libc::c_int as uint32_t == 0
                {
                    macespin = true_0 as libc::c_int;
                }
                if if macespin != 0 {
                    (*player).powers[pw_ignorelatch as libc::c_int as usize]
                        as libc::c_int & (1 as libc::c_int) << 15 as libc::c_int
                } else {
                    (*player).powers[pw_ignorelatch as libc::c_int as usize]
                        as libc::c_int
                } != 0
                {
                    return;
                }
                if (*special).movefactor != 0 && !((*special).tracer).is_null()
                    && (*(*special).tracer).angle != 0x40000000 as libc::c_int as angle_t
                    && (*(*special).tracer).angle != 0xc0000000 as libc::c_uint
                {
                    let mut ang: angle_t = (R_PointToAngle2(
                        (*special).x,
                        (*special).y,
                        (*toucher).x,
                        (*toucher).y,
                    ))
                        .wrapping_sub((*(*special).tracer).angle);
                    if ((*special).movefactor > 0 as libc::c_int) as libc::c_int
                        == ((*(*special).tracer).angle
                            > 0x40000000 as libc::c_int as angle_t
                            && (*(*special).tracer).angle < 0xc0000000 as libc::c_uint)
                            as libc::c_int
                    {
                        ang = (ang as libc::c_uint)
                            .wrapping_add(0x80000000 as libc::c_uint) as angle_t
                            as angle_t;
                    }
                    if ang < 0x80000000 as libc::c_uint {
                        return;
                    }
                }
                P_ResetPlayer(player);
                P_SetTarget2(&mut (*toucher).tracer, special);
                if macespin != 0 {
                    (*player)
                        .powers[pw_carry as libc::c_int
                        as usize] = CR_MACESPIN as libc::c_int as uint16_t;
                    S_StartSound(toucher as *const libc::c_void, sfx_spin);
                    P_SetPlayerMobjState(toucher, S_PLAY_ROLL);
                } else {
                    (*player)
                        .powers[pw_carry as libc::c_int
                        as usize] = CR_GENERIC as libc::c_int as uint16_t;
                }
                (*player)
                    .pflags = ::core::mem::transmute::<
                    libc::c_uint,
                    pflags_t,
                >(
                    (*player).pflags as libc::c_uint
                        | PF_JUMPSTASIS as libc::c_int as libc::c_uint,
                );
                return;
            }
            67 => {
                if ((*special).target).is_null() || (*(*special).target).health == 0 {
                    return;
                }
                if (*(*special).target).momz < 0 as libc::c_int {
                    P_DamageMobj(
                        toucher,
                        special,
                        (*special).target,
                        1 as libc::c_int,
                        0 as libc::c_int as uint8_t,
                    );
                    (*(*special).target).momy = 0 as libc::c_int;
                    (*(*special).target).momx = (*(*special).target).momy;
                    (*(*special).target).momz = 0 as libc::c_int;
                    (*(*special).target).flags
                        |= MF_NOGRAVITY as libc::c_int as uint32_t;
                    P_SetMobjState((*special).target, (*(*special).info).raisestate);
                    S_StartSound(
                        (*special).target as *const libc::c_void,
                        (*(*special).info).activesound,
                    );
                    P_RemoveMobj(special);
                }
                return;
            }
            513 => {
                if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                    & SH_PROTECTWATER as libc::c_int != 0
                {
                    return;
                }
                if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
                    return;
                }
                if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                    return;
                }
                if ((*special).state).offset_from(states.as_mut_ptr()) as libc::c_long
                    != S_EXTRALARGEBUBBLE as libc::c_int as libc::c_long
                {
                    return
                } else if (*toucher).eflags as libc::c_int
                    & MFE_VERTICALFLIP as libc::c_int != 0
                {
                    if (*special).z + (*special).height < (*toucher).z
                        || (*special).z + (*special).height
                            > (*toucher).z
                                + (*toucher).height * 2 as libc::c_int / 3 as libc::c_int
                    {
                        return;
                    }
                } else if (*special).z < (*toucher).z
                    || (*special).z
                        > (*toucher).z
                            + (*toucher).height * 2 as libc::c_int / 3 as libc::c_int
                {
                    return
                }
                if ((*player).bot == 0
                    || (*player).bot as libc::c_int == BOT_MPAI as libc::c_int)
                    && ((*player).powers[pw_underwater as libc::c_int as usize]
                        as libc::c_int != 0
                        && (*player).powers[pw_underwater as libc::c_int as usize]
                            as libc::c_int
                            <= 12 as libc::c_int * 35 as libc::c_int + 1 as libc::c_int)
                {
                    (*player)
                        .powers[pw_underwater as libc::c_int
                        as usize] = (underwatertics as libc::c_int + 1 as libc::c_int)
                        as uint16_t;
                    P_RestoreMusic(player);
                }
                if ((*player).powers[pw_underwater as libc::c_int as usize]
                    as libc::c_int) < underwatertics as libc::c_int + 1 as libc::c_int
                {
                    (*player)
                        .powers[pw_underwater as libc::c_int
                        as usize] = (underwatertics as libc::c_int + 1 as libc::c_int)
                        as uint16_t;
                }
                if (*player).climbing == 0 {
                    if (*player).bot as libc::c_int != 0
                        && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                        && ((*toucher).state).offset_from(states.as_mut_ptr())
                            as libc::c_long != S_PLAY_GASP as libc::c_int as libc::c_long
                    {
                        S_StartSound(
                            toucher as *const libc::c_void,
                            (*(*special).info).deathsound,
                        );
                    }
                    P_SetPlayerMobjState(toucher, S_PLAY_GASP);
                    P_ResetPlayer(player);
                }
                (*toucher).momz = 0 as libc::c_int;
                (*toucher).momy = (*toucher).momz;
                (*toucher).momx = (*toucher).momy;
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return
                } else {
                    current_block_856 = 14894058611842117342;
                }
            }
            255 => {
                if (*special).state
                    == &mut *states
                        .as_mut_ptr()
                        .offset((*(*special).info).spawnstate as isize) as *mut state_t
                {
                    (*special)
                        .z = (*toucher).z + (*toucher).height
                        - FixedMul(
                            8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                            (*special).scale,
                        );
                    (*special).momz = 0 as libc::c_int;
                    (*special).flags |= MF_NOGRAVITY as libc::c_int as uint32_t;
                    P_SetMobjState(special, (*(*special).info).deathstate);
                    S_StartSound(
                        special as *const libc::c_void,
                        ((*(*special).info).deathsound as libc::c_uint)
                            .wrapping_add(
                                P_RandomKey((*(*special).info).mass) as libc::c_uint,
                            ) as sfxenum_t,
                    );
                }
                return;
            }
            44 => {
                (*special).flags
                    |= (MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                        as uint32_t;
                P_UnsetThingPosition(special);
                (*special).x = (*toucher).x - (*toucher).momx / 2 as libc::c_int;
                (*special).y = (*toucher).y - (*toucher).momy / 2 as libc::c_int;
                (*special).z = (*toucher).z - (*toucher).momz / 2 as libc::c_int;
                P_SetThingPosition(special);
                (*toucher)
                    .momx = FixedMul(
                    (*toucher).momx,
                    50 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 51 as libc::c_int,
                );
                (*toucher)
                    .momy = FixedMul(
                    (*toucher).momy,
                    50 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 51 as libc::c_int,
                );
                (*special).momx = (*toucher).momx;
                (*special).momy = (*toucher).momy;
                (*special).momz = (*toucher).momz;
                return;
            }
            335 => {
                if (*player).bot == 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                    && (*special).fuse <= 35 as libc::c_int
                    && (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                        != CR_MINECART as libc::c_int
                    && (*player).powers[pw_ignorelatch as libc::c_int as usize]
                        as libc::c_int & (1 as libc::c_int) << 15 as libc::c_int == 0
                {
                    let mut mcart: *mut mobj_t = P_SpawnMobj(
                        (*special).x,
                        (*special).y,
                        (*special).z,
                        MT_MINECART,
                    );
                    P_SetTarget2(&mut (*mcart).target, toucher);
                    (*player).drawangle = (*special).angle;
                    (*toucher).angle = (*player).drawangle;
                    (*mcart).angle = (*toucher).angle;
                    (*mcart).friction = (1 as libc::c_int) << 16 as libc::c_int;
                    P_ResetPlayer(player);
                    (*player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*player).pflags as libc::c_uint
                            | PF_JUMPDOWN as libc::c_int as libc::c_uint,
                    );
                    (*player)
                        .powers[pw_carry as libc::c_int
                        as usize] = CR_MINECART as libc::c_int as uint16_t;
                    (*player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*player).pflags as libc::c_uint
                            & !(PF_APPLYAUTOBRAKE as libc::c_int) as libc::c_uint,
                    );
                    P_SetTarget2(&mut (*toucher).tracer, mcart);
                    (*toucher).momz = 0 as libc::c_int;
                    (*toucher).momy = (*toucher).momz;
                    (*toucher).momx = (*toucher).momy;
                    (*special).fuse = 3 as libc::c_int * 35 as libc::c_int;
                    (*special).flags2 |= MF2_DONTDRAW as libc::c_int as uint32_t;
                }
                return;
            }
            336 => {
                if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                    == CR_MINECART as libc::c_int && !((*toucher).tracer).is_null()
                    && P_MobjWasRemoved((*toucher).tracer) == 0
                    && (*(*toucher).tracer).health != 0
                {
                    let mut maxz: fixed_t = if (*toucher).z
                        > (*special).z + 35 as libc::c_int * (*special).scale
                    {
                        (*toucher).z
                    } else {
                        (*special).z + 35 as libc::c_int * (*special).scale
                    };
                    (*toucher).momx = (*(*toucher).tracer).momx / 2 as libc::c_int;
                    (*toucher).momy = (*(*toucher).tracer).momy / 2 as libc::c_int;
                    (*toucher)
                        .momz = (*(*toucher).tracer).momz
                        + P_AproxDistance(
                            (*(*toucher).tracer).momx,
                            (*(*toucher).tracer).momy,
                        ) / 2 as libc::c_int;
                    P_ResetPlayer(player);
                    (*player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*player).pflags as libc::c_uint
                            & !(PF_APPLYAUTOBRAKE as libc::c_int) as libc::c_uint,
                    );
                    P_SetPlayerMobjState(toucher, S_PLAY_FALL);
                    P_SetTarget2(&mut (*(*toucher).tracer).target, 0 as *mut mobj_t);
                    P_KillMobj(
                        (*toucher).tracer,
                        toucher,
                        special,
                        0 as libc::c_int as uint8_t,
                    );
                    P_SetTarget2(&mut (*toucher).tracer, 0 as *mut mobj_t);
                    (*player)
                        .powers[pw_carry as libc::c_int
                        as usize] = CR_NONE as libc::c_int as uint16_t;
                    P_UnsetThingPosition(toucher);
                    (*toucher).x = (*special).x;
                    (*toucher).y = (*special).y;
                    (*toucher).z = maxz;
                    P_SetThingPosition(toucher);
                }
                return;
            }
            346 => {
                if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                    == CR_MINECART as libc::c_int && !((*toucher).tracer).is_null()
                    && P_MobjWasRemoved((*toucher).tracer) == 0
                    && (*(*toucher).tracer).health != 0
                {
                    let mut destambush: mobjflag2_t = ((*special).flags2
                        & MF2_AMBUSH as libc::c_int as uint32_t) as mobjflag2_t;
                    let mut angdiff: angle_t = ((*(*toucher).tracer).angle)
                        .wrapping_sub((*special).angle);
                    if angdiff > 0x40000000 as libc::c_int as angle_t
                        && angdiff < 0xc0000000 as libc::c_uint
                    {
                        destambush = ::core::mem::transmute::<
                            libc::c_uint,
                            mobjflag2_t,
                        >(
                            destambush as libc::c_uint
                                ^ MF2_AMBUSH as libc::c_int as libc::c_uint,
                        );
                    }
                    (*(*toucher).tracer)
                        .flags2 = (*(*toucher).tracer).flags2
                        & !(MF2_AMBUSH as libc::c_int) as uint32_t
                        | destambush as libc::c_uint;
                }
                return;
            }
            _ => {
                if (*player).bot as libc::c_int != 0
                    && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                {
                    return;
                }
                P_SetTarget2(&mut (*special).target, toucher);
                current_block_856 = 14894058611842117342;
            }
        }
        match current_block_856 {
            16677036429817807717 => {
                if (*special).type_0 as libc::c_uint
                    == MT_BLUETEAMRING as libc::c_int as libc::c_uint
                    && (*player).ctfteam != 2 as libc::c_int
                {
                    return;
                }
                current_block_856 = 4166128649646877775;
            }
            _ => {}
        }
        match current_block_856 {
            4166128649646877775 => {
                if P_CanPickupItem(player, false_0 as libc::c_int) == 0
                    && (*special).flags2 & MF2_NIGHTSPULL as libc::c_int as uint32_t == 0
                {
                    return;
                }
                (*special).momz = 0 as libc::c_int;
                (*special).momy = (*special).momz;
                (*special).momx = (*special).momy;
                P_GivePlayerRings(player, 1 as libc::c_int);
                if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
                    && (*special).type_0 as libc::c_uint
                        != MT_FLINGRING as libc::c_int as libc::c_uint
                    && (*special).type_0 as libc::c_uint
                        != MT_FLINGCOIN as libc::c_int as libc::c_uint
                {
                    P_DoNightsScore(player);
                }
            }
            _ => {}
        }
    }
    S_StartSound(toucher as *const libc::c_void, (*(*special).info).deathsound);
    P_KillMobj(special, 0 as *mut mobj_t, toucher, 0 as libc::c_int as uint8_t);
    (*special).shadowscale = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_TouchStarPost(
    mut post: *mut mobj_t,
    mut player: *mut player_t,
    mut snaptopost: boolean,
) {
    let mut i: size_t = 0;
    let mut toucher: *mut mobj_t = (*player).mo;
    let mut checkbase: *mut mobj_t = if snaptopost != 0 { post } else { toucher };
    if (*player).bot as libc::c_int != 0
        && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
    {
        return;
    }
    if circuitmap != 0 && (*post).health - (*player).starpostnum > 1 as libc::c_int {
        if (*player).tossdelay == 0 {
            S_StartSound(toucher as *const libc::c_void, sfx_lose);
        }
        (*player).tossdelay = 3 as libc::c_int;
        return;
    }
    if (*post).health > 1365 as libc::c_int {
        CONS_Debug(
            0x80 as libc::c_int,
            b"Bad Starpost Number!\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*player).starpostnum >= (*post).health {
        return;
    }
    if cv_coopstarposts.value != 0 && G_GametypeUsesCoopStarposts() != 0
        && (netgame != 0 || multiplayer != 0)
    {
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as size_t {
            if playeringame[i as usize] != 0 {
                if !(players[i as usize].bot != 0) {
                    players[i as usize].starposttime = leveltime;
                    players[i as usize]
                        .starpostx = ((*checkbase).x >> 16 as libc::c_int) as int16_t;
                    players[i as usize]
                        .starposty = ((*checkbase).y >> 16 as libc::c_int) as int16_t;
                    players[i as usize]
                        .starpostz = ((*post).z >> 16 as libc::c_int) as int16_t;
                    players[i as usize].starpostangle = (*post).angle;
                    players[i as usize].starpostscale = (*(*player).mo).destscale;
                    if (*post).flags2 & MF2_OBJECTFLIP as libc::c_int as uint32_t != 0 {
                        players[i as usize].starpostscale *= -(1 as libc::c_int);
                        players[i as usize]
                            .starpostz = (players[i as usize].starpostz as libc::c_int
                            + ((*post).height >> 16 as libc::c_int)) as int16_t;
                    }
                    players[i as usize].starpostnum = (*post).health;
                    if cv_coopstarposts.value == 2 as libc::c_int
                        && (players[i as usize].playerstate as libc::c_uint
                            == PST_DEAD as libc::c_int as libc::c_uint
                            || players[i as usize].spectator != 0)
                        && P_GetLives(&mut *players.as_mut_ptr().offset(i as isize)) != 0
                    {
                        P_SpectatorJoinGame(
                            &mut *players.as_mut_ptr().offset(i as isize),
                        );
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        S_StartSound(0 as *const libc::c_void, (*(*post).info).painsound);
    } else {
        (*player).starposttime = leveltime;
        (*player).starpostx = ((*checkbase).x >> 16 as libc::c_int) as int16_t;
        (*player).starposty = ((*checkbase).y >> 16 as libc::c_int) as int16_t;
        (*player).starpostz = ((*post).z >> 16 as libc::c_int) as int16_t;
        (*player).starpostangle = (*post).angle;
        (*player).starpostscale = (*(*player).mo).destscale;
        if (*post).flags2 & MF2_OBJECTFLIP as libc::c_int as uint32_t != 0 {
            (*player).starpostscale *= -(1 as libc::c_int);
            (*player)
                .starpostz = ((*player).starpostz as libc::c_int
                + ((*post).height >> 16 as libc::c_int)) as int16_t;
        }
        (*player).starpostnum = (*post).health;
        S_StartSound(toucher as *const libc::c_void, (*(*post).info).painsound);
    }
    P_ClearStarPost((*post).health);
    if !(netgame != 0 && circuitmap != 0
        && player
            != &mut *players.as_mut_ptr().offset(consoleplayer as isize)
                as *mut player_t)
    {
        let mut th: *mut thinker_t = 0 as *mut thinker_t;
        let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
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
                if !((*mo2).type_0 as libc::c_uint
                    != MT_STARPOST as libc::c_int as libc::c_uint)
                {
                    if !((*mo2).health != (*post).health) {
                        P_SetMobjState(mo2, (*(*mo2).info).painstate);
                    }
                }
            }
            th = (*th).next;
        }
    }
}
unsafe extern "C" fn P_HitDeathMessages(
    mut player: *mut player_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damagetype: uint8_t,
) {
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut deathonly: boolean = false_0 as libc::c_int;
    let mut deadsource: boolean = false_0 as libc::c_int;
    let mut deadtarget: boolean = false_0 as libc::c_int;
    let mut targetname: [libc::c_char; 25] = [0; 25];
    let mut sourcename: [libc::c_char; 25] = [0; 25];
    if gametyperules
        & (GTR_RINGSLINGER as libc::c_int | GTR_HURTMESSAGES as libc::c_int) as uint32_t
        == 0
    {
        return;
    }
    if player.is_null() {
        return;
    }
    if ((*player).mo).is_null() {
        return;
    }
    if (*player).spectator != 0 {
        return;
    }
    if netgame == 0 {
        return;
    }
    if LUA_HookHurtMsg(player, inflictor, source, damagetype) != 0 {
        return;
    }
    deadtarget = ((*(*player).mo).health <= 0 as libc::c_int) as libc::c_int;
    if deadtarget == 0 && cv_hazardlog.value == 0 {
        return;
    }
    snprintf(
        targetname.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        if (*player).ctfteam != 0 {
            if (*player).ctfteam == 1 as libc::c_int {
                b"\x85\0" as *const u8 as *const libc::c_char
            } else {
                b"\x84\0" as *const u8 as *const libc::c_char
            }
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long as usize])
            .as_mut_ptr(),
        if (*player).ctfteam != 0 {
            b"\x80\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    if !source.is_null() {
        if !((*source).player).is_null() {
            snprintf(
                sourcename.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
                b"%s%s%s\0" as *const u8 as *const libc::c_char,
                if (*(*source).player).ctfteam != 0 {
                    if (*(*source).player).ctfteam == 1 as libc::c_int {
                        b"\x85\0" as *const u8 as *const libc::c_char
                    } else {
                        b"\x84\0" as *const u8 as *const libc::c_char
                    }
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (player_names[((*source).player).offset_from(players.as_mut_ptr())
                    as libc::c_long as usize])
                    .as_mut_ptr(),
                if (*(*source).player).ctfteam != 0 {
                    b"\x80\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if (*(*source).player).playerstate as libc::c_uint
                == PST_DEAD as libc::c_int as libc::c_uint && (*source).player != player
                && (*inflictor).flags2 & MF2_BEYONDTHEGRAVE as libc::c_int as uint32_t
                    != 0
            {
                deadsource = true_0 as libc::c_int;
            }
            if (*inflictor).flags & MF_PUSHABLE as libc::c_int as uint32_t != 0 {
                str = b"%s%s's playtime with heavy objects %s %s.\n\0" as *const u8
                    as *const libc::c_char;
            } else {
                match (*inflictor).type_0 as libc::c_uint {
                    3 => {
                        if damagetype as libc::c_int == 5 as libc::c_int {
                            str = b"%s%s's armageddon blast %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        } else if (*(*inflictor).player)
                            .powers[pw_shield as libc::c_int as usize] as libc::c_int
                            & SH_NOSTACK as libc::c_int == SH_ELEMENTAL as libc::c_int
                            && (*(*inflictor).player).pflags as libc::c_uint
                                & PF_SHIELDABILITY as libc::c_int as libc::c_uint != 0
                        {
                            str = b"%s%s's elemental stomp %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        } else if (*(*inflictor).player)
                            .powers[pw_invulnerability as libc::c_int as usize] != 0
                        {
                            str = b"%s%s's invincibility aura %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        } else if (*(*inflictor).player)
                            .powers[pw_super as libc::c_int as usize] != 0
                        {
                            str = b"%s%s's super aura %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        } else {
                            str = b"%s%s's tagging hand %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        }
                    }
                    148 => {
                        str = b"%s%s's elemental fire trail %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    544 => {
                        str = b"%s%s's bounce ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    545 => {
                        str = b"%s%s's infinity ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    546 => {
                        str = b"%s%s's automatic ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    547 => {
                        str = b"%s%s's scatter ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    548 => {
                        str = b"%s%s's explosion ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    549 => {
                        str = b"%s%s's grenade ring %s %s.\n\0" as *const u8
                            as *const libc::c_char;
                    }
                    530 => {
                        if (*inflictor).flags2 & MF2_RAILRING as libc::c_int as uint32_t
                            != 0
                        {
                            str = b"%s%s's rail ring %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        } else {
                            str = b"%s%s's thrown ring %s %s.\n\0" as *const u8
                                as *const libc::c_char;
                        }
                    }
                    _ => {
                        str = b"%s%s %s %s.\n\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
            CONS_Printf(
                str,
                if deadsource != 0 {
                    b"The late \0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                sourcename.as_mut_ptr(),
                if deadtarget != 0 {
                    b"killed\0" as *const u8 as *const libc::c_char
                } else {
                    b"hit\0" as *const u8 as *const libc::c_char
                },
                targetname.as_mut_ptr(),
            );
            return;
        } else {
            match (*source).type_0 as libc::c_uint {
                202 => {
                    str = b"%s was %s by Eggman's nefarious TV magic.\n\0" as *const u8
                        as *const libc::c_char;
                }
                149 | 150 => {
                    str = b"%s was %s by spikes.\n\0" as *const u8
                        as *const libc::c_char;
                }
                _ => {
                    str = b"%s was %s by an environmental hazard.\n\0" as *const u8
                        as *const libc::c_char;
                }
            }
        }
    } else {
        match damagetype as libc::c_int {
            1 => {
                str = b"%s was %s by dangerous water.\n\0" as *const u8
                    as *const libc::c_char;
            }
            2 => {
                str = b"%s was %s by molten lava.\n\0" as *const u8
                    as *const libc::c_char;
            }
            3 => {
                str = b"%s was %s by electricity.\n\0" as *const u8
                    as *const libc::c_char;
            }
            4 => {
                str = b"%s was %s by spikes.\n\0" as *const u8 as *const libc::c_char;
            }
            129 => {
                deathonly = true_0 as libc::c_int;
                str = b"%s drowned.\n\0" as *const u8 as *const libc::c_char;
            }
            132 => {
                deathonly = true_0 as libc::c_int;
                str = b"%s was crushed.\n\0" as *const u8 as *const libc::c_char;
            }
            131 => {
                if deadtarget != 0 {
                    deathonly = true_0 as libc::c_int;
                    str = b"%s fell into a bottomless pit.\n\0" as *const u8
                        as *const libc::c_char;
                }
            }
            130 => {
                if deadtarget != 0 {
                    deathonly = true_0 as libc::c_int;
                    str = b"%s asphyxiated in space.\n\0" as *const u8
                        as *const libc::c_char;
                }
            }
            _ => {
                if deadtarget != 0 {
                    deathonly = true_0 as libc::c_int;
                    str = b"%s died.\n\0" as *const u8 as *const libc::c_char;
                }
            }
        }
        if str.is_null() {
            str = b"%s was %s by an environmental hazard.\n\0" as *const u8
                as *const libc::c_char;
        }
    }
    if str.is_null() {
        return;
    }
    if deathonly != 0 {
        if deadtarget == 0 {
            return;
        }
        CONS_Printf(str, targetname.as_mut_ptr());
    } else {
        CONS_Printf(
            str,
            targetname.as_mut_ptr(),
            if deadtarget != 0 {
                b"killed\0" as *const u8 as *const libc::c_char
            } else {
                b"hit\0" as *const u8 as *const libc::c_char
            },
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckTimeLimit() {
    let mut i: int32_t = 0;
    let mut k: int32_t = 0;
    if cv_timelimit.value == 0 {
        return;
    }
    if !(multiplayer != 0 || netgame != 0) {
        return;
    }
    if gametyperules & GTR_TIMELIMIT as libc::c_int as uint32_t == 0 {
        return;
    }
    if leveltime < timelimitintics {
        return;
    }
    if gameaction as libc::c_uint == ga_completed as libc::c_int as libc::c_uint {
        return;
    }
    if G_TagGametype() != 0 {
        if leveltime == timelimitintics.wrapping_add(1 as libc::c_int as uint32_t) {
            i = 0 as libc::c_int;
            while i < 32 as libc::c_int {
                if !(playeringame[i as usize] == 0 || players[i as usize].spectator != 0
                    || players[i as usize].pflags as libc::c_uint
                        & PF_GAMETYPEOVER as libc::c_int as libc::c_uint != 0
                    || players[i as usize].pflags as libc::c_uint
                        & PF_TAGIT as libc::c_int as libc::c_uint != 0)
                {
                    CONS_Printf(
                        b"%s received double points for surviving the round.\n\0"
                            as *const u8 as *const libc::c_char,
                        (player_names[i as usize]).as_mut_ptr(),
                    );
                    P_AddPlayerScore(
                        &mut *players.as_mut_ptr().offset(i as isize),
                        players[i as usize].score,
                    );
                }
                i += 1;
                i;
            }
        }
        if server != 0 {
            D_SendExitLevel(false_0 as libc::c_int);
        }
    } else if cv_overtime.value != 0
        && gametyperules & GTR_OVERTIME as libc::c_int as uint32_t != 0
    {
        let mut playerarray: [int32_t; 32] = [0; 32];
        let mut tempplayer: int32_t = 0 as libc::c_int;
        let mut spectators: int32_t = 0 as libc::c_int;
        let mut playercount: int32_t = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0 && players[i as usize].spectator != 0 {
                spectators += 1;
                spectators;
            }
            i += 1;
            i;
        }
        if D_NumPlayers() - spectators > 1 as libc::c_int {
            if gamestate as libc::c_uint == GS_LEVEL as libc::c_int as libc::c_uint
                && leveltime
                    == timelimitintics.wrapping_add(35 as libc::c_int as uint32_t)
            {
                S_StartSound(0 as *const libc::c_void, sfx_strpst);
            }
            if G_GametypeHasTeams() == 0 {
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    if playeringame[i as usize] != 0
                        && players[i as usize].spectator == 0
                    {
                        playerarray[playercount as usize] = i;
                        playercount += 1;
                        playercount;
                    }
                    i += 1;
                    i;
                }
                i = 1 as libc::c_int;
                while i < playercount {
                    k = i;
                    while k < playercount {
                        if players[playerarray[(i - 1 as libc::c_int) as usize] as usize]
                            .score < players[playerarray[k as usize] as usize].score
                        {
                            tempplayer = playerarray[(i - 1 as libc::c_int) as usize];
                            playerarray[(i - 1 as libc::c_int)
                                as usize] = playerarray[k as usize];
                            playerarray[k as usize] = tempplayer;
                        }
                        k += 1;
                        k;
                    }
                    i += 1;
                    i;
                }
                if players[playerarray[0 as libc::c_int as usize] as usize].score
                    == players[playerarray[1 as libc::c_int as usize] as usize].score
                {
                    return;
                }
            } else if redscore == bluescore {
                return
            }
        }
        if server != 0 {
            D_SendExitLevel(false_0 as libc::c_int);
        }
    }
    if server != 0 {
        D_SendExitLevel(false_0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckPointLimit() {
    let mut i: int32_t = 0;
    if cv_pointlimit.value == 0 {
        return;
    }
    if !(multiplayer != 0 || netgame != 0) {
        return;
    }
    if gametyperules & GTR_POINTLIMIT as libc::c_int as uint32_t == 0 {
        return;
    }
    if G_GametypeHasTeams() != 0 {
        if cv_pointlimit.value as uint32_t <= redscore
            || cv_pointlimit.value as uint32_t <= bluescore
        {
            if server != 0 {
                D_SendExitLevel(false_0 as libc::c_int);
            }
        }
    } else {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if !(playeringame[i as usize] == 0 || players[i as usize].spectator != 0) {
                if cv_pointlimit.value as uint32_t <= players[i as usize].score {
                    if server != 0 {
                        D_SendExitLevel(false_0 as libc::c_int);
                    }
                    return;
                }
            }
            i += 1;
            i;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckSurvivors() {
    let mut i: int32_t = 0;
    let mut survivors: int32_t = 0 as libc::c_int;
    let mut taggers: int32_t = 0 as libc::c_int;
    let mut spectators: int32_t = 0 as libc::c_int;
    let mut survivorarray: [int32_t; 32] = [0; 32];
    if D_NumPlayers() == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 {
            if players[i as usize].spectator != 0 {
                spectators += 1;
                spectators;
            } else if players[i as usize].pflags as libc::c_uint
                & PF_TAGIT as libc::c_int as libc::c_uint != 0
                && players[i as usize].quittime
                    < (30 as libc::c_int * 35 as libc::c_int) as tic_t
            {
                taggers += 1;
                taggers;
            } else if players[i as usize].pflags as libc::c_uint
                & PF_GAMETYPEOVER as libc::c_int as libc::c_uint == 0
                && players[i as usize].quittime
                    < (30 as libc::c_int * 35 as libc::c_int) as tic_t
            {
                survivorarray[survivors as usize] = i;
                survivors += 1;
                survivors;
            }
        }
        i += 1;
        i;
    }
    if taggers == 0 {
        if gametyperules & GTR_HIDEFROZEN as libc::c_int as uint32_t != 0
            && leveltime >= hidetime * 35 as libc::c_int as tic_t
        {
            CONS_Printf(
                b"The IT player has left the game.\n\0" as *const u8
                    as *const libc::c_char,
            );
            if server != 0 {
                D_SendExitLevel(false_0 as libc::c_int);
            }
            return;
        }
        if survivors != 0 {
            let mut newtagger: int32_t = survivorarray[P_RandomKey(survivors) as usize];
            CONS_Printf(
                b"%s is now IT!\n\0" as *const u8 as *const libc::c_char,
                (player_names[newtagger as usize]).as_mut_ptr(),
            );
            players[newtagger as usize]
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                players[newtagger as usize].pflags as libc::c_uint
                    | PF_TAGIT as libc::c_int as libc::c_uint,
            );
            survivors -= 1;
            survivors;
            if survivors == 0 && D_NumPlayers() - spectators > 1 as libc::c_int {
                CONS_Printf(
                    b"All players have been tagged!\n\0" as *const u8
                        as *const libc::c_char,
                );
                if server != 0 {
                    D_SendExitLevel(false_0 as libc::c_int);
                }
            }
            return;
        }
        if D_NumPlayers() - spectators > 1 as libc::c_int {
            CONS_Printf(
                b"There are no players able to become IT.\n\0" as *const u8
                    as *const libc::c_char,
            );
            if server != 0 {
                D_SendExitLevel(false_0 as libc::c_int);
            }
        }
        return;
    }
    if survivors == 0 && D_NumPlayers() - spectators > 1 as libc::c_int {
        CONS_Printf(
            b"All players have been tagged!\n\0" as *const u8 as *const libc::c_char,
        );
        if server != 0 {
            D_SendExitLevel(false_0 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckRacers() -> boolean {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 && players[i as usize].exiting == 0
            && players[i as usize].lives as libc::c_int > 0 as libc::c_int
        {
            break;
        }
        i += 1;
        i;
    }
    if i == 32 as libc::c_int {
        countdown = 0 as libc::c_int as uint32_t;
        countdown2 = 0 as libc::c_int as uint32_t;
        return true_0 as libc::c_int;
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_KillMobj(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damagetype: uint8_t,
) {
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    if !inflictor.is_null()
        && ((*inflictor).type_0 as libc::c_uint
            == MT_SHELL as libc::c_int as libc::c_uint
            || (*inflictor).type_0 as libc::c_uint
                == MT_FIREBALL as libc::c_int as libc::c_uint)
    {
        S_StartSound(target as *const libc::c_void, sfx_mario2);
    }
    if maptol & TOL_NIGHTS as libc::c_int as uint32_t == 0
        && G_IsSpecialStage(gamemap as int32_t) != 0 && !((*target).player).is_null()
        && (*(*target).player).nightstime > 6 as libc::c_int as tic_t
    {
        (*(*target).player).nightstime = 6 as libc::c_int as tic_t;
    }
    if (*target).flags & (MF_ENEMY as libc::c_int | MF_BOSS as libc::c_int) as uint32_t
        != 0
    {
        (*target).momz = 0 as libc::c_int;
        (*target).momy = (*target).momz;
        (*target).momx = (*target).momy;
    }
    if (*target).type_0 as libc::c_uint != MT_PLAYER as libc::c_int as libc::c_uint
        && (*target).flags & MF_MONITOR as libc::c_int as uint32_t == 0
    {
        (*target).flags
            |= (MF_NOGRAVITY as libc::c_int | MF_NOCLIP as libc::c_int
                | MF_NOCLIPHEIGHT as libc::c_int) as uint32_t;
    }
    if (*target).flags2 & MF2_NIGHTSPULL as libc::c_int as uint32_t != 0 {
        P_SetTarget2(&mut (*target).tracer, 0 as *mut mobj_t);
        (*target).movefactor = 0 as libc::c_int;
    }
    (*target).flags
        &= !(MF_SHOOTABLE as libc::c_int | MF_FLOAT as libc::c_int
            | MF_SPECIAL as libc::c_int) as uint32_t;
    (*target).flags2
        &= !(MF2_SKULLFLY as libc::c_int | MF2_NIGHTSPULL as libc::c_int) as uint32_t;
    (*target).health = 0 as libc::c_int;
    if LUA_HookMobjDeath(target, inflictor, source, damagetype) != 0
        || P_MobjWasRemoved(target) != 0
    {
        return;
    }
    if !((*target).player).is_null() && (*(*target).player).spectator == 0 {
        if metalrecording != 0 {
            G_StopMetalRecording(true_0 as libc::c_int);
        }
        if gametyperules & GTR_DEATHPENALTY as libc::c_int as uint32_t != 0
            && (target == source || source.is_null() && inflictor.is_null()
                || !source.is_null() && ((*source).player).is_null())
        {
            if (*(*target).player).score >= 50 as libc::c_int as uint32_t {
                (*(*target).player)
                    .score = ((*(*target).player).score)
                    .wrapping_sub(50 as libc::c_int as uint32_t);
            } else {
                (*(*target).player).score = 0 as libc::c_int as uint32_t;
            }
        }
        (*target).flags2 &= !(MF2_DONTDRAW as libc::c_int) as uint32_t;
    }
    if !source.is_null() && !((*source).player).is_null() {
        if (*target).flags & MF_MONITOR as libc::c_int as uint32_t != 0 {
            P_SetTarget2(&mut (*target).target, source);
            (*(*source).player).numboxes += 1;
            (*(*source).player).numboxes;
            if cv_itemrespawn.value != 0
                && gametype as libc::c_int != GT_COOP as libc::c_int
                && (modifiedgame != 0 || netgame != 0 || multiplayer != 0)
            {
                (*target)
                    .fuse = cv_itemrespawntime.value * 35 as libc::c_int
                    + 2 as libc::c_int;
            }
        }
        let mut score: int32_t = 0 as libc::c_int;
        if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
            if (*target).flags & MF_ENEMY as libc::c_int as uint32_t != 0
                && (*target).flags
                    & (MF_MISSILE as libc::c_int | MF_BOSS as libc::c_int) as uint32_t
                    == 0
            {
                score = 200 as libc::c_int;
                if (*(*source).player).bonustime != 0 {
                    score *= 2 as libc::c_int;
                }
                (*(*source).player).linkcount += 1;
                if (*(*source).player).linkcount > (*(*source).player).maxlink {
                    (*(*source).player).maxlink = (*(*source).player).linkcount;
                }
                (*(*source).player).linktimer = nightslinktics as tic_t;
            }
        } else if (*target).flags & MF_BOSS as libc::c_int as uint32_t != 0 {
            score = 1000 as libc::c_int;
        } else if (*target).flags & MF_ENEMY as libc::c_int as uint32_t != 0
            && (*target).flags & MF_MISSILE as libc::c_int as uint32_t == 0
            && (*(*target).info).spawnhealth != 0
        {
            let mut locscoreadd: uint8_t = ((*(*source).player).scoreadd as libc::c_int
                + (*(*target).info).spawnhealth) as uint8_t;
            let mut scoremobj: *mut mobj_t = 0 as *mut mobj_t;
            let mut scorestate: uint32_t = mobjinfo[MT_SCORE as libc::c_int as usize]
                .spawnstate as uint32_t;
            scoremobj = P_SpawnMobj(
                (*target).x,
                (*target).y,
                (*target).z + (*target).height / 2 as libc::c_int,
                MT_SCORE,
            );
            if maptol & TOL_MARIO as libc::c_int as uint32_t == 0 {
                match locscoreadd as libc::c_int {
                    1 => {
                        score = 100 as libc::c_int;
                    }
                    2 => {
                        score = 200 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(1 as libc::c_int as uint32_t);
                    }
                    3 => {
                        score = 500 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(2 as libc::c_int as uint32_t);
                    }
                    4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 => {
                        score = 1000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(3 as libc::c_int as uint32_t);
                    }
                    _ => {
                        score = 10000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(4 as libc::c_int as uint32_t);
                    }
                }
            } else {
                match locscoreadd as libc::c_int {
                    1 => {
                        score = 100 as libc::c_int;
                    }
                    2 => {
                        score = 200 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(1 as libc::c_int as uint32_t);
                    }
                    3 => {
                        score = 400 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(5 as libc::c_int as uint32_t);
                    }
                    4 => {
                        score = 800 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(6 as libc::c_int as uint32_t);
                    }
                    5 => {
                        score = 1000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(3 as libc::c_int as uint32_t);
                    }
                    6 => {
                        score = 2000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(7 as libc::c_int as uint32_t);
                    }
                    7 => {
                        score = 4000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(8 as libc::c_int as uint32_t);
                    }
                    8 => {
                        score = 8000 as libc::c_int;
                        scorestate = scorestate
                            .wrapping_add(9 as libc::c_int as uint32_t);
                    }
                    _ => {
                        if modeattacking != 0 {
                            score = 10000 as libc::c_int;
                            scorestate = scorestate
                                .wrapping_add(4 as libc::c_int as uint32_t);
                        } else {
                            P_GivePlayerLives((*source).player, 1 as libc::c_int);
                            P_PlayLivesJingle((*source).player);
                            scorestate = scorestate
                                .wrapping_add(10 as libc::c_int as uint32_t);
                        }
                    }
                }
            }
            P_SetMobjState(scoremobj, scorestate as statenum_t);
            (*(*source).player).scoreadd = locscoreadd;
        }
        P_AddPlayerScore((*source).player, score as uint32_t);
    }
    if !((*target).player).is_null() {
        (*target).flags
            &= !(MF_SOLID as libc::c_int | MF_SHOOTABLE as libc::c_int) as uint32_t;
        P_UnsetThingPosition(target);
        (*target).flags
            |= (MF_NOBLOCKMAP as libc::c_int | MF_NOCLIP as libc::c_int
                | MF_NOCLIPHEIGHT as libc::c_int | MF_NOGRAVITY as libc::c_int)
                as uint32_t;
        P_SetThingPosition(target);
        (*target).standingslope = 0 as *mut pslope_s;
        (*target).pmomz = 0 as libc::c_int;
        if (*(*target).player).powers[pw_super as libc::c_int as usize] != 0 {
            (*(*target).player)
                .powers[pw_super as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
            if P_IsLocalPlayer((*target).player) != 0 {
                music_stack_noposition = true_0 as libc::c_int;
                music_stack_fadeout = (1000 as libc::c_int / 2 as libc::c_int)
                    as uint32_t;
            }
            P_RestoreMusic((*target).player);
            if G_CoopGametype() == 0 {
                HU_SetCEchoFlags(0 as libc::c_int);
                HU_SetCEchoDuration(5 as libc::c_int);
                HU_DoCEcho(
                    va(
                        b"%s\\is no longer super.\\\\\\\\\0" as *const u8
                            as *const libc::c_char,
                        (player_names[((*target).player)
                            .offset_from(players.as_mut_ptr()) as libc::c_long as usize])
                            .as_mut_ptr(),
                    ),
                );
            }
        }
        (*target).color = (*(*target).player).skincolor;
        (*target).colorized = false_0 as libc::c_int;
        G_GhostAddColor(GHC_NORMAL);
        if !((*(*target).player).lives as libc::c_int <= 1 as libc::c_int
            && (netgame != 0 || multiplayer != 0) && G_GametypeUsesCoopLives() != 0
            && cv_cooplives.value == 0 as libc::c_int)
        {
            if ((*(*target).player).bot == 0
                || (*(*target).player).bot as libc::c_int == BOT_MPAI as libc::c_int)
                && (*(*target).player).spectator == 0
                && (*(*target).player).lives as libc::c_int != 0x7f as libc::c_int
                && G_GametypeUsesLives() != 0
            {
                if (*(*target).player).pflags as libc::c_uint
                    & PF_FINISHED as libc::c_int as libc::c_uint == 0
                {
                    (*(*target).player)
                        .lives = ((*(*target).player).lives as libc::c_int
                        - 1 as libc::c_int) as int8_t;
                }
                if (*(*target).player).lives as libc::c_int <= 0 as libc::c_int {
                    let mut gameovermus: boolean = false_0 as libc::c_int;
                    if (netgame != 0 || multiplayer != 0)
                        && G_GametypeUsesCoopLives() != 0
                        && cv_cooplives.value != 1 as libc::c_int
                    {
                        let mut i: int32_t = 0;
                        i = 0 as libc::c_int;
                        while i < 32 as libc::c_int {
                            if !(playeringame[i as usize] == 0) {
                                if players[i as usize].lives as libc::c_int
                                    > 0 as libc::c_int
                                {
                                    break;
                                }
                            }
                            i += 1;
                            i;
                        }
                        if i == 32 as libc::c_int {
                            gameovermus = true_0 as libc::c_int;
                        }
                    } else if P_IsLocalPlayer((*target).player) != 0 {
                        gameovermus = true_0 as libc::c_int;
                    }
                    if gameovermus != 0 {
                        S_ChangeMusicEx(
                            b"_gover\0" as *const u8 as *const libc::c_char,
                            0 as libc::c_int as uint16_t,
                            0 as libc::c_int,
                            0 as libc::c_int as uint32_t,
                            (2 as libc::c_int * 1000 as libc::c_int
                                - 1000 as libc::c_int / 25 as libc::c_int) as uint32_t,
                            0 as libc::c_int as uint32_t,
                        );
                    }
                    if !(netgame != 0 || multiplayer != 0 || demoplayback != 0
                        || demorecording != 0 || metalrecording != 0
                        || modeattacking as libc::c_int != 0)
                        && (numgameovers as libc::c_int) < 13 as libc::c_int
                    {
                        numgameovers = numgameovers.wrapping_add(1);
                        numgameovers;
                        if usedCheats == 0 && cursaveslot > 0 as libc::c_int {
                            G_SaveGameOver(
                                cursaveslot as uint32_t,
                                ((*(*target).player).continues as libc::c_int
                                    <= 0 as libc::c_int) as libc::c_int,
                            );
                        }
                    }
                }
            }
        }
        (*(*target).player).playerstate = PST_DEAD;
        if (*target).player
            == &mut *players.as_mut_ptr().offset(consoleplayer as isize) as *mut player_t
        {
            if automapactive != 0 {
                AM_Stop();
            }
            localaiming = 0 as libc::c_int;
        }
        if (*target).player
            == &mut *players.as_mut_ptr().offset(secondarydisplayplayer as isize)
                as *mut player_t
        {
            localaiming2 = 0 as libc::c_int;
        }
        if G_TagGametype() != 0
            && (*(*target).player).pflags as libc::c_uint
                & PF_TAGIT as libc::c_int as libc::c_uint == 0
            && (source.is_null() || ((*source).player).is_null())
            && (*(*target).player).spectator == 0
        {
            if leveltime >= hidetime * 35 as libc::c_int as tic_t {
                if gametyperules & GTR_HIDEFROZEN as libc::c_int as uint32_t == 0 {
                    (*(*target).player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*(*target).player).pflags as libc::c_uint
                            | PF_TAGIT as libc::c_int as libc::c_uint,
                    );
                    CONS_Printf(
                        b"%s is now IT!\n\0" as *const u8 as *const libc::c_char,
                        (player_names[((*target).player)
                            .offset_from(players.as_mut_ptr()) as libc::c_long as usize])
                            .as_mut_ptr(),
                    );
                    P_CheckSurvivors();
                } else if (*(*target).player).pflags as libc::c_uint
                    & PF_GAMETYPEOVER as libc::c_int as libc::c_uint == 0
                {
                    let mut w: int32_t = 0;
                    w = 0 as libc::c_int;
                    while w < 32 as libc::c_int {
                        if players[w as usize].pflags as libc::c_uint
                            & PF_TAGIT as libc::c_int as libc::c_uint != 0
                        {
                            P_AddPlayerScore(
                                &mut *players.as_mut_ptr().offset(w as isize),
                                100 as libc::c_int as uint32_t,
                            );
                        }
                        w += 1;
                        w;
                    }
                    (*(*target).player)
                        .pflags = ::core::mem::transmute::<
                        libc::c_uint,
                        pflags_t,
                    >(
                        (*(*target).player).pflags as libc::c_uint
                            | PF_GAMETYPEOVER as libc::c_int as libc::c_uint,
                    );
                    CONS_Printf(
                        b"%s was found!\n\0" as *const u8 as *const libc::c_char,
                        (player_names[((*target).player)
                            .offset_from(players.as_mut_ptr()) as libc::c_long as usize])
                            .as_mut_ptr(),
                    );
                    P_CheckSurvivors();
                }
            }
        }
    }
    if !source.is_null() && !target.is_null() && !((*target).player).is_null()
        && !((*source).player).is_null()
    {
        S_StartSound(
            source as *const libc::c_void,
            (sfx_victr1 as libc::c_int + P_RandomKey(4 as libc::c_int)) as sfxenum_t,
        );
    }
    match (*target).type_0 as libc::c_uint {
        538 | 539 | 540 | 541 | 542 | 543 => {
            P_SetObjectMomZ(
                target,
                (1 as libc::c_int) << 16 as libc::c_int,
                false_0 as libc::c_int,
            );
            (*target).fuse = (*(*target).info).damage;
        }
        601 => {
            if !inflictor.is_null() && !((*inflictor).player).is_null()
                && P_AproxDistance(
                    (*inflictor).x - (*target).x,
                    (*inflictor).y - (*target).y,
                )
                    <= (*inflictor).radius + (*target).radius
                        + FixedMul(
                            8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                            (*inflictor).scale,
                        )
                && (*inflictor).z
                    <= (*target).z + (*target).height
                        + FixedMul(
                            8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                            (*inflictor).scale,
                        )
                && (*inflictor).z + (*inflictor).height
                    >= (*target).z
                        - FixedMul(
                            8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                            (*inflictor).scale,
                        )
            {
                mo = P_SpawnMobj(
                    (*inflictor).x + (*inflictor).momx,
                    (*inflictor).y + (*inflictor).momy,
                    (*inflictor).z + (*inflictor).height / 2 as libc::c_int
                        + (*inflictor).momz,
                    MT_EXTRALARGEBUBBLE,
                );
            } else {
                mo = P_SpawnMobj(
                    (*target).x,
                    (*target).y,
                    (*target).z,
                    MT_EXTRALARGEBUBBLE,
                );
            }
            (*mo).destscale = (*target).scale;
            P_SetScale(mo, (*mo).destscale);
            P_SetMobjState(mo, (*(*mo).info).raisestate);
        }
        40 => {
            P_SpawnMobjFromMobj(
                target,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                MT_YELLOWSPRING,
            );
        }
        13 => {
            (*target).momz = 0 as libc::c_int;
            (*target).momy = (*target).momz;
            (*target).momx = (*target).momy;
        }
        19 => {
            if !((*target).tracer).is_null() {
                let mut chain: *mut mobj_t = (*(*target).tracer).target;
                let mut chainnext: *mut mobj_t = 0 as *mut mobj_t;
                while !chain.is_null() {
                    chainnext = (*chain).target;
                    P_RemoveMobj(chain);
                    chain = chainnext;
                }
                S_StopSound((*target).tracer as *mut libc::c_void);
                P_KillMobj((*target).tracer, inflictor, source, damagetype);
            }
        }
        22 => {
            if !((*target).tracer).is_null() {
                S_StopSound((*target).tracer as *mut libc::c_void);
                P_KillMobj((*target).tracer, inflictor, source, damagetype);
            }
        }
        33 => {
            P_SetObjectMomZ(
                target,
                4 as libc::c_int * (*target).scale,
                false_0 as libc::c_int,
            );
            P_InstaThrust(target, (*target).angle, 3 as libc::c_int * (*target).scale);
            (*target)
                .flags = ((*target).flags | MF_NOCLIPHEIGHT as libc::c_int as uint32_t)
                & !(MF_NOGRAVITY as libc::c_int) as uint32_t;
        }
        50 => {
            let mut segment: *mut mobj_t = target;
            while !((*segment).tracer).is_null() {
                P_KillMobj(
                    (*segment).tracer,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    0 as libc::c_int as uint8_t,
                );
                segment = (*segment).tracer;
            }
        }
        70 => {
            let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
            let mut th: *mut thinker_t = 0 as *mut thinker_t;
            let mut i_0: uint32_t = 0 as libc::c_int as uint32_t;
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
                    mo = th as *mut mobj_t;
                    if !((*mo).type_0 as libc::c_uint
                        != (*(*target).info).mass as mobjtype_t as libc::c_uint)
                    {
                        if !((*mo).tracer != target) {
                            P_KillMobj(mo, inflictor, source, damagetype);
                            (*mo).destscale = (*mo).scale / 8 as libc::c_int;
                            (*mo)
                                .scalespeed = ((*mo).scale - (*mo).destscale)
                                / (2 as libc::c_int * 35 as libc::c_int);
                            (*mo).momz = (*(*mo).info).speed;
                            (*mo)
                                .angle = FixedAngle(
                                (P_RandomKey(36 as libc::c_int) * 10 as libc::c_int)
                                    << 16 as libc::c_int,
                            );
                            mo2 = P_SpawnMobjFromMobj(
                                mo,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                0 as libc::c_int,
                                MT_BOSSJUNK,
                            );
                            (*mo2).angle = (*mo).angle;
                            P_SetMobjState(mo2, S_BOSSSEBH2);
                            i_0 = i_0.wrapping_add(1);
                            if i_0 == 2 as libc::c_int as uint32_t {
                                break;
                            }
                            S_StartSound(
                                mo as *const libc::c_void,
                                (*(*mo).info).deathsound,
                            );
                        }
                    }
                }
                th = (*th).next;
            }
        }
        153 => {
            if !inflictor.is_null() {
                let mut dx: fixed_t = (*target).x - (*inflictor).x;
                let mut dy: fixed_t = (*target).y - (*inflictor).y;
                let mut dz: fixed_t = (*target).z - (*inflictor).z;
                let mut dm: fixed_t = FixedHypot(dz, FixedHypot(dy, dx));
                (*target).momx = FixedDiv(FixedDiv(dx, dm), dm) * 512 as libc::c_int;
                (*target).momy = FixedDiv(FixedDiv(dy, dm), dm) * 512 as libc::c_int;
                (*target).momz = FixedDiv(FixedDiv(dz, dm), dm) * 512 as libc::c_int;
            }
            if !source.is_null() {
                P_SetTarget2(&mut (*target).tracer, source);
            }
        }
        154 => {
            if !((*target).spawnpoint).is_null() {
                P_LinedefExecute(
                    (*(*target).spawnpoint).args[0 as libc::c_int as usize] as int16_t,
                    if !source.is_null() { source } else { inflictor },
                    (*(*target).subsector).sector,
                );
            }
        }
        606 => {
            if !((*target).hnext).is_null() {
                P_KillMobj((*target).hnext, inflictor, source, damagetype);
            }
            if !((*target).hprev).is_null() {
                P_KillMobj((*target).hprev, inflictor, source, damagetype);
            }
        }
        57 => {
            (*target).fuse = 35 as libc::c_int;
        }
        333 => {
            A_Scream(target);
            (*target).momz = 0 as libc::c_int;
            (*target).momy = (*target).momz;
            (*target).momx = (*target).momy;
            if !((*target).target).is_null() && (*(*target).target).health != 0 {
                P_KillMobj(
                    (*target).target,
                    target,
                    source,
                    0 as libc::c_int as uint8_t,
                );
            }
        }
        3 => {
            (*target).fuse = 35 as libc::c_int * 3 as libc::c_int;
            (*target).momz = 0 as libc::c_int;
            (*target).momy = (*target).momz;
            (*target).momx = (*target).momy;
            if damagetype as libc::c_int == 0x80 as libc::c_int + 1 as libc::c_int {
                (*target).movedir = damagetype as angle_t;
                if (*(*target).player).charflags & SF_MACHINE as libc::c_int as uint32_t
                    != 0
                {
                    S_StartSound(target as *const libc::c_void, sfx_fizzle);
                } else {
                    S_StartSound(target as *const libc::c_void, sfx_drown);
                }
            } else {
                P_SetObjectMomZ(
                    target,
                    14 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    false_0 as libc::c_int,
                );
                if damagetype as libc::c_int == 4 as libc::c_int {
                    S_StartSound(target as *const libc::c_void, sfx_spkdth);
                } else {
                    S_StartSound(
                        target as *const libc::c_void,
                        (sfx_altdi1 as libc::c_int + P_RandomKey(4 as libc::c_int))
                            as sfxenum_t,
                    );
                }
            }
        }
        103 => {
            (*target).fuse = 35 as libc::c_int * 3 as libc::c_int;
            (*target).momz = 0 as libc::c_int;
            (*target).momy = (*target).momz;
            (*target).momx = (*target).momy;
            P_SetObjectMomZ(
                target,
                14 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                false_0 as libc::c_int,
            );
            (*target)
                .flags = (*target).flags & !(MF_NOGRAVITY as libc::c_int) as uint32_t
                | (MF_NOCLIP as libc::c_int | MF_NOCLIPTHING as libc::c_int) as uint32_t;
        }
        _ => {}
    }
    if (*target).type_0 as libc::c_uint == MT_SPIKE as libc::c_int as libc::c_uint
        && (*(*target).info).deathstate as libc::c_uint
            != S_NULL as libc::c_int as libc::c_uint
    {
        let ang: angle_t = (if !inflictor.is_null() {
            (*inflictor).angle
        } else {
            0 as libc::c_int as angle_t
        })
            .wrapping_add(0x40000000 as libc::c_int as angle_t);
        let scale: fixed_t = (*target).scale;
        let xoffs: fixed_t = P_ReturnThrustX(target, ang, 8 as libc::c_int * scale);
        let yoffs: fixed_t = P_ReturnThrustY(target, ang, 8 as libc::c_int * scale);
        let flip: uint16_t = ((*target).eflags as libc::c_int
            & MFE_VERTICALFLIP as libc::c_int) as uint16_t;
        let mut chunk: *mut mobj_t = 0 as *mut mobj_t;
        let mut momz: fixed_t = 0;
        S_StartSound(target as *const libc::c_void, (*(*target).info).deathsound);
        if (*(*target).info).xdeathstate as libc::c_uint
            != S_NULL as libc::c_int as libc::c_uint
        {
            momz = 6 as libc::c_int * scale;
            if flip != 0 {
                momz *= -(1 as libc::c_int);
            }
            chunk = P_SpawnMobjFromMobj(
                target,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                MT_SPIKE,
            );
            P_SetMobjState(chunk, (*(*target).info).xdeathstate);
            (*chunk).health = 0 as libc::c_int;
            (*chunk).angle = ang.wrapping_add(0x80000000 as libc::c_uint);
            P_UnsetThingPosition(chunk);
            (*chunk).flags = MF_NOCLIP as libc::c_int as uint32_t;
            (*chunk).x += -xoffs;
            (*chunk).y += -yoffs;
            P_SetThingPosition(chunk);
            P_InstaThrust(chunk, (*chunk).angle, 4 as libc::c_int * scale);
            (*chunk).momz = momz;
            chunk = P_SpawnMobjFromMobj(
                target,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                MT_SPIKE,
            );
            P_SetMobjState(chunk, (*(*target).info).xdeathstate);
            (*chunk).health = 0 as libc::c_int;
            (*chunk).angle = ang;
            P_UnsetThingPosition(chunk);
            (*chunk).flags = MF_NOCLIP as libc::c_int as uint32_t;
            (*chunk).x += xoffs;
            (*chunk).y += yoffs;
            P_SetThingPosition(chunk);
            P_InstaThrust(chunk, (*chunk).angle, 4 as libc::c_int * scale);
            (*chunk).momz = momz;
        }
        momz = 7 as libc::c_int * scale;
        if flip != 0 {
            momz *= -(1 as libc::c_int);
        }
        chunk = P_SpawnMobjFromMobj(
            target,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            MT_SPIKE,
        );
        P_SetMobjState(chunk, (*(*target).info).deathstate);
        (*chunk).health = 0 as libc::c_int;
        (*chunk).angle = ang.wrapping_add(0x80000000 as libc::c_uint);
        P_UnsetThingPosition(chunk);
        (*chunk).flags = MF_NOCLIP as libc::c_int as uint32_t;
        (*chunk).x -= xoffs;
        (*chunk).y -= yoffs;
        if flip != 0 {
            (*chunk).z -= 12 as libc::c_int * scale;
        } else {
            (*chunk).z += 12 as libc::c_int * scale;
        }
        P_SetThingPosition(chunk);
        P_InstaThrust(chunk, (*chunk).angle, 2 as libc::c_int * scale);
        (*chunk).momz = momz;
        P_SetMobjState(target, (*(*target).info).deathstate);
        (*target).health = 0 as libc::c_int;
        (*target).angle = ang;
        P_UnsetThingPosition(target);
        (*target).flags = MF_NOCLIP as libc::c_int as uint32_t;
        (*target).x += xoffs;
        (*target).y += yoffs;
        (*target).z = (*chunk).z;
        P_SetThingPosition(target);
        P_InstaThrust(target, (*target).angle, 2 as libc::c_int * scale);
        (*target).momz = momz;
    } else if (*target).type_0 as libc::c_uint
        == MT_WALLSPIKE as libc::c_int as libc::c_uint
        && (*(*target).info).deathstate as libc::c_uint
            != S_NULL as libc::c_int as libc::c_uint
    {
        let ang_0: angle_t = ((*target).angle)
            .wrapping_add(0x40000000 as libc::c_int as angle_t);
        let scale_0: fixed_t = (*target).scale;
        let xoffs_0: fixed_t = P_ReturnThrustX(
            target,
            ang_0,
            8 as libc::c_int * scale_0,
        );
        let yoffs_0: fixed_t = P_ReturnThrustY(
            target,
            ang_0,
            8 as libc::c_int * scale_0,
        );
        let forwardxoffs: fixed_t = P_ReturnThrustX(
            target,
            (*target).angle,
            7 as libc::c_int * scale_0,
        );
        let forwardyoffs: fixed_t = P_ReturnThrustY(
            target,
            (*target).angle,
            7 as libc::c_int * scale_0,
        );
        let flip_0: uint16_t = ((*target).eflags as libc::c_int
            & MFE_VERTICALFLIP as libc::c_int) as uint16_t;
        let mut chunk_0: *mut mobj_t = 0 as *mut mobj_t;
        let mut sprflip: boolean = 0;
        S_StartSound(target as *const libc::c_void, (*(*target).info).deathsound);
        if P_MobjWasRemoved((*target).tracer) == 0 {
            P_RemoveMobj((*target).tracer);
        }
        if (*(*target).info).xdeathstate as libc::c_uint
            != S_NULL as libc::c_int as libc::c_uint
        {
            sprflip = (P_RandomFixed()
                < ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int)
                as libc::c_int;
            chunk_0 = P_SpawnMobjFromMobj(
                target,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                MT_WALLSPIKE,
            );
            P_SetMobjState(chunk_0, (*(*target).info).xdeathstate);
            (*chunk_0).health = 0 as libc::c_int;
            (*chunk_0).angle = (*target).angle;
            P_UnsetThingPosition(chunk_0);
            (*chunk_0).flags = MF_NOCLIP as libc::c_int as uint32_t;
            (*chunk_0).x += -xoffs_0 - forwardxoffs;
            (*chunk_0).y += -yoffs_0 - forwardyoffs;
            P_SetThingPosition(chunk_0);
            P_InstaThrust(
                chunk_0,
                ang_0.wrapping_add(0x80000000 as libc::c_uint),
                4 as libc::c_int * scale_0,
            );
            (*chunk_0)
                .momz = P_RandomRange(5 as libc::c_int, 7 as libc::c_int) * scale_0;
            if flip_0 != 0 {
                (*chunk_0).momz *= -(1 as libc::c_int);
            }
            if sprflip != 0 {
                (*chunk_0).frame |= 0x1000000 as libc::c_int as uint32_t;
            }
            sprflip = (sprflip == 0) as libc::c_int;
            chunk_0 = P_SpawnMobjFromMobj(
                target,
                0 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
                MT_WALLSPIKE,
            );
            P_SetMobjState(chunk_0, (*(*target).info).xdeathstate);
            (*chunk_0).health = 0 as libc::c_int;
            (*chunk_0).angle = (*target).angle;
            P_UnsetThingPosition(chunk_0);
            (*chunk_0).flags = MF_NOCLIP as libc::c_int as uint32_t;
            (*chunk_0).x += xoffs_0 - forwardxoffs;
            (*chunk_0).y += yoffs_0 - forwardyoffs;
            P_SetThingPosition(chunk_0);
            P_InstaThrust(chunk_0, ang_0, 4 as libc::c_int * scale_0);
            (*chunk_0)
                .momz = P_RandomRange(5 as libc::c_int, 7 as libc::c_int) * scale_0;
            if flip_0 != 0 {
                (*chunk_0).momz *= -(1 as libc::c_int);
            }
            if sprflip != 0 {
                (*chunk_0).frame |= 0x1000000 as libc::c_int as uint32_t;
            }
        }
        sprflip = (P_RandomFixed()
            < ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int)
            as libc::c_int;
        chunk_0 = P_SpawnMobjFromMobj(
            target,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            MT_WALLSPIKE,
        );
        P_SetMobjState(chunk_0, (*(*target).info).deathstate);
        (*chunk_0).health = 0 as libc::c_int;
        (*chunk_0).angle = (*target).angle;
        P_UnsetThingPosition(chunk_0);
        (*chunk_0).flags = MF_NOCLIP as libc::c_int as uint32_t;
        (*chunk_0).x += forwardxoffs - xoffs_0;
        (*chunk_0).y += forwardyoffs - yoffs_0;
        P_SetThingPosition(chunk_0);
        P_InstaThrust(
            chunk_0,
            ang_0.wrapping_add(0x80000000 as libc::c_uint),
            2 as libc::c_int * scale_0,
        );
        (*chunk_0).momz = P_RandomRange(5 as libc::c_int, 7 as libc::c_int) * scale_0;
        if flip_0 != 0 {
            (*chunk_0).momz *= -(1 as libc::c_int);
        }
        if sprflip != 0 {
            (*chunk_0).frame |= 0x1000000 as libc::c_int as uint32_t;
        }
        P_SetMobjState(target, (*(*target).info).deathstate);
        (*target).health = 0 as libc::c_int;
        P_UnsetThingPosition(target);
        (*target).flags = MF_NOCLIP as libc::c_int as uint32_t;
        (*target).x += forwardxoffs + xoffs_0;
        (*target).y += forwardyoffs + yoffs_0;
        P_SetThingPosition(target);
        P_InstaThrust(target, ang_0, 2 as libc::c_int * scale_0);
        (*target).momz = P_RandomRange(5 as libc::c_int, 7 as libc::c_int) * scale_0;
        if flip_0 != 0 {
            (*target).momz *= -(1 as libc::c_int);
        }
        if sprflip == 0 {
            (*target).frame |= 0x1000000 as libc::c_int as uint32_t;
        }
    } else if !((*target).player).is_null() {
        if damagetype as libc::c_int == 0x80 as libc::c_int + 1 as libc::c_int
            || damagetype as libc::c_int == 0x80 as libc::c_int + 2 as libc::c_int
        {
            P_SetPlayerMobjState(target, (*(*target).info).xdeathstate);
        } else {
            P_SetPlayerMobjState(target, (*(*target).info).deathstate);
        }
    } else {
        P_SetMobjState(target, (*(*target).info).deathstate);
    };
}
unsafe extern "C" fn P_NiGHTSDamage(mut target: *mut mobj_t, mut source: *mut mobj_t) {
    let mut player: *mut player_t = (*target).player;
    let mut oldnightstime: tic_t = (*player).nightstime;
    if (*player).powers[pw_flashing as libc::c_int as usize] == 0 {
        let mut fa: angle_t = 0;
        (*player).angle_pos = (*player).old_angle_pos;
        (*player).speed /= 5 as libc::c_int;
        (*player).flyangle += 180 as libc::c_int;
        (*player).flyangle %= 360 as libc::c_int;
        if gametyperules & GTR_RACE as libc::c_int as uint32_t != 0 {
            (*player).drillmeter -= 5 as libc::c_int * 20 as libc::c_int;
        } else if (*player).nightstime > (5 as libc::c_int * 35 as libc::c_int) as tic_t
        {
            (*player)
                .nightstime = ((*player).nightstime)
                .wrapping_sub((5 as libc::c_int * 35 as libc::c_int) as tic_t);
        } else {
            (*player).nightstime = 1 as libc::c_int as tic_t;
        }
        if (*player).pflags as libc::c_uint
            & PF_TRANSFERTOCLOSEST as libc::c_int as libc::c_uint != 0
        {
            (*target).momx = -(*target).momx;
            (*target).momy = -(*target).momy;
        } else {
            fa = (*player).old_angle_pos >> 19 as libc::c_int;
            (*target)
                .momx = FixedMul(
                *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
                (*(*target).target).radius,
            );
            (*target)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                (*(*target).target).radius,
            );
        }
        (*player).powers[pw_flashing as libc::c_int as usize] = flashingtics;
        P_SetPlayerMobjState(target, S_PLAY_NIGHTS_STUN);
        S_StartSound(target as *const libc::c_void, sfx_nghurt);
        (*(*player).mo).spriteroll = 0 as libc::c_int as angle_t;
        if oldnightstime > (10 as libc::c_int * 35 as libc::c_int) as tic_t
            && (*player).nightstime < (10 as libc::c_int * 35 as libc::c_int) as tic_t
        {
            if (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
                .levelflags as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0
            {
                S_FadeMusicFromVolume(
                    0 as libc::c_int as uint8_t,
                    -(1 as libc::c_int) as int16_t,
                    (10 as libc::c_int * 1000 as libc::c_int) as uint32_t,
                );
                S_StartSound(0 as *const libc::c_void, sfx_timeup);
            } else {
                P_PlayJingle(
                    player,
                    (if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
                        && G_IsSpecialStage(gamemap as int32_t) == 0
                    {
                        JT_NIGHTSTIMEOUT as libc::c_int
                    } else {
                        JT_SSTIMEOUT as libc::c_int
                    }) as jingletype_t,
                );
            }
        }
    }
}
unsafe extern "C" fn P_TagDamage(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
    mut damagetype: uint8_t,
) -> boolean {
    let mut player: *mut player_t = (*target).player;
    if (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int != 0
        || (*player).powers[pw_invulnerability as libc::c_int as usize] as libc::c_int
            != 0
    {
        return false_0 as libc::c_int;
    }
    if leveltime <= hidetime * 35 as libc::c_int as tic_t {
        return false_0 as libc::c_int;
    }
    if (*player).pflags as libc::c_uint & PF_TAGIT as libc::c_int as libc::c_uint != 0
        && !((cv_friendlyfire.value != 0
            || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t != 0
            || damagetype as libc::c_int & 0x40 as libc::c_int != 0) && !source.is_null()
            && !((*source).player).is_null()
            && (*(*source).player).pflags as libc::c_uint
                & PF_TAGIT as libc::c_int as libc::c_uint != 0)
    {
        if (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint
            && (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                & SH_NOSTACK as libc::c_int == 0
        {
            if (*player).revitem as libc::c_uint
                != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).spinitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).thokitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
            {
                P_SwitchShield(player, SH_PINK as libc::c_int as uint16_t);
                S_StartSound(
                    target as *const libc::c_void,
                    mobjinfo[MT_PITY_ICON as libc::c_int as usize].seesound,
                );
            }
        }
        return false_0 as libc::c_int;
    }
    if !(cv_friendlyfire.value != 0
        || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t != 0
        || damagetype as libc::c_int & 0x40 as libc::c_int != 0)
        && (*player).pflags as libc::c_uint & PF_TAGIT as libc::c_int as libc::c_uint
            == (*(*source).player).pflags as libc::c_uint
                & PF_TAGIT as libc::c_int as libc::c_uint
    {
        if (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint
            && (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                & SH_NOSTACK as libc::c_int == 0
        {
            if (*player).revitem as libc::c_uint
                != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).spinitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).thokitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
            {
                P_SwitchShield(player, SH_PINK as libc::c_int as uint16_t);
                S_StartSound(
                    target as *const libc::c_void,
                    mobjinfo[MT_PITY_ICON as libc::c_int as usize].seesound,
                );
            }
        } else if (*inflictor).flags & MF_FIRE as libc::c_int as uint32_t == 0 {
            P_GivePlayerRings(player, 1 as libc::c_int);
        }
        if (*inflictor).flags2 & MF2_BOUNCERING as libc::c_int as uint32_t != 0 {
            (*inflictor).fuse = 0 as libc::c_int;
        }
        return false_0 as libc::c_int;
    }
    if (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint {
        return false_0 as libc::c_int;
    }
    if (*(*source).player).pflags as libc::c_uint
        & PF_TAGIT as libc::c_int as libc::c_uint != 0
        && (*player).pflags as libc::c_uint & PF_TAGIT as libc::c_int as libc::c_uint
            == 0
    {
        P_AddPlayerScore((*source).player, 100 as libc::c_int as uint32_t);
        P_HitDeathMessages(player, inflictor, source, 0 as libc::c_int as uint8_t);
        if gametyperules & GTR_HIDEFROZEN as libc::c_int as uint32_t == 0 {
            (*player)
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                (*player).pflags as libc::c_uint
                    | PF_TAGIT as libc::c_int as libc::c_uint,
            );
            CONS_Printf(
                b"%s is now IT!\n\0" as *const u8 as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
            );
        } else {
            (*player)
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                (*player).pflags as libc::c_uint
                    | PF_GAMETYPEOVER as libc::c_int as libc::c_uint,
            );
            CONS_Printf(
                b"%s was found!\n\0" as *const u8 as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
            );
        }
        P_CheckSurvivors();
    }
    P_DoPlayerPain(player, source, inflictor);
    if (*player).powers[pw_shield as libc::c_int as usize] != 0 {
        P_RemoveShield(player);
        S_StartSound(target as *const libc::c_void, sfx_shldls);
        return true_0 as libc::c_int;
    }
    if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
        == CR_NIGHTSFALL as libc::c_int
    {
        if (*player).spheres as libc::c_int > 0 as libc::c_int {
            S_StartSound(
                target as *const libc::c_void,
                (if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                    sfx_mario8 as libc::c_int
                } else {
                    sfx_altow1 as libc::c_int + P_RandomKey(4 as libc::c_int)
                }) as sfxenum_t,
            );
            P_PlayerRingBurst(player, (*player).spheres as int32_t);
            (*player).spheres = 0 as libc::c_int as int16_t;
        }
    } else if (*player).rings as libc::c_int > 0 as libc::c_int {
        S_StartSound(
            target as *const libc::c_void,
            (if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
                sfx_mario8 as libc::c_int
            } else {
                sfx_altow1 as libc::c_int + P_RandomKey(4 as libc::c_int)
            }) as sfxenum_t,
        );
        P_PlayerRingBurst(player, (*player).rings as int32_t);
        (*player).rings = 0 as libc::c_int as int16_t;
    } else {
        S_StartSound(
            target as *const libc::c_void,
            (sfx_altdi1 as libc::c_int + P_RandomKey(4 as libc::c_int)) as sfxenum_t,
        );
        S_StartSound(
            source as *const libc::c_void,
            (sfx_victr1 as libc::c_int + P_RandomKey(4 as libc::c_int)) as sfxenum_t,
        );
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn P_PlayerHitsPlayer(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
    mut damagetype: uint8_t,
) -> boolean {
    let mut player: *mut player_t = (*target).player;
    if damagetype as libc::c_int & 0x40 as libc::c_int == 0 {
        if source == target {
            return false_0 as libc::c_int;
        }
        if !(cv_friendlyfire.value != 0
            || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t != 0)
            && gametyperules & GTR_FRIENDLY as libc::c_int as uint32_t != 0
        {
            if gametyperules & GTR_FRIENDLY as libc::c_int as uint32_t != 0
                && (*inflictor).type_0 as libc::c_uint
                    == MT_LHRT as libc::c_int as libc::c_uint
                && (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                    & SH_NOSTACK as libc::c_int == 0
            {
                if (*player).revitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
                    && (*player).spinitem as libc::c_uint
                        != MT_LHRT as libc::c_int as libc::c_uint
                    && (*player).thokitem as libc::c_uint
                        != MT_LHRT as libc::c_int as libc::c_uint
                {
                    P_SwitchShield(player, SH_PINK as libc::c_int as uint16_t);
                    S_StartSound(
                        target as *const libc::c_void,
                        mobjinfo[MT_PITY_ICON as libc::c_int as usize].seesound,
                    );
                }
            }
            return false_0 as libc::c_int;
        }
    }
    if G_TagGametype() != 0 {
        return P_TagDamage(target, inflictor, source, damage, damagetype)
    } else if damagetype as libc::c_int & 0x40 as libc::c_int != 0 {
        return true_0 as libc::c_int
    } else if G_GametypeHasTeams() != 0 {
        if !(cv_friendlyfire.value != 0
            || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t != 0)
            && (*(*target).player).ctfteam == (*(*source).player).ctfteam
        {
            if (*inflictor).type_0 as libc::c_uint
                == MT_LHRT as libc::c_int as libc::c_uint
                && (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                    & SH_NOSTACK as libc::c_int == 0
            {
                if (*player).revitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
                    && (*player).spinitem as libc::c_uint
                        != MT_LHRT as libc::c_int as libc::c_uint
                    && (*player).thokitem as libc::c_uint
                        != MT_LHRT as libc::c_int as libc::c_uint
                {
                    P_SwitchShield(player, SH_PINK as libc::c_int as uint16_t);
                    S_StartSound(
                        target as *const libc::c_void,
                        mobjinfo[MT_PITY_ICON as libc::c_int as usize].seesound,
                    );
                }
            } else if (*inflictor).flags & MF_FIRE as libc::c_int as uint32_t == 0 {
                P_GivePlayerRings((*target).player, 1 as libc::c_int);
            }
            if (*inflictor).flags2 & MF2_BOUNCERING as libc::c_int as uint32_t != 0 {
                (*inflictor).fuse = 0 as libc::c_int;
            }
            return false_0 as libc::c_int;
        }
    }
    if (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint {
        return false_0 as libc::c_int;
    }
    if (*player).powers[pw_flashing as libc::c_int as usize] == 0
        && (*player).powers[pw_invulnerability as libc::c_int as usize] == 0
        && (*player).powers[pw_super as libc::c_int as usize] == 0
        && (*player).powers[pw_strong as libc::c_int as usize] as libc::c_int
            & STR_GUARD as libc::c_int == 0
        && (*(*source).player).score > (*player).score
    {
        (*player).pity += 1;
        (*player).pity;
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn P_KillPlayer(
    mut player: *mut player_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
) {
    (*player)
        .pflags = ::core::mem::transmute::<
        libc::c_uint,
        pflags_t,
    >((*player).pflags as libc::c_uint & !(PF_SLIDING as libc::c_int) as libc::c_uint);
    (*player)
        .powers[pw_carry as libc::c_int as usize] = CR_NONE as libc::c_int as uint16_t;
    if !source.is_null() {
        if gametyperules & GTR_RINGSLINGER as libc::c_int as uint32_t != 0
            && gametyperules & GTR_TAG as libc::c_int as uint32_t == 0
        {
            P_PlayerRingBurst(player, (*player).rings as int32_t);
        }
        if gametyperules & GTR_POWERSTONES as libc::c_int as uint32_t != 0 {
            P_PlayerEmeraldBurst(player, false_0 as libc::c_int);
        }
    }
    (*player)
        .powers[pw_shield as libc::c_int as usize] = SH_NONE as libc::c_int as uint16_t;
    (*(*player).mo).color = (*player).skincolor;
    (*player).powers[pw_emeralds as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
    P_ForceFeed(
        player,
        40 as libc::c_int,
        10 as libc::c_int,
        35 as libc::c_int as tic_t,
        40 as libc::c_int
            + (if damage < 100 as libc::c_int { damage } else { 100 as libc::c_int })
                * 2 as libc::c_int,
    );
    P_ResetPlayer(player);
    if (*player).spectator == 0 {
        (*(*player).mo).flags2 &= !(MF2_DONTDRAW as libc::c_int) as uint32_t;
    }
    P_SetPlayerMobjState((*player).mo, (*(*(*player).mo).info).deathstate);
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t != 0
        && (*player).gotflag as libc::c_int & (1 as libc::c_int | 2 as libc::c_int) != 0
    {
        P_PlayerFlagBurst(player, false_0 as libc::c_int);
        if !source.is_null() && !((*source).player).is_null() {
            if G_GametypeHasTeams() == 0
                || !((*(*source).player).ctfteam == (*player).ctfteam
                    && source != (*player).mo)
            {
                P_AddPlayerScore((*source).player, 25 as libc::c_int as uint32_t);
            }
        }
    }
    if !source.is_null() && !((*source).player).is_null()
        && (*player).powers[pw_super as libc::c_int as usize] == 0
    {
        if G_GametypeHasTeams() == 0
            || !((*(*source).player).ctfteam == (*player).ctfteam
                && source != (*player).mo)
        {
            P_AddPlayerScore((*source).player, 100 as libc::c_int as uint32_t);
        }
    }
    if G_CoopGametype() == 0
        && (*player).powers[pw_super as libc::c_int as usize] as libc::c_int != 0
    {
        S_StartSound(0 as *const libc::c_void, sfx_s3k66);
        HU_SetCEchoFlags(0 as libc::c_int);
        HU_SetCEchoDuration(5 as libc::c_int);
        HU_DoCEcho(
            va(
                b"%s\\is no longer super.\\\\\\\\\0" as *const u8 as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
            ),
        );
    }
}
unsafe extern "C" fn P_SuperDamage(
    mut player: *mut player_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
) {
    let mut fallbackspeed: fixed_t = 0;
    let mut ang: angle_t = 0;
    P_ForceFeed(
        player,
        40 as libc::c_int,
        10 as libc::c_int,
        35 as libc::c_int as tic_t,
        40 as libc::c_int
            + (if damage < 100 as libc::c_int { damage } else { 100 as libc::c_int })
                * 2 as libc::c_int,
    );
    if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
        (*(*player).mo).z -= 1;
        (*(*player).mo).z;
    } else {
        (*(*player).mo).z += 1;
        (*(*player).mo).z;
    }
    if (*(*player).mo).eflags as libc::c_int & MFE_UNDERWATER as libc::c_int != 0 {
        P_SetObjectMomZ(
            (*player).mo,
            FixedDiv(
                10511 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                2600 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            ),
            false_0 as libc::c_int,
        );
    } else {
        P_SetObjectMomZ(
            (*player).mo,
            FixedDiv(
                69 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                10 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            ),
            false_0 as libc::c_int,
        );
    }
    ang = R_PointToAngle2(
        (*inflictor).x,
        (*inflictor).y,
        (*(*player).mo).x,
        (*(*player).mo).y,
    );
    if (*inflictor).flags2 & MF2_SCATTER as libc::c_int as uint32_t != 0
        && !source.is_null()
    {
        let mut dist: fixed_t = P_AproxDistance(
            P_AproxDistance(
                (*source).x - (*(*player).mo).x,
                (*source).y - (*(*player).mo).y,
            ),
            (*source).z - (*(*player).mo).z,
        );
        dist = FixedMul(
            128 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*inflictor).scale,
        ) - dist / 4 as libc::c_int;
        if dist
            < FixedMul(
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*inflictor).scale,
            )
        {
            dist = FixedMul(
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*inflictor).scale,
            );
        }
        fallbackspeed = dist;
    } else if (*inflictor).flags2 & MF2_EXPLOSION as libc::c_int as uint32_t != 0 {
        if (*inflictor).flags2 & MF2_RAILRING as libc::c_int as uint32_t != 0 {
            fallbackspeed = FixedMul(
                28 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*inflictor).scale,
            );
        } else {
            fallbackspeed = FixedMul(
                20 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*inflictor).scale,
            );
        }
    } else if (*inflictor).flags2 & MF2_RAILRING as libc::c_int as uint32_t != 0 {
        fallbackspeed = FixedMul(
            16 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*inflictor).scale,
        );
    } else {
        fallbackspeed = FixedMul(
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*inflictor).scale,
        );
    }
    P_InstaThrust((*player).mo, ang, fallbackspeed);
    P_SetPlayerMobjState((*player).mo, S_PLAY_STUN);
    P_ResetPlayer(player);
    if (*player).timeshit as libc::c_int != 255 as libc::c_int {
        (*player).timeshit = ((*player).timeshit).wrapping_add(1);
        (*player).timeshit;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RemoveShield(mut player: *mut player_t) {
    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
        & SH_FORCE as libc::c_int != 0
    {
        if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_FORCEHP as libc::c_int != 0
        {
            (*player)
                .powers[pw_shield as libc::c_int
                as usize] = ((*player).powers[pw_shield as libc::c_int as usize])
                .wrapping_sub(1);
            (*player).powers[pw_shield as libc::c_int as usize];
        } else {
            (*player)
                .powers[pw_shield as libc::c_int
                as usize] = ((*player).powers[pw_shield as libc::c_int as usize]
                as libc::c_int & SH_STACK as libc::c_int) as uint16_t;
        }
    } else if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
        & SH_NOSTACK as libc::c_int != 0
    {
        if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_NOSTACK as libc::c_int == SH_ARMAGEDDON as libc::c_int
        {
            P_BlackOw(player);
            (*player)
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                (*player).pflags as libc::c_uint
                    | PF_JUMPDOWN as libc::c_int as libc::c_uint,
            );
        } else {
            (*player)
                .powers[pw_shield as libc::c_int
                as usize] = ((*player).powers[pw_shield as libc::c_int as usize]
                as libc::c_int & SH_STACK as libc::c_int) as uint16_t;
        }
    } else {
        if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_STACK as libc::c_int == SH_FIREFLOWER as libc::c_int
            && !((*player).powers[pw_super as libc::c_int as usize] as libc::c_int != 0
                || maptol & TOL_MARIO as libc::c_int as uint32_t != 0
                    && (*player).powers[pw_invulnerability as libc::c_int as usize]
                        as libc::c_int != 0)
        {
            (*(*player).mo).color = (*player).skincolor;
            G_GhostAddColor(GHC_NORMAL);
        }
        (*player)
            .powers[pw_shield as libc::c_int
            as usize] = SH_NONE as libc::c_int as uint16_t;
    };
}
unsafe extern "C" fn P_ShieldDamage(
    mut player: *mut player_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
    mut damagetype: uint8_t,
) {
    P_DoPlayerPain(player, source, inflictor);
    P_RemoveShield(player);
    P_ForceFeed(
        player,
        40 as libc::c_int,
        10 as libc::c_int,
        35 as libc::c_int as tic_t,
        40 as libc::c_int
            + (if damage < 100 as libc::c_int { damage } else { 100 as libc::c_int })
                * 2 as libc::c_int,
    );
    if damagetype as libc::c_int == 4 as libc::c_int {
        S_StartSound((*player).mo as *const libc::c_void, sfx_spkdth);
    } else {
        S_StartSound((*player).mo as *const libc::c_void, sfx_shldls);
    }
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t != 0
        && (*player).gotflag as libc::c_int & (1 as libc::c_int | 2 as libc::c_int) != 0
    {
        P_PlayerFlagBurst(player, false_0 as libc::c_int);
        if !source.is_null() && !((*source).player).is_null() {
            if G_GametypeHasTeams() == 0
                || !((*(*source).player).ctfteam == (*player).ctfteam
                    && source != (*player).mo)
            {
                P_AddPlayerScore((*source).player, 25 as libc::c_int as uint32_t);
            }
        }
    }
    if !source.is_null() && !((*source).player).is_null()
        && (*player).powers[pw_super as libc::c_int as usize] == 0
    {
        if G_GametypeHasTeams() == 0
            || !((*(*source).player).ctfteam == (*player).ctfteam
                && source != (*player).mo)
        {
            P_AddPlayerScore((*source).player, 50 as libc::c_int as uint32_t);
        }
    }
}
unsafe extern "C" fn P_RingDamage(
    mut player: *mut player_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
    mut damagetype: uint8_t,
    mut dospheres: boolean,
) {
    P_DoPlayerPain(player, source, inflictor);
    P_ForceFeed(
        player,
        40 as libc::c_int,
        10 as libc::c_int,
        35 as libc::c_int as tic_t,
        40 as libc::c_int
            + (if damage < 100 as libc::c_int { damage } else { 100 as libc::c_int })
                * 2 as libc::c_int,
    );
    if damagetype as libc::c_int == 4 as libc::c_int {
        S_StartSound((*player).mo as *const libc::c_void, sfx_spkdth);
    }
    if !source.is_null() && !((*source).player).is_null()
        && (*player).powers[pw_super as libc::c_int as usize] == 0
    {
        if G_GametypeHasTeams() == 0
            || !((*(*source).player).ctfteam == (*player).ctfteam
                && source != (*player).mo)
        {
            P_AddPlayerScore((*source).player, 50 as libc::c_int as uint32_t);
        }
    }
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t != 0
        && (*player).gotflag as libc::c_int & (1 as libc::c_int | 2 as libc::c_int) != 0
    {
        P_PlayerFlagBurst(player, false_0 as libc::c_int);
        if !source.is_null() && !((*source).player).is_null() {
            if G_GametypeHasTeams() == 0
                || !((*(*source).player).ctfteam == (*player).ctfteam
                    && source != (*player).mo)
            {
                P_AddPlayerScore((*source).player, 25 as libc::c_int as uint32_t);
            }
        }
    }
    S_StartSound(
        (*player).mo as *const libc::c_void,
        (if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
            sfx_mario8 as libc::c_int
        } else {
            sfx_altow1 as libc::c_int + P_RandomKey(4 as libc::c_int)
        }) as sfxenum_t,
    );
    P_PlayerRingBurst(player, damage);
    if dospheres != 0 {
        (*player).spheres = ((*player).spheres as libc::c_int - damage) as int16_t;
        if ((*player).spheres as libc::c_int) < 0 as libc::c_int {
            (*player).spheres = 0 as libc::c_int as int16_t;
        }
    } else {
        (*player).rings = ((*player).rings as libc::c_int - damage) as int16_t;
        if ((*player).rings as libc::c_int) < 0 as libc::c_int {
            (*player).rings = 0 as libc::c_int as int16_t;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_SpecialStageDamage(
    mut player: *mut player_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
) {
    let mut oldnightstime: tic_t = (*player).nightstime;
    if (*player).powers[pw_invulnerability as libc::c_int as usize] as libc::c_int != 0
        || (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int != 0
        || (*player).powers[pw_super as libc::c_int as usize] as libc::c_int != 0
    {
        return;
    }
    if cv_friendlyfire.value == 0 && !source.is_null() && !((*source).player).is_null() {
        if (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint
            && (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                & SH_NOSTACK as libc::c_int == 0
        {
            if (*player).revitem as libc::c_uint
                != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).spinitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
                && (*player).thokitem as libc::c_uint
                    != MT_LHRT as libc::c_int as libc::c_uint
            {
                P_SwitchShield(player, SH_PINK as libc::c_int as uint16_t);
                S_StartSound(
                    (*player).mo as *const libc::c_void,
                    mobjinfo[MT_PITY_ICON as libc::c_int as usize].seesound,
                );
            }
        }
        if (*(*source).player).ctfteam == (*player).ctfteam {
            return;
        }
    }
    if !inflictor.is_null()
        && (*inflictor).type_0 as libc::c_uint == MT_LHRT as libc::c_int as libc::c_uint
    {
        return;
    }
    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int != 0
        || (*player).bot as libc::c_int != 0
            && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
    {
        P_RemoveShield(player);
        S_StartSound((*player).mo as *const libc::c_void, sfx_shldls);
    } else {
        S_StartSound((*player).mo as *const libc::c_void, sfx_nghurt);
        if (*player).nightstime > (5 as libc::c_int * 35 as libc::c_int) as tic_t {
            (*player)
                .nightstime = ((*player).nightstime)
                .wrapping_sub((5 as libc::c_int * 35 as libc::c_int) as tic_t);
        } else {
            (*player).nightstime = 0 as libc::c_int as tic_t;
        }
    }
    P_DoPlayerPain(player, inflictor, source);
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t != 0
        && (*player).gotflag as libc::c_int & (1 as libc::c_int | 2 as libc::c_int) != 0
    {
        P_PlayerFlagBurst(player, false_0 as libc::c_int);
    }
    if oldnightstime > (10 as libc::c_int * 35 as libc::c_int) as tic_t
        && (*player).nightstime < (10 as libc::c_int * 35 as libc::c_int) as tic_t
    {
        if (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .levelflags as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0
        {
            S_FadeMusicFromVolume(
                0 as libc::c_int as uint8_t,
                -(1 as libc::c_int) as int16_t,
                (10 as libc::c_int * 1000 as libc::c_int) as uint32_t,
            );
            S_StartSound(0 as *const libc::c_void, sfx_timeup);
        } else {
            S_ChangeMusicEx(
                if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
                    && G_IsSpecialStage(gamemap as int32_t) == 0
                {
                    b"_ntime\0" as *const u8 as *const libc::c_char
                } else {
                    b"_drown\0" as *const u8 as *const libc::c_char
                },
                0 as libc::c_int as uint16_t,
                false_0 as libc::c_int,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
                0 as libc::c_int as uint32_t,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_DamageMobj(
    mut target: *mut mobj_t,
    mut inflictor: *mut mobj_t,
    mut source: *mut mobj_t,
    mut damage: int32_t,
    mut damagetype: uint8_t,
) -> boolean {
    let mut player: *mut player_t = 0 as *mut player_t;
    let mut force: boolean = false_0 as libc::c_int;
    if objectplacing != 0 {
        return false_0 as libc::c_int;
    }
    if (*target).health <= 0 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if multiplayer != 0 {
        if damagetype as libc::c_int != 0x80 as libc::c_int + 5 as libc::c_int
            && !((*target).player).is_null() && (*(*target).player).spectator != 0
        {
            return false_0 as libc::c_int;
        }
        if !source.is_null() && !((*source).player).is_null()
            && (*(*source).player).spectator != 0
        {
            return false_0 as libc::c_int;
        }
    }
    if metalrecording == 0 {
        let mut shouldForce: uint8_t = LUA_HookShouldDamage(
            target,
            inflictor,
            source,
            damage,
            damagetype,
        ) as uint8_t;
        if P_MobjWasRemoved(target) != 0 {
            return (shouldForce as libc::c_int == 1 as libc::c_int) as libc::c_int;
        }
        if P_MobjWasRemoved(source) != 0 {
            source = 0 as *mut mobj_t;
        }
        if shouldForce as libc::c_int == 1 as libc::c_int {
            force = true_0 as libc::c_int;
        } else if shouldForce as libc::c_int == 2 as libc::c_int {
            return false_0 as libc::c_int
        }
    }
    if force == 0 {
        if (*target).flags & MF_SHOOTABLE as libc::c_int as uint32_t == 0 {
            return false_0 as libc::c_int;
        }
        if (*target).type_0 as libc::c_uint
            == MT_BLACKEGGMAN as libc::c_int as libc::c_uint
        {
            return false_0 as libc::c_int;
        }
        if (*target).flags & MF_MONITOR as libc::c_int as uint32_t != 0
            && (source.is_null() || ((*source).player).is_null()
                || (*(*source).player).bot as libc::c_int != 0
                    && (*(*source).player).bot as libc::c_int != BOT_MPAI as libc::c_int
                || !inflictor.is_null()
                    && ((*inflictor).type_0 as libc::c_uint
                        == MT_REDRING as libc::c_int as libc::c_uint
                        || (*inflictor).type_0 as libc::c_uint
                            >= MT_THROWNBOUNCE as libc::c_int as libc::c_uint
                            && (*inflictor).type_0 as libc::c_uint
                                <= MT_THROWNGRENADE as libc::c_int as libc::c_uint))
        {
            return false_0 as libc::c_int;
        }
    }
    if (*target).flags2 & MF2_SKULLFLY as libc::c_int as uint32_t != 0 {
        (*target).momz = 0 as libc::c_int;
        (*target).momy = (*target).momz;
        (*target).momx = (*target).momy;
    }
    if force == 0 {
        if (*target).type_0 as libc::c_uint
            == MT_RING_REDBOX as libc::c_int as libc::c_uint
            && !((*(*source).player).ctfteam == 1 as libc::c_int)
        {
            return false_0 as libc::c_int;
        }
        if (*target).type_0 as libc::c_uint
            == MT_RING_BLUEBOX as libc::c_int as libc::c_uint
            && !((*(*source).player).ctfteam == 2 as libc::c_int)
        {
            return false_0 as libc::c_int;
        }
    }
    if (*target).flags & (MF_ENEMY as libc::c_int | MF_BOSS as libc::c_int) as uint32_t
        != 0
    {
        if force == 0 && (*target).flags2 & MF2_FRET as libc::c_int as uint32_t != 0 {
            return false_0 as libc::c_int;
        }
        if LUA_HookMobjDamage(target, inflictor, source, damage, damagetype) != 0
            || P_MobjWasRemoved(target) != 0
        {
            return true_0 as libc::c_int;
        }
        if (*target).health > 1 as libc::c_int {
            (*target).flags2 |= MF2_FRET as libc::c_int as uint32_t;
        }
    }
    player = (*target).player;
    if !player.is_null() {
        if force == 0 {
            if (*player).exiting != 0 {
                return false_0 as libc::c_int;
            }
            if (*player).pflags as libc::c_uint
                & PF_GODMODE as libc::c_int as libc::c_uint != 0
            {
                return false_0 as libc::c_int;
            }
            if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
                && (*(*target).player).powers[pw_carry as libc::c_int as usize]
                    as libc::c_int != CR_NIGHTSMODE as libc::c_int
                && (*(*target).player).powers[pw_carry as libc::c_int as usize]
                    as libc::c_int != CR_NIGHTSFALL as libc::c_int
            {
                return false_0 as libc::c_int;
            }
            match damagetype as libc::c_int {
                1 => {
                    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                        & SH_PROTECTWATER as libc::c_int != 0
                    {
                        return false_0 as libc::c_int;
                    }
                }
                2 => {
                    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                        & SH_PROTECTFIRE as libc::c_int != 0
                    {
                        return false_0 as libc::c_int;
                    }
                }
                3 => {
                    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                        & SH_PROTECTELECTRIC as libc::c_int != 0
                    {
                        return false_0 as libc::c_int;
                    }
                }
                4 => {
                    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                        & SH_PROTECTSPIKE as libc::c_int != 0
                    {
                        return false_0 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_NIGHTSMODE as libc::c_int
        {
            if force == 0 {
                if source == target {
                    return false_0 as libc::c_int;
                }
                if !source.is_null() && !((*source).player).is_null()
                    && !(cv_friendlyfire.value != 0
                        || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t
                            != 0)
                    && (gametyperules & GTR_FRIENDLY as libc::c_int as uint32_t != 0
                        || G_GametypeHasTeams() != 0
                            && (*player).ctfteam == (*(*source).player).ctfteam)
                {
                    return false_0 as libc::c_int;
                }
            }
            if LUA_HookMobjDamage(target, inflictor, source, damage, damagetype) != 0 {
                return true_0 as libc::c_int;
            }
            P_NiGHTSDamage(target, source);
            return true_0 as libc::c_int;
        }
        if G_IsSpecialStage(gamemap as int32_t) != 0
            && damagetype as libc::c_int & 0x80 as libc::c_int == 0
        {
            P_SpecialStageDamage(player, inflictor, source);
            return true_0 as libc::c_int;
        }
        if force == 0 && !inflictor.is_null()
            && (*inflictor).flags & MF_FIRE as libc::c_int as uint32_t != 0
            && !(damagetype as libc::c_int != 0
                && damagetype as libc::c_int != 2 as libc::c_int)
        {
            if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                & SH_PROTECTFIRE as libc::c_int != 0
            {
                return false_0 as libc::c_int;
            }
            if G_PlatformGametype() != 0 && !inflictor.is_null() && !source.is_null()
                && !((*source).player).is_null()
            {
                return false_0 as libc::c_int;
            }
        }
        if force == 0 && !source.is_null() && !((*source).player).is_null() {
            if P_PlayerHitsPlayer(target, inflictor, source, damage, damagetype) == 0 {
                return false_0 as libc::c_int;
            }
        }
        if damagetype as libc::c_int & 0x80 as libc::c_int != 0 {
            P_KillPlayer(player, source, damage);
        } else if metalrecording != 0 {
            if inflictor.is_null() {
                inflictor = source;
            }
            if !inflictor.is_null()
                && (*inflictor).flags & MF_ENEMY as libc::c_int as uint32_t != 0
            {
                P_KillMobj(inflictor, 0 as *mut mobj_t, target, damagetype);
                return false_0 as libc::c_int;
            } else if !inflictor.is_null()
                && (*inflictor).flags & MF_MISSILE as libc::c_int as uint32_t != 0
            {
                return false_0 as libc::c_int
            } else if (*player).powers[pw_flashing as libc::c_int as usize] == 0 {
                P_ShieldDamage(player, inflictor, source, damage, damagetype);
                return true_0 as libc::c_int;
            }
            return false_0 as libc::c_int;
        } else if (*player).powers[pw_invulnerability as libc::c_int as usize]
            as libc::c_int != 0
            || (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int != 0
            || (*player).powers[pw_super as libc::c_int as usize] as libc::c_int != 0
            || (*player).powers[pw_strong as libc::c_int as usize] as libc::c_int
                & STR_GUARD as libc::c_int != 0
        {
            if force != 0
                || !inflictor.is_null()
                    && (*inflictor).flags & MF_MISSILE as libc::c_int as uint32_t != 0
                    && (*inflictor).flags2 & MF2_SUPERFIRE as libc::c_int as uint32_t
                        != 0
            {
                if LUA_HookMobjDamage(target, inflictor, source, damage, damagetype) == 0
                {
                    P_SuperDamage(player, inflictor, source, damage);
                }
                return true_0 as libc::c_int;
            }
            return false_0 as libc::c_int;
        } else if LUA_HookMobjDamage(target, inflictor, source, damage, damagetype) != 0
        {
            return true_0 as libc::c_int
        } else if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int != 0
            || (*player).bot as libc::c_int != 0
                && (*player).bot as libc::c_int != BOT_MPAI as libc::c_int
                && ultimatemode == 0
        {
            P_ShieldDamage(player, inflictor, source, damage, damagetype);
            damage = 0 as libc::c_int;
        } else if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_NIGHTSFALL as libc::c_int
        {
            damage = (*player).spheres as int32_t;
            P_RingDamage(
                player,
                inflictor,
                source,
                damage,
                damagetype,
                true_0 as libc::c_int,
            );
            damage = 0 as libc::c_int;
        } else if (*player).rings as libc::c_int > 0 as libc::c_int {
            damage = (*player).rings as int32_t;
            P_RingDamage(
                player,
                inflictor,
                source,
                damage,
                damagetype,
                false_0 as libc::c_int,
            );
            damage = 0 as libc::c_int;
        } else if force == 0 && G_GametypeHasTeams() != 0 && !source.is_null()
            && !((*source).player).is_null()
            && (*(*source).player).ctfteam == (*player).ctfteam
            && (cv_friendlyfire.value != 0
                || gametyperules & GTR_FRIENDLYFIRE as libc::c_int as uint32_t != 0)
        {
            damage = 0 as libc::c_int;
            P_ShieldDamage(player, inflictor, source, damage, damagetype);
        } else {
            damage = 1 as libc::c_int;
            P_KillPlayer(player, source, damage);
        }
        P_ForceFeed(
            player,
            40 as libc::c_int,
            10 as libc::c_int,
            35 as libc::c_int as tic_t,
            40 as libc::c_int
                + (if damage < 100 as libc::c_int { damage } else { 100 as libc::c_int })
                    * 2 as libc::c_int,
        );
    }
    if cv_killingdead.value != 0 && (!source.is_null() && !((*source).player).is_null())
        && (!inflictor.is_null() && !((*inflictor).player).is_null())
        && P_RandomFixed()
            < 5 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                / 16 as libc::c_int
    {
        P_DamageMobj(
            source,
            target,
            target,
            1 as libc::c_int,
            0 as libc::c_int as uint8_t,
        );
    }
    if damagetype as libc::c_int & 0x80 as libc::c_int != 0 {
        (*target).health = 0 as libc::c_int;
    } else {
        (*target).health -= damage;
    }
    if !player.is_null() {
        P_HitDeathMessages(player, inflictor, source, damagetype);
    }
    if !source.is_null() && !((*source).player).is_null() && !target.is_null() {
        G_GhostAddHit(target);
    }
    if (*target).health <= 0 as libc::c_int {
        P_KillMobj(target, inflictor, source, damagetype);
        return true_0 as libc::c_int;
    }
    if !player.is_null() {
        P_ResetPlayer((*target).player);
    } else if (*target).type_0 as libc::c_uint
        == MT_EGGMOBILE2 as libc::c_int as libc::c_uint
        && (*target).health < (*(*target).info).damage
    {
        P_SetMobjState(target, (*(*target).info).meleestate);
    } else {
        P_SetMobjState(target, (*(*target).info).painstate);
    }
    if P_MobjWasRemoved(target) != 0 {
        return false_0 as libc::c_int;
    }
    if (*target).type_0 as libc::c_uint
        == MT_HIVEELEMENTAL as libc::c_int as libc::c_uint
    {
        (*target).extravalue1 += 3 as libc::c_int;
    }
    (*target).reactiontime = 0 as libc::c_int;
    if !source.is_null() && source != target {
        P_SetTarget2(&mut (*target).target, source);
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerRingBurst(
    mut player: *mut player_t,
    mut num_rings: int32_t,
) {
    let mut i: int32_t = 0;
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut fa: angle_t = 0;
    let mut va_0: angle_t = 0;
    let mut ns: fixed_t = 0;
    let mut z: fixed_t = 0;
    let mut nightsreplace: boolean = (maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
        && G_IsSpecialStage(gamemap as int32_t) == 0) as libc::c_int;
    if player.is_null() {
        return;
    }
    if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0
        && (*player).spheres as libc::c_int <= 0 as libc::c_int
        || maptol & TOL_NIGHTS as libc::c_int as uint32_t == 0
            && (*player).rings as libc::c_int <= 0 as libc::c_int
    {
        num_rings = 0 as libc::c_int;
    }
    if num_rings > 32 as libc::c_int
        && (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            != CR_NIGHTSFALL as libc::c_int
    {
        num_rings = 32 as libc::c_int;
    }
    if (*player).powers[pw_emeralds as libc::c_int as usize] != 0 {
        P_PlayerEmeraldBurst(player, false_0 as libc::c_int);
    }
    P_PlayerWeaponPanelOrAmmoBurst(player);
    if abs((*(*player).mo).momx) > (*(*player).mo).scale
        || abs((*(*player).mo).momy) > (*(*player).mo).scale
    {
        va_0 = R_PointToAngle2(
            (*(*player).mo).momx,
            (*(*player).mo).momy,
            0 as libc::c_int,
            0 as libc::c_int,
        ) >> 19 as libc::c_int;
    } else {
        va_0 = (*(*player).mo).angle >> 19 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < num_rings {
        let mut objType: int32_t = mobjinfo[MT_RING as libc::c_int as usize]
            .reactiontime;
        if maptol & TOL_MARIO as libc::c_int as uint32_t != 0 {
            objType = mobjinfo[MT_COIN as libc::c_int as usize].reactiontime;
        } else if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_NIGHTSFALL as libc::c_int
        {
            objType = mobjinfo[(if nightsreplace != 0 {
                    MT_NIGHTSCHIP as libc::c_int
                } else {
                    MT_BLUESPHERE as libc::c_int
                }) as usize]
                .reactiontime;
        }
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z += (*(*player).mo).height - mobjinfo[objType as usize].height;
        }
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, objType as mobjtype_t);
        (*mo).fuse = 8 as libc::c_int * 35 as libc::c_int;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add(va_0)
            .wrapping_sub(
                ((num_rings - 1 as libc::c_int) * 8192 as libc::c_int
                    / 32 as libc::c_int) as angle_t,
            ) & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_NIGHTSFALL as libc::c_int
        {
            ns = FixedMul(
                i * ((1 as libc::c_int) << 16 as libc::c_int) / 16 as libc::c_int
                    + 2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*mo).scale,
            );
            (*mo)
                .momx = FixedMul(
                *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
            if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
                || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
            {
                (*mo)
                    .momy = FixedMul(
                    finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                    ns,
                );
            }
            P_SetObjectMomZ(
                mo,
                8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                false_0 as libc::c_int,
            );
            (*mo).fuse = 20 as libc::c_int * 35 as libc::c_int;
            P_SetMobjState(
                mo,
                (if (*player).bonustime != 0 {
                    (*(*mo).info).raisestate as libc::c_uint
                } else {
                    (*(*mo).info).spawnstate as libc::c_uint
                }) as statenum_t,
            );
        } else {
            let mut momxy: fixed_t = 0;
            let mut momz: fixed_t = 0;
            if i > 15 as libc::c_int {
                momxy = 3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
                momz = 4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
            } else {
                momxy = 2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
                momz = 3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
            }
            ns = FixedMul(
                FixedMul(
                    momxy,
                    ((1 as libc::c_int) << 16 as libc::c_int)
                        + FixedDiv(
                            ((*player).losstime << 16 as libc::c_int) as fixed_t,
                            (10 as libc::c_int * 35 as libc::c_int) << 16 as libc::c_int,
                        ),
                ),
                (*mo).scale,
            );
            (*mo)
                .momx = FixedMul(
                *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
            if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
                || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
            {
                (*mo)
                    .momy = FixedMul(
                    finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                    ns,
                );
            }
            ns = FixedMul(
                momz,
                ((1 as libc::c_int) << 16 as libc::c_int)
                    + FixedDiv(
                        ((*player).losstime << 16 as libc::c_int) as fixed_t,
                        (10 as libc::c_int * 35 as libc::c_int) << 16 as libc::c_int,
                    ),
            );
            P_SetObjectMomZ(mo, ns, false_0 as libc::c_int);
            if i & 1 as libc::c_int != 0 {
                P_SetObjectMomZ(mo, ns, true_0 as libc::c_int);
            }
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).momz *= -(1 as libc::c_int);
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    }
    (*player)
        .losstime = ((*player).losstime)
        .wrapping_add((10 as libc::c_int * 35 as libc::c_int) as tic_t);
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerWeaponPanelBurst(mut player: *mut player_t) {
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut fa: angle_t = 0;
    let mut ns: fixed_t = 0;
    let mut i: int32_t = 0;
    let mut z: fixed_t = 0;
    let mut num_weapons: int32_t = M_CountBits(
        (*player).ringweapons as uint32_t,
        (7 as libc::c_int - 1 as libc::c_int) as uint8_t,
    ) as int32_t;
    let mut ammoamt: uint16_t = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while i < num_weapons {
        let mut weptype: mobjtype_t = MT_NULL;
        let mut power: powertype_t = pw_invulnerability;
        if (*player).ringweapons & RW_BOUNCE as libc::c_int != 0 {
            weptype = MT_BOUNCEPICKUP;
            (*player).ringweapons &= !(RW_BOUNCE as libc::c_int);
            power = pw_bouncering;
        } else if (*player).ringweapons & RW_RAIL as libc::c_int != 0 {
            weptype = MT_RAILPICKUP;
            (*player).ringweapons &= !(RW_RAIL as libc::c_int);
            power = pw_railring;
        } else if (*player).ringweapons & RW_AUTO as libc::c_int != 0 {
            weptype = MT_AUTOPICKUP;
            (*player).ringweapons &= !(RW_AUTO as libc::c_int);
            power = pw_automaticring;
        } else if (*player).ringweapons & RW_EXPLODE as libc::c_int != 0 {
            weptype = MT_EXPLODEPICKUP;
            (*player).ringweapons &= !(RW_EXPLODE as libc::c_int);
            power = pw_explosionring;
        } else if (*player).ringweapons & RW_SCATTER as libc::c_int != 0 {
            weptype = MT_SCATTERPICKUP;
            (*player).ringweapons &= !(RW_SCATTER as libc::c_int);
            power = pw_scatterring;
        } else if (*player).ringweapons & RW_GRENADE as libc::c_int != 0 {
            weptype = MT_GRENADEPICKUP;
            (*player).ringweapons &= !(RW_GRENADE as libc::c_int);
            power = pw_grenadering;
        }
        if !(weptype as u64 == 0) {
            if (*player).powers[power as usize] as libc::c_int
                >= mobjinfo[weptype as usize].reactiontime
            {
                ammoamt = mobjinfo[weptype as usize].reactiontime as uint16_t;
            } else {
                ammoamt = (*player).powers[power as usize];
            }
            (*player)
                .powers[power
                as usize] = ((*player).powers[power as usize] as libc::c_int
                - ammoamt as libc::c_int) as uint16_t;
            z = (*(*player).mo).z;
            if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int
                != 0
            {
                z += (*(*player).mo).height - mobjinfo[weptype as usize].height;
            }
            mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, weptype);
            (*mo).reactiontime = ammoamt as int32_t;
            (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
            (*mo).flags
                &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                    as uint32_t;
            P_SetTarget2(&mut (*mo).target, (*player).mo);
            (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
            (*mo).destscale = (*(*player).mo).scale;
            P_SetScale(mo, (*(*player).mo).scale);
            fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
                .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
                & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
            ns = FixedMul(
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*mo).scale,
            );
            (*mo)
                .momx = FixedMul(
                *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
            if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
                || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
            {
                (*mo)
                    .momy = FixedMul(
                    finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                    ns,
                );
            }
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                false_0 as libc::c_int,
            );
            if i & 1 as libc::c_int != 0 {
                P_SetObjectMomZ(
                    mo,
                    4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    true_0 as libc::c_int,
                );
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerWeaponAmmoBurst(mut player: *mut player_t) {
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut fa: angle_t = 0;
    let mut ns: fixed_t = 0;
    let mut i: int32_t = 0 as libc::c_int;
    let mut z: fixed_t = 0;
    let mut weptype: mobjtype_t = MT_NULL;
    let mut power: powertype_t = pw_invulnerability;
    while true_0 as libc::c_int != 0 {
        if (*player).powers[pw_bouncering as libc::c_int as usize] != 0 {
            weptype = MT_BOUNCERING;
            power = pw_bouncering;
        } else if (*player).powers[pw_railring as libc::c_int as usize] != 0 {
            weptype = MT_RAILRING;
            power = pw_railring;
        } else if (*player).powers[pw_infinityring as libc::c_int as usize] != 0 {
            weptype = MT_INFINITYRING;
            power = pw_infinityring;
        } else if (*player).powers[pw_automaticring as libc::c_int as usize] != 0 {
            weptype = MT_AUTOMATICRING;
            power = pw_automaticring;
        } else if (*player).powers[pw_explosionring as libc::c_int as usize] != 0 {
            weptype = MT_EXPLOSIONRING;
            power = pw_explosionring;
        } else if (*player).powers[pw_scatterring as libc::c_int as usize] != 0 {
            weptype = MT_SCATTERRING;
            power = pw_scatterring;
        } else {
            if !((*player).powers[pw_grenadering as libc::c_int as usize] != 0) {
                break;
            }
            weptype = MT_GRENADERING;
            power = pw_grenadering;
        }
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z += (*(*player).mo).height - mobjinfo[weptype as usize].height;
        }
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, weptype);
        (*mo).health = (*player).powers[power as usize] as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*player).powers[power as usize] = 0 as libc::c_int as uint16_t;
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            2 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*mo).scale,
        );
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerWeaponPanelOrAmmoBurst(mut player: *mut player_t) {
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut fa: angle_t = 0;
    let mut ns: fixed_t = 0;
    let mut i: int32_t = 0 as libc::c_int;
    let mut z: fixed_t = 0;
    if (*player).ringweapons & RW_BOUNCE as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_BOUNCE as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_BOUNCEPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_BOUNCEPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_bouncering as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_BOUNCERING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_BOUNCERING);
        (*mo)
            .health = (*player).powers[pw_bouncering as libc::c_int as usize] as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_bouncering as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & RW_RAIL as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_RAIL as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_RAILPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_RAILPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_railring as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_RAILRING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_RAILRING);
        (*mo).health = (*player).powers[pw_railring as libc::c_int as usize] as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_railring as libc::c_int as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & RW_AUTO as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_AUTO as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_AUTOPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_AUTOPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_automaticring as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_AUTOMATICRING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_AUTOMATICRING);
        (*mo)
            .health = (*player).powers[pw_automaticring as libc::c_int as usize]
            as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_automaticring as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & RW_EXPLODE as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_EXPLODE as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_EXPLODEPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_EXPLODEPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_explosionring as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_EXPLOSIONRING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_EXPLOSIONRING);
        (*mo)
            .health = (*player).powers[pw_explosionring as libc::c_int as usize]
            as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_explosionring as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & RW_SCATTER as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_SCATTER as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_SCATTERPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_SCATTERPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_scatterring as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_SCATTERRING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_SCATTERRING);
        (*mo)
            .health = (*player).powers[pw_scatterring as libc::c_int as usize]
            as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_scatterring as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & RW_GRENADE as libc::c_int != 0 {
        (*player).ringweapons &= !(RW_GRENADE as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_GRENADEPICKUP as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_GRENADEPICKUP);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_grenadering as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_GRENADERING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_GRENADERING);
        (*mo)
            .health = (*player).powers[pw_grenadering as libc::c_int as usize]
            as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_grenadering as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
    if (*player).ringweapons & 0 as libc::c_int != 0 {
        (*player).ringweapons &= !(0 as libc::c_int);
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z += (*(*player).mo).height - mobjinfo[0 as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_NULL);
        (*mo).reactiontime = 0 as libc::c_int;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        i += 1;
        i;
    } else if (*player).powers[pw_infinityring as libc::c_int as usize] as libc::c_int
        > 0 as libc::c_int
    {
        z = (*(*player).mo).z;
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            z
                += (*(*player).mo).height
                    - mobjinfo[MT_INFINITYRING as libc::c_int as usize].height;
        }
        fa = ((i * 8192 as libc::c_int / 16 as libc::c_int) as angle_t)
            .wrapping_add((*(*player).mo).angle >> 19 as libc::c_int)
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t;
        ns = FixedMul(
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            (*(*player).mo).scale,
        );
        mo = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, z, MT_INFINITYRING);
        (*mo)
            .health = (*player).powers[pw_infinityring as libc::c_int as usize]
            as int32_t;
        (*mo).flags2 |= MF2_DONTRESPAWN as libc::c_int as uint32_t;
        (*mo).flags
            &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                as uint32_t;
        P_SetTarget2(&mut (*mo).target, (*player).mo);
        (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
        (*mo).destscale = (*(*player).mo).scale;
        P_SetScale(mo, (*(*player).mo).scale);
        (*mo)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            ns,
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*mo)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                ns,
            );
        }
        P_SetObjectMomZ(
            mo,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            false_0 as libc::c_int,
        );
        if i & 1 as libc::c_int != 0 {
            P_SetObjectMomZ(
                mo,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                true_0 as libc::c_int,
            );
        }
        if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
            (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
        }
        (*player)
            .powers[pw_infinityring as libc::c_int
            as usize] = 0 as libc::c_int as uint16_t;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerEmeraldBurst(
    mut player: *mut player_t,
    mut toss: boolean,
) {
    let mut i: int32_t = 0;
    let mut fa: angle_t = 0;
    let mut ns: fixed_t = 0;
    let mut z: fixed_t = 0 as libc::c_int;
    let mut momx: fixed_t = 0 as libc::c_int;
    let mut momy: fixed_t = 0 as libc::c_int;
    if player.is_null() {
        return;
    }
    if (*player).powers[pw_emeralds as libc::c_int as usize] != 0 {
        let mut num_stones: int32_t = 0 as libc::c_int;
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 1 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 2 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 4 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 8 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 16 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 32 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
            & 64 as libc::c_int != 0
        {
            num_stones += 1;
            num_stones;
        }
        i = 0 as libc::c_int;
        while i < num_stones {
            let mut stoneflag: int32_t = 0 as libc::c_int;
            let mut statenum: statenum_t = S_CEMG1;
            let mut mo: *mut mobj_t = 0 as *mut mobj_t;
            if (*player).powers[pw_emeralds as libc::c_int as usize] as libc::c_int
                & 1 as libc::c_int != 0
            {
                stoneflag = 1 as libc::c_int;
                statenum = S_CEMG1;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 2 as libc::c_int != 0
            {
                stoneflag = 2 as libc::c_int;
                statenum = S_CEMG2;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 4 as libc::c_int != 0
            {
                stoneflag = 4 as libc::c_int;
                statenum = S_CEMG3;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 8 as libc::c_int != 0
            {
                stoneflag = 8 as libc::c_int;
                statenum = S_CEMG4;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 16 as libc::c_int != 0
            {
                stoneflag = 16 as libc::c_int;
                statenum = S_CEMG5;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 32 as libc::c_int != 0
            {
                stoneflag = 32 as libc::c_int;
                statenum = S_CEMG6;
            } else if (*player).powers[pw_emeralds as libc::c_int as usize]
                as libc::c_int & 64 as libc::c_int != 0
            {
                stoneflag = 64 as libc::c_int;
                statenum = S_CEMG7;
            }
            if !(stoneflag == 0) {
                (*player)
                    .powers[pw_emeralds as libc::c_int
                    as usize] = ((*player).powers[pw_emeralds as libc::c_int as usize]
                    as libc::c_int & !stoneflag) as uint16_t;
                if toss != 0 {
                    fa = (*(*player).mo).angle >> 19 as libc::c_int;
                    z = (*(*player).mo).z + (*(*player).mo).height;
                    if (*(*player).mo).eflags as libc::c_int
                        & MFE_VERTICALFLIP as libc::c_int != 0
                    {
                        z
                            -= mobjinfo[MT_FLINGEMERALD as libc::c_int as usize].height
                                + (*(*player).mo).height;
                    }
                    ns = FixedMul(
                        8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        (*(*player).mo).scale,
                    );
                } else {
                    fa = (255 as libc::c_int / num_stones * i * 8192 as libc::c_int
                        / 256 as libc::c_int) as angle_t;
                    z = (*(*player).mo).z + (*(*player).mo).height / 2 as libc::c_int;
                    if (*(*player).mo).eflags as libc::c_int
                        & MFE_VERTICALFLIP as libc::c_int != 0
                    {
                        z -= mobjinfo[MT_FLINGEMERALD as libc::c_int as usize].height;
                    }
                    ns = FixedMul(
                        4 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        (*(*player).mo).scale,
                    );
                }
                momx = FixedMul(
                    *finecosine.offset(fa as isize)
                        >> 16 as libc::c_int - 16 as libc::c_int,
                    ns,
                );
                if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
                    || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
                {
                    momy = FixedMul(
                        finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                        ns,
                    );
                } else {
                    momy = 0 as libc::c_int;
                }
                mo = P_SpawnMobj(
                    (*(*player).mo).x,
                    (*(*player).mo).y,
                    z,
                    MT_FLINGEMERALD,
                );
                (*mo).health = 1 as libc::c_int;
                (*mo).threshold = stoneflag;
                (*mo).flags2
                    |= (MF2_DONTRESPAWN as libc::c_int | MF2_SLIDEPUSH as libc::c_int)
                        as uint32_t;
                (*mo).flags
                    &= !(MF_NOGRAVITY as libc::c_int | MF_NOCLIPHEIGHT as libc::c_int)
                        as uint32_t;
                P_SetTarget2(&mut (*mo).target, (*player).mo);
                (*mo).fuse = 12 as libc::c_int * 35 as libc::c_int;
                P_SetMobjState(mo, statenum);
                (*mo).momx = momx;
                (*mo).momy = momy;
                P_SetObjectMomZ(
                    mo,
                    3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    false_0 as libc::c_int,
                );
                if (*(*player).mo).eflags as libc::c_int
                    & MFE_VERTICALFLIP as libc::c_int != 0
                {
                    (*mo).momz = -(*mo).momz;
                    (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
                }
                if toss != 0 {
                    (*player).tossdelay = 2 as libc::c_int * 35 as libc::c_int;
                }
            }
            i += 1;
            i;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerFlagBurst(
    mut player: *mut player_t,
    mut toss: boolean,
) {
    let mut flag: *mut mobj_t = 0 as *mut mobj_t;
    let mut type_0: mobjtype_t = MT_NULL;
    if (*player).gotflag as libc::c_int & (1 as libc::c_int | 2 as libc::c_int) == 0 {
        return;
    }
    if (*player).gotflag as libc::c_int & 1 as libc::c_int != 0 {
        type_0 = MT_REDFLAG;
    } else {
        type_0 = MT_BLUEFLAG;
    }
    flag = P_SpawnMobj((*(*player).mo).x, (*(*player).mo).y, (*(*player).mo).z, type_0);
    if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
        (*flag).z += (*(*player).mo).height - (*flag).height;
        (*flag).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
    }
    if toss != 0 {
        P_InstaThrust(
            flag,
            (*(*player).mo).angle,
            FixedMul(
                6 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*(*player).mo).scale,
            ),
        );
    } else {
        let mut fa: angle_t = (P_RandomByte() as libc::c_int * 8192 as libc::c_int
            / 256 as libc::c_int) as angle_t;
        (*flag)
            .momx = FixedMul(
            *finecosine.offset(fa as isize) >> 16 as libc::c_int - 16 as libc::c_int,
            FixedMul(
                6 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                (*(*player).mo).scale,
            ),
        );
        if !(maptol & TOL_2D as libc::c_int as uint32_t != 0
            || (*(*player).mo).flags2 & MF2_TWOD as libc::c_int as uint32_t != 0)
        {
            (*flag)
                .momy = FixedMul(
                finesine[fa as usize] >> 16 as libc::c_int - 16 as libc::c_int,
                FixedMul(
                    6 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    (*(*player).mo).scale,
                ),
            );
        }
    }
    (*flag)
        .momz = FixedMul(
        8 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        (*(*player).mo).scale,
    );
    if (*(*player).mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0 {
        (*flag).momz = -(*flag).momz;
    }
    if type_0 as libc::c_uint == MT_REDFLAG as libc::c_int as libc::c_uint {
        (*flag).spawnpoint = rflagpoint;
    } else {
        (*flag).spawnpoint = bflagpoint;
    }
    (*flag).fuse = cv_flagtime.value * 35 as libc::c_int;
    P_SetTarget2(&mut (*flag).target, (*player).mo);
    let mut plname: [libc::c_char; 25] = [0; 25];
    let mut flagtext: *const libc::c_char = 0 as *const libc::c_char;
    let mut flagcolor: libc::c_char = 0;
    snprintf(
        plname.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong,
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        if (*player).ctfteam != 0 {
            if (*player).ctfteam == 1 as libc::c_int {
                b"\x85\0" as *const u8 as *const libc::c_char
            } else {
                b"\x84\0" as *const u8 as *const libc::c_char
            }
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long as usize])
            .as_mut_ptr(),
        if (*player).ctfteam != 0 {
            b"\x80\0" as *const u8 as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    if type_0 as libc::c_uint == MT_REDFLAG as libc::c_int as libc::c_uint {
        flagtext = b"Red flag\0" as *const u8 as *const libc::c_char;
        flagcolor = -123i32 as libc::c_char;
    } else {
        flagtext = b"Blue flag\0" as *const u8 as *const libc::c_char;
        flagcolor = -124i32 as libc::c_char;
    }
    if toss != 0 {
        CONS_Printf(
            b"%s tossed the %c%s%c.\n\0" as *const u8 as *const libc::c_char,
            plname.as_mut_ptr(),
            flagcolor as libc::c_int,
            flagtext,
            0x80 as libc::c_int,
        );
    } else {
        CONS_Printf(
            b"%s dropped the %c%s%c.\n\0" as *const u8 as *const libc::c_char,
            plname.as_mut_ptr(),
            flagcolor as libc::c_int,
            flagtext,
            0x80 as libc::c_int,
        );
    }
    (*player).gotflag = 0 as libc::c_int as uint16_t;
    if type_0 as libc::c_uint == MT_REDFLAG as libc::c_int as libc::c_uint {
        redflag = flag;
    } else {
        blueflag = flag;
    }
    if toss != 0 {
        (*player).tossdelay = 2 as libc::c_int * 35 as libc::c_int;
    }
}
