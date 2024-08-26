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
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    static mut M_Memcpy: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            size_t,
        ) -> *mut libc::c_void,
    >;
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    fn I_OutputMsg(error: *const libc::c_char, _: ...);
    static in6addr_any: in6_addr;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn getsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *mut libc::c_void,
        __optlen: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn I_AddExitFunc(func: Option::<unsafe extern "C" fn() -> ()>);
    fn COM_BufAddTextEx(btext: *const libc::c_char, flags: com_flags_t);
    static mut hardware_MAXPACKETLENGTH: int16_t;
    static mut net_bandwidth: int32_t;
    static mut doomcom: *mut doomcom_t;
    static mut I_NetGet: Option::<unsafe extern "C" fn() -> boolean>;
    static mut I_NetCanGet: Option::<unsafe extern "C" fn() -> boolean>;
    static mut I_NetSend: Option::<unsafe extern "C" fn() -> ()>;
    static mut I_NetCanSend: Option::<unsafe extern "C" fn() -> boolean>;
    static mut I_NetFreeNodenum: Option::<unsafe extern "C" fn(int32_t) -> ()>;
    static mut I_NetMakeNodewPort: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> int8_t,
    >;
    static mut I_NetOpenSocket: Option::<unsafe extern "C" fn() -> boolean>;
    static mut I_NetCloseSocket: Option::<unsafe extern "C" fn() -> ()>;
    static mut I_Ban: Option::<unsafe extern "C" fn(int32_t) -> boolean>;
    static mut I_ClearBans: Option::<unsafe extern "C" fn() -> ()>;
    static mut I_GetNodeAddress: Option::<
        unsafe extern "C" fn(int32_t) -> *const libc::c_char,
    >;
    static mut I_GetBanAddress: Option::<
        unsafe extern "C" fn(size_t) -> *const libc::c_char,
    >;
    static mut I_GetBanMask: Option::<
        unsafe extern "C" fn(size_t) -> *const libc::c_char,
    >;
    static mut I_SetBanAddress: Option::<
        unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> boolean,
    >;
    static mut bannednode: *mut boolean;
    static mut packetheaderlength: int32_t;
    static mut nodeingame: [boolean; 127];
    static mut serverrunning: boolean;
    static mut server: boolean;
    static mut dedicated: boolean;
    static mut servernode: int8_t;
    fn Playing() -> boolean;
    fn SendingFile(node: int32_t) -> boolean;
    static mut current_port: uint16_t;
    fn M_GetUrlProtocolArg() -> *const libc::c_char;
    fn M_CheckParm(check: *const libc::c_char) -> int32_t;
    fn M_IsNextParm() -> boolean;
    fn M_GetNextParm() -> *const libc::c_char;
    static mut debugfile: *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
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
pub type socklen_t = __socklen_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const true_0: C2RustUnnamed = 1;
pub const false_0: C2RustUnnamed = 0;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_0 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_0 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_0 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mysockaddr_t {
    pub any: sockaddr,
    pub ip4: sockaddr_in,
    pub ip6: sockaddr_in6,
}
pub type com_flags_t = libc::c_uint;
pub const COM_LUA: com_flags_t = 8;
pub const COM_LOCAL: com_flags_t = 4;
pub const COM_SPLITSCREEN: com_flags_t = 2;
pub const COM_ADMIN: com_flags_t = 1;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct doomcom_t {
    pub id: int32_t,
    pub intnum: int16_t,
    pub command: int16_t,
    pub remotenode: int16_t,
    pub datalength: int16_t,
    pub numnodes: int16_t,
    pub ticdup: int16_t,
    pub extratics: int16_t,
    pub gametype: int16_t,
    pub savegame: int16_t,
    pub map: int16_t,
    pub consoleplayer: int16_t,
    pub numslots: int16_t,
    pub data: [libc::c_char; 1450],
}
pub type SOCKET_TYPE = libc::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
static mut mysockets: [SOCKET_TYPE; 128] = [
    -(1 as libc::c_int),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut mysocketses: size_t = 0 as libc::c_int as size_t;
static mut myfamily: [libc::c_int; 128] = [
    0 as libc::c_int,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut nodesocket: [SOCKET_TYPE; 128] = [
    -(1 as libc::c_int),
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut clientaddress: [mysockaddr_t; 128] = [mysockaddr_t {
    any: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    },
}; 128];
static mut broadcastaddress: [mysockaddr_t; 128] = [mysockaddr_t {
    any: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    },
}; 128];
static mut broadcastaddresses: size_t = 0 as libc::c_int as size_t;
static mut nodeconnected: [boolean; 128] = [0; 128];
static mut banned: [mysockaddr_t; 100] = [mysockaddr_t {
    any: sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    },
}; 100];
static mut bannedmask: [uint8_t; 100] = [0; 100];
static mut numbans: size_t = 0 as libc::c_int as size_t;
static mut SOCK_bannednode: [boolean; 128] = [0; 128];
static mut init_tcp_driver: boolean = false_0 as libc::c_int;
static mut serverport_name: *const libc::c_char = b"5029\0" as *const u8
    as *const libc::c_char;
static mut clientport_name: *const libc::c_char = 0 as *const libc::c_char;
unsafe extern "C" fn SOCK_AddrToStr(mut sk: *mut mysockaddr_t) -> *const libc::c_char {
    static mut s: [libc::c_char; 64] = [0; 64];
    let mut v6: libc::c_int = ((*sk).any.sa_family as libc::c_int == 10 as libc::c_int)
        as libc::c_int;
    let mut addr: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*sk).any.sa_family as libc::c_int == 2 as libc::c_int {
        addr = &mut (*sk).ip4.sin_addr as *mut in_addr as *mut libc::c_void;
    } else if (*sk).any.sa_family as libc::c_int == 10 as libc::c_int {
        addr = &mut (*sk).ip6.sin6_addr as *mut in6_addr as *mut libc::c_void;
    } else {
        addr = 0 as *mut libc::c_void;
    }
    if addr.is_null() {
        sprintf(s.as_mut_ptr(), b"No address\0" as *const u8 as *const libc::c_char);
    } else if (inet_ntop(
        (*sk).any.sa_family as libc::c_int,
        addr,
        &mut *s.as_mut_ptr().offset(v6 as isize),
        (::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(v6 as libc::c_ulong) as socklen_t,
    ))
        .is_null()
    {
        sprintf(
            s.as_mut_ptr(),
            b"Unknown family type, error #%u\0" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
    } else if (*sk).any.sa_family as libc::c_int == 10 as libc::c_int {
        s[0 as libc::c_int as usize] = '[' as i32 as libc::c_char;
        strcat(s.as_mut_ptr(), b"]\0" as *const u8 as *const libc::c_char);
        if (*sk).ip6.sin6_port as libc::c_int != 0 as libc::c_int {
            strcat(
                s.as_mut_ptr(),
                va(
                    b":%d\0" as *const u8 as *const libc::c_char,
                    __bswap_16((*sk).ip6.sin6_port) as libc::c_int,
                ),
            );
        }
    } else if (*sk).any.sa_family as libc::c_int == 2 as libc::c_int
        && (*sk).ip4.sin_port as libc::c_int != 0 as libc::c_int
    {
        strcat(
            s.as_mut_ptr(),
            va(
                b":%d\0" as *const u8 as *const libc::c_char,
                __bswap_16((*sk).ip4.sin_port) as libc::c_int,
            ),
        );
    }
    return s.as_mut_ptr();
}
unsafe extern "C" fn SOCK_GetNodeAddress(mut node: int32_t) -> *const libc::c_char {
    if node == 0 as libc::c_int {
        return b"self\0" as *const u8 as *const libc::c_char;
    }
    if nodeconnected[node as usize] == 0 {
        return 0 as *const libc::c_char;
    }
    return SOCK_AddrToStr(&mut *clientaddress.as_mut_ptr().offset(node as isize));
}
unsafe extern "C" fn SOCK_GetBanAddress(mut ban: size_t) -> *const libc::c_char {
    if ban >= numbans {
        return 0 as *const libc::c_char;
    }
    return SOCK_AddrToStr(&mut *banned.as_mut_ptr().offset(ban as isize));
}
unsafe extern "C" fn SOCK_GetBanMask(mut ban: size_t) -> *const libc::c_char {
    static mut s: [libc::c_char; 16] = [0; 16];
    if ban >= numbans {
        return 0 as *const libc::c_char;
    }
    if sprintf(
        s.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        bannedmask[ban as usize] as libc::c_int,
    ) > 0 as libc::c_int
    {
        return s.as_mut_ptr();
    }
    return 0 as *const libc::c_char;
}
unsafe extern "C" fn SOCK_cmpaddr(
    mut a: *mut mysockaddr_t,
    mut b: *mut mysockaddr_t,
    mut mask: uint8_t,
) -> boolean {
    let mut bitmask: uint32_t = 0xffffffff as libc::c_uint;
    if mask as libc::c_int != 0 && (mask as libc::c_int) < 32 as libc::c_int {
        bitmask = __bswap_32(
            (-(1 as libc::c_int) as uint32_t) << 32 as libc::c_int - mask as libc::c_int,
        );
    }
    if (*b).any.sa_family as libc::c_int == 2 as libc::c_int {
        return ((*a).ip4.sin_addr.s_addr & bitmask == (*b).ip4.sin_addr.s_addr & bitmask
            && ((*b).ip4.sin_port as libc::c_int == 0 as libc::c_int
                || (*a).ip4.sin_port as libc::c_int == (*b).ip4.sin_port as libc::c_int))
            as libc::c_int
    } else if (*b).any.sa_family as libc::c_int == 10 as libc::c_int {
        return (memcmp(
            &mut (*a).ip6.sin6_addr as *mut in6_addr as *const libc::c_void,
            &mut (*b).ip6.sin6_addr as *mut in6_addr as *const libc::c_void,
            ::core::mem::size_of::<in6_addr>() as libc::c_ulong,
        ) == 0
            && ((*b).ip6.sin6_port as libc::c_int == 0 as libc::c_int
                || (*a).ip6.sin6_port as libc::c_int
                    == (*b).ip6.sin6_port as libc::c_int)) as libc::c_int
    } else {
        return false_0 as libc::c_int
    };
}
unsafe extern "C" fn cleanupnodes() {
    let mut j: int8_t = 0;
    if Playing() == 0 {
        return;
    }
    j = 1 as libc::c_int as int8_t;
    while (j as libc::c_int) < 127 as libc::c_int {
        if !(nodeingame[j as usize] != 0 || SendingFile(j as int32_t) != 0) {
            nodeconnected[j as usize] = false_0 as libc::c_int;
        }
        j += 1;
        j;
    }
}
unsafe extern "C" fn getfreenode() -> int8_t {
    let mut j: int8_t = 0;
    cleanupnodes();
    j = 0 as libc::c_int as int8_t;
    while (j as libc::c_int) < 127 as libc::c_int {
        if nodeconnected[j as usize] == 0 {
            nodeconnected[j as usize] = true_0 as libc::c_int;
            return j;
        }
        j += 1;
        j;
    }
    return -(1 as libc::c_int) as int8_t;
}
unsafe extern "C" fn SOCK_Get() -> boolean {
    let mut i: size_t = 0;
    let mut n: size_t = 0;
    let mut j: libc::c_int = 0;
    let mut c: ssize_t = 0;
    let mut fromaddress: mysockaddr_t = mysockaddr_t {
        any: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut fromlen: socklen_t = 0;
    n = 0 as libc::c_int as size_t;
    while n < mysocketses {
        fromlen = ::core::mem::size_of::<mysockaddr_t>() as libc::c_ulong as socklen_t;
        c = recvfrom(
            mysockets[n as usize],
            &mut (*doomcom).data as *mut [libc::c_char; 1450] as *mut libc::c_char
                as *mut libc::c_void,
            1450 as libc::c_int as size_t,
            0 as libc::c_int,
            &mut fromaddress as *mut mysockaddr_t as *mut libc::c_void as *mut sockaddr,
            &mut fromlen,
        );
        if c != -(1 as libc::c_int) as ssize_t {
            j = 1 as libc::c_int;
            while j <= 127 as libc::c_int {
                if SOCK_cmpaddr(
                    &mut fromaddress,
                    &mut *clientaddress.as_mut_ptr().offset(j as isize),
                    0 as libc::c_int as uint8_t,
                ) != 0
                {
                    (*doomcom).remotenode = j as int16_t;
                    (*doomcom).datalength = c as int16_t;
                    nodesocket[j as usize] = mysockets[n as usize];
                    return false_0 as libc::c_int;
                }
                j += 1;
                j;
            }
            j = getfreenode() as libc::c_int;
            if j > 0 as libc::c_int {
                M_Memcpy
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *clientaddress.as_mut_ptr().offset(j as isize)
                        as *mut mysockaddr_t as *mut libc::c_void,
                    &mut fromaddress as *mut mysockaddr_t as *const libc::c_void,
                    fromlen as size_t,
                );
                nodesocket[j as usize] = mysockets[n as usize];
                if !debugfile.is_null() {
                    fputs(
                        va(
                            b"New node detected: node:%d address:%s\n\0" as *const u8
                                as *const libc::c_char,
                            j,
                            SOCK_GetNodeAddress(j),
                        ),
                        debugfile,
                    );
                    fflush(debugfile);
                }
                (*doomcom).remotenode = j as int16_t;
                (*doomcom).datalength = c as int16_t;
                i = 0 as libc::c_int as size_t;
                while i < numbans {
                    if SOCK_cmpaddr(
                        &mut fromaddress,
                        &mut *banned.as_mut_ptr().offset(i as isize),
                        bannedmask[i as usize],
                    ) != 0
                    {
                        SOCK_bannednode[j as usize] = true_0 as libc::c_int;
                        if !debugfile.is_null() {
                            fputs(
                                b"This dude has been banned\n\0" as *const u8
                                    as *const libc::c_char,
                                debugfile,
                            );
                            fflush(debugfile);
                        }
                        break;
                    } else {
                        i = i.wrapping_add(1);
                        i;
                    }
                }
                if i == numbans {
                    SOCK_bannednode[j as usize] = false_0 as libc::c_int;
                }
                return true_0 as libc::c_int;
            } else if !debugfile.is_null() {
                fputs(
                    b"New node detected: No more free slots\n\0" as *const u8
                        as *const libc::c_char,
                    debugfile,
                );
                fflush(debugfile);
            }
        }
        n = n.wrapping_add(1);
        n;
    }
    (*doomcom).remotenode = -(1 as libc::c_int) as int16_t;
    return false_0 as libc::c_int;
}
static mut masterset: fd_set = fd_set { __fds_bits: [0; 16] };
unsafe extern "C" fn FD_CPY(
    mut src: *mut fd_set,
    mut dst: *mut fd_set,
    mut fd: *mut SOCKET_TYPE,
    mut len: size_t,
) -> boolean {
    let mut i: size_t = 0;
    let mut testset: boolean = false_0 as libc::c_int;
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = dst;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
        __i;
    }
    i = 0 as libc::c_int as size_t;
    while i < len {
        if *fd.offset(i as isize) != -(1 as libc::c_int)
            && (*src)
                .__fds_bits[(*fd.offset(i as isize)
                / (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << *fd.offset(i as isize)
                        % (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as __fd_mask
            && !((*dst)
                .__fds_bits[(*fd.offset(i as isize)
                / (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << *fd.offset(i as isize)
                        % (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as __fd_mask)
        {
            (*dst)
                .__fds_bits[(*fd.offset(i as isize)
                / (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << *fd.offset(i as isize)
                        % (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            testset = true_0 as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    return testset;
}
unsafe extern "C" fn SOCK_CanSend() -> boolean {
    let mut timeval_for_select: timeval = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 0 as libc::c_int as __suseconds_t,
        };
        init
    };
    let mut tset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut wselect: libc::c_int = 0;
    if FD_CPY(&mut masterset, &mut tset, mysockets.as_mut_ptr(), mysocketses) == 0 {
        return false_0 as libc::c_int;
    }
    wselect = select(
        255 as libc::c_int,
        0 as *mut fd_set,
        &mut tset,
        0 as *mut fd_set,
        &mut timeval_for_select,
    );
    if wselect >= 1 as libc::c_int {
        return true_0 as libc::c_int;
    }
    return false_0 as libc::c_int;
}
unsafe extern "C" fn SOCK_CanGet() -> boolean {
    let mut timeval_for_select: timeval = {
        let mut init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 0 as libc::c_int as __suseconds_t,
        };
        init
    };
    let mut tset: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut rselect: libc::c_int = 0;
    if FD_CPY(&mut masterset, &mut tset, mysockets.as_mut_ptr(), mysocketses) == 0 {
        return false_0 as libc::c_int;
    }
    rselect = select(
        255 as libc::c_int,
        &mut tset,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut timeval_for_select,
    );
    if rselect >= 1 as libc::c_int {
        return true_0 as libc::c_int;
    }
    return false_0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn SOCK_SendToAddr(
    mut socket_0: SOCKET_TYPE,
    mut sockaddr: *mut mysockaddr_t,
) -> ssize_t {
    let mut d4: socklen_t = ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        as socklen_t;
    let mut d6: socklen_t = ::core::mem::size_of::<sockaddr_in6>() as libc::c_ulong
        as socklen_t;
    let mut d: socklen_t = 0;
    let mut da: socklen_t = ::core::mem::size_of::<mysockaddr_t>() as libc::c_ulong
        as socklen_t;
    match (*sockaddr).any.sa_family as libc::c_int {
        2 => {
            d = d4;
        }
        10 => {
            d = d6;
        }
        _ => {
            d = da;
        }
    }
    return sendto(
        socket_0,
        &mut (*doomcom).data as *mut [libc::c_char; 1450] as *mut libc::c_char
            as *const libc::c_void,
        (*doomcom).datalength as size_t,
        0 as libc::c_int,
        &mut (*sockaddr).any,
        d,
    );
}
unsafe extern "C" fn SOCK_Send() {
    let mut c: ssize_t = -(1 as libc::c_int) as ssize_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    if nodeconnected[(*doomcom).remotenode as usize] == 0 {
        return;
    }
    if (*doomcom).remotenode as libc::c_int == 127 as libc::c_int {
        i = 0 as libc::c_int as size_t;
        while i < mysocketses {
            j = 0 as libc::c_int as size_t;
            while j < broadcastaddresses {
                if myfamily[i as usize]
                    == broadcastaddress[j as usize].any.sa_family as libc::c_int
                {
                    SOCK_SendToAddr(
                        mysockets[i as usize],
                        &mut *broadcastaddress.as_mut_ptr().offset(j as isize),
                    );
                }
                j = j.wrapping_add(1);
                j;
            }
            i = i.wrapping_add(1);
            i;
        }
        return;
    } else if nodesocket[(*doomcom).remotenode as usize] == -(1 as libc::c_int) {
        i = 0 as libc::c_int as size_t;
        while i < mysocketses {
            if myfamily[i as usize]
                == clientaddress[(*doomcom).remotenode as usize].any.sa_family
                    as libc::c_int
            {
                SOCK_SendToAddr(
                    mysockets[i as usize],
                    &mut *clientaddress
                        .as_mut_ptr()
                        .offset((*doomcom).remotenode as isize),
                );
            }
            i = i.wrapping_add(1);
            i;
        }
        return;
    } else {
        c = SOCK_SendToAddr(
            nodesocket[(*doomcom).remotenode as usize],
            &mut *clientaddress.as_mut_ptr().offset((*doomcom).remotenode as isize),
        );
    }
    if c == -(1 as libc::c_int) as ssize_t {
        let mut e: libc::c_int = *__errno_location();
        if e != 111 as libc::c_int && e != 11 as libc::c_int {
            I_Error(
                b"SOCK_Send, error sending to node %d (%s) #%u: %s\0" as *const u8
                    as *const libc::c_char,
                (*doomcom).remotenode as libc::c_int,
                SOCK_GetNodeAddress((*doomcom).remotenode as int32_t),
                e,
                strerror(e),
            );
        }
    }
}
unsafe extern "C" fn SOCK_FreeNodenum(mut numnode: int32_t) {
    if numnode == 0 || numnode > 127 as libc::c_int {
        return;
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"Free node %d (%s)\n\0" as *const u8 as *const libc::c_char,
                numnode,
                SOCK_GetNodeAddress(numnode),
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    nodeconnected[numnode as usize] = false_0 as libc::c_int;
    nodesocket[numnode as usize] = -(1 as libc::c_int);
    memset(
        &mut *clientaddress.as_mut_ptr().offset(numnode as isize) as *mut mysockaddr_t
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<mysockaddr_t>() as libc::c_ulong,
    );
}
unsafe extern "C" fn UDP_Bind(
    mut family: libc::c_int,
    mut addr: *mut sockaddr,
    mut addrlen: socklen_t,
) -> SOCKET_TYPE {
    let mut s: SOCKET_TYPE = socket(
        family,
        SOCK_DGRAM as libc::c_int,
        IPPROTO_UDP as libc::c_int,
    );
    let mut opt: libc::c_int = 0;
    let mut opts: socklen_t = 0;
    let mut trueval: libc::c_ulong = true_0 as libc::c_int as libc::c_ulong;
    let mut straddr: mysockaddr_t = mysockaddr_t {
        any: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut len: socklen_t = ::core::mem::size_of::<mysockaddr_t>() as libc::c_ulong
        as socklen_t;
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memcpy(
        &mut straddr as *mut mysockaddr_t as *mut libc::c_void,
        addr as *const libc::c_void,
        addrlen as libc::c_ulong,
    );
    I_OutputMsg(
        b"Binding to %s\n\0" as *const u8 as *const libc::c_char,
        SOCK_AddrToStr(&mut straddr),
    );
    if family == 2 as libc::c_int {
        if straddr.ip4.sin_addr.s_addr == __bswap_32(0 as libc::c_int as in_addr_t) {
            opt = true_0 as libc::c_int;
            opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
            setsockopt(
                s,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                opts,
            );
        }
        opt = true_0 as libc::c_int;
        opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
        if setsockopt(
            s,
            1 as libc::c_int,
            6 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
            opts,
        ) != 0
        {
            CONS_Alert(
                CONS_WARNING,
                b"Could not get broadcast rights\n\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if family == 10 as libc::c_int {
        if memcmp(
            &mut straddr.ip6.sin6_addr as *mut in6_addr as *const libc::c_void,
            &in6addr_any as *const in6_addr as *const libc::c_void,
            ::core::mem::size_of::<in6_addr>() as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            opt = true_0 as libc::c_int;
            opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
            setsockopt(
                s,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
                opts,
            );
        }
        opt = true_0 as libc::c_int;
        opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
        if setsockopt(
            s,
            IPPROTO_IPV6 as libc::c_int,
            26 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
            opts,
        ) != 0
        {
            CONS_Alert(
                CONS_WARNING,
                b"Could not limit IPv6 bind\n\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if bind(s, addr, addrlen) == -(1 as libc::c_int) {
        close(s);
        I_OutputMsg(b"Binding failed\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    opt = true_0 as libc::c_int;
    if ioctl(
        s,
        0x5421 as libc::c_int as libc::c_ulong,
        &mut trueval as *mut libc::c_ulong,
    ) != 0 as libc::c_int
    {
        close(s);
        I_OutputMsg(b"Seting FIOBIO on failed\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int);
    }
    opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
    getsockopt(
        s,
        1 as libc::c_int,
        8 as libc::c_int,
        &mut opt as *mut libc::c_int as *mut libc::c_char as *mut libc::c_void,
        &mut opts,
    );
    CONS_Printf(
        b"Network system buffer: %dKb\n\0" as *const u8 as *const libc::c_char,
        opt >> 10 as libc::c_int,
    );
    if opt < (64 as libc::c_int) << 10 as libc::c_int {
        opt = (64 as libc::c_int) << 10 as libc::c_int;
        opts = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t;
        setsockopt(
            s,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
            opts,
        );
        getsockopt(
            s,
            1 as libc::c_int,
            8 as libc::c_int,
            &mut opt as *mut libc::c_int as *mut libc::c_char as *mut libc::c_void,
            &mut opts,
        );
        if opt < (64 as libc::c_int) << 10 as libc::c_int {
            CONS_Alert(
                CONS_WARNING,
                b"Can't set buffer length to 64k, file transfer will be bad\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            CONS_Printf(
                b"Network system buffer set to: %dKb\n\0" as *const u8
                    as *const libc::c_char,
                opt >> 10 as libc::c_int,
            );
        }
    }
    if getsockname(s, &mut straddr.any, &mut len) == -(1 as libc::c_int) {
        CONS_Alert(
            CONS_WARNING,
            b"Failed to get port number\n\0" as *const u8 as *const libc::c_char,
        );
    } else if family == 2 as libc::c_int {
        current_port = __bswap_16(straddr.ip4.sin_port);
    } else if family == 10 as libc::c_int {
        current_port = __bswap_16(straddr.ip6.sin6_port);
    }
    return s;
}
unsafe extern "C" fn UDP_Socket() -> boolean {
    let mut s: size_t = 0;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut runp: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut gaie: libc::c_int = 0;
    let b_ipv6: int32_t = (M_CheckParm(b"-noipv6\0" as *const u8 as *const libc::c_char)
        == 0) as libc::c_int;
    let mut serv: *const libc::c_char = 0 as *const libc::c_char;
    s = 0 as libc::c_int as size_t;
    while s < mysocketses {
        mysockets[s as usize] = -(1 as libc::c_int);
        s = s.wrapping_add(1);
        s;
    }
    s = 0 as libc::c_int as size_t;
    while s < (127 as libc::c_int + 1 as libc::c_int) as size_t {
        nodesocket[s as usize] = -(1 as libc::c_int);
        s = s.wrapping_add(1);
        s;
    }
    let mut __i: libc::c_uint = 0;
    let mut __arr: *mut fd_set = &mut masterset;
    __i = 0 as libc::c_int as libc::c_uint;
    while (__i as libc::c_ulong)
        < (::core::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as libc::c_ulong)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as libc::c_int as __fd_mask;
        __i = __i.wrapping_add(1);
        __i;
    }
    s = 0 as libc::c_int as size_t;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x4 as libc::c_int;
    hints.ai_family = 2 as libc::c_int;
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    hints.ai_protocol = IPPROTO_UDP as libc::c_int;
    if serverrunning != 0 {
        serv = serverport_name;
    } else {
        serv = clientport_name;
    }
    if M_CheckParm(b"-bindaddr\0" as *const u8 as *const libc::c_char) != 0 {
        while M_IsNextParm() != 0 {
            gaie = getaddrinfo(M_GetNextParm(), serv, &mut hints, &mut ai);
            if gaie == 0 as libc::c_int {
                runp = ai;
                while !runp.is_null()
                    && s < (127 as libc::c_int + 1 as libc::c_int) as size_t
                {
                    mysockets[s
                        as usize] = UDP_Bind(
                        (*runp).ai_family,
                        (*runp).ai_addr,
                        (*runp).ai_addrlen,
                    );
                    if mysockets[s as usize] != -(1 as libc::c_int) {
                        masterset
                            .__fds_bits[(mysockets[s as usize]
                            / (8 as libc::c_int
                                * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            |= ((1 as libc::c_ulong)
                                << mysockets[s as usize]
                                    % (8 as libc::c_int
                                        * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask;
                        myfamily[s as usize] = hints.ai_family;
                        s = s.wrapping_add(1);
                        s;
                    }
                    runp = (*runp).ai_next;
                }
                freeaddrinfo(ai);
            }
        }
    } else {
        gaie = getaddrinfo(
            b"0.0.0.0\0" as *const u8 as *const libc::c_char,
            serv,
            &mut hints,
            &mut ai,
        );
        if gaie == 0 as libc::c_int {
            runp = ai;
            while !runp.is_null()
                && s < (127 as libc::c_int + 1 as libc::c_int) as size_t
            {
                mysockets[s
                    as usize] = UDP_Bind(
                    (*runp).ai_family,
                    (*runp).ai_addr,
                    (*runp).ai_addrlen,
                );
                if mysockets[s as usize] != -(1 as libc::c_int) {
                    masterset
                        .__fds_bits[(mysockets[s as usize]
                        / (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as usize]
                        |= ((1 as libc::c_ulong)
                            << mysockets[s as usize]
                                % (8 as libc::c_int
                                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as __fd_mask;
                    myfamily[s as usize] = hints.ai_family;
                    s = s.wrapping_add(1);
                    s;
                }
                runp = (*runp).ai_next;
            }
            freeaddrinfo(ai);
        }
    }
    if b_ipv6 != 0 {
        hints.ai_family = 10 as libc::c_int;
        if M_CheckParm(b"-bindaddr6\0" as *const u8 as *const libc::c_char) != 0 {
            while M_IsNextParm() != 0 {
                gaie = getaddrinfo(M_GetNextParm(), serv, &mut hints, &mut ai);
                if gaie == 0 as libc::c_int {
                    runp = ai;
                    while !runp.is_null()
                        && s < (127 as libc::c_int + 1 as libc::c_int) as size_t
                    {
                        mysockets[s
                            as usize] = UDP_Bind(
                            (*runp).ai_family,
                            (*runp).ai_addr,
                            (*runp).ai_addrlen,
                        );
                        if mysockets[s as usize] != -(1 as libc::c_int) {
                            masterset
                                .__fds_bits[(mysockets[s as usize]
                                / (8 as libc::c_int
                                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                        as libc::c_int)) as usize]
                                |= ((1 as libc::c_ulong)
                                    << mysockets[s as usize]
                                        % (8 as libc::c_int
                                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                                as libc::c_int)) as __fd_mask;
                            myfamily[s as usize] = hints.ai_family;
                            s = s.wrapping_add(1);
                            s;
                        }
                        runp = (*runp).ai_next;
                    }
                    freeaddrinfo(ai);
                }
            }
        } else {
            gaie = getaddrinfo(
                b"::\0" as *const u8 as *const libc::c_char,
                serv,
                &mut hints,
                &mut ai,
            );
            if gaie == 0 as libc::c_int {
                runp = ai;
                while !runp.is_null()
                    && s < (127 as libc::c_int + 1 as libc::c_int) as size_t
                {
                    mysockets[s
                        as usize] = UDP_Bind(
                        (*runp).ai_family,
                        (*runp).ai_addr,
                        (*runp).ai_addrlen,
                    );
                    if mysockets[s as usize] != -(1 as libc::c_int) {
                        masterset
                            .__fds_bits[(mysockets[s as usize]
                            / (8 as libc::c_int
                                * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as usize]
                            |= ((1 as libc::c_ulong)
                                << mysockets[s as usize]
                                    % (8 as libc::c_int
                                        * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                            as libc::c_int)) as __fd_mask;
                        myfamily[s as usize] = hints.ai_family;
                        s = s.wrapping_add(1);
                        s;
                    }
                    runp = (*runp).ai_next;
                }
                freeaddrinfo(ai);
            }
        }
    }
    mysocketses = s;
    if s == 0 as libc::c_int as size_t {
        return false_0 as libc::c_int;
    }
    s = 0 as libc::c_int as size_t;
    packetheaderlength = 20 as libc::c_int + 8 as libc::c_int;
    hints.ai_family = 2 as libc::c_int;
    gaie = getaddrinfo(
        b"127.0.0.1\0" as *const u8 as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        &mut hints,
        &mut ai,
    );
    if gaie == 0 as libc::c_int {
        runp = ai;
        while !runp.is_null() && s < (127 as libc::c_int + 1 as libc::c_int) as size_t {
            memcpy(
                &mut *clientaddress.as_mut_ptr().offset(s as isize) as *mut mysockaddr_t
                    as *mut libc::c_void,
                (*runp).ai_addr as *const libc::c_void,
                (*runp).ai_addrlen as libc::c_ulong,
            );
            s = s.wrapping_add(1);
            s;
            runp = (*runp).ai_next;
        }
        freeaddrinfo(ai);
    } else {
        clientaddress[s as usize].any.sa_family = 2 as libc::c_int as sa_family_t;
        clientaddress[s as usize]
            .ip4
            .sin_port = __bswap_16(0 as libc::c_int as __uint16_t);
        clientaddress[s as usize]
            .ip4
            .sin_addr
            .s_addr = __bswap_32(0x7f000001 as libc::c_int as in_addr_t);
        s = s.wrapping_add(1);
        s;
    }
    s = 0 as libc::c_int as size_t;
    gaie = getaddrinfo(
        b"255.255.255.255\0" as *const u8 as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        &mut hints,
        &mut ai,
    );
    if gaie == 0 as libc::c_int {
        runp = ai;
        while !runp.is_null() && s < (127 as libc::c_int + 1 as libc::c_int) as size_t {
            memcpy(
                &mut *broadcastaddress.as_mut_ptr().offset(s as isize)
                    as *mut mysockaddr_t as *mut libc::c_void,
                (*runp).ai_addr as *const libc::c_void,
                (*runp).ai_addrlen as libc::c_ulong,
            );
            s = s.wrapping_add(1);
            s;
            runp = (*runp).ai_next;
        }
        freeaddrinfo(ai);
    } else {
        broadcastaddress[s as usize].any.sa_family = 2 as libc::c_int as sa_family_t;
        broadcastaddress[s as usize]
            .ip4
            .sin_port = __bswap_16(0 as libc::c_int as __uint16_t);
        broadcastaddress[s as usize]
            .ip4
            .sin_addr
            .s_addr = __bswap_32(0xffffffff as libc::c_uint);
        s = s.wrapping_add(1);
        s;
    }
    if b_ipv6 != 0 {
        hints.ai_family = 10 as libc::c_int;
        gaie = getaddrinfo(
            b"ff02::1\0" as *const u8 as *const libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char,
            &mut hints,
            &mut ai,
        );
        if gaie == 0 as libc::c_int {
            runp = ai;
            while !runp.is_null()
                && s < (127 as libc::c_int + 1 as libc::c_int) as size_t
            {
                memcpy(
                    &mut *broadcastaddress.as_mut_ptr().offset(s as isize)
                        as *mut mysockaddr_t as *mut libc::c_void,
                    (*runp).ai_addr as *const libc::c_void,
                    (*runp).ai_addrlen as libc::c_ulong,
                );
                s = s.wrapping_add(1);
                s;
                runp = (*runp).ai_next;
            }
            freeaddrinfo(ai);
        }
    }
    broadcastaddresses = s;
    (*doomcom).extratics = 1 as libc::c_int as int16_t;
    return true_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn I_InitTcpDriver() -> boolean {
    let mut tcp_was_up: boolean = init_tcp_driver;
    if init_tcp_driver == 0 {
        init_tcp_driver = true_0 as libc::c_int;
    }
    if tcp_was_up == 0 && init_tcp_driver != 0 {
        I_AddExitFunc(
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn() -> ()>,
                Option::<unsafe extern "C" fn() -> ()>,
            >(Some(I_ShutdownTcpDriver as unsafe extern "C" fn() -> ())),
        );
    }
    return init_tcp_driver;
}
unsafe extern "C" fn SOCK_CloseSocket() {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (127 as libc::c_int + 1 as libc::c_int) as size_t {
        if mysockets[i as usize] != -(1 as libc::c_int)
            && masterset
                .__fds_bits[(mysockets[i as usize]
                / (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << mysockets[i as usize]
                        % (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask
                != 0 as libc::c_int as __fd_mask
        {
            masterset
                .__fds_bits[(mysockets[i as usize]
                / (8 as libc::c_int
                    * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                &= !(((1 as libc::c_ulong)
                    << mysockets[i as usize]
                        % (8 as libc::c_int
                            * ::core::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask);
            close(mysockets[i as usize]);
        }
        mysockets[i as usize] = -(1 as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn I_ShutdownTcpDriver() {
    SOCK_CloseSocket();
    CONS_Printf(b"I_ShutdownTcpDriver: \0" as *const u8 as *const libc::c_char);
    CONS_Printf(b"shut down\n\0" as *const u8 as *const libc::c_char);
    init_tcp_driver = false_0 as libc::c_int;
}
unsafe extern "C" fn SOCK_NetMakeNodewPort(
    mut address: *const libc::c_char,
    mut port: *const libc::c_char,
) -> int8_t {
    let mut newnode: int8_t = -(1 as libc::c_int) as int8_t;
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut runp: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut gaie: libc::c_int = 0;
    let mut i: size_t = 0;
    if port.is_null() || *port.offset(0 as libc::c_int as isize) == 0 {
        port = b"5029\0" as *const u8 as *const libc::c_char;
    }
    if !debugfile.is_null() {
        fputs(
            va(
                b"Creating new node: %s@%s\n\0" as *const u8 as *const libc::c_char,
                address,
                port,
            ),
            debugfile,
        );
        fflush(debugfile);
    }
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0 as libc::c_int;
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    hints.ai_protocol = IPPROTO_UDP as libc::c_int;
    gaie = getaddrinfo(address, port, &mut hints, &mut ai);
    if gaie == 0 as libc::c_int {
        newnode = getfreenode();
    }
    if newnode as libc::c_int == -(1 as libc::c_int) {
        freeaddrinfo(ai);
        return -(1 as libc::c_int) as int8_t;
    } else {
        runp = ai;
    }
    while !runp.is_null() {
        i = 0 as libc::c_int as size_t;
        while i < mysocketses {
            if (*(*runp).ai_addr).sa_family as libc::c_int == myfamily[i as usize]
                && sendto(
                    mysockets[i as usize],
                    0 as *const libc::c_void,
                    0 as libc::c_int as size_t,
                    0 as libc::c_int,
                    (*runp).ai_addr,
                    (*runp).ai_addrlen,
                ) == 0 as libc::c_int as ssize_t
            {
                memcpy(
                    &mut *clientaddress.as_mut_ptr().offset(newnode as isize)
                        as *mut mysockaddr_t as *mut libc::c_void,
                    (*runp).ai_addr as *const libc::c_void,
                    (*runp).ai_addrlen as libc::c_ulong,
                );
                break;
            } else {
                i = i.wrapping_add(1);
                i;
            }
        }
        if !(i < mysocketses) {
            break;
        }
        runp = (*runp).ai_next;
    }
    freeaddrinfo(ai);
    return newnode;
}
unsafe extern "C" fn SOCK_OpenSocket() -> boolean {
    let mut i: size_t = 0;
    memset(
        clientaddress.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[mysockaddr_t; 128]>() as libc::c_ulong,
    );
    nodeconnected[0 as libc::c_int as usize] = true_0 as libc::c_int;
    i = 1 as libc::c_int as size_t;
    while i < 127 as libc::c_int as size_t {
        nodeconnected[i as usize] = false_0 as libc::c_int;
        i = i.wrapping_add(1);
        i;
    }
    nodeconnected[127 as libc::c_int as usize] = true_0 as libc::c_int;
    I_NetSend = Some(SOCK_Send as unsafe extern "C" fn() -> ());
    I_NetGet = Some(SOCK_Get as unsafe extern "C" fn() -> boolean);
    I_NetCloseSocket = Some(SOCK_CloseSocket as unsafe extern "C" fn() -> ());
    I_NetFreeNodenum = Some(SOCK_FreeNodenum as unsafe extern "C" fn(int32_t) -> ());
    I_NetMakeNodewPort = Some(
        SOCK_NetMakeNodewPort
            as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> int8_t,
    );
    I_NetCanSend = Some(SOCK_CanSend as unsafe extern "C" fn() -> boolean);
    I_NetCanGet = Some(SOCK_CanGet as unsafe extern "C" fn() -> boolean);
    SOCK_CloseSocket();
    return UDP_Socket();
}
unsafe extern "C" fn SOCK_Ban(mut node: int32_t) -> boolean {
    if node > 127 as libc::c_int {
        return false_0 as libc::c_int;
    }
    if numbans == 100 as libc::c_int as size_t {
        return false_0 as libc::c_int;
    }
    M_Memcpy
        .expect(
            "non-null function pointer",
        )(
        &mut *banned.as_mut_ptr().offset(numbans as isize) as *mut mysockaddr_t
            as *mut libc::c_void,
        &mut *clientaddress.as_mut_ptr().offset(node as isize) as *mut mysockaddr_t
            as *const libc::c_void,
        ::core::mem::size_of::<mysockaddr_t>() as libc::c_ulong,
    );
    if banned[numbans as usize].any.sa_family as libc::c_int == 2 as libc::c_int {
        banned[numbans as usize].ip4.sin_port = 0 as libc::c_int as in_port_t;
        bannedmask[numbans as usize] = 32 as libc::c_int as uint8_t;
    } else if banned[numbans as usize].any.sa_family as libc::c_int == 10 as libc::c_int
    {
        banned[numbans as usize].ip6.sin6_port = 0 as libc::c_int as in_port_t;
        bannedmask[numbans as usize] = 128 as libc::c_int as uint8_t;
    }
    numbans = numbans.wrapping_add(1);
    numbans;
    return true_0 as libc::c_int;
}
unsafe extern "C" fn SOCK_SetBanAddress(
    mut address: *const libc::c_char,
    mut mask: *const libc::c_char,
) -> boolean {
    let mut ai: *mut addrinfo = 0 as *mut addrinfo;
    let mut runp: *mut addrinfo = 0 as *mut addrinfo;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut gaie: libc::c_int = 0;
    if numbans == 100 as libc::c_int as size_t || address.is_null() {
        return false_0 as libc::c_int;
    }
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0 as libc::c_int;
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    hints.ai_protocol = IPPROTO_UDP as libc::c_int;
    gaie = getaddrinfo(
        address,
        b"0\0" as *const u8 as *const libc::c_char,
        &mut hints,
        &mut ai,
    );
    if gaie != 0 as libc::c_int {
        return false_0 as libc::c_int;
    }
    runp = ai;
    while !runp.is_null() && numbans != 100 as libc::c_int as size_t {
        memcpy(
            &mut *banned.as_mut_ptr().offset(numbans as isize) as *mut mysockaddr_t
                as *mut libc::c_void,
            (*runp).ai_addr as *const libc::c_void,
            (*runp).ai_addrlen as libc::c_ulong,
        );
        if !mask.is_null() {
            bannedmask[numbans as usize] = atoi(mask) as uint8_t;
        } else if (*runp).ai_family == 10 as libc::c_int {
            bannedmask[numbans as usize] = 128 as libc::c_int as uint8_t;
        } else {
            bannedmask[numbans as usize] = 32 as libc::c_int as uint8_t;
        }
        if bannedmask[numbans as usize] as libc::c_int > 32 as libc::c_int
            && (*runp).ai_family == 2 as libc::c_int
        {
            bannedmask[numbans as usize] = 32 as libc::c_int as uint8_t;
        } else if bannedmask[numbans as usize] as libc::c_int > 128 as libc::c_int
            && (*runp).ai_family == 10 as libc::c_int
        {
            bannedmask[numbans as usize] = 128 as libc::c_int as uint8_t;
        }
        numbans = numbans.wrapping_add(1);
        numbans;
        runp = (*runp).ai_next;
    }
    freeaddrinfo(ai);
    return true_0 as libc::c_int;
}
unsafe extern "C" fn SOCK_ClearBans() {
    numbans = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn I_InitTcpNetwork() -> boolean {
    let mut serverhostname: [libc::c_char; 255] = [0; 255];
    let mut urlparam: *const libc::c_char = 0 as *const libc::c_char;
    let mut ret: boolean = false_0 as libc::c_int;
    if I_InitTcpDriver() == 0 {
        return false_0 as libc::c_int;
    }
    if M_CheckParm(b"-port\0" as *const u8 as *const libc::c_char) != 0
        || M_CheckParm(b"-serverport\0" as *const u8 as *const libc::c_char) != 0
    {
        serverport_name = M_GetNextParm();
    }
    if M_CheckParm(b"-clientport\0" as *const u8 as *const libc::c_char) != 0 {
        clientport_name = M_GetNextParm();
    }
    if M_CheckParm(b"-server\0" as *const u8 as *const libc::c_char) != 0
        || dedicated != 0
    {
        server = true_0 as libc::c_int;
        if dedicated != 0 {
            (*doomcom).numnodes = 0 as libc::c_int as int16_t;
        } else {
            (*doomcom).numnodes = 1 as libc::c_int as int16_t;
        }
        if ((*doomcom).numnodes as libc::c_int) < 0 as libc::c_int {
            (*doomcom).numnodes = 0 as libc::c_int as int16_t;
        }
        if (*doomcom).numnodes as libc::c_int > 127 as libc::c_int {
            (*doomcom).numnodes = 127 as libc::c_int as int16_t;
        }
        servernode = 0 as libc::c_int as int8_t;
        net_bandwidth = 16000 as libc::c_int;
        hardware_MAXPACKETLENGTH = 1024 as libc::c_int as int16_t;
        ret = true_0 as libc::c_int;
    } else {
        urlparam = M_GetUrlProtocolArg();
        if !urlparam.is_null()
            || M_CheckParm(b"-connect\0" as *const u8 as *const libc::c_char) != 0
        {
            if !urlparam.is_null() {
                strlcpy(
                    serverhostname.as_mut_ptr(),
                    urlparam,
                    ::core::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                );
            } else if M_IsNextParm() != 0 {
                strlcpy(
                    serverhostname.as_mut_ptr(),
                    M_GetNextParm(),
                    ::core::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                );
            } else {
                serverhostname[0 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_char;
            }
            if serverhostname[0 as libc::c_int as usize] != 0 {
                COM_BufAddTextEx(
                    b"connect \"\0" as *const u8 as *const libc::c_char,
                    0 as com_flags_t,
                );
                COM_BufAddTextEx(serverhostname.as_mut_ptr(), 0 as com_flags_t);
                COM_BufAddTextEx(
                    b"\"\n\0" as *const u8 as *const libc::c_char,
                    0 as com_flags_t,
                );
                hardware_MAXPACKETLENGTH = 1024 as libc::c_int as int16_t;
            } else {
                COM_BufAddTextEx(
                    b"connect any\n\0" as *const u8 as *const libc::c_char,
                    0 as com_flags_t,
                );
                net_bandwidth = 800000 as libc::c_int;
                hardware_MAXPACKETLENGTH = 1450 as libc::c_int as int16_t;
            }
        }
    }
    I_NetOpenSocket = Some(SOCK_OpenSocket as unsafe extern "C" fn() -> boolean);
    I_Ban = Some(SOCK_Ban as unsafe extern "C" fn(int32_t) -> boolean);
    I_ClearBans = Some(SOCK_ClearBans as unsafe extern "C" fn() -> ());
    I_GetNodeAddress = Some(
        SOCK_GetNodeAddress as unsafe extern "C" fn(int32_t) -> *const libc::c_char,
    );
    I_GetBanAddress = Some(
        SOCK_GetBanAddress as unsafe extern "C" fn(size_t) -> *const libc::c_char,
    );
    I_GetBanMask = Some(
        SOCK_GetBanMask as unsafe extern "C" fn(size_t) -> *const libc::c_char,
    );
    I_SetBanAddress = Some(
        SOCK_SetBanAddress
            as unsafe extern "C" fn(*const libc::c_char, *const libc::c_char) -> boolean,
    );
    bannednode = SOCK_bannednode.as_mut_ptr();
    return ret;
}
