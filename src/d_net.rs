use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
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
    static mut netgame: boolean;
    static mut addedtogame: boolean;
    static mut multiplayer: boolean;
    static mut consoleplayer: int32_t;
    fn ExpandTics(low: int32_t, node: int32_t) -> tic_t;
    static mut server: boolean;
    fn D_ClientServerInit();
    static mut software_MAXPACKETLENGTH: uint16_t;
    fn D_ResetTiccmds();
    static mut playerpingtable: [uint32_t; 32];
    static mut playeringame: [boolean; 32];
    static mut player_names: [[libc::c_char; 22]; 32];
    static mut netxcmdnames: [*const libc::c_char; 24];
    static mut cv_sleep: consvar_t;
    static mut cv_timescale: consvar_t;
    fn I_GetTime() -> tic_t;
    fn I_UpdateTime(timescale: fixed_t);
    fn I_InitNetwork() -> boolean;
    fn I_Sleep(ms: uint32_t);
    fn M_CheckParm(check: *const libc::c_char) -> int32_t;
    fn M_IsNextParm() -> boolean;
    fn M_GetNextParm() -> *const libc::c_char;
    fn SV_AbortLuaFileTransfer(node: int32_t);
    fn SV_AbortSendFiles(node: int32_t);
    fn Z_CallocAlign(
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn I_InitTcpNetwork() -> boolean;
    static mut srb2home: [libc::c_char; 256];
}
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
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
pub type tic_t = uint32_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type fixed_t = int32_t;
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
pub struct ackpak_t {
    pub acknum: uint8_t,
    pub nextacknum: uint8_t,
    pub destinationnode: uint8_t,
    pub senttime: tic_t,
    pub length: uint16_t,
    pub resentnum: uint16_t,
    pub pak: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub raw: [int8_t; 1450],
    pub data: doomdata_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct doomdata_t {
    pub checksum: uint32_t,
    pub ack: uint8_t,
    pub ackreturn: uint8_t,
    pub packettype: uint8_t,
    pub reserved: uint8_t,
    pub u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
#[repr(C, packed)]
pub struct filesneededconfig_pak {
    pub first: int32_t,
    pub num: uint8_t,
    pub more: uint8_t,
    pub files: [uint8_t; 915],
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
pub struct msaskinfo_pak {
    pub clientaddr: [libc::c_char; 22],
    pub time: tic_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct askinfo_pak {
    pub version: uint8_t,
    pub time: tic_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct serverrefuse_pak {
    pub reason: [libc::c_char; 255],
}
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
pub struct clientconfig_pak {
    pub modversion: uint8_t,
    pub application: [libc::c_char; 16],
    pub localplayers: uint8_t,
    pub mode: uint8_t,
    pub names: [[libc::c_char; 21]; 2],
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
pub struct fileacksegment_t {
    pub start: uint32_t,
    pub acks: uint32_t,
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
pub struct servertics_pak {
    pub starttic: tic_t,
    pub numtics: uint8_t,
    pub numslots: uint8_t,
    pub cmds: [ticcmd_t; 45],
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
pub struct clientcmd_pak {
    pub client_tic: uint8_t,
    pub resendfrom: uint8_t,
    pub consistancy: int16_t,
    pub cmd: ticcmd_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct node_t {
    pub firstacktosend: uint8_t,
    pub acktosend_head: uint8_t,
    pub acktosend_tail: uint8_t,
    pub acktosend: [uint8_t; 96],
    pub lasttimeacktosend_sent: tic_t,
    pub lasttimepacketreceived: tic_t,
    pub remotefirstack: uint8_t,
    pub nextacknum: uint8_t,
    pub flags: uint8_t,
}
pub const PT_NODETIMEOUT: C2RustUnnamed_4 = 30;
pub const NF_TIMEOUT: C2RustUnnamed_6 = 2;
pub const NF_CLOSE: C2RustUnnamed_6 = 1;
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
pub const PT_REQUESTFILE: C2RustUnnamed_4 = 15;
pub const PT_FILEFRAGMENT: C2RustUnnamed_4 = 24;
pub const PT_SERVERREFUSE: C2RustUnnamed_4 = 9;
pub const PT_SERVERINFO: C2RustUnnamed_4 = 13;
pub const PT_SERVERCFG: C2RustUnnamed_4 = 1;
pub const PT_TEXTCMD2: C2RustUnnamed_4 = 28;
pub const PT_TEXTCMD: C2RustUnnamed_4 = 27;
pub const PT_BASICKEEPALIVE: C2RustUnnamed_4 = 23;
pub const PT_NODEKEEPALIVEMIS: C2RustUnnamed_4 = 7;
pub const PT_NODEKEEPALIVE: C2RustUnnamed_4 = 6;
pub const PT_CLIENT2MIS: C2RustUnnamed_4 = 5;
pub const PT_CLIENTMIS: C2RustUnnamed_4 = 3;
pub const PT_CLIENT2CMD: C2RustUnnamed_4 = 4;
pub const PT_CLIENTCMD: C2RustUnnamed_4 = 2;
pub const PT_SERVERTICS: C2RustUnnamed_4 = 8;
pub const PT_CLIENTJOIN: C2RustUnnamed_4 = 29;
pub const PT_ASKINFOVIAMS: C2RustUnnamed_4 = 16;
pub const PT_ASKINFO: C2RustUnnamed_4 = 12;
pub const PT_CANFAIL: C2RustUnnamed_4 = 24;
pub const PT_NOTHING: C2RustUnnamed_4 = 0;
pub const PU_STATIC: C2RustUnnamed_5 = 1;
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
pub type CV_PossibleValue_t = CV_PossibleValue_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CV_PossibleValue_s {
    pub value: int32_t,
    pub strvalue: *const libc::c_char,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NUMPACKETTYPE: C2RustUnnamed_4 = 35;
pub const PT_PING: C2RustUnnamed_4 = 34;
pub const PT_MOREFILESNEEDED: C2RustUnnamed_4 = 33;
pub const PT_TELLFILESNEEDED: C2RustUnnamed_4 = 32;
pub const PT_LOGIN: C2RustUnnamed_4 = 31;
pub const PT_FILERECEIVED: C2RustUnnamed_4 = 26;
pub const PT_FILEACK: C2RustUnnamed_4 = 25;
pub const PT_HASLUAFILE: C2RustUnnamed_4 = 22;
pub const PT_ASKLUAFILE: C2RustUnnamed_4 = 21;
pub const PT_SENDINGLUAFILE: C2RustUnnamed_4 = 20;
pub const PT_RECEIVEDGAMESTATE: C2RustUnnamed_4 = 19;
pub const PT_CANRECEIVEGAMESTATE: C2RustUnnamed_4 = 18;
pub const PT_WILLRESENDGAMESTATE: C2RustUnnamed_4 = 17;
pub const PT_PLAYERINFO: C2RustUnnamed_4 = 14;
pub const PT_CLIENTQUIT: C2RustUnnamed_4 = 11;
pub const PT_SERVERSHUTDOWN: C2RustUnnamed_4 = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pingcell {
    pub num: int32_t,
    pub ms: int32_t,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_5 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_5 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_5 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_5 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_5 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_5 = 51;
pub const PU_LEVEL: C2RustUnnamed_5 = 50;
pub const PU_CACHE: C2RustUnnamed_5 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_5 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_5 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_5 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_5 = 21;
pub const PU_HUDGFX: C2RustUnnamed_5 = 19;
pub const PU_SPRITE: C2RustUnnamed_5 = 18;
pub const PU_PATCH_DATA: C2RustUnnamed_5 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_5 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_5 = 15;
pub const PU_PATCH: C2RustUnnamed_5 = 14;
pub const PU_MUSIC: C2RustUnnamed_5 = 12;
pub const PU_SOUND: C2RustUnnamed_5 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_5 = 3;
pub const PU_LUA: C2RustUnnamed_5 = 2;
pub type C2RustUnnamed_6 = libc::c_uint;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut connectiontimeout: tic_t = (10 as libc::c_int * 35 as libc::c_int)
    as tic_t;
#[no_mangle]
pub static mut doomcom: *mut doomcom_t = 0 as *const doomcom_t as *mut doomcom_t;
#[no_mangle]
pub static mut netbuffer: *mut doomdata_t = 0 as *const doomdata_t as *mut doomdata_t;
#[no_mangle]
pub static mut debugfile: *mut FILE = 0 as *const FILE as *mut FILE;
static mut reboundstore: [doomdata_t; 8] = [doomdata_t {
    checksum: 0,
    ack: 0,
    ackreturn: 0,
    packettype: 0,
    reserved: 0,
    u: C2RustUnnamed_1 {
        clientpak: clientcmd_pak {
            client_tic: 0,
            resendfrom: 0,
            consistancy: 0,
            cmd: ticcmd_t {
                forwardmove: 0,
                sidemove: 0,
                angleturn: 0,
                aiming: 0,
                buttons: 0,
                latency: 0,
            },
        },
    },
}; 8];
static mut reboundsize: [int16_t; 8] = [0; 8];
static mut rebound_head: int32_t = 0;
static mut rebound_tail: int32_t = 0;
#[no_mangle]
pub static mut net_bandwidth: int32_t = 0;
#[no_mangle]
pub static mut hardware_MAXPACKETLENGTH: int16_t = 0;
#[no_mangle]
pub static mut I_NetGet: Option::<unsafe extern "C" fn() -> boolean> = None;
#[no_mangle]
pub static mut I_NetSend: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut I_NetCanSend: Option::<unsafe extern "C" fn() -> boolean> = None;
#[no_mangle]
pub static mut I_NetCanGet: Option::<unsafe extern "C" fn() -> boolean> = None;
#[no_mangle]
pub static mut I_NetCloseSocket: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut I_NetFreeNodenum: Option::<unsafe extern "C" fn(int32_t) -> ()> = None;
#[no_mangle]
pub static mut I_NetMakeNodewPort: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> int8_t,
> = None;
#[no_mangle]
pub static mut I_NetOpenSocket: Option::<unsafe extern "C" fn() -> boolean> = None;
#[no_mangle]
pub static mut I_Ban: Option::<unsafe extern "C" fn(int32_t) -> boolean> = None;
#[no_mangle]
pub static mut I_ClearBans: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut I_GetNodeAddress: Option::<
    unsafe extern "C" fn(int32_t) -> *const libc::c_char,
> = None;
#[no_mangle]
pub static mut I_GetBanAddress: Option::<
    unsafe extern "C" fn(size_t) -> *const libc::c_char,
> = None;
#[no_mangle]
pub static mut I_GetBanMask: Option::<
    unsafe extern "C" fn(size_t) -> *const libc::c_char,
> = None;
#[no_mangle]
pub static mut I_SetBanAddress: Option::<
    unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> boolean,
> = None;
#[no_mangle]
pub static mut bannednode: *mut boolean = 0 as *const boolean as *mut boolean;
static mut statstarttic: tic_t = 0;
#[no_mangle]
pub static mut getbytes: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut sendbytes: int64_t = 0 as libc::c_int as int64_t;
static mut retransmit: int32_t = 0 as libc::c_int;
static mut duppacket: int32_t = 0 as libc::c_int;
static mut sendackpacket: int32_t = 0 as libc::c_int;
static mut getackpacket: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut ticruned: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut ticmiss: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut getbps: int32_t = 0;
#[no_mangle]
pub static mut sendbps: int32_t = 0;
#[no_mangle]
pub static mut lostpercent: libc::c_float = 0.;
#[no_mangle]
pub static mut duppercent: libc::c_float = 0.;
#[no_mangle]
pub static mut gamelostpercent: libc::c_float = 0.;
#[no_mangle]
pub static mut packetheaderlength: int32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn Net_GetNetStat() -> boolean {
    let t: tic_t = I_GetTime();
    static mut oldsendbyte: int64_t = 0 as libc::c_int as int64_t;
    if statstarttic.wrapping_add((35 as libc::c_int * 2 as libc::c_int) as tic_t) <= t {
        let df: tic_t = t.wrapping_sub(statstarttic);
        let newsendbyte: int64_t = sendbytes - oldsendbyte;
        sendbps = ((newsendbyte * 35 as libc::c_int as int64_t) as int32_t as tic_t / df)
            as int32_t;
        getbps = ((getbytes * 35 as libc::c_int) as tic_t / df) as int32_t;
        if sendackpacket != 0 {
            lostpercent = 100.0f32 * retransmit as libc::c_float
                / sendackpacket as libc::c_float;
        } else {
            lostpercent = 0.0f32;
        }
        if getackpacket != 0 {
            duppercent = 100.0f32 * duppacket as libc::c_float
                / getackpacket as libc::c_float;
        } else {
            duppercent = 0.0f32;
        }
        if ticruned != 0 {
            gamelostpercent = 100.0f32 * ticmiss as libc::c_float
                / ticruned as libc::c_float;
        } else {
            gamelostpercent = 0.0f32;
        }
        ticruned = 0 as libc::c_int;
        ticmiss = ticruned;
        oldsendbyte = sendbytes;
        getbytes = 0 as libc::c_int;
        retransmit = 0 as libc::c_int;
        duppacket = retransmit;
        getackpacket = duppacket;
        sendackpacket = getackpacket;
        statstarttic = t;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
static mut ackpak: [ackpak_t; 96] = [ackpak_t {
    acknum: 0,
    nextacknum: 0,
    destinationnode: 0,
    senttime: 0,
    length: 0,
    resentnum: 0,
    pak: C2RustUnnamed_0 { raw: [0; 1450] },
}; 96];
static mut nodes: [node_t; 127] = [node_t {
    firstacktosend: 0,
    acktosend_head: 0,
    acktosend_tail: 0,
    acktosend: [0; 96],
    lasttimeacktosend_sent: 0,
    lasttimepacketreceived: 0,
    remotefirstack: 0,
    nextacknum: 0,
    flags: 0,
}; 127];
unsafe extern "C" fn cmpack(mut a: uint8_t, mut b: uint8_t) -> int32_t {
    let mut d: int32_t = a as libc::c_int - b as libc::c_int;
    if d >= 127 as libc::c_int || d < -(128 as libc::c_int) {
        return -d;
    }
    return d;
}
unsafe extern "C" fn GetFreeAcknum(
    mut freeack: *mut uint8_t,
    mut lowtimer: boolean,
) -> boolean {
    let mut node: *mut node_t = &mut *nodes
        .as_mut_ptr()
        .offset((*doomcom).remotenode as isize) as *mut node_t;
    let mut i: int32_t = 0;
    let mut numfreeslot: int32_t = 0 as libc::c_int;
    if cmpack(
        (((*node).remotefirstack as libc::c_int + 96 as libc::c_int)
            % 256 as libc::c_int) as uint8_t,
        (*node).nextacknum,
    ) < 0 as libc::c_int
    {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"too fast %d %d\n\0" as *const u8 as *const libc::c_char,
                    (*node).remotefirstack as libc::c_int,
                    (*node).nextacknum as libc::c_int,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        return false_0 as libc::c_int;
    }
    let mut current_block_28: u64;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        if ackpak[i as usize].acknum == 0 {
            if (*netbuffer).packettype as libc::c_int >= PT_CANFAIL as libc::c_int {
                numfreeslot += 1;
                numfreeslot;
                if numfreeslot <= 10 as libc::c_int {
                    current_block_28 = 6937071982253665452;
                } else {
                    current_block_28 = 5399440093318478209;
                }
            } else {
                current_block_28 = 5399440093318478209;
            }
            match current_block_28 {
                6937071982253665452 => {}
                _ => {
                    ackpak[i as usize].acknum = (*node).nextacknum;
                    ackpak[i as usize].nextacknum = (*node).nextacknum;
                    (*node).nextacknum = ((*node).nextacknum).wrapping_add(1);
                    (*node).nextacknum;
                    if (*node).nextacknum == 0 {
                        (*node).nextacknum = ((*node).nextacknum).wrapping_add(1);
                        (*node).nextacknum;
                    }
                    ackpak[i as usize]
                        .destinationnode = node.offset_from(nodes.as_mut_ptr())
                        as libc::c_long as uint8_t;
                    ackpak[i as usize].length = (*doomcom).datalength as uint16_t;
                    if lowtimer != 0 {
                        ackpak[i as usize].senttime = 0 as libc::c_int as tic_t;
                        ackpak[i as usize].resentnum = 1 as libc::c_int as uint16_t;
                    } else {
                        ackpak[i as usize].senttime = I_GetTime();
                        ackpak[i as usize].resentnum = 0 as libc::c_int as uint16_t;
                    }
                    M_Memcpy
                        .expect(
                            "non-null function pointer",
                        )(
                        (ackpak[i as usize].pak.raw).as_mut_ptr() as *mut libc::c_void,
                        netbuffer as *const libc::c_void,
                        ackpak[i as usize].length as size_t,
                    );
                    *freeack = ackpak[i as usize].acknum;
                    sendackpacket += 1;
                    sendackpacket;
                    return true_0 as libc::c_int;
                }
            }
        }
        i += 1;
        i;
    }
    if ((*netbuffer).packettype as libc::c_int) < PT_CANFAIL as libc::c_int {
        I_Error(b"Connection lost\n\0" as *const u8 as *const libc::c_char);
    }
    return false_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Net_GetFreeAcks(mut urgent: boolean) -> int32_t {
    let mut i: int32_t = 0;
    let mut numfreeslot: int32_t = 0 as libc::c_int;
    let mut n: int32_t = 0 as libc::c_int;
    let mut current_block_2: u64;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        if ackpak[i as usize].acknum == 0 {
            if urgent == 0 {
                numfreeslot += 1;
                numfreeslot;
                if numfreeslot <= 10 as libc::c_int {
                    current_block_2 = 16668937799742929182;
                } else {
                    current_block_2 = 6873731126896040597;
                }
            } else {
                current_block_2 = 6873731126896040597;
            }
            match current_block_2 {
                16668937799742929182 => {}
                _ => {
                    n += 1;
                    n;
                }
            }
        }
        i += 1;
        i;
    }
    return n;
}
unsafe extern "C" fn GetAcktosend(mut node: int32_t) -> uint8_t {
    nodes[node as usize].lasttimeacktosend_sent = I_GetTime();
    return nodes[node as usize].firstacktosend;
}
unsafe extern "C" fn RemoveAck(mut i: int32_t) {
    let mut node: int32_t = ackpak[i as usize].destinationnode as int32_t;
    if !debugfile.is_null() {
        fputs(
            va(
                b"Remove ack %d\n\0" as *const u8 as *const libc::c_char,
                ackpak[i as usize].acknum as libc::c_int,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    ackpak[i as usize].acknum = 0 as libc::c_int as uint8_t;
    if nodes[node as usize].flags as libc::c_int & NF_CLOSE as libc::c_int != 0 {
        Net_CloseConnection(node);
    }
}
unsafe extern "C" fn Processackpak() -> boolean {
    let mut i: int32_t = 0;
    let mut goodpacket: boolean = true_0 as libc::c_int;
    let mut node: *mut node_t = &mut *nodes
        .as_mut_ptr()
        .offset((*doomcom).remotenode as isize) as *mut node_t;
    if (*netbuffer).ackreturn as libc::c_int != 0
        && cmpack((*node).remotefirstack, (*netbuffer).ackreturn) < 0 as libc::c_int
    {
        (*node).remotefirstack = (*netbuffer).ackreturn;
        i = 0 as libc::c_int;
        while i < 96 as libc::c_int {
            if ackpak[i as usize].acknum as libc::c_int != 0
                && ackpak[i as usize].destinationnode as libc::c_long
                    == node.offset_from(nodes.as_mut_ptr()) as libc::c_long
                && cmpack(ackpak[i as usize].acknum, (*netbuffer).ackreturn)
                    <= 0 as libc::c_int
            {
                RemoveAck(i);
            }
            i += 1;
            i;
        }
    }
    if (*netbuffer).ack != 0 {
        let mut ack: uint8_t = (*netbuffer).ack;
        getackpacket += 1;
        getackpacket;
        if cmpack(ack, (*node).firstacktosend) <= 0 as libc::c_int {
            if !debugfile.is_null() {
                fputs(
                    va(
                        b"Discard(1) ack %d (duplicated)\n\0" as *const u8
                            as *const libc::c_char,
                        ack as libc::c_int,
                    ),
                    debugfile,
                );
                fflush(debugfile);
            }
            duppacket += 1;
            duppacket;
            goodpacket = false_0 as libc::c_int;
        } else {
            i = (*node).acktosend_tail as int32_t;
            while i != (*node).acktosend_head as libc::c_int {
                if (*node).acktosend[i as usize] as libc::c_int == ack as libc::c_int {
                    if !debugfile.is_null() {
                        fputs(
                            va(
                                b"Discard(2) ack %d (duplicated)\n\0" as *const u8
                                    as *const libc::c_char,
                                ack as libc::c_int,
                            ),
                            debugfile,
                        );
                        fflush(debugfile);
                    }
                    duppacket += 1;
                    duppacket;
                    goodpacket = false_0 as libc::c_int;
                    break;
                } else {
                    i = (i + 1 as libc::c_int) % 96 as libc::c_int;
                }
            }
            if goodpacket != 0 {
                let mut nextfirstack: uint8_t = ((*node).firstacktosend as libc::c_int
                    + 1 as libc::c_int) as uint8_t;
                if nextfirstack == 0 {
                    nextfirstack = 1 as libc::c_int as uint8_t;
                }
                if ack as libc::c_int == nextfirstack as libc::c_int {
                    let mut hm1: uint8_t = 0;
                    let mut change: boolean = true_0 as libc::c_int;
                    let fresh0 = nextfirstack;
                    nextfirstack = nextfirstack.wrapping_add(1);
                    (*node).firstacktosend = fresh0;
                    if nextfirstack == 0 {
                        nextfirstack = 1 as libc::c_int as uint8_t;
                    }
                    hm1 = (((*node).acktosend_head as libc::c_int - 1 as libc::c_int
                        + 96 as libc::c_int) % 96 as libc::c_int) as uint8_t;
                    while change != 0 {
                        change = false_0 as libc::c_int;
                        i = (*node).acktosend_tail as int32_t;
                        while i != (*node).acktosend_head as libc::c_int {
                            if cmpack((*node).acktosend[i as usize], nextfirstack)
                                <= 0 as libc::c_int
                            {
                                if (*node).acktosend[i as usize] as libc::c_int
                                    == nextfirstack as libc::c_int
                                {
                                    let fresh1 = nextfirstack;
                                    nextfirstack = nextfirstack.wrapping_add(1);
                                    (*node).firstacktosend = fresh1;
                                    if nextfirstack == 0 {
                                        nextfirstack = 1 as libc::c_int as uint8_t;
                                    }
                                    change = true_0 as libc::c_int;
                                }
                                if i == (*node).acktosend_tail as libc::c_int {
                                    (*node)
                                        .acktosend[(*node).acktosend_tail
                                        as usize] = 0 as libc::c_int as uint8_t;
                                    (*node)
                                        .acktosend_tail = ((i + 1 as libc::c_int)
                                        % 96 as libc::c_int) as uint8_t;
                                } else if i == hm1 as libc::c_int {
                                    (*node)
                                        .acktosend[hm1 as usize] = 0 as libc::c_int as uint8_t;
                                    (*node).acktosend_head = hm1;
                                    hm1 = ((hm1 as libc::c_int - 1 as libc::c_int
                                        + 96 as libc::c_int) % 96 as libc::c_int) as uint8_t;
                                }
                            }
                            i = (i + 1 as libc::c_int) % 96 as libc::c_int;
                        }
                    }
                } else {
                    let mut newhead: uint8_t = (((*node).acktosend_head as libc::c_int
                        + 1 as libc::c_int) % 96 as libc::c_int) as uint8_t;
                    if !debugfile.is_null() {
                        fputs(
                            va(
                                b"out of order packet (%d expected)\n\0" as *const u8
                                    as *const libc::c_char,
                                nextfirstack as libc::c_int,
                            ),
                            debugfile,
                        );
                        fflush(debugfile);
                    }
                    if newhead as libc::c_int != (*node).acktosend_tail as libc::c_int {
                        (*node).acktosend[(*node).acktosend_head as usize] = ack;
                        (*node).acktosend_head = newhead;
                    } else {
                        if !debugfile.is_null() {
                            fputs(
                                b"no more freeackret\n\0" as *const u8
                                    as *const libc::c_char,
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                        goodpacket = false_0 as libc::c_int;
                    }
                }
            }
        }
    }
    return goodpacket;
}
#[no_mangle]
pub unsafe extern "C" fn Net_SendAcks(mut node: int32_t) {
    (*netbuffer).packettype = PT_NOTHING as libc::c_int as uint8_t;
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        ((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_void,
        (nodes[node as usize].acktosend).as_mut_ptr() as *const libc::c_void,
        96 as libc::c_int as size_t,
    );
    HSendPacket(
        node,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        96 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn GotAcks() {
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    j = 0 as libc::c_int;
    while j < 96 as libc::c_int {
        if (*netbuffer).u.textcmd[j as usize] != 0 {
            i = 0 as libc::c_int;
            while i < 96 as libc::c_int {
                if ackpak[i as usize].acknum as libc::c_int != 0
                    && ackpak[i as usize].destinationnode as libc::c_int
                        == (*doomcom).remotenode as libc::c_int
                {
                    if ackpak[i as usize].acknum as libc::c_int
                        == (*netbuffer).u.textcmd[j as usize] as libc::c_int
                    {
                        RemoveAck(i);
                    } else if cmpack(
                        ackpak[i as usize].nextacknum,
                        (*netbuffer).u.textcmd[j as usize],
                    ) <= 0 as libc::c_int
                        && ackpak[i as usize].senttime > 0 as libc::c_int as tic_t
                    {
                        ackpak[i as usize]
                            .senttime = (ackpak[i as usize].senttime).wrapping_sub(1);
                        ackpak[i as usize].senttime;
                    }
                }
                i += 1;
                i;
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Net_ConnectionTimeout(mut node: int32_t) {
    if nodes[node as usize].flags as libc::c_int & NF_TIMEOUT as libc::c_int != 0 {
        return;
    }
    nodes[node as usize]
        .flags = (nodes[node as usize].flags as libc::c_int | NF_TIMEOUT as libc::c_int)
        as uint8_t;
    reboundstore[rebound_head as usize]
        .packettype = PT_NODETIMEOUT as libc::c_int as uint8_t;
    reboundstore[rebound_head as usize].ack = 0 as libc::c_int as uint8_t;
    reboundstore[rebound_head as usize].ackreturn = 0 as libc::c_int as uint8_t;
    reboundstore[rebound_head as usize]
        .u
        .textcmd[0 as libc::c_int as usize] = node as uint8_t;
    reboundsize[rebound_head
        as usize] = (8 as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as int16_t;
    rebound_head = (rebound_head + 1 as libc::c_int) % 8 as libc::c_int;
    nodes[node as usize].lasttimepacketreceived = I_GetTime();
}
#[no_mangle]
pub unsafe extern "C" fn Net_AckTicker() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        let nodei: int32_t = ackpak[i as usize].destinationnode as int32_t;
        let mut node: *mut node_t = &mut *nodes.as_mut_ptr().offset(nodei as isize)
            as *mut node_t;
        if ackpak[i as usize].acknum as libc::c_int != 0
            && (ackpak[i as usize].senttime).wrapping_add(14 as libc::c_int as tic_t)
                < I_GetTime()
        {
            if ackpak[i as usize].resentnum as libc::c_int > 20 as libc::c_int
                && (*node).flags as libc::c_int & NF_CLOSE as libc::c_int != 0
            {
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"ack %d sent 20 times so connection is supposed lost: node %d\n\0"
                                as *const u8 as *const libc::c_char,
                            i,
                            nodei,
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                Net_CloseConnection(nodei | 0x8000 as libc::c_int);
                ackpak[i as usize].acknum = 0 as libc::c_int as uint8_t;
            } else {
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"Resend ack %d, %u<%d at %u\n\0" as *const u8
                                as *const libc::c_char,
                            ackpak[i as usize].acknum as libc::c_int,
                            ackpak[i as usize].senttime,
                            14 as libc::c_int,
                            I_GetTime(),
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    netbuffer as *mut libc::c_void,
                    (ackpak[i as usize].pak.raw).as_mut_ptr() as *const libc::c_void,
                    ackpak[i as usize].length as size_t,
                );
                ackpak[i as usize].senttime = I_GetTime();
                ackpak[i as usize]
                    .resentnum = (ackpak[i as usize].resentnum).wrapping_add(1);
                ackpak[i as usize].resentnum;
                ackpak[i as usize].nextacknum = (*node).nextacknum;
                retransmit += 1;
                retransmit;
                HSendPacket(
                    node.offset_from(nodes.as_mut_ptr()) as libc::c_long as int32_t,
                    false_0 as libc::c_int,
                    ackpak[i as usize].acknum,
                    (ackpak[i as usize].length as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_ulong),
                );
            }
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < 127 as libc::c_int {
        if nodes[i as usize].firstacktosend != 0 {
            if (nodes[i as usize].lasttimeacktosend_sent)
                .wrapping_add((35 as libc::c_int / 11 as libc::c_int) as tic_t)
                < I_GetTime()
            {
                Net_SendAcks(i);
            }
            if nodes[i as usize].flags as libc::c_int & NF_CLOSE as libc::c_int == 0
                && (nodes[i as usize].lasttimepacketreceived)
                    .wrapping_add(connectiontimeout) < I_GetTime()
            {
                Net_ConnectionTimeout(i);
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Net_UnAcknowledgePacket(mut node: int32_t) {
    let mut hm1: int32_t = (nodes[node as usize].acktosend_head as libc::c_int
        - 1 as libc::c_int + 96 as libc::c_int) % 96 as libc::c_int;
    if !debugfile.is_null() {
        fputs(
            va(b"UnAcknowledge node %d\n\0" as *const u8 as *const libc::c_char, node),
            debugfile,
        );
        fflush(debugfile);
    }
    if node == 0 {
        return;
    }
    if nodes[node as usize].acktosend[hm1 as usize] as libc::c_int
        == (*netbuffer).ack as libc::c_int
    {
        nodes[node as usize].acktosend[hm1 as usize] = 0 as libc::c_int as uint8_t;
        nodes[node as usize].acktosend_head = hm1 as uint8_t;
    } else if nodes[node as usize].firstacktosend as libc::c_int
        == (*netbuffer).ack as libc::c_int
    {
        nodes[node as usize]
            .firstacktosend = (nodes[node as usize].firstacktosend).wrapping_sub(1);
        nodes[node as usize].firstacktosend;
        if nodes[node as usize].firstacktosend == 0 {
            nodes[node as usize].firstacktosend = 255 as libc::c_int as uint8_t;
        }
    } else {
        while nodes[node as usize].firstacktosend as libc::c_int
            != (*netbuffer).ack as libc::c_int
        {
            nodes[node as usize]
                .acktosend_tail = ((nodes[node as usize].acktosend_tail as libc::c_int
                - 1 as libc::c_int + 96 as libc::c_int) % 96 as libc::c_int) as uint8_t;
            nodes[node as usize]
                .acktosend[nodes[node as usize].acktosend_tail
                as usize] = nodes[node as usize].firstacktosend;
            nodes[node as usize]
                .firstacktosend = (nodes[node as usize].firstacktosend).wrapping_sub(1);
            nodes[node as usize].firstacktosend;
            if nodes[node as usize].firstacktosend == 0 {
                nodes[node as usize].firstacktosend = 255 as libc::c_int as uint8_t;
            }
        }
        nodes[node as usize]
            .firstacktosend = (nodes[node as usize].firstacktosend).wrapping_add(1);
        nodes[node as usize].firstacktosend;
        if nodes[node as usize].firstacktosend == 0 {
            nodes[node as usize].firstacktosend = 1 as libc::c_int as uint8_t;
        }
    };
}
unsafe extern "C" fn Net_AllAcksReceived() -> boolean {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        if ackpak[i as usize].acknum != 0 {
            return false_0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Net_WaitAllAckReceived(mut timeout: uint32_t) {
    let mut tictac: tic_t = I_GetTime();
    timeout = tictac
        .wrapping_add(timeout * (35 as libc::c_int * 1 as libc::c_int) as uint32_t);
    HGetPacket();
    while timeout > I_GetTime() && Net_AllAcksReceived() == 0 {
        while tictac == I_GetTime() {
            I_Sleep(cv_sleep.value as uint32_t);
            I_UpdateTime(cv_timescale.value);
        }
        tictac = I_GetTime();
        HGetPacket();
        Net_AckTicker();
    }
}
unsafe extern "C" fn InitNode(mut node: *mut node_t) {
    (*node).acktosend_tail = 0 as libc::c_int as uint8_t;
    (*node).acktosend_head = (*node).acktosend_tail;
    (*node).firstacktosend = 0 as libc::c_int as uint8_t;
    (*node).nextacknum = 1 as libc::c_int as uint8_t;
    (*node).remotefirstack = 0 as libc::c_int as uint8_t;
    (*node).flags = 0 as libc::c_int as uint8_t;
}
unsafe extern "C" fn InitAck() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        ackpak[i as usize].acknum = 0 as libc::c_int as uint8_t;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        InitNode(&mut *nodes.as_mut_ptr().offset(i as isize));
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Net_AbortPacketType(mut packettype: uint8_t) {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        if ackpak[i as usize].acknum as libc::c_int != 0
            && (ackpak[i as usize].pak.data.packettype as libc::c_int
                == packettype as libc::c_int
                || packettype as libc::c_int == 255 as libc::c_int)
        {
            ackpak[i as usize].acknum = 0 as libc::c_int as uint8_t;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Net_CloseConnection(mut node: int32_t) {
    let mut i: int32_t = 0;
    let mut forceclose: boolean = (node & 0x8000 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
    if node == -(1 as libc::c_int) {
        if !debugfile.is_null() {
            fputs(
                b"Net_CloseConnection: node -1 detected!\n\0" as *const u8
                    as *const libc::c_char,
                debugfile,
            );
            fflush(debugfile);
        }
        return;
    }
    node &= !(0x8000 as libc::c_int);
    if node == 0 {
        return;
    }
    if node < 0 as libc::c_int || node >= 127 as libc::c_int {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"Net_CloseConnection: invalid node %d detected!\n\0" as *const u8
                        as *const libc::c_char,
                    node,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        return;
    }
    nodes[node as usize]
        .flags = (nodes[node as usize].flags as libc::c_int | NF_CLOSE as libc::c_int)
        as uint8_t;
    if GetAcktosend(node) != 0 {
        Net_SendAcks(node);
        Net_SendAcks(node);
    }
    i = 0 as libc::c_int;
    while i < 96 as libc::c_int {
        if ackpak[i as usize].acknum as libc::c_int != 0
            && ackpak[i as usize].destinationnode as libc::c_int == node
        {
            if forceclose == 0 {
                return
            } else {
                ackpak[i as usize].acknum = 0 as libc::c_int as uint8_t;
            }
        }
        i += 1;
        i;
    }
    InitNode(&mut *nodes.as_mut_ptr().offset(node as isize));
    SV_AbortSendFiles(node);
    if server != 0 {
        SV_AbortLuaFileTransfer(node);
    }
    I_NetFreeNodenum.expect("non-null function pointer")(node);
}
unsafe extern "C" fn NetbufferChecksum() -> uint32_t {
    let mut c: uint32_t = 0x1234567 as libc::c_int as uint32_t;
    let l: int32_t = (*doomcom).datalength as libc::c_int - 4 as libc::c_int;
    let mut buf: *const uint8_t = (netbuffer as *mut uint8_t)
        .offset(4 as libc::c_int as isize);
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < l {
        c = c.wrapping_add((*buf as libc::c_int * (i + 1 as libc::c_int)) as uint32_t);
        i += 1;
        i;
        buf = buf.offset(1);
        buf;
    }
    return c as int32_t as uint32_t;
}
unsafe extern "C" fn fprintfstring(mut s: *mut libc::c_char, mut len: size_t) {
    let mut mode: int32_t = 0 as libc::c_int;
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if (*s.offset(i as isize) as libc::c_int) < 32 as libc::c_int {
            if mode == 0 {
                fprintf(
                    debugfile,
                    b"[%d\0" as *const u8 as *const libc::c_char,
                    *s.offset(i as isize) as uint8_t as libc::c_int,
                );
                mode = 1 as libc::c_int;
            } else {
                fprintf(
                    debugfile,
                    b",%d\0" as *const u8 as *const libc::c_char,
                    *s.offset(i as isize) as uint8_t as libc::c_int,
                );
            }
        } else {
            if mode != 0 {
                fprintf(debugfile, b"]\0" as *const u8 as *const libc::c_char);
                mode = 0 as libc::c_int;
            }
            fprintf(
                debugfile,
                b"%c\0" as *const u8 as *const libc::c_char,
                *s.offset(i as isize) as libc::c_int,
            );
        }
        i = i.wrapping_add(1);
        i;
    }
    if mode != 0 {
        fprintf(debugfile, b"]\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn fprintfstringnewline(mut s: *mut libc::c_char, mut len: size_t) {
    fprintfstring(s, len);
    fprintf(debugfile, b"\n\0" as *const u8 as *const libc::c_char);
}
static mut packettypename: [*const libc::c_char; 35] = [
    b"NOTHING\0" as *const u8 as *const libc::c_char,
    b"SERVERCFG\0" as *const u8 as *const libc::c_char,
    b"CLIENTCMD\0" as *const u8 as *const libc::c_char,
    b"CLIENTMIS\0" as *const u8 as *const libc::c_char,
    b"CLIENT2CMD\0" as *const u8 as *const libc::c_char,
    b"CLIENT2MIS\0" as *const u8 as *const libc::c_char,
    b"NODEKEEPALIVE\0" as *const u8 as *const libc::c_char,
    b"NODEKEEPALIVEMIS\0" as *const u8 as *const libc::c_char,
    b"SERVERTICS\0" as *const u8 as *const libc::c_char,
    b"SERVERREFUSE\0" as *const u8 as *const libc::c_char,
    b"SERVERSHUTDOWN\0" as *const u8 as *const libc::c_char,
    b"CLIENTQUIT\0" as *const u8 as *const libc::c_char,
    b"ASKINFO\0" as *const u8 as *const libc::c_char,
    b"SERVERINFO\0" as *const u8 as *const libc::c_char,
    b"PLAYERINFO\0" as *const u8 as *const libc::c_char,
    b"REQUESTFILE\0" as *const u8 as *const libc::c_char,
    b"ASKINFOVIAMS\0" as *const u8 as *const libc::c_char,
    b"WILLRESENDGAMESTATE\0" as *const u8 as *const libc::c_char,
    b"CANRECEIVEGAMESTATE\0" as *const u8 as *const libc::c_char,
    b"RECEIVEDGAMESTATE\0" as *const u8 as *const libc::c_char,
    b"SENDINGLUAFILE\0" as *const u8 as *const libc::c_char,
    b"ASKLUAFILE\0" as *const u8 as *const libc::c_char,
    b"HASLUAFILE\0" as *const u8 as *const libc::c_char,
    b"FILEFRAGMENT\0" as *const u8 as *const libc::c_char,
    b"FILEACK\0" as *const u8 as *const libc::c_char,
    b"FILERECEIVED\0" as *const u8 as *const libc::c_char,
    b"TEXTCMD\0" as *const u8 as *const libc::c_char,
    b"TEXTCMD2\0" as *const u8 as *const libc::c_char,
    b"CLIENTJOIN\0" as *const u8 as *const libc::c_char,
    b"NODETIMEOUT\0" as *const u8 as *const libc::c_char,
    b"LOGIN\0" as *const u8 as *const libc::c_char,
    b"TELLFILESNEEDED\0" as *const u8 as *const libc::c_char,
    b"MOREFILESNEEDED\0" as *const u8 as *const libc::c_char,
    b"PING\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
unsafe extern "C" fn DebugPrintpacket(mut header: *const libc::c_char) {
    fprintf(
        debugfile,
        b"%-12s (node %d,ack %d,ackret %d,size %d) type(%d) : %s\n\0" as *const u8
            as *const libc::c_char,
        header,
        (*doomcom).remotenode as libc::c_int,
        (*netbuffer).ack as libc::c_int,
        (*netbuffer).ackreturn as libc::c_int,
        (*doomcom).datalength as libc::c_int,
        (*netbuffer).packettype as libc::c_int,
        packettypename[(*netbuffer).packettype as usize],
    );
    match (*netbuffer).packettype as libc::c_int {
        12 | 16 => {
            fprintf(
                debugfile,
                b"    time %u\n\0" as *const u8 as *const libc::c_char,
                (*netbuffer).u.askinfo.time as int32_t as tic_t,
            );
        }
        29 => {
            fprintf(
                debugfile,
                b"    number %d mode %d\n\0" as *const u8 as *const libc::c_char,
                (*netbuffer).u.clientcfg.localplayers as libc::c_int,
                (*netbuffer).u.clientcfg.mode as libc::c_int,
            );
        }
        8 => {
            let mut serverpak: *mut servertics_pak = &mut (*netbuffer).u.serverpak;
            let mut cmd: *mut uint8_t = &mut *((*serverpak).cmds)
                .as_mut_ptr()
                .offset(
                    ((*serverpak).numslots as libc::c_int
                        * (*serverpak).numtics as libc::c_int) as isize,
                ) as *mut ticcmd_t as *mut uint8_t;
            let mut ntxtcmd: size_t = (&mut *(netbuffer as *mut uint8_t)
                .offset((*doomcom).datalength as isize) as *mut uint8_t)
                .offset_from(cmd) as libc::c_long as size_t;
            fprintf(
                debugfile,
                b"    firsttic %u ply %d tics %d ntxtcmd %s\n    \0" as *const u8
                    as *const libc::c_char,
                (*serverpak).starttic,
                (*serverpak).numslots as libc::c_int,
                (*serverpak).numtics as libc::c_int,
                sizeu1(ntxtcmd),
            );
            fprintfstringnewline(cmd as *mut libc::c_char, ntxtcmd);
        }
        2 | 4 | 3 | 5 | 6 | 7 => {
            fprintf(
                debugfile,
                b"    tic %4u resendfrom %u\n\0" as *const u8 as *const libc::c_char,
                ExpandTics(
                    (*netbuffer).u.clientpak.client_tic as int32_t,
                    (*doomcom).remotenode as int32_t,
                ),
                ExpandTics(
                    (*netbuffer).u.clientpak.resendfrom as int32_t,
                    (*doomcom).remotenode as int32_t,
                ),
            );
        }
        23 => {
            fprintf(debugfile, b"    wipetime\n\0" as *const u8 as *const libc::c_char);
        }
        27 | 28 => {
            fprintf(
                debugfile,
                b"    length %d\n    \0" as *const u8 as *const libc::c_char,
                (*netbuffer).u.textcmd[0 as libc::c_int as usize] as libc::c_int,
            );
            fprintf(
                debugfile,
                b"[%s]\0" as *const u8 as *const libc::c_char,
                netxcmdnames[((*netbuffer).u.textcmd[1 as libc::c_int as usize]
                    as libc::c_int - 1 as libc::c_int) as usize],
            );
            fprintfstringnewline(
                (((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_char)
                    .offset(2 as libc::c_int as isize),
                ((*netbuffer).u.textcmd[0 as libc::c_int as usize] as libc::c_int
                    - 1 as libc::c_int) as size_t,
            );
        }
        1 => {
            fprintf(
                debugfile,
                b"    playerslots %d clientnode %d serverplayer %d gametic %u gamestate %d gametype %d modifiedgame %d\n\0"
                    as *const u8 as *const libc::c_char,
                (*netbuffer).u.servercfg.totalslotnum as libc::c_int,
                (*netbuffer).u.servercfg.clientnode as libc::c_int,
                (*netbuffer).u.servercfg.serverplayer as libc::c_int,
                (*netbuffer).u.servercfg.gametic as int32_t as uint32_t,
                (*netbuffer).u.servercfg.gamestate as libc::c_int,
                (*netbuffer).u.servercfg.gametype as libc::c_int,
                (*netbuffer).u.servercfg.modifiedgame as libc::c_int,
            );
        }
        13 => {
            fprintf(
                debugfile,
                b"    '%s' player %d/%d, map %s, filenum %d, time %u \n\0" as *const u8
                    as *const libc::c_char,
                ((*netbuffer).u.serverinfo.servername).as_mut_ptr(),
                (*netbuffer).u.serverinfo.numberofplayer as libc::c_int,
                (*netbuffer).u.serverinfo.maxplayer as libc::c_int,
                ((*netbuffer).u.serverinfo.mapname).as_mut_ptr(),
                (*netbuffer).u.serverinfo.fileneedednum as libc::c_int,
                (*netbuffer).u.serverinfo.time as int32_t as uint32_t,
            );
            fprintfstringnewline(
                ((*netbuffer).u.serverinfo.fileneeded).as_mut_ptr() as *mut libc::c_char,
                (netbuffer as *mut uint8_t)
                    .offset((*doomcom).datalength as libc::c_int as isize)
                    .offset_from(((*netbuffer).u.serverinfo.fileneeded).as_mut_ptr())
                    as libc::c_long as uint8_t as size_t,
            );
        }
        9 => {
            fprintf(
                debugfile,
                b"    reason %s\n\0" as *const u8 as *const libc::c_char,
                ((*netbuffer).u.serverrefuse.reason).as_mut_ptr(),
            );
        }
        24 => {
            fprintf(
                debugfile,
                b"    fileid %d datasize %d position %u\n\0" as *const u8
                    as *const libc::c_char,
                (*netbuffer).u.filetxpak.fileid as libc::c_int,
                (*netbuffer).u.filetxpak.size as int16_t as uint16_t as libc::c_int,
                (*netbuffer).u.filetxpak.position as int32_t as uint32_t,
            );
        }
        15 | _ => {
            fprintfstringnewline(
                ((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_char,
                (netbuffer as *mut uint8_t)
                    .offset((*doomcom).datalength as libc::c_int as isize)
                    .offset_from(((*netbuffer).u.textcmd).as_mut_ptr()) as libc::c_long
                    as uint8_t as size_t,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn HSendPacket(
    mut node: int32_t,
    mut reliable: boolean,
    mut acknum: uint8_t,
    mut packetlength: size_t,
) -> boolean {
    (*doomcom).datalength = packetlength.wrapping_add(8 as libc::c_ulong) as int16_t;
    if node == 0 as libc::c_int {
        if (rebound_head + 1 as libc::c_int) % 8 as libc::c_int == rebound_tail {
            return false_0 as libc::c_int;
        }
        (*netbuffer).ackreturn = 0 as libc::c_int as uint8_t;
        (*netbuffer).ack = (*netbuffer).ackreturn;
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            &mut *reboundstore.as_mut_ptr().offset(rebound_head as isize)
                as *mut doomdata_t as *mut libc::c_void,
            netbuffer as *const libc::c_void,
            (*doomcom).datalength as size_t,
        );
        reboundsize[rebound_head as usize] = (*doomcom).datalength;
        rebound_head = (rebound_head + 1 as libc::c_int) % 8 as libc::c_int;
        if !debugfile.is_null() {
            (*doomcom).remotenode = node as int16_t;
            DebugPrintpacket(b"SENDLOCAL\0" as *const u8 as *const libc::c_char);
        }
        return true_0 as libc::c_int;
    }
    if netgame == 0 {
        I_Error(
            b"Tried to transmit to another node\0" as *const u8 as *const libc::c_char,
        );
    }
    (*doomcom).remotenode = node as int16_t;
    if (*doomcom).datalength as libc::c_int <= 0 as libc::c_int {
        if !debugfile.is_null() {
            fputs(
                b"HSendPacket: nothing to send\n\0" as *const u8 as *const libc::c_char,
                debugfile,
            );
            fflush(debugfile);
        }
        if !debugfile.is_null() {
            DebugPrintpacket(b"TRISEND\0" as *const u8 as *const libc::c_char);
        }
        return false_0 as libc::c_int;
    }
    if node < 127 as libc::c_int {
        (*netbuffer).ackreturn = GetAcktosend(node);
    } else {
        (*netbuffer).ackreturn = 0 as libc::c_int as uint8_t;
    }
    if reliable != 0 {
        if I_NetCanSend.is_some()
            && I_NetCanSend.expect("non-null function pointer")() == 0
        {
            if ((*netbuffer).packettype as libc::c_int) < PT_CANFAIL as libc::c_int {
                GetFreeAcknum(&mut (*netbuffer).ack, true_0 as libc::c_int);
            }
            if !debugfile.is_null() {
                fputs(
                    b"HSendPacket: Out of bandwidth\n\0" as *const u8
                        as *const libc::c_char,
                    debugfile,
                );
                fflush(debugfile);
            }
            return false_0 as libc::c_int;
        } else if GetFreeAcknum(&mut (*netbuffer).ack, false_0 as libc::c_int) == 0 {
            return false_0 as libc::c_int
        }
    } else {
        (*netbuffer).ack = acknum;
    }
    (*netbuffer).checksum = NetbufferChecksum();
    sendbytes += (packetheaderlength + (*doomcom).datalength as libc::c_int) as int64_t;
    if !debugfile.is_null() {
        DebugPrintpacket(b"SENT\0" as *const u8 as *const libc::c_char);
    }
    I_NetSend.expect("non-null function pointer")();
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn HGetPacket() -> boolean {
    if rebound_tail != rebound_head {
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            netbuffer as *mut libc::c_void,
            &mut *reboundstore.as_mut_ptr().offset(rebound_tail as isize)
                as *mut doomdata_t as *const libc::c_void,
            reboundsize[rebound_tail as usize] as size_t,
        );
        (*doomcom).datalength = reboundsize[rebound_tail as usize];
        if (*netbuffer).packettype as libc::c_int == PT_NODETIMEOUT as libc::c_int {
            (*doomcom)
                .remotenode = (*netbuffer).u.textcmd[0 as libc::c_int as usize]
                as int16_t;
        } else {
            (*doomcom).remotenode = 0 as libc::c_int as int16_t;
        }
        rebound_tail = (rebound_tail + 1 as libc::c_int) % 8 as libc::c_int;
        if !debugfile.is_null() {
            DebugPrintpacket(b"GETLOCAL\0" as *const u8 as *const libc::c_char);
        }
        return true_0 as libc::c_int;
    }
    if netgame == 0 {
        return false_0 as libc::c_int;
    }
    while true_0 as libc::c_int != 0 {
        I_NetGet.expect("non-null function pointer")();
        if (*doomcom).remotenode as libc::c_int == -(1 as libc::c_int) {
            return false_0 as libc::c_int;
        }
        getbytes += packetheaderlength + (*doomcom).datalength as libc::c_int;
        if (*doomcom).remotenode as libc::c_int >= 127 as libc::c_int {
            if !debugfile.is_null() {
                fputs(
                    va(
                        b"Received packet from node %d!\n\0" as *const u8
                            as *const libc::c_char,
                        (*doomcom).remotenode as libc::c_int,
                    ),
                    debugfile,
                );
                fflush(debugfile);
            }
        } else {
            nodes[(*doomcom).remotenode as usize].lasttimepacketreceived = I_GetTime();
            if (*netbuffer).checksum != NetbufferChecksum() {
                if !debugfile.is_null() {
                    fputs(
                        b"Bad packet checksum\n\0" as *const u8 as *const libc::c_char,
                        debugfile,
                    );
                    fflush(debugfile);
                }
            } else {
                if !debugfile.is_null() {
                    DebugPrintpacket(b"GET\0" as *const u8 as *const libc::c_char);
                }
                if Processackpak() == 0 {
                    continue;
                }
                if !((*netbuffer).packettype as libc::c_int == PT_NOTHING as libc::c_int)
                {
                    break;
                }
                GotAcks();
            }
        }
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn Internal_Get() -> boolean {
    (*doomcom).remotenode = -(1 as libc::c_int) as int16_t;
    return false_0 as libc::c_int;
}
unsafe extern "C" fn Internal_Send() -> ! {
    I_Error(b"Send without netgame\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn Internal_FreeNodenum(mut nodenum: int32_t) {}
#[no_mangle]
pub unsafe extern "C" fn I_NetSplitAddress(
    mut host: *mut libc::c_char,
    mut port: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut v4: boolean = (strchr(host, '.' as i32)
        != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    host = strtok(
        host,
        if v4 != 0 {
            b":\0" as *const u8 as *const libc::c_char
        } else {
            b"[]\0" as *const u8 as *const libc::c_char
        },
    );
    if !port.is_null() {
        *port = strtok(
            0 as *mut libc::c_char,
            b":\0" as *const u8 as *const libc::c_char,
        );
    }
    return host;
}
#[no_mangle]
pub unsafe extern "C" fn I_NetMakeNode(mut hostname: *const libc::c_char) -> int8_t {
    let mut newnode: int8_t = -(1 as libc::c_int) as int8_t;
    if I_NetMakeNodewPort.is_some() {
        let mut localhostname: *mut libc::c_char = strdup(hostname);
        let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
        if localhostname.is_null() {
            return newnode;
        }
        hostname = I_NetSplitAddress(localhostname, &mut port);
        newnode = I_NetMakeNodewPort.expect("non-null function pointer")(hostname, port);
        free(localhostname as *mut libc::c_void);
    }
    return newnode;
}
#[no_mangle]
pub unsafe extern "C" fn D_SetDoomcom() {
    if !doomcom.is_null() {
        return;
    }
    doomcom = Z_CallocAlign(
        ::core::mem::size_of::<doomcom_t>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as *mut doomcom_t;
    (*doomcom).id = 0x12345678 as libc::c_long as int32_t;
    (*doomcom).numnodes = 1 as libc::c_int as int16_t;
    (*doomcom).numslots = (*doomcom).numnodes;
    (*doomcom).gametype = 0 as libc::c_int as int16_t;
    (*doomcom).consoleplayer = 0 as libc::c_int as int16_t;
    (*doomcom).extratics = 0 as libc::c_int as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn D_CheckNetGame() -> boolean {
    let mut ret: boolean = false_0 as libc::c_int;
    InitAck();
    rebound_head = 0 as libc::c_int;
    rebound_tail = rebound_head;
    statstarttic = I_GetTime();
    I_NetGet = Some(Internal_Get as unsafe extern "C" fn() -> boolean);
    I_NetSend = ::core::mem::transmute::<
        Option::<unsafe extern "C" fn() -> !>,
        Option::<unsafe extern "C" fn() -> ()>,
    >(Some(Internal_Send as unsafe extern "C" fn() -> !));
    I_NetCanSend = None;
    I_NetCloseSocket = None;
    I_NetFreeNodenum = Some(Internal_FreeNodenum as unsafe extern "C" fn(int32_t) -> ());
    I_NetMakeNodewPort = None;
    hardware_MAXPACKETLENGTH = 1450 as libc::c_int as int16_t;
    net_bandwidth = 30000 as libc::c_int;
    multiplayer = false_0 as libc::c_int;
    netgame = I_InitNetwork();
    if netgame == 0 && I_NetOpenSocket.is_none() {
        D_SetDoomcom();
        netgame = I_InitTcpNetwork();
    }
    if netgame != 0 {
        ret = true_0 as libc::c_int;
    }
    if server == 0 && netgame != 0 {
        netgame = false_0 as libc::c_int;
    }
    server = true_0 as libc::c_int;
    (*doomcom).ticdup = 1 as libc::c_int as int16_t;
    if M_CheckParm(b"-extratic\0" as *const u8 as *const libc::c_char) != 0 {
        if M_IsNextParm() != 0 {
            (*doomcom).extratics = atoi(M_GetNextParm()) as int16_t;
        } else {
            (*doomcom).extratics = 1 as libc::c_int as int16_t;
        }
        CONS_Printf(
            b"Set extratics to %d\n\0" as *const u8 as *const libc::c_char,
            (*doomcom).extratics as libc::c_int,
        );
    }
    if M_CheckParm(b"-bandwidth\0" as *const u8 as *const libc::c_char) != 0 {
        if M_IsNextParm() != 0 {
            net_bandwidth = atoi(M_GetNextParm());
            if net_bandwidth < 1000 as libc::c_int {
                net_bandwidth = 1000 as libc::c_int;
            }
            if net_bandwidth > 100000 as libc::c_int {
                hardware_MAXPACKETLENGTH = 1450 as libc::c_int as int16_t;
            }
            CONS_Printf(
                b"Network bandwidth set to %d\n\0" as *const u8 as *const libc::c_char,
                net_bandwidth,
            );
        } else {
            I_Error(
                b"usage: -bandwidth <byte_per_sec>\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    software_MAXPACKETLENGTH = hardware_MAXPACKETLENGTH as uint16_t;
    if M_CheckParm(b"-packetsize\0" as *const u8 as *const libc::c_char) != 0 {
        if M_IsNextParm() != 0 {
            let mut p: int32_t = atoi(M_GetNextParm());
            if p < 75 as libc::c_int {
                p = 75 as libc::c_int;
            }
            if p > hardware_MAXPACKETLENGTH as libc::c_int {
                p = hardware_MAXPACKETLENGTH as int32_t;
            }
            software_MAXPACKETLENGTH = p as uint16_t;
        } else {
            I_Error(
                b"usage: -packetsize <bytes_per_packet>\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if netgame != 0 {
        multiplayer = true_0 as libc::c_int;
    }
    if (*doomcom).id != 0x12345678 as libc::c_long as int32_t {
        I_Error(b"Doomcom buffer invalid!\0" as *const u8 as *const libc::c_char);
    }
    if (*doomcom).numnodes as libc::c_int > 127 as libc::c_int {
        I_Error(
            b"Too many nodes (%d), max:%d\0" as *const u8 as *const libc::c_char,
            (*doomcom).numnodes as libc::c_int,
            127 as libc::c_int,
        );
    }
    netbuffer = &mut (*doomcom).data as *mut [libc::c_char; 1450] as *mut libc::c_void
        as *mut doomdata_t;
    if M_CheckParm(b"-debugfile\0" as *const u8 as *const libc::c_char) != 0 {
        let mut filename: [libc::c_char; 21] = [0; 21];
        let mut k: int32_t = (*doomcom).consoleplayer as libc::c_int - 1 as libc::c_int;
        if M_IsNextParm() != 0 {
            k = atoi(M_GetNextParm()) - 1 as libc::c_int;
        }
        while debugfile.is_null() && k < 32 as libc::c_int {
            k += 1;
            k;
            sprintf(
                filename.as_mut_ptr(),
                b"debug%d.txt\0" as *const u8 as *const libc::c_char,
                k,
            );
            debugfile = fopen(
                va(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    srb2home.as_mut_ptr(),
                    filename.as_mut_ptr(),
                ),
                b"w\0" as *const u8 as *const libc::c_char,
            );
        }
        if !debugfile.is_null() {
            CONS_Printf(
                b"debug output to: %s\n\0" as *const u8 as *const libc::c_char,
                va(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    srb2home.as_mut_ptr(),
                    filename.as_mut_ptr(),
                ),
            );
        } else {
            CONS_Alert(
                CONS_WARNING,
                b"cannot debug output to file %s!\n\0" as *const u8
                    as *const libc::c_char,
                va(
                    b"%s/%s\0" as *const u8 as *const libc::c_char,
                    srb2home.as_mut_ptr(),
                    filename.as_mut_ptr(),
                ),
            );
        }
    }
    D_ClientServerInit();
    return ret;
}
unsafe extern "C" fn pingcellcmp(
    mut va_0: *const libc::c_void,
    mut vb: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const pingcell = 0 as *const pingcell;
    let mut b: *const pingcell = 0 as *const pingcell;
    a = va_0 as *const pingcell;
    b = vb as *const pingcell;
    return (*a).ms - (*b).ms;
}
#[no_mangle]
pub unsafe extern "C" fn Command_Ping_f() {
    let mut pingv: [pingcell; 32] = [pingcell { num: 0, ms: 0 }; 32];
    let mut pingc: int32_t = 0;
    let mut name_width: libc::c_int = 0 as libc::c_int;
    let mut ms_width: libc::c_int = 0 as libc::c_int;
    let mut n: libc::c_int = 0;
    let mut i: int32_t = 0;
    pingc = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < 32 as libc::c_int {
        if playeringame[i as usize] != 0 {
            n = strlen((player_names[i as usize]).as_mut_ptr()) as libc::c_int;
            if n > name_width {
                name_width = n;
            }
            n = playerpingtable[i as usize] as libc::c_int;
            if n > ms_width {
                ms_width = n;
            }
            pingv[pingc as usize].num = i;
            pingv[pingc as usize].ms = playerpingtable[i as usize] as int32_t;
            pingc += 1;
            pingc;
        }
        i += 1;
        i;
    }
    if ms_width < 10 as libc::c_int {
        ms_width = 1 as libc::c_int;
    } else if ms_width < 100 as libc::c_int {
        ms_width = 2 as libc::c_int;
    } else {
        ms_width = 3 as libc::c_int;
    }
    qsort(
        pingv.as_mut_ptr() as *mut libc::c_void,
        pingc as size_t,
        ::core::mem::size_of::<pingcell>() as libc::c_ulong,
        Some(
            pingcellcmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    i = 0 as libc::c_int;
    while i < pingc {
        CONS_Printf(
            b"%02d : %-*s %*d ms\n\0" as *const u8 as *const libc::c_char,
            pingv[i as usize].num,
            name_width,
            (player_names[pingv[i as usize].num as usize]).as_mut_ptr(),
            ms_width,
            pingv[i as usize].ms,
        );
        i += 1;
        i;
    }
    if server == 0 && playeringame[consoleplayer as usize] != 0 {
        CONS_Printf(
            b"\nYour ping is %d ms\n\0" as *const u8 as *const libc::c_char,
            playerpingtable[consoleplayer as usize],
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_CloseConnection() {
    let mut i: int32_t = 0;
    if netgame != 0 {
        Net_WaitAllAckReceived(5 as libc::c_int as uint32_t);
        i = 0 as libc::c_int;
        while i < 127 as libc::c_int {
            Net_CloseConnection(i | 0x8000 as libc::c_int);
            i += 1;
            i;
        }
        InitAck();
        if I_NetCloseSocket.is_some() {
            I_NetCloseSocket.expect("non-null function pointer")();
        }
        I_NetGet = Some(Internal_Get as unsafe extern "C" fn() -> boolean);
        I_NetSend = ::core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> !>,
            Option::<unsafe extern "C" fn() -> ()>,
        >(Some(Internal_Send as unsafe extern "C" fn() -> !));
        I_NetCanSend = None;
        I_NetCloseSocket = None;
        I_NetFreeNodenum = Some(
            Internal_FreeNodenum as unsafe extern "C" fn(int32_t) -> (),
        );
        I_NetMakeNodewPort = None;
        netgame = false_0 as libc::c_int;
        addedtogame = false_0 as libc::c_int;
    }
    D_ResetTiccmds();
}
