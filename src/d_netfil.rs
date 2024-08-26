use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn sizeu2(num: size_t) -> *mut libc::c_char;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
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
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    static mut cv_downloading: consvar_t;
    static mut modifiedgame: boolean;
    static mut mainwads: uint16_t;
    static mut debugfile: *mut FILE;
    static mut nodeingame: [boolean; 127];
    fn HSendPacket(
        node: int32_t,
        reliable: boolean,
        acknum: uint8_t,
        packetlength: size_t,
    ) -> boolean;
    fn Net_CloseConnection(node: int32_t);
    fn Net_ConnectionTimeout(node: int32_t);
    fn SendNetXCmd(id: netxcmd_t, param: *const libc::c_void, nparam: size_t);
    static mut cv_downloadspeed: consvar_t;
    static mut cv_noticedownload: consvar_t;
    static mut cv_maxsend: consvar_t;
    static mut servernode: int8_t;
    static mut software_MAXPACKETLENGTH: uint16_t;
    static mut server: boolean;
    static mut netbuffer: *mut doomdata_t;
    static mut numwadfiles: uint16_t;
    static mut wadfiles: *mut *mut wadfile_t;
    static mut srb2path: [libc::c_char; 256];
    static mut srb2home: [libc::c_char; 256];
    fn G_SetGameModified(silent: boolean);
    fn I_GetTime() -> tic_t;
    static mut doomcom: *mut doomcom_t;
    static mut I_GetNodeAddress: Option::<
        unsafe extern "C" fn(int32_t) -> *const libc::c_char,
    >;
    fn I_mkdir(dirname: *const libc::c_char, unixright: int32_t) -> int32_t;
    fn I_GetDiskFreeSpace(freespace: *mut int64_t);
    fn M_CheckParm(check: *const libc::c_char) -> int32_t;
    fn StoreLuaFileCallback(id: int32_t);
    fn RemoveLuaFileCallback(id: int32_t);
    fn MakePathDirs(path: *mut libc::c_char);
    fn Z_Free(ptr: *mut libc::c_void);
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
    fn P_AddFolder(folderpath: *const libc::c_char) -> boolean;
    fn P_AddWadFile(wadfilename: *const libc::c_char) -> boolean;
    fn FIL_ConvertTextFileToBinary(
        textfilename: *const libc::c_char,
        binfilename: *const libc::c_char,
    ) -> boolean;
    fn FIL_ReadFileOK(name: *const libc::c_char) -> boolean;
    fn strcatbf(s1: *mut libc::c_char, s2: *const libc::c_char, s3: *const libc::c_char);
    fn M_FileError(handle: *mut FILE) -> *const libc::c_char;
    fn md5_stream(stream: *mut FILE, resblock: *mut libc::c_void) -> libc::c_int;
    fn filesearch(
        filename: *mut libc::c_char,
        startpath: *const libc::c_char,
        wantedmd5sum: *const uint8_t,
        completepath: boolean,
        maxsearchdepth: libc::c_int,
    ) -> filestatus_t;
    fn concatpaths(path: *const libc::c_char, startpath: *const libc::c_char) -> int32_t;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type tic_t = uint32_t;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
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
pub type consvar_t = consvar_s;
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
pub type C2RustUnnamed_2 = libc::c_uint;
pub const NUMPACKETTYPE: C2RustUnnamed_2 = 35;
pub const PT_PING: C2RustUnnamed_2 = 34;
pub const PT_MOREFILESNEEDED: C2RustUnnamed_2 = 33;
pub const PT_TELLFILESNEEDED: C2RustUnnamed_2 = 32;
pub const PT_LOGIN: C2RustUnnamed_2 = 31;
pub const PT_NODETIMEOUT: C2RustUnnamed_2 = 30;
pub const PT_CLIENTJOIN: C2RustUnnamed_2 = 29;
pub const PT_TEXTCMD2: C2RustUnnamed_2 = 28;
pub const PT_TEXTCMD: C2RustUnnamed_2 = 27;
pub const PT_FILERECEIVED: C2RustUnnamed_2 = 26;
pub const PT_FILEACK: C2RustUnnamed_2 = 25;
pub const PT_FILEFRAGMENT: C2RustUnnamed_2 = 24;
pub const PT_CANFAIL: C2RustUnnamed_2 = 24;
pub const PT_BASICKEEPALIVE: C2RustUnnamed_2 = 23;
pub const PT_HASLUAFILE: C2RustUnnamed_2 = 22;
pub const PT_ASKLUAFILE: C2RustUnnamed_2 = 21;
pub const PT_SENDINGLUAFILE: C2RustUnnamed_2 = 20;
pub const PT_RECEIVEDGAMESTATE: C2RustUnnamed_2 = 19;
pub const PT_CANRECEIVEGAMESTATE: C2RustUnnamed_2 = 18;
pub const PT_WILLRESENDGAMESTATE: C2RustUnnamed_2 = 17;
pub const PT_ASKINFOVIAMS: C2RustUnnamed_2 = 16;
pub const PT_REQUESTFILE: C2RustUnnamed_2 = 15;
pub const PT_PLAYERINFO: C2RustUnnamed_2 = 14;
pub const PT_SERVERINFO: C2RustUnnamed_2 = 13;
pub const PT_ASKINFO: C2RustUnnamed_2 = 12;
pub const PT_CLIENTQUIT: C2RustUnnamed_2 = 11;
pub const PT_SERVERSHUTDOWN: C2RustUnnamed_2 = 10;
pub const PT_SERVERREFUSE: C2RustUnnamed_2 = 9;
pub const PT_SERVERTICS: C2RustUnnamed_2 = 8;
pub const PT_NODEKEEPALIVEMIS: C2RustUnnamed_2 = 7;
pub const PT_NODEKEEPALIVE: C2RustUnnamed_2 = 6;
pub const PT_CLIENT2MIS: C2RustUnnamed_2 = 5;
pub const PT_CLIENT2CMD: C2RustUnnamed_2 = 4;
pub const PT_CLIENTMIS: C2RustUnnamed_2 = 3;
pub const PT_CLIENTCMD: C2RustUnnamed_2 = 2;
pub const PT_SERVERCFG: C2RustUnnamed_2 = 1;
pub const PT_NOTHING: C2RustUnnamed_2 = 0;
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
    pub u: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type compmethod = libc::c_uint;
pub const CM_UNSUPPORTED: compmethod = 3;
pub const CM_LZF: compmethod = 2;
pub const CM_DEFLATE: compmethod = 1;
pub const CM_NOCOMPRESSION: compmethod = 0;
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
pub type restype = libc::c_uint;
pub const RET_UNKNOWN: restype = 5;
pub const RET_FOLDER: restype = 4;
pub const RET_PK3: restype = 3;
pub const RET_LUA: restype = 2;
pub const RET_SOC: restype = 1;
pub const RET_WAD: restype = 0;
pub type restype_t = restype;
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
pub type wadfile_t = wadfile_s;
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
pub type freemethod_t = libc::c_uint;
pub const SF_NOFREERAM: freemethod_t = 3;
pub const SF_RAM: freemethod_t = 2;
pub const SF_Z_RAM: freemethod_t = 1;
pub const SF_FILE: freemethod_t = 0;
pub type filestatus_t = libc::c_uint;
pub const FS_MD5SUMBAD: filestatus_t = 6;
pub const FS_OPEN: filestatus_t = 5;
pub const FS_DOWNLOADING: filestatus_t = 4;
pub const FS_REQUESTED: filestatus_t = 3;
pub const FS_FOUND: filestatus_t = 2;
pub const FS_NOTFOUND: filestatus_t = 1;
pub const FS_NOTCHECKED: filestatus_t = 0;
pub type fileneededtype_t = libc::c_uint;
pub const FILENEEDED_LUAFILE: fileneededtype_t = 2;
pub const FILENEEDED_SAVEGAME: fileneededtype_t = 1;
pub const FILENEEDED_WAD: fileneededtype_t = 0;
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
pub const PU_STATIC: C2RustUnnamed_5 = 1;
pub type filetx_t = filetx_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filetx_s {
    pub ram: int32_t,
    pub id: C2RustUnnamed_4,
    pub size: uint32_t,
    pub fileid: uint8_t,
    pub node: int32_t,
    pub next: *mut filetx_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub filename: *mut libc::c_char,
    pub ram: *mut libc::c_char,
}
pub type filetran_t = filetran_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filetran_s {
    pub txlist: *mut filetx_t,
    pub iteration: uint8_t,
    pub ackediteration: uint8_t,
    pub position: uint32_t,
    pub ackedfragments: *mut boolean,
    pub ackedsize: uint32_t,
    pub currentfile: *mut FILE,
    pub dontsenduntil: tic_t,
}
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
pub type luafiletransfernodestatus_t = libc::c_uint;
pub const LFTNS_SENT: luafiletransfernodestatus_t = 4;
pub const LFTNS_SENDING: luafiletransfernodestatus_t = 3;
pub const LFTNS_ASKED: luafiletransfernodestatus_t = 2;
pub const LFTNS_WAITING: luafiletransfernodestatus_t = 1;
pub const LFTNS_NONE: luafiletransfernodestatus_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pauseddownload_t {
    pub filename: [libc::c_char; 512],
    pub md5sum: [uint8_t; 16],
    pub receivedfragments: *mut boolean,
    pub fragmentsize: uint32_t,
    pub currentsize: uint32_t,
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
static mut transfer: [filetran_t; 127] = [filetran_s {
    txlist: 0 as *const filetx_t as *mut filetx_t,
    iteration: 0,
    ackediteration: 0,
    position: 0,
    ackedfragments: 0 as *const boolean as *mut boolean,
    ackedsize: 0,
    currentfile: 0 as *const FILE as *mut FILE,
    dontsenduntil: 0,
}; 127];
#[no_mangle]
pub static mut fileneedednum: int32_t = 0;
#[no_mangle]
pub static mut fileneeded: *mut fileneeded_t = 0 as *const fileneeded_t
    as *mut fileneeded_t;
static mut lasttimeackpacketsent: tic_t = 0 as libc::c_int as tic_t;
#[no_mangle]
pub static mut downloaddir: [libc::c_char; 512] = unsafe {
    *::core::mem::transmute::<
        &[u8; 512],
        &mut [libc::c_char; 512],
    >(
        b"DOWNLOAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
static mut pauseddownload: *mut pauseddownload_t = 0 as *const pauseddownload_t
    as *mut pauseddownload_t;
#[no_mangle]
pub static mut lastfilenum: int32_t = -(1 as libc::c_int);
#[no_mangle]
pub static mut downloadcompletednum: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut downloadcompletedsize: uint32_t = 0 as libc::c_int as uint32_t;
#[no_mangle]
pub static mut totalfilesrequestednum: int32_t = 0 as libc::c_int;
#[no_mangle]
pub static mut totalfilesrequestedsize: uint32_t = 0 as libc::c_int as uint32_t;
#[no_mangle]
pub static mut luafiletransfers: *mut luafiletransfer_t = 0 as *const luafiletransfer_t
    as *mut luafiletransfer_t;
#[no_mangle]
pub static mut waitingforluafiletransfer: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut waitingforluafilecommand: boolean = false_0 as libc::c_int;
#[no_mangle]
pub static mut luafiledir: [libc::c_char; 272] = unsafe {
    *::core::mem::transmute::<
        &[u8; 272],
        &mut [libc::c_char; 272],
    >(
        b"luafiles\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    )
};
unsafe extern "C" fn GetWadNumFromFileNeededId(mut id: uint8_t) -> uint16_t {
    let mut wadnum: uint16_t = 0;
    wadnum = mainwads;
    while (wadnum as libc::c_int) < numwadfiles as libc::c_int {
        if !((**wadfiles.offset(wadnum as isize)).important == 0) {
            if id as libc::c_int == 0 as libc::c_int {
                return wadnum;
            }
            id = id.wrapping_sub(1);
            id;
        }
        wadnum = wadnum.wrapping_add(1);
        wadnum;
    }
    return 65535 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn PutFileNeeded(mut firstfile: uint16_t) -> *mut uint8_t {
    let mut i: size_t = 0;
    let mut count: uint8_t = 0 as libc::c_int as uint8_t;
    let mut p_start: *mut uint8_t = if (*netbuffer).packettype as libc::c_int
        == PT_MOREFILESNEEDED as libc::c_int
    {
        ((*netbuffer).u.filesneededcfg.files).as_mut_ptr()
    } else {
        ((*netbuffer).u.serverinfo.fileneeded).as_mut_ptr()
    };
    let mut p: *mut uint8_t = p_start;
    let mut wadfilename: [libc::c_char; 512] = *::core::mem::transmute::<
        &[u8; 512],
        &mut [libc::c_char; 512],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut filestatus: uint8_t = 0;
    let mut folder: uint8_t = 0;
    i = mainwads as size_t;
    while i < numwadfiles as size_t {
        if !((**wadfiles.offset(i as isize)).important == 0) {
            if firstfile != 0 {
                firstfile = firstfile.wrapping_sub(1);
                firstfile;
            } else {
                nameonly(
                    strcpy(
                        wadfilename.as_mut_ptr(),
                        (**wadfiles.offset(i as isize)).filename,
                    ),
                );
                if p
                    .offset(1 as libc::c_int as isize)
                    .offset(4 as libc::c_int as isize)
                    .offset(
                        (if (strlen(wadfilename.as_mut_ptr()))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            < 512 as libc::c_int as libc::c_ulong
                        {
                            (strlen(wadfilename.as_mut_ptr()))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                        } else {
                            512 as libc::c_int as libc::c_ulong
                        }) as isize,
                    )
                    .offset(16 as libc::c_int as isize)
                    > p_start.offset(915 as libc::c_int as isize)
                {
                    if (*netbuffer).packettype as libc::c_int
                        == PT_MOREFILESNEEDED as libc::c_int
                    {
                        (*netbuffer).u.filesneededcfg.more = 1 as libc::c_int as uint8_t;
                    } else {
                        (*netbuffer)
                            .u
                            .serverinfo
                            .flags = ((*netbuffer).u.serverinfo.flags as libc::c_int
                            | 0x20 as libc::c_int) as uint8_t;
                    }
                    break;
                } else {
                    filestatus = 1 as libc::c_int as uint8_t;
                    folder = ((**wadfiles.offset(i as isize)).type_0 as libc::c_uint
                        == RET_FOLDER as libc::c_int as libc::c_uint) as libc::c_int
                        as uint8_t;
                    if cv_downloading.value == 0 || folder as libc::c_int != 0 {
                        filestatus = (filestatus as libc::c_int
                            + ((2 as libc::c_int) << 4 as libc::c_int)) as uint8_t;
                    } else if (**wadfiles.offset(i as isize)).filesize
                        <= cv_maxsend.value as uint32_t * 1024 as libc::c_int as uint32_t
                    {
                        filestatus = (filestatus as libc::c_int
                            + ((1 as libc::c_int) << 4 as libc::c_int)) as uint8_t;
                    }
                    let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
                    let tv: uint8_t = filestatus;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv as *const uint8_t as *const libc::c_void,
                        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                    );
                    p_tmp = p_tmp.offset(1);
                    p_tmp;
                    p = p_tmp as *mut libc::c_void as *mut uint8_t;
                    let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void
                        as *mut uint8_t;
                    let tv_0: uint8_t = folder;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_0 as *const uint8_t as *const libc::c_void,
                        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
                    );
                    p_tmp_0 = p_tmp_0.offset(1);
                    p_tmp_0;
                    p = p_tmp_0 as *mut libc::c_void as *mut uint8_t;
                    count = count.wrapping_add(1);
                    count;
                    let mut p_tmp_1: *mut uint32_t = p as *mut libc::c_void
                        as *mut uint32_t;
                    let tv_1: uint32_t = (**wadfiles.offset(i as isize)).filesize;
                    memcpy(
                        p as *mut libc::c_void,
                        &tv_1 as *const uint32_t as *const libc::c_void,
                        ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
                    );
                    p_tmp_1 = p_tmp_1.offset(1);
                    p_tmp_1;
                    p = p_tmp_1 as *mut libc::c_void as *mut uint8_t;
                    let mut tmp_i: size_t = 0;
                    tmp_i = 0 as libc::c_int as size_t;
                    while tmp_i < 512 as libc::c_int as size_t
                        && wadfilename[tmp_i as usize] as libc::c_int != '\0' as i32
                    {
                        let mut p_tmp_2: *mut libc::c_char = p as *mut libc::c_void
                            as *mut libc::c_char;
                        let tv_2: libc::c_char = wadfilename[tmp_i as usize];
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
                    if tmp_i < 512 as libc::c_int as size_t {
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
                    memcpy(
                        p as *mut libc::c_void,
                        ((**wadfiles.offset(i as isize)).md5sum).as_mut_ptr()
                            as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    p = p.offset(16 as libc::c_int as isize);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if (*netbuffer).packettype as libc::c_int == PT_MOREFILESNEEDED as libc::c_int {
        (*netbuffer).u.filesneededcfg.num = count;
    } else {
        (*netbuffer).u.serverinfo.fileneedednum = count;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn AllocFileNeeded(mut size: int32_t) {
    if fileneeded.is_null() {
        fileneeded = Z_CallocAlign(
            (::core::mem::size_of::<fileneeded_t>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut fileneeded_t;
    } else {
        fileneeded = Z_ReallocAlign(
            fileneeded as *mut libc::c_void,
            (::core::mem::size_of::<fileneeded_t>() as libc::c_ulong)
                .wrapping_mul(size as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut fileneeded_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn FreeFileNeeded() {
    Z_Free(fileneeded as *mut libc::c_void);
    fileneeded = 0 as *mut fileneeded_t;
}
#[no_mangle]
pub unsafe extern "C" fn D_ParseFileneeded(
    mut fileneedednum_parm: int32_t,
    mut fileneededstr: *mut uint8_t,
    mut firstfile: uint16_t,
) {
    let mut i: int32_t = 0;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut filestatus: uint8_t = 0;
    fileneedednum = firstfile as libc::c_int + fileneedednum_parm;
    p = fileneededstr;
    AllocFileNeeded(fileneedednum);
    i = firstfile as int32_t;
    while i < fileneedednum {
        (*fileneeded.offset(i as isize)).type_0 = FILENEEDED_WAD;
        (*fileneeded.offset(i as isize)).status = FS_NOTCHECKED;
        (*fileneeded.offset(i as isize)).justdownloaded = false_0 as libc::c_int;
        filestatus = ({
            let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let mut b: uint8_t = 0;
            memcpy(
                &mut b as *mut uint8_t as *mut libc::c_void,
                p as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut uint8_t;
            b
        });
        (*fileneeded.offset(i as isize))
            .folder = ({
            let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let mut b: uint8_t = 0;
            memcpy(
                &mut b as *mut uint8_t as *mut libc::c_void,
                p as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut uint8_t;
            b
        });
        (*fileneeded.offset(i as isize))
            .willsend = (filestatus as libc::c_int >> 4 as libc::c_int) as uint8_t;
        (*fileneeded.offset(i as isize))
            .totalsize = ({
            let mut p_tmp: *mut uint32_t = p as *mut libc::c_void as *mut uint32_t;
            let mut b: uint32_t = 0;
            memcpy(
                &mut b as *mut uint32_t as *mut libc::c_void,
                p as *const libc::c_void,
                ::core::mem::size_of::<uint32_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut uint8_t;
            b
        });
        let ref mut fresh0 = (*fileneeded.offset(i as isize)).file;
        *fresh0 = 0 as *mut FILE;
        let mut tmp_i: size_t = 0 as libc::c_int as size_t;
        while tmp_i < 512 as libc::c_int as size_t
            && {
                let ref mut fresh1 = (*fileneeded.offset(i as isize))
                    .filename[tmp_i as usize];
                *fresh1 = ({
                    let mut p_tmp: *mut libc::c_char = p as *mut libc::c_void
                        as *mut libc::c_char;
                    let mut b: libc::c_char = 0;
                    memcpy(
                        &mut b as *mut libc::c_char as *mut libc::c_void,
                        p as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    p_tmp = p_tmp.offset(1);
                    p_tmp;
                    p = p_tmp as *mut libc::c_void as *mut uint8_t;
                    b
                });
                *fresh1 as libc::c_int != '\0' as i32
            }
        {
            tmp_i = tmp_i.wrapping_add(1);
            tmp_i;
        }
        (*fileneeded.offset(i as isize))
            .filename[tmp_i as usize] = '\0' as i32 as libc::c_char;
        memcpy(
            ((*fileneeded.offset(i as isize)).md5sum).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        p = p.offset(16 as libc::c_int as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CL_PrepareDownloadSaveGame(mut tmpsave: *const libc::c_char) {
    lastfilenum = -(1 as libc::c_int);
    FreeFileNeeded();
    AllocFileNeeded(1 as libc::c_int);
    fileneedednum = 1 as libc::c_int;
    (*fileneeded.offset(0 as libc::c_int as isize)).type_0 = FILENEEDED_SAVEGAME;
    (*fileneeded.offset(0 as libc::c_int as isize)).status = FS_REQUESTED;
    (*fileneeded.offset(0 as libc::c_int as isize))
        .justdownloaded = false_0 as libc::c_int;
    (*fileneeded.offset(0 as libc::c_int as isize))
        .totalsize = 4294967295 as libc::c_uint;
    let ref mut fresh2 = (*fileneeded.offset(0 as libc::c_int as isize)).file;
    *fresh2 = 0 as *mut FILE;
    memset(
        ((*fileneeded.offset(0 as libc::c_int as isize)).md5sum).as_mut_ptr()
            as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    strcpy(
        ((*fileneeded.offset(0 as libc::c_int as isize)).filename).as_mut_ptr(),
        tmpsave,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CL_CheckDownloadable() -> boolean {
    let mut i: uint8_t = 0;
    let mut dlstatus: uint8_t = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < fileneedednum {
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            != FS_FOUND as libc::c_int as libc::c_uint
            && (*fileneeded.offset(i as isize)).status as libc::c_uint
                != FS_OPEN as libc::c_int as libc::c_uint
        {
            if !((*fileneeded.offset(i as isize)).willsend as libc::c_int
                == 1 as libc::c_int)
            {
                if (*fileneeded.offset(i as isize)).willsend as libc::c_int
                    == 0 as libc::c_int
                {
                    dlstatus = 1 as libc::c_int as uint8_t;
                } else {
                    dlstatus = 2 as libc::c_int as uint8_t;
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    if dlstatus == 0
        && M_CheckParm(b"-nodownload\0" as *const u8 as *const libc::c_char) != 0
    {
        dlstatus = 3 as libc::c_int as uint8_t;
    }
    if dlstatus == 0 {
        return true_0 as libc::c_int;
    }
    CONS_Alert(
        CONS_NOTICE,
        b"You need additional files to connect to this server:\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < fileneedednum {
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            != FS_FOUND as libc::c_int as libc::c_uint
            && (*fileneeded.offset(i as isize)).status as libc::c_uint
                != FS_OPEN as libc::c_int as libc::c_uint
        {
            CONS_Printf(
                b" * \"%s\" (%dK)\0" as *const u8 as *const libc::c_char,
                ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                (*fileneeded.offset(i as isize)).totalsize >> 10 as libc::c_int,
            );
            if (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_NOTFOUND as libc::c_int as libc::c_uint
            {
                CONS_Printf(b" not found, md5: \0" as *const u8 as *const libc::c_char);
            } else if (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_MD5SUMBAD as libc::c_int as libc::c_uint
            {
                CONS_Printf(
                    b" wrong version, md5: \0" as *const u8 as *const libc::c_char,
                );
            }
            let mut j: int32_t = 0;
            let mut md5tmp: [libc::c_char; 33] = [0; 33];
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                sprintf(
                    &mut *md5tmp.as_mut_ptr().offset((j * 2 as libc::c_int) as isize)
                        as *mut libc::c_char,
                    b"%02x\0" as *const u8 as *const libc::c_char,
                    (*fileneeded.offset(i as isize)).md5sum[j as usize] as libc::c_int,
                );
                j += 1;
                j;
            }
            CONS_Printf(
                b"%s\0" as *const u8 as *const libc::c_char,
                md5tmp.as_mut_ptr(),
            );
            CONS_Printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        i = i.wrapping_add(1);
        i;
    }
    match dlstatus as libc::c_int {
        1 => {
            CONS_Printf(
                b"Some files are larger than the server is willing to send.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        2 => {
            CONS_Printf(
                b"The server is not allowing download requests.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        3 => {
            CONS_Printf(
                b"All files downloadable, but you have chosen to disable downloading locally.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn CL_CanResumeDownload(mut file: *mut fileneeded_t) -> boolean {
    return (!pauseddownload.is_null()
        && strcmp(
            ((*pauseddownload).filename).as_mut_ptr(),
            ((*file).filename).as_mut_ptr(),
        ) == 0
        && memcmp(
            ((*pauseddownload).md5sum).as_mut_ptr() as *const libc::c_void,
            ((*file).md5sum).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        ) == 0 && (*pauseddownload).fragmentsize == (*file).fragmentsize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_AbortDownloadResume() {
    if pauseddownload.is_null() {
        return;
    }
    free((*pauseddownload).receivedfragments as *mut libc::c_void);
    remove(((*pauseddownload).filename).as_mut_ptr());
    free(pauseddownload as *mut libc::c_void);
    pauseddownload = 0 as *mut pauseddownload_t;
}
#[no_mangle]
pub unsafe extern "C" fn CL_SendFileRequest() -> boolean {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: int32_t = 0;
    let mut totalfreespaceneeded: int64_t = 0 as libc::c_int as int64_t;
    let mut availablefreespace: int64_t = 0;
    (*netbuffer).packettype = PT_REQUESTFILE as libc::c_int as uint8_t;
    p = ((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < fileneedednum {
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            == FS_NOTFOUND as libc::c_int as libc::c_uint
            || (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_MD5SUMBAD as libc::c_int as libc::c_uint
        {
            totalfreespaceneeded
                += (*fileneeded.offset(i as isize)).totalsize as int64_t;
            let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let tv: uint8_t = i as uint8_t;
            memcpy(
                p as *mut libc::c_void,
                &tv as *const uint8_t as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut libc::c_char;
            nameonly(((*fileneeded.offset(i as isize)).filename).as_mut_ptr());
            strcatbf(
                ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                downloaddir.as_mut_ptr(),
                b"/\0" as *const u8 as *const libc::c_char,
            );
            (*fileneeded.offset(i as isize)).status = FS_REQUESTED;
        }
        i += 1;
        i;
    }
    let mut p_tmp_0: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
    let tv_0: uint8_t = 0xff as libc::c_int as uint8_t;
    memcpy(
        p as *mut libc::c_void,
        &tv_0 as *const uint8_t as *const libc::c_void,
        ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
    );
    p_tmp_0 = p_tmp_0.offset(1);
    p_tmp_0;
    p = p_tmp_0 as *mut libc::c_void as *mut libc::c_char;
    I_GetDiskFreeSpace(&mut availablefreespace);
    if totalfreespaceneeded > availablefreespace {
        I_Error(
            b"To play on this server you must download %s KB,\nbut you have only %s KB free space on this drive\n\0"
                as *const u8 as *const libc::c_char,
            sizeu1((totalfreespaceneeded >> 10 as libc::c_int) as size_t),
            sizeu2((availablefreespace >> 10 as libc::c_int) as size_t),
        );
    }
    I_mkdir(downloaddir.as_mut_ptr(), 0o755 as libc::c_int);
    return HSendPacket(
        servernode as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        p.offset_from(((*netbuffer).u.textcmd).as_mut_ptr() as *mut libc::c_char)
            as libc::c_long as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PT_RequestFile(mut node: int32_t) -> boolean {
    let mut p: *mut uint8_t = ((*netbuffer).u.textcmd).as_mut_ptr();
    let mut id: uint8_t = 0;
    while p
        < ((*netbuffer).u.textcmd)
            .as_mut_ptr()
            .offset(256 as libc::c_int as isize)
            .offset(-(1 as libc::c_int as isize))
    {
        id = ({
            let mut p_tmp: *mut uint8_t = p as *mut libc::c_void as *mut uint8_t;
            let mut b: uint8_t = 0;
            memcpy(
                &mut b as *mut uint8_t as *mut libc::c_void,
                p as *const libc::c_void,
                ::core::mem::size_of::<uint8_t>() as libc::c_ulong,
            );
            p_tmp = p_tmp.offset(1);
            p_tmp;
            p = p_tmp as *mut libc::c_void as *mut uint8_t;
            b
        });
        if id as libc::c_int == 0xff as libc::c_int {
            break;
        }
        if AddFileToSendQueue(node, id) == 0 {
            SV_AbortSendFiles(node);
            return false_0 as libc::c_int;
        }
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CL_CheckFiles() -> int32_t {
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut wadfilename: [libc::c_char; 512] = [0; 512];
    let mut filestoload: size_t = 0 as libc::c_int as size_t;
    let mut downloadrequired: boolean = false_0 as libc::c_int;
    if modifiedgame != 0 {
        CONS_Debug(
            0x100 as libc::c_int,
            b"game is modified; only doing basic checks\n\0" as *const u8
                as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        j = mainwads as int32_t;
        while i < fileneedednum || j < numwadfiles as libc::c_int {
            if j < numwadfiles as libc::c_int
                && (**wadfiles.offset(j as isize)).important == 0
            {
                j += 1;
                j;
            } else {
                if i >= fileneedednum || j >= numwadfiles as libc::c_int {
                    return 2 as libc::c_int;
                }
                if memcmp(
                    ((**wadfiles.offset(j as isize)).md5sum).as_mut_ptr()
                        as *const libc::c_void,
                    ((*fileneeded.offset(i as isize)).md5sum).as_mut_ptr()
                        as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                ) != 0
                {
                    return 2 as libc::c_int;
                }
                CONS_Debug(
                    0x100 as libc::c_int,
                    b"'%s' accounted for\n\0" as *const u8 as *const libc::c_char,
                    ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                );
                (*fileneeded.offset(i as isize)).status = FS_OPEN;
                i += 1;
                i;
                j += 1;
                j;
            }
        }
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < fileneedednum {
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            == FS_NOTFOUND as libc::c_int as libc::c_uint
            || (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_MD5SUMBAD as libc::c_int as libc::c_uint
        {
            downloadrequired = true_0 as libc::c_int;
        }
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            != FS_OPEN as libc::c_int as libc::c_uint
        {
            filestoload = filestoload.wrapping_add(1);
            filestoload;
        }
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            != FS_NOTCHECKED as libc::c_int as libc::c_uint
        {
            i += 1;
            i;
        } else {
            CONS_Debug(
                0x100 as libc::c_int,
                b"searching for '%s' \0" as *const u8 as *const libc::c_char,
                ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
            );
            j = mainwads as int32_t;
            while j < numwadfiles as libc::c_int {
                nameonly(
                    strcpy(
                        wadfilename.as_mut_ptr(),
                        (**wadfiles.offset(j as isize)).filename,
                    ),
                );
                if strcasecmp(
                    wadfilename.as_mut_ptr(),
                    ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                ) == 0
                    && memcmp(
                        ((**wadfiles.offset(j as isize)).md5sum).as_mut_ptr()
                            as *const libc::c_void,
                        ((*fileneeded.offset(i as isize)).md5sum).as_mut_ptr()
                            as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    CONS_Debug(
                        0x100 as libc::c_int,
                        b"already loaded\n\0" as *const u8 as *const libc::c_char,
                    );
                    (*fileneeded.offset(i as isize)).status = FS_OPEN;
                    return 4 as libc::c_int;
                }
                j += 1;
                j;
            }
            if (*fileneeded.offset(i as isize)).folder != 0 {
                (*fileneeded.offset(i as isize))
                    .status = findfolder(
                    ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                );
            } else {
                (*fileneeded.offset(i as isize))
                    .status = findfile(
                    ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                    ((*fileneeded.offset(i as isize)).md5sum).as_mut_ptr(),
                    true_0 as libc::c_int,
                );
            }
            CONS_Debug(
                0x100 as libc::c_int,
                b"found %d\n\0" as *const u8 as *const libc::c_char,
                (*fileneeded.offset(i as isize)).status as libc::c_uint,
            );
            return 4 as libc::c_int;
        }
    }
    if (numwadfiles as size_t).wrapping_add(filestoload) > 65535 as libc::c_int as size_t
    {
        return 3 as libc::c_int
    } else if downloadrequired != 0 {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_LoadServerFiles() -> boolean {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < fileneedednum {
        if (*fileneeded.offset(i as isize)).status as libc::c_uint
            == FS_OPEN as libc::c_int as libc::c_uint
        {
            i += 1;
            i;
        } else if (*fileneeded.offset(i as isize)).status as libc::c_uint
            == FS_FOUND as libc::c_int as libc::c_uint
        {
            if (*fileneeded.offset(i as isize)).folder != 0 {
                P_AddFolder(((*fileneeded.offset(i as isize)).filename).as_mut_ptr());
            } else {
                P_AddWadFile(((*fileneeded.offset(i as isize)).filename).as_mut_ptr());
            }
            G_SetGameModified(true_0 as libc::c_int);
            (*fileneeded.offset(i as isize)).status = FS_OPEN;
            return false_0 as libc::c_int;
        } else if (*fileneeded.offset(i as isize)).status as libc::c_uint
            == FS_MD5SUMBAD as libc::c_int as libc::c_uint
        {
            I_Error(
                b"Wrong version of file %s\0" as *const u8 as *const libc::c_char,
                ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
            );
        } else {
            let mut s: *const libc::c_char = 0 as *const libc::c_char;
            match (*fileneeded.offset(i as isize)).status as libc::c_uint {
                1 => {
                    s = b"FS_NOTFOUND\0" as *const u8 as *const libc::c_char;
                }
                3 => {
                    s = b"FS_REQUESTED\0" as *const u8 as *const libc::c_char;
                }
                4 => {
                    s = b"FS_DOWNLOADING\0" as *const u8 as *const libc::c_char;
                }
                _ => {
                    s = b"unknown\0" as *const u8 as *const libc::c_char;
                }
            }
            I_Error(
                b"Try to load file \"%s\" with status of %d (%s)\n\0" as *const u8
                    as *const libc::c_char,
                ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                (*fileneeded.offset(i as isize)).status as libc::c_uint,
                s,
            );
        }
    }
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AddLuaFileTransfer(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) {
    let mut prevnext: *mut *mut luafiletransfer_t = 0 as *mut *mut luafiletransfer_t;
    let mut filetransfer: *mut luafiletransfer_t = 0 as *mut luafiletransfer_t;
    static mut id: int32_t = 0;
    prevnext = &mut luafiletransfers;
    while !(*prevnext).is_null() {
        prevnext = &mut (**prevnext).next;
    }
    filetransfer = malloc(::core::mem::size_of::<luafiletransfer_t>() as libc::c_ulong)
        as *mut luafiletransfer_t;
    if filetransfer.is_null() {
        I_Error(
            b"AddLuaFileTransfer: Out of memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    *prevnext = filetransfer;
    (*filetransfer).next = 0 as *mut luafiletransfer_s;
    (*filetransfer).filename = strdup(filename);
    if ((*filetransfer).filename).is_null() {
        I_Error(
            b"AddLuaFileTransfer: Out of memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if server != 0 {
        (*filetransfer)
            .realfilename = strdup(
            va(
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                luafiledir.as_mut_ptr(),
                filename,
            ),
        );
    } else {
        (*filetransfer)
            .realfilename = strdup(
            va(
                b"%s/client/$$$%d%d.tmp\0" as *const u8 as *const libc::c_char,
                luafiledir.as_mut_ptr(),
                rand(),
                rand(),
            ),
        );
    }
    if ((*filetransfer).realfilename).is_null() {
        I_Error(
            b"AddLuaFileTransfer: Out of memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    strlcpy(
        ((*filetransfer).mode).as_mut_ptr(),
        mode,
        ::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong,
    );
    if server != 0 && filetransfer == luafiletransfers {
        SV_PrepareSendLuaFile();
    } else {
        (*filetransfer).ongoing = false_0 as libc::c_int;
    }
    (*filetransfer).id = id;
    StoreLuaFileCallback(id);
    id += 1;
    id;
    if waitingforluafiletransfer != 0 {
        waitingforluafiletransfer = false_0 as libc::c_int;
        CL_PrepareDownloadLuaFile();
    }
}
unsafe extern "C" fn SV_PrepareSendLuaFileToNextNode() {
    let mut i: int32_t = 0;
    let mut success: uint8_t = 1 as libc::c_int as uint8_t;
    i = 1 as libc::c_int;
    while i < 127 as libc::c_int {
        if (*luafiletransfers).nodestatus[i as usize] as libc::c_uint
            == LFTNS_WAITING as libc::c_int as libc::c_uint
        {
            (*netbuffer).packettype = PT_SENDINGLUAFILE as libc::c_int as uint8_t;
            if HSendPacket(
                i,
                true_0 as libc::c_int,
                0 as libc::c_int as uint8_t,
                0 as libc::c_int as size_t,
            ) == 0
            {
                I_Error(
                    b"Failed to send a PT_SENDINGLUAFILE packet\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*luafiletransfers).nodestatus[i as usize] = LFTNS_ASKED;
            (*luafiletransfers)
                .nodetimeouts[i
                as usize] = (I_GetTime())
                .wrapping_add((30 as libc::c_int * 35 as libc::c_int) as tic_t);
            return;
        }
        i += 1;
        i;
    }
    SendNetXCmd(
        XD_LUAFILE,
        &mut success as *mut uint8_t as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_PrepareSendLuaFile() {
    let mut binfilename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: int32_t = 0;
    (*luafiletransfers).ongoing = true_0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        (*luafiletransfers)
            .nodestatus[i
            as usize] = (if nodeingame[i as usize] != 0 {
            LFTNS_WAITING as libc::c_int
        } else {
            LFTNS_NONE as libc::c_int
        }) as luafiletransfernodestatus_t;
        i += 1;
        i;
    }
    if FIL_ReadFileOK((*luafiletransfers).realfilename) != 0 {
        if (strchr(((*luafiletransfers).mode).as_mut_ptr(), 'b' as i32)).is_null() {
            binfilename = strdup(
                va(
                    b"%s/$$$%d%d.tmp\0" as *const u8 as *const libc::c_char,
                    luafiledir.as_mut_ptr(),
                    rand(),
                    rand(),
                ),
            );
            if binfilename.is_null() {
                I_Error(
                    b"SV_PrepareSendLuaFile: Out of memory\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if FIL_ConvertTextFileToBinary((*luafiletransfers).realfilename, binfilename)
                == 0
            {
                I_Error(
                    b"SV_PrepareSendLuaFile: Failed to convert file newlines\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            free((*luafiletransfers).realfilename as *mut libc::c_void);
            (*luafiletransfers).realfilename = binfilename;
        }
        SV_PrepareSendLuaFileToNextNode();
    } else {
        let mut success: uint8_t = 0 as libc::c_int as uint8_t;
        SendNetXCmd(
            XD_LUAFILE,
            &mut success as *mut uint8_t as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_HandleLuaFileSent(mut node: uint8_t) {
    (*luafiletransfers).nodestatus[node as usize] = LFTNS_SENT;
    SV_PrepareSendLuaFileToNextNode();
}
#[no_mangle]
pub unsafe extern "C" fn RemoveLuaFileTransfer() {
    let mut filetransfer: *mut luafiletransfer_t = luafiletransfers;
    if server != 0 && (strchr(((*filetransfer).mode).as_mut_ptr(), 'b' as i32)).is_null()
    {
        remove((*filetransfer).realfilename);
    }
    RemoveLuaFileCallback((*filetransfer).id);
    luafiletransfers = (*filetransfer).next;
    free((*filetransfer).filename as *mut libc::c_void);
    free((*filetransfer).realfilename as *mut libc::c_void);
    free(filetransfer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RemoveAllLuaFileTransfers() {
    while !luafiletransfers.is_null() {
        RemoveLuaFileTransfer();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_AbortLuaFileTransfer(mut node: int32_t) {
    if !luafiletransfers.is_null() {
        if (*luafiletransfers).nodestatus[node as usize] as libc::c_uint
            == LFTNS_ASKED as libc::c_int as libc::c_uint
            || (*luafiletransfers).nodestatus[node as usize] as libc::c_uint
                == LFTNS_SENDING as libc::c_int as libc::c_uint
        {
            SV_PrepareSendLuaFileToNextNode();
        }
        (*luafiletransfers).nodestatus[node as usize] = LFTNS_NONE;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CL_PrepareDownloadLuaFile() {
    if luafiletransfers.is_null() {
        waitingforluafiletransfer = true_0 as libc::c_int;
        return;
    }
    if (*luafiletransfers).ongoing != 0 {
        waitingforluafilecommand = true_0 as libc::c_int;
        return;
    }
    (*netbuffer).packettype = PT_ASKLUAFILE as libc::c_int as uint8_t;
    HSendPacket(
        servernode as int32_t,
        true_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as size_t,
    );
    FreeFileNeeded();
    AllocFileNeeded(1 as libc::c_int);
    fileneedednum = 1 as libc::c_int;
    (*fileneeded.offset(0 as libc::c_int as isize)).type_0 = FILENEEDED_LUAFILE;
    (*fileneeded.offset(0 as libc::c_int as isize)).status = FS_REQUESTED;
    (*fileneeded.offset(0 as libc::c_int as isize))
        .justdownloaded = false_0 as libc::c_int;
    (*fileneeded.offset(0 as libc::c_int as isize))
        .totalsize = 4294967295 as libc::c_uint;
    let ref mut fresh3 = (*fileneeded.offset(0 as libc::c_int as isize)).file;
    *fresh3 = 0 as *mut FILE;
    memset(
        ((*fileneeded.offset(0 as libc::c_int as isize)).md5sum).as_mut_ptr()
            as *mut libc::c_void,
        0 as libc::c_int,
        16 as libc::c_int as libc::c_ulong,
    );
    strcpy(
        ((*fileneeded.offset(0 as libc::c_int as isize)).filename).as_mut_ptr(),
        (*luafiletransfers).realfilename,
    );
    MakePathDirs(
        ((*fileneeded.offset(0 as libc::c_int as isize)).filename).as_mut_ptr(),
    );
    (*luafiletransfers).ongoing = true_0 as libc::c_int;
}
static mut filestosend: int32_t = 0 as libc::c_int;
unsafe extern "C" fn AddFileToSendQueue(
    mut node: int32_t,
    mut fileid: uint8_t,
) -> boolean {
    let mut q: *mut *mut filetx_t = 0 as *mut *mut filetx_t;
    let mut p: *mut filetx_t = 0 as *mut filetx_t;
    let mut wadnum: uint16_t = 0;
    q = &mut (*transfer.as_mut_ptr().offset(node as isize)).txlist;
    while !(*q).is_null() {
        q = &mut (**q).next;
    }
    *q = malloc(::core::mem::size_of::<filetx_t>() as libc::c_ulong) as *mut filetx_t;
    p = *q;
    if p.is_null() {
        I_Error(
            b"AddFileToSendQueue: No more memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<filetx_t>() as libc::c_ulong,
    );
    (*p).id.filename = malloc(512 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if ((*p).id.filename).is_null() {
        I_Error(
            b"AddFileToSendQueue: No more memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    wadnum = GetWadNumFromFileNeededId(fileid);
    if wadnum as libc::c_int == 65535 as libc::c_int {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"fileneeded %d not found in wadfiles\n\0" as *const u8
                        as *const libc::c_char,
                    fileid as libc::c_int,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        if !debugfile.is_null() {
            fputs(
                va(
                    b"Client %d request fileneeded %d: not found\n\0" as *const u8
                        as *const libc::c_char,
                    node,
                    fileid as libc::c_int,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        free((*p).id.filename as *mut libc::c_void);
        free(p as *mut libc::c_void);
        *q = 0 as *mut filetx_t;
        return false_0 as libc::c_int;
    }
    strlcpy(
        (*p).id.filename,
        (**wadfiles.offset(wadnum as isize)).filename,
        512 as libc::c_int as libc::c_ulong,
    );
    if (**wadfiles.offset(wadnum as isize)).filesize
        > cv_maxsend.value as uint32_t * 1024 as libc::c_int as uint32_t
    {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"Client %d request %s: file too big, not sending\n\0" as *const u8
                        as *const libc::c_char,
                    node,
                    (*p).id.filename,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        free((*p).id.filename as *mut libc::c_void);
        free(p as *mut libc::c_void);
        *q = 0 as *mut filetx_t;
        return false_0 as libc::c_int;
    }
    if cv_noticedownload.value != 0 {
        CONS_Printf(
            b"Sending file \"%s\" to node %d (%s)\n\0" as *const u8
                as *const libc::c_char,
            (*p).id.filename,
            node,
            I_GetNodeAddress.expect("non-null function pointer")(node),
        );
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"Sending file %s (id=%d) to %d\n\0" as *const u8 as *const libc::c_char,
                (*p).id.filename,
                fileid as libc::c_int,
                node,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    (*p).ram = SF_FILE as libc::c_int;
    (*p).fileid = fileid;
    (*p).next = 0 as *mut filetx_s;
    filestosend += 1;
    filestosend;
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AddRamToSendQueue(
    mut node: int32_t,
    mut data: *mut libc::c_void,
    mut size: size_t,
    mut freemethod: freemethod_t,
    mut fileid: uint8_t,
) {
    let mut q: *mut *mut filetx_t = 0 as *mut *mut filetx_t;
    let mut p: *mut filetx_t = 0 as *mut filetx_t;
    q = &mut (*transfer.as_mut_ptr().offset(node as isize)).txlist;
    while !(*q).is_null() {
        q = &mut (**q).next;
    }
    *q = malloc(::core::mem::size_of::<filetx_t>() as libc::c_ulong) as *mut filetx_t;
    p = *q;
    if p.is_null() {
        I_Error(
            b"AddRamToSendQueue: No more memory\n\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<filetx_t>() as libc::c_ulong,
    );
    (*p).ram = freemethod as int32_t;
    (*p).id.ram = data as *mut libc::c_char;
    (*p).size = size as uint32_t;
    (*p).fileid = fileid;
    (*p).next = 0 as *mut filetx_s;
    if !debugfile.is_null() {
        fputs(
            va(
                b"Sending ram %p(size:%u) to %d (id=%u)\n\0" as *const u8
                    as *const libc::c_char,
                (*p).id.ram,
                (*p).size,
                node,
                fileid as libc::c_int,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    filestosend += 1;
    filestosend;
}
#[no_mangle]
pub unsafe extern "C" fn AddLuaFileToSendQueue(
    mut node: int32_t,
    mut filename: *const libc::c_char,
) -> boolean {
    let mut q: *mut *mut filetx_t = 0 as *mut *mut filetx_t;
    let mut p: *mut filetx_t = 0 as *mut filetx_t;
    (*luafiletransfers).nodestatus[node as usize] = LFTNS_SENDING;
    q = &mut (*transfer.as_mut_ptr().offset(node as isize)).txlist;
    while !(*q).is_null() {
        q = &mut (**q).next;
    }
    *q = malloc(::core::mem::size_of::<filetx_t>() as libc::c_ulong) as *mut filetx_t;
    p = *q;
    if p.is_null() {
        I_Error(
            b"AddLuaFileToSendQueue: No more memory\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<filetx_t>() as libc::c_ulong,
    );
    (*p).id.filename = malloc(512 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    if ((*p).id.filename).is_null() {
        I_Error(
            b"AddLuaFileToSendQueue: No more memory\n\0" as *const u8
                as *const libc::c_char,
        );
    }
    strlcpy((*p).id.filename, filename, 512 as libc::c_int as libc::c_ulong);
    if !debugfile.is_null() {
        fputs(
            va(
                b"Sending Lua file %s to %d\n\0" as *const u8 as *const libc::c_char,
                filename,
                node,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    (*p).ram = SF_FILE as libc::c_int;
    (*p).next = 0 as *mut filetx_s;
    filestosend += 1;
    filestosend;
    return true_0 as libc::c_int;
}
unsafe extern "C" fn SV_EndFileSend(mut node: int32_t) {
    let mut p: *mut filetx_t = transfer[node as usize].txlist;
    match (*p).ram {
        0 => {
            if cv_noticedownload.value != 0 {
                CONS_Printf(
                    b"Ending file transfer for node %d\n\0" as *const u8
                        as *const libc::c_char,
                    node,
                );
            }
            if !(transfer[node as usize].currentfile).is_null() {
                fclose(transfer[node as usize].currentfile);
            }
            free((*p).id.filename as *mut libc::c_void);
        }
        1 => {
            Z_Free((*p).id.ram as *mut libc::c_void);
        }
        2 => {
            free((*p).id.ram as *mut libc::c_void);
        }
        3 | _ => {}
    }
    transfer[node as usize].txlist = (*p).next;
    free(p as *mut libc::c_void);
    transfer[node as usize].currentfile = 0 as *mut FILE;
    if !(transfer[node as usize].ackedfragments).is_null() {
        free(transfer[node as usize].ackedfragments as *mut libc::c_void);
    }
    transfer[node as usize].ackedfragments = 0 as *mut boolean;
    filestosend -= 1;
    filestosend;
}
#[no_mangle]
pub unsafe extern "C" fn FileSendTicker() {
    static mut currentnode: int32_t = 0 as libc::c_int;
    let mut p: *mut filetx_pak = 0 as *mut filetx_pak;
    let mut fragmentsize: size_t = 0;
    let mut f: *mut filetx_t = 0 as *mut filetx_t;
    let mut packetsent: int32_t = 0;
    let mut ram: int32_t = 0;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    if !luafiletransfers.is_null() {
        i = 1 as libc::c_int;
        while i < 127 as libc::c_int {
            let mut status: luafiletransfernodestatus_t = (*luafiletransfers)
                .nodestatus[i as usize];
            if status as libc::c_uint != LFTNS_NONE as libc::c_int as libc::c_uint
                && status as libc::c_uint != LFTNS_WAITING as libc::c_int as libc::c_uint
                && status as libc::c_uint != LFTNS_SENT as libc::c_int as libc::c_uint
                && I_GetTime() > (*luafiletransfers).nodetimeouts[i as usize]
            {
                Net_ConnectionTimeout(i);
            }
            i += 1;
            i;
        }
    }
    if filestosend == 0 {
        return;
    }
    packetsent = cv_downloadspeed.value;
    (*netbuffer).packettype = PT_FILEFRAGMENT as libc::c_int as uint8_t;
    loop {
        let fresh4 = packetsent;
        packetsent = packetsent - 1;
        if !(fresh4 != 0 && filestosend != 0 as libc::c_int) {
            break;
        }
        i = currentnode;
        j = 0 as libc::c_int;
        while j < 127 as libc::c_int {
            if !(transfer[i as usize].txlist).is_null() {
                break;
            }
            i = (i + 1 as libc::c_int) % 127 as libc::c_int;
            j += 1;
            j;
        }
        if j >= 127 as libc::c_int {
            I_Error(
                b"filestosend=%d but no file to send found\n\0" as *const u8
                    as *const libc::c_char,
                filestosend,
            );
        }
        currentnode = (i + 1 as libc::c_int) % 127 as libc::c_int;
        f = transfer[i as usize].txlist;
        ram = (*f).ram;
        if (transfer[i as usize].currentfile).is_null() {
            if ram == 0 {
                let mut filesize: libc::c_long = 0;
                transfer[i as usize]
                    .currentfile = fopen(
                    (*f).id.filename,
                    b"rb\0" as *const u8 as *const libc::c_char,
                );
                if (transfer[i as usize].currentfile).is_null() {
                    I_Error(
                        b"File %s does not exist\0" as *const u8 as *const libc::c_char,
                        (*f).id.filename,
                    );
                }
                fseek(
                    transfer[i as usize].currentfile,
                    0 as libc::c_int as libc::c_long,
                    2 as libc::c_int,
                );
                filesize = ftell(transfer[i as usize].currentfile);
                if filesize >= 9223372036854775807 as libc::c_long {
                    I_Error(
                        b"filesize of %s is too large\0" as *const u8
                            as *const libc::c_char,
                        (*f).id.filename,
                    );
                }
                if filesize == -(1 as libc::c_int) as libc::c_long {
                    I_Error(
                        b"Error getting filesize of %s\0" as *const u8
                            as *const libc::c_char,
                        (*f).id.filename,
                    );
                }
                (*f).size = filesize as uint32_t;
                fseek(
                    transfer[i as usize].currentfile,
                    0 as libc::c_int as libc::c_long,
                    0 as libc::c_int,
                );
            } else {
                transfer[i as usize].currentfile = 1 as libc::c_int as *mut FILE;
            }
            transfer[i as usize].iteration = 1 as libc::c_int as uint8_t;
            transfer[i as usize].ackediteration = 0 as libc::c_int as uint8_t;
            transfer[i as usize].position = 0 as libc::c_int as uint32_t;
            transfer[i as usize].ackedsize = 0 as libc::c_int as uint32_t;
            transfer[i as usize]
                .ackedfragments = calloc(
                ((*f).size as libc::c_ulong)
                    .wrapping_div(
                        (software_MAXPACKETLENGTH as libc::c_ulong)
                            .wrapping_sub(
                                (12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong),
                            ),
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ::core::mem::size_of::<boolean>() as libc::c_ulong,
            ) as *mut boolean;
            if (transfer[i as usize].ackedfragments).is_null() {
                I_Error(
                    b"FileSendTicker: No more memory\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            transfer[i as usize].dontsenduntil = 0 as libc::c_int as tic_t;
        }
        if I_GetTime() < transfer[i as usize].dontsenduntil {
            continue;
        }
        while *(transfer[i as usize].ackedfragments)
            .offset(
                (transfer[i as usize].position as libc::c_ulong)
                    .wrapping_div(
                        (software_MAXPACKETLENGTH as libc::c_ulong)
                            .wrapping_sub(
                                (12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong),
                            ),
                    ) as isize,
            ) != 0
        {
            transfer[i as usize]
                .position = (transfer[i as usize].position as libc::c_ulong)
                .wrapping_add(
                    (software_MAXPACKETLENGTH as libc::c_ulong)
                        .wrapping_sub(
                            (12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong),
                        ),
                ) as uint32_t as uint32_t;
            if transfer[i as usize].position >= (*f).size {
                if (transfer[i as usize].ackediteration as libc::c_int)
                    < transfer[i as usize].iteration as libc::c_int
                {
                    transfer[i as usize]
                        .dontsenduntil = (I_GetTime())
                        .wrapping_add((35 as libc::c_int / 2 as libc::c_int) as tic_t);
                }
                transfer[i as usize].position = 0 as libc::c_int as uint32_t;
                transfer[i as usize]
                    .iteration = (transfer[i as usize].iteration).wrapping_add(1);
                transfer[i as usize].iteration;
            }
        }
        p = &mut (*netbuffer).u.filetxpak;
        fragmentsize = (software_MAXPACKETLENGTH as libc::c_ulong)
            .wrapping_sub((12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong));
        if (((*f).size).wrapping_sub(transfer[i as usize].position) as size_t)
            < fragmentsize
        {
            fragmentsize = ((*f).size).wrapping_sub(transfer[i as usize].position)
                as size_t;
        }
        if ram != 0 {
            M_Memcpy
                .expect(
                    "non-null function pointer",
                )(
                ((*p).data).as_mut_ptr() as *mut libc::c_void,
                &mut *((*f).id.ram)
                    .offset(
                        (*transfer.as_mut_ptr().offset(i as isize)).position as isize,
                    ) as *mut libc::c_char as *const libc::c_void,
                fragmentsize,
            );
        } else {
            fseek(
                transfer[i as usize].currentfile,
                transfer[i as usize].position as libc::c_long,
                0 as libc::c_int,
            );
            if fread(
                ((*p).data).as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                fragmentsize,
                transfer[i as usize].currentfile,
            ) != fragmentsize
            {
                I_Error(
                    b"FileSendTicker: can't read %s byte on %s at %d because %s\0"
                        as *const u8 as *const libc::c_char,
                    sizeu1(fragmentsize),
                    (*f).id.filename,
                    transfer[i as usize].position,
                    M_FileError(transfer[i as usize].currentfile),
                );
            }
        }
        (*p).iteration = transfer[i as usize].iteration;
        (*p).position = transfer[i as usize].position as int32_t as uint32_t;
        (*p).fileid = (*f).fileid;
        (*p).filesize = (*f).size as int32_t as uint32_t;
        (*p)
            .size = (software_MAXPACKETLENGTH as libc::c_ulong)
            .wrapping_sub((12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong))
            as uint16_t as int16_t as uint16_t;
        if !(HSendPacket(
            i,
            false_0 as libc::c_int,
            0 as libc::c_int as uint8_t,
            (12 as libc::c_ulong).wrapping_add(fragmentsize),
        ) != 0)
        {
            break;
        }
        transfer[i as usize]
            .position = (transfer[i as usize].position as size_t)
            .wrapping_add(fragmentsize) as uint32_t;
        if transfer[i as usize].position >= (*f).size {
            if (transfer[i as usize].ackediteration as libc::c_int)
                < transfer[i as usize].iteration as libc::c_int
            {
                transfer[i as usize]
                    .dontsenduntil = (I_GetTime())
                    .wrapping_add((35 as libc::c_int / 2 as libc::c_int) as tic_t);
            }
            transfer[i as usize].position = 0 as libc::c_int as uint32_t;
            transfer[i as usize]
                .iteration = (transfer[i as usize].iteration).wrapping_add(1);
            transfer[i as usize].iteration;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn PT_FileAck() {
    let mut packet: *mut fileack_pak = &mut (*netbuffer).u.fileack;
    let mut node: int32_t = (*doomcom).remotenode as int32_t;
    let mut trans: *mut filetran_t = &mut *transfer.as_mut_ptr().offset(node as isize)
        as *mut filetran_t;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    if !(!((*trans).txlist).is_null()
        && (*packet).fileid as libc::c_int == (*(*trans).txlist).fileid as libc::c_int)
    {
        return;
    }
    if ((*packet).numsegments as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<fileacksegment_t>() as libc::c_ulong)
        != ((*doomcom).datalength as libc::c_ulong)
            .wrapping_sub(8 as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<fileack_pak>() as libc::c_ulong)
    {
        Net_CloseConnection(node);
        return;
    }
    if (*packet).iteration as libc::c_int > (*trans).ackediteration as libc::c_int {
        (*trans).ackediteration = (*packet).iteration;
        if (*trans).ackediteration as libc::c_int
            >= (*trans).iteration as libc::c_int - 1 as libc::c_int
        {
            (*trans).dontsenduntil = 0 as libc::c_int as tic_t;
        }
    }
    i = 0 as libc::c_int;
    while i < (*packet).numsegments as libc::c_int {
        let mut segment: *mut fileacksegment_t = &mut *((*packet).segments)
            .as_mut_ptr()
            .offset(i as isize) as *mut fileacksegment_t;
        j = 0 as libc::c_int;
        while j < 32 as libc::c_int {
            if (*segment).acks as int32_t & (1 as libc::c_int) << j != 0 {
                if ((*segment).start as int32_t as libc::c_ulong)
                    .wrapping_mul(
                        (software_MAXPACKETLENGTH as libc::c_ulong)
                            .wrapping_sub(
                                (12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong),
                            ),
                    ) >= (*(*trans).txlist).size as libc::c_ulong
                {
                    Net_CloseConnection(node);
                    return;
                }
                if *((*trans).ackedfragments)
                    .offset(((*segment).start as int32_t + j) as isize) == 0
                {
                    *((*trans).ackedfragments)
                        .offset(
                            ((*segment).start as int32_t + j) as isize,
                        ) = true_0 as libc::c_int;
                    (*trans)
                        .ackedsize = ((*trans).ackedsize as libc::c_ulong)
                        .wrapping_add(
                            (software_MAXPACKETLENGTH as libc::c_ulong)
                                .wrapping_sub(
                                    (12 as libc::c_ulong).wrapping_add(8 as libc::c_ulong),
                                ),
                        ) as uint32_t as uint32_t;
                    if (*trans).ackedsize == (*(*trans).txlist).size {
                        SV_EndFileSend(node);
                        return;
                    }
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PT_FileReceived() {
    let mut trans: *mut filetx_t = transfer[(*doomcom).remotenode as usize].txlist;
    if !trans.is_null()
        && (*netbuffer).u.filereceived as libc::c_int == (*trans).fileid as libc::c_int
    {
        SV_EndFileSend((*doomcom).remotenode as int32_t);
    }
}
unsafe extern "C" fn SendAckPacket(mut packet: *mut fileack_pak, mut fileid: uint8_t) {
    let mut packetsize: size_t = 0;
    let mut i: int32_t = 0;
    packetsize = (::core::mem::size_of::<fileack_pak>() as libc::c_ulong)
        .wrapping_add(
            ((*packet).numsegments as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<fileacksegment_t>() as libc::c_ulong,
                ),
        );
    (*packet).fileid = fileid;
    i = 0 as libc::c_int;
    while i < (*packet).numsegments as libc::c_int {
        (*((*packet).segments).as_mut_ptr().offset(i as isize))
            .start = (*((*packet).segments).as_mut_ptr().offset(i as isize)).start
            as int32_t as uint32_t;
        (*((*packet).segments).as_mut_ptr().offset(i as isize))
            .acks = (*((*packet).segments).as_mut_ptr().offset(i as isize)).acks
            as int32_t as uint32_t;
        i += 1;
        i;
    }
    (*netbuffer).packettype = PT_FILEACK as libc::c_int as uint8_t;
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        &mut (*netbuffer).u.fileack as *mut fileack_pak as *mut libc::c_void,
        packet as *const libc::c_void,
        packetsize,
    );
    HSendPacket(
        servernode as int32_t,
        false_0 as libc::c_int,
        0 as libc::c_int as uint8_t,
        packetsize,
    );
    memset(
        packet as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<fileack_pak>() as libc::c_ulong)
            .wrapping_add(512 as libc::c_int as libc::c_ulong),
    );
}
unsafe extern "C" fn AddFragmentToAckPacket(
    mut packet: *mut fileack_pak,
    mut iteration: uint8_t,
    mut fragmentpos: uint32_t,
    mut fileid: uint8_t,
) {
    let mut segment: *mut fileacksegment_t = &mut *((*packet).segments)
        .as_mut_ptr()
        .offset(((*packet).numsegments as libc::c_int - 1 as libc::c_int) as isize)
        as *mut fileacksegment_t;
    (*packet)
        .iteration = (if (*packet).iteration as libc::c_int > iteration as libc::c_int {
        (*packet).iteration as libc::c_int
    } else {
        iteration as libc::c_int
    }) as uint8_t;
    if (*packet).numsegments as libc::c_int == 0 as libc::c_int
        || fragmentpos < (*segment).start
        || fragmentpos.wrapping_sub((*segment).start) >= 32 as libc::c_int as uint32_t
    {
        if (((*packet).numsegments as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<fileacksegment_t>() as libc::c_ulong)
            > 512 as libc::c_int as libc::c_ulong
        {
            SendAckPacket(packet, fileid);
        }
        (*packet).numsegments = ((*packet).numsegments).wrapping_add(1);
        (*packet).numsegments;
        segment = &mut *((*packet).segments)
            .as_mut_ptr()
            .offset(((*packet).numsegments as libc::c_int - 1 as libc::c_int) as isize)
            as *mut fileacksegment_t;
        (*segment).start = fragmentpos;
    }
    (*segment).acks
        |= ((1 as libc::c_int) << fragmentpos.wrapping_sub((*segment).start))
            as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn FileReceiveTicker() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < fileneedednum {
        let mut file: *mut fileneeded_t = &mut *fileneeded.offset(i as isize)
            as *mut fileneeded_t;
        if (*file).status as libc::c_uint
            == FS_DOWNLOADING as libc::c_int as libc::c_uint
        {
            if lasttimeackpacketsent.wrapping_sub(I_GetTime())
                > (35 as libc::c_int / 2 as libc::c_int) as tic_t
            {
                SendAckPacket((*file).ackpacket, i as uint8_t);
            }
            if (*file).ackresendposition != 4294967295 as libc::c_uint
                && (*file).status as libc::c_uint
                    == FS_DOWNLOADING as libc::c_int as libc::c_uint
            {
                let mut j: int32_t = 0;
                j = 0 as libc::c_int;
                while j < 2048 as libc::c_int {
                    if *((*file).receivedfragments)
                        .offset((*file).ackresendposition as isize) != 0
                    {
                        AddFragmentToAckPacket(
                            (*file).ackpacket,
                            (*file).iteration,
                            (*file).ackresendposition,
                            i as uint8_t,
                        );
                    }
                    (*file)
                        .ackresendposition = ((*file).ackresendposition).wrapping_add(1);
                    (*file).ackresendposition;
                    if (*file).ackresendposition * (*file).fragmentsize
                        >= (*file).totalsize
                    {
                        (*file).ackresendposition = 4294967295 as libc::c_uint;
                        break;
                    } else {
                        j += 1;
                        j;
                    }
                }
            }
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PT_FileFragment() {
    let mut filenum: int32_t = (*netbuffer).u.filetxpak.fileid as int32_t;
    let mut file: *mut fileneeded_t = &mut *fileneeded.offset(filenum as isize)
        as *mut fileneeded_t;
    let mut fragmentpos: uint32_t = (*netbuffer).u.filetxpak.position as int32_t
        as uint32_t;
    let mut fragmentsize: uint16_t = (*netbuffer).u.filetxpak.size as int16_t
        as uint16_t;
    let mut boundedfragmentsize: uint16_t = ((*doomcom).datalength as libc::c_ulong)
        .wrapping_sub(8 as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<filetx_pak>() as libc::c_ulong) as uint16_t;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    if file.is_null() {
        return;
    }
    filename = va(
        b"%s\0" as *const u8 as *const libc::c_char,
        ((*file).filename).as_mut_ptr(),
    );
    nameonly(filename);
    if !(strcmp(filename, b"srb2.pk3\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(filename, b"zones.pk3\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(filename, b"player.dta\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(filename, b"patch.pk3\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(filename, b"music.dta\0" as *const u8 as *const libc::c_char) != 0)
    {
        I_Error(
            b"Tried to download \"%s\"\0" as *const u8 as *const libc::c_char,
            filename,
        );
    }
    filename = ((*file).filename).as_mut_ptr();
    if filenum >= fileneedednum {
        if !debugfile.is_null() {
            fputs(
                va(
                    b"fileframent not needed %d>%d\n\0" as *const u8
                        as *const libc::c_char,
                    filenum,
                    fileneedednum,
                ),
                debugfile,
            );
            fflush(debugfile);
        }
        return;
    }
    if (*file).status as libc::c_uint == FS_REQUESTED as libc::c_int as libc::c_uint {
        if !((*file).file).is_null() {
            I_Error(
                b"PT_FileFragment: already open file\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        (*file).status = FS_DOWNLOADING;
        (*file).fragmentsize = fragmentsize as uint32_t;
        (*file).iteration = 0 as libc::c_int as uint8_t;
        (*file)
            .ackpacket = calloc(
            1 as libc::c_int as libc::c_ulong,
            (::core::mem::size_of::<fileack_pak>() as libc::c_ulong)
                .wrapping_add(512 as libc::c_int as libc::c_ulong),
        ) as *mut fileack_pak;
        if ((*file).ackpacket).is_null() {
            I_Error(
                b"FileSendTicker: No more memory\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if CL_CanResumeDownload(file) != 0 {
            (*file).file = fopen(filename, b"r+b\0" as *const u8 as *const libc::c_char);
            if ((*file).file).is_null() {
                I_Error(
                    b"Can't reopen file %s: %s\0" as *const u8 as *const libc::c_char,
                    filename,
                    strerror(*__errno_location()),
                );
            }
            CONS_Printf(b"\r%s...\n\0" as *const u8 as *const libc::c_char, filename);
            CONS_Printf(b"Resuming download...\n\0" as *const u8 as *const libc::c_char);
            (*file).currentsize = (*pauseddownload).currentsize;
            (*file).receivedfragments = (*pauseddownload).receivedfragments;
            (*file).ackresendposition = 0 as libc::c_int as uint32_t;
            free(pauseddownload as *mut libc::c_void);
            pauseddownload = 0 as *mut pauseddownload_t;
        } else {
            CL_AbortDownloadResume();
            (*file).file = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
            if ((*file).file).is_null() {
                I_Error(
                    b"Can't create file %s: %s\0" as *const u8 as *const libc::c_char,
                    filename,
                    strerror(*__errno_location()),
                );
            }
            CONS_Printf(b"\r%s...\n\0" as *const u8 as *const libc::c_char, filename);
            (*file).currentsize = 0 as libc::c_int as uint32_t;
            (*file).totalsize = (*netbuffer).u.filetxpak.filesize as int32_t as uint32_t;
            (*file).ackresendposition = 4294967295 as libc::c_uint;
            (*file)
                .receivedfragments = calloc(
                ((*file).totalsize / fragmentsize as uint32_t)
                    .wrapping_add(1 as libc::c_int as uint32_t) as libc::c_ulong,
                ::core::mem::size_of::<boolean>() as libc::c_ulong,
            ) as *mut boolean;
            if ((*file).receivedfragments).is_null() {
                I_Error(
                    b"FileSendTicker: No more memory\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        lasttimeackpacketsent = I_GetTime();
    }
    if (*file).status as libc::c_uint == FS_DOWNLOADING as libc::c_int as libc::c_uint {
        if fragmentpos >= (*file).totalsize {
            I_Error(b"Invalid file fragment\n\0" as *const u8 as *const libc::c_char);
        }
        (*file)
            .iteration = (if (*file).iteration as libc::c_int
            > (*netbuffer).u.filetxpak.iteration as libc::c_int
        {
            (*file).iteration as libc::c_int
        } else {
            (*netbuffer).u.filetxpak.iteration as libc::c_int
        }) as uint8_t;
        if *((*file).receivedfragments)
            .offset((fragmentpos / fragmentsize as uint32_t) as isize) == 0
        {
            *((*file).receivedfragments)
                .offset(
                    (fragmentpos / fragmentsize as uint32_t) as isize,
                ) = true_0 as libc::c_int;
            fseek((*file).file, fragmentpos as libc::c_long, 0 as libc::c_int);
            if fragmentsize as libc::c_int != 0
                && fwrite(
                    ((*netbuffer).u.filetxpak.data).as_mut_ptr() as *const libc::c_void,
                    boundedfragmentsize as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    (*file).file,
                ) != 1 as libc::c_int as libc::c_ulong
            {
                I_Error(
                    b"Can't write to %s: %s\n\0" as *const u8 as *const libc::c_char,
                    filename,
                    M_FileError((*file).file),
                );
            }
            (*file)
                .currentsize = ((*file).currentsize)
                .wrapping_add(boundedfragmentsize as uint32_t);
            AddFragmentToAckPacket(
                (*file).ackpacket,
                (*file).iteration,
                fragmentpos / fragmentsize as uint32_t,
                filenum as uint8_t,
            );
            if (*file).currentsize == (*file).totalsize {
                fclose((*file).file);
                (*file).file = 0 as *mut FILE;
                free((*file).receivedfragments as *mut libc::c_void);
                free((*file).ackpacket as *mut libc::c_void);
                (*file).status = FS_FOUND;
                (*file).justdownloaded = true_0 as libc::c_int;
                CONS_Printf(
                    b"Downloading %s...(done)\n\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                (*netbuffer).packettype = PT_FILERECEIVED as libc::c_int as uint8_t;
                (*netbuffer).u.filereceived = filenum as uint8_t;
                HSendPacket(
                    servernode as int32_t,
                    true_0 as libc::c_int,
                    0 as libc::c_int as uint8_t,
                    1 as libc::c_int as size_t,
                );
                if !luafiletransfers.is_null() {
                    (*netbuffer).packettype = PT_HASLUAFILE as libc::c_int as uint8_t;
                    HSendPacket(
                        servernode as int32_t,
                        true_0 as libc::c_int,
                        0 as libc::c_int as uint8_t,
                        0 as libc::c_int as size_t,
                    );
                    FreeFileNeeded();
                }
            }
        } else {
            AddFragmentToAckPacket(
                (*file).ackpacket,
                (*file).iteration,
                fragmentpos / fragmentsize as uint32_t,
                filenum as uint8_t,
            );
        }
    } else if (*file).justdownloaded == 0 {
        let mut s: *const libc::c_char = 0 as *const libc::c_char;
        match (*file).status as libc::c_uint {
            1 => {
                s = b"FS_NOTFOUND\0" as *const u8 as *const libc::c_char;
            }
            2 => {
                s = b"FS_FOUND\0" as *const u8 as *const libc::c_char;
            }
            5 => {
                s = b"FS_OPEN\0" as *const u8 as *const libc::c_char;
            }
            6 => {
                s = b"FS_MD5SUMBAD\0" as *const u8 as *const libc::c_char;
            }
            _ => {
                s = b"unknown\0" as *const u8 as *const libc::c_char;
            }
        }
        I_Error(
            b"Received a file not requested (file id: %d, file status: %s)\n\0"
                as *const u8 as *const libc::c_char,
            filenum,
            s,
        );
    }
    lastfilenum = filenum;
}
#[no_mangle]
pub unsafe extern "C" fn SendingFile(mut node: int32_t) -> boolean {
    return (transfer[node as usize].txlist != 0 as *mut libc::c_void as *mut filetx_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_AbortSendFiles(mut node: int32_t) {
    while !(transfer[node as usize].txlist).is_null() {
        SV_EndFileSend(node);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CloseNetFile() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 127 as libc::c_int {
        SV_AbortSendFiles(i);
        i += 1;
        i;
    }
    if !fileneeded.is_null() {
        i = 0 as libc::c_int;
        while i < fileneedednum {
            if (*fileneeded.offset(i as isize)).status as libc::c_uint
                == FS_DOWNLOADING as libc::c_int as libc::c_uint
                && !((*fileneeded.offset(i as isize)).file).is_null()
            {
                fclose((*fileneeded.offset(i as isize)).file);
                free((*fileneeded.offset(i as isize)).ackpacket as *mut libc::c_void);
                if pauseddownload.is_null()
                    && ((*fileneeded.offset(i as isize)).type_0 as libc::c_uint
                        == FILENEEDED_WAD as libc::c_int as libc::c_uint
                        || i != 0 as libc::c_int)
                {
                    pauseddownload = malloc(
                        ::core::mem::size_of::<pauseddownload_t>() as libc::c_ulong,
                    ) as *mut pauseddownload_t;
                    if pauseddownload.is_null() {
                        I_Error(
                            b"CloseNetFile: No more memory\n\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    strcpy(
                        ((*pauseddownload).filename).as_mut_ptr(),
                        ((*fileneeded.offset(i as isize)).filename).as_mut_ptr(),
                    );
                    memcpy(
                        ((*pauseddownload).md5sum).as_mut_ptr() as *mut libc::c_void,
                        ((*fileneeded.offset(i as isize)).md5sum).as_mut_ptr()
                            as *const libc::c_void,
                        16 as libc::c_int as libc::c_ulong,
                    );
                    (*pauseddownload)
                        .currentsize = (*fileneeded.offset(i as isize)).currentsize;
                    (*pauseddownload)
                        .receivedfragments = (*fileneeded.offset(i as isize))
                        .receivedfragments;
                    (*pauseddownload)
                        .fragmentsize = (*fileneeded.offset(i as isize)).fragmentsize;
                } else {
                    free(
                        (*fileneeded.offset(i as isize)).receivedfragments
                            as *mut libc::c_void,
                    );
                    remove(((*fileneeded.offset(i as isize)).filename).as_mut_ptr());
                }
            }
            i += 1;
            i;
        }
    }
    FreeFileNeeded();
}
#[no_mangle]
pub unsafe extern "C" fn Command_Downloads_f() {
    let mut node: int32_t = 0;
    node = 0 as libc::c_int;
    while node < 127 as libc::c_int {
        if !(transfer[node as usize].txlist).is_null()
            && (*transfer[node as usize].txlist).ram == SF_FILE as libc::c_int
        {
            let mut name: *const libc::c_char = (*transfer[node as usize].txlist)
                .id
                .filename;
            let mut position: uint32_t = transfer[node as usize].ackedsize;
            let mut size: uint32_t = (*transfer[node as usize].txlist).size;
            let mut ratecolor: libc::c_char = 0;
            if size == 0 {
                size = 1 as libc::c_int as uint32_t;
            }
            name = &*name
                .offset(
                    ((strlen
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                        ) -> libc::c_ulong)(name))
                        .wrapping_sub(
                            (nameonlylength
                                as unsafe extern "C" fn(
                                    *const libc::c_char,
                                ) -> size_t)(name),
                        ) as isize,
                ) as *const libc::c_char;
            match 4 as libc::c_int as uint32_t
                * position.wrapping_sub(1 as libc::c_int as uint32_t) / size
            {
                0 => {
                    ratecolor = -123i32 as libc::c_char;
                }
                1 => {
                    ratecolor = -121i32 as libc::c_char;
                }
                2 => {
                    ratecolor = -126i32 as libc::c_char;
                }
                3 => {
                    ratecolor = -125i32 as libc::c_char;
                }
                _ => {
                    ratecolor = -128i32 as libc::c_char;
                }
            }
            CONS_Printf(
                b"%2d  %c%s  \0" as *const u8 as *const libc::c_char,
                node,
                ratecolor as libc::c_int,
                name,
            );
            CONS_Printf(
                b"\x80%uK\x84/\x80%uK \0" as *const u8 as *const libc::c_char,
                position / 1024 as libc::c_int as uint32_t,
                size / 1024 as libc::c_int as uint32_t,
            );
            CONS_Printf(
                b"\x80(%c%u%%\x80)  \0" as *const u8 as *const libc::c_char,
                ratecolor as libc::c_int,
                (100.0f64 * position as libc::c_double / size as libc::c_double)
                    as uint32_t,
            );
            CONS_Printf(
                b"%s\n\0" as *const u8 as *const libc::c_char,
                I_GetNodeAddress.expect("non-null function pointer")(node),
            );
        }
        node += 1;
        node;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nameonly(mut s: *mut libc::c_char) {
    let mut j: size_t = 0;
    let mut len: size_t = 0;
    let mut ns: *mut libc::c_void = 0 as *mut libc::c_void;
    j = strlen(s);
    while j != -(1 as libc::c_int) as size_t {
        if *s.offset(j as isize) as libc::c_int == '\\' as i32
            || *s.offset(j as isize) as libc::c_int == ':' as i32
            || *s.offset(j as isize) as libc::c_int == '/' as i32
        {
            ns = &mut *s.offset(j.wrapping_add(1 as libc::c_int as size_t) as isize)
                as *mut libc::c_char as *mut libc::c_void;
            len = strlen(ns as *const libc::c_char);
            memmove(
                s as *mut libc::c_void,
                ns,
                len.wrapping_add(1 as libc::c_int as size_t),
            );
            return;
        }
        j = j.wrapping_sub(1);
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn nameonlylength(mut s: *const libc::c_char) -> size_t {
    let mut j: size_t = 0;
    let mut len: size_t = strlen(s);
    j = len;
    while j != -(1 as libc::c_int) as size_t {
        if *s.offset(j as isize) as libc::c_int == '\\' as i32
            || *s.offset(j as isize) as libc::c_int == ':' as i32
            || *s.offset(j as isize) as libc::c_int == '/' as i32
        {
            return len.wrapping_sub(j).wrapping_sub(1 as libc::c_int as size_t);
        }
        j = j.wrapping_sub(1);
        j;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn checkfilemd5(
    mut filename: *mut libc::c_char,
    mut wantedmd5sum: *const uint8_t,
) -> filestatus_t {
    let mut fhandle: *mut FILE = 0 as *mut FILE;
    let mut md5sum: [uint8_t; 16] = [0; 16];
    if wantedmd5sum.is_null() {
        return FS_FOUND;
    }
    fhandle = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if !fhandle.is_null() {
        md5_stream(fhandle, md5sum.as_mut_ptr() as *mut libc::c_void);
        fclose(fhandle);
        if memcmp(
            wantedmd5sum as *const libc::c_void,
            md5sum.as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            return FS_FOUND;
        }
        return FS_MD5SUMBAD;
    }
    I_Error(
        b"Couldn't open %s for md5 check\0" as *const u8 as *const libc::c_char,
        filename,
    );
}
#[no_mangle]
pub unsafe extern "C" fn findfile(
    mut filename: *mut libc::c_char,
    mut wantedmd5sum: *const uint8_t,
    mut completepath: boolean,
) -> filestatus_t {
    let mut homecheck: filestatus_t = FS_NOTCHECKED;
    let mut badmd5: boolean = false_0 as libc::c_int;
    homecheck = filesearch(
        filename,
        srb2home.as_mut_ptr(),
        wantedmd5sum,
        completepath,
        10 as libc::c_int,
    );
    if homecheck as libc::c_uint == FS_FOUND as libc::c_int as libc::c_uint {
        return FS_FOUND
    } else if homecheck as libc::c_uint == FS_MD5SUMBAD as libc::c_int as libc::c_uint {
        badmd5 = true_0 as libc::c_int;
    }
    homecheck = filesearch(
        filename,
        srb2path.as_mut_ptr(),
        wantedmd5sum,
        completepath,
        10 as libc::c_int,
    );
    if homecheck as libc::c_uint == FS_FOUND as libc::c_int as libc::c_uint {
        return FS_FOUND
    } else if homecheck as libc::c_uint == FS_MD5SUMBAD as libc::c_int as libc::c_uint {
        badmd5 = true_0 as libc::c_int;
    }
    homecheck = filesearch(
        filename,
        b".\0" as *const u8 as *const libc::c_char,
        wantedmd5sum,
        completepath,
        10 as libc::c_int,
    );
    if homecheck as libc::c_uint != FS_NOTFOUND as libc::c_int as libc::c_uint {
        return homecheck;
    }
    return (if badmd5 != 0 {
        FS_MD5SUMBAD as libc::c_int
    } else {
        FS_NOTFOUND as libc::c_int
    }) as filestatus_t;
}
#[no_mangle]
pub unsafe extern "C" fn findfolder(mut path: *const libc::c_char) -> filestatus_t {
    if concatpaths(path, 0 as *const libc::c_char) == 1 as libc::c_int {
        return FS_FOUND;
    }
    if concatpaths(path, srb2home.as_mut_ptr()) == 1 as libc::c_int {
        return FS_FOUND;
    }
    if concatpaths(path, srb2path.as_mut_ptr()) == 1 as libc::c_int {
        return FS_FOUND;
    }
    if concatpaths(path, b".\0" as *const u8 as *const libc::c_char) == 1 as libc::c_int
    {
        return FS_FOUND;
    }
    return FS_NOTFOUND;
}
