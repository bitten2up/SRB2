use ::libc;
extern "C" {
    fn modf(_: libc::c_double, _: *mut libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn CV_RegisterVar(variable: *mut consvar_t);
    static mut cv_sleep: consvar_t;
    fn I_Sleep(ms: uint32_t);
    fn I_GetPrecisePrecision() -> uint64_t;
    fn I_StartupTimer();
    fn I_GetPreciseTime() -> precise_t;
}
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type tic_t = uint32_t;
pub type precise_t = uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const CV_ALLOWLUA: C2RustUnnamed = 4096;
pub const CV_CHEAT: C2RustUnnamed = 2048;
pub const CV_HIDEN: C2RustUnnamed = 1024;
pub const CV_NOSHOWHELP: C2RustUnnamed = 512;
pub const CV_SHOWMODIFONETIME: C2RustUnnamed = 256;
pub const CV_SHOWMODIF: C2RustUnnamed = 128;
pub const CV_MODIFIED: C2RustUnnamed = 64;
pub const CV_NOTINNET: C2RustUnnamed = 32;
pub const CV_FLOAT: C2RustUnnamed = 16;
pub const CV_NOINIT: C2RustUnnamed = 8;
pub const CV_NETVAR: C2RustUnnamed = 4;
pub const CV_CALL: C2RustUnnamed = 2;
pub const CV_SAVE: C2RustUnnamed = 1;
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
pub type fixed_t = int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timestate_s {
    pub time: tic_t,
    pub timefrac: fixed_t,
}
pub type timestate_t = timestate_s;
#[inline(always)]
unsafe extern "C" fn FixedToFloat(mut x: fixed_t) -> libc::c_float {
    return x as libc::c_float
        / ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float;
}
#[inline(always)]
unsafe extern "C" fn FloatToFixed(mut f: libc::c_float) -> fixed_t {
    return (f * ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_float) as fixed_t;
}
#[no_mangle]
pub static mut g_time: timestate_t = timestate_s {
    time: 0,
    timefrac: 0,
};
static mut timescale_cons_t: [CV_PossibleValue_t; 3] = [
    {
        let mut init = CV_PossibleValue_s {
            value: ((1 as libc::c_int) << 16 as libc::c_int) / 20 as libc::c_int,
            strvalue: b"MIN\0" as *const u8 as *const libc::c_char,
        };
        init
    },
    {
        let mut init = CV_PossibleValue_s {
            value: 20 as libc::c_int * ((1 as libc::c_int) << 16 as libc::c_int),
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
pub static mut cv_timescale: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"timescale\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"1.0\0" as *const u8 as *const libc::c_char,
            flags: CV_NETVAR as libc::c_int | CV_CHEAT as libc::c_int
                | CV_FLOAT as libc::c_int,
            PossibleValue: timescale_cons_t.as_ptr() as *mut _,
            func: None,
            value: 0 as libc::c_int,
            string: 0 as *const libc::c_char,
            zstring: 0 as *const libc::c_char as *mut libc::c_char,
            revert: {
                let mut init = C2RustUnnamed_0 {
                    allocated: 0 as libc::c_int as libc::c_char,
                    v: C2RustUnnamed_1 {
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
static mut enterprecise: precise_t = 0;
static mut oldenterprecise: precise_t = 0;
static mut entertic: fixed_t = 0;
static mut oldentertics: fixed_t = 0;
static mut tictimer: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn I_GetTime() -> tic_t {
    return g_time.time;
}
#[no_mangle]
pub unsafe extern "C" fn I_InitializeTime() {
    CV_RegisterVar(&mut cv_timescale);
    I_StartupTimer();
    g_time.time = 0 as libc::c_int as tic_t;
    g_time.timefrac = 0 as libc::c_int;
    enterprecise = I_GetPreciseTime();
    oldenterprecise = enterprecise;
    entertic = 0 as libc::c_int;
    oldentertics = 0 as libc::c_int;
    tictimer = 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn I_UpdateTime(mut timescale: fixed_t) {
    let mut ticratescaled: libc::c_double = 0.;
    let mut elapsedseconds: libc::c_double = 0.;
    let mut realtics: tic_t = 0;
    ticratescaled = 35 as libc::c_int as libc::c_double
        * FixedToFloat(timescale) as libc::c_double;
    enterprecise = I_GetPreciseTime();
    elapsedseconds = enterprecise.wrapping_sub(oldenterprecise) as libc::c_double
        / I_GetPrecisePrecision() as libc::c_double;
    tictimer += elapsedseconds;
    while tictimer > 1.0f64 / ticratescaled {
        entertic += 1 as libc::c_int;
        tictimer -= 1.0f64 / ticratescaled;
    }
    realtics = (entertic - oldentertics) as tic_t;
    oldentertics = entertic;
    oldenterprecise = enterprecise;
    g_time.time = (g_time.time).wrapping_add(realtics);
    let mut fractional: libc::c_double = 0.;
    let mut integral: libc::c_double = 0.;
    fractional = modf(tictimer * ticratescaled, &mut integral);
    g_time.timefrac = FloatToFixed(fractional as libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn I_SleepDuration(mut duration: precise_t) {
    let mut precision: uint64_t = I_GetPrecisePrecision();
    let mut sleepvalue: int32_t = cv_sleep.value;
    let mut delaygranularity: uint64_t = 0;
    let mut cur: precise_t = 0;
    let mut dest: precise_t = 0;
    let mut gran: libc::c_double = round(
        (precision / 1000 as libc::c_int as uint64_t) as libc::c_double
            * sleepvalue as libc::c_double * 2.1f64,
    );
    delaygranularity = gran as uint64_t;
    cur = I_GetPreciseTime();
    dest = cur.wrapping_add(duration);
    while dest.wrapping_sub(cur) as int64_t > 0 as libc::c_int as int64_t {
        if sleepvalue > 0 as libc::c_int && dest.wrapping_sub(cur) > delaygranularity {
            I_Sleep(sleepvalue as uint32_t);
        }
        cur = I_GetPreciseTime();
    }
}
