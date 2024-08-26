use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    static mut menuactive: boolean;
    fn COM_Argc() -> size_t;
    fn COM_Argv(arg: size_t) -> *const libc::c_char;
    static mut cv_execversion: consvar_t;
    static mut chat_on: boolean;
    fn CON_Ready() -> boolean;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const NUMINPUTS: C2RustUnnamed_3 = 484;
pub const KEY_2MOUSEWHEELDOWN: C2RustUnnamed_3 = 483;
pub const KEY_2MOUSEWHEELUP: C2RustUnnamed_3 = 482;
pub const KEY_MOUSEWHEELDOWN: C2RustUnnamed_3 = 481;
pub const KEY_MOUSEWHEELUP: C2RustUnnamed_3 = 480;
pub const KEY_DBL2HAT1: C2RustUnnamed_3 = 464;
pub const KEY_DBL2JOY1: C2RustUnnamed_3 = 432;
pub const KEY_DBL2MOUSE1: C2RustUnnamed_3 = 424;
pub const KEY_2HAT1: C2RustUnnamed_3 = 408;
pub const KEY_2JOY1: C2RustUnnamed_3 = 376;
pub const KEY_2MOUSE1: C2RustUnnamed_3 = 368;
pub const KEY_DBLHAT1: C2RustUnnamed_3 = 352;
pub const KEY_DBLJOY1: C2RustUnnamed_3 = 320;
pub const KEY_DBLMOUSE1: C2RustUnnamed_3 = 312;
pub const KEY_HAT1: C2RustUnnamed_3 = 296;
pub const KEY_JOY1: C2RustUnnamed_3 = 264;
pub const KEY_MOUSE1: C2RustUnnamed_3 = 256;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const NUM_GAMECONTROLS: C2RustUnnamed_4 = 43;
pub const GC_CUSTOM3: C2RustUnnamed_4 = 42;
pub const GC_CUSTOM2: C2RustUnnamed_4 = 41;
pub const GC_CUSTOM1: C2RustUnnamed_4 = 40;
pub const GC_VIEWPOINTPREV: C2RustUnnamed_4 = 39;
pub const GC_VIEWPOINTNEXT: C2RustUnnamed_4 = 38;
pub const GC_RECORDGIF: C2RustUnnamed_4 = 37;
pub const GC_SCREENSHOT: C2RustUnnamed_4 = 36;
pub const GC_SYSTEMMENU: C2RustUnnamed_4 = 35;
pub const GC_PAUSE: C2RustUnnamed_4 = 34;
pub const GC_CONSOLE: C2RustUnnamed_4 = 33;
pub const GC_JUMP: C2RustUnnamed_4 = 32;
pub const GC_SCORES: C2RustUnnamed_4 = 31;
pub const GC_TEAMKEY: C2RustUnnamed_4 = 30;
pub const GC_TALKKEY: C2RustUnnamed_4 = 29;
pub const GC_MOUSEAIMING: C2RustUnnamed_4 = 28;
pub const GC_CENTERVIEW: C2RustUnnamed_4 = 27;
pub const GC_LOOKDOWN: C2RustUnnamed_4 = 26;
pub const GC_LOOKUP: C2RustUnnamed_4 = 25;
pub const GC_CAMRESET: C2RustUnnamed_4 = 24;
pub const GC_CAMTOGGLE: C2RustUnnamed_4 = 23;
pub const GC_SPIN: C2RustUnnamed_4 = 22;
pub const GC_TOSSFLAG: C2RustUnnamed_4 = 21;
pub const GC_FIRENORMAL: C2RustUnnamed_4 = 20;
pub const GC_FIRE: C2RustUnnamed_4 = 19;
pub const GC_WEPSLOT10: C2RustUnnamed_4 = 18;
pub const GC_WEPSLOT9: C2RustUnnamed_4 = 17;
pub const GC_WEPSLOT8: C2RustUnnamed_4 = 16;
pub const GC_WEPSLOT7: C2RustUnnamed_4 = 15;
pub const GC_WEPSLOT6: C2RustUnnamed_4 = 14;
pub const GC_WEPSLOT5: C2RustUnnamed_4 = 13;
pub const GC_WEPSLOT4: C2RustUnnamed_4 = 12;
pub const GC_WEPSLOT3: C2RustUnnamed_4 = 11;
pub const GC_WEPSLOT2: C2RustUnnamed_4 = 10;
pub const GC_WEPSLOT1: C2RustUnnamed_4 = 9;
pub const GC_WEAPONPREV: C2RustUnnamed_4 = 8;
pub const GC_WEAPONNEXT: C2RustUnnamed_4 = 7;
pub const GC_TURNRIGHT: C2RustUnnamed_4 = 6;
pub const GC_TURNLEFT: C2RustUnnamed_4 = 5;
pub const GC_STRAFERIGHT: C2RustUnnamed_4 = 4;
pub const GC_STRAFELEFT: C2RustUnnamed_4 = 3;
pub const GC_BACKWARD: C2RustUnnamed_4 = 2;
pub const GC_FORWARD: C2RustUnnamed_4 = 1;
pub const GC_NULL: C2RustUnnamed_4 = 0;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const num_gamecontrolschemes: C2RustUnnamed_5 = 3;
pub const gcs_platform: C2RustUnnamed_5 = 2;
pub const gcs_fps: C2RustUnnamed_5 = 1;
pub const gcs_custom: C2RustUnnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_t {
    pub dx: int32_t,
    pub dy: int32_t,
    pub mlookdy: int32_t,
    pub rdx: int32_t,
    pub rdy: int32_t,
    pub buttons: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dclick_t {
    pub time: uint8_t,
    pub state: uint8_t,
    pub clicks: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyname_t {
    pub keynum: int32_t,
    pub name: *const libc::c_char,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut mousesens_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 100 as libc::c_int,
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
static mut onecontrolperkey_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: 1 as libc::c_int,
            strvalue: b"One\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 2 as libc::c_int,
            strvalue: b"Several\0" as *const u8 as *const libc::c_char,
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
pub static mut cv_mousesens: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"mousesens\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"20\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: mousesens_cons_t.as_ptr() as *mut _,
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
pub static mut cv_mousesens2: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"mousesens2\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"20\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: mousesens_cons_t.as_ptr() as *mut _,
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
pub static mut cv_mouseysens: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"mouseysens\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"20\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: mousesens_cons_t.as_ptr() as *mut _,
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
pub static mut cv_mouseysens2: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"mouseysens2\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"20\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: mousesens_cons_t.as_ptr() as *mut _,
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
pub static mut cv_controlperkey: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"controlperkey\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"One\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: onecontrolperkey_cons_t.as_ptr() as *mut _,
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
pub static mut mouse: mouse_t = mouse_t {
    dx: 0,
    dy: 0,
    mlookdy: 0,
    rdx: 0,
    rdy: 0,
    buttons: 0,
};
#[no_mangle]
pub static mut mouse2: mouse_t = mouse_t {
    dx: 0,
    dy: 0,
    mlookdy: 0,
    rdx: 0,
    rdy: 0,
    buttons: 0,
};
#[no_mangle]
pub static mut joyxmove: [int32_t; 4] = [0; 4];
#[no_mangle]
pub static mut joyymove: [int32_t; 4] = [0; 4];
#[no_mangle]
pub static mut joy2xmove: [int32_t; 4] = [0; 4];
#[no_mangle]
pub static mut joy2ymove: [int32_t; 4] = [0; 4];
#[no_mangle]
pub static mut gamekeydown: [uint8_t; 484] = [0; 484];
#[no_mangle]
pub static mut gamecontrol: [[int32_t; 2]; 43] = [[0; 2]; 43];
#[no_mangle]
pub static mut gamecontrolbis: [[int32_t; 2]; 43] = [[0; 2]; 43];
#[no_mangle]
pub static mut gamecontroldefault: [[[int32_t; 2]; 43]; 3] = [[[0; 2]; 43]; 3];
#[no_mangle]
pub static mut gamecontrolbisdefault: [[[int32_t; 2]; 43]; 3] = [[[0; 2]; 43]; 3];
#[no_mangle]
pub static mut gcl_tutorial_check: [int32_t; 6] = [
    GC_FORWARD as libc::c_int,
    GC_BACKWARD as libc::c_int,
    GC_STRAFELEFT as libc::c_int,
    GC_STRAFERIGHT as libc::c_int,
    GC_TURNLEFT as libc::c_int,
    GC_TURNRIGHT as libc::c_int,
];
#[no_mangle]
pub static mut gcl_tutorial_used: [int32_t; 8] = [
    GC_FORWARD as libc::c_int,
    GC_BACKWARD as libc::c_int,
    GC_STRAFELEFT as libc::c_int,
    GC_STRAFERIGHT as libc::c_int,
    GC_TURNLEFT as libc::c_int,
    GC_TURNRIGHT as libc::c_int,
    GC_JUMP as libc::c_int,
    GC_SPIN as libc::c_int,
];
#[no_mangle]
pub static mut gcl_tutorial_full: [int32_t; 13] = [
    GC_FORWARD as libc::c_int,
    GC_BACKWARD as libc::c_int,
    GC_STRAFELEFT as libc::c_int,
    GC_STRAFERIGHT as libc::c_int,
    GC_LOOKUP as libc::c_int,
    GC_LOOKDOWN as libc::c_int,
    GC_TURNLEFT as libc::c_int,
    GC_TURNRIGHT as libc::c_int,
    GC_CENTERVIEW as libc::c_int,
    GC_JUMP as libc::c_int,
    GC_SPIN as libc::c_int,
    GC_FIRE as libc::c_int,
    GC_FIRENORMAL as libc::c_int,
];
#[no_mangle]
pub static mut gcl_movement: [int32_t; 4] = [
    GC_FORWARD as libc::c_int,
    GC_BACKWARD as libc::c_int,
    GC_STRAFELEFT as libc::c_int,
    GC_STRAFERIGHT as libc::c_int,
];
#[no_mangle]
pub static mut gcl_camera: [int32_t; 2] = [
    GC_TURNLEFT as libc::c_int,
    GC_TURNRIGHT as libc::c_int,
];
#[no_mangle]
pub static mut gcl_movement_camera: [int32_t; 6] = [
    GC_FORWARD as libc::c_int,
    GC_BACKWARD as libc::c_int,
    GC_STRAFELEFT as libc::c_int,
    GC_STRAFERIGHT as libc::c_int,
    GC_TURNLEFT as libc::c_int,
    GC_TURNRIGHT as libc::c_int,
];
#[no_mangle]
pub static mut gcl_jump: [int32_t; 1] = [GC_JUMP as libc::c_int];
#[no_mangle]
pub static mut gcl_spin: [int32_t; 1] = [GC_SPIN as libc::c_int];
#[no_mangle]
pub static mut gcl_jump_spin: [int32_t; 2] = [
    GC_JUMP as libc::c_int,
    GC_SPIN as libc::c_int,
];
static mut mousedclicks: [dclick_t; 8] = [dclick_t {
    time: 0,
    state: 0,
    clicks: 0,
}; 8];
static mut joydclicks: [dclick_t; 48] = [dclick_t {
    time: 0,
    state: 0,
    clicks: 0,
}; 48];
static mut mouse2dclicks: [dclick_t; 8] = [dclick_t {
    time: 0,
    state: 0,
    clicks: 0,
}; 8];
static mut joy2dclicks: [dclick_t; 48] = [dclick_t {
    time: 0,
    state: 0,
    clicks: 0,
}; 48];
#[no_mangle]
pub unsafe extern "C" fn G_MapEventsToControls(mut ev: *mut event_t) {
    let mut i: int32_t = 0;
    let mut flag: uint8_t = 0;
    match (*ev).type_0 as libc::c_uint {
        0 => {
            if (*ev).key < NUMINPUTS as libc::c_int {
                gamekeydown[(*ev).key as usize] = 1 as libc::c_int as uint8_t;
            }
        }
        1 => {
            if (*ev).key < NUMINPUTS as libc::c_int {
                gamekeydown[(*ev).key as usize] = 0 as libc::c_int as uint8_t;
            }
        }
        3 => {
            mouse.rdx = (*ev).x;
            mouse.rdy = (*ev).y;
        }
        4 => {
            i = (*ev).key;
            if !(i >= 4 as libc::c_int || menuactive != 0 || CON_Ready() != 0
                || chat_on != 0)
            {
                if (*ev).x != 2147483647 as libc::c_int {
                    joyxmove[i as usize] = (*ev).x;
                }
                if (*ev).y != 2147483647 as libc::c_int {
                    joyymove[i as usize] = (*ev).y;
                }
            }
        }
        6 => {
            i = (*ev).key;
            if !(i >= 4 as libc::c_int || menuactive != 0 || CON_Ready() != 0
                || chat_on != 0)
            {
                if (*ev).x != 2147483647 as libc::c_int {
                    joy2xmove[i as usize] = (*ev).x;
                }
                if (*ev).y != 2147483647 as libc::c_int {
                    joy2ymove[i as usize] = (*ev).y;
                }
            }
        }
        5 => {
            if !(menuactive != 0 || CON_Ready() != 0 || chat_on != 0) {
                mouse2.rdx = (*ev).x;
                mouse2.rdy = (*ev).y;
            }
        }
        _ => {}
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        flag = G_CheckDoubleClick(
            gamekeydown[(KEY_MOUSE1 as libc::c_int + i) as usize],
            &mut *mousedclicks.as_mut_ptr().offset(i as isize),
        );
        gamekeydown[(KEY_DBLMOUSE1 as libc::c_int + i) as usize] = flag;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int + 4 as libc::c_int * 4 as libc::c_int {
        flag = G_CheckDoubleClick(
            gamekeydown[(KEY_JOY1 as libc::c_int + i) as usize],
            &mut *joydclicks.as_mut_ptr().offset(i as isize),
        );
        gamekeydown[(KEY_DBLJOY1 as libc::c_int + i) as usize] = flag;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        flag = G_CheckDoubleClick(
            gamekeydown[(KEY_2MOUSE1 as libc::c_int + i) as usize],
            &mut *mouse2dclicks.as_mut_ptr().offset(i as isize),
        );
        gamekeydown[(KEY_DBL2MOUSE1 as libc::c_int + i) as usize] = flag;
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int + 4 as libc::c_int * 4 as libc::c_int {
        flag = G_CheckDoubleClick(
            gamekeydown[(KEY_2JOY1 as libc::c_int + i) as usize],
            &mut *joy2dclicks.as_mut_ptr().offset(i as isize),
        );
        gamekeydown[(KEY_DBL2JOY1 as libc::c_int + i) as usize] = flag;
        i += 1;
        i;
    }
}
unsafe extern "C" fn G_CheckDoubleClick(
    mut state: uint8_t,
    mut dt: *mut dclick_t,
) -> uint8_t {
    if state as libc::c_int != (*dt).state as libc::c_int
        && (*dt).time as libc::c_int > 1 as libc::c_int
    {
        (*dt).state = state;
        if state != 0 {
            (*dt).clicks = ((*dt).clicks).wrapping_add(1);
            (*dt).clicks;
        }
        if (*dt).clicks as libc::c_int == 2 as libc::c_int {
            (*dt).clicks = 0 as libc::c_int as uint8_t;
            return true_0 as libc::c_int as uint8_t;
        } else {
            (*dt).time = 0 as libc::c_int as uint8_t;
        }
    } else {
        (*dt).time = ((*dt).time).wrapping_add(1);
        (*dt).time;
        if (*dt).time as libc::c_int > 20 as libc::c_int {
            (*dt).clicks = 0 as libc::c_int as uint8_t;
            (*dt).state = 0 as libc::c_int as uint8_t;
        }
    }
    return false_0 as libc::c_int as uint8_t;
}
static mut keynames: [keyname_t; 286] = [
    {
        let mut init = keyname_t {
            keynum: 32 as libc::c_int,
            name: b"space\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 58 as libc::c_int,
            name: b"caps lock\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 13 as libc::c_int,
            name: b"enter\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 9 as libc::c_int,
            name: b"tab\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 27 as libc::c_int,
            name: b"escape\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 8 as libc::c_int,
            name: b"backspace\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 69 as libc::c_int,
            name: b"numlock\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 70 as libc::c_int,
            name: b"scrolllock\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 91 as libc::c_int,
            name: b"leftwin\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 92 as libc::c_int,
            name: b"rightwin\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 93 as libc::c_int,
            name: b"menu\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 54 as libc::c_int,
            name: b"lshift\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 55 as libc::c_int,
            name: b"rshift\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 54 as libc::c_int,
            name: b"shift\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 29 as libc::c_int,
            name: b"lctrl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 30 as libc::c_int,
            name: b"rctrl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 29 as libc::c_int,
            name: b"ctrl\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 56 as libc::c_int,
            name: b"lalt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 57 as libc::c_int,
            name: b"ralt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 56 as libc::c_int,
            name: b"alt\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 100 as libc::c_int,
            name: b"keypad /\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 71 as libc::c_int,
            name: b"keypad 7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 72 as libc::c_int,
            name: b"keypad 8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 73 as libc::c_int,
            name: b"keypad 9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 74 as libc::c_int,
            name: b"keypad -\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 75 as libc::c_int,
            name: b"keypad 4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 76 as libc::c_int,
            name: b"keypad 5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 77 as libc::c_int,
            name: b"keypad 6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 78 as libc::c_int,
            name: b"keypad +\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 79 as libc::c_int,
            name: b"keypad 1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 80 as libc::c_int,
            name: b"keypad 2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 81 as libc::c_int,
            name: b"keypad 3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 82 as libc::c_int,
            name: b"keypad 0\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 83 as libc::c_int,
            name: b"keypad .\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 101 as libc::c_int,
            name: b"home\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 102 as libc::c_int,
            name: b"up arrow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 103 as libc::c_int,
            name: b"pgup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 105 as libc::c_int,
            name: b"left arrow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 107 as libc::c_int,
            name: b"right arrow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 109 as libc::c_int,
            name: b"end\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 110 as libc::c_int,
            name: b"down arrow\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 111 as libc::c_int,
            name: b"pgdn\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 112 as libc::c_int,
            name: b"ins\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 113 as libc::c_int,
            name: b"del\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x3b as libc::c_int,
            name: b"f1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x3c as libc::c_int,
            name: b"f2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x3d as libc::c_int,
            name: b"f3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x3e as libc::c_int,
            name: b"f4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x3f as libc::c_int,
            name: b"f5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x40 as libc::c_int,
            name: b"f6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x41 as libc::c_int,
            name: b"f7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x42 as libc::c_int,
            name: b"f8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x43 as libc::c_int,
            name: b"f9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x44 as libc::c_int,
            name: b"f10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x57 as libc::c_int,
            name: b"f11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 0x80 as libc::c_int + 0x58 as libc::c_int,
            name: b"f12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: '`' as i32,
            name: b"TILDE\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: 255 as libc::c_int,
            name: b"pause/break\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 0 as libc::c_int,
            name: b"mouse1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 1 as libc::c_int,
            name: b"mouse2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 2 as libc::c_int,
            name: b"mouse3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 3 as libc::c_int,
            name: b"mouse4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 4 as libc::c_int,
            name: b"mouse5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 5 as libc::c_int,
            name: b"mouse6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 6 as libc::c_int,
            name: b"mouse7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSE1 as libc::c_int + 7 as libc::c_int,
            name: b"mouse8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 0 as libc::c_int,
            name: b"sec_mouse2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 1 as libc::c_int,
            name: b"sec_mouse1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 2 as libc::c_int,
            name: b"sec_mouse3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 3 as libc::c_int,
            name: b"sec_mouse4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 4 as libc::c_int,
            name: b"sec_mouse5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 5 as libc::c_int,
            name: b"sec_mouse6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 6 as libc::c_int,
            name: b"sec_mouse7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSE1 as libc::c_int + 7 as libc::c_int,
            name: b"sec_mouse8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSEWHEELUP as libc::c_int,
            name: b"wheel 1 up\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_MOUSEWHEELDOWN as libc::c_int,
            name: b"wheel 1 down\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSEWHEELUP as libc::c_int,
            name: b"wheel 2 up\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2MOUSEWHEELDOWN as libc::c_int,
            name: b"wheel 2 down\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 0 as libc::c_int,
            name: b"joy1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 1 as libc::c_int,
            name: b"joy2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 2 as libc::c_int,
            name: b"joy3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 3 as libc::c_int,
            name: b"joy4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 4 as libc::c_int,
            name: b"joy5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 5 as libc::c_int,
            name: b"joy6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 6 as libc::c_int,
            name: b"joy7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 7 as libc::c_int,
            name: b"joy8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 8 as libc::c_int,
            name: b"joy9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 9 as libc::c_int,
            name: b"joy10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 10 as libc::c_int,
            name: b"joy11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 11 as libc::c_int,
            name: b"joy12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 12 as libc::c_int,
            name: b"joy13\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 13 as libc::c_int,
            name: b"joy14\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 14 as libc::c_int,
            name: b"joy15\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 15 as libc::c_int,
            name: b"joy16\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 16 as libc::c_int,
            name: b"joy17\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 17 as libc::c_int,
            name: b"joy18\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 18 as libc::c_int,
            name: b"joy19\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 19 as libc::c_int,
            name: b"joy20\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 20 as libc::c_int,
            name: b"joy21\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 21 as libc::c_int,
            name: b"joy22\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 22 as libc::c_int,
            name: b"joy23\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 23 as libc::c_int,
            name: b"joy24\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 24 as libc::c_int,
            name: b"joy25\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 25 as libc::c_int,
            name: b"joy26\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 26 as libc::c_int,
            name: b"joy27\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 27 as libc::c_int,
            name: b"joy28\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 28 as libc::c_int,
            name: b"joy29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 29 as libc::c_int,
            name: b"joy30\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 30 as libc::c_int,
            name: b"joy31\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_JOY1 as libc::c_int + 31 as libc::c_int,
            name: b"joy32\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 0 as libc::c_int,
            name: b"hatup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 1 as libc::c_int,
            name: b"hatdown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 2 as libc::c_int,
            name: b"hatleft\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 3 as libc::c_int,
            name: b"hatright\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 4 as libc::c_int,
            name: b"hatup2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 5 as libc::c_int,
            name: b"hatdown2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 6 as libc::c_int,
            name: b"hatleft2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 7 as libc::c_int,
            name: b"hatright2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 8 as libc::c_int,
            name: b"hatup3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 9 as libc::c_int,
            name: b"hatdown3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 10 as libc::c_int,
            name: b"hatleft3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 11 as libc::c_int,
            name: b"hatright3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 12 as libc::c_int,
            name: b"hatup4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 13 as libc::c_int,
            name: b"hatdown4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 14 as libc::c_int,
            name: b"hatleft4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_HAT1 as libc::c_int + 15 as libc::c_int,
            name: b"hatright4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 0 as libc::c_int,
            name: b"dblmouse1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 1 as libc::c_int,
            name: b"dblmouse2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 2 as libc::c_int,
            name: b"dblmouse3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 3 as libc::c_int,
            name: b"dblmouse4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 4 as libc::c_int,
            name: b"dblmouse5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 5 as libc::c_int,
            name: b"dblmouse6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 6 as libc::c_int,
            name: b"dblmouse7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLMOUSE1 as libc::c_int + 7 as libc::c_int,
            name: b"dblmouse8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 0 as libc::c_int,
            name: b"dblsec_mouse2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 1 as libc::c_int,
            name: b"dblsec_mouse1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 2 as libc::c_int,
            name: b"dblsec_mouse3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 3 as libc::c_int,
            name: b"dblsec_mouse4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 4 as libc::c_int,
            name: b"dblsec_mouse5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 5 as libc::c_int,
            name: b"dblsec_mouse6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 6 as libc::c_int,
            name: b"dblsec_mouse7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2MOUSE1 as libc::c_int + 7 as libc::c_int,
            name: b"dblsec_mouse8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 0 as libc::c_int,
            name: b"dbljoy1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 1 as libc::c_int,
            name: b"dbljoy2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 2 as libc::c_int,
            name: b"dbljoy3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 3 as libc::c_int,
            name: b"dbljoy4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 4 as libc::c_int,
            name: b"dbljoy5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 5 as libc::c_int,
            name: b"dbljoy6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 6 as libc::c_int,
            name: b"dbljoy7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 7 as libc::c_int,
            name: b"dbljoy8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 8 as libc::c_int,
            name: b"dbljoy9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 9 as libc::c_int,
            name: b"dbljoy10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 10 as libc::c_int,
            name: b"dbljoy11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 11 as libc::c_int,
            name: b"dbljoy12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 12 as libc::c_int,
            name: b"dbljoy13\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 13 as libc::c_int,
            name: b"dbljoy14\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 14 as libc::c_int,
            name: b"dbljoy15\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 15 as libc::c_int,
            name: b"dbljoy16\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 16 as libc::c_int,
            name: b"dbljoy17\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 17 as libc::c_int,
            name: b"dbljoy18\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 18 as libc::c_int,
            name: b"dbljoy19\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 19 as libc::c_int,
            name: b"dbljoy20\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 20 as libc::c_int,
            name: b"dbljoy21\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 21 as libc::c_int,
            name: b"dbljoy22\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 22 as libc::c_int,
            name: b"dbljoy23\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 23 as libc::c_int,
            name: b"dbljoy24\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 24 as libc::c_int,
            name: b"dbljoy25\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 25 as libc::c_int,
            name: b"dbljoy26\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 26 as libc::c_int,
            name: b"dbljoy27\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 27 as libc::c_int,
            name: b"dbljoy28\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 28 as libc::c_int,
            name: b"dbljoy29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 29 as libc::c_int,
            name: b"dbljoy30\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 30 as libc::c_int,
            name: b"dbljoy31\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLJOY1 as libc::c_int + 31 as libc::c_int,
            name: b"dbljoy32\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 0 as libc::c_int,
            name: b"dblhatup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 1 as libc::c_int,
            name: b"dblhatdown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 2 as libc::c_int,
            name: b"dblhatleft\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 3 as libc::c_int,
            name: b"dblhatright\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 4 as libc::c_int,
            name: b"dblhatup2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 5 as libc::c_int,
            name: b"dblhatdown2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 6 as libc::c_int,
            name: b"dblhatleft2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 7 as libc::c_int,
            name: b"dblhatright2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 8 as libc::c_int,
            name: b"dblhatup3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 9 as libc::c_int,
            name: b"dblhatdown3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 10 as libc::c_int,
            name: b"dblhatleft3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 11 as libc::c_int,
            name: b"dblhatright3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 12 as libc::c_int,
            name: b"dblhatup4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 13 as libc::c_int,
            name: b"dblhatdown4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 14 as libc::c_int,
            name: b"dblhatleft4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBLHAT1 as libc::c_int + 15 as libc::c_int,
            name: b"dblhatright4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 0 as libc::c_int,
            name: b"sec_joy1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 1 as libc::c_int,
            name: b"sec_joy2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 2 as libc::c_int,
            name: b"sec_joy3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 3 as libc::c_int,
            name: b"sec_joy4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 4 as libc::c_int,
            name: b"sec_joy5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 5 as libc::c_int,
            name: b"sec_joy6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 6 as libc::c_int,
            name: b"sec_joy7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 7 as libc::c_int,
            name: b"sec_joy8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 8 as libc::c_int,
            name: b"sec_joy9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 9 as libc::c_int,
            name: b"sec_joy10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 10 as libc::c_int,
            name: b"sec_joy11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 11 as libc::c_int,
            name: b"sec_joy12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 12 as libc::c_int,
            name: b"sec_joy13\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 13 as libc::c_int,
            name: b"sec_joy14\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 14 as libc::c_int,
            name: b"sec_joy15\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 15 as libc::c_int,
            name: b"sec_joy16\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 16 as libc::c_int,
            name: b"sec_joy17\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 17 as libc::c_int,
            name: b"sec_joy18\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 18 as libc::c_int,
            name: b"sec_joy19\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 19 as libc::c_int,
            name: b"sec_joy20\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 20 as libc::c_int,
            name: b"sec_joy21\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 21 as libc::c_int,
            name: b"sec_joy22\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 22 as libc::c_int,
            name: b"sec_joy23\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 23 as libc::c_int,
            name: b"sec_joy24\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 24 as libc::c_int,
            name: b"sec_joy25\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 25 as libc::c_int,
            name: b"sec_joy26\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 26 as libc::c_int,
            name: b"sec_joy27\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 27 as libc::c_int,
            name: b"sec_joy28\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 28 as libc::c_int,
            name: b"sec_joy29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 29 as libc::c_int,
            name: b"sec_joy30\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 30 as libc::c_int,
            name: b"sec_joy31\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2JOY1 as libc::c_int + 31 as libc::c_int,
            name: b"sec_joy32\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 0 as libc::c_int,
            name: b"sec_hatup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 1 as libc::c_int,
            name: b"sec_hatdown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 2 as libc::c_int,
            name: b"sec_hatleft\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 3 as libc::c_int,
            name: b"sec_hatright\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 4 as libc::c_int,
            name: b"sec_hatup2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 5 as libc::c_int,
            name: b"sec_hatdown2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 6 as libc::c_int,
            name: b"sec_hatleft2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 7 as libc::c_int,
            name: b"sec_hatright2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 8 as libc::c_int,
            name: b"sec_hatup3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 9 as libc::c_int,
            name: b"sec_hatdown3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 10 as libc::c_int,
            name: b"sec_hatleft3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 11 as libc::c_int,
            name: b"sec_hatright3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 12 as libc::c_int,
            name: b"sec_hatup4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 13 as libc::c_int,
            name: b"sec_hatdown4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 14 as libc::c_int,
            name: b"sec_hatleft4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_2HAT1 as libc::c_int + 15 as libc::c_int,
            name: b"sec_hatright4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 0 as libc::c_int,
            name: b"dblsec_joy1\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 1 as libc::c_int,
            name: b"dblsec_joy2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 2 as libc::c_int,
            name: b"dblsec_joy3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 3 as libc::c_int,
            name: b"dblsec_joy4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 4 as libc::c_int,
            name: b"dblsec_joy5\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 5 as libc::c_int,
            name: b"dblsec_joy6\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 6 as libc::c_int,
            name: b"dblsec_joy7\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 7 as libc::c_int,
            name: b"dblsec_joy8\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 8 as libc::c_int,
            name: b"dblsec_joy9\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 9 as libc::c_int,
            name: b"dblsec_joy10\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 10 as libc::c_int,
            name: b"dblsec_joy11\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 11 as libc::c_int,
            name: b"dblsec_joy12\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 12 as libc::c_int,
            name: b"dblsec_joy13\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 13 as libc::c_int,
            name: b"dblsec_joy14\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 14 as libc::c_int,
            name: b"dblsec_joy15\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 15 as libc::c_int,
            name: b"dblsec_joy16\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 16 as libc::c_int,
            name: b"dblsec_joy17\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 17 as libc::c_int,
            name: b"dblsec_joy18\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 18 as libc::c_int,
            name: b"dblsec_joy19\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 19 as libc::c_int,
            name: b"dblsec_joy20\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 20 as libc::c_int,
            name: b"dblsec_joy21\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 21 as libc::c_int,
            name: b"dblsec_joy22\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 22 as libc::c_int,
            name: b"dblsec_joy23\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 23 as libc::c_int,
            name: b"dblsec_joy24\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 24 as libc::c_int,
            name: b"dblsec_joy25\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 25 as libc::c_int,
            name: b"dblsec_joy26\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 26 as libc::c_int,
            name: b"dblsec_joy27\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 27 as libc::c_int,
            name: b"dblsec_joy28\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 28 as libc::c_int,
            name: b"dblsec_joy29\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 29 as libc::c_int,
            name: b"dblsec_joy30\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 30 as libc::c_int,
            name: b"dblsec_joy31\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2JOY1 as libc::c_int + 31 as libc::c_int,
            name: b"dblsec_joy32\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 0 as libc::c_int,
            name: b"dblsec_hatup\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 1 as libc::c_int,
            name: b"dblsec_hatdown\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 2 as libc::c_int,
            name: b"dblsec_hatleft\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 3 as libc::c_int,
            name: b"dblsec_hatright\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 4 as libc::c_int,
            name: b"dblsec_hatup2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 5 as libc::c_int,
            name: b"dblsec_hatdown2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 6 as libc::c_int,
            name: b"dblsec_hatleft2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 7 as libc::c_int,
            name: b"dblsec_hatright2\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 8 as libc::c_int,
            name: b"dblsec_hatup3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 9 as libc::c_int,
            name: b"dblsec_hatdown3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 10 as libc::c_int,
            name: b"dblsec_hatleft3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 11 as libc::c_int,
            name: b"dblsec_hatright3\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 12 as libc::c_int,
            name: b"dblsec_hatup4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 13 as libc::c_int,
            name: b"dblsec_hatdown4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 14 as libc::c_int,
            name: b"dblsec_hatleft4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = keyname_t {
            keynum: KEY_DBL2HAT1 as libc::c_int + 15 as libc::c_int,
            name: b"dblsec_hatright4\0" as *const u8 as *const libc::c_char,
        };
        init
    },
];
static mut gamecontrolname: [*const libc::c_char; 43] = [
    b"nothing\0" as *const u8 as *const libc::c_char,
    b"forward\0" as *const u8 as *const libc::c_char,
    b"backward\0" as *const u8 as *const libc::c_char,
    b"strafeleft\0" as *const u8 as *const libc::c_char,
    b"straferight\0" as *const u8 as *const libc::c_char,
    b"turnleft\0" as *const u8 as *const libc::c_char,
    b"turnright\0" as *const u8 as *const libc::c_char,
    b"weaponnext\0" as *const u8 as *const libc::c_char,
    b"weaponprev\0" as *const u8 as *const libc::c_char,
    b"weapon1\0" as *const u8 as *const libc::c_char,
    b"weapon2\0" as *const u8 as *const libc::c_char,
    b"weapon3\0" as *const u8 as *const libc::c_char,
    b"weapon4\0" as *const u8 as *const libc::c_char,
    b"weapon5\0" as *const u8 as *const libc::c_char,
    b"weapon6\0" as *const u8 as *const libc::c_char,
    b"weapon7\0" as *const u8 as *const libc::c_char,
    b"weapon8\0" as *const u8 as *const libc::c_char,
    b"weapon9\0" as *const u8 as *const libc::c_char,
    b"weapon10\0" as *const u8 as *const libc::c_char,
    b"fire\0" as *const u8 as *const libc::c_char,
    b"firenormal\0" as *const u8 as *const libc::c_char,
    b"tossflag\0" as *const u8 as *const libc::c_char,
    b"spin\0" as *const u8 as *const libc::c_char,
    b"camtoggle\0" as *const u8 as *const libc::c_char,
    b"camreset\0" as *const u8 as *const libc::c_char,
    b"lookup\0" as *const u8 as *const libc::c_char,
    b"lookdown\0" as *const u8 as *const libc::c_char,
    b"centerview\0" as *const u8 as *const libc::c_char,
    b"mouseaiming\0" as *const u8 as *const libc::c_char,
    b"talkkey\0" as *const u8 as *const libc::c_char,
    b"teamtalkkey\0" as *const u8 as *const libc::c_char,
    b"scores\0" as *const u8 as *const libc::c_char,
    b"jump\0" as *const u8 as *const libc::c_char,
    b"console\0" as *const u8 as *const libc::c_char,
    b"pause\0" as *const u8 as *const libc::c_char,
    b"systemmenu\0" as *const u8 as *const libc::c_char,
    b"screenshot\0" as *const u8 as *const libc::c_char,
    b"recordgif\0" as *const u8 as *const libc::c_char,
    b"viewpoint\0" as *const u8 as *const libc::c_char,
    b"viewpointprev\0" as *const u8 as *const libc::c_char,
    b"custom1\0" as *const u8 as *const libc::c_char,
    b"custom2\0" as *const u8 as *const libc::c_char,
    b"custom3\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn G_ClearControlKeys(
    mut setupcontrols: *mut [int32_t; 2],
    mut control: int32_t,
) {
    (*setupcontrols
        .offset(control as isize))[0 as libc::c_int as usize] = 0 as libc::c_int;
    (*setupcontrols
        .offset(control as isize))[1 as libc::c_int as usize] = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn G_ClearAllControlKeys() {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < NUM_GAMECONTROLS as libc::c_int {
        G_ClearControlKeys(gamecontrol.as_mut_ptr(), i);
        G_ClearControlKeys(gamecontrolbis.as_mut_ptr(), i);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_KeyNumToName(mut keynum: int32_t) -> *const libc::c_char {
    static mut keynamestr: [libc::c_char; 8] = [0; 8];
    let mut j: uint32_t = 0;
    if keynum > ' ' as i32 && keynum <= 'z' as i32 && keynum != '`' as i32 {
        keynamestr[0 as libc::c_int as usize] = keynum as libc::c_char;
        keynamestr[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        return keynamestr.as_mut_ptr();
    }
    j = 0 as libc::c_int as uint32_t;
    while (j as libc::c_ulong)
        < (::core::mem::size_of::<[keyname_t; 286]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<keyname_t>() as libc::c_ulong)
    {
        if keynames[j as usize].keynum == keynum {
            return keynames[j as usize].name;
        }
        j = j.wrapping_add(1);
        j;
    }
    sprintf(
        keynamestr.as_mut_ptr(),
        b"KEY%d\0" as *const u8 as *const libc::c_char,
        keynum,
    );
    return keynamestr.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn G_KeyNameToNum(mut keystr: *const libc::c_char) -> int32_t {
    let mut j: uint32_t = 0;
    if *keystr.offset(1 as libc::c_int as isize) == 0
        && *keystr.offset(0 as libc::c_int as isize) as libc::c_int > ' ' as i32
        && *keystr.offset(0 as libc::c_int as isize) as libc::c_int <= 'z' as i32
    {
        return *keystr.offset(0 as libc::c_int as isize) as int32_t;
    }
    if strncmp(
        keystr,
        b"KEY\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 && *keystr.offset(3 as libc::c_int as isize) as libc::c_int >= '0' as i32
        && *keystr.offset(3 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        j = atoi(&*keystr.offset(3 as libc::c_int as isize)) as uint32_t;
        if j < NUMINPUTS as libc::c_int as uint32_t {
            return j as int32_t;
        }
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int as uint32_t;
    while (j as libc::c_ulong)
        < (::core::mem::size_of::<[keyname_t; 286]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<keyname_t>() as libc::c_ulong)
    {
        if strcasecmp(keynames[j as usize].name, keystr) == 0 {
            return keynames[j as usize].keynum;
        }
        j = j.wrapping_add(1);
        j;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn G_DefineDefaultControls() {
    let mut i: int32_t = 0;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_FORWARD as libc::c_int
        as usize][0 as libc::c_int as usize] = 'w' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_BACKWARD as libc::c_int
        as usize][0 as libc::c_int as usize] = 's' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_STRAFELEFT as libc::c_int
        as usize][0 as libc::c_int as usize] = 'a' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_STRAFERIGHT as libc::c_int
        as usize][0 as libc::c_int as usize] = 'd' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_LOOKUP as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 102 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_LOOKDOWN as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 110 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_TURNLEFT as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 105 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_TURNRIGHT as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 107 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_CENTERVIEW as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 29 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_JUMP as libc::c_int
        as usize][0 as libc::c_int as usize] = 32 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_SPIN as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 54 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_FIRE as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 30 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_FIRE as libc::c_int
        as usize][1 as libc::c_int
        as usize] = KEY_MOUSE1 as libc::c_int + 0 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_FIRENORMAL as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 57 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_FIRENORMAL as libc::c_int
        as usize][1 as libc::c_int
        as usize] = KEY_MOUSE1 as libc::c_int + 1 as libc::c_int;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_CUSTOM1 as libc::c_int
        as usize][0 as libc::c_int as usize] = 'z' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_CUSTOM2 as libc::c_int
        as usize][0 as libc::c_int as usize] = 'x' as i32;
    gamecontroldefault[gcs_fps as libc::c_int
        as usize][GC_CUSTOM3 as libc::c_int
        as usize][0 as libc::c_int as usize] = 'c' as i32;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_FORWARD as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 102 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_BACKWARD as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 110 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_STRAFELEFT as libc::c_int
        as usize][0 as libc::c_int as usize] = 'a' as i32;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_STRAFERIGHT as libc::c_int
        as usize][0 as libc::c_int as usize] = 'd' as i32;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_LOOKUP as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 103 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_LOOKDOWN as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 111 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_TURNLEFT as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 105 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_TURNRIGHT as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 107 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_CENTERVIEW as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 109 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_JUMP as libc::c_int
        as usize][0 as libc::c_int as usize] = 32 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_SPIN as libc::c_int
        as usize][0 as libc::c_int as usize] = 0x80 as libc::c_int + 54 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_FIRE as libc::c_int
        as usize][0 as libc::c_int as usize] = 's' as i32;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_FIRE as libc::c_int
        as usize][1 as libc::c_int
        as usize] = KEY_MOUSE1 as libc::c_int + 0 as libc::c_int;
    gamecontroldefault[gcs_platform as libc::c_int
        as usize][GC_FIRENORMAL as libc::c_int
        as usize][0 as libc::c_int as usize] = 'w' as i32;
    i = 1 as libc::c_int;
    while i < num_gamecontrolschemes as libc::c_int {
        gamecontroldefault[i
            as usize][GC_WEAPONNEXT as libc::c_int
            as usize][0 as libc::c_int
            as usize] = KEY_MOUSEWHEELUP as libc::c_int + 0 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_WEAPONPREV as libc::c_int
            as usize][0 as libc::c_int
            as usize] = KEY_MOUSEWHEELDOWN as libc::c_int + 0 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_WEPSLOT1 as libc::c_int
            as usize][0 as libc::c_int as usize] = '1' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT2 as libc::c_int
            as usize][0 as libc::c_int as usize] = '2' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT3 as libc::c_int
            as usize][0 as libc::c_int as usize] = '3' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT4 as libc::c_int
            as usize][0 as libc::c_int as usize] = '4' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT5 as libc::c_int
            as usize][0 as libc::c_int as usize] = '5' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT6 as libc::c_int
            as usize][0 as libc::c_int as usize] = '6' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT7 as libc::c_int
            as usize][0 as libc::c_int as usize] = '7' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT8 as libc::c_int
            as usize][0 as libc::c_int as usize] = '8' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT9 as libc::c_int
            as usize][0 as libc::c_int as usize] = '9' as i32;
        gamecontroldefault[i
            as usize][GC_WEPSLOT10 as libc::c_int
            as usize][0 as libc::c_int as usize] = '0' as i32;
        gamecontroldefault[i
            as usize][GC_TOSSFLAG as libc::c_int
            as usize][0 as libc::c_int as usize] = '\'' as i32;
        gamecontroldefault[i
            as usize][GC_CAMTOGGLE as libc::c_int
            as usize][0 as libc::c_int as usize] = 'v' as i32;
        gamecontroldefault[i
            as usize][GC_CAMRESET as libc::c_int
            as usize][0 as libc::c_int as usize] = 'r' as i32;
        gamecontroldefault[i
            as usize][GC_TALKKEY as libc::c_int
            as usize][0 as libc::c_int as usize] = 't' as i32;
        gamecontroldefault[i
            as usize][GC_TEAMKEY as libc::c_int
            as usize][0 as libc::c_int as usize] = 'y' as i32;
        gamecontroldefault[i
            as usize][GC_SCORES as libc::c_int
            as usize][0 as libc::c_int as usize] = 9 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CONSOLE as libc::c_int
            as usize][0 as libc::c_int as usize] = '`' as i32;
        gamecontroldefault[i
            as usize][GC_PAUSE as libc::c_int
            as usize][0 as libc::c_int as usize] = 'p' as i32;
        gamecontroldefault[i
            as usize][GC_SCREENSHOT as libc::c_int
            as usize][0 as libc::c_int
            as usize] = 0x80 as libc::c_int + 0x42 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_RECORDGIF as libc::c_int
            as usize][0 as libc::c_int
            as usize] = 0x80 as libc::c_int + 0x43 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_VIEWPOINTNEXT as libc::c_int
            as usize][0 as libc::c_int
            as usize] = 0x80 as libc::c_int + 0x58 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_JUMP as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 0 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_SPIN as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 2 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CUSTOM1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 1 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CUSTOM2 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 3 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CUSTOM3 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 8 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CAMTOGGLE as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 4 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_CENTERVIEW as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 5 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_SCREENSHOT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 6 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_SYSTEMMENU as libc::c_int
            as usize][0 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 7 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_WEAPONPREV as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_HAT1 as libc::c_int + 2 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_WEAPONNEXT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_HAT1 as libc::c_int + 3 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_VIEWPOINTNEXT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_JOY1 as libc::c_int + 9 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_TOSSFLAG as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_HAT1 as libc::c_int + 0 as libc::c_int;
        gamecontroldefault[i
            as usize][GC_SCORES as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_HAT1 as libc::c_int + 1 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_JUMP as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 0 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_SPIN as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 2 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_CUSTOM1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 1 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_CUSTOM2 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 3 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_CUSTOM3 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 8 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_CAMTOGGLE as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 4 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_CENTERVIEW as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 5 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_SCREENSHOT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 6 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_WEAPONPREV as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2HAT1 as libc::c_int + 2 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_WEAPONNEXT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2HAT1 as libc::c_int + 3 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_VIEWPOINTNEXT as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2JOY1 as libc::c_int + 9 as libc::c_int;
        gamecontrolbisdefault[i
            as usize][GC_TOSSFLAG as libc::c_int
            as usize][1 as libc::c_int
            as usize] = KEY_2HAT1 as libc::c_int + 0 as libc::c_int;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_GetControlScheme(
    mut fromcontrols: *mut [int32_t; 2],
    mut gclist: *const int32_t,
    mut gclen: int32_t,
) -> int32_t {
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut gc: int32_t = 0;
    let mut skipscheme: boolean = 0;
    i = 1 as libc::c_int;
    while i < num_gamecontrolschemes as libc::c_int {
        skipscheme = false_0 as libc::c_int;
        j = 0 as libc::c_int;
        while j
            < (if !gclist.is_null() && gclen != 0 {
                gclen
            } else {
                NUM_GAMECONTROLS as libc::c_int
            })
        {
            gc = if !gclist.is_null() && gclen != 0 {
                *gclist.offset(j as isize)
            } else {
                j
            };
            if (if (*fromcontrols.offset(gc as isize))[0 as libc::c_int as usize] != 0
                && gamecontroldefault[i as usize][gc as usize][0 as libc::c_int as usize]
                    != 0
            {
                ((*fromcontrols.offset(gc as isize))[0 as libc::c_int as usize]
                    != gamecontroldefault[i
                        as usize][gc as usize][0 as libc::c_int as usize]) as libc::c_int
            } else {
                true_0 as libc::c_int
            }) != 0
                && (if (*fromcontrols.offset(gc as isize))[0 as libc::c_int as usize]
                    != 0
                    && gamecontroldefault[i
                        as usize][gc as usize][1 as libc::c_int as usize] != 0
                {
                    ((*fromcontrols.offset(gc as isize))[0 as libc::c_int as usize]
                        != gamecontroldefault[i
                            as usize][gc as usize][1 as libc::c_int as usize])
                        as libc::c_int
                } else {
                    true_0 as libc::c_int
                }) != 0
                && (if (*fromcontrols.offset(gc as isize))[1 as libc::c_int as usize]
                    != 0
                    && gamecontroldefault[i
                        as usize][gc as usize][0 as libc::c_int as usize] != 0
                {
                    ((*fromcontrols.offset(gc as isize))[1 as libc::c_int as usize]
                        != gamecontroldefault[i
                            as usize][gc as usize][0 as libc::c_int as usize])
                        as libc::c_int
                } else {
                    true_0 as libc::c_int
                }) != 0
                && (if (*fromcontrols.offset(gc as isize))[1 as libc::c_int as usize]
                    != 0
                    && gamecontroldefault[i
                        as usize][gc as usize][1 as libc::c_int as usize] != 0
                {
                    ((*fromcontrols.offset(gc as isize))[1 as libc::c_int as usize]
                        != gamecontroldefault[i
                            as usize][gc as usize][1 as libc::c_int as usize])
                        as libc::c_int
                } else {
                    true_0 as libc::c_int
                }) != 0
            {
                skipscheme = true_0 as libc::c_int;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if skipscheme == 0 {
            return i;
        }
        i += 1;
        i;
    }
    return gcs_custom as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn G_CopyControls(
    mut setupcontrols: *mut [int32_t; 2],
    mut fromcontrols: *mut [int32_t; 2],
    mut gclist: *const int32_t,
    mut gclen: int32_t,
) {
    let mut i: int32_t = 0;
    let mut gc: int32_t = 0;
    i = 0 as libc::c_int;
    while i
        < (if !gclist.is_null() && gclen != 0 {
            gclen
        } else {
            NUM_GAMECONTROLS as libc::c_int
        })
    {
        gc = if !gclist.is_null() && gclen != 0 {
            *gclist.offset(i as isize)
        } else {
            i
        };
        (*setupcontrols
            .offset(
                gc as isize,
            ))[0 as libc::c_int
            as usize] = (*fromcontrols.offset(gc as isize))[0 as libc::c_int as usize];
        (*setupcontrols
            .offset(
                gc as isize,
            ))[1 as libc::c_int
            as usize] = (*fromcontrols.offset(gc as isize))[1 as libc::c_int as usize];
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_SaveKeySetting(
    mut f: *mut FILE,
    mut fromcontrols: *mut [int32_t; 2],
    mut fromcontrolsbis: *mut [int32_t; 2],
) {
    let mut i: int32_t = 0;
    i = 1 as libc::c_int;
    while i < NUM_GAMECONTROLS as libc::c_int {
        fprintf(
            f,
            b"setcontrol \"%s\" \"%s\"\0" as *const u8 as *const libc::c_char,
            gamecontrolname[i as usize],
            G_KeyNumToName((*fromcontrols.offset(i as isize))[0 as libc::c_int as usize]),
        );
        if (*fromcontrols.offset(i as isize))[1 as libc::c_int as usize] != 0 {
            fprintf(
                f,
                b" \"%s\"\n\0" as *const u8 as *const libc::c_char,
                G_KeyNumToName(
                    (*fromcontrols.offset(i as isize))[1 as libc::c_int as usize],
                ),
            );
        } else {
            fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    i = 1 as libc::c_int;
    while i < NUM_GAMECONTROLS as libc::c_int {
        fprintf(
            f,
            b"setcontrol2 \"%s\" \"%s\"\0" as *const u8 as *const libc::c_char,
            gamecontrolname[i as usize],
            G_KeyNumToName(
                (*fromcontrolsbis.offset(i as isize))[0 as libc::c_int as usize],
            ),
        );
        if (*fromcontrolsbis.offset(i as isize))[1 as libc::c_int as usize] != 0 {
            fprintf(
                f,
                b" \"%s\"\n\0" as *const u8 as *const libc::c_char,
                G_KeyNumToName(
                    (*fromcontrolsbis.offset(i as isize))[1 as libc::c_int as usize],
                ),
            );
        } else {
            fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_CheckDoubleUsage(
    mut keynum: int32_t,
    mut modify: boolean,
) -> int32_t {
    let mut result: int32_t = GC_NULL as libc::c_int;
    if cv_controlperkey.value == 1 as libc::c_int {
        let mut i: int32_t = 0;
        i = 0 as libc::c_int;
        while i < NUM_GAMECONTROLS as libc::c_int {
            if gamecontrol[i as usize][0 as libc::c_int as usize] == keynum {
                result = i;
                if modify != 0 {
                    gamecontrol[i
                        as usize][0 as libc::c_int as usize] = 0 as libc::c_int;
                }
            }
            if gamecontrol[i as usize][1 as libc::c_int as usize] == keynum {
                result = i;
                if modify != 0 {
                    gamecontrol[i
                        as usize][1 as libc::c_int as usize] = 0 as libc::c_int;
                }
            }
            if gamecontrolbis[i as usize][0 as libc::c_int as usize] == keynum {
                result = i;
                if modify != 0 {
                    gamecontrolbis[i
                        as usize][0 as libc::c_int as usize] = 0 as libc::c_int;
                }
            }
            if gamecontrolbis[i as usize][1 as libc::c_int as usize] == keynum {
                result = i;
                if modify != 0 {
                    gamecontrolbis[i
                        as usize][1 as libc::c_int as usize] = 0 as libc::c_int;
                }
            }
            if result != 0 && modify == 0 {
                return result;
            }
            i += 1;
            i;
        }
    }
    return result;
}
unsafe extern "C" fn G_FilterKeyByVersion(
    mut numctrl: int32_t,
    mut keyidx: int32_t,
    mut player: int32_t,
    mut keynum1: *mut int32_t,
    mut keynum2: *mut int32_t,
    mut nestedoverride: *mut boolean,
) -> int32_t {
    if keyidx == 0 as libc::c_int && *keynum1 == 255 as libc::c_int {
        if *keynum2 != 255 as libc::c_int {
            *keynum1 = *keynum2;
            *keynum2 = 0 as libc::c_int;
        } else {
            return -(1 as libc::c_int)
        }
    } else if keyidx == 1 as libc::c_int && *keynum2 == 255 as libc::c_int {
        return -(1 as libc::c_int)
    }
    if (cv_execversion.value & 0xffff as libc::c_int) < 27 as libc::c_int
        && (numctrl == GC_WEAPONNEXT as libc::c_int
            || numctrl == GC_WEAPONPREV as libc::c_int
            || numctrl == GC_TOSSFLAG as libc::c_int || numctrl == GC_SPIN as libc::c_int
            || numctrl == GC_CAMRESET as libc::c_int || numctrl == GC_JUMP as libc::c_int
            || numctrl == GC_PAUSE as libc::c_int
            || numctrl == GC_SYSTEMMENU as libc::c_int
            || numctrl == GC_CAMTOGGLE as libc::c_int
            || numctrl == GC_SCREENSHOT as libc::c_int
            || numctrl == GC_TALKKEY as libc::c_int
            || numctrl == GC_SCORES as libc::c_int
            || numctrl == GC_CENTERVIEW as libc::c_int)
    {
        let mut keynum: int32_t = 0 as libc::c_int;
        let mut existingctrl: int32_t = 0 as libc::c_int;
        let mut defaultkey: int32_t = 0;
        let mut defaultoverride: boolean = false_0 as libc::c_int;
        if player == 0 as libc::c_int && numctrl == GC_SYSTEMMENU as libc::c_int {
            defaultkey = gamecontrol[numctrl as usize][0 as libc::c_int as usize];
        } else {
            defaultkey = if player == 1 as libc::c_int {
                gamecontrolbis[numctrl as usize][0 as libc::c_int as usize]
            } else {
                gamecontrol[numctrl as usize][1 as libc::c_int as usize]
            };
        }
        if keyidx == 0 as libc::c_int && *keynum1 == 0 {
            if *keynum2 != 0 {
                *keynum1 = *keynum2;
                *keynum2 = 0 as libc::c_int;
                keynum = *keynum1;
            } else {
                keynum = defaultkey;
                defaultoverride = true_0 as libc::c_int;
            }
        } else if keyidx == 1 as libc::c_int
            && (*keynum2 == 0 || *keynum1 == 0 && *keynum2 != 0)
        {
            keynum = defaultkey;
            defaultoverride = true_0 as libc::c_int;
        } else {
            keynum = if keyidx == 1 as libc::c_int { *keynum2 } else { *keynum1 };
        }
        if *nestedoverride != 0 {
            defaultoverride = true_0 as libc::c_int;
            *nestedoverride = false_0 as libc::c_int;
        }
        if keyidx == 0 as libc::c_int && *keynum2 == 0 {
            *keynum2 = defaultkey;
            *nestedoverride = true_0 as libc::c_int;
            if *keynum1 == *keynum2 {
                *keynum2 = 0 as libc::c_int;
                *nestedoverride = false_0 as libc::c_int;
            }
        }
        if defaultoverride != 0 {
            existingctrl = G_CheckDoubleUsage(keynum, false_0 as libc::c_int);
        }
        if keynum != 0 && (existingctrl == 0 || existingctrl == numctrl) {
            return keynum
        } else if keyidx == 0 as libc::c_int && *keynum2 != 0 {
            *keynum1 = *keynum2;
            *keynum2 = 0 as libc::c_int;
            return G_FilterKeyByVersion(
                numctrl,
                keyidx,
                player,
                keynum1,
                keynum2,
                nestedoverride,
            );
        } else {
            return 0 as libc::c_int
        }
    }
    if keyidx == 1 as libc::c_int { return *keynum2 } else { return *keynum1 };
}
unsafe extern "C" fn setcontrol(mut gc: *mut [int32_t; 2]) {
    let mut numctrl: int32_t = 0;
    let mut namectrl: *const libc::c_char = 0 as *const libc::c_char;
    let mut keynum: int32_t = 0;
    let mut keynum1: int32_t = 0;
    let mut keynum2: int32_t = 0 as libc::c_int;
    let mut player: int32_t = if gc as *mut libc::c_void
        == &mut gamecontrolbis as *mut [[int32_t; 2]; 43] as *mut libc::c_void
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut nestedoverride: boolean = false_0 as libc::c_int;
    namectrl = if strcasecmp(
        COM_Argv(1 as libc::c_int as size_t),
        b"use\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        COM_Argv(1 as libc::c_int as size_t)
    } else {
        b"spin\0" as *const u8 as *const libc::c_char
    };
    numctrl = 0 as libc::c_int;
    while numctrl < NUM_GAMECONTROLS as libc::c_int
        && strcasecmp(namectrl, gamecontrolname[numctrl as usize]) != 0
    {
        numctrl += 1;
        numctrl;
    }
    if numctrl == NUM_GAMECONTROLS as libc::c_int {
        CONS_Printf(
            b"Control '%s' unknown\n\0" as *const u8 as *const libc::c_char,
            namectrl,
        );
        return;
    }
    keynum1 = G_KeyNameToNum(COM_Argv(2 as libc::c_int as size_t));
    if COM_Argc() > 3 as libc::c_int as size_t {
        keynum2 = G_KeyNameToNum(COM_Argv(3 as libc::c_int as size_t));
    }
    keynum = G_FilterKeyByVersion(
        numctrl,
        0 as libc::c_int,
        player,
        &mut keynum1,
        &mut keynum2,
        &mut nestedoverride,
    );
    if keynum >= 0 as libc::c_int {
        G_CheckDoubleUsage(keynum, true_0 as libc::c_int);
        if keynum == 0 && keynum2 != 0 {
            keynum1 = keynum2;
            keynum2 = 0 as libc::c_int;
            keynum = G_FilterKeyByVersion(
                numctrl,
                0 as libc::c_int,
                player,
                &mut keynum1,
                &mut keynum2,
                &mut nestedoverride,
            );
            if keynum >= 0 as libc::c_int {
                G_CheckDoubleUsage(keynum, true_0 as libc::c_int);
            }
        }
    }
    if keynum >= 0 as libc::c_int {
        (*gc.offset(numctrl as isize))[0 as libc::c_int as usize] = keynum;
    }
    if keynum2 != 0 {
        keynum = G_FilterKeyByVersion(
            numctrl,
            1 as libc::c_int,
            player,
            &mut keynum1,
            &mut keynum2,
            &mut nestedoverride,
        );
        if keynum >= 0 as libc::c_int {
            if keynum != (*gc.offset(numctrl as isize))[0 as libc::c_int as usize] {
                (*gc.offset(numctrl as isize))[1 as libc::c_int as usize] = keynum;
            } else {
                (*gc
                    .offset(
                        numctrl as isize,
                    ))[1 as libc::c_int as usize] = 0 as libc::c_int;
            }
        }
    } else {
        (*gc.offset(numctrl as isize))[1 as libc::c_int as usize] = 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Command_Setcontrol_f() {
    let mut na: int32_t = 0;
    na = COM_Argc() as int32_t;
    if na != 3 as libc::c_int && na != 4 as libc::c_int {
        CONS_Printf(
            b"setcontrol <controlname> <keyname> [<2nd keyname>]: set controls for player 1\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    setcontrol(gamecontrol.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Command_Setcontrol2_f() {
    let mut na: int32_t = 0;
    na = COM_Argc() as int32_t;
    if na != 3 as libc::c_int && na != 4 as libc::c_int {
        CONS_Printf(
            b"setcontrol2 <controlname> <keyname> [<2nd keyname>]: set controls for player 2\n\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    setcontrol(gamecontrolbis.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn G_SetMouseDeltas(
    mut dx: int32_t,
    mut dy: int32_t,
    mut ssplayer: uint8_t,
) {
    let mut m: *mut mouse_t = if ssplayer as libc::c_int == 1 as libc::c_int {
        &mut mouse
    } else {
        &mut mouse2
    };
    let mut cvsens: *mut consvar_t = 0 as *mut consvar_t;
    let mut cvysens: *mut consvar_t = 0 as *mut consvar_t;
    cvsens = if ssplayer as libc::c_int == 1 as libc::c_int {
        &mut cv_mousesens
    } else {
        &mut cv_mousesens2
    };
    cvysens = if ssplayer as libc::c_int == 1 as libc::c_int {
        &mut cv_mouseysens
    } else {
        &mut cv_mouseysens2
    };
    (*m).rdx = dx;
    (*m).rdy = dy;
    (*m)
        .dx = ((*m).rdx as libc::c_float
        * (((*cvsens).value * (*cvsens).value) as libc::c_float / 110.0f32 + 0.1f32))
        as int32_t;
    (*m)
        .dy = ((*m).rdy as libc::c_float
        * (((*cvsens).value * (*cvsens).value) as libc::c_float / 110.0f32 + 0.1f32))
        as int32_t;
    (*m)
        .mlookdy = ((*m).rdy as libc::c_float
        * (((*cvysens).value * (*cvsens).value) as libc::c_float / 110.0f32 + 0.1f32))
        as int32_t;
}
