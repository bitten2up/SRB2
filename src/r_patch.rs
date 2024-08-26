use ::libc;
extern "C" {
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn Picture_Convert(
        informat: pictureformat_t,
        picture: *mut libc::c_void,
        outformat: pictureformat_t,
        insize: size_t,
        outsize: *mut size_t,
        inwidth: int32_t,
        inheight: int32_t,
        inleftoffset: int32_t,
        intopoffset: int32_t,
        flags: pictureflags_t,
    ) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_CallocAlign(
        size: size_t,
        tag: int32_t,
        user: *mut libc::c_void,
        alignbits: int32_t,
    ) -> *mut libc::c_void;
    fn Z_IterateTags(
        lowtag: int32_t,
        hightag: int32_t,
        iterfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> boolean>,
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int32_t = libc::c_int;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rotsprite_t {
    pub angles: int32_t,
    pub patches: *mut *mut libc::c_void,
}
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
#[repr(C, packed)]
pub struct softwarepatch_t {
    pub width: int16_t,
    pub height: int16_t,
    pub leftoffset: int16_t,
    pub topoffset: int16_t,
    pub columnofs: [int32_t; 8],
}
pub type pictureformat_t = libc::c_uint;
pub const PICFMT_DOOMPATCH32: pictureformat_t = 10;
pub const PICFMT_FLAT32: pictureformat_t = 9;
pub const PICFMT_PATCH32: pictureformat_t = 8;
pub const PICFMT_DOOMPATCH16: pictureformat_t = 7;
pub const PICFMT_FLAT16: pictureformat_t = 6;
pub const PICFMT_PATCH16: pictureformat_t = 5;
pub const PICFMT_PNG: pictureformat_t = 4;
pub const PICFMT_DOOMPATCH: pictureformat_t = 3;
pub const PICFMT_FLAT: pictureformat_t = 2;
pub const PICFMT_PATCH: pictureformat_t = 1;
pub const PICFMT_NONE: pictureformat_t = 0;
pub type pictureflags_t = libc::c_uint;
pub const PICFLAGS_YFLIP: pictureflags_t = 2;
pub const PICFLAGS_XFLIP: pictureflags_t = 1;
pub const PU_PATCH: C2RustUnnamed_0 = 14;
pub const PU_PATCH_DATA: C2RustUnnamed_0 = 17;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed_0 = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed_0 = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed_0 = 101;
pub const PU_PURGELEVEL: C2RustUnnamed_0 = 100;
pub const PU_HWRPLANE: C2RustUnnamed_0 = 52;
pub const PU_LEVSPEC: C2RustUnnamed_0 = 51;
pub const PU_LEVEL: C2RustUnnamed_0 = 50;
pub const PU_CACHE: C2RustUnnamed_0 = 49;
pub const PU_HWRCACHE: C2RustUnnamed_0 = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed_0 = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed_0 = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed_0 = 21;
pub const PU_HUDGFX: C2RustUnnamed_0 = 19;
pub const PU_SPRITE: C2RustUnnamed_0 = 18;
pub const PU_PATCH_ROTATED: C2RustUnnamed_0 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_0 = 15;
pub const PU_MUSIC: C2RustUnnamed_0 = 12;
pub const PU_SOUND: C2RustUnnamed_0 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_0 = 3;
pub const PU_LUA: C2RustUnnamed_0 = 2;
pub const PU_STATIC: C2RustUnnamed_0 = 1;
#[no_mangle]
pub unsafe extern "C" fn Patch_Create(
    mut source: *mut softwarepatch_t,
    mut srcsize: size_t,
    mut dest: *mut libc::c_void,
) -> *mut patch_t {
    let mut patch: *mut patch_t = (if dest.is_null() {
        Z_CallocAlign(
            ::core::mem::size_of::<patch_t>() as libc::c_ulong,
            PU_PATCH as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        )
    } else {
        dest as *mut patch_t as *mut libc::c_void
    }) as *mut patch_t;
    if !source.is_null() {
        let mut col: int32_t = 0;
        let mut colsize: int32_t = 0;
        let mut size: size_t = (::core::mem::size_of::<int32_t>() as libc::c_ulong)
            .wrapping_mul((*source).width as libc::c_ulong);
        let mut offs: size_t = (::core::mem::size_of::<int16_t>() as libc::c_ulong)
            .wrapping_mul(4 as libc::c_int as libc::c_ulong)
            .wrapping_add(size);
        (*patch).width = (*source).width;
        (*patch).height = (*source).height;
        (*patch).leftoffset = (*source).leftoffset;
        (*patch).topoffset = (*source).topoffset;
        (*patch)
            .columnofs = Z_CallocAlign(
            size,
            PU_PATCH_DATA as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut int32_t;
        col = 0 as libc::c_int;
        while col < (*source).width as libc::c_int {
            *((*patch).columnofs)
                .offset(
                    col as isize,
                ) = ((*source).columnofs[col as usize] as size_t).wrapping_sub(offs)
                as int32_t;
            col += 1;
            col;
        }
        if srcsize == 0 {
            I_Error(
                b"Patch_Create: no source size!\0" as *const u8 as *const libc::c_char,
            );
        }
        colsize = srcsize as int32_t - offs as int32_t;
        if colsize <= 0 as libc::c_int {
            I_Error(
                b"Patch_Create: no column data!\0" as *const u8 as *const libc::c_char,
            );
        }
        (*patch)
            .columns = Z_CallocAlign(
            colsize as size_t,
            PU_PATCH_DATA as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut uint8_t;
        M_Memcpy
            .expect(
                "non-null function pointer",
            )(
            (*patch).columns as *mut libc::c_void,
            (source as *mut uint8_t)
                .offset((*source).columnofs[0 as libc::c_int as usize] as isize)
                as *const libc::c_void,
            colsize as size_t,
        );
    }
    return patch;
}
unsafe extern "C" fn Patch_FreeData(mut patch: *mut patch_t) {
    let mut i: int32_t = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if !((*patch).flats[i as usize]).is_null() {
            Z_Free((*patch).flats[i as usize]);
        }
        i += 1;
        i;
    }
    if !((*patch).rotated).is_null() {
        let mut rotsprite: *mut rotsprite_t = (*patch).rotated;
        i = 0 as libc::c_int;
        while i < (*rotsprite).angles {
            if !(*((*rotsprite).patches).offset(i as isize)).is_null() {
                Patch_Free(*((*rotsprite).patches).offset(i as isize) as *mut patch_t);
            }
            i += 1;
            i;
        }
        Z_Free((*rotsprite).patches as *mut libc::c_void);
        Z_Free(rotsprite as *mut libc::c_void);
    }
    if !((*patch).columnofs).is_null() {
        Z_Free((*patch).columnofs as *mut libc::c_void);
    }
    if !((*patch).columns).is_null() {
        Z_Free((*patch).columns as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Patch_Free(mut patch: *mut patch_t) {
    Patch_FreeData(patch);
    Z_Free(patch as *mut libc::c_void);
}
unsafe extern "C" fn Patch_FreeTagsCallback(mut mem: *mut libc::c_void) -> boolean {
    let mut patch: *mut patch_t = mem as *mut patch_t;
    Patch_FreeData(patch);
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Patch_FreeTags(mut lowtag: int32_t, mut hightag: int32_t) {
    Z_IterateTags(
        lowtag,
        hightag,
        Some(
            Patch_FreeTagsCallback as unsafe extern "C" fn(*mut libc::c_void) -> boolean,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Patch_GenerateFlat(
    mut patch: *mut patch_t,
    mut flags: pictureflags_t,
) {
    let mut flip: uint8_t = (flags as libc::c_uint
        & (PICFLAGS_XFLIP as libc::c_int | PICFLAGS_YFLIP as libc::c_int)
            as libc::c_uint) as uint8_t;
    if ((*patch).flats[flip as usize]).is_null() {
        (*patch)
            .flats[flip
            as usize] = Picture_Convert(
            PICFMT_PATCH,
            patch as *mut libc::c_void,
            PICFMT_FLAT16,
            0 as libc::c_int as size_t,
            0 as *mut size_t,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            flags,
        );
    }
}
