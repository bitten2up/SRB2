use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn V_DrawStretchyFixedPatch(
        x: fixed_t,
        y: fixed_t,
        pscale: fixed_t,
        vscale: fixed_t,
        scrn: int32_t,
        patch: *mut patch_t,
        colormap: *const uint8_t,
    );
    fn V_DrawCroppedPatch(
        x: fixed_t,
        y: fixed_t,
        pscale: fixed_t,
        vscale: fixed_t,
        scrn: int32_t,
        patch: *mut patch_t,
        colormap: *const uint8_t,
        sx: fixed_t,
        sy: fixed_t,
        w: fixed_t,
        h: fixed_t,
    );
    fn V_DrawFill(x: int32_t, y: int32_t, w: int32_t, h: int32_t, c: int32_t);
    fn V_DrawFadeScreen(color: uint16_t, strength: uint8_t);
    fn V_DrawLevelTitle(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
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
    fn V_DrawSmallString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredSmallString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedSmallString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawSmallThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredSmallThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedSmallThinString(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawSmallStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredSmallStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedSmallStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawSmallThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawCenteredSmallThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawRightAlignedSmallThinStringAtFixed(
        x: fixed_t,
        y: fixed_t,
        option: int32_t,
        string: *const libc::c_char,
    );
    fn V_DrawTallNum(x: int32_t, y: int32_t, flags: int32_t, num: int32_t);
    fn V_DrawPaddedTallNum(
        x: int32_t,
        y: int32_t,
        flags: int32_t,
        num: int32_t,
        digits: int32_t,
    );
    fn V_DrawNameTag(
        x: int32_t,
        y: int32_t,
        option: int32_t,
        scale: fixed_t,
        basecolormap: *mut uint8_t,
        outlinecolormap: *mut uint8_t,
        string: *const libc::c_char,
    );
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
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type fixed_t = int32_t;
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
#[repr(C)]
pub struct huddrawlist_s {
    pub items: *mut drawitem_t,
    pub items_capacity: size_t,
    pub items_len: size_t,
    pub strbuf: *mut libc::c_char,
    pub strbuf_capacity: size_t,
    pub strbuf_len: size_t,
}
pub type drawitem_t = drawitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawitem_s {
    pub type_0: drawitem_e,
    pub x: fixed_t,
    pub y: fixed_t,
    pub w: fixed_t,
    pub h: fixed_t,
    pub c: int32_t,
    pub scale: fixed_t,
    pub hscale: fixed_t,
    pub vscale: fixed_t,
    pub patch: *mut patch_t,
    pub flags: int32_t,
    pub basecolor: uint16_t,
    pub outlinecolor: uint16_t,
    pub colormap: *mut uint8_t,
    pub basecolormap: *mut uint8_t,
    pub outlinecolormap: *mut uint8_t,
    pub sx: fixed_t,
    pub sy: fixed_t,
    pub num: int32_t,
    pub digits: int32_t,
    pub str_0: *const libc::c_char,
    pub color: uint16_t,
    pub strength: uint8_t,
    pub align: int32_t,
}
pub type drawitem_e = libc::c_uint;
pub const DI_MAX: drawitem_e = 12;
pub const DI_FadeScreen: drawitem_e = 11;
pub const DI_DrawLevelTitle: drawitem_e = 10;
pub const DI_DrawScaledNameTag: drawitem_e = 9;
pub const DI_DrawNameTag: drawitem_e = 8;
pub const DI_DrawString: drawitem_e = 7;
pub const DI_DrawFill: drawitem_e = 6;
pub const DI_DrawPaddedNum: drawitem_e = 5;
pub const DI_DrawNum: drawitem_e = 4;
pub const DI_DrawCropped: drawitem_e = 3;
pub const DI_DrawStretched: drawitem_e = 2;
pub const DI_DrawScaled: drawitem_e = 1;
pub const DI_Draw: drawitem_e = 0;
pub type huddrawlist_h = *mut huddrawlist_s;
pub const PU_STATIC: C2RustUnnamed_0 = 1;
pub const align_thinfixedright: align = 21;
pub const align_thinfixedcenter: align = 20;
pub const align_thinfixed: align = 19;
pub const align_thinright: align = 23;
pub const align_thincenter: align = 22;
pub const align_thin: align = 18;
pub const align_smallthinfixedright: align = 17;
pub const align_smallthinfixedcenter: align = 16;
pub const align_smallthinfixed: align = 15;
pub const align_smallthinright: align = 14;
pub const align_smallthincenter: align = 13;
pub const align_smallthin: align = 12;
pub const align_smallright: align = 11;
pub const align_smallcenter: align = 10;
pub const align_smallfixedright: align = 9;
pub const align_smallfixedcenter: align = 8;
pub const align_smallfixed: align = 7;
pub const align_small: align = 6;
pub const align_fixedright: align = 5;
pub const align_fixedcenter: align = 4;
pub const align_fixed: align = 3;
pub const align_right: align = 2;
pub const align_center: align = 1;
pub const align_left: align = 0;
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
pub const PU_PATCH_DATA: C2RustUnnamed_0 = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed_0 = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed_0 = 15;
pub const PU_PATCH: C2RustUnnamed_0 = 14;
pub const PU_MUSIC: C2RustUnnamed_0 = 12;
pub const PU_SOUND: C2RustUnnamed_0 = 11;
pub const PU_PERFSTATS: C2RustUnnamed_0 = 3;
pub const PU_LUA: C2RustUnnamed_0 = 2;
pub type align = libc::c_uint;
#[inline(always)]
unsafe extern "C" fn FixedMul(mut a: fixed_t, mut b: fixed_t) -> fixed_t {
    return ((a as int64_t * b as int64_t) as uint64_t >> 16 as libc::c_int) as fixed_t;
}
#[inline(always)]
unsafe extern "C" fn FixedInt(mut a: fixed_t) -> fixed_t {
    return FixedMul(a, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_CreateDrawList() -> huddrawlist_h {
    let mut drawlist: huddrawlist_h = 0 as *mut huddrawlist_s;
    drawlist = Z_CallocAlign(
        ::core::mem::size_of::<huddrawlist_s>() as libc::c_ulong,
        PU_STATIC as libc::c_int,
        0 as *mut libc::c_void,
        ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
    ) as huddrawlist_h;
    (*drawlist).items = 0 as *mut drawitem_t;
    (*drawlist).items_capacity = 0 as libc::c_int as size_t;
    (*drawlist).items_len = 0 as libc::c_int as size_t;
    (*drawlist).strbuf = 0 as *mut libc::c_char;
    (*drawlist).strbuf_capacity = 0 as libc::c_int as size_t;
    (*drawlist).strbuf_len = 0 as libc::c_int as size_t;
    return drawlist;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_ClearDrawList(mut list: huddrawlist_h) {
    if !((*list).items).is_null() {
        memset(
            (*list).items as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<drawitem_t>() as libc::c_ulong)
                .wrapping_mul((*list).items_capacity),
        );
    }
    (*list).items_len = 0 as libc::c_int as size_t;
    if !((*list).strbuf).is_null() {
        *((*list).strbuf)
            .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    (*list).strbuf_len = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_DestroyDrawList(mut list: huddrawlist_h) {
    if list.is_null() {
        return;
    }
    if !((*list).items).is_null() {
        Z_Free((*list).items as *mut libc::c_void);
    }
    Z_Free(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_IsDrawListValid(mut list: huddrawlist_h) -> boolean {
    if list.is_null() {
        return false_0 as libc::c_int;
    }
    return true_0 as libc::c_int;
}
unsafe extern "C" fn AllocateDrawItem(mut list: huddrawlist_h) -> size_t {
    if list.is_null() {
        I_Error(
            b"can't allocate draw item: invalid list\0" as *const u8
                as *const libc::c_char,
        );
    }
    if (*list).items_capacity
        <= ((*list).items_len).wrapping_add(1 as libc::c_int as size_t)
    {
        if (*list).items_capacity == 0 as libc::c_int as size_t {
            (*list).items_capacity = 128 as libc::c_int as size_t;
        } else {
            (*list).items_capacity = (*list).items_capacity * 2 as libc::c_int as size_t;
        }
        (*list)
            .items = Z_ReallocAlign(
            (*list).items as *mut libc::c_void,
            (::core::mem::size_of::<drawitem_s>() as libc::c_ulong)
                .wrapping_mul((*list).items_capacity),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut drawitem_t;
    }
    let fresh0 = (*list).items_len;
    (*list).items_len = ((*list).items_len).wrapping_add(1);
    return fresh0;
}
unsafe extern "C" fn CopyString(
    mut list: huddrawlist_h,
    mut str: *const libc::c_char,
) -> *const libc::c_char {
    let mut lenstr: size_t = 0;
    if list.is_null() {
        I_Error(
            b"can't allocate string; invalid list\0" as *const u8 as *const libc::c_char,
        );
    }
    lenstr = strlen(str);
    if (*list).strbuf_capacity
        <= ((*list).strbuf_len)
            .wrapping_add(lenstr)
            .wrapping_add(1 as libc::c_int as size_t)
    {
        let mut old_offset: *const libc::c_char = (*list).strbuf;
        let mut i: size_t = 0;
        if (*list).strbuf_capacity == 0 as libc::c_int as size_t {
            (*list).strbuf_capacity = 256 as libc::c_int as size_t;
        } else {
            (*list)
                .strbuf_capacity = (*list).strbuf_capacity * 2 as libc::c_int as size_t;
        }
        (*list)
            .strbuf = Z_ReallocAlign(
            (*list).strbuf as *mut libc::c_void,
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*list).strbuf_capacity),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < (*list).items_len {
            let ref mut fresh1 = (*((*list).items).offset(i as isize)).str_0;
            *fresh1 = (*fresh1)
                .offset(
                    ((*list).strbuf).offset_from(old_offset) as libc::c_long as isize,
                );
            i = i.wrapping_add(1);
            i;
        }
    }
    let mut result: *const libc::c_char = &mut *((*list).strbuf)
        .offset((*list).strbuf_len as isize) as *mut libc::c_char as *const libc::c_char;
    strncpy(
        &mut *((*list).strbuf).offset((*list).strbuf_len as isize),
        str,
        lenstr.wrapping_add(1 as libc::c_int as size_t),
    );
    (*list)
        .strbuf_len = ((*list).strbuf_len)
        .wrapping_add(lenstr.wrapping_add(1 as libc::c_int as size_t));
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDraw(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut patch: *mut patch_t,
    mut flags: int32_t,
    mut colormap: *mut uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_Draw;
    (*item).x = x;
    (*item).y = y;
    (*item).patch = patch;
    (*item).flags = flags;
    (*item).colormap = colormap;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawScaled(
    mut list: huddrawlist_h,
    mut x: fixed_t,
    mut y: fixed_t,
    mut scale: fixed_t,
    mut patch: *mut patch_t,
    mut flags: int32_t,
    mut colormap: *mut uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawScaled;
    (*item).x = x;
    (*item).y = y;
    (*item).scale = scale;
    (*item).patch = patch;
    (*item).flags = flags;
    (*item).colormap = colormap;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawStretched(
    mut list: huddrawlist_h,
    mut x: fixed_t,
    mut y: fixed_t,
    mut hscale: fixed_t,
    mut vscale: fixed_t,
    mut patch: *mut patch_t,
    mut flags: int32_t,
    mut colormap: *mut uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawStretched;
    (*item).x = x;
    (*item).y = y;
    (*item).hscale = hscale;
    (*item).vscale = vscale;
    (*item).patch = patch;
    (*item).flags = flags;
    (*item).colormap = colormap;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawCropped(
    mut list: huddrawlist_h,
    mut x: fixed_t,
    mut y: fixed_t,
    mut hscale: fixed_t,
    mut vscale: fixed_t,
    mut patch: *mut patch_t,
    mut flags: int32_t,
    mut colormap: *mut uint8_t,
    mut sx: fixed_t,
    mut sy: fixed_t,
    mut w: fixed_t,
    mut h: fixed_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawCropped;
    (*item).x = x;
    (*item).y = y;
    (*item).hscale = hscale;
    (*item).vscale = vscale;
    (*item).patch = patch;
    (*item).flags = flags;
    (*item).colormap = colormap;
    (*item).sx = sx;
    (*item).sy = sy;
    (*item).w = w;
    (*item).h = h;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawNum(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut num: int32_t,
    mut flags: int32_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawNum;
    (*item).x = x;
    (*item).y = y;
    (*item).num = num;
    (*item).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawPaddedNum(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut num: int32_t,
    mut digits: int32_t,
    mut flags: int32_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawPaddedNum;
    (*item).x = x;
    (*item).y = y;
    (*item).num = num;
    (*item).digits = digits;
    (*item).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawFill(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut w: int32_t,
    mut h: int32_t,
    mut c: int32_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawFill;
    (*item).x = x;
    (*item).y = y;
    (*item).w = w;
    (*item).h = h;
    (*item).c = c;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawString(
    mut list: huddrawlist_h,
    mut x: fixed_t,
    mut y: fixed_t,
    mut str: *const libc::c_char,
    mut flags: int32_t,
    mut align: int32_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawString;
    (*item).x = x;
    (*item).y = y;
    (*item).str_0 = CopyString(list, str);
    (*item).flags = flags;
    (*item).align = align;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawNameTag(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut str: *const libc::c_char,
    mut flags: int32_t,
    mut basecolor: uint16_t,
    mut outlinecolor: uint16_t,
    mut basecolormap: *mut uint8_t,
    mut outlinecolormap: *mut uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawNameTag;
    (*item).x = x;
    (*item).y = y;
    (*item).str_0 = CopyString(list, str);
    (*item).flags = flags;
    (*item).basecolor = basecolor;
    (*item).outlinecolor = outlinecolor;
    (*item).basecolormap = basecolormap;
    (*item).outlinecolormap = outlinecolormap;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawScaledNameTag(
    mut list: huddrawlist_h,
    mut x: fixed_t,
    mut y: fixed_t,
    mut str: *const libc::c_char,
    mut flags: int32_t,
    mut scale: fixed_t,
    mut basecolor: uint16_t,
    mut outlinecolor: uint16_t,
    mut basecolormap: *mut uint8_t,
    mut outlinecolormap: *mut uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawScaledNameTag;
    (*item).x = x;
    (*item).y = y;
    (*item).str_0 = CopyString(list, str);
    (*item).flags = flags;
    (*item).scale = scale;
    (*item).basecolor = basecolor;
    (*item).outlinecolor = outlinecolor;
    (*item).basecolormap = basecolormap;
    (*item).outlinecolormap = outlinecolormap;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddDrawLevelTitle(
    mut list: huddrawlist_h,
    mut x: int32_t,
    mut y: int32_t,
    mut str: *const libc::c_char,
    mut flags: int32_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_DrawLevelTitle;
    (*item).x = x;
    (*item).y = y;
    (*item).str_0 = CopyString(list, str);
    (*item).flags = flags;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_AddFadeScreen(
    mut list: huddrawlist_h,
    mut color: uint16_t,
    mut strength: uint8_t,
) {
    let mut i: size_t = AllocateDrawItem(list);
    let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
        as *mut drawitem_t;
    (*item).type_0 = DI_FadeScreen;
    (*item).color = color;
    (*item).strength = strength;
}
#[no_mangle]
pub unsafe extern "C" fn LUA_HUD_DrawList(mut list: huddrawlist_h) {
    let mut i: size_t = 0;
    if list.is_null() {
        I_Error(b"HUD drawlist invalid\0" as *const u8 as *const libc::c_char);
    }
    if (*list).items_len <= 0 as libc::c_int as size_t {
        return;
    }
    if ((*list).items).is_null() {
        I_Error(b"HUD drawlist->items invalid\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*list).items_len {
        let mut item: *mut drawitem_t = &mut *((*list).items).offset(i as isize)
            as *mut drawitem_t;
        match (*item).type_0 as libc::c_uint {
            0 => {
                V_DrawStretchyFixedPatch(
                    (*item).x << 16 as libc::c_int,
                    (*item).y << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (*item).flags,
                    (*item).patch,
                    (*item).colormap,
                );
            }
            1 => {
                V_DrawStretchyFixedPatch(
                    (*item).x,
                    (*item).y,
                    (*item).scale,
                    (*item).scale,
                    (*item).flags,
                    (*item).patch,
                    (*item).colormap,
                );
            }
            2 => {
                V_DrawStretchyFixedPatch(
                    (*item).x,
                    (*item).y,
                    (*item).hscale,
                    (*item).vscale,
                    (*item).flags,
                    (*item).patch,
                    (*item).colormap,
                );
            }
            3 => {
                V_DrawCroppedPatch(
                    (*item).x,
                    (*item).y,
                    (*item).hscale,
                    (*item).vscale,
                    (*item).flags,
                    (*item).patch,
                    (*item).colormap,
                    (*item).sx,
                    (*item).sy,
                    (*item).w,
                    (*item).h,
                );
            }
            4 => {
                V_DrawTallNum((*item).x, (*item).y, (*item).flags, (*item).num);
            }
            5 => {
                V_DrawPaddedTallNum(
                    (*item).x,
                    (*item).y,
                    (*item).flags,
                    (*item).num,
                    (*item).digits,
                );
            }
            6 => {
                V_DrawFill((*item).x, (*item).y, (*item).w, (*item).h, (*item).c);
            }
            7 => {
                match (*item).align {
                    0 => {
                        V_DrawString((*item).x, (*item).y, (*item).flags, (*item).str_0);
                    }
                    1 => {
                        V_DrawCenteredString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    2 => {
                        V_DrawRightAlignedString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    3 => {
                        V_DrawStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    4 => {
                        V_DrawCenteredStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    5 => {
                        V_DrawRightAlignedStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    6 => {
                        V_DrawSmallString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    7 => {
                        V_DrawSmallStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    8 => {
                        V_DrawCenteredSmallStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    9 => {
                        V_DrawRightAlignedSmallStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    10 => {
                        V_DrawCenteredSmallString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    11 => {
                        V_DrawRightAlignedSmallString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    12 => {
                        V_DrawSmallThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    13 => {
                        V_DrawCenteredSmallThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    14 => {
                        V_DrawRightAlignedSmallThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    15 => {
                        V_DrawSmallThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    16 => {
                        V_DrawCenteredSmallThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    17 => {
                        V_DrawRightAlignedSmallThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    18 => {
                        V_DrawThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    22 => {
                        V_DrawCenteredThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    23 => {
                        V_DrawRightAlignedThinString(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    19 => {
                        V_DrawThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    20 => {
                        V_DrawCenteredThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    21 => {
                        V_DrawRightAlignedThinStringAtFixed(
                            (*item).x,
                            (*item).y,
                            (*item).flags,
                            (*item).str_0,
                        );
                    }
                    _ => {}
                }
            }
            8 => {
                V_DrawNameTag(
                    (*item).x,
                    (*item).y,
                    (*item).flags,
                    (1 as libc::c_int) << 16 as libc::c_int,
                    (*item).basecolormap,
                    (*item).outlinecolormap,
                    (*item).str_0,
                );
            }
            9 => {
                V_DrawNameTag(
                    FixedInt((*item).x),
                    FixedInt((*item).y),
                    (*item).flags,
                    (*item).scale,
                    (*item).basecolormap,
                    (*item).outlinecolormap,
                    (*item).str_0,
                );
            }
            10 => {
                V_DrawLevelTitle((*item).x, (*item).y, (*item).flags, (*item).str_0);
            }
            11 => {
                V_DrawFadeScreen((*item).color, (*item).strength);
            }
            _ => {
                I_Error(
                    b"can't draw draw list item: invalid draw list item type\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        i = i.wrapping_add(1);
        i;
    }
}
