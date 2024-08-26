use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn strlcat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn I_UpdateTime(timescale: fixed_t);
    fn I_GetTime() -> tic_t;
    static mut cv_timescale: consvar_t;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut gamestate: gamestate_t;
    static mut titlemapinaction: uint8_t;
    static mut ultimatemode: uint8_t;
    static mut gameaction: gameaction_t;
    static mut botingame: boolean;
    static mut botskin: uint8_t;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    fn sizeu2(num: size_t) -> *mut libc::c_char;
    static mut VERSION: libc::c_int;
    static mut SUBVERSION: libc::c_int;
    fn COM_AddCommand(name: *const libc::c_char, func: com_func_t, flags: com_flags_t);
    fn COM_Argc() -> size_t;
    fn COM_Argv(arg: size_t) -> *const libc::c_char;
    fn COM_BufAddTextEx(btext: *const libc::c_char, flags: com_flags_t);
    fn COM_BufInsertTextEx(btext: *const libc::c_char, flags: com_flags_t);
    fn COM_ImmedExecute(ptext: *const libc::c_char);
    fn COM_BufTicker();
    static mut CV_OnOff: [CV_PossibleValue_t; 0];
    static mut CV_Unsigned: [CV_PossibleValue_t; 0];
    fn CV_RevertNetVars();
    fn CV_CheatsEnabled() -> uint8_t;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut doomcom: *mut doomcom_t;
    fn I_NetSplitAddress(
        address: *mut libc::c_char,
        port: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn I_NetMakeNode(address: *const libc::c_char) -> int8_t;
    static mut I_NetMakeNodewPort: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> int8_t,
    >;
    static mut I_NetOpenSocket: Option::<unsafe extern "C" fn() -> boolean>;
    static mut I_Ban: Option::<unsafe extern "C" fn(int32_t) -> boolean>;
    static mut I_ClearBans: Option::<unsafe extern "C" fn() -> ()>;
    static mut I_GetNodeAddress: Option::<
        unsafe extern "C" fn(int32_t) -> *const libc::c_char,
    >;
    static mut I_GetBanAddress: Option::<
        unsafe extern "C" fn(size_t) -> *const libc::c_char,
    >;
    static mut I_GetBanMask: Option::<
        unsafe extern "C" fn(size_t) -> *const libc::c_char,
    >;
    static mut I_SetBanAddress: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> boolean,
    >;
    static mut bannednode: *mut boolean;
    static mut events: [event_t; 128];
    static mut eventhead: int32_t;
    static mut eventtail: int32_t;
    fn I_OsPolling();
    fn I_Sleep(ms: uint32_t);
    fn I_GetPreciseTime() -> precise_t;
    fn I_UpdateMouseGrab();
    static mut states: [state_t; 6735];
    fn S_StartSound(origin: *const libc::c_void, sound_id: sfxenum_t);
    fn S_UpdateSounds();
    fn S_UpdateClosedCaptions();
    fn I_UpdateNoVsync();
    static mut getbps: int32_t;
    fn Net_GetNetStat() -> boolean;
    fn Net_AckTicker();
    fn HSendPacket(
        node: int32_t,
        reliable: boolean,
        acknum: uint8_t,
        packetlength: size_t,
    ) -> boolean;
    fn HGetPacket() -> boolean;
    fn D_CloseConnection();
    fn Net_UnAcknowledgePacket(node: int32_t);
    fn Net_CloseConnection(node: int32_t);
    fn Net_ConnectionTimeout(node: int32_t);
    static mut rendergametic: tic_t;
    static mut srb2home: [libc::c_char; 256];
    static mut advancedemo: boolean;
    fn W_CachePatchLongName(
        name: *const libc::c_char,
        tag: int32_t,
    ) -> *mut libc::c_void;
    fn W_GetNumForName(name: *const libc::c_char) -> lumpnum_t;
    fn D_ProcessEvents();
    fn D_StartTitle();
    static mut cv_playername: consvar_t;
    static mut cv_playername2: consvar_t;
    static mut cv_nettimeout: consvar_t;
    static mut cv_maxping: consvar_t;
    static mut cv_pingtimeout: consvar_t;
    static mut cv_sleep: consvar_t;
    static mut timedemo_quit: boolean;
    static mut netxcmdnames: [*const libc::c_char; 24];
    static mut consoleplayer: int32_t;
    static mut displayplayer: int32_t;
    static mut secondarydisplayplayer: int32_t;
    static mut mapheaderinfo: [*mut mapheader_t; 1035];
    static mut Gametype_Names: [*const libc::c_char; 136];
    static mut gametic: tic_t;
    static mut debugfile: *mut FILE;
    static mut debugload: int32_t;
    static mut wipegamestate: gamestate_t;
    static mut singletics: boolean;
    static mut splitscreen: boolean;
    static mut fromlevelselect: boolean;
    static mut gametyperules: uint32_t;
    static mut gametype: int16_t;
    static mut multiplayer: boolean;
    static mut addedtogame: boolean;
    static mut netgame: boolean;
    static mut metalrecording: boolean;
    static mut usedCheats: boolean;
    static mut modifiedgame: boolean;
    static mut gamemap: int16_t;
    fn CleanupPlayerName(playernum: int32_t, newname: *const libc::c_char);
    fn EnsurePlayerNameIsGood(name: *mut libc::c_char, playernum: int32_t) -> boolean;
    fn D_SendPlayerConfig();
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
    fn ClearAdminPlayers();
    fn RemoveAdminPlayer(playernum: int32_t);
    static mut cv_servername: consvar_t;
    static mut ms_RoomId: int16_t;
    fn RegisterServer();
    fn UnregisterServer();
    fn MasterClient_Ticker();
    fn GetShortServersList(room: int32_t, id: libc::c_int) -> *mut msg_server_t;
    static mut mapchangepending: int32_t;
    static mut netbuffer: *mut doomdata_t;
    static mut dedicated: boolean;
    static mut connectiontimeout: tic_t;
    static mut paused: uint8_t;
    static mut playeringame: [boolean; 32];
    static mut cv_digitaldeadzone: consvar_t;
    static mut clientGamedata: *mut gamedata_t;
    static mut serverGamedata: *mut gamedata_t;
    fn M_CopyGameData(dest: *mut gamedata_t, src: *mut gamedata_t);
    static mut player_name_changes: [int32_t; 32];
    static mut adminplayers: [int32_t; 32];
    static mut player_names: [[libc::c_char; 22]; 32];
    static mut players: [player_t; 32];
    static mut demorecording: boolean;
    static mut metalplayback: *mut mobj_t;
    static mut cv_downloading: consvar_t;
    fn G_SetUsedCheats(silent: boolean);
    fn G_SetGametype(gametype_0: int16_t);
    fn G_GametypeHasTeams() -> boolean;
    fn G_BuildMapName(map: int32_t) -> *const libc::c_char;
    fn G_MoveTiccmd(
        dest: *mut ticcmd_t,
        src: *const ticcmd_t,
        n: size_t,
    ) -> *mut ticcmd_t;
    fn G_SetGamestate(newstate: gamestate_t);
    fn G_PlatformGametype() -> boolean;
    static mut titledemo: boolean;
    static mut demoplayback: boolean;
    fn G_Ticker(run: boolean);
    fn G_TicsToMilliseconds(tics: tic_t) -> int32_t;
    fn G_BuildTiccmd(cmd: *mut ticcmd_t, realtics: int32_t, ssplayer: uint8_t);
    static mut ticcmd_oldangleturn: [int16_t; 2];
    fn G_AddPlayer(playernum: int32_t);
    fn G_IsSpecialStage(mapnum: int32_t) -> boolean;
    fn G_TagGametype() -> boolean;
    fn G_StopMetalDemo();
    fn G_StopMetalRecording(kill: boolean) -> !;
    fn G_StopDemo();
    fn G_CheckDemoStatus() -> boolean;
    fn ST_Start();
    fn HU_AddChatText(text: *const libc::c_char, playsound: boolean);
    fn HU_Start();
    static mut gamekeydown: [uint8_t; 484];
    static mut gamecontrol: [[int32_t; 2]; 43];
    fn G_MapEventsToControls(ev: *mut event_t);
    fn P_AutoPause() -> boolean;
    static mut ps_tictime: ps_metric_t;
    fn PS_UpdateTickStats();
    fn M_Ticker();
    fn M_ClearMenus(callexitmenufunc: boolean);
    fn P_PlayerFlagBurst(player: *mut player_t, toss: boolean);
    fn P_GivePlayerSpheres(player: *mut player_t, num_spheres: int32_t);
    fn P_GivePlayerRings(player: *mut player_t, num_rings: int32_t);
    fn P_CheckSurvivors();
    fn P_CheckRacers() -> boolean;
    fn P_RemoveMobj(th: *mut mobj_t);
    fn P_ForceLocalAngle(player: *mut player_t, angle: angle_t);
    static mut camera: camera_t;
    fn P_ResetCamera(player: *mut player_t, thiscam: *mut camera_t);
    static mut camera2: camera_t;
    static mut curfadevalue: int8_t;
    fn M_SortServerList();
    static mut leveltime: tic_t;
    static mut mapmd5: [libc::c_uchar; 16];
    fn M_StartMessage(
        string: *const libc::c_char,
        routine: *mut libc::c_void,
        itemtype: menumessagetype_t,
    );
    fn LUA_InvalidatePlayer(player: *mut player_t);
    fn F_MenuPresTicker();
    fn F_TitleScreenTicker(run: boolean);
    fn F_TitleScreenDrawer();
    fn M_DrawTextBox(x: int32_t, y: int32_t, width: int32_t, boxlines: int32_t);
    fn M_Drawer();
    fn F_StartWaitingPlayers();
    fn CON_Ticker();
    fn CON_ToggleOff();
    static mut fileneedednum: int32_t;
    static mut fileneeded: *mut fileneeded_t;
    static mut lastfilenum: int32_t;
    static mut downloadcompletednum: int32_t;
    static mut downloadcompletedsize: uint32_t;
    static mut totalfilesrequestednum: int32_t;
    static mut totalfilesrequestedsize: uint32_t;
    fn FreeFileNeeded();
    fn PutFileNeeded(firstfile: uint16_t) -> *mut uint8_t;
    fn D_ParseFileneeded(
        fileneedednum_parm: int32_t,
        fileneededstr: *mut uint8_t,
        firstfile: uint16_t,
    );
    fn CL_PrepareDownloadSaveGame(tmpsave: *const libc::c_char);
    fn CL_CheckFiles() -> int32_t;
    fn CL_LoadServerFiles() -> boolean;
    fn AddRamToSendQueue(
        node: int32_t,
        data: *mut libc::c_void,
        size: size_t,
        freemethod: freemethod_t,
        fileid: uint8_t,
    );
    fn FileSendTicker();
    fn PT_FileAck();
    fn PT_FileReceived();
    fn FileReceiveTicker();
    fn PT_FileFragment();
    fn CL_CheckDownloadable() -> boolean;
    fn CL_SendFileRequest() -> boolean;
    fn PT_RequestFile(node: int32_t) -> boolean;
    static mut luafiletransfers: *mut luafiletransfer_t;
    static mut waitingforluafiletransfer: boolean;
    static mut waitingforluafilecommand: boolean;
    fn AddLuaFileToSendQueue(node: int32_t, filename: *const libc::c_char) -> boolean;
    fn SV_HandleLuaFileSent(node: uint8_t);
    fn RemoveAllLuaFileTransfers();
    fn CL_PrepareDownloadLuaFile();
    fn CloseNetFile();
    fn nameonlylength(s: *const libc::c_char) -> size_t;
    fn P_SaveNetGame(resending: boolean);
    fn P_LoadNetGame(reloading: boolean) -> boolean;
    static mut save_p: *mut uint8_t;
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
    static mut moviemode: moviemode_t;
    fn M_SaveFrame();
    fn FIL_ReadFileTag(
        name: *const libc::c_char,
        buffer: *mut *mut uint8_t,
        tag: int32_t,
    ) -> size_t;
    fn FIL_FileExists(name: *const libc::c_char) -> boolean;
    static mut automapactive: boolean;
    fn M_RandomKey(a: int32_t) -> int32_t;
    fn P_GetRandSeed() -> uint32_t;
    fn Y_EndIntermission();
    fn SplitScreen_OnChange();
    fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t;
    fn M_CheckParm(check: *const libc::c_char) -> int32_t;
    fn lzf_compress(
        in_data: *const libc::c_void,
        in_len: size_t,
        out_data: *mut libc::c_void,
        out_len: size_t,
    ) -> size_t;
    fn lzf_decompress(
        in_data: *const libc::c_void,
        in_len: size_t,
        out_data: *mut libc::c_void,
        out_len: size_t,
    ) -> size_t;
    fn LUA_HookInt(integer: int32_t, hook: libc::c_int);
    fn LUA_HookBool(value: boolean, hook: libc::c_int);
    fn LUA_HookPlayerQuit(_: *mut player_t, _: kickreason_t);
    fn LUA_HookViewpointSwitch(
        player: *mut player_t,
        newdisplayplayer: *mut player_t,
        forced: boolean,
    ) -> libc::c_int;
    static mut mousegrabbedbylua: boolean;
    fn md5_buffer(
        buffer: *const libc::c_char,
        len: size_t,
        resblock: *mut libc::c_void,
    ) -> *mut libc::c_void;
    static mut Joystick: JoyType_t;
    fn V_DrawStretchyFixedPatch(
        x: fixed_t,
        y: fixed_t,
        pscale: fixed_t,
        vscale: fixed_t,
        scrn: int32_t,
        patch: *mut patch_t,
        colormap: *const uint8_t,
    );
    fn V_DrawFill(x: int32_t, y: int32_t, w: int32_t, h: int32_t, c: int32_t);
    fn V_DrawFlatFill(
        x: int32_t,
        y: int32_t,
        w: int32_t,
        h: int32_t,
        flatnum: lumpnum_t,
    );
    fn V_DrawFadeScreen(color: uint16_t, strength: uint8_t);
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
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
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
pub type precise_t = uint64_t;
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
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
pub type com_func_t = Option::<unsafe extern "C" fn() -> ()>;
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
pub struct doomcom_t {
    pub id: int32_t,
    pub intnum: int16_t,
    pub command: int16_t,
    pub remotenode: int16_t,
    pub datalength: int16_t,
    pub numnodes: int16_t,
    pub ticdup: int16_t,
    pub extratics: int16_t,
    pub gametype: int16_t,
    pub savegame: int16_t,
    pub map: int16_t,
    pub consoleplayer: int16_t,
    pub numslots: int16_t,
    pub data: [libc::c_char; 1450],
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const MFE_REVERSESUPER: C2RustUnnamed_3 = 12288;
pub const MFE_FORCENOSUPER: C2RustUnnamed_3 = 8192;
pub const MFE_FORCESUPER: C2RustUnnamed_3 = 4096;
pub const MFE_TRACERANGLE: C2RustUnnamed_3 = 2048;
pub const MFE_APPLYPMOMZ: C2RustUnnamed_3 = 1024;
pub const MFE_SPRUNG: C2RustUnnamed_3 = 512;
pub const MFE_PUSHED: C2RustUnnamed_3 = 256;
pub const MFE_TOUCHLAVA: C2RustUnnamed_3 = 128;
pub const MFE_GOOWATER: C2RustUnnamed_3 = 64;
pub const MFE_VERTICALFLIP: C2RustUnnamed_3 = 32;
pub const MFE_JUSTSTEPPEDDOWN: C2RustUnnamed_3 = 16;
pub const MFE_UNDERWATER: C2RustUnnamed_3 = 8;
pub const MFE_TOUCHWATER: C2RustUnnamed_3 = 4;
pub const MFE_JUSTHITFLOOR: C2RustUnnamed_3 = 2;
pub const MFE_ONGROUND: C2RustUnnamed_3 = 1;
pub type banreason_t = banreason_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct banreason_s {
    pub reason: *mut libc::c_char,
    pub prev: *mut banreason_s,
    pub next: *mut banreason_s,
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
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NUMPOWERS: C2RustUnnamed_4 = 30;
pub const pw_strong: C2RustUnnamed_4 = 29;
pub const pw_ignorelatch: C2RustUnnamed_4 = 28;
pub const pw_justlaunched: C2RustUnnamed_4 = 27;
pub const pw_dye: C2RustUnnamed_4 = 26;
pub const pw_nocontrol: C2RustUnnamed_4 = 25;
pub const pw_nights_linkfreeze: C2RustUnnamed_4 = 24;
pub const pw_nights_helper: C2RustUnnamed_4 = 23;
pub const pw_nights_superloop: C2RustUnnamed_4 = 22;
pub const pw_emeralds: C2RustUnnamed_4 = 21;
pub const pw_railring: C2RustUnnamed_4 = 20;
pub const pw_explosionring: C2RustUnnamed_4 = 19;
pub const pw_grenadering: C2RustUnnamed_4 = 18;
pub const pw_scatterring: C2RustUnnamed_4 = 17;
pub const pw_bouncering: C2RustUnnamed_4 = 16;
pub const pw_automaticring: C2RustUnnamed_4 = 15;
pub const pw_infinityring: C2RustUnnamed_4 = 14;
pub const pw_gravityboots: C2RustUnnamed_4 = 13;
pub const pw_super: C2RustUnnamed_4 = 12;
pub const pw_noautobrake: C2RustUnnamed_4 = 11;
pub const pw_justsprung: C2RustUnnamed_4 = 10;
pub const pw_pushing: C2RustUnnamed_4 = 9;
pub const pw_extralife: C2RustUnnamed_4 = 8;
pub const pw_spacetime: C2RustUnnamed_4 = 7;
pub const pw_underwater: C2RustUnnamed_4 = 6;
pub const pw_tailsfly: C2RustUnnamed_4 = 5;
pub const pw_carry: C2RustUnnamed_4 = 4;
pub const pw_shield: C2RustUnnamed_4 = 3;
pub const pw_flashing: C2RustUnnamed_4 = 2;
pub const pw_sneakers: C2RustUnnamed_4 = 1;
pub const pw_invulnerability: C2RustUnnamed_4 = 0;
pub type player_t = player_s;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union msg_header_t {
    pub buffer: [libc::c_char; 16],
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct msg_server_t {
    pub header: msg_header_t,
    pub ip: [libc::c_char; 40],
    pub port: [libc::c_char; 8],
    pub name: [libc::c_char; 32],
    pub room: int32_t,
    pub version: [libc::c_char; 8],
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const NUMPACKETTYPE: C2RustUnnamed_5 = 35;
pub const PT_PING: C2RustUnnamed_5 = 34;
pub const PT_MOREFILESNEEDED: C2RustUnnamed_5 = 33;
pub const PT_TELLFILESNEEDED: C2RustUnnamed_5 = 32;
pub const PT_LOGIN: C2RustUnnamed_5 = 31;
pub const PT_NODETIMEOUT: C2RustUnnamed_5 = 30;
pub const PT_CLIENTJOIN: C2RustUnnamed_5 = 29;
pub const PT_TEXTCMD2: C2RustUnnamed_5 = 28;
pub const PT_TEXTCMD: C2RustUnnamed_5 = 27;
pub const PT_FILERECEIVED: C2RustUnnamed_5 = 26;
pub const PT_FILEACK: C2RustUnnamed_5 = 25;
pub const PT_FILEFRAGMENT: C2RustUnnamed_5 = 24;
pub const PT_CANFAIL: C2RustUnnamed_5 = 24;
pub const PT_BASICKEEPALIVE: C2RustUnnamed_5 = 23;
pub const PT_HASLUAFILE: C2RustUnnamed_5 = 22;
pub const PT_ASKLUAFILE: C2RustUnnamed_5 = 21;
pub const PT_SENDINGLUAFILE: C2RustUnnamed_5 = 20;
pub const PT_RECEIVEDGAMESTATE: C2RustUnnamed_5 = 19;
pub const PT_CANRECEIVEGAMESTATE: C2RustUnnamed_5 = 18;
pub const PT_WILLRESENDGAMESTATE: C2RustUnnamed_5 = 17;
pub const PT_ASKINFOVIAMS: C2RustUnnamed_5 = 16;
pub const PT_REQUESTFILE: C2RustUnnamed_5 = 15;
pub const PT_PLAYERINFO: C2RustUnnamed_5 = 14;
pub const PT_SERVERINFO: C2RustUnnamed_5 = 13;
pub const PT_ASKINFO: C2RustUnnamed_5 = 12;
pub const PT_CLIENTQUIT: C2RustUnnamed_5 = 11;
pub const PT_SERVERSHUTDOWN: C2RustUnnamed_5 = 10;
pub const PT_SERVERREFUSE: C2RustUnnamed_5 = 9;
pub const PT_SERVERTICS: C2RustUnnamed_5 = 8;
pub const PT_NODEKEEPALIVEMIS: C2RustUnnamed_5 = 7;
pub const PT_NODEKEEPALIVE: C2RustUnnamed_5 = 6;
pub const PT_CLIENT2MIS: C2RustUnnamed_5 = 5;
pub const PT_CLIENT2CMD: C2RustUnnamed_5 = 4;
pub const PT_CLIENTMIS: C2RustUnnamed_5 = 3;
pub const PT_CLIENTCMD: C2RustUnnamed_5 = 2;
pub const PT_SERVERCFG: C2RustUnnamed_5 = 1;
pub const PT_NOTHING: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct clientcmd_pak {
    pub client_tic: uint8_t,
    pub resendfrom: uint8_t,
    pub consistancy: int16_t,
    pub cmd: ticcmd_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct client2cmd_pak {
    pub client_tic: uint8_t,
    pub resendfrom: uint8_t,
    pub consistancy: int16_t,
    pub cmd: ticcmd_t,
    pub cmd2: ticcmd_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct servertics_pak {
    pub starttic: tic_t,
    pub numtics: uint8_t,
    pub numslots: uint8_t,
    pub cmds: [ticcmd_t; 45],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct serverconfig_pak {
    pub serverplayer: uint8_t,
    pub totalslotnum: uint8_t,
    pub gametic: tic_t,
    pub clientnode: uint8_t,
    pub gamestate: uint8_t,
    pub gametype: uint8_t,
    pub modifiedgame: uint8_t,
    pub usedCheats: uint8_t,
    pub server_context: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct filetx_pak {
    pub fileid: uint8_t,
    pub filesize: uint32_t,
    pub iteration: uint8_t,
    pub position: uint32_t,
    pub size: uint16_t,
    pub data: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fileacksegment_t {
    pub start: uint32_t,
    pub acks: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct fileack_pak {
    pub fileid: uint8_t,
    pub iteration: uint8_t,
    pub numsegments: uint8_t,
    pub segments: [fileacksegment_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct clientconfig_pak {
    pub modversion: uint8_t,
    pub application: [libc::c_char; 16],
    pub localplayers: uint8_t,
    pub mode: uint8_t,
    pub names: [[libc::c_char; 21]; 2],
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const REFUSE_BANNED: C2RustUnnamed_6 = 3;
pub const REFUSE_SLOTS_FULL: C2RustUnnamed_6 = 2;
pub const REFUSE_JOINS_DISABLED: C2RustUnnamed_6 = 1;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct serverinfo_pak {
    pub _255: uint8_t,
    pub packetversion: uint8_t,
    pub application: [libc::c_char; 16],
    pub version: uint8_t,
    pub subversion: uint8_t,
    pub numberofplayer: uint8_t,
    pub maxplayer: uint8_t,
    pub refusereason: uint8_t,
    pub gametypename: [libc::c_char; 24],
    pub modifiedgame: uint8_t,
    pub cheatsenabled: uint8_t,
    pub flags: uint8_t,
    pub fileneedednum: uint8_t,
    pub time: tic_t,
    pub leveltime: tic_t,
    pub servername: [libc::c_char; 32],
    pub mapname: [libc::c_char; 8],
    pub maptitle: [libc::c_char; 33],
    pub mapmd5: [libc::c_uchar; 16],
    pub actnum: uint8_t,
    pub iszone: uint8_t,
    pub fileneeded: [uint8_t; 915],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct serverrefuse_pak {
    pub reason: [libc::c_char; 255],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct askinfo_pak {
    pub version: uint8_t,
    pub time: tic_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct msaskinfo_pak {
    pub clientaddr: [libc::c_char; 22],
    pub time: tic_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct plrinfo {
    pub num: uint8_t,
    pub name: [libc::c_char; 22],
    pub address: [uint8_t; 4],
    pub team: uint8_t,
    pub skin: uint8_t,
    pub data: uint8_t,
    pub score: uint32_t,
    pub timeinserver: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct plrconfig {
    pub name: [libc::c_char; 22],
    pub skin: uint8_t,
    pub color: uint16_t,
    pub pflags: uint32_t,
    pub score: uint32_t,
    pub ctfteam: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct filesneededconfig_pak {
    pub first: int32_t,
    pub num: uint8_t,
    pub more: uint8_t,
    pub files: [uint8_t; 915],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct doomdata_t {
    pub checksum: uint32_t,
    pub ack: uint8_t,
    pub ackreturn: uint8_t,
    pub packettype: uint8_t,
    pub reserved: uint8_t,
    pub u: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub clientpak: clientcmd_pak,
    pub client2pak: client2cmd_pak,
    pub serverpak: servertics_pak,
    pub servercfg: serverconfig_pak,
    pub textcmd: [uint8_t; 257],
    pub filetxpak: filetx_pak,
    pub fileack: fileack_pak,
    pub filereceived: uint8_t,
    pub clientcfg: clientconfig_pak,
    pub md5sum: [uint8_t; 16],
    pub serverinfo: serverinfo_pak,
    pub serverrefuse: serverrefuse_pak,
    pub askinfo: askinfo_pak,
    pub msaskinfo: msaskinfo_pak,
    pub playerinfo: [plrinfo; 32],
    pub playerconfig: [plrconfig; 32],
    pub filesneedednum: int32_t,
    pub filesneededcfg: filesneededconfig_pak,
    pub pingtable: [uint32_t; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct serverelem_t {
    pub node: int8_t,
    pub info: serverinfo_pak,
}
pub type kickreason_t = libc::c_uint;
pub const KR_LEAVE: kickreason_t = 6;
pub const KR_BAN: kickreason_t = 5;
pub const KR_TIMEOUT: kickreason_t = 4;
pub const KR_SYNCH: kickreason_t = 3;
pub const KR_PINGLIMIT: kickreason_t = 2;
pub const KR_KICK: kickreason_t = 1;
pub const CL_CONNECTED: cl_mode_t = 7;
pub type cl_mode_t = libc::c_uint;
pub const CL_CONFIRMCONNECT: cl_mode_t = 10;
pub const CL_ASKFULLFILELIST: cl_mode_t = 9;
pub const CL_ABORTED: cl_mode_t = 8;
pub const CL_DOWNLOADSAVEGAME: cl_mode_t = 6;
pub const CL_WAITJOINRESPONSE: cl_mode_t = 5;
pub const CL_LOADFILES: cl_mode_t = 4;
pub const CL_ASKJOIN: cl_mode_t = 3;
pub const CL_DOWNLOADFILES: cl_mode_t = 2;
pub const CL_CHECKFILES: cl_mode_t = 1;
pub const CL_SEARCHING: cl_mode_t = 0;
pub type moviemode_t = libc::c_uint;
pub const MM_SCREENSHOT: moviemode_t = 3;
pub const MM_GIF: moviemode_t = 2;
pub const MM_APNG: moviemode_t = 1;
pub const MM_OFF: moviemode_t = 0;
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
pub const PU_HUDGFX: C2RustUnnamed_12 = 19;
pub type snake_bonustype_s = libc::c_uint;
pub const SNAKE_NUM_BONUSES: snake_bonustype_s = 8;
pub const SNAKE_BONUS_EGGMAN: snake_bonustype_s = 7;
pub const SNAKE_BONUS_REVERSE: snake_bonustype_s = 6;
pub const SNAKE_BONUS_SCISSORS: snake_bonustype_s = 5;
pub const SNAKE_BONUS_NUKE: snake_bonustype_s = 4;
pub const SNAKE_BONUS_GHOST: snake_bonustype_s = 3;
pub const SNAKE_BONUS_FAST: snake_bonustype_s = 2;
pub const SNAKE_BONUS_SLOW: snake_bonustype_s = 1;
pub const SNAKE_BONUS_NONE: snake_bonustype_s = 0;
pub type snake_t = snake_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snake_s {
    pub paused: boolean,
    pub pausepressed: boolean,
    pub time: tic_t,
    pub nextupdate: tic_t,
    pub gameover: boolean,
    pub background: uint8_t,
    pub snakelength: uint16_t,
    pub snakebonus: snake_bonustype_s,
    pub snakebonustime: tic_t,
    pub snakex: [uint8_t; 200],
    pub snakey: [uint8_t; 200],
    pub snakedir: [uint8_t; 200],
    pub applex: uint8_t,
    pub appley: uint8_t,
    pub bonustype: snake_bonustype_s,
    pub bonusx: uint8_t,
    pub bonusy: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fileneeded_t {
    pub filename: [libc::c_char; 512],
    pub md5sum: [uint8_t; 16],
    pub status: filestatus_t,
    pub willsend: uint8_t,
    pub folder: uint8_t,
    pub type_0: fileneededtype_t,
    pub justdownloaded: boolean,
    pub file: *mut FILE,
    pub receivedfragments: *mut boolean,
    pub fragmentsize: uint32_t,
    pub iteration: uint8_t,
    pub ackpacket: *mut fileack_pak,
    pub currentsize: uint32_t,
    pub totalsize: uint32_t,
    pub ackresendposition: uint32_t,
}
pub type fileneededtype_t = libc::c_uint;
pub const FILENEEDED_LUAFILE: fileneededtype_t = 2;
pub const FILENEEDED_SAVEGAME: fileneededtype_t = 1;
pub const FILENEEDED_WAD: fileneededtype_t = 0;
pub type filestatus_t = libc::c_uint;
pub const FS_MD5SUMBAD: filestatus_t = 6;
pub const FS_OPEN: filestatus_t = 5;
pub const FS_DOWNLOADING: filestatus_t = 4;
pub const FS_REQUESTED: filestatus_t = 3;
pub const FS_FOUND: filestatus_t = 2;
pub const FS_NOTFOUND: filestatus_t = 1;
pub const FS_NOTCHECKED: filestatus_t = 0;
pub const GC_BACKWARD: C2RustUnnamed_10 = 2;
pub const GC_FORWARD: C2RustUnnamed_10 = 1;
pub const GC_STRAFERIGHT: C2RustUnnamed_10 = 4;
pub const GC_STRAFELEFT: C2RustUnnamed_10 = 3;
pub type JoyType_t = JoyType_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JoyType_s {
    pub bJoyNeedPoll: int32_t,
    pub bGamepadStyle: int32_t,
}
pub const GC_PAUSE: C2RustUnnamed_10 = 34;
pub const GC_JUMP: C2RustUnnamed_10 = 32;
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
pub type textcmdtic_t = textcmdtic_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textcmdtic_s {
    pub tic: tic_t,
    pub playercmds: [*mut textcmdplayer_t; 4],
    pub next: *mut textcmdtic_s,
}
pub type textcmdplayer_t = textcmdplayer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textcmdplayer_s {
    pub playernum: int32_t,
    pub cmd: [uint8_t; 256],
    pub next: *mut textcmdplayer_s,
}
pub type menumessagetype_t = libc::c_uint;
pub const MM_EVENTHANDLER: menumessagetype_t = 2;
pub const MM_YESNO: menumessagetype_t = 1;
pub const MM_NOTHING: menumessagetype_t = 0;
pub const KEY_JOY1: C2RustUnnamed_9 = 264;
pub const PU_STATIC: C2RustUnnamed_12 = 1;
pub const LFTNS_SENDING: luafiletransfernodestatus_t = 3;
pub type luafiletransfernodestatus_t = libc::c_uint;
pub const LFTNS_SENT: luafiletransfernodestatus_t = 4;
pub const LFTNS_ASKED: luafiletransfernodestatus_t = 2;
pub const LFTNS_WAITING: luafiletransfernodestatus_t = 1;
pub const LFTNS_NONE: luafiletransfernodestatus_t = 0;
pub type luafiletransfer_t = luafiletransfer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luafiletransfer_s {
    pub filename: *mut libc::c_char,
    pub realfilename: *mut libc::c_char,
    pub mode: [libc::c_char; 4],
    pub id: int32_t,
    pub ongoing: boolean,
    pub nodestatus: [luafiletransfernodestatus_t; 127],
    pub nodetimeouts: [tic_t; 127],
    pub next: *mut luafiletransfer_s,
}
pub type freemethod_t = libc::c_uint;
pub const SF_NOFREERAM: freemethod_t = 3;
pub const SF_RAM: freemethod_t = 2;
pub const SF_Z_RAM: freemethod_t = 1;
pub const SF_FILE: freemethod_t = 0;
pub const hook_GameQuit: C2RustUnnamed_13 = 23;
pub const TITLEMAP_OFF: C2RustUnnamed_11 = 0;
pub const hook_PlayerJoin: C2RustUnnamed_13 = 3;
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
pub type subsector_t = subsector_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub p: precise_t,
    pub i: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ps_metric_t {
    pub value: C2RustUnnamed_8,
    pub history: *mut libc::c_void,
}
pub type C2RustUnnamed_9 = libc::c_uint;
pub const NUMINPUTS: C2RustUnnamed_9 = 484;
pub const KEY_2MOUSEWHEELDOWN: C2RustUnnamed_9 = 483;
pub const KEY_2MOUSEWHEELUP: C2RustUnnamed_9 = 482;
pub const KEY_MOUSEWHEELDOWN: C2RustUnnamed_9 = 481;
pub const KEY_MOUSEWHEELUP: C2RustUnnamed_9 = 480;
pub const KEY_DBL2HAT1: C2RustUnnamed_9 = 464;
pub const KEY_DBL2JOY1: C2RustUnnamed_9 = 432;
pub const KEY_DBL2MOUSE1: C2RustUnnamed_9 = 424;
pub const KEY_2HAT1: C2RustUnnamed_9 = 408;
pub const KEY_2JOY1: C2RustUnnamed_9 = 376;
pub const KEY_2MOUSE1: C2RustUnnamed_9 = 368;
pub const KEY_DBLHAT1: C2RustUnnamed_9 = 352;
pub const KEY_DBLJOY1: C2RustUnnamed_9 = 320;
pub const KEY_DBLMOUSE1: C2RustUnnamed_9 = 312;
pub const KEY_HAT1: C2RustUnnamed_9 = 296;
pub const KEY_MOUSE1: C2RustUnnamed_9 = 256;
pub type C2RustUnnamed_10 = libc::c_uint;
pub const NUM_GAMECONTROLS: C2RustUnnamed_10 = 43;
pub const GC_CUSTOM3: C2RustUnnamed_10 = 42;
pub const GC_CUSTOM2: C2RustUnnamed_10 = 41;
pub const GC_CUSTOM1: C2RustUnnamed_10 = 40;
pub const GC_VIEWPOINTPREV: C2RustUnnamed_10 = 39;
pub const GC_VIEWPOINTNEXT: C2RustUnnamed_10 = 38;
pub const GC_RECORDGIF: C2RustUnnamed_10 = 37;
pub const GC_SCREENSHOT: C2RustUnnamed_10 = 36;
pub const GC_SYSTEMMENU: C2RustUnnamed_10 = 35;
pub const GC_CONSOLE: C2RustUnnamed_10 = 33;
pub const GC_SCORES: C2RustUnnamed_10 = 31;
pub const GC_TEAMKEY: C2RustUnnamed_10 = 30;
pub const GC_TALKKEY: C2RustUnnamed_10 = 29;
pub const GC_MOUSEAIMING: C2RustUnnamed_10 = 28;
pub const GC_CENTERVIEW: C2RustUnnamed_10 = 27;
pub const GC_LOOKDOWN: C2RustUnnamed_10 = 26;
pub const GC_LOOKUP: C2RustUnnamed_10 = 25;
pub const GC_CAMRESET: C2RustUnnamed_10 = 24;
pub const GC_CAMTOGGLE: C2RustUnnamed_10 = 23;
pub const GC_SPIN: C2RustUnnamed_10 = 22;
pub const GC_TOSSFLAG: C2RustUnnamed_10 = 21;
pub const GC_FIRENORMAL: C2RustUnnamed_10 = 20;
pub const GC_FIRE: C2RustUnnamed_10 = 19;
pub const GC_WEPSLOT10: C2RustUnnamed_10 = 18;
pub const GC_WEPSLOT9: C2RustUnnamed_10 = 17;
pub const GC_WEPSLOT8: C2RustUnnamed_10 = 16;
pub const GC_WEPSLOT7: C2RustUnnamed_10 = 15;
pub const GC_WEPSLOT6: C2RustUnnamed_10 = 14;
pub const GC_WEPSLOT5: C2RustUnnamed_10 = 13;
pub const GC_WEPSLOT4: C2RustUnnamed_10 = 12;
pub const GC_WEPSLOT3: C2RustUnnamed_10 = 11;
pub const GC_WEPSLOT2: C2RustUnnamed_10 = 10;
pub const GC_WEPSLOT1: C2RustUnnamed_10 = 9;
pub const GC_WEAPONPREV: C2RustUnnamed_10 = 8;
pub const GC_WEAPONNEXT: C2RustUnnamed_10 = 7;
pub const GC_TURNRIGHT: C2RustUnnamed_10 = 6;
pub const GC_TURNLEFT: C2RustUnnamed_10 = 5;
pub const GC_NULL: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = libc::c_uint;
pub const TITLEMAP_RUNNING: C2RustUnnamed_11 = 2;
pub const TITLEMAP_LOADING: C2RustUnnamed_11 = 1;
pub type C2RustUnnamed_12 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_12 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_12 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_12 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_12 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_12 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_12 = 51;
pub const PU_LEVEL: C2RustUnnamed_12 = 50;
pub const PU_CACHE: C2RustUnnamed_12 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_12 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_12 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_12 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_12 = 21;
pub const PU_SPRITE: C2RustUnnamed_12 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_12 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_12 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_12 = 15;
pub const PU_PATCH: C2RustUnnamed_12 = 14;
pub const PU_MUSIC: C2RustUnnamed_12 = 12;
pub const PU_SOUND: C2RustUnnamed_12 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_12 = 3;
pub const PU_LUA: C2RustUnnamed_12 = 2;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const hook_MAX: C2RustUnnamed_13 = 30;
pub const hook_KeyUp: C2RustUnnamed_13 = 29;
pub const hook_KeyDown: C2RustUnnamed_13 = 28;
pub const hook_PlayerCanEnterSpinGaps: C2RustUnnamed_13 = 27;
pub const hook_PlayerHeight: C2RustUnnamed_13 = 26;
pub const hook_MusicChange: C2RustUnnamed_13 = 25;
pub const hook_PlayerCmd: C2RustUnnamed_13 = 24;
pub const hook_PlayerThink: C2RustUnnamed_13 = 22;
pub const hook_SeenPlayer: C2RustUnnamed_13 = 21;
pub const hook_ViewpointSwitch: C2RustUnnamed_13 = 20;
pub const hook_TeamSwitch: C2RustUnnamed_13 = 19;
pub const hook_IntermissionThinker: C2RustUnnamed_13 = 18;
pub const hook_PlayerQuit: C2RustUnnamed_13 = 17;
pub const hook_PlayerCanDamage: C2RustUnnamed_13 = 16;
pub const hook_ShieldSpecial: C2RustUnnamed_13 = 15;
pub const hook_ShieldSpawn: C2RustUnnamed_13 = 14;
pub const hook_PlayerSpawn: C2RustUnnamed_13 = 13;
pub const hook_PlayerMsg: C2RustUnnamed_13 = 12;
pub const hook_BotTiccmd: C2RustUnnamed_13 = 11;
pub const hook_JumpSpinSpecial: C2RustUnnamed_13 = 10;
pub const hook_SpinSpecial: C2RustUnnamed_13 = 9;
pub const hook_AbilitySpecial: C2RustUnnamed_13 = 8;
pub const hook_JumpSpecial: C2RustUnnamed_13 = 7;
pub const hook_PostThinkFrame: C2RustUnnamed_13 = 6;
pub const hook_ThinkFrame: C2RustUnnamed_13 = 5;
pub const hook_PreThinkFrame: C2RustUnnamed_13 = 4;
pub const hook_MapLoad: C2RustUnnamed_13 = 2;
pub const hook_MapChange: C2RustUnnamed_13 = 1;
pub const hook_NetVars: C2RustUnnamed_13 = 0;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub static mut server: boolean = true_0 as libc::c_int;
#[no_mangle]
pub static mut nodownload: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut serverrunning: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut serverplayer: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut motd: [libc::c_char; 254] = [0; 254];
#[no_mangle]
pub static mut server_context: [libc::c_char; 8] = [0; 8];
#[no_mangle]
pub static mut playernode: [uint8_t; 32] = [0; 32];
#[no_mangle]
pub static mut playeraddress: [[libc::c_char; 64]; 32] = [[0; 64]; 32];
#[no_mangle]
pub static mut jointimeout: tic_t = (10 as libc::c_int * 35 as libc::c_int) as tic_t;
static mut sendingsavegame: [boolean; 127] = [0; 127];
static mut resendingsavegame: [boolean; 127] = [0; 127];
static mut savegameresendcooldown: [tic_t; 127] = [0; 127];
static mut freezetimeout: [tic_t; 127] = [0; 127];
static mut joindelay: tic_t = 0 as libc::c_int as tic_t;
#[no_mangle]
pub static mut pingmeasurecount: uint16_t = 1 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut realpingtable: [uint32_t; 32] = [0; 32];
#[no_mangle]
pub static mut playerpingtable: [uint32_t; 32] = [0; 32];
#[no_mangle]
pub static mut nodetoplayer: [int8_t; 127] = [0; 127];
#[no_mangle]
pub static mut nodetoplayer2: [int8_t; 127] = [0; 127];
#[no_mangle]
pub static mut playerpernode: [uint8_t; 127] = [0; 127];
#[no_mangle]
pub static mut nodeingame: [boolean; 127] = [0; 127];
#[no_mangle]
pub static mut servermaxping: tic_t = 800 as libc::c_int as tic_t;
static mut nettics: [tic_t; 127] = [0; 127];
static mut supposedtics: [tic_t; 127] = [0; 127];
static mut nodewaiting: [uint8_t; 127] = [0; 127];
static mut firstticstosend: tic_t = 0;
static mut tictoclear: tic_t = 0 as libc::c_int as tic_t;
static mut maketic: tic_t = 0;
static mut consistancy: [int16_t; 1024] = [0; 1024];
static mut player_joining: uint8_t = false_0 as libc::c_int as uint8_t;
#[no_mangle]
pub static mut hu_redownloadinggamestate: uint8_t = 0 as libc::c_int as uint8_t;
#[no_mangle]
pub static mut hu_stopped: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut cv_dedicatedidletime: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"dedicatedidletime\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"10\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_Unsigned.as_ptr() as *mut _,
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
pub static mut adminpassmd5: [uint8_t; 16] = [0; 16];
#[no_mangle]
pub static mut adminpasswordset: boolean = false_0 as libc::c_int;
static mut localcmds: ticcmd_t = ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    aiming: 0,
    buttons: 0,
    latency: 0,
};
static mut localcmds2: ticcmd_t = ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    aiming: 0,
    buttons: 0,
    latency: 0,
};
static mut cl_packetmissed: boolean = 0;
static mut mynode: uint8_t = 0;
static mut cl_redownloadinggamestate: boolean = false_0 as libc::c_int;
static mut localtextcmd: [uint8_t; 256] = [0; 256];
static mut localtextcmd2: [uint8_t; 256] = [0; 256];
static mut neededtic: tic_t = 0;
#[no_mangle]
pub static mut servernode: int8_t = 0 as libc::c_int as int8_t;
#[no_mangle]
pub static mut acceptnewnode: boolean = true_0 as libc::c_int;
static mut serverisfull: boolean = false_0 as libc::c_int;
static mut firstconnectattempttime: tic_t = 0 as libc::c_int as tic_t;
#[no_mangle]
pub static mut netcmds: [[ticcmd_t; 32]; 1024] = [[ticcmd_t {
    forwardmove: 0,
    sidemove: 0,
    angleturn: 0,
    aiming: 0,
    buttons: 0,
    latency: 0,
}; 32]; 1024];
static mut textcmds: [*mut textcmdtic_t; 4] = [
    0 as *const textcmdtic_t as *mut textcmdtic_t,
    0 as *const textcmdtic_t as *mut textcmdtic_t,
    0 as *const textcmdtic_t as *mut textcmdtic_t,
    0 as *const textcmdtic_t as *mut textcmdtic_t,
];
#[no_mangle]
pub static mut cv_showjoinaddress: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"showjoinaddress\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Off\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
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
static mut playbackspeed_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 10 as libc::c_int,
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
pub static mut cv_playbackspeed: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"playbackspeed\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"1\0" as *const u8 as *const libc::c_char,
            flags: 0 as libc::c_int,
            PossibleValue: playbackspeed_cons_t.as_ptr() as *mut _,
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
#[inline]
unsafe extern "C" fn G_DcpyTiccmd(
    mut dest: *mut libc::c_void,
    mut src: *const ticcmd_t,
    n: size_t,
) -> *mut libc::c_void {
    let d: size_t = n.wrapping_div(::core::mem::size_of::<ticcmd_t>() as libc::c_ulong);
    let r: size_t = n.wrapping_rem(::core::mem::size_of::<ticcmd_t>() as libc::c_ulong);
    let mut ret: *mut uint8_t = dest as *mut uint8_t;
    if r != 0 {
        M_Memcpy
            .expect("non-null function pointer")(dest, src as *const libc::c_void, n);
    } else if d != 0 {
        G_MoveTiccmd(dest as *mut ticcmd_t, src, d);
    }
    return ret.offset(n as isize) as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn G_ScpyTiccmd(
    mut dest: *mut ticcmd_t,
    mut src: *mut libc::c_void,
    n: size_t,
) -> *mut libc::c_void {
    let d: size_t = n.wrapping_div(::core::mem::size_of::<ticcmd_t>() as libc::c_ulong);
    let r: size_t = n.wrapping_rem(::core::mem::size_of::<ticcmd_t>() as libc::c_ulong);
    let mut ret: *mut uint8_t = src as *mut uint8_t;
    if r != 0 {
        M_Memcpy.expect("non-null function pointer")(dest as *mut libc::c_void, src, n);
    } else if d != 0 {
        G_MoveTiccmd(dest, src as *const ticcmd_t, d);
    }
    return ret.offset(n as isize) as *mut libc::c_void;
}
#[no_mangle]
pub static mut software_MAXPACKETLENGTH: uint16_t = 0;
#[no_mangle]
pub unsafe extern "C" fn ExpandTics(mut low: int32_t, mut node: int32_t) -> tic_t {
    let mut delta: int32_t = 0;
    delta = (low as tic_t)
        .wrapping_sub(nettics[node as usize] & 255 as libc::c_int as tic_t) as int32_t;
    if delta >= -(64 as libc::c_int) && delta <= 64 as libc::c_int {
        return (nettics[node as usize] & !(255 as libc::c_int) as tic_t)
            .wrapping_add(low as tic_t)
    } else if delta > 64 as libc::c_int {
        return (nettics[node as usize] & !(255 as libc::c_int) as tic_t)
            .wrapping_sub(256 as libc::c_int as tic_t)
            .wrapping_add(low as tic_t)
    } else {
        return (nettics[node as usize] & !(255 as libc::c_int) as tic_t)
            .wrapping_add(256 as libc::c_int as tic_t)
            .wrapping_add(low as tic_t)
    };
}
static mut listnetxcmd: [Option::<
    unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> (),
>; 25] = [None; 25];
#[no_mangle]
pub unsafe extern "C" fn RegisterNetXCmd(
    mut id: netxcmd_t,
    mut cmd_f: Option::<unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> ()>,
) {
    listnetxcmd[id as usize] = cmd_f;
}
#[no_mangle]
pub unsafe extern "C" fn SendNetXCmd(
    mut id: netxcmd_t,
    mut param: *const libc::c_void,
    mut nparam: size_t,
) {
    if ((localtextcmd[0 as libc::c_int as usize] as libc::c_int + 2 as libc::c_int)
        as size_t)
        .wrapping_add(nparam) > 256 as libc::c_int as size_t
    {
        CONS_Alert(
            CONS_ERROR,
            b"NetXCmd buffer full, cannot add netcmd %d! (size: %d, needed: %s)\n\0"
                as *const u8 as *const libc::c_char,
            id as libc::c_uint,
            localtextcmd[0 as libc::c_int as usize] as libc::c_int,
            sizeu1(nparam),
        );
        return;
    }
    localtextcmd[0 as libc::c_int
        as usize] = (localtextcmd[0 as libc::c_int as usize]).wrapping_add(1);
    localtextcmd[0 as libc::c_int as usize];
    localtextcmd[localtextcmd[0 as libc::c_int as usize] as usize] = id as uint8_t;
    if !param.is_null() && nparam != 0 {
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            &mut *localtextcmd
                .as_mut_ptr()
                .offset(
                    (*localtextcmd.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut libc::c_void,
            param,
            nparam,
        );
        localtextcmd[0 as libc::c_int
            as usize] = (localtextcmd[0 as libc::c_int as usize] as libc::c_int
            + nparam as uint8_t as libc::c_int) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SendNetXCmd2(
    mut id: netxcmd_t,
    mut param: *const libc::c_void,
    mut nparam: size_t,
) {
    if ((localtextcmd2[0 as libc::c_int as usize] as libc::c_int + 2 as libc::c_int)
        as size_t)
        .wrapping_add(nparam) > 256 as libc::c_int as size_t
    {
        I_Error(
            b"No more place in the buffer for netcmd %d\n\0" as *const u8
                as *const libc::c_char,
            id as libc::c_uint,
        );
    }
    localtextcmd2[0 as libc::c_int
        as usize] = (localtextcmd2[0 as libc::c_int as usize]).wrapping_add(1);
    localtextcmd2[0 as libc::c_int as usize];
    localtextcmd2[localtextcmd2[0 as libc::c_int as usize] as usize] = id as uint8_t;
    if !param.is_null() && nparam != 0 {
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            &mut *localtextcmd2
                .as_mut_ptr()
                .offset(
                    (*localtextcmd2.as_mut_ptr().offset(0 as libc::c_int as isize)
                        as libc::c_int + 1 as libc::c_int) as isize,
                ) as *mut uint8_t as *mut libc::c_void,
            param,
            nparam,
        );
        localtextcmd2[0 as libc::c_int
            as usize] = (localtextcmd2[0 as libc::c_int as usize] as libc::c_int
            + nparam as uint8_t as libc::c_int) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GetFreeXCmdSize() -> uint8_t {
    return (localtextcmd[0 as libc::c_int as usize] as libc::c_int - 2 as libc::c_int)
        as uint8_t;
}
unsafe extern "C" fn D_FreeTextcmd(mut tic: tic_t) {
    let mut tctprev: *mut *mut textcmdtic_t = &mut *textcmds
        .as_mut_ptr()
        .offset((tic & (4 as libc::c_int - 1 as libc::c_int) as tic_t) as isize)
        as *mut *mut textcmdtic_t;
    let mut textcmdtic: *mut textcmdtic_t = *tctprev;
    while !textcmdtic.is_null() && (*textcmdtic).tic != tic {
        tctprev = &mut (*textcmdtic).next;
        textcmdtic = (*textcmdtic).next;
    }
    if !textcmdtic.is_null() {
        let mut i: int32_t = 0;
        *tctprev = (*textcmdtic).next;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            let mut textcmdplayer: *mut textcmdplayer_t = (*textcmdtic)
                .playercmds[i as usize];
            while !textcmdplayer.is_null() {
                let mut tcpnext: *mut textcmdplayer_t = (*textcmdplayer).next;
                Z_Free(textcmdplayer as *mut libc::c_void);
                textcmdplayer = tcpnext;
            }
            i += 1;
            i;
        }
        Z_Free(textcmdtic as *mut libc::c_void);
    }
}
unsafe extern "C" fn D_GetExistingTextcmd(
    mut tic: tic_t,
    mut playernum: int32_t,
) -> *mut uint8_t {
    let mut textcmdtic: *mut textcmdtic_t = textcmds[(tic
        & (4 as libc::c_int - 1 as libc::c_int) as tic_t) as usize];
    while !textcmdtic.is_null() && (*textcmdtic).tic != tic {
        textcmdtic = (*textcmdtic).next;
    }
    if !textcmdtic.is_null() {
        let mut textcmdplayer: *mut textcmdplayer_t = (*textcmdtic)
            .playercmds[(playernum & 4 as libc::c_int - 1 as libc::c_int) as usize];
        while !textcmdplayer.is_null() && (*textcmdplayer).playernum != playernum {
            textcmdplayer = (*textcmdplayer).next;
        }
        if !textcmdplayer.is_null() {
            return ((*textcmdplayer).cmd).as_mut_ptr();
        }
    }
    return 0 as *mut uint8_t;
}
unsafe extern "C" fn D_GetTextcmd(
    mut tic: tic_t,
    mut playernum: int32_t,
) -> *mut uint8_t {
    let mut textcmdtic: *mut textcmdtic_t = textcmds[(tic
        & (4 as libc::c_int - 1 as libc::c_int) as tic_t) as usize];
    let mut tctprev: *mut *mut textcmdtic_t = &mut *textcmds
        .as_mut_ptr()
        .offset((tic & (4 as libc::c_int - 1 as libc::c_int) as tic_t) as isize)
        as *mut *mut textcmdtic_t;
    let mut textcmdplayer: *mut textcmdplayer_t = 0 as *mut textcmdplayer_t;
    let mut tcpprev: *mut *mut textcmdplayer_t = 0 as *mut *mut textcmdplayer_t;
    while !textcmdtic.is_null() && (*textcmdtic).tic != tic {
        tctprev = &mut (*textcmdtic).next;
        textcmdtic = (*textcmdtic).next;
    }
    if textcmdtic.is_null() {
        *tctprev = Z_CallocAlign(
            ::core::mem::size_of::<textcmdtic_t>() as libc::c_ulong,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut textcmdtic_t;
        textcmdtic = *tctprev;
        (*textcmdtic).tic = tic;
    }
    tcpprev = &mut *((*textcmdtic).playercmds)
        .as_mut_ptr()
        .offset((playernum & 4 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut *mut textcmdplayer_t;
    textcmdplayer = *tcpprev;
    while !textcmdplayer.is_null() && (*textcmdplayer).playernum != playernum {
        tcpprev = &mut (*textcmdplayer).next;
        textcmdplayer = (*textcmdplayer).next;
    }
    if textcmdplayer.is_null() {
        *tcpprev = Z_CallocAlign(
            ::core::mem::size_of::<textcmdplayer_t>() as libc::c_ulong,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut textcmdplayer_t;
        textcmdplayer = *tcpprev;
        (*textcmdplayer).playernum = playernum;
    }
    return ((*textcmdplayer).cmd).as_mut_ptr();
}
unsafe extern "C" fn ExtraDataTicker() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 || i == 0 as libc::c_int {
            let mut bufferstart: *mut uint8_t = D_GetExistingTextcmd(gametic, i);
            if !bufferstart.is_null() {
                let mut curpos: *mut uint8_t = bufferstart;
                let mut bufferend: *mut uint8_t = &mut *curpos
                    .offset(
                        (*curpos.offset(0 as libc::c_int as isize) as libc::c_int
                            + 1 as libc::c_int) as isize,
                    ) as *mut uint8_t;
                curpos = curpos.offset(1);
                curpos;
                while curpos < bufferend {
                    if (*curpos as libc::c_int) < MAXNETXCMD as libc::c_int
                        && (listnetxcmd[*curpos as usize]).is_some()
                    {
                        let id: uint8_t = *curpos;
                        curpos = curpos.offset(1);
                        curpos;
                        if !debugfile.is_null() {
                            fputs(
                                va(
                                    b"executing x_cmd %s ply %u \0" as *const u8
                                        as *const libc::c_char,
                                    netxcmdnames[(id as libc::c_int - 1 as libc::c_int)
                                        as usize],
                                    i,
                                ),
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                        (listnetxcmd[id as usize])
                            .expect("non-null function pointer")(&mut curpos, i);
                        if !debugfile.is_null() {
                            fputs(
                                b"done\n\0" as *const u8 as *const libc::c_char,
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                    } else {
                        if server != 0 {
                            SendKick(
                                i as uint8_t,
                                (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                            );
                            if !debugfile.is_null() {
                                fputs(
                                    va(
                                        b"player %d kicked [gametic=%u] reason as follows:\n\0"
                                            as *const u8 as *const libc::c_char,
                                        i,
                                        gametic,
                                    ),
                                    debugfile,
                                );
                                fflush(debugfile);
                            }
                        }
                        CONS_Alert(
                            CONS_WARNING,
                            b"Got unknown net command [%s]=%d (max %d)\n\0" as *const u8
                                as *const libc::c_char,
                            sizeu1(
                                curpos.offset_from(bufferstart) as libc::c_long as size_t,
                            ),
                            *curpos as libc::c_int,
                            *bufferstart.offset(0 as libc::c_int as isize) as libc::c_int,
                        );
                        break;
                    }
                }
            }
        }
        i += 1;
        i;
    }
    if server == 0 {
        D_FreeTextcmd(gametic);
    }
}
unsafe extern "C" fn D_Clearticcmd(mut tic: tic_t) {
    let mut i: int32_t = 0;
    D_FreeTextcmd(tic);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        netcmds[(tic % 1024 as libc::c_int as tic_t) as usize][i as usize]
            .angleturn = 0 as libc::c_int as int16_t;
        i += 1;
        i;
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"clear tic %5u (%2u)\n\0" as *const u8 as *const libc::c_char,
                tic,
                tic % 1024 as libc::c_int as tic_t,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_ResetTiccmds() {
    let mut i: int32_t = 0;
    memset(
        &mut localcmds as *mut ticcmd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
    );
    memset(
        &mut localcmds2 as *mut ticcmd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        while !(textcmds[i as usize]).is_null() {
            D_Clearticcmd((*textcmds[i as usize]).tic);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SendKick(mut playernum: uint8_t, mut msg: uint8_t) {
    let mut buf: [uint8_t; 2] = [0; 2];
    if !(server != 0 && cv_rejointimeout.value != 0) {
        msg = (msg as libc::c_int & !(0x80 as libc::c_int)) as uint8_t;
    }
    buf[0 as libc::c_int as usize] = playernum;
    buf[1 as libc::c_int as usize] = msg;
    SendNetXCmd(
        XD_KICK,
        &mut buf as *mut [uint8_t; 2] as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
}
static mut cl_mode: cl_mode_t = CL_SEARCHING;
static mut cl_lastcheckedfilecount: uint16_t = 0 as libc::c_int as uint16_t;
static mut snake_bonuspatches: [*const libc::c_char; 8] = [
    0 as *const libc::c_char,
    b"DL_SLOW\0" as *const u8 as *const libc::c_char,
    b"TVSSC0\0" as *const u8 as *const libc::c_char,
    b"TVIVC0\0" as *const u8 as *const libc::c_char,
    b"TVARC0\0" as *const u8 as *const libc::c_char,
    b"DL_SCISSORS\0" as *const u8 as *const libc::c_char,
    b"TVRCC0\0" as *const u8 as *const libc::c_char,
    b"TVEGC0\0" as *const u8 as *const libc::c_char,
];
static mut snake_backgrounds: [*const libc::c_char; 10] = [
    b"RVPUMICF\0" as *const u8 as *const libc::c_char,
    b"FRSTRCKF\0" as *const u8 as *const libc::c_char,
    b"TAR\0" as *const u8 as *const libc::c_char,
    b"MMFLRB4\0" as *const u8 as *const libc::c_char,
    b"RVDARKF1\0" as *const u8 as *const libc::c_char,
    b"RVZWALF1\0" as *const u8 as *const libc::c_char,
    b"RVZWALF4\0" as *const u8 as *const libc::c_char,
    b"RVZWALF5\0" as *const u8 as *const libc::c_char,
    b"RVZGRS02\0" as *const u8 as *const libc::c_char,
    b"RVZGRS04\0" as *const u8 as *const libc::c_char,
];
static mut snake: *mut snake_t = 0 as *const snake_t as *mut snake_t;
unsafe extern "C" fn Snake_Initialise() {
    if snake.is_null() {
        snake = malloc(::core::mem::size_of::<snake_t>() as libc::c_ulong)
            as *mut snake_t;
    }
    (*snake).paused = false_0 as libc::c_int;
    (*snake).pausepressed = false_0 as libc::c_int;
    (*snake).time = 0 as libc::c_int as tic_t;
    (*snake).nextupdate = 5 as libc::c_int as tic_t;
    (*snake).gameover = false_0 as libc::c_int;
    (*snake)
        .background = M_RandomKey(
        (::core::mem::size_of::<[*const libc::c_char; 10]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as int32_t,
    ) as uint8_t;
    (*snake).snakelength = 1 as libc::c_int as uint16_t;
    (*snake).snakebonus = SNAKE_BONUS_NONE;
    (*snake)
        .snakex[0 as libc::c_int as usize] = M_RandomKey(20 as libc::c_int) as uint8_t;
    (*snake)
        .snakey[0 as libc::c_int as usize] = M_RandomKey(10 as libc::c_int) as uint8_t;
    (*snake).snakedir[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    (*snake).snakedir[1 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    (*snake).applex = M_RandomKey(20 as libc::c_int) as uint8_t;
    (*snake).appley = M_RandomKey(10 as libc::c_int) as uint8_t;
    (*snake).bonustype = SNAKE_BONUS_NONE;
}
unsafe extern "C" fn Snake_GetOppositeDir(mut dir: uint8_t) -> uint8_t {
    if dir as libc::c_int == 1 as libc::c_int || dir as libc::c_int == 3 as libc::c_int {
        return (dir as libc::c_int + 1 as libc::c_int) as uint8_t
    } else if dir as libc::c_int == 2 as libc::c_int
        || dir as libc::c_int == 4 as libc::c_int
    {
        return (dir as libc::c_int - 1 as libc::c_int) as uint8_t
    } else {
        return (12 as libc::c_int + 5 as libc::c_int - dir as libc::c_int) as uint8_t
    };
}
#[no_mangle]
pub static mut snakejoyevents: [*mut event_t; 128] = [0 as *const event_t
    as *mut event_t; 128];
#[no_mangle]
pub static mut joyeventcount: uint16_t = 0 as libc::c_int as uint16_t;
unsafe extern "C" fn Snake_Joy_Grabber(mut ev: *mut event_t) -> boolean {
    if (*ev).type_0 as libc::c_uint == ev_joystick as libc::c_int as libc::c_uint
        && (*ev).key == 0 as libc::c_int
    {
        snakejoyevents[joyeventcount as usize] = ev;
        joyeventcount = joyeventcount.wrapping_add(1);
        joyeventcount;
        return true_0 as libc::c_int;
    } else {
        return false_0 as libc::c_int
    };
}
unsafe extern "C" fn Snake_FindFreeSlot(
    mut freex: *mut uint8_t,
    mut freey: *mut uint8_t,
    mut headx: uint8_t,
    mut heady: uint8_t,
) {
    let mut x: uint8_t = 0;
    let mut y: uint8_t = 0;
    let mut i: uint16_t = 0;
    loop {
        x = M_RandomKey(20 as libc::c_int) as uint8_t;
        y = M_RandomKey(10 as libc::c_int) as uint8_t;
        i = 0 as libc::c_int as uint16_t;
        while (i as libc::c_int) < (*snake).snakelength as libc::c_int {
            if x as libc::c_int == (*snake).snakex[i as usize] as libc::c_int
                && y as libc::c_int == (*snake).snakey[i as usize] as libc::c_int
            {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
        if !((i as libc::c_int) < (*snake).snakelength as libc::c_int
            || x as libc::c_int == headx as libc::c_int
                && y as libc::c_int == heady as libc::c_int
            || x as libc::c_int == (*snake).applex as libc::c_int
                && y as libc::c_int == (*snake).appley as libc::c_int
            || (*snake).bonustype as libc::c_uint
                != SNAKE_BONUS_NONE as libc::c_int as libc::c_uint
                && x as libc::c_int == (*snake).bonusx as libc::c_int
                && y as libc::c_int == (*snake).bonusy as libc::c_int)
        {
            break;
        }
    }
    *freex = x;
    *freey = y;
}
unsafe extern "C" fn Snake_Handle() {
    let mut x: uint8_t = 0;
    let mut y: uint8_t = 0;
    let mut oldx: uint8_t = 0;
    let mut oldy: uint8_t = 0;
    let mut i: uint16_t = 0;
    let mut j: uint16_t = 0;
    let mut joystate: uint16_t = 0 as libc::c_int as uint16_t;
    static mut pjoyx: int32_t = 0 as libc::c_int;
    static mut pjoyy: int32_t = 0 as libc::c_int;
    if (*snake).gameover != 0
        && (gamekeydown[gamecontrol[GC_JUMP as libc::c_int
            as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
            || gamekeydown[gamecontrol[GC_JUMP as libc::c_int
                as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
            || gamekeydown[13 as libc::c_int as usize] as libc::c_int != 0)
    {
        Snake_Initialise();
        (*snake).pausepressed = true_0 as libc::c_int;
    }
    if gamekeydown[gamecontrol[GC_PAUSE as libc::c_int
        as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[gamecontrol[GC_PAUSE as libc::c_int
            as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[13 as libc::c_int as usize] as libc::c_int != 0
    {
        if (*snake).pausepressed == 0 {
            (*snake).paused = ((*snake).paused == 0) as libc::c_int;
        }
        (*snake).pausepressed = true_0 as libc::c_int;
    } else {
        (*snake).pausepressed = false_0 as libc::c_int;
    }
    if (*snake).paused != 0 {
        return;
    }
    (*snake).time = ((*snake).time).wrapping_add(1);
    (*snake).time;
    x = (*snake).snakex[0 as libc::c_int as usize];
    y = (*snake).snakey[0 as libc::c_int as usize];
    oldx = (*snake).snakex[1 as libc::c_int as usize];
    oldy = (*snake).snakey[1 as libc::c_int as usize];
    j = 0 as libc::c_int as uint16_t;
    while (j as libc::c_int) < joyeventcount as libc::c_int {
        let mut ev: *mut event_t = snakejoyevents[j as usize];
        let jdeadzone: int32_t = 1023 as libc::c_int * cv_digitaldeadzone.value
            / ((1 as libc::c_int) << 16 as libc::c_int);
        if (*ev).y != 2147483647 as libc::c_int {
            if Joystick.bGamepadStyle != 0 || abs((*ev).y) > jdeadzone {
                if (*ev).y < 0 as libc::c_int && pjoyy >= 0 as libc::c_int {
                    joystate = 1 as libc::c_int as uint16_t;
                } else if (*ev).y > 0 as libc::c_int && pjoyy <= 0 as libc::c_int {
                    joystate = 2 as libc::c_int as uint16_t;
                }
                pjoyy = (*ev).y;
            } else {
                pjoyy = 0 as libc::c_int;
            }
        }
        if (*ev).x != 2147483647 as libc::c_int {
            if Joystick.bGamepadStyle != 0 || abs((*ev).x) > jdeadzone {
                if (*ev).x < 0 as libc::c_int && pjoyx >= 0 as libc::c_int {
                    joystate = 3 as libc::c_int as uint16_t;
                } else if (*ev).x > 0 as libc::c_int && pjoyx <= 0 as libc::c_int {
                    joystate = 4 as libc::c_int as uint16_t;
                }
                pjoyx = (*ev).x;
            } else {
                pjoyx = 0 as libc::c_int;
            }
        }
        j = j.wrapping_add(1);
        j;
    }
    joyeventcount = 0 as libc::c_int as uint16_t;
    if gamekeydown[gamecontrol[GC_STRAFELEFT as libc::c_int
        as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[gamecontrol[GC_STRAFELEFT as libc::c_int
            as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[(0x80 as libc::c_int + 105 as libc::c_int) as usize]
            as libc::c_int != 0 || joystate as libc::c_int == 3 as libc::c_int
    {
        if ((*snake).snakelength as libc::c_int) < 2 as libc::c_int
            || x as libc::c_int <= oldx as libc::c_int
        {
            (*snake).snakedir[0 as libc::c_int as usize] = 1 as libc::c_int as uint8_t;
        }
    } else if gamekeydown[gamecontrol[GC_STRAFERIGHT as libc::c_int
        as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[gamecontrol[GC_STRAFERIGHT as libc::c_int
            as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[(0x80 as libc::c_int + 107 as libc::c_int) as usize]
            as libc::c_int != 0 || joystate as libc::c_int == 4 as libc::c_int
    {
        if ((*snake).snakelength as libc::c_int) < 2 as libc::c_int
            || x as libc::c_int >= oldx as libc::c_int
        {
            (*snake).snakedir[0 as libc::c_int as usize] = 2 as libc::c_int as uint8_t;
        }
    } else if gamekeydown[gamecontrol[GC_FORWARD as libc::c_int
        as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[gamecontrol[GC_FORWARD as libc::c_int
            as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[(0x80 as libc::c_int + 102 as libc::c_int) as usize]
            as libc::c_int != 0 || joystate as libc::c_int == 1 as libc::c_int
    {
        if ((*snake).snakelength as libc::c_int) < 2 as libc::c_int
            || y as libc::c_int <= oldy as libc::c_int
        {
            (*snake).snakedir[0 as libc::c_int as usize] = 3 as libc::c_int as uint8_t;
        }
    } else if gamekeydown[gamecontrol[GC_BACKWARD as libc::c_int
        as usize][0 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[gamecontrol[GC_BACKWARD as libc::c_int
            as usize][1 as libc::c_int as usize] as usize] as libc::c_int != 0
        || gamekeydown[(0x80 as libc::c_int + 110 as libc::c_int) as usize]
            as libc::c_int != 0 || joystate as libc::c_int == 2 as libc::c_int
    {
        if ((*snake).snakelength as libc::c_int) < 2 as libc::c_int
            || y as libc::c_int >= oldy as libc::c_int
        {
            (*snake).snakedir[0 as libc::c_int as usize] = 4 as libc::c_int as uint8_t;
        }
    }
    if (*snake).snakebonustime != 0 {
        (*snake).snakebonustime = ((*snake).snakebonustime).wrapping_sub(1);
        (*snake).snakebonustime;
        if (*snake).snakebonustime == 0 {
            (*snake).snakebonus = SNAKE_BONUS_NONE;
        }
    }
    (*snake).nextupdate = ((*snake).nextupdate).wrapping_sub(1);
    (*snake).nextupdate;
    if (*snake).nextupdate != 0 {
        return;
    }
    if (*snake).snakebonus as libc::c_uint
        == SNAKE_BONUS_SLOW as libc::c_int as libc::c_uint
    {
        (*snake).nextupdate = (5 as libc::c_int * 2 as libc::c_int) as tic_t;
    } else if (*snake).snakebonus as libc::c_uint
        == SNAKE_BONUS_FAST as libc::c_int as libc::c_uint
    {
        (*snake)
            .nextupdate = (5 as libc::c_int * 2 as libc::c_int / 3 as libc::c_int)
            as tic_t;
    } else {
        (*snake).nextupdate = 5 as libc::c_int as tic_t;
    }
    if (*snake).gameover != 0 {
        return;
    }
    match (*snake).snakedir[0 as libc::c_int as usize] as libc::c_int {
        1 => {
            if x as libc::c_int > 0 as libc::c_int {
                x = x.wrapping_sub(1);
                x;
            } else {
                (*snake).gameover = true_0 as libc::c_int;
            }
        }
        2 => {
            if (x as libc::c_int) < 20 as libc::c_int - 1 as libc::c_int {
                x = x.wrapping_add(1);
                x;
            } else {
                (*snake).gameover = true_0 as libc::c_int;
            }
        }
        3 => {
            if y as libc::c_int > 0 as libc::c_int {
                y = y.wrapping_sub(1);
                y;
            } else {
                (*snake).gameover = true_0 as libc::c_int;
            }
        }
        4 => {
            if (y as libc::c_int) < 10 as libc::c_int - 1 as libc::c_int {
                y = y.wrapping_add(1);
                y;
            } else {
                (*snake).gameover = true_0 as libc::c_int;
            }
        }
        _ => {}
    }
    if (*snake).snakebonus as libc::c_uint
        != SNAKE_BONUS_GHOST as libc::c_int as libc::c_uint
    {
        i = 1 as libc::c_int as uint16_t;
        while (i as libc::c_int) < (*snake).snakelength as libc::c_int - 1 as libc::c_int
        {
            if x as libc::c_int == (*snake).snakex[i as usize] as libc::c_int
                && y as libc::c_int == (*snake).snakey[i as usize] as libc::c_int
            {
                if (*snake).snakebonus as libc::c_uint
                    == SNAKE_BONUS_SCISSORS as libc::c_int as libc::c_uint
                {
                    (*snake).snakebonus = SNAKE_BONUS_NONE;
                    (*snake).snakelength = i;
                    S_StartSound(0 as *const libc::c_void, sfx_adderr);
                } else {
                    (*snake).gameover = true_0 as libc::c_int;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    if (*snake).gameover != 0 {
        S_StartSound(0 as *const libc::c_void, sfx_lose);
        return;
    }
    if x as libc::c_int == (*snake).applex as libc::c_int
        && y as libc::c_int == (*snake).appley as libc::c_int
    {
        if ((*snake).snakelength as libc::c_int + 3 as libc::c_int)
            < 20 as libc::c_int * 10 as libc::c_int
        {
            (*snake).snakelength = ((*snake).snakelength).wrapping_add(1);
            (*snake).snakelength;
            (*snake)
                .snakex[((*snake).snakelength as libc::c_int - 1 as libc::c_int)
                as usize] = (*snake)
                .snakex[((*snake).snakelength as libc::c_int - 2 as libc::c_int)
                as usize];
            (*snake)
                .snakey[((*snake).snakelength as libc::c_int - 1 as libc::c_int)
                as usize] = (*snake)
                .snakey[((*snake).snakelength as libc::c_int - 2 as libc::c_int)
                as usize];
            (*snake)
                .snakedir[((*snake).snakelength as libc::c_int - 1 as libc::c_int)
                as usize] = (*snake)
                .snakedir[((*snake).snakelength as libc::c_int - 2 as libc::c_int)
                as usize];
        }
        Snake_FindFreeSlot(&mut (*snake).applex, &mut (*snake).appley, x, y);
        if (*snake).snakelength as libc::c_int % 5 as libc::c_int == 0 {
            loop {
                (*snake)
                    .bonustype = (M_RandomKey(
                    SNAKE_NUM_BONUSES as libc::c_int - 1 as libc::c_int,
                ) + 1 as libc::c_int) as snake_bonustype_s;
                if !((*snake).snakelength as libc::c_int
                    > 20 as libc::c_int * 10 as libc::c_int * 3 as libc::c_int
                        / 4 as libc::c_int
                    && ((*snake).bonustype as libc::c_uint
                        == SNAKE_BONUS_EGGMAN as libc::c_int as libc::c_uint
                        || (*snake).bonustype as libc::c_uint
                            == SNAKE_BONUS_FAST as libc::c_int as libc::c_uint
                        || (*snake).bonustype as libc::c_uint
                            == SNAKE_BONUS_REVERSE as libc::c_int as libc::c_uint))
                {
                    break;
                }
            }
            Snake_FindFreeSlot(&mut (*snake).bonusx, &mut (*snake).bonusy, x, y);
        }
        S_StartSound(0 as *const libc::c_void, sfx_s3k6b);
    }
    if (*snake).snakelength as libc::c_int > 1 as libc::c_int
        && (*snake).snakedir[0 as libc::c_int as usize] as libc::c_int != 0
    {
        let mut dir: uint8_t = (*snake).snakedir[0 as libc::c_int as usize];
        oldx = (*snake).snakex[1 as libc::c_int as usize];
        oldy = (*snake).snakey[1 as libc::c_int as usize];
        i = ((*snake).snakelength as libc::c_int - 1 as libc::c_int) as uint16_t;
        while i as libc::c_int > 0 as libc::c_int {
            (*snake)
                .snakex[i
                as usize] = (*snake)
                .snakex[(i as libc::c_int - 1 as libc::c_int) as usize];
            (*snake)
                .snakey[i
                as usize] = (*snake)
                .snakey[(i as libc::c_int - 1 as libc::c_int) as usize];
            (*snake)
                .snakedir[i
                as usize] = (*snake)
                .snakedir[(i as libc::c_int - 1 as libc::c_int) as usize];
            i = i.wrapping_sub(1);
            i;
        }
        if (x as libc::c_int) < oldx as libc::c_int
            && dir as libc::c_int == 3 as libc::c_int
        {
            dir = 5 as libc::c_int as uint8_t;
        } else if x as libc::c_int > oldx as libc::c_int
            && dir as libc::c_int == 3 as libc::c_int
        {
            dir = 6 as libc::c_int as uint8_t;
        } else if (x as libc::c_int) < oldx as libc::c_int
            && dir as libc::c_int == 4 as libc::c_int
        {
            dir = 7 as libc::c_int as uint8_t;
        } else if x as libc::c_int > oldx as libc::c_int
            && dir as libc::c_int == 4 as libc::c_int
        {
            dir = 8 as libc::c_int as uint8_t;
        } else if (y as libc::c_int) < oldy as libc::c_int
            && dir as libc::c_int == 1 as libc::c_int
        {
            dir = 9 as libc::c_int as uint8_t;
        } else if (y as libc::c_int) < oldy as libc::c_int
            && dir as libc::c_int == 2 as libc::c_int
        {
            dir = 10 as libc::c_int as uint8_t;
        } else if y as libc::c_int > oldy as libc::c_int
            && dir as libc::c_int == 1 as libc::c_int
        {
            dir = 11 as libc::c_int as uint8_t;
        } else if y as libc::c_int > oldy as libc::c_int
            && dir as libc::c_int == 2 as libc::c_int
        {
            dir = 12 as libc::c_int as uint8_t;
        }
        (*snake).snakedir[1 as libc::c_int as usize] = dir;
    }
    (*snake).snakex[0 as libc::c_int as usize] = x;
    (*snake).snakey[0 as libc::c_int as usize] = y;
    if (*snake).bonustype as libc::c_uint
        != SNAKE_BONUS_NONE as libc::c_int as libc::c_uint
        && x as libc::c_int == (*snake).bonusx as libc::c_int
        && y as libc::c_int == (*snake).bonusy as libc::c_int
    {
        S_StartSound(0 as *const libc::c_void, sfx_ncchip);
        match (*snake).bonustype as libc::c_uint {
            1 => {
                (*snake).snakebonus = SNAKE_BONUS_SLOW;
                (*snake)
                    .snakebonustime = (20 as libc::c_int * 35 as libc::c_int) as tic_t;
            }
            2 => {
                (*snake).snakebonus = SNAKE_BONUS_FAST;
                (*snake)
                    .snakebonustime = (20 as libc::c_int * 35 as libc::c_int) as tic_t;
            }
            3 => {
                (*snake).snakebonus = SNAKE_BONUS_GHOST;
                (*snake)
                    .snakebonustime = (10 as libc::c_int * 35 as libc::c_int) as tic_t;
            }
            4 => {
                i = 0 as libc::c_int as uint16_t;
                while (i as libc::c_int) < (*snake).snakelength as libc::c_int {
                    (*snake)
                        .snakex[i as usize] = (*snake).snakex[0 as libc::c_int as usize];
                    (*snake)
                        .snakey[i as usize] = (*snake).snakey[0 as libc::c_int as usize];
                    (*snake)
                        .snakedir[i
                        as usize] = (*snake).snakedir[0 as libc::c_int as usize];
                    i = i.wrapping_add(1);
                    i;
                }
                S_StartSound(0 as *const libc::c_void, sfx_bkpoof);
            }
            5 => {
                (*snake).snakebonus = SNAKE_BONUS_SCISSORS;
                (*snake)
                    .snakebonustime = (60 as libc::c_int * 35 as libc::c_int) as tic_t;
            }
            6 => {
                i = 0 as libc::c_int as uint16_t;
                while (i as libc::c_int)
                    < ((*snake).snakelength as libc::c_int + 1 as libc::c_int)
                        / 2 as libc::c_int
                {
                    let mut i2: uint16_t = ((*snake).snakelength as libc::c_int
                        - 1 as libc::c_int - i as libc::c_int) as uint16_t;
                    let mut tmpx: uint8_t = (*snake).snakex[i as usize];
                    let mut tmpy: uint8_t = (*snake).snakey[i as usize];
                    let mut tmpdir: uint8_t = (*snake).snakedir[i as usize];
                    (*snake).snakex[i as usize] = (*snake).snakex[i2 as usize];
                    (*snake).snakey[i as usize] = (*snake).snakey[i2 as usize];
                    (*snake)
                        .snakedir[i
                        as usize] = Snake_GetOppositeDir((*snake).snakedir[i2 as usize]);
                    (*snake).snakex[i2 as usize] = tmpx;
                    (*snake).snakey[i2 as usize] = tmpy;
                    (*snake).snakedir[i2 as usize] = Snake_GetOppositeDir(tmpdir);
                    i = i.wrapping_add(1);
                    i;
                }
                (*snake)
                    .snakedir[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
                S_StartSound(0 as *const libc::c_void, sfx_gravch);
            }
            _ => {
                if (*snake).snakebonus as libc::c_uint
                    != SNAKE_BONUS_GHOST as libc::c_int as libc::c_uint
                {
                    (*snake).gameover = true_0 as libc::c_int;
                    S_StartSound(0 as *const libc::c_void, sfx_lose);
                }
            }
        }
        (*snake).bonustype = SNAKE_BONUS_NONE;
    }
}
unsafe extern "C" fn Snake_Draw() {
    let mut i: int16_t = 0;
    V_DrawFill(
        0 as libc::c_int,
        0 as libc::c_int,
        320 as libc::c_int,
        200 as libc::c_int,
        31 as libc::c_int,
    );
    V_DrawFlatFill(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int + 12 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            + 12 as libc::c_int,
        20 as libc::c_int * 12 as libc::c_int,
        10 as libc::c_int * 12 as libc::c_int,
        W_GetNumForName(snake_backgrounds[(*snake).background as usize]),
    );
    V_DrawFill(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        12 as libc::c_int + 20 as libc::c_int * 12 as libc::c_int,
        12 as libc::c_int,
        242 as libc::c_int,
    );
    V_DrawFill(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int + 12 as libc::c_int
            + 20 as libc::c_int * 12 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        12 as libc::c_int,
        12 as libc::c_int + 10 as libc::c_int * 12 as libc::c_int,
        242 as libc::c_int,
    );
    V_DrawFill(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int + 12 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 12 as libc::c_int
            + 10 as libc::c_int * 12 as libc::c_int,
        12 as libc::c_int + 20 as libc::c_int * 12 as libc::c_int,
        12 as libc::c_int,
        242 as libc::c_int,
    );
    V_DrawFill(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
            + 12 as libc::c_int,
        12 as libc::c_int,
        12 as libc::c_int + 10 as libc::c_int * 12 as libc::c_int,
        242 as libc::c_int,
    );
    V_DrawStretchyFixedPatch(
        ((320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int + 12 as libc::c_int
            + (*snake).applex as libc::c_int * 12 as libc::c_int
            + 12 as libc::c_int / 2 as libc::c_int)
            * ((1 as libc::c_int) << 16 as libc::c_int),
        (200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int + 12 as libc::c_int
            + (*snake).appley as libc::c_int * 12 as libc::c_int
            + 12 as libc::c_int / 2 as libc::c_int)
            * ((1 as libc::c_int) << 16 as libc::c_int),
        ((1 as libc::c_int) << 16 as libc::c_int) / 4 as libc::c_int,
        ((1 as libc::c_int) << 16 as libc::c_int) / 4 as libc::c_int,
        0 as libc::c_int,
        W_CachePatchLongName(
            b"DL_APPLE\0" as *const u8 as *const libc::c_char,
            PU_HUDGFX as libc::c_int,
        ) as *mut patch_t,
        0 as *const uint8_t,
    );
    if (*snake).bonustype as libc::c_uint
        != SNAKE_BONUS_NONE as libc::c_int as libc::c_uint
    {
        V_DrawStretchyFixedPatch(
            ((320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int)
                / 2 as libc::c_int - 12 as libc::c_int + 12 as libc::c_int
                + (*snake).bonusx as libc::c_int * 12 as libc::c_int
                + 12 as libc::c_int / 2 as libc::c_int)
                * ((1 as libc::c_int) << 16 as libc::c_int),
            (200 as libc::c_int - 48 as libc::c_int
                - 10 as libc::c_int * 12 as libc::c_int
                - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                + 12 as libc::c_int + (*snake).bonusy as libc::c_int * 12 as libc::c_int
                + 12 as libc::c_int / 2 as libc::c_int + 4 as libc::c_int)
                * ((1 as libc::c_int) << 16 as libc::c_int),
            ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
            ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchLongName(
                snake_bonuspatches[(*snake).bonustype as usize],
                PU_HUDGFX as libc::c_int,
            ) as *mut patch_t,
            0 as *const uint8_t,
        );
    }
    if (*snake).gameover == 0
        || ((*snake).time % 8 as libc::c_int as tic_t)
            < (8 as libc::c_int / 2 as libc::c_int) as tic_t
    {
        i = ((*snake).snakelength as libc::c_int - 1 as libc::c_int) as int16_t;
        while i as libc::c_int >= 0 as libc::c_int {
            let mut patchname: *const libc::c_char = 0 as *const libc::c_char;
            let mut dir: uint8_t = (*snake).snakedir[i as usize];
            if i as libc::c_int == 0 as libc::c_int {
                match dir as libc::c_int {
                    1 => {
                        patchname = b"DL_SNAKEHEAD_L\0" as *const u8
                            as *const libc::c_char;
                    }
                    2 => {
                        patchname = b"DL_SNAKEHEAD_R\0" as *const u8
                            as *const libc::c_char;
                    }
                    3 => {
                        patchname = b"DL_SNAKEHEAD_T\0" as *const u8
                            as *const libc::c_char;
                    }
                    4 => {
                        patchname = b"DL_SNAKEHEAD_B\0" as *const u8
                            as *const libc::c_char;
                    }
                    _ => {
                        patchname = b"DL_SNAKEHEAD_M\0" as *const u8
                            as *const libc::c_char;
                    }
                }
            } else {
                match dir as libc::c_int {
                    1 => {
                        patchname = b"DL_SNAKEBODY_L\0" as *const u8
                            as *const libc::c_char;
                    }
                    2 => {
                        patchname = b"DL_SNAKEBODY_R\0" as *const u8
                            as *const libc::c_char;
                    }
                    3 => {
                        patchname = b"DL_SNAKEBODY_T\0" as *const u8
                            as *const libc::c_char;
                    }
                    4 => {
                        patchname = b"DL_SNAKEBODY_B\0" as *const u8
                            as *const libc::c_char;
                    }
                    5 => {
                        patchname = b"DL_SNAKEBODY_LT\0" as *const u8
                            as *const libc::c_char;
                    }
                    6 => {
                        patchname = b"DL_SNAKEBODY_RT\0" as *const u8
                            as *const libc::c_char;
                    }
                    7 => {
                        patchname = b"DL_SNAKEBODY_LB\0" as *const u8
                            as *const libc::c_char;
                    }
                    8 => {
                        patchname = b"DL_SNAKEBODY_RB\0" as *const u8
                            as *const libc::c_char;
                    }
                    9 => {
                        patchname = b"DL_SNAKEBODY_TL\0" as *const u8
                            as *const libc::c_char;
                    }
                    10 => {
                        patchname = b"DL_SNAKEBODY_TR\0" as *const u8
                            as *const libc::c_char;
                    }
                    11 => {
                        patchname = b"DL_SNAKEBODY_BL\0" as *const u8
                            as *const libc::c_char;
                    }
                    12 => {
                        patchname = b"DL_SNAKEBODY_BR\0" as *const u8
                            as *const libc::c_char;
                    }
                    _ => {
                        patchname = b"DL_SNAKEBODY_B\0" as *const u8
                            as *const libc::c_char;
                    }
                }
            }
            V_DrawStretchyFixedPatch(
                ((320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int)
                    / 2 as libc::c_int - 12 as libc::c_int + 12 as libc::c_int
                    + (*snake).snakex[i as usize] as libc::c_int * 12 as libc::c_int
                    + 12 as libc::c_int / 2 as libc::c_int)
                    * ((1 as libc::c_int) << 16 as libc::c_int),
                (200 as libc::c_int - 48 as libc::c_int
                    - 10 as libc::c_int * 12 as libc::c_int
                    - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    + 12 as libc::c_int
                    + (*snake).snakey[i as usize] as libc::c_int * 12 as libc::c_int
                    + 12 as libc::c_int / 2 as libc::c_int)
                    * ((1 as libc::c_int) << 16 as libc::c_int),
                if i as libc::c_int == 0 as libc::c_int
                    && dir as libc::c_int == 0 as libc::c_int
                {
                    ((1 as libc::c_int) << 16 as libc::c_int) / 5 as libc::c_int
                } else {
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int
                },
                if i as libc::c_int == 0 as libc::c_int
                    && dir as libc::c_int == 0 as libc::c_int
                {
                    ((1 as libc::c_int) << 16 as libc::c_int) / 5 as libc::c_int
                } else {
                    ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int
                },
                if (*snake).snakebonus as libc::c_uint
                    == SNAKE_BONUS_GHOST as libc::c_int as libc::c_uint
                {
                    0x50000 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                W_CachePatchLongName(patchname, PU_HUDGFX as libc::c_int)
                    as *mut patch_t,
                0 as *const uint8_t,
            );
            i -= 1;
            i;
        }
    }
    V_DrawString(
        (320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int) / 2 as libc::c_int
            - 12 as libc::c_int + 20 as libc::c_int * 12 as libc::c_int
            + 12 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int + 4 as libc::c_int,
        200 as libc::c_int - 48 as libc::c_int - 10 as libc::c_int * 12 as libc::c_int
            - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int,
        0xc00 as libc::c_int,
        va(
            b"%u\0" as *const u8 as *const libc::c_char,
            (*snake).snakelength as libc::c_int,
        ),
    );
    if (*snake).snakebonus as libc::c_uint
        != SNAKE_BONUS_NONE as libc::c_int as libc::c_uint
        && ((*snake).snakebonustime >= (3 as libc::c_int * 35 as libc::c_int) as tic_t
            || ((*snake).time % 4 as libc::c_int as tic_t)
                < (4 as libc::c_int / 2 as libc::c_int) as tic_t)
    {
        V_DrawStretchyFixedPatch(
            ((320 as libc::c_int - 20 as libc::c_int * 12 as libc::c_int)
                / 2 as libc::c_int - 12 as libc::c_int
                + 20 as libc::c_int * 12 as libc::c_int
                + 12 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int
                + 10 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
            (200 as libc::c_int - 48 as libc::c_int
                - 10 as libc::c_int * 12 as libc::c_int
                - 12 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                + 24 as libc::c_int) * ((1 as libc::c_int) << 16 as libc::c_int),
            ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
            ((1 as libc::c_int) << 16 as libc::c_int) / 2 as libc::c_int,
            0 as libc::c_int,
            W_CachePatchLongName(
                snake_bonuspatches[(*snake).snakebonus as usize],
                PU_HUDGFX as libc::c_int,
            ) as *mut patch_t,
            0 as *const uint8_t,
        );
    }
}
unsafe extern "C" fn CL_DrawConnectionStatusBox() {
    M_DrawTextBox(
        320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int - 8 as libc::c_int,
        200 as libc::c_int - 16 as libc::c_int - 8 as libc::c_int,
        32 as libc::c_int,
        1 as libc::c_int,
    );
    if cl_mode as libc::c_uint != CL_CONFIRMCONNECT as libc::c_int as libc::c_uint {
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 16 as libc::c_int,
            0x2000 as libc::c_int,
            b"Press ESC to abort\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[inline]
unsafe extern "C" fn CL_DrawConnectionStatus() {
    let mut ccstime: int32_t = I_GetTime() as int32_t;
    V_DrawFadeScreen(0xff00 as libc::c_int as uint16_t, 16 as libc::c_int as uint8_t);
    if cl_mode as libc::c_uint != CL_DOWNLOADFILES as libc::c_int as libc::c_uint
        && cl_mode as libc::c_uint != CL_LOADFILES as libc::c_int as libc::c_uint
    {
        let mut i: int32_t = 0;
        let mut animtime: int32_t = (ccstime / 4 as libc::c_int & 15 as libc::c_int)
            + 16 as libc::c_int;
        let mut palstart: uint8_t = 0;
        let mut cltext: *const libc::c_char = 0 as *const libc::c_char;
        CL_DrawConnectionStatusBox();
        if cl_mode as libc::c_uint == CL_SEARCHING as libc::c_int as libc::c_uint {
            palstart = 32 as libc::c_int as uint8_t;
        } else if cl_mode as libc::c_uint
            == CL_CONFIRMCONNECT as libc::c_int as libc::c_uint
        {
            palstart = 48 as libc::c_int as uint8_t;
        } else {
            palstart = 96 as libc::c_int as uint8_t;
        }
        if !(cl_mode as libc::c_uint
            == CL_DOWNLOADSAVEGAME as libc::c_int as libc::c_uint
            && lastfilenum != -(1 as libc::c_int))
        {
            i = 0 as libc::c_int;
            while i < 16 as libc::c_int {
                V_DrawFill(
                    320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int
                        + i * 16 as libc::c_int,
                    200 as libc::c_int - 16 as libc::c_int,
                    16 as libc::c_int,
                    8 as libc::c_int,
                    palstart as libc::c_int + (animtime - i & 15 as libc::c_int),
                );
                i += 1;
                i;
            }
        }
        match cl_mode as libc::c_uint {
            6 => {
                if !fileneeded.is_null() && lastfilenum != -(1 as libc::c_int) {
                    let mut currentsize: uint32_t = (*fileneeded
                        .offset(lastfilenum as isize))
                        .currentsize;
                    let mut totalsize: uint32_t = (*fileneeded
                        .offset(lastfilenum as isize))
                        .totalsize;
                    let mut dldlength: int32_t = 0;
                    cltext = b"Downloading game state...\0" as *const u8
                        as *const libc::c_char;
                    Net_GetNetStat();
                    dldlength = (currentsize as libc::c_double
                        / totalsize as libc::c_double
                        * 256 as libc::c_int as libc::c_double) as int32_t;
                    if dldlength > 256 as libc::c_int {
                        dldlength = 256 as libc::c_int;
                    }
                    V_DrawFill(
                        320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
                        200 as libc::c_int - 16 as libc::c_int,
                        256 as libc::c_int,
                        8 as libc::c_int,
                        111 as libc::c_int,
                    );
                    V_DrawFill(
                        320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
                        200 as libc::c_int - 16 as libc::c_int,
                        dldlength,
                        8 as libc::c_int,
                        96 as libc::c_int,
                    );
                    V_DrawString(
                        320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
                        200 as libc::c_int - 16 as libc::c_int,
                        0x20000 as libc::c_int | 0xc00 as libc::c_int,
                        va(
                            b" %4uK/%4uK\0" as *const u8 as *const libc::c_char,
                            currentsize >> 10 as libc::c_int,
                            totalsize >> 10 as libc::c_int,
                        ),
                    );
                    V_DrawRightAlignedString(
                        320 as libc::c_int / 2 as libc::c_int + 128 as libc::c_int,
                        200 as libc::c_int - 16 as libc::c_int,
                        0x20000 as libc::c_int | 0xc00 as libc::c_int,
                        va(
                            b"%3.1fK/s \0" as *const u8 as *const libc::c_char,
                            getbps as libc::c_double
                                / 1024 as libc::c_int as libc::c_double,
                        ),
                    );
                } else {
                    cltext = b"Waiting to download game state...\0" as *const u8
                        as *const libc::c_char;
                }
            }
            9 | 1 => {
                cltext = b"Checking server addon list...\0" as *const u8
                    as *const libc::c_char;
            }
            10 => {
                cltext = b"\0" as *const u8 as *const libc::c_char;
            }
            4 => {
                cltext = b"Loading server addons...\0" as *const u8
                    as *const libc::c_char;
            }
            3 | 5 => {
                if serverisfull != 0 {
                    cltext = b"Server full, waiting for a slot...\0" as *const u8
                        as *const libc::c_char;
                } else {
                    cltext = b"Requesting to join...\0" as *const u8
                        as *const libc::c_char;
                }
            }
            _ => {
                cltext = b"Connecting to server...\0" as *const u8
                    as *const libc::c_char;
            }
        }
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 24 as libc::c_int,
            0x2000 as libc::c_int,
            cltext,
        );
    } else if cl_mode as libc::c_uint == CL_LOADFILES as libc::c_int as libc::c_uint {
        let mut totalfileslength: int32_t = 0;
        let mut loadcompletednum: int32_t = 0 as libc::c_int;
        let mut i_0: int32_t = 0;
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 16 as libc::c_int,
            0x2000 as libc::c_int,
            b"Press ESC to abort\0" as *const u8 as *const libc::c_char,
        );
        if !fileneeded.is_null() {
            i_0 = 0 as libc::c_int;
            while i_0 < fileneedednum {
                if (*fileneeded.offset(i_0 as isize)).status as libc::c_uint
                    == FS_OPEN as libc::c_int as libc::c_uint
                {
                    loadcompletednum += 1;
                    loadcompletednum;
                }
                i_0 += 1;
                i_0;
            }
        }
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 24 as libc::c_int,
            0x2000 as libc::c_int,
            b"Loading server addons...\0" as *const u8 as *const libc::c_char,
        );
        totalfileslength = (loadcompletednum as libc::c_double
            / fileneedednum as libc::c_double * 256 as libc::c_int as libc::c_double)
            as int32_t;
        M_DrawTextBox(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int
                - 8 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 8 as libc::c_int,
            32 as libc::c_int,
            1 as libc::c_int,
        );
        V_DrawFill(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            256 as libc::c_int,
            8 as libc::c_int,
            111 as libc::c_int,
        );
        V_DrawFill(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            totalfileslength,
            8 as libc::c_int,
            96 as libc::c_int,
        );
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            0x20000 as libc::c_int | 0xc00 as libc::c_int,
            va(
                b" %2u/%2u Files\0" as *const u8 as *const libc::c_char,
                loadcompletednum,
                fileneedednum,
            ),
        );
    } else if lastfilenum != -(1 as libc::c_int) {
        let mut dldlength_0: int32_t = 0;
        static mut tempname: [libc::c_char; 28] = [0; 28];
        let mut file: *mut fileneeded_t = 0 as *mut fileneeded_t;
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        if !snake.is_null() {
            Snake_Draw();
        }
        CL_DrawConnectionStatusBox();
        if !fileneeded.is_null() {
            file = &mut *fileneeded.offset(lastfilenum as isize) as *mut fileneeded_t;
            filename = ((*file).filename).as_mut_ptr();
        } else {
            return
        }
        Net_GetNetStat();
        dldlength_0 = ((*file).currentsize as libc::c_double
            / (*file).totalsize as libc::c_double * 256 as libc::c_int as libc::c_double)
            as int32_t;
        if dldlength_0 > 256 as libc::c_int {
            dldlength_0 = 256 as libc::c_int;
        }
        V_DrawFill(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            256 as libc::c_int,
            8 as libc::c_int,
            111 as libc::c_int,
        );
        V_DrawFill(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            dldlength_0,
            8 as libc::c_int,
            96 as libc::c_int,
        );
        memset(
            tempname.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
        );
        filename = filename
            .offset((strlen(filename)).wrapping_sub(nameonlylength(filename)) as isize);
        if strlen(filename)
            > (::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        {
            let mut endhalfpos: size_t = (strlen(filename))
                .wrapping_sub(10 as libc::c_int as libc::c_ulong);
            snprintf(
                tempname.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong,
                b"%.14s...%.10s\0" as *const u8 as *const libc::c_char,
                filename,
                filename.offset(endhalfpos as isize),
            );
        } else {
            strncpy(
                tempname.as_mut_ptr(),
                filename,
                (::core::mem::size_of::<[libc::c_char; 28]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 24 as libc::c_int,
            0x2000 as libc::c_int,
            va(
                b"Downloading \"%s\"\0" as *const u8 as *const libc::c_char,
                tempname.as_mut_ptr(),
            ),
        );
        V_DrawString(
            320 as libc::c_int / 2 as libc::c_int - 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            0x20000 as libc::c_int | 0xc00 as libc::c_int,
            va(
                b" %4uK/%4uK\0" as *const u8 as *const libc::c_char,
                (*fileneeded.offset(lastfilenum as isize)).currentsize
                    >> 10 as libc::c_int,
                (*file).totalsize >> 10 as libc::c_int,
            ),
        );
        V_DrawRightAlignedString(
            320 as libc::c_int / 2 as libc::c_int + 128 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int,
            0x20000 as libc::c_int | 0xc00 as libc::c_int,
            va(
                b"%3.1fK/s \0" as *const u8 as *const libc::c_char,
                getbps as libc::c_double / 1024 as libc::c_int as libc::c_double,
            ),
        );
    } else {
        if !snake.is_null() {
            Snake_Draw();
        }
        CL_DrawConnectionStatusBox();
        V_DrawCenteredString(
            320 as libc::c_int / 2 as libc::c_int,
            200 as libc::c_int - 16 as libc::c_int - 24 as libc::c_int,
            0x2000 as libc::c_int,
            b"Waiting to download files...\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn CL_AskFileList(mut firstfile: int32_t) -> boolean {
    (*netbuffer).packettype = PT_TELLFILESNEEDED as libc::c_int as uint8_t;
    (*netbuffer).u.filesneedednum = firstfile;
    return HSendPacket(
        servernode as int32_t,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        ::core::mem::size_of::<int32_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn CL_SendJoin() -> boolean {
    let mut localplayers: uint8_t = 1 as libc::c_int as uint8_t;
    let mut player2name: *const libc::c_char = 0 as *const libc::c_char;
    if netgame != 0 {
        CONS_Printf(b"Sending join request...\n\0" as *const u8 as *const libc::c_char);
    }
    (*netbuffer).packettype = PT_CLIENTJOIN as libc::c_int as uint8_t;
    (*netbuffer).u.clientcfg.modversion = 54 as libc::c_int as uint8_t;
    strncpy(
        ((*netbuffer).u.clientcfg.application).as_mut_ptr(),
        b"SRB2\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    if splitscreen != 0 || botingame != 0 {
        localplayers = localplayers.wrapping_add(1);
        localplayers;
    }
    (*netbuffer).u.clientcfg.localplayers = localplayers;
    CleanupPlayerName(consoleplayer, cv_playername.zstring);
    if splitscreen != 0 {
        CleanupPlayerName(1 as libc::c_int, cv_playername2.zstring);
    }
    if botingame != 0 {
        player2name = if strcmp(
            cv_playername.zstring,
            b"Tails\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            b"Tail\0" as *const u8 as *const libc::c_char
        } else {
            b"Tails\0" as *const u8 as *const libc::c_char
        };
    } else {
        player2name = cv_playername2.zstring;
    }
    strncpy(
        ((*netbuffer).u.clientcfg.names[0 as libc::c_int as usize]).as_mut_ptr(),
        cv_playername.zstring,
        21 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        ((*netbuffer).u.clientcfg.names[1 as libc::c_int as usize]).as_mut_ptr(),
        player2name,
        21 as libc::c_int as libc::c_ulong,
    );
    return HSendPacket(
        servernode as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        ::core::mem::size_of::<clientconfig_pak>() as libc::c_ulong,
    );
}
unsafe extern "C" fn FindRejoinerNum(mut node: int8_t) -> int32_t {
    let mut addressbuffer: [libc::c_char; 64] = [0; 64];
    let mut nodeaddress: *const libc::c_char = 0 as *const libc::c_char;
    let mut strippednodeaddress: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: int32_t = 0;
    if I_GetNodeAddress.is_none() {
        return -(1 as libc::c_int);
    }
    nodeaddress = I_GetNodeAddress.expect("non-null function pointer")(node as int32_t);
    if nodeaddress.is_null() {
        return -(1 as libc::c_int);
    }
    strcpy(addressbuffer.as_mut_ptr(), nodeaddress);
    strippednodeaddress = I_NetSplitAddress(
        addressbuffer.as_mut_ptr(),
        0 as *mut *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0
            && playeraddress[i as usize][0 as libc::c_int as usize] as libc::c_int != 0
            && playernode[i as usize] as libc::c_int == 255 as libc::c_int
            && strcmp((playeraddress[i as usize]).as_mut_ptr(), strippednodeaddress) == 0
        {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn GetRefuseReason(mut node: int32_t) -> uint8_t {
    if node == 0 || FindRejoinerNum(node as int8_t) != -(1 as libc::c_int) {
        return 0 as libc::c_int as uint8_t
    } else if !bannednode.is_null() && *bannednode.offset(node as isize) != 0 {
        return REFUSE_BANNED as libc::c_int as uint8_t
    } else if cv_allownewplayer.value == 0 {
        return REFUSE_JOINS_DISABLED as libc::c_int as uint8_t
    } else if D_NumPlayers() >= cv_maxplayers.value {
        return REFUSE_SLOTS_FULL as libc::c_int as uint8_t
    } else {
        return 0 as libc::c_int as uint8_t
    };
}
unsafe extern "C" fn SV_SendServerInfo(mut node: int32_t, mut servertime: tic_t) {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    (*netbuffer).packettype = PT_SERVERINFO as libc::c_int as uint8_t;
    (*netbuffer).u.serverinfo._255 = 255 as libc::c_int as uint8_t;
    (*netbuffer).u.serverinfo.packetversion = 4 as libc::c_int as uint8_t;
    (*netbuffer).u.serverinfo.version = VERSION as uint8_t;
    (*netbuffer).u.serverinfo.subversion = SUBVERSION as uint8_t;
    strncpy(
        ((*netbuffer).u.serverinfo.application).as_mut_ptr(),
        b"SRB2\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    (*netbuffer).u.serverinfo.time = servertime as int32_t as tic_t;
    (*netbuffer).u.serverinfo.leveltime = leveltime as int32_t as tic_t;
    (*netbuffer).u.serverinfo.numberofplayer = (D_NumPlayers() - D_NumBots()) as uint8_t;
    (*netbuffer).u.serverinfo.maxplayer = (cv_maxplayers.value - D_NumBots()) as uint8_t;
    (*netbuffer).u.serverinfo.refusereason = GetRefuseReason(node);
    strncpy(
        ((*netbuffer).u.serverinfo.gametypename).as_mut_ptr(),
        Gametype_Names[gametype as usize],
        ::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong,
    );
    (*netbuffer).u.serverinfo.modifiedgame = modifiedgame as uint8_t;
    (*netbuffer).u.serverinfo.cheatsenabled = CV_CheatsEnabled();
    (*netbuffer)
        .u
        .serverinfo
        .flags = (if dedicated != 0 { 0x40 as libc::c_int } else { 0 as libc::c_int })
        as uint8_t;
    strncpy(
        ((*netbuffer).u.serverinfo.servername).as_mut_ptr(),
        cv_servername.string,
        32 as libc::c_int as libc::c_ulong,
    );
    strncpy(
        ((*netbuffer).u.serverinfo.mapname).as_mut_ptr(),
        G_BuildMapName(gamemap as int32_t),
        7 as libc::c_int as libc::c_ulong,
    );
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        ((*netbuffer).u.serverinfo.mapmd5).as_mut_ptr() as *mut libc::c_void,
        mapmd5.as_mut_ptr() as *const libc::c_void,
        16 as libc::c_int as size_t,
    );
    memset(
        ((*netbuffer).u.serverinfo.maptitle).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong,
    );
    if !(mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize]).is_null()
        && *((*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .lvlttl)
            .as_mut_ptr() as libc::c_int != 0
    {
        let mut read: *mut libc::c_char = ((*mapheaderinfo[(gamemap as libc::c_int
            - 1 as libc::c_int) as usize])
            .lvlttl)
            .as_mut_ptr();
        let mut writ: *mut libc::c_char = ((*netbuffer).u.serverinfo.maptitle)
            .as_mut_ptr();
        while writ
            < ((*netbuffer).u.serverinfo.maptitle)
                .as_mut_ptr()
                .offset(32 as libc::c_int as isize)
            && *read as libc::c_int != '\0' as i32
        {
            if *read as libc::c_int & 0x80 as libc::c_int == 0 {
                *writ = ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *read as libc::c_int;
                            __res = if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = toupper(*read as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*read as libc::c_int as isize);
                    }
                    __res
                }) as libc::c_char;
                writ = writ.offset(1);
                writ;
            }
            read = read.offset(1);
            read;
        }
        *writ = '\0' as i32 as libc::c_char;
    } else {
        strncpy(
            ((*netbuffer).u.serverinfo.maptitle).as_mut_ptr(),
            b"UNKNOWN\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int as libc::c_ulong,
        );
    }
    if !(mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize]).is_null()
        && (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
            .levelflags as libc::c_int & (1 as libc::c_int) << 4 as libc::c_int == 0
    {
        (*netbuffer).u.serverinfo.iszone = 1 as libc::c_int as uint8_t;
    } else {
        (*netbuffer).u.serverinfo.iszone = 0 as libc::c_int as uint8_t;
    }
    if !(mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize]).is_null() {
        (*netbuffer)
            .u
            .serverinfo
            .actnum = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int)
            as usize])
            .actnum;
    }
    p = PutFileNeeded(0 as libc::c_int as uint16_t);
    HSendPacket(
        node,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        p.offset_from(&mut (*netbuffer).u as *mut C2RustUnnamed_7 as *mut uint8_t)
            as libc::c_long as size_t,
    );
}
unsafe extern "C" fn SV_SendPlayerInfo(mut node: int32_t) {
    let mut i: uint8_t = 0;
    (*netbuffer).packettype = PT_PLAYERINFO as libc::c_int as uint8_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 32 as libc::c_int {
        if playeringame[i as usize] == 0 {
            (*netbuffer).u.playerinfo[i as usize].num = 255 as libc::c_int as uint8_t;
        } else {
            (*netbuffer).u.playerinfo[i as usize].num = i;
            strncpy(
                ((*netbuffer).u.playerinfo[i as usize].name).as_mut_ptr(),
                &mut *player_names.as_mut_ptr().offset(i as isize)
                    as *mut [libc::c_char; 22] as *const libc::c_char,
                (21 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            );
            (*netbuffer)
                .u
                .playerinfo[i as usize]
                .name[21 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            memset(
                ((*netbuffer).u.playerinfo[i as usize].address).as_mut_ptr()
                    as *mut libc::c_void,
                0 as libc::c_int,
                4 as libc::c_int as libc::c_ulong,
            );
            if G_GametypeHasTeams() != 0 {
                if players[i as usize].ctfteam == 0 {
                    (*netbuffer)
                        .u
                        .playerinfo[i as usize]
                        .team = 255 as libc::c_int as uint8_t;
                } else {
                    (*netbuffer)
                        .u
                        .playerinfo[i as usize]
                        .team = players[i as usize].ctfteam as uint8_t;
                }
            } else if players[i as usize].spectator != 0 {
                (*netbuffer)
                    .u
                    .playerinfo[i as usize]
                    .team = 255 as libc::c_int as uint8_t;
            } else {
                (*netbuffer).u.playerinfo[i as usize].team = 0 as libc::c_int as uint8_t;
            }
            (*netbuffer)
                .u
                .playerinfo[i as usize]
                .score = players[i as usize].score as int32_t as uint32_t;
            (*netbuffer)
                .u
                .playerinfo[i as usize]
                .timeinserver = (players[i as usize].jointime
                / 35 as libc::c_int as tic_t) as uint16_t as int16_t as uint16_t;
            (*netbuffer)
                .u
                .playerinfo[i as usize]
                .skin = players[i as usize].skin as uint8_t;
            (*netbuffer).u.playerinfo[i as usize].data = 0 as libc::c_int as uint8_t;
            if players[i as usize].pflags as libc::c_uint
                & PF_TAGIT as libc::c_int as libc::c_uint != 0
            {
                (*netbuffer)
                    .u
                    .playerinfo[i as usize]
                    .data = ((*netbuffer).u.playerinfo[i as usize].data as libc::c_int
                    | 0x20 as libc::c_int) as uint8_t;
            }
            if players[i as usize].gotflag != 0 {
                (*netbuffer)
                    .u
                    .playerinfo[i as usize]
                    .data = ((*netbuffer).u.playerinfo[i as usize].data as libc::c_int
                    | 0x40 as libc::c_int) as uint8_t;
            }
            if players[i as usize].powers[pw_super as libc::c_int as usize] != 0 {
                (*netbuffer)
                    .u
                    .playerinfo[i as usize]
                    .data = ((*netbuffer).u.playerinfo[i as usize].data as libc::c_int
                    | 0x80 as libc::c_int) as uint8_t;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    HSendPacket(
        node,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        (::core::mem::size_of::<plrinfo>() as libc::c_ulong)
            .wrapping_mul(32 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn SV_SendServerConfig(mut node: int32_t) -> boolean {
    let mut waspacketsent: boolean = 0;
    (*netbuffer).packettype = PT_SERVERCFG as libc::c_int as uint8_t;
    (*netbuffer).u.servercfg.serverplayer = serverplayer as uint8_t;
    (*netbuffer).u.servercfg.totalslotnum = (*doomcom).numslots as uint8_t;
    (*netbuffer).u.servercfg.gametic = gametic as int32_t as tic_t;
    (*netbuffer).u.servercfg.clientnode = node as uint8_t;
    (*netbuffer).u.servercfg.gamestate = gamestate as uint8_t;
    (*netbuffer).u.servercfg.gametype = gametype as uint8_t;
    (*netbuffer).u.servercfg.modifiedgame = modifiedgame as uint8_t;
    (*netbuffer).u.servercfg.usedCheats = usedCheats as uint8_t;
    memcpy(
        ((*netbuffer).u.servercfg.server_context).as_mut_ptr() as *mut libc::c_void,
        server_context.as_mut_ptr() as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    let len: size_t = ::core::mem::size_of::<serverconfig_pak>() as libc::c_ulong;
    if !debugfile.is_null() {
        fprintf(
            debugfile,
            b"ServerConfig Packet about to be sent, size of packet:%s to node:%d\n\0"
                as *const u8 as *const libc::c_char,
            sizeu1(len),
            node,
        );
    }
    waspacketsent = HSendPacket(
        node,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        len,
    );
    if !debugfile.is_null() {
        if waspacketsent != 0 {
            fprintf(
                debugfile,
                b"ServerConfig Packet was sent\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            fprintf(
                debugfile,
                b"ServerConfig Packet could not be sent right now\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return waspacketsent;
}
unsafe extern "C" fn SV_ResendingSavegameToAnyone() -> boolean {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        if resendingsavegame[i as usize] != 0 {
            return true_0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn SV_SendSaveGame(mut node: int32_t, mut resending: boolean) {
    let mut length: size_t = 0;
    let mut compressedlen: size_t = 0;
    let mut savebuffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut compressedsave: *mut uint8_t = 0 as *mut uint8_t;
    let mut buffertosend: *mut uint8_t = 0 as *mut uint8_t;
    savebuffer = malloc((768 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
        as *mut uint8_t;
    if savebuffer.is_null() {
        CONS_Alert(
            CONS_ERROR,
            b"No more free memory for savegame\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    save_p = savebuffer
        .offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
    P_SaveNetGame(resending);
    length = save_p.offset_from(savebuffer) as libc::c_long as size_t;
    if length > (768 as libc::c_int * 1024 as libc::c_int) as size_t {
        free(savebuffer as *mut libc::c_void);
        save_p = 0 as *mut uint8_t;
        I_Error(b"Savegame buffer overrun\0" as *const u8 as *const libc::c_char);
    }
    compressedsave = malloc(length.wrapping_sub(1 as libc::c_int as size_t))
        as *mut uint8_t;
    if compressedsave.is_null() {
        CONS_Alert(
            CONS_ERROR,
            b"No more free memory for savegame\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    compressedlen = lzf_compress(
        savebuffer.offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *const libc::c_void,
        length.wrapping_sub(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
        compressedsave
            .offset(::core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        length
            .wrapping_sub(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if compressedlen != 0 {
        free(savebuffer as *mut libc::c_void);
        buffertosend = compressedsave;
        let mut p_tmp: *mut uint32_t = compressedsave as *mut libc::c_void
            as *mut uint32_t;
        let tv: uint32_t = length
            .wrapping_sub(::core::mem::size_of::<uint32_t>() as libc::c_ulong)
            as uint32_t;
        memcpy(
            compressedsave as *mut libc::c_void,
            &tv as *const uint32_t as *const libc::c_void,
            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        compressedsave = p_tmp as *mut libc::c_void as *mut uint8_t;
        length = compressedlen
            .wrapping_add(::core::mem::size_of::<uint32_t>() as libc::c_ulong);
    } else {
        free(compressedsave as *mut libc::c_void);
        buffertosend = savebuffer;
        let mut p_tmp_0: *mut uint32_t = savebuffer as *mut libc::c_void
            as *mut uint32_t;
        let tv_0: uint32_t = 0 as libc::c_int as uint32_t;
        memcpy(
            savebuffer as *mut libc::c_void,
            &tv_0 as *const uint32_t as *const libc::c_void,
            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        p_tmp_0 = p_tmp_0.offset(1);
        p_tmp_0;
        savebuffer = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
    }
    AddRamToSendQueue(
        node,
        buffertosend as *mut libc::c_void,
        length,
        SF_RAM,
        0 as libc::c_int as uint8_t,
    );
    save_p = 0 as *mut uint8_t;
    sendingsavegame[node as usize] = true_0 as libc::c_int;
    freezetimeout[node
        as usize] = ((I_GetTime()).wrapping_add(jointimeout) as size_t)
        .wrapping_add(length / 1024 as libc::c_int as size_t) as tic_t;
}
unsafe extern "C" fn CL_LoadReceivedSavegame(mut reloading: boolean) {
    let mut savebuffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut length: size_t = 0;
    let mut decompressedlen: size_t = 0;
    let mut tmpsave: [libc::c_char; 256] = [0; 256];
    FreeFileNeeded();
    sprintf(
        tmpsave.as_mut_ptr(),
        b"%s/$$$.sav\0" as *const u8 as *const libc::c_char,
        srb2home.as_mut_ptr(),
    );
    length = FIL_ReadFileTag(
        tmpsave.as_mut_ptr(),
        &mut savebuffer,
        PU_STATIC as libc::c_int,
    );
    CONS_Printf(
        b"Loading savegame length %s\n\0" as *const u8 as *const libc::c_char,
        sizeu1(length),
    );
    if length == 0 {
        I_Error(b"Can't read savegame sent\0" as *const u8 as *const libc::c_char);
    }
    save_p = savebuffer;
    decompressedlen = ({
        let mut p_tmp: *mut uint32_t = save_p as *mut libc::c_void as *mut uint32_t;
        let mut b: uint32_t = 0;
        memcpy(
            &mut b as *mut uint32_t as *mut libc::c_void,
            save_p as *const libc::c_void,
            ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        save_p = p_tmp as *mut libc::c_void as *mut uint8_t;
        b
    }) as size_t;
    if decompressedlen > 0 as libc::c_int as size_t {
        let mut decompressedbuffer: *mut uint8_t = Z_MallocAlign(
            decompressedlen,
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint8_t;
        lzf_decompress(
            save_p as *const libc::c_void,
            length.wrapping_sub(::core::mem::size_of::<uint32_t>() as libc::c_ulong),
            decompressedbuffer as *mut libc::c_void,
            decompressedlen,
        );
        Z_Free(savebuffer as *mut libc::c_void);
        savebuffer = decompressedbuffer;
        save_p = savebuffer;
    }
    paused = false_0 as libc::c_int as uint8_t;
    demoplayback = false_0 as libc::c_int;
    titlemapinaction = TITLEMAP_OFF as libc::c_int as uint8_t;
    titledemo = false_0 as libc::c_int;
    automapactive = false_0 as libc::c_int;
    if P_LoadNetGame(reloading) != 0 {
        let actnum: uint8_t = (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int)
            as usize])
            .actnum;
        CONS_Printf(
            b"Map is now \"%s\0" as *const u8 as *const libc::c_char,
            G_BuildMapName(gamemap as int32_t),
        );
        if strcmp(
            ((*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
                .lvlttl)
                .as_mut_ptr(),
            b"\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            CONS_Printf(
                b": %s\0" as *const u8 as *const libc::c_char,
                ((*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
                    .lvlttl)
                    .as_mut_ptr(),
            );
            if (*mapheaderinfo[(gamemap as libc::c_int - 1 as libc::c_int) as usize])
                .levelflags as libc::c_int & (1 as libc::c_int) << 4 as libc::c_int == 0
            {
                CONS_Printf(b" Zone\0" as *const u8 as *const libc::c_char);
            }
            if actnum as libc::c_int > 0 as libc::c_int {
                CONS_Printf(
                    b" %2d\0" as *const u8 as *const libc::c_char,
                    actnum as libc::c_int,
                );
            }
        }
        CONS_Printf(b"\"\n\0" as *const u8 as *const libc::c_char);
    }
    Z_Free(savebuffer as *mut libc::c_void);
    save_p = 0 as *mut uint8_t;
    if unlink(tmpsave.as_mut_ptr()) == -(1 as libc::c_int) {
        CONS_Alert(
            CONS_ERROR,
            b"Can't delete %s\n\0" as *const u8 as *const libc::c_char,
            tmpsave.as_mut_ptr(),
        );
    }
    consistancy[(gametic % 1024 as libc::c_int as tic_t) as usize] = Consistancy();
    CON_ToggleOff();
    (*netbuffer).packettype = PT_RECEIVEDGAMESTATE as libc::c_int as uint8_t;
    HSendPacket(
        servernode as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn CL_ReloadReceivedSavegame() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        LUA_InvalidatePlayer(&mut *players.as_mut_ptr().offset(i as isize));
        sprintf(
            (player_names[i as usize]).as_mut_ptr(),
            b"Player %d\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        i += 1;
        i;
    }
    CL_LoadReceivedSavegame(true_0 as libc::c_int);
    if neededtic < gametic {
        neededtic = gametic;
    }
    maketic = neededtic;
    ticcmd_oldangleturn[0 as libc::c_int
        as usize] = players[consoleplayer as usize].oldrelangleturn;
    P_ForceLocalAngle(
        &mut *players.as_mut_ptr().offset(consoleplayer as isize),
        ((players[consoleplayer as usize].angleturn as libc::c_int) << 16 as libc::c_int)
            as angle_t,
    );
    if splitscreen != 0 {
        ticcmd_oldangleturn[1 as libc::c_int
            as usize] = players[secondarydisplayplayer as usize].oldrelangleturn;
        P_ForceLocalAngle(
            &mut *players.as_mut_ptr().offset(secondarydisplayplayer as isize),
            ((players[secondarydisplayplayer as usize].angleturn as libc::c_int)
                << 16 as libc::c_int) as angle_t,
        );
    }
    camera.subsector = R_PointInSubsector(camera.x, camera.y);
    camera2.subsector = R_PointInSubsector(camera2.x, camera2.y);
    cl_redownloadinggamestate = false_0 as libc::c_int;
    CONS_Printf(b"Game state reloaded\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn SendAskInfo(mut node: int32_t) {
    let asktime: tic_t = I_GetTime();
    (*netbuffer).packettype = PT_ASKINFO as libc::c_int as uint8_t;
    (*netbuffer).u.askinfo.version = VERSION as uint8_t;
    (*netbuffer).u.askinfo.time = asktime as int32_t as tic_t;
    HSendPacket(
        node,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        ::core::mem::size_of::<askinfo_pak>() as libc::c_ulong,
    );
}
#[no_mangle]
pub static mut serverlist: [serverelem_t; 126] = [serverelem_t {
    node: 0,
    info: serverinfo_pak {
        _255: 0,
        packetversion: 0,
        application: [0; 16],
        version: 0,
        subversion: 0,
        numberofplayer: 0,
        maxplayer: 0,
        refusereason: 0,
        gametypename: [0; 24],
        modifiedgame: 0,
        cheatsenabled: 0,
        flags: 0,
        fileneedednum: 0,
        time: 0,
        leveltime: 0,
        servername: [0; 32],
        mapname: [0; 8],
        maptitle: [0; 33],
        mapmd5: [0; 16],
        actnum: 0,
        iszone: 0,
        fileneeded: [0; 915],
    },
}; 126];
#[no_mangle]
pub static mut serverlistcount: uint32_t = 0 as libc::c_int as uint32_t;
unsafe extern "C" fn SL_ClearServerList(mut connectedserver: int32_t) {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < serverlistcount {
        if connectedserver != serverlist[i as usize].node as libc::c_int {
            Net_CloseConnection(
                serverlist[i as usize].node as libc::c_int | 0x8000 as libc::c_int,
            );
            serverlist[i as usize].node = 0 as libc::c_int as int8_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    serverlistcount = 0 as libc::c_int as uint32_t;
}
unsafe extern "C" fn SL_SearchServer(mut node: int32_t) -> uint32_t {
    let mut i: uint32_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < serverlistcount {
        if serverlist[i as usize].node as libc::c_int == node {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 4294967295 as libc::c_uint;
}
unsafe extern "C" fn SL_InsertServer(mut info: *mut serverinfo_pak, mut node: int8_t) {
    let mut i: uint32_t = 0;
    i = SL_SearchServer(node as int32_t);
    if i == 4294967295 as libc::c_uint {
        if serverlistcount >= (127 as libc::c_int - 1 as libc::c_int) as uint32_t {
            return;
        }
        if node as libc::c_int != servernode as libc::c_int {
            if (*info)._255 as libc::c_int != 255 as libc::c_int {
                return;
            }
            if (*info).packetversion as libc::c_int != 4 as libc::c_int {
                return;
            }
            if (*info).version as libc::c_int != VERSION {
                return;
            }
            if (*info).subversion as libc::c_int != SUBVERSION {
                return;
            }
            if strcmp(
                ((*info).application).as_mut_ptr(),
                b"SRB2\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                return;
            }
        }
        let fresh0 = serverlistcount;
        serverlistcount = serverlistcount.wrapping_add(1);
        i = fresh0;
    }
    serverlist[i as usize].info = *info;
    serverlist[i as usize].node = node;
    M_SortServerList();
}
#[no_mangle]
pub unsafe extern "C" fn CL_QueryServerList(mut server_list: *mut msg_server_t) {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while (*server_list.offset(i as isize)).header.buffer[0 as libc::c_int as usize] != 0
    {
        let mut node: int32_t = I_NetMakeNodewPort
            .expect(
                "non-null function pointer",
            )(
            ((*server_list.offset(i as isize)).ip).as_mut_ptr(),
            ((*server_list.offset(i as isize)).port).as_mut_ptr(),
        ) as int32_t;
        if node == -(1 as libc::c_int) {
            break;
        }
        SendAskInfo(node);
        Net_CloseConnection(node | 0x8000 as libc::c_int);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CL_UpdateServerList(
    mut internetsearch: boolean,
    mut room: int32_t,
) {
    SL_ClearServerList(0 as libc::c_int);
    if netgame == 0 && I_NetOpenSocket.is_some() {
        if I_NetOpenSocket.expect("non-null function pointer")() != 0 {
            netgame = true_0 as libc::c_int;
            multiplayer = true_0 as libc::c_int;
        }
    }
    if netgame != 0 {
        SendAskInfo(127 as libc::c_int);
    }
    if internetsearch != 0 {
        let mut server_list: *mut msg_server_t = 0 as *mut msg_server_t;
        server_list = GetShortServersList(room, 0 as libc::c_int);
        if !server_list.is_null() {
            CL_QueryServerList(server_list);
            free(server_list as *mut libc::c_void);
        }
    }
}
unsafe extern "C" fn M_ConfirmConnect(mut ev: *mut event_t) {
    if (*ev).type_0 as libc::c_uint == ev_keydown as libc::c_int as libc::c_uint {
        if (*ev).key == ' ' as i32 || (*ev).key == 'y' as i32
            || (*ev).key == 13 as libc::c_int || (*ev).key == KEY_JOY1 as libc::c_int
        {
            if totalfilesrequestednum > 0 as libc::c_int {
                if CL_SendFileRequest() != 0 {
                    cl_mode = CL_DOWNLOADFILES;
                    Snake_Initialise();
                }
            } else {
                cl_mode = CL_LOADFILES;
            }
            M_ClearMenus(true_0 as libc::c_int);
        } else if (*ev).key == 'n' as i32 || (*ev).key == 27 as libc::c_int
            || (*ev).key == KEY_JOY1 as libc::c_int + 3 as libc::c_int
        {
            cl_mode = CL_ABORTED;
            M_ClearMenus(true_0 as libc::c_int);
        }
    }
}
unsafe extern "C" fn CL_FinishedFileList() -> boolean {
    let mut i: int32_t = 0;
    let mut downloadsize: *mut libc::c_char = 0 as *mut libc::c_char;
    i = CL_CheckFiles();
    if i == 4 as libc::c_int {
        return true_0 as libc::c_int
    } else if i == 3 as libc::c_int {
        D_QuitNetGame();
        CL_Reset();
        D_StartTitle();
        M_StartMessage(
            b"You have too many WAD files loaded\nto add ones the server is using.\nPlease restart SRB2 before connecting.\n\nPress ESC\n\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
            MM_NOTHING,
        );
        return false_0 as libc::c_int;
    } else if i == 2 as libc::c_int {
        D_QuitNetGame();
        CL_Reset();
        D_StartTitle();
        M_StartMessage(
            b"You have the wrong addons loaded.\n\nTo play on this server, restart\nthe game and don't load any addons.\nSRB2 will automatically add\neverything you need when you join.\n\nPress ESC\n\0"
                as *const u8 as *const libc::c_char,
            0 as *mut libc::c_void,
            MM_NOTHING,
        );
        return false_0 as libc::c_int;
    } else if i == 1 as libc::c_int {
        if serverisfull != 0 {
            M_StartMessage(
                b"This server is full!\n\nYou may load server addons (if any), and wait for a slot.\n\nPress ENTER to continue\nor ESC to cancel.\n\n\0"
                    as *const u8 as *const libc::c_char,
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut event_t) -> ()>,
                    *mut libc::c_void,
                >(Some(M_ConfirmConnect as unsafe extern "C" fn(*mut event_t) -> ())),
                MM_EVENTHANDLER,
            );
            cl_mode = CL_CONFIRMCONNECT;
            curfadevalue = 0 as libc::c_int as int8_t;
        } else {
            cl_mode = CL_LOADFILES;
        }
    } else {
        if CL_CheckDownloadable() == 0 {
            D_QuitNetGame();
            CL_Reset();
            D_StartTitle();
            M_StartMessage(
                b"An error occured when trying to\ndownload missing addons.\n(This is almost always a problem\nwith the server, not your game.)\n\nSee the console or log file\nfor additional details.\n\nPress ESC\n\0"
                    as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
            return false_0 as libc::c_int;
        }
        downloadcompletednum = 0 as libc::c_int;
        downloadcompletedsize = 0 as libc::c_int as uint32_t;
        totalfilesrequestednum = 0 as libc::c_int;
        totalfilesrequestedsize = 0 as libc::c_int as uint32_t;
        if fileneeded.is_null() {
            I_Error(
                b"CL_FinishedFileList: fileneeded == NULL\0" as *const u8
                    as *const libc::c_char,
            );
        }
        i = 0 as libc::c_int;
        while i < fileneedednum {
            if (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_NOTFOUND as libc::c_int as libc::c_uint
                || (*fileneeded.offset(i as isize)).status as libc::c_uint
                    == FS_MD5SUMBAD as libc::c_int as libc::c_uint
            {
                totalfilesrequestednum += 1;
                totalfilesrequestednum;
                totalfilesrequestedsize = totalfilesrequestedsize
                    .wrapping_add((*fileneeded.offset(i as isize)).totalsize);
            }
            i += 1;
            i;
        }
        if totalfilesrequestedsize >> 20 as libc::c_int >= 100 as libc::c_int as uint32_t
        {
            downloadsize = Z_StrDup(
                va(
                    b"%uM\0" as *const u8 as *const libc::c_char,
                    totalfilesrequestedsize >> 20 as libc::c_int,
                ),
            );
        } else {
            downloadsize = Z_StrDup(
                va(
                    b"%uK\0" as *const u8 as *const libc::c_char,
                    totalfilesrequestedsize >> 10 as libc::c_int,
                ),
            );
        }
        if serverisfull != 0 {
            M_StartMessage(
                va(
                    b"This server is full!\nDownload of %s additional content\nis required to join.\n\nYou may download, load server addons,\nand wait for a slot.\n\nPress ENTER to continue\nor ESC to cancel.\n\0"
                        as *const u8 as *const libc::c_char,
                    downloadsize,
                ),
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut event_t) -> ()>,
                    *mut libc::c_void,
                >(Some(M_ConfirmConnect as unsafe extern "C" fn(*mut event_t) -> ())),
                MM_EVENTHANDLER,
            );
        } else {
            M_StartMessage(
                va(
                    b"Download of %s additional content\nis required to join.\n\nPress ENTER to continue\nor ESC to cancel.\n\0"
                        as *const u8 as *const libc::c_char,
                    downloadsize,
                ),
                ::core::mem::transmute::<
                    Option::<unsafe extern "C" fn(*mut event_t) -> ()>,
                    *mut libc::c_void,
                >(Some(M_ConfirmConnect as unsafe extern "C" fn(*mut event_t) -> ())),
                MM_EVENTHANDLER,
            );
        }
        Z_Free(downloadsize as *mut libc::c_void);
        cl_mode = CL_CONFIRMCONNECT;
        curfadevalue = 0 as libc::c_int as int8_t;
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn InvalidServerReason(
    mut info: *mut serverinfo_pak,
) -> *const libc::c_char {
    if (*info)._255 as libc::c_int != 255 as libc::c_int {
        return b"Outdated server (version unknown).\n\nPress ESC\n\0" as *const u8
            as *const libc::c_char;
    }
    if strncmp(
        ((*info).application).as_mut_ptr(),
        b"SRB2\0" as *const u8 as *const libc::c_char,
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    ) != 0
    {
        return va(
            b"%s cannot connect\nto %s servers.\n\nPress ESC\n\0" as *const u8
                as *const libc::c_char,
            b"SRB2\0" as *const u8 as *const libc::c_char,
            ((*info).application).as_mut_ptr(),
        );
    }
    if (*info).packetversion as libc::c_int != 4 as libc::c_int
        || (*info).version as libc::c_int != VERSION
        || (*info).subversion as libc::c_int != SUBVERSION
    {
        return va(
            b"Incompatible %s versions.\n(server version %d.%d.%d)\n\nPress ESC\n\0"
                as *const u8 as *const libc::c_char,
            b"SRB2\0" as *const u8 as *const libc::c_char,
            (*info).version as libc::c_int / 100 as libc::c_int,
            (*info).version as libc::c_int % 100 as libc::c_int,
            (*info).subversion as libc::c_int,
        );
    }
    match (*info).refusereason as libc::c_int {
        3 => {
            return b"You have been banned\nfrom the server.\n\nPress ESC\n\0"
                as *const u8 as *const libc::c_char;
        }
        1 => {
            return b"The server is not accepting\njoins for the moment.\n\nPress ESC\n\0"
                as *const u8 as *const libc::c_char;
        }
        2 => {
            return va(
                b"Maximum players reached: %d\n\nPress ESC\n\0" as *const u8
                    as *const libc::c_char,
                (*info).maxplayer as libc::c_int,
            );
        }
        _ => {
            if (*info).refusereason != 0 {
                return b"You can't join.\nI don't know why,\nbut you can't join.\n\nPress ESC\n\0"
                    as *const u8 as *const libc::c_char;
            }
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn CL_ServerConnectionSearchTicker(
    mut asksent: *mut tic_t,
) -> boolean {
    let mut i: int32_t = 0;
    if serverlistcount > 0 as libc::c_int as uint32_t {
        if servernode as libc::c_int == -(1 as libc::c_int)
            || servernode as libc::c_int >= 127 as libc::c_int
        {
            i = 0 as libc::c_int;
            servernode = serverlist[i as usize].node;
            CONS_Printf(b"Found, \0" as *const u8 as *const libc::c_char);
        } else {
            i = SL_SearchServer(servernode as int32_t) as int32_t;
            if i < 0 as libc::c_int {
                return true_0 as libc::c_int;
            }
        }
        if server == 0 {
            let mut info: *mut serverinfo_pak = &mut (*serverlist
                .as_mut_ptr()
                .offset(i as isize))
                .info;
            if (*info).refusereason as libc::c_int == REFUSE_SLOTS_FULL as libc::c_int {
                serverisfull = true_0 as libc::c_int;
            } else {
                let mut reason: *const libc::c_char = InvalidServerReason(info);
                if !reason.is_null() {
                    let mut message: *mut libc::c_char = Z_StrDup(reason);
                    D_QuitNetGame();
                    CL_Reset();
                    D_StartTitle();
                    M_StartMessage(message, 0 as *mut libc::c_void, MM_NOTHING);
                    Z_Free(message as *mut libc::c_void);
                    return false_0 as libc::c_int;
                }
            }
            D_ParseFileneeded(
                (*info).fileneedednum as int32_t,
                ((*info).fileneeded).as_mut_ptr(),
                0 as libc::c_int as uint16_t,
            );
            if (*info).flags as libc::c_int & 0x20 as libc::c_int != 0 {
                cl_mode = CL_ASKFULLFILELIST;
                cl_lastcheckedfilecount = 0 as libc::c_int as uint16_t;
                return true_0 as libc::c_int;
            }
            cl_mode = CL_CHECKFILES;
        } else {
            cl_mode = CL_ASKJOIN;
            *asksent = 0 as libc::c_int as tic_t;
        }
        return true_0 as libc::c_int;
    }
    if (*asksent).wrapping_add((35 as libc::c_int * 1 as libc::c_int) as tic_t)
        < I_GetTime()
    {
        SendAskInfo(servernode as int32_t);
        *asksent = I_GetTime();
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn CL_ServerConnectionTicker(
    mut tmpsave: *const libc::c_char,
    mut oldtic: *mut tic_t,
    mut asksent: *mut tic_t,
) -> boolean {
    let mut waitmore: boolean = 0;
    let mut i: int32_t = 0;
    match cl_mode as libc::c_uint {
        0 => {
            if CL_ServerConnectionSearchTicker(asksent) == 0 {
                return false_0 as libc::c_int;
            }
        }
        9 => {
            if cl_lastcheckedfilecount as libc::c_int == 65535 as libc::c_int {
                cl_mode = CL_CHECKFILES;
            } else if fileneedednum != cl_lastcheckedfilecount as libc::c_int
                || I_GetTime() >= *asksent
            {
                if CL_AskFileList(fileneedednum) != 0 {
                    cl_lastcheckedfilecount = fileneedednum as uint16_t;
                    *asksent = (I_GetTime())
                        .wrapping_add((35 as libc::c_int * 1 as libc::c_int) as tic_t);
                }
            }
        }
        1 => {
            if CL_FinishedFileList() == 0 {
                return false_0 as libc::c_int;
            }
        }
        2 => {
            waitmore = false_0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < fileneedednum {
                if (*fileneeded.offset(i as isize)).status as libc::c_uint
                    == FS_DOWNLOADING as libc::c_int as libc::c_uint
                    || (*fileneeded.offset(i as isize)).status as libc::c_uint
                        == FS_REQUESTED as libc::c_int as libc::c_uint
                {
                    waitmore = true_0 as libc::c_int;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if !(waitmore != 0) {
                if !snake.is_null() {
                    free(snake as *mut libc::c_void);
                    snake = 0 as *mut snake_t;
                }
                cl_mode = CL_LOADFILES;
            }
        }
        4 => {
            if CL_LoadServerFiles() != 0 {
                FreeFileNeeded();
                *asksent = 0 as libc::c_int as tic_t;
                firstconnectattempttime = I_GetTime();
                cl_mode = CL_ASKJOIN;
            }
        }
        3 => {
            if firstconnectattempttime
                .wrapping_add(
                    (35 as libc::c_int * 1 as libc::c_int * 300 as libc::c_int) as tic_t,
                ) < I_GetTime() && server == 0
            {
                CONS_Printf(
                    b"5 minute wait time exceeded.\n\0" as *const u8
                        as *const libc::c_char,
                );
                CONS_Printf(
                    b"Network game synchronization aborted.\n\0" as *const u8
                        as *const libc::c_char,
                );
                D_QuitNetGame();
                CL_Reset();
                D_StartTitle();
                M_StartMessage(
                    b"5 minute wait time exceeded.\nYou may retry connection.\n\nPress ESC\n\0"
                        as *const u8 as *const libc::c_char,
                    0 as *mut libc::c_void,
                    MM_NOTHING,
                );
                return false_0 as libc::c_int;
            }
            CL_PrepareDownloadSaveGame(tmpsave);
            if I_GetTime() >= *asksent && CL_SendJoin() != 0 {
                *asksent = (I_GetTime())
                    .wrapping_add(
                        (35 as libc::c_int * 1 as libc::c_int * 3 as libc::c_int)
                            as tic_t,
                    );
                cl_mode = CL_WAITJOINRESPONSE;
            }
        }
        5 => {
            if I_GetTime() >= *asksent {
                cl_mode = CL_ASKJOIN;
            }
        }
        6 => {
            if (*fileneeded.offset(0 as libc::c_int as isize)).status as libc::c_uint
                == FS_FOUND as libc::c_int as libc::c_uint
            {
                CL_LoadReceivedSavegame(false_0 as libc::c_int);
                cl_mode = CL_CONNECTED;
            }
        }
        8 => {
            cl_mode = CL_SEARCHING;
            return false_0 as libc::c_int;
        }
        7 | 10 | _ => {}
    }
    GetPackets();
    Net_AckTicker();
    if *oldtic != I_GetTime() {
        I_OsPolling();
        if cl_mode as libc::c_uint == CL_CONFIRMCONNECT as libc::c_int as libc::c_uint {
            D_ProcessEvents();
        } else {
            while eventtail != eventhead {
                if Snake_Joy_Grabber(
                    &mut *events.as_mut_ptr().offset(eventtail as isize),
                ) == 0
                {
                    G_MapEventsToControls(
                        &mut *events.as_mut_ptr().offset(eventtail as isize),
                    );
                }
                eventtail = eventtail + 1 as libc::c_int
                    & 128 as libc::c_int - 1 as libc::c_int;
            }
        }
        if gamekeydown[27 as libc::c_int as usize] as libc::c_int != 0
            || gamekeydown[(KEY_JOY1 as libc::c_int + 1 as libc::c_int) as usize]
                as libc::c_int != 0
            || cl_mode as libc::c_uint == CL_ABORTED as libc::c_int as libc::c_uint
        {
            CONS_Printf(
                b"Network game synchronization aborted.\n\0" as *const u8
                    as *const libc::c_char,
            );
            M_StartMessage(
                b"Network game synchronization aborted.\n\nPress ESC\n\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
            if !snake.is_null() {
                free(snake as *mut libc::c_void);
                snake = 0 as *mut snake_t;
            }
            D_QuitNetGame();
            CL_Reset();
            D_StartTitle();
            memset(
                gamekeydown.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                256 as libc::c_int as libc::c_ulong,
            );
            return false_0 as libc::c_int;
        } else if cl_mode as libc::c_uint
            == CL_DOWNLOADFILES as libc::c_int as libc::c_uint && !snake.is_null()
        {
            Snake_Handle();
        }
        if server == 0
            && (cl_mode as libc::c_uint
                == CL_DOWNLOADFILES as libc::c_int as libc::c_uint
                || cl_mode as libc::c_uint
                    == CL_DOWNLOADSAVEGAME as libc::c_int as libc::c_uint)
        {
            FileReceiveTicker();
        }
        *oldtic = I_GetTime();
        if server == 0
            && cl_mode as libc::c_uint != CL_CONNECTED as libc::c_int as libc::c_uint
            && cl_mode as libc::c_uint != CL_ABORTED as libc::c_int as libc::c_uint
        {
            if snake.is_null() {
                F_MenuPresTicker();
                F_TitleScreenTicker(true_0 as libc::c_int);
                F_TitleScreenDrawer();
            }
            CL_DrawConnectionStatus();
            M_Drawer();
            I_UpdateNoVsync();
            if moviemode as u64 != 0 {
                M_SaveFrame();
            }
            S_UpdateSounds();
            S_UpdateClosedCaptions();
        }
    } else {
        I_Sleep(cv_sleep.value as uint32_t);
        I_UpdateTime(cv_timescale.value);
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn CL_ConnectToServer() {
    let mut pnumnodes: int32_t = 0;
    let mut nodewaited: int32_t = (*doomcom).numnodes as int32_t;
    let mut i: int32_t = 0;
    let mut oldtic: tic_t = 0;
    let mut asksent: tic_t = 0;
    let mut tmpsave: [libc::c_char; 256] = [0; 256];
    sprintf(
        tmpsave.as_mut_ptr(),
        b"%s/$$$.sav\0" as *const u8 as *const libc::c_char,
        srb2home.as_mut_ptr(),
    );
    lastfilenum = -(1 as libc::c_int);
    cl_mode = CL_SEARCHING;
    if FIL_FileExists(tmpsave.as_mut_ptr()) != 0
        && unlink(tmpsave.as_mut_ptr()) == -(1 as libc::c_int)
    {
        I_Error(
            b"Can't delete %s\n\0" as *const u8 as *const libc::c_char,
            tmpsave.as_mut_ptr(),
        );
    }
    if netgame != 0 {
        if (servernode as libc::c_int) < 0 as libc::c_int
            || servernode as libc::c_int >= 127 as libc::c_int
        {
            CONS_Printf(
                b"Searching for a server...\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            CONS_Printf(
                b"Contacting the server...\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if gamestate as libc::c_uint == GS_INTERMISSION as libc::c_int as libc::c_uint {
        Y_EndIntermission();
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"waiting %d nodes\n\0" as *const u8 as *const libc::c_char,
                (*doomcom).numnodes as libc::c_int,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    G_SetGamestate(GS_WAITINGPLAYERS);
    wipegamestate = GS_WAITINGPLAYERS;
    ClearAdminPlayers();
    pnumnodes = 1 as libc::c_int;
    oldtic = (I_GetTime()).wrapping_sub(1 as libc::c_int as tic_t);
    asksent = -(35 as libc::c_int) as tic_t;
    firstconnectattempttime = I_GetTime();
    i = SL_SearchServer(servernode as int32_t) as int32_t;
    if i != -(1 as libc::c_int) {
        let mut gametypestr: *mut libc::c_char = (serverlist[i as usize]
            .info
            .gametypename)
            .as_mut_ptr();
        CONS_Printf(
            b"Connecting to: %s\n\0" as *const u8 as *const libc::c_char,
            (serverlist[i as usize].info.servername).as_mut_ptr(),
        );
        *gametypestr
            .offset(
                (::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
        CONS_Printf(
            b"Gametype: %s\n\0" as *const u8 as *const libc::c_char,
            gametypestr,
        );
        CONS_Printf(
            b"Version: %d.%d.%u\n\0" as *const u8 as *const libc::c_char,
            serverlist[i as usize].info.version as libc::c_int / 100 as libc::c_int,
            serverlist[i as usize].info.version as libc::c_int % 100 as libc::c_int,
            serverlist[i as usize].info.subversion as libc::c_int,
        );
    }
    SL_ClearServerList(servernode as int32_t);
    loop {
        if CL_ServerConnectionTicker(tmpsave.as_mut_ptr(), &mut oldtic, &mut asksent)
            == 0
        {
            return;
        }
        if server != 0 {
            pnumnodes = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 127 as libc::c_int {
                if nodeingame[i as usize] != 0 {
                    pnumnodes += 1;
                    pnumnodes;
                }
                i += 1;
                i;
            }
        }
        if cl_mode as libc::c_uint == CL_CONNECTED as libc::c_int as libc::c_uint
            && (server == 0 || server != 0 && nodewaited <= pnumnodes)
        {
            break;
        }
    }
    if netgame != 0 {
        F_StartWaitingPlayers();
    }
    if !debugfile.is_null() {
        fputs(
            va(b"Synchronisation Finished\n\0" as *const u8 as *const libc::c_char),
            debugfile,
        );
        fflush(debugfile);
    }
    displayplayer = consoleplayer;
}
static mut reasontail: *mut banreason_t = 0 as *const banreason_t as *mut banreason_t;
static mut reasonhead: *mut banreason_t = 0 as *const banreason_t as *mut banreason_t;
unsafe extern "C" fn Command_ShowBan() {
    let mut i: size_t = 0;
    let mut address: *const libc::c_char = 0 as *const libc::c_char;
    let mut mask: *const libc::c_char = 0 as *const libc::c_char;
    let mut reasonlist: *mut banreason_t = reasonhead;
    if I_GetBanAddress.is_some() {
        CONS_Printf(b"Ban List:\n\0" as *const u8 as *const libc::c_char);
    } else {
        return
    }
    i = 0 as libc::c_int as size_t;
    loop {
        address = I_GetBanAddress.expect("non-null function pointer")(i);
        if address.is_null() {
            break;
        }
        if I_GetBanMask.is_none()
            || {
                mask = I_GetBanMask.expect("non-null function pointer")(i);
                mask.is_null()
            }
        {
            CONS_Printf(
                b"%s: %s \0" as *const u8 as *const libc::c_char,
                sizeu1(i.wrapping_add(1 as libc::c_int as size_t)),
                address,
            );
        } else {
            CONS_Printf(
                b"%s: %s/%s \0" as *const u8 as *const libc::c_char,
                sizeu1(i.wrapping_add(1 as libc::c_int as size_t)),
                address,
                mask,
            );
        }
        if !reasonlist.is_null() && !((*reasonlist).reason).is_null() {
            CONS_Printf(
                b"(%s)\n\0" as *const u8 as *const libc::c_char,
                (*reasonlist).reason,
            );
        } else {
            CONS_Printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        if !reasonlist.is_null() {
            reasonlist = (*reasonlist).next;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i == 0 as libc::c_int as size_t && address.is_null() {
        CONS_Printf(b"(empty)\n\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_SaveBan() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: size_t = 0;
    let mut reasonlist: *mut banreason_t = reasonhead;
    let mut address: *const libc::c_char = 0 as *const libc::c_char;
    let mut mask: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = va(
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        srb2home.as_mut_ptr(),
        b"ban.txt\0" as *const u8 as *const libc::c_char,
    );
    if reasonhead.is_null() {
        remove(path);
        return;
    }
    f = fopen(path, b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        CONS_Alert(
            CONS_WARNING,
            b"Could not save ban list into ban.txt\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int as size_t;
    loop {
        address = I_GetBanAddress.expect("non-null function pointer")(i);
        if address.is_null() {
            break;
        }
        if I_GetBanMask.is_none()
            || {
                mask = I_GetBanMask.expect("non-null function pointer")(i);
                mask.is_null()
            }
        {
            fprintf(f, b"%s 0\0" as *const u8 as *const libc::c_char, address);
        } else {
            fprintf(f, b"%s %s\0" as *const u8 as *const libc::c_char, address, mask);
        }
        if !reasonlist.is_null() && !((*reasonlist).reason).is_null() {
            fprintf(
                f,
                b" %s\n\0" as *const u8 as *const libc::c_char,
                (*reasonlist).reason,
            );
        } else {
            fprintf(
                f,
                b" %s\n\0" as *const u8 as *const libc::c_char,
                b"NA\0" as *const u8 as *const libc::c_char,
            );
        }
        if !reasonlist.is_null() {
            reasonlist = (*reasonlist).next;
        }
        i = i.wrapping_add(1);
        i;
    }
    fclose(f);
}
unsafe extern "C" fn Ban_Add(mut reason: *const libc::c_char) {
    let mut reasonlist: *mut banreason_t = malloc(
        ::core::mem::size_of::<banreason_t>() as libc::c_ulong,
    ) as *mut banreason_t;
    if reasonlist.is_null() {
        return;
    }
    if reason.is_null() {
        reason = b"NA\0" as *const u8 as *const libc::c_char;
    }
    (*reasonlist).next = 0 as *mut banreason_s;
    (*reasonlist).reason = Z_StrDup(reason);
    (*reasonlist).prev = reasontail;
    if ((*reasonlist).prev).is_null() {
        reasonhead = reasonlist;
    } else {
        (*reasontail).next = reasonlist;
    }
    reasontail = reasonlist;
}
unsafe extern "C" fn Ban_Clear() {
    let mut temp: *mut banreason_t = 0 as *mut banreason_t;
    I_ClearBans.expect("non-null function pointer")();
    reasontail = 0 as *mut banreason_t;
    while !reasonhead.is_null() {
        temp = (*reasonhead).next;
        Z_Free((*reasonhead).reason as *mut libc::c_void);
        free(reasonhead as *mut libc::c_void);
        reasonhead = temp;
    }
}
unsafe extern "C" fn Command_ClearBans() {
    if I_ClearBans.is_none() {
        return;
    }
    Ban_Clear();
    D_SaveBan();
}
unsafe extern "C" fn Ban_Load_File(mut warning: boolean) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut address: *const libc::c_char = 0 as *const libc::c_char;
    let mut mask: *const libc::c_char = 0 as *const libc::c_char;
    let mut buffer: [libc::c_char; 512] = [0; 512];
    if I_ClearBans.is_none() {
        return;
    }
    f = fopen(
        va(
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            srb2home.as_mut_ptr(),
            b"ban.txt\0" as *const u8 as *const libc::c_char,
        ),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        if warning != 0 {
            CONS_Alert(
                CONS_WARNING,
                b"Could not open ban.txt for ban list\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    }
    Ban_Clear();
    while !(fgets(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        address = strtok(
            buffer.as_mut_ptr(),
            b" \t\r\n\0" as *const u8 as *const libc::c_char,
        );
        mask = strtok(
            0 as *mut libc::c_char,
            b" \t\r\n\0" as *const u8 as *const libc::c_char,
        );
        I_SetBanAddress.expect("non-null function pointer")(address, mask);
        Ban_Add(
            strtok(0 as *mut libc::c_char, b"\r\n\0" as *const u8 as *const libc::c_char),
        );
    }
    fclose(f);
}
unsafe extern "C" fn Command_ReloadBan() {
    Ban_Load_File(true_0 as libc::c_int);
}
unsafe extern "C" fn Command_connect() {
    if COM_Argc() < 2 as libc::c_int as size_t
        || *COM_Argv(1 as libc::c_int as size_t) as libc::c_int == 0 as libc::c_int
    {
        CONS_Printf(
            b"Connect <serveraddress> (port): connect to a server\nConnect ANY: connect to the first lan server found\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if Playing() != 0 || titledemo != 0 {
        CONS_Printf(
            b"You cannot connect while in a game. End this game first.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    server = false_0 as libc::c_int;
    if netgame != 0
        && strcasecmp(
            COM_Argv(1 as libc::c_int as size_t),
            b"node\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        servernode = atoi(COM_Argv(2 as libc::c_int as size_t)) as int8_t;
    } else if netgame != 0 {
        CONS_Printf(
            b"You cannot connect while in a game. End this game first.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    } else if I_NetOpenSocket.is_some() {
        I_NetOpenSocket.expect("non-null function pointer")();
        netgame = true_0 as libc::c_int;
        multiplayer = true_0 as libc::c_int;
        if strcasecmp(
            COM_Argv(1 as libc::c_int as size_t),
            b"any\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            servernode = 127 as libc::c_int as int8_t;
        } else if I_NetMakeNodewPort.is_some() {
            if COM_Argc() >= 3 as libc::c_int as size_t {
                servernode = I_NetMakeNodewPort
                    .expect(
                        "non-null function pointer",
                    )(
                    COM_Argv(1 as libc::c_int as size_t),
                    COM_Argv(2 as libc::c_int as size_t),
                );
            } else {
                servernode = I_NetMakeNode(COM_Argv(1 as libc::c_int as size_t));
            }
        } else {
            CONS_Alert(
                CONS_ERROR,
                b"There is no server identification with this network driver\n\0"
                    as *const u8 as *const libc::c_char,
            );
            D_CloseConnection();
            return;
        }
    } else {
        CONS_Alert(
            CONS_ERROR,
            b"There is no network driver\n\0" as *const u8 as *const libc::c_char,
        );
    }
    splitscreen = false_0 as libc::c_int;
    SplitScreen_OnChange();
    botingame = false_0 as libc::c_int;
    botskin = 0 as libc::c_int as uint8_t;
    CL_ConnectToServer();
}
#[no_mangle]
pub unsafe extern "C" fn CL_ClearPlayer(mut playernum: int32_t) {
    if !(players[playernum as usize].mo).is_null() {
        P_RemoveMobj(players[playernum as usize].mo);
    }
    memset(
        &mut *players.as_mut_ptr().offset(playernum as isize) as *mut player_t
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<player_t>() as libc::c_ulong,
    );
    memset(
        (playeraddress[playernum as usize]).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CL_RemovePlayer(
    mut playernum: int32_t,
    mut reason: kickreason_t,
) {
    if playeringame[playernum as usize] == 0 {
        return;
    }
    if server != 0 && demoplayback == 0
        && playernode[playernum as usize] as libc::c_int != 255 as libc::c_int
    {
        let mut node: int32_t = playernode[playernum as usize] as int32_t;
        playerpernode[node as usize] = (playerpernode[node as usize]).wrapping_sub(1);
        playerpernode[node as usize];
        if playerpernode[node as usize] as libc::c_int <= 0 as libc::c_int {
            nodeingame[node as usize] = false_0 as libc::c_int;
            Net_CloseConnection(node);
            ResetNode(node);
        }
    }
    if gametyperules & GTR_TEAMFLAGS as libc::c_int as uint32_t != 0 {
        P_PlayerFlagBurst(
            &mut *players.as_mut_ptr().offset(playernum as isize),
            false_0 as libc::c_int,
        );
    }
    if G_IsSpecialStage(gamemap as int32_t) != 0 {
        let mut i: int32_t = 0;
        let mut count: int32_t = 0;
        let mut sincrement: int32_t = 0;
        let mut spheres: int32_t = 0;
        let mut rincrement: int32_t = 0;
        let mut rings: int32_t = 0;
        i = 0 as libc::c_int;
        count = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0 {
                count += 1;
                count;
            }
            i += 1;
            i;
        }
        count -= 1;
        count;
        spheres = players[playernum as usize].spheres as int32_t;
        sincrement = spheres;
        rings = players[playernum as usize].rings as int32_t;
        rincrement = rings;
        if count != 0 {
            sincrement /= count;
            rincrement /= count;
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0 && i != playernum {
                if spheres < 2 as libc::c_int * sincrement {
                    P_GivePlayerSpheres(
                        &mut *players.as_mut_ptr().offset(i as isize),
                        spheres,
                    );
                    spheres = 0 as libc::c_int;
                } else {
                    P_GivePlayerSpheres(
                        &mut *players.as_mut_ptr().offset(i as isize),
                        sincrement,
                    );
                    spheres -= sincrement;
                }
                if rings < 2 as libc::c_int * rincrement {
                    P_GivePlayerRings(
                        &mut *players.as_mut_ptr().offset(i as isize),
                        rings,
                    );
                    rings = 0 as libc::c_int;
                } else {
                    P_GivePlayerRings(
                        &mut *players.as_mut_ptr().offset(i as isize),
                        rincrement,
                    );
                    rings -= rincrement;
                }
            }
            i += 1;
            i;
        }
    }
    LUA_HookPlayerQuit(&mut *players.as_mut_ptr().offset(playernum as isize), reason);
    if playernum == displayplayer {
        LUA_HookViewpointSwitch(
            &mut *players.as_mut_ptr().offset(consoleplayer as isize),
            &mut *players.as_mut_ptr().offset(consoleplayer as isize),
            true_0 as libc::c_int,
        );
        displayplayer = consoleplayer;
    }
    CL_ClearPlayer(playernum);
    playeringame[playernum as usize] = false_0 as libc::c_int;
    playernode[playernum as usize] = 255 as libc::c_int as uint8_t;
    while playeringame[((*doomcom).numslots as libc::c_int - 1 as libc::c_int) as usize]
        == 0 && (*doomcom).numslots as libc::c_int > 1 as libc::c_int
    {
        (*doomcom).numslots -= 1;
        (*doomcom).numslots;
    }
    sprintf(
        (player_names[playernum as usize]).as_mut_ptr(),
        b"Player %d\0" as *const u8 as *const libc::c_char,
        playernum + 1 as libc::c_int,
    );
    player_name_changes[playernum as usize] = 0 as libc::c_int;
    if IsPlayerAdmin(playernum) != 0 {
        RemoveAdminPlayer(playernum);
    }
    LUA_InvalidatePlayer(&mut *players.as_mut_ptr().offset(playernum as isize));
    if G_TagGametype() != 0 {
        P_CheckSurvivors();
    } else if gametyperules & GTR_RACE as libc::c_int as uint32_t != 0 {
        P_CheckRacers();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CL_Reset() {
    if metalrecording != 0 {
        G_StopMetalRecording(false_0 as libc::c_int);
    }
    if !metalplayback.is_null() {
        G_StopMetalDemo();
    }
    if demorecording != 0 {
        G_CheckDemoStatus();
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"\n-=-=-=-=-=-=-= Client reset =-=-=-=-=-=-=-\n\n\0" as *const u8
                    as *const libc::c_char,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    if servernode as libc::c_int > 0 as libc::c_int
        && (servernode as libc::c_int) < 127 as libc::c_int
    {
        nodeingame[servernode as uint8_t as usize] = false_0 as libc::c_int;
        Net_CloseConnection(servernode as int32_t);
    }
    D_CloseConnection();
    multiplayer = false_0 as libc::c_int;
    servernode = 0 as libc::c_int as int8_t;
    server = true_0 as libc::c_int;
    (*doomcom).numnodes = 1 as libc::c_int as int16_t;
    (*doomcom).numslots = 1 as libc::c_int as int16_t;
    SV_StopServer();
    SV_ResetServer();
    FreeFileNeeded();
    fileneedednum = 0 as libc::c_int;
    totalfilesrequestednum = 0 as libc::c_int;
    totalfilesrequestedsize = 0 as libc::c_int as uint32_t;
    firstconnectattempttime = 0 as libc::c_int as tic_t;
    serverisfull = false_0 as libc::c_int;
    connectiontimeout = cv_nettimeout.value as tic_t;
}
unsafe extern "C" fn Command_GetPlayerNum() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 {
            if serverplayer == i {
                CONS_Printf(
                    b"num:%2d  node:%2d  %s\n\0" as *const u8 as *const libc::c_char,
                    i,
                    playernode[i as usize] as libc::c_int,
                    (player_names[i as usize]).as_mut_ptr(),
                );
            } else {
                CONS_Printf(
                    b"\x82num:%2d  node:%2d  %s\n\0" as *const u8 as *const libc::c_char,
                    i,
                    playernode[i as usize] as libc::c_int,
                    (player_names[i as usize]).as_mut_ptr(),
                );
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nametonum(mut name: *const libc::c_char) -> int8_t {
    let mut playernum: int32_t = 0;
    let mut i: int32_t = 0;
    if strcmp(name, b"0\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int as int8_t;
    }
    playernum = atoi(name) as int8_t as int32_t;
    if playernum < 0 as libc::c_int || playernum >= 32 as libc::c_int {
        return -(1 as libc::c_int) as int8_t;
    }
    if playernum != 0 {
        if playeringame[playernum as usize] != 0 {
            return playernum as int8_t
        } else {
            return -(1 as libc::c_int) as int8_t
        }
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0
            && strcasecmp((player_names[i as usize]).as_mut_ptr(), name) == 0
        {
            return i as int8_t;
        }
        i += 1;
        i;
    }
    CONS_Printf(
        b"There is no player named \"%s\"\n\0" as *const u8 as *const libc::c_char,
        name,
    );
    return -(1 as libc::c_int) as int8_t;
}
unsafe extern "C" fn Command_Nodes() {
    let mut i: int32_t = 0;
    let mut maxlen: size_t = 0 as libc::c_int as size_t;
    let mut address: *const libc::c_char = 0 as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let plen: size_t = strlen((player_names[i as usize]).as_mut_ptr());
        if playeringame[i as usize] != 0 && plen > maxlen {
            maxlen = plen;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 {
            CONS_Printf(
                b"%.2u: %*s\0" as *const u8 as *const libc::c_char,
                i,
                maxlen as libc::c_int,
                (player_names[i as usize]).as_mut_ptr(),
            );
            if playernode[i as usize] as libc::c_int != 255 as libc::c_int {
                CONS_Printf(
                    b" - node %.2d\0" as *const u8 as *const libc::c_char,
                    playernode[i as usize] as libc::c_int,
                );
                if I_GetNodeAddress.is_some()
                    && {
                        address = I_GetNodeAddress
                            .expect(
                                "non-null function pointer",
                            )(playernode[i as usize] as int32_t);
                        !address.is_null()
                    }
                {
                    CONS_Printf(b" - %s\0" as *const u8 as *const libc::c_char, address);
                }
            }
            if IsPlayerAdmin(i) != 0 {
                CONS_Printf(b" (verified admin)\0" as *const u8 as *const libc::c_char);
            }
            if players[i as usize].spectator != 0 {
                CONS_Printf(b" (spectator)\0" as *const u8 as *const libc::c_char);
            }
            CONS_Printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn Command_Ban() {
    if COM_Argc() < 2 as libc::c_int as size_t {
        CONS_Printf(
            b"Ban <playername/playernum> <reason>: ban and kick a player\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if netgame == 0 {
        CONS_Printf(
            b"This only works in a netgame.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if server != 0 || IsPlayerAdmin(consoleplayer) != 0 {
        let mut buf: [uint8_t; 33] = [0; 33];
        let mut p: *mut uint8_t = buf.as_mut_ptr();
        let pn: int8_t = nametonum(COM_Argv(1 as libc::c_int as size_t));
        let node: int32_t = playernode[pn as int32_t as usize] as int32_t;
        if pn as libc::c_int == -(1 as libc::c_int)
            || pn as libc::c_int == 0 as libc::c_int
        {
            return;
        }
        let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
        let tv: uint8_t = pn as uint8_t;
        memcpy(
            p as *mut libc::c_void,
            &tv as *const uint8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        p = p_tmp as *mut libc::c_void as *mut uint8_t;
        if server != 0 && I_Ban.is_some()
            && I_Ban.expect("non-null function pointer")(node) == 0
        {
            CONS_Alert(
                CONS_WARNING,
                b"Too many bans! Geez, that's a lot of people you're excluding...\n\0"
                    as *const u8 as *const libc::c_char,
            );
            let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let tv_0: uint8_t = 1 as libc::c_int as uint8_t;
            memcpy(
                p as *mut libc::c_void,
                &tv_0 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_0 = p_tmp_0.offset(1);
            p_tmp_0;
            p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
            SendNetXCmd(
                XD_KICK,
                &mut buf as *mut [uint8_t; 33] as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
        } else {
            if server != 0 {
                Ban_Add(COM_Argv(2 as libc::c_int as size_t));
                D_SaveBan();
            }
            if COM_Argc() == 2 as libc::c_int as size_t {
                let mut p_tmp_1: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
                let tv_1: uint8_t = 5 as libc::c_int as uint8_t;
                memcpy(
                    p as *mut libc::c_void,
                    &tv_1 as *const uint8_t as *const libc::c_void,
                    ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                );
                p_tmp_1 = p_tmp_1.offset(1);
                p_tmp_1;
                p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
                SendNetXCmd(
                    XD_KICK,
                    &mut buf as *mut [uint8_t; 33] as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            } else {
                let mut i: size_t = 0;
                let mut j: size_t = COM_Argc();
                let mut message: [libc::c_char; 30] = [0; 30];
                strlcpy(
                    message.as_mut_ptr(),
                    COM_Argv(2 as libc::c_int as size_t),
                    ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                );
                i = 3 as libc::c_int as size_t;
                while i < j {
                    strlcat(
                        message.as_mut_ptr(),
                        b" \0" as *const u8 as *const libc::c_char,
                        ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                    );
                    strlcat(
                        message.as_mut_ptr(),
                        COM_Argv(i),
                        ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                    );
                    i = i.wrapping_add(1);
                    i;
                }
                let mut p_tmp_2: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
                let tv_2: uint8_t = 8 as libc::c_int as uint8_t;
                memcpy(
                    p as *mut libc::c_void,
                    &tv_2 as *const uint8_t as *const libc::c_void,
                    ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                );
                p_tmp_2 = p_tmp_2.offset(1);
                p_tmp_2;
                p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
                let mut tmp_i: size_t = 0;
                tmp_i = 0 as libc::c_int as size_t;
                while tmp_i < 30 as libc::c_int as size_t
                    && message[tmp_i as usize] as libc::c_int != '\0' as i32
                {
                    let mut p_tmp_3: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_3: libc::c_char = message[tmp_i as usize];
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_3 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_3 = p_tmp_3.offset(1);
                    p_tmp_3;
                    p = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
                    tmp_i = tmp_i.wrapping_add(1);
                    tmp_i;
                }
                if tmp_i < 30 as libc::c_int as size_t {
                    let mut p_tmp_4: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_4: libc::c_char = '\0' as i32 as libc::c_char;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_4 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_4 = p_tmp_4.offset(1);
                    p_tmp_4;
                    p = p_tmp_4 as *mut libc::c_void as *mut uint8_t;
                }
                SendNetXCmd(
                    XD_KICK,
                    &mut buf as *mut [uint8_t; 33] as *const libc::c_void,
                    p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
                );
            }
        }
    } else {
        CONS_Printf(
            b"Only the server or a remote admin can use this.\n\0" as *const u8
                as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn Command_BanIP() {
    if COM_Argc() < 2 as libc::c_int as size_t {
        CONS_Printf(
            b"banip <ip> <reason>: ban an ip address\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if server != 0 {
        let mut address: *const libc::c_char = COM_Argv(1 as libc::c_int as size_t);
        let mut reason: *const libc::c_char = 0 as *const libc::c_char;
        if COM_Argc() == 2 as libc::c_int as size_t {
            reason = 0 as *const libc::c_char;
        } else {
            reason = COM_Argv(2 as libc::c_int as size_t);
        }
        if I_SetBanAddress.is_some()
            && I_SetBanAddress
                .expect("non-null function pointer")(address, 0 as *const libc::c_char)
                != 0
        {
            if !reason.is_null() {
                CONS_Printf(
                    b"Banned IP address %s for: %s\n\0" as *const u8
                        as *const libc::c_char,
                    address,
                    reason,
                );
            } else {
                CONS_Printf(
                    b"Banned IP address %s\n\0" as *const u8 as *const libc::c_char,
                    address,
                );
            }
            Ban_Add(reason);
            D_SaveBan();
        } else {
            return
        }
    }
}
unsafe extern "C" fn Command_Kick() {
    if COM_Argc() < 2 as libc::c_int as size_t {
        CONS_Printf(
            b"kick <playername/playernum> <reason>: kick a player\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if netgame == 0 {
        CONS_Printf(
            b"This only works in a netgame.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if server != 0 || IsPlayerAdmin(consoleplayer) != 0 {
        let mut buf: [uint8_t; 33] = [0; 33];
        let mut p: *mut uint8_t = buf.as_mut_ptr();
        let pn: int8_t = nametonum(COM_Argv(1 as libc::c_int as size_t));
        if pn as libc::c_int == -(1 as libc::c_int)
            || pn as libc::c_int == 0 as libc::c_int
        {
            return;
        }
        if server != 0 && playernode[pn as usize] as libc::c_int != 255 as libc::c_int
            && sendingsavegame[playernode[pn as usize] as usize] != 0
        {
            Net_ConnectionTimeout(playernode[pn as usize] as int32_t);
            return;
        }
        let mut p_tmp: *mut int8_t = p as *mut libc::c_void as *mut int8_t;
        let tv: int8_t = pn as uint8_t as int8_t;
        memcpy(
            p as *mut libc::c_void,
            &tv as *const int8_t as *const libc::c_void,
            ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
        );
        p_tmp = p_tmp.offset(1);
        p_tmp;
        p = p_tmp as *mut libc::c_void as *mut uint8_t;
        if COM_Argc() == 2 as libc::c_int as size_t {
            let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let tv_0: uint8_t = 1 as libc::c_int as uint8_t;
            memcpy(
                p as *mut libc::c_void,
                &tv_0 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_0 = p_tmp_0.offset(1);
            p_tmp_0;
            p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
            SendNetXCmd(
                XD_KICK,
                &mut buf as *mut [uint8_t; 33] as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
        } else {
            let mut i: size_t = 0;
            let mut j: size_t = COM_Argc();
            let mut message: [libc::c_char; 30] = [0; 30];
            strlcpy(
                message.as_mut_ptr(),
                COM_Argv(2 as libc::c_int as size_t),
                ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
            );
            i = 3 as libc::c_int as size_t;
            while i < j {
                strlcat(
                    message.as_mut_ptr(),
                    b" \0" as *const u8 as *const libc::c_char,
                    ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                );
                strlcat(
                    message.as_mut_ptr(),
                    COM_Argv(i),
                    ::core::mem::size_of::<[libc::c_char; 30]>() as libc::c_ulong,
                );
                i = i.wrapping_add(1);
                i;
            }
            let mut p_tmp_1: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let tv_1: uint8_t = 7 as libc::c_int as uint8_t;
            memcpy(
                p as *mut libc::c_void,
                &tv_1 as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp_1 = p_tmp_1.offset(1);
            p_tmp_1;
            p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
            let mut tmp_i: size_t = 0;
            tmp_i = 0 as libc::c_int as size_t;
            while tmp_i < 30 as libc::c_int as size_t
                && message[tmp_i as usize] as libc::c_int != '\0' as i32
            {
                let mut p_tmp_2: *mut libc::c_char = p as *mut libc::c_void
                    as *mut libc::c_char;
                let tv_2: libc::c_char = message[tmp_i as usize];
                memcpy(
                    p as *mut libc::c_void,
                    &tv_2 as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
                p_tmp_2 = p_tmp_2.offset(1);
                p_tmp_2;
                p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
                tmp_i = tmp_i.wrapping_add(1);
                tmp_i;
            }
            if tmp_i < 30 as libc::c_int as size_t {
                let mut p_tmp_3: *mut libc::c_char = p as *mut libc::c_void
                    as *mut libc::c_char;
                let tv_3: libc::c_char = '\0' as i32 as libc::c_char;
                memcpy(
                    p as *mut libc::c_void,
                    &tv_3 as *const libc::c_char as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                );
                p_tmp_3 = p_tmp_3.offset(1);
                p_tmp_3;
                p = p_tmp_3 as *mut libc::c_void as *mut uint8_t;
            }
            SendNetXCmd(
                XD_KICK,
                &mut buf as *mut [uint8_t; 33] as *const libc::c_void,
                p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
            );
        }
    } else {
        CONS_Printf(
            b"Only the server or a remote admin can use this.\n\0" as *const u8
                as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn Command_ResendGamestate() {
    let mut playernum: int8_t = 0;
    if COM_Argc() == 1 as libc::c_int as size_t {
        CONS_Printf(
            b"resendgamestate <playername/playernum>: resend the game state to a player\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    } else if server == 0 {
        CONS_Printf(
            b"Only the server can use this.\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    playernum = nametonum(COM_Argv(1 as libc::c_int as size_t));
    if playernum as libc::c_int == -(1 as libc::c_int)
        || playernum as libc::c_int == 0 as libc::c_int
    {
        return;
    }
    (*netbuffer).packettype = PT_WILLRESENDGAMESTATE as libc::c_int as uint8_t;
    if HSendPacket(
        playernode[playernum as usize] as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as size_t,
    ) == 0
    {
        CONS_Alert(
            CONS_ERROR,
            b"A problem occured, please try again.\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
}
unsafe extern "C" fn Got_KickCmd(mut p: *mut *mut uint8_t, mut playernum: int32_t) {
    let mut pnum: int32_t = 0;
    let mut msg: int32_t = 0;
    let mut buf: [libc::c_char; 33] = [0; 33];
    let mut reason: *mut libc::c_char = buf.as_mut_ptr();
    let mut kickreason: kickreason_t = KR_KICK;
    let mut keepbody: boolean = 0;
    pnum = ({
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
    }) as int32_t;
    msg = ({
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
    }) as int32_t;
    keepbody = (msg & 0x80 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    msg &= !(0x80 as libc::c_int);
    if pnum == serverplayer && IsPlayerAdmin(playernum) != 0 {
        CONS_Printf(
            b"Server is being shut down remotely. Goodbye!\n\0" as *const u8
                as *const libc::c_char,
        );
        if server != 0 {
            COM_BufAddTextEx(
                b"quit\n\0" as *const u8 as *const libc::c_char,
                0 as com_flags_t,
            );
        }
        return;
    }
    if playernum != serverplayer && IsPlayerAdmin(playernum) == 0
        && !(playernode[playernum as usize] as libc::c_int != 255 as libc::c_int
            && playerpernode[playernode[playernum as usize] as usize] as libc::c_int
                == 2 as libc::c_int
            && nodetoplayer2[playernode[playernum as usize] as usize] as libc::c_int
                == pnum)
    {
        CONS_Alert(
            CONS_WARNING,
            b"Illegal kick command received from %s for player %d\n\0" as *const u8
                as *const libc::c_char,
            (player_names[playernum as usize]).as_mut_ptr(),
            pnum,
        );
        pnum = playernum;
        msg = 2 as libc::c_int;
        keepbody = true_0 as libc::c_int;
    }
    if server != 0 && playernum != 0
        && (msg == 5 as libc::c_int || msg == 8 as libc::c_int)
    {
        if I_Ban.is_some()
            && I_Ban
                .expect(
                    "non-null function pointer",
                )(playernode[pnum as usize] as int32_t) == 0
        {
            CONS_Alert(
                CONS_WARNING,
                b"Too many bans! Geez, that's a lot of people you're excluding...\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            Ban_Add(reason);
        }
    }
    match msg {
        1 => {
            if players[pnum as usize].quittime == 0 {
                HU_AddChatText(
                    va(
                        b"\x82*%s has been kicked (No reason given)\0" as *const u8
                            as *const libc::c_char,
                        (player_names[pnum as usize]).as_mut_ptr(),
                    ),
                    false_0 as libc::c_int,
                );
            }
            kickreason = KR_KICK;
        }
        6 => {
            HU_AddChatText(
                va(
                    b"\x82*%s left the game (Broke ping limit)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_PINGLIMIT;
        }
        2 => {
            HU_AddChatText(
                va(
                    b"\x82*%s left the game (Synch failure)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_SYNCH;
            if M_CheckParm(b"-consisdump\0" as *const u8 as *const libc::c_char) != 0 {
                let mut i: int32_t = 0;
                CONS_Printf(
                    b"Player kicked is #%d, dumping consistency...\n\0" as *const u8
                        as *const libc::c_char,
                    pnum,
                );
                i = 0 as libc::c_int;
                while i < 32 as libc::c_int {
                    if !(playeringame[i as usize] == 0) {
                        CONS_Printf(
                            b"-------------------------------------\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        CONS_Printf(
                            b"Player %d: %s\n\0" as *const u8 as *const libc::c_char,
                            i,
                            (player_names[i as usize]).as_mut_ptr(),
                        );
                        CONS_Printf(
                            b"Skin: %d\n\0" as *const u8 as *const libc::c_char,
                            players[i as usize].skin,
                        );
                        CONS_Printf(
                            b"Color: %d\n\0" as *const u8 as *const libc::c_char,
                            players[i as usize].skincolor as libc::c_int,
                        );
                        CONS_Printf(
                            b"Speed: %d\n\0" as *const u8 as *const libc::c_char,
                            players[i as usize].speed >> 16 as libc::c_int,
                        );
                        if !(players[i as usize].mo).is_null() {
                            if ((*players[i as usize].mo).skin).is_null() {
                                CONS_Printf(
                                    b"Mobj skin: NULL!\n\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                CONS_Printf(
                                    b"Mobj skin: %s\n\0" as *const u8 as *const libc::c_char,
                                    ((*((*players[i as usize].mo).skin as *mut skin_t)).name)
                                        .as_mut_ptr(),
                                );
                            }
                            CONS_Printf(
                                b"Position: %d, %d, %d\n\0" as *const u8
                                    as *const libc::c_char,
                                (*players[i as usize].mo).x,
                                (*players[i as usize].mo).y,
                                (*players[i as usize].mo).z,
                            );
                            if ((*players[i as usize].mo).state).is_null() {
                                CONS_Printf(
                                    b"State: S_NULL\n\0" as *const u8 as *const libc::c_char,
                                );
                            } else {
                                CONS_Printf(
                                    b"State: %d\n\0" as *const u8 as *const libc::c_char,
                                    ((*players[i as usize].mo).state)
                                        .offset_from(states.as_mut_ptr()) as libc::c_long
                                        as statenum_t as libc::c_uint,
                                );
                            }
                        } else {
                            CONS_Printf(
                                b"Mobj: NULL\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        CONS_Printf(
                            b"-------------------------------------\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    i += 1;
                    i;
                }
            }
        }
        4 => {
            HU_AddChatText(
                va(
                    b"\x82*%s left the game (Connection timeout)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_TIMEOUT;
        }
        3 => {
            if netgame != 0 && players[pnum as usize].quittime == 0 {
                HU_AddChatText(
                    va(
                        b"\x82*%s left the game\0" as *const u8 as *const libc::c_char,
                        (player_names[pnum as usize]).as_mut_ptr(),
                    ),
                    false_0 as libc::c_int,
                );
            }
            kickreason = KR_LEAVE;
        }
        5 => {
            HU_AddChatText(
                va(
                    b"\x82*%s has been banned (No reason given)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_BAN;
        }
        7 => {
            let mut tmp_i: size_t = 0 as libc::c_int as size_t;
            while tmp_i < (30 as libc::c_int + 1 as libc::c_int) as size_t
                && {
                    let ref mut fresh1 = *reason.offset(tmp_i as isize);
                    *fresh1 = ({
                        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void
                            as *mut libc::c_char;
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
                    });
                    *fresh1 as libc::c_int != '\0' as i32
                }
            {
                tmp_i = tmp_i.wrapping_add(1);
                tmp_i;
            }
            *reason.offset(tmp_i as isize) = '\0' as i32 as libc::c_char;
            HU_AddChatText(
                va(
                    b"\x82*%s has been kicked (%s)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                    reason,
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_KICK;
        }
        8 => {
            let mut tmp_i_0: size_t = 0 as libc::c_int as size_t;
            while tmp_i_0 < (30 as libc::c_int + 1 as libc::c_int) as size_t
                && {
                    let ref mut fresh2 = *reason.offset(tmp_i_0 as isize);
                    *fresh2 = ({
                        let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void
                            as *mut libc::c_char;
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
                    });
                    *fresh2 as libc::c_int != '\0' as i32
                }
            {
                tmp_i_0 = tmp_i_0.wrapping_add(1);
                tmp_i_0;
            }
            *reason.offset(tmp_i_0 as isize) = '\0' as i32 as libc::c_char;
            HU_AddChatText(
                va(
                    b"\x82*%s has been banned (%s)\0" as *const u8
                        as *const libc::c_char,
                    (player_names[pnum as usize]).as_mut_ptr(),
                    reason,
                ),
                false_0 as libc::c_int,
            );
            kickreason = KR_BAN;
        }
        _ => {}
    }
    if pnum == consoleplayer {
        LUA_HookBool(false_0 as libc::c_int, hook_GameQuit as libc::c_int);
        D_QuitNetGame();
        CL_Reset();
        D_StartTitle();
        if msg == 2 as libc::c_int {
            M_StartMessage(
                b"Server closed connection\n(synch failure)\nPress ESC\n\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        } else if msg == 6 as libc::c_int {
            M_StartMessage(
                b"Server closed connection\n(Broke ping limit)\nPress ESC\n\0"
                    as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        } else if msg == 5 as libc::c_int {
            M_StartMessage(
                b"You have been banned by the server\n\nPress ESC\n\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        } else if msg == 7 as libc::c_int {
            M_StartMessage(
                va(
                    b"You have been kicked\n(%s)\nPress ESC\n\0" as *const u8
                        as *const libc::c_char,
                    reason,
                ),
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        } else if msg == 8 as libc::c_int {
            M_StartMessage(
                va(
                    b"You have been banned\n(%s)\nPress ESC\n\0" as *const u8
                        as *const libc::c_char,
                    reason,
                ),
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        } else {
            M_StartMessage(
                b"You have been kicked by the server\n\nPress ESC\n\0" as *const u8
                    as *const libc::c_char,
                0 as *mut libc::c_void,
                MM_NOTHING,
            );
        }
    } else if keepbody != 0 {
        if server != 0 && demoplayback == 0
            && playernode[pnum as usize] as libc::c_int != 255 as libc::c_int
        {
            let mut node: int32_t = playernode[pnum as usize] as int32_t;
            playerpernode[node
                as usize] = (playerpernode[node as usize]).wrapping_sub(1);
            playerpernode[node as usize];
            if playerpernode[node as usize] as libc::c_int <= 0 as libc::c_int {
                nodeingame[node as usize] = false_0 as libc::c_int;
                Net_CloseConnection(node);
                ResetNode(node);
            }
        }
        playernode[pnum as usize] = 255 as libc::c_int as uint8_t;
        players[pnum as usize].quittime = 1 as libc::c_int as tic_t;
    } else {
        CL_RemovePlayer(pnum, kickreason);
    };
}
static mut netticbuffer_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 3 as libc::c_int,
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
pub static mut cv_netticbuffer: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"netticbuffer\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"1\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: netticbuffer_cons_t.as_ptr() as *mut _,
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
pub static mut cv_allownewplayer: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"allowjoin\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"On\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int
                | CV_ALLOWLUA as libc::c_int,
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
pub static mut cv_joinnextround: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"joinnextround\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Off\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
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
static mut maxplayers_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 32 as libc::c_int,
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
pub static mut cv_maxplayers: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"maxplayers\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"8\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int
                | CV_ALLOWLUA as libc::c_int,
            PossibleValue: maxplayers_cons_t.as_ptr() as *mut _,
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
static mut joindelay_cons_t: [CV_PossibleValue_t; 4] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 3600 as libc::c_int,
            strvalue: b"MAX\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Off\0" as *const u8 as *const libc::c_char,
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
pub static mut cv_joindelay: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"joindelay\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"10\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
            PossibleValue: joindelay_cons_t.as_ptr() as *mut _,
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
static mut rejointimeout_cons_t: [CV_PossibleValue_t; 4] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 60 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
            strvalue: b"MAX\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"Off\0" as *const u8 as *const libc::c_char,
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
pub static mut cv_rejointimeout: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"rejointimeout\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"2\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int
                | CV_FLOAT as libc::c_int,
            PossibleValue: rejointimeout_cons_t.as_ptr() as *mut _,
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
static mut resynchattempts_cons_t: [CV_PossibleValue_t; 4] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
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
            strvalue: b"No\0" as *const u8 as *const libc::c_char,
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
pub static mut cv_resynchattempts: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"resynchattempts\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"10\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
            PossibleValue: resynchattempts_cons_t.as_ptr() as *mut _,
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
pub static mut cv_blamecfail: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"blamecfail\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Off\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
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
static mut maxsend_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 0 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 204800 as libc::c_int,
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
pub static mut cv_maxsend: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"maxsend\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"4096\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
            PossibleValue: maxsend_cons_t.as_ptr() as *mut _,
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
pub static mut cv_noticedownload: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"noticedownload\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Off\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
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
static mut downloadspeed_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 300 as libc::c_int,
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
pub static mut cv_downloadspeed: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"downloadspeed\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"16\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int,
            PossibleValue: downloadspeed_cons_t.as_ptr() as *mut _,
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
pub unsafe extern "C" fn D_ClientServerInit() {
    if !debugfile.is_null() {
        fputs(
            va(
                b"- - -== SRB2 v%d.%.2d.%d v2.2.13 debugfile ==- - -\n\0" as *const u8
                    as *const libc::c_char,
                VERSION / 100 as libc::c_int,
                VERSION % 100 as libc::c_int,
                SUBVERSION,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    COM_AddCommand(
        b"getplayernum\0" as *const u8 as *const libc::c_char,
        Some(Command_GetPlayerNum as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"kick\0" as *const u8 as *const libc::c_char,
        Some(Command_Kick as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"ban\0" as *const u8 as *const libc::c_char,
        Some(Command_Ban as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"banip\0" as *const u8 as *const libc::c_char,
        Some(Command_BanIP as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"clearbans\0" as *const u8 as *const libc::c_char,
        Some(Command_ClearBans as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"showbanlist\0" as *const u8 as *const libc::c_char,
        Some(Command_ShowBan as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"reloadbans\0" as *const u8 as *const libc::c_char,
        Some(Command_ReloadBan as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"connect\0" as *const u8 as *const libc::c_char,
        Some(Command_connect as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"nodes\0" as *const u8 as *const libc::c_char,
        Some(Command_Nodes as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    COM_AddCommand(
        b"resendgamestate\0" as *const u8 as *const libc::c_char,
        Some(Command_ResendGamestate as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
    RegisterNetXCmd(
        XD_KICK,
        Some(Got_KickCmd as unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> ()),
    );
    RegisterNetXCmd(
        XD_ADDPLAYER,
        Some(Got_AddPlayer as unsafe extern "C" fn(*mut *mut uint8_t, int32_t) -> ()),
    );
    Ban_Load_File(false_0 as libc::c_int);
    gametic = 0 as libc::c_int as tic_t;
    leveltime = 0 as libc::c_int as tic_t;
    SV_StopServer();
    SV_ResetServer();
    if dedicated != 0 {
        SV_SpawnServer();
    }
}
unsafe extern "C" fn ResetNode(mut node: int32_t) {
    nodeingame[node as usize] = false_0 as libc::c_int;
    nodewaiting[node as usize] = 0 as libc::c_int as uint8_t;
    nettics[node as usize] = gametic;
    supposedtics[node as usize] = gametic;
    nodetoplayer[node as usize] = -(1 as libc::c_int) as int8_t;
    nodetoplayer2[node as usize] = -(1 as libc::c_int) as int8_t;
    playerpernode[node as usize] = 0 as libc::c_int as uint8_t;
    sendingsavegame[node as usize] = false_0 as libc::c_int;
    resendingsavegame[node as usize] = false_0 as libc::c_int;
    savegameresendcooldown[node as usize] = 0 as libc::c_int as tic_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ResetServer() {
    let mut i: int32_t = 0;
    maketic = gametic.wrapping_add(1 as libc::c_int as tic_t);
    neededtic = maketic;
    tictoclear = maketic;
    joindelay = 0 as libc::c_int as tic_t;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        ResetNode(i);
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        LUA_InvalidatePlayer(&mut *players.as_mut_ptr().offset(i as isize));
        playeringame[i as usize] = false_0 as libc::c_int;
        playernode[i as usize] = 255 as libc::c_int as uint8_t;
        memset(
            (playeraddress[i as usize]).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
        sprintf(
            (player_names[i as usize]).as_mut_ptr(),
            b"Player %d\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        adminplayers[i as usize] = -(1 as libc::c_int);
        i += 1;
        i;
    }
    memset(
        player_name_changes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[int32_t; 32]>() as libc::c_ulong,
    );
    mynode = 0 as libc::c_int as uint8_t;
    cl_packetmissed = false_0 as libc::c_int;
    cl_redownloadinggamestate = false_0 as libc::c_int;
    if dedicated != 0 {
        nodeingame[0 as libc::c_int as usize] = true_0 as libc::c_int;
        serverplayer = 0 as libc::c_int;
    } else {
        serverplayer = consoleplayer;
    }
    if server != 0 {
        servernode = 0 as libc::c_int as int8_t;
    }
    (*doomcom).numslots = 0 as libc::c_int as int16_t;
    memset(
        server_context.as_mut_ptr() as *mut libc::c_void,
        '-' as i32,
        8 as libc::c_int as libc::c_ulong,
    );
    CV_RevertNetVars();
    M_CopyGameData(serverGamedata, clientGamedata);
    if !debugfile.is_null() {
        fputs(
            b"\n-=-=-=-=-=-=-= Server Reset =-=-=-=-=-=-=-\n\n\0" as *const u8
                as *const libc::c_char,
            debugfile,
        );
        fflush(debugfile);
    }
}
#[inline]
unsafe extern "C" fn SV_GenContext() {
    let mut i: uint8_t = 0;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 8 as libc::c_int {
        let a: libc::c_char = M_RandomKey(26 as libc::c_int * 2 as libc::c_int)
            as libc::c_char;
        if (a as libc::c_int) < 26 as libc::c_int {
            server_context[i as usize] = ('A' as i32 + a as libc::c_int) as libc::c_char;
        } else {
            server_context[i
                as usize] = ('a' as i32 + (a as libc::c_int - 26 as libc::c_int))
                as libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_QuitNetGame() {
    mousegrabbedbylua = true_0 as libc::c_int;
    I_UpdateMouseGrab();
    if netgame == 0 || netbuffer.is_null() {
        return;
    }
    if !debugfile.is_null() {
        fputs(
            b"===========================================================================\n                  Quitting Game, closing connection\n===========================================================================\n\0"
                as *const u8 as *const libc::c_char,
            debugfile,
        );
        fflush(debugfile);
    }
    CloseNetFile();
    RemoveAllLuaFileTransfers();
    waitingforluafiletransfer = false_0 as libc::c_int;
    waitingforluafilecommand = false_0 as libc::c_int;
    if server != 0 {
        let mut i: int32_t = 0;
        (*netbuffer).packettype = PT_SERVERSHUTDOWN as libc::c_int as uint8_t;
        i = 0 as libc::c_int;
        while i < 127 as libc::c_int {
            if nodeingame[i as usize] != 0 {
                HSendPacket(
                    i,
                    true_0 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                    0 as libc::c_int as size_t,
                );
            }
            i += 1;
            i;
        }
        if serverrunning != 0 && ms_RoomId as libc::c_int > 0 as libc::c_int {
            UnregisterServer();
        }
    } else if servernode as libc::c_int > 0 as libc::c_int
        && (servernode as libc::c_int) < 127 as libc::c_int
        && nodeingame[servernode as uint8_t as usize] != 0
    {
        (*netbuffer).packettype = PT_CLIENTQUIT as libc::c_int as uint8_t;
        HSendPacket(
            servernode as int32_t,
            true_0 as libc::c_int,
            0 as libc::c_int as uint8_t,
            0 as libc::c_int as size_t,
        );
    }
    D_CloseConnection();
    ClearAdminPlayers();
    if !debugfile.is_null() {
        fputs(
            b"===========================================================================\n                         Log finish\n===========================================================================\n\0"
                as *const u8 as *const libc::c_char,
            debugfile,
        );
        fflush(debugfile);
    }
    if !debugfile.is_null() {
        fclose(debugfile);
        debugfile = 0 as *mut FILE;
    }
}
#[inline]
unsafe extern "C" fn SV_AddNode(mut node: int32_t) {
    nettics[node as usize] = gametic;
    supposedtics[node as usize] = gametic;
    if node != 0 {
        nodeingame[node as usize] = true_0 as libc::c_int;
    }
}
unsafe extern "C" fn Got_AddPlayer(mut p: *mut *mut uint8_t, mut playernum: int32_t) {
    let mut node: int16_t = 0;
    let mut newplayernum: int16_t = 0;
    let mut splitscreenplayer: boolean = 0;
    let mut rejoined: boolean = 0;
    let mut newplayer: *mut player_t = 0 as *mut player_t;
    if playernum != serverplayer && IsPlayerAdmin(playernum) == 0 {
        CONS_Alert(
            CONS_WARNING,
            b"Illegal add player command received from %s\n\0" as *const u8
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
    node = ({
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
    }) as int16_t;
    newplayernum = ({
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
    }) as int16_t;
    splitscreenplayer = newplayernum as libc::c_int & 0x80 as libc::c_int;
    newplayernum = (newplayernum as libc::c_int & !(0x80 as libc::c_int)) as int16_t;
    rejoined = playeringame[newplayernum as usize];
    if rejoined == 0 {
        if splitscreen == 0 && botingame == 0 {
            CL_ClearPlayer(newplayernum as int32_t);
        }
        playeringame[newplayernum as usize] = true_0 as libc::c_int;
        G_AddPlayer(newplayernum as int32_t);
        if newplayernum as libc::c_int + 1 as libc::c_int
            > (*doomcom).numslots as libc::c_int
        {
            (*doomcom)
                .numslots = (newplayernum as libc::c_int + 1 as libc::c_int) as int16_t;
        }
        if server != 0 && I_GetNodeAddress.is_some() {
            let mut addressbuffer: [libc::c_char; 64] = [0; 64];
            let mut address: *const libc::c_char = I_GetNodeAddress
                .expect("non-null function pointer")(node as int32_t);
            if !address.is_null() {
                strcpy(addressbuffer.as_mut_ptr(), address);
                strcpy(
                    (playeraddress[newplayernum as usize]).as_mut_ptr(),
                    I_NetSplitAddress(
                        addressbuffer.as_mut_ptr(),
                        0 as *mut *mut libc::c_char,
                    ),
                );
            }
        }
    }
    newplayer = &mut *players.as_mut_ptr().offset(newplayernum as isize)
        as *mut player_t;
    (*newplayer).jointime = 0 as libc::c_int as tic_t;
    (*newplayer).quittime = 0 as libc::c_int as tic_t;
    let mut tmp_i: size_t = 0 as libc::c_int as size_t;
    while tmp_i < 21 as libc::c_int as size_t
        && {
            player_names[newplayernum
                as usize][tmp_i
                as usize] = ({
                let mut p_tmp: *mut libc::c_char = *p as *mut libc::c_void
                    as *mut libc::c_char;
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
            });
            player_names[newplayernum as usize][tmp_i as usize] as libc::c_int
                != '\0' as i32
        }
    {
        tmp_i = tmp_i.wrapping_add(1);
        tmp_i;
    }
    player_names[newplayernum as usize][tmp_i as usize] = '\0' as i32 as libc::c_char;
    if node as libc::c_int == mynode as libc::c_int {
        playernode[newplayernum as usize] = 0 as libc::c_int as uint8_t;
        if splitscreenplayer == 0 {
            consoleplayer = newplayernum as int32_t;
            displayplayer = newplayernum as int32_t;
            secondarydisplayplayer = newplayernum as int32_t;
            if !debugfile.is_null() {
                fputs(b"spawning me\n\0" as *const u8 as *const libc::c_char, debugfile);
                fflush(debugfile);
            }
            ticcmd_oldangleturn[0 as libc::c_int
                as usize] = (*newplayer).oldrelangleturn;
        } else {
            secondarydisplayplayer = newplayernum as int32_t;
            if !debugfile.is_null() {
                fputs(
                    b"spawning my brother\n\0" as *const u8 as *const libc::c_char,
                    debugfile,
                );
                fflush(debugfile);
            }
            if botingame != 0 {
                (*newplayer).bot = 1 as libc::c_int as uint8_t;
            }
            ticcmd_oldangleturn[1 as libc::c_int
                as usize] = (*newplayer).oldrelangleturn;
        }
        P_ForceLocalAngle(
            newplayer,
            (((*newplayer).angleturn as libc::c_int) << 16 as libc::c_int) as angle_t,
        );
        D_SendPlayerConfig();
        addedtogame = true_0 as libc::c_int;
        if rejoined != 0 {
            if !((*newplayer).mo).is_null() {
                (*newplayer)
                    .viewheight = 41 as libc::c_int * (*newplayer).height
                    / 48 as libc::c_int;
                if (*(*newplayer).mo).eflags as libc::c_int
                    & MFE_VERTICALFLIP as libc::c_int != 0
                {
                    (*newplayer)
                        .viewz = (*(*newplayer).mo).z + (*(*newplayer).mo).height
                        - (*newplayer).viewheight;
                } else {
                    (*newplayer).viewz = (*(*newplayer).mo).z + (*newplayer).viewheight;
                }
            }
            ST_Start();
            HU_Start();
            if camera.chase != 0 && splitscreenplayer == 0 {
                P_ResetCamera(newplayer, &mut camera);
            }
            if camera2.chase != 0 && splitscreenplayer != 0 {
                P_ResetCamera(newplayer, &mut camera2);
            }
        }
    }
    if netgame != 0 {
        let mut joinmsg: [libc::c_char; 256] = [0; 256];
        if rejoined != 0 {
            strcpy(
                joinmsg.as_mut_ptr(),
                b"\x82*%s has rejoined the game (player %d)\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            strcpy(
                joinmsg.as_mut_ptr(),
                b"\x82*%s has joined the game (player %d)\0" as *const u8
                    as *const libc::c_char,
            );
        }
        strcpy(
            joinmsg.as_mut_ptr(),
            va(
                joinmsg.as_mut_ptr(),
                (player_names[newplayernum as usize]).as_mut_ptr(),
                newplayernum as libc::c_int,
            ),
        );
        if server != 0 && cv_showjoinaddress.value != 0 && I_GetNodeAddress.is_some() {
            let mut address_0: *const libc::c_char = I_GetNodeAddress
                .expect("non-null function pointer")(node as int32_t);
            if !address_0.is_null() {
                strcat(
                    joinmsg.as_mut_ptr(),
                    va(b" (%s)\0" as *const u8 as *const libc::c_char, address_0),
                );
            }
        }
        HU_AddChatText(joinmsg.as_mut_ptr(), false_0 as libc::c_int);
    }
    if server != 0 && multiplayer != 0
        && motd[0 as libc::c_int as usize] as libc::c_int != '\0' as i32
    {
        COM_BufAddTextEx(
            va(
                b"sayto %d %s\n\0" as *const u8 as *const libc::c_char,
                newplayernum as libc::c_int,
                motd.as_mut_ptr(),
            ),
            0 as com_flags_t,
        );
    }
    if rejoined == 0 {
        LUA_HookInt(newplayernum as int32_t, hook_PlayerJoin as libc::c_int);
    }
}
unsafe extern "C" fn SV_AddWaitingPlayers(
    mut name: *const libc::c_char,
    mut name2: *const libc::c_char,
) -> boolean {
    let mut node: int32_t = 0;
    let mut n: int32_t = 0;
    let mut newplayer: int32_t = false_0 as libc::c_int;
    let mut buf: [uint8_t; 23] = [0; 23];
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut newplayernum: int32_t = 0;
    node = 0 as libc::c_int;
    while node < 127 as libc::c_int {
        while nodewaiting[node as usize] as libc::c_int > 0 as libc::c_int {
            newplayer = true_0 as libc::c_int;
            newplayernum = FindRejoinerNum(node as int8_t);
            if newplayernum == -(1 as libc::c_int) {
                newplayernum = if dedicated != 0 {
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                };
                while newplayernum < 32 as libc::c_int {
                    if !(playeringame[newplayernum as usize] != 0) {
                        n = 0 as libc::c_int;
                        while n < 127 as libc::c_int {
                            if nodetoplayer[n as usize] as libc::c_int == newplayernum
                                || nodetoplayer2[n as usize] as libc::c_int == newplayernum
                            {
                                break;
                            }
                            n += 1;
                            n;
                        }
                        if n == 127 as libc::c_int {
                            break;
                        }
                    }
                    newplayernum += 1;
                    newplayernum;
                }
            }
            playernode[newplayernum as usize] = node as uint8_t;
            p = buf.as_mut_ptr().offset(2 as libc::c_int as isize);
            buf[0 as libc::c_int as usize] = node as uint8_t;
            buf[1 as libc::c_int as usize] = newplayernum as uint8_t;
            if (playerpernode[node as usize] as libc::c_int) < 1 as libc::c_int {
                nodetoplayer[node as usize] = newplayernum as int8_t;
                let mut tmp_i: size_t = 0;
                tmp_i = 0 as libc::c_int as size_t;
                while tmp_i < 21 as libc::c_int as size_t
                    && *name.offset(tmp_i as isize) as libc::c_int != '\0' as i32
                {
                    let mut p_tmp: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv: libc::c_char = *name.offset(tmp_i as isize);
                    memcpy(
                        p as *mut libc::c_void,
                        &tv as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp = p_tmp.offset(1);
                    p_tmp;
                    p = p_tmp as *mut libc::c_void as *mut uint8_t;
                    tmp_i = tmp_i.wrapping_add(1);
                    tmp_i;
                }
                if tmp_i < 21 as libc::c_int as size_t {
                    let mut p_tmp_0: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_0: libc::c_char = '\0' as i32 as libc::c_char;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_0 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_0 = p_tmp_0.offset(1);
                    p_tmp_0;
                    p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
                }
            } else {
                nodetoplayer2[node as usize] = newplayernum as int8_t;
                buf[1 as libc::c_int
                    as usize] = (buf[1 as libc::c_int as usize] as libc::c_int
                    | 0x80 as libc::c_int) as uint8_t;
                let mut tmp_i_0: size_t = 0;
                tmp_i_0 = 0 as libc::c_int as size_t;
                while tmp_i_0 < 21 as libc::c_int as size_t
                    && *name2.offset(tmp_i_0 as isize) as libc::c_int != '\0' as i32
                {
                    let mut p_tmp_1: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_1: libc::c_char = *name2.offset(tmp_i_0 as isize);
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_1 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_1 = p_tmp_1.offset(1);
                    p_tmp_1;
                    p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
                    tmp_i_0 = tmp_i_0.wrapping_add(1);
                    tmp_i_0;
                }
                if tmp_i_0 < 21 as libc::c_int as size_t {
                    let mut p_tmp_2: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let tv_2: libc::c_char = '\0' as i32 as libc::c_char;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_2 as *const libc::c_char as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp_2 = p_tmp_2.offset(1);
                    p_tmp_2;
                    p = p_tmp_2 as *mut libc::c_void as *mut uint8_t;
                }
            }
            playerpernode[node
                as usize] = (playerpernode[node as usize]).wrapping_add(1);
            playerpernode[node as usize];
            SendNetXCmd(
                XD_ADDPLAYER,
                &mut buf as *mut [uint8_t; 23] as *const libc::c_void,
                p.offset_from(buf.as_mut_ptr()) as libc::c_long as size_t,
            );
            if !debugfile.is_null() {
                fputs(
                    va(
                        b"Server added player %d node %d\n\0" as *const u8
                            as *const libc::c_char,
                        newplayernum,
                        node,
                    ),
                    debugfile,
                );
                fflush(debugfile);
            }
            nodewaiting[node as usize] = (nodewaiting[node as usize]).wrapping_sub(1);
            nodewaiting[node as usize];
        }
        node += 1;
        node;
    }
    return newplayer;
}
#[no_mangle]
pub unsafe extern "C" fn CL_AddSplitscreenPlayer() {
    if cl_mode as libc::c_uint == CL_CONNECTED as libc::c_int as libc::c_uint {
        CL_SendJoin();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CL_RemoveSplitscreenPlayer() {
    if cl_mode as libc::c_uint != CL_CONNECTED as libc::c_int as libc::c_uint {
        return;
    }
    SendKick(secondarydisplayplayer as uint8_t, 3 as libc::c_int as uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn Playing() -> boolean {
    return (server != 0 && serverrunning != 0
        || server == 0
            && cl_mode as libc::c_uint == CL_CONNECTED as libc::c_int as libc::c_uint)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SpawnServer() -> boolean {
    if demoplayback != 0 {
        G_StopDemo();
    }
    if !metalplayback.is_null() {
        G_StopMetalDemo();
    }
    if serverrunning == 0 {
        CONS_Printf(b"Starting Server....\n\0" as *const u8 as *const libc::c_char);
        serverrunning = true_0 as libc::c_int;
        SV_ResetServer();
        SV_GenContext();
        if netgame != 0 && I_NetOpenSocket.is_some() {
            I_NetOpenSocket.expect("non-null function pointer")();
            if ms_RoomId as libc::c_int > 0 as libc::c_int {
                RegisterServer();
            }
        }
        if dedicated == 0 {
            CL_ConnectToServer();
        } else {
            (*doomcom).numslots = 1 as libc::c_int as int16_t;
        }
    }
    return SV_AddWaitingPlayers(cv_playername.zstring, cv_playername2.zstring);
}
#[no_mangle]
pub unsafe extern "C" fn SV_StopServer() {
    let mut i: tic_t = 0;
    if gamestate as libc::c_uint == GS_INTERMISSION as libc::c_int as libc::c_uint {
        Y_EndIntermission();
    }
    wipegamestate = GS_NULL;
    gamestate = wipegamestate;
    localtextcmd[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    localtextcmd2[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
    i = firstticstosend;
    while i < firstticstosend.wrapping_add(1024 as libc::c_int as tic_t) {
        D_Clearticcmd(i);
        i = i.wrapping_add(1);
        i;
    }
    consoleplayer = 0 as libc::c_int;
    cl_mode = CL_SEARCHING;
    maketic = gametic.wrapping_add(1 as libc::c_int as tic_t);
    neededtic = maketic;
    serverrunning = false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_StartSinglePlayerServer() {
    server = true_0 as libc::c_int;
    netgame = false_0 as libc::c_int;
    multiplayer = false_0 as libc::c_int;
    G_SetGametype(GT_COOP as libc::c_int as int16_t);
    SV_StopServer();
    if splitscreen != 0 {
        multiplayer = true_0 as libc::c_int;
    }
}
unsafe extern "C" fn SV_SendRefuse(mut node: int32_t, mut reason: *const libc::c_char) {
    strcpy(((*netbuffer).u.serverrefuse.reason).as_mut_ptr(), reason);
    (*netbuffer).packettype = PT_SERVERREFUSE as libc::c_int as uint8_t;
    HSendPacket(
        node,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        (strlen(((*netbuffer).u.serverrefuse.reason).as_mut_ptr()))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    Net_CloseConnection(node);
}
unsafe extern "C" fn TotalTextCmdPerTic(mut tic: tic_t) -> size_t {
    let mut i: int32_t = 0;
    let mut total: size_t = 1 as libc::c_int as size_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut textcmd: *mut uint8_t = D_GetExistingTextcmd(tic, i);
        if (i == 0 || playeringame[i as usize] != 0) && !textcmd.is_null() {
            total = total
                .wrapping_add(
                    (2 as libc::c_int
                        + *textcmd.offset(0 as libc::c_int as isize) as libc::c_int)
                        as size_t,
                );
        }
        i += 1;
        i;
    }
    return total;
}
unsafe extern "C" fn ConnectionRefused(
    mut node: int8_t,
    mut rejoinernum: int32_t,
) -> *const libc::c_char {
    let mut cc: *mut clientconfig_pak = &mut (*netbuffer).u.clientcfg;
    let mut rejoining: boolean = (rejoinernum != -(1 as libc::c_int)) as libc::c_int;
    if node == 0 {
        return 0 as *const libc::c_char;
    }
    if (*cc).modversion as libc::c_int != 54 as libc::c_int
        || strncmp(
            ((*cc).application).as_mut_ptr(),
            b"SRB2\0" as *const u8 as *const libc::c_char,
            ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        ) != 0
    {
        return b"Incompatible.\0" as *const u8 as *const libc::c_char
    } else if !bannednode.is_null() && *bannednode.offset(node as isize) != 0 {
        return b"You have been banned\nfrom the server.\0" as *const u8
            as *const libc::c_char
    } else if (*cc).localplayers as libc::c_int != 1 as libc::c_int {
        return b"Wrong player count.\0" as *const u8 as *const libc::c_char
    }
    if rejoining == 0 {
        if cv_allownewplayer.value == 0 {
            return b"The server is not accepting\njoins for the moment.\0" as *const u8
                as *const libc::c_char
        } else if D_NumPlayers() >= cv_maxplayers.value {
            return va(
                b"Maximum players reached: %d\0" as *const u8 as *const libc::c_char,
                cv_maxplayers.value - D_NumBots(),
            )
        }
    }
    if !luafiletransfers.is_null() {
        return b"The serveris broadcasting a file\nrequested by a Lua script.\nPlease wait a bit and then\ntry rejoining.\0"
            as *const u8 as *const libc::c_char;
    }
    if netgame != 0 {
        let th: tic_t = (2 as libc::c_int * cv_joindelay.value * 35 as libc::c_int)
            as tic_t;
        if joindelay > th {
            return va(
                b"Too many people are connecting.\nPlease wait %d seconds and then\ntry rejoining.\0"
                    as *const u8 as *const libc::c_char,
                joindelay.wrapping_sub(th) / 35 as libc::c_int as tic_t,
            );
        }
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn HandleConnect(mut node: int8_t) {
    let mut names: [[libc::c_char; 22]; 2] = [[0; 22]; 2];
    let mut rejoinernum: int32_t = 0;
    let mut i: int32_t = 0;
    let mut refuse: *const libc::c_char = 0 as *const libc::c_char;
    rejoinernum = FindRejoinerNum(node);
    refuse = ConnectionRefused(node, rejoinernum);
    if !refuse.is_null() {
        SV_SendRefuse(node as int32_t, refuse);
    } else {
        let mut newnode: boolean = false_0 as libc::c_int;
        i = 0 as libc::c_int;
        while i
            < (*netbuffer).u.clientcfg.localplayers as libc::c_int
                - playerpernode[node as usize] as libc::c_int
        {
            strlcpy(
                (names[i as usize]).as_mut_ptr(),
                ((*netbuffer).u.clientcfg.names[i as usize]).as_mut_ptr(),
                (21 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            );
            if EnsurePlayerNameIsGood((names[i as usize]).as_mut_ptr(), rejoinernum) == 0
            {
                SV_SendRefuse(
                    node as int32_t,
                    b"Bad player name\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            i += 1;
            i;
        }
        nodewaiting[node
            as usize] = ((*netbuffer).u.clientcfg.localplayers as libc::c_int
            - playerpernode[node as usize] as libc::c_int) as uint8_t;
        if nodeingame[node as usize] == 0 {
            let mut backupstate: gamestate_t = gamestate;
            newnode = true_0 as libc::c_int;
            SV_AddNode(node as int32_t);
            if cv_joinnextround.value != 0
                && gameaction as libc::c_uint
                    == ga_nothing as libc::c_int as libc::c_uint
            {
                G_SetGamestate(GS_WAITINGPLAYERS);
            }
            if SV_SendServerConfig(node as int32_t) == 0 {
                G_SetGamestate(backupstate);
                ResetNode(node as int32_t);
                SV_SendRefuse(
                    node as int32_t,
                    b"Server couldn't send info, please try again\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            G_SetGamestate(backupstate);
            if !debugfile.is_null() {
                fputs(
                    b"new node joined\n\0" as *const u8 as *const libc::c_char,
                    debugfile,
                );
                fflush(debugfile);
            }
        }
        if nodewaiting[node as usize] != 0 {
            if (gamestate as libc::c_uint == GS_LEVEL as libc::c_int as libc::c_uint
                || gamestate as libc::c_uint
                    == GS_INTERMISSION as libc::c_int as libc::c_uint) && newnode != 0
            {
                SV_SendSaveGame(node as int32_t, false_0 as libc::c_int);
                if !debugfile.is_null() {
                    fputs(
                        b"send savegame\n\0" as *const u8 as *const libc::c_char,
                        debugfile,
                    );
                    fflush(debugfile);
                }
            }
            SV_AddWaitingPlayers(
                (names[0 as libc::c_int as usize]).as_mut_ptr(),
                (names[1 as libc::c_int as usize]).as_mut_ptr(),
            );
            joindelay = joindelay
                .wrapping_add((cv_joindelay.value * 35 as libc::c_int) as tic_t);
            player_joining = true_0 as libc::c_int as uint8_t;
        }
    };
}
unsafe extern "C" fn HandleShutdown(mut node: int8_t) {
    LUA_HookBool(false_0 as libc::c_int, hook_GameQuit as libc::c_int);
    D_QuitNetGame();
    CL_Reset();
    D_StartTitle();
    M_StartMessage(
        b"Server has shutdown\n\nPress Esc\n\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        MM_NOTHING,
    );
}
unsafe extern "C" fn HandleTimeout(mut node: int8_t) {
    LUA_HookBool(false_0 as libc::c_int, hook_GameQuit as libc::c_int);
    D_QuitNetGame();
    CL_Reset();
    D_StartTitle();
    M_StartMessage(
        b"Server Timeout\n\nPress Esc\n\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        MM_NOTHING,
    );
}
unsafe extern "C" fn HandleServerInfo(mut node: int8_t) {
    let ticnow: tic_t = I_GetTime();
    let ticthen: tic_t = (*netbuffer).u.serverinfo.time as int32_t as tic_t;
    let ticdiff: tic_t = ticnow.wrapping_sub(ticthen) * 1000 as libc::c_int as tic_t
        / (35 as libc::c_int * 1 as libc::c_int) as tic_t;
    (*netbuffer).u.serverinfo.time = ticdiff as int32_t as tic_t;
    (*netbuffer)
        .u
        .serverinfo
        .servername[(32 as libc::c_int - 1 as libc::c_int)
        as usize] = 0 as libc::c_int as libc::c_char;
    (*netbuffer)
        .u
        .serverinfo
        .application[(::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    (*netbuffer)
        .u
        .serverinfo
        .gametypename[(::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = '\0' as i32 as libc::c_char;
    SL_InsertServer(&mut (*netbuffer).u.serverinfo, node);
}
unsafe extern "C" fn PT_WillResendGamestate() {
    let mut tmpsave: [libc::c_char; 256] = [0; 256];
    if server != 0 || cl_redownloadinggamestate != 0 {
        return;
    }
    (*netbuffer).packettype = PT_CANRECEIVEGAMESTATE as libc::c_int as uint8_t;
    if HSendPacket(
        servernode as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as size_t,
    ) == 0
    {
        return;
    }
    CONS_Printf(b"Reloading game state...\n\0" as *const u8 as *const libc::c_char);
    sprintf(
        tmpsave.as_mut_ptr(),
        b"%s/$$$.sav\0" as *const u8 as *const libc::c_char,
        srb2home.as_mut_ptr(),
    );
    if FIL_FileExists(tmpsave.as_mut_ptr()) != 0
        && unlink(tmpsave.as_mut_ptr()) == -(1 as libc::c_int)
    {
        I_Error(
            b"Can't delete %s\n\0" as *const u8 as *const libc::c_char,
            tmpsave.as_mut_ptr(),
        );
    }
    CL_PrepareDownloadSaveGame(tmpsave.as_mut_ptr());
    cl_redownloadinggamestate = true_0 as libc::c_int;
}
unsafe extern "C" fn PT_CanReceiveGamestate(mut node: int8_t) {
    if server == 0 || sendingsavegame[node as usize] != 0 {
        return;
    }
    CONS_Printf(
        b"Resending game state to %s...\n\0" as *const u8 as *const libc::c_char,
        (player_names[nodetoplayer[node as usize] as usize]).as_mut_ptr(),
    );
    SV_SendSaveGame(node as int32_t, true_0 as libc::c_int);
    resendingsavegame[node as usize] = true_0 as libc::c_int;
}
unsafe extern "C" fn HandlePacketFromAwayNode(mut node: int8_t) {
    if node as libc::c_int != servernode as libc::c_int {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"Received packet from unknown host %d\n\0" as *const u8
                        as *const libc::c_char,
                    node as libc::c_int,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
    }
    let mut current_block_89: u64;
    match (*netbuffer).packettype as libc::c_int {
        16 => {
            Net_CloseConnection(node as int32_t);
            current_block_89 = 7301440000599063274;
        }
        32 => {
            if server != 0 && serverrunning != 0 {
                let mut p: *mut uint8_t = 0 as *mut uint8_t;
                let mut firstfile: int32_t = (*netbuffer).u.filesneedednum;
                (*netbuffer).packettype = PT_MOREFILESNEEDED as libc::c_int as uint8_t;
                (*netbuffer).u.filesneededcfg.first = firstfile;
                (*netbuffer).u.filesneededcfg.more = 0 as libc::c_int as uint8_t;
                p = PutFileNeeded(firstfile as uint16_t);
                HSendPacket(
                    node as int32_t,
                    false_0 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                    p
                        .offset_from(
                            &mut (*netbuffer).u as *mut C2RustUnnamed_7 as *mut uint8_t,
                        ) as libc::c_long as size_t,
                );
            } else {
                Net_CloseConnection(node as int32_t);
            }
            current_block_89 = 7301440000599063274;
        }
        33 => {
            if server != 0 && serverrunning != 0 {
                Net_CloseConnection(node as int32_t);
            } else if node as libc::c_int != servernode as libc::c_int {
                Net_CloseConnection(node as int32_t);
            } else if cl_mode as libc::c_uint
                == CL_ASKFULLFILELIST as libc::c_int as libc::c_uint
                && (*netbuffer).u.filesneededcfg.first == fileneedednum
            {
                D_ParseFileneeded(
                    (*netbuffer).u.filesneededcfg.num as int32_t,
                    ((*netbuffer).u.filesneededcfg.files).as_mut_ptr(),
                    (*netbuffer).u.filesneededcfg.first as uint16_t,
                );
                if (*netbuffer).u.filesneededcfg.more == 0 {
                    cl_lastcheckedfilecount = 65535 as libc::c_int as uint16_t;
                }
            }
            current_block_89 = 7301440000599063274;
        }
        12 => {
            if server != 0 && serverrunning != 0 {
                SV_SendServerInfo(
                    node as int32_t,
                    (*netbuffer).u.askinfo.time as int32_t as tic_t,
                );
                SV_SendPlayerInfo(node as int32_t);
            }
            Net_CloseConnection(node as int32_t);
            current_block_89 = 7301440000599063274;
        }
        9 => {
            if server != 0 && serverrunning != 0 {
                Net_CloseConnection(node as int32_t);
            } else if node as libc::c_int != servernode as libc::c_int {
                Net_CloseConnection(node as int32_t);
            } else if cl_mode as libc::c_uint
                == CL_WAITJOINRESPONSE as libc::c_int as libc::c_uint
            {
                let mut reason: *mut libc::c_char = strdup(
                    ((*netbuffer).u.serverrefuse.reason).as_mut_ptr(),
                );
                if reason.is_null() {
                    I_Error(b"Out of memory!\n\0" as *const u8 as *const libc::c_char);
                }
                if !(strstr(
                    reason,
                    b"Maximum players reached\0" as *const u8 as *const libc::c_char,
                ))
                    .is_null()
                {
                    serverisfull = true_0 as libc::c_int;
                    connectiontimeout = (35 as libc::c_int * 1 as libc::c_int
                        * 7 as libc::c_int) as tic_t;
                    cl_mode = CL_ASKJOIN;
                    free(reason as *mut libc::c_void);
                } else {
                    M_StartMessage(
                        va(
                            b"Server refuses connection\n\nReason:\n%s\0" as *const u8
                                as *const libc::c_char,
                            reason,
                        ),
                        0 as *mut libc::c_void,
                        MM_NOTHING,
                    );
                    D_QuitNetGame();
                    CL_Reset();
                    D_StartTitle();
                    free(reason as *mut libc::c_void);
                    cl_mode = CL_ABORTED;
                }
            }
            current_block_89 = 7301440000599063274;
        }
        1 => {
            if server != 0 && serverrunning != 0
                && node as libc::c_int != servernode as libc::c_int
            {
                Net_CloseConnection(node as int32_t);
            } else if node as libc::c_int != servernode as libc::c_int {
                Net_CloseConnection(node as int32_t);
            } else if !(cl_mode as libc::c_uint
                != CL_WAITJOINRESPONSE as libc::c_int as libc::c_uint)
            {
                if server == 0 {
                    neededtic = (*netbuffer).u.servercfg.gametic as int32_t as tic_t;
                    gametic = neededtic;
                    maketic = gametic;
                    G_SetGametype((*netbuffer).u.servercfg.gametype as int16_t);
                    modifiedgame = (*netbuffer).u.servercfg.modifiedgame as boolean;
                    if (*netbuffer).u.servercfg.usedCheats != 0 {
                        G_SetUsedCheats(true_0 as libc::c_int);
                    }
                    memcpy(
                        server_context.as_mut_ptr() as *mut libc::c_void,
                        ((*netbuffer).u.servercfg.server_context).as_mut_ptr()
                            as *const libc::c_void,
                        8 as libc::c_int as libc::c_ulong,
                    );
                }
                nodeingame[servernode as uint8_t as usize] = true_0 as libc::c_int;
                serverplayer = (*netbuffer).u.servercfg.serverplayer as int32_t;
                (*doomcom).numslots = (*netbuffer).u.servercfg.totalslotnum as int16_t;
                mynode = (*netbuffer).u.servercfg.clientnode;
                if serverplayer >= 0 as libc::c_int {
                    playernode[serverplayer as uint8_t as usize] = servernode as uint8_t;
                }
                if netgame != 0 {
                    CONS_Printf(
                        b"Join accepted, waiting for complete game state...\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"Server accept join gametic=%u mynode=%d\n\0" as *const u8
                                as *const libc::c_char,
                            gametic,
                            mynode as libc::c_int,
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                if (*netbuffer).u.servercfg.gamestate as libc::c_int
                    == GS_LEVEL as libc::c_int
                {
                    cl_mode = CL_DOWNLOADSAVEGAME;
                } else {
                    cl_mode = CL_CONNECTED;
                }
            }
            current_block_89 = 7301440000599063274;
        }
        24 => {
            if server != 0 {
                Net_CloseConnection(node as int32_t);
            } else if node as libc::c_int != servernode as libc::c_int {
                Net_CloseConnection(node as int32_t);
            } else {
                PT_FileFragment();
            }
            current_block_89 = 7301440000599063274;
        }
        25 => {
            if server != 0 {
                PT_FileAck();
            }
            current_block_89 = 7301440000599063274;
        }
        26 => {
            if server != 0 {
                PT_FileReceived();
            }
            current_block_89 = 7301440000599063274;
        }
        15 => {
            if server != 0 {
                if cv_downloading.value == 0 || PT_RequestFile(node as int32_t) == 0 {
                    Net_CloseConnection(node as int32_t);
                }
            } else {
                Net_CloseConnection(node as int32_t);
            }
            current_block_89 = 7301440000599063274;
        }
        30 | 11 => {
            if server != 0 {
                Net_CloseConnection(node as int32_t);
            }
            current_block_89 = 7301440000599063274;
        }
        2 => {
            current_block_89 = 7301440000599063274;
        }
        8 => {
            if node as libc::c_int == servernode as libc::c_int {
                current_block_89 = 7301440000599063274;
            } else {
                current_block_89 = 11694504731362138453;
            }
        }
        _ => {
            current_block_89 = 11694504731362138453;
        }
    }
    match current_block_89 {
        11694504731362138453 => {
            if !debugfile.is_null() {
                fputs(
                    va(
                        b"unknown packet received (%d) from unknown host\n\0"
                            as *const u8 as *const libc::c_char,
                        (*netbuffer).packettype as libc::c_int,
                    ),
                    debugfile,
                );
                fflush(debugfile);
            }
            Net_CloseConnection(node as int32_t);
        }
        _ => {}
    };
}
unsafe extern "C" fn HandlePacketFromPlayer(mut node: int8_t) {
    let mut netconsole: int32_t = 0;
    let mut realend: tic_t = 0;
    let mut realstart: tic_t = 0;
    let mut pak: *mut uint8_t = 0 as *mut uint8_t;
    let mut txtpak: *mut uint8_t = 0 as *mut uint8_t;
    let mut numtxtpak: uint8_t = 0;
    let mut finalmd5: [uint8_t; 16] = [0; 16];
    txtpak = 0 as *mut uint8_t;
    if dedicated != 0 && node as libc::c_int == 0 as libc::c_int {
        netconsole = 0 as libc::c_int;
    } else {
        netconsole = nodetoplayer[node as usize] as int32_t;
    }
    let mut faketic: tic_t = 0;
    let mut current_block_170: u64;
    match (*netbuffer).packettype as libc::c_int {
        2 | 4 | 3 | 5 | 6 | 7 => {
            if server == 0 {
                current_block_170 = 9879293939512133550;
            } else {
                realstart = ExpandTics(
                    (*netbuffer).u.clientpak.client_tic as int32_t,
                    node as int32_t,
                );
                realend = ExpandTics(
                    (*netbuffer).u.clientpak.resendfrom as int32_t,
                    node as int32_t,
                );
                if (*netbuffer).packettype as libc::c_int == PT_CLIENTMIS as libc::c_int
                    || (*netbuffer).packettype as libc::c_int
                        == PT_CLIENT2MIS as libc::c_int
                    || (*netbuffer).packettype as libc::c_int
                        == PT_NODEKEEPALIVEMIS as libc::c_int
                    || supposedtics[node as usize] < realend
                {
                    supposedtics[node as usize] = realend;
                }
                if nettics[node as usize] > realend {
                    if !debugfile.is_null() {
                        fputs(
                            va(
                                b"out of order ticcmd discarded nettics = %u\n\0"
                                    as *const u8 as *const libc::c_char,
                                nettics[node as usize],
                            ),
                            debugfile,
                        );
                        fflush(debugfile);
                    }
                } else {
                    nettics[node as usize] = realend;
                    if !(netconsole == -(1 as libc::c_int)) {
                        freezetimeout[node
                            as usize] = (I_GetTime()).wrapping_add(connectiontimeout);
                        if !((*netbuffer).packettype as libc::c_int
                            == PT_NODEKEEPALIVE as libc::c_int
                            || (*netbuffer).packettype as libc::c_int
                                == PT_NODEKEEPALIVEMIS as libc::c_int)
                        {
                            faketic = maketic;
                            if netcmds[(maketic % 1024 as libc::c_int as tic_t)
                                    as usize][netconsole as usize]
                                .angleturn as libc::c_int & 1 as libc::c_int != 0
                                && maketic.wrapping_sub(firstticstosend)
                                    < (1024 as libc::c_int - 1 as libc::c_int) as tic_t
                            {
                                faketic = faketic.wrapping_add(1);
                                faketic;
                            }
                            G_MoveTiccmd(
                                &mut *(*netcmds
                                    .as_mut_ptr()
                                    .offset((faketic % 1024 as libc::c_int as tic_t) as isize))
                                    .as_mut_ptr()
                                    .offset(netconsole as isize),
                                &mut (*netbuffer).u.clientpak.cmd,
                                1 as libc::c_int as size_t,
                            );
                            if netcmds[(faketic % 1024 as libc::c_int as tic_t)
                                    as usize][netconsole as usize]
                                .forwardmove as libc::c_int > 50 as libc::c_int
                                || (netcmds[(faketic % 1024 as libc::c_int as tic_t)
                                        as usize][netconsole as usize]
                                    .forwardmove as libc::c_int) < -(50 as libc::c_int)
                                || netcmds[(faketic % 1024 as libc::c_int as tic_t)
                                        as usize][netconsole as usize]
                                    .sidemove as libc::c_int > 50 as libc::c_int
                                || (netcmds[(faketic % 1024 as libc::c_int as tic_t)
                                        as usize][netconsole as usize]
                                    .sidemove as libc::c_int) < -(50 as libc::c_int)
                            {
                                CONS_Alert(
                                    CONS_WARNING,
                                    b"Illegal movement value received from node %d\n\0"
                                        as *const u8 as *const libc::c_char,
                                    netconsole,
                                );
                                SendKick(
                                    netconsole as uint8_t,
                                    2 as libc::c_int as uint8_t,
                                );
                            } else {
                                if ((*netbuffer).packettype as libc::c_int
                                    == PT_CLIENT2CMD as libc::c_int
                                    || (*netbuffer).packettype as libc::c_int
                                        == PT_CLIENT2MIS as libc::c_int)
                                    && nodetoplayer2[node as usize] as libc::c_int
                                        >= 0 as libc::c_int
                                {
                                    G_MoveTiccmd(
                                        &mut *(*netcmds
                                            .as_mut_ptr()
                                            .offset((faketic % 1024 as libc::c_int as tic_t) as isize))
                                            .as_mut_ptr()
                                            .offset(
                                                *nodetoplayer2.as_mut_ptr().offset(node as isize) as uint8_t
                                                    as isize,
                                            ),
                                        &mut (*netbuffer).u.client2pak.cmd2,
                                        1 as libc::c_int as size_t,
                                    );
                                }
                                if realstart <= gametic
                                    && realstart
                                        .wrapping_add(1024 as libc::c_int as tic_t)
                                        .wrapping_sub(1 as libc::c_int as tic_t) > gametic
                                    && gamestate as libc::c_uint
                                        == GS_LEVEL as libc::c_int as libc::c_uint
                                    && consistancy[(realstart % 1024 as libc::c_int as tic_t)
                                        as usize] as libc::c_int
                                        != (*netbuffer).u.clientpak.consistancy as libc::c_int
                                    && SV_ResendingSavegameToAnyone() == 0
                                    && resendingsavegame[node as usize] == 0
                                    && savegameresendcooldown[node as usize] <= I_GetTime()
                                {
                                    if cv_resynchattempts.value != 0 {
                                        (*netbuffer)
                                            .packettype = PT_WILLRESENDGAMESTATE as libc::c_int
                                            as uint8_t;
                                        HSendPacket(
                                            node as int32_t,
                                            true_0 as libc::c_int,
                                            0 as libc::c_int as uint8_t,
                                            0 as libc::c_int as size_t,
                                        );
                                        resendingsavegame[node as usize] = true_0 as libc::c_int;
                                        if cv_blamecfail.value != 0 {
                                            CONS_Printf(
                                                b"Synch failure for player %d (%s); expected %hd, got %hd\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                netconsole + 1 as libc::c_int,
                                                (player_names[netconsole as usize]).as_mut_ptr(),
                                                consistancy[(realstart % 1024 as libc::c_int as tic_t)
                                                    as usize] as libc::c_int,
                                                (*netbuffer).u.clientpak.consistancy as libc::c_int,
                                            );
                                        }
                                        if !debugfile.is_null() {
                                            fputs(
                                                va(
                                                    b"Restoring player %d (synch failure) [%update] %d!=%d\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    netconsole,
                                                    realstart,
                                                    consistancy[(realstart % 1024 as libc::c_int as tic_t)
                                                        as usize] as libc::c_int,
                                                    (*netbuffer).u.clientpak.consistancy as libc::c_int,
                                                ),
                                                debugfile,
                                            );
                                            fflush(debugfile);
                                        }
                                    } else {
                                        SendKick(
                                            netconsole as uint8_t,
                                            (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                                        );
                                        if !debugfile.is_null() {
                                            fputs(
                                                va(
                                                    b"player %d kicked (synch failure) [%u] %d!=%d\n\0"
                                                        as *const u8 as *const libc::c_char,
                                                    netconsole,
                                                    realstart,
                                                    consistancy[(realstart % 1024 as libc::c_int as tic_t)
                                                        as usize] as libc::c_int,
                                                    (*netbuffer).u.clientpak.consistancy as libc::c_int,
                                                ),
                                                debugfile,
                                            );
                                            fflush(debugfile);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                current_block_170 = 9879293939512133550;
            }
        }
        23 => {
            if server == 0 {
                current_block_170 = 9879293939512133550;
            } else if netconsole == -(1 as libc::c_int) {
                current_block_170 = 9879293939512133550;
            } else {
                sendingsavegame[node as usize] = false_0 as libc::c_int;
                freezetimeout[node
                    as usize] = (I_GetTime()).wrapping_add(connectiontimeout);
                current_block_170 = 9879293939512133550;
            }
        }
        28 => {
            netconsole = nodetoplayer2[node as usize] as int32_t;
            current_block_170 = 9566691321365426470;
        }
        27 => {
            current_block_170 = 9566691321365426470;
        }
        31 => {
            if server == 0 {
                current_block_170 = 9879293939512133550;
            } else if ((*doomcom).datalength as libc::c_int) < 16 as libc::c_int {
                current_block_170 = 9879293939512133550;
            } else {
                if adminpasswordset == 0 {
                    CONS_Printf(
                        b"Password from %s failed (no password set).\n\0" as *const u8
                            as *const libc::c_char,
                        (player_names[netconsole as usize]).as_mut_ptr(),
                    );
                } else {
                    D_MD5PasswordPass(
                        adminpassmd5.as_mut_ptr(),
                        16 as libc::c_int as size_t,
                        va(
                            b"PNUM%02d\0" as *const u8 as *const libc::c_char,
                            netconsole,
                        ),
                        &mut finalmd5 as *mut [uint8_t; 16] as *mut libc::c_void,
                    );
                    if memcmp(
                        ((*netbuffer).u.md5sum).as_mut_ptr() as *const libc::c_void,
                        finalmd5.as_mut_ptr() as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    ) == 0
                    {
                        CONS_Printf(
                            b"%s passed authentication.\n\0" as *const u8
                                as *const libc::c_char,
                            (player_names[netconsole as usize]).as_mut_ptr(),
                        );
                        COM_BufInsertTextEx(
                            va(
                                b"promote %d\n\0" as *const u8 as *const libc::c_char,
                                netconsole,
                            ),
                            0 as com_flags_t,
                        );
                    } else {
                        CONS_Printf(
                            b"Password from %s failed.\n\0" as *const u8
                                as *const libc::c_char,
                            (player_names[netconsole as usize]).as_mut_ptr(),
                        );
                    }
                }
                current_block_170 = 9879293939512133550;
            }
        }
        30 | 11 => {
            if server == 0 {
                current_block_170 = 9879293939512133550;
            } else {
                nodewaiting[node as usize] = 0 as libc::c_int as uint8_t;
                if netconsole != -(1 as libc::c_int)
                    && playeringame[netconsole as usize] != 0
                {
                    let mut kickmsg: uint8_t = 0;
                    if (*netbuffer).packettype as libc::c_int
                        == PT_NODETIMEOUT as libc::c_int
                    {
                        kickmsg = 4 as libc::c_int as uint8_t;
                    } else {
                        kickmsg = 3 as libc::c_int as uint8_t;
                    }
                    kickmsg = (kickmsg as libc::c_int | 0x80 as libc::c_int) as uint8_t;
                    SendKick(netconsole as uint8_t, kickmsg);
                    nodetoplayer[node as usize] = -(1 as libc::c_int) as int8_t;
                    if nodetoplayer2[node as usize] as libc::c_int != -(1 as libc::c_int)
                        && nodetoplayer2[node as usize] as libc::c_int
                            >= 0 as libc::c_int
                        && playeringame[nodetoplayer2[node as usize] as uint8_t as usize]
                            != 0
                    {
                        SendKick(nodetoplayer2[node as usize] as uint8_t, kickmsg);
                        nodetoplayer2[node as usize] = -(1 as libc::c_int) as int8_t;
                    }
                }
                Net_CloseConnection(node as int32_t);
                nodeingame[node as usize] = false_0 as libc::c_int;
                current_block_170 = 9879293939512133550;
            }
        }
        18 => {
            PT_CanReceiveGamestate(node);
            current_block_170 = 9879293939512133550;
        }
        21 => {
            if server != 0 && !luafiletransfers.is_null()
                && (*luafiletransfers).nodestatus[node as usize] as libc::c_uint
                    == LFTNS_ASKED as libc::c_int as libc::c_uint
            {
                AddLuaFileToSendQueue(node as int32_t, (*luafiletransfers).realfilename);
            }
            current_block_170 = 9879293939512133550;
        }
        22 => {
            if server != 0 && !luafiletransfers.is_null()
                && (*luafiletransfers).nodestatus[node as usize] as libc::c_uint
                    == LFTNS_SENDING as libc::c_int as libc::c_uint
            {
                SV_HandleLuaFileSent(node as uint8_t);
            }
            current_block_170 = 9879293939512133550;
        }
        19 => {
            sendingsavegame[node as usize] = false_0 as libc::c_int;
            resendingsavegame[node as usize] = false_0 as libc::c_int;
            savegameresendcooldown[node
                as usize] = (I_GetTime())
                .wrapping_add((5 as libc::c_int * 35 as libc::c_int) as tic_t);
            current_block_170 = 9879293939512133550;
        }
        8 => {
            if node as libc::c_int != servernode as libc::c_int {
                CONS_Alert(
                    CONS_WARNING,
                    b"%s received from non-host %d\n\0" as *const u8
                        as *const libc::c_char,
                    b"PT_SERVERTICS\0" as *const u8 as *const libc::c_char,
                    node as libc::c_int,
                );
                if server != 0 {
                    SendKick(
                        netconsole as uint8_t,
                        (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                    );
                }
            } else {
                realstart = (*netbuffer).u.serverpak.starttic;
                realend = realstart
                    .wrapping_add((*netbuffer).u.serverpak.numtics as tic_t);
                if txtpak.is_null() {
                    txtpak = &mut *((*netbuffer).u.serverpak.cmds)
                        .as_mut_ptr()
                        .offset(
                            ((*netbuffer).u.serverpak.numslots as libc::c_int
                                * (*netbuffer).u.serverpak.numtics as libc::c_int) as isize,
                        ) as *mut ticcmd_t as *mut uint8_t;
                }
                if realend > gametic.wrapping_add(32 as libc::c_int as tic_t) {
                    realend = gametic.wrapping_add(32 as libc::c_int as tic_t);
                }
                cl_packetmissed = (realstart > neededtic) as libc::c_int;
                if realstart <= neededtic && realend > neededtic {
                    let mut i: tic_t = 0;
                    let mut j_0: tic_t = 0;
                    pak = &mut (*netbuffer).u.serverpak.cmds as *mut [ticcmd_t; 45]
                        as *mut uint8_t;
                    i = realstart;
                    while i < realend {
                        D_Clearticcmd(i);
                        pak = G_ScpyTiccmd(
                            (netcmds[(i % 1024 as libc::c_int as tic_t) as usize])
                                .as_mut_ptr(),
                            pak as *mut libc::c_void,
                            ((*netbuffer).u.serverpak.numslots as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        let fresh4 = txtpak;
                        txtpak = txtpak.offset(1);
                        numtxtpak = *fresh4;
                        j_0 = 0 as libc::c_int as tic_t;
                        while j_0 < numtxtpak as tic_t {
                            let fresh5 = txtpak;
                            txtpak = txtpak.offset(1);
                            let mut k: int32_t = *fresh5 as int32_t;
                            let txtsize: size_t = (*txtpak
                                .offset(0 as libc::c_int as isize) as libc::c_int
                                + 1 as libc::c_int) as size_t;
                            if i >= gametic {
                                M_Memcpy
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    D_GetTextcmd(i, k) as *mut libc::c_void,
                                    txtpak as *const libc::c_void,
                                    txtsize,
                                );
                            }
                            txtpak = txtpak.offset(txtsize as isize);
                            j_0 = j_0.wrapping_add(1);
                            j_0;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                    neededtic = realend;
                } else if !debugfile.is_null() {
                    fputs(
                        va(
                            b"frame not in bound: %u\n\0" as *const u8
                                as *const libc::c_char,
                            neededtic,
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
            }
            current_block_170 = 9879293939512133550;
        }
        34 => {
            if node as libc::c_int != servernode as libc::c_int {
                CONS_Alert(
                    CONS_WARNING,
                    b"%s received from non-host %d\n\0" as *const u8
                        as *const libc::c_char,
                    b"PT_PING\0" as *const u8 as *const libc::c_char,
                    node as libc::c_int,
                );
                if server != 0 {
                    SendKick(
                        netconsole as uint8_t,
                        (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                    );
                }
            } else if server == 0 {
                let mut i_0: uint8_t = 0;
                i_0 = 0 as libc::c_int as uint8_t;
                while (i_0 as libc::c_int) < 32 as libc::c_int {
                    if playeringame[i_0 as usize] != 0 {
                        playerpingtable[i_0
                            as usize] = (*netbuffer).u.pingtable[i_0 as usize];
                    }
                    i_0 = i_0.wrapping_add(1);
                    i_0;
                }
                servermaxping = (*netbuffer).u.pingtable[32 as libc::c_int as usize];
            }
            current_block_170 = 9879293939512133550;
        }
        1 => {
            current_block_170 = 9879293939512133550;
        }
        24 => {
            if node as libc::c_int != servernode as libc::c_int {
                CONS_Alert(
                    CONS_WARNING,
                    b"%s received from non-host %d\n\0" as *const u8
                        as *const libc::c_char,
                    b"PT_FILEFRAGMENT\0" as *const u8 as *const libc::c_char,
                    node as libc::c_int,
                );
                if server != 0 {
                    SendKick(
                        netconsole as uint8_t,
                        (2 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                    );
                }
            } else if server == 0 {
                PT_FileFragment();
            }
            current_block_170 = 9879293939512133550;
        }
        25 => {
            if server != 0 {
                PT_FileAck();
            }
            current_block_170 = 9879293939512133550;
        }
        26 => {
            if server != 0 {
                PT_FileReceived();
            }
            current_block_170 = 9879293939512133550;
        }
        17 => {
            PT_WillResendGamestate();
            current_block_170 = 9879293939512133550;
        }
        20 => {
            if server == 0 {
                CL_PrepareDownloadLuaFile();
            }
            current_block_170 = 9879293939512133550;
        }
        _ => {
            if !debugfile.is_null() {
                fputs(
                    va(
                        b"UNKNOWN PACKET TYPE RECEIVED %d from host %d\n\0" as *const u8
                            as *const libc::c_char,
                        (*netbuffer).packettype as libc::c_int,
                        node as libc::c_int,
                    ),
                    debugfile,
                );
                fflush(debugfile);
            }
            current_block_170 = 9879293939512133550;
        }
    }
    match current_block_170 {
        9566691321365426470 => {
            if !(server == 0) {
                if netconsole < 0 as libc::c_int || netconsole >= 32 as libc::c_int {
                    Net_UnAcknowledgePacket(node as int32_t);
                } else {
                    let mut j: size_t = 0;
                    let mut tic: tic_t = maketic;
                    let mut textcmd: *mut uint8_t = 0 as *mut uint8_t;
                    if (*netbuffer).u.textcmd[0 as libc::c_int as usize] == 0 {
                        if !debugfile.is_null() {
                            fputs(
                                va(
                                    b"GetPacket: Textcmd with size 0 detected! (node %u, player %d)\n\0"
                                        as *const u8 as *const libc::c_char,
                                    node as libc::c_int,
                                    netconsole,
                                ),
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                        Net_UnAcknowledgePacket(node as int32_t);
                    } else if (*netbuffer).u.textcmd[0 as libc::c_int as usize]
                        as libc::c_ulong
                        > ((*doomcom).datalength as size_t)
                            .wrapping_sub(8 as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    {
                        if !debugfile.is_null() {
                            fputs(
                                va(
                                    b"GetPacket: Bad Textcmd packet size! (expected %d, actual %s, node %u, player %d)\n\0"
                                        as *const u8 as *const libc::c_char,
                                    (*netbuffer).u.textcmd[0 as libc::c_int as usize]
                                        as libc::c_int,
                                    sizeu1(
                                        ((*doomcom).datalength as size_t)
                                            .wrapping_sub(8 as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                    ),
                                    node as libc::c_int,
                                    netconsole,
                                ),
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                        Net_UnAcknowledgePacket(node as int32_t);
                    } else {
                        j = (software_MAXPACKETLENGTH as libc::c_ulong)
                            .wrapping_sub(
                                (((*netbuffer).u.textcmd[0 as libc::c_int as usize]
                                    as libc::c_int + 2 as libc::c_int) as libc::c_ulong)
                                    .wrapping_add(14 as libc::c_ulong)
                                    .wrapping_add(
                                        (((*doomcom).numslots as libc::c_int + 1 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_mul(
                                                ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
                                            ),
                                    ),
                            );
                        loop {
                            textcmd = D_GetExistingTextcmd(tic, netconsole);
                            if !((TotalTextCmdPerTic(tic) > j
                                || (*netbuffer).u.textcmd[0 as libc::c_int as usize]
                                    as libc::c_int
                                    + (if !textcmd.is_null() {
                                        *textcmd.offset(0 as libc::c_int as isize) as libc::c_int
                                    } else {
                                        0 as libc::c_int
                                    }) > 256 as libc::c_int)
                                && tic
                                    < firstticstosend
                                        .wrapping_add(1024 as libc::c_int as tic_t))
                            {
                                break;
                            }
                            tic = tic.wrapping_add(1);
                            tic;
                        }
                        if tic
                            >= firstticstosend.wrapping_add(1024 as libc::c_int as tic_t)
                        {
                            if !debugfile.is_null() {
                                fputs(
                                    va(
                                        b"GetPacket: Textcmd too long (max %s, used %s, mak %d, tosend %u, node %u, player %d)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        sizeu1(j),
                                        sizeu2(TotalTextCmdPerTic(maketic)),
                                        maketic,
                                        firstticstosend,
                                        node as libc::c_int,
                                        netconsole,
                                    ),
                                    debugfile,
                                );
                                fflush(debugfile);
                            }
                            Net_UnAcknowledgePacket(node as int32_t);
                        } else {
                            if textcmd.is_null() {
                                textcmd = D_GetTextcmd(tic, netconsole);
                            }
                            if !debugfile.is_null() {
                                fputs(
                                    va(
                                        b"textcmd put in tic %u at position %d (player %d) ftts %u mk %u\n\0"
                                            as *const u8 as *const libc::c_char,
                                        tic,
                                        *textcmd.offset(0 as libc::c_int as isize) as libc::c_int
                                            + 1 as libc::c_int,
                                        netconsole,
                                        firstticstosend,
                                        maketic,
                                    ),
                                    debugfile,
                                );
                                fflush(debugfile);
                            }
                            M_Memcpy
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut *textcmd
                                    .offset(
                                        (*textcmd.offset(0 as libc::c_int as isize) as libc::c_int
                                            + 1 as libc::c_int) as isize,
                                    ) as *mut uint8_t as *mut libc::c_void,
                                ((*netbuffer).u.textcmd)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                                (*netbuffer).u.textcmd[0 as libc::c_int as usize] as size_t,
                            );
                            let ref mut fresh3 = *textcmd
                                .offset(0 as libc::c_int as isize);
                            *fresh3 = (*fresh3 as libc::c_int
                                + (*netbuffer).u.textcmd[0 as libc::c_int as usize]
                                    as libc::c_int) as uint8_t;
                        }
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn GetPackets() {
    let mut node: int8_t = 0;
    player_joining = false_0 as libc::c_int as uint8_t;
    while HGetPacket() != 0 {
        node = (*doomcom).remotenode as int8_t;
        if (*netbuffer).packettype as libc::c_int == PT_CLIENTJOIN as libc::c_int
            && server != 0
        {
            HandleConnect(node);
        } else {
            if node as libc::c_int == servernode as libc::c_int && server == 0
                && cl_mode as libc::c_uint != CL_SEARCHING as libc::c_int as libc::c_uint
            {
                if (*netbuffer).packettype as libc::c_int
                    == PT_SERVERSHUTDOWN as libc::c_int
                {
                    HandleShutdown(node);
                    continue;
                } else if (*netbuffer).packettype as libc::c_int
                    == PT_NODETIMEOUT as libc::c_int
                {
                    HandleTimeout(node);
                    continue;
                }
            }
            if (*netbuffer).packettype as libc::c_int == PT_SERVERINFO as libc::c_int {
                HandleServerInfo(node);
            } else {
                if (*netbuffer).packettype as libc::c_int == PT_PLAYERINFO as libc::c_int
                {
                    continue;
                }
                if nodeingame[node as usize] != 0 {
                    HandlePacketFromPlayer(node);
                } else {
                    HandlePacketFromAwayNode(node);
                }
            }
        }
    }
}
unsafe extern "C" fn Consistancy() -> int16_t {
    let mut i: int32_t = 0;
    let mut ret: uint32_t = 0 as libc::c_int as uint32_t;
    if !debugfile.is_null() {
        fputs(va(b"TIC %u \0" as *const u8 as *const libc::c_char, gametic), debugfile);
        fflush(debugfile);
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] == 0 {
            ret ^= 0xcccc as libc::c_int as uint32_t;
        } else if !(players[i as usize].mo).is_null() {
            ret = ret.wrapping_add((*players[i as usize].mo).x as uint32_t);
            ret = ret.wrapping_sub((*players[i as usize].mo).y as uint32_t);
            ret = ret
                .wrapping_add(
                    players[i as usize].powers[pw_shield as libc::c_int as usize]
                        as uint32_t,
                );
            ret = ret * (i + 1 as libc::c_int) as uint32_t;
        }
        i += 1;
        i;
    }
    if G_PlatformGametype() == 0 {
        ret = ret.wrapping_add(P_GetRandSeed());
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"Consistancy = %u\n\0" as *const u8 as *const libc::c_char,
                ret & 0xffff as libc::c_int as uint32_t,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    return (ret & 0xffff as libc::c_int as uint32_t) as int16_t;
}
unsafe extern "C" fn CL_SendClientKeepAlive() {
    (*netbuffer).packettype = PT_BASICKEEPALIVE as libc::c_int as uint8_t;
    HSendPacket(
        servernode as int32_t,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn SV_SendServerKeepAlive() {
    let mut n: int32_t = 0;
    n = 1 as libc::c_int;
    while n < 127 as libc::c_int {
        if nodeingame[n as usize] != 0 {
            (*netbuffer).packettype = PT_BASICKEEPALIVE as libc::c_int as uint8_t;
            HSendPacket(
                n,
                false_0 as libc::c_int,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as size_t,
            );
        }
        n += 1;
        n;
    }
}
unsafe extern "C" fn CL_SendClientCmd() {
    let mut packetsize: size_t = 0 as libc::c_int as size_t;
    let mut mis: boolean = false_0 as libc::c_int;
    (*netbuffer).packettype = PT_CLIENTCMD as libc::c_int as uint8_t;
    if cl_packetmissed != 0 {
        (*netbuffer).packettype = PT_CLIENTMIS as libc::c_int as uint8_t;
        mis = true_0 as libc::c_int;
    }
    (*netbuffer)
        .u
        .clientpak
        .resendfrom = (neededtic & 255 as libc::c_int as tic_t) as uint8_t;
    (*netbuffer)
        .u
        .clientpak
        .client_tic = (gametic & 255 as libc::c_int as tic_t) as uint8_t;
    if gamestate as libc::c_uint == GS_WAITINGPLAYERS as libc::c_int as libc::c_uint {
        (*netbuffer)
            .packettype = (if mis != 0 {
            PT_NODEKEEPALIVEMIS as libc::c_int
        } else {
            PT_NODEKEEPALIVE as libc::c_int
        }) as uint8_t;
        packetsize = (::core::mem::size_of::<clientcmd_pak>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<ticcmd_t>() as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<int16_t>() as libc::c_ulong);
        HSendPacket(
            servernode as int32_t,
            false_0 as libc::c_int,
            0 as libc::c_int as uint8_t,
            packetsize,
        );
    } else if gamestate as libc::c_uint != GS_NULL as libc::c_int as libc::c_uint
        && (addedtogame != 0 || dedicated != 0)
    {
        packetsize = ::core::mem::size_of::<clientcmd_pak>() as libc::c_ulong;
        G_MoveTiccmd(
            &mut (*netbuffer).u.clientpak.cmd,
            &mut localcmds,
            1 as libc::c_int as size_t,
        );
        (*netbuffer)
            .u
            .clientpak
            .consistancy = consistancy[(gametic % 1024 as libc::c_int as tic_t)
            as usize];
        if splitscreen != 0 || botingame != 0 {
            (*netbuffer)
                .packettype = (if mis != 0 {
                PT_CLIENT2MIS as libc::c_int
            } else {
                PT_CLIENT2CMD as libc::c_int
            }) as uint8_t;
            packetsize = ::core::mem::size_of::<client2cmd_pak>() as libc::c_ulong;
            G_MoveTiccmd(
                &mut (*netbuffer).u.client2pak.cmd2,
                &mut localcmds2,
                1 as libc::c_int as size_t,
            );
        }
        HSendPacket(
            servernode as int32_t,
            false_0 as libc::c_int,
            0 as libc::c_int as uint8_t,
            packetsize,
        );
    }
    if cl_mode as libc::c_uint == CL_CONNECTED as libc::c_int as libc::c_uint
        || dedicated != 0
    {
        if localtextcmd[0 as libc::c_int as usize] != 0 {
            (*netbuffer).packettype = PT_TEXTCMD as libc::c_int as uint8_t;
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                ((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_void,
                localtextcmd.as_mut_ptr() as *const libc::c_void,
                (localtextcmd[0 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as size_t,
            );
            if HSendPacket(
                servernode as int32_t,
                true_0 as libc::c_int,
                0 as libc::c_int as uint8_t,
                (localtextcmd[0 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as size_t,
            ) != 0
            {
                localtextcmd[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
            }
        }
        if localtextcmd2[0 as libc::c_int as usize] != 0 {
            (*netbuffer).packettype = PT_TEXTCMD2 as libc::c_int as uint8_t;
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                ((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_void,
                localtextcmd2.as_mut_ptr() as *const libc::c_void,
                (localtextcmd2[0 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as size_t,
            );
            if HSendPacket(
                servernode as int32_t,
                true_0 as libc::c_int,
                0 as libc::c_int as uint8_t,
                (localtextcmd2[0 as libc::c_int as usize] as libc::c_int
                    + 1 as libc::c_int) as size_t,
            ) != 0
            {
                localtextcmd2[0 as libc::c_int as usize] = 0 as libc::c_int as uint8_t;
            }
        }
    }
}
unsafe extern "C" fn SV_SendTics() {
    let mut realfirsttic: tic_t = 0;
    let mut lasttictosend: tic_t = 0;
    let mut i: tic_t = 0;
    let mut n: uint32_t = 0;
    let mut j: int32_t = 0;
    let mut packsize: size_t = 0;
    let mut bufpos: *mut uint8_t = 0 as *mut uint8_t;
    let mut ntextcmd: *mut uint8_t = 0 as *mut uint8_t;
    let mut current_block_75: u64;
    n = 1 as libc::c_int as uint32_t;
    while n < 127 as libc::c_int as uint32_t {
        if nodeingame[n as usize] != 0 {
            realfirsttic = supposedtics[n as usize];
            lasttictosend = if maketic
                < (nettics[n as usize]).wrapping_add(32 as libc::c_int as tic_t)
            {
                maketic
            } else {
                (nettics[n as usize]).wrapping_add(32 as libc::c_int as tic_t)
            };
            if realfirsttic >= lasttictosend {
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"Nothing to send node %u mak=%u sup=%u net=%u \n\0"
                                as *const u8 as *const libc::c_char,
                            n,
                            maketic,
                            supposedtics[n as usize],
                            nettics[n as usize],
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                realfirsttic = nettics[n as usize];
                if realfirsttic >= lasttictosend
                    || (I_GetTime()).wrapping_add(n) & 3 as libc::c_int as uint32_t != 0
                {
                    current_block_75 = 12517898123489920830;
                } else {
                    if !debugfile.is_null() {
                        fputs(
                            va(
                                b"Sent %d anyway\n\0" as *const u8 as *const libc::c_char,
                                realfirsttic,
                            ),
                            debugfile,
                        );
                        fflush(debugfile);
                    }
                    current_block_75 = 15652330335145281839;
                }
            } else {
                current_block_75 = 15652330335145281839;
            }
            match current_block_75 {
                12517898123489920830 => {}
                _ => {
                    if realfirsttic < firstticstosend {
                        realfirsttic = firstticstosend;
                    }
                    packsize = 14 as libc::c_ulong;
                    i = realfirsttic;
                    while i < lasttictosend {
                        packsize = (packsize as libc::c_ulong)
                            .wrapping_add(
                                (::core::mem::size_of::<ticcmd_t>() as libc::c_ulong)
                                    .wrapping_mul((*doomcom).numslots as libc::c_ulong),
                            ) as size_t as size_t;
                        packsize = packsize.wrapping_add(TotalTextCmdPerTic(i));
                        if packsize > software_MAXPACKETLENGTH as size_t {
                            if !debugfile.is_null() {
                                fputs(
                                    va(
                                        b"packet too large (%s) at tic %d (should be from %d to %d)\n\0"
                                            as *const u8 as *const libc::c_char,
                                        sizeu1(packsize),
                                        i,
                                        realfirsttic,
                                        lasttictosend,
                                    ),
                                    debugfile,
                                );
                                fflush(debugfile);
                            }
                            lasttictosend = i;
                            if lasttictosend == realfirsttic {
                                if packsize > 1450 as libc::c_int as size_t {
                                    I_Error(
                                        b"Too many players: can't send %s data for %d players to node %d\nWell sorry nobody is perfect....\n\0"
                                            as *const u8 as *const libc::c_char,
                                        sizeu1(packsize),
                                        (*doomcom).numslots as libc::c_int,
                                        n,
                                    );
                                } else {
                                    lasttictosend = lasttictosend.wrapping_add(1);
                                    lasttictosend;
                                    if !debugfile.is_null() {
                                        fputs(
                                            b"sending it anyway\n\0" as *const u8
                                                as *const libc::c_char,
                                            debugfile,
                                        );
                                        fflush(debugfile);
                                    }
                                }
                            }
                            break;
                        } else {
                            i = i.wrapping_add(1);
                            i;
                        }
                    }
                    (*netbuffer).packettype = PT_SERVERTICS as libc::c_int as uint8_t;
                    (*netbuffer).u.serverpak.starttic = realfirsttic;
                    (*netbuffer)
                        .u
                        .serverpak
                        .numtics = lasttictosend.wrapping_sub(realfirsttic) as uint8_t;
                    (*netbuffer).u.serverpak.numslots = (*doomcom).numslots as uint8_t;
                    bufpos = &mut (*netbuffer).u.serverpak.cmds as *mut [ticcmd_t; 45]
                        as *mut uint8_t;
                    i = realfirsttic;
                    while i < lasttictosend {
                        bufpos = G_DcpyTiccmd(
                            bufpos as *mut libc::c_void,
                            (netcmds[(i % 1024 as libc::c_int as tic_t) as usize])
                                .as_mut_ptr(),
                            ((*doomcom).numslots as libc::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
                                ),
                        ) as *mut uint8_t;
                        i = i.wrapping_add(1);
                        i;
                    }
                    i = realfirsttic;
                    while i < lasttictosend {
                        let fresh6 = bufpos;
                        bufpos = bufpos.offset(1);
                        ntextcmd = fresh6;
                        *ntextcmd = 0 as libc::c_int as uint8_t;
                        j = 0 as libc::c_int;
                        while j < 32 as libc::c_int {
                            let mut textcmd: *mut uint8_t = D_GetExistingTextcmd(i, j);
                            let mut size: int32_t = if !textcmd.is_null() {
                                *textcmd.offset(0 as libc::c_int as isize) as libc::c_int
                            } else {
                                0 as libc::c_int
                            };
                            if (j == 0 || playeringame[j as usize] != 0) && size != 0 {
                                *ntextcmd = (*ntextcmd).wrapping_add(1);
                                *ntextcmd;
                                let mut p_tmp: *mut uint8_t = bufpos as *mut libc::c_void
                                    as *mut uint8_t;
                                let tv: uint8_t = j as uint8_t;
                                memcpy(
                                    bufpos as *mut libc::c_void,
                                    &tv as *const uint8_t as *const libc::c_void,
                                    ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                                );
                                p_tmp = p_tmp.offset(1);
                                p_tmp;
                                bufpos = p_tmp as *mut libc::c_void as *mut uint8_t;
                                M_Memcpy
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    bufpos as *mut libc::c_void,
                                    textcmd as *const libc::c_void,
                                    (size + 1 as libc::c_int) as size_t,
                                );
                                bufpos = bufpos.offset((size + 1 as libc::c_int) as isize);
                            }
                            j += 1;
                            j;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                    packsize = bufpos
                        .offset_from(
                            &mut (*netbuffer).u as *mut C2RustUnnamed_7 as *mut uint8_t,
                        ) as libc::c_long as size_t;
                    HSendPacket(
                        n as int32_t,
                        false_0 as libc::c_int,
                        0 as libc::c_int as uint8_t,
                        packsize,
                    );
                    if lasttictosend.wrapping_sub((*doomcom).extratics as tic_t)
                        > realfirsttic
                    {
                        supposedtics[n
                            as usize] = lasttictosend
                            .wrapping_sub((*doomcom).extratics as tic_t);
                    } else {
                        supposedtics[n as usize] = lasttictosend;
                    }
                    if supposedtics[n as usize] < nettics[n as usize] {
                        supposedtics[n as usize] = nettics[n as usize];
                    }
                }
            }
        }
        n = n.wrapping_add(1);
        n;
    }
    supposedtics[0 as libc::c_int as usize] = maketic;
}
unsafe extern "C" fn Local_Maketic(mut realtics: int32_t) {
    I_OsPolling();
    D_ProcessEvents();
    if dedicated == 0 {
        rendergametic = gametic;
    }
    G_BuildTiccmd(&mut localcmds, realtics, 1 as libc::c_int as uint8_t);
    if splitscreen != 0 || botingame != 0 {
        G_BuildTiccmd(&mut localcmds2, realtics, 2 as libc::c_int as uint8_t);
    }
    localcmds
        .angleturn = (localcmds.angleturn as libc::c_int | 1 as libc::c_int) as int16_t;
    localcmds2
        .angleturn = (localcmds2.angleturn as libc::c_int | 1 as libc::c_int) as int16_t;
}
unsafe extern "C" fn SV_Maketic() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !(playeringame[i as usize] == 0) {
            if netcmds[(maketic % 1024 as libc::c_int as tic_t) as usize][i as usize]
                .angleturn as libc::c_int & 1 as libc::c_int == 0 as libc::c_int
            {
                let mut ticcmd: *mut ticcmd_t = &mut *(*netcmds
                    .as_mut_ptr()
                    .offset((maketic % 1024 as libc::c_int as tic_t) as isize))
                    .as_mut_ptr()
                    .offset(i as isize) as *mut ticcmd_t;
                let mut prevticcmd: *mut ticcmd_t = &mut *(*netcmds
                    .as_mut_ptr()
                    .offset(
                        (maketic.wrapping_sub(1 as libc::c_int as tic_t)
                            % 1024 as libc::c_int as tic_t) as isize,
                    ))
                    .as_mut_ptr()
                    .offset(i as isize) as *mut ticcmd_t;
                if players[i as usize].quittime != 0 {
                    memset(
                        ticcmd as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<ticcmd_t>() as libc::c_ulong,
                    );
                    (*ticcmd)
                        .angleturn = ((*prevticcmd).angleturn as libc::c_int
                        | 1 as libc::c_int) as int16_t;
                    (*ticcmd).aiming = (*prevticcmd).aiming;
                } else {
                    if !debugfile.is_null() {
                        fputs(
                            va(
                                b"MISS tic%4d for player %d\n\0" as *const u8
                                    as *const libc::c_char,
                                maketic,
                                i,
                            ),
                            debugfile,
                        );
                        fflush(debugfile);
                    }
                    *ticcmd = *prevticcmd;
                    (*ticcmd)
                        .angleturn = ((*ticcmd).angleturn as libc::c_int
                        & !(1 as libc::c_int)) as int16_t;
                }
            }
        }
        i += 1;
        i;
    }
    maketic = maketic.wrapping_add(1);
    maketic;
}
#[no_mangle]
pub unsafe extern "C" fn TryRunTics(mut realtics: tic_t) -> boolean {
    let mut ticking: boolean = 0;
    if realtics > (35 as libc::c_int / 7 as libc::c_int) as tic_t {
        if server != 0 {
            realtics = 1 as libc::c_int as tic_t;
        } else {
            realtics = (35 as libc::c_int / 7 as libc::c_int) as tic_t;
        }
    }
    if singletics != 0 {
        realtics = 1 as libc::c_int as tic_t;
    }
    if realtics >= 1 as libc::c_int as tic_t {
        COM_BufTicker();
        if mapchangepending != 0 {
            D_MapChange(
                -(1 as libc::c_int),
                0 as libc::c_int,
                ultimatemode as boolean,
                false_0 as libc::c_int,
                2 as libc::c_int,
                false_0 as libc::c_int,
                fromlevelselect,
            );
        }
    }
    NetUpdate();
    if demoplayback != 0 {
        neededtic = gametic.wrapping_add(realtics);
        maketic = maketic.wrapping_add(realtics);
        firstticstosend = maketic;
        tictoclear = firstticstosend;
    }
    GetPackets();
    if !debugfile.is_null() && (realtics != 0 || neededtic > gametic) {
        fprintf(
            debugfile,
            b"------------ Tryruntic: REAL:%d NEED:%d GAME:%d LOAD: %d\n\0" as *const u8
                as *const libc::c_char,
            realtics,
            neededtic,
            gametic,
            debugload,
        );
        debugload = 100000 as libc::c_int;
    }
    ticking = (neededtic > gametic) as libc::c_int;
    if ticking != 0 {
        if realtics != 0 {
            hu_stopped = false_0 as libc::c_int;
        }
    }
    if player_joining != 0 {
        if realtics != 0 {
            hu_stopped = true_0 as libc::c_int;
        }
        return false_0 as libc::c_int;
    }
    if ticking != 0 {
        if advancedemo != 0 {
            if timedemo_quit != 0 {
                COM_ImmedExecute(b"quit\0" as *const u8 as *const libc::c_char);
            } else {
                D_StartTitle();
            }
        } else {
            while neededtic > gametic {
                let mut update_stats: boolean = !(paused as libc::c_int != 0
                    || P_AutoPause() != 0) as libc::c_int;
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"============ Running tic %d (local %d)\n\0" as *const u8
                                as *const libc::c_char,
                            gametic,
                            leveltime,
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                if update_stats != 0 {
                    ps_tictime.value.p = I_GetPreciseTime();
                }
                G_Ticker(
                    (gametic % 1 as libc::c_int as tic_t == 0 as libc::c_int as tic_t)
                        as libc::c_int,
                );
                ExtraDataTicker();
                gametic = gametic.wrapping_add(1);
                gametic;
                consistancy[(gametic % 1024 as libc::c_int as tic_t)
                    as usize] = Consistancy();
                if update_stats != 0 {
                    ps_tictime
                        .value
                        .p = (I_GetPreciseTime()).wrapping_sub(ps_tictime.value.p);
                    PS_UpdateTickStats();
                }
                if server == 0
                    && gamestate as libc::c_uint
                        == GS_LEVEL as libc::c_int as libc::c_uint
                    && leveltime > 3 as libc::c_int as tic_t
                    && neededtic <= gametic.wrapping_add(cv_netticbuffer.value as tic_t)
                {
                    break;
                }
            }
        }
    } else if realtics != 0 {
        hu_stopped = true_0 as libc::c_int;
    }
    return ticking;
}
static mut pingtimeout: [int32_t; 32] = [0; 32];
#[inline]
unsafe extern "C" fn PingUpdate() {
    let mut i: int32_t = 0;
    let mut laggers: [boolean; 32] = [0; 32];
    let mut numlaggers: uint8_t = 0 as libc::c_int as uint8_t;
    memset(
        laggers.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<boolean>() as libc::c_ulong)
            .wrapping_mul(32 as libc::c_int as libc::c_ulong),
    );
    (*netbuffer).packettype = PT_PING as libc::c_int as uint8_t;
    if cv_maxping.value != 0 {
        i = 1 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0 && players[i as usize].quittime == 0
                && realpingtable[i as usize] / pingmeasurecount as uint32_t
                    > cv_maxping.value as libc::c_uint
            {
                if players[i as usize].jointime
                    > (30 as libc::c_int * 35 as libc::c_int) as tic_t
                {
                    laggers[i as usize] = true_0 as libc::c_int;
                }
                numlaggers = numlaggers.wrapping_add(1);
                numlaggers;
            } else {
                pingtimeout[i as usize] = 0 as libc::c_int;
            }
            i += 1;
            i;
        }
        if (numlaggers as libc::c_int) < D_NumPlayers() - 1 as libc::c_int {
            i = 1 as libc::c_int;
            while i < 32 as libc::c_int {
                if playeringame[i as usize] != 0 && laggers[i as usize] != 0 {
                    pingtimeout[i as usize] += 1;
                    pingtimeout[i as usize];
                    if pingtimeout[i as usize] > cv_pingtimeout.value {
                        pingtimeout[i as usize] = 0 as libc::c_int;
                        SendKick(
                            i as uint8_t,
                            (6 as libc::c_int | 0x80 as libc::c_int) as uint8_t,
                        );
                    }
                } else {
                    pingtimeout[i
                        as usize] = if pingtimeout[i as usize] == 0 as libc::c_int {
                        0 as libc::c_int
                    } else {
                        pingtimeout[i as usize] - 1 as libc::c_int
                    };
                }
                i += 1;
                i;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        (*netbuffer)
            .u
            .pingtable[i
            as usize] = realpingtable[i as usize] / pingmeasurecount as uint32_t;
        playerpingtable[i
            as usize] = realpingtable[i as usize] / pingmeasurecount as uint32_t;
        realpingtable[i as usize] = 0 as libc::c_int as uint32_t;
        i += 1;
        i;
    }
    (*netbuffer).u.pingtable[32 as libc::c_int as usize] = cv_maxping.value as uint32_t;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        if nodeingame[i as usize] != 0 {
            HSendPacket(
                i,
                true_0 as libc::c_int,
                0 as libc::c_int as uint8_t,
                (::core::mem::size_of::<int32_t>() as libc::c_ulong)
                    .wrapping_mul(
                        (32 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                    ),
            );
        }
        i += 1;
        i;
    }
    pingmeasurecount = 1 as libc::c_int as uint16_t;
}
static mut gametime: tic_t = 0 as libc::c_int as tic_t;
unsafe extern "C" fn UpdatePingTable() {
    let mut i: int32_t = 0;
    if server != 0 {
        if netgame != 0 && gametime % 35 as libc::c_int as tic_t == 0 {
            PingUpdate();
        }
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if playeringame[i as usize] != 0
                && playernode[i as usize] as libc::c_int != 255 as libc::c_int
            {
                realpingtable[i
                    as usize] = (realpingtable[i as usize])
                    .wrapping_add(
                        G_TicsToMilliseconds(GetLag(playernode[i as usize] as int32_t))
                            as uint32_t,
                    );
            }
            i += 1;
            i;
        }
        pingmeasurecount = pingmeasurecount.wrapping_add(1);
        pingmeasurecount;
    }
}
unsafe extern "C" fn HandleNodeTimeouts() {
    let mut i: int32_t = 0;
    if server != 0 {
        i = 1 as libc::c_int;
        while i < 127 as libc::c_int {
            if nodeingame[i as usize] != 0 && freezetimeout[i as usize] < I_GetTime() {
                Net_ConnectionTimeout(i);
            }
            i += 1;
            i;
        }
        if joindelay != 0 {
            joindelay = if joindelay.wrapping_sub(1 as libc::c_int as tic_t)
                < 3 as libc::c_int as tic_t * cv_joindelay.value as tic_t
                    * 35 as libc::c_int as tic_t
            {
                joindelay.wrapping_sub(1 as libc::c_int as tic_t)
            } else {
                3 as libc::c_int as tic_t * cv_joindelay.value as tic_t
                    * 35 as libc::c_int as tic_t
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn NetKeepAlive() {
    let mut nowtime: tic_t = 0;
    let mut realtics: int32_t = 0;
    nowtime = I_GetTime();
    realtics = nowtime.wrapping_sub(gametime) as int32_t;
    if realtics <= 0 as libc::c_int {
        return;
    }
    UpdatePingTable();
    GetPackets();
    MasterClient_Ticker();
    if server == 0 {
        CL_SendClientKeepAlive();
    } else {
        SV_SendServerKeepAlive();
    }
    Net_AckTicker();
    HandleNodeTimeouts();
    FileSendTicker();
}
#[no_mangle]
pub unsafe extern "C" fn NetUpdate() {
    static mut resptime: tic_t = 0 as libc::c_int as tic_t;
    let mut nowtime: tic_t = 0;
    let mut i: int32_t = 0;
    let mut realtics: int32_t = 0;
    nowtime = I_GetTime();
    realtics = nowtime.wrapping_sub(gametime) as int32_t;
    if realtics <= 0 as libc::c_int {
        return;
    }
    if realtics > 5 as libc::c_int {
        if server != 0 {
            realtics = 1 as libc::c_int;
        } else {
            realtics = 5 as libc::c_int;
        }
    }
    if server != 0 && dedicated != 0
        && gamestate as libc::c_uint == GS_LEVEL as libc::c_int as libc::c_uint
    {
        let dedicatedidletime: tic_t = (cv_dedicatedidletime.value * 35 as libc::c_int)
            as tic_t;
        static mut dedicatedidletimeprev: tic_t = 0 as libc::c_int as tic_t;
        static mut dedicatedidle: tic_t = 0 as libc::c_int as tic_t;
        if dedicatedidletime > 0 as libc::c_int as tic_t {
            i = 1 as libc::c_int;
            while i < 127 as libc::c_int {
                if nodeingame[i as usize] != 0 {
                    if dedicatedidle >= dedicatedidletime {
                        CONS_Printf(
                            b"DEDICATED: Awakening from idle (Node %d detected...)\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                        );
                        dedicatedidle = 0 as libc::c_int as tic_t;
                    }
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            if i == 127 as libc::c_int {
                if leveltime == 2 as libc::c_int as tic_t {
                    dedicatedidle = dedicatedidletime
                        .wrapping_sub(1 as libc::c_int as tic_t);
                } else if dedicatedidle >= dedicatedidletime {
                    if !(D_GetExistingTextcmd(gametic, 0 as libc::c_int)).is_null()
                        || !(D_GetExistingTextcmd(
                            gametic.wrapping_add(1 as libc::c_int as tic_t),
                            0 as libc::c_int,
                        ))
                            .is_null()
                    {
                        CONS_Printf(
                            b"DEDICATED: Awakening from idle (Netxcmd detected...)\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        dedicatedidle = 0 as libc::c_int as tic_t;
                    } else {
                        realtics = 0 as libc::c_int;
                    }
                } else {
                    dedicatedidle = dedicatedidle.wrapping_add(realtics as tic_t);
                    if dedicatedidle >= dedicatedidletime {
                        let mut idlereason: *const libc::c_char = b"at round start\0"
                            as *const u8 as *const libc::c_char;
                        if leveltime > 3 as libc::c_int as tic_t {
                            idlereason = va(
                                b"for %d seconds\0" as *const u8 as *const libc::c_char,
                                dedicatedidle / 35 as libc::c_int as tic_t,
                            );
                        }
                        CONS_Printf(
                            b"DEDICATED: No nodes %s, idling...\n\0" as *const u8
                                as *const libc::c_char,
                            idlereason,
                        );
                        realtics = 0 as libc::c_int;
                        dedicatedidle = dedicatedidletime;
                    }
                }
            }
        } else {
            if dedicatedidletimeprev > 0 as libc::c_int as tic_t
                && dedicatedidle >= dedicatedidletimeprev
            {
                CONS_Printf(
                    b"DEDICATED: Awakening from idle (Idle disabled...)\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            dedicatedidle = 0 as libc::c_int as tic_t;
        }
        dedicatedidletimeprev = dedicatedidletime;
    }
    gametime = nowtime;
    UpdatePingTable();
    if server == 0 {
        maketic = neededtic;
    }
    Local_Maketic(realtics);
    if server != 0 {
        CL_SendClientCmd();
    }
    GetPackets();
    MasterClient_Ticker();
    if server == 0 {
        if cl_redownloadinggamestate != 0
            && (*fileneeded.offset(0 as libc::c_int as isize)).status as libc::c_uint
                == FS_FOUND as libc::c_int as libc::c_uint
        {
            CL_ReloadReceivedSavegame();
        }
        CL_SendClientCmd();
        hu_redownloadinggamestate = cl_redownloadinggamestate as uint8_t;
    } else if demoplayback == 0 && realtics > 0 as libc::c_int {
        let mut counts: int32_t = 0;
        hu_redownloadinggamestate = false_0 as libc::c_int as uint8_t;
        counts = realtics;
        firstticstosend = gametic;
        i = 0 as libc::c_int;
        while i < 127 as libc::c_int {
            if !(nodeingame[i as usize] == 0) {
                if nettics[i as usize] < firstticstosend {
                    firstticstosend = nettics[i as usize];
                }
                if maketic.wrapping_add(counts as tic_t)
                    >= (nettics[i as usize])
                        .wrapping_add((1024 as libc::c_int - 35 as libc::c_int) as tic_t)
                {
                    Net_ConnectionTimeout(i);
                }
            }
            i += 1;
            i;
        }
        if maketic.wrapping_add(counts as tic_t)
            >= firstticstosend.wrapping_add(1024 as libc::c_int as tic_t)
        {
            counts = firstticstosend
                .wrapping_add(1024 as libc::c_int as tic_t)
                .wrapping_sub(maketic)
                .wrapping_sub(1 as libc::c_int as tic_t) as int32_t;
        }
        i = 0 as libc::c_int;
        while i < counts {
            SV_Maketic();
            i += 1;
            i;
        }
        while tictoclear < firstticstosend {
            D_Clearticcmd(tictoclear);
            tictoclear = tictoclear.wrapping_add(1);
            tictoclear;
        }
        SV_SendTics();
        neededtic = maketic;
    }
    Net_AckTicker();
    HandleNodeTimeouts();
    nowtime = nowtime / 1 as libc::c_int as tic_t;
    if nowtime > resptime {
        resptime = nowtime;
        M_Ticker();
        CON_Ticker();
    }
    FileSendTicker();
}
#[no_mangle]
pub unsafe extern "C" fn D_NumPlayers() -> int32_t {
    let mut num: int32_t = 0 as libc::c_int;
    let mut ix: int32_t = 0;
    ix = 0 as libc::c_int;
    while ix < 32 as libc::c_int {
        if playeringame[ix as usize] != 0 {
            num += 1;
            num;
        }
        ix += 1;
        ix;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn D_NumBots() -> int32_t {
    let mut num: int32_t = 0 as libc::c_int;
    let mut ix: int32_t = 0;
    ix = 0 as libc::c_int;
    while ix < 32 as libc::c_int {
        if playeringame[ix as usize] != 0 && players[ix as usize].bot as libc::c_int != 0
        {
            num += 1;
            num;
        }
        ix += 1;
        ix;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn GetLag(mut node: int32_t) -> tic_t {
    return gametic.wrapping_sub(nettics[node as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn D_MD5PasswordPass(
    mut buffer: *const uint8_t,
    mut len: size_t,
    mut salt: *const libc::c_char,
    mut dest: *mut libc::c_void,
) {
    let mut tmpbuf: [libc::c_char; 256] = [0; 256];
    let sl: size_t = strlen(salt);
    if len > (256 as libc::c_int as size_t).wrapping_sub(sl) {
        len = (256 as libc::c_int as size_t).wrapping_sub(sl);
    }
    memcpy(tmpbuf.as_mut_ptr() as *mut libc::c_void, buffer as *const libc::c_void, len);
    memmove(
        &mut *tmpbuf.as_mut_ptr().offset(len as isize) as *mut libc::c_char
            as *mut libc::c_void,
        salt as *const libc::c_void,
        sl,
    );
    len = (len as libc::c_ulong).wrapping_add(strlen(salt)) as size_t as size_t;
    if len < 256 as libc::c_int as size_t {
        memset(
            &mut *tmpbuf.as_mut_ptr().offset(len as isize) as *mut libc::c_char
                as *mut libc::c_void,
            0 as libc::c_int,
            (256 as libc::c_int as size_t).wrapping_sub(len),
        );
    }
    md5_buffer(tmpbuf.as_mut_ptr(), 256 as libc::c_int as size_t, dest);
}
