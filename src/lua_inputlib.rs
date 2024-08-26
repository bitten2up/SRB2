use ::libc;
extern "C" {
    pub type lua_State;
    static mut mouse: mouse_t;
    static mut gamekeydown: [uint8_t; 484];
    static mut gamecontrol: [[int32_t; 2]; 43];
    static mut gamecontrolbis: [[int32_t; 2]; 43];
    fn G_KeyNumToName(keynum: int32_t) -> *const libc::c_char;
    fn G_KeyNameToNum(keystr: *const libc::c_char) -> int32_t;
    fn JoyAxis(axissel: joyaxis_e) -> int32_t;
    fn Joy2Axis(axissel: joyaxis_e) -> int32_t;
    static mut shiftxform: *mut libc::c_char;
    fn I_GetCursorPosition(x: *mut int32_t, y: *mut int32_t);
    fn I_UpdateMouseGrab();
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
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
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn luaL_checkboolean(L: *mut lua_State, narg: libc::c_int) -> boolean;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const NUMINPUTS: C2RustUnnamed_0 = 484;
pub const KEY_2MOUSEWHEELDOWN: C2RustUnnamed_0 = 483;
pub const KEY_2MOUSEWHEELUP: C2RustUnnamed_0 = 482;
pub const KEY_MOUSEWHEELDOWN: C2RustUnnamed_0 = 481;
pub const KEY_MOUSEWHEELUP: C2RustUnnamed_0 = 480;
pub const KEY_DBL2HAT1: C2RustUnnamed_0 = 464;
pub const KEY_DBL2JOY1: C2RustUnnamed_0 = 432;
pub const KEY_DBL2MOUSE1: C2RustUnnamed_0 = 424;
pub const KEY_2HAT1: C2RustUnnamed_0 = 408;
pub const KEY_2JOY1: C2RustUnnamed_0 = 376;
pub const KEY_2MOUSE1: C2RustUnnamed_0 = 368;
pub const KEY_DBLHAT1: C2RustUnnamed_0 = 352;
pub const KEY_DBLJOY1: C2RustUnnamed_0 = 320;
pub const KEY_DBLMOUSE1: C2RustUnnamed_0 = 312;
pub const KEY_HAT1: C2RustUnnamed_0 = 296;
pub const KEY_JOY1: C2RustUnnamed_0 = 264;
pub const KEY_MOUSE1: C2RustUnnamed_0 = 256;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const NUM_GAMECONTROLS: C2RustUnnamed_1 = 43;
pub const GC_CUSTOM3: C2RustUnnamed_1 = 42;
pub const GC_CUSTOM2: C2RustUnnamed_1 = 41;
pub const GC_CUSTOM1: C2RustUnnamed_1 = 40;
pub const GC_VIEWPOINTPREV: C2RustUnnamed_1 = 39;
pub const GC_VIEWPOINTNEXT: C2RustUnnamed_1 = 38;
pub const GC_RECORDGIF: C2RustUnnamed_1 = 37;
pub const GC_SCREENSHOT: C2RustUnnamed_1 = 36;
pub const GC_SYSTEMMENU: C2RustUnnamed_1 = 35;
pub const GC_PAUSE: C2RustUnnamed_1 = 34;
pub const GC_CONSOLE: C2RustUnnamed_1 = 33;
pub const GC_JUMP: C2RustUnnamed_1 = 32;
pub const GC_SCORES: C2RustUnnamed_1 = 31;
pub const GC_TEAMKEY: C2RustUnnamed_1 = 30;
pub const GC_TALKKEY: C2RustUnnamed_1 = 29;
pub const GC_MOUSEAIMING: C2RustUnnamed_1 = 28;
pub const GC_CENTERVIEW: C2RustUnnamed_1 = 27;
pub const GC_LOOKDOWN: C2RustUnnamed_1 = 26;
pub const GC_LOOKUP: C2RustUnnamed_1 = 25;
pub const GC_CAMRESET: C2RustUnnamed_1 = 24;
pub const GC_CAMTOGGLE: C2RustUnnamed_1 = 23;
pub const GC_SPIN: C2RustUnnamed_1 = 22;
pub const GC_TOSSFLAG: C2RustUnnamed_1 = 21;
pub const GC_FIRENORMAL: C2RustUnnamed_1 = 20;
pub const GC_FIRE: C2RustUnnamed_1 = 19;
pub const GC_WEPSLOT10: C2RustUnnamed_1 = 18;
pub const GC_WEPSLOT9: C2RustUnnamed_1 = 17;
pub const GC_WEPSLOT8: C2RustUnnamed_1 = 16;
pub const GC_WEPSLOT7: C2RustUnnamed_1 = 15;
pub const GC_WEPSLOT6: C2RustUnnamed_1 = 14;
pub const GC_WEPSLOT5: C2RustUnnamed_1 = 13;
pub const GC_WEPSLOT4: C2RustUnnamed_1 = 12;
pub const GC_WEPSLOT3: C2RustUnnamed_1 = 11;
pub const GC_WEPSLOT2: C2RustUnnamed_1 = 10;
pub const GC_WEPSLOT1: C2RustUnnamed_1 = 9;
pub const GC_WEAPONPREV: C2RustUnnamed_1 = 8;
pub const GC_WEAPONNEXT: C2RustUnnamed_1 = 7;
pub const GC_TURNRIGHT: C2RustUnnamed_1 = 6;
pub const GC_TURNLEFT: C2RustUnnamed_1 = 5;
pub const GC_STRAFERIGHT: C2RustUnnamed_1 = 4;
pub const GC_STRAFELEFT: C2RustUnnamed_1 = 3;
pub const GC_BACKWARD: C2RustUnnamed_1 = 2;
pub const GC_FORWARD: C2RustUnnamed_1 = 1;
pub const GC_NULL: C2RustUnnamed_1 = 0;
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
pub type joyaxis_e = libc::c_uint;
pub const JA_FIRENORMAL: joyaxis_e = 8;
pub const JA_FIRE: joyaxis_e = 7;
pub const JA_SPIN: joyaxis_e = 6;
pub const JA_JUMP: joyaxis_e = 5;
pub const JA_DIGITAL: joyaxis_e = 5;
pub const JA_STRAFE: joyaxis_e = 4;
pub const JA_LOOK: joyaxis_e = 3;
pub const JA_MOVE: joyaxis_e = 2;
pub const JA_TURN: joyaxis_e = 1;
pub const JA_NONE: joyaxis_e = 0;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
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
pub static mut mousegrabbedbylua: boolean = true_0 as libc::c_int;
unsafe extern "C" fn lib_gameControlDown(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUM_GAMECONTROLS as libc::c_int {
        return luaL_error(
            L,
            b"GC_* constant %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUM_GAMECONTROLS as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushnumber(
        L,
        (gamekeydown[gamecontrol[i as usize][0 as libc::c_int as usize] as usize]
            as libc::c_int != 0
            || gamekeydown[gamecontrol[i as usize][1 as libc::c_int as usize] as usize]
                as libc::c_int != 0) as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_gameControl2Down(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUM_GAMECONTROLS as libc::c_int {
        return luaL_error(
            L,
            b"GC_* constant %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUM_GAMECONTROLS as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushnumber(
        L,
        (gamekeydown[gamecontrolbis[i as usize][0 as libc::c_int as usize] as usize]
            as libc::c_int != 0
            || gamekeydown[gamecontrolbis[i as usize][1 as libc::c_int as usize]
                as usize] as libc::c_int != 0) as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_gameControlToKeyNum(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUM_GAMECONTROLS as libc::c_int {
        return luaL_error(
            L,
            b"GC_* constant %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUM_GAMECONTROLS as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushnumber(L, gamecontrol[i as usize][0 as libc::c_int as usize]);
    lua_pushnumber(L, gamecontrol[i as usize][1 as libc::c_int as usize]);
    return 2 as libc::c_int;
}
unsafe extern "C" fn lib_gameControl2ToKeyNum(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUM_GAMECONTROLS as libc::c_int {
        return luaL_error(
            L,
            b"GC_* constant %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUM_GAMECONTROLS as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushnumber(L, gamecontrolbis[i as usize][0 as libc::c_int as usize]);
    lua_pushnumber(L, gamecontrolbis[i as usize][1 as libc::c_int as usize]);
    return 2 as libc::c_int;
}
unsafe extern "C" fn lib_joyAxis(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    lua_pushnumber(L, JoyAxis(i as joyaxis_e));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_joy2Axis(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    lua_pushnumber(L, Joy2Axis(i as joyaxis_e));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_keyNumToName(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    lua_pushstring(L, G_KeyNumToName(i));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_keyNameToNum(mut L: *mut lua_State) -> libc::c_int {
    let mut str: *const libc::c_char = luaL_checklstring(
        L,
        1 as libc::c_int,
        0 as *mut size_t,
    );
    lua_pushnumber(L, G_KeyNameToNum(str));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_keyNumPrintable(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    lua_pushboolean(
        L,
        (i >= 32 as libc::c_int && i <= 127 as libc::c_int) as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_shiftKeyNum(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    if i >= 32 as libc::c_int && i <= 127 as libc::c_int {
        lua_pushnumber(L, *shiftxform.offset(i as isize) as lua_Number);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_getMouseGrab(mut L: *mut lua_State) -> libc::c_int {
    lua_pushboolean(L, mousegrabbedbylua);
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_setMouseGrab(mut L: *mut lua_State) -> libc::c_int {
    mousegrabbedbylua = luaL_checkboolean(L, 1 as libc::c_int);
    I_UpdateMouseGrab();
    return 0 as libc::c_int;
}
unsafe extern "C" fn lib_getCursorPosition(mut L: *mut lua_State) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    I_GetCursorPosition(&mut x, &mut y);
    lua_pushnumber(L, x);
    lua_pushnumber(L, y);
    return 2 as libc::c_int;
}
static mut lib: [luaL_Reg; 14] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"gameControlDown\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_gameControlDown
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gameControl2Down\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_gameControl2Down
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gameControlToKeyNum\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_gameControlToKeyNum
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"gameControl2ToKeyNum\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_gameControl2ToKeyNum
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"joyAxis\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_joyAxis as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"joy2Axis\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_joy2Axis as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"keyNumToName\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_keyNumToName
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"keyNameToNum\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_keyNameToNum
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"keyNumPrintable\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_keyNumPrintable
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"shiftKeyNum\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_shiftKeyNum
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getMouseGrab\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_getMouseGrab
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"setMouseGrab\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_setMouseGrab
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"getCursorPosition\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_getCursorPosition
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: 0 as *const libc::c_char,
                func: None,
            };
            init
        },
    ]
};
unsafe extern "C" fn lib_getGameKeyDown(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 2 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUMINPUTS as libc::c_int {
        return luaL_error(
            L,
            b"gamekeydown[] index %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUMINPUTS as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushboolean(L, gamekeydown[i as usize] as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_setGameKeyDown(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = luaL_checknumber(L, 2 as libc::c_int);
    let mut j: boolean = luaL_checkboolean(L, 3 as libc::c_int);
    if i < 0 as libc::c_int || i >= NUMINPUTS as libc::c_int {
        return luaL_error(
            L,
            b"gamekeydown[] index %d out of range (0 - %d)\0" as *const u8
                as *const libc::c_char,
            i,
            NUMINPUTS as libc::c_int - 1 as libc::c_int,
        );
    }
    gamekeydown[i as usize] = j as uint8_t;
    return 0 as libc::c_int;
}
unsafe extern "C" fn lib_lenGameKeyDown(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, NUMINPUTS as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn keyevent_get(mut L: *mut lua_State) -> libc::c_int {
    let mut event: *mut event_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"KEYEVENT_T*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut event_t);
    let mut field: *const libc::c_char = luaL_checklstring(
        L,
        2 as libc::c_int,
        0 as *mut size_t,
    );
    if fastcmp(field, b"name\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushstring(L, G_KeyNumToName((*event).key));
    } else if fastcmp(field, b"num\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*event).key);
    } else if fastcmp(field, b"repeated\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushboolean(L, (*event).repeated);
    } else {
        return luaL_error(
            L,
            b"keyevent_t has no field named %s\0" as *const u8 as *const libc::c_char,
            field,
        )
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mouse_get(mut L: *mut lua_State) -> libc::c_int {
    let mut m: *mut mouse_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"MOUSE_T*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut mouse_t);
    let mut field: *const libc::c_char = luaL_checklstring(
        L,
        2 as libc::c_int,
        0 as *mut size_t,
    );
    if fastcmp(field, b"dx\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).dx);
    } else if fastcmp(field, b"dy\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).dy);
    } else if fastcmp(field, b"mlookdy\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).mlookdy);
    } else if fastcmp(field, b"rdx\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).rdx);
    } else if fastcmp(field, b"rdy\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).rdy);
    } else if fastcmp(field, b"buttons\0" as *const u8 as *const libc::c_char) != 0 {
        lua_pushnumber(L, (*m).buttons as lua_Number);
    } else {
        return luaL_error(
            L,
            b"mouse_t has no field named %s\0" as *const u8 as *const libc::c_char,
            field,
        )
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn mouse_num(mut L: *mut lua_State) -> libc::c_int {
    let mut m: *mut mouse_t = *(luaL_checkudata(
        L,
        1 as libc::c_int,
        b"MOUSE_T*\0" as *const u8 as *const libc::c_char,
    ) as *mut *mut mouse_t);
    lua_pushnumber(
        L,
        if m == &mut mouse as *mut mouse_t { 1 as libc::c_int } else { 2 as libc::c_int },
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_InputLib(mut L: *mut lua_State) -> libc::c_int {
    lua_newuserdata(L, 0 as libc::c_int as size_t);
    lua_createtable(L, 0 as libc::c_int, 2 as libc::c_int);
    lua_pushcclosure(
        L,
        Some(lib_getGameKeyDown as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(lib_setGameKeyDown as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__newindex\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(lib_lenGameKeyDown as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -(2 as libc::c_int));
    lua_setfield(
        L,
        -(10002 as libc::c_int),
        b"gamekeydown\0" as *const u8 as *const libc::c_char,
    );
    luaL_newmetatable(L, b"KEYEVENT_T*\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(keyevent_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    luaL_newmetatable(L, b"MOUSE_T*\0" as *const u8 as *const libc::c_char);
    lua_pushcclosure(
        L,
        Some(mouse_get as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(
        L,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        L,
        Some(mouse_num as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_setfield(L, -(2 as libc::c_int), b"__len\0" as *const u8 as *const libc::c_char);
    lua_settop(L, -(1 as libc::c_int) - 1 as libc::c_int);
    luaL_register(L, b"input\0" as *const u8 as *const libc::c_char, lib.as_mut_ptr());
    return 0 as libc::c_int;
}
