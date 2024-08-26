use ::libc;
extern "C" {
    pub type lua_State;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut numskincolors: uint16_t;
    static mut skincolors: [skincolor_t; 1182];
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn FixedSqrt(x: fixed_t) -> fixed_t;
    fn FixedAngle(fa: fixed_t) -> angle_t;
    static mut finesine: [fixed_t; 10240];
    static mut finecosine: *mut fixed_t;
    static mut finetangent: [fixed_t; 4096];
    fn AngleFixed(af: angle_t) -> fixed_t;
    fn FixedAcos(x: fixed_t) -> angle_t;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int);
    fn luaL_register(
        L: *mut lua_State,
        libname: *const libc::c_char,
        l: *const luaL_Reg,
    );
    fn luaL_checknumber(L: *mut lua_State, numArg: libc::c_int) -> lua_Number;
    fn luaL_optnumber(
        L: *mut lua_State,
        nArg: libc::c_int,
        def: lua_Number,
    ) -> lua_Number;
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn R_PointToDist2(px2: fixed_t, py2: fixed_t, px1: fixed_t, py1: fixed_t) -> fixed_t;
    fn Easing_OutExpo(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InCubic(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InSine(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutExpo(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InBack(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutBack(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutBack(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutQuart(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InQuint(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InExpo(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InQuart(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutCubic(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutCubic(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutQuart(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutQuad(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutQuad(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InQuad(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutSine(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutSine(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_Linear(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InOutQuint(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_OutQuint(t: fixed_t, start: fixed_t, end: fixed_t) -> fixed_t;
    fn Easing_InBackParameterized(
        t: fixed_t,
        start: fixed_t,
        end: fixed_t,
        param: fixed_t,
    ) -> fixed_t;
    fn Easing_OutBackParameterized(
        t: fixed_t,
        start: fixed_t,
        end: fixed_t,
        param: fixed_t,
    ) -> fixed_t;
    fn Easing_InOutBackParameterized(
        t: fixed_t,
        start: fixed_t,
        end: fixed_t,
        param: fixed_t,
    ) -> fixed_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct skincolor_s {
    pub name: [libc::c_char; 33],
    pub ramp: [uint8_t; 16],
    pub invcolor: uint16_t,
    pub invshade: uint8_t,
    pub chatcolor: uint16_t,
    pub accessible: boolean,
}
pub type skincolor_t = skincolor_s;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type fixed_t = int32_t;
pub type angle_t = uint32_t;
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
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
unsafe extern "C" fn FixedTrunc(mut x: fixed_t) -> fixed_t {
    let a: fixed_t = abs(x);
    let i: fixed_t = (a >> 16 as libc::c_int) << 16 as libc::c_int;
    let f: fixed_t = a - i;
    if x != -(2147483647 as libc::c_int) - 1 as libc::c_int {
        if x > 0 as libc::c_int { return x - f } else { return x + f }
    }
    return -(2147483647 as libc::c_int) - 1 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn FixedCeil(mut x: fixed_t) -> fixed_t {
    let a: fixed_t = abs(x);
    let i: fixed_t = (a >> 16 as libc::c_int) << 16 as libc::c_int;
    let f: fixed_t = a - i;
    if f == 0 as libc::c_int {
        return x;
    }
    if x == -(2147483647 as libc::c_int) - 1 as libc::c_int {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int
    } else if x < FixedFloor(2147483647 as libc::c_int) {
        if x > 0 as libc::c_int {
            return x + (((1 as libc::c_int) << 16 as libc::c_int) - f)
        } else {
            return x + f
        }
    }
    return 2147483647 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn FixedRound(mut x: fixed_t) -> fixed_t {
    let a: fixed_t = abs(x);
    let i: fixed_t = (a >> 16 as libc::c_int) << 16 as libc::c_int;
    let f: fixed_t = a - i;
    if f == 0 as libc::c_int {
        return x;
    }
    if x == -(2147483647 as libc::c_int) - 1 as libc::c_int {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int
    } else if x < FixedFloor(2147483647 as libc::c_int) {
        if x > 0 as libc::c_int {
            return x + (((1 as libc::c_int) << 16 as libc::c_int) - f)
        } else {
            return x - (((1 as libc::c_int) << 16 as libc::c_int) - f)
        }
    }
    return 2147483647 as libc::c_int;
}
#[inline(always)]
unsafe extern "C" fn InvAngle(mut a: angle_t) -> angle_t {
    return (0xffffffff as libc::c_uint)
        .wrapping_sub(a)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn lib_abs(mut L: *mut lua_State) -> libc::c_int {
    let mut a: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    lua_pushnumber(L, abs(a));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_min(mut L: *mut lua_State) -> libc::c_int {
    let mut a: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    let mut b: libc::c_int = luaL_checknumber(L, 2 as libc::c_int);
    lua_pushnumber(L, if a < b { a } else { b });
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_max(mut L: *mut lua_State) -> libc::c_int {
    let mut a: libc::c_int = luaL_checknumber(L, 1 as libc::c_int);
    let mut b: libc::c_int = luaL_checknumber(L, 2 as libc::c_int);
    lua_pushnumber(L, if a > b { a } else { b });
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedangle(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedAngle(luaL_checknumber(L, 1 as libc::c_int)) as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_anglefixed(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, AngleFixed(luaL_checknumber(L, 1 as libc::c_int) as angle_t));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_invangle(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        InvAngle(luaL_checknumber(L, 1 as libc::c_int) as angle_t) as lua_Number,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_finesine(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        finesine[(luaL_checknumber(L, 1 as libc::c_int) as angle_t >> 19 as libc::c_int
            & (8192 as libc::c_int - 1 as libc::c_int) as angle_t) as usize]
            >> 16 as libc::c_int - 16 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_finecosine(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        *finecosine
            .offset(
                (luaL_checknumber(L, 1 as libc::c_int) as angle_t >> 19 as libc::c_int
                    & (8192 as libc::c_int - 1 as libc::c_int) as angle_t) as isize,
            ) >> 16 as libc::c_int - 16 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_finetangent(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        finetangent[((luaL_checknumber(L, 1 as libc::c_int) as angle_t)
            .wrapping_add(0x40000000 as libc::c_int as angle_t) >> 19 as libc::c_int
            & 4095 as libc::c_int as angle_t) as usize]
            >> 16 as libc::c_int - 16 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedasin(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        (FixedAcos(luaL_checknumber(L, 1 as libc::c_int)))
            .wrapping_neg()
            .wrapping_add(0x40000000 as libc::c_int as angle_t) as lua_Number,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedacos(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedAcos(luaL_checknumber(L, 1 as libc::c_int)) as lua_Number);
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedmul(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        FixedMul(
            luaL_checknumber(L, 1 as libc::c_int),
            luaL_checknumber(L, 2 as libc::c_int),
        ),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedint(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedInt(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixeddiv(mut L: *mut lua_State) -> libc::c_int {
    let mut i: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut j: fixed_t = luaL_checknumber(L, 2 as libc::c_int);
    if j == 0 as libc::c_int {
        return luaL_error(L, b"divide by zero\0" as *const u8 as *const libc::c_char);
    }
    lua_pushnumber(L, FixedDiv(i, j));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedrem(mut L: *mut lua_State) -> libc::c_int {
    static mut seen: uint8_t = 0 as libc::c_int as uint8_t;
    if seen == 0 {
        seen = 1 as libc::c_int as uint8_t;
        CONS_Alert(
            CONS_WARNING,
            b"\"%s\" is deprecated and will be removed.\nUse \"%s\" instead.\n\0"
                as *const u8 as *const libc::c_char,
            b"FixedRem(a, b)\0" as *const u8 as *const libc::c_char,
            b"a % b\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as libc::c_int) % luaL_checknumber(L, 2 as libc::c_int),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedsqrt(mut L: *mut lua_State) -> libc::c_int {
    let mut i: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    if i < 0 as libc::c_int {
        return luaL_error(
            L,
            b"square root domain error\0" as *const u8 as *const libc::c_char,
        );
    }
    lua_pushnumber(L, FixedSqrt(i));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedhypot(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        R_PointToDist2(
            0 as libc::c_int,
            0 as libc::c_int,
            luaL_checknumber(L, 1 as libc::c_int),
            luaL_checknumber(L, 2 as libc::c_int),
        ),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedfloor(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedFloor(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedtrunc(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedTrunc(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedceil(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedCeil(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_fixedround(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(L, FixedRound(luaL_checknumber(L, 1 as libc::c_int)));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_getsecspecial(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1 as libc::c_int)
            >> (luaL_checknumber(L, 2 as libc::c_int) - 1 as libc::c_int)
                * 4 as libc::c_int & 15 as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_all7emeralds(mut L: *mut lua_State) -> libc::c_int {
    lua_pushboolean(
        L,
        (luaL_checknumber(L, 1 as libc::c_int)
            & (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
            == 1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int | 8 as libc::c_int
                | 16 as libc::c_int | 32 as libc::c_int | 64 as libc::c_int)
            as libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_coloropposite(mut L: *mut lua_State) -> libc::c_int {
    let mut colornum: uint16_t = luaL_checknumber(L, 1 as libc::c_int) as uint16_t;
    if colornum == 0 || colornum as libc::c_int >= numskincolors as libc::c_int {
        return luaL_error(
            L,
            b"skincolor %d out of range (1 - %d).\0" as *const u8 as *const libc::c_char,
            colornum as libc::c_int,
            numskincolors as libc::c_int - 1 as libc::c_int,
        );
    }
    lua_pushnumber(L, skincolors[colornum as usize].invcolor as lua_Number);
    lua_pushnumber(L, skincolors[colornum as usize].invshade as lua_Number);
    return 2 as libc::c_int;
}
static mut lib_math: [luaL_Reg; 37] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"abs\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_abs as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"min\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_min as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"max\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_max as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"sin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_finesine as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"cos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_finecosine as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"tan\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_finetangent
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"asin\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedasin as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"acos\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedacos as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedAngle\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedangle as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixangle\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedangle as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"AngleFixed\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_anglefixed as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"anglefix\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_anglefixed as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"InvAngle\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_invangle as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedMul\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedmul as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixmul\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedmul as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedInt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedint as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedDiv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixeddiv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixdiv\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixeddiv as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedRem\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedrem as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixrem\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedrem as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedSqrt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedsqrt as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixsqrt\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedsqrt as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedHypot\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedhypot as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixhypot\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedhypot as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedFloor\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedfloor as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixfloor\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedfloor as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedTrunc\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedtrunc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixtrunc\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedtrunc as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedCeil\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedceil as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixceil\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedceil as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"FixedRound\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedround as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"fixround\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_fixedround as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"GetSecSpecial\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_getsecspecial
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"All7Emeralds\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_all7emeralds
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"ColorOpposite\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_coloropposite
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
unsafe extern "C" fn lib_easelinear(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_Linear(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinsine(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InSine(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutsine(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutSine(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutsine(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutSine(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinquad(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InQuad(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutquad(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutQuad(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutquad(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutQuad(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeincubic(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InCubic(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutcubic(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutCubic(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutcubic(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutCubic(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinquart(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InQuart(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutquart(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutQuart(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutquart(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutQuart(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinquint(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InQuint(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutquint(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutQuint(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutquint(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutQuint(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinexpo(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InExpo(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutexpo(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_OutExpo(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutexpo(mut L: *mut lua_State) -> libc::c_int {
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_checknumber(L, 2 as libc::c_int);
        end = luaL_checknumber(L, 3 as libc::c_int);
    }
    lua_pushnumber(L, Easing_InOutExpo(t, start, end));
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinback(mut L: *mut lua_State) -> libc::c_int {
    let mut useparam: boolean = false_0 as libc::c_int;
    let mut param: fixed_t = 0 as libc::c_int;
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_optnumber(L, 2 as libc::c_int, start);
        end = luaL_optnumber(L, 3 as libc::c_int, end);
        if n >= 4 as libc::c_int
            && {
                useparam = !(lua_type(L, 4 as libc::c_int) == 0 as libc::c_int)
                    as libc::c_int;
                useparam != 0
            }
        {
            param = luaL_checknumber(L, 4 as libc::c_int);
        }
    }
    if useparam != 0 {
        lua_pushnumber(L, Easing_InBackParameterized(t, start, end, param));
    } else {
        lua_pushnumber(L, Easing_InBack(t, start, end));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeoutback(mut L: *mut lua_State) -> libc::c_int {
    let mut useparam: boolean = false_0 as libc::c_int;
    let mut param: fixed_t = 0 as libc::c_int;
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_optnumber(L, 2 as libc::c_int, start);
        end = luaL_optnumber(L, 3 as libc::c_int, end);
        if n >= 4 as libc::c_int
            && {
                useparam = !(lua_type(L, 4 as libc::c_int) == 0 as libc::c_int)
                    as libc::c_int;
                useparam != 0
            }
        {
            param = luaL_checknumber(L, 4 as libc::c_int);
        }
    }
    if useparam != 0 {
        lua_pushnumber(L, Easing_OutBackParameterized(t, start, end, param));
    } else {
        lua_pushnumber(L, Easing_OutBack(t, start, end));
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn lib_easeinoutback(mut L: *mut lua_State) -> libc::c_int {
    let mut useparam: boolean = false_0 as libc::c_int;
    let mut param: fixed_t = 0 as libc::c_int;
    let mut start: fixed_t = 0 as libc::c_int;
    let mut end: fixed_t = (1 as libc::c_int) << 16 as libc::c_int;
    let mut t: fixed_t = luaL_checknumber(L, 1 as libc::c_int);
    let mut n: libc::c_int = lua_gettop(L);
    if n == 2 as libc::c_int {
        end = luaL_checknumber(L, 2 as libc::c_int);
    } else if n >= 3 as libc::c_int {
        start = luaL_optnumber(L, 2 as libc::c_int, start);
        end = luaL_optnumber(L, 3 as libc::c_int, end);
        if n >= 4 as libc::c_int
            && {
                useparam = !(lua_type(L, 4 as libc::c_int) == 0 as libc::c_int)
                    as libc::c_int;
                useparam != 0
            }
        {
            param = luaL_checknumber(L, 4 as libc::c_int);
        }
    }
    if useparam != 0 {
        lua_pushnumber(L, Easing_InOutBackParameterized(t, start, end, param));
    } else {
        lua_pushnumber(L, Easing_InOutBack(t, start, end));
    }
    return 1 as libc::c_int;
}
static mut lib_ease: [luaL_Reg; 23] = unsafe {
    [
        {
            let mut init = luaL_Reg {
                name: b"linear\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easelinear as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"insine\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinsine as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outsine\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutsine
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutsine\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutsine
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inquad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinquad as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outquad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutquad
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutquad\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutquad
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"incubic\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeincubic
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outcubic\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutcubic
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutcubic\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutcubic
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inquart\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinquart
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outquart\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutquart
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutquart\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutquart
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inquint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinquint
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outquint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutquint
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutquint\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutquint
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inexpo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinexpo as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outexpo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutexpo
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutexpo\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutexpo
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inback\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinback as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"outback\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeoutback
                        as unsafe extern "C" fn(*mut lua_State) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = luaL_Reg {
                name: b"inoutback\0" as *const u8 as *const libc::c_char,
                func: Some(
                    lib_easeinoutback
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
#[no_mangle]
pub unsafe extern "C" fn LUA_MathLib(mut L: *mut lua_State) -> libc::c_int {
    lua_pushvalue(L, -(10002 as libc::c_int));
    luaL_register(L, 0 as *const libc::c_char, lib_math.as_mut_ptr());
    luaL_register(
        L,
        b"ease\0" as *const u8 as *const libc::c_char,
        lib_ease.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
