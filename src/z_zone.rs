use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn sizeu1(num: size_t) -> *mut libc::c_char;
    fn sizeu2(num: size_t) -> *mut libc::c_char;
    fn COM_AddCommand(name: *const libc::c_char, func: com_func_t, flags: com_flags_t);
    fn LUA_InvalidateUserdata(data: *mut libc::c_void);
    fn I_GetFreeMem(total: *mut size_t) -> size_t;
}
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
pub type com_func_t = Option::<unsafe extern "C" fn() -> ()>;
pub type C2RustUnnamed = libc::c_uint;
pub const PU_HWRMODELTEXTURE_UNLOCKED: C2RustUnnamed = 103;
pub const PU_HWRCACHE_UNLOCKED: C2RustUnnamed = 102;
pub const PU_CACHE_UNLOCKED: C2RustUnnamed = 101;
pub const PU_PURGELEVEL: C2RustUnnamed = 100;
pub const PU_HWRPLANE: C2RustUnnamed = 52;
pub const PU_LEVSPEC: C2RustUnnamed = 51;
pub const PU_LEVEL: C2RustUnnamed = 50;
pub const PU_CACHE: C2RustUnnamed = 49;
pub const PU_HWRCACHE: C2RustUnnamed = 48;
pub const PU_HWRMODELTEXTURE: C2RustUnnamed = 23;
pub const PU_HWRPATCHCOLMIPMAP: C2RustUnnamed = 22;
pub const PU_HWRPATCHINFO: C2RustUnnamed = 21;
pub const PU_HUDGFX: C2RustUnnamed = 19;
pub const PU_SPRITE: C2RustUnnamed = 18;
pub const PU_PATCH_DATA: C2RustUnnamed = 17;
pub const PU_PATCH_ROTATED: C2RustUnnamed = 16;
pub const PU_PATCH_LOWPRIORITY: C2RustUnnamed = 15;
pub const PU_PATCH: C2RustUnnamed = 14;
pub const PU_MUSIC: C2RustUnnamed = 12;
pub const PU_SOUND: C2RustUnnamed = 11;
pub const PU_PERFSTATS: C2RustUnnamed = 3;
pub const PU_LUA: C2RustUnnamed = 2;
pub const PU_STATIC: C2RustUnnamed = 1;
pub type memblock_t = memblock_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memblock_s {
    pub user: *mut *mut libc::c_void,
    pub tag: int32_t,
    pub id: uint32_t,
    pub size: size_t,
    pub realsize: size_t,
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
}
static mut head: memblock_t = memblock_s {
    user: 0 as *const *mut libc::c_void as *mut *mut libc::c_void,
    tag: 0,
    id: 0,
    size: 0,
    realsize: 0,
    next: 0 as *const memblock_s as *mut memblock_s,
    prev: 0 as *const memblock_s as *mut memblock_s,
};
#[no_mangle]
pub unsafe extern "C" fn Z_Init() {
    let mut total: size_t = 0;
    let mut memfree: size_t = 0;
    memset(
        &mut head as *mut memblock_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<memblock_t>() as libc::c_ulong,
    );
    head.prev = &mut head;
    head.next = head.prev;
    memfree = I_GetFreeMem(&mut total) >> 20 as libc::c_int;
    CONS_Printf(
        b"System memory: %sMB - Free: %sMB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(total >> 20 as libc::c_int),
        sizeu2(memfree),
    );
    COM_AddCommand(
        b"memfree\0" as *const u8 as *const libc::c_char,
        Some(Command_Memfree_f as unsafe extern "C" fn() -> ()),
        COM_LUA,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Z_Free(mut ptr: *mut libc::c_void) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    if ptr.is_null() {
        return;
    }
    block = (ptr as uintptr_t)
        .wrapping_sub(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        as *mut memblock_t;
    if (*block).tag != PU_LUA as libc::c_int {
        LUA_InvalidateUserdata(ptr);
    }
    if !((*block).user).is_null() {
        *(*block).user = 0 as *mut libc::c_void;
    }
    (*(*block).prev).next = (*block).next;
    (*(*block).next).prev = (*block).prev;
    free(block as *mut libc::c_void);
}
unsafe extern "C" fn xm(mut size: size_t) -> *mut libc::c_void {
    let padedsize: size_t = size
        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong);
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if padedsize < size {
        I_Error(
            b"You are allocating memory too large!\0" as *const u8 as *const libc::c_char,
        );
    }
    p = malloc(padedsize);
    if p.is_null() {
        Z_FreeTags(PU_PURGELEVEL as libc::c_int, 2147483647 as libc::c_int);
        p = malloc(padedsize);
        if p.is_null() {
            I_Error(
                b"Out of memory allocating %s bytes\0" as *const u8
                    as *const libc::c_char,
                sizeu1(size),
            );
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn Z_MallocAlign(
    mut size: size_t,
    mut tag: int32_t,
    mut user: *mut libc::c_void,
    mut alignbits: int32_t,
) -> *mut libc::c_void {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    block = xm(
        (::core::mem::size_of::<memblock_t>() as libc::c_ulong).wrapping_add(size),
    ) as *mut memblock_t;
    ptr = (block as uintptr_t)
        .wrapping_add(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        as *mut libc::c_void;
    (*block).next = head.next;
    (*block).prev = &mut head;
    head.next = block;
    (*(*block).next).prev = block;
    (*block).tag = tag;
    (*block).user = 0 as *mut *mut libc::c_void;
    (*block)
        .size = (::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        .wrapping_add(size);
    (*block).realsize = size;
    (*block).id = 0xa441d13d as libc::c_uint;
    if !user.is_null() {
        (*block).user = user as *mut *mut libc::c_void;
        let ref mut fresh0 = *(user as *mut *mut libc::c_void);
        *fresh0 = ptr;
    } else if tag >= PU_PURGELEVEL as libc::c_int {
        I_Error(
            b"Z_Malloc: attempted to allocate purgable block (size %s) with no user\0"
                as *const u8 as *const libc::c_char,
            sizeu1(size),
        );
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn Z_CallocAlign(
    mut size: size_t,
    mut tag: int32_t,
    mut user: *mut libc::c_void,
    mut alignbits: int32_t,
) -> *mut libc::c_void {
    return memset(Z_MallocAlign(size, tag, user, alignbits), 0 as libc::c_int, size);
}
#[no_mangle]
pub unsafe extern "C" fn Z_ReallocAlign(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut tag: int32_t,
    mut user: *mut libc::c_void,
    mut alignbits: int32_t,
) -> *mut libc::c_void {
    let mut rez: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut copysize: size_t = 0;
    if size == 0 {
        Z_Free(ptr);
        return 0 as *mut libc::c_void;
    }
    if ptr.is_null() {
        return Z_CallocAlign(size, tag, user, alignbits);
    }
    block = (ptr as uintptr_t)
        .wrapping_sub(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        as *mut memblock_t;
    if block.is_null() {
        return 0 as *mut libc::c_void;
    }
    rez = Z_MallocAlign(size, tag, user, alignbits);
    if size < (*block).realsize {
        copysize = size;
    } else {
        copysize = (*block).realsize;
    }
    M_Memcpy.expect("non-null function pointer")(rez, ptr, copysize);
    Z_Free(ptr);
    if !user.is_null() {
        let ref mut fresh1 = *(user as *mut *mut libc::c_void);
        *fresh1 = rez;
    }
    if size > copysize {
        memset(
            (rez as *mut libc::c_char).offset(copysize as isize) as *mut libc::c_void,
            0 as libc::c_int,
            size.wrapping_sub(copysize),
        );
    }
    return rez;
}
#[no_mangle]
pub unsafe extern "C" fn Z_FreeTags(mut lowtag: int32_t, mut hightag: int32_t) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut next: *mut memblock_t = 0 as *mut memblock_t;
    Z_CheckHeap(420 as libc::c_int);
    block = head.next;
    while block != &mut head as *mut memblock_t {
        next = (*block).next;
        if (*block).tag >= lowtag && (*block).tag <= hightag {
            Z_Free(
                (block as uintptr_t)
                    .wrapping_add(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
                    as *mut libc::c_void,
            );
        }
        block = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Z_IterateTags(
    mut lowtag: int32_t,
    mut hightag: int32_t,
    mut iterfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> boolean>,
) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut next: *mut memblock_t = 0 as *mut memblock_t;
    if iterfunc.is_none() {
        I_Error(
            b"Z_IterateTags: no iterator function was given\0" as *const u8
                as *const libc::c_char,
        );
    }
    block = head.next;
    while block != &mut head as *mut memblock_t {
        next = (*block).next;
        if (*block).tag >= lowtag && (*block).tag <= hightag {
            let mut mem: *mut libc::c_void = (block as uintptr_t)
                .wrapping_add(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
                as *mut libc::c_void;
            let mut free_0: boolean = iterfunc.expect("non-null function pointer")(mem);
            if free_0 != 0 {
                Z_Free(mem);
            }
        }
        block = next;
    }
}
static mut nextcleanup: int32_t = 2000 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn Z_CheckMemCleanup() {
    let fresh2 = nextcleanup;
    nextcleanup = nextcleanup - 1;
    if fresh2 == 0 as libc::c_int {
        nextcleanup = 2000 as libc::c_int;
        Z_FreeTags(PU_PURGELEVEL as libc::c_int, 2147483647 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Z_CheckHeap(mut i: int32_t) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut blocknumon: uint32_t = 0 as libc::c_int as uint32_t;
    let mut given: *mut libc::c_void = 0 as *mut libc::c_void;
    block = head.next;
    while block != &mut head as *mut memblock_t {
        blocknumon = blocknumon.wrapping_add(1);
        blocknumon;
        given = (block as uintptr_t)
            .wrapping_add(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
            as *mut libc::c_void;
        if !((*block).user).is_null() && *(*block).user != given {
            I_Error(
                b"Z_CheckHeap %d: block %u doesn't have a proper user\0" as *const u8
                    as *const libc::c_char,
                i,
                blocknumon,
            );
        }
        if (*(*block).next).prev != block {
            I_Error(
                b"Z_CheckHeap %d: block %u lacks proper backlink\0" as *const u8
                    as *const libc::c_char,
                i,
                blocknumon,
            );
        }
        if (*(*block).prev).next != block {
            I_Error(
                b"Z_CheckHeap %d: block %u lacks proper forward link\0" as *const u8
                    as *const libc::c_char,
                i,
                blocknumon,
            );
        }
        if (*block).id != 0xa441d13d as libc::c_uint {
            I_Error(
                b"Z_CheckHeap %d: block %u have the wrong ID\0" as *const u8
                    as *const libc::c_char,
                i,
                blocknumon,
            );
        }
        block = (*block).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Z_ChangeTag(mut ptr: *mut libc::c_void, mut tag: int32_t) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    if ptr.is_null() {
        return;
    }
    block = (ptr as uintptr_t)
        .wrapping_sub(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        as *mut memblock_t;
    if tag >= PU_PURGELEVEL as libc::c_int && ((*block).user).is_null() {
        I_Error(
            b"Internal memory management error: tried to make block purgable but it has no owner\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*block).tag = tag;
}
#[no_mangle]
pub unsafe extern "C" fn Z_SetUser(
    mut ptr: *mut libc::c_void,
    mut newuser: *mut *mut libc::c_void,
) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    if ptr.is_null() {
        return;
    }
    block = (ptr as uintptr_t)
        .wrapping_sub(::core::mem::size_of::<memblock_t>() as libc::c_ulong)
        as *mut memblock_t;
    if (*block).tag >= PU_PURGELEVEL as libc::c_int && newuser.is_null() {
        I_Error(
            b"Internal memory management error: tried to make block purgable but it has no owner\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*block).user = newuser as *mut libc::c_void as *mut *mut libc::c_void;
    *newuser = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn Z_TagsUsage(
    mut lowtag: int32_t,
    mut hightag: int32_t,
) -> size_t {
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut rover: *mut memblock_t = 0 as *mut memblock_t;
    rover = head.next;
    while rover != &mut head as *mut memblock_t {
        if !((*rover).tag < lowtag || (*rover).tag > hightag) {
            cnt = (cnt as libc::c_ulong)
                .wrapping_add(
                    ((*rover).size)
                        .wrapping_add(
                            ::core::mem::size_of::<memblock_t>() as libc::c_ulong,
                        ),
                ) as size_t as size_t;
        }
        rover = (*rover).next;
    }
    return cnt;
}
unsafe extern "C" fn Command_Memfree_f() {
    let mut freebytes: size_t = 0;
    let mut totalbytes: size_t = 0;
    Z_CheckHeap(-(1 as libc::c_int));
    CONS_Printf(
        b"\x82%s\0" as *const u8 as *const libc::c_char,
        b"Memory Info\n\0" as *const u8 as *const libc::c_char,
    );
    CONS_Printf(
        b"Total heap used        : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(0 as libc::c_int, 2147483647 as libc::c_int) >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Static                 : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_STATIC as libc::c_int, PU_STATIC as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Static (sound)         : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_SOUND as libc::c_int, PU_SOUND as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Static (music)         : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_MUSIC as libc::c_int, PU_MUSIC as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Patches                : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_PATCH as libc::c_int, PU_PATCH as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Patches (low priority) : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(
                PU_PATCH_LOWPRIORITY as libc::c_int,
                PU_PATCH_LOWPRIORITY as libc::c_int,
            ) >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Patches (rotated)      : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_PATCH_ROTATED as libc::c_int, PU_PATCH_ROTATED as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Sprites                : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_SPRITE as libc::c_int, PU_SPRITE as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"HUD graphics           : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_HUDGFX as libc::c_int, PU_HUDGFX as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Locked cache           : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_CACHE as libc::c_int, PU_CACHE as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Level                  : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_LEVEL as libc::c_int, PU_LEVEL as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"Special thinker        : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_LEVSPEC as libc::c_int, PU_LEVSPEC as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"All purgable           : %7s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(
            Z_TagsUsage(PU_PURGELEVEL as libc::c_int, 2147483647 as libc::c_int)
                >> 10 as libc::c_int,
        ),
    );
    CONS_Printf(
        b"\x82%s\0" as *const u8 as *const libc::c_char,
        b"System Memory Info\n\0" as *const u8 as *const libc::c_char,
    );
    freebytes = I_GetFreeMem(&mut totalbytes);
    CONS_Printf(
        b"    Total physical memory: %s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(totalbytes >> 10 as libc::c_int),
    );
    CONS_Printf(
        b"Available physical memory: %s KB\n\0" as *const u8 as *const libc::c_char,
        sizeu1(freebytes >> 10 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Z_StrDup(mut s: *const libc::c_char) -> *mut libc::c_char {
    return strcpy(
        Z_MallocAlign(
            (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            PU_STATIC as libc::c_int,
            0 as *mut libc::c_void,
            ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as int32_t,
        ) as *mut libc::c_char,
        s,
    );
}
