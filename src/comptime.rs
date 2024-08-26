use ::libc;
#[no_mangle]
pub static mut compbranch: *const libc::c_char = b"rust\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut comprevision: *const libc::c_char = b"dd54f749\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut compnote: *const libc::c_char = b"Merge-branch-freebsd-memfix-cmake-into-master\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut compuncommitted: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut compdate: *const libc::c_char = b"Aug 25 2024\0" as *const u8
    as *const libc::c_char;
#[no_mangle]
pub static mut comptime: *const libc::c_char = b"20:27:02\0" as *const u8
    as *const libc::c_char;
