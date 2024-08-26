use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut textures: *mut *mut texture_t;
    static mut fovtan: fixed_t;
    static mut vid: viddef_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type fixed_t = int32_t;
pub type viddef_t = viddef_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_s {
    pub modenum: int32_t,
    pub buffer: *mut uint8_t,
    pub rowbytes: size_t,
    pub width: int32_t,
    pub height: int32_t,
    pub u: C2RustUnnamed,
    pub recalc: int32_t,
    pub direct: *mut uint8_t,
    pub dupx: int32_t,
    pub dupy: int32_t,
    pub fdupx: int32_t,
    pub fdupy: int32_t,
    pub bpp: int32_t,
    pub baseratio: int32_t,
    pub WndParent: *mut libc::c_void,
    pub smalldupx: uint8_t,
    pub smalldupy: uint8_t,
    pub meddupx: uint8_t,
    pub meddupy: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub numpages: int32_t,
    pub windowed: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texture_t {
    pub name: [libc::c_char; 8],
    pub hash: uint32_t,
    pub type_0: uint8_t,
    pub width: int16_t,
    pub height: int16_t,
    pub holes: boolean,
    pub flip: uint8_t,
    pub flat: *mut libc::c_void,
    pub patchcount: int16_t,
    pub patches: [texpatch_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texpatch_t {
    pub originx: int16_t,
    pub originy: int16_t,
    pub wad: uint16_t,
    pub lump: uint16_t,
    pub flip: uint8_t,
    pub alpha: uint8_t,
    pub style: patchalphastyle,
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
unsafe extern "C" fn FixedDiv2(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return (a as int64_t * ((1 as libc::c_int) << 16 as libc::c_int) as int64_t
        / b as int64_t) as fixed_t;
}
#[no_mangle]
pub static mut skyflatnum: int32_t = 0;
#[no_mangle]
pub static mut skytexture: int32_t = 0;
#[no_mangle]
pub static mut skytexturemid: int32_t = 0;
#[no_mangle]
pub static mut skyscale: fixed_t = 0;
#[no_mangle]
pub static mut levelskynum: int32_t = 0;
#[no_mangle]
pub static mut globallevelskynum: int32_t = 0;
#[no_mangle]
pub unsafe extern "C" fn R_SetupSkyDraw() {
    skytexturemid = ((**textures.offset(skytexture as isize)).height as libc::c_int
        / 2 as libc::c_int) << 16 as libc::c_int;
    R_SetSkyScale();
}
#[no_mangle]
pub unsafe extern "C" fn R_SetSkyScale() {
    let mut difference: fixed_t = vid.fdupx - (vid.dupx << 16 as libc::c_int);
    skyscale = FixedDiv(fovtan, vid.fdupx + difference);
}
