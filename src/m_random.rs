use ::libc;
extern "C" {
    fn I_GetRandomBytes(destination: *mut libc::c_char, count: size_t) -> size_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type fixed_t = int32_t;
pub type rnstate_t = rnstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rnstate_s {
    pub data: [uint32_t; 3],
    pub counter: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub words: [uint32_t; 3],
    pub bytes: [libc::c_char; 12],
}
#[inline]
unsafe extern "C" fn RandomState_Get32(mut state: *mut rnstate_t) -> uint32_t {
    let mut result: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    b = (*state).data[1 as libc::c_int as usize];
    c = (*state).data[2 as libc::c_int as usize];
    let fresh0 = (*state).counter;
    (*state).counter = ((*state).counter).wrapping_add(1);
    result = ((*state).data[0 as libc::c_int as usize])
        .wrapping_add(b)
        .wrapping_add(fresh0);
    (*state).data[0 as libc::c_int as usize] = b ^ b >> 9 as libc::c_int;
    (*state).data[1 as libc::c_int as usize] = c * 9 as libc::c_int as uint32_t;
    (*state)
        .data[2 as libc::c_int
        as usize] = (c << 21 as libc::c_int | c >> 11 as libc::c_int)
        .wrapping_add(result);
    return result;
}
#[inline]
unsafe extern "C" fn RandomState_Seed(
    mut state: *mut rnstate_t,
    mut seeds: *mut uint32_t,
    mut seed_count: size_t,
) {
    let mut i: size_t = 0;
    (*state).counter = 1 as libc::c_int as uint32_t;
    i = 0 as libc::c_int as size_t;
    while i < 3 as libc::c_int as size_t {
        let mut seed_word: uint32_t = 0;
        if i < seed_count {
            seed_word = *seeds.offset(i as isize);
        } else {
            seed_word = 0 as libc::c_int as uint32_t;
        }
        (*state).data[(2 as libc::c_int as size_t).wrapping_sub(i) as usize] = seed_word;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < 16 as libc::c_int as size_t {
        RandomState_Get32(state);
        i = i.wrapping_add(1);
        i;
    }
}
#[inline]
unsafe extern "C" fn RandomState_GetKey32(
    mut state: *mut rnstate_t,
    limit: uint32_t,
) -> uint32_t {
    let mut raw_random: uint32_t = 0;
    let mut scaled_lower_word: uint32_t = 0;
    let mut scaled_random: uint64_t = 0;
    if limit == 0 as libc::c_int as uint32_t {
        return 0 as libc::c_int as uint32_t;
    }
    raw_random = RandomState_Get32(state);
    scaled_random = raw_random as uint64_t * limit as uint64_t;
    scaled_lower_word = scaled_random as uint32_t;
    if scaled_lower_word < limit {
        let mut scaled_limit: uint32_t = 0;
        scaled_limit = limit.wrapping_neg() % limit;
        while scaled_lower_word < scaled_limit {
            raw_random = RandomState_Get32(state);
            scaled_random = raw_random as uint64_t * limit as uint64_t;
            scaled_lower_word = scaled_random as uint32_t;
        }
    }
    return (scaled_random >> 32 as libc::c_int) as uint32_t;
}
static mut m_randomstate: rnstate_t = {
    let mut init = rnstate_s {
        data: [
            0x4a3b6035 as libc::c_uint,
            0x99555606 as libc::c_uint,
            0x6f603421 as libc::c_uint,
        ],
        counter: 16 as libc::c_int as uint32_t,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn M_RandomFixed() -> fixed_t {
    return (RandomState_Get32(&mut m_randomstate)
        >> 32 as libc::c_int - 16 as libc::c_int) as fixed_t;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomByte() -> uint8_t {
    return (RandomState_Get32(&mut m_randomstate) >> 24 as libc::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomKey(mut a: int32_t) -> int32_t {
    let mut range_is_negative: boolean = 0;
    let mut range: int64_t = 0;
    let mut random_result: int32_t = 0;
    range = a as int64_t;
    range_is_negative = (range < 0 as libc::c_int as int64_t) as libc::c_int;
    if range_is_negative != 0 {
        range = -range;
    }
    random_result = RandomState_GetKey32(&mut m_randomstate, range as uint32_t)
        as int32_t;
    if range_is_negative != 0 {
        random_result = -random_result;
    }
    return random_result;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomRange(mut a: int32_t, mut b: int32_t) -> int32_t {
    if b < a {
        let mut temp: int32_t = 0;
        temp = a;
        a = b;
        b = temp;
    }
    let spread: uint32_t = (b - a + 1 as libc::c_int) as uint32_t;
    return (RandomState_GetKey32(&mut m_randomstate, spread) as int64_t + a as int64_t)
        as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomSeedFromOS() -> boolean {
    let mut complete_word_count: uint32_t = 0;
    let mut seed_data: C2RustUnnamed_0 = C2RustUnnamed_0 { words: [0; 3] };
    complete_word_count = (I_GetRandomBytes(
        &mut seed_data.bytes as *mut [libc::c_char; 12] as *mut libc::c_char,
        ::core::mem::size_of::<C2RustUnnamed_0>() as libc::c_ulong,
    ))
        .wrapping_div(::core::mem::size_of::<uint32_t>() as libc::c_ulong) as uint32_t;
    if complete_word_count == 0 as libc::c_int as uint32_t {
        return false_0 as libc::c_int;
    }
    RandomState_Seed(
        &mut m_randomstate,
        &mut seed_data.words as *mut [uint32_t; 3] as *mut uint32_t,
        complete_word_count as size_t,
    );
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomSeed(mut seed: uint32_t) {
    RandomState_Seed(&mut m_randomstate, &mut seed, 1 as libc::c_int as size_t);
}
static mut randomseed: uint32_t = 0xbade4404 as libc::c_uint;
static mut initialseed: uint32_t = 0xbade4404 as libc::c_uint;
#[inline(always)]
unsafe extern "C" fn __internal_prng__() -> fixed_t {
    randomseed ^= randomseed >> 13 as libc::c_int;
    randomseed ^= randomseed >> 11 as libc::c_int;
    randomseed ^= randomseed << 21 as libc::c_int;
    return (randomseed * 36548569 as libc::c_int as uint32_t >> 4 as libc::c_int
        & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int) as uint32_t)
        as fixed_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_RandomFixed() -> fixed_t {
    return __internal_prng__();
}
#[no_mangle]
pub unsafe extern "C" fn P_RandomByte() -> uint8_t {
    return ((__internal_prng__() & 0xff00 as libc::c_int) >> 8 as libc::c_int)
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_RandomKey(mut a: int32_t) -> int32_t {
    return (__internal_prng__() as int64_t * a as int64_t >> 16 as libc::c_int)
        as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn P_RandomRange(mut a: int32_t, mut b: int32_t) -> int32_t {
    return (__internal_prng__() as int64_t * (b - a + 1 as libc::c_int) as int64_t
        >> 16 as libc::c_int) as int32_t + a;
}
#[no_mangle]
pub unsafe extern "C" fn P_RandomPeek() -> fixed_t {
    let mut r: uint32_t = randomseed;
    let mut ret: fixed_t = __internal_prng__();
    randomseed = r;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn P_GetRandSeed() -> uint32_t {
    return randomseed;
}
#[no_mangle]
pub unsafe extern "C" fn P_GetInitSeed() -> uint32_t {
    return initialseed;
}
#[no_mangle]
pub unsafe extern "C" fn P_SetRandSeed(mut seed: uint32_t) {
    if seed == 0 {
        seed = 0xbade4404 as libc::c_uint;
    }
    initialseed = seed;
    randomseed = initialseed;
}
#[no_mangle]
pub unsafe extern "C" fn M_RandomizedSeed() -> uint32_t {
    let mut seed: uint32_t = 0;
    loop {
        seed = RandomState_Get32(&mut m_randomstate);
        if !(seed == 0 as libc::c_int as uint32_t) {
            break;
        }
    }
    return seed;
}
