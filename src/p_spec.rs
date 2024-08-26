use ::libc;
extern "C" {
    pub type visplane_s;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn get_number(word: *const libc::c_char) -> fixed_t;
    fn strncasecmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut numskincolors: uint16_t;
    static mut titlemapinaction: uint8_t;
    static mut botingame: boolean;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Debug(debugflags: int32_t, fmt: *const libc::c_char, _: ...);
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn M_GetToken(inputString: *const libc::c_char) -> *mut libc::c_char;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    fn sizeu2(num: size_t) -> *mut libc::c_char;
    static mut demoplayback: boolean;
    static mut metalplayback: *mut mobj_t;
    fn Tag_SectorFSet(id: size_t, tag: mtag_t);
    fn AngleFixed(af: angle_t) -> fixed_t;
    fn FixedAngle(fa: fixed_t) -> angle_t;
    fn A_Dye();
    static mut states: [state_t; 6735];
    static mut mobjinfo: [mobjinfo_t; 1163];
    fn P_SpawnPrecipitation();
    fn P_NullPrecipThinker(mobj: *mut precipmobj_t);
    fn P_RemovePrecipMobj(mobj: *mut precipmobj_t);
    static mut numstarposts: int32_t;
    static mut bossdisabled: uint16_t;
    static mut stoppedclock: boolean;
    static mut gamemap: int16_t;
    static mut mapmusname: [libc::c_char; 7];
    static mut mapmusflags: uint16_t;
    static mut mapmusposition: uint32_t;
    static mut maptol: uint32_t;
    static mut globalweather: uint8_t;
    static mut curWeather: int32_t;
    static mut metalrecording: boolean;
    static mut modeattacking: uint8_t;
    static mut netgame: boolean;
    static mut multiplayer: boolean;
    static mut gametyperules: uint32_t;
    static mut splitscreen: boolean;
    static mut circuitmap: boolean;
    static mut consoleplayer: int32_t;
    static mut displayplayer: int32_t;
    static mut secondarydisplayplayer: int32_t;
    static mut nextmapoverride: int16_t;
    static mut skipstats: uint8_t;
    static mut ssspheres: uint32_t;
    static mut rflagpoint: *mut mapthing_t;
    static mut bflagpoint: *mut mapthing_t;
    static mut quake: quake;
    static mut mapheaderinfo: [*mut mapheader_t; 1035];
    static mut stagefailed: boolean;
    static mut emeralds: uint16_t;
    static mut sstimer: int32_t;
    static mut bluescore: uint32_t;
    static mut redscore: uint32_t;
    static mut CheckForBustableBlocks: boolean;
    static mut CheckForBouncySector: boolean;
    static mut CheckForQuicksand: boolean;
    static mut CheckForMarioBlocks: boolean;
    static mut CheckForFloatBob: boolean;
    static mut CheckForReverseGravity: boolean;
    static mut flashingtics: uint16_t;
    static mut spacetimetics: uint16_t;
    static mut gravity: fixed_t;
    static mut numwaypoints: [uint16_t; 256];
    fn P_GetFirstWaypoint(sequence: uint8_t) -> *mut mobj_t;
    fn P_GetLastWaypoint(sequence: uint8_t) -> *mut mobj_t;
    fn P_GetPreviousWaypoint(current: *mut mobj_t, wrap: boolean) -> *mut mobj_t;
    fn P_GetNextWaypoint(current: *mut mobj_t, wrap: boolean) -> *mut mobj_t;
    fn P_GetClosestWaypoint(sequence: uint8_t, mo: *mut mobj_t) -> *mut mobj_t;
    fn COM_BufInsertTextEx(btext: *const libc::c_char, flags: com_flags_t);
    static mut cv_numlaps: consvar_t;
    static mut cv_runscripts: consvar_t;
    static mut cv_exitmove: consvar_t;
    fn Tag_SectorRemove(id: size_t, tag: mtag_t);
    fn Tag_SectorAdd(id: size_t, tag: mtag_t);
    fn Tag_Find(list: *const taglist_t, tag: mtag_t) -> boolean;
    fn Tag_FGet(list: *const taglist_t) -> mtag_t;
    fn Tag_FindLineSpecial(special: int16_t, tag: mtag_t) -> int32_t;
    fn Tag_Iterate_Things(tag: mtag_t, p: size_t) -> int32_t;
    fn Tag_Iterate_Lines(tag: mtag_t, p: size_t) -> int32_t;
    fn Tag_Iterate_Sectors(tag: mtag_t, p: size_t) -> int32_t;
    fn G_DoPlayMetal();
    static mut clientGamedata: *mut gamedata_t;
    static mut serverGamedata: *mut gamedata_t;
    static mut unlocktriggers: uint32_t;
    fn M_UpdateUnlockablesAndExtraEmblems(data: *mut gamedata_t) -> uint8_t;
    fn M_SilentUpdateUnlockablesAndEmblems(data: *mut gamedata_t);
    static mut player_names: [[libc::c_char; 22]; 32];
    static mut players: [player_t; 32];
    static mut playeringame: [boolean; 32];
    fn G_MovePlayerToSpawnOrStarpost(playernum: int32_t);
    fn G_SaveGameData(data: *mut gamedata_t);
    fn G_IsSpecialStage(mapnum: int32_t) -> boolean;
    fn G_PlatformGametype() -> boolean;
    fn G_CoopGametype() -> boolean;
    static mut thlist: [thinker_t; 0];
    fn P_AddThinker(n: thinklistnum_t, thinker: *mut thinker_t);
    fn P_RemoveThinker(thinker: *mut thinker_t);
    static mut camera: camera_t;
    static mut camera2: camera_t;
    fn P_AddPlayerScore(player: *mut player_t, amount: uint32_t);
    fn P_ResetCamera(player: *mut player_t, thiscam: *mut camera_t);
    fn P_ResetPlayer(player: *mut player_t);
    fn P_IsLocalPlayer(player: *mut player_t) -> boolean;
    fn P_SetPlayerAngle(player: *mut player_t, angle: angle_t);
    fn P_IsObjectOnGround(mo: *mut mobj_t) -> boolean;
    fn P_SetObjectMomZ(mo: *mut mobj_t, value: fixed_t, relative: boolean);
    fn P_SetPower(player: *mut player_t, power: powertype_t, value: uint16_t);
    fn P_GivePlayerRings(player: *mut player_t, num_rings: int32_t);
    fn P_DoPlayerFinish(player: *mut player_t);
    fn P_DoPlayerExit(player: *mut player_t);
    fn P_InstaThrust(mo: *mut mobj_t, angle: angle_t, move_0: fixed_t);
    fn P_FindLowestMare() -> uint8_t;
    fn P_SpawnMobj(
        x: fixed_t,
        y: fixed_t,
        z: fixed_t,
        type_0: mobjtype_t,
    ) -> *mut mobj_t;
    fn P_RecalcPrecipInSector(sector: *mut sector_t);
    fn P_MobjWasRemoved(th: *mut mobj_t) -> boolean;
    fn P_SetPlayerMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
    fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> boolean;
    fn P_MobjFloorZ(
        mobj: *mut mobj_t,
        sector: *mut sector_t,
        boundsec: *mut sector_t,
        x: fixed_t,
        y: fixed_t,
        line: *mut line_t,
        lowest: boolean,
        perfect: boolean,
    ) -> fixed_t;
    fn P_MobjCeilingZ(
        mobj: *mut mobj_t,
        sector: *mut sector_t,
        boundsec: *mut sector_t,
        x: fixed_t,
        y: fixed_t,
        line: *mut line_t,
        lowest: boolean,
        perfect: boolean,
    ) -> fixed_t;
    static mut var1: int32_t;
    static mut var2: int32_t;
    fn P_UnsetThingPosition(thing: *mut mobj_t);
    fn P_SetThingPosition(thing: *mut mobj_t);
    fn P_SetOrigin(thing: *mut mobj_t, x: fixed_t, y: fixed_t, z: fixed_t) -> boolean;
    fn P_SpecialStageDamage(
        player: *mut player_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
    );
    fn P_DamageMobj(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damage: int32_t,
        damagetype: uint8_t,
    ) -> boolean;
    fn P_KillMobj(
        target: *mut mobj_t,
        inflictor: *mut mobj_t,
        source: *mut mobj_t,
        damagetype: uint8_t,
    );
    fn P_TouchStarPost(
        starpost: *mut mobj_t,
        player: *mut player_t,
        snaptopost: boolean,
    );
    fn P_CheckTimeLimit();
    fn P_CheckPointLimit();
    fn P_ResetStarposts();
    fn P_ClosestPointOnLine3D(
        p: *const vector3_t,
        line: *const vector3_t,
        result: *mut vector3_t,
    );
    fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t;
    fn EV_DoPolyObjFade(_: *mut polyfadedata_t) -> boolean;
    fn EV_DoPolyObjFlag(_: *mut polyflagdata_t) -> boolean;
    fn EV_DoPolyObjRotDisplace(_: *mut polyrotdisplacedata_t) -> boolean;
    fn EV_DoPolyObjDisplace(_: *mut polydisplacedata_t) -> boolean;
    fn EV_DoPolyObjRotate(_: *mut polyrotdata_t) -> boolean;
    fn EV_DoPolyObjWaypoint(_: *mut polywaypointdata_t) -> boolean;
    fn EV_DoPolyObjMove(_: *mut polymovedata_t) -> boolean;
    fn EV_DoPolyDoor(_: *mut polydoordata_t) -> boolean;
    fn T_PolyObjFade(_: *mut polyfade_t);
    fn P_MobjInsidePolyobj(po: *mut polyobj_t, mo: *mut mobj_t) -> boolean;
    fn P_MobjTouchingPolyobj(po: *mut polyobj_t, mo: *mut mobj_t) -> boolean;
    fn Polyobj_InitLevel();
    fn Polyobj_GetForNum(id: int32_t) -> *mut polyobj_t;
    fn P_SetTarget2(mo: *mut *mut mobj_t, target: *mut mobj_t) -> *mut mobj_t;
    fn P_RemoveThinkerDelayed(thinker: *mut thinker_t);
    fn P_DoSuperTransformation(player: *mut player_t, giverings: boolean);
    fn P_RemoveLighting(sector: *mut sector_t);
    fn P_FadeLightBySector(
        sector: *mut sector_t,
        destvalue: int32_t,
        speed: int32_t,
        ticbased: boolean,
    );
    fn EV_StartCrumble(
        sector: *mut sector_t,
        rover: *mut ffloor_t,
        floating: boolean,
        player: *mut player_t,
        origalpha: fixed_t,
        crumblereturn: boolean,
    ) -> int32_t;
    fn EV_DoContinuousFall(
        sec: *mut sector_t,
        backsector: *mut sector_t,
        spd: fixed_t,
        backwards: boolean,
    );
    fn EV_CrumbleChain(sec: *mut sector_t, rover: *mut ffloor_t);
    fn EV_DoCrush(tag: mtag_t, line: *mut line_t, type_0: ceiling_e) -> int32_t;
    fn EV_DoFloor(tag: mtag_t, line: *mut line_t, floortype: floor_e);
    fn EV_DoElevator(tag: mtag_t, line: *mut line_t, elevtype: elevator_e);
    static mut leveltime: tic_t;
    fn P_FadeLight(
        tag: int16_t,
        destvalue: int32_t,
        speed: int32_t,
        ticbased: boolean,
        force: boolean,
        relative: boolean,
    );
    fn P_SpawnAdjustableStrobeFlash(
        sector: *mut sector_t,
        lighta: int16_t,
        lightb: int16_t,
        darktime: int32_t,
        brighttime: int32_t,
        inSync: boolean,
    ) -> *mut strobe_t;
    fn P_SpawnAdjustableGlowingLight(
        sector: *mut sector_t,
        lighta: int16_t,
        lightb: int16_t,
        length: int32_t,
    ) -> *mut glow_t;
    fn P_SpawnAdjustableFireFlicker(
        sector: *mut sector_t,
        lighta: int16_t,
        lightb: int16_t,
        length: int32_t,
    ) -> *mut fireflicker_t;
    fn P_Teleport(
        thing: *mut mobj_t,
        x: fixed_t,
        y: fixed_t,
        z: fixed_t,
        angle: angle_t,
        flash: boolean,
        dontstopmove: boolean,
    ) -> boolean;
    fn EV_DoCeiling(tag: mtag_t, line: *mut line_t, type_0: ceiling_e) -> int32_t;
    fn T_CameraScanner(elevator: *mut elevator_t);
    fn T_FloatSector(floater: *mut floatthink_t);
    fn T_MarioBlockChecker(block: *mut mariocheck_t);
    fn T_ThwompSector(thwomp: *mut thwomp_t);
    fn T_RaiseSector(raise: *mut raise_t);
    fn T_NoEnemiesSector(nobaddies: *mut noenemies_t);
    fn T_EachTimeThinker(eachtime: *mut eachtime_t);
    fn T_PlaneDisplace(pd: *mut planedisplace_t);
    static mut numlevelflats: size_t;
    static mut levelflats: *mut levelflat_t;
    static mut mapthings: *mut mapthing_t;
    fn P_SetupLevelSky(skynum: int32_t, global: boolean);
    static mut texturetranslation: *mut int32_t;
    static mut udmf: boolean;
    static mut vertexes: *mut vertex_t;
    static mut numsectors: size_t;
    static mut sectors: *mut sector_t;
    static mut numlines: size_t;
    static mut lines: *mut line_t;
    fn R_CreateDefaultColormap(lighttable: boolean) -> *mut extracolormap_t;
    fn R_GetColormapFromListByValues(
        rgba: int32_t,
        fadergba: int32_t,
        fadestart: uint8_t,
        fadeend: uint8_t,
        flags: uint8_t,
    ) -> *mut extracolormap_t;
    fn R_CreateLightTable(extra_colormap: *mut extracolormap_t) -> *mut lighttable_t;
    fn R_GetDefaultColormap() -> *mut extracolormap_t;
    fn R_AddColormapToList(extra_colormap: *mut extracolormap_t);
    static mut sides: *mut side_t;
    fn R_CheckDefaultColormap(
        extra_colormap: *mut extracolormap_t,
        checkrgba: boolean,
        checkfadergba: boolean,
        checkparams: boolean,
    ) -> boolean;
    fn R_CheckEqualColormaps(
        exc_a: *mut extracolormap_t,
        exc_b: *mut extracolormap_t,
        checkrgba: boolean,
        checkfadergba: boolean,
        checkparams: boolean,
    ) -> boolean;
    fn R_GetColormapFromList(
        extra_colormap: *mut extracolormap_t,
    ) -> *mut extracolormap_t;
    fn R_CopyColormap(
        extra_colormap: *mut extracolormap_t,
        lighttable: boolean,
    ) -> *mut extracolormap_t;
    fn R_AddColormaps(
        exc_augend: *mut extracolormap_t,
        exc_addend: *mut extracolormap_t,
        subR: boolean,
        subG: boolean,
        subB: boolean,
        subA: boolean,
        subFadeR: boolean,
        subFadeG: boolean,
        subFadeB: boolean,
        subFadeA: boolean,
        subFadeStart: boolean,
        subFadeEnd: boolean,
        ignoreFlags: boolean,
        lighttable: boolean,
    ) -> *mut extracolormap_t;
    fn R_CreateInterpolator_SectorPlane(
        thinker: *mut thinker_t,
        sector: *mut sector_t,
        ceiling: boolean,
    );
    fn R_CreateInterpolator_SectorScroll(
        thinker: *mut thinker_t,
        sector: *mut sector_t,
        ceiling: boolean,
    );
    fn R_CreateInterpolator_SideScroll(thinker: *mut thinker_t, side: *mut side_t);
    fn R_ClearTextureNumCache(btell: boolean);
    fn R_TextureNumForName(name: *const libc::c_char) -> int32_t;
    fn R_CheckTextureNumForName(name: *const libc::c_char) -> int32_t;
    fn R_GetFlatNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn M_RandomByte() -> uint8_t;
    fn P_RandomRange(a: int32_t, b: int32_t) -> int32_t;
    fn S_StartSound(origin: *const libc::c_void, sound_id: sfxenum_t);
    fn S_MusicName() -> *const libc::c_char;
    fn S_GetMusicLength() -> uint32_t;
    fn S_FadeMusicFromVolume(
        target_volume: uint8_t,
        source_volume: int16_t,
        ms: uint32_t,
    ) -> boolean;
    fn S_SetInternalMusicVolume(volume: int32_t);
    fn S_ChangeMusicEx(
        mmusic: *const libc::c_char,
        mflags: uint16_t,
        looping: boolean,
        position: uint32_t,
        prefadems: uint32_t,
        fadeinms: uint32_t,
    );
    fn S_SetMusicPosition(position: uint32_t) -> boolean;
    fn S_GetMusicPosition() -> uint32_t;
    fn S_GetMusicLoopPoint() -> uint32_t;
    static mut numwadfiles: uint16_t;
    fn W_CheckNumForNamePwad(
        name: *const libc::c_char,
        wad: uint16_t,
        startlump: uint16_t,
    ) -> uint16_t;
    fn W_CheckNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn W_LumpLengthPwad(wad: uint16_t, lump: uint16_t) -> size_t;
    fn W_LumpLength(lumpnum: lumpnum_t) -> size_t;
    fn W_CacheLumpNumPwad(
        wad: uint16_t,
        lump: uint16_t,
        tag: int32_t,
    ) -> *mut libc::c_void;
    fn W_CacheLumpNum(lump: lumpnum_t, tag: int32_t) -> *mut libc::c_void;
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
    fn R_PointToAngle2(
        px2: fixed_t,
        py2: fixed_t,
        px1: fixed_t,
        py1: fixed_t,
    ) -> angle_t;
    fn R_PointToDist2(px2: fixed_t, py2: fixed_t, px1: fixed_t, py1: fixed_t) -> fixed_t;
    fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t;
    fn P_GetFFloorTopZAt(ffloor: *const ffloor_t, x: fixed_t, y: fixed_t) -> fixed_t;
    fn P_GetFFloorBottomZAt(ffloor: *const ffloor_t, x: fixed_t, y: fixed_t) -> fixed_t;
    fn HU_SetCEchoDuration(seconds: int32_t);
    fn HU_SetCEchoFlags(flags: int32_t);
    fn HU_DoCEcho(msg: *const libc::c_char);
    fn LUA_HookLinedefExecute(_: *mut line_t, _: *mut mobj_t, _: *mut sector_t);
    fn F_StartTextPrompt(
        promptnum: int32_t,
        pagenum: int32_t,
        mo: *mut mobj_t,
        postexectag: uint16_t,
        blockcontrols: boolean,
        freezerealtime: boolean,
    );
    fn F_GetPromptPageByNamedTag(
        tag: *const libc::c_char,
        promptnum: *mut int32_t,
        pagenum: *mut int32_t,
    );
    fn F_EndTextPrompt(forceexec: boolean, noexec: boolean);
    static mut titlemapcameraref: *mut mobj_t;
    static mut skins: [skin_t; 32];
    fn __errno_location() -> *mut libc::c_int;
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
pub type C2RustUnnamed_1 = libc::c_int;
pub const LE_PARAMWIDTH: C2RustUnnamed_1 = -100;
pub const LE_AXE: C2RustUnnamed_1 = 649;
pub const LE_KOOPA: C2RustUnnamed_1 = 650;
pub const LE_CAPSULE0: C2RustUnnamed_1 = 680;
pub const LE_CAPSULE1: C2RustUnnamed_1 = 681;
pub const LE_CAPSULE2: C2RustUnnamed_1 = 682;
pub const LE_BRAKPLATFORM: C2RustUnnamed_1 = 4200;
pub const LE_TURRET: C2RustUnnamed_1 = 32000;
pub const LE_BRAKVILEATACK: C2RustUnnamed_1 = -6;
pub const LE_BOSS4DROP: C2RustUnnamed_1 = -5;
pub const LE_BOSSDEAD: C2RustUnnamed_1 = -4;
pub const LE_ALLBOSSESDEAD: C2RustUnnamed_1 = -3;
pub const LE_PINCHPHASE: C2RustUnnamed_1 = -2;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NUMTRANSMAPS: C2RustUnnamed_2 = 10;
pub const tr_trans90: C2RustUnnamed_2 = 9;
pub const tr_trans80: C2RustUnnamed_2 = 8;
pub const tr_trans70: C2RustUnnamed_2 = 7;
pub const tr_trans60: C2RustUnnamed_2 = 6;
pub const tr_trans50: C2RustUnnamed_2 = 5;
pub const tr_trans40: C2RustUnnamed_2 = 4;
pub const tr_trans30: C2RustUnnamed_2 = 3;
pub const tr_trans20: C2RustUnnamed_2 = 2;
pub const tr_trans10: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MF_RUNSPAWNFUNC: C2RustUnnamed_3 = 536870912;
pub const MF_GRENADEBOUNCE: C2RustUnnamed_3 = 268435456;
pub const MF_NOCLIPTHING: C2RustUnnamed_3 = 134217728;
pub const MF_NIGHTSITEM: C2RustUnnamed_3 = 67108864;
pub const MF_STICKY: C2RustUnnamed_3 = 33554432;
pub const MF_PAIN: C2RustUnnamed_3 = 16777216;
pub const MF_SCENERY: C2RustUnnamed_3 = 8388608;
pub const MF_ENEMY: C2RustUnnamed_3 = 4194304;
pub const MF_NOCLIPHEIGHT: C2RustUnnamed_3 = 2097152;
pub const MF_FIRE: C2RustUnnamed_3 = 1048576;
pub const MF_NOTHINK: C2RustUnnamed_3 = 524288;
pub const MF_MONITOR: C2RustUnnamed_3 = 262144;
pub const MF_BOUNCE: C2RustUnnamed_3 = 131072;
pub const MF_SPRING: C2RustUnnamed_3 = 65536;
pub const MF_MISSILE: C2RustUnnamed_3 = 32768;
pub const MF_BOXICON: C2RustUnnamed_3 = 16384;
pub const MF_FLOAT: C2RustUnnamed_3 = 8192;
pub const MF_NOCLIP: C2RustUnnamed_3 = 4096;
pub const MF_SLIDEME: C2RustUnnamed_3 = 2048;
pub const MF_AMBIENT: C2RustUnnamed_3 = 1024;
pub const MF_NOGRAVITY: C2RustUnnamed_3 = 512;
pub const MF_SPAWNCEILING: C2RustUnnamed_3 = 256;
pub const MF_BOSS: C2RustUnnamed_3 = 128;
pub const MF_PUSHABLE: C2RustUnnamed_3 = 64;
pub const MF_PAPERCOLLISION: C2RustUnnamed_3 = 32;
pub const MF_NOBLOCKMAP: C2RustUnnamed_3 = 16;
pub const MF_NOSECTOR: C2RustUnnamed_3 = 8;
pub const MF_SHOOTABLE: C2RustUnnamed_3 = 4;
pub const MF_SOLID: C2RustUnnamed_3 = 2;
pub const MF_SPECIAL: C2RustUnnamed_3 = 1;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const MF2_SPLAT: C2RustUnnamed_4 = 1073741824;
pub const MF2_SHIELD: C2RustUnnamed_4 = 536870912;
pub const MF2_LINKDRAW: C2RustUnnamed_4 = 268435456;
pub const MF2_AMBUSH: C2RustUnnamed_4 = 134217728;
pub const MF2_BOSSDEAD: C2RustUnnamed_4 = 67108864;
pub const MF2_BOSSFLEE: C2RustUnnamed_4 = 33554432;
pub const MF2_BOSSNOTRAP: C2RustUnnamed_4 = 16777216;
pub const MF2_FRET: C2RustUnnamed_4 = 8388608;
pub const MF2_SKULLFLY: C2RustUnnamed_4 = 4194304;
pub const MF2_OBJECTFLIP: C2RustUnnamed_4 = 2097152;
pub const MF2_STRONGBOX: C2RustUnnamed_4 = 1048576;
pub const MF2_SHADOW: C2RustUnnamed_4 = 524288;
pub const MF2_SUPERFIRE: C2RustUnnamed_4 = 262144;
pub const MF2_FIRING: C2RustUnnamed_4 = 131072;
pub const MF2_JUSTATTACKED: C2RustUnnamed_4 = 65536;
pub const MF2_NIGHTSPULL: C2RustUnnamed_4 = 32768;
pub const MF2_DEBRIS: C2RustUnnamed_4 = 16384;
pub const MF2_INFLOAT: C2RustUnnamed_4 = 8192;
pub const MF2_INVERTAIMABLE: C2RustUnnamed_4 = 4096;
pub const MF2_CLASSICPUSH: C2RustUnnamed_4 = 2048;
pub const MF2_SLIDEPUSH: C2RustUnnamed_4 = 1024;
pub const MF2_BEYONDTHEGRAVE: C2RustUnnamed_4 = 512;
pub const MF2_SCATTER: C2RustUnnamed_4 = 256;
pub const MF2_EXPLOSION: C2RustUnnamed_4 = 128;
pub const MF2_BOUNCERING: C2RustUnnamed_4 = 64;
pub const MF2_RAILRING: C2RustUnnamed_4 = 32;
pub const MF2_AUTOMATIC: C2RustUnnamed_4 = 16;
pub const MF2_DONTDRAW: C2RustUnnamed_4 = 8;
pub const MF2_DONTRESPAWN: C2RustUnnamed_4 = 4;
pub const MF2_TWOD: C2RustUnnamed_4 = 2;
pub const MF2_AXIS: C2RustUnnamed_4 = 1;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const MFE_REVERSESUPER: C2RustUnnamed_5 = 12288;
pub const MFE_FORCENOSUPER: C2RustUnnamed_5 = 8192;
pub const MFE_FORCESUPER: C2RustUnnamed_5 = 4096;
pub const MFE_TRACERANGLE: C2RustUnnamed_5 = 2048;
pub const MFE_APPLYPMOMZ: C2RustUnnamed_5 = 1024;
pub const MFE_SPRUNG: C2RustUnnamed_5 = 512;
pub const MFE_PUSHED: C2RustUnnamed_5 = 256;
pub const MFE_TOUCHLAVA: C2RustUnnamed_5 = 128;
pub const MFE_GOOWATER: C2RustUnnamed_5 = 64;
pub const MFE_VERTICALFLIP: C2RustUnnamed_5 = 32;
pub const MFE_JUSTSTEPPEDDOWN: C2RustUnnamed_5 = 16;
pub const MFE_UNDERWATER: C2RustUnnamed_5 = 8;
pub const MFE_TOUCHWATER: C2RustUnnamed_5 = 4;
pub const MFE_JUSTHITFLOOR: C2RustUnnamed_5 = 2;
pub const MFE_ONGROUND: C2RustUnnamed_5 = 1;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const PCF_THUNK: C2RustUnnamed_6 = 32;
pub const PCF_RAIN: C2RustUnnamed_6 = 16;
pub const PCF_MOVINGFOF: C2RustUnnamed_6 = 8;
pub const PCF_FOF: C2RustUnnamed_6 = 4;
pub const PCF_PIT: C2RustUnnamed_6 = 2;
pub const PCF_INVISIBLE: C2RustUnnamed_6 = 1;
pub type C2RustUnnamed_7 = libc::c_uint;
pub const BT_CUSTOM3: C2RustUnnamed_7 = 32768;
pub const BT_CUSTOM2: C2RustUnnamed_7 = 16384;
pub const BT_CUSTOM1: C2RustUnnamed_7 = 8192;
pub const BT_FIRENORMAL: C2RustUnnamed_7 = 4096;
pub const BT_JUMP: C2RustUnnamed_7 = 2048;
pub const BT_TOSSFLAG: C2RustUnnamed_7 = 1024;
pub const BT_CAMRIGHT: C2RustUnnamed_7 = 512;
pub const BT_CAMLEFT: C2RustUnnamed_7 = 256;
pub const BT_SPIN: C2RustUnnamed_7 = 128;
pub const BT_ATTACK: C2RustUnnamed_7 = 64;
pub const BT_WEAPONPREV: C2RustUnnamed_7 = 32;
pub const BT_WEAPONNEXT: C2RustUnnamed_7 = 16;
pub const BT_WEAPONMASK: C2RustUnnamed_7 = 15;
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
pub type player_t = player_s;
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
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
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
    pub revert: C2RustUnnamed_10,
    pub netid: uint16_t,
    pub changed: libc::c_char,
    pub next: *mut consvar_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub allocated: libc::c_char,
    pub v: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub string: *mut libc::c_char,
    pub const_munge: *const libc::c_char,
}
pub type consvar_t = consvar_s;
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
pub type C2RustUnnamed_12 = libc::c_uint;
pub const CS_SIMPLE: C2RustUnnamed_12 = 3;
pub const CS_STANDARD: C2RustUnnamed_12 = 2;
pub const CS_LMAOGALOG: C2RustUnnamed_12 = 1;
pub const CS_LEGACY: C2RustUnnamed_12 = 0;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const POF_SPLAT: C2RustUnnamed_13 = 8192;
pub const POF_NOSPECIALS: C2RustUnnamed_13 = 4096;
pub const POF_ONESIDE: C2RustUnnamed_13 = 2048;
pub const POF_LDEXEC: C2RustUnnamed_13 = 1024;
pub const POF_PUSHABLESTOP: C2RustUnnamed_13 = 512;
pub const POF_INVERTPLANESONLY: C2RustUnnamed_13 = 256;
pub const POF_INVERTPLANES: C2RustUnnamed_13 = 128;
pub const POF_INVERT: C2RustUnnamed_13 = 64;
pub const POF_RENDERALL: C2RustUnnamed_13 = 56;
pub const POF_RENDERPLANES: C2RustUnnamed_13 = 48;
pub const POF_RENDERBOTTOM: C2RustUnnamed_13 = 32;
pub const POF_RENDERTOP: C2RustUnnamed_13 = 16;
pub const POF_RENDERSIDES: C2RustUnnamed_13 = 8;
pub const POF_TESTHEIGHT: C2RustUnnamed_13 = 4;
pub const POF_SOLID: C2RustUnnamed_13 = 3;
pub const POF_CLIPPLANES: C2RustUnnamed_13 = 2;
pub const POF_CLIPLINES: C2RustUnnamed_13 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyfade_s {
    pub thinker: thinker_t,
    pub polyObjNum: int32_t,
    pub sourcevalue: int32_t,
    pub destvalue: int32_t,
    pub docollision: boolean,
    pub doghostfade: boolean,
    pub ticbased: boolean,
    pub duration: int32_t,
    pub timer: int32_t,
}
pub type polyfade_t = polyfade_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyrotdata_s {
    pub polyObjNum: int32_t,
    pub direction: int32_t,
    pub speed: int32_t,
    pub distance: int32_t,
    pub flags: uint8_t,
}
pub type polyrotdata_t = polyrotdata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polymovedata_s {
    pub polyObjNum: int32_t,
    pub distance: fixed_t,
    pub speed: fixed_t,
    pub angle: angle_t,
    pub overRide: uint8_t,
}
pub type polymovedata_t = polymovedata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polywaypointdata_s {
    pub polyObjNum: int32_t,
    pub sequence: int32_t,
    pub speed: fixed_t,
    pub returnbehavior: uint8_t,
    pub flags: uint8_t,
}
pub type polywaypointdata_t = polywaypointdata_s;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const TMPV_INVISIBLE: C2RustUnnamed_14 = 2;
pub const TMPV_VISIBLE: C2RustUnnamed_14 = 1;
pub const TMPV_NOCHANGE: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const TMPT_INTANGIBLE: C2RustUnnamed_15 = 2;
pub const TMPT_TANGIBLE: C2RustUnnamed_15 = 1;
pub const TMPT_NOCHANGE: C2RustUnnamed_15 = 0;
pub type C2RustUnnamed_16 = libc::c_uint;
pub const POLY_DOOR_SWING: C2RustUnnamed_16 = 1;
pub const POLY_DOOR_SLIDE: C2RustUnnamed_16 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polydoordata_s {
    pub polyObjNum: int32_t,
    pub doorType: int32_t,
    pub speed: int32_t,
    pub angle: angle_t,
    pub distance: int32_t,
    pub delay: int32_t,
}
pub type polydoordata_t = polydoordata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polydisplacedata_s {
    pub polyObjNum: int32_t,
    pub controlSector: *mut sector_s,
    pub dx: fixed_t,
    pub dy: fixed_t,
}
pub type polydisplacedata_t = polydisplacedata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyrotdisplacedata_s {
    pub polyObjNum: int32_t,
    pub controlSector: *mut sector_s,
    pub rotscale: fixed_t,
    pub turnobjs: uint8_t,
}
pub type polyrotdisplacedata_t = polyrotdisplacedata_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyflagdata_s {
    pub polyObjNum: int32_t,
    pub speed: int32_t,
    pub angle: uint32_t,
    pub momx: fixed_t,
}
pub type polyflagdata_t = polyflagdata_s;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const TMPF_GHOSTFADE: C2RustUnnamed_17 = 16;
pub const TMPF_IGNORECOLLISION: C2RustUnnamed_17 = 8;
pub const TMPF_TICBASED: C2RustUnnamed_17 = 4;
pub const TMPF_OVERRIDE: C2RustUnnamed_17 = 2;
pub const TMPF_RELATIVE: C2RustUnnamed_17 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polyfadedata_s {
    pub polyObjNum: int32_t,
    pub destvalue: int32_t,
    pub docollision: boolean,
    pub doghostfade: boolean,
    pub ticbased: boolean,
    pub speed: int32_t,
}
pub type polyfadedata_t = polyfadedata_s;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const BT_STRONG: C2RustUnnamed_18 = 3;
pub const BT_REGULAR: C2RustUnnamed_18 = 2;
pub const BT_SPINBUST: C2RustUnnamed_18 = 1;
pub const BT_TOUCH: C2RustUnnamed_18 = 0;
pub type C2RustUnnamed_19 = libc::c_uint;
pub const SD_SPECIALSTAGE: C2RustUnnamed_19 = 10;
pub const SD_INSTAKILL: C2RustUnnamed_19 = 9;
pub const SD_DEATHPITNOTILT: C2RustUnnamed_19 = 8;
pub const SD_DEATHPITTILT: C2RustUnnamed_19 = 7;
pub const SD_SPIKE: C2RustUnnamed_19 = 6;
pub const SD_ELECTRIC: C2RustUnnamed_19 = 5;
pub const SD_LAVA: C2RustUnnamed_19 = 4;
pub const SD_FIRE: C2RustUnnamed_19 = 3;
pub const SD_WATER: C2RustUnnamed_19 = 2;
pub const SD_GENERIC: C2RustUnnamed_19 = 1;
pub const SD_NONE: C2RustUnnamed_19 = 0;
pub type C2RustUnnamed_20 = libc::c_uint;
pub const TO_PLAYERNIGHTS: C2RustUnnamed_20 = 4;
pub const TO_PLAYEREMERALDS: C2RustUnnamed_20 = 3;
pub const TO_MOBJ: C2RustUnnamed_20 = 2;
pub const TO_ALLPLAYERS: C2RustUnnamed_20 = 1;
pub const TO_PLAYER: C2RustUnnamed_20 = 0;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const CRUMBLE_RESTORE: C2RustUnnamed_21 = 4;
pub const CRUMBLE_FALL: C2RustUnnamed_21 = 3;
pub const CRUMBLE_ACTIVATED: C2RustUnnamed_21 = 2;
pub const CRUMBLE_WAIT: C2RustUnnamed_21 = 1;
pub const CRUMBLE_NONE: C2RustUnnamed_21 = 0;
pub type subsector_t = subsector_s;
pub type msecnode_t = msecnode_s;
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
pub type thinklistnum_t = libc::c_uint;
pub const NUM_THINKERLISTS: thinklistnum_t = 5;
pub const THINK_PRECIP: thinklistnum_t = 4;
pub const THINK_DYNSLOPE: thinklistnum_t = 3;
pub const THINK_MOBJ: thinklistnum_t = 2;
pub const THINK_MAIN: thinklistnum_t = 1;
pub const THINK_POLYOBJ: thinklistnum_t = 0;
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
pub type camera_t = camera_s;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const TMEF_EMERALDCHECK: C2RustUnnamed_22 = 2;
pub const TMEF_SKIPTALLY: C2RustUnnamed_22 = 1;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const TMSP_FORCESPIN: C2RustUnnamed_23 = 2;
pub const TMSP_NOTELEPORT: C2RustUnnamed_23 = 1;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const TMFA_SPLAT: C2RustUnnamed_24 = 32;
pub const TMFA_NOSHADE: C2RustUnnamed_24 = 16;
pub const TMFA_ONLYINSIDES: C2RustUnnamed_24 = 8;
pub const TMFA_INSIDES: C2RustUnnamed_24 = 4;
pub const TMFA_NOSIDES: C2RustUnnamed_24 = 2;
pub const TMFA_NOPLANES: C2RustUnnamed_24 = 1;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const TMFT_INTANGIBLE: C2RustUnnamed_25 = 12;
pub const TMFT_DONTBLOCKOTHERS: C2RustUnnamed_25 = 8;
pub const TMFT_VISIBLEFROMINSIDE: C2RustUnnamed_25 = 7;
pub const TMFT_DONTBLOCKPLAYER: C2RustUnnamed_25 = 4;
pub const TMFT_INTANGIBLEBOTTOM: C2RustUnnamed_25 = 2;
pub const TMFT_INTANGIBLETOP: C2RustUnnamed_25 = 1;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const TMFW_SPLAT: C2RustUnnamed_26 = 32;
pub const TMFW_GOOWATER: C2RustUnnamed_26 = 16;
pub const TMFW_NORIPPLE: C2RustUnnamed_26 = 8;
pub const TMFW_COLORMAPONLY: C2RustUnnamed_26 = 4;
pub const TMFW_DOUBLESHADOW: C2RustUnnamed_26 = 2;
pub const TMFW_NOSIDES: C2RustUnnamed_26 = 1;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const TMFB_DYNAMIC: C2RustUnnamed_27 = 4;
pub const TMFB_SPINDASH: C2RustUnnamed_27 = 2;
pub const TMFB_REVERSE: C2RustUnnamed_27 = 1;
pub type C2RustUnnamed_28 = libc::c_uint;
pub const TMFC_SPLAT: C2RustUnnamed_28 = 16;
pub const TMFC_FLOATBOB: C2RustUnnamed_28 = 8;
pub const TMFC_AIRBOB: C2RustUnnamed_28 = 4;
pub const TMFC_NORETURN: C2RustUnnamed_28 = 2;
pub const TMFC_NOSHADE: C2RustUnnamed_28 = 1;
pub type C2RustUnnamed_29 = libc::c_uint;
pub const TMFR_SPINDASH: C2RustUnnamed_29 = 2;
pub const TMFR_REVERSE: C2RustUnnamed_29 = 1;
pub type C2RustUnnamed_30 = libc::c_uint;
pub const TMFM_INVISIBLE: C2RustUnnamed_30 = 2;
pub const TMFM_BRICK: C2RustUnnamed_30 = 1;
pub type C2RustUnnamed_31 = libc::c_uint;
pub const TMFB_STRONG: C2RustUnnamed_31 = 3;
pub const TMFB_REGULAR: C2RustUnnamed_31 = 2;
pub const TMFB_SPIN: C2RustUnnamed_31 = 1;
pub const TMFB_TOUCH: C2RustUnnamed_31 = 0;
pub type C2RustUnnamed_32 = libc::c_uint;
pub const TMFB_SPLAT: C2RustUnnamed_32 = 8;
pub const TMFB_ONLYBOTTOM: C2RustUnnamed_32 = 4;
pub const TMFB_EXECUTOR: C2RustUnnamed_32 = 2;
pub const TMFB_PUSHABLES: C2RustUnnamed_32 = 1;
pub type C2RustUnnamed_33 = libc::c_uint;
pub const TMFL_SPLAT: C2RustUnnamed_33 = 2;
pub const TMFL_NOBOSSES: C2RustUnnamed_33 = 1;
pub type C2RustUnnamed_34 = libc::c_uint;
pub const TMT_EACHTIMEENTERANDEXIT: C2RustUnnamed_34 = 3;
pub const TMT_EACHTIMEENTER: C2RustUnnamed_34 = 2;
pub const TMT_EACHTIMEMASK: C2RustUnnamed_34 = 1;
pub const TMT_ONCE: C2RustUnnamed_34 = 1;
pub const TMT_CONTINUOUS: C2RustUnnamed_34 = 0;
pub type C2RustUnnamed_35 = libc::c_uint;
pub const TMXT_EACHTIMEENTERANDEXIT: C2RustUnnamed_35 = 2;
pub const TMXT_EACHTIMEENTER: C2RustUnnamed_35 = 1;
pub const TMXT_EACHTIMEMASK: C2RustUnnamed_35 = 0;
pub const TMXT_CONTINUOUS: C2RustUnnamed_35 = 0;
pub type C2RustUnnamed_36 = libc::c_uint;
pub const TMF_DOESNTHAVEANY: C2RustUnnamed_36 = 4;
pub const TMF_DOESNTHAVEALL: C2RustUnnamed_36 = 3;
pub const TMF_HASEXACTLY: C2RustUnnamed_36 = 2;
pub const TMF_HASANY: C2RustUnnamed_36 = 1;
pub const TMF_HASALL: C2RustUnnamed_36 = 0;
pub type C2RustUnnamed_37 = libc::c_uint;
pub const TMT_BLUE: C2RustUnnamed_37 = 1;
pub const TMT_RED: C2RustUnnamed_37 = 0;
pub type textmapcomparison_t = libc::c_uint;
pub const TMC_GTE: textmapcomparison_t = 2;
pub const TMC_LTE: textmapcomparison_t = 1;
pub const TMC_EQUAL: textmapcomparison_t = 0;
pub type C2RustUnnamed_38 = libc::c_uint;
pub const TMG_TEMPREVERSE: C2RustUnnamed_38 = 2;
pub const TMG_REVERSE: C2RustUnnamed_38 = 1;
pub const TMG_NORMAL: C2RustUnnamed_38 = 0;
pub type textmapnightsplayer_t = libc::c_uint;
pub const TMNP_TRIGGERER: textmapnightsplayer_t = 2;
pub const TMNP_SLOWEST: textmapnightsplayer_t = 1;
pub const TMNP_FASTEST: textmapnightsplayer_t = 0;
pub type C2RustUnnamed_39 = libc::c_uint;
pub const TMN_FROMNIGHTS: C2RustUnnamed_39 = 2;
pub const TMN_FROMNONIGHTS: C2RustUnnamed_39 = 1;
pub const TMN_ALWAYS: C2RustUnnamed_39 = 0;
pub type C2RustUnnamed_40 = libc::c_uint;
pub const TMN_LEVELCOMPLETION: C2RustUnnamed_40 = 4;
pub const TMN_BONUSLAPS: C2RustUnnamed_40 = 1;
pub type C2RustUnnamed_41 = libc::c_uint;
pub const TMD_SOMEBODYNIGHTS: C2RustUnnamed_41 = 2;
pub const TMD_NOBODYNIGHTS: C2RustUnnamed_41 = 1;
pub const TMD_ALWAYS: C2RustUnnamed_41 = 0;
pub type C2RustUnnamed_42 = libc::c_uint;
pub const TMS_ALWAYS: C2RustUnnamed_42 = 2;
pub const TMS_IFNOTENOUGH: C2RustUnnamed_42 = 1;
pub const TMS_IFENOUGH: C2RustUnnamed_42 = 0;
pub type C2RustUnnamed_43 = libc::c_uint;
pub const TMI_ENTER: C2RustUnnamed_43 = 4;
pub const TMI_BONUSLAPS: C2RustUnnamed_43 = 1;
pub type C2RustUnnamed_44 = libc::c_uint;
pub const TMP_BOTH: C2RustUnnamed_44 = 2;
pub const TMP_CEILING: C2RustUnnamed_44 = 1;
pub const TMP_FLOOR: C2RustUnnamed_44 = 0;
pub type C2RustUnnamed_45 = libc::c_uint;
pub const TMT_TRIGGERTAG: C2RustUnnamed_45 = 3;
pub const TMT_REPLACEFIRST: C2RustUnnamed_45 = 2;
pub const TMT_REMOVE: C2RustUnnamed_45 = 1;
pub const TMT_ADD: C2RustUnnamed_45 = 0;
pub type C2RustUnnamed_46 = libc::c_uint;
pub const TMT_RELATIVE: C2RustUnnamed_46 = 8;
pub const TMT_KEEPMOMENTUM: C2RustUnnamed_46 = 4;
pub const TMT_KEEPANGLE: C2RustUnnamed_46 = 2;
pub const TMT_SILENT: C2RustUnnamed_46 = 1;
pub type C2RustUnnamed_47 = libc::c_uint;
pub const TMM_NOLOOP: C2RustUnnamed_47 = 32;
pub const TMM_FORCERESET: C2RustUnnamed_47 = 16;
pub const TMM_NORELOAD: C2RustUnnamed_47 = 8;
pub const TMM_FADE: C2RustUnnamed_47 = 4;
pub const TMM_OFFSET: C2RustUnnamed_47 = 2;
pub const TMM_ALLPLAYERS: C2RustUnnamed_47 = 1;
pub type textmapsoundsource_t = libc::c_uint;
pub const TMSS_TAGGEDSECTOR: textmapsoundsource_t = 3;
pub const TMSS_NOWHERE: textmapsoundsource_t = 2;
pub const TMSS_TRIGGERSECTOR: textmapsoundsource_t = 1;
pub const TMSS_TRIGGERMOBJ: textmapsoundsource_t = 0;
pub type textmapsoundlistener_t = libc::c_uint;
pub const TMSL_TAGGEDSECTOR: textmapsoundlistener_t = 2;
pub const TMSL_TRIGGERER: textmapsoundlistener_t = 1;
pub const TMSL_EVERYONE: textmapsoundlistener_t = 0;
pub type C2RustUnnamed_48 = libc::c_uint;
pub const TML_CEILING: C2RustUnnamed_48 = 2;
pub const TML_FLOOR: C2RustUnnamed_48 = 1;
pub const TML_SECTOR: C2RustUnnamed_48 = 0;
pub type C2RustUnnamed_49 = libc::c_uint;
pub const TMLC_NOCEILING: C2RustUnnamed_49 = 4;
pub const TMLC_NOFLOOR: C2RustUnnamed_49 = 2;
pub const TMLC_NOSECTOR: C2RustUnnamed_49 = 1;
pub type C2RustUnnamed_50 = libc::c_uint;
pub const TMF_TICBASED: C2RustUnnamed_50 = 4;
pub const TMF_OVERRIDE: C2RustUnnamed_50 = 2;
pub const TMF_RELATIVE: C2RustUnnamed_50 = 1;
pub type C2RustUnnamed_51 = libc::c_uint;
pub const TMB_SYNC: C2RustUnnamed_51 = 2;
pub const TMB_USETARGET: C2RustUnnamed_51 = 1;
pub type C2RustUnnamed_52 = libc::c_uint;
pub const TMFR_CHECKFLAG: C2RustUnnamed_52 = 2;
pub const TMFR_NORETURN: C2RustUnnamed_52 = 1;
pub type C2RustUnnamed_53 = libc::c_uint;
pub const TMST_DONTDOTRANSLUCENT: C2RustUnnamed_53 = 2;
pub const TMST_RELATIVE: C2RustUnnamed_53 = 1;
pub type C2RustUnnamed_54 = libc::c_uint;
pub const TMFT_USEEXACTALPHA: C2RustUnnamed_54 = 512;
pub const TMFT_DONTDOCOLORMAP: C2RustUnnamed_54 = 256;
pub const TMFT_DONTDOLIGHTING: C2RustUnnamed_54 = 128;
pub const TMFT_DONTDOEXISTS: C2RustUnnamed_54 = 64;
pub const TMFT_DONTDOTRANSLUCENT: C2RustUnnamed_54 = 32;
pub const TMFT_GHOSTFADE: C2RustUnnamed_54 = 16;
pub const TMFT_IGNORECOLLISION: C2RustUnnamed_54 = 8;
pub const TMFT_TICBASED: C2RustUnnamed_54 = 4;
pub const TMFT_OVERRIDE: C2RustUnnamed_54 = 2;
pub const TMFT_RELATIVE: C2RustUnnamed_54 = 1;
pub type C2RustUnnamed_55 = libc::c_uint;
pub const TMS_BOTH: C2RustUnnamed_55 = 2;
pub const TMS_CENTERPOINT: C2RustUnnamed_55 = 1;
pub const TMS_VIEWPOINT: C2RustUnnamed_55 = 0;
pub type C2RustUnnamed_56 = libc::c_uint;
pub const TMP_KEEPREALTIME: C2RustUnnamed_56 = 16;
pub const TMP_KEEPCONTROLS: C2RustUnnamed_56 = 8;
pub const TMP_CALLBYNAME: C2RustUnnamed_56 = 4;
pub const TMP_RUNPOSTEXEC: C2RustUnnamed_56 = 2;
pub const TMP_CLOSE: C2RustUnnamed_56 = 1;
pub type C2RustUnnamed_57 = libc::c_uint;
pub const TMF_REMOVE: C2RustUnnamed_57 = 2;
pub const TMF_ADD: C2RustUnnamed_57 = 1;
pub const TMF_NOCHANGE: C2RustUnnamed_57 = 0;
pub type C2RustUnnamed_58 = libc::c_uint;
pub const TMSD_FRONTBACK: C2RustUnnamed_58 = 2;
pub const TMSD_BACK: C2RustUnnamed_58 = 1;
pub const TMSD_FRONT: C2RustUnnamed_58 = 0;
pub type C2RustUnnamed_59 = libc::c_uint;
pub const TMS_CARRYONLY: C2RustUnnamed_59 = 2;
pub const TMS_SCROLLONLY: C2RustUnnamed_59 = 1;
pub const TMS_SCROLLCARRY: C2RustUnnamed_59 = 0;
pub type C2RustUnnamed_60 = libc::c_uint;
pub const TMST_NONEXCLUSIVE: C2RustUnnamed_60 = 4;
pub const TMST_TYPEMASK: C2RustUnnamed_60 = 3;
pub const TMST_DISPLACEMENT: C2RustUnnamed_60 = 2;
pub const TMST_ACCELERATIVE: C2RustUnnamed_60 = 1;
pub const TMST_REGULAR: C2RustUnnamed_60 = 0;
pub type C2RustUnnamed_61 = libc::c_uint;
pub const TMPF_NONEXCLUSIVE: C2RustUnnamed_61 = 2;
pub const TMPF_SLIDE: C2RustUnnamed_61 = 1;
pub type C2RustUnnamed_62 = libc::c_uint;
pub const TMB_MODULATE: C2RustUnnamed_62 = 4;
pub const TMB_REVERSESUBTRACT: C2RustUnnamed_62 = 3;
pub const TMB_SUBTRACT: C2RustUnnamed_62 = 2;
pub const TMB_ADD: C2RustUnnamed_62 = 1;
pub const TMB_TRANSLUCENT: C2RustUnnamed_62 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct animdef_t {
    pub istexture: int8_t,
    pub endname: [libc::c_char; 9],
    pub startname: [libc::c_char; 9],
    pub speed: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct anim_t {
    pub istexture: int8_t,
    pub picnum: int32_t,
    pub basepic: int32_t,
    pub numpics: int32_t,
    pub speed: tic_t,
}
pub const PU_STATIC: C2RustUnnamed_71 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct levelflat_t {
    pub name: [libc::c_char; 9],
    pub type_0: uint8_t,
    pub u: C2RustUnnamed_63,
    pub width: uint16_t,
    pub height: uint16_t,
    pub animseq: int32_t,
    pub numpics: int32_t,
    pub speed: int32_t,
    pub picture: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_63 {
    pub flat: C2RustUnnamed_65,
    pub texture: C2RustUnnamed_64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_64 {
    pub num: int32_t,
    pub lastnum: int32_t,
    pub basenum: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_65 {
    pub lumpnum: lumpnum_t,
    pub baselumpnum: lumpnum_t,
}
pub const LEVELFLAT_FLAT: C2RustUnnamed_69 = 1;
pub const LEVELFLAT_TEXTURE: C2RustUnnamed_69 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinkerdata_t {
    pub thinker: thinker_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fadecolormap_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub source_exc: *mut extracolormap_t,
    pub dest_exc: *mut extracolormap_t,
    pub ticbased: boolean,
    pub duration: int32_t,
    pub timer: int32_t,
}
pub const PU_LEVSPEC: C2RustUnnamed_71 = 51;
pub const TMCF_IGNOREFLAGS: C2RustUnnamed_70 = 2048;
pub const TMCF_SUBFADEEND: C2RustUnnamed_70 = 1024;
pub const TMCF_SUBFADESTART: C2RustUnnamed_70 = 512;
pub const TMCF_SUBFADEA: C2RustUnnamed_70 = 256;
pub const TMCF_SUBFADEB: C2RustUnnamed_70 = 128;
pub const TMCF_SUBFADEG: C2RustUnnamed_70 = 64;
pub const TMCF_SUBFADER: C2RustUnnamed_70 = 32;
pub const TMCF_SUBLIGHTA: C2RustUnnamed_70 = 16;
pub const TMCF_SUBLIGHTB: C2RustUnnamed_70 = 8;
pub const TMCF_SUBLIGHTG: C2RustUnnamed_70 = 4;
pub const TMCF_SUBLIGHTR: C2RustUnnamed_70 = 2;
pub const TMCF_RELATIVE: C2RustUnnamed_70 = 1;
pub const TMCF_FROMBLACK: C2RustUnnamed_70 = 4096;
pub const TMCF_OVERRIDE: C2RustUnnamed_70 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fade_t {
    pub thinker: thinker_t,
    pub rover: *mut ffloor_t,
    pub dest_exc: *mut extracolormap_t,
    pub sectornum: uint32_t,
    pub ffloornum: uint32_t,
    pub alpha: int32_t,
    pub sourcevalue: int16_t,
    pub destvalue: int16_t,
    pub destlightlevel: int16_t,
    pub speed: int16_t,
    pub ticbased: boolean,
    pub timer: int32_t,
    pub doexists: boolean,
    pub dotranslucent: boolean,
    pub dolighting: boolean,
    pub docolormap: boolean,
    pub docollision: boolean,
    pub doghostfade: boolean,
    pub exactalpha: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scroll_t {
    pub thinker: thinker_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
    pub affectee: int32_t,
    pub control: int32_t,
    pub last_height: fixed_t,
    pub vdx: fixed_t,
    pub vdy: fixed_t,
    pub accel: int32_t,
    pub exclusive: int32_t,
    pub type_0: C2RustUnnamed_66,
}
pub type C2RustUnnamed_66 = libc::c_uint;
pub const sc_carry_ceiling: C2RustUnnamed_66 = 4;
pub const sc_carry: C2RustUnnamed_66 = 3;
pub const sc_ceiling: C2RustUnnamed_66 = 2;
pub const sc_floor: C2RustUnnamed_66 = 1;
pub const sc_side: C2RustUnnamed_66 = 0;
pub type ceiling_e = libc::c_uint;
pub const bounceCeilingCrush: ceiling_e = 11;
pub const bounceCeiling: ceiling_e = 10;
pub const moveCeilingByDistance: ceiling_e = 9;
pub const instantMoveCeilingByFrontSector: ceiling_e = 8;
pub const moveCeilingByFrontSector: ceiling_e = 7;
pub const crushBothOnce: ceiling_e = 6;
pub const crushCeilOnce: ceiling_e = 5;
pub const raiseAndCrush: ceiling_e = 4;
pub const crushAndRaise: ceiling_e = 3;
pub const instantRaise: ceiling_e = 2;
pub const lowerToLowestFast: ceiling_e = 1;
pub const raiseToHighest: ceiling_e = 0;
pub type floor_e = libc::c_uint;
pub const crushFloorOnce: floor_e = 7;
pub const bounceFloorCrush: floor_e = 6;
pub const bounceFloor: floor_e = 5;
pub const moveFloorByDistance: floor_e = 4;
pub const instantMoveFloorByFrontSector: floor_e = 3;
pub const moveFloorByFrontSector: floor_e = 2;
pub const instantLower: floor_e = 1;
pub const raiseFloorToNearestFast: floor_e = 0;
pub type elevator_e = libc::c_uint;
pub const bridgeFall: elevator_e = 5;
pub const elevateHighest: elevator_e = 4;
pub const elevateBounce: elevator_e = 3;
pub const elevateContinuous: elevator_e = 2;
pub const elevateDown: elevator_e = 1;
pub const elevateUp: elevator_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: int32_t,
    pub minlight: int16_t,
    pub maxlight: int16_t,
    pub darktime: int32_t,
    pub brighttime: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: int16_t,
    pub maxlight: int16_t,
    pub direction: int16_t,
    pub speed: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: int32_t,
    pub resetcount: int32_t,
    pub maxlight: int16_t,
    pub minlight: int16_t,
}
pub const PU_CACHE: C2RustUnnamed_71 = 49;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ceiling_t {
    pub thinker: thinker_t,
    pub type_0: ceiling_e,
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub delay: fixed_t,
    pub delaytimer: fixed_t,
    pub crush: uint8_t,
    pub texture: int32_t,
    pub direction: int32_t,
    pub tag: int16_t,
    pub origspeed: fixed_t,
    pub sourceline: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floormove_t {
    pub thinker: thinker_t,
    pub type_0: floor_e,
    pub crush: uint8_t,
    pub sector: *mut sector_t,
    pub direction: int32_t,
    pub texture: int32_t,
    pub floordestheight: fixed_t,
    pub speed: fixed_t,
    pub origspeed: fixed_t,
    pub delay: fixed_t,
    pub delaytimer: fixed_t,
    pub tag: int16_t,
    pub sourceline: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct elevator_t {
    pub thinker: thinker_t,
    pub type_0: elevator_e,
    pub sector: *mut sector_t,
    pub actionsector: *mut sector_t,
    pub direction: int32_t,
    pub floordestheight: fixed_t,
    pub ceilingdestheight: fixed_t,
    pub speed: fixed_t,
    pub origspeed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub distance: fixed_t,
    pub delay: fixed_t,
    pub delaytimer: fixed_t,
    pub floorwasheight: fixed_t,
    pub ceilingwasheight: fixed_t,
    pub sourceline: *mut line_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct executor_t {
    pub thinker: thinker_t,
    pub line: *mut line_t,
    pub caller: *mut mobj_t,
    pub sector: *mut sector_t,
    pub timer: int32_t,
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
pub struct thinkerlist_t {
    pub count: uint32_t,
    pub thinkers: *mut *mut thinker_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eachtime_t {
    pub thinker: thinker_t,
    pub sourceline: *mut line_t,
    pub playersInArea: [boolean; 32],
    pub triggerOnExit: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct noenemies_t {
    pub thinker: thinker_t,
    pub sourceline: *mut line_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floatthink_t {
    pub thinker: thinker_t,
    pub sourceline: *mut line_t,
    pub sector: *mut sector_t,
    pub tag: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mariocheck_t {
    pub thinker: thinker_t,
    pub sourceline: *mut line_t,
    pub sector: *mut sector_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pusher_t {
    pub thinker: thinker_t,
    pub type_0: pushertype_e,
    pub x_mag: fixed_t,
    pub y_mag: fixed_t,
    pub z_mag: fixed_t,
    pub affectee: int32_t,
    pub roverpusher: uint8_t,
    pub referrer: int32_t,
    pub exclusive: int32_t,
    pub slider: int32_t,
}
pub type pushertype_e = libc::c_uint;
pub const p_current: pushertype_e = 1;
pub const p_wind: pushertype_e = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct friction_t {
    pub thinker: thinker_t,
    pub friction: int32_t,
    pub movefactor: int32_t,
    pub affectee: int32_t,
    pub referrer: int32_t,
    pub roverfriction: uint8_t,
}
pub const PU_LEVEL: C2RustUnnamed_71 = 50;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct laserthink_t {
    pub thinker: thinker_t,
    pub tag: int16_t,
    pub sourceline: *mut line_t,
    pub nobosses: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thwomp_t {
    pub thinker: thinker_t,
    pub sourceline: *mut line_t,
    pub sector: *mut sector_t,
    pub crushspeed: fixed_t,
    pub retractspeed: fixed_t,
    pub direction: int32_t,
    pub floorstartheight: fixed_t,
    pub ceilingstartheight: fixed_t,
    pub delay: int32_t,
    pub tag: int16_t,
    pub sound: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raise_t {
    pub thinker: thinker_t,
    pub tag: int16_t,
    pub sector: *mut sector_t,
    pub ceilingbottom: fixed_t,
    pub ceilingtop: fixed_t,
    pub basespeed: fixed_t,
    pub extraspeed: fixed_t,
    pub shaketimer: uint8_t,
    pub flags: uint8_t,
}
pub const RF_SPINDASH: C2RustUnnamed_68 = 2;
pub const RF_REVERSE: C2RustUnnamed_68 = 1;
pub const RF_DYNAMIC: C2RustUnnamed_68 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct planedisplace_t {
    pub thinker: thinker_t,
    pub affectee: int32_t,
    pub control: int32_t,
    pub last_height: fixed_t,
    pub speed: fixed_t,
    pub reverse: uint8_t,
    pub type_0: C2RustUnnamed_67,
}
pub type C2RustUnnamed_67 = libc::c_uint;
pub const pd_both: C2RustUnnamed_67 = 2;
pub const pd_ceiling: C2RustUnnamed_67 = 1;
pub const pd_floor: C2RustUnnamed_67 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct disappear_t {
    pub thinker: thinker_t,
    pub appeartime: tic_t,
    pub disappeartime: tic_t,
    pub offset: tic_t,
    pub timer: tic_t,
    pub affectee: int32_t,
    pub sourceline: int32_t,
    pub exists: int32_t,
}
pub type C2RustUnnamed_68 = libc::c_uint;
pub type C2RustUnnamed_69 = libc::c_uint;
pub const LEVELFLAT_PNG: C2RustUnnamed_69 = 3;
pub const LEVELFLAT_PATCH: C2RustUnnamed_69 = 2;
pub const LEVELFLAT_NONE: C2RustUnnamed_69 = 0;
pub type C2RustUnnamed_70 = libc::c_uint;
pub type C2RustUnnamed_71 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_71 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_71 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_71 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_71 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_71 = 52;
pub const PU_HWRCACHE: C2RustUnnamed_71 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_71 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_71 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_71 = 21;
pub const PU_HUDGFX: C2RustUnnamed_71 = 19;
pub const PU_SPRITE: C2RustUnnamed_71 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_71 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_71 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_71 = 15;
pub const PU_PATCH: C2RustUnnamed_71 = 14;
pub const PU_MUSIC: C2RustUnnamed_71 = 12;
pub const PU_SOUND: C2RustUnnamed_71 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_71 = 3;
pub const PU_LUA: C2RustUnnamed_71 = 2;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline(always)]
unsafe extern "C" fn FloatToFixed(mut f: libc::c_float) -> fixed_t {
    return (f * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float) as fixed_t;
}
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
unsafe extern "C" fn FixedInt(mut a: fixed_t) -> fixed_t {
    return FixedMul(a, 1 as libc::c_int);
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
#[inline(always)]
unsafe extern "C" fn FixedFloor(mut x: fixed_t) -> fixed_t {
    let a: fixed_t = abs(x);
    let i: fixed_t = (a >> 16 as libc::c_int) << 16 as libc::c_int;
    let f: fixed_t = a - i;
    if f == 0 as libc::c_int {
        return x;
    }
    if x != -(2147483647 as libc::c_int) - 1 as libc::c_int {
        if x > 0 as libc::c_int {
            return x - f
        } else {
            return x - (((1 as libc::c_int) << 16 as libc::c_int) - f)
        }
    }
    return -(2147483647 as libc::c_int) - 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn InvAngle(mut a: angle_t) -> angle_t {
    return (0xffffffff as libc::c_uint)
        .wrapping_sub(a)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
}
#[no_mangle]
pub static mut skyboxmo: [*mut mobj_t; 2] = [0 as *const mobj_t as *mut mobj_t; 2];
#[no_mangle]
pub static mut skyboxviewpnts: [*mut mobj_t; 16] = [0 as *const mobj_t
    as *mut mobj_t; 16];
#[no_mangle]
pub static mut skyboxcenterpnts: [*mut mobj_t; 16] = [0 as *const mobj_t
    as *mut mobj_t; 16];
static mut lastanim: *mut anim_t = 0 as *const anim_t as *mut anim_t;
static mut anims: *mut anim_t = 0 as *const anim_t as *mut anim_t;
static mut maxanims: size_t = 0;
static mut animdefs: *mut animdef_t = 0 as *const animdef_t as *mut animdef_t;
unsafe extern "C" fn GrowAnimDefs() {
    maxanims = maxanims.wrapping_add(1);
    maxanims;
    animdefs = Z_ReallocAlign(
        animdefs as *mut libc::c_void,
        (::core::mem::size_of::<animdef_t>() as libc::c_ulong)
            .wrapping_mul(maxanims.wrapping_add(1 as libc::c_int as size_t)),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut animdef_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_InitPicAnims() {
    let mut w: int32_t = 0;
    let mut i: size_t = 0;
    maxanims = 0 as libc::c_int as size_t;
    w = numwadfiles as libc::c_int - 1 as libc::c_int;
    while w >= 0 as libc::c_int {
        let mut animdefsLumpNum: uint16_t = 0;
        animdefsLumpNum = W_CheckNumForNamePwad(
            b"ANIMDEFS\0" as *const u8 as *const libc::c_char,
            w as uint16_t,
            0 as libc::c_int as uint16_t,
        );
        while animdefsLumpNum as libc::c_int != 32767 as libc::c_int {
            P_ParseANIMDEFSLump(w, animdefsLumpNum);
            animdefsLumpNum = W_CheckNumForNamePwad(
                b"ANIMDEFS\0" as *const u8 as *const libc::c_char,
                w as uint16_t,
                (animdefsLumpNum as libc::c_int + 1 as libc::c_int) as uint16_t,
            );
        }
        w -= 1;
        w;
    }
    (*animdefs.offset(maxanims as isize)).istexture = -(1 as libc::c_int) as int8_t;
    strncpy(
        ((*animdefs.offset(maxanims as isize)).endname).as_mut_ptr(),
        b"\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        ((*animdefs.offset(maxanims as isize)).startname).as_mut_ptr(),
        b"\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    );
    (*animdefs.offset(maxanims as isize)).speed = 0 as libc::c_int;
    if !anims.is_null() {
        free(anims as *mut libc::c_void);
    }
    anims = malloc(
        (::core::mem::size_of::<anim_t>() as libc::c_ulong)
            .wrapping_mul(maxanims.wrapping_add(1 as libc::c_int as size_t)),
    ) as *mut anim_t;
    if anims.is_null() {
        I_Error(
            b"Not enough free memory for ANIMDEFS data\0" as *const u8
                as *const libc::c_char,
        );
    }
    lastanim = anims;
    let mut current_block_32: u64;
    i = 0 as libc::c_int as size_t;
    while (*animdefs.offset(i as isize)).istexture as libc::c_int != -(1 as libc::c_int)
    {
        if (*animdefs.offset(i as isize)).istexture != 0 {
            if R_CheckTextureNumForName(
                ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            ) == -(1 as libc::c_int)
            {
                current_block_32 = 12039483399334584727;
            } else {
                (*lastanim)
                    .picnum = R_TextureNumForName(
                    ((*animdefs.offset(i as isize)).endname).as_mut_ptr(),
                );
                (*lastanim)
                    .basepic = R_TextureNumForName(
                    ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
                );
                current_block_32 = 11194104282611034094;
            }
        } else if W_CheckNumForName(
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
        ) == 4294967295 as libc::c_uint
        {
            current_block_32 = 12039483399334584727;
        } else {
            (*lastanim)
                .picnum = R_GetFlatNumForName(
                ((*animdefs.offset(i as isize)).endname).as_mut_ptr(),
            ) as int32_t;
            (*lastanim)
                .basepic = R_GetFlatNumForName(
                ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            ) as int32_t;
            current_block_32 = 11194104282611034094;
        }
        match current_block_32 {
            11194104282611034094 => {
                (*lastanim).istexture = (*animdefs.offset(i as isize)).istexture;
                (*lastanim)
                    .numpics = (*lastanim).picnum - (*lastanim).basepic
                    + 1 as libc::c_int;
                if (*lastanim).numpics < 2 as libc::c_int {
                    free(anims as *mut libc::c_void);
                    I_Error(
                        b"P_InitPicAnims: bad cycle from %s to %s\0" as *const u8
                            as *const libc::c_char,
                        ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
                        ((*animdefs.offset(i as isize)).endname).as_mut_ptr(),
                    );
                }
                (*lastanim).speed = (*animdefs.offset(i as isize)).speed as tic_t;
                lastanim = lastanim.offset(1);
                lastanim;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    (*lastanim).istexture = -(1 as libc::c_int) as int8_t;
    R_ClearTextureNumCache(false_0 as libc::c_int);
    Z_Free(animdefs as *mut libc::c_void);
    animdefs = 0 as *mut animdef_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_ParseANIMDEFSLump(
    mut wadNum: int32_t,
    mut lumpnum: uint16_t,
) {
    let mut animdefsLump: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut animdefsLumpLength: size_t = 0;
    let mut animdefsText: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut animdefsToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    animdefsLump = W_CacheLumpNumPwad(
        wadNum as uint16_t,
        lumpnum,
        PU_STATIC as libc::c_int,
    ) as *mut libc::c_char;
    if animdefsLump.is_null() {
        return;
    }
    animdefsLumpLength = W_LumpLengthPwad(wadNum as uint16_t, lumpnum);
    animdefsText = Z_MallocAlign(
        animdefsLumpLength
            .wrapping_add(1 as libc::c_int as size_t)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut libc::c_char;
    memmove(
        animdefsText as *mut libc::c_void,
        animdefsLump as *const libc::c_void,
        animdefsLumpLength,
    );
    *animdefsText.offset(animdefsLumpLength as isize) = '\0' as i32 as libc::c_char;
    Z_Free(animdefsLump as *mut libc::c_void);
    p = animdefsText;
    animdefsToken = M_GetToken(p);
    while !animdefsToken.is_null() {
        if strcasecmp(animdefsToken, b"TEXTURE\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            Z_Free(animdefsToken as *mut libc::c_void);
            P_ParseAnimationDefintion(1 as libc::c_int as int8_t);
        } else if strcasecmp(
            animdefsToken,
            b"FLAT\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            Z_Free(animdefsToken as *mut libc::c_void);
            P_ParseAnimationDefintion(0 as libc::c_int as int8_t);
        } else if strcasecmp(
            animdefsToken,
            b"OSCILLATE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            I_Error(
                b"Error parsing ANIMDEFS lump: Animation definitions utilizing \"OSCILLATE\" (the animation plays in reverse when it reaches the end) are not supported by SRB2\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            I_Error(
                b"Error parsing ANIMDEFS lump: Expected \"TEXTURE\" or \"FLAT\", got \"%s\"\0"
                    as *const u8 as *const libc::c_char,
                animdefsToken,
            );
        }
        while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != '\n' as i32 {
            p = p.offset(1);
            p;
        }
        if *p as libc::c_int == '\n' as i32 {
            p = p.offset(1);
            p;
        }
        animdefsToken = M_GetToken(p);
    }
    Z_Free(animdefsToken as *mut libc::c_void);
    Z_Free(animdefsText as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn P_ParseAnimationDefintion(mut istexture: int8_t) {
    let mut animdefsToken: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut animdefsTokenLength: size_t = 0;
    let mut endPos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut animSpeed: int32_t = 0;
    let mut i: size_t = 0;
    animdefsToken = M_GetToken(0 as *const libc::c_char);
    if animdefsToken.is_null() {
        I_Error(
            b"Error parsing ANIMDEFS lump: Unexpected end of file where start texture/flat name should be\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if strcasecmp(animdefsToken, b"OPTIONAL\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        Z_Free(animdefsToken as *mut libc::c_void);
        animdefsToken = M_GetToken(0 as *const libc::c_char);
        if animdefsToken.is_null() {
            I_Error(
                b"Error parsing ANIMDEFS lump: Unexpected end of file where start texture/flat name should be\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if strcasecmp(
            animdefsToken,
            b"RANGE\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            I_Error(
                b"Error parsing ANIMDEFS lump: \"OPTIONAL\" is a keyword; you cannot use it as the startname of an animation\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    animdefsTokenLength = strlen(animdefsToken);
    if animdefsTokenLength > 8 as libc::c_int as size_t {
        I_Error(
            b"Error parsing ANIMDEFS lump: lump name \"%s\" exceeds 8 characters\0"
                as *const u8 as *const libc::c_char,
            animdefsToken,
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < maxanims {
        if (*animdefs.offset(i as isize)).istexture as libc::c_int
            == istexture as libc::c_int
            && strcasecmp(
                animdefsToken,
                ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            ) == 0 as libc::c_int
        {
            Z_Free(animdefsToken as *mut libc::c_void);
            return;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == maxanims {
        GrowAnimDefs();
        strncpy(
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            animdefsToken,
            9 as libc::c_int as libc::c_ulong,
        );
    }
    Z_Free(animdefsToken as *mut libc::c_void);
    (*animdefs.offset(i as isize)).istexture = istexture;
    animdefsToken = M_GetToken(0 as *const libc::c_char);
    if animdefsToken.is_null() {
        I_Error(
            b"Error parsing ANIMDEFS lump: Unexpected end of file where \"RANGE\" after \"%s\"'s startname should be\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
        );
    }
    if strcasecmp(animdefsToken, b"ALLOWDECALS\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        Z_Free(animdefsToken as *mut libc::c_void);
        animdefsToken = M_GetToken(0 as *const libc::c_char);
    }
    if strcasecmp(animdefsToken, b"PIC\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        I_Error(
            b"Error parsing ANIMDEFS lump: Animation definitions utilizing \"PIC\" (specific frames instead of a consecutive range) are not supported by SRB2\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if strcasecmp(animdefsToken, b"RANGE\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing ANIMDEFS lump: Expected \"RANGE\" after \"%s\"'s startname, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            animdefsToken,
        );
    }
    Z_Free(animdefsToken as *mut libc::c_void);
    animdefsToken = M_GetToken(0 as *const libc::c_char);
    if animdefsToken.is_null() {
        I_Error(
            b"Error parsing ANIMDEFS lump: Unexpected end of file where \"%s\"'s end texture/flat name should be\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
        );
    }
    animdefsTokenLength = strlen(animdefsToken);
    if animdefsTokenLength > 8 as libc::c_int as size_t {
        I_Error(
            b"Error parsing ANIMDEFS lump: lump name \"%s\" exceeds 8 characters\0"
                as *const u8 as *const libc::c_char,
            animdefsToken,
        );
    }
    strncpy(
        ((*animdefs.offset(i as isize)).endname).as_mut_ptr(),
        animdefsToken,
        9 as libc::c_int as libc::c_ulong,
    );
    Z_Free(animdefsToken as *mut libc::c_void);
    animdefsToken = M_GetToken(0 as *const libc::c_char);
    if animdefsToken.is_null() {
        I_Error(
            b"Error parsing ANIMDEFS lump: Unexpected end of file where \"%s\"'s \"TICS\" should be\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
        );
    }
    if strcasecmp(animdefsToken, b"RAND\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        I_Error(
            b"Error parsing ANIMDEFS lump: Animation definitions utilizing \"RAND\" (random duration per frame) are not supported by SRB2\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if strcasecmp(animdefsToken, b"TICS\0" as *const u8 as *const libc::c_char)
        != 0 as libc::c_int
    {
        I_Error(
            b"Error parsing ANIMDEFS lump: Expected \"TICS\" in animation definition for \"%s\", got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            animdefsToken,
        );
    }
    Z_Free(animdefsToken as *mut libc::c_void);
    animdefsToken = M_GetToken(0 as *const libc::c_char);
    if animdefsToken.is_null() {
        I_Error(
            b"Error parsing ANIMDEFS lump: Unexpected end of file where \"%s\"'s animation speed should be\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
        );
    }
    endPos = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    animSpeed = strtol(animdefsToken, &mut endPos, 10 as libc::c_int) as int32_t;
    if endPos == animdefsToken || *endPos as libc::c_int != '\0' as i32
        || *__errno_location() == 34 as libc::c_int || animSpeed < 0 as libc::c_int
    {
        I_Error(
            b"Error parsing ANIMDEFS lump: Expected a positive integer for \"%s\"'s animation speed, got \"%s\"\0"
                as *const u8 as *const libc::c_char,
            ((*animdefs.offset(i as isize)).startname).as_mut_ptr(),
            animdefsToken,
        );
    }
    (*animdefs.offset(i as isize)).speed = animSpeed;
    Z_Free(animdefsToken as *mut libc::c_void);
    if istexture == 0 {
        GrowAnimDefs();
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            &mut *animdefs
                .offset(maxanims.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as *mut animdef_t as *mut libc::c_void,
            &mut *animdefs.offset(i as isize) as *mut animdef_t as *const libc::c_void,
            ::core::mem::size_of::<animdef_t>() as libc::c_ulong,
        );
        (*animdefs.offset(maxanims.wrapping_sub(1 as libc::c_int as size_t) as isize))
            .istexture = 1 as libc::c_int as int8_t;
    }
}
#[inline]
unsafe extern "C" fn P_FindAnimatedFlat(mut animnum: int32_t) {
    let mut i: size_t = 0;
    let mut startflatnum: lumpnum_t = 0;
    let mut endflatnum: lumpnum_t = 0;
    let mut foundflats: *mut levelflat_t = 0 as *mut levelflat_t;
    foundflats = levelflats;
    startflatnum = (*anims.offset(animnum as isize)).basepic as lumpnum_t;
    endflatnum = (*anims.offset(animnum as isize)).picnum as lumpnum_t;
    if startflatnum >> 16 as libc::c_int != endflatnum >> 16 as libc::c_int {
        I_Error(
            b"AnimatedFlat start %s not in same wad as end %s\n\0" as *const u8
                as *const libc::c_char,
            ((*animdefs.offset(animnum as isize)).startname).as_mut_ptr(),
            ((*animdefs.offset(animnum as isize)).endname).as_mut_ptr(),
        );
    }
    i = 0 as libc::c_int as size_t;
    while i < numlevelflats {
        if (*anims.offset(animnum as isize)).istexture as libc::c_int != 0
            && (*foundflats).type_0 as libc::c_int == LEVELFLAT_TEXTURE as libc::c_int
            && ((*foundflats).u.texture.num as uint16_t as lumpnum_t >= startflatnum
                && (*foundflats).u.texture.num as uint16_t as lumpnum_t <= endflatnum)
        {
            (*foundflats).u.texture.basenum = startflatnum as int32_t;
            (*foundflats)
                .animseq = ((*foundflats).u.texture.num as lumpnum_t)
                .wrapping_sub(startflatnum) as int32_t;
            (*foundflats)
                .numpics = endflatnum
                .wrapping_sub(startflatnum)
                .wrapping_add(1 as libc::c_int as lumpnum_t) as int32_t;
            (*foundflats).speed = (*anims.offset(animnum as isize)).speed as int32_t;
            CONS_Debug(
                0x400 as libc::c_int,
                b"animflat: #%03d name:%.8s animseq:%d numpics:%d speed:%d\n\0"
                    as *const u8 as *const libc::c_char,
                atoi(sizeu1(i)),
                ((*foundflats).name).as_mut_ptr(),
                (*foundflats).animseq,
                (*foundflats).numpics,
                (*foundflats).speed,
            );
        } else if (*anims.offset(animnum as isize)).istexture == 0
            && (*foundflats).type_0 as libc::c_int == LEVELFLAT_FLAT as libc::c_int
            && ((*foundflats).u.flat.lumpnum >= startflatnum
                && (*foundflats).u.flat.lumpnum <= endflatnum)
        {
            (*foundflats).u.flat.baselumpnum = startflatnum;
            (*foundflats)
                .animseq = ((*foundflats).u.flat.lumpnum).wrapping_sub(startflatnum)
                as int32_t;
            (*foundflats)
                .numpics = endflatnum
                .wrapping_sub(startflatnum)
                .wrapping_add(1 as libc::c_int as lumpnum_t) as int32_t;
            (*foundflats).speed = (*anims.offset(animnum as isize)).speed as int32_t;
            CONS_Debug(
                0x400 as libc::c_int,
                b"animflat: #%03d name:%.8s animseq:%d numpics:%d speed:%d\n\0"
                    as *const u8 as *const libc::c_char,
                atoi(sizeu1(i)),
                ((*foundflats).name).as_mut_ptr(),
                (*foundflats).animseq,
                (*foundflats).numpics,
                (*foundflats).speed,
            );
        }
        i = i.wrapping_add(1);
        i;
        foundflats = foundflats.offset(1);
        foundflats;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_SetupLevelFlatAnims() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while (*anims.offset(i as isize)).istexture as libc::c_int != -(1 as libc::c_int) {
        P_FindAnimatedFlat(i);
        i += 1;
        i;
    }
}
unsafe extern "C" fn getNextSector(
    mut line: *mut line_t,
    mut sec: *mut sector_t,
) -> *mut sector_t {
    if (*line).frontsector == sec {
        if (*line).backsector != sec {
            return (*line).backsector
        } else {
            return 0 as *mut sector_t
        }
    }
    return (*line).frontsector;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindLowestFloorSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: size_t = 0;
    let mut check: *mut line_t = 0 as *mut line_t;
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut floorh: fixed_t = 0;
    floorh = (*sec).floorheight;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        check = *((*sec).lines).offset(i as isize);
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).floorheight < floorh {
                floorh = (*other).floorheight;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return floorh;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindHighestFloorSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: size_t = 0;
    let mut check: *mut line_t = 0 as *mut line_t;
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut floorh: fixed_t = -(500 as libc::c_int)
        * ((1 as libc::c_int) << 16 as libc::c_int);
    let mut foundsector: int32_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        check = *((*sec).lines).offset(i as isize);
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).floorheight > floorh || foundsector == 0 {
                floorh = (*other).floorheight;
            }
            if foundsector == 0 {
                foundsector = 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return floorh;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindNextHighestFloor(
    mut sec: *mut sector_t,
    mut currentheight: fixed_t,
) -> fixed_t {
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut i: size_t = 0;
    let mut height: fixed_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        other = getNextSector(*((*sec).lines).offset(i as isize), sec);
        if !other.is_null() && (*other).floorheight > currentheight {
            height = (*other).floorheight;
            loop {
                i = i.wrapping_add(1);
                if !(i < (*sec).linecount) {
                    break;
                }
                other = getNextSector(*((*sec).lines).offset(i as isize), sec);
                if !other.is_null() && (*other).floorheight < height
                    && (*other).floorheight > currentheight
                {
                    height = (*other).floorheight;
                }
            }
            return height;
        }
        i = i.wrapping_add(1);
        i;
    }
    return currentheight;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindNextLowestFloor(
    mut sec: *mut sector_t,
    mut currentheight: fixed_t,
) -> fixed_t {
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut i: size_t = 0;
    let mut height: fixed_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        other = getNextSector(*((*sec).lines).offset(i as isize), sec);
        if !other.is_null() && (*other).floorheight < currentheight {
            height = (*other).floorheight;
            loop {
                i = i.wrapping_add(1);
                if !(i < (*sec).linecount) {
                    break;
                }
                other = getNextSector(*((*sec).lines).offset(i as isize), sec);
                if !other.is_null() && (*other).floorheight > height
                    && (*other).floorheight < currentheight
                {
                    height = (*other).floorheight;
                }
            }
            return height;
        }
        i = i.wrapping_add(1);
        i;
    }
    return currentheight;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindLowestCeilingSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: size_t = 0;
    let mut check: *mut line_t = 0 as *mut line_t;
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut height: fixed_t = 32000 as libc::c_int
        * ((1 as libc::c_int) << 16 as libc::c_int);
    let mut foundsector: int32_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        check = *((*sec).lines).offset(i as isize);
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).ceilingheight < height || foundsector == 0 {
                height = (*other).ceilingheight;
            }
            if foundsector == 0 {
                foundsector = 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return height;
}
#[no_mangle]
pub unsafe extern "C" fn P_FindHighestCeilingSurrounding(
    mut sec: *mut sector_t,
) -> fixed_t {
    let mut i: size_t = 0;
    let mut check: *mut line_t = 0 as *mut line_t;
    let mut other: *mut sector_t = 0 as *mut sector_t;
    let mut height: fixed_t = 0 as libc::c_int;
    let mut foundsector: int32_t = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        check = *((*sec).lines).offset(i as isize);
        other = getNextSector(check, sec);
        if !other.is_null() {
            if (*other).ceilingheight > height || foundsector == 0 {
                height = (*other).ceilingheight;
            }
            if foundsector == 0 {
                foundsector = 1 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return height;
}
unsafe extern "C" fn PolyDoor(mut line: *mut line_t) -> boolean {
    let mut pdd: polydoordata_t = polydoordata_s {
        polyObjNum: 0,
        doorType: 0,
        speed: 0,
        angle: 0,
        distance: 0,
        delay: 0,
    };
    pdd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    match (*line).special as libc::c_int {
        480 => {
            pdd.doorType = POLY_DOOR_SLIDE as libc::c_int;
            pdd
                .speed = (*line).args[1 as libc::c_int as usize]
                << 16 as libc::c_int - 3 as libc::c_int;
            pdd
                .angle = R_PointToAngle2(
                (*(*line).v1).x,
                (*(*line).v1).y,
                (*(*line).v2).x,
                (*(*line).v2).y,
            );
            pdd.distance = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
            pdd.delay = (*line).args[3 as libc::c_int as usize];
        }
        481 => {
            pdd.doorType = POLY_DOOR_SWING as libc::c_int;
            pdd.speed = (*line).args[1 as libc::c_int as usize];
            pdd.distance = (*line).args[2 as libc::c_int as usize];
            pdd.delay = (*line).args[3 as libc::c_int as usize];
        }
        _ => return 0 as libc::c_int,
    }
    return EV_DoPolyDoor(&mut pdd);
}
unsafe extern "C" fn PolyMove(mut line: *mut line_t) -> boolean {
    let mut pmd: polymovedata_t = polymovedata_s {
        polyObjNum: 0,
        distance: 0,
        speed: 0,
        angle: 0,
        overRide: 0,
    };
    pmd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    pmd
        .speed = (*line).args[1 as libc::c_int as usize]
        << 16 as libc::c_int - 3 as libc::c_int;
    pmd
        .angle = R_PointToAngle2(
        (*(*line).v1).x,
        (*(*line).v1).y,
        (*(*line).v2).x,
        (*(*line).v2).y,
    );
    pmd.distance = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
    pmd
        .overRide = ((*line).args[3 as libc::c_int as usize] != 0) as libc::c_int
        as uint8_t;
    return EV_DoPolyObjMove(&mut pmd);
}
unsafe extern "C" fn PolySetVisibilityTangibility(mut line: *mut line_t) {
    let mut polyObjNum: int32_t = (*line).args[0 as libc::c_int as usize];
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    po = Polyobj_GetForNum(polyObjNum);
    if po.is_null() {
        CONS_Debug(
            0x40 as libc::c_int,
            b"PolySetVisibilityTangibility: bad polyobj %d\n\0" as *const u8
                as *const libc::c_char,
            polyObjNum,
        );
        return;
    }
    if (*po).isBad != 0 {
        return;
    }
    if (*line).args[1 as libc::c_int as usize] == TMPV_VISIBLE as libc::c_int {
        (*po).flags &= !(POF_NOSPECIALS as libc::c_int);
        (*po).flags |= (*po).spawnflags & POF_RENDERALL as libc::c_int;
    } else if (*line).args[1 as libc::c_int as usize] == TMPV_INVISIBLE as libc::c_int {
        (*po).flags |= POF_NOSPECIALS as libc::c_int;
        (*po).flags &= !(POF_RENDERALL as libc::c_int);
    }
    if (*line).args[2 as libc::c_int as usize] == TMPT_TANGIBLE as libc::c_int {
        (*po).flags |= POF_SOLID as libc::c_int;
    } else if (*line).args[2 as libc::c_int as usize] == TMPT_INTANGIBLE as libc::c_int {
        (*po).flags &= !(POF_SOLID as libc::c_int);
    }
}
unsafe extern "C" fn PolyTranslucency(mut line: *mut line_t) {
    let mut polyObjNum: int32_t = (*line).args[0 as libc::c_int as usize];
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    po = Polyobj_GetForNum(polyObjNum);
    if po.is_null() {
        CONS_Debug(
            0x40 as libc::c_int,
            b"PolyTranslucency: bad polyobj %d\n\0" as *const u8 as *const libc::c_char,
            polyObjNum,
        );
        return;
    }
    if (*po).isBad != 0 {
        return;
    }
    if (*lines).args[2 as libc::c_int as usize] != 0 {
        (*po).translucency += (*line).args[1 as libc::c_int as usize];
    } else {
        (*po).translucency = (*line).args[1 as libc::c_int as usize];
    }
    (*po)
        .translucency = if (if (*po).translucency < NUMTRANSMAPS as libc::c_int {
        (*po).translucency
    } else {
        NUMTRANSMAPS as libc::c_int
    }) > 0 as libc::c_int
    {
        if (*po).translucency < NUMTRANSMAPS as libc::c_int {
            (*po).translucency
        } else {
            NUMTRANSMAPS as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
}
unsafe extern "C" fn PolyFade(mut line: *mut line_t) -> boolean {
    let mut polyObjNum: int32_t = (*line).args[0 as libc::c_int as usize];
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut pfd: polyfadedata_t = polyfadedata_s {
        polyObjNum: 0,
        destvalue: 0,
        docollision: 0,
        doghostfade: 0,
        ticbased: 0,
        speed: 0,
    };
    po = Polyobj_GetForNum(polyObjNum);
    if po.is_null() {
        CONS_Debug(
            0x40 as libc::c_int,
            b"PolyFade: bad polyobj %d\n\0" as *const u8 as *const libc::c_char,
            polyObjNum,
        );
        return 0 as libc::c_int;
    }
    if (*po).isBad != 0 {
        return 0 as libc::c_int;
    }
    if (*line).args[3 as libc::c_int as usize] & TMPF_OVERRIDE as libc::c_int == 0
        && !((*po).thinker).is_null()
        && (*(*po).thinker).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut polyfade_t) -> ()>,
                actionf_p1,
            >(Some(T_PolyObjFade as unsafe extern "C" fn(*mut polyfade_t) -> ()))
    {
        CONS_Debug(
            0x40 as libc::c_int,
            b"Line type 492 Executor: Fade PolyObject thinker already exists\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    pfd.polyObjNum = polyObjNum;
    if (*line).args[3 as libc::c_int as usize] & TMPF_RELATIVE as libc::c_int != 0 {
        pfd.destvalue = (*po).translucency + (*line).args[1 as libc::c_int as usize];
    } else {
        pfd.destvalue = (*line).args[1 as libc::c_int as usize];
    }
    pfd
        .destvalue = if (if pfd.destvalue < NUMTRANSMAPS as libc::c_int {
        pfd.destvalue
    } else {
        NUMTRANSMAPS as libc::c_int
    }) > 0 as libc::c_int
    {
        if pfd.destvalue < NUMTRANSMAPS as libc::c_int {
            pfd.destvalue
        } else {
            NUMTRANSMAPS as libc::c_int
        }
    } else {
        0 as libc::c_int
    };
    if (*po).translucency == pfd.destvalue {
        return 1 as libc::c_int;
    }
    pfd
        .docollision = ((*line).args[3 as libc::c_int as usize]
        & TMPF_IGNORECOLLISION as libc::c_int == 0) as libc::c_int;
    pfd
        .doghostfade = (*line).args[3 as libc::c_int as usize]
        & TMPF_GHOSTFADE as libc::c_int;
    pfd
        .ticbased = (*line).args[3 as libc::c_int as usize]
        & TMPF_TICBASED as libc::c_int;
    pfd.speed = (*line).args[2 as libc::c_int as usize];
    return EV_DoPolyObjFade(&mut pfd);
}
unsafe extern "C" fn PolyWaypoint(mut line: *mut line_t) -> boolean {
    let mut pwd: polywaypointdata_t = polywaypointdata_s {
        polyObjNum: 0,
        sequence: 0,
        speed: 0,
        returnbehavior: 0,
        flags: 0,
    };
    pwd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    pwd
        .speed = (*line).args[1 as libc::c_int as usize]
        << 16 as libc::c_int - 3 as libc::c_int;
    pwd.sequence = (*line).args[2 as libc::c_int as usize];
    pwd.returnbehavior = (*line).args[3 as libc::c_int as usize] as uint8_t;
    pwd.flags = (*line).args[4 as libc::c_int as usize] as uint8_t;
    return EV_DoPolyObjWaypoint(&mut pwd);
}
unsafe extern "C" fn PolyRotate(mut line: *mut line_t) -> boolean {
    let mut prd: polyrotdata_t = polyrotdata_s {
        polyObjNum: 0,
        direction: 0,
        speed: 0,
        distance: 0,
        flags: 0,
    };
    prd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    prd.speed = (*line).args[1 as libc::c_int as usize];
    prd.distance = abs((*line).args[2 as libc::c_int as usize]);
    prd
        .direction = if (*line).args[2 as libc::c_int as usize] < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    };
    prd.flags = (*line).args[3 as libc::c_int as usize] as uint8_t;
    return EV_DoPolyObjRotate(&mut prd);
}
unsafe extern "C" fn PolyFlag(mut line: *mut line_t) -> boolean {
    let mut pfd: polyflagdata_t = polyflagdata_s {
        polyObjNum: 0,
        speed: 0,
        angle: 0,
        momx: 0,
    };
    pfd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    pfd.speed = (*line).args[1 as libc::c_int as usize];
    pfd.angle = (*line).angle >> 19 as libc::c_int;
    pfd.momx = (*line).args[2 as libc::c_int as usize];
    return EV_DoPolyObjFlag(&mut pfd);
}
unsafe extern "C" fn PolyDisplace(mut line: *mut line_t) -> boolean {
    let mut pdd: polydisplacedata_t = polydisplacedata_s {
        polyObjNum: 0,
        controlSector: 0 as *mut sector_s,
        dx: 0,
        dy: 0,
    };
    let mut length: fixed_t = R_PointToDist2(
        (*(*line).v2).x,
        (*(*line).v2).y,
        (*(*line).v1).x,
        (*(*line).v1).y,
    );
    let mut speed: fixed_t = (*line).args[1 as libc::c_int as usize]
        << 16 as libc::c_int;
    pdd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    pdd.controlSector = (*line).frontsector;
    pdd.dx = FixedMul(FixedDiv((*line).dx, length), speed) >> 8 as libc::c_int;
    pdd.dy = FixedMul(FixedDiv((*line).dy, length), speed) >> 8 as libc::c_int;
    return EV_DoPolyObjDisplace(&mut pdd);
}
unsafe extern "C" fn PolyRotDisplace(mut line: *mut line_t) -> boolean {
    let mut pdd: polyrotdisplacedata_t = polyrotdisplacedata_s {
        polyObjNum: 0,
        controlSector: 0 as *mut sector_s,
        rotscale: 0,
        turnobjs: 0,
    };
    let mut anginter: fixed_t = 0;
    let mut distinter: fixed_t = 0;
    pdd.polyObjNum = (*line).args[0 as libc::c_int as usize];
    pdd.controlSector = (*line).frontsector;
    anginter = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
    distinter = (*line).args[1 as libc::c_int as usize] << 16 as libc::c_int;
    pdd.rotscale = FixedDiv(anginter, distinter);
    pdd.turnobjs = (*line).args[3 as libc::c_int as usize] as uint8_t;
    return EV_DoPolyObjRotDisplace(&mut pdd);
}
#[no_mangle]
pub unsafe extern "C" fn P_RunNightserizeExecutors(mut actor: *mut mobj_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if (*lines.offset(i as isize)).special as libc::c_int == 323 as libc::c_int {
            P_RunTriggerLinedef(
                &mut *lines.offset(i as isize),
                actor,
                0 as *mut sector_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RunDeNightserizeExecutors(mut actor: *mut mobj_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if (*lines.offset(i as isize)).special as libc::c_int == 325 as libc::c_int {
            P_RunTriggerLinedef(
                &mut *lines.offset(i as isize),
                actor,
                0 as *mut sector_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RunNightsLapExecutors(mut actor: *mut mobj_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if (*lines.offset(i as isize)).special as libc::c_int == 327 as libc::c_int {
            P_RunTriggerLinedef(
                &mut *lines.offset(i as isize),
                actor,
                0 as *mut sector_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_RunNightsCapsuleTouchExecutors(
    mut actor: *mut mobj_t,
    mut entering: boolean,
    mut enoughspheres: boolean,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if !((*lines.offset(i as isize)).special as libc::c_int != 329 as libc::c_int) {
            if !(((*lines.offset(i as isize)).args[7 as libc::c_int as usize]
                & TMI_ENTER as libc::c_int != 0) as libc::c_int != entering)
            {
                if !((*lines.offset(i as isize)).args[6 as libc::c_int as usize]
                    == TMS_IFENOUGH as libc::c_int && enoughspheres == 0)
                {
                    if !((*lines.offset(i as isize)).args[6 as libc::c_int as usize]
                        == TMS_IFNOTENOUGH as libc::c_int && enoughspheres != 0)
                    {
                        P_RunTriggerLinedef(
                            &mut *lines.offset(i as isize),
                            actor,
                            0 as *mut sector_t,
                        );
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_FindMinSurroundingLight(
    mut sector: *mut sector_t,
    mut max: int32_t,
) -> int32_t {
    let mut i: size_t = 0;
    let mut min: int32_t = max;
    let mut line: *mut line_t = 0 as *mut line_t;
    let mut check: *mut sector_t = 0 as *mut sector_t;
    i = 0 as libc::c_int as size_t;
    while i < (*sector).linecount {
        line = *((*sector).lines).offset(i as isize);
        check = getNextSector(line, sector);
        if !check.is_null() {
            if ((*check).lightlevel as libc::c_int) < min {
                min = (*check).lightlevel as int32_t;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return min;
}
#[no_mangle]
pub unsafe extern "C" fn T_ExecutorDelay(mut e: *mut executor_t) {
    (*e).timer -= 1;
    if (*e).timer <= 0 as libc::c_int {
        if !((*e).caller).is_null() && P_MobjWasRemoved((*e).caller) != 0 {
            P_SetTarget2(&mut (*e).caller, 0 as *mut mobj_t);
        }
        P_ProcessLineSpecial((*e).line, (*e).caller, (*e).sector);
        P_SetTarget2(&mut (*e).caller, 0 as *mut mobj_t);
        P_RemoveThinker(&mut (*e).thinker);
    }
}
unsafe extern "C" fn P_AddExecutorDelay(
    mut line: *mut line_t,
    mut mobj: *mut mobj_t,
    mut sector: *mut sector_t,
) {
    let mut e: *mut executor_t = 0 as *mut executor_t;
    let mut delay: int32_t = 0;
    if udmf != 0 {
        delay = (*line).executordelay;
    } else {
        if ((*line).backsector).is_null() {
            I_Error(
                b"P_AddExecutorDelay: Line has no backsector!\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        delay = ((*(*line).backsector).ceilingheight >> 16 as libc::c_int)
            + ((*(*line).backsector).floorheight >> 16 as libc::c_int);
    }
    e = Z_CallocAlign(
        ::core::mem::size_of::<executor_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut executor_t;
    (*e)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut executor_t) -> ()>,
        actionf_p1,
    >(Some(T_ExecutorDelay as unsafe extern "C" fn(*mut executor_t) -> ()));
    (*e).line = line;
    (*e).sector = sector;
    (*e).timer = delay;
    P_SetTarget2(&mut (*e).caller, mobj);
    P_AddThinker(THINK_MAIN, &mut (*e).thinker);
}
unsafe extern "C" fn P_CheckNightsTriggerLine(
    mut triggerline: *mut line_t,
    mut actor: *mut mobj_t,
) -> boolean {
    let mut specialtype: int16_t = (*triggerline).special;
    let mut i: size_t = 0;
    let mut inputmare: uint8_t = (if 0 as libc::c_int
        > (if (255 as libc::c_int) < (*triggerline).args[1 as libc::c_int as usize] {
            255 as libc::c_int
        } else {
            (*triggerline).args[1 as libc::c_int as usize]
        })
    {
        0 as libc::c_int
    } else if (255 as libc::c_int) < (*triggerline).args[1 as libc::c_int as usize] {
        255 as libc::c_int
    } else {
        (*triggerline).args[1 as libc::c_int as usize]
    }) as uint8_t;
    let mut inputlap: uint8_t = (if 0 as libc::c_int
        > (if (255 as libc::c_int) < (*triggerline).args[2 as libc::c_int as usize] {
            255 as libc::c_int
        } else {
            (*triggerline).args[2 as libc::c_int as usize]
        })
    {
        0 as libc::c_int
    } else if (255 as libc::c_int) < (*triggerline).args[2 as libc::c_int as usize] {
        255 as libc::c_int
    } else {
        (*triggerline).args[2 as libc::c_int as usize]
    }) as uint8_t;
    let mut marecomp: textmapcomparison_t = (*triggerline)
        .args[3 as libc::c_int as usize] as textmapcomparison_t;
    let mut lapcomp: textmapcomparison_t = (*triggerline).args[4 as libc::c_int as usize]
        as textmapcomparison_t;
    let mut checkplayer: textmapnightsplayer_t = (*triggerline)
        .args[5 as libc::c_int as usize] as textmapnightsplayer_t;
    let mut lapfrombonustime: boolean = 0;
    let mut donomares: boolean = (specialtype as libc::c_int == 323 as libc::c_int
        && (*triggerline).args[7 as libc::c_int as usize]
            & TMN_LEVELCOMPLETION as libc::c_int != 0) as libc::c_int;
    let mut currentmare: uint8_t = 255 as libc::c_int as uint8_t;
    let mut currentlap: uint8_t = 255 as libc::c_int as uint8_t;
    match specialtype as libc::c_int {
        323 => {
            lapfrombonustime = ((*triggerline).args[7 as libc::c_int as usize]
                & TMN_BONUSLAPS as libc::c_int != 0) as libc::c_int;
        }
        325 => {
            lapfrombonustime = ((*triggerline).args[7 as libc::c_int as usize] != 0)
                as libc::c_int;
        }
        327 => {
            lapfrombonustime = ((*triggerline).args[6 as libc::c_int as usize] != 0)
                as libc::c_int;
        }
        329 => {
            lapfrombonustime = ((*triggerline).args[7 as libc::c_int as usize]
                & TMI_BONUSLAPS as libc::c_int != 0) as libc::c_int;
        }
        _ => {
            lapfrombonustime = false_0 as libc::c_int;
        }
    }
    if specialtype as libc::c_int == 323 as libc::c_int {
        if donomares != 0 && P_FindLowestMare() as libc::c_int != 255 as libc::c_int {
            return false_0 as libc::c_int;
        }
        if donomares == 0 && P_FindLowestMare() as libc::c_int == 255 as libc::c_int {
            return false_0 as libc::c_int;
        }
        if (*triggerline).args[6 as libc::c_int as usize]
            == TMN_FROMNONIGHTS as libc::c_int
        {
            if ((*actor).player).is_null() {
                return false_0 as libc::c_int
            } else if (*(*actor).player).powers[pw_carry as libc::c_int as usize]
                as libc::c_int == CR_NIGHTSMODE as libc::c_int
            {
                return false_0 as libc::c_int
            }
        } else if (*triggerline).args[6 as libc::c_int as usize]
            == TMN_FROMNIGHTS as libc::c_int
        {
            if ((*actor).player).is_null() {
                return false_0 as libc::c_int
            } else if (*(*actor).player).powers[pw_carry as libc::c_int as usize]
                as libc::c_int != CR_NIGHTSMODE as libc::c_int
            {
                return false_0 as libc::c_int
            }
        }
    }
    if checkplayer as libc::c_uint != TMNP_TRIGGERER as libc::c_int as libc::c_uint
        || specialtype as libc::c_int == 325 as libc::c_int
            && (*triggerline).args[6 as libc::c_int as usize]
                != TMD_ALWAYS as libc::c_int
    {
        let mut playersarenights: uint8_t = 0 as libc::c_int as uint8_t;
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as size_t {
            let mut lap: uint8_t = 0;
            if !(playeringame[i as usize] == 0 || players[i as usize].spectator != 0) {
                if specialtype as libc::c_int == 325 as libc::c_int
                    && (*triggerline).args[6 as libc::c_int as usize]
                        == TMD_NOBODYNIGHTS as libc::c_int
                    && players[i as usize].powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int
                {
                    return false_0 as libc::c_int;
                }
                if specialtype as libc::c_int == 325 as libc::c_int
                    && (*triggerline).args[6 as libc::c_int as usize]
                        == TMD_SOMEBODYNIGHTS as libc::c_int
                    && players[i as usize].powers[pw_carry as libc::c_int as usize]
                        as libc::c_int == CR_NIGHTSMODE as libc::c_int
                {
                    playersarenights = playersarenights.wrapping_add(1);
                    playersarenights;
                }
                lap = (if lapfrombonustime != 0 {
                    players[i as usize].marebonuslap as libc::c_int
                } else {
                    players[i as usize].marelap as libc::c_int
                }) as uint8_t;
                if checkplayer as libc::c_uint
                    == TMNP_FASTEST as libc::c_int as libc::c_uint
                {
                    if players[i as usize].mare as libc::c_int
                        > currentmare as libc::c_int
                        || currentmare as libc::c_int == 255 as libc::c_int
                    {
                        currentmare = players[i as usize].mare;
                        currentlap = 255 as libc::c_int as uint8_t;
                    }
                    if players[i as usize].mare as libc::c_int
                        == currentmare as libc::c_int
                        && (lap as libc::c_int > currentlap as libc::c_int
                            || currentlap as libc::c_int == 255 as libc::c_int)
                    {
                        currentlap = lap;
                    }
                } else if checkplayer as libc::c_uint
                    == TMNP_SLOWEST as libc::c_int as libc::c_uint
                {
                    if (players[i as usize].mare as libc::c_int)
                        < currentmare as libc::c_int
                        || currentmare as libc::c_int == 255 as libc::c_int
                    {
                        currentmare = players[i as usize].mare;
                        currentlap = 255 as libc::c_int as uint8_t;
                    }
                    if players[i as usize].mare as libc::c_int
                        == currentmare as libc::c_int
                        && ((lap as libc::c_int) < currentlap as libc::c_int
                            || currentlap as libc::c_int == 255 as libc::c_int)
                    {
                        currentlap = lap;
                    }
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        if specialtype as libc::c_int == 325 as libc::c_int
            && (*triggerline).args[6 as libc::c_int as usize]
                == TMD_SOMEBODYNIGHTS as libc::c_int
            && (playersarenights as libc::c_int) < 1 as libc::c_int
        {
            return false_0 as libc::c_int;
        }
    } else if checkplayer as libc::c_uint
        == TMNP_TRIGGERER as libc::c_int as libc::c_uint
    {
        if ((*actor).player).is_null() {
            return false_0 as libc::c_int;
        }
        currentmare = (*(*actor).player).mare;
        currentlap = (if lapfrombonustime != 0 {
            (*(*actor).player).marebonuslap as libc::c_int
        } else {
            (*(*actor).player).marelap as libc::c_int
        }) as uint8_t;
    }
    if lapfrombonustime != 0 && currentlap == 0 {
        return false_0 as libc::c_int;
    }
    if donomares == 0
        && (marecomp as libc::c_uint == TMC_LTE as libc::c_int as libc::c_uint
            && currentmare as libc::c_int > inputmare as libc::c_int
            || marecomp as libc::c_uint == TMC_GTE as libc::c_int as libc::c_uint
                && (currentmare as libc::c_int) < inputmare as libc::c_int
            || marecomp as libc::c_uint == TMC_EQUAL as libc::c_int as libc::c_uint
                && currentmare as libc::c_int != inputmare as libc::c_int
            || lapcomp as libc::c_uint == TMC_LTE as libc::c_int as libc::c_uint
                && currentlap as libc::c_int > inputlap as libc::c_int
            || lapcomp as libc::c_uint == TMC_GTE as libc::c_int as libc::c_uint
                && (currentlap as libc::c_int) < inputlap as libc::c_int
            || lapcomp as libc::c_uint == TMC_EQUAL as libc::c_int as libc::c_uint
                && currentlap as libc::c_int != inputlap as libc::c_int)
    {
        return false_0 as libc::c_int;
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn P_CheckPlayerMareOld(mut triggerline: *mut line_t) -> boolean {
    let mut mare: uint8_t = 0;
    let mut targetmare: int32_t = P_AproxDistance((*triggerline).dx, (*triggerline).dy)
        >> 16 as libc::c_int;
    if maptol & TOL_NIGHTS as libc::c_int as uint32_t == 0 {
        return false_0 as libc::c_int;
    }
    mare = P_FindLowestMare();
    if (*triggerline).flags as libc::c_int & 64 as libc::c_int != 0 {
        return (mare as libc::c_int <= targetmare) as libc::c_int;
    }
    if (*triggerline).flags as libc::c_int & 2 as libc::c_int != 0 {
        return (mare as libc::c_int >= targetmare) as libc::c_int;
    }
    return (mare as libc::c_int == targetmare) as libc::c_int;
}
unsafe extern "C" fn P_CheckPlayerMare(mut triggerline: *mut line_t) -> boolean {
    let mut mare: uint8_t = 0;
    let mut targetmare: int32_t = (*triggerline).args[1 as libc::c_int as usize];
    if maptol & TOL_NIGHTS as libc::c_int as uint32_t == 0 {
        return false_0 as libc::c_int;
    }
    mare = P_FindLowestMare();
    match (*triggerline).args[2 as libc::c_int as usize] {
        2 => return (mare as libc::c_int >= targetmare) as libc::c_int,
        1 => return (mare as libc::c_int <= targetmare) as libc::c_int,
        0 | _ => return (mare as libc::c_int == targetmare) as libc::c_int,
    };
}
unsafe extern "C" fn P_CheckPlayerRings(
    mut triggerline: *mut line_t,
    mut actor: *mut mobj_t,
) -> boolean {
    let mut rings: int32_t = 0 as libc::c_int;
    let mut targetrings: int32_t = (*triggerline).args[1 as libc::c_int as usize];
    let mut i: size_t = 0;
    if (*triggerline).args[3 as libc::c_int as usize] != 0 {
        i = 0 as libc::c_int as size_t;
        while i < 32 as libc::c_int as size_t {
            if !(playeringame[i as usize] == 0 || players[i as usize].spectator != 0) {
                if !((players[i as usize].mo).is_null()
                    || (if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
                        players[i as usize].spheres as libc::c_int
                    } else {
                        players[i as usize].rings as libc::c_int
                    }) <= 0 as libc::c_int)
                {
                    rings
                        += if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
                            players[i as usize].spheres as libc::c_int
                        } else {
                            players[i as usize].rings as libc::c_int
                        };
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    } else {
        if !(!actor.is_null() && !((*actor).player).is_null()) {
            return false_0 as libc::c_int;
        }
        rings = if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
            (*(*actor).player).spheres as libc::c_int
        } else {
            (*(*actor).player).rings as libc::c_int
        };
    }
    match (*triggerline).args[2 as libc::c_int as usize] {
        2 => return (rings >= targetrings) as libc::c_int,
        1 => return (rings <= targetrings) as libc::c_int,
        0 | _ => return (rings == targetrings) as libc::c_int,
    };
}
unsafe extern "C" fn P_CheckPushables(
    mut triggerline: *mut line_t,
    mut caller: *mut sector_t,
) -> boolean {
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut numpushables: int32_t = 0 as libc::c_int;
    let mut targetpushables: int32_t = (*triggerline).args[1 as libc::c_int as usize];
    if caller.is_null() {
        return false_0 as libc::c_int;
    }
    node = (*caller).touching_thinglist;
    while !node.is_null() {
        mo = (*node).m_thing;
        if (*mo).flags & MF_PUSHABLE as libc::c_int as uint32_t != 0
            || (*(*mo).info).flags & MF_PUSHABLE as libc::c_int as uint32_t != 0
                && (*mo).fuse != 0
        {
            numpushables += 1;
            numpushables;
        }
        node = (*node).m_thinglist_next;
    }
    match (*triggerline).args[2 as libc::c_int as usize] {
        2 => return (numpushables >= targetpushables) as libc::c_int,
        1 => return (numpushables <= targetpushables) as libc::c_int,
        0 | _ => return (numpushables == targetpushables) as libc::c_int,
    };
}
unsafe extern "C" fn P_CheckEmeralds(
    mut checktype: int32_t,
    mut target: uint16_t,
) -> boolean {
    match checktype {
        1 => return (emeralds as libc::c_int & target as libc::c_int != 0) as libc::c_int,
        2 => return (emeralds as libc::c_int == target as libc::c_int) as libc::c_int,
        3 => {
            return (emeralds as libc::c_int & target as libc::c_int
                != target as libc::c_int) as libc::c_int;
        }
        4 => return (emeralds as libc::c_int & target as libc::c_int == 0) as libc::c_int,
        0 | _ => {
            return (emeralds as libc::c_int & target as libc::c_int
                == target as libc::c_int) as libc::c_int;
        }
    };
}
unsafe extern "C" fn P_ActivateLinedefExecutor(
    mut line: *mut line_t,
    mut actor: *mut mobj_t,
    mut caller: *mut sector_t,
) {
    if ((*line).special as libc::c_int) < 400 as libc::c_int
        || (*line).special as libc::c_int >= 500 as libc::c_int
    {
        return;
    }
    if (*line).executordelay != 0 {
        P_AddExecutorDelay(line, actor, caller);
    } else {
        P_ProcessLineSpecial(line, actor, caller);
    };
}
unsafe extern "C" fn P_ActivateLinedefExecutorsInSector(
    mut triggerline: *mut line_t,
    mut actor: *mut mobj_t,
    mut caller: *mut sector_t,
) -> boolean {
    let mut ctlsector: *mut sector_t = (*triggerline).frontsector;
    let mut sectori: size_t = ctlsector.offset_from(sectors) as libc::c_long as size_t;
    let mut linecnt: size_t = (*ctlsector).linecount;
    let mut i: size_t = 0;
    if udmf == 0 && (*triggerline).flags as libc::c_int & 1024 as libc::c_int != 0 {
        i = 0 as libc::c_int as size_t;
        while i < linecnt {
            P_ActivateLinedefExecutor(
                *((*ctlsector).lines).offset(i as isize),
                actor,
                caller,
            );
            i = i.wrapping_add(1);
            i;
        }
    } else {
        let mut backwards: boolean = false_0 as libc::c_int;
        let mut j: size_t = 0;
        let mut masterlineindex: size_t = -(1 as libc::c_int) as size_t;
        i = 0 as libc::c_int as size_t;
        while i < linecnt {
            if *((*ctlsector).lines).offset(i as isize) == triggerline {
                masterlineindex = i;
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        loop {
            if backwards != 0 {
                j = 0 as libc::c_int as size_t;
                while j < linecnt {
                    if !(i == j) {
                        if (**((*ctlsector).lines).offset(i as isize)).v1
                            == (**((*ctlsector).lines).offset(j as isize)).v2
                        {
                            i = j;
                            break;
                        } else if (**((*ctlsector).lines).offset(i as isize)).v1
                            == (**((*ctlsector).lines).offset(j as isize)).v1
                        {
                            i = j;
                            backwards = false_0 as libc::c_int;
                            break;
                        }
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if j == linecnt {
                    let vertexei: size_t = ((**((*ctlsector).lines).offset(i as isize))
                        .v1)
                        .offset_from(vertexes) as libc::c_long as size_t;
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Warning: Sector %s is not closed at vertex %s (%d, %d)\n\0"
                            as *const u8 as *const libc::c_char,
                        sizeu1(sectori),
                        sizeu2(vertexei),
                        (*(**((*ctlsector).lines).offset(i as isize)).v1).x,
                        (*(**((*ctlsector).lines).offset(i as isize)).v1).y,
                    );
                    return false_0 as libc::c_int;
                }
            } else {
                j = 0 as libc::c_int as size_t;
                while j < linecnt {
                    if !(i == j) {
                        if (**((*ctlsector).lines).offset(i as isize)).v2
                            == (**((*ctlsector).lines).offset(j as isize)).v1
                        {
                            i = j;
                            break;
                        } else if (**((*ctlsector).lines).offset(i as isize)).v2
                            == (**((*ctlsector).lines).offset(j as isize)).v2
                        {
                            i = j;
                            backwards = true_0 as libc::c_int;
                            break;
                        }
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                if j == linecnt {
                    let vertexei_0: size_t = ((**((*ctlsector).lines).offset(i as isize))
                        .v1)
                        .offset_from(vertexes) as libc::c_long as size_t;
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Warning: Sector %s is not closed at vertex %s (%d, %d)\n\0"
                            as *const u8 as *const libc::c_char,
                        sizeu1(sectori),
                        sizeu2(vertexei_0),
                        (*(**((*ctlsector).lines).offset(i as isize)).v2).x,
                        (*(**((*ctlsector).lines).offset(i as isize)).v2).y,
                    );
                    return false_0 as libc::c_int;
                }
            }
            if i == masterlineindex {
                break;
            }
            P_ActivateLinedefExecutor(
                *((*ctlsector).lines).offset(i as isize),
                actor,
                caller,
            );
        }
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_RunTriggerLinedef(
    mut triggerline: *mut line_t,
    mut actor: *mut mobj_t,
    mut caller: *mut sector_t,
) {
    let mut specialtype: int16_t = (*triggerline).special;
    if !caller.is_null() && udmf == 0 {
        if (*caller).triggerer as libc::c_int == TO_PLAYEREMERALDS as libc::c_int {
            if !(emeralds as libc::c_int
                & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                    | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                    | 64 as libc::c_int)
                == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                    | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                    | 64 as libc::c_int)
            {
                return;
            }
        } else if (*caller).triggerer as libc::c_int == TO_PLAYERNIGHTS as libc::c_int {
            if P_CheckPlayerMareOld(triggerline) == 0 {
                return;
            }
        }
    }
    let mut current_block_56: u64;
    match specialtype as libc::c_int {
        303 => {
            if P_CheckPlayerRings(triggerline, actor) == 0 {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        305 => {
            if !(!actor.is_null() && !((*actor).player).is_null()
                && (*(*actor).player).charability as libc::c_int
                    == (*triggerline).args[1 as libc::c_int as usize])
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        309 => {
            if !(!actor.is_null() && !((*actor).player).is_null()) {
                return;
            }
            if (*(*actor).player).ctfteam
                != (if (*triggerline).args[1 as libc::c_int as usize]
                    == TMT_RED as libc::c_int
                {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                })
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        314 => {
            if P_CheckPushables(triggerline, caller) == 0 {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        317 => {
            let mut trigid: int32_t = (*triggerline).args[1 as libc::c_int as usize];
            if trigid < 0 as libc::c_int || trigid > 31 as libc::c_int {
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Unlockable trigger (sidedef %hu): bad trigger ID %d\n\0"
                        as *const u8 as *const libc::c_char,
                    (*triggerline).sidenum[0 as libc::c_int as usize] as libc::c_int,
                    trigid,
                );
                return;
            } else if unlocktriggers & ((1 as libc::c_int) << trigid) as uint32_t == 0 {
                return
            }
            current_block_56 = 16415152177862271243;
        }
        319 => {
            let mut unlockid: int32_t = (*triggerline).args[1 as libc::c_int as usize];
            if unlockid <= 0 as libc::c_int || unlockid > 80 as libc::c_int {
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Unlockable check (sidedef %hu): bad unlockable ID %d\n\0"
                        as *const u8 as *const libc::c_char,
                    (*triggerline).sidenum[0 as libc::c_int as usize] as libc::c_int,
                    unlockid,
                );
                return;
            } else if (*serverGamedata).unlocked[(unlockid - 1 as libc::c_int) as usize]
                == 0
            {
                return
            }
            current_block_56 = 16415152177862271243;
        }
        321 => {
            if (*triggerline).callcount as libc::c_int > 0 as libc::c_int {
                (*triggerline).callcount -= 1;
                if (*triggerline).callcount as libc::c_int > 0 as libc::c_int {
                    return;
                }
            }
            current_block_56 = 16415152177862271243;
        }
        323 => {
            current_block_56 = 8387747836218688260;
        }
        325 => {
            current_block_56 = 8387747836218688260;
        }
        327 | 329 => {
            current_block_56 = 15268509009714454502;
        }
        331 => {
            if !(!actor.is_null() && !((*actor).player).is_null()) {
                return;
            }
            if ((*triggerline).stringargs[0 as libc::c_int as usize]).is_null() {
                return;
            }
            if !(strcasecmp(
                (*triggerline).stringargs[0 as libc::c_int as usize],
                (skins[(*(*actor).player).skin as usize].name).as_mut_ptr(),
            ) == 0 as libc::c_int) as libc::c_int
                ^ ((*triggerline).args[1 as libc::c_int as usize] != 0) as libc::c_int
                != 0
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        334 => {
            let mut triggercolor: int32_t = if !((*triggerline)
                .stringargs[0 as libc::c_int as usize])
                .is_null()
            {
                get_number((*triggerline).stringargs[0 as libc::c_int as usize])
            } else {
                SKINCOLOR_NONE as libc::c_int
            };
            let mut color: uint16_t = (if !((*actor).player).is_null() {
                (*(*actor).player).powers[pw_dye as libc::c_int as usize] as libc::c_int
            } else {
                (*actor).color as libc::c_int
            }) as uint16_t;
            if ((*triggerline).args[1 as libc::c_int as usize] != 0) as libc::c_int
                ^ (triggercolor != color as libc::c_int) as libc::c_int != 0
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        337 => {
            if P_CheckEmeralds(
                (*triggerline).args[2 as libc::c_int as usize],
                (*triggerline).args[1 as libc::c_int as usize] as uint16_t,
            ) == 0
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        340 => {
            if P_CheckPlayerMare(triggerline) == 0 {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        343 => {
            if (*triggerline).args[1 as libc::c_int as usize]
                == TMG_TEMPREVERSE as libc::c_int
                && ((*actor).flags2 & MF2_OBJECTFLIP as libc::c_int as uint32_t == 0)
                    as libc::c_int
                    != ((*(*actor).player)
                        .powers[pw_gravityboots as libc::c_int as usize] == 0)
                        as libc::c_int
            {
                return;
            }
            if ((*triggerline).args[1 as libc::c_int as usize]
                == TMG_NORMAL as libc::c_int) as libc::c_int
                != ((*actor).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int
                    == 0) as libc::c_int
            {
                return;
            }
            current_block_56 = 16415152177862271243;
        }
        _ => {
            current_block_56 = 16415152177862271243;
        }
    }
    match current_block_56 {
        8387747836218688260 => {
            current_block_56 = 15268509009714454502;
        }
        _ => {}
    }
    match current_block_56 {
        15268509009714454502 => {
            if P_CheckNightsTriggerLine(triggerline, actor) == 0 {
                return;
            }
        }
        _ => {}
    }
    if P_ActivateLinedefExecutorsInSector(triggerline, actor, caller) == 0 {
        return;
    }
    if specialtype as libc::c_int == 321 as libc::c_int
        && (*triggerline).args[2 as libc::c_int as usize] != 0
    {
        (*triggerline)
            .callcount = (*triggerline).args[1 as libc::c_int as usize] as int16_t;
    } else if specialtype as libc::c_int == 313 as libc::c_int
        || specialtype as libc::c_int == 321 as libc::c_int
        || specialtype as libc::c_int == 399 as libc::c_int
    {
        (*triggerline).special = 0 as libc::c_int as int16_t;
    } else if (specialtype as libc::c_int == 323 as libc::c_int
        || specialtype as libc::c_int == 325 as libc::c_int
        || specialtype as libc::c_int == 327 as libc::c_int
        || specialtype as libc::c_int == 329 as libc::c_int)
        && (*triggerline).args[0 as libc::c_int as usize] != 0
    {
        (*triggerline).special = 0 as libc::c_int as int16_t;
    } else if (specialtype as libc::c_int == 300 as libc::c_int
        || specialtype as libc::c_int == 303 as libc::c_int
        || specialtype as libc::c_int == 305 as libc::c_int
        || specialtype as libc::c_int == 308 as libc::c_int
        || specialtype as libc::c_int == 309 as libc::c_int
        || specialtype as libc::c_int == 314 as libc::c_int
        || specialtype as libc::c_int == 317 as libc::c_int
        || specialtype as libc::c_int == 319 as libc::c_int
        || specialtype as libc::c_int == 331 as libc::c_int
        || specialtype as libc::c_int == 334 as libc::c_int
        || specialtype as libc::c_int == 337 as libc::c_int
        || specialtype as libc::c_int == 343 as libc::c_int)
        && (*triggerline).args[0 as libc::c_int as usize] == TMT_ONCE as libc::c_int
    {
        (*triggerline).special = 0 as libc::c_int as int16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_LinedefExecute(
    mut tag: int16_t,
    mut actor: *mut mobj_t,
    mut caller: *mut sector_t,
) {
    let mut masterline: int32_t = 0;
    CONS_Debug(
        0x80 as libc::c_int,
        b"P_LinedefExecute: Executing trigger linedefs of tag %d\n\0" as *const u8
            as *const libc::c_char,
        tag as libc::c_int,
    );
    let mut ICNT_1930: size_t = 0 as libc::c_int as size_t;
    loop {
        masterline = Tag_Iterate_Lines(tag, ICNT_1930);
        if !(masterline >= 0 as libc::c_int) {
            break;
        }
        if !(((*lines.offset(masterline as isize)).special as libc::c_int)
            < 300 as libc::c_int
            || (*lines.offset(masterline as isize)).special as libc::c_int
                > 399 as libc::c_int)
        {
            if !((*lines.offset(masterline as isize)).special as libc::c_int
                == 313 as libc::c_int
                || (*lines.offset(masterline as isize)).special as libc::c_int
                    == 399 as libc::c_int)
            {
                if !(((*lines.offset(masterline as isize)).special as libc::c_int
                    == 300 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 303 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 305 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 308 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 309 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 314 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 317 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 319 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 331 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 334 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 337 as libc::c_int
                    || (*lines.offset(masterline as isize)).special as libc::c_int
                        == 343 as libc::c_int)
                    && (*lines.offset(masterline as isize))
                        .args[0 as libc::c_int as usize]
                        > TMT_EACHTIMEMASK as libc::c_int)
                {
                    if !((*lines.offset(masterline as isize)).special as libc::c_int
                        == 321 as libc::c_int
                        && (*lines.offset(masterline as isize))
                            .args[0 as libc::c_int as usize]
                            > TMXT_EACHTIMEMASK as libc::c_int)
                    {
                        P_RunTriggerLinedef(
                            &mut *lines.offset(masterline as isize),
                            actor,
                            caller,
                        );
                    }
                }
            }
        }
        ICNT_1930 = ICNT_1930.wrapping_add(1);
        ICNT_1930;
    };
}
unsafe extern "C" fn P_PlaySFX(
    mut sfxnum: int32_t,
    mut mo: *mut mobj_t,
    mut callsec: *mut sector_t,
    mut tag: int16_t,
    mut source: textmapsoundsource_t,
    mut listener: textmapsoundlistener_t,
) {
    if sfxnum == sfx_None as libc::c_int {
        return;
    }
    if sfxnum < sfx_None as libc::c_int || sfxnum >= NUMSFX as libc::c_int {
        CONS_Debug(
            0x80 as libc::c_int,
            b"Line type 414 Executor: sfx number %d is invalid!\n\0" as *const u8
                as *const libc::c_char,
            sfxnum,
        );
        return;
    }
    match listener as libc::c_uint {
        1 => {
            if mo.is_null() {
                return;
            }
            if ((*mo).player).is_null() {
                return;
            }
            if (*mo).player
                != &mut *players.as_mut_ptr().offset(displayplayer as isize)
                    as *mut player_t
                && (*mo).player
                    != &mut *players.as_mut_ptr().offset(secondarydisplayplayer as isize)
                        as *mut player_t
            {
                return;
            }
        }
        2 => {
            let mut i: uint8_t = 0 as libc::c_int as uint8_t;
            let mut camobj: *mut mobj_t = players[displayplayer as usize].mo;
            let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundit: boolean = false_0 as libc::c_int;
            i = 0 as libc::c_int as uint8_t;
            while (i as libc::c_int) < 2 as libc::c_int {
                if !camobj.is_null() {
                    if foundit != 0
                        || Tag_Find(&mut (*(*(*camobj).subsector).sector).tags, tag) != 0
                    {
                        foundit = true_0 as libc::c_int;
                        break;
                    } else {
                        rover = (*(*(*camobj).subsector).sector).ffloors;
                        while !rover.is_null() {
                            if !(Tag_Find(
                                &mut (*(*(*rover).master).frontsector).tags,
                                tag,
                            ) == 0)
                            {
                                if !((*camobj).z
                                    > P_MobjCeilingZ(
                                        camobj,
                                        sectors.offset((*rover).secnum as isize),
                                        (*(*camobj).subsector).sector,
                                        (*camobj).x,
                                        (*camobj).y,
                                        0 as *mut line_t,
                                        (sectors.offset((*rover).secnum as isize)
                                            == (*(*camobj).subsector).sector) as libc::c_int,
                                        true_0 as libc::c_int,
                                    ))
                                {
                                    if !((*camobj).z + (*camobj).height
                                        < P_MobjFloorZ(
                                            camobj,
                                            sectors.offset((*rover).secnum as isize),
                                            (*(*camobj).subsector).sector,
                                            (*camobj).x,
                                            (*camobj).y,
                                            0 as *mut line_t,
                                            (sectors.offset((*rover).secnum as isize)
                                                != (*(*camobj).subsector).sector) as libc::c_int,
                                            true_0 as libc::c_int,
                                        ))
                                    {
                                        foundit = true_0 as libc::c_int;
                                        break;
                                    }
                                }
                            }
                            rover = (*rover).next;
                        }
                    }
                }
                camobj = players[secondarydisplayplayer as usize].mo;
                i = i.wrapping_add(1);
                i;
            }
            if foundit == 0 {
                return;
            }
        }
        0 | _ => {}
    }
    match source as libc::c_uint {
        0 => {
            if !mo.is_null() {
                S_StartSound(mo as *const libc::c_void, sfxnum as sfxenum_t);
            }
        }
        1 => {
            if !callsec.is_null() {
                S_StartSound(
                    &mut (*callsec).soundorg as *mut degenmobj_t as *const libc::c_void,
                    sfxnum as sfxenum_t,
                );
            } else if !mo.is_null() {
                S_StartSound(
                    &mut (*(*(*mo).subsector).sector).soundorg as *mut degenmobj_t
                        as *const libc::c_void,
                    sfxnum as sfxenum_t,
                );
            }
        }
        2 => {
            S_StartSound(0 as *const libc::c_void, sfxnum as sfxenum_t);
        }
        3 => {
            let mut secnum: int32_t = 0;
            let mut ICNT_2053: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(tag, ICNT_2053);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                S_StartSound(
                    &mut (*sectors.offset(secnum as isize)).soundorg as *mut degenmobj_t
                        as *const libc::c_void,
                    sfxnum as sfxenum_t,
                );
                ICNT_2053 = ICNT_2053.wrapping_add(1);
                ICNT_2053;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_rain_type(mut weathernum: int32_t) -> boolean {
    match weathernum {
        2 | 3 | 1 | 6 | 4 => return true_0 as libc::c_int,
        _ => return false_0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_SwitchWeather(mut weathernum: int32_t) {
    let mut purge: boolean = true_0 as libc::c_int;
    if weathernum == curWeather {
        return;
    }
    if is_rain_type(weathernum) != 0 && is_rain_type(curWeather) != 0 {
        purge = false_0 as libc::c_int;
    }
    if purge != 0 {
        let mut think: *mut thinker_t = 0 as *mut thinker_t;
        let mut precipmobj: *mut precipmobj_t = 0 as *mut precipmobj_t;
        think = (*thlist.as_mut_ptr().offset(THINK_PRECIP as libc::c_int as isize)).next;
        while think
            != &mut *thlist.as_mut_ptr().offset(THINK_PRECIP as libc::c_int as isize)
                as *mut thinker_t
        {
            if !((*think).function.acp1
                != ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut precipmobj_t) -> ()>,
                    actionf_p1,
                >(
                    Some(
                        P_NullPrecipThinker
                            as unsafe extern "C" fn(*mut precipmobj_t) -> (),
                    ),
                ))
            {
                precipmobj = think as *mut precipmobj_t;
                P_RemovePrecipMobj(precipmobj);
            }
            think = (*think).next;
        }
    } else {
        let mut think_0: *mut thinker_t = 0 as *mut thinker_t;
        let mut precipmobj_0: *mut precipmobj_t = 0 as *mut precipmobj_t;
        let mut st: *mut state_t = 0 as *mut state_t;
        think_0 = (*thlist.as_mut_ptr().offset(THINK_PRECIP as libc::c_int as isize))
            .next;
        while think_0
            != &mut *thlist.as_mut_ptr().offset(THINK_PRECIP as libc::c_int as isize)
                as *mut thinker_t
        {
            if !((*think_0).function.acp1
                != ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut precipmobj_t) -> ()>,
                    actionf_p1,
                >(
                    Some(
                        P_NullPrecipThinker
                            as unsafe extern "C" fn(*mut precipmobj_t) -> (),
                    ),
                ))
            {
                precipmobj_0 = think_0 as *mut precipmobj_t;
                if weathernum == 3 as libc::c_int || weathernum == 1 as libc::c_int
                    || weathernum == 6 as libc::c_int
                {
                    (*precipmobj_0)
                        .flags = mobjinfo[MT_RAIN as libc::c_int as usize].flags
                        as int32_t;
                    st = &mut *states
                        .as_mut_ptr()
                        .offset(
                            (*mobjinfo
                                .as_mut_ptr()
                                .offset(MT_RAIN as libc::c_int as isize))
                                .spawnstate as isize,
                        ) as *mut state_t;
                    (*precipmobj_0).state = st;
                    (*precipmobj_0).tics = (*st).tics;
                    (*precipmobj_0).sprite = (*st).sprite;
                    (*precipmobj_0).frame = (*st).frame;
                    (*precipmobj_0)
                        .momz = mobjinfo[MT_RAIN as libc::c_int as usize].speed;
                    (*precipmobj_0).precipflags &= !(PCF_INVISIBLE as libc::c_int);
                    (*precipmobj_0).precipflags |= PCF_RAIN as libc::c_int;
                } else if weathernum == 2 as libc::c_int {
                    let mut z: int32_t = 0;
                    (*precipmobj_0)
                        .flags = mobjinfo[MT_SNOWFLAKE as libc::c_int as usize].flags
                        as int32_t;
                    z = M_RandomByte() as int32_t;
                    if z < 64 as libc::c_int {
                        z = 2 as libc::c_int;
                    } else if z < 144 as libc::c_int {
                        z = 1 as libc::c_int;
                    } else {
                        z = 0 as libc::c_int;
                    }
                    st = &mut *states
                        .as_mut_ptr()
                        .offset(
                            ((*mobjinfo
                                .as_mut_ptr()
                                .offset(MT_SNOWFLAKE as libc::c_int as isize))
                                .spawnstate as libc::c_uint)
                                .wrapping_add(z as libc::c_uint) as isize,
                        ) as *mut state_t;
                    (*precipmobj_0).state = st;
                    (*precipmobj_0).tics = (*st).tics;
                    (*precipmobj_0).sprite = (*st).sprite;
                    (*precipmobj_0).frame = (*st).frame;
                    (*precipmobj_0)
                        .momz = mobjinfo[MT_SNOWFLAKE as libc::c_int as usize].speed;
                    (*precipmobj_0).precipflags
                        &= !(PCF_INVISIBLE as libc::c_int | PCF_RAIN as libc::c_int);
                } else {
                    (*precipmobj_0).precipflags |= PCF_INVISIBLE as libc::c_int;
                }
            }
            think_0 = (*think_0).next;
        }
    }
    match weathernum {
        2 => {
            curWeather = 2 as libc::c_int;
            if purge != 0 {
                P_SpawnPrecipitation();
            }
        }
        3 => {
            curWeather = 3 as libc::c_int;
            if purge != 0 {
                P_SpawnPrecipitation();
            }
        }
        1 => {
            curWeather = 1 as libc::c_int;
            if purge != 0 {
                P_SpawnPrecipitation();
            }
        }
        6 => {
            curWeather = 6 as libc::c_int;
            if purge != 0 {
                P_SpawnPrecipitation();
            }
        }
        5 => {
            curWeather = 5 as libc::c_int;
        }
        4 => {
            curWeather = 4 as libc::c_int;
            if purge != 0 {
                P_SpawnPrecipitation();
            }
        }
        _ => {
            curWeather = 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn P_GetObjectTypeInSectorNum(
    mut type_0: mobjtype_t,
    mut s: size_t,
) -> *mut mobj_t {
    let mut sec: *mut sector_t = sectors.offset(s as isize);
    let mut thing: *mut mobj_t = (*sec).thinglist;
    while !thing.is_null() {
        if (*thing).type_0 as libc::c_uint == type_0 as libc::c_uint {
            return thing;
        }
        thing = (*thing).snext;
    }
    return 0 as *mut mobj_t;
}
unsafe extern "C" fn P_FindObjectTypeFromTag(
    mut type_0: mobjtype_t,
    mut tag: mtag_t,
) -> *mut mobj_t {
    if udmf != 0 {
        let mut mtnum: int32_t = 0;
        let mut mo: *mut mobj_t = 0 as *mut mobj_t;
        let mut ICNT_2251: size_t = 0 as libc::c_int as size_t;
        loop {
            mtnum = Tag_Iterate_Things(tag, ICNT_2251);
            if !(mtnum >= 0 as libc::c_int) {
                break;
            }
            mo = (*mapthings.offset(mtnum as isize)).mobj;
            if !mo.is_null() {
                if !((*mo).type_0 as libc::c_uint != type_0 as libc::c_uint) {
                    return mo;
                }
            }
            ICNT_2251 = ICNT_2251.wrapping_add(1);
            ICNT_2251;
        }
        return 0 as *mut mobj_t;
    } else {
        let mut secnum: int32_t = 0;
        secnum = Tag_Iterate_Sectors(tag, 0 as libc::c_int as size_t);
        if secnum < 0 as libc::c_int {
            return 0 as *mut mobj_t;
        }
        return P_GetObjectTypeInSectorNum(type_0, secnum as size_t);
    };
}
unsafe extern "C" fn P_ProcessLineSpecial(
    mut line: *mut line_t,
    mut mo: *mut mobj_t,
    mut callsec: *mut sector_t,
) {
    let mut secnum: int32_t = -(1 as libc::c_int);
    let mut bot: *mut mobj_t = 0 as *mut mobj_t;
    if !mo.is_null() && !((*mo).player).is_null() && botingame != 0 {
        bot = players[secondarydisplayplayer as usize].mo;
    }
    let mut current_block_696: u64;
    match (*line).special as libc::c_int {
        400 => {
            if (*line).args[1 as libc::c_int as usize] != TMP_CEILING as libc::c_int {
                EV_DoFloor(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    instantMoveFloorByFrontSector,
                );
            }
            if (*line).args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int {
                EV_DoCeiling(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    instantMoveCeilingByFrontSector,
                );
            }
        }
        402 => {
            let mut newlightlevel: int16_t = 0;
            let mut newfloorlightlevel: int16_t = 0;
            let mut newceilinglightlevel: int16_t = 0;
            let mut newfloorlightabsolute: boolean = 0;
            let mut newceilinglightabsolute: boolean = 0;
            let mut newfloorlightsec: int32_t = 0;
            let mut newceilinglightsec: int32_t = 0;
            newlightlevel = (*(*line).frontsector).lightlevel;
            newfloorlightlevel = (*(*line).frontsector).floorlightlevel;
            newfloorlightabsolute = (*(*line).frontsector).floorlightabsolute;
            newceilinglightlevel = (*(*line).frontsector).ceilinglightlevel;
            newceilinglightabsolute = (*(*line).frontsector).ceilinglightabsolute;
            newfloorlightsec = (*(*line).frontsector).floorlightsec;
            newceilinglightsec = (*(*line).frontsector).ceilinglightsec;
            let mut ICNT_2326: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2326,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if !((*sectors.offset(secnum as isize)).lightingdata).is_null() {
                    P_RemoveThinker(
                        &mut (*((*sectors.offset(secnum as isize)).lightingdata
                            as *mut thinkerdata_t))
                            .thinker,
                    );
                    let ref mut fresh0 = (*sectors.offset(secnum as isize)).lightingdata;
                    *fresh0 = 0 as *mut libc::c_void;
                }
                if (*line).args[1 as libc::c_int as usize] & TMLC_NOSECTOR as libc::c_int
                    == 0
                {
                    (*sectors.offset(secnum as isize)).lightlevel = newlightlevel;
                }
                if (*line).args[1 as libc::c_int as usize] & TMLC_NOFLOOR as libc::c_int
                    == 0
                {
                    (*sectors.offset(secnum as isize))
                        .floorlightlevel = newfloorlightlevel;
                    (*sectors.offset(secnum as isize))
                        .floorlightabsolute = newfloorlightabsolute;
                    (*sectors.offset(secnum as isize)).floorlightsec = newfloorlightsec;
                }
                if (*line).args[1 as libc::c_int as usize]
                    & TMLC_NOCEILING as libc::c_int == 0
                {
                    (*sectors.offset(secnum as isize))
                        .ceilinglightlevel = newceilinglightlevel;
                    (*sectors.offset(secnum as isize))
                        .ceilinglightabsolute = newceilinglightabsolute;
                    (*sectors.offset(secnum as isize))
                        .ceilinglightsec = newceilinglightsec;
                }
                ICNT_2326 = ICNT_2326.wrapping_add(1);
                ICNT_2326;
            }
        }
        403 => {
            if (*line).args[1 as libc::c_int as usize] != TMP_CEILING as libc::c_int {
                EV_DoFloor(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    moveFloorByFrontSector,
                );
            }
            if (*line).args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int {
                EV_DoCeiling(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    moveCeilingByFrontSector,
                );
            }
        }
        405 => {
            if (*line).args[1 as libc::c_int as usize] != TMP_CEILING as libc::c_int {
                EV_DoFloor(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    moveFloorByDistance,
                );
            }
            if (*line).args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int {
                EV_DoCeiling(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    moveCeilingByDistance,
                );
            }
        }
        408 => {
            let mut ICNT_2369: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2369,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if (*line).args[1 as libc::c_int as usize] != TMP_CEILING as libc::c_int
                {
                    (*sectors.offset(secnum as isize))
                        .floorpic = (*(*line).frontsector).floorpic;
                }
                if (*line).args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int {
                    (*sectors.offset(secnum as isize))
                        .ceilingpic = (*(*line).frontsector).ceilingpic;
                }
                ICNT_2369 = ICNT_2369.wrapping_add(1);
                ICNT_2369;
            }
        }
        409 => {
            let mut newtag: mtag_t = (*line).args[1 as libc::c_int as usize] as mtag_t;
            let mut ICNT_2384: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2384,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                match (*line).args[2 as libc::c_int as usize] {
                    0 => {
                        Tag_SectorAdd(secnum as size_t, newtag);
                    }
                    1 => {
                        Tag_SectorRemove(secnum as size_t, newtag);
                    }
                    3 => {
                        (*sectors.offset(secnum as isize)).triggertag = newtag;
                    }
                    2 | _ => {
                        Tag_SectorFSet(secnum as size_t, newtag);
                    }
                }
                ICNT_2384 = ICNT_2384.wrapping_add(1);
                ICNT_2384;
            }
        }
        410 => {
            let mut newtag_0: mtag_t = (*line).args[1 as libc::c_int as usize] as mtag_t;
            secnum = ((*line).frontsector).offset_from(sectors) as libc::c_long
                as uint32_t as int32_t;
            match (*line).args[2 as libc::c_int as usize] {
                0 => {
                    Tag_SectorAdd(secnum as size_t, newtag_0);
                }
                1 => {
                    Tag_SectorRemove(secnum as size_t, newtag_0);
                }
                3 => {
                    (*sectors.offset(secnum as isize)).triggertag = newtag_0;
                }
                2 | _ => {
                    Tag_SectorFSet(secnum as size_t, newtag_0);
                }
            }
        }
        411 => {
            let mut ICNT_2431: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2431,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if !((*sectors.offset(secnum as isize)).floordata).is_null() {
                    if (*sectors.offset(secnum as isize)).floordata
                        == (*sectors.offset(secnum as isize)).ceilingdata
                    {
                        P_RemoveThinker(
                            &mut (*((*sectors.offset(secnum as isize)).floordata
                                as *mut elevator_t))
                                .thinker,
                        );
                        let ref mut fresh1 = (*sectors.offset(secnum as isize))
                            .ceilingdata;
                        *fresh1 = 0 as *mut libc::c_void;
                        let ref mut fresh2 = (*sectors.offset(secnum as isize))
                            .floordata;
                        *fresh2 = *fresh1;
                        let ref mut fresh3 = (*sectors.offset(secnum as isize))
                            .ceilspeed;
                        *fresh3 = 0 as libc::c_int;
                        (*sectors.offset(secnum as isize)).floorspeed = *fresh3;
                    } else {
                        P_RemoveThinker(
                            &mut (*((*sectors.offset(secnum as isize)).floordata
                                as *mut floormove_t))
                                .thinker,
                        );
                        let ref mut fresh4 = (*sectors.offset(secnum as isize))
                            .floordata;
                        *fresh4 = 0 as *mut libc::c_void;
                        (*sectors.offset(secnum as isize)).floorspeed = 0 as libc::c_int;
                    }
                }
                if !((*sectors.offset(secnum as isize)).ceilingdata).is_null() {
                    P_RemoveThinker(
                        &mut (*((*sectors.offset(secnum as isize)).ceilingdata
                            as *mut ceiling_t))
                            .thinker,
                    );
                    let ref mut fresh5 = (*sectors.offset(secnum as isize)).ceilingdata;
                    *fresh5 = 0 as *mut libc::c_void;
                    (*sectors.offset(secnum as isize)).ceilspeed = 0 as libc::c_int;
                }
                ICNT_2431 = ICNT_2431.wrapping_add(1);
                ICNT_2431;
            }
        }
        412 => {
            let mut dest: *mut mobj_t = 0 as *mut mobj_t;
            if mo.is_null() {
                return;
            }
            if (*line).args[1 as libc::c_int as usize] & TMT_RELATIVE as libc::c_int != 0
            {
                let mut x: fixed_t = 0;
                let mut y: fixed_t = 0;
                let mut z: fixed_t = 0;
                x = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
                y = (*line).args[3 as libc::c_int as usize] << 16 as libc::c_int;
                z = (*line).args[4 as libc::c_int as usize] << 16 as libc::c_int;
                P_UnsetThingPosition(mo);
                (*mo).x += x;
                (*mo).y += y;
                (*mo).z += z;
                P_SetThingPosition(mo);
                if !((*mo).player).is_null() {
                    if !bot.is_null() {
                        P_SetOrigin(bot, (*bot).x + x, (*bot).y + y, (*bot).z + z);
                    }
                    if splitscreen != 0
                        && (*mo).player
                            == &mut *players
                                .as_mut_ptr()
                                .offset(secondarydisplayplayer as isize) as *mut player_t
                        && camera2.chase != 0
                    {
                        camera2.x += x;
                        camera2.y += y;
                        camera2.z += z;
                        camera2.subsector = R_PointInSubsector(camera2.x, camera2.y);
                    } else if camera.chase != 0
                        && (*mo).player
                            == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                                as *mut player_t
                    {
                        camera.x += x;
                        camera.y += y;
                        camera.z += z;
                        camera.subsector = R_PointInSubsector(camera.x, camera.y);
                    }
                }
            } else {
                let mut angle: angle_t = 0;
                let mut silent: boolean = 0;
                let mut keepmomentum: boolean = 0;
                dest = P_FindObjectTypeFromTag(
                    MT_TELEPORTMAN,
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                );
                if dest.is_null() {
                    return;
                }
                angle = if (*line).args[1 as libc::c_int as usize]
                    & TMT_KEEPANGLE as libc::c_int != 0
                {
                    (*mo).angle
                } else {
                    (*dest).angle
                };
                silent = ((*line).args[1 as libc::c_int as usize]
                    & TMT_SILENT as libc::c_int != 0) as libc::c_int;
                keepmomentum = ((*line).args[1 as libc::c_int as usize]
                    & TMT_KEEPMOMENTUM as libc::c_int != 0) as libc::c_int;
                if !bot.is_null() {
                    P_Teleport(
                        bot,
                        (*dest).x,
                        (*dest).y,
                        (*dest).z,
                        angle,
                        (silent == 0) as libc::c_int,
                        keepmomentum,
                    );
                }
                P_Teleport(
                    mo,
                    (*dest).x,
                    (*dest).y,
                    (*dest).z,
                    angle,
                    (silent == 0) as libc::c_int,
                    keepmomentum,
                );
                if silent == 0 {
                    S_StartSound(dest as *const libc::c_void, sfx_mixup);
                }
            }
        }
        413 => {
            if (*line).args[0 as libc::c_int as usize] & TMM_ALLPLAYERS as libc::c_int
                != 0
                || !mo.is_null() && !((*mo).player).is_null()
                    && P_IsLocalPlayer((*mo).player) != 0
                || titlemapinaction as libc::c_int != 0
            {
                let mut musicsame: boolean = (((*line)
                    .stringargs[0 as libc::c_int as usize])
                    .is_null()
                    || *((*line).stringargs[0 as libc::c_int as usize])
                        .offset(0 as libc::c_int as isize) == 0
                    || strncasecmp(
                        (*line).stringargs[0 as libc::c_int as usize],
                        S_MusicName(),
                        7 as libc::c_int as libc::c_ulong,
                    ) == 0) as libc::c_int;
                let mut tracknum: uint16_t = (if (*line).args[6 as libc::c_int as usize]
                    > 0 as libc::c_int
                {
                    (*line).args[6 as libc::c_int as usize]
                } else {
                    0 as libc::c_int
                }) as uint16_t;
                let mut position: int32_t = if (*line).args[1 as libc::c_int as usize]
                    > 0 as libc::c_int
                {
                    (*line).args[1 as libc::c_int as usize]
                } else {
                    0 as libc::c_int
                };
                let mut prefadems: uint32_t = (if (*line).args[2 as libc::c_int as usize]
                    > 0 as libc::c_int
                {
                    (*line).args[2 as libc::c_int as usize]
                } else {
                    0 as libc::c_int
                }) as uint32_t;
                let mut postfadems: uint32_t = (if (*line)
                    .args[3 as libc::c_int as usize] > 0 as libc::c_int
                {
                    (*line).args[3 as libc::c_int as usize]
                } else {
                    0 as libc::c_int
                }) as uint32_t;
                let mut fadetarget: uint8_t = (if (*line).args[4 as libc::c_int as usize]
                    > 0 as libc::c_int
                {
                    (*line).args[4 as libc::c_int as usize]
                } else {
                    0 as libc::c_int
                }) as uint8_t;
                let mut fadesource: int16_t = (if (*line).args[5 as libc::c_int as usize]
                    > -(1 as libc::c_int)
                {
                    (*line).args[5 as libc::c_int as usize]
                } else {
                    -(1 as libc::c_int)
                }) as int16_t;
                if (*line).args[0 as libc::c_int as usize] & TMM_OFFSET as libc::c_int
                    != 0
                {
                    if position < 0 as libc::c_int && S_GetMusicLength() != 0
                        && S_GetMusicPosition() > S_GetMusicLoopPoint()
                        && (S_GetMusicPosition()).wrapping_add(position as uint32_t)
                            < S_GetMusicLoopPoint()
                    {
                        position = (if (S_GetMusicLength())
                            .wrapping_sub(
                                (S_GetMusicLoopPoint())
                                    .wrapping_sub(
                                        (S_GetMusicPosition()).wrapping_add(position as uint32_t),
                                    ),
                            ) > 0 as libc::c_int as uint32_t
                        {
                            (S_GetMusicLength())
                                .wrapping_sub(
                                    (S_GetMusicLoopPoint())
                                        .wrapping_sub(
                                            (S_GetMusicPosition()).wrapping_add(position as uint32_t),
                                        ),
                                )
                        } else {
                            0 as libc::c_int as uint32_t
                        }) as int32_t;
                    } else {
                        position = (if (S_GetMusicPosition())
                            .wrapping_add(position as uint32_t)
                            > 0 as libc::c_int as uint32_t
                        {
                            (S_GetMusicPosition()).wrapping_add(position as uint32_t)
                        } else {
                            0 as libc::c_int as uint32_t
                        }) as int32_t;
                    }
                }
                if (*line).args[0 as libc::c_int as usize] & TMM_FADE as libc::c_int != 0
                    && fadetarget as libc::c_int != 0 && musicsame != 0
                {
                    if fadesource == 0 {
                        fadesource = -(1 as libc::c_int) as int16_t;
                    }
                    if postfadems == 0 {
                        S_SetInternalMusicVolume(fadetarget as int32_t);
                    } else {
                        S_FadeMusicFromVolume(fadetarget, fadesource, postfadems);
                    }
                    if position != 0 {
                        S_SetMusicPosition(position as uint32_t);
                    }
                } else {
                    if ((*line).stringargs[0 as libc::c_int as usize]).is_null()
                        || strcmp(
                            (*line).stringargs[0 as libc::c_int as usize],
                            b"-\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        strcpy(
                            mapmusname.as_mut_ptr(),
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        strncpy(
                            mapmusname.as_mut_ptr(),
                            (*line).stringargs[0 as libc::c_int as usize],
                            7 as libc::c_int as libc::c_ulong,
                        );
                        mapmusname[6 as libc::c_int
                            as usize] = 0 as libc::c_int as libc::c_char;
                    }
                    mapmusflags = (tracknum as libc::c_int & 0xfff as libc::c_int)
                        as uint16_t;
                    if (*line).args[0 as libc::c_int as usize]
                        & TMM_NORELOAD as libc::c_int == 0
                    {
                        mapmusflags = (mapmusflags as libc::c_int
                            | 0x8000 as libc::c_int) as uint16_t;
                    }
                    if (*line).args[0 as libc::c_int as usize]
                        & TMM_FORCERESET as libc::c_int != 0
                    {
                        mapmusflags = (mapmusflags as libc::c_int
                            | 0x4000 as libc::c_int) as uint16_t;
                    }
                    mapmusposition = position as uint32_t;
                    S_ChangeMusicEx(
                        mapmusname.as_mut_ptr(),
                        mapmusflags,
                        ((*line).args[0 as libc::c_int as usize]
                            & TMM_NOLOOP as libc::c_int == 0) as libc::c_int,
                        position as uint32_t,
                        if (*line).args[0 as libc::c_int as usize]
                            & TMM_FADE as libc::c_int == 0
                        {
                            prefadems
                        } else {
                            0 as libc::c_int as uint32_t
                        },
                        if (*line).args[0 as libc::c_int as usize]
                            & TMM_FADE as libc::c_int == 0
                        {
                            postfadems
                        } else {
                            0 as libc::c_int as uint32_t
                        },
                    );
                    if (*line).args[0 as libc::c_int as usize] & TMM_FADE as libc::c_int
                        != 0 && fadetarget as libc::c_int != 0
                    {
                        if postfadems == 0 {
                            S_SetInternalMusicVolume(fadetarget as int32_t);
                        } else {
                            S_FadeMusicFromVolume(fadetarget, fadesource, postfadems);
                        }
                    }
                }
            }
        }
        414 => {
            P_PlaySFX(
                if !((*line).stringargs[0 as libc::c_int as usize]).is_null() {
                    get_number((*line).stringargs[0 as libc::c_int as usize])
                } else {
                    sfx_None as libc::c_int
                },
                mo,
                callsec,
                (*line).args[2 as libc::c_int as usize] as int16_t,
                (*line).args[0 as libc::c_int as usize] as textmapsoundsource_t,
                (*line).args[1 as libc::c_int as usize] as textmapsoundlistener_t,
            );
        }
        415 => {
            if cv_runscripts.value != 0 {
                let mut lumpnum: lumpnum_t = W_CheckNumForName(
                    (*line).stringargs[0 as libc::c_int as usize],
                );
                if lumpnum == 4294967295 as libc::c_uint
                    || W_LumpLength(lumpnum) == 0 as libc::c_int as size_t
                {
                    CONS_Debug(
                        0x400 as libc::c_int,
                        b"Line type 415 Executor: script lump %s not found/not valid.\n\0"
                            as *const u8 as *const libc::c_char,
                        (*line).stringargs[0 as libc::c_int as usize],
                    );
                } else {
                    COM_BufInsertTextEx(
                        W_CacheLumpNum(lumpnum, PU_CACHE as libc::c_int)
                            as *const libc::c_char,
                        0 as com_flags_t,
                    );
                }
            }
        }
        416 => {
            let mut ICNT_2615: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2615,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                P_SpawnAdjustableFireFlicker(
                    &mut *sectors.offset(secnum as isize),
                    (*line).args[2 as libc::c_int as usize] as int16_t,
                    (if (*line).args[3 as libc::c_int as usize] != 0 {
                        (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                    } else {
                        (*line).args[4 as libc::c_int as usize]
                    }) as int16_t,
                    (*line).args[1 as libc::c_int as usize],
                );
                ICNT_2615 = ICNT_2615.wrapping_add(1);
                ICNT_2615;
            }
        }
        417 => {
            let mut ICNT_2621: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2621,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                P_SpawnAdjustableGlowingLight(
                    &mut *sectors.offset(secnum as isize),
                    (*line).args[2 as libc::c_int as usize] as int16_t,
                    (if (*line).args[3 as libc::c_int as usize] != 0 {
                        (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                    } else {
                        (*line).args[4 as libc::c_int as usize]
                    }) as int16_t,
                    (*line).args[1 as libc::c_int as usize],
                );
                ICNT_2621 = ICNT_2621.wrapping_add(1);
                ICNT_2621;
            }
        }
        418 => {
            let mut ICNT_2627: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2627,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                P_SpawnAdjustableStrobeFlash(
                    &mut *sectors.offset(secnum as isize),
                    (*line).args[3 as libc::c_int as usize] as int16_t,
                    (if (*line).args[4 as libc::c_int as usize]
                        & TMB_USETARGET as libc::c_int != 0
                    {
                        (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                    } else {
                        (*line).args[5 as libc::c_int as usize]
                    }) as int16_t,
                    (*line).args[1 as libc::c_int as usize],
                    (*line).args[2 as libc::c_int as usize],
                    (*line).args[4 as libc::c_int as usize] & TMB_SYNC as libc::c_int,
                );
                ICNT_2627 = ICNT_2627.wrapping_add(1);
                ICNT_2627;
            }
        }
        420 => {
            P_FadeLight(
                (*line).args[0 as libc::c_int as usize] as int16_t,
                (*line).args[1 as libc::c_int as usize],
                (*line).args[2 as libc::c_int as usize],
                (*line).args[3 as libc::c_int as usize] & TMF_TICBASED as libc::c_int,
                (*line).args[3 as libc::c_int as usize] & TMF_OVERRIDE as libc::c_int,
                (*line).args[3 as libc::c_int as usize] & TMF_RELATIVE as libc::c_int,
            );
        }
        421 => {
            let mut ICNT_2638: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_2638,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if !((*sectors.offset(secnum as isize)).lightingdata).is_null() {
                    P_RemoveThinker(
                        &mut (*((*sectors.offset(secnum as isize)).lightingdata
                            as *mut thinkerdata_t))
                            .thinker,
                    );
                    let ref mut fresh6 = (*sectors.offset(secnum as isize)).lightingdata;
                    *fresh6 = 0 as *mut libc::c_void;
                }
                ICNT_2638 = ICNT_2638.wrapping_add(1);
                ICNT_2638;
            }
        }
        422 => {
            let mut altview: *mut mobj_t = 0 as *mut mobj_t;
            let mut aim: int32_t = 0;
            if (mo.is_null() || ((*mo).player).is_null()) && titlemapinaction == 0 {
                return;
            }
            altview = P_FindObjectTypeFromTag(
                MT_ALTVIEWMAN,
                (*line).args[0 as libc::c_int as usize] as mtag_t,
            );
            if altview.is_null() || ((*altview).spawnpoint).is_null() {
                return;
            }
            if titlemapinaction != 0 {
                titlemapcameraref = altview;
            } else if (*(*mo).player).awayviewtics == 0
                || (*(*mo).player).awayviewmobj != altview
            {
                P_SetTarget2(&mut (*(*mo).player).awayviewmobj, altview);
                if (*mo).player
                    == &mut *players.as_mut_ptr().offset(displayplayer as isize)
                        as *mut player_t
                {
                    P_ResetCamera((*mo).player, &mut camera);
                } else if splitscreen != 0
                    && (*mo).player
                        == &mut *players
                            .as_mut_ptr()
                            .offset(secondarydisplayplayer as isize) as *mut player_t
                {
                    P_ResetCamera((*mo).player, &mut camera2);
                }
            }
            aim = if udmf != 0 {
                (*(*altview).spawnpoint).pitch as libc::c_int
            } else {
                (*line).args[2 as libc::c_int as usize]
            };
            aim = (aim + 360 as libc::c_int) % 360 as libc::c_int;
            aim *= 0x40000000 as libc::c_int >> 8 as libc::c_int;
            aim /= 90 as libc::c_int;
            aim <<= 8 as libc::c_int;
            if titlemapinaction != 0 {
                (*titlemapcameraref).cusval = aim as angle_t as int32_t;
            } else {
                (*(*mo).player).awayviewaiming = aim as angle_t;
                (*(*mo).player).awayviewtics = (*line).args[1 as libc::c_int as usize];
            }
        }
        423 => {
            if !mo.is_null() && !((*mo).player).is_null()
                && P_IsLocalPlayer((*mo).player) != 0
                || (*line).args[1 as libc::c_int as usize] != 0
            {
                P_SetupLevelSky(
                    (*line).args[0 as libc::c_int as usize],
                    (*line).args[1 as libc::c_int as usize],
                );
            }
        }
        424 => {
            if (*line).args[1 as libc::c_int as usize] != 0 {
                globalweather = (*line).args[0 as libc::c_int as usize] as uint8_t;
                P_SwitchWeather(globalweather as int32_t);
            } else if !mo.is_null() && !((*mo).player).is_null()
                && P_IsLocalPlayer((*mo).player) != 0
            {
                P_SwitchWeather((*line).args[0 as libc::c_int as usize]);
            }
        }
        425 => {
            if !mo.is_null() && ((*mo).player).is_null() {
                let mut state: statenum_t = (if !((*line)
                    .stringargs[0 as libc::c_int as usize])
                    .is_null()
                {
                    get_number((*line).stringargs[0 as libc::c_int as usize])
                } else {
                    S_NULL as libc::c_int
                }) as statenum_t;
                if state as libc::c_uint >= 0 as libc::c_int as libc::c_uint
                    && (state as libc::c_uint) < NUMSTATES as libc::c_int as libc::c_uint
                {
                    P_SetMobjState(mo, state);
                }
            }
        }
        426 => {
            if mo.is_null() {
                return;
            }
            if (*line).args[0 as libc::c_int as usize] != 0 {
                P_UnsetThingPosition(mo);
                (*mo).x = (*(*(*mo).subsector).sector).soundorg.x;
                (*mo).y = (*(*(*mo).subsector).sector).soundorg.y;
                (*mo).z = (*mo).floorz;
                P_SetThingPosition(mo);
            }
            (*mo).momz = 1 as libc::c_int;
            (*mo).momy = (*mo).momz;
            (*mo).momx = (*mo).momy;
            (*mo).pmomz = 0 as libc::c_int;
            if !((*mo).player).is_null() {
                (*(*mo).player).rmomy = 1 as libc::c_int;
                (*(*mo).player).rmomx = (*(*mo).player).rmomy;
                (*(*mo).player).cmomy = 0 as libc::c_int;
                (*(*mo).player).cmomx = (*(*mo).player).cmomy;
                P_ResetPlayer((*mo).player);
                P_SetPlayerMobjState(mo, S_PLAY_STND);
                if !bot.is_null() {
                    if (*line).args[0 as libc::c_int as usize] != 0 {
                        P_SetOrigin(bot, (*mo).x, (*mo).y, (*mo).z);
                    }
                    (*bot).momz = 1 as libc::c_int;
                    (*bot).momy = (*bot).momz;
                    (*bot).momx = (*bot).momy;
                    (*bot).pmomz = 0 as libc::c_int;
                    (*(*bot).player).rmomy = 1 as libc::c_int;
                    (*(*bot).player).rmomx = (*(*bot).player).rmomy;
                    (*(*bot).player).cmomy = 0 as libc::c_int;
                    (*(*bot).player).cmomx = (*(*bot).player).cmomy;
                    P_ResetPlayer((*bot).player);
                    P_SetPlayerMobjState(bot, S_PLAY_STND);
                }
            }
        }
        427 => {
            if !mo.is_null() && !((*mo).player).is_null() {
                P_AddPlayerScore(
                    (*mo).player,
                    (*line).args[0 as libc::c_int as usize] as uint32_t,
                );
            }
        }
        428 => {
            EV_DoElevator(
                (*line).args[0 as libc::c_int as usize] as mtag_t,
                line,
                elevateContinuous,
            );
        }
        429 => {
            if (*line).args[1 as libc::c_int as usize] == TMP_FLOOR as libc::c_int {
                EV_DoFloor(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    crushFloorOnce,
                );
            } else if (*line).args[1 as libc::c_int as usize]
                == TMP_CEILING as libc::c_int
            {
                EV_DoCrush(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    crushCeilOnce,
                );
            } else {
                EV_DoCrush(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    line,
                    crushBothOnce,
                );
            }
        }
        432 => {
            if !mo.is_null() && !((*mo).player).is_null() {
                if (*line).args[0 as libc::c_int as usize] != 0 {
                    (*mo).flags2 &= !(MF2_TWOD as libc::c_int) as uint32_t;
                } else {
                    (*mo).flags2 |= MF2_TWOD as libc::c_int as uint32_t;
                }
                if !bot.is_null()
                    && (*bot).flags2 & MF2_TWOD as libc::c_int as uint32_t
                        != (*mo).flags2 & MF2_TWOD as libc::c_int as uint32_t
                {
                    (*bot)
                        .flags2 = (*bot).flags2 & !(MF2_TWOD as libc::c_int) as uint32_t
                        | (*mo).flags2 & MF2_TWOD as libc::c_int as uint32_t;
                    P_SetOrigin(bot, (*mo).x, (*mo).y, (*mo).z);
                }
            }
        }
        433 => {
            if (*line).args[1 as libc::c_int as usize] != 0 {
                (*mo).flags2 ^= MF2_OBJECTFLIP as libc::c_int as uint32_t;
            } else if (*line).args[0 as libc::c_int as usize] != 0 {
                (*mo).flags2 &= !(MF2_OBJECTFLIP as libc::c_int) as uint32_t;
            } else {
                (*mo).flags2 |= MF2_OBJECTFLIP as libc::c_int as uint32_t;
            }
            if (*line).args[2 as libc::c_int as usize] != 0 {
                (*mo)
                    .eflags = ((*mo).eflags as libc::c_int
                    | MFE_VERTICALFLIP as libc::c_int) as uint16_t;
            }
            if !bot.is_null() {
                (*bot)
                    .flags2 = (*bot).flags2
                    & !(MF2_OBJECTFLIP as libc::c_int) as uint32_t
                    | (*mo).flags2 & MF2_OBJECTFLIP as libc::c_int as uint32_t;
            }
        }
        434 => {
            if !mo.is_null() && !((*mo).player).is_null() {
                let mut power: powertype_t = (if !((*line)
                    .stringargs[0 as libc::c_int as usize])
                    .is_null()
                {
                    get_number((*line).stringargs[0 as libc::c_int as usize])
                } else {
                    0 as libc::c_int
                }) as powertype_t;
                let mut value: int32_t = if !((*line)
                    .stringargs[1 as libc::c_int as usize])
                    .is_null()
                {
                    get_number((*line).stringargs[1 as libc::c_int as usize])
                } else {
                    0 as libc::c_int
                };
                if value == -(1 as libc::c_int) {
                    value = 65535 as libc::c_int;
                }
                P_SetPower((*mo).player, power, value as uint16_t);
                if !bot.is_null() {
                    P_SetPower((*bot).player, power, value as uint16_t);
                }
            }
        }
        435 => {
            let mut scroller: *mut scroll_t = 0 as *mut scroll_t;
            let mut th: *mut thinker_t = 0 as *mut thinker_t;
            let mut length: fixed_t = R_PointToDist2(
                (*(*line).v2).x,
                (*(*line).v2).y,
                (*(*line).v1).x,
                (*(*line).v1).y,
            );
            let mut speed: fixed_t = (*line).args[1 as libc::c_int as usize]
                << 16 as libc::c_int;
            let mut dx: fixed_t = FixedMul(
                FixedMul(FixedDiv((*line).dx, length), speed) >> 5 as libc::c_int,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                    / 32 as libc::c_int,
            );
            let mut dy: fixed_t = FixedMul(
                FixedMul(FixedDiv((*line).dy, length), speed) >> 5 as libc::c_int,
                3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                    / 32 as libc::c_int,
            );
            th = (*thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)).next;
            while th
                != &mut *thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)
                    as *mut thinker_t
            {
                if !((*th).function.acp1
                    != ::core::mem::transmute::<
                        Option::<unsafe extern "C" fn(*mut scroll_t) -> ()>,
                        actionf_p1,
                    >(Some(T_Scroll as unsafe extern "C" fn(*mut scroll_t) -> ())))
                {
                    scroller = th as *mut scroll_t;
                    if !(Tag_Find(
                        &mut (*sectors.offset((*scroller).affectee as isize)).tags,
                        (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ) == 0)
                    {
                        (*scroller).dx = dx;
                        (*scroller).dy = dy;
                    }
                }
                th = (*th).next;
            }
        }
        436 => {
            let mut sectag: int16_t = (*line).args[0 as libc::c_int as usize] as int16_t;
            let mut foftag: int16_t = (*line).args[1 as libc::c_int as usize] as int16_t;
            let mut sec: *mut sector_t = 0 as *mut sector_t;
            let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover: boolean = false_0 as libc::c_int;
            let mut ICNT_2842: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag, ICNT_2842);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec = sectors.offset(secnum as isize);
                if ((*sec).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 436 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                rover = (*sec).ffloors;
                while !rover.is_null() {
                    if Tag_Find(&mut (*(*(*rover).master).frontsector).tags, foftag) != 0
                    {
                        foundrover = true_0 as libc::c_int;
                        EV_CrumbleChain(sec, rover);
                    }
                    rover = (*rover).next;
                }
                if foundrover == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 436 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag as libc::c_int,
                    );
                    return;
                }
                ICNT_2842 = ICNT_2842.wrapping_add(1);
                ICNT_2842;
            }
        }
        437 => {
            if !mo.is_null() && !((*mo).player).is_null() {
                let mut fractime: uint16_t = (*line).args[0 as libc::c_int as usize]
                    as uint16_t;
                if (fractime as libc::c_int) < 1 as libc::c_int {
                    fractime = 1 as libc::c_int as uint16_t;
                }
                if (*line).args[1 as libc::c_int as usize] != 0 {
                    fractime = (fractime as libc::c_int
                        | (1 as libc::c_int) << 15 as libc::c_int) as uint16_t;
                }
                (*(*mo).player).powers[pw_nocontrol as libc::c_int as usize] = fractime;
                if !bot.is_null() {
                    (*(*bot).player)
                        .powers[pw_nocontrol as libc::c_int as usize] = fractime;
                }
            }
        }
        438 => {
            if !mo.is_null() {
                (*mo)
                    .destscale = FixedDiv(
                    (*line).args[0 as libc::c_int as usize] << 16 as libc::c_int,
                    (100 as libc::c_int) << 16 as libc::c_int,
                );
                if (*mo).destscale
                    < ((1 as libc::c_int) << 16 as libc::c_int) / 100 as libc::c_int
                {
                    (*mo)
                        .destscale = ((1 as libc::c_int) << 16 as libc::c_int)
                        / 100 as libc::c_int;
                }
                if !((*mo).player).is_null() && !bot.is_null() {
                    (*bot).destscale = (*mo).destscale;
                }
            }
        }
        439 => {
            let mut linenum: size_t = 0;
            let mut setfront: *mut side_t = &mut *sides
                .offset(
                    *((*line).sidenum).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as isize,
                ) as *mut side_t;
            let mut setback: *mut side_t = if (*line).args[3 as libc::c_int as usize]
                != 0
                && (*line).sidenum[1 as libc::c_int as usize] as libc::c_int
                    != 0xffff as libc::c_int
            {
                &mut *sides
                    .offset(
                        *((*line).sidenum).as_mut_ptr().offset(1 as libc::c_int as isize)
                            as isize,
                    ) as *mut side_t
            } else {
                setfront
            };
            let mut this: *mut side_t = 0 as *mut side_t;
            let mut always: boolean = ((*line).args[2 as libc::c_int as usize] == 0)
                as libc::c_int;
            linenum = 0 as libc::c_int as size_t;
            while linenum < numlines {
                if !((*lines.offset(linenum as isize)).special as libc::c_int
                    == 439 as libc::c_int)
                {
                    if !(Tag_Find(
                        &mut (*lines.offset(linenum as isize)).tags,
                        (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ) == 0)
                    {
                        if (*line).args[1 as libc::c_int as usize]
                            != TMSD_BACK as libc::c_int
                        {
                            this = &mut *sides
                                .offset(
                                    *((*lines.offset(linenum as isize)).sidenum)
                                        .as_mut_ptr()
                                        .offset(0 as libc::c_int as isize) as isize,
                                ) as *mut side_t;
                            if always != 0 || (*this).toptexture != 0 {
                                (*this).toptexture = (*setfront).toptexture;
                            }
                            if always != 0 || (*this).midtexture != 0 {
                                (*this).midtexture = (*setfront).midtexture;
                            }
                            if always != 0 || (*this).bottomtexture != 0 {
                                (*this).bottomtexture = (*setfront).bottomtexture;
                            }
                        }
                        if (*line).args[1 as libc::c_int as usize]
                            != TMSD_FRONT as libc::c_int
                            && (*lines.offset(linenum as isize))
                                .sidenum[1 as libc::c_int as usize] as libc::c_int
                                != 0xffff as libc::c_int
                        {
                            this = &mut *sides
                                .offset(
                                    *((*lines.offset(linenum as isize)).sidenum)
                                        .as_mut_ptr()
                                        .offset(1 as libc::c_int as isize) as isize,
                                ) as *mut side_t;
                            if always != 0 || (*this).toptexture != 0 {
                                (*this).toptexture = (*setback).toptexture;
                            }
                            if always != 0 || (*this).midtexture != 0 {
                                (*this).midtexture = (*setback).midtexture;
                            }
                            if always != 0 || (*this).bottomtexture != 0 {
                                (*this).bottomtexture = (*setback).bottomtexture;
                            }
                        }
                    }
                }
                linenum = linenum.wrapping_add(1);
                linenum;
            }
        }
        440 => {
            if metalrecording == 0 && metalplayback.is_null() {
                G_DoPlayMetal();
            }
        }
        441 => {
            let mut trigid: int32_t = (*line).args[0 as libc::c_int as usize];
            if trigid < 0 as libc::c_int || trigid > 31 as libc::c_int {
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Unlockable trigger (sidedef %hu): bad trigger ID %d\n\0"
                        as *const u8 as *const libc::c_char,
                    (*line).sidenum[0 as libc::c_int as usize] as libc::c_int,
                    trigid,
                );
            } else {
                unlocktriggers |= ((1 as libc::c_int) << trigid) as uint32_t;
                M_SilentUpdateUnlockablesAndEmblems(serverGamedata);
                if M_UpdateUnlockablesAndExtraEmblems(clientGamedata) != 0 {
                    S_StartSound(0 as *const libc::c_void, sfx_s3k68);
                    G_SaveGameData(clientGamedata);
                }
            }
            (*line).special = 0 as libc::c_int as int16_t;
        }
        442 => {
            let type_0: mobjtype_t = (if !((*line).stringargs[0 as libc::c_int as usize])
                .is_null()
            {
                get_number((*line).stringargs[0 as libc::c_int as usize])
            } else {
                MT_NULL as libc::c_int
            }) as mobjtype_t;
            let mut state_0: statenum_t = NUMSTATES;
            let mut thing: *mut mobj_t = 0 as *mut mobj_t;
            if !((type_0 as libc::c_uint) < 0 as libc::c_int as libc::c_uint
                || type_0 as libc::c_uint >= NUMMOBJTYPES as libc::c_int as libc::c_uint)
            {
                if (*line).args[1 as libc::c_int as usize] == 0 {
                    state_0 = (if !((*line).stringargs[1 as libc::c_int as usize])
                        .is_null()
                    {
                        get_number((*line).stringargs[1 as libc::c_int as usize])
                    } else {
                        S_NULL as libc::c_int
                    }) as statenum_t;
                    if (state_0 as libc::c_uint) < 0 as libc::c_int as libc::c_uint
                        || state_0 as libc::c_uint
                            >= NUMSTATES as libc::c_int as libc::c_uint
                    {
                        current_block_696 = 4228772074787123047;
                    } else {
                        current_block_696 = 46020513507895021;
                    }
                } else {
                    current_block_696 = 46020513507895021;
                }
                match current_block_696 {
                    4228772074787123047 => {}
                    _ => {
                        let mut ICNT_2980: size_t = 0 as libc::c_int as size_t;
                        loop {
                            secnum = Tag_Iterate_Sectors(
                                (*line).args[0 as libc::c_int as usize] as mtag_t,
                                ICNT_2980,
                            );
                            if !(secnum >= 0 as libc::c_int) {
                                break;
                            }
                            let mut tryagain: boolean = 0;
                            loop {
                                tryagain = false_0 as libc::c_int;
                                thing = (*sectors.offset(secnum as isize)).thinglist;
                                while !thing.is_null() {
                                    if !((*thing).type_0 as libc::c_uint
                                        != type_0 as libc::c_uint)
                                    {
                                        if P_SetMobjState(
                                            thing,
                                            (if (*line).args[1 as libc::c_int as usize] != 0 {
                                                (*(*thing).state).nextstate as libc::c_uint
                                            } else {
                                                state_0 as libc::c_uint
                                            }) as statenum_t,
                                        ) == 0
                                        {
                                            tryagain = true_0 as libc::c_int;
                                            break;
                                        }
                                    }
                                    thing = (*thing).snext;
                                }
                                if !(tryagain != 0) {
                                    break;
                                }
                            }
                            ICNT_2980 = ICNT_2980.wrapping_add(1);
                            ICNT_2980;
                        }
                    }
                }
            }
        }
        443 => {
            if !((*line).stringargs[0 as libc::c_int as usize]).is_null() {
                LUA_HookLinedefExecute(line, mo, callsec);
            } else {
                CONS_Alert(
                    CONS_WARNING,
                    b"Linedef %s is missing the hook name of the Lua function to call! (This should be given in stringarg0)\n\0"
                        as *const u8 as *const libc::c_char,
                    sizeu1(line.offset_from(lines) as libc::c_long as size_t),
                );
            }
        }
        444 => {
            quake
                .intensity = (*line).args[1 as libc::c_int as usize]
                << 16 as libc::c_int;
            quake.radius = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
            quake.time = (*line).args[0 as libc::c_int as usize] as uint16_t;
            quake.epicenter = 0 as *mut mappoint_t;
            if quake.intensity == 0 {
                quake.intensity = (8 as libc::c_int) << 16 as libc::c_int;
            }
            if quake.radius == 0 {
                quake.radius = (512 as libc::c_int) << 16 as libc::c_int;
            }
        }
        445 => {
            let mut sectag_0: int16_t = (*line).args[0 as libc::c_int as usize]
                as int16_t;
            let mut foftag_0: int16_t = (*line).args[1 as libc::c_int as usize]
                as int16_t;
            let mut sec_0: *mut sector_t = 0 as *mut sector_t;
            let mut rover_0: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover_0: boolean = false_0 as libc::c_int;
            let mut oldflags: ffloortype_e = 0 as ffloortype_e;
            let mut ICNT_3033: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag_0, ICNT_3033);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec_0 = sectors.offset(secnum as isize);
                if ((*sec_0).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 445 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                rover_0 = (*sec_0).ffloors;
                while !rover_0.is_null() {
                    if Tag_Find(&mut (*(*(*rover_0).master).frontsector).tags, foftag_0)
                        != 0
                    {
                        foundrover_0 = true_0 as libc::c_int;
                        oldflags = (*rover_0).fofflags;
                        if (*line).args[2 as libc::c_int as usize] != 0 {
                            (*rover_0)
                                .fofflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                (*rover_0).fofflags as libc::c_uint
                                    | FOF_EXISTS as libc::c_int as libc::c_uint,
                            );
                        } else {
                            (*rover_0)
                                .fofflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                (*rover_0).fofflags as libc::c_uint
                                    & !(FOF_EXISTS as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*rover_0).fofflags as libc::c_uint
                            != oldflags as libc::c_uint
                        {
                            (*sec_0).moved = true_0 as libc::c_int;
                            P_RecalcPrecipInSector(sec_0);
                        }
                    }
                    rover_0 = (*rover_0).next;
                }
                if foundrover_0 == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 445 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag_0 as libc::c_int,
                    );
                    return;
                }
                ICNT_3033 = ICNT_3033.wrapping_add(1);
                ICNT_3033;
            }
        }
        446 => {
            let mut sectag_1: int16_t = (*line).args[0 as libc::c_int as usize]
                as int16_t;
            let mut foftag_1: int16_t = (*line).args[1 as libc::c_int as usize]
                as int16_t;
            let mut sec_1: *mut sector_t = 0 as *mut sector_t;
            let mut rover_1: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover_1: boolean = false_0 as libc::c_int;
            let mut player: *mut player_t = 0 as *mut player_t;
            let mut respawn: boolean = true_0 as libc::c_int;
            if !mo.is_null() {
                player = (*mo).player;
            }
            if (*line).args[2 as libc::c_int as usize] & TMFR_NORETURN as libc::c_int
                != 0
            {
                respawn = false_0 as libc::c_int;
            }
            let mut ICNT_3091: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag_1, ICNT_3091);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec_1 = sectors.offset(secnum as isize);
                if ((*sec_1).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 446 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                rover_1 = (*sec_1).ffloors;
                while !rover_1.is_null() {
                    if Tag_Find(&mut (*(*(*rover_1).master).frontsector).tags, foftag_1)
                        != 0
                    {
                        foundrover_1 = true_0 as libc::c_int;
                        if (*line).args[2 as libc::c_int as usize]
                            & TMFR_CHECKFLAG as libc::c_int != 0
                        {
                            respawn = ((*rover_1).fofflags as libc::c_uint
                                & FOF_NORETURN as libc::c_int as libc::c_uint == 0)
                                as libc::c_int
                                ^ ((*line).args[2 as libc::c_int as usize]
                                    & TMFR_NORETURN as libc::c_int != 0) as libc::c_int;
                        }
                        EV_StartCrumble(
                            (*(*rover_1).master).frontsector,
                            rover_1,
                            ((*rover_1).fofflags as libc::c_uint
                                & FOF_FLOATBOB as libc::c_int as libc::c_uint) as boolean,
                            player,
                            (*rover_1).alpha,
                            respawn,
                        );
                    }
                    rover_1 = (*rover_1).next;
                }
                if foundrover_1 == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 446 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag_1 as libc::c_int,
                    );
                    return;
                }
                ICNT_3091 = ICNT_3091.wrapping_add(1);
                ICNT_3091;
            }
        }
        447 => {
            let mut source: *mut extracolormap_t = 0 as *mut extracolormap_t;
            if udmf == 0 {
                source = (*sides
                    .offset((*line).sidenum[0 as libc::c_int as usize] as isize))
                    .colormap_data;
            } else if (*line).args[1 as libc::c_int as usize] == 0 {
                source = (*(*line).frontsector).extra_colormap;
            } else {
                let mut sourcesec: int32_t = Tag_Iterate_Sectors(
                    (*line).args[1 as libc::c_int as usize] as mtag_t,
                    0 as libc::c_int as size_t,
                );
                if sourcesec == -(1 as libc::c_int) {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 447 Executor: Can't find sector with source colormap (tag %d)!\n\0"
                            as *const u8 as *const libc::c_char,
                        (*line).args[1 as libc::c_int as usize],
                    );
                    return;
                }
                source = (*sectors.offset(sourcesec as isize)).extra_colormap;
            }
            let mut ICNT_3147: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_3147,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if !((*sectors.offset(secnum as isize)).colormap_protected != 0) {
                    P_ResetColormapFader(&mut *sectors.offset(secnum as isize));
                    if (*line).args[2 as libc::c_int as usize]
                        & TMCF_RELATIVE as libc::c_int != 0
                    {
                        let mut target: *mut extracolormap_t = if udmf == 0
                            && (*line).flags as libc::c_int & 32768 as libc::c_int != 0
                            && (*line).sidenum[1 as libc::c_int as usize] as libc::c_int
                                != 0xffff as libc::c_int
                        {
                            (*sides
                                .offset(
                                    (*line).sidenum[1 as libc::c_int as usize] as isize,
                                ))
                                .colormap_data
                        } else {
                            (*sectors.offset(secnum as isize)).extra_colormap
                        };
                        let mut exc: *mut extracolormap_t = R_AddColormaps(
                            target,
                            source,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBLIGHTR as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBLIGHTG as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBLIGHTB as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBLIGHTA as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADER as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADEG as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADEB as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADEA as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADESTART as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_SUBFADEEND as libc::c_int,
                            (*line).args[2 as libc::c_int as usize]
                                & TMCF_IGNOREFLAGS as libc::c_int,
                            false_0 as libc::c_int,
                        );
                        let ref mut fresh7 = (*sectors.offset(secnum as isize))
                            .extra_colormap;
                        *fresh7 = R_GetColormapFromList(exc);
                        if (*fresh7).is_null() {
                            (*exc).colormap = R_CreateLightTable(exc);
                            R_AddColormapToList(exc);
                            let ref mut fresh8 = (*sectors.offset(secnum as isize))
                                .extra_colormap;
                            *fresh8 = exc;
                        } else {
                            Z_Free(exc as *mut libc::c_void);
                        }
                    } else {
                        let ref mut fresh9 = (*sectors.offset(secnum as isize))
                            .extra_colormap;
                        *fresh9 = source;
                    }
                }
                ICNT_3147 = ICNT_3147.wrapping_add(1);
                ICNT_3147;
            }
        }
        448 => {
            if !mo.is_null() && !((*mo).player).is_null()
                && P_IsLocalPlayer((*mo).player) != 0
                || (*line).args[3 as libc::c_int as usize] != 0
            {
                let mut viewid: int32_t = (*line).args[0 as libc::c_int as usize];
                let mut centerid: int32_t = (*line).args[1 as libc::c_int as usize];
                if (*line).args[2 as libc::c_int as usize]
                    != TMS_CENTERPOINT as libc::c_int
                {
                    if viewid >= 0 as libc::c_int && viewid < 16 as libc::c_int {
                        skyboxmo[0 as libc::c_int
                            as usize] = skyboxviewpnts[viewid as usize];
                    } else {
                        skyboxmo[0 as libc::c_int as usize] = 0 as *mut mobj_t;
                    }
                }
                if (*line).args[2 as libc::c_int as usize]
                    != TMS_VIEWPOINT as libc::c_int
                {
                    if centerid >= 0 as libc::c_int && centerid < 16 as libc::c_int {
                        skyboxmo[1 as libc::c_int
                            as usize] = skyboxcenterpnts[centerid as usize];
                    } else {
                        skyboxmo[1 as libc::c_int as usize] = 0 as *mut mobj_t;
                    }
                }
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Line type 448 Executor: viewid = %d, centerid = %d, viewpoint? = %s, centerpoint? = %s\n\0"
                        as *const u8 as *const libc::c_char,
                    viewid,
                    centerid,
                    if (*line).args[2 as libc::c_int as usize]
                        == TMS_CENTERPOINT as libc::c_int
                    {
                        b"no\0" as *const u8 as *const libc::c_char
                    } else {
                        b"yes\0" as *const u8 as *const libc::c_char
                    },
                    if (*line).args[2 as libc::c_int as usize]
                        == TMS_VIEWPOINT as libc::c_int
                    {
                        b"no\0" as *const u8 as *const libc::c_char
                    } else {
                        b"yes\0" as *const u8 as *const libc::c_char
                    },
                );
            }
        }
        449 => {
            let mut bossid: int32_t = (*line).args[0 as libc::c_int as usize];
            if bossid & !(15 as libc::c_int) != 0 {
                CONS_Alert(
                    CONS_WARNING,
                    b"Boss enable linedef has an invalid boss ID (%d).\nConsider changing it or removing it entirely.\n\0"
                        as *const u8 as *const libc::c_char,
                    bossid,
                );
            } else if (*line).args[1 as libc::c_int as usize] != 0 {
                bossdisabled = (bossdisabled as libc::c_int
                    | (1 as libc::c_int) << bossid) as uint16_t;
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Line type 449 Executor: bossid disabled = %d\0" as *const u8
                        as *const libc::c_char,
                    bossid,
                );
            } else {
                bossdisabled = (bossdisabled as libc::c_int
                    & !((1 as libc::c_int) << bossid)) as uint16_t;
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Line type 449 Executor: bossid enabled = %d\0" as *const u8
                        as *const libc::c_char,
                    bossid,
                );
            }
        }
        450 => {
            P_LinedefExecute(
                (*line).args[0 as libc::c_int as usize] as int16_t,
                mo,
                0 as *mut sector_t,
            );
        }
        451 => {
            let mut rvalue1: int32_t = (*line).args[0 as libc::c_int as usize];
            let mut rvalue2: int32_t = (*line).args[1 as libc::c_int as usize];
            let mut result: int32_t = 0;
            if rvalue1 <= rvalue2 {
                result = P_RandomRange(rvalue1, rvalue2);
            } else {
                result = P_RandomRange(rvalue2, rvalue1);
            }
            P_LinedefExecute(result as int16_t, mo, 0 as *mut sector_t);
        }
        452 => {
            let mut destvalue: int16_t = (*line).args[2 as libc::c_int as usize]
                as int16_t;
            let mut sectag_2: int16_t = (*line).args[0 as libc::c_int as usize]
                as int16_t;
            let mut foftag_2: int16_t = (*line).args[1 as libc::c_int as usize]
                as int16_t;
            let mut sec_2: *mut sector_t = 0 as *mut sector_t;
            let mut rover_2: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover_2: boolean = false_0 as libc::c_int;
            let mut ICNT_3272: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag_2, ICNT_3272);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec_2 = sectors.offset(secnum as isize);
                if ((*sec_2).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 452 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                rover_2 = (*sec_2).ffloors;
                while !rover_2.is_null() {
                    if Tag_Find(&mut (*(*(*rover_2).master).frontsector).tags, foftag_2)
                        != 0
                    {
                        foundrover_2 = true_0 as libc::c_int;
                        if (*line).args[3 as libc::c_int as usize]
                            & TMST_DONTDOTRANSLUCENT as libc::c_int == 0
                            && (*rover_2).spawnflags as libc::c_uint
                                & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
                            && (*rover_2).spawnflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
                            && (*rover_2).spawnflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
                            && (*rover_2).fofflags as libc::c_uint
                                & FOF_RENDERALL as libc::c_int as libc::c_uint == 0
                        {
                            (*rover_2).alpha = 0 as libc::c_int;
                        }
                        P_ResetFakeFloorFader(
                            rover_2,
                            0 as *mut fade_t,
                            false_0 as libc::c_int,
                        );
                        P_FadeFakeFloor(
                            rover_2,
                            (*rover_2).alpha as int16_t,
                            (if 0 as libc::c_int
                                > (if (255 as libc::c_int)
                                    < (if (*line).args[3 as libc::c_int as usize]
                                        & TMST_RELATIVE as libc::c_int != 0
                                    {
                                        (*rover_2).alpha + destvalue as libc::c_int
                                    } else {
                                        destvalue as libc::c_int
                                    })
                                {
                                    255 as libc::c_int
                                } else {
                                    (if (*line).args[3 as libc::c_int as usize]
                                        & TMST_RELATIVE as libc::c_int != 0
                                    {
                                        (*rover_2).alpha + destvalue as libc::c_int
                                    } else {
                                        destvalue as libc::c_int
                                    })
                                })
                            {
                                0 as libc::c_int
                            } else if (255 as libc::c_int)
                                < (if (*line).args[3 as libc::c_int as usize]
                                    & TMST_RELATIVE as libc::c_int != 0
                                {
                                    (*rover_2).alpha + destvalue as libc::c_int
                                } else {
                                    destvalue as libc::c_int
                                })
                            {
                                255 as libc::c_int
                            } else if (*line).args[3 as libc::c_int as usize]
                                & TMST_RELATIVE as libc::c_int != 0
                            {
                                (*rover_2).alpha + destvalue as libc::c_int
                            } else {
                                destvalue as libc::c_int
                            }) as int16_t,
                            0 as libc::c_int as int16_t,
                            false_0 as libc::c_int,
                            0 as *mut int32_t,
                            false_0 as libc::c_int,
                            ((*line).args[3 as libc::c_int as usize]
                                & TMST_DONTDOTRANSLUCENT as libc::c_int == 0)
                                as libc::c_int,
                            false_0 as libc::c_int,
                            false_0 as libc::c_int,
                            false_0 as libc::c_int,
                            false_0 as libc::c_int,
                            true_0 as libc::c_int,
                        );
                    }
                    rover_2 = (*rover_2).next;
                }
                if foundrover_2 == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 452 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag_2 as libc::c_int,
                    );
                    return;
                }
                ICNT_3272 = ICNT_3272.wrapping_add(1);
                ICNT_3272;
            }
        }
        453 => {
            let mut destvalue_0: int16_t = (*line).args[2 as libc::c_int as usize]
                as int16_t;
            let mut speed_0: int16_t = (*line).args[3 as libc::c_int as usize]
                as int16_t;
            let mut sectag_3: int16_t = (*line).args[0 as libc::c_int as usize]
                as int16_t;
            let mut foftag_3: int16_t = (*line).args[1 as libc::c_int as usize]
                as int16_t;
            let mut sec_3: *mut sector_t = 0 as *mut sector_t;
            let mut rover_3: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover_3: boolean = false_0 as libc::c_int;
            let mut j: size_t = 0 as libc::c_int as size_t;
            let mut ICNT_3333: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag_3, ICNT_3333);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec_3 = sectors.offset(secnum as isize);
                if ((*sec_3).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 453 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                let mut current_block_500: u64;
                rover_3 = (*sec_3).ffloors;
                while !rover_3.is_null() {
                    if Tag_Find(&mut (*(*(*rover_3).master).frontsector).tags, foftag_3)
                        != 0
                    {
                        foundrover_3 = true_0 as libc::c_int;
                        if (*line).args[4 as libc::c_int as usize]
                            & TMFT_OVERRIDE as libc::c_int == 0
                            && !((*rover_3).fadingdata).is_null()
                        {
                            CONS_Debug(
                                0x80 as libc::c_int,
                                b"Line type 453 Executor: Fade FOF thinker already exists, timer: %d\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*((*rover_3).fadingdata as *mut fade_t)).timer,
                            );
                            current_block_500 = 10433728831355628950;
                        } else {
                            if speed_0 as libc::c_int > 0 as libc::c_int {
                                P_AddFakeFloorFader(
                                    rover_3,
                                    secnum as size_t,
                                    j,
                                    destvalue_0,
                                    speed_0,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_TICBASED as libc::c_int,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_RELATIVE as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOEXISTS as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOTRANSLUCENT as libc::c_int == 0)
                                        as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOLIGHTING as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOCOLORMAP as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_IGNORECOLLISION as libc::c_int == 0) as libc::c_int,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_GHOSTFADE as libc::c_int,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_USEEXACTALPHA as libc::c_int,
                                );
                            } else {
                                if (*line).args[4 as libc::c_int as usize]
                                    & TMFT_DONTDOTRANSLUCENT as libc::c_int == 0
                                    && (*rover_3).spawnflags as libc::c_uint
                                        & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
                                    && (*rover_3).spawnflags as libc::c_uint
                                        & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
                                    && (*rover_3).spawnflags as libc::c_uint
                                        & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
                                    && (*rover_3).fofflags as libc::c_uint
                                        & FOF_RENDERALL as libc::c_int as libc::c_uint == 0
                                {
                                    (*rover_3).alpha = 0 as libc::c_int;
                                }
                                P_ResetFakeFloorFader(
                                    rover_3,
                                    0 as *mut fade_t,
                                    false_0 as libc::c_int,
                                );
                                P_FadeFakeFloor(
                                    rover_3,
                                    (*rover_3).alpha as int16_t,
                                    (if 0 as libc::c_int
                                        > (if (255 as libc::c_int)
                                            < (if (*line).args[4 as libc::c_int as usize]
                                                & TMFT_RELATIVE as libc::c_int != 0
                                            {
                                                (*rover_3).alpha + destvalue_0 as libc::c_int
                                            } else {
                                                destvalue_0 as libc::c_int
                                            })
                                        {
                                            255 as libc::c_int
                                        } else {
                                            (if (*line).args[4 as libc::c_int as usize]
                                                & TMFT_RELATIVE as libc::c_int != 0
                                            {
                                                (*rover_3).alpha + destvalue_0 as libc::c_int
                                            } else {
                                                destvalue_0 as libc::c_int
                                            })
                                        })
                                    {
                                        0 as libc::c_int
                                    } else if (255 as libc::c_int)
                                        < (if (*line).args[4 as libc::c_int as usize]
                                            & TMFT_RELATIVE as libc::c_int != 0
                                        {
                                            (*rover_3).alpha + destvalue_0 as libc::c_int
                                        } else {
                                            destvalue_0 as libc::c_int
                                        })
                                    {
                                        255 as libc::c_int
                                    } else if (*line).args[4 as libc::c_int as usize]
                                        & TMFT_RELATIVE as libc::c_int != 0
                                    {
                                        (*rover_3).alpha + destvalue_0 as libc::c_int
                                    } else {
                                        destvalue_0 as libc::c_int
                                    }) as int16_t,
                                    0 as libc::c_int as int16_t,
                                    false_0 as libc::c_int,
                                    0 as *mut int32_t,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOEXISTS as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOTRANSLUCENT as libc::c_int == 0)
                                        as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOLIGHTING as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_DONTDOCOLORMAP as libc::c_int == 0) as libc::c_int,
                                    ((*line).args[4 as libc::c_int as usize]
                                        & TMFT_IGNORECOLLISION as libc::c_int == 0) as libc::c_int,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_GHOSTFADE as libc::c_int,
                                    (*line).args[4 as libc::c_int as usize]
                                        & TMFT_USEEXACTALPHA as libc::c_int,
                                );
                            }
                            current_block_500 = 17336618653903260752;
                        }
                    } else {
                        current_block_500 = 17336618653903260752;
                    }
                    match current_block_500 {
                        17336618653903260752 => {
                            j = j.wrapping_add(1);
                            j;
                        }
                        _ => {}
                    }
                    rover_3 = (*rover_3).next;
                }
                if foundrover_3 == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 453 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag_3 as libc::c_int,
                    );
                    return;
                }
                ICNT_3333 = ICNT_3333.wrapping_add(1);
                ICNT_3333;
            }
        }
        454 => {
            let mut sectag_4: int16_t = (*line).args[0 as libc::c_int as usize]
                as int16_t;
            let mut foftag_4: int16_t = (*line).args[1 as libc::c_int as usize]
                as int16_t;
            let mut sec_4: *mut sector_t = 0 as *mut sector_t;
            let mut rover_4: *mut ffloor_t = 0 as *mut ffloor_t;
            let mut foundrover_4: boolean = false_0 as libc::c_int;
            let mut ICNT_3417: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(sectag_4, ICNT_3417);
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                sec_4 = sectors.offset(secnum as isize);
                if ((*sec_4).ffloors).is_null() {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 454 Executor: Target sector #%d has no FOFs.\n\0"
                            as *const u8 as *const libc::c_char,
                        secnum,
                    );
                    return;
                }
                rover_4 = (*sec_4).ffloors;
                while !rover_4.is_null() {
                    if Tag_Find(&mut (*(*(*rover_4).master).frontsector).tags, foftag_4)
                        != 0
                    {
                        foundrover_4 = true_0 as libc::c_int;
                        P_ResetFakeFloorFader(
                            rover_4,
                            0 as *mut fade_t,
                            ((*line).args[2 as libc::c_int as usize] == 0) as libc::c_int,
                        );
                    }
                    rover_4 = (*rover_4).next;
                }
                if foundrover_4 == 0 {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 454 Executor: Can't find a FOF control sector with tag %d\n\0"
                            as *const u8 as *const libc::c_char,
                        foftag_4 as libc::c_int,
                    );
                    return;
                }
                ICNT_3417 = ICNT_3417.wrapping_add(1);
                ICNT_3417;
            }
        }
        455 => {
            let mut dest_0: *mut extracolormap_t = 0 as *mut extracolormap_t;
            if udmf == 0 {
                dest_0 = (*sides
                    .offset((*line).sidenum[0 as libc::c_int as usize] as isize))
                    .colormap_data;
            } else if (*line).args[1 as libc::c_int as usize] == 0 {
                dest_0 = (*(*line).frontsector).extra_colormap;
            } else {
                let mut destsec: int32_t = Tag_Iterate_Sectors(
                    (*line).args[1 as libc::c_int as usize] as mtag_t,
                    0 as libc::c_int as size_t,
                );
                if destsec == -(1 as libc::c_int) {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Line type 455 Executor: Can't find sector with destination colormap (tag %d)!\n\0"
                            as *const u8 as *const libc::c_char,
                        (*line).args[1 as libc::c_int as usize],
                    );
                    return;
                }
                dest_0 = (*sectors.offset(destsec as isize)).extra_colormap;
            }
            let mut ICNT_3468: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_3468,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                let mut source_exc: *mut extracolormap_t = 0 as *mut extracolormap_t;
                let mut dest_exc: *mut extracolormap_t = 0 as *mut extracolormap_t;
                let mut exc_0: *mut extracolormap_t = 0 as *mut extracolormap_t;
                if !((*sectors.offset(secnum as isize)).colormap_protected != 0) {
                    if (*line).args[3 as libc::c_int as usize]
                        & TMCF_OVERRIDE as libc::c_int == 0
                        && !((*sectors.offset(secnum as isize)).fadecolormapdata)
                            .is_null()
                    {
                        CONS_Debug(
                            0x80 as libc::c_int,
                            b"Line type 455 Executor: Fade color thinker already exists, timer: %d\n\0"
                                as *const u8 as *const libc::c_char,
                            (*((*sectors.offset(secnum as isize)).fadecolormapdata
                                as *mut fadecolormap_t))
                                .timer,
                        );
                    } else {
                        if udmf == 0
                            && (*line).flags as libc::c_int & 32768 as libc::c_int != 0
                        {
                            let ref mut fresh10 = (*sectors.offset(secnum as isize))
                                .extra_colormap;
                            *fresh10 = if (*line).sidenum[1 as libc::c_int as usize]
                                as libc::c_int != 0xffff as libc::c_int
                            {
                                (*sides
                                    .offset(
                                        (*line).sidenum[1 as libc::c_int as usize] as isize,
                                    ))
                                    .colormap_data
                            } else {
                                0 as *mut extracolormap_t
                            };
                        }
                        exc_0 = (*sectors.offset(secnum as isize)).extra_colormap;
                        if (*line).args[3 as libc::c_int as usize]
                            & TMCF_FROMBLACK as libc::c_int == 0
                            && R_CheckDefaultColormap(
                                dest_0,
                                true_0 as libc::c_int,
                                false_0 as libc::c_int,
                                false_0 as libc::c_int,
                            ) == 0
                            && R_CheckDefaultColormap(
                                exc_0,
                                true_0 as libc::c_int,
                                false_0 as libc::c_int,
                                false_0 as libc::c_int,
                            ) != 0
                        {
                            exc_0 = R_CopyColormap(exc_0, false_0 as libc::c_int);
                            (*exc_0)
                                .rgba = ((*dest_0).rgba & 0xffffff as libc::c_int)
                                + (((*exc_0).rgba >> 24 as libc::c_int
                                    & 0xff as libc::c_int) << 24 as libc::c_int);
                            source_exc = R_GetColormapFromList(exc_0);
                            if source_exc.is_null() {
                                (*exc_0).colormap = R_CreateLightTable(exc_0);
                                R_AddColormapToList(exc_0);
                                source_exc = exc_0;
                            } else {
                                Z_Free(exc_0 as *mut libc::c_void);
                            }
                            let ref mut fresh11 = (*sectors.offset(secnum as isize))
                                .extra_colormap;
                            *fresh11 = source_exc;
                        } else {
                            source_exc = if !exc_0.is_null() {
                                exc_0
                            } else {
                                R_GetDefaultColormap()
                            };
                        }
                        if (*line).args[3 as libc::c_int as usize]
                            & TMCF_RELATIVE as libc::c_int != 0
                        {
                            exc_0 = R_AddColormaps(
                                source_exc,
                                dest_0,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBLIGHTR as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBLIGHTG as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBLIGHTB as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBLIGHTA as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADER as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADEG as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADEB as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADEA as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADESTART as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_SUBFADEEND as libc::c_int,
                                (*line).args[3 as libc::c_int as usize]
                                    & TMCF_IGNOREFLAGS as libc::c_int,
                                false_0 as libc::c_int,
                            );
                        } else {
                            exc_0 = R_CopyColormap(dest_0, false_0 as libc::c_int);
                        }
                        dest_exc = R_GetColormapFromList(exc_0);
                        if dest_exc.is_null() {
                            (*exc_0).colormap = R_CreateLightTable(exc_0);
                            R_AddColormapToList(exc_0);
                            dest_exc = exc_0;
                        } else {
                            Z_Free(exc_0 as *mut libc::c_void);
                        }
                        Add_ColormapFader(
                            &mut *sectors.offset(secnum as isize),
                            source_exc,
                            dest_exc,
                            true_0 as libc::c_int,
                            (*line).args[2 as libc::c_int as usize],
                        );
                    }
                }
                ICNT_3468 = ICNT_3468.wrapping_add(1);
                ICNT_3468;
            }
        }
        456 => {
            let mut ICNT_3548: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_3548,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                P_ResetColormapFader(&mut *sectors.offset(secnum as isize));
                ICNT_3548 = ICNT_3548.wrapping_add(1);
                ICNT_3548;
            }
        }
        457 => {
            if !mo.is_null() {
                let mut failureangle: int32_t = FixedAngle(
                    (if (if abs((*line).args[1 as libc::c_int as usize])
                        > 0 as libc::c_int
                    {
                        abs((*line).args[1 as libc::c_int as usize])
                    } else {
                        0 as libc::c_int
                    }) < 360 as libc::c_int
                    {
                        (if abs((*line).args[1 as libc::c_int as usize])
                            > 0 as libc::c_int
                        {
                            abs((*line).args[1 as libc::c_int as usize])
                        } else {
                            0 as libc::c_int
                        })
                    } else {
                        360 as libc::c_int
                    }) * ((1 as libc::c_int) << 16 as libc::c_int),
                ) as int32_t;
                let mut failuredelay: int32_t = abs(
                    (*line).args[2 as libc::c_int as usize],
                );
                let mut failureexectag: int32_t = (*line)
                    .args[3 as libc::c_int as usize];
                let mut persist: boolean = ((*line).args[4 as libc::c_int as usize] != 0)
                    as libc::c_int;
                let mut anchormo: *mut mobj_t = 0 as *mut mobj_t;
                anchormo = P_FindObjectTypeFromTag(
                    MT_ANGLEMAN,
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                );
                if anchormo.is_null() {
                    return;
                }
                (*mo)
                    .eflags = ((*mo).eflags as libc::c_int
                    | MFE_TRACERANGLE as libc::c_int) as uint16_t;
                P_SetTarget2(&mut (*mo).tracer, anchormo);
                (*mo).lastlook = persist;
                (*mo).extravalue1 = failureangle;
                (*mo).extravalue2 = failureexectag;
                (*mo).cvmem = failuredelay;
                (*mo).cusval = (*mo).cvmem;
            }
        }
        458 => {
            if !mo.is_null()
                && (*mo).eflags as libc::c_int & MFE_TRACERANGLE as libc::c_int != 0
            {
                (*mo)
                    .eflags = ((*mo).eflags as libc::c_int
                    & !(MFE_TRACERANGLE as libc::c_int)) as uint16_t;
                P_SetTarget2(&mut (*mo).tracer, 0 as *mut mobj_t);
                (*mo).extravalue2 = 0 as libc::c_int;
                (*mo).extravalue1 = (*mo).extravalue2;
                (*mo).cusval = (*mo).extravalue1;
                (*mo).cvmem = (*mo).cusval;
                (*mo).lastlook = (*mo).cvmem;
            }
        }
        459 => {
            if !mo.is_null() && !((*mo).player).is_null()
                && P_IsLocalPlayer((*mo).player) != 0 && (bot.is_null() || bot != mo)
            {
                let mut promptnum: int32_t = if 0 as libc::c_int
                    > (*line).args[0 as libc::c_int as usize] - 1 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    (*line).args[0 as libc::c_int as usize] - 1 as libc::c_int
                };
                let mut pagenum: int32_t = if 0 as libc::c_int
                    > (*line).args[1 as libc::c_int as usize] - 1 as libc::c_int
                {
                    0 as libc::c_int
                } else {
                    (*line).args[1 as libc::c_int as usize] - 1 as libc::c_int
                };
                let mut postexectag: int32_t = abs(
                    (*line).args[3 as libc::c_int as usize],
                );
                let mut closetextprompt: boolean = (*line)
                    .args[2 as libc::c_int as usize] & TMP_CLOSE as libc::c_int;
                let mut runpostexec: boolean = (*line).args[2 as libc::c_int as usize]
                    & TMP_RUNPOSTEXEC as libc::c_int;
                let mut blockcontrols: boolean = ((*line).args[2 as libc::c_int as usize]
                    & TMP_KEEPCONTROLS as libc::c_int == 0) as libc::c_int;
                let mut freezerealtime: boolean = ((*line)
                    .args[2 as libc::c_int as usize] & TMP_KEEPREALTIME as libc::c_int
                    == 0) as libc::c_int;
                let mut callbynamedtag: boolean = (*line).args[2 as libc::c_int as usize]
                    & TMP_CALLBYNAME as libc::c_int;
                if closetextprompt != 0 {
                    F_EndTextPrompt(false_0 as libc::c_int, false_0 as libc::c_int);
                } else {
                    if callbynamedtag != 0
                        && !((*line).stringargs[0 as libc::c_int as usize]).is_null()
                        && *((*line).stringargs[0 as libc::c_int as usize])
                            .offset(0 as libc::c_int as isize) as libc::c_int != 0
                    {
                        F_GetPromptPageByNamedTag(
                            (*line).stringargs[0 as libc::c_int as usize],
                            &mut promptnum,
                            &mut pagenum,
                        );
                    }
                    F_StartTextPrompt(
                        promptnum,
                        pagenum,
                        mo,
                        (if runpostexec != 0 { postexectag } else { 0 as libc::c_int })
                            as uint16_t,
                        blockcontrols,
                        freezerealtime,
                    );
                }
            }
        }
        460 => {
            let mut rings: int16_t = (*line).args[0 as libc::c_int as usize] as int16_t;
            let mut delay: int32_t = (*line).args[1 as libc::c_int as usize];
            if !mo.is_null() && !((*mo).player).is_null() {
                if delay <= 0 as libc::c_int || leveltime % delay as tic_t == 0 {
                    P_GivePlayerRings((*mo).player, rings as int32_t);
                }
            }
        }
        461 => {
            let type_1: mobjtype_t = (if !((*line).stringargs[0 as libc::c_int as usize])
                .is_null()
            {
                get_number((*line).stringargs[0 as libc::c_int as usize])
            } else {
                MT_NULL as libc::c_int
            }) as mobjtype_t;
            let mut mobj: *mut mobj_t = 0 as *mut mobj_t;
            let mut x_0: fixed_t = 0;
            let mut y_0: fixed_t = 0;
            let mut z_0: fixed_t = 0;
            if (*line).args[4 as libc::c_int as usize] != 0 {
                x_0 = P_RandomRange(
                    (*line).args[0 as libc::c_int as usize],
                    (*line).args[5 as libc::c_int as usize],
                ) << 16 as libc::c_int;
                y_0 = P_RandomRange(
                    (*line).args[1 as libc::c_int as usize],
                    (*line).args[6 as libc::c_int as usize],
                ) << 16 as libc::c_int;
                z_0 = P_RandomRange(
                    (*line).args[2 as libc::c_int as usize],
                    (*line).args[7 as libc::c_int as usize],
                ) << 16 as libc::c_int;
            } else {
                x_0 = (*line).args[0 as libc::c_int as usize] << 16 as libc::c_int;
                y_0 = (*line).args[1 as libc::c_int as usize] << 16 as libc::c_int;
                z_0 = (*line).args[2 as libc::c_int as usize] << 16 as libc::c_int;
            }
            mobj = P_SpawnMobj(x_0, y_0, z_0, type_1);
            if !mobj.is_null() {
                (*mobj)
                    .angle = FixedAngle(
                    (*line).args[3 as libc::c_int as usize] << 16 as libc::c_int,
                );
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Linedef Type %d - Spawn Object: %d spawned at (%d, %d, %d)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*line).special as libc::c_int,
                    (*mobj).type_0 as libc::c_uint,
                    (*mobj).x >> 16 as libc::c_int,
                    (*mobj).y >> 16 as libc::c_int,
                    (*mobj).z >> 16 as libc::c_int,
                );
            } else {
                CONS_Alert(
                    CONS_ERROR,
                    b"Linedef Type %d - Spawn Object: Object did not spawn!\n\0"
                        as *const u8 as *const libc::c_char,
                    (*line).special as libc::c_int,
                );
            }
        }
        462 => {
            if G_PlatformGametype() != 0 {
                stoppedclock = true_0 as libc::c_int;
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Clock stopped!\n\0" as *const u8 as *const libc::c_char,
                );
                if modeattacking != 0 {
                    let mut i: uint8_t = 0;
                    i = 0 as libc::c_int as uint8_t;
                    while (i as libc::c_int) < 32 as libc::c_int {
                        if !(playeringame[i as usize] == 0) {
                            P_DoPlayerExit(
                                &mut *players.as_mut_ptr().offset(i as isize),
                            );
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
        }
        463 => {
            if !mo.is_null() {
                let mut color: int32_t = if !((*line)
                    .stringargs[0 as libc::c_int as usize])
                    .is_null()
                {
                    get_number((*line).stringargs[0 as libc::c_int as usize])
                } else {
                    SKINCOLOR_NONE as libc::c_int
                };
                if color < 0 as libc::c_int || color >= numskincolors as libc::c_int {
                    return;
                }
                var1 = 0 as libc::c_int;
                var2 = color;
                A_Dye(mo);
            }
        }
        464 => {
            let mut mtnum: int32_t = 0;
            let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
            let mut ICNT_3694: size_t = 0 as libc::c_int as size_t;
            loop {
                mtnum = Tag_Iterate_Things(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_3694,
                );
                if !(mtnum >= 0 as libc::c_int) {
                    break;
                }
                mo2 = (*mapthings.offset(mtnum as isize)).mobj;
                if !mo2.is_null() {
                    if !((*mo2).type_0 as libc::c_uint
                        != MT_EGGTRAP as libc::c_int as libc::c_uint)
                    {
                        if !((*mo2).thinker.function.acp1
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
                            P_KillMobj(
                                mo2,
                                0 as *mut mobj_t,
                                mo,
                                0 as libc::c_int as uint8_t,
                            );
                        }
                    }
                }
                ICNT_3694 = ICNT_3694.wrapping_add(1);
                ICNT_3694;
            }
            if (*line).args[1 as libc::c_int as usize] == 0 {
                let mut i_0: int32_t = 0;
                i_0 = 0 as libc::c_int;
                while i_0 < 32 as libc::c_int {
                    if !(playeringame[i_0 as usize] == 0) {
                        P_DoPlayerExit(&mut *players.as_mut_ptr().offset(i_0 as isize));
                    }
                    i_0 += 1;
                    i_0;
                }
            }
        }
        465 => {
            let mut linenum_0: int32_t = 0;
            if !(udmf == 0) {
                let mut ICNT_3732: size_t = 0 as libc::c_int as size_t;
                loop {
                    linenum_0 = Tag_Iterate_Lines(
                        (*line).args[0 as libc::c_int as usize] as mtag_t,
                        ICNT_3732,
                    );
                    if !(linenum_0 >= 0 as libc::c_int) {
                        break;
                    }
                    if (*line).args[2 as libc::c_int as usize] != 0 {
                        (*lines.offset(linenum_0 as isize)).executordelay
                            += (*line).args[1 as libc::c_int as usize];
                    } else {
                        (*lines.offset(linenum_0 as isize))
                            .executordelay = (*line).args[1 as libc::c_int as usize];
                    }
                    ICNT_3732 = ICNT_3732.wrapping_add(1);
                    ICNT_3732;
                }
            }
        }
        466 => {
            if (*line).args[1 as libc::c_int as usize] != 0 {
                stagefailed = false_0 as libc::c_int;
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Stage can be completed successfully!\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                stagefailed = true_0 as libc::c_int;
                CONS_Debug(
                    0x80 as libc::c_int,
                    b"Stage will end in failure...\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        467 => {
            let mut ICNT_3758: size_t = 0 as libc::c_int as size_t;
            loop {
                secnum = Tag_Iterate_Sectors(
                    (*line).args[0 as libc::c_int as usize] as mtag_t,
                    ICNT_3758,
                );
                if !(secnum >= 0 as libc::c_int) {
                    break;
                }
                if !((*sectors.offset(secnum as isize)).lightingdata).is_null() {
                    P_RemoveThinker(
                        &mut (*((*sectors.offset(secnum as isize)).lightingdata
                            as *mut thinkerdata_t))
                            .thinker,
                    );
                    let ref mut fresh12 = (*sectors.offset(secnum as isize))
                        .lightingdata;
                    *fresh12 = 0 as *mut libc::c_void;
                }
                if (*line).args[2 as libc::c_int as usize] == TML_FLOOR as libc::c_int {
                    if (*line).args[3 as libc::c_int as usize] != 0 {
                        let ref mut fresh13 = (*sectors.offset(secnum as isize))
                            .floorlightlevel;
                        *fresh13 = (*fresh13 as libc::c_int
                            + (*line).args[1 as libc::c_int as usize]) as int16_t;
                    } else {
                        (*sectors.offset(secnum as isize))
                            .floorlightlevel = (*line).args[1 as libc::c_int as usize]
                            as int16_t;
                    }
                } else if (*line).args[2 as libc::c_int as usize]
                    == TML_CEILING as libc::c_int
                {
                    if (*line).args[3 as libc::c_int as usize] != 0 {
                        let ref mut fresh14 = (*sectors.offset(secnum as isize))
                            .ceilinglightlevel;
                        *fresh14 = (*fresh14 as libc::c_int
                            + (*line).args[1 as libc::c_int as usize]) as int16_t;
                    } else {
                        (*sectors.offset(secnum as isize))
                            .ceilinglightlevel = (*line).args[1 as libc::c_int as usize]
                            as int16_t;
                    }
                } else {
                    if (*line).args[3 as libc::c_int as usize] != 0 {
                        let ref mut fresh15 = (*sectors.offset(secnum as isize))
                            .lightlevel;
                        *fresh15 = (*fresh15 as libc::c_int
                            + (*line).args[1 as libc::c_int as usize]) as int16_t;
                    } else {
                        (*sectors.offset(secnum as isize))
                            .lightlevel = (*line).args[1 as libc::c_int as usize]
                            as int16_t;
                    }
                    (*sectors.offset(secnum as isize))
                        .lightlevel = (if 0 as libc::c_int
                        > (if (255 as libc::c_int)
                            < (*sectors.offset(secnum as isize)).lightlevel
                                as libc::c_int
                        {
                            255 as libc::c_int
                        } else {
                            (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                        })
                    {
                        0 as libc::c_int
                    } else if (255 as libc::c_int)
                        < (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                    {
                        255 as libc::c_int
                    } else {
                        (*sectors.offset(secnum as isize)).lightlevel as libc::c_int
                    }) as int16_t;
                }
                ICNT_3758 = ICNT_3758.wrapping_add(1);
                ICNT_3758;
            }
        }
        468 => {
            let mut linenum_1: int32_t = 0;
            if !(udmf == 0) {
                if (*line).args[1 as libc::c_int as usize] < 0 as libc::c_int
                    || (*line).args[1 as libc::c_int as usize] >= 10 as libc::c_int
                {
                    CONS_Debug(
                        0x80 as libc::c_int,
                        b"Linedef type 468: Invalid linedef arg %d\n\0" as *const u8
                            as *const libc::c_char,
                        (*line).args[1 as libc::c_int as usize],
                    );
                } else {
                    let mut ICNT_3805: size_t = 0 as libc::c_int as size_t;
                    loop {
                        linenum_1 = Tag_Iterate_Lines(
                            (*line).args[0 as libc::c_int as usize] as mtag_t,
                            ICNT_3805,
                        );
                        if !(linenum_1 >= 0 as libc::c_int) {
                            break;
                        }
                        if (*line).args[3 as libc::c_int as usize] != 0 {
                            (*lines.offset(linenum_1 as isize))
                                .args[(*line).args[1 as libc::c_int as usize] as usize]
                                += (*line).args[2 as libc::c_int as usize];
                        } else {
                            (*lines.offset(linenum_1 as isize))
                                .args[(*line).args[1 as libc::c_int as usize]
                                as usize] = (*line).args[2 as libc::c_int as usize];
                        }
                        ICNT_3805 = ICNT_3805.wrapping_add(1);
                        ICNT_3805;
                    }
                }
            }
        }
        469 => {
            let mut gravityvalue: fixed_t = 0;
            if !(udmf == 0) {
                if !((*line).stringargs[0 as libc::c_int as usize]).is_null() {
                    gravityvalue = FloatToFixed(
                        atof((*line).stringargs[0 as libc::c_int as usize])
                            as libc::c_float,
                    );
                    let mut ICNT_3827: size_t = 0 as libc::c_int as size_t;
                    loop {
                        secnum = Tag_Iterate_Sectors(
                            (*line).args[0 as libc::c_int as usize] as mtag_t,
                            ICNT_3827,
                        );
                        if !(secnum >= 0 as libc::c_int) {
                            break;
                        }
                        if (*line).args[1 as libc::c_int as usize] != 0 {
                            (*sectors.offset(secnum as isize))
                                .gravity = FixedMul(
                                (*sectors.offset(secnum as isize)).gravity,
                                gravityvalue,
                            );
                        } else {
                            (*sectors.offset(secnum as isize)).gravity = gravityvalue;
                        }
                        if (*line).args[2 as libc::c_int as usize]
                            == TMF_ADD as libc::c_int
                        {
                            let ref mut fresh16 = (*sectors.offset(secnum as isize))
                                .flags;
                            *fresh16 = ::core::mem::transmute::<
                                libc::c_uint,
                                sectorflags_t,
                            >(
                                *fresh16 as libc::c_uint
                                    | MSF_GRAVITYFLIP as libc::c_int as libc::c_uint,
                            );
                        } else if (*line).args[2 as libc::c_int as usize]
                            == TMF_REMOVE as libc::c_int
                        {
                            let ref mut fresh17 = (*sectors.offset(secnum as isize))
                                .flags;
                            *fresh17 = ::core::mem::transmute::<
                                libc::c_uint,
                                sectorflags_t,
                            >(
                                *fresh17 as libc::c_uint
                                    & !(MSF_GRAVITYFLIP as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*line).args[3 as libc::c_int as usize] != 0 {
                            let ref mut fresh18 = (*sectors.offset(secnum as isize))
                                .specialflags;
                            *fresh18 = ::core::mem::transmute::<
                                libc::c_uint,
                                sectorspecialflags_t,
                            >(
                                *fresh18 as libc::c_uint
                                    | SSF_GRAVITYOVERRIDE as libc::c_int as libc::c_uint,
                            );
                        }
                        ICNT_3827 = ICNT_3827.wrapping_add(1);
                        ICNT_3827;
                    }
                }
            }
        }
        480 | 481 => {
            PolyDoor(line);
        }
        482 => {
            PolyMove(line);
        }
        484 => {
            PolyRotate(line);
        }
        488 => {
            PolyWaypoint(line);
        }
        489 => {
            PolySetVisibilityTangibility(line);
        }
        491 => {
            PolyTranslucency(line);
        }
        492 => {
            PolyFade(line);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_SetupSignExit(mut player: *mut player_t) {
    let mut thing: *mut mobj_t = 0 as *mut mobj_t;
    let mut node: *mut msecnode_t = (*(*(*(*player).mo).subsector).sector)
        .touching_thinglist;
    let mut think: *mut thinker_t = 0 as *mut thinker_t;
    let mut numfound: int32_t = 0 as libc::c_int;
    while !node.is_null() {
        thing = (*node).m_thing;
        if !((*thing).type_0 as libc::c_uint != MT_SIGN as libc::c_int as libc::c_uint) {
            if numfound == 0
                && !(!((*(*player).mo).target).is_null()
                    && (*(*(*player).mo).target).type_0 as libc::c_uint
                        == MT_SIGN as libc::c_int as libc::c_uint)
                && !(gametyperules & GTR_FRIENDLY as libc::c_int as uint32_t != 0
                    && (netgame != 0 || multiplayer != 0) && cv_exitmove.value != 0)
            {
                P_SetTarget2(&mut (*(*player).mo).target, thing);
            }
            if !((*thing).state
                != &mut *states.as_mut_ptr().offset((*(*thing).info).spawnstate as isize)
                    as *mut state_t)
            {
                P_SetTarget2(&mut (*thing).target, (*player).mo);
                P_SetObjectMomZ(
                    thing,
                    12 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    false_0 as libc::c_int,
                );
                P_SetMobjState(thing, S_SIGNSPIN1);
                if (*(*thing).info).seesound as u64 != 0 {
                    S_StartSound(
                        thing as *const libc::c_void,
                        (*(*thing).info).seesound,
                    );
                }
                numfound += 1;
                numfound;
            }
        }
        node = (*node).m_thinglist_next;
    }
    if numfound != 0 {
        return;
    }
    think = (*thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)).next;
    while think
        != &mut *thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)
            as *mut thinker_t
    {
        if !((*think).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut thinker_t) -> ()>,
                actionf_p1,
            >(
                Some(
                    P_RemoveThinkerDelayed as unsafe extern "C" fn(*mut thinker_t) -> (),
                ),
            ))
        {
            thing = think as *mut mobj_t;
            if !((*thing).type_0 as libc::c_uint
                != MT_SIGN as libc::c_int as libc::c_uint)
            {
                if numfound == 0
                    && !(!((*(*player).mo).target).is_null()
                        && (*(*(*player).mo).target).type_0 as libc::c_uint
                            == MT_SIGN as libc::c_int as libc::c_uint)
                    && !(gametyperules & GTR_FRIENDLY as libc::c_int as uint32_t != 0
                        && (netgame != 0 || multiplayer != 0) && cv_exitmove.value != 0)
                {
                    P_SetTarget2(&mut (*(*player).mo).target, thing);
                }
                if !((*thing).state
                    != &mut *states
                        .as_mut_ptr()
                        .offset((*(*thing).info).spawnstate as isize) as *mut state_t)
                {
                    P_SetTarget2(&mut (*thing).target, (*player).mo);
                    P_SetObjectMomZ(
                        thing,
                        12 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        false_0 as libc::c_int,
                    );
                    P_SetMobjState(thing, S_SIGNSPIN1);
                    if (*(*thing).info).seesound as u64 != 0 {
                        S_StartSound(
                            thing as *const libc::c_void,
                            (*(*thing).info).seesound,
                        );
                    }
                    numfound += 1;
                    numfound;
                }
            }
        }
        think = (*think).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_IsFlagAtBase(mut flag: mobjtype_t) -> boolean {
    let mut think: *mut thinker_t = 0 as *mut thinker_t;
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    let mut specialflag: sectorspecialflags_t = (if flag as libc::c_uint
        == MT_REDFLAG as libc::c_int as libc::c_uint
    {
        SSF_REDTEAMBASE as libc::c_int
    } else {
        SSF_BLUETEAMBASE as libc::c_int
    }) as sectorspecialflags_t;
    think = (*thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)).next;
    while think
        != &mut *thlist.as_mut_ptr().offset(THINK_MOBJ as libc::c_int as isize)
            as *mut thinker_t
    {
        if !((*think).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut thinker_t) -> ()>,
                actionf_p1,
            >(
                Some(
                    P_RemoveThinkerDelayed as unsafe extern "C" fn(*mut thinker_t) -> (),
                ),
            ))
        {
            mo = think as *mut mobj_t;
            if !((*mo).type_0 as libc::c_uint != flag as libc::c_uint) {
                if (*(*(*mo).subsector).sector).specialflags as libc::c_uint
                    & specialflag as libc::c_uint != 0
                {
                    return true_0 as libc::c_int
                } else if !((*(*(*mo).subsector).sector).ffloors).is_null() {
                    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
                    rover = (*(*(*mo).subsector).sector).ffloors;
                    while !rover.is_null() {
                        if !((*rover).fofflags as libc::c_uint
                            & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
                        {
                            if !((*(*(*rover).master).frontsector).specialflags
                                as libc::c_uint & specialflag as libc::c_uint == 0)
                            {
                                if (*mo).z
                                    <= P_MobjCeilingZ(
                                        mo,
                                        sectors.offset((*rover).secnum as isize),
                                        (*(*mo).subsector).sector,
                                        (*mo).x,
                                        (*mo).y,
                                        0 as *mut line_t,
                                        (sectors.offset((*rover).secnum as isize)
                                            == (*(*mo).subsector).sector) as libc::c_int,
                                        true_0 as libc::c_int,
                                    )
                                    && (*mo).z
                                        >= P_MobjFloorZ(
                                            mo,
                                            sectors.offset((*rover).secnum as isize),
                                            (*(*mo).subsector).sector,
                                            (*mo).x,
                                            (*mo).y,
                                            0 as *mut line_t,
                                            (sectors.offset((*rover).secnum as isize)
                                                != (*(*mo).subsector).sector) as libc::c_int,
                                            true_0 as libc::c_int,
                                        )
                                {
                                    return true_0 as libc::c_int;
                                }
                            }
                        }
                        rover = (*rover).next;
                    }
                }
            }
        }
        think = (*think).next;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn P_IsMobjTouchingPlane(
    mut mo: *mut mobj_t,
    mut sec: *mut sector_t,
    mut floorz: fixed_t,
    mut ceilingz: fixed_t,
) -> boolean {
    let mut floorallowed: boolean = ((*sec).flags as libc::c_uint
        & MSF_FLIPSPECIAL_FLOOR as libc::c_int as libc::c_uint != 0
        && ((*sec).flags as libc::c_uint
            & MSF_TRIGGERSPECIAL_HEADBUMP as libc::c_int as libc::c_uint != 0
            || (*mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int == 0)
        && (*mo).z == floorz) as libc::c_int;
    let mut ceilingallowed: boolean = ((*sec).flags as libc::c_uint
        & MSF_FLIPSPECIAL_CEILING as libc::c_int as libc::c_uint != 0
        && ((*sec).flags as libc::c_uint
            & MSF_TRIGGERSPECIAL_HEADBUMP as libc::c_int as libc::c_uint != 0
            || (*mo).eflags as libc::c_int & MFE_VERTICALFLIP as libc::c_int != 0)
        && (*mo).z + (*mo).height == ceilingz) as libc::c_int;
    return (floorallowed != 0 || ceilingallowed != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_IsMobjTouchingSectorPlane(
    mut mo: *mut mobj_t,
    mut sec: *mut sector_t,
) -> boolean {
    return P_IsMobjTouchingPlane(
        mo,
        sec,
        P_MobjFloorZ(
            mo,
            sec,
            sec,
            (*mo).x,
            (*mo).y,
            0 as *mut line_t,
            (sec != sec) as libc::c_int,
            true_0 as libc::c_int,
        ),
        P_MobjCeilingZ(
            mo,
            sec,
            sec,
            (*mo).x,
            (*mo).y,
            0 as *mut line_t,
            (sec == sec) as libc::c_int,
            true_0 as libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn P_IsMobjTouching3DFloor(
    mut mo: *mut mobj_t,
    mut ffloor: *mut ffloor_t,
    mut sec: *mut sector_t,
) -> boolean {
    let mut topheight: fixed_t = P_MobjCeilingZ(
        mo,
        sectors.offset((*ffloor).secnum as isize),
        sec,
        (*mo).x,
        (*mo).y,
        0 as *mut line_t,
        (sectors.offset((*ffloor).secnum as isize) == sec) as libc::c_int,
        true_0 as libc::c_int,
    );
    let mut bottomheight: fixed_t = P_MobjFloorZ(
        mo,
        sectors.offset((*ffloor).secnum as isize),
        sec,
        (*mo).x,
        (*mo).y,
        0 as *mut line_t,
        (sectors.offset((*ffloor).secnum as isize) != sec) as libc::c_int,
        true_0 as libc::c_int,
    );
    if (*ffloor).fofflags as libc::c_uint
        & FOF_BLOCKPLAYER as libc::c_int as libc::c_uint != 0
        && !((*mo).player).is_null()
        || (*ffloor).fofflags as libc::c_uint
            & FOF_BLOCKOTHERS as libc::c_int as libc::c_uint != 0
            && ((*mo).player).is_null()
    {
        return P_IsMobjTouchingPlane(
            mo,
            (*(*ffloor).master).frontsector,
            topheight,
            bottomheight,
        )
    } else {
        return ((*mo).z <= topheight && (*mo).z + (*mo).height >= bottomheight)
            as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_IsMobjTouchingPolyobj(
    mut mo: *mut mobj_t,
    mut po: *mut polyobj_t,
    mut polysec: *mut sector_t,
) -> boolean {
    if (*po).flags & POF_TESTHEIGHT as libc::c_int == 0 {
        return true_0 as libc::c_int;
    }
    if (*po).flags & POF_SOLID as libc::c_int != 0 {
        return P_IsMobjTouchingPlane(
            mo,
            polysec,
            (*polysec).ceilingheight,
            (*polysec).floorheight,
        )
    } else {
        return ((*mo).z <= (*polysec).ceilingheight
            && (*mo).z + (*mo).height >= (*polysec).floorheight) as libc::c_int
    };
}
unsafe extern "C" fn P_MobjTouching3DFloorSpecial(
    mut mo: *mut mobj_t,
    mut sector: *mut sector_t,
    mut section: int32_t,
    mut number: int32_t,
) -> *mut sector_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*sector).ffloors;
    while !rover.is_null() {
        if !((*(*(*rover).master).frontsector).special as libc::c_int
            >> (section - 1 as libc::c_int) * 4 as libc::c_int & 15 as libc::c_int
            != number)
        {
            if !((*rover).fofflags as libc::c_uint
                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
            {
                if !(P_IsMobjTouching3DFloor(mo, rover, sector) == 0) {
                    if sector == (*(*mo).subsector).sector
                        || (*(*(*rover).master).frontsector).flags as libc::c_uint
                            & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint
                            != 0
                    {
                        return (*(*rover).master).frontsector;
                    }
                }
            }
        }
        rover = (*rover).next;
    }
    return 0 as *mut sector_t;
}
unsafe extern "C" fn P_MobjTouching3DFloorSpecialFlag(
    mut mo: *mut mobj_t,
    mut sector: *mut sector_t,
    mut flag: sectorspecialflags_t,
) -> *mut sector_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*sector).ffloors;
    while !rover.is_null() {
        if !((*(*(*rover).master).frontsector).specialflags as libc::c_uint
            & flag as libc::c_uint == 0)
        {
            if !((*rover).fofflags as libc::c_uint
                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
            {
                if !(P_IsMobjTouching3DFloor(mo, rover, sector) == 0) {
                    if sector == (*(*mo).subsector).sector
                        || (*(*(*rover).master).frontsector).flags as libc::c_uint
                            & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint
                            != 0
                    {
                        return (*(*rover).master).frontsector;
                    }
                }
            }
        }
        rover = (*rover).next;
    }
    return 0 as *mut sector_t;
}
unsafe extern "C" fn P_MobjTouchingPolyobjSpecial(
    mut mo: *mut mobj_t,
    mut section: int32_t,
    mut number: int32_t,
) -> *mut sector_t {
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut polysec: *mut sector_t = 0 as *mut sector_t;
    let mut touching: boolean = false_0 as libc::c_int;
    let mut inside: boolean = false_0 as libc::c_int;
    po = (*(*mo).subsector).polyList;
    while !po.is_null() {
        if !((*po).flags & POF_NOSPECIALS as libc::c_int != 0) {
            polysec = (**((*po).lines).offset(0 as libc::c_int as isize)).backsector;
            if !((*polysec).special as libc::c_int
                >> (section - 1 as libc::c_int) * 4 as libc::c_int & 15 as libc::c_int
                != number)
            {
                touching = ((*polysec).flags as libc::c_uint
                    & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint != 0
                    && P_MobjTouchingPolyobj(po, mo) != 0) as libc::c_int;
                inside = P_MobjInsidePolyobj(po, mo);
                if inside != 0 || touching != 0 {
                    if !(P_IsMobjTouchingPolyobj(mo, po, polysec) == 0) {
                        return polysec;
                    }
                }
            }
        }
        po = (*po).link.next as *mut polyobj_t;
    }
    return 0 as *mut sector_t;
}
unsafe extern "C" fn P_MobjTouchingPolyobjSpecialFlag(
    mut mo: *mut mobj_t,
    mut flag: sectorspecialflags_t,
) -> *mut sector_t {
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut polysec: *mut sector_t = 0 as *mut sector_t;
    let mut touching: boolean = false_0 as libc::c_int;
    let mut inside: boolean = false_0 as libc::c_int;
    po = (*(*mo).subsector).polyList;
    while !po.is_null() {
        if !((*po).flags & POF_NOSPECIALS as libc::c_int != 0) {
            polysec = (**((*po).lines).offset(0 as libc::c_int as isize)).backsector;
            if !((*polysec).specialflags as libc::c_uint & flag as libc::c_uint == 0) {
                touching = ((*polysec).flags as libc::c_uint
                    & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint != 0
                    && P_MobjTouchingPolyobj(po, mo) != 0) as libc::c_int;
                inside = P_MobjInsidePolyobj(po, mo);
                if inside != 0 || touching != 0 {
                    if !(P_IsMobjTouchingPolyobj(mo, po, polysec) == 0) {
                        return polysec;
                    }
                }
            }
        }
        po = (*po).link.next as *mut polyobj_t;
    }
    return 0 as *mut sector_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_MobjTouchingSectorSpecial(
    mut mo: *mut mobj_t,
    mut section: int32_t,
    mut number: int32_t,
) -> *mut sector_t {
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut result: *mut sector_t = 0 as *mut sector_t;
    result = P_MobjTouching3DFloorSpecial(
        mo,
        (*(*mo).subsector).sector,
        section,
        number,
    );
    if !result.is_null() {
        return result;
    }
    result = P_MobjTouchingPolyobjSpecial(mo, section, number);
    if !result.is_null() {
        return result;
    }
    if (*(*(*mo).subsector).sector).special as libc::c_int
        >> (section - 1 as libc::c_int) * 4 as libc::c_int & 15 as libc::c_int == number
    {
        return (*(*mo).subsector).sector;
    }
    node = (*mo).touching_sectorlist;
    while !node.is_null() {
        if !((*node).m_sector == (*(*mo).subsector).sector) {
            result = P_MobjTouching3DFloorSpecial(mo, (*node).m_sector, section, number);
            if !result.is_null() {
                return result;
            }
            if !((*(*node).m_sector).flags as libc::c_uint
                & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint == 0)
            {
                if (*(*node).m_sector).special as libc::c_int
                    >> (section - 1 as libc::c_int) * 4 as libc::c_int
                    & 15 as libc::c_int == number
                {
                    return (*node).m_sector;
                }
            }
        }
        node = (*node).m_sectorlist_next;
    }
    return 0 as *mut sector_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_ThingOnSpecial3DFloor(mut mo: *mut mobj_t) -> *mut sector_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*(*(*mo).subsector).sector).ffloors;
    while !rover.is_null() {
        if !((*(*(*rover).master).frontsector).special == 0) {
            if !((*rover).fofflags as libc::c_uint
                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
            {
                if !(P_IsMobjTouching3DFloor(mo, rover, (*(*mo).subsector).sector) == 0)
                {
                    return (*(*rover).master).frontsector;
                }
            }
        }
        rover = (*rover).next;
    }
    return 0 as *mut sector_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_MobjTouchingSectorSpecialFlag(
    mut mo: *mut mobj_t,
    mut flag: sectorspecialflags_t,
) -> *mut sector_t {
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut result: *mut sector_t = 0 as *mut sector_t;
    result = P_MobjTouching3DFloorSpecialFlag(mo, (*(*mo).subsector).sector, flag);
    if !result.is_null() {
        return result;
    }
    result = P_MobjTouchingPolyobjSpecialFlag(mo, flag);
    if !result.is_null() {
        return result;
    }
    if (*(*(*mo).subsector).sector).specialflags as libc::c_uint & flag as libc::c_uint
        != 0
    {
        return (*(*mo).subsector).sector;
    }
    node = (*mo).touching_sectorlist;
    while !node.is_null() {
        if !((*node).m_sector == (*(*mo).subsector).sector) {
            result = P_MobjTouching3DFloorSpecialFlag(mo, (*node).m_sector, flag);
            if !result.is_null() {
                return result;
            }
            if !((*(*node).m_sector).flags as libc::c_uint
                & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint == 0)
            {
                if (*(*node).m_sector).specialflags as libc::c_uint
                    & flag as libc::c_uint != 0
                {
                    return (*node).m_sector;
                }
            }
        }
        node = (*node).m_sectorlist_next;
    }
    return 0 as *mut sector_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerTouchingSectorSpecial(
    mut player: *mut player_t,
    mut section: int32_t,
    mut number: int32_t,
) -> *mut sector_t {
    if ((*player).mo).is_null() {
        return 0 as *mut sector_t;
    }
    return P_MobjTouchingSectorSpecial((*player).mo, section, number);
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerTouchingSectorSpecialFlag(
    mut player: *mut player_t,
    mut flag: sectorspecialflags_t,
) -> *mut sector_t {
    if ((*player).mo).is_null() {
        return 0 as *mut sector_t;
    }
    return P_MobjTouchingSectorSpecialFlag((*player).mo, flag);
}
unsafe extern "C" fn P_CheckPlayer3DFloorTrigger(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut sourceline: *mut line_t,
) -> *mut sector_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*sector).ffloors;
    while !rover.is_null() {
        if !((*(*(*rover).master).frontsector).triggertag == 0) {
            if !((*(*(*rover).master).frontsector).triggerer as libc::c_int
                == TO_MOBJ as libc::c_int)
            {
                if !((*rover).fofflags as libc::c_uint
                    & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
                {
                    if !(Tag_Find(
                        &mut (*sourceline).tags,
                        (*(*(*rover).master).frontsector).triggertag,
                    ) == 0)
                    {
                        if !(P_IsMobjTouching3DFloor((*player).mo, rover, sector) == 0) {
                            if sector == (*(*(*player).mo).subsector).sector
                                || (*(*(*rover).master).frontsector).flags as libc::c_uint
                                    & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint
                                    != 0
                            {
                                return (*(*rover).master).frontsector;
                            }
                        }
                    }
                }
            }
        }
        rover = (*rover).next;
    }
    return 0 as *mut sector_t;
}
unsafe extern "C" fn P_CheckPlayerPolyobjTrigger(
    mut player: *mut player_t,
    mut sourceline: *mut line_t,
) -> *mut sector_t {
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut polysec: *mut sector_t = 0 as *mut sector_t;
    let mut touching: boolean = false_0 as libc::c_int;
    let mut inside: boolean = false_0 as libc::c_int;
    po = (*(*(*player).mo).subsector).polyList;
    while !po.is_null() {
        if !((*po).flags & POF_NOSPECIALS as libc::c_int != 0) {
            polysec = (**((*po).lines).offset(0 as libc::c_int as isize)).backsector;
            if !((*polysec).triggertag == 0) {
                if !((*polysec).triggerer as libc::c_int == TO_MOBJ as libc::c_int) {
                    if !(Tag_Find(&mut (*sourceline).tags, (*polysec).triggertag) == 0) {
                        touching = ((*polysec).flags as libc::c_uint
                            & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint
                            != 0 && P_MobjTouchingPolyobj(po, (*player).mo) != 0)
                            as libc::c_int;
                        inside = P_MobjInsidePolyobj(po, (*player).mo);
                        if inside != 0 || touching != 0 {
                            if !(P_IsMobjTouchingPolyobj((*player).mo, po, polysec) == 0)
                            {
                                return polysec;
                            }
                        }
                    }
                }
            }
        }
        po = (*po).link.next as *mut polyobj_t;
    }
    return 0 as *mut sector_t;
}
unsafe extern "C" fn P_CheckPlayerSectorTrigger(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut sourceline: *mut line_t,
) -> boolean {
    if (*sector).triggertag == 0 {
        return false_0 as libc::c_int;
    }
    if (*sector).triggerer as libc::c_int == TO_MOBJ as libc::c_int {
        return false_0 as libc::c_int;
    }
    if Tag_Find(&mut (*sourceline).tags, (*sector).triggertag) == 0 {
        return false_0 as libc::c_int;
    }
    if (*sector).flags as libc::c_uint
        & MSF_TRIGGERLINE_PLANE as libc::c_int as libc::c_uint == 0
    {
        return true_0 as libc::c_int;
    }
    return P_IsMobjTouchingSectorPlane((*player).mo, sector);
}
#[no_mangle]
pub unsafe extern "C" fn P_FindPlayerTrigger(
    mut player: *mut player_t,
    mut sourceline: *mut line_t,
) -> *mut sector_t {
    let mut originalsector: *mut sector_t = 0 as *mut sector_t;
    let mut loopsector: *mut sector_t = 0 as *mut sector_t;
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut caller: *mut sector_t = 0 as *mut sector_t;
    if ((*player).mo).is_null() {
        return 0 as *mut sector_t;
    }
    originalsector = (*(*(*player).mo).subsector).sector;
    caller = P_CheckPlayer3DFloorTrigger(player, originalsector, sourceline);
    if !caller.is_null() {
        return caller;
    }
    caller = P_CheckPlayerPolyobjTrigger(player, sourceline);
    if !caller.is_null() {
        return caller;
    }
    if P_CheckPlayerSectorTrigger(player, originalsector, sourceline) != 0 {
        return originalsector;
    }
    node = (*(*player).mo).touching_sectorlist;
    while !node.is_null() {
        loopsector = (*node).m_sector;
        if !(loopsector == originalsector) {
            caller = P_CheckPlayer3DFloorTrigger(player, loopsector, sourceline);
            if !caller.is_null() {
                return caller;
            }
            if !((*loopsector).flags as libc::c_uint
                & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint == 0)
            {
                if P_CheckPlayerSectorTrigger(player, loopsector, sourceline) != 0 {
                    return loopsector;
                }
            }
        }
        node = (*node).m_sectorlist_next;
    }
    return 0 as *mut sector_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_IsPlayerValid(mut playernum: size_t) -> boolean {
    if playeringame[playernum as usize] == 0 {
        return false_0 as libc::c_int;
    }
    if (players[playernum as usize].mo).is_null() {
        return false_0 as libc::c_int;
    }
    if (*players[playernum as usize].mo).health <= 0 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if players[playernum as usize].spectator != 0 {
        return false_0 as libc::c_int;
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn P_CanPlayerTrigger(mut playernum: size_t) -> boolean {
    return (P_IsPlayerValid(playernum) != 0 && players[playernum as usize].bot == 0)
        as libc::c_int;
}
unsafe extern "C" fn P_DoAllPlayersTrigger(mut triggertag: mtag_t) -> boolean {
    let mut i: int32_t = 0;
    let mut dummyline: line_t = line_s {
        v1: 0 as *mut vertex_t,
        v2: 0 as *mut vertex_t,
        dx: 0,
        dy: 0,
        angle: 0,
        flags: 0,
        special: 0,
        tags: taglist_t {
            tags: 0 as *mut mtag_t,
            count: 0,
        },
        args: [0; 10],
        stringargs: [0 as *mut libc::c_char; 2],
        sidenum: [0; 2],
        alpha: 0,
        blendmode: 0,
        executordelay: 0,
        bbox: [0; 4],
        slopetype: ST_HORIZONTAL,
        frontsector: 0 as *mut sector_t,
        backsector: 0 as *mut sector_t,
        validcount: 0,
        polyobj: 0 as *mut polyobj_t,
        callcount: 0,
    };
    dummyline.tags.count = 1 as libc::c_int as uint16_t;
    dummyline.tags.tags = &mut triggertag;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(P_CanPlayerTrigger(i as size_t) == 0) {
            if (P_FindPlayerTrigger(
                &mut *players.as_mut_ptr().offset(i as isize),
                &mut dummyline,
            ))
                .is_null()
            {
                return false_0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn P_ProcessEggCapsule(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
) {
    let mut th: *mut thinker_t = 0 as *mut thinker_t;
    let mut mo2: *mut mobj_t = 0 as *mut mobj_t;
    let mut i: int32_t = 0;
    if (*player).bot as libc::c_int != 0 || !((*sector).ceilingdata).is_null()
        || !((*sector).floordata).is_null()
    {
        return;
    }
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
                != MT_EGGTRAP as libc::c_int as libc::c_uint)
            {
                P_KillMobj(
                    mo2,
                    0 as *mut mobj_t,
                    (*player).mo,
                    0 as libc::c_int as uint8_t,
                );
            }
        }
        th = (*th).next;
    }
    (*sector).special = 0 as libc::c_int as int16_t;
    EV_DoElevator(LE_CAPSULE0 as libc::c_int as mtag_t, 0 as *mut line_t, elevateDown);
    EV_DoFloor(
        LE_CAPSULE1 as libc::c_int as mtag_t,
        0 as *mut line_t,
        raiseFloorToNearestFast,
    );
    EV_DoCeiling(
        LE_CAPSULE2 as libc::c_int as mtag_t,
        0 as *mut line_t,
        lowerToLowestFast,
    );
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(playeringame[i as usize] == 0) {
            P_DoPlayerExit(&mut *players.as_mut_ptr().offset(i as isize));
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn P_ProcessSpeedPad(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut roversector: *mut sector_t,
    mut sectag: mtag_t,
) {
    let mut lineindex: int32_t = -(1 as libc::c_int);
    let mut lineangle: angle_t = 0;
    let mut linespeed: fixed_t = 0;
    let mut sfxnum: fixed_t = 0;
    let mut i: size_t = 0;
    if (*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int
        != 0 as libc::c_int
        && ((*player).powers[pw_flashing as libc::c_int as usize] as libc::c_int)
            < 35 as libc::c_int / 2 as libc::c_int
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < (*sector).linecount {
        let mut li: *mut line_t = *((*sector).lines).offset(i as isize);
        if !((*li).frontsector != sector) {
            if !((*li).special as libc::c_int != 4 as libc::c_int) {
                if !(Tag_Find(&mut (*li).tags, 0 as libc::c_int as mtag_t) == 0) {
                    lineindex = li.offset_from(lines) as libc::c_long as int32_t;
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if lineindex == -(1 as libc::c_int) {
        lineindex = Tag_FindLineSpecial(4 as libc::c_int as int16_t, sectag);
    }
    if lineindex == -(1 as libc::c_int) {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Speed pad missing line special #4.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    lineangle = R_PointToAngle2(
        (*(*lines.offset(lineindex as isize)).v1).x,
        (*(*lines.offset(lineindex as isize)).v1).y,
        (*(*lines.offset(lineindex as isize)).v2).x,
        (*(*lines.offset(lineindex as isize)).v2).y,
    );
    linespeed = (*lines.offset(lineindex as isize)).args[0 as libc::c_int as usize]
        << 16 as libc::c_int;
    if linespeed == 0 as libc::c_int {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Speed pad (tag %d) at zero speed.\n\0" as *const u8
                as *const libc::c_char,
            sectag as libc::c_int,
        );
        return;
    }
    (*player).drawangle = lineangle;
    (*(*player).mo).angle = (*player).drawangle;
    if demoplayback == 0
        || (if (*player).pflags as libc::c_uint
            & PF_ANALOGMODE as libc::c_int as libc::c_uint != 0
        {
            CS_LMAOGALOG as libc::c_int
        } else {
            0 as libc::c_int
        })
            | (if (*player).pflags as libc::c_uint
                & PF_DIRECTIONCHAR as libc::c_int as libc::c_uint != 0
            {
                CS_STANDARD as libc::c_int
            } else {
                0 as libc::c_int
            }) == CS_LMAOGALOG as libc::c_int
    {
        P_SetPlayerAngle(player, (*(*player).mo).angle);
    }
    if (*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]
        & TMSP_NOTELEPORT as libc::c_int == 0
    {
        P_UnsetThingPosition((*player).mo);
        if !roversector.is_null() {
            (*(*player).mo).x = (*roversector).soundorg.x;
            (*(*player).mo).y = (*roversector).soundorg.y;
        } else {
            (*(*player).mo).x = (*sector).soundorg.x;
            (*(*player).mo).y = (*sector).soundorg.y;
        }
        P_SetThingPosition((*player).mo);
    }
    P_InstaThrust((*player).mo, (*(*player).mo).angle, linespeed);
    if (*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]
        & TMSP_FORCESPIN as libc::c_int != 0
    {
        if (*player).pflags as libc::c_uint & PF_SPINNING as libc::c_int as libc::c_uint
            == 0
        {
            (*player)
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                (*player).pflags as libc::c_uint
                    | PF_SPINNING as libc::c_int as libc::c_uint,
            );
        }
        P_SetPlayerMobjState((*player).mo, S_PLAY_ROLL);
    }
    (*player)
        .powers[pw_flashing as libc::c_int
        as usize] = (35 as libc::c_int / 3 as libc::c_int) as uint16_t;
    sfxnum = if !((*lines.offset(lineindex as isize))
        .stringargs[0 as libc::c_int as usize])
        .is_null()
    {
        get_number(
            (*lines.offset(lineindex as isize)).stringargs[0 as libc::c_int as usize],
        )
    } else {
        sfx_spdpad as libc::c_int
    };
    if sfxnum == 0 {
        sfxnum = sfx_spdpad as libc::c_int;
    }
    S_StartSound((*player).mo as *const libc::c_void, sfxnum as sfxenum_t);
}
unsafe extern "C" fn P_ProcessSpecialStagePit(mut player: *mut player_t) {
    if gametyperules & GTR_ALLOWEXIT as libc::c_int as uint32_t == 0 {
        return;
    }
    if (*player).bot != 0 {
        return;
    }
    if G_IsSpecialStage(gamemap as int32_t) == 0 {
        return;
    }
    if maptol & TOL_NIGHTS as libc::c_int as uint32_t != 0 {
        return;
    }
    if (*player).nightstime <= 6 as libc::c_int as tic_t {
        return;
    }
    (*player).nightstime = 6 as libc::c_int as tic_t;
}
unsafe extern "C" fn P_ProcessExitSector(mut player: *mut player_t, mut sectag: mtag_t) {
    let mut lineindex: int32_t = 0;
    if gametyperules & GTR_ALLOWEXIT as libc::c_int as uint32_t == 0 {
        return;
    }
    if (*player).bot != 0 {
        return;
    }
    if G_IsSpecialStage(gamemap as int32_t) != 0
        && maptol & TOL_NIGHTS as libc::c_int as uint32_t == 0
    {
        return;
    }
    P_DoPlayerFinish(player);
    P_SetupSignExit(player);
    if G_CoopGametype() == 0 {
        return;
    }
    lineindex = Tag_FindLineSpecial(2 as libc::c_int as int16_t, sectag);
    if lineindex == -(1 as libc::c_int) {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Exit sector missing line special #2.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]
        & TMEF_EMERALDCHECK as libc::c_int != 0
        && emeralds as libc::c_int
            & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
            == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int
    {
        nextmapoverride = (if udmf != 0 {
            (*lines.offset(lineindex as isize)).args[2 as libc::c_int as usize]
        } else {
            (*(*lines.offset(lineindex as isize)).frontsector).ceilingheight
                >> 16 as libc::c_int
        }) as int16_t;
    } else {
        nextmapoverride = (if udmf != 0 {
            (*lines.offset(lineindex as isize)).args[0 as libc::c_int as usize]
        } else {
            (*(*lines.offset(lineindex as isize)).frontsector).floorheight
                >> 16 as libc::c_int
        }) as int16_t;
    }
    if (*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]
        & TMEF_SKIPTALLY as libc::c_int != 0
    {
        skipstats = 1 as libc::c_int as uint8_t;
    }
}
unsafe extern "C" fn P_ProcessTeamBase(mut player: *mut player_t, mut redteam: boolean) {
    let mut mo: *mut mobj_t = 0 as *mut mobj_t;
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t == 0 {
        return;
    }
    if P_IsObjectOnGround((*player).mo) == 0 {
        return;
    }
    if (*player).ctfteam
        != (if redteam != 0 { 1 as libc::c_int } else { 2 as libc::c_int })
    {
        return;
    }
    if (*player).gotflag as libc::c_int
        & (if redteam != 0 { 2 as libc::c_int } else { 1 as libc::c_int }) == 0
    {
        return;
    }
    if P_IsFlagAtBase(
        (if redteam != 0 {
            MT_REDFLAG as libc::c_int
        } else {
            MT_BLUEFLAG as libc::c_int
        }) as mobjtype_t,
    ) == 0
    {
        return;
    }
    HU_SetCEchoFlags(0x10000000 as libc::c_int | 0x800000 as libc::c_int);
    HU_SetCEchoDuration(5 as libc::c_int);
    HU_DoCEcho(
        va(
            b"%s%s\x80\\CAPTURED THE %s%s FLAG\x80.\\\\\\\\\0" as *const u8
                as *const libc::c_char,
            if redteam != 0 {
                b"\x85\0" as *const u8 as *const libc::c_char
            } else {
                b"\x84\0" as *const u8 as *const libc::c_char
            },
            (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                as usize])
                .as_mut_ptr(),
            if redteam != 0 {
                b"\x84\0" as *const u8 as *const libc::c_char
            } else {
                b"\x85\0" as *const u8 as *const libc::c_char
            },
            if redteam != 0 {
                b"BLUE\0" as *const u8 as *const libc::c_char
            } else {
                b"RED\0" as *const u8 as *const libc::c_char
            },
        ),
    );
    if splitscreen != 0
        || players[consoleplayer as usize].ctfteam
            == (if redteam != 0 { 1 as libc::c_int } else { 2 as libc::c_int })
    {
        S_StartSound(0 as *const libc::c_void, sfx_flgcap);
    } else if players[consoleplayer as usize].ctfteam
        == (if redteam != 0 { 2 as libc::c_int } else { 1 as libc::c_int })
    {
        S_StartSound(0 as *const libc::c_void, sfx_lose);
    }
    mo = P_SpawnMobj(
        (*(*player).mo).x,
        (*(*player).mo).y,
        (*(*player).mo).z,
        (if redteam != 0 {
            MT_BLUEFLAG as libc::c_int
        } else {
            MT_REDFLAG as libc::c_int
        }) as mobjtype_t,
    );
    (*player)
        .gotflag = ((*player).gotflag as libc::c_int
        & !if redteam != 0 { 2 as libc::c_int } else { 1 as libc::c_int }) as uint16_t;
    (*mo).flags &= !(MF_SPECIAL as libc::c_int) as uint32_t;
    (*mo).fuse = 35 as libc::c_int;
    (*mo).spawnpoint = if redteam != 0 { bflagpoint } else { rflagpoint };
    (*mo).flags2 |= MF2_JUSTATTACKED as libc::c_int as uint32_t;
    if redteam != 0 {
        redscore = redscore.wrapping_add(1 as libc::c_int as uint32_t);
    } else {
        bluescore = bluescore.wrapping_add(1 as libc::c_int as uint32_t);
    }
    P_AddPlayerScore(player, 250 as libc::c_int as uint32_t);
}
unsafe extern "C" fn P_ProcessZoomTube(
    mut player: *mut player_t,
    mut sectag: mtag_t,
    mut end: boolean,
) {
    let mut sequence: int32_t = 0;
    let mut speed: fixed_t = 0;
    let mut lineindex: int32_t = 0;
    let mut waypoint: *mut mobj_t = 0 as *mut mobj_t;
    let mut an: angle_t = 0;
    if !((*(*player).mo).tracer).is_null()
        && (*(*(*player).mo).tracer).type_0 as libc::c_uint
            == MT_TUBEWAYPOINT as libc::c_int as libc::c_uint
        && (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_ZOOMTUBE as libc::c_int
    {
        return;
    }
    if (*player).powers[pw_ignorelatch as libc::c_int as usize] as libc::c_int
        & (1 as libc::c_int) << 15 as libc::c_int != 0
    {
        return;
    }
    lineindex = Tag_FindLineSpecial(3 as libc::c_int as int16_t, sectag);
    if lineindex == -(1 as libc::c_int) {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Zoom tube missing line special #3.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    speed = abs((*lines.offset(lineindex as isize)).args[0 as libc::c_int as usize])
        << 16 as libc::c_int - 3 as libc::c_int;
    if end != 0 {
        speed *= -(1 as libc::c_int);
    }
    sequence = abs((*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]);
    if speed == 0 as libc::c_int {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Waypoint sequence %d at zero speed.\n\0" as *const u8
                as *const libc::c_char,
            sequence,
        );
        return;
    }
    waypoint = if end != 0 {
        P_GetLastWaypoint(sequence as uint8_t)
    } else {
        P_GetFirstWaypoint(sequence as uint8_t)
    };
    if waypoint.is_null() {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: %s WAYPOINT IN SEQUENCE %d NOT FOUND.\n\0" as *const u8
                as *const libc::c_char,
            if end != 0 {
                b"LAST\0" as *const u8 as *const libc::c_char
            } else {
                b"FIRST\0" as *const u8 as *const libc::c_char
            },
            sequence,
        );
        return;
    }
    CONS_Debug(
        0x80 as libc::c_int,
        b"Waypoint %d found in sequence %d - speed = %d\n\0" as *const u8
            as *const libc::c_char,
        (*waypoint).health,
        sequence,
        speed,
    );
    an = (R_PointToAngle2(
        (*(*player).mo).x,
        (*(*player).mo).y,
        (*waypoint).x,
        (*waypoint).y,
    ))
        .wrapping_sub((*(*player).mo).angle);
    if an > 0x40000000 as libc::c_int as angle_t && an < 0xc0000000 as libc::c_uint
        && (*lines.offset(lineindex as isize)).args[2 as libc::c_int as usize] == 0
    {
        return;
    }
    P_SetTarget2(&mut (*(*player).mo).tracer, waypoint);
    (*player)
        .powers[pw_carry as libc::c_int
        as usize] = CR_ZOOMTUBE as libc::c_int as uint16_t;
    (*player).speed = speed;
    (*player)
        .pflags = ::core::mem::transmute::<
        libc::c_uint,
        pflags_t,
    >((*player).pflags as libc::c_uint | PF_SPINNING as libc::c_int as libc::c_uint);
    (*player)
        .pflags = ::core::mem::transmute::<
        libc::c_uint,
        pflags_t,
    >(
        (*player).pflags as libc::c_uint
            & !(PF_JUMPED as libc::c_int | PF_NOJUMPDAMAGE as libc::c_int
                | PF_GLIDING as libc::c_int | PF_BOUNCING as libc::c_int
                | PF_SLIDING as libc::c_int | PF_CANCARRY as libc::c_int) as libc::c_uint,
    );
    (*player).climbing = 0 as libc::c_int as uint8_t;
    if ((*(*player).mo).state).offset_from(states.as_mut_ptr()) as libc::c_long
        != S_PLAY_ROLL as libc::c_int as libc::c_long
    {
        P_SetPlayerMobjState((*player).mo, S_PLAY_ROLL);
        S_StartSound((*player).mo as *const libc::c_void, sfx_spin);
    }
}
unsafe extern "C" fn P_ProcessFinishLine(mut player: *mut player_t) {
    if gametyperules & (GTR_RACE as libc::c_int | GTR_LIVES as libc::c_int) as uint32_t
        != GTR_RACE as libc::c_int as uint32_t
    {
        return;
    }
    if (*player).exiting != 0 {
        return;
    }
    if (*player).starpostnum == numstarposts {
        (*player).laps = ((*player).laps).wrapping_add(1);
        (*player).laps;
        if (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_NIGHTSMODE as libc::c_int
        {
            (*player).drillmeter += 48 as libc::c_int * 20 as libc::c_int;
        }
        if (*player).laps as libc::c_int >= cv_numlaps.value as uint8_t as libc::c_int {
            CONS_Printf(
                b"%s has finished the race.\n\0" as *const u8 as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
            );
        } else if (*player).laps as libc::c_int
            == cv_numlaps.value as uint8_t as libc::c_int - 1 as libc::c_int
        {
            CONS_Printf(
                b"%s started the \x85final lap\x80!\n\0" as *const u8
                    as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
            );
        } else {
            CONS_Printf(
                b"%s started lap %u\n\0" as *const u8 as *const libc::c_char,
                (player_names[player.offset_from(players.as_mut_ptr()) as libc::c_long
                    as usize])
                    .as_mut_ptr(),
                ((*player).laps as uint32_t).wrapping_add(1 as libc::c_int as uint32_t),
            );
        }
        (*player).starpostnum = 0 as libc::c_int;
        (*player).starposttime = (*player).starpostnum as tic_t;
        (*player).starpostangle = (*player).starposttime;
        (*player).starpostscale = (*player).starpostangle as fixed_t;
        (*player).starpostz = 0 as libc::c_int as int16_t;
        (*player).starposty = (*player).starpostz;
        (*player).starpostx = (*player).starposty;
        P_ResetStarposts();
        S_StartSound((*player).mo as *const libc::c_void, sfx_strpst);
    } else if (*player).starpostnum != 0 {
        if (*player).tossdelay == 0 {
            S_StartSound((*player).mo as *const libc::c_void, sfx_lose);
        }
        (*player).tossdelay = 3 as libc::c_int;
    }
    if (*player).laps as libc::c_uint >= cv_numlaps.value as libc::c_uint {
        if P_IsLocalPlayer(player) != 0 {
            HU_SetCEchoFlags(0 as libc::c_int);
            HU_SetCEchoDuration(5 as libc::c_int);
            HU_DoCEcho(b"FINISHED!\0" as *const u8 as *const libc::c_char);
        }
        P_DoPlayerExit(player);
    }
}
unsafe extern "C" fn P_ProcessRopeHang(mut player: *mut player_t, mut sectag: mtag_t) {
    let mut sequence: int32_t = 0;
    let mut speed: fixed_t = 0;
    let mut lineindex: int32_t = 0;
    let mut waypointmid: *mut mobj_t = 0 as *mut mobj_t;
    let mut waypointhigh: *mut mobj_t = 0 as *mut mobj_t;
    let mut waypointlow: *mut mobj_t = 0 as *mut mobj_t;
    let mut closest: *mut mobj_t = 0 as *mut mobj_t;
    let mut p: vector3_t = vector3_t { x: 0, y: 0, z: 0 };
    let mut line: [vector3_t; 2] = [vector3_t { x: 0, y: 0, z: 0 }; 2];
    let mut resulthigh: vector3_t = vector3_t { x: 0, y: 0, z: 0 };
    let mut resultlow: vector3_t = vector3_t { x: 0, y: 0, z: 0 };
    if !((*(*player).mo).tracer).is_null()
        && (*(*(*player).mo).tracer).type_0 as libc::c_uint
            == MT_TUBEWAYPOINT as libc::c_int as libc::c_uint
        && (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
            == CR_ROPEHANG as libc::c_int
    {
        return;
    }
    if (*player).powers[pw_ignorelatch as libc::c_int as usize] as libc::c_int
        & (1 as libc::c_int) << 15 as libc::c_int != 0
    {
        return;
    }
    if (*(*player).mo).momz > 0 as libc::c_int {
        return;
    }
    if (*player).cmd.buttons as libc::c_int & BT_SPIN as libc::c_int != 0 {
        return;
    }
    if (*player).pflags as libc::c_uint & PF_SLIDING as libc::c_int as libc::c_uint == 0
        && (*(*player).mo).state
            == &mut *states
                .as_mut_ptr()
                .offset((*(*(*player).mo).info).painstate as isize) as *mut state_t
    {
        return;
    }
    if (*player).exiting != 0 {
        return;
    }
    memset(
        &mut resultlow as *mut vector3_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<vector3_t>() as libc::c_ulong,
    );
    memset(
        &mut resulthigh as *mut vector3_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<vector3_t>() as libc::c_ulong,
    );
    lineindex = Tag_FindLineSpecial(11 as libc::c_int as int16_t, sectag);
    if lineindex == -(1 as libc::c_int) {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: Rope hang missing line special #11.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    speed = abs((*lines.offset(lineindex as isize)).args[0 as libc::c_int as usize])
        << 16 as libc::c_int - 3 as libc::c_int;
    sequence = abs((*lines.offset(lineindex as isize)).args[1 as libc::c_int as usize]);
    waypointmid = P_GetClosestWaypoint(sequence as uint8_t, (*player).mo);
    if waypointmid.is_null() {
        CONS_Debug(
            0x80 as libc::c_int,
            b"ERROR: WAYPOINT(S) IN SEQUENCE %d NOT FOUND.\n\0" as *const u8
                as *const libc::c_char,
            sequence,
        );
        return;
    }
    waypointlow = P_GetPreviousWaypoint(waypointmid, true_0 as libc::c_int);
    waypointhigh = P_GetNextWaypoint(waypointmid, true_0 as libc::c_int);
    CONS_Debug(
        0x80 as libc::c_int,
        b"WaypointMid: %d; WaypointLow: %d; WaypointHigh: %d\n\0" as *const u8
            as *const libc::c_char,
        (*waypointmid).health,
        if !waypointlow.is_null() { (*waypointlow).health } else { -(1 as libc::c_int) },
        if !waypointhigh.is_null() {
            (*waypointhigh).health
        } else {
            -(1 as libc::c_int)
        },
    );
    p.x = (*(*player).mo).x;
    p.y = (*(*player).mo).y;
    p.z = (*(*player).mo).z;
    if !waypointlow.is_null() {
        line[0 as libc::c_int as usize].x = (*waypointmid).x;
        line[0 as libc::c_int as usize].y = (*waypointmid).y;
        line[0 as libc::c_int as usize].z = (*waypointmid).z;
        line[1 as libc::c_int as usize].x = (*waypointlow).x;
        line[1 as libc::c_int as usize].y = (*waypointlow).y;
        line[1 as libc::c_int as usize].z = (*waypointlow).z;
        P_ClosestPointOnLine3D(&mut p, line.as_mut_ptr(), &mut resultlow);
    }
    if !waypointhigh.is_null() {
        line[0 as libc::c_int as usize].x = (*waypointmid).x;
        line[0 as libc::c_int as usize].y = (*waypointmid).y;
        line[0 as libc::c_int as usize].z = (*waypointmid).z;
        line[1 as libc::c_int as usize].x = (*waypointhigh).x;
        line[1 as libc::c_int as usize].y = (*waypointhigh).y;
        line[1 as libc::c_int as usize].z = (*waypointhigh).z;
        P_ClosestPointOnLine3D(&mut p, line.as_mut_ptr(), &mut resulthigh);
    }
    P_UnsetThingPosition((*player).mo);
    P_ResetPlayer(player);
    (*(*player).mo).momz = 0 as libc::c_int;
    (*(*player).mo).momy = (*(*player).mo).momz;
    (*(*player).mo).momx = (*(*player).mo).momy;
    if (*lines.offset(lineindex as isize)).args[2 as libc::c_int as usize] != 0 {
        let mut highest: *mut mobj_t = P_GetLastWaypoint(sequence as uint8_t);
        (*highest).flags |= MF_SLIDEME as libc::c_int as uint32_t;
    }
    if (*lines.offset(lineindex as isize)).args[2 as libc::c_int as usize] != 0
        && (*waypointmid).health == 0 as libc::c_int
    {
        closest = waypointhigh;
        (*(*player).mo).x = resulthigh.x;
        (*(*player).mo).y = resulthigh.y;
        (*(*player).mo)
            .z = resulthigh.z - FixedMul((*player).height, (*(*player).mo).scale);
    } else if (*lines.offset(lineindex as isize)).args[2 as libc::c_int as usize] != 0
        && (*waypointmid).health
            == numwaypoints[sequence as usize] as libc::c_int - 1 as libc::c_int
    {
        closest = waypointmid;
        (*(*player).mo).x = resultlow.x;
        (*(*player).mo).y = resultlow.y;
        (*(*player).mo)
            .z = resultlow.z - FixedMul((*player).height, (*(*player).mo).scale);
    } else if P_AproxDistance(
        P_AproxDistance(
            (*(*player).mo).x - resultlow.x,
            (*(*player).mo).y - resultlow.y,
        ),
        (*(*player).mo).z - resultlow.z,
    )
        < P_AproxDistance(
            P_AproxDistance(
                (*(*player).mo).x - resulthigh.x,
                (*(*player).mo).y - resulthigh.y,
            ),
            (*(*player).mo).z - resulthigh.z,
        )
    {
        closest = waypointmid;
        (*(*player).mo).x = resultlow.x;
        (*(*player).mo).y = resultlow.y;
        (*(*player).mo)
            .z = resultlow.z - FixedMul((*player).height, (*(*player).mo).scale);
    } else {
        closest = waypointhigh;
        (*(*player).mo).x = resulthigh.x;
        (*(*player).mo).y = resulthigh.y;
        (*(*player).mo)
            .z = resulthigh.z - FixedMul((*player).height, (*(*player).mo).scale);
    }
    P_SetTarget2(&mut (*(*player).mo).tracer, closest);
    (*player)
        .powers[pw_carry as libc::c_int
        as usize] = CR_ROPEHANG as libc::c_int as uint16_t;
    (*player).speed = speed;
    S_StartSound((*player).mo as *const libc::c_void, sfx_s3k4a);
    (*player)
        .pflags = ::core::mem::transmute::<
        libc::c_uint,
        pflags_t,
    >(
        (*player).pflags as libc::c_uint
            & !(PF_JUMPED as libc::c_int | PF_NOJUMPDAMAGE as libc::c_int
                | PF_GLIDING as libc::c_int | PF_BOUNCING as libc::c_int
                | PF_SLIDING as libc::c_int | PF_CANCARRY as libc::c_int) as libc::c_uint,
    );
    (*player).climbing = 0 as libc::c_int as uint8_t;
    P_SetThingPosition((*player).mo);
    P_SetPlayerMobjState((*player).mo, S_PLAY_RIDE);
}
unsafe extern "C" fn P_SectorHasSpecial(mut sec: *mut sector_t) -> boolean {
    if (*sec).specialflags as u64 != 0 {
        return true_0 as libc::c_int;
    }
    if (*sec).damagetype as libc::c_int != SD_NONE as libc::c_int {
        return true_0 as libc::c_int;
    }
    if (*sec).triggertag != 0 {
        return true_0 as libc::c_int;
    }
    if (*sec).special != 0 {
        return true_0 as libc::c_int;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn P_EvaluateSpecialFlags(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut roversector: *mut sector_t,
    mut isTouching: boolean,
) {
    let mut sectag: mtag_t = Tag_FGet(&mut (*sector).tags);
    if (*sector).specialflags as libc::c_uint
        & SSF_OUTERSPACE as libc::c_int as libc::c_uint != 0
    {
        if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
            & SH_PROTECTWATER as libc::c_int == 0
            && (*player).powers[pw_spacetime as libc::c_int as usize] == 0
        {
            (*player)
                .powers[pw_spacetime as libc::c_int
                as usize] = (spacetimetics as libc::c_int + 1 as libc::c_int)
                as uint16_t;
        }
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_WINDCURRENT as libc::c_int as libc::c_uint != 0
    {
        (*player).onconveyor = 2 as libc::c_int;
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_CONVEYOR as libc::c_int as libc::c_uint != 0
    {
        (*player).onconveyor = 4 as libc::c_int;
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_SPEEDPAD as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        P_ProcessSpeedPad(player, sector, roversector, sectag);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_STARPOSTACTIVATOR as libc::c_int as libc::c_uint != 0
    {
        let mut post: *mut mobj_t = P_GetObjectTypeInSectorNum(
            MT_STARPOST,
            sector.offset_from(sectors) as libc::c_long as size_t,
        );
        if !post.is_null() {
            P_TouchStarPost(post, player, false_0 as libc::c_int);
        }
    }
    if (*sector).specialflags as libc::c_uint & SSF_EXIT as libc::c_int as libc::c_uint
        != 0
    {
        P_ProcessExitSector(player, sectag);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_SPECIALSTAGEPIT as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        P_ProcessSpecialStagePit(player);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_REDTEAMBASE as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        P_ProcessTeamBase(player, true_0 as libc::c_int);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_BLUETEAMBASE as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        P_ProcessTeamBase(player, false_0 as libc::c_int);
    }
    if (*sector).specialflags as libc::c_uint & SSF_FAN as libc::c_int as libc::c_uint
        != 0
    {
        (*(*player).mo).momz
            += mobjinfo[MT_FAN as libc::c_int as usize].mass / 4 as libc::c_int;
        if (*(*player).mo).momz > mobjinfo[MT_FAN as libc::c_int as usize].mass {
            (*(*player).mo).momz = mobjinfo[MT_FAN as libc::c_int as usize].mass;
        }
        if (*player).powers[pw_carry as libc::c_int as usize] == 0 {
            P_ResetPlayer(player);
            P_SetPlayerMobjState((*player).mo, S_PLAY_FALL);
            P_SetTarget2(&mut (*(*player).mo).tracer, (*player).mo);
            (*player)
                .powers[pw_carry as libc::c_int
                as usize] = CR_FAN as libc::c_int as uint16_t;
        }
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_SUPERTRANSFORM as libc::c_int as libc::c_uint != 0
    {
        if (*(*player).mo).health > 0 as libc::c_int && (*player).bot == 0
            && (*player).charflags & SF_SUPER as libc::c_int as uint32_t != 0
            && (*player).powers[pw_super as libc::c_int as usize] == 0
            && emeralds as libc::c_int
                & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                    | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                    | 64 as libc::c_int)
                == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int
                    | 8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int
                    | 64 as libc::c_int
        {
            P_DoSuperTransformation(player, true_0 as libc::c_int);
        }
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_FORCESPIN as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        if (*player).pflags as libc::c_uint & PF_SPINNING as libc::c_int as libc::c_uint
            == 0
        {
            (*player)
                .pflags = ::core::mem::transmute::<
                libc::c_uint,
                pflags_t,
            >(
                (*player).pflags as libc::c_uint
                    | PF_SPINNING as libc::c_int as libc::c_uint,
            );
            P_SetPlayerMobjState((*player).mo, S_PLAY_ROLL);
            S_StartSound((*player).mo as *const libc::c_void, sfx_spin);
            if abs((*player).rmomx)
                < FixedMul(
                    5 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                    (*(*player).mo).scale,
                )
                && abs((*player).rmomy)
                    < FixedMul(
                        5 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        (*(*player).mo).scale,
                    )
            {
                P_InstaThrust(
                    (*player).mo,
                    (*(*player).mo).angle,
                    FixedMul(
                        10 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
                        (*(*player).mo).scale,
                    ),
                );
            }
        }
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_ZOOMTUBESTART as libc::c_int as libc::c_uint != 0
    {
        P_ProcessZoomTube(player, sectag, false_0 as libc::c_int);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_ZOOMTUBEEND as libc::c_int as libc::c_uint != 0
    {
        P_ProcessZoomTube(player, sectag, true_0 as libc::c_int);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_FINISHLINE as libc::c_int as libc::c_uint != 0
    {
        P_ProcessFinishLine(player);
    }
    if (*sector).specialflags as libc::c_uint
        & SSF_ROPEHANG as libc::c_int as libc::c_uint != 0 && isTouching != 0
    {
        P_ProcessRopeHang(player, sectag);
    }
}
unsafe extern "C" fn P_EvaluateDamageType(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut isTouching: boolean,
) {
    match (*sector).damagetype as libc::c_int {
        1 => {
            if isTouching != 0 {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                );
            }
        }
        2 => {
            if isTouching != 0
                && ((*player).powers[pw_underwater as libc::c_int as usize]
                    as libc::c_int != 0
                    || (*player).powers[pw_carry as libc::c_int as usize] as libc::c_int
                        == CR_NIGHTSMODE as libc::c_int)
            {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    1 as libc::c_int as uint8_t,
                );
            }
        }
        3 | 4 => {
            if isTouching != 0 {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    2 as libc::c_int as uint8_t,
                );
            }
        }
        5 => {
            if isTouching != 0 {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    3 as libc::c_int as uint8_t,
                );
            }
        }
        6 => {
            if isTouching != 0 {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    4 as libc::c_int as uint8_t,
                );
            }
        }
        7 | 8 => {
            if !(isTouching == 0) {
                if (*player).quittime != 0 {
                    G_MovePlayerToSpawnOrStarpost(
                        player.offset_from(players.as_mut_ptr()) as libc::c_long
                            as int32_t,
                    );
                } else {
                    P_DamageMobj(
                        (*player).mo,
                        0 as *mut mobj_t,
                        0 as *mut mobj_t,
                        1 as libc::c_int,
                        (0x80 as libc::c_int + 3 as libc::c_int) as uint8_t,
                    );
                }
            }
        }
        9 => {
            if (*player).quittime != 0 {
                G_MovePlayerToSpawnOrStarpost(
                    player.offset_from(players.as_mut_ptr()) as libc::c_long as int32_t,
                );
            } else {
                P_DamageMobj(
                    (*player).mo,
                    0 as *mut mobj_t,
                    0 as *mut mobj_t,
                    1 as libc::c_int,
                    0x80 as libc::c_int as uint8_t,
                );
            }
        }
        10 => {
            if !(isTouching == 0) {
                if !((*player).exiting != 0 || (*player).bot as libc::c_int != 0) {
                    if (*player).powers[pw_shield as libc::c_int as usize] as libc::c_int
                        != 0 || (*player).spheres as libc::c_int > 0 as libc::c_int
                    {
                        P_SpecialStageDamage(player, 0 as *mut mobj_t, 0 as *mut mobj_t);
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn P_EvaluateLinedefExecutorTrigger(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut isTouching: boolean,
) {
    if (*player).bot != 0 {
        return;
    }
    if (*sector).triggertag == 0 {
        return;
    }
    if (*sector).triggerer as libc::c_int == TO_MOBJ as libc::c_int {
        return
    } else if (*sector).triggerer as libc::c_int == TO_ALLPLAYERS as libc::c_int
        && P_DoAllPlayersTrigger((*sector).triggertag) == 0
    {
        return
    }
    if (*sector).flags as libc::c_uint
        & MSF_TRIGGERLINE_PLANE as libc::c_int as libc::c_uint != 0 && isTouching == 0
    {
        return;
    }
    P_LinedefExecute((*sector).triggertag, (*player).mo, sector);
}
unsafe extern "C" fn P_EvaluateOldSectorSpecial(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut roversector: *mut sector_t,
    mut isTouching: boolean,
) {
    let mut current_block_3: u64;
    match (*sector).special as libc::c_int
        >> (1 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int & 15 as libc::c_int
    {
        9 => {
            if isTouching == 0 {
                current_block_3 = 6873731126896040597;
            } else {
                current_block_3 = 15990647557822983428;
            }
        }
        10 => {
            current_block_3 = 15990647557822983428;
        }
        _ => {
            current_block_3 = 6873731126896040597;
        }
    }
    match current_block_3 {
        15990647557822983428 => {
            if leveltime % (35 as libc::c_int / 2 as libc::c_int) as tic_t
                == 0 as libc::c_int as tic_t
                && (*player).rings as libc::c_int > 0 as libc::c_int
            {
                (*player).rings -= 1;
                (*player).rings;
                S_StartSound((*player).mo as *const libc::c_void, sfx_antiri);
            }
        }
        _ => {}
    }
    match (*sector).special as libc::c_int
        >> (2 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int & 15 as libc::c_int
    {
        9 => {
            if !roversector.is_null() {
                P_ProcessEggCapsule(player, sector);
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_ProcessSpecialSector(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
    mut roversector: *mut sector_t,
) {
    let mut isTouching: boolean = 0;
    if P_SectorHasSpecial(sector) == 0 {
        return;
    }
    if (*player).spectator != 0 {
        return;
    }
    if (*player).playerstate as libc::c_uint != PST_LIVE as libc::c_int as libc::c_uint {
        return;
    }
    isTouching = (!roversector.is_null()
        || P_IsMobjTouchingSectorPlane((*player).mo, sector) != 0) as libc::c_int;
    P_EvaluateSpecialFlags(player, sector, roversector, isTouching);
    P_EvaluateDamageType(player, sector, isTouching);
    P_EvaluateLinedefExecutorTrigger(player, sector, isTouching);
    if udmf == 0 {
        P_EvaluateOldSectorSpecial(player, sector, roversector, isTouching);
    }
}
unsafe extern "C" fn P_PlayerOnSpecial3DFloor(
    mut player: *mut player_t,
    mut sector: *mut sector_t,
) {
    let mut originalsector: *mut sector_t = (*(*(*player).mo).subsector).sector;
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*sector).ffloors;
    while !rover.is_null() {
        if !(P_SectorHasSpecial((*(*rover).master).frontsector) == 0) {
            if !((*rover).fofflags as libc::c_uint
                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
            {
                if !(P_IsMobjTouching3DFloor((*player).mo, rover, sector) == 0) {
                    if sector == (*(*(*player).mo).subsector).sector
                        || (*(*(*rover).master).frontsector).flags as libc::c_uint
                            & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint
                            != 0
                    {
                        P_ProcessSpecialSector(
                            player,
                            (*(*rover).master).frontsector,
                            sector,
                        );
                        if (*(*(*player).mo).subsector).sector != originalsector {
                            return;
                        }
                    }
                }
            }
        }
        rover = (*rover).next;
    }
}
unsafe extern "C" fn P_PlayerOnSpecialPolyobj(mut player: *mut player_t) {
    let mut originalsector: *mut sector_t = (*(*(*player).mo).subsector).sector;
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut polysec: *mut sector_t = 0 as *mut sector_t;
    let mut touching: boolean = false_0 as libc::c_int;
    let mut inside: boolean = false_0 as libc::c_int;
    po = (*(*(*player).mo).subsector).polyList;
    while !po.is_null() {
        if !((*po).flags & POF_NOSPECIALS as libc::c_int != 0) {
            polysec = (**((*po).lines).offset(0 as libc::c_int as isize)).backsector;
            if !(P_SectorHasSpecial(polysec) == 0) {
                touching = ((*polysec).flags as libc::c_uint
                    & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint != 0
                    && P_MobjTouchingPolyobj(po, (*player).mo) != 0) as libc::c_int;
                inside = P_MobjInsidePolyobj(po, (*player).mo);
                if inside != 0 || touching != 0 {
                    if !(P_IsMobjTouchingPolyobj((*player).mo, po, polysec) == 0) {
                        P_ProcessSpecialSector(player, polysec, originalsector);
                        if (*(*(*player).mo).subsector).sector != originalsector {
                            return;
                        }
                    }
                }
            }
        }
        po = (*po).link.next as *mut polyobj_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_PlayerInSpecialSector(mut player: *mut player_t) {
    let mut originalsector: *mut sector_t = 0 as *mut sector_t;
    let mut loopsector: *mut sector_t = 0 as *mut sector_t;
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    if ((*player).mo).is_null() {
        return;
    }
    originalsector = (*(*(*player).mo).subsector).sector;
    P_PlayerOnSpecial3DFloor(player, originalsector);
    if (*(*(*player).mo).subsector).sector != originalsector {
        return;
    }
    P_PlayerOnSpecialPolyobj(player);
    if (*(*(*player).mo).subsector).sector != originalsector {
        return;
    }
    P_ProcessSpecialSector(player, originalsector, 0 as *mut sector_t);
    if (*(*(*player).mo).subsector).sector != originalsector {
        return;
    }
    node = (*(*player).mo).touching_sectorlist;
    while !node.is_null() {
        loopsector = (*node).m_sector;
        if !(loopsector == originalsector) {
            P_PlayerOnSpecial3DFloor(player, loopsector);
            if (*(*(*player).mo).subsector).sector != originalsector {
                return;
            }
            if !((*loopsector).flags as libc::c_uint
                & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint == 0)
            {
                P_ProcessSpecialSector(player, loopsector, 0 as *mut sector_t);
                if (*(*(*player).mo).subsector).sector != originalsector {
                    return;
                }
            }
        }
        node = (*node).m_sectorlist_next;
    }
}
unsafe extern "C" fn P_CheckMobj3DFloorTrigger(
    mut mo: *mut mobj_t,
    mut sec: *mut sector_t,
) {
    let mut originalsector: *mut sector_t = (*(*mo).subsector).sector;
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    rover = (*sec).ffloors;
    while !rover.is_null() {
        if !((*(*(*rover).master).frontsector).triggertag == 0) {
            if !((*(*(*rover).master).frontsector).triggerer as libc::c_int
                != TO_MOBJ as libc::c_int)
            {
                if !((*rover).fofflags as libc::c_uint
                    & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
                {
                    if !(P_IsMobjTouching3DFloor(mo, rover, sec) == 0) {
                        P_LinedefExecute(
                            (*(*(*rover).master).frontsector).triggertag,
                            mo,
                            (*(*rover).master).frontsector,
                        );
                        if (*(*mo).subsector).sector != originalsector {
                            return;
                        }
                    }
                }
            }
        }
        rover = (*rover).next;
    }
}
unsafe extern "C" fn P_CheckMobjPolyobjTrigger(mut mo: *mut mobj_t) {
    let mut originalsector: *mut sector_t = (*(*mo).subsector).sector;
    let mut po: *mut polyobj_t = 0 as *mut polyobj_t;
    let mut polysec: *mut sector_t = 0 as *mut sector_t;
    let mut touching: boolean = false_0 as libc::c_int;
    let mut inside: boolean = false_0 as libc::c_int;
    po = (*(*mo).subsector).polyList;
    while !po.is_null() {
        if !((*po).flags & POF_NOSPECIALS as libc::c_int != 0) {
            polysec = (**((*po).lines).offset(0 as libc::c_int as isize)).backsector;
            if !((*polysec).triggertag == 0) {
                if !((*polysec).triggerer as libc::c_int != TO_MOBJ as libc::c_int) {
                    touching = ((*polysec).flags as libc::c_uint
                        & MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint != 0
                        && P_MobjTouchingPolyobj(po, mo) != 0) as libc::c_int;
                    inside = P_MobjInsidePolyobj(po, mo);
                    if inside != 0 || touching != 0 {
                        if !(P_IsMobjTouchingPolyobj(mo, po, polysec) == 0) {
                            P_LinedefExecute((*polysec).triggertag, mo, polysec);
                            if (*(*mo).subsector).sector != originalsector {
                                return;
                            }
                        }
                    }
                }
            }
        }
        po = (*po).link.next as *mut polyobj_t;
    }
}
unsafe extern "C" fn P_CheckMobjSectorTrigger(
    mut mo: *mut mobj_t,
    mut sec: *mut sector_t,
) {
    if (*sec).triggertag == 0 {
        return;
    }
    if (*sec).triggerer as libc::c_int != TO_MOBJ as libc::c_int {
        return;
    }
    if (*sec).flags as libc::c_uint
        & MSF_TRIGGERLINE_PLANE as libc::c_int as libc::c_uint != 0
        && P_IsMobjTouchingSectorPlane(mo, sec) == 0
    {
        return;
    }
    P_LinedefExecute((*sec).triggertag, mo, sec);
}
#[no_mangle]
pub unsafe extern "C" fn P_CheckMobjTrigger(
    mut mobj: *mut mobj_t,
    mut pushable: boolean,
) {
    let mut originalsector: *mut sector_t = 0 as *mut sector_t;
    if ((*mobj).subsector).is_null() {
        return;
    }
    originalsector = (*(*mobj).subsector).sector;
    if pushable == 0
        && (*originalsector).flags as libc::c_uint
            & MSF_TRIGGERLINE_MOBJ as libc::c_int as libc::c_uint == 0
    {
        return;
    }
    P_CheckMobj3DFloorTrigger(mobj, originalsector);
    if (*(*mobj).subsector).sector != originalsector {
        return;
    }
    P_CheckMobjPolyobjTrigger(mobj);
    if (*(*mobj).subsector).sector != originalsector {
        return;
    }
    P_CheckMobjSectorTrigger(mobj, originalsector);
}
#[no_mangle]
pub unsafe extern "C" fn P_UpdateSpecials() {
    let mut anim: *mut anim_t = 0 as *mut anim_t;
    let mut i: int32_t = 0;
    let mut pic: int32_t = 0;
    let mut j: size_t = 0;
    let mut foundflats: *mut levelflat_t = 0 as *mut levelflat_t;
    P_CheckTimeLimit();
    P_CheckPointLimit();
    anim = anims;
    while anim < lastanim {
        i = 0 as libc::c_int;
        while i < (*anim).numpics {
            pic = ((*anim).basepic as tic_t)
                .wrapping_add(
                    (leveltime / (*anim).speed).wrapping_add(i as tic_t)
                        % (*anim).numpics as tic_t,
                ) as int32_t;
            if (*anim).istexture != 0 {
                *texturetranslation.offset(((*anim).basepic + i) as isize) = pic;
            }
            i += 1;
            i;
        }
        anim = anim.offset(1);
        anim;
    }
    foundflats = levelflats;
    j = 0 as libc::c_int as size_t;
    while j < numlevelflats {
        if (*foundflats).speed != 0 {
            if (*foundflats).type_0 as libc::c_int == LEVELFLAT_TEXTURE as libc::c_int
                && (*foundflats).u.texture.basenum != -(1 as libc::c_int)
            {
                (*foundflats)
                    .u
                    .texture
                    .num = ((*foundflats).u.texture.basenum as tic_t)
                    .wrapping_add(
                        (leveltime / (*foundflats).speed as tic_t)
                            .wrapping_add((*foundflats).animseq as tic_t)
                            % (*foundflats).numpics as tic_t,
                    ) as int32_t;
            } else if (*foundflats).type_0 as libc::c_int
                == LEVELFLAT_FLAT as libc::c_int
                && (*foundflats).u.flat.baselumpnum != 4294967295 as libc::c_uint
            {
                (*foundflats)
                    .u
                    .flat
                    .lumpnum = ((*foundflats).u.flat.baselumpnum)
                    .wrapping_add(
                        (leveltime / (*foundflats).speed as tic_t)
                            .wrapping_add((*foundflats).animseq as tic_t)
                            % (*foundflats).numpics as tic_t,
                    );
            }
        }
        j = j.wrapping_add(1);
        j;
        foundflats = foundflats.offset(1);
        foundflats;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_GetFFloorID(mut fflr: *mut ffloor_t) -> uint16_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    let mut sec: *mut sector_t = 0 as *mut sector_t;
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    if fflr.is_null() {
        return 65535 as libc::c_int as uint16_t;
    }
    sec = (*fflr).target;
    if ((*sec).ffloors).is_null() {
        return 65535 as libc::c_int as uint16_t;
    }
    rover = (*sec).ffloors;
    while !rover.is_null() {
        if rover == fflr {
            return i;
        }
        rover = (*rover).next;
        i = i.wrapping_add(1);
        i;
    }
    return 65535 as libc::c_int as uint16_t;
}
#[inline]
unsafe extern "C" fn P_GetFFloorBySec(
    mut sec: *mut sector_t,
    mut sec2: *mut sector_t,
) -> *mut ffloor_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    if ((*sec).ffloors).is_null() {
        return 0 as *mut ffloor_t;
    }
    rover = (*sec).ffloors;
    while !rover.is_null() {
        if (*rover).secnum == sec2.offset_from(sectors) as libc::c_long as size_t {
            return rover;
        }
        rover = (*rover).next;
    }
    return 0 as *mut ffloor_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_GetFFloorByID(
    mut sec: *mut sector_t,
    mut id: uint16_t,
) -> *mut ffloor_t {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    let mut i: uint16_t = 0 as libc::c_int as uint16_t;
    if ((*sec).ffloors).is_null() {
        return 0 as *mut ffloor_t;
    }
    rover = (*sec).ffloors;
    while !rover.is_null() {
        let fresh19 = i;
        i = i.wrapping_add(1);
        if fresh19 as libc::c_int == id as libc::c_int {
            return rover;
        }
        rover = (*rover).next;
    }
    return 0 as *mut ffloor_t;
}
#[inline]
unsafe extern "C" fn P_AddFFloorToList(mut sec: *mut sector_t, mut fflr: *mut ffloor_t) {
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    if ((*sec).ffloors).is_null() {
        (*sec).ffloors = fflr;
        (*fflr).next = 0 as *mut ffloor_s;
        (*fflr).prev = 0 as *mut ffloor_s;
        return;
    }
    rover = (*sec).ffloors;
    while !((*rover).next).is_null() {
        rover = (*rover).next;
    }
    (*rover).next = fflr;
    (*fflr).prev = rover;
    (*fflr).next = 0 as *mut ffloor_s;
}
unsafe extern "C" fn P_AddFakeFloor(
    mut sec: *mut sector_t,
    mut sec2: *mut sector_t,
    mut master: *mut line_t,
    mut alpha: int32_t,
    mut blendmode: uint8_t,
    mut flags: ffloortype_e,
    mut secthinkers: *mut thinkerlist_t,
) -> *mut ffloor_t {
    let mut fflr: *mut ffloor_t = 0 as *mut ffloor_t;
    let mut th: *mut thinker_t = 0 as *mut thinker_t;
    let mut f: *mut friction_t = 0 as *mut friction_t;
    let mut p: *mut pusher_t = 0 as *mut pusher_t;
    let mut sec2num: size_t = 0;
    let mut i: size_t = 0;
    if sec == sec2 {
        return 0 as *mut ffloor_t;
    }
    fflr = P_GetFFloorBySec(sec, sec2);
    if !fflr.is_null() {
        return fflr;
    }
    if (*sec2).ceilingheight < (*sec2).floorheight {
        let mut tempceiling: fixed_t = (*sec2).ceilingheight;
        CONS_Alert(
            CONS_ERROR,
            b"A FOF tagged %d has a top height below its bottom.\n\0" as *const u8
                as *const libc::c_char,
            (*master).args[0 as libc::c_int as usize],
        );
        (*sec2).ceilingheight = (*sec2).floorheight;
        (*sec2).floorheight = tempceiling;
    }
    if (*sec2).numattached == 0 as libc::c_int as size_t {
        (*sec2)
            .attached = Z_MallocAlign(
            (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul((*sec2).maxattached),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut size_t;
        (*sec2)
            .attachedsolid = Z_MallocAlign(
            (::core::mem::size_of::<boolean>() as libc::c_ulong)
                .wrapping_mul((*sec2).maxattached),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut boolean;
        *((*sec2).attached)
            .offset(
                0 as libc::c_int as isize,
            ) = sec.offset_from(sectors) as libc::c_long as size_t;
        (*sec2).numattached = 1 as libc::c_int as size_t;
        *((*sec2).attachedsolid)
            .offset(
                0 as libc::c_int as isize,
            ) = (flags as libc::c_uint & FOF_SOLID as libc::c_int as libc::c_uint)
            as boolean;
    } else {
        i = 0 as libc::c_int as size_t;
        while i < (*sec2).numattached {
            if *((*sec2).attached).offset(i as isize)
                == sec.offset_from(sectors) as libc::c_long as size_t
            {
                return 0 as *mut ffloor_t;
            }
            i = i.wrapping_add(1);
            i;
        }
        if (*sec2).numattached >= (*sec2).maxattached {
            (*sec2).maxattached = (*sec2).maxattached * 2 as libc::c_int as size_t;
            (*sec2)
                .attached = Z_ReallocAlign(
                (*sec2).attached as *mut libc::c_void,
                (::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul((*sec2).maxattached),
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut size_t;
            (*sec2)
                .attachedsolid = Z_ReallocAlign(
                (*sec2).attachedsolid as *mut libc::c_void,
                (::core::mem::size_of::<boolean>() as libc::c_ulong)
                    .wrapping_mul((*sec2).maxattached),
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut boolean;
        }
        *((*sec2).attached)
            .offset(
                (*sec2).numattached as isize,
            ) = sec.offset_from(sectors) as libc::c_long as size_t;
        *((*sec2).attachedsolid)
            .offset(
                (*sec2).numattached as isize,
            ) = (flags as libc::c_uint & FOF_SOLID as libc::c_int as libc::c_uint)
            as boolean;
        (*sec2).numattached = ((*sec2).numattached).wrapping_add(1);
        (*sec2).numattached;
    }
    fflr = Z_CallocAlign(
        ::core::mem::size_of::<ffloor_t>() as libc::c_ulong,
        PU_LEVEL as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut ffloor_t;
    (*fflr).secnum = sec2.offset_from(sectors) as libc::c_long as size_t;
    (*fflr).target = sec;
    (*fflr).bottomheight = &mut (*sec2).floorheight;
    (*fflr).bottompic = &mut (*sec2).floorpic;
    (*fflr).bottomxoffs = &mut (*sec2).floorxoffset;
    (*fflr).bottomyoffs = &mut (*sec2).flooryoffset;
    (*fflr).bottomangle = &mut (*sec2).floorangle;
    (*fflr).topheight = &mut (*sec2).ceilingheight;
    (*fflr).toppic = &mut (*sec2).ceilingpic;
    (*fflr).toplightlevel = &mut (*sec2).lightlevel;
    (*fflr).topxoffs = &mut (*sec2).ceilingxoffset;
    (*fflr).topyoffs = &mut (*sec2).ceilingyoffset;
    (*fflr).topangle = &mut (*sec2).ceilingangle;
    (*fflr).t_slope = &mut (*sec2).c_slope;
    (*fflr).b_slope = &mut (*sec2).f_slope;
    if (*sec2).hasslope != 0 {
        (*sec).hasslope = true_0 as libc::c_int;
    }
    (*fflr).fofflags = flags;
    (*fflr).spawnflags = (*fflr).fofflags;
    (*fflr).master = master;
    (*fflr).norender = 4294967295 as libc::c_uint;
    (*fflr).fadingdata = 0 as *mut libc::c_void;
    sec2num = sec2.offset_from(sectors) as libc::c_long as size_t;
    i = 0 as libc::c_int as size_t;
    th = (*thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)).next;
    loop {
        if !secthinkers.is_null() {
            if !(i < (*secthinkers.offset(sec2num as isize)).count as size_t) {
                break;
            }
            th = *((*secthinkers.offset(sec2num as isize)).thinkers).offset(i as isize);
        } else if th
            == &mut *thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)
                as *mut thinker_t
        {
            break;
        }
        if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut friction_t) -> ()>,
                actionf_p1,
            >(Some(T_Friction as unsafe extern "C" fn(*mut friction_t) -> ()))
        {
            f = th as *mut friction_t;
            if (*f).affectee == sec2num as int32_t {
                Add_Friction(
                    (*f).friction,
                    (*f).movefactor,
                    sec.offset_from(sectors) as libc::c_long as int32_t,
                    (*f).affectee,
                );
            }
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut pusher_t) -> ()>,
                actionf_p1,
            >(Some(T_Pusher as unsafe extern "C" fn(*mut pusher_t) -> ()))
        {
            p = th as *mut pusher_t;
            if (*p).affectee == sec2num as int32_t {
                Add_Pusher(
                    (*p).type_0,
                    (*p).x_mag,
                    (*p).y_mag,
                    (*p).z_mag,
                    sec.offset_from(sectors) as libc::c_long as int32_t,
                    (*p).affectee,
                    (*p).exclusive,
                    (*p).slider,
                );
            }
        }
        if !secthinkers.is_null() {
            i = i.wrapping_add(1);
            i;
        } else {
            th = (*th).next;
        }
    }
    (*fflr)
        .alpha = if 0 as libc::c_int
        > (if (0xff as libc::c_int) < alpha { 0xff as libc::c_int } else { alpha })
    {
        0 as libc::c_int
    } else if (0xff as libc::c_int) < alpha {
        0xff as libc::c_int
    } else {
        alpha
    };
    if (*fflr).alpha < 0xff as libc::c_int
        || flags as libc::c_uint & FOF_SPLAT as libc::c_int as libc::c_uint != 0
    {
        (*fflr)
            .fofflags = ::core::mem::transmute::<
            libc::c_uint,
            ffloortype_e,
        >(
            (*fflr).fofflags as libc::c_uint
                | FOF_TRANSLUCENT as libc::c_int as libc::c_uint,
        );
        (*fflr).spawnflags = (*fflr).fofflags;
    }
    (*fflr).spawnalpha = (*fflr).alpha;
    match blendmode as libc::c_int {
        1 => {
            (*fflr).blend = AST_ADD as libc::c_int as uint8_t;
        }
        2 => {
            (*fflr).blend = AST_SUBTRACT as libc::c_int as uint8_t;
        }
        3 => {
            (*fflr).blend = AST_REVERSESUBTRACT as libc::c_int as uint8_t;
        }
        4 => {
            (*fflr).blend = AST_MODULATE as libc::c_int as uint8_t;
        }
        0 | _ => {
            (*fflr).blend = AST_COPY as libc::c_int as uint8_t;
        }
    }
    if flags as libc::c_uint & FOF_QUICKSAND as libc::c_int as libc::c_uint != 0 {
        CheckForQuicksand = true_0 as libc::c_int;
    }
    if flags as libc::c_uint & FOF_BUSTUP as libc::c_int as libc::c_uint != 0 {
        CheckForBustableBlocks = true_0 as libc::c_int;
    }
    if flags as libc::c_uint & FOF_MARIO as libc::c_int as libc::c_uint != 0 {
        if flags as libc::c_uint & FOF_GOOWATER as libc::c_int as libc::c_uint == 0 {
            P_AddBlockThinker(sec2, master);
        }
        CheckForMarioBlocks = true_0 as libc::c_int;
    }
    if flags as libc::c_uint & FOF_CRUMBLE as libc::c_int as libc::c_uint != 0 {
        (*sec2).crumblestate = CRUMBLE_WAIT as libc::c_int;
    }
    if flags as libc::c_uint & FOF_FLOATBOB as libc::c_int as libc::c_uint != 0 {
        P_AddFloatThinker(
            sec2,
            (*master).args[0 as libc::c_int as usize] as uint16_t,
            master,
        );
        CheckForFloatBob = true_0 as libc::c_int;
    }
    P_AddFFloorToList(sec, fflr);
    return fflr;
}
unsafe extern "C" fn P_AddFloatThinker(
    mut sec: *mut sector_t,
    mut tag: uint16_t,
    mut sourceline: *mut line_t,
) {
    let mut floater: *mut floatthink_t = 0 as *mut floatthink_t;
    floater = Z_CallocAlign(
        ::core::mem::size_of::<floatthink_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut floatthink_t;
    P_AddThinker(THINK_MAIN, &mut (*floater).thinker);
    (*floater)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut floatthink_t) -> ()>,
        actionf_p1,
    >(Some(T_FloatSector as unsafe extern "C" fn(*mut floatthink_t) -> ()));
    (*floater).sector = sec;
    (*floater).tag = tag as int16_t;
    (*floater).sourceline = sourceline;
    R_CreateInterpolator_SectorPlane(
        &mut (*floater).thinker,
        sec,
        false_0 as libc::c_int,
    );
    R_CreateInterpolator_SectorPlane(
        &mut (*floater).thinker,
        sec,
        true_0 as libc::c_int,
    );
}
unsafe extern "C" fn P_AddPlaneDisplaceThinker(
    mut type_0: int32_t,
    mut speed: fixed_t,
    mut control: int32_t,
    mut affectee: int32_t,
    mut reverse: uint8_t,
) {
    let mut displace: *mut planedisplace_t = 0 as *mut planedisplace_t;
    displace = Z_CallocAlign(
        ::core::mem::size_of::<planedisplace_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut planedisplace_t;
    P_AddThinker(THINK_MAIN, &mut (*displace).thinker);
    (*displace)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut planedisplace_t) -> ()>,
        actionf_p1,
    >(Some(T_PlaneDisplace as unsafe extern "C" fn(*mut planedisplace_t) -> ()));
    (*displace).affectee = affectee;
    (*displace).control = control;
    (*displace).last_height = (*sectors.offset(control as isize)).floorheight;
    (*displace).speed = speed;
    (*displace).type_0 = type_0 as C2RustUnnamed_67;
    (*displace).reverse = reverse;
    R_CreateInterpolator_SectorPlane(
        &mut (*displace).thinker,
        &mut *sectors.offset(affectee as isize),
        false_0 as libc::c_int,
    );
}
unsafe extern "C" fn P_AddBlockThinker(
    mut sec: *mut sector_t,
    mut sourceline: *mut line_t,
) {
    let mut block: *mut mariocheck_t = 0 as *mut mariocheck_t;
    block = Z_CallocAlign(
        ::core::mem::size_of::<mariocheck_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut mariocheck_t;
    P_AddThinker(THINK_MAIN, &mut (*block).thinker);
    (*block)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut mariocheck_t) -> ()>,
        actionf_p1,
    >(Some(T_MarioBlockChecker as unsafe extern "C" fn(*mut mariocheck_t) -> ()));
    (*block).sourceline = sourceline;
    (*block).sector = sec;
}
unsafe extern "C" fn P_AddRaiseThinker(
    mut sec: *mut sector_t,
    mut tag: int16_t,
    mut speed: fixed_t,
    mut ceilingtop: fixed_t,
    mut ceilingbottom: fixed_t,
    mut lower: boolean,
    mut spindash: boolean,
) {
    let mut raise: *mut raise_t = 0 as *mut raise_t;
    raise = Z_CallocAlign(
        ::core::mem::size_of::<raise_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut raise_t;
    P_AddThinker(THINK_MAIN, &mut (*raise).thinker);
    (*raise)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut raise_t) -> ()>,
        actionf_p1,
    >(Some(T_RaiseSector as unsafe extern "C" fn(*mut raise_t) -> ()));
    (*raise).tag = tag;
    (*raise).sector = sec;
    (*raise).ceilingtop = ceilingtop;
    (*raise).ceilingbottom = ceilingbottom;
    (*raise).basespeed = speed >> 2 as libc::c_int;
    if lower != 0 {
        (*raise)
            .flags = ((*raise).flags as libc::c_int | RF_REVERSE as libc::c_int)
            as uint8_t;
    }
    if spindash != 0 {
        (*raise)
            .flags = ((*raise).flags as libc::c_int | RF_SPINDASH as libc::c_int)
            as uint8_t;
    }
    R_CreateInterpolator_SectorPlane(&mut (*raise).thinker, sec, false_0 as libc::c_int);
    R_CreateInterpolator_SectorPlane(&mut (*raise).thinker, sec, true_0 as libc::c_int);
}
unsafe extern "C" fn P_AddAirbob(
    mut sec: *mut sector_t,
    mut tag: int16_t,
    mut dist: fixed_t,
    mut raise: boolean,
    mut spindash: boolean,
    mut dynamic: boolean,
) {
    let mut airbob: *mut raise_t = 0 as *mut raise_t;
    airbob = Z_CallocAlign(
        ::core::mem::size_of::<raise_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut raise_t;
    P_AddThinker(THINK_MAIN, &mut (*airbob).thinker);
    (*airbob)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut raise_t) -> ()>,
        actionf_p1,
    >(Some(T_RaiseSector as unsafe extern "C" fn(*mut raise_t) -> ()));
    (*airbob).tag = tag;
    (*airbob).sector = sec;
    (*airbob).ceilingtop = (*sec).ceilingheight;
    (*airbob).ceilingbottom = (*sec).ceilingheight - dist;
    (*airbob).basespeed = (1 as libc::c_int) << 16 as libc::c_int;
    if raise == 0 {
        (*airbob)
            .flags = ((*airbob).flags as libc::c_int | RF_REVERSE as libc::c_int)
            as uint8_t;
    }
    if spindash != 0 {
        (*airbob)
            .flags = ((*airbob).flags as libc::c_int | RF_SPINDASH as libc::c_int)
            as uint8_t;
    }
    if dynamic != 0 {
        (*airbob)
            .flags = ((*airbob).flags as libc::c_int | RF_DYNAMIC as libc::c_int)
            as uint8_t;
    }
    R_CreateInterpolator_SectorPlane(
        &mut (*airbob).thinker,
        sec,
        false_0 as libc::c_int,
    );
    R_CreateInterpolator_SectorPlane(&mut (*airbob).thinker, sec, true_0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn P_AddThwompThinker(
    mut sec: *mut sector_t,
    mut sourceline: *mut line_t,
    mut crushspeed: fixed_t,
    mut retractspeed: fixed_t,
    mut sound: uint16_t,
) {
    let mut thwomp: *mut thwomp_t = 0 as *mut thwomp_t;
    if !((*sec).floordata).is_null() || !((*sec).ceilingdata).is_null() {
        return;
    }
    thwomp = Z_CallocAlign(
        ::core::mem::size_of::<thwomp_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut thwomp_t;
    P_AddThinker(THINK_MAIN, &mut (*thwomp).thinker);
    (*thwomp)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut thwomp_t) -> ()>,
        actionf_p1,
    >(Some(T_ThwompSector as unsafe extern "C" fn(*mut thwomp_t) -> ()));
    (*thwomp).sourceline = sourceline;
    (*thwomp).sector = sec;
    (*thwomp).crushspeed = crushspeed;
    (*thwomp).retractspeed = retractspeed;
    (*thwomp).direction = 0 as libc::c_int;
    (*thwomp).floorstartheight = (*sec).floorheight;
    (*thwomp).ceilingstartheight = (*sec).ceilingheight;
    (*thwomp).delay = 1 as libc::c_int;
    (*thwomp).tag = (*sourceline).args[0 as libc::c_int as usize] as int16_t;
    (*thwomp).sound = sound;
    (*sec).floordata = thwomp as *mut libc::c_void;
    (*sec).ceilingdata = thwomp as *mut libc::c_void;
    (*sides.offset((*sourceline).sidenum[0 as libc::c_int as usize] as isize))
        .midtexture = (*sides
        .offset((*sourceline).sidenum[0 as libc::c_int as usize] as isize))
        .bottomtexture;
    R_CreateInterpolator_SectorPlane(
        &mut (*thwomp).thinker,
        sec,
        false_0 as libc::c_int,
    );
    R_CreateInterpolator_SectorPlane(&mut (*thwomp).thinker, sec, true_0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn P_AddNoEnemiesThinker(mut sourceline: *mut line_t) {
    let mut nobaddies: *mut noenemies_t = 0 as *mut noenemies_t;
    nobaddies = Z_CallocAlign(
        ::core::mem::size_of::<noenemies_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut noenemies_t;
    P_AddThinker(THINK_MAIN, &mut (*nobaddies).thinker);
    (*nobaddies)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut noenemies_t) -> ()>,
        actionf_p1,
    >(Some(T_NoEnemiesSector as unsafe extern "C" fn(*mut noenemies_t) -> ()));
    (*nobaddies).sourceline = sourceline;
}
unsafe extern "C" fn P_AddEachTimeThinker(
    mut sourceline: *mut line_t,
    mut triggerOnExit: boolean,
) {
    let mut eachtime: *mut eachtime_t = 0 as *mut eachtime_t;
    eachtime = Z_CallocAlign(
        ::core::mem::size_of::<eachtime_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut eachtime_t;
    P_AddThinker(THINK_MAIN, &mut (*eachtime).thinker);
    (*eachtime)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut eachtime_t) -> ()>,
        actionf_p1,
    >(Some(T_EachTimeThinker as unsafe extern "C" fn(*mut eachtime_t) -> ()));
    (*eachtime).sourceline = sourceline;
    (*eachtime).triggerOnExit = triggerOnExit;
}
#[inline]
unsafe extern "C" fn P_AddCameraScanner(
    mut sourcesec: *mut sector_t,
    mut actionsector: *mut sector_t,
    mut angle: angle_t,
) {
    let mut elevator: *mut elevator_t = 0 as *mut elevator_t;
    elevator = Z_CallocAlign(
        ::core::mem::size_of::<elevator_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut elevator_t;
    P_AddThinker(THINK_MAIN, &mut (*elevator).thinker);
    (*elevator)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut elevator_t) -> ()>,
        actionf_p1,
    >(Some(T_CameraScanner as unsafe extern "C" fn(*mut elevator_t) -> ()));
    (*elevator).type_0 = elevateBounce;
    (*elevator).sector = sourcesec;
    (*elevator).actionsector = actionsector;
    (*elevator).distance = FixedInt(AngleFixed(angle));
}
#[no_mangle]
pub unsafe extern "C" fn T_LaserFlash(mut flash: *mut laserthink_t) {
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut thing: *mut mobj_t = 0 as *mut mobj_t;
    let mut s: int32_t = 0;
    let mut fflr: *mut ffloor_t = 0 as *mut ffloor_t;
    let mut sector: *mut sector_t = 0 as *mut sector_t;
    let mut sourcesec: *mut sector_t = (*(*flash).sourceline).frontsector;
    let mut top: fixed_t = 0;
    let mut bottom: fixed_t = 0;
    let mut ICNT_6000: size_t = 0 as libc::c_int as size_t;
    loop {
        s = Tag_Iterate_Sectors((*flash).tag, ICNT_6000);
        if !(s >= 0 as libc::c_int) {
            break;
        }
        sector = &mut *sectors.offset(s as isize) as *mut sector_t;
        fflr = (*sector).ffloors;
        while !fflr.is_null() {
            if (*fflr).master != (*flash).sourceline {
                fflr = (*fflr).next;
            } else {
                if (*fflr).fofflags as libc::c_uint
                    & FOF_EXISTS as libc::c_int as libc::c_uint == 0
                {
                    break;
                }
                if leveltime & 2 as libc::c_int as tic_t != 0 {
                    (*fflr).alpha = 0xb0 as libc::c_int;
                } else {
                    (*fflr).alpha = 0x90 as libc::c_int;
                }
                top = P_GetFFloorTopZAt(
                    fflr,
                    (*sector).soundorg.x,
                    (*sector).soundorg.y,
                );
                bottom = P_GetFFloorBottomZAt(
                    fflr,
                    (*sector).soundorg.x,
                    (*sector).soundorg.y,
                );
                (*sector).soundorg.z = (top + bottom) / 2 as libc::c_int;
                S_StartSound(
                    &mut (*sector).soundorg as *mut degenmobj_t as *const libc::c_void,
                    sfx_laser,
                );
                node = (*sector).touching_thinglist;
                while !node.is_null() && !((*node).m_thing).is_null() {
                    thing = (*node).m_thing;
                    if !((*flash).nobosses as libc::c_int != 0
                        && (*thing).flags & MF_BOSS as libc::c_int as uint32_t != 0)
                    {
                        if !((*thing).health <= 0 as libc::c_int) {
                            top = P_MobjCeilingZ(
                                thing,
                                sourcesec,
                                sector,
                                (*thing).x,
                                (*thing).y,
                                0 as *mut line_t,
                                (sourcesec == sector) as libc::c_int,
                                true_0 as libc::c_int,
                            );
                            bottom = P_MobjFloorZ(
                                thing,
                                sourcesec,
                                sector,
                                (*thing).x,
                                (*thing).y,
                                0 as *mut line_t,
                                (sourcesec != sector) as libc::c_int,
                                true_0 as libc::c_int,
                            );
                            if !((*thing).z >= top
                                || (*thing).z + (*thing).height <= bottom)
                            {
                                if (*thing).flags & MF_SHOOTABLE as libc::c_int as uint32_t
                                    != 0
                                {
                                    P_DamageMobj(
                                        thing,
                                        0 as *mut mobj_t,
                                        0 as *mut mobj_t,
                                        1 as libc::c_int,
                                        0 as libc::c_int as uint8_t,
                                    );
                                } else if (*thing).type_0 as libc::c_uint
                                    == MT_EGGSHIELD as libc::c_int as libc::c_uint
                                {
                                    P_KillMobj(
                                        thing,
                                        0 as *mut mobj_t,
                                        0 as *mut mobj_t,
                                        0 as libc::c_int as uint8_t,
                                    );
                                }
                            }
                        }
                    }
                    node = (*node).m_thinglist_next;
                }
                break;
            }
        }
        ICNT_6000 = ICNT_6000.wrapping_add(1);
        ICNT_6000;
    };
}
#[inline]
unsafe extern "C" fn P_AddLaserThinker(
    mut tag: int16_t,
    mut line: *mut line_t,
    mut nobosses: boolean,
) {
    let mut flash: *mut laserthink_t = Z_CallocAlign(
        ::core::mem::size_of::<laserthink_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut laserthink_t;
    P_AddThinker(THINK_MAIN, &mut (*flash).thinker);
    (*flash)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut laserthink_t) -> ()>,
        actionf_p1,
    >(Some(T_LaserFlash as unsafe extern "C" fn(*mut laserthink_t) -> ()));
    (*flash).tag = tag;
    (*flash).sourceline = line;
    (*flash).nobosses = nobosses as uint8_t;
}
unsafe extern "C" fn P_RunLevelLoadExecutors() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if (*lines.offset(i as isize)).special as libc::c_int == 399 as libc::c_int {
            P_RunTriggerLinedef(
                &mut *lines.offset(i as isize),
                0 as *mut mobj_t,
                0 as *mut sector_t,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_InitSpecials() {
    gravity = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
        .gravity;
    sstimer = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
        .sstimer * 35 as libc::c_int + 6 as libc::c_int;
    ssspheres = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
        .ssspheres;
    CheckForReverseGravity = false_0 as libc::c_int;
    CheckForFloatBob = CheckForReverseGravity;
    CheckForMarioBlocks = CheckForFloatBob;
    CheckForQuicksand = CheckForMarioBlocks;
    CheckForBouncySector = CheckForQuicksand;
    CheckForBustableBlocks = CheckForBouncySector;
    let mut current_block_5: u64;
    match (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize]).weather
        as libc::c_int
    {
        3 => {
            current_block_5 = 8132578072319285558;
        }
        1 => {
            current_block_5 = 8132578072319285558;
        }
        5 => {
            current_block_5 = 10569942955920597798;
        }
        2 | 6 => {
            current_block_5 = 11274586060294963737;
        }
        _ => {
            curWeather = 0 as libc::c_int;
            current_block_5 = 7351195479953500246;
        }
    }
    match current_block_5 {
        8132578072319285558 => {
            current_block_5 = 10569942955920597798;
        }
        _ => {}
    }
    match current_block_5 {
        10569942955920597798 => {
            current_block_5 = 11274586060294963737;
        }
        _ => {}
    }
    match current_block_5 {
        11274586060294963737 => {
            curWeather = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int)
                as usize])
                .weather as int32_t;
        }
        _ => {}
    }
    globalweather = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int)
        as usize])
        .weather;
}
#[no_mangle]
pub unsafe extern "C" fn P_ApplyFlatAlignment(
    mut sector: *mut sector_t,
    mut flatangle: angle_t,
    mut xoffs: fixed_t,
    mut yoffs: fixed_t,
    mut floor: boolean,
    mut ceiling: boolean,
) {
    if floor != 0 {
        (*sector).floorangle = flatangle;
        (*sector).floorxoffset += xoffs;
        (*sector).flooryoffset += yoffs;
    }
    if ceiling != 0 {
        (*sector).ceilingangle = flatangle;
        (*sector).ceilingxoffset += xoffs;
        (*sector).ceilingyoffset += yoffs;
    }
}
unsafe extern "C" fn P_MakeFOFBouncy(
    mut paramline: *mut line_t,
    mut masterline: *mut line_t,
) {
    let mut s: int32_t = 0;
    if ((*masterline).special as libc::c_int) < 100 as libc::c_int
        || (*masterline).special as libc::c_int >= 300 as libc::c_int
    {
        return;
    }
    let mut ICNT_6144: size_t = 0 as libc::c_int as size_t;
    loop {
        s = Tag_Iterate_Sectors(
            (*masterline).args[0 as libc::c_int as usize] as mtag_t,
            ICNT_6144,
        );
        if !(s >= 0 as libc::c_int) {
            break;
        }
        let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
        rover = (*sectors.offset(s as isize)).ffloors;
        while !rover.is_null() {
            if (*rover).master != masterline {
                rover = (*rover).next;
            } else {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_BOUNCY as libc::c_int as libc::c_uint,
                );
                (*rover)
                    .spawnflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).spawnflags as libc::c_uint
                        | FOF_BOUNCY as libc::c_int as libc::c_uint,
                );
                (*rover)
                    .bouncestrength = ((*paramline).args[1 as libc::c_int as usize]
                    << 16 as libc::c_int) / 100 as libc::c_int;
                CheckForBouncySector = true_0 as libc::c_int;
                break;
            }
        }
        ICNT_6144 = ICNT_6144.wrapping_add(1);
        ICNT_6144;
    };
}
unsafe extern "C" fn P_CheckGametypeRules(
    mut checktype: int32_t,
    mut target: uint32_t,
) -> boolean {
    match checktype {
        1 => return (gametyperules & target != 0) as libc::c_int,
        2 => return (gametyperules == target) as libc::c_int,
        3 => return (gametyperules & target != target) as libc::c_int,
        4 => return (gametyperules & target == 0) as libc::c_int,
        0 | _ => return (gametyperules & target == target) as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_GetSectorGravityFactor(mut sec: *mut sector_t) -> fixed_t {
    if !((*sec).gravityptr).is_null() {
        return FixedDiv(*(*sec).gravityptr >> 16 as libc::c_int, 1000 as libc::c_int)
    } else {
        return (*sec).gravity
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_SpawnSpecials(mut fromnetsave: boolean) {
    let mut sector: *mut sector_t = 0 as *mut sector_t;
    let mut i: size_t = 0;
    let mut j: int32_t = 0;
    let mut secthinkers: *mut thinkerlist_t = 0 as *mut thinkerlist_t;
    let mut th: *mut thinker_t = 0 as *mut thinker_t;
    bossdisabled = 0 as libc::c_int as uint16_t;
    stoppedclock = false_0 as libc::c_int;
    sector = sectors;
    i = 0 as libc::c_int as size_t;
    while i < numsectors {
        CheckForReverseGravity = (CheckForReverseGravity as libc::c_uint
            | (*sector).flags as libc::c_uint
                & MSF_GRAVITYFLIP as libc::c_int as libc::c_uint) as boolean;
        if (*sector).specialflags as libc::c_uint
            & SSF_FINISHLINE as libc::c_int as libc::c_uint != 0
        {
            if gametyperules
                & (GTR_RACE as libc::c_int | GTR_LIVES as libc::c_int) as uint32_t
                == GTR_RACE as libc::c_int as uint32_t
            {
                circuitmap = true_0 as libc::c_int;
            }
        }
        if (*sector).damagetype as libc::c_int == SD_SPIKE as libc::c_int {
            (*sector)
                .flags = ::core::mem::transmute::<
                libc::c_uint,
                sectorflags_t,
            >(
                (*sector).flags as libc::c_uint
                    | MSF_TRIGGERSPECIAL_TOUCH as libc::c_int as libc::c_uint,
            );
        }
        if !(udmf != 0 || (*sector).special == 0) {
            match (*sector).special as libc::c_int
                >> (1 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                & 15 as libc::c_int
            {
                15 => {
                    CheckForBouncySector = true_0 as libc::c_int;
                }
                _ => {}
            }
            match (*sector).special as libc::c_int
                >> (2 as libc::c_int - 1 as libc::c_int) * 4 as libc::c_int
                & 15 as libc::c_int
            {
                10 => {
                    sstimer = ((*sector).floorheight >> 16 as libc::c_int)
                        * 35 as libc::c_int + 6 as libc::c_int;
                    ssspheres = ((*sector).ceilingheight >> 16 as libc::c_int)
                        as uint32_t;
                }
                11 => {
                    gravity = (*sector).floorheight / 1000 as libc::c_int;
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
        i;
        sector = sector.offset(1);
        sector;
    }
    P_SpawnScrollers();
    P_SpawnFriction();
    P_SpawnPushers();
    secthinkers = Z_CallocAlign(
        numsectors
            .wrapping_mul(::core::mem::size_of::<thinkerlist_t>() as libc::c_ulong),
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut thinkerlist_t;
    th = (*thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)).next;
    while th
        != &mut *thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)
            as *mut thinker_t
    {
        if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut friction_t) -> ()>,
                actionf_p1,
            >(Some(T_Friction as unsafe extern "C" fn(*mut friction_t) -> ()))
        {
            let ref mut fresh20 = (*secthinkers
                .offset((*(th as *mut friction_t)).affectee as isize))
                .count;
            *fresh20 = (*fresh20).wrapping_add(1);
            *fresh20;
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut pusher_t) -> ()>,
                actionf_p1,
            >(Some(T_Pusher as unsafe extern "C" fn(*mut pusher_t) -> ()))
        {
            let ref mut fresh21 = (*secthinkers
                .offset((*(th as *mut pusher_t)).affectee as isize))
                .count;
            *fresh21 = (*fresh21).wrapping_add(1);
            *fresh21;
        }
        th = (*th).next;
    }
    i = 0 as libc::c_int as size_t;
    while i < numsectors {
        if (*secthinkers.offset(i as isize)).count > 0 as libc::c_int as uint32_t {
            let ref mut fresh22 = (*secthinkers.offset(i as isize)).thinkers;
            *fresh22 = Z_MallocAlign(
                ((*secthinkers.offset(i as isize)).count as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<*mut thinker_t>() as libc::c_ulong,
                    ),
                PU_STATIC as libc::c_int,
                0 as *mut libc::c_void,
                ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
            ) as *mut *mut thinker_t;
            (*secthinkers.offset(i as isize)).count = 0 as libc::c_int as uint32_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    th = (*thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)).next;
    while th
        != &mut *thlist.as_mut_ptr().offset(THINK_MAIN as libc::c_int as isize)
            as *mut thinker_t
    {
        let mut secnum: size_t = -(1 as libc::c_int) as size_t;
        if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut friction_t) -> ()>,
                actionf_p1,
            >(Some(T_Friction as unsafe extern "C" fn(*mut friction_t) -> ()))
        {
            secnum = (*(th as *mut friction_t)).affectee as size_t;
        } else if (*th).function.acp1
            == ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut pusher_t) -> ()>,
                actionf_p1,
            >(Some(T_Pusher as unsafe extern "C" fn(*mut pusher_t) -> ()))
        {
            secnum = (*(th as *mut pusher_t)).affectee as size_t;
        }
        if secnum != -(1 as libc::c_int) as size_t {
            let ref mut fresh23 = (*secthinkers.offset(secnum as isize)).count;
            let fresh24 = *fresh23;
            *fresh23 = (*fresh23).wrapping_add(1);
            let ref mut fresh25 = *((*secthinkers.offset(secnum as isize)).thinkers)
                .offset(fresh24 as isize);
            *fresh25 = th;
        }
        th = (*th).next;
    }
    let mut current_block_383: u64;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if netgame != 0 || multiplayer != 0 {
            if (*lines.offset(i as isize)).flags as libc::c_int & 4096 as libc::c_int
                != 0
            {
                (*lines.offset(i as isize)).special = 0 as libc::c_int as int16_t;
                current_block_383 = 15004371738079956865;
            } else {
                current_block_383 = 7990025728955927862;
            }
        } else if (*lines.offset(i as isize)).flags as libc::c_int & 2048 as libc::c_int
            != 0
        {
            (*lines.offset(i as isize)).special = 0 as libc::c_int as int16_t;
            current_block_383 = 15004371738079956865;
        } else {
            current_block_383 = 7990025728955927862;
        }
        match current_block_383 {
            7990025728955927862 => {
                let mut s: int32_t = 0;
                let mut l: int32_t = 0;
                let mut sec: size_t = 0;
                let mut ffloorflags: ffloortype_e = 0 as ffloortype_e;
                let mut current_block_382: u64;
                match (*lines.offset(i as isize)).special as libc::c_int {
                    1 => {
                        if udmf != 0 {
                            current_block_382 = 15849589987095405551;
                        } else {
                            sec = ((*sides
                                .offset(
                                    *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                                ))
                                .sector)
                                .offset_from(sectors) as libc::c_long as size_t;
                            let mut ICNT_6329: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Sectors(
                                    Tag_FGet(&mut (*lines.offset(i as isize)).tags),
                                    ICNT_6329,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                let ref mut fresh26 = (*sectors.offset(s as isize))
                                    .gravityptr;
                                *fresh26 = &mut (*sectors.offset(sec as isize)).floorheight;
                                if (*lines.offset(i as isize)).flags as libc::c_int
                                    & 64 as libc::c_int != 0
                                {
                                    let ref mut fresh27 = (*sectors.offset(s as isize)).flags;
                                    *fresh27 = ::core::mem::transmute::<
                                        libc::c_uint,
                                        sectorflags_t,
                                    >(
                                        *fresh27 as libc::c_uint
                                            | MSF_GRAVITYFLIP as libc::c_int as libc::c_uint,
                                    );
                                } else {
                                    let ref mut fresh28 = (*sectors.offset(s as isize)).flags;
                                    *fresh28 = ::core::mem::transmute::<
                                        libc::c_uint,
                                        sectorflags_t,
                                    >(
                                        *fresh28 as libc::c_uint
                                            & !(MSF_GRAVITYFLIP as libc::c_int) as libc::c_uint,
                                    );
                                }
                                if (*lines.offset(i as isize)).flags as libc::c_int
                                    & 8192 as libc::c_int != 0
                                {
                                    let ref mut fresh29 = (*sectors.offset(s as isize))
                                        .specialflags;
                                    *fresh29 = ::core::mem::transmute::<
                                        libc::c_uint,
                                        sectorspecialflags_t,
                                    >(
                                        *fresh29 as libc::c_uint
                                            | SSF_GRAVITYOVERRIDE as libc::c_int as libc::c_uint,
                                    );
                                }
                                CheckForReverseGravity = (CheckForReverseGravity
                                    as libc::c_uint
                                    | (*sectors.offset(s as isize)).flags as libc::c_uint
                                        & MSF_GRAVITYFLIP as libc::c_int as libc::c_uint)
                                    as boolean;
                                ICNT_6329 = ICNT_6329.wrapping_add(1);
                                ICNT_6329;
                            }
                            current_block_382 = 15849589987095405551;
                        }
                    }
                    5 => {
                        if udmf != 0 {
                            current_block_382 = 15849589987095405551;
                        } else {
                            sec = ((*sides
                                .offset(
                                    *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                                ))
                                .sector)
                                .offset_from(sectors) as libc::c_long as size_t;
                            let mut ICNT_6350: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Sectors(
                                    Tag_FGet(&mut (*lines.offset(i as isize)).tags),
                                    ICNT_6350,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                P_AddCameraScanner(
                                    &mut *sectors.offset(sec as isize),
                                    &mut *sectors.offset(s as isize),
                                    R_PointToAngle2(
                                        (*(*lines.offset(i as isize)).v2).x,
                                        (*(*lines.offset(i as isize)).v2).y,
                                        (*(*lines.offset(i as isize)).v1).x,
                                        (*(*lines.offset(i as isize)).v1).y,
                                    ),
                                );
                                ICNT_6350 = ICNT_6350.wrapping_add(1);
                                ICNT_6350;
                            }
                            current_block_382 = 15849589987095405551;
                        }
                    }
                    7 => {
                        let mut flatangle: angle_t = InvAngle(
                            R_PointToAngle2(
                                (*(*lines.offset(i as isize)).v1).x,
                                (*(*lines.offset(i as isize)).v1).y,
                                (*(*lines.offset(i as isize)).v2).x,
                                (*(*lines.offset(i as isize)).v2).y,
                            ),
                        );
                        let mut xoffs: fixed_t = -(*(*lines.offset(i as isize)).v1).x;
                        let mut yoffs: fixed_t = (*(*lines.offset(i as isize)).v1).y;
                        if (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                            == 0 as libc::c_int
                        {
                            P_ApplyFlatAlignment(
                                (*lines.offset(i as isize)).frontsector,
                                flatangle,
                                xoffs,
                                yoffs,
                                ((*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                    != TMP_CEILING as libc::c_int) as libc::c_int,
                                ((*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                    != TMP_FLOOR as libc::c_int) as libc::c_int,
                            );
                        } else {
                            let mut ICNT_6366: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Sectors(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6366,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                P_ApplyFlatAlignment(
                                    sectors.offset(s as isize),
                                    flatangle,
                                    xoffs,
                                    yoffs,
                                    ((*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                        != TMP_CEILING as libc::c_int) as libc::c_int,
                                    ((*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                        != TMP_FLOOR as libc::c_int) as libc::c_int,
                                );
                                ICNT_6366 = ICNT_6366.wrapping_add(1);
                                ICNT_6366;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    8 => {
                        if !((*lines.offset(i as isize)).frontsector).is_null() {
                            let mut ICNT_6374: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Sectors(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6374,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                (*sectors.offset(s as isize))
                                    .camsec = ((*lines.offset(i as isize)).frontsector)
                                    .offset_from(sectors) as libc::c_long as int32_t;
                                ICNT_6374 = ICNT_6374.wrapping_add(1);
                                ICNT_6374;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    10 => {
                        let mut ICNT_6379: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6379,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            let ref mut fresh30 = (*sectors.offset(s as isize))
                                .cullheight;
                            *fresh30 = &mut *lines.offset(i as isize) as *mut line_t;
                            ICNT_6379 = ICNT_6379.wrapping_add(1);
                            ICNT_6379;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    50 => {
                        if udmf == 0 {
                            EV_DoFloor(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                &mut *lines.offset(i as isize),
                                instantLower,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    51 => {
                        if udmf == 0 {
                            EV_DoCeiling(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                &mut *lines.offset(i as isize),
                                instantRaise,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    52 => {
                        EV_DoContinuousFall(
                            (*lines.offset(i as isize)).frontsector,
                            (*lines.offset(i as isize)).backsector,
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                << 16 as libc::c_int,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    53 => {
                        if !((*lines.offset(i as isize)).backsector).is_null() {
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize]
                                != TMP_CEILING as libc::c_int
                            {
                                EV_DoFloor(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    &mut *lines.offset(i as isize),
                                    bounceFloor,
                                );
                            }
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int
                            {
                                EV_DoCeiling(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    &mut *lines.offset(i as isize),
                                    bounceCeiling,
                                );
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    56 => {
                        if !((*lines.offset(i as isize)).backsector).is_null() {
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize]
                                != TMP_CEILING as libc::c_int
                            {
                                EV_DoFloor(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    &mut *lines.offset(i as isize),
                                    bounceFloorCrush,
                                );
                            }
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int
                            {
                                EV_DoCeiling(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    &mut *lines.offset(i as isize),
                                    bounceCeilingCrush,
                                );
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    60 => {
                        EV_DoElevator(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            &mut *lines.offset(i as isize),
                            elevateContinuous,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    61 => {
                        EV_DoCrush(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            &mut *lines.offset(i as isize),
                            (if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] != 0
                            {
                                raiseAndCrush as libc::c_int
                            } else {
                                crushAndRaise as libc::c_int
                            }) as ceiling_e,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    63 => {
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        let mut ICNT_6427: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6427,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            (*sectors.offset(s as isize)).heightsec = sec as int32_t;
                            ICNT_6427 = ICNT_6427.wrapping_add(1);
                            ICNT_6427;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    64 => {
                        if (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                            == 0 as libc::c_int
                        {
                            let mut ICNT_6434: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Sectors(
                                    (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6434,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                j = 0 as libc::c_int;
                                while (j as libc::c_uint as size_t)
                                    < (*sectors.offset(s as isize)).linecount
                                {
                                    if !(((**((*sectors.offset(s as isize)).lines)
                                        .offset(j as isize))
                                        .special as libc::c_int) < 100 as libc::c_int
                                        || (**((*sectors.offset(s as isize)).lines)
                                            .offset(j as isize))
                                            .special as libc::c_int >= 300 as libc::c_int)
                                    {
                                        Add_MasterDisappearer(
                                            abs(
                                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                            ) as tic_t,
                                            abs(
                                                (*lines.offset(i as isize)).args[3 as libc::c_int as usize],
                                            ) as tic_t,
                                            abs(
                                                (*lines.offset(i as isize)).args[4 as libc::c_int as usize],
                                            ) as tic_t,
                                            (*((*sectors.offset(s as isize)).lines).offset(j as isize))
                                                .offset_from(lines) as libc::c_long as int32_t,
                                            i as int32_t,
                                        );
                                    }
                                    j += 1;
                                    j;
                                }
                                ICNT_6434 = ICNT_6434.wrapping_add(1);
                                ICNT_6434;
                            }
                        } else {
                            let mut ICNT_6447: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s = Tag_Iterate_Lines(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6447,
                                );
                                if !(s >= 0 as libc::c_int) {
                                    break;
                                }
                                if !(((*lines.offset(s as isize)).special as libc::c_int)
                                    < 100 as libc::c_int
                                    || (*lines.offset(s as isize)).special as libc::c_int
                                        >= 300 as libc::c_int)
                                {
                                    if !((*lines.offset(i as isize))
                                        .args[1 as libc::c_int as usize] != 0 as libc::c_int
                                        && Tag_Find(
                                            &mut (*(*lines.offset(s as isize)).frontsector).tags,
                                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                                as mtag_t,
                                        ) == 0)
                                    {
                                        Add_MasterDisappearer(
                                            abs(
                                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                            ) as tic_t,
                                            abs(
                                                (*lines.offset(i as isize)).args[3 as libc::c_int as usize],
                                            ) as tic_t,
                                            abs(
                                                (*lines.offset(i as isize)).args[4 as libc::c_int as usize],
                                            ) as tic_t,
                                            s,
                                            i as int32_t,
                                        );
                                    }
                                }
                                ICNT_6447 = ICNT_6447.wrapping_add(1);
                                ICNT_6447;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    66 => {
                        let mut ICNT_6461: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6461,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            P_AddPlaneDisplaceThinker(
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                abs(
                                    (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                ) << 8 as libc::c_int,
                                ((*sides
                                    .offset(
                                        (*lines.offset(i as isize))
                                            .sidenum[0 as libc::c_int as usize] as isize,
                                    ))
                                    .sector)
                                    .offset_from(sectors) as libc::c_long as int32_t,
                                s,
                                ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                    < 0 as libc::c_int) as libc::c_int as uint8_t,
                            );
                            ICNT_6461 = ICNT_6461.wrapping_add(1);
                            ICNT_6461;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    70 => {
                        if udmf != 0 {
                            let mut destheight: fixed_t = (*lines.offset(i as isize))
                                .args[2 as libc::c_int as usize] << 16 as libc::c_int;
                            let mut startheight: fixed_t = 0;
                            let mut topheight: fixed_t = 0;
                            let mut bottomheight: fixed_t = 0;
                            let mut ICNT_6471: size_t = 0 as libc::c_int as size_t;
                            loop {
                                l = Tag_Iterate_Lines(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6471,
                                );
                                if !(l >= 0 as libc::c_int) {
                                    break;
                                }
                                if !(((*lines.offset(l as isize)).special as libc::c_int)
                                    < 100 as libc::c_int
                                    || (*lines.offset(l as isize)).special as libc::c_int
                                        >= 300 as libc::c_int)
                                {
                                    startheight = (*(*lines.offset(l as isize)).frontsector)
                                        .ceilingheight;
                                    topheight = if startheight > destheight {
                                        startheight
                                    } else {
                                        destheight
                                    };
                                    bottomheight = if startheight < destheight {
                                        startheight
                                    } else {
                                        destheight
                                    };
                                    P_AddRaiseThinker(
                                        (*lines.offset(l as isize)).frontsector,
                                        (*lines.offset(l as isize)).args[0 as libc::c_int as usize]
                                            as int16_t,
                                        (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                            << 16 as libc::c_int,
                                        topheight,
                                        bottomheight,
                                        (destheight < startheight) as libc::c_int,
                                        ((*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                                            != 0) as libc::c_int,
                                    );
                                }
                                ICNT_6471 = ICNT_6471.wrapping_add(1);
                                ICNT_6471;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    71 => {
                        if udmf != 0 {
                            let mut ICNT_6488: size_t = 0 as libc::c_int as size_t;
                            loop {
                                l = Tag_Iterate_Lines(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6488,
                                );
                                if !(l >= 0 as libc::c_int) {
                                    break;
                                }
                                if !(((*lines.offset(l as isize)).special as libc::c_int)
                                    < 100 as libc::c_int
                                    || (*lines.offset(l as isize)).special as libc::c_int
                                        >= 300 as libc::c_int)
                                {
                                    P_AddAirbob(
                                        (*lines.offset(l as isize)).frontsector,
                                        (*lines.offset(l as isize)).args[0 as libc::c_int as usize]
                                            as int16_t,
                                        (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                            << 16 as libc::c_int,
                                        ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                            & TMFB_REVERSE as libc::c_int != 0) as libc::c_int,
                                        ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                            & TMFB_SPINDASH as libc::c_int != 0) as libc::c_int,
                                        ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                            & TMFB_DYNAMIC as libc::c_int != 0) as libc::c_int,
                                    );
                                }
                                ICNT_6488 = ICNT_6488.wrapping_add(1);
                                ICNT_6488;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    72 => {
                        if udmf != 0 {
                            let mut sound: uint16_t = (if !((*lines.offset(i as isize))
                                .stringargs[0 as libc::c_int as usize])
                                .is_null()
                            {
                                get_number(
                                    (*lines.offset(i as isize))
                                        .stringargs[0 as libc::c_int as usize],
                                )
                            } else {
                                sfx_thwomp as libc::c_int
                            }) as uint16_t;
                            let mut ICNT_6503: size_t = 0 as libc::c_int as size_t;
                            loop {
                                l = Tag_Iterate_Lines(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6503,
                                );
                                if !(l >= 0 as libc::c_int) {
                                    break;
                                }
                                if !(((*lines.offset(l as isize)).special as libc::c_int)
                                    < 100 as libc::c_int
                                    || (*lines.offset(l as isize)).special as libc::c_int
                                        >= 300 as libc::c_int)
                                {
                                    P_AddThwompThinker(
                                        (*lines.offset(l as isize)).frontsector,
                                        &mut *lines.offset(l as isize),
                                        (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                            << 16 as libc::c_int - 3 as libc::c_int,
                                        (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                            << 16 as libc::c_int - 3 as libc::c_int,
                                        sound,
                                    );
                                }
                                ICNT_6503 = ICNT_6503.wrapping_add(1);
                                ICNT_6503;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    73 => {
                        if udmf != 0 {
                            let mut ICNT_6516: size_t = 0 as libc::c_int as size_t;
                            loop {
                                l = Tag_Iterate_Lines(
                                    (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                        as mtag_t,
                                    ICNT_6516,
                                );
                                if !(l >= 0 as libc::c_int) {
                                    break;
                                }
                                if !(((*lines.offset(l as isize)).special as libc::c_int)
                                    < 100 as libc::c_int
                                    || (*lines.offset(l as isize)).special as libc::c_int
                                        >= 300 as libc::c_int)
                                {
                                    P_AddLaserThinker(
                                        (*lines.offset(l as isize)).args[0 as libc::c_int as usize]
                                            as int16_t,
                                        lines.offset(l as isize),
                                        ((*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                            != 0) as libc::c_int,
                                    );
                                }
                                ICNT_6516 = ICNT_6516.wrapping_add(1);
                                ICNT_6516;
                            }
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    100 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_SOLID as libc::c_int | FOF_RENDERALL as libc::c_int)
                            as ffloortype_e;
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOPLANES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERPLANES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSIDES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERSIDES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_INSIDES as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_BOTHPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_ALLSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_ONLYINSIDES as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSHADE as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_NOSHADE as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_INTANGIBLETOP as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_REVERSEPLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_INTANGIBLEBOTTOM as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_PLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_DONTBLOCKPLAYER as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKPLAYER as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_DONTBLOCKOTHERS as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKOTHERS as libc::c_int) as libc::c_uint,
                            );
                        }
                        if ffloorflags as libc::c_uint
                            & FOF_RENDERALL as libc::c_int as libc::c_uint != 0
                        {
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] < 255 as libc::c_int
                                || (*lines.offset(i as isize))
                                    .args[3 as libc::c_int as usize] & TMFA_SPLAT as libc::c_int
                                    != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | (FOF_CUTEXTRA as libc::c_int | FOF_EXTRA as libc::c_int)
                                            as libc::c_uint,
                                );
                            } else if (*lines.offset(i as isize))
                                .args[4 as libc::c_int as usize]
                                & TMFT_VISIBLEFROMINSIDE as libc::c_int == 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_CUTLEVEL as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    120 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_RENDERPLANES as libc::c_int
                            | FOF_SWIMMABLE as libc::c_int
                            | FOF_BOTHPLANES as libc::c_int | FOF_CUTEXTRA as libc::c_int
                            | FOF_EXTRA as libc::c_int | FOF_CUTSPRITES as libc::c_int)
                            as ffloortype_e;
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_NOSIDES as libc::c_int == 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | (FOF_RENDERSIDES as libc::c_int
                                        | FOF_ALLSIDES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_DOUBLESHADOW as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_DOUBLESHADOW as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_COLORMAPONLY as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_COLORMAPONLY as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_NORIPPLE as libc::c_int == 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_RIPPLE as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_GOOWATER as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_GOOWATER as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFW_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    150 => {
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            (FOF_EXISTS as libc::c_int | FOF_SOLID as libc::c_int
                                | FOF_RENDERALL as libc::c_int) as ffloortype_e,
                            secthinkers,
                        );
                        P_AddAirbob(
                            (*lines.offset(i as isize)).frontsector,
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as int16_t,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                << 16 as libc::c_int,
                            ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                & TMFB_REVERSE as libc::c_int != 0) as libc::c_int,
                            ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                & TMFB_SPINDASH as libc::c_int != 0) as libc::c_int,
                            ((*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                & TMFB_DYNAMIC as libc::c_int != 0) as libc::c_int,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    160 => {
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            (FOF_EXISTS as libc::c_int | FOF_SOLID as libc::c_int
                                | FOF_RENDERALL as libc::c_int
                                | FOF_FLOATBOB as libc::c_int) as ffloortype_e,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    170 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_SOLID as libc::c_int | FOF_RENDERALL as libc::c_int
                            | FOF_CRUMBLE as libc::c_int) as ffloortype_e;
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFT_INTANGIBLETOP as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_REVERSEPLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFT_INTANGIBLEBOTTOM as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_PLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFT_DONTBLOCKPLAYER as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKPLAYER as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFT_DONTBLOCKOTHERS as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKOTHERS as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFC_NOSHADE as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_NOSHADE as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFC_NORETURN as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_NORETURN as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFC_FLOATBOB as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_FLOATBOB as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFC_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                            < 255 as libc::c_int
                            || (*lines.offset(i as isize))
                                .args[4 as libc::c_int as usize] & TMFC_SPLAT as libc::c_int
                                != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | (FOF_CUTEXTRA as libc::c_int | FOF_EXTRA as libc::c_int)
                                        as libc::c_uint,
                            );
                        } else if (*lines.offset(i as isize))
                            .args[3 as libc::c_int as usize]
                            & TMFT_VISIBLEFROMINSIDE as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_BOTHPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_ALLSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        } else {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_CUTLEVEL as libc::c_int as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFC_AIRBOB as libc::c_int != 0
                        {
                            P_AddAirbob(
                                (*lines.offset(i as isize)).frontsector,
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as int16_t,
                                16 as libc::c_int
                                    * ((1 as libc::c_int) << 16 as libc::c_int),
                                false_0 as libc::c_int,
                                false_0 as libc::c_int,
                                false_0 as libc::c_int,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    190 => {
                        let mut ceilingtop: fixed_t = P_FindHighestCeilingSurrounding(
                            (*lines.offset(i as isize)).frontsector,
                        );
                        let mut ceilingbottom: fixed_t = P_FindLowestCeilingSurrounding(
                            (*lines.offset(i as isize)).frontsector,
                        );
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_SOLID as libc::c_int | FOF_RENDERALL as libc::c_int)
                            as ffloortype_e;
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOPLANES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERPLANES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSIDES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERSIDES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_INSIDES as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_BOTHPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_ALLSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_ONLYINSIDES as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSHADE as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_NOSHADE as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_INTANGIBLETOP as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_REVERSEPLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_INTANGIBLEBOTTOM as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_PLATFORM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_DONTBLOCKPLAYER as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKPLAYER as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFT_DONTBLOCKOTHERS as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_BLOCKOTHERS as libc::c_int) as libc::c_uint,
                            );
                        }
                        if ffloorflags as libc::c_uint
                            & FOF_RENDERALL as libc::c_int as libc::c_uint != 0
                        {
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] < 255 as libc::c_int
                                || (*lines.offset(i as isize))
                                    .args[3 as libc::c_int as usize] & TMFA_SPLAT as libc::c_int
                                    != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | (FOF_CUTEXTRA as libc::c_int | FOF_EXTRA as libc::c_int)
                                            as libc::c_uint,
                                );
                            } else if (*lines.offset(i as isize))
                                .args[4 as libc::c_int as usize]
                                & TMFT_VISIBLEFROMINSIDE as libc::c_int == 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_CUTLEVEL as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        P_AddRaiseThinker(
                            (*lines.offset(i as isize)).frontsector,
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as int16_t,
                            (*lines.offset(i as isize)).args[5 as libc::c_int as usize]
                                << 16 as libc::c_int,
                            ceilingtop,
                            ceilingbottom,
                            ((*lines.offset(i as isize)).args[6 as libc::c_int as usize]
                                & TMFR_REVERSE as libc::c_int != 0) as libc::c_int,
                            ((*lines.offset(i as isize)).args[6 as libc::c_int as usize]
                                & TMFR_SPINDASH as libc::c_int != 0) as libc::c_int,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    200 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_CUTSPRITES as libc::c_int) as ffloortype_e;
                        if (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                            == 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_DOUBLESHADOW as libc::c_int as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    202 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_RENDERALL as libc::c_int | FOF_FOG as libc::c_int
                            | FOF_INVERTPLANES as libc::c_int
                            | FOF_INVERTSIDES as libc::c_int
                            | FOF_CUTEXTRA as libc::c_int | FOF_EXTRA as libc::c_int
                            | FOF_DOUBLESHADOW as libc::c_int
                            | FOF_CUTSPRITES as libc::c_int) as ffloortype_e;
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        if !((*sectors.offset(sec as isize)).extra_colormap).is_null() {
                            (*(*sectors.offset(sec as isize)).extra_colormap)
                                .flags = 4 as libc::c_int as uint8_t;
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    220 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_RENDERALL as libc::c_int | FOF_CUTEXTRA as libc::c_int
                            | FOF_EXTRA as libc::c_int | FOF_CUTSPRITES as libc::c_int)
                            as ffloortype_e;
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOPLANES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERPLANES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSIDES as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_RENDERSIDES as libc::c_int) as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_INSIDES as libc::c_int == 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_BOTHPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_ALLSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_ONLYINSIDES as libc::c_int != 0
                        {
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERPLANES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTPLANES as libc::c_int as libc::c_uint,
                                );
                            }
                            if ffloorflags as libc::c_uint
                                & FOF_RENDERSIDES as libc::c_int as libc::c_uint != 0
                            {
                                ffloorflags = ::core::mem::transmute::<
                                    libc::c_uint,
                                    ffloortype_e,
                                >(
                                    ffloorflags as libc::c_uint
                                        | FOF_INVERTSIDES as libc::c_int as libc::c_uint,
                                );
                            }
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_NOSHADE as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_NOSHADE as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFA_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    223 => {
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            (FOF_EXISTS as libc::c_int | FOF_NOSHADE as libc::c_int)
                                as ffloortype_e,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    250 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_SOLID as libc::c_int | FOF_RENDERALL as libc::c_int
                            | FOF_CUTLEVEL as libc::c_int | FOF_MARIO as libc::c_int)
                            as ffloortype_e;
                        if (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                            & TMFM_BRICK as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_GOOWATER as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                            & TMFM_INVISIBLE as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    & !(FOF_SOLID as libc::c_int | FOF_RENDERALL as libc::c_int
                                        | FOF_CUTLEVEL as libc::c_int) as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    251 => {
                        let mut sound_0: uint16_t = (if !((*lines.offset(i as isize))
                            .stringargs[0 as libc::c_int as usize])
                            .is_null()
                        {
                            get_number(
                                (*lines.offset(i as isize))
                                    .stringargs[0 as libc::c_int as usize],
                            )
                        } else {
                            sfx_thwomp as libc::c_int
                        }) as uint16_t;
                        P_AddThwompThinker(
                            (*lines.offset(i as isize)).frontsector,
                            &mut *lines.offset(i as isize),
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                << 16 as libc::c_int - 3 as libc::c_int,
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                << 16 as libc::c_int - 3 as libc::c_int,
                            sound_0,
                        );
                        P_AddFakeFloorsByLine(
                            i,
                            0xff as libc::c_int,
                            TMB_TRANSLUCENT as libc::c_int as uint8_t,
                            (FOF_EXISTS as libc::c_int | FOF_SOLID as libc::c_int
                                | FOF_RENDERALL as libc::c_int
                                | FOF_CUTLEVEL as libc::c_int) as ffloortype_e,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    254 => {
                        let mut busttype: uint8_t = BT_REGULAR as libc::c_int as uint8_t;
                        let mut bustflags: ffloorbustflags_e = 0 as ffloorbustflags_e;
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_BLOCKOTHERS as libc::c_int
                            | FOF_RENDERALL as libc::c_int | FOF_BUSTUP as libc::c_int)
                            as ffloortype_e;
                        match (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                        {
                            0 => {
                                busttype = BT_TOUCH as libc::c_int as uint8_t;
                            }
                            1 => {
                                busttype = BT_SPINBUST as libc::c_int as uint8_t;
                            }
                            2 => {
                                busttype = BT_REGULAR as libc::c_int as uint8_t;
                            }
                            3 => {
                                busttype = BT_STRONG as libc::c_int as uint8_t;
                            }
                            _ => {}
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFB_PUSHABLES as libc::c_int != 0
                        {
                            bustflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloorbustflags_e,
                            >(
                                bustflags as libc::c_uint
                                    | FB_PUSHABLES as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFB_EXECUTOR as libc::c_int != 0
                        {
                            bustflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloorbustflags_e,
                            >(
                                bustflags as libc::c_uint
                                    | FB_EXECUTOR as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFB_ONLYBOTTOM as libc::c_int != 0
                        {
                            bustflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloorbustflags_e,
                            >(
                                bustflags as libc::c_uint
                                    | FB_ONLYBOTTOM as libc::c_int as libc::c_uint,
                            );
                        }
                        if (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                            & TMFB_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        if busttype as libc::c_int != BT_TOUCH as libc::c_int
                            || bustflags as libc::c_uint
                                & FB_ONLYBOTTOM as libc::c_int as libc::c_uint != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_BLOCKPLAYER as libc::c_int as libc::c_uint,
                            );
                        }
                        let mut ICNT_6804: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6804,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            let mut fflr: *mut ffloor_t = P_AddFakeFloor(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).frontsector,
                                lines.offset(i as isize),
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                    as uint8_t,
                                ffloorflags,
                                secthinkers,
                            );
                            if !fflr.is_null() {
                                (*fflr).bustflags = bustflags;
                                (*fflr).busttype = busttype;
                                (*fflr)
                                    .busttag = (*lines.offset(i as isize))
                                    .args[5 as libc::c_int as usize] as int16_t;
                            }
                            ICNT_6804 = ICNT_6804.wrapping_add(1);
                            ICNT_6804;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    257 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_QUICKSAND as libc::c_int | FOF_RENDERALL as libc::c_int
                            | FOF_ALLSIDES as libc::c_int
                            | FOF_CUTSPRITES as libc::c_int) as ffloortype_e;
                        if (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                            == 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_RIPPLE as libc::c_int as libc::c_uint,
                            );
                        }
                        let mut ICNT_6820: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6820,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            let mut fflr_0: *mut ffloor_t = P_AddFakeFloor(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).frontsector,
                                lines.offset(i as isize),
                                0xff as libc::c_int,
                                TMB_TRANSLUCENT as libc::c_int as uint8_t,
                                ffloorflags,
                                secthinkers,
                            );
                            if !fflr_0.is_null() {
                                (*fflr_0)
                                    .sinkspeed = abs(
                                    (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                ) << 16 as libc::c_int - 1 as libc::c_int;
                                (*fflr_0)
                                    .friction = abs(
                                    (*lines.offset(i as isize)).args[3 as libc::c_int as usize],
                                ) << 16 as libc::c_int - 6 as libc::c_int;
                            }
                            ICNT_6820 = ICNT_6820.wrapping_add(1);
                            ICNT_6820;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    258 => {
                        ffloorflags = (FOF_EXISTS as libc::c_int
                            | FOF_RENDERALL as libc::c_int | FOF_NOSHADE as libc::c_int
                            | FOF_EXTRA as libc::c_int | FOF_CUTEXTRA as libc::c_int
                            | FOF_TRANSLUCENT as libc::c_int) as ffloortype_e;
                        P_AddLaserThinker(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as int16_t,
                            lines.offset(i as isize),
                            ((*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                                & TMFL_NOBOSSES as libc::c_int != 0) as libc::c_int,
                        );
                        if (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                            & TMFL_SPLAT as libc::c_int != 0
                        {
                            ffloorflags = ::core::mem::transmute::<
                                libc::c_uint,
                                ffloortype_e,
                            >(
                                ffloorflags as libc::c_uint
                                    | FOF_SPLAT as libc::c_int as libc::c_uint,
                            );
                        }
                        P_AddFakeFloorsByLine(
                            i,
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                as uint8_t,
                            ffloorflags,
                            secthinkers,
                        );
                        current_block_382 = 15849589987095405551;
                    }
                    259 => {
                        let mut ICNT_6839: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6839,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            let mut fflr_1: *mut ffloor_t = P_AddFakeFloor(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).frontsector,
                                lines.offset(i as isize),
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                    as uint8_t,
                                (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                                    as ffloortype_e,
                                secthinkers,
                            );
                            if !fflr_1.is_null() {
                                if udmf == 0 {
                                    if (*lines.offset(i as isize))
                                        .args[3 as libc::c_int as usize]
                                        & FOF_QUICKSAND as libc::c_int != 0
                                    {
                                        (*fflr_1)
                                            .sinkspeed = abs((*lines.offset(i as isize)).dx)
                                            >> 1 as libc::c_int;
                                        (*fflr_1)
                                            .friction = abs((*lines.offset(i as isize)).dy)
                                            >> 6 as libc::c_int;
                                    }
                                    if (*lines.offset(i as isize))
                                        .args[3 as libc::c_int as usize] & FOF_BUSTUP as libc::c_int
                                        != 0
                                    {
                                        match (*lines.offset(i as isize))
                                            .args[4 as libc::c_int as usize]
                                            % TMFB_ONLYBOTTOM as libc::c_int
                                        {
                                            0 => {
                                                (*fflr_1).busttype = BT_TOUCH as libc::c_int as uint8_t;
                                            }
                                            1 => {
                                                (*fflr_1).busttype = BT_SPINBUST as libc::c_int as uint8_t;
                                            }
                                            2 => {
                                                (*fflr_1).busttype = BT_REGULAR as libc::c_int as uint8_t;
                                            }
                                            3 => {
                                                (*fflr_1).busttype = BT_STRONG as libc::c_int as uint8_t;
                                            }
                                            _ => {}
                                        }
                                        if (*lines.offset(i as isize))
                                            .args[4 as libc::c_int as usize]
                                            & TMFB_ONLYBOTTOM as libc::c_int != 0
                                        {
                                            (*fflr_1)
                                                .bustflags = ::core::mem::transmute::<
                                                libc::c_uint,
                                                ffloorbustflags_e,
                                            >(
                                                (*fflr_1).bustflags as libc::c_uint
                                                    | FB_ONLYBOTTOM as libc::c_int as libc::c_uint,
                                            );
                                        }
                                        if (*lines.offset(i as isize)).flags as libc::c_int
                                            & 512 as libc::c_int != 0
                                        {
                                            (*fflr_1)
                                                .bustflags = ::core::mem::transmute::<
                                                libc::c_uint,
                                                ffloorbustflags_e,
                                            >(
                                                (*fflr_1).bustflags as libc::c_uint
                                                    | FB_PUSHABLES as libc::c_int as libc::c_uint,
                                            );
                                        }
                                        if (*lines.offset(i as isize)).flags as libc::c_int
                                            & 1024 as libc::c_int != 0
                                        {
                                            (*fflr_1)
                                                .bustflags = ::core::mem::transmute::<
                                                libc::c_uint,
                                                ffloorbustflags_e,
                                            >(
                                                (*fflr_1).bustflags as libc::c_uint
                                                    | FB_EXECUTOR as libc::c_int as libc::c_uint,
                                            );
                                            (*fflr_1)
                                                .busttag = (P_AproxDistance(
                                                (*lines.offset(i as isize)).dx,
                                                (*lines.offset(i as isize)).dy,
                                            ) >> 16 as libc::c_int) as int16_t;
                                        }
                                    }
                                }
                            }
                            ICNT_6839 = ICNT_6839.wrapping_add(1);
                            ICNT_6839;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    300 => {
                        current_block_382 = 15233050716034445302;
                    }
                    303 => {
                        current_block_382 = 15233050716034445302;
                    }
                    305 => {
                        current_block_382 = 9297118320085539986;
                    }
                    314 => {
                        current_block_382 = 12744522843097846142;
                    }
                    317 => {
                        current_block_382 = 10575870603838195425;
                    }
                    319 => {
                        current_block_382 = 15471672177304102772;
                    }
                    331 => {
                        current_block_382 = 8898299451911775310;
                    }
                    334 => {
                        current_block_382 = 17343443802423163925;
                    }
                    337 | 343 => {
                        current_block_382 = 7096050567950060764;
                    }
                    308 => {
                        if P_CheckGametypeRules(
                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                as uint32_t,
                        ) == 0
                        {
                            (*lines.offset(i as isize))
                                .special = 0 as libc::c_int as int16_t;
                        } else if (*lines.offset(i as isize))
                            .args[0 as libc::c_int as usize]
                            > TMT_EACHTIMEMASK as libc::c_int
                        {
                            P_AddEachTimeThinker(
                                &mut *lines.offset(i as isize),
                                ((*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    == TMT_EACHTIMEENTERANDEXIT as libc::c_int) as libc::c_int,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    309 => {
                        if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t == 0
                        {
                            (*lines.offset(i as isize))
                                .special = 0 as libc::c_int as int16_t;
                        } else if (*lines.offset(i as isize))
                            .args[0 as libc::c_int as usize]
                            > TMT_EACHTIMEMASK as libc::c_int
                        {
                            P_AddEachTimeThinker(
                                &mut *lines.offset(i as isize),
                                ((*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    == TMT_EACHTIMEENTERANDEXIT as libc::c_int) as libc::c_int,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    313 => {
                        P_AddNoEnemiesThinker(&mut *lines.offset(i as isize));
                        current_block_382 = 15849589987095405551;
                    }
                    321 => {
                        (*lines.offset(i as isize))
                            .callcount = (if (*lines.offset(i as isize))
                            .args[2 as libc::c_int as usize] != 0
                            && (*lines.offset(i as isize))
                                .args[3 as libc::c_int as usize] > 0 as libc::c_int
                        {
                            (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                        } else {
                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                        }) as int16_t;
                        if (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                            > TMXT_EACHTIMEMASK as libc::c_int
                        {
                            P_AddEachTimeThinker(
                                &mut *lines.offset(i as isize),
                                ((*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    == TMXT_EACHTIMEENTERANDEXIT as libc::c_int) as libc::c_int,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    449 => {
                        let mut bossid: int32_t = (*lines.offset(i as isize))
                            .args[0 as libc::c_int as usize];
                        if bossid & !(15 as libc::c_int) != 0 {
                            CONS_Alert(
                                CONS_WARNING,
                                b"Boss enable linedef has an invalid boss ID (%d).\nConsider changing it or removing it entirely.\n\0"
                                    as *const u8 as *const libc::c_char,
                                bossid,
                            );
                        } else if (*lines.offset(i as isize))
                            .args[1 as libc::c_int as usize] == 0
                        {
                            bossdisabled = (bossdisabled as libc::c_int
                                | (1 as libc::c_int) << bossid) as uint16_t;
                            CONS_Debug(
                                0x80 as libc::c_int,
                                b"Line type 449 spawn effect: bossid disabled = %d\0"
                                    as *const u8 as *const libc::c_char,
                                bossid,
                            );
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    600 => {
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        let mut ICNT_6950: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6950,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize]
                                != TMP_CEILING as libc::c_int
                            {
                                (*sectors.offset(s as isize))
                                    .floorlightsec = sec as int32_t;
                            }
                            if (*lines.offset(i as isize))
                                .args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int
                            {
                                (*sectors.offset(s as isize))
                                    .ceilinglightsec = sec as int32_t;
                            }
                            ICNT_6950 = ICNT_6950.wrapping_add(1);
                            ICNT_6950;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    602 => {
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        let mut ICNT_6961: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6961,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            P_SpawnAdjustableGlowingLight(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                    as int16_t,
                                (if (*lines.offset(i as isize))
                                    .args[3 as libc::c_int as usize] != 0
                                {
                                    (*sectors.offset(s as isize)).lightlevel as libc::c_int
                                } else {
                                    (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                                }) as int16_t,
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            );
                            ICNT_6961 = ICNT_6961.wrapping_add(1);
                            ICNT_6961;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    603 => {
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        let mut ICNT_6968: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6968,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            P_SpawnAdjustableFireFlicker(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                                    as int16_t,
                                (if (*lines.offset(i as isize))
                                    .args[3 as libc::c_int as usize] != 0
                                {
                                    (*sectors.offset(s as isize)).lightlevel as libc::c_int
                                } else {
                                    (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                                }) as int16_t,
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                            );
                            ICNT_6968 = ICNT_6968.wrapping_add(1);
                            ICNT_6968;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    604 => {
                        sec = ((*sides
                            .offset(
                                *((*lines.offset(i as isize)).sidenum).as_mut_ptr() as isize,
                            ))
                            .sector)
                            .offset_from(sectors) as libc::c_long as size_t;
                        let mut ICNT_6975: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6975,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            P_SpawnAdjustableStrobeFlash(
                                &mut *sectors.offset(s as isize),
                                (*lines.offset(i as isize)).args[3 as libc::c_int as usize]
                                    as int16_t,
                                (if (*lines.offset(i as isize))
                                    .args[4 as libc::c_int as usize]
                                    & TMB_USETARGET as libc::c_int != 0
                                {
                                    (*sectors.offset(s as isize)).lightlevel as libc::c_int
                                } else {
                                    (*lines.offset(i as isize)).args[5 as libc::c_int as usize]
                                }) as int16_t,
                                (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                (*lines.offset(i as isize)).args[4 as libc::c_int as usize]
                                    & TMB_SYNC as libc::c_int,
                            );
                            ICNT_6975 = ICNT_6975.wrapping_add(1);
                            ICNT_6975;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    606 => {
                        let mut ICNT_6982: size_t = 0 as libc::c_int as size_t;
                        loop {
                            s = Tag_Iterate_Sectors(
                                (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    as mtag_t,
                                ICNT_6982,
                            );
                            if !(s >= 0 as libc::c_int) {
                                break;
                            }
                            let mut exc: *mut extracolormap_t = 0
                                as *mut extracolormap_t;
                            if !((*sectors.offset(s as isize)).colormap_protected != 0) {
                                if udmf == 0 {
                                    exc = (*sides
                                        .offset(
                                            (*lines.offset(i as isize))
                                                .sidenum[0 as libc::c_int as usize] as isize,
                                        ))
                                        .colormap_data;
                                } else if (*lines.offset(i as isize))
                                    .args[1 as libc::c_int as usize] == 0
                                {
                                    exc = (*(*lines.offset(i as isize)).frontsector)
                                        .extra_colormap;
                                } else {
                                    let mut sourcesec: int32_t = Tag_Iterate_Sectors(
                                        (*lines.offset(i as isize)).args[1 as libc::c_int as usize]
                                            as mtag_t,
                                        0 as libc::c_int as size_t,
                                    );
                                    if sourcesec == -(1 as libc::c_int) {
                                        CONS_Debug(
                                            0x80 as libc::c_int,
                                            b"Line type 606: Can't find sector with source colormap (tag %d)!\n\0"
                                                as *const u8 as *const libc::c_char,
                                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                        );
                                        return;
                                    }
                                    exc = (*sectors.offset(sourcesec as isize)).extra_colormap;
                                }
                                let ref mut fresh31 = (*sectors.offset(s as isize))
                                    .spawn_extra_colormap;
                                *fresh31 = exc;
                                let ref mut fresh32 = (*sectors.offset(s as isize))
                                    .extra_colormap;
                                *fresh32 = *fresh31;
                            }
                            ICNT_6982 = ICNT_6982.wrapping_add(1);
                            ICNT_6982;
                        }
                        current_block_382 = 15849589987095405551;
                    }
                    _ => {
                        current_block_382 = 15849589987095405551;
                    }
                }
                match current_block_382 {
                    15233050716034445302 => {
                        current_block_382 = 9297118320085539986;
                    }
                    _ => {}
                }
                match current_block_382 {
                    9297118320085539986 => {
                        current_block_382 = 12744522843097846142;
                    }
                    _ => {}
                }
                match current_block_382 {
                    12744522843097846142 => {
                        current_block_382 = 10575870603838195425;
                    }
                    _ => {}
                }
                match current_block_382 {
                    10575870603838195425 => {
                        current_block_382 = 15471672177304102772;
                    }
                    _ => {}
                }
                match current_block_382 {
                    15471672177304102772 => {
                        current_block_382 = 8898299451911775310;
                    }
                    _ => {}
                }
                match current_block_382 {
                    8898299451911775310 => {
                        current_block_382 = 17343443802423163925;
                    }
                    _ => {}
                }
                match current_block_382 {
                    17343443802423163925 => {
                        current_block_382 = 7096050567950060764;
                    }
                    _ => {}
                }
                match current_block_382 {
                    7096050567950060764 => {
                        if (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                            > TMT_EACHTIMEMASK as libc::c_int
                        {
                            P_AddEachTimeThinker(
                                &mut *lines.offset(i as isize),
                                ((*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                    == TMT_EACHTIMEENTERANDEXIT as libc::c_int) as libc::c_int,
                            );
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        let mut s_0: int32_t = 0;
        let mut l_0: int32_t = 0;
        match (*lines.offset(i as isize)).special as libc::c_int {
            74 => {
                let mut busttype_0: uint8_t = BT_REGULAR as libc::c_int as uint8_t;
                let mut bustflags_0: ffloorbustflags_e = 0 as ffloorbustflags_e;
                if !(udmf == 0) {
                    match (*lines.offset(i as isize)).args[1 as libc::c_int as usize] {
                        0 => {
                            busttype_0 = BT_TOUCH as libc::c_int as uint8_t;
                        }
                        1 => {
                            busttype_0 = BT_SPINBUST as libc::c_int as uint8_t;
                        }
                        2 => {
                            busttype_0 = BT_REGULAR as libc::c_int as uint8_t;
                        }
                        3 => {
                            busttype_0 = BT_STRONG as libc::c_int as uint8_t;
                        }
                        _ => {}
                    }
                    if (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                        & TMFB_PUSHABLES as libc::c_int != 0
                    {
                        bustflags_0 = ::core::mem::transmute::<
                            libc::c_uint,
                            ffloorbustflags_e,
                        >(
                            bustflags_0 as libc::c_uint
                                | FB_PUSHABLES as libc::c_int as libc::c_uint,
                        );
                    }
                    if (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                        & TMFB_EXECUTOR as libc::c_int != 0
                    {
                        bustflags_0 = ::core::mem::transmute::<
                            libc::c_uint,
                            ffloorbustflags_e,
                        >(
                            bustflags_0 as libc::c_uint
                                | FB_EXECUTOR as libc::c_int as libc::c_uint,
                        );
                    }
                    if (*lines.offset(i as isize)).args[2 as libc::c_int as usize]
                        & TMFB_ONLYBOTTOM as libc::c_int != 0
                    {
                        bustflags_0 = ::core::mem::transmute::<
                            libc::c_uint,
                            ffloorbustflags_e,
                        >(
                            bustflags_0 as libc::c_uint
                                | FB_ONLYBOTTOM as libc::c_int as libc::c_uint,
                        );
                    }
                    let mut ICNT_7054: size_t = 0 as libc::c_int as size_t;
                    loop {
                        l_0 = Tag_Iterate_Lines(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            ICNT_7054,
                        );
                        if !(l_0 >= 0 as libc::c_int) {
                            break;
                        }
                        if !(((*lines.offset(l_0 as isize)).special as libc::c_int)
                            < 100 as libc::c_int
                            || (*lines.offset(l_0 as isize)).special as libc::c_int
                                >= 300 as libc::c_int)
                        {
                            let mut ICNT_7059: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s_0 = Tag_Iterate_Sectors(
                                    (*lines.offset(l_0 as isize))
                                        .args[0 as libc::c_int as usize] as mtag_t,
                                    ICNT_7059,
                                );
                                if !(s_0 >= 0 as libc::c_int) {
                                    break;
                                }
                                let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
                                rover = (*sectors.offset(s_0 as isize)).ffloors;
                                while !rover.is_null() {
                                    if (*rover).master != lines.offset(l_0 as isize) {
                                        rover = (*rover).next;
                                    } else {
                                        (*rover)
                                            .fofflags = ::core::mem::transmute::<
                                            libc::c_uint,
                                            ffloortype_e,
                                        >(
                                            (*rover).fofflags as libc::c_uint
                                                | FOF_BUSTUP as libc::c_int as libc::c_uint,
                                        );
                                        (*rover)
                                            .spawnflags = ::core::mem::transmute::<
                                            libc::c_uint,
                                            ffloortype_e,
                                        >(
                                            (*rover).spawnflags as libc::c_uint
                                                | FOF_BUSTUP as libc::c_int as libc::c_uint,
                                        );
                                        (*rover).bustflags = bustflags_0;
                                        (*rover).busttype = busttype_0;
                                        (*rover)
                                            .busttag = (*lines.offset(i as isize))
                                            .args[3 as libc::c_int as usize] as int16_t;
                                        CheckForBustableBlocks = true_0 as libc::c_int;
                                        break;
                                    }
                                }
                                ICNT_7059 = ICNT_7059.wrapping_add(1);
                                ICNT_7059;
                            }
                        }
                        ICNT_7054 = ICNT_7054.wrapping_add(1);
                        ICNT_7054;
                    }
                }
            }
            75 => {
                if !(udmf == 0) {
                    let mut ICNT_7085: size_t = 0 as libc::c_int as size_t;
                    loop {
                        l_0 = Tag_Iterate_Lines(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            ICNT_7085,
                        );
                        if !(l_0 >= 0 as libc::c_int) {
                            break;
                        }
                        if !(((*lines.offset(l_0 as isize)).special as libc::c_int)
                            < 100 as libc::c_int
                            || (*lines.offset(l_0 as isize)).special as libc::c_int
                                >= 300 as libc::c_int)
                        {
                            let mut ICNT_7090: size_t = 0 as libc::c_int as size_t;
                            loop {
                                s_0 = Tag_Iterate_Sectors(
                                    (*lines.offset(l_0 as isize))
                                        .args[0 as libc::c_int as usize] as mtag_t,
                                    ICNT_7090,
                                );
                                if !(s_0 >= 0 as libc::c_int) {
                                    break;
                                }
                                let mut rover_0: *mut ffloor_t = 0 as *mut ffloor_t;
                                rover_0 = (*sectors.offset(s_0 as isize)).ffloors;
                                while !rover_0.is_null() {
                                    if (*rover_0).master != lines.offset(l_0 as isize) {
                                        rover_0 = (*rover_0).next;
                                    } else {
                                        (*rover_0)
                                            .fofflags = ::core::mem::transmute::<
                                            libc::c_uint,
                                            ffloortype_e,
                                        >(
                                            (*rover_0).fofflags as libc::c_uint
                                                | FOF_QUICKSAND as libc::c_int as libc::c_uint,
                                        );
                                        (*rover_0)
                                            .spawnflags = ::core::mem::transmute::<
                                            libc::c_uint,
                                            ffloortype_e,
                                        >(
                                            (*rover_0).spawnflags as libc::c_uint
                                                | FOF_QUICKSAND as libc::c_int as libc::c_uint,
                                        );
                                        (*rover_0)
                                            .sinkspeed = abs(
                                            (*lines.offset(i as isize)).args[1 as libc::c_int as usize],
                                        ) << 16 as libc::c_int - 1 as libc::c_int;
                                        (*rover_0)
                                            .friction = abs(
                                            (*lines.offset(i as isize)).args[2 as libc::c_int as usize],
                                        ) << 16 as libc::c_int - 6 as libc::c_int;
                                        CheckForQuicksand = true_0 as libc::c_int;
                                        break;
                                    }
                                }
                                ICNT_7090 = ICNT_7090.wrapping_add(1);
                                ICNT_7090;
                            }
                        }
                        ICNT_7085 = ICNT_7085.wrapping_add(1);
                        ICNT_7085;
                    }
                }
            }
            76 => {
                if udmf != 0 {
                    let mut ICNT_7115: size_t = 0 as libc::c_int as size_t;
                    loop {
                        l_0 = Tag_Iterate_Lines(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            ICNT_7115,
                        );
                        if !(l_0 >= 0 as libc::c_int) {
                            break;
                        }
                        P_MakeFOFBouncy(
                            lines.offset(i as isize),
                            lines.offset(l_0 as isize),
                        );
                        ICNT_7115 = ICNT_7115.wrapping_add(1);
                        ICNT_7115;
                    }
                } else {
                    let mut ICNT_7120: size_t = 0 as libc::c_int as size_t;
                    loop {
                        s_0 = Tag_Iterate_Sectors(
                            (*lines.offset(i as isize)).args[0 as libc::c_int as usize]
                                as mtag_t,
                            ICNT_7120,
                        );
                        if !(s_0 >= 0 as libc::c_int) {
                            break;
                        }
                        j = 0 as libc::c_int;
                        while (j as libc::c_uint as size_t)
                            < (*sectors.offset(s_0 as isize)).linecount
                        {
                            P_MakeFOFBouncy(
                                lines.offset(i as isize),
                                *((*sectors.offset(s_0 as isize)).lines).offset(j as isize),
                            );
                            j += 1;
                            j;
                        }
                        ICNT_7120 = ICNT_7120.wrapping_add(1);
                        ICNT_7120;
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < numsectors {
        if !((*secthinkers.offset(i as isize)).thinkers).is_null() {
            Z_Free((*secthinkers.offset(i as isize)).thinkers as *mut libc::c_void);
        }
        i = i.wrapping_add(1);
        i;
    }
    Z_Free(secthinkers as *mut libc::c_void);
    Polyobj_InitLevel();
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        match (*lines.offset(i as isize)).special as libc::c_int {
            30 => {
                PolyFlag(&mut *lines.offset(i as isize));
            }
            31 => {
                PolyDisplace(&mut *lines.offset(i as isize));
            }
            32 => {
                PolyRotDisplace(&mut *lines.offset(i as isize));
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
    }
    if fromnetsave == 0 {
        P_RunLevelLoadExecutors();
    }
}
unsafe extern "C" fn P_AddFakeFloorsByLine(
    mut line: size_t,
    mut alpha: int32_t,
    mut blendmode: uint8_t,
    mut ffloorflags: ffloortype_e,
    mut secthinkers: *mut thinkerlist_t,
) {
    let mut s: int32_t = 0;
    let mut tag: mtag_t = (*lines.offset(line as isize)).args[0 as libc::c_int as usize]
        as mtag_t;
    let mut sec: size_t = ((*sides
        .offset(*((*lines.offset(line as isize)).sidenum).as_mut_ptr() as isize))
        .sector)
        .offset_from(sectors) as libc::c_long as size_t;
    let mut li: *mut line_t = lines.offset(line as isize);
    let mut ICNT_7182: size_t = 0 as libc::c_int as size_t;
    loop {
        s = Tag_Iterate_Sectors(tag, ICNT_7182);
        if !(s >= 0 as libc::c_int) {
            break;
        }
        P_AddFakeFloor(
            &mut *sectors.offset(s as isize),
            &mut *sectors.offset(sec as isize),
            li,
            alpha,
            blendmode,
            ffloorflags,
            secthinkers,
        );
        ICNT_7182 = ICNT_7182.wrapping_add(1);
        ICNT_7182;
    };
}
unsafe extern "C" fn P_DoScrollMove(
    mut thing: *mut mobj_t,
    mut dx: fixed_t,
    mut dy: fixed_t,
    mut exclusive: int32_t,
) {
    let mut fuckaj: fixed_t = 0 as libc::c_int;
    if !((*thing).player).is_null() {
        if dx | dy == 0 {
            (*(*thing).player).cmomx = 0 as libc::c_int;
            (*(*thing).player).cmomy = 0 as libc::c_int;
        } else {
            (*(*thing).player).cmomx += dx;
            (*(*thing).player).cmomy += dy;
            (*(*thing).player)
                .cmomx = FixedMul((*(*thing).player).cmomx, 0xe800 as libc::c_int);
            (*(*thing).player)
                .cmomy = FixedMul((*(*thing).player).cmomy, 0xe800 as libc::c_int);
        }
    }
    if !((*thing).player).is_null()
        && (*(*thing).player).pflags as libc::c_uint
            & PF_SPINNING as libc::c_int as libc::c_uint != 0
        && ((*(*thing).player).rmomx != 0 || (*(*thing).player).rmomy != 0)
        && (*(*thing).player).pflags as libc::c_uint
            & PF_STARTDASH as libc::c_int as libc::c_uint == 0
    {
        fuckaj = FixedDiv(
            549 as libc::c_int
                * ((0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int),
            500 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
        );
    } else if (*thing).friction
        != (0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int
    {
        fuckaj = (*thing).friction;
    }
    if fuckaj != 0 {
        dx = FixedDiv(
            dx,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                / 32 as libc::c_int,
        );
        dy = FixedDiv(
            dy,
            3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                / 32 as libc::c_int,
        );
        dx = FixedMul(dx, ((1 as libc::c_int) << 16 as libc::c_int) - fuckaj);
        dy = FixedMul(dy, ((1 as libc::c_int) << 16 as libc::c_int) - fuckaj);
    }
    (*thing).momx += dx;
    (*thing).momy += dy;
    if exclusive != 0 {
        (*thing)
            .eflags = ((*thing).eflags as libc::c_int | MFE_PUSHED as libc::c_int)
            as uint16_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn T_Scroll(mut s: *mut scroll_t) {
    let mut dx: fixed_t = (*s).dx;
    let mut dy: fixed_t = (*s).dy;
    let mut is3dblock: boolean = false_0 as libc::c_int;
    if (*s).control != -(1 as libc::c_int) {
        let mut height: fixed_t = (*sectors.offset((*s).control as isize)).floorheight
            + (*sectors.offset((*s).control as isize)).ceilingheight;
        let mut delta: fixed_t = height - (*s).last_height;
        (*s).last_height = height;
        dx = FixedMul(dx, delta);
        dy = FixedMul(dy, delta);
    }
    if (*s).accel != 0 {
        dx += (*s).vdx;
        (*s).vdx = dx;
        dy += (*s).vdy;
        (*s).vdy = dy;
    }
    let mut side: *mut side_t = 0 as *mut side_t;
    let mut sec: *mut sector_t = 0 as *mut sector_t;
    let mut height_0: fixed_t = 0;
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut thing: *mut mobj_t = 0 as *mut mobj_t;
    let mut line: *mut line_t = 0 as *mut line_t;
    let mut i: size_t = 0;
    let mut sect: int32_t = 0;
    let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
    match (*s).type_0 as libc::c_uint {
        0 => {
            side = sides.offset((*s).affectee as isize);
            (*side).textureoffset += dx;
            (*side).rowoffset += dy;
        }
        1 => {
            sec = sectors.offset((*s).affectee as isize);
            (*sec).floorxoffset += dx;
            (*sec).flooryoffset += dy;
        }
        2 => {
            sec = sectors.offset((*s).affectee as isize);
            (*sec).ceilingxoffset += dx;
            (*sec).ceilingyoffset += dy;
        }
        3 => {
            sec = sectors.offset((*s).affectee as isize);
            height_0 = (*sec).floorheight;
            i = 0 as libc::c_int as size_t;
            while i < (*sec).linecount {
                line = *((*sec).lines).offset(i as isize);
                if ((*line).special as libc::c_int) < 100 as libc::c_int
                    || (*line).special as libc::c_int >= 300 as libc::c_int
                {
                    is3dblock = false_0 as libc::c_int;
                } else {
                    is3dblock = true_0 as libc::c_int;
                }
                if !(is3dblock == 0) {
                    let mut ICNT_7328: size_t = 0 as libc::c_int as size_t;
                    loop {
                        sect = Tag_Iterate_Sectors(
                            (*line).args[0 as libc::c_int as usize] as mtag_t,
                            ICNT_7328,
                        );
                        if !(sect >= 0 as libc::c_int) {
                            break;
                        }
                        let mut psec: *mut sector_t = 0 as *mut sector_t;
                        psec = sectors.offset(sect as isize);
                        rover = (*psec).ffloors;
                        while !rover.is_null() {
                            if (*rover).master == *((*sec).lines).offset(i as isize) {
                                break;
                            }
                            rover = (*rover).next;
                        }
                        if !rover.is_null() {
                            if !((*rover).fofflags as libc::c_uint
                                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
                            {
                                node = (*psec).touching_thinglist;
                                while !node.is_null() {
                                    thing = (*node).m_thing;
                                    if !((*thing).eflags as libc::c_int
                                        & MFE_PUSHED as libc::c_int != 0)
                                    {
                                        height_0 = P_MobjFloorZ(
                                            thing,
                                            sec,
                                            psec,
                                            (*thing).x,
                                            (*thing).y,
                                            0 as *mut line_t,
                                            (sec != psec) as libc::c_int,
                                            true_0 as libc::c_int,
                                        );
                                        if (*thing).flags & MF_NOCLIP as libc::c_int as uint32_t
                                            == 0
                                        {
                                            if !((*thing).flags
                                                & MF_NOGRAVITY as libc::c_int as uint32_t != 0
                                                || (*thing).z + (*thing).height != height_0)
                                            {
                                                P_DoScrollMove(thing, dx, dy, (*s).exclusive);
                                            }
                                        }
                                    }
                                    node = (*node).m_thinglist_next;
                                }
                            }
                        }
                        ICNT_7328 = ICNT_7328.wrapping_add(1);
                        ICNT_7328;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if is3dblock == 0 {
                node = (*sec).touching_thinglist;
                while !node.is_null() {
                    thing = (*node).m_thing;
                    if !((*thing).eflags as libc::c_int & MFE_PUSHED as libc::c_int != 0)
                    {
                        height_0 = P_MobjFloorZ(
                            thing,
                            sec,
                            sec,
                            (*thing).x,
                            (*thing).y,
                            0 as *mut line_t,
                            (sec != sec) as libc::c_int,
                            true_0 as libc::c_int,
                        );
                        if (*thing).flags & MF_NOCLIP as libc::c_int as uint32_t == 0
                            && !((*thing).flags & MF_NOGRAVITY as libc::c_int as uint32_t
                                != 0 || (*thing).z > height_0)
                        {
                            P_DoScrollMove(thing, dx, dy, (*s).exclusive);
                        }
                    }
                    node = (*node).m_thinglist_next;
                }
            }
        }
        4 => {
            sec = sectors.offset((*s).affectee as isize);
            height_0 = (*sec).ceilingheight;
            i = 0 as libc::c_int as size_t;
            while i < (*sec).linecount {
                line = *((*sec).lines).offset(i as isize);
                if ((*line).special as libc::c_int) < 100 as libc::c_int
                    || (*line).special as libc::c_int >= 300 as libc::c_int
                {
                    is3dblock = false_0 as libc::c_int;
                } else {
                    is3dblock = true_0 as libc::c_int;
                }
                if !(is3dblock == 0) {
                    let mut ICNT_7403: size_t = 0 as libc::c_int as size_t;
                    loop {
                        sect = Tag_Iterate_Sectors(
                            (*line).args[0 as libc::c_int as usize] as mtag_t,
                            ICNT_7403,
                        );
                        if !(sect >= 0 as libc::c_int) {
                            break;
                        }
                        let mut psec_0: *mut sector_t = 0 as *mut sector_t;
                        psec_0 = sectors.offset(sect as isize);
                        rover = (*psec_0).ffloors;
                        while !rover.is_null() {
                            if (*rover).master == *((*sec).lines).offset(i as isize) {
                                break;
                            }
                            rover = (*rover).next;
                        }
                        if !rover.is_null() {
                            if !((*rover).fofflags as libc::c_uint
                                & FOF_EXISTS as libc::c_int as libc::c_uint == 0)
                            {
                                node = (*psec_0).touching_thinglist;
                                while !node.is_null() {
                                    thing = (*node).m_thing;
                                    if !((*thing).eflags as libc::c_int
                                        & MFE_PUSHED as libc::c_int != 0)
                                    {
                                        height_0 = P_MobjCeilingZ(
                                            thing,
                                            sec,
                                            psec_0,
                                            (*thing).x,
                                            (*thing).y,
                                            0 as *mut line_t,
                                            (sec == psec_0) as libc::c_int,
                                            true_0 as libc::c_int,
                                        );
                                        if (*thing).flags & MF_NOCLIP as libc::c_int as uint32_t
                                            == 0
                                        {
                                            if !((*thing).flags
                                                & MF_NOGRAVITY as libc::c_int as uint32_t != 0
                                                || (*thing).z != height_0)
                                            {
                                                P_DoScrollMove(thing, dx, dy, (*s).exclusive);
                                            }
                                        }
                                    }
                                    node = (*node).m_thinglist_next;
                                }
                            }
                        }
                        ICNT_7403 = ICNT_7403.wrapping_add(1);
                        ICNT_7403;
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if is3dblock == 0 {
                node = (*sec).touching_thinglist;
                while !node.is_null() {
                    thing = (*node).m_thing;
                    if !((*thing).eflags as libc::c_int & MFE_PUSHED as libc::c_int != 0)
                    {
                        height_0 = P_MobjCeilingZ(
                            thing,
                            sec,
                            sec,
                            (*thing).x,
                            (*thing).y,
                            0 as *mut line_t,
                            (sec == sec) as libc::c_int,
                            true_0 as libc::c_int,
                        );
                        if (*thing).flags & MF_NOCLIP as libc::c_int as uint32_t == 0
                            && !((*thing).flags & MF_NOGRAVITY as libc::c_int as uint32_t
                                != 0 || (*thing).z + (*thing).height < height_0)
                        {
                            P_DoScrollMove(thing, dx, dy, (*s).exclusive);
                        }
                    }
                    node = (*node).m_thinglist_next;
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn IsSector3DBlock(mut sec: *mut sector_t) -> boolean {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*sec).linecount {
        if (**((*sec).lines).offset(i as isize)).special as libc::c_int
            >= 100 as libc::c_int
            && ((**((*sec).lines).offset(i as isize)).special as libc::c_int)
                < 300 as libc::c_int
        {
            return true_0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn Add_Scroller(
    mut type_0: int32_t,
    mut dx: fixed_t,
    mut dy: fixed_t,
    mut control: int32_t,
    mut affectee: int32_t,
    mut accel: int32_t,
    mut exclusive: int32_t,
) {
    let mut s: *mut scroll_t = Z_CallocAlign(
        ::core::mem::size_of::<scroll_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut scroll_t;
    (*s)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut scroll_t) -> ()>,
        actionf_p1,
    >(Some(T_Scroll as unsafe extern "C" fn(*mut scroll_t) -> ()));
    (*s).type_0 = type_0 as C2RustUnnamed_66;
    (*s).dx = dx;
    (*s).dy = dy;
    (*s).accel = accel;
    (*s).exclusive = exclusive;
    (*s).vdy = 0 as libc::c_int;
    (*s).vdx = (*s).vdy;
    (*s).control = control;
    if (*s).control != -(1 as libc::c_int) {
        (*s)
            .last_height = (*sectors.offset(control as isize)).floorheight
            + (*sectors.offset(control as isize)).ceilingheight;
    }
    (*s).affectee = affectee;
    if type_0 == sc_carry as libc::c_int || type_0 == sc_carry_ceiling as libc::c_int {
        let ref mut fresh33 = (*sectors.offset(affectee as isize)).specialflags;
        *fresh33 = ::core::mem::transmute::<
            libc::c_uint,
            sectorspecialflags_t,
        >(*fresh33 as libc::c_uint | SSF_CONVEYOR as libc::c_int as libc::c_uint);
        if IsSector3DBlock(&mut *sectors.offset(affectee as isize)) != 0 {
            if type_0 == sc_carry as libc::c_int {
                let ref mut fresh34 = (*sectors.offset(affectee as isize)).flags;
                *fresh34 = ::core::mem::transmute::<
                    libc::c_uint,
                    sectorflags_t,
                >(
                    *fresh34 as libc::c_uint
                        | MSF_FLIPSPECIAL_CEILING as libc::c_int as libc::c_uint,
                );
            } else {
                let ref mut fresh35 = (*sectors.offset(affectee as isize)).flags;
                *fresh35 = ::core::mem::transmute::<
                    libc::c_uint,
                    sectorflags_t,
                >(
                    *fresh35 as libc::c_uint
                        | MSF_FLIPSPECIAL_FLOOR as libc::c_int as libc::c_uint,
                );
            }
        }
    }
    P_AddThinker(THINK_MAIN, &mut (*s).thinker);
    match type_0 {
        0 => {
            R_CreateInterpolator_SideScroll(
                &mut (*s).thinker,
                &mut *sides.offset(affectee as isize),
            );
        }
        1 => {
            R_CreateInterpolator_SectorScroll(
                &mut (*s).thinker,
                &mut *sectors.offset(affectee as isize),
                false_0 as libc::c_int,
            );
        }
        2 => {
            R_CreateInterpolator_SectorScroll(
                &mut (*s).thinker,
                &mut *sectors.offset(affectee as isize),
                true_0 as libc::c_int,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn P_SpawnPlaneScroller(
    mut l: *mut line_t,
    mut dx: fixed_t,
    mut dy: fixed_t,
    mut control: int32_t,
    mut affectee: int32_t,
    mut accel: int32_t,
    mut exclusive: int32_t,
) {
    if (*l).args[1 as libc::c_int as usize] != TMP_CEILING as libc::c_int {
        if (*l).args[2 as libc::c_int as usize] != TMS_SCROLLONLY as libc::c_int {
            Add_Scroller(
                sc_carry as libc::c_int,
                FixedMul(
                    dx,
                    3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 32 as libc::c_int,
                ),
                FixedMul(
                    dy,
                    3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 32 as libc::c_int,
                ),
                control,
                affectee,
                accel,
                exclusive,
            );
        }
        if (*l).args[2 as libc::c_int as usize] != TMS_CARRYONLY as libc::c_int {
            Add_Scroller(
                sc_floor as libc::c_int,
                -dx,
                dy,
                control,
                affectee,
                accel,
                exclusive,
            );
        }
    }
    if (*l).args[1 as libc::c_int as usize] != TMP_FLOOR as libc::c_int {
        if (*l).args[2 as libc::c_int as usize] != TMS_SCROLLONLY as libc::c_int {
            Add_Scroller(
                sc_carry_ceiling as libc::c_int,
                FixedMul(
                    dx,
                    3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 32 as libc::c_int,
                ),
                FixedMul(
                    dy,
                    3 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                        / 32 as libc::c_int,
                ),
                control,
                affectee,
                accel,
                exclusive,
            );
        }
        if (*l).args[2 as libc::c_int as usize] != TMS_CARRYONLY as libc::c_int {
            Add_Scroller(
                sc_ceiling as libc::c_int,
                -dx,
                dy,
                control,
                affectee,
                accel,
                exclusive,
            );
        }
    }
}
unsafe extern "C" fn P_SpawnScrollers() {
    let mut i: size_t = 0;
    let mut l: *mut line_t = lines;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        let mut control: int32_t = -(1 as libc::c_int);
        let mut accel: int32_t = 0 as libc::c_int;
        if (*l).special as libc::c_int == 502 as libc::c_int
            || (*l).special as libc::c_int == 510 as libc::c_int
        {
            if (*l).args[4 as libc::c_int as usize] & TMST_TYPEMASK as libc::c_int
                != TMST_REGULAR as libc::c_int
            {
                control = ((*sides.offset(*((*l).sidenum).as_mut_ptr() as isize)).sector)
                    .offset_from(sectors) as libc::c_long as int32_t;
            }
            if (*l).args[4 as libc::c_int as usize] & TMST_TYPEMASK as libc::c_int
                == TMST_ACCELERATIVE as libc::c_int
            {
                accel = 1 as libc::c_int;
            }
        }
        let mut s: int32_t = 0;
        match (*l).special as libc::c_int {
            510 => {
                let mut length: fixed_t = R_PointToDist2(
                    (*(*l).v2).x,
                    (*(*l).v2).y,
                    (*(*l).v1).x,
                    (*(*l).v1).y,
                );
                let mut speed: fixed_t = (*l).args[3 as libc::c_int as usize]
                    << 16 as libc::c_int;
                let mut dx: fixed_t = FixedMul(FixedDiv((*l).dx, length), speed)
                    >> 5 as libc::c_int;
                let mut dy: fixed_t = FixedMul(FixedDiv((*l).dy, length), speed)
                    >> 5 as libc::c_int;
                if (*l).args[0 as libc::c_int as usize] == 0 as libc::c_int {
                    P_SpawnPlaneScroller(
                        l,
                        dx,
                        dy,
                        control,
                        ((*l).frontsector).offset_from(sectors) as libc::c_long
                            as int32_t,
                        accel,
                        ((*l).args[4 as libc::c_int as usize]
                            & TMST_NONEXCLUSIVE as libc::c_int == 0) as libc::c_int,
                    );
                } else {
                    let mut ICNT_7586: size_t = 0 as libc::c_int as size_t;
                    loop {
                        s = Tag_Iterate_Sectors(
                            (*l).args[0 as libc::c_int as usize] as mtag_t,
                            ICNT_7586,
                        );
                        if !(s >= 0 as libc::c_int) {
                            break;
                        }
                        P_SpawnPlaneScroller(
                            l,
                            dx,
                            dy,
                            control,
                            s,
                            accel,
                            ((*l).args[4 as libc::c_int as usize]
                                & TMST_NONEXCLUSIVE as libc::c_int == 0) as libc::c_int,
                        );
                        ICNT_7586 = ICNT_7586.wrapping_add(1);
                        ICNT_7586;
                    }
                }
            }
            502 => {
                let mut ICNT_7596: size_t = 0 as libc::c_int as size_t;
                loop {
                    s = Tag_Iterate_Lines(
                        (*l).args[0 as libc::c_int as usize] as mtag_t,
                        ICNT_7596,
                    );
                    if !(s >= 0 as libc::c_int) {
                        break;
                    }
                    if s != i as int32_t {
                        if (*l).args[1 as libc::c_int as usize]
                            != TMSD_BACK as libc::c_int
                        {
                            Add_Scroller(
                                sc_side as libc::c_int,
                                (*l).args[2 as libc::c_int as usize]
                                    << 16 as libc::c_int - 5 as libc::c_int,
                                (*l).args[3 as libc::c_int as usize]
                                    << 16 as libc::c_int - 5 as libc::c_int,
                                control,
                                (*lines.offset(s as isize))
                                    .sidenum[0 as libc::c_int as usize] as int32_t,
                                accel,
                                0 as libc::c_int,
                            );
                        }
                        if (*l).args[1 as libc::c_int as usize]
                            != TMSD_FRONT as libc::c_int
                            && (*lines.offset(s as isize))
                                .sidenum[1 as libc::c_int as usize] as libc::c_int
                                != 0xffff as libc::c_int
                        {
                            Add_Scroller(
                                sc_side as libc::c_int,
                                (*l).args[2 as libc::c_int as usize]
                                    << 16 as libc::c_int - 5 as libc::c_int,
                                (*l).args[3 as libc::c_int as usize]
                                    << 16 as libc::c_int - 5 as libc::c_int,
                                control,
                                (*lines.offset(s as isize))
                                    .sidenum[1 as libc::c_int as usize] as int32_t,
                                accel,
                                0 as libc::c_int,
                            );
                        }
                    }
                    ICNT_7596 = ICNT_7596.wrapping_add(1);
                    ICNT_7596;
                }
            }
            500 => {
                if (*l).args[0 as libc::c_int as usize] != TMSD_BACK as libc::c_int {
                    Add_Scroller(
                        sc_side as libc::c_int,
                        -(*l).args[1 as libc::c_int as usize] << 16 as libc::c_int,
                        (*l).args[2 as libc::c_int as usize] << 16 as libc::c_int,
                        -(1 as libc::c_int),
                        (*l).sidenum[0 as libc::c_int as usize] as int32_t,
                        accel,
                        0 as libc::c_int,
                    );
                }
                if (*l).args[0 as libc::c_int as usize] != TMSD_FRONT as libc::c_int {
                    if (*l).sidenum[1 as libc::c_int as usize] as libc::c_int
                        != 0xffff as libc::c_int
                    {
                        Add_Scroller(
                            sc_side as libc::c_int,
                            -(*l).args[1 as libc::c_int as usize] << 16 as libc::c_int,
                            (*l).args[2 as libc::c_int as usize] << 16 as libc::c_int,
                            -(1 as libc::c_int),
                            (*l).sidenum[1 as libc::c_int as usize] as int32_t,
                            accel,
                            0 as libc::c_int,
                        );
                    } else {
                        CONS_Debug(
                            0x80 as libc::c_int,
                            b"Line special 500 (line #%s) missing back side!\n\0"
                                as *const u8 as *const libc::c_char,
                            sizeu1(i),
                        );
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
        i;
        l = l.offset(1);
        l;
    }
}
unsafe extern "C" fn Add_MasterDisappearer(
    mut appeartime: tic_t,
    mut disappeartime: tic_t,
    mut offset: tic_t,
    mut line: int32_t,
    mut sourceline: int32_t,
) {
    let mut d: *mut disappear_t = Z_MallocAlign(
        ::core::mem::size_of::<disappear_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut disappear_t;
    (*d)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut disappear_t) -> ()>,
        actionf_p1,
    >(Some(T_Disappear as unsafe extern "C" fn(*mut disappear_t) -> ()));
    (*d).appeartime = appeartime;
    (*d).disappeartime = disappeartime;
    (*d).offset = offset;
    (*d).affectee = line;
    (*d).sourceline = sourceline;
    (*d).exists = true_0 as libc::c_int;
    (*d).timer = 1 as libc::c_int as tic_t;
    P_AddThinker(THINK_MAIN, &mut (*d).thinker);
}
#[no_mangle]
pub unsafe extern "C" fn T_Disappear(mut d: *mut disappear_t) {
    if (*d).offset != 0 && (*d).exists == 0 {
        (*d).offset = ((*d).offset).wrapping_sub(1);
        (*d).offset;
        return;
    }
    (*d).timer = ((*d).timer).wrapping_sub(1);
    if (*d).timer <= 0 as libc::c_int as tic_t {
        let mut rover: *mut ffloor_t = 0 as *mut ffloor_t;
        let mut s: int32_t = 0;
        let mut afftag: mtag_t = (*lines.offset((*d).affectee as isize))
            .args[0 as libc::c_int as usize] as mtag_t;
        let mut ICNT_7663: size_t = 0 as libc::c_int as size_t;
        loop {
            s = Tag_Iterate_Sectors(afftag, ICNT_7663);
            if !(s >= 0 as libc::c_int) {
                break;
            }
            rover = (*sectors.offset(s as isize)).ffloors;
            while !rover.is_null() {
                if !((*rover).master
                    != &mut *lines.offset((*d).affectee as isize) as *mut line_t)
                {
                    if (*d).exists != 0 {
                        (*rover)
                            .fofflags = ::core::mem::transmute::<
                            libc::c_uint,
                            ffloortype_e,
                        >(
                            (*rover).fofflags as libc::c_uint
                                & !(FOF_EXISTS as libc::c_int) as libc::c_uint,
                        );
                    } else {
                        (*rover)
                            .fofflags = ::core::mem::transmute::<
                            libc::c_uint,
                            ffloortype_e,
                        >(
                            (*rover).fofflags as libc::c_uint
                                | FOF_EXISTS as libc::c_int as libc::c_uint,
                        );
                        if (*lines.offset((*d).sourceline as isize))
                            .args[5 as libc::c_int as usize] == 0
                        {
                            (*sectors.offset(s as isize))
                                .soundorg
                                .z = P_GetFFloorTopZAt(
                                rover,
                                (*sectors.offset(s as isize)).soundorg.x,
                                (*sectors.offset(s as isize)).soundorg.y,
                            );
                            S_StartSound(
                                &mut (*sectors.offset(s as isize)).soundorg
                                    as *mut degenmobj_t as *const libc::c_void,
                                sfx_appear,
                            );
                        }
                    }
                }
                rover = (*rover).next;
            }
            (*sectors.offset(s as isize)).moved = true_0 as libc::c_int;
            P_RecalcPrecipInSector(&mut *sectors.offset(s as isize));
            ICNT_7663 = ICNT_7663.wrapping_add(1);
            ICNT_7663;
        }
        if (*d).exists != 0 {
            (*d).timer = (*d).disappeartime;
            (*d).exists = false_0 as libc::c_int;
        } else {
            (*d).timer = (*d).appeartime;
            (*d).exists = true_0 as libc::c_int;
        }
    }
}
unsafe extern "C" fn P_ResetFakeFloorFader(
    mut rover: *mut ffloor_t,
    mut data: *mut fade_t,
    mut finalize: boolean,
) {
    let mut fadingdata: *mut fade_t = (*rover).fadingdata as *mut fade_t;
    if fadingdata != data {
        if !fadingdata.is_null() {
            if finalize != 0 {
                P_FadeFakeFloor(
                    rover,
                    (*fadingdata).sourcevalue,
                    (if (*fadingdata).alpha >= (*fadingdata).destvalue as libc::c_int {
                        (*fadingdata).alpha - 1 as libc::c_int
                    } else {
                        (*fadingdata).alpha + 1 as libc::c_int
                    }) as int16_t,
                    0 as libc::c_int as int16_t,
                    (*fadingdata).ticbased,
                    &mut (*fadingdata).timer,
                    (*fadingdata).doexists,
                    (*fadingdata).dotranslucent,
                    (*fadingdata).dolighting,
                    (*fadingdata).docolormap,
                    (*fadingdata).docollision,
                    (*fadingdata).doghostfade,
                    (*fadingdata).exactalpha,
                );
            }
            (*rover).alpha = (*fadingdata).alpha;
            if (*fadingdata).dolighting != 0 {
                P_RemoveLighting(&mut *sectors.offset((*rover).secnum as isize));
            }
            if (*fadingdata).docolormap != 0 {
                P_ResetColormapFader(&mut *sectors.offset((*rover).secnum as isize));
            }
            P_RemoveThinker(&mut (*fadingdata).thinker);
        }
        (*rover).fadingdata = data as *mut libc::c_void;
    }
}
unsafe extern "C" fn P_FadeFakeFloor(
    mut rover: *mut ffloor_t,
    mut sourcevalue: int16_t,
    mut destvalue: int16_t,
    mut speed: int16_t,
    mut ticbased: boolean,
    mut timer: *mut int32_t,
    mut doexists: boolean,
    mut dotranslucent: boolean,
    mut dolighting: boolean,
    mut docolormap: boolean,
    mut docollision: boolean,
    mut doghostfade: boolean,
    mut exactalpha: boolean,
) -> boolean {
    let mut stillfading: boolean = false_0 as libc::c_int;
    let mut alpha: int32_t = 0;
    let mut fadingdata: *mut fade_t = (*rover).fadingdata as *mut fade_t;
    if (*(*rover).master).special as libc::c_int == 258 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if dotranslucent != 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
        && (*rover).fofflags as libc::c_uint & FOF_FOG as libc::c_int as libc::c_uint
            == 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
        && (*rover).fofflags as libc::c_uint
            & FOF_RENDERALL as libc::c_int as libc::c_uint == 0
    {
        (*rover).alpha = 0 as libc::c_int;
    }
    if !fadingdata.is_null() {
        alpha = (*fadingdata).alpha;
    } else {
        alpha = (*rover).alpha;
    }
    if ticbased == 0 && alpha == destvalue as libc::c_int {
        return stillfading
    } else if alpha > destvalue as libc::c_int {
        if (speed as libc::c_int) < 1 as libc::c_int
            || ticbased == 0
                && alpha - speed as libc::c_int
                    <= destvalue as libc::c_int + speed as libc::c_int
            || ticbased != 0
                && {
                    *timer -= 1;
                    *timer <= 0 as libc::c_int || alpha <= destvalue as libc::c_int
                }
        {
            alpha = destvalue as int32_t;
            if docollision != 0 {
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SOLID as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_SOLID as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SWIMMABLE as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_SWIMMABLE as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_QUICKSAND as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_QUICKSAND as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_BUSTUP as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_BUSTUP as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_MARIO as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_MARIO as libc::c_int) as libc::c_uint,
                    );
                }
            }
        } else {
            if ticbased == 0 {
                alpha -= speed as libc::c_int;
            } else {
                let mut delta: int16_t = abs(
                    destvalue as libc::c_int - sourcevalue as libc::c_int,
                ) as int16_t;
                let mut factor: fixed_t = if FixedDiv(
                    speed as libc::c_int - *timer,
                    speed as fixed_t,
                ) < 1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                {
                    FixedDiv(speed as libc::c_int - *timer, speed as fixed_t)
                } else {
                    1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
                };
                alpha = if (if alpha
                    < sourcevalue as libc::c_int
                        - FixedMul(delta as fixed_t, factor) as int16_t as libc::c_int
                {
                    alpha
                } else {
                    sourcevalue as libc::c_int
                        - FixedMul(delta as fixed_t, factor) as int16_t as libc::c_int
                }) > destvalue as libc::c_int
                {
                    if alpha
                        < sourcevalue as libc::c_int
                            - FixedMul(delta as fixed_t, factor) as int16_t
                                as libc::c_int
                    {
                        alpha
                    } else {
                        sourcevalue as libc::c_int
                            - FixedMul(delta as fixed_t, factor) as int16_t
                                as libc::c_int
                    }
                } else {
                    destvalue as libc::c_int
                };
            }
            stillfading = true_0 as libc::c_int;
        }
    } else if (speed as libc::c_int) < 1 as libc::c_int
        || ticbased == 0
            && alpha + speed as libc::c_int
                >= destvalue as libc::c_int - speed as libc::c_int
        || ticbased != 0
            && {
                *timer -= 1;
                *timer <= 0 as libc::c_int || alpha >= destvalue as libc::c_int
            }
    {
        alpha = destvalue as int32_t;
        if docollision != 0 {
            if (*rover).spawnflags as libc::c_uint
                & FOF_SOLID as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_SOLID as libc::c_int as libc::c_uint,
                );
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_SWIMMABLE as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_SWIMMABLE as libc::c_int as libc::c_uint,
                );
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_QUICKSAND as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_QUICKSAND as libc::c_int as libc::c_uint,
                );
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_BUSTUP as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_BUSTUP as libc::c_int as libc::c_uint,
                );
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_MARIO as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_MARIO as libc::c_int as libc::c_uint,
                );
            }
        }
    } else {
        if ticbased == 0 {
            alpha += speed as libc::c_int;
        } else {
            let mut delta_0: int16_t = abs(
                destvalue as libc::c_int - sourcevalue as libc::c_int,
            ) as int16_t;
            let mut factor_0: fixed_t = if FixedDiv(
                speed as libc::c_int - *timer,
                speed as fixed_t,
            ) < 1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
            {
                FixedDiv(speed as libc::c_int - *timer, speed as fixed_t)
            } else {
                1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
            };
            alpha = if (if alpha
                > sourcevalue as libc::c_int
                    + FixedMul(delta_0 as fixed_t, factor_0) as int16_t as libc::c_int
            {
                alpha
            } else {
                sourcevalue as libc::c_int
                    + FixedMul(delta_0 as fixed_t, factor_0) as int16_t as libc::c_int
            }) < destvalue as libc::c_int
            {
                if alpha
                    > sourcevalue as libc::c_int
                        + FixedMul(delta_0 as fixed_t, factor_0) as int16_t
                            as libc::c_int
                {
                    alpha
                } else {
                    sourcevalue as libc::c_int
                        + FixedMul(delta_0 as fixed_t, factor_0) as int16_t
                            as libc::c_int
                }
            } else {
                destvalue as libc::c_int
            };
        }
        stillfading = true_0 as libc::c_int;
    }
    if stillfading == 0 {
        if doexists != 0
            && (*rover).spawnflags as libc::c_uint
                & FOF_BUSTUP as libc::c_int as libc::c_uint == 0
        {
            if alpha <= 0 as libc::c_int {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        & !(FOF_EXISTS as libc::c_int) as libc::c_uint,
                );
            } else {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_EXISTS as libc::c_int as libc::c_uint,
                );
            }
            if dolighting != 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_NOSHADE as libc::c_int as libc::c_uint == 0
                && (*rover).fofflags as libc::c_uint
                    & FOF_EXISTS as libc::c_int as libc::c_uint == 0
            {
                (*(*rover).target).moved = true_0 as libc::c_int;
            }
        }
        if dotranslucent != 0
            && (*rover).fofflags as libc::c_uint & FOF_FOG as libc::c_int as libc::c_uint
                == 0
        {
            if alpha >= 255 as libc::c_int {
                if (*rover).fofflags as libc::c_uint
                    & FOF_CUTSOLIDS as libc::c_int as libc::c_uint == 0
                    && (*rover).spawnflags as libc::c_uint
                        & FOF_CUTSOLIDS as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_CUTSOLIDS as libc::c_int as libc::c_uint,
                    );
                    (*(*rover).target).moved = true_0 as libc::c_int;
                }
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        & !(FOF_TRANSLUCENT as libc::c_int) as libc::c_uint,
                );
            } else {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_TRANSLUCENT as libc::c_int as libc::c_uint,
                );
                if (*rover).fofflags as libc::c_uint
                    & FOF_CUTSOLIDS as libc::c_int as libc::c_uint != 0
                    && (*rover).spawnflags as libc::c_uint
                        & FOF_CUTSOLIDS as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_CUTSOLIDS as libc::c_int) as libc::c_uint,
                    );
                    (*(*rover).target).moved = true_0 as libc::c_int;
                }
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
            {
                if (*rover).alpha > 1 as libc::c_int {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_RENDERALL as libc::c_int as libc::c_uint,
                    );
                } else {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_RENDERALL as libc::c_int) as libc::c_uint,
                    );
                }
            }
        }
    } else {
        if doexists != 0
            && (*rover).spawnflags as libc::c_uint
                & FOF_BUSTUP as libc::c_int as libc::c_uint == 0
        {
            if dolighting != 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_NOSHADE as libc::c_int as libc::c_uint == 0
                && (*rover).fofflags as libc::c_uint
                    & FOF_EXISTS as libc::c_int as libc::c_uint == 0
            {
                (*(*rover).target).moved = true_0 as libc::c_int;
            }
            (*rover)
                .fofflags = ::core::mem::transmute::<
                libc::c_uint,
                ffloortype_e,
            >(
                (*rover).fofflags as libc::c_uint
                    | FOF_EXISTS as libc::c_int as libc::c_uint,
            );
        }
        if dotranslucent != 0
            && (*rover).fofflags as libc::c_uint & FOF_FOG as libc::c_int as libc::c_uint
                == 0
        {
            (*rover)
                .fofflags = ::core::mem::transmute::<
                libc::c_uint,
                ffloortype_e,
            >(
                (*rover).fofflags as libc::c_uint
                    | FOF_TRANSLUCENT as libc::c_int as libc::c_uint,
            );
            if (*rover).fofflags as libc::c_uint
                & FOF_CUTSOLIDS as libc::c_int as libc::c_uint != 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_CUTSOLIDS as libc::c_int as libc::c_uint != 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        & !(FOF_CUTSOLIDS as libc::c_int) as libc::c_uint,
                );
                (*(*rover).target).moved = true_0 as libc::c_int;
            }
            if (*rover).spawnflags as libc::c_uint
                & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
                && (*rover).spawnflags as libc::c_uint
                    & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
            {
                (*rover)
                    .fofflags = ::core::mem::transmute::<
                    libc::c_uint,
                    ffloortype_e,
                >(
                    (*rover).fofflags as libc::c_uint
                        | FOF_RENDERALL as libc::c_int as libc::c_uint,
                );
            }
        }
        if docollision != 0 {
            if doghostfade != 0 {
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SOLID as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_SOLID as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SWIMMABLE as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_SWIMMABLE as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_QUICKSAND as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_QUICKSAND as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_BUSTUP as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_BUSTUP as libc::c_int) as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_MARIO as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            & !(FOF_MARIO as libc::c_int) as libc::c_uint,
                    );
                }
            } else {
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SOLID as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_SOLID as libc::c_int as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_SWIMMABLE as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_SWIMMABLE as libc::c_int as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_QUICKSAND as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_QUICKSAND as libc::c_int as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_BUSTUP as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_BUSTUP as libc::c_int as libc::c_uint,
                    );
                }
                if (*rover).spawnflags as libc::c_uint
                    & FOF_MARIO as libc::c_int as libc::c_uint != 0
                {
                    (*rover)
                        .fofflags = ::core::mem::transmute::<
                        libc::c_uint,
                        ffloortype_e,
                    >(
                        (*rover).fofflags as libc::c_uint
                            | FOF_MARIO as libc::c_int as libc::c_uint,
                    );
                }
            }
        }
    }
    if (*rover).fofflags as libc::c_uint & FOF_FOG as libc::c_int as libc::c_uint == 0 {
        if stillfading == 0 || exactalpha != 0 {
            (*rover).alpha = alpha;
        } else if alpha < 12 as libc::c_int {
            (*rover)
                .alpha = if (destvalue as libc::c_int) < 12 as libc::c_int {
                destvalue as libc::c_int
            } else {
                0 as libc::c_int
            };
        } else if alpha < 38 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 12 as libc::c_int
                && (destvalue as libc::c_int) < 38 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                25 as libc::c_int
            };
        } else if alpha < 64 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 38 as libc::c_int
                && (destvalue as libc::c_int) < 64 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                51 as libc::c_int
            };
        } else if alpha < 89 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 64 as libc::c_int
                && (destvalue as libc::c_int) < 89 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                76 as libc::c_int
            };
        } else if alpha < 115 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 89 as libc::c_int
                && (destvalue as libc::c_int) < 115 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                102 as libc::c_int
            };
        } else if alpha < 140 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 115 as libc::c_int
                && (destvalue as libc::c_int) < 140 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                128 as libc::c_int
            };
        } else if alpha < 166 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 140 as libc::c_int
                && (destvalue as libc::c_int) < 166 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                154 as libc::c_int
            };
        } else if alpha < 192 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 166 as libc::c_int
                && (destvalue as libc::c_int) < 192 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                179 as libc::c_int
            };
        } else if alpha < 217 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 192 as libc::c_int
                && (destvalue as libc::c_int) < 217 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                204 as libc::c_int
            };
        } else if alpha < 243 as libc::c_int {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 217 as libc::c_int
                && (destvalue as libc::c_int) < 243 as libc::c_int
            {
                destvalue as libc::c_int
            } else {
                230 as libc::c_int
            };
        } else {
            (*rover)
                .alpha = if destvalue as libc::c_int >= 243 as libc::c_int {
                destvalue as libc::c_int
            } else {
                255 as libc::c_int
            };
        }
    }
    if !fadingdata.is_null() {
        (*fadingdata).alpha = alpha;
    }
    return stillfading;
}
unsafe extern "C" fn P_AddFakeFloorFader(
    mut rover: *mut ffloor_t,
    mut sectornum: size_t,
    mut ffloornum: size_t,
    mut destvalue: int16_t,
    mut speed: int16_t,
    mut ticbased: boolean,
    mut relative: boolean,
    mut doexists: boolean,
    mut dotranslucent: boolean,
    mut dolighting: boolean,
    mut docolormap: boolean,
    mut docollision: boolean,
    mut doghostfade: boolean,
    mut exactalpha: boolean,
) {
    let mut d: *mut fade_t = 0 as *mut fade_t;
    if dotranslucent != 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_NOSHADE as libc::c_int as libc::c_uint != 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_RENDERSIDES as libc::c_int as libc::c_uint == 0
        && (*rover).spawnflags as libc::c_uint
            & FOF_RENDERPLANES as libc::c_int as libc::c_uint == 0
        && (*rover).fofflags as libc::c_uint
            & FOF_RENDERALL as libc::c_int as libc::c_uint == 0
    {
        (*rover).alpha = 0 as libc::c_int;
    }
    if (*rover).alpha
        == (if 0 as libc::c_int
            > (if (255 as libc::c_int)
                < (if relative != 0 {
                    (*rover).alpha + destvalue as libc::c_int
                } else {
                    destvalue as libc::c_int
                })
            {
                255 as libc::c_int
            } else {
                (if relative != 0 {
                    (*rover).alpha + destvalue as libc::c_int
                } else {
                    destvalue as libc::c_int
                })
            })
        {
            0 as libc::c_int
        } else {
            (if (255 as libc::c_int)
                < (if relative != 0 {
                    (*rover).alpha + destvalue as libc::c_int
                } else {
                    destvalue as libc::c_int
                })
            {
                255 as libc::c_int
            } else {
                (if relative != 0 {
                    (*rover).alpha + destvalue as libc::c_int
                } else {
                    destvalue as libc::c_int
                })
            })
        })
    {
        return;
    }
    d = Z_MallocAlign(
        ::core::mem::size_of::<fade_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut fade_t;
    (*d)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut fade_t) -> ()>,
        actionf_p1,
    >(Some(T_Fade as unsafe extern "C" fn(*mut fade_t) -> ()));
    (*d).rover = rover;
    (*d).sectornum = sectornum as uint32_t;
    (*d).ffloornum = ffloornum as uint32_t;
    (*d).sourcevalue = (*rover).alpha as int16_t;
    (*d).alpha = (*d).sourcevalue as int32_t;
    (*d)
        .destvalue = (if 0 as libc::c_int
        > (if (255 as libc::c_int)
            < (if relative != 0 {
                (*rover).alpha + destvalue as libc::c_int
            } else {
                destvalue as libc::c_int
            })
        {
            255 as libc::c_int
        } else {
            (if relative != 0 {
                (*rover).alpha + destvalue as libc::c_int
            } else {
                destvalue as libc::c_int
            })
        })
    {
        0 as libc::c_int
    } else if (255 as libc::c_int)
        < (if relative != 0 {
            (*rover).alpha + destvalue as libc::c_int
        } else {
            destvalue as libc::c_int
        })
    {
        255 as libc::c_int
    } else if relative != 0 {
        (*rover).alpha + destvalue as libc::c_int
    } else {
        destvalue as libc::c_int
    }) as int16_t;
    if ticbased != 0 {
        (*d).ticbased = true_0 as libc::c_int;
        (*d).speed = abs(speed as libc::c_int) as int16_t;
        (*d).timer = (*d).speed as int32_t;
    } else {
        (*d).ticbased = false_0 as libc::c_int;
        (*d)
            .speed = (if 1 as libc::c_int > speed as libc::c_int {
            1 as libc::c_int
        } else {
            speed as libc::c_int
        }) as int16_t;
        (*d).timer = -(1 as libc::c_int);
    }
    (*d).doexists = doexists;
    (*d).dotranslucent = dotranslucent;
    (*d).dolighting = dolighting;
    (*d).docolormap = docolormap;
    (*d).docollision = docollision;
    (*d).doghostfade = doghostfade;
    (*d).exactalpha = exactalpha;
    P_ResetFakeFloorFader(rover, d, false_0 as libc::c_int);
    if dolighting != 0
        && (*rover).fofflags as libc::c_uint & FOF_NOSHADE as libc::c_int as libc::c_uint
            == 0
    {
        let mut lightdelta: uint16_t = abs(
            (*sectors.offset((*rover).secnum as isize)).spawn_lightlevel as libc::c_int
                - (*(*rover).target).lightlevel as libc::c_int,
        ) as uint16_t;
        let mut alphapercent: fixed_t = if FixedDiv(
            (*d).destvalue as fixed_t,
            (*rover).spawnalpha,
        ) < 1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        {
            FixedDiv((*d).destvalue as fixed_t, (*rover).spawnalpha)
        } else {
            1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        };
        let mut adjustedlightdelta: fixed_t = FixedMul(
            lightdelta as fixed_t,
            alphapercent,
        );
        if (*(*rover).target).lightlevel as libc::c_int
            >= (*sectors.offset((*rover).secnum as isize)).spawn_lightlevel
                as libc::c_int
        {
            (*d)
                .destlightlevel = ((*(*rover).target).lightlevel as libc::c_int
                - adjustedlightdelta) as int16_t;
        } else {
            (*d)
                .destlightlevel = ((*(*rover).target).lightlevel as libc::c_int
                + adjustedlightdelta) as int16_t;
        }
        P_FadeLightBySector(
            &mut *sectors.offset((*rover).secnum as isize),
            (*d).destlightlevel as int32_t,
            if ticbased != 0 {
                (*d).timer
            } else {
                FixedFloor(
                    FixedDiv(
                        abs((*d).destvalue as libc::c_int - (*d).alpha),
                        (*d).speed as fixed_t,
                    ),
                ) / ((1 as libc::c_int) << 16 as libc::c_int)
            },
            true_0 as libc::c_int,
        );
    } else {
        (*d).destlightlevel = -(1 as libc::c_int) as int16_t;
    }
    if docolormap != 0
        && (*rover).fofflags as libc::c_uint & FOF_NOSHADE as libc::c_int as libc::c_uint
            == 0
        && !((*sectors.offset((*rover).secnum as isize)).spawn_extra_colormap).is_null()
        && (*sectors.offset((*rover).secnum as isize)).colormap_protected == 0
    {
        let mut dest_exc: *mut extracolormap_t = 0 as *mut extracolormap_t;
        let mut source_exc: *mut extracolormap_t = if !((*sectors
            .offset((*rover).secnum as isize))
            .extra_colormap)
            .is_null()
        {
            (*sectors.offset((*rover).secnum as isize)).extra_colormap
        } else {
            R_GetDefaultColormap()
        };
        let mut colordelta: int32_t = (*(*sectors.offset((*rover).secnum as isize))
            .spawn_extra_colormap)
            .rgba >> 24 as libc::c_int & 0xff as libc::c_int;
        let mut alphapercent_0: fixed_t = if FixedDiv(
            (*d).destvalue as fixed_t,
            (*rover).spawnalpha,
        ) < 1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        {
            FixedDiv((*d).destvalue as fixed_t, (*rover).spawnalpha)
        } else {
            1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        };
        let mut adjustedcolordelta: fixed_t = FixedMul(colordelta, alphapercent_0);
        let mut coloralpha: int32_t = 0;
        coloralpha = adjustedcolordelta;
        dest_exc = R_CopyColormap(
            (*sectors.offset((*rover).secnum as isize)).spawn_extra_colormap,
            false_0 as libc::c_int,
        );
        (*dest_exc)
            .rgba = ((*dest_exc).rgba & 0xffffff as libc::c_int)
            + (coloralpha << 24 as libc::c_int);
        (*d).dest_exc = R_GetColormapFromList(dest_exc);
        if ((*d).dest_exc).is_null() {
            (*dest_exc).colormap = R_CreateLightTable(dest_exc);
            R_AddColormapToList(dest_exc);
            (*d).dest_exc = dest_exc;
        } else {
            Z_Free(dest_exc as *mut libc::c_void);
        }
        if R_CheckDefaultColormap(
            (*d).dest_exc,
            true_0 as libc::c_int,
            false_0 as libc::c_int,
            false_0 as libc::c_int,
        ) == 0
            && R_CheckDefaultColormap(
                source_exc,
                true_0 as libc::c_int,
                false_0 as libc::c_int,
                false_0 as libc::c_int,
            ) != 0
        {
            let mut exc: *mut extracolormap_t = R_CopyColormap(
                source_exc,
                false_0 as libc::c_int,
            );
            (*exc)
                .rgba = ((*(*d).dest_exc).rgba & 0xffffff as libc::c_int)
                + (((*source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                    << 24 as libc::c_int);
            (*exc)
                .fadergba = ((*(*d).dest_exc).rgba & 0xffffff as libc::c_int)
                + (((*source_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
                    << 24 as libc::c_int);
            source_exc = R_GetColormapFromList(exc);
            if source_exc.is_null() {
                (*exc).colormap = R_CreateLightTable(exc);
                R_AddColormapToList(exc);
                source_exc = exc;
            } else {
                Z_Free(exc as *mut libc::c_void);
            }
        }
        Add_ColormapFader(
            &mut *sectors.offset((*rover).secnum as isize),
            source_exc,
            (*d).dest_exc,
            true_0 as libc::c_int,
            if ticbased != 0 {
                (*d).timer
            } else {
                FixedFloor(
                    FixedDiv(
                        abs((*d).destvalue as libc::c_int - (*d).alpha),
                        (*d).speed as fixed_t,
                    ),
                ) / ((1 as libc::c_int) << 16 as libc::c_int)
            },
        );
    }
    P_AddThinker(THINK_MAIN, &mut (*d).thinker);
}
#[no_mangle]
pub unsafe extern "C" fn T_Fade(mut d: *mut fade_t) {
    if !((*d).rover).is_null()
        && P_FadeFakeFloor(
            (*d).rover,
            (*d).sourcevalue,
            (*d).destvalue,
            (*d).speed,
            (*d).ticbased,
            &mut (*d).timer,
            (*d).doexists,
            (*d).dotranslucent,
            (*d).dolighting,
            (*d).docolormap,
            (*d).docollision,
            (*d).doghostfade,
            (*d).exactalpha,
        ) == 0
    {
        if (*d).dolighting != 0
            && (*(*d).rover).fofflags as libc::c_uint
                & FOF_NOSHADE as libc::c_int as libc::c_uint == 0
            && (*d).destlightlevel as libc::c_int > -(1 as libc::c_int)
        {
            (*sectors.offset((*(*d).rover).secnum as isize))
                .lightlevel = (*d).destlightlevel;
        }
        if (*d).docolormap != 0
            && (*(*d).rover).fofflags as libc::c_uint
                & FOF_NOSHADE as libc::c_int as libc::c_uint == 0
            && !((*sectors.offset((*(*d).rover).secnum as isize)).spawn_extra_colormap)
                .is_null()
        {
            let ref mut fresh36 = (*sectors.offset((*(*d).rover).secnum as isize))
                .extra_colormap;
            *fresh36 = (*d).dest_exc;
        }
        P_ResetFakeFloorFader((*d).rover, 0 as *mut fade_t, false_0 as libc::c_int);
    }
}
unsafe extern "C" fn P_ResetColormapFader(mut sector: *mut sector_t) {
    if !((*sector).fadecolormapdata).is_null() {
        P_RemoveThinker(
            &mut (*((*sector).fadecolormapdata as *mut thinkerdata_t)).thinker,
        );
        (*sector).fadecolormapdata = 0 as *mut libc::c_void;
    }
}
unsafe extern "C" fn Add_ColormapFader(
    mut sector: *mut sector_t,
    mut source_exc: *mut extracolormap_t,
    mut dest_exc: *mut extracolormap_t,
    mut ticbased: boolean,
    mut duration: int32_t,
) {
    let mut d: *mut fadecolormap_t = 0 as *mut fadecolormap_t;
    P_ResetColormapFader(sector);
    if duration == 0
        || R_CheckEqualColormaps(
            source_exc,
            dest_exc,
            true_0 as libc::c_int,
            true_0 as libc::c_int,
            true_0 as libc::c_int,
        ) != 0
    {
        (*sector).extra_colormap = dest_exc;
        return;
    }
    d = Z_MallocAlign(
        ::core::mem::size_of::<fadecolormap_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut fadecolormap_t;
    (*d)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut fadecolormap_t) -> ()>,
        actionf_p1,
    >(Some(T_FadeColormap as unsafe extern "C" fn(*mut fadecolormap_t) -> ()));
    (*d).sector = sector;
    (*d).source_exc = source_exc;
    (*d).dest_exc = dest_exc;
    if ticbased != 0 {
        (*d).ticbased = true_0 as libc::c_int;
        (*d).timer = duration;
        (*d).duration = (*d).timer;
    } else {
        (*d).ticbased = false_0 as libc::c_int;
        (*d).timer = 256 as libc::c_int;
        (*d).duration = duration;
    }
    (*sector).fadecolormapdata = d as *mut libc::c_void;
    P_AddThinker(THINK_MAIN, &mut (*d).thinker);
}
#[no_mangle]
pub unsafe extern "C" fn T_FadeColormap(mut d: *mut fadecolormap_t) {
    if (*d).ticbased != 0
        && {
            (*d).timer -= 1;
            (*d).timer <= 0 as libc::c_int
        }
        || (*d).ticbased == 0
            && {
                (*d).timer -= (*d).duration;
                (*d).timer <= 0 as libc::c_int
            }
    {
        (*(*d).sector).extra_colormap = (*d).dest_exc;
        P_ResetColormapFader((*d).sector);
    } else {
        let mut exc: *mut extracolormap_t = 0 as *mut extracolormap_t;
        let mut duration: int32_t = if (*d).ticbased != 0 {
            (*d).duration
        } else {
            256 as libc::c_int
        };
        let mut factor: fixed_t = if FixedDiv(duration - (*d).timer, duration)
            < 1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        {
            FixedDiv(duration - (*d).timer, duration)
        } else {
            1 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int)
        };
        let mut cr: int16_t = 0;
        let mut cg: int16_t = 0;
        let mut cb: int16_t = 0;
        let mut ca: int16_t = 0;
        let mut fadestart: int16_t = 0;
        let mut fadeend: int16_t = 0;
        let mut flags: int16_t = 0;
        let mut rgba: int32_t = 0;
        let mut fadergba: int32_t = 0;
        if ((*(*d).sector).extra_colormap).is_null() {
            (*(*d).sector).extra_colormap = R_GetDefaultColormap();
        }
        if ((*d).source_exc).is_null() {
            (*d).source_exc = R_GetDefaultColormap();
        }
        if ((*d).dest_exc).is_null() {
            (*d).dest_exc = R_GetDefaultColormap();
        }
        cr = (if ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba & 0xff as libc::c_int) < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int)
                < ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).rgba & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int)
                    < ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba & 0xff as libc::c_int) > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
                > ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).rgba & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
                    > ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).rgba & 0xff as libc::c_int
        }) as int16_t;
        cg = (if ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 8 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).rgba >> 8 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        cb = (if ((*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).rgba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).rgba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 16 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).rgba >> 16 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        ca = (if ((*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).rgba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).rgba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).rgba >> 24 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).rgba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).rgba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).rgba >> 24 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        rgba = cr as libc::c_int + ((cg as libc::c_int) << 8 as libc::c_int)
            + ((cb as libc::c_int) << 16 as libc::c_int)
            + ((ca as libc::c_int) << 24 as libc::c_int);
        cr = (if ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int) < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int)
                < ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadergba & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int)
                    < ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int) > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
                > ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadergba & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
                    > ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadergba & 0xff as libc::c_int
        }) as int16_t;
        cg = (if ((*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 8 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 8 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadergba >> 8 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        cb = (if ((*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 16 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 16 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadergba >> 16 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        ca = (if ((*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
            < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                & 0xff as libc::c_int)
                < ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int)
                    < ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int
            }
        } else if ((*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
            > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                & 0xff as libc::c_int
                > ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int
            } else {
                ((*(*d).source_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int)
                    + FixedMul(
                        ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                            & 0xff as libc::c_int)
                            - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int),
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                    & 0xff as libc::c_int
                    > ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int
                } else {
                    ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                        & 0xff as libc::c_int)
                        + FixedMul(
                            ((*(*d).dest_exc).fadergba >> 24 as libc::c_int
                                & 0xff as libc::c_int)
                                - ((*(*d).source_exc).fadergba >> 24 as libc::c_int
                                    & 0xff as libc::c_int),
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadergba >> 24 as libc::c_int & 0xff as libc::c_int
        }) as int16_t;
        fadergba = cr as libc::c_int + ((cg as libc::c_int) << 8 as libc::c_int)
            + ((cb as libc::c_int) << 16 as libc::c_int)
            + ((ca as libc::c_int) << 24 as libc::c_int);
        fadestart = (if ((*(*d).dest_exc).fadestart as libc::c_int
            - (*(*d).source_exc).fadestart as libc::c_int) < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadestart as libc::c_int)
                < (*(*d).source_exc).fadestart as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadestart as libc::c_int
                            - (*(*d).source_exc).fadestart as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
            } else {
                (*(*d).source_exc).fadestart as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadestart as libc::c_int
                            - (*(*d).source_exc).fadestart as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadestart as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadestart as libc::c_int)
                    < (*(*d).source_exc).fadestart as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadestart as libc::c_int
                                - (*(*d).source_exc).fadestart as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
                } else {
                    (*(*d).source_exc).fadestart as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadestart as libc::c_int
                                - (*(*d).source_exc).fadestart as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadestart as libc::c_int
            }
        } else if (*(*d).dest_exc).fadestart as libc::c_int
            - (*(*d).source_exc).fadestart as libc::c_int > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
                > (*(*d).source_exc).fadestart as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadestart as libc::c_int
                            - (*(*d).source_exc).fadestart as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
            } else {
                (*(*d).source_exc).fadestart as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadestart as libc::c_int
                            - (*(*d).source_exc).fadestart as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadestart as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
                    > (*(*d).source_exc).fadestart as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadestart as libc::c_int
                                - (*(*d).source_exc).fadestart as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadestart as libc::c_int
                } else {
                    (*(*d).source_exc).fadestart as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadestart as libc::c_int
                                - (*(*d).source_exc).fadestart as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadestart as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadestart as libc::c_int
        }) as int16_t;
        fadeend = (if ((*(*d).dest_exc).fadeend as libc::c_int
            - (*(*d).source_exc).fadeend as libc::c_int) < 0 as libc::c_int
        {
            if (if ((*(*(*d).sector).extra_colormap).fadeend as libc::c_int)
                < (*(*d).source_exc).fadeend as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadeend as libc::c_int
                            - (*(*d).source_exc).fadeend as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
            } else {
                (*(*d).source_exc).fadeend as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadeend as libc::c_int
                            - (*(*d).source_exc).fadeend as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            }) > (*(*d).dest_exc).fadeend as libc::c_int
            {
                if ((*(*(*d).sector).extra_colormap).fadeend as libc::c_int)
                    < (*(*d).source_exc).fadeend as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadeend as libc::c_int
                                - (*(*d).source_exc).fadeend as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
                } else {
                    (*(*d).source_exc).fadeend as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadeend as libc::c_int
                                - (*(*d).source_exc).fadeend as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadeend as libc::c_int
            }
        } else if (*(*d).dest_exc).fadeend as libc::c_int
            - (*(*d).source_exc).fadeend as libc::c_int > 0 as libc::c_int
        {
            if (if (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
                > (*(*d).source_exc).fadeend as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadeend as libc::c_int
                            - (*(*d).source_exc).fadeend as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            {
                (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
            } else {
                (*(*d).source_exc).fadeend as libc::c_int
                    + FixedMul(
                        (*(*d).dest_exc).fadeend as libc::c_int
                            - (*(*d).source_exc).fadeend as libc::c_int,
                        factor,
                    ) as int16_t as libc::c_int
            }) < (*(*d).dest_exc).fadeend as libc::c_int
            {
                if (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
                    > (*(*d).source_exc).fadeend as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadeend as libc::c_int
                                - (*(*d).source_exc).fadeend as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                {
                    (*(*(*d).sector).extra_colormap).fadeend as libc::c_int
                } else {
                    (*(*d).source_exc).fadeend as libc::c_int
                        + FixedMul(
                            (*(*d).dest_exc).fadeend as libc::c_int
                                - (*(*d).source_exc).fadeend as libc::c_int,
                            factor,
                        ) as int16_t as libc::c_int
                }
            } else {
                (*(*d).dest_exc).fadeend as libc::c_int
            }
        } else {
            (*(*d).dest_exc).fadeend as libc::c_int
        }) as int16_t;
        flags = (if abs(factor)
            > ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int
        {
            (*(*d).dest_exc).flags as libc::c_int
        } else {
            (*(*d).source_exc).flags as libc::c_int
        }) as int16_t;
        (*(*d).sector)
            .extra_colormap = R_GetColormapFromListByValues(
            rgba,
            fadergba,
            fadestart as uint8_t,
            fadeend as uint8_t,
            flags as uint8_t,
        );
        if ((*(*d).sector).extra_colormap).is_null() {
            exc = R_CreateDefaultColormap(false_0 as libc::c_int);
            (*exc).fadestart = fadestart as uint8_t;
            (*exc).fadeend = fadeend as uint8_t;
            (*exc).flags = flags as uint8_t;
            (*exc).rgba = rgba;
            (*exc).fadergba = fadergba;
            (*exc).colormap = R_CreateLightTable(exc);
            R_AddColormapToList(exc);
            (*(*d).sector).extra_colormap = exc;
        }
    };
}
unsafe extern "C" fn Add_Friction(
    mut friction: int32_t,
    mut movefactor: int32_t,
    mut affectee: int32_t,
    mut referrer: int32_t,
) {
    let mut f: *mut friction_t = Z_CallocAlign(
        ::core::mem::size_of::<friction_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut friction_t;
    (*f)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut friction_t) -> ()>,
        actionf_p1,
    >(Some(T_Friction as unsafe extern "C" fn(*mut friction_t) -> ()));
    (*f).friction = friction;
    (*f).movefactor = movefactor;
    (*f).affectee = affectee;
    if referrer != -(1 as libc::c_int) {
        (*f).roverfriction = true_0 as libc::c_int as uint8_t;
        (*f).referrer = referrer;
    } else {
        (*f).roverfriction = false_0 as libc::c_int as uint8_t;
    }
    P_AddThinker(THINK_MAIN, &mut (*f).thinker);
}
#[no_mangle]
pub unsafe extern "C" fn T_Friction(mut f: *mut friction_t) {
    let mut sec: *mut sector_t = 0 as *mut sector_t;
    let mut referrer: *mut sector_t = 0 as *mut sector_t;
    let mut thing: *mut mobj_t = 0 as *mut mobj_t;
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    sec = sectors.offset((*f).affectee as isize);
    if (*f).roverfriction != 0 {
        referrer = sectors.offset((*f).referrer as isize);
    }
    node = (*sec).touching_thinglist;
    while !node.is_null() {
        thing = (*node).m_thing;
        if (*thing).flags
            & (MF_NOGRAVITY as libc::c_int | MF_NOCLIP as libc::c_int) as uint32_t == 0
            && (*thing).z == (*thing).floorz
        {
            if (*f).roverfriction != 0 {
                if (*thing).floorz
                    != P_MobjCeilingZ(
                        thing,
                        referrer,
                        sec,
                        (*thing).x,
                        (*thing).y,
                        0 as *mut line_t,
                        (referrer == sec) as libc::c_int,
                        true_0 as libc::c_int,
                    )
                {
                    node = (*node).m_thinglist_next;
                    continue;
                } else if (*thing).friction
                    == (0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int
                    || (*f).friction < (*thing).friction
                {
                    (*thing).friction = (*f).friction;
                    if !((*thing).player).is_null() {
                        (*thing).movefactor = (*f).movefactor;
                    }
                }
            } else if P_MobjFloorZ(
                thing,
                sec,
                sec,
                (*thing).x,
                (*thing).y,
                0 as *mut line_t,
                (sec != sec) as libc::c_int,
                true_0 as libc::c_int,
            ) == (*thing).floorz
                && ((*thing).friction
                    == (0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int
                    || (*f).friction < (*thing).friction)
            {
                (*thing).friction = (*f).friction;
                if !((*thing).player).is_null() {
                    (*thing).movefactor = (*f).movefactor;
                }
            }
        }
        node = (*node).m_thinglist_next;
    }
}
unsafe extern "C" fn P_SpawnFriction() {
    let mut i: size_t = 0;
    let mut s: *mut sector_t = sectors;
    let mut friction: fixed_t = 0;
    let mut movefactor: int32_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numsectors {
        if !((*s).friction
            == (0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int)
        {
            friction = (*s).friction;
            if friction > (1 as libc::c_int) << 16 as libc::c_int {
                friction = (1 as libc::c_int) << 16 as libc::c_int;
            }
            if friction < 0 as libc::c_int {
                friction = 0 as libc::c_int;
            }
            movefactor = FixedDiv(
                (0xe8 as libc::c_int) << 16 as libc::c_int - 8 as libc::c_int,
                friction,
            );
            if movefactor < (1 as libc::c_int) << 16 as libc::c_int {
                movefactor = 8 as libc::c_int * movefactor
                    - 7 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int);
            } else {
                movefactor = (1 as libc::c_int) << 16 as libc::c_int;
            }
            Add_Friction(
                friction,
                movefactor,
                s.offset_from(sectors) as libc::c_long as int32_t,
                -(1 as libc::c_int),
            );
        }
        i = i.wrapping_add(1);
        i;
        s = s.offset(1);
        s;
    }
}
unsafe extern "C" fn Add_Pusher(
    mut type_0: pushertype_e,
    mut x_mag: fixed_t,
    mut y_mag: fixed_t,
    mut z_mag: fixed_t,
    mut affectee: int32_t,
    mut referrer: int32_t,
    mut exclusive: int32_t,
    mut slider: int32_t,
) {
    let mut p: *mut pusher_t = Z_CallocAlign(
        ::core::mem::size_of::<pusher_t>() as libc::c_ulong,
        PU_LEVSPEC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut pusher_t;
    (*p)
        .thinker
        .function
        .acp1 = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut pusher_t) -> ()>,
        actionf_p1,
    >(Some(T_Pusher as unsafe extern "C" fn(*mut pusher_t) -> ()));
    (*p).type_0 = type_0;
    (*p).x_mag = x_mag;
    (*p).y_mag = y_mag;
    (*p).z_mag = z_mag;
    (*p).exclusive = exclusive;
    (*p).slider = slider;
    if referrer != -(1 as libc::c_int) {
        (*p).roverpusher = true_0 as libc::c_int as uint8_t;
        (*p).referrer = referrer;
        let ref mut fresh37 = (*sectors.offset(referrer as isize)).specialflags;
        *fresh37 = ::core::mem::transmute::<
            libc::c_uint,
            sectorspecialflags_t,
        >(*fresh37 as libc::c_uint | SSF_WINDCURRENT as libc::c_int as libc::c_uint);
    } else {
        (*p).roverpusher = false_0 as libc::c_int as uint8_t;
        let ref mut fresh38 = (*sectors.offset(affectee as isize)).specialflags;
        *fresh38 = ::core::mem::transmute::<
            libc::c_uint,
            sectorspecialflags_t,
        >(*fresh38 as libc::c_uint | SSF_WINDCURRENT as libc::c_int as libc::c_uint);
    }
    (*p).affectee = affectee;
    P_AddThinker(THINK_MAIN, &mut (*p).thinker);
}
#[no_mangle]
pub unsafe extern "C" fn T_Pusher(mut p: *mut pusher_t) {
    let mut sec: *mut sector_t = 0 as *mut sector_t;
    let mut referrer: *mut sector_t = 0 as *mut sector_t;
    let mut thing: *mut mobj_t = 0 as *mut mobj_t;
    let mut node: *mut msecnode_t = 0 as *mut msecnode_t;
    let mut x_mag: fixed_t = 0;
    let mut y_mag: fixed_t = 0;
    let mut z_mag: fixed_t = 0;
    let mut xspeed: fixed_t = 0 as libc::c_int;
    let mut yspeed: fixed_t = 0 as libc::c_int;
    let mut zspeed: fixed_t = 0 as libc::c_int;
    let mut inFOF: boolean = 0;
    let mut touching: boolean = 0;
    let mut moved: boolean = 0;
    x_mag = (*p).x_mag >> 7 as libc::c_int;
    y_mag = (*p).y_mag >> 7 as libc::c_int;
    z_mag = (*p).z_mag >> 7 as libc::c_int;
    sec = sectors.offset((*p).affectee as isize);
    if (*p).roverpusher != 0 {
        referrer = sectors.offset((*p).referrer as isize);
    }
    node = (*sec).touching_thinglist;
    let mut current_block_62: u64;
    while !node.is_null() {
        thing = (*node).m_thing;
        if !((*thing).flags
            & (MF_NOGRAVITY as libc::c_int | MF_NOCLIP as libc::c_int) as uint32_t != 0
            && !((*thing).type_0 as libc::c_uint
                == MT_SMALLBUBBLE as libc::c_int as libc::c_uint
                || (*thing).type_0 as libc::c_uint
                    == MT_MEDIUMBUBBLE as libc::c_int as libc::c_uint
                || (*thing).type_0 as libc::c_uint
                    == MT_EXTRALARGEBUBBLE as libc::c_int as libc::c_uint))
        {
            if !(!((*thing).flags & MF_PUSHABLE as libc::c_int as uint32_t != 0
                || (*(*thing).info).flags & MF_PUSHABLE as libc::c_int as uint32_t != 0
                    && (*thing).fuse != 0)
                && !((*thing).type_0 as libc::c_uint
                    == MT_PLAYER as libc::c_int as libc::c_uint
                    || (*thing).type_0 as libc::c_uint
                        == MT_SMALLBUBBLE as libc::c_int as libc::c_uint
                    || (*thing).type_0 as libc::c_uint
                        == MT_MEDIUMBUBBLE as libc::c_int as libc::c_uint
                    || (*thing).type_0 as libc::c_uint
                        == MT_EXTRALARGEBUBBLE as libc::c_int as libc::c_uint
                    || (*thing).type_0 as libc::c_uint
                        == MT_LITTLETUMBLEWEED as libc::c_int as libc::c_uint
                    || (*thing).type_0 as libc::c_uint
                        == MT_BIGTUMBLEWEED as libc::c_int as libc::c_uint))
            {
                if !((*thing).eflags as libc::c_int & MFE_PUSHED as libc::c_int != 0) {
                    if !(!((*thing).player).is_null()
                        && (*(*thing).player).powers[pw_carry as libc::c_int as usize]
                            as libc::c_int == CR_ROPEHANG as libc::c_int)
                    {
                        if !(!((*thing).player).is_null()
                            && (*thing).state
                                == &mut *states
                                    .as_mut_ptr()
                                    .offset((*(*thing).info).painstate as isize) as *mut state_t
                            && ((*(*thing).player)
                                .powers[pw_flashing as libc::c_int as usize] as libc::c_int
                                > flashingtics as libc::c_int / 4 as libc::c_int
                                    * 3 as libc::c_int
                                && (*(*thing).player)
                                    .powers[pw_flashing as libc::c_int as usize] as libc::c_int
                                    <= flashingtics as libc::c_int))
                        {
                            moved = false_0 as libc::c_int;
                            touching = moved;
                            inFOF = touching;
                            if (*p).roverpusher != 0 {
                                let mut top: fixed_t = 0;
                                let mut bottom: fixed_t = 0;
                                top = P_MobjCeilingZ(
                                    thing,
                                    referrer,
                                    sec,
                                    (*thing).x,
                                    (*thing).y,
                                    0 as *mut line_t,
                                    (referrer == sec) as libc::c_int,
                                    true_0 as libc::c_int,
                                );
                                bottom = P_MobjFloorZ(
                                    thing,
                                    referrer,
                                    sec,
                                    (*thing).x,
                                    (*thing).y,
                                    0 as *mut line_t,
                                    (referrer != sec) as libc::c_int,
                                    true_0 as libc::c_int,
                                );
                                if (*thing).eflags as libc::c_int
                                    & MFE_VERTICALFLIP as libc::c_int != 0
                                {
                                    if bottom > (*thing).z + (*thing).height
                                        || top < (*thing).z + ((*thing).height >> 1 as libc::c_int)
                                    {
                                        current_block_62 = 17216689946888361452;
                                    } else {
                                        if (*thing).z < bottom {
                                            touching = true_0 as libc::c_int;
                                        }
                                        if (*thing).z + ((*thing).height >> 1 as libc::c_int)
                                            > bottom
                                        {
                                            inFOF = true_0 as libc::c_int;
                                        }
                                        current_block_62 = 11913429853522160501;
                                    }
                                } else if top < (*thing).z
                                    || bottom
                                        > (*thing).z + ((*thing).height >> 1 as libc::c_int)
                                {
                                    current_block_62 = 17216689946888361452;
                                } else {
                                    if (*thing).z + (*thing).height > top {
                                        touching = true_0 as libc::c_int;
                                    }
                                    if (*thing).z + ((*thing).height >> 1 as libc::c_int) < top
                                    {
                                        inFOF = true_0 as libc::c_int;
                                    }
                                    current_block_62 = 11913429853522160501;
                                }
                            } else {
                                if (*thing).z
                                    == P_MobjFloorZ(
                                        thing,
                                        sec,
                                        sec,
                                        (*thing).x,
                                        (*thing).y,
                                        0 as *mut line_t,
                                        (sec != sec) as libc::c_int,
                                        true_0 as libc::c_int,
                                    )
                                {
                                    touching = true_0 as libc::c_int;
                                } else if udmf != 0
                                    || (*p).type_0 as libc::c_uint
                                        != p_current as libc::c_int as libc::c_uint
                                    || z_mag != 0 as libc::c_int
                                {
                                    inFOF = true_0 as libc::c_int;
                                }
                                current_block_62 = 11913429853522160501;
                            }
                            match current_block_62 {
                                17216689946888361452 => {}
                                _ => {
                                    if !(touching == 0 && inFOF == 0) {
                                        if inFOF != 0
                                            || (*p).type_0 as libc::c_uint
                                                == p_current as libc::c_int as libc::c_uint && touching != 0
                                        {
                                            xspeed = x_mag;
                                            yspeed = y_mag;
                                            zspeed = z_mag;
                                            moved = true_0 as libc::c_int;
                                        } else if (*p).type_0 as libc::c_uint
                                            == p_wind as libc::c_int as libc::c_uint && touching != 0
                                        {
                                            xspeed = x_mag >> 1 as libc::c_int;
                                            yspeed = y_mag >> 1 as libc::c_int;
                                            zspeed = z_mag >> 1 as libc::c_int;
                                            moved = true_0 as libc::c_int;
                                        }
                                        (*thing).momx += xspeed;
                                        (*thing).momy += yspeed;
                                        (*thing).momz += zspeed;
                                        if !((*thing).player).is_null() {
                                            (*(*thing).player).cmomx += xspeed;
                                            (*(*thing).player).cmomy += yspeed;
                                            (*(*thing).player)
                                                .cmomx = FixedMul(
                                                (*(*thing).player).cmomx,
                                                (0xe8 as libc::c_int)
                                                    << 16 as libc::c_int - 8 as libc::c_int,
                                            );
                                            (*(*thing).player)
                                                .cmomy = FixedMul(
                                                (*(*thing).player).cmomy,
                                                (0xe8 as libc::c_int)
                                                    << 16 as libc::c_int - 8 as libc::c_int,
                                            );
                                        }
                                        if (*thing).type_0 as libc::c_uint
                                            == MT_LITTLETUMBLEWEED as libc::c_int as libc::c_uint
                                            || (*thing).type_0 as libc::c_uint
                                                == MT_BIGTUMBLEWEED as libc::c_int as libc::c_uint
                                        {
                                            (*thing).momz
                                                += P_AproxDistance(xspeed, yspeed) >> 2 as libc::c_int;
                                        }
                                        if moved != 0 {
                                            if (*p).slider != 0 && !((*thing).player).is_null() {
                                                let mut jumped: pflags_t = ((*(*thing).player).pflags
                                                    as libc::c_uint
                                                    & (PF_JUMPED as libc::c_int
                                                        | PF_NOJUMPDAMAGE as libc::c_int) as libc::c_uint)
                                                    as pflags_t;
                                                P_ResetPlayer((*thing).player);
                                                if jumped as u64 != 0 {
                                                    (*(*thing).player)
                                                        .pflags = ::core::mem::transmute::<
                                                        libc::c_uint,
                                                        pflags_t,
                                                    >(
                                                        (*(*thing).player).pflags as libc::c_uint
                                                            | jumped as libc::c_uint,
                                                    );
                                                }
                                                (*(*thing).player)
                                                    .pflags = ::core::mem::transmute::<
                                                    libc::c_uint,
                                                    pflags_t,
                                                >(
                                                    (*(*thing).player).pflags as libc::c_uint
                                                        | PF_SLIDING as libc::c_int as libc::c_uint,
                                                );
                                                (*thing)
                                                    .angle = R_PointToAngle2(
                                                    0 as libc::c_int,
                                                    0 as libc::c_int,
                                                    xspeed,
                                                    yspeed,
                                                );
                                                if demoplayback == 0
                                                    || (if (*(*thing).player).pflags as libc::c_uint
                                                        & PF_ANALOGMODE as libc::c_int as libc::c_uint != 0
                                                    {
                                                        CS_LMAOGALOG as libc::c_int
                                                    } else {
                                                        0 as libc::c_int
                                                    })
                                                        | (if (*(*thing).player).pflags as libc::c_uint
                                                            & PF_DIRECTIONCHAR as libc::c_int as libc::c_uint != 0
                                                        {
                                                            CS_STANDARD as libc::c_int
                                                        } else {
                                                            0 as libc::c_int
                                                        }) == CS_LMAOGALOG as libc::c_int
                                                {
                                                    let mut angle: angle_t = (((*(*thing).player).angleturn
                                                        as libc::c_int) << 16 as libc::c_int) as angle_t;
                                                    if ((*thing).angle).wrapping_sub(angle)
                                                        > 0x80000000 as libc::c_uint
                                                    {
                                                        P_SetPlayerAngle(
                                                            (*thing).player,
                                                            angle
                                                                .wrapping_sub(
                                                                    angle.wrapping_sub((*thing).angle)
                                                                        / 8 as libc::c_int as angle_t,
                                                                ),
                                                        );
                                                    } else {
                                                        P_SetPlayerAngle(
                                                            (*thing).player,
                                                            angle
                                                                .wrapping_add(
                                                                    ((*thing).angle).wrapping_sub(angle)
                                                                        / 8 as libc::c_int as angle_t,
                                                                ),
                                                        );
                                                    }
                                                }
                                            }
                                            if (*p).exclusive != 0 {
                                                (*thing)
                                                    .eflags = ((*thing).eflags as libc::c_int
                                                    | MFE_PUSHED as libc::c_int) as uint16_t;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        node = (*node).m_thinglist_next;
    }
}
unsafe extern "C" fn P_SpawnPushers() {
    let mut i: size_t = 0;
    let mut l: *mut line_t = lines;
    let mut s: int32_t = 0;
    let mut length: fixed_t = 0;
    let mut hspeed: fixed_t = 0;
    let mut dx: fixed_t = 0;
    let mut dy: fixed_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < numlines {
        if !((*l).special as libc::c_int != 541 as libc::c_int) {
            length = R_PointToDist2(
                (*(*l).v2).x,
                (*(*l).v2).y,
                (*(*l).v1).x,
                (*(*l).v1).y,
            );
            hspeed = (*l).args[1 as libc::c_int as usize] << 16 as libc::c_int;
            dx = FixedMul(FixedDiv((*l).dx, length), hspeed);
            dy = FixedMul(FixedDiv((*l).dy, length), hspeed);
            if (*l).args[0 as libc::c_int as usize] == 0 as libc::c_int {
                Add_Pusher(
                    (*l).args[3 as libc::c_int as usize] as pushertype_e,
                    dx,
                    dy,
                    (*l).args[2 as libc::c_int as usize] << 16 as libc::c_int,
                    ((*l).frontsector).offset_from(sectors) as libc::c_long as int32_t,
                    -(1 as libc::c_int),
                    ((*l).args[4 as libc::c_int as usize]
                        & TMPF_NONEXCLUSIVE as libc::c_int == 0) as libc::c_int,
                    ((*l).args[4 as libc::c_int as usize] & TMPF_SLIDE as libc::c_int
                        != 0) as libc::c_int,
                );
            } else {
                let mut ICNT_8667: size_t = 0 as libc::c_int as size_t;
                loop {
                    s = Tag_Iterate_Sectors(
                        (*l).args[0 as libc::c_int as usize] as mtag_t,
                        ICNT_8667,
                    );
                    if !(s >= 0 as libc::c_int) {
                        break;
                    }
                    Add_Pusher(
                        (*l).args[3 as libc::c_int as usize] as pushertype_e,
                        dx,
                        dy,
                        (*l).args[2 as libc::c_int as usize] << 16 as libc::c_int,
                        s,
                        -(1 as libc::c_int),
                        ((*l).args[4 as libc::c_int as usize]
                            & TMPF_NONEXCLUSIVE as libc::c_int == 0) as libc::c_int,
                        ((*l).args[4 as libc::c_int as usize] & TMPF_SLIDE as libc::c_int
                            != 0) as libc::c_int,
                    );
                    ICNT_8667 = ICNT_8667.wrapping_add(1);
                    ICNT_8667;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
        l = l.offset(1);
        l;
    }
}
