use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    static mut serverrunning: boolean;
    fn COM_AddCommand(name: *const libc::c_char, func: com_func_t, flags: com_flags_t);
    fn CV_RegisterVar(variable: *mut consvar_t);
    fn CV_StealthSet(var: *mut consvar_t, value: *const libc::c_char);
    fn HMS_register() -> libc::c_int;
    fn HMS_set_api(api: *mut libc::c_char);
    fn HMS_unlist() -> libc::c_int;
    fn HMS_update() -> libc::c_int;
    static mut cv_masterserver_timeout: consvar_t;
    static mut cv_masterserver_debug: consvar_t;
    static mut cv_masterserver_token: consvar_t;
    fn HMS_fetch_servers(
        list: *mut msg_server_t,
        room: libc::c_int,
        id: libc::c_int,
    ) -> *mut msg_server_t;
    fn HMS_fetch_rooms(joining: libc::c_int, id: libc::c_int) -> libc::c_int;
    fn HMS_compare_mod_version(
        buffer: *mut libc::c_char,
        size_of_buffer: size_t,
    ) -> libc::c_int;
    fn HMS_list_servers();
    fn M_StartMessage(
        string: *const libc::c_char,
        routine: *mut libc::c_void,
        itemtype: menumessagetype_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct msg_rooms_t {
    pub header: msg_header_t,
    pub id: int32_t,
    pub name: [libc::c_char; 32],
    pub motd: [libc::c_char; 255],
}
pub type menumessagetype_t = libc::c_uint;
pub const MM_EVENTHANDLER: menumessagetype_t = 2;
pub const MM_YESNO: menumessagetype_t = 1;
pub const MM_NOTHING: menumessagetype_t = 0;
static mut MSId: libc::c_int = 0;
static mut MSRegisteredId: libc::c_int = -(1 as libc::c_int);
static mut MSRegistered: boolean = 0;
static mut MSInProgress: boolean = 0;
static mut MSUpdateAgain: boolean = 0;
static mut MSLastPing: time_t = 0;
static mut masterserver_update_rate_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 60 as libc::c_int,
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
pub static mut cv_masterserver: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"masterserver\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"https://ds.ms.srb2.org/MS/0\0" as *const u8
                as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_CALL as libc::c_int,
            PossibleValue: 0 as *const CV_PossibleValue_t as *mut CV_PossibleValue_t,
            func: Some(MasterServer_OnChange as unsafe extern "C" fn() -> ()),
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
pub static mut cv_servername: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"servername\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"SRB2 server\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_NETVAR as libc::c_int
                | CV_CALL as libc::c_int | CV_NOINIT as libc::c_int
                | CV_ALLOWLUA as libc::c_int,
            PossibleValue: 0 as *const CV_PossibleValue_t as *mut CV_PossibleValue_t,
            func: Some(Update_parameters as unsafe extern "C" fn() -> ()),
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
pub static mut cv_masterserver_update_rate: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"masterserver_update_rate\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"15\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_CALL as libc::c_int
                | CV_NOINIT as libc::c_int,
            PossibleValue: masterserver_update_rate_cons_t.as_ptr() as *mut _,
            func: Some(Update_parameters as unsafe extern "C" fn() -> ()),
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
pub static mut ms_RoomId: int16_t = -(1 as libc::c_int) as int16_t;
#[no_mangle]
pub static mut current_port: uint16_t = 0 as libc::c_int as uint16_t;
#[no_mangle]
pub static mut room_list: [msg_rooms_t; 17] = [msg_rooms_t {
    header: msg_header_t { buffer: [0; 16] },
    id: 0,
    name: [0; 32],
    motd: [0; 255],
}; 17];
#[no_mangle]
pub unsafe extern "C" fn AddMServCommands() {
    CV_RegisterVar(&mut cv_masterserver);
    CV_RegisterVar(&mut cv_masterserver_update_rate);
    CV_RegisterVar(&mut cv_masterserver_timeout);
    CV_RegisterVar(&mut cv_masterserver_debug);
    CV_RegisterVar(&mut cv_masterserver_token);
    CV_RegisterVar(&mut cv_servername);
    COM_AddCommand(
        b"listserv\0" as *const u8 as *const libc::c_char,
        Some(Command_Listserv_f as unsafe extern "C" fn() -> ()),
        0 as com_flags_t,
    );
    COM_AddCommand(
        b"masterserver_update\0" as *const u8 as *const libc::c_char,
        Some(Update_parameters as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
}
unsafe extern "C" fn WarnGUI() {
    M_StartMessage(
        b"There was a problem connecting to\nthe Master Server\n\nCheck the console for details.\n\0"
            as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        MM_NOTHING,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GetShortServersList(
    mut room: int32_t,
    mut id: libc::c_int,
) -> *mut msg_server_t {
    let mut server_list: *mut msg_server_t = 0 as *mut msg_server_t;
    server_list = malloc(
        ((127 as libc::c_int - 1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<msg_server_t>() as libc::c_ulong),
    ) as *mut msg_server_t;
    if !(HMS_fetch_servers(server_list, room, id)).is_null() {
        return server_list
    } else {
        free(server_list as *mut libc::c_void);
        WarnGUI();
        return 0 as *mut msg_server_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetRoomsList(
    mut hosting: boolean,
    mut id: libc::c_int,
) -> int32_t {
    if HMS_fetch_rooms((hosting == 0) as libc::c_int, id) != 0 {
        return 1 as libc::c_int
    } else {
        WarnGUI();
        return -(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetMODVersion(mut id: libc::c_int) -> *mut libc::c_char {
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    buffer = malloc(16 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    c = HMS_compare_mod_version(buffer, 16 as libc::c_int as size_t);
    if c > 0 as libc::c_int {
        return buffer
    } else {
        free(buffer as *mut libc::c_void);
        if c == 0 {
            WarnGUI();
        }
        return 0 as *mut libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn GetMODVersion_Console() {
    let mut buffer: [libc::c_char; 16] = [0; 16];
    if HMS_compare_mod_version(
        buffer.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    ) > 0 as libc::c_int
    {
        I_Error(
            b"A new update is available for SRB2.\nPlease visit SRB2.org to download it.\n\nYou are using version: %s\nThe newest version is: %s\n\nThis update is required for online play using the Master Server.\nYou will not be able to connect to the Master Server\nuntil you update to the newest version of the game.\n\0"
                as *const u8 as *const libc::c_char,
            b"v2.2.13\0" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr(),
        );
    }
}
unsafe extern "C" fn Command_Listserv_f() {
    CONS_Printf(b"Retrieving server list...\n\0" as *const u8 as *const libc::c_char);
    HMS_list_servers();
}
unsafe extern "C" fn Finish_registration() {
    let mut registered: libc::c_int = 0;
    CONS_Printf(
        b"Registering this server on the master server...\n\0" as *const u8
            as *const libc::c_char,
    );
    registered = HMS_register();
    MSRegistered = registered;
    MSRegisteredId = MSId;
    time(&mut MSLastPing);
    if registered != 0 {
        CONS_Printf(
            b"Master server registration successful.\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn Finish_update() {
    let mut registered: libc::c_int = 0;
    let mut done: libc::c_int = 0;
    registered = MSRegistered;
    MSUpdateAgain = false_0 as libc::c_int;
    if registered != 0 {
        if HMS_update() != 0 {
            time(&mut MSLastPing);
            MSRegistered = true_0 as libc::c_int;
            CONS_Printf(
                b"Updated master server listing.\n\0" as *const u8 as *const libc::c_char,
            );
        } else {
            Finish_registration();
        }
    } else {
        Finish_registration();
    }
    done = (MSUpdateAgain == 0) as libc::c_int;
    if done != 0 {
        MSInProgress = false_0 as libc::c_int;
    }
    if done == 0 {
        Finish_update();
    }
}
unsafe extern "C" fn Finish_unlist() {
    let mut registered: libc::c_int = 0;
    registered = MSRegistered;
    if registered != 0 {
        CONS_Printf(
            b"Removing this server from the master server...\n\0" as *const u8
                as *const libc::c_char,
        );
        if HMS_unlist() != 0 {
            CONS_Printf(
                b"Server deregistration request successfully sent.\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        MSRegistered = false_0 as libc::c_int;
    }
    if MSId == MSRegisteredId {
        MSId += 1;
        MSId;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RegisterServer() {
    Finish_registration();
}
unsafe extern "C" fn UpdateServer() {
    Finish_update();
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterServer() {
    Finish_unlist();
}
unsafe extern "C" fn Online() -> boolean {
    return (serverrunning != 0 && ms_RoomId as libc::c_int > 0 as libc::c_int)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn SendPingToMasterServer() {
    let mut ready: libc::c_int = 0;
    let mut now: time_t = 0;
    if Online() != 0 {
        time(&mut now);
        ready = (MSRegisteredId == MSId && MSInProgress == 0
            && now
                >= MSLastPing
                    + (60 as libc::c_int * cv_masterserver_update_rate.value) as time_t)
            as libc::c_int;
        if ready != 0 {
            MSInProgress = true_0 as libc::c_int;
        }
        if ready != 0 {
            UpdateServer();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn MasterClient_Ticker() {
    SendPingToMasterServer();
}
unsafe extern "C" fn Set_api(mut api: *const libc::c_char) {
    HMS_set_api(strdup(api));
}
unsafe extern "C" fn Update_parameters() {
    let mut registered: libc::c_int = 0 as libc::c_int;
    let mut delayed: libc::c_int = 0;
    if Online() != 0 {
        delayed = MSInProgress;
        if delayed != 0 {
            MSUpdateAgain = true_0 as libc::c_int;
        } else {
            registered = MSRegistered;
        }
        if delayed == 0 && registered != 0 {
            UpdateServer();
        }
    }
}
unsafe extern "C" fn MasterServer_OnChange() {
    UnregisterServer();
    if cv_masterserver.changed == 0
        && strcmp(
            cv_masterserver.string,
            b"ms.srb2.org:28900\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        CV_StealthSet(&mut cv_masterserver, cv_masterserver.defaultvalue);
    }
    if cv_masterserver.changed == 0
        && strcmp(
            cv_masterserver.string,
            b"https://mb.srb2.org/MS/0\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        CV_StealthSet(&mut cv_masterserver, cv_masterserver.defaultvalue);
    }
    Set_api(cv_masterserver.string);
    if Online() != 0 {
        RegisterServer();
    }
}
