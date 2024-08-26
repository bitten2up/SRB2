use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn curl_easy_escape(
        handle: *mut libc::c_void,
        string: *const libc::c_char,
        length: libc::c_int,
    ) -> *mut libc::c_char;
    fn curl_easy_unescape(
        handle: *mut libc::c_void,
        string: *const libc::c_char,
        length: libc::c_int,
        outlength: *mut libc::c_int,
    ) -> *mut libc::c_char;
    fn curl_free(p: *mut libc::c_void);
    fn curl_global_init(flags: libc::c_long) -> CURLcode;
    fn curl_global_cleanup();
    fn curl_easy_strerror(_: CURLcode) -> *const libc::c_char;
    fn curl_easy_init() -> *mut libc::c_void;
    fn curl_easy_setopt(curl: *mut libc::c_void, option: CURLoption, _: ...) -> CURLcode;
    fn curl_easy_perform(curl: *mut libc::c_void) -> CURLcode;
    fn curl_easy_cleanup(curl: *mut libc::c_void);
    fn curl_easy_getinfo(curl: *mut libc::c_void, info: CURLINFO, _: ...) -> CURLcode;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn atexit(__func: Option::<unsafe extern "C" fn() -> ()>) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strlcpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_ulong;
    static mut compbranch: *const libc::c_char;
    static mut comprevision: *const libc::c_char;
    fn CONS_Alert(level: alerttype_t, fmt: *const libc::c_char, _: ...);
    fn CONS_Printf(fmt: *const libc::c_char, _: ...);
    fn I_Error(error: *const libc::c_char, _: ...) -> !;
    static mut logstream: *mut FILE;
    static mut CV_Unsigned: [CV_PossibleValue_t; 0];
    static mut CV_OnOff: [CV_PossibleValue_t; 0];
    static mut room_list: [msg_rooms_t; 17];
    static mut ms_RoomId: int16_t;
    static mut cv_servername: consvar_t;
    fn M_CheckParm(check: *const libc::c_char) -> int32_t;
    fn M_IsNextParm() -> boolean;
    fn M_GetNextParm() -> *const libc::c_char;
    static mut roomIds: [uint32_t; 16];
    static mut MP_RoomMenu: [menuitem_t; 0];
    static mut current_port: uint16_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
pub type CURL = ();
pub type CURLcode = libc::c_uint;
pub const CURL_LAST: CURLcode = 99;
pub const CURLE_SSL_CLIENTCERT: CURLcode = 98;
pub const CURLE_PROXY: CURLcode = 97;
pub const CURLE_QUIC_CONNECT_ERROR: CURLcode = 96;
pub const CURLE_HTTP3: CURLcode = 95;
pub const CURLE_AUTH_ERROR: CURLcode = 94;
pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
pub const CURLE_HTTP2_STREAM: CURLcode = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
pub const CURLE_CHUNK_FAILED: CURLcode = 88;
pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
pub const CURLE_AGAIN: CURLcode = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
pub const CURLE_SSH: CURLcode = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
pub const CURLE_CONV_REQD: CURLcode = 76;
pub const CURLE_CONV_FAILED: CURLcode = 75;
pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
pub const CURLE_TFTP_PERM: CURLcode = 69;
pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
pub const CURLE_LOGIN_DENIED: CURLcode = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
pub const CURLE_SSL_CIPHER: CURLcode = 59;
pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
pub const CURLE_OBSOLETE57: CURLcode = 57;
pub const CURLE_RECV_ERROR: CURLcode = 56;
pub const CURLE_SEND_ERROR: CURLcode = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
pub const CURLE_GOT_NOTHING: CURLcode = 52;
pub const CURLE_OBSOLETE51: CURLcode = 51;
pub const CURLE_OBSOLETE50: CURLcode = 50;
pub const CURLE_SETOPT_OPTION_SYNTAX: CURLcode = 49;
pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
pub const CURLE_OBSOLETE46: CURLcode = 46;
pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
pub const CURLE_OBSOLETE44: CURLcode = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
pub const CURLE_OBSOLETE40: CURLcode = 40;
pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
pub const CURLE_RANGE_ERROR: CURLcode = 33;
pub const CURLE_OBSOLETE32: CURLcode = 32;
pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
pub const CURLE_OBSOLETE29: CURLcode = 29;
pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
pub const CURLE_READ_ERROR: CURLcode = 26;
pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
pub const CURLE_OBSOLETE24: CURLcode = 24;
pub const CURLE_WRITE_ERROR: CURLcode = 23;
pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
pub const CURLE_QUOTE_ERROR: CURLcode = 21;
pub const CURLE_OBSOLETE20: CURLcode = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
pub const CURLE_PARTIAL_FILE: CURLcode = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
pub const CURLE_HTTP2: CURLcode = 16;
pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
pub const CURLE_URL_MALFORMAT: CURLcode = 3;
pub const CURLE_FAILED_INIT: CURLcode = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
pub const CURLE_OK: CURLcode = 0;
pub type CURLoption = libc::c_uint;
pub const CURLOPT_LASTENTRY: CURLoption = 40311;
pub const CURLOPT_PROXY_CAINFO_BLOB: CURLoption = 40310;
pub const CURLOPT_CAINFO_BLOB: CURLoption = 40309;
pub const CURLOPT_DOH_SSL_VERIFYSTATUS: CURLoption = 308;
pub const CURLOPT_DOH_SSL_VERIFYHOST: CURLoption = 307;
pub const CURLOPT_DOH_SSL_VERIFYPEER: CURLoption = 306;
pub const CURLOPT_AWS_SIGV4: CURLoption = 10305;
pub const CURLOPT_HSTSWRITEDATA: CURLoption = 10304;
pub const CURLOPT_HSTSWRITEFUNCTION: CURLoption = 20303;
pub const CURLOPT_HSTSREADDATA: CURLoption = 10302;
pub const CURLOPT_HSTSREADFUNCTION: CURLoption = 20301;
pub const CURLOPT_HSTS: CURLoption = 10300;
pub const CURLOPT_HSTS_CTRL: CURLoption = 299;
pub const CURLOPT_SSL_EC_CURVES: CURLoption = 10298;
pub const CURLOPT_PROXY_ISSUERCERT_BLOB: CURLoption = 40297;
pub const CURLOPT_PROXY_ISSUERCERT: CURLoption = 10296;
pub const CURLOPT_ISSUERCERT_BLOB: CURLoption = 40295;
pub const CURLOPT_PROXY_SSLKEY_BLOB: CURLoption = 40294;
pub const CURLOPT_PROXY_SSLCERT_BLOB: CURLoption = 40293;
pub const CURLOPT_SSLKEY_BLOB: CURLoption = 40292;
pub const CURLOPT_SSLCERT_BLOB: CURLoption = 40291;
pub const CURLOPT_MAIL_RCPT_ALLLOWFAILS: CURLoption = 290;
pub const CURLOPT_SASL_AUTHZID: CURLoption = 10289;
pub const CURLOPT_MAXAGE_CONN: CURLoption = 288;
pub const CURLOPT_ALTSVC: CURLoption = 10287;
pub const CURLOPT_ALTSVC_CTRL: CURLoption = 286;
pub const CURLOPT_HTTP09_ALLOWED: CURLoption = 285;
pub const CURLOPT_TRAILERDATA: CURLoption = 10284;
pub const CURLOPT_TRAILERFUNCTION: CURLoption = 20283;
pub const CURLOPT_CURLU: CURLoption = 10282;
pub const CURLOPT_UPKEEP_INTERVAL_MS: CURLoption = 281;
pub const CURLOPT_UPLOAD_BUFFERSIZE: CURLoption = 280;
pub const CURLOPT_DOH_URL: CURLoption = 10279;
pub const CURLOPT_DISALLOW_USERNAME_IN_URL: CURLoption = 278;
pub const CURLOPT_PROXY_TLS13_CIPHERS: CURLoption = 10277;
pub const CURLOPT_TLS13_CIPHERS: CURLoption = 10276;
pub const CURLOPT_DNS_SHUFFLE_ADDRESSES: CURLoption = 275;
pub const CURLOPT_HAPROXYPROTOCOL: CURLoption = 274;
pub const CURLOPT_RESOLVER_START_DATA: CURLoption = 10273;
pub const CURLOPT_RESOLVER_START_FUNCTION: CURLoption = 20272;
pub const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: CURLoption = 271;
pub const CURLOPT_TIMEVALUE_LARGE: CURLoption = 30270;
pub const CURLOPT_MIMEPOST: CURLoption = 10269;
pub const CURLOPT_SSH_COMPRESSION: CURLoption = 268;
pub const CURLOPT_SOCKS5_AUTH: CURLoption = 267;
pub const CURLOPT_REQUEST_TARGET: CURLoption = 10266;
pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: CURLoption = 265;
pub const CURLOPT_ABSTRACT_UNIX_SOCKET: CURLoption = 10264;
pub const CURLOPT_PROXY_PINNEDPUBLICKEY: CURLoption = 10263;
pub const CURLOPT_PRE_PROXY: CURLoption = 10262;
pub const CURLOPT_PROXY_SSL_OPTIONS: CURLoption = 261;
pub const CURLOPT_PROXY_CRLFILE: CURLoption = 10260;
pub const CURLOPT_PROXY_SSL_CIPHER_LIST: CURLoption = 10259;
pub const CURLOPT_PROXY_KEYPASSWD: CURLoption = 10258;
pub const CURLOPT_PROXY_SSLKEYTYPE: CURLoption = 10257;
pub const CURLOPT_PROXY_SSLKEY: CURLoption = 10256;
pub const CURLOPT_PROXY_SSLCERTTYPE: CURLoption = 10255;
pub const CURLOPT_PROXY_SSLCERT: CURLoption = 10254;
pub const CURLOPT_PROXY_TLSAUTH_TYPE: CURLoption = 10253;
pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: CURLoption = 10252;
pub const CURLOPT_PROXY_TLSAUTH_USERNAME: CURLoption = 10251;
pub const CURLOPT_PROXY_SSLVERSION: CURLoption = 250;
pub const CURLOPT_PROXY_SSL_VERIFYHOST: CURLoption = 249;
pub const CURLOPT_PROXY_SSL_VERIFYPEER: CURLoption = 248;
pub const CURLOPT_PROXY_CAPATH: CURLoption = 10247;
pub const CURLOPT_PROXY_CAINFO: CURLoption = 10246;
pub const CURLOPT_KEEP_SENDING_ON_ERROR: CURLoption = 245;
pub const CURLOPT_TCP_FASTOPEN: CURLoption = 244;
pub const CURLOPT_CONNECT_TO: CURLoption = 10243;
pub const CURLOPT_TFTP_NO_OPTIONS: CURLoption = 242;
pub const CURLOPT_STREAM_DEPENDS_E: CURLoption = 10241;
pub const CURLOPT_STREAM_DEPENDS: CURLoption = 10240;
pub const CURLOPT_STREAM_WEIGHT: CURLoption = 239;
pub const CURLOPT_DEFAULT_PROTOCOL: CURLoption = 10238;
pub const CURLOPT_PIPEWAIT: CURLoption = 237;
pub const CURLOPT_SERVICE_NAME: CURLoption = 10236;
pub const CURLOPT_PROXY_SERVICE_NAME: CURLoption = 10235;
pub const CURLOPT_PATH_AS_IS: CURLoption = 234;
pub const CURLOPT_SSL_FALSESTART: CURLoption = 233;
pub const CURLOPT_SSL_VERIFYSTATUS: CURLoption = 232;
pub const CURLOPT_UNIX_SOCKET_PATH: CURLoption = 10231;
pub const CURLOPT_PINNEDPUBLICKEY: CURLoption = 10230;
pub const CURLOPT_HEADEROPT: CURLoption = 229;
pub const CURLOPT_PROXYHEADER: CURLoption = 10228;
pub const CURLOPT_EXPECT_100_TIMEOUT_MS: CURLoption = 227;
pub const CURLOPT_SSL_ENABLE_ALPN: CURLoption = 226;
pub const CURLOPT_SSL_ENABLE_NPN: CURLoption = 225;
pub const CURLOPT_LOGIN_OPTIONS: CURLoption = 10224;
pub const CURLOPT_DNS_LOCAL_IP6: CURLoption = 10223;
pub const CURLOPT_DNS_LOCAL_IP4: CURLoption = 10222;
pub const CURLOPT_DNS_INTERFACE: CURLoption = 10221;
pub const CURLOPT_XOAUTH2_BEARER: CURLoption = 10220;
pub const CURLOPT_XFERINFOFUNCTION: CURLoption = 20219;
pub const CURLOPT_SASL_IR: CURLoption = 218;
pub const CURLOPT_MAIL_AUTH: CURLoption = 10217;
pub const CURLOPT_SSL_OPTIONS: CURLoption = 216;
pub const CURLOPT_TCP_KEEPINTVL: CURLoption = 215;
pub const CURLOPT_TCP_KEEPIDLE: CURLoption = 214;
pub const CURLOPT_TCP_KEEPALIVE: CURLoption = 213;
pub const CURLOPT_ACCEPTTIMEOUT_MS: CURLoption = 212;
pub const CURLOPT_DNS_SERVERS: CURLoption = 10211;
pub const CURLOPT_GSSAPI_DELEGATION: CURLoption = 210;
pub const CURLOPT_CLOSESOCKETDATA: CURLoption = 10209;
pub const CURLOPT_CLOSESOCKETFUNCTION: CURLoption = 20208;
pub const CURLOPT_TRANSFER_ENCODING: CURLoption = 207;
pub const CURLOPT_TLSAUTH_TYPE: CURLoption = 10206;
pub const CURLOPT_TLSAUTH_PASSWORD: CURLoption = 10205;
pub const CURLOPT_TLSAUTH_USERNAME: CURLoption = 10204;
pub const CURLOPT_RESOLVE: CURLoption = 10203;
pub const CURLOPT_FNMATCH_DATA: CURLoption = 10202;
pub const CURLOPT_CHUNK_DATA: CURLoption = 10201;
pub const CURLOPT_FNMATCH_FUNCTION: CURLoption = 20200;
pub const CURLOPT_CHUNK_END_FUNCTION: CURLoption = 20199;
pub const CURLOPT_CHUNK_BGN_FUNCTION: CURLoption = 20198;
pub const CURLOPT_WILDCARDMATCH: CURLoption = 197;
pub const CURLOPT_INTERLEAVEFUNCTION: CURLoption = 20196;
pub const CURLOPT_INTERLEAVEDATA: CURLoption = 10195;
pub const CURLOPT_RTSP_SERVER_CSEQ: CURLoption = 194;
pub const CURLOPT_RTSP_CLIENT_CSEQ: CURLoption = 193;
pub const CURLOPT_RTSP_TRANSPORT: CURLoption = 10192;
pub const CURLOPT_RTSP_STREAM_URI: CURLoption = 10191;
pub const CURLOPT_RTSP_SESSION_ID: CURLoption = 10190;
pub const CURLOPT_RTSP_REQUEST: CURLoption = 189;
pub const CURLOPT_FTP_USE_PRET: CURLoption = 188;
pub const CURLOPT_MAIL_RCPT: CURLoption = 10187;
pub const CURLOPT_MAIL_FROM: CURLoption = 10186;
pub const CURLOPT_SSH_KEYDATA: CURLoption = 10185;
pub const CURLOPT_SSH_KEYFUNCTION: CURLoption = 20184;
pub const CURLOPT_SSH_KNOWNHOSTS: CURLoption = 10183;
pub const CURLOPT_REDIR_PROTOCOLS: CURLoption = 182;
pub const CURLOPT_PROTOCOLS: CURLoption = 181;
pub const CURLOPT_SOCKS5_GSSAPI_NEC: CURLoption = 180;
pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: CURLoption = 10179;
pub const CURLOPT_TFTP_BLKSIZE: CURLoption = 178;
pub const CURLOPT_NOPROXY: CURLoption = 10177;
pub const CURLOPT_PROXYPASSWORD: CURLoption = 10176;
pub const CURLOPT_PROXYUSERNAME: CURLoption = 10175;
pub const CURLOPT_PASSWORD: CURLoption = 10174;
pub const CURLOPT_USERNAME: CURLoption = 10173;
pub const CURLOPT_CERTINFO: CURLoption = 172;
pub const CURLOPT_ADDRESS_SCOPE: CURLoption = 171;
pub const CURLOPT_ISSUERCERT: CURLoption = 10170;
pub const CURLOPT_CRLFILE: CURLoption = 10169;
pub const CURLOPT_SEEKDATA: CURLoption = 10168;
pub const CURLOPT_SEEKFUNCTION: CURLoption = 20167;
pub const CURLOPT_PROXY_TRANSFER_MODE: CURLoption = 166;
pub const CURLOPT_COPYPOSTFIELDS: CURLoption = 10165;
pub const CURLOPT_OPENSOCKETDATA: CURLoption = 10164;
pub const CURLOPT_OPENSOCKETFUNCTION: CURLoption = 20163;
pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: CURLoption = 10162;
pub const CURLOPT_POSTREDIR: CURLoption = 161;
pub const CURLOPT_NEW_DIRECTORY_PERMS: CURLoption = 160;
pub const CURLOPT_NEW_FILE_PERMS: CURLoption = 159;
pub const CURLOPT_HTTP_CONTENT_DECODING: CURLoption = 158;
pub const CURLOPT_HTTP_TRANSFER_DECODING: CURLoption = 157;
pub const CURLOPT_CONNECTTIMEOUT_MS: CURLoption = 156;
pub const CURLOPT_TIMEOUT_MS: CURLoption = 155;
pub const CURLOPT_FTP_SSL_CCC: CURLoption = 154;
pub const CURLOPT_SSH_PRIVATE_KEYFILE: CURLoption = 10153;
pub const CURLOPT_SSH_PUBLIC_KEYFILE: CURLoption = 10152;
pub const CURLOPT_SSH_AUTH_TYPES: CURLoption = 151;
pub const CURLOPT_SSL_SESSIONID_CACHE: CURLoption = 150;
pub const CURLOPT_SOCKOPTDATA: CURLoption = 10149;
pub const CURLOPT_SOCKOPTFUNCTION: CURLoption = 20148;
pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: CURLoption = 10147;
pub const CURLOPT_MAX_RECV_SPEED_LARGE: CURLoption = 30146;
pub const CURLOPT_MAX_SEND_SPEED_LARGE: CURLoption = 30145;
pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: CURLoption = 20144;
pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: CURLoption = 20143;
pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: CURLoption = 20142;
pub const CURLOPT_CONNECT_ONLY: CURLoption = 141;
pub const CURLOPT_LOCALPORTRANGE: CURLoption = 140;
pub const CURLOPT_LOCALPORT: CURLoption = 139;
pub const CURLOPT_FTP_FILEMETHOD: CURLoption = 138;
pub const CURLOPT_FTP_SKIP_PASV_IP: CURLoption = 137;
pub const CURLOPT_IGNORE_CONTENT_LENGTH: CURLoption = 136;
pub const CURLOPT_COOKIELIST: CURLoption = 10135;
pub const CURLOPT_FTP_ACCOUNT: CURLoption = 10134;
pub const CURLOPT_IOCTLDATA: CURLoption = 10131;
pub const CURLOPT_IOCTLFUNCTION: CURLoption = 20130;
pub const CURLOPT_FTPSSLAUTH: CURLoption = 129;
pub const CURLOPT_TCP_NODELAY: CURLoption = 121;
pub const CURLOPT_POSTFIELDSIZE_LARGE: CURLoption = 30120;
pub const CURLOPT_USE_SSL: CURLoption = 119;
pub const CURLOPT_NETRC_FILE: CURLoption = 10118;
pub const CURLOPT_MAXFILESIZE_LARGE: CURLoption = 30117;
pub const CURLOPT_RESUME_FROM_LARGE: CURLoption = 30116;
pub const CURLOPT_INFILESIZE_LARGE: CURLoption = 30115;
pub const CURLOPT_MAXFILESIZE: CURLoption = 114;
pub const CURLOPT_IPRESOLVE: CURLoption = 113;
pub const CURLOPT_FTP_RESPONSE_TIMEOUT: CURLoption = 112;
pub const CURLOPT_PROXYAUTH: CURLoption = 111;
pub const CURLOPT_FTP_CREATE_MISSING_DIRS: CURLoption = 110;
pub const CURLOPT_SSL_CTX_DATA: CURLoption = 10109;
pub const CURLOPT_SSL_CTX_FUNCTION: CURLoption = 20108;
pub const CURLOPT_HTTPAUTH: CURLoption = 107;
pub const CURLOPT_FTP_USE_EPRT: CURLoption = 106;
pub const CURLOPT_UNRESTRICTED_AUTH: CURLoption = 105;
pub const CURLOPT_HTTP200ALIASES: CURLoption = 10104;
pub const CURLOPT_PRIVATE: CURLoption = 10103;
pub const CURLOPT_ACCEPT_ENCODING: CURLoption = 10102;
pub const CURLOPT_PROXYTYPE: CURLoption = 101;
pub const CURLOPT_SHARE: CURLoption = 10100;
pub const CURLOPT_NOSIGNAL: CURLoption = 99;
pub const CURLOPT_BUFFERSIZE: CURLoption = 98;
pub const CURLOPT_CAPATH: CURLoption = 10097;
pub const CURLOPT_COOKIESESSION: CURLoption = 96;
pub const CURLOPT_DEBUGDATA: CURLoption = 10095;
pub const CURLOPT_DEBUGFUNCTION: CURLoption = 20094;
pub const CURLOPT_PREQUOTE: CURLoption = 10093;
pub const CURLOPT_DNS_CACHE_TIMEOUT: CURLoption = 92;
pub const CURLOPT_DNS_USE_GLOBAL_CACHE: CURLoption = 91;
pub const CURLOPT_SSLENGINE_DEFAULT: CURLoption = 90;
pub const CURLOPT_SSLENGINE: CURLoption = 10089;
pub const CURLOPT_SSLKEYTYPE: CURLoption = 10088;
pub const CURLOPT_SSLKEY: CURLoption = 10087;
pub const CURLOPT_SSLCERTTYPE: CURLoption = 10086;
pub const CURLOPT_FTP_USE_EPSV: CURLoption = 85;
pub const CURLOPT_HTTP_VERSION: CURLoption = 84;
pub const CURLOPT_SSL_CIPHER_LIST: CURLoption = 10083;
pub const CURLOPT_COOKIEJAR: CURLoption = 10082;
pub const CURLOPT_SSL_VERIFYHOST: CURLoption = 81;
pub const CURLOPT_HTTPGET: CURLoption = 80;
pub const CURLOPT_HEADERFUNCTION: CURLoption = 20079;
pub const CURLOPT_CONNECTTIMEOUT: CURLoption = 78;
pub const CURLOPT_EGDSOCKET: CURLoption = 10077;
pub const CURLOPT_RANDOM_FILE: CURLoption = 10076;
pub const CURLOPT_FORBID_REUSE: CURLoption = 75;
pub const CURLOPT_FRESH_CONNECT: CURLoption = 74;
pub const CURLOPT_OBSOLETE72: CURLoption = 72;
pub const CURLOPT_MAXCONNECTS: CURLoption = 71;
pub const CURLOPT_TELNETOPTIONS: CURLoption = 10070;
pub const CURLOPT_FILETIME: CURLoption = 69;
pub const CURLOPT_MAXREDIRS: CURLoption = 68;
pub const CURLOPT_CAINFO: CURLoption = 10065;
pub const CURLOPT_SSL_VERIFYPEER: CURLoption = 64;
pub const CURLOPT_KRBLEVEL: CURLoption = 10063;
pub const CURLOPT_INTERFACE: CURLoption = 10062;
pub const CURLOPT_HTTPPROXYTUNNEL: CURLoption = 61;
pub const CURLOPT_POSTFIELDSIZE: CURLoption = 60;
pub const CURLOPT_PROXYPORT: CURLoption = 59;
pub const CURLOPT_AUTOREFERER: CURLoption = 58;
pub const CURLOPT_XFERINFODATA: CURLoption = 10057;
pub const CURLOPT_PROGRESSFUNCTION: CURLoption = 20056;
pub const CURLOPT_PUT: CURLoption = 54;
pub const CURLOPT_TRANSFERTEXT: CURLoption = 53;
pub const CURLOPT_FOLLOWLOCATION: CURLoption = 52;
pub const CURLOPT_NETRC: CURLoption = 51;
pub const CURLOPT_APPEND: CURLoption = 50;
pub const CURLOPT_DIRLISTONLY: CURLoption = 48;
pub const CURLOPT_POST: CURLoption = 47;
pub const CURLOPT_UPLOAD: CURLoption = 46;
pub const CURLOPT_FAILONERROR: CURLoption = 45;
pub const CURLOPT_NOBODY: CURLoption = 44;
pub const CURLOPT_NOPROGRESS: CURLoption = 43;
pub const CURLOPT_HEADER: CURLoption = 42;
pub const CURLOPT_VERBOSE: CURLoption = 41;
pub const CURLOPT_OBSOLETE40: CURLoption = 10040;
pub const CURLOPT_POSTQUOTE: CURLoption = 10039;
pub const CURLOPT_STDERR: CURLoption = 10037;
pub const CURLOPT_CUSTOMREQUEST: CURLoption = 10036;
pub const CURLOPT_TIMEVALUE: CURLoption = 34;
pub const CURLOPT_TIMECONDITION: CURLoption = 33;
pub const CURLOPT_SSLVERSION: CURLoption = 32;
pub const CURLOPT_COOKIEFILE: CURLoption = 10031;
pub const CURLOPT_HEADERDATA: CURLoption = 10029;
pub const CURLOPT_QUOTE: CURLoption = 10028;
pub const CURLOPT_CRLF: CURLoption = 27;
pub const CURLOPT_KEYPASSWD: CURLoption = 10026;
pub const CURLOPT_SSLCERT: CURLoption = 10025;
pub const CURLOPT_HTTPPOST: CURLoption = 10024;
pub const CURLOPT_HTTPHEADER: CURLoption = 10023;
pub const CURLOPT_COOKIE: CURLoption = 10022;
pub const CURLOPT_RESUME_FROM: CURLoption = 21;
pub const CURLOPT_LOW_SPEED_TIME: CURLoption = 20;
pub const CURLOPT_LOW_SPEED_LIMIT: CURLoption = 19;
pub const CURLOPT_USERAGENT: CURLoption = 10018;
pub const CURLOPT_FTPPORT: CURLoption = 10017;
pub const CURLOPT_REFERER: CURLoption = 10016;
pub const CURLOPT_POSTFIELDS: CURLoption = 10015;
pub const CURLOPT_INFILESIZE: CURLoption = 14;
pub const CURLOPT_TIMEOUT: CURLoption = 13;
pub const CURLOPT_READFUNCTION: CURLoption = 20012;
pub const CURLOPT_WRITEFUNCTION: CURLoption = 20011;
pub const CURLOPT_ERRORBUFFER: CURLoption = 10010;
pub const CURLOPT_READDATA: CURLoption = 10009;
pub const CURLOPT_RANGE: CURLoption = 10007;
pub const CURLOPT_PROXYUSERPWD: CURLoption = 10006;
pub const CURLOPT_USERPWD: CURLoption = 10005;
pub const CURLOPT_PROXY: CURLoption = 10004;
pub const CURLOPT_PORT: CURLoption = 3;
pub const CURLOPT_URL: CURLoption = 10002;
pub const CURLOPT_WRITEDATA: CURLoption = 10001;
pub type CURLINFO = libc::c_uint;
pub const CURLINFO_LASTONE: CURLINFO = 60;
pub const CURLINFO_REFERER: CURLINFO = 1048636;
pub const CURLINFO_PROXY_ERROR: CURLINFO = 2097211;
pub const CURLINFO_EFFECTIVE_METHOD: CURLINFO = 1048634;
pub const CURLINFO_RETRY_AFTER: CURLINFO = 6291513;
pub const CURLINFO_APPCONNECT_TIME_T: CURLINFO = 6291512;
pub const CURLINFO_REDIRECT_TIME_T: CURLINFO = 6291511;
pub const CURLINFO_STARTTRANSFER_TIME_T: CURLINFO = 6291510;
pub const CURLINFO_PRETRANSFER_TIME_T: CURLINFO = 6291509;
pub const CURLINFO_CONNECT_TIME_T: CURLINFO = 6291508;
pub const CURLINFO_NAMELOOKUP_TIME_T: CURLINFO = 6291507;
pub const CURLINFO_TOTAL_TIME_T: CURLINFO = 6291506;
pub const CURLINFO_SCHEME: CURLINFO = 1048625;
pub const CURLINFO_PROTOCOL: CURLINFO = 2097200;
pub const CURLINFO_PROXY_SSL_VERIFYRESULT: CURLINFO = 2097199;
pub const CURLINFO_HTTP_VERSION: CURLINFO = 2097198;
pub const CURLINFO_TLS_SSL_PTR: CURLINFO = 4194349;
pub const CURLINFO_ACTIVESOCKET: CURLINFO = 5242924;
pub const CURLINFO_TLS_SESSION: CURLINFO = 4194347;
pub const CURLINFO_LOCAL_PORT: CURLINFO = 2097194;
pub const CURLINFO_LOCAL_IP: CURLINFO = 1048617;
pub const CURLINFO_PRIMARY_PORT: CURLINFO = 2097192;
pub const CURLINFO_RTSP_CSEQ_RECV: CURLINFO = 2097191;
pub const CURLINFO_RTSP_SERVER_CSEQ: CURLINFO = 2097190;
pub const CURLINFO_RTSP_CLIENT_CSEQ: CURLINFO = 2097189;
pub const CURLINFO_RTSP_SESSION_ID: CURLINFO = 1048612;
pub const CURLINFO_CONDITION_UNMET: CURLINFO = 2097187;
pub const CURLINFO_CERTINFO: CURLINFO = 4194338;
pub const CURLINFO_APPCONNECT_TIME: CURLINFO = 3145761;
pub const CURLINFO_PRIMARY_IP: CURLINFO = 1048608;
pub const CURLINFO_REDIRECT_URL: CURLINFO = 1048607;
pub const CURLINFO_FTP_ENTRY_PATH: CURLINFO = 1048606;
pub const CURLINFO_LASTSOCKET: CURLINFO = 2097181;
pub const CURLINFO_COOKIELIST: CURLINFO = 4194332;
pub const CURLINFO_SSL_ENGINES: CURLINFO = 4194331;
pub const CURLINFO_NUM_CONNECTS: CURLINFO = 2097178;
pub const CURLINFO_OS_ERRNO: CURLINFO = 2097177;
pub const CURLINFO_PROXYAUTH_AVAIL: CURLINFO = 2097176;
pub const CURLINFO_HTTPAUTH_AVAIL: CURLINFO = 2097175;
pub const CURLINFO_HTTP_CONNECTCODE: CURLINFO = 2097174;
pub const CURLINFO_PRIVATE: CURLINFO = 1048597;
pub const CURLINFO_REDIRECT_COUNT: CURLINFO = 2097172;
pub const CURLINFO_REDIRECT_TIME: CURLINFO = 3145747;
pub const CURLINFO_CONTENT_TYPE: CURLINFO = 1048594;
pub const CURLINFO_STARTTRANSFER_TIME: CURLINFO = 3145745;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD_T: CURLINFO = 6291472;
pub const CURLINFO_CONTENT_LENGTH_UPLOAD: CURLINFO = 3145744;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: CURLINFO = 6291471;
pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: CURLINFO = 3145743;
pub const CURLINFO_FILETIME_T: CURLINFO = 6291470;
pub const CURLINFO_FILETIME: CURLINFO = 2097166;
pub const CURLINFO_SSL_VERIFYRESULT: CURLINFO = 2097165;
pub const CURLINFO_REQUEST_SIZE: CURLINFO = 2097164;
pub const CURLINFO_HEADER_SIZE: CURLINFO = 2097163;
pub const CURLINFO_SPEED_UPLOAD_T: CURLINFO = 6291466;
pub const CURLINFO_SPEED_UPLOAD: CURLINFO = 3145738;
pub const CURLINFO_SPEED_DOWNLOAD_T: CURLINFO = 6291465;
pub const CURLINFO_SPEED_DOWNLOAD: CURLINFO = 3145737;
pub const CURLINFO_SIZE_DOWNLOAD_T: CURLINFO = 6291464;
pub const CURLINFO_SIZE_DOWNLOAD: CURLINFO = 3145736;
pub const CURLINFO_SIZE_UPLOAD_T: CURLINFO = 6291463;
pub const CURLINFO_SIZE_UPLOAD: CURLINFO = 3145735;
pub const CURLINFO_PRETRANSFER_TIME: CURLINFO = 3145734;
pub const CURLINFO_CONNECT_TIME: CURLINFO = 3145733;
pub const CURLINFO_NAMELOOKUP_TIME: CURLINFO = 3145732;
pub const CURLINFO_TOTAL_TIME: CURLINFO = 3145731;
pub const CURLINFO_RESPONSE_CODE: CURLINFO = 2097154;
pub const CURLINFO_EFFECTIVE_URL: CURLINFO = 1048577;
pub const CURLINFO_NONE: CURLINFO = 0;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type boolean = int32_t;
pub type alerttype_t = libc::c_uint;
pub const CONS_ERROR: alerttype_t = 2;
pub const CONS_WARNING: alerttype_t = 1;
pub const CONS_NOTICE: alerttype_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union msg_header_t {
    pub buffer: [libc::c_char; 16],
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct msg_server_t {
    pub header: msg_header_t,
    pub ip: [libc::c_char; 40],
    pub port: [libc::c_char; 8],
    pub name: [libc::c_char; 32],
    pub room: int32_t,
    pub version: [libc::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct msg_rooms_t {
    pub header: msg_header_t,
    pub id: int32_t,
    pub name: [libc::c_char; 32],
    pub motd: [libc::c_char; 255],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HMS_buffer {
    pub curl: *mut libc::c_void,
    pub buffer: *mut libc::c_char,
    pub needle: libc::c_int,
    pub end: libc::c_int,
}
pub type menuitem_t = menuitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuitem_s {
    pub status: uint16_t,
    pub patch: *const libc::c_char,
    pub text: *const libc::c_char,
    pub itemaction: *mut libc::c_void,
    pub alphaKey: uint16_t,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut cv_masterserver_timeout: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"masterserver_timeout\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"5\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int,
            PossibleValue: CV_Unsigned.as_ptr() as *mut _,
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
#[no_mangle]
pub static mut cv_masterserver_debug: consvar_t = unsafe {
    {
        let mut init = consvar_s {
            name: b"masterserver_debug\0" as *const u8 as *const libc::c_char,
            defaultvalue: b"Off\0" as *const u8 as *const libc::c_char,
            flags: CV_SAVE as libc::c_int | CV_CALL as libc::c_int,
            PossibleValue: CV_OnOff.as_ptr() as *mut _,
            func: Some(MasterServer_Debug_OnChange as unsafe extern "C" fn() -> ()),
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
#[no_mangle]
pub static mut cv_masterserver_token: consvar_t = {
    let mut init = consvar_s {
        name: b"masterserver_token\0" as *const u8 as *const libc::c_char,
        defaultvalue: b"\0" as *const u8 as *const libc::c_char,
        flags: CV_SAVE as libc::c_int,
        PossibleValue: 0 as *const CV_PossibleValue_t as *mut CV_PossibleValue_t,
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
};
static mut hms_started: libc::c_int = 0;
static mut hms_api: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut hms_server_token: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut hms_useragent: [libc::c_char; 512] = [0; 512];
unsafe extern "C" fn Contact_error() {
    CONS_Alert(
        CONS_ERROR,
        b"There was a problem contacting the master server...\n\0" as *const u8
            as *const libc::c_char,
    );
}
unsafe extern "C" fn get_user_agent(mut buf: *mut libc::c_char, mut len: size_t) {
    if snprintf(
        buf,
        len,
        b"%s/%s (%s; %s; %i; %i) SRB2BASE/%i\0" as *const u8 as *const libc::c_char,
        b"SRB2\0" as *const u8 as *const libc::c_char,
        b"v2.2.13\0" as *const u8 as *const libc::c_char,
        compbranch,
        comprevision,
        18 as libc::c_int,
        54 as libc::c_int,
        220 as libc::c_int,
    ) < 0 as libc::c_int
    {
        I_Error(
            b"http-mserv: get_user_agent failed\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn init_user_agent_once() {
    if hms_useragent[0 as libc::c_int as usize] as libc::c_int != '\0' as i32 {
        return;
    }
    get_user_agent(hms_useragent.as_mut_ptr(), 512 as libc::c_int as size_t);
}
unsafe extern "C" fn HMS_on_read(
    mut s: *mut libc::c_char,
    mut _1: size_t,
    mut n: size_t,
    mut userdata: *mut libc::c_void,
) -> size_t {
    let mut buffer: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut blocks: size_t = 0;
    buffer = userdata as *mut HMS_buffer;
    if n >= ((*buffer).end - (*buffer).needle) as size_t {
        blocks = (n / 4096 as libc::c_int as size_t)
            .wrapping_add(1 as libc::c_int as size_t);
        (*buffer)
            .end = ((*buffer).end as size_t)
            .wrapping_add(blocks * 4096 as libc::c_int as size_t) as libc::c_int
            as libc::c_int;
        (*buffer)
            .buffer = realloc(
            (*buffer).buffer as *mut libc::c_void,
            (*buffer).end as libc::c_ulong,
        ) as *mut libc::c_char;
    }
    memcpy(
        &mut *((*buffer).buffer).offset((*buffer).needle as isize) as *mut libc::c_char
            as *mut libc::c_void,
        s as *const libc::c_void,
        n,
    );
    (*buffer)
        .needle = ((*buffer).needle as size_t).wrapping_add(n) as libc::c_int
        as libc::c_int;
    return n;
}
unsafe extern "C" fn HMS_connect(
    mut format: *const libc::c_char,
    mut args: ...
) -> *mut HMS_buffer {
    let mut ap: ::core::ffi::VaListImpl;
    let mut curl: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut url: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quack_token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut seek: size_t = 0;
    let mut token_length: size_t = 0;
    let mut buffer: *mut HMS_buffer = 0 as *mut HMS_buffer;
    if hms_started == 0 {
        if curl_global_init(
            ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_long,
        ) as libc::c_uint != 0 as libc::c_int as libc::c_uint
        {
            Contact_error();
            CONS_Printf(
                b"\x85From curl_global_init.\n\0" as *const u8 as *const libc::c_char,
            );
            return 0 as *mut HMS_buffer;
        } else {
            atexit(Some(curl_global_cleanup as unsafe extern "C" fn() -> ()));
            hms_started = 1 as libc::c_int;
        }
    }
    curl = curl_easy_init();
    if curl.is_null() {
        Contact_error();
        CONS_Printf(b"\x85From curl_easy_init.\n\0" as *const u8 as *const libc::c_char);
        return 0 as *mut HMS_buffer;
    }
    if !(cv_masterserver_token.string).is_null()
        && *(cv_masterserver_token.string).offset(0 as libc::c_int as isize)
            as libc::c_int != 0
    {
        quack_token = curl_easy_escape(
            curl,
            cv_masterserver_token.string,
            0 as libc::c_int,
        );
        token_length = (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(strlen(quack_token));
    } else {
        quack_token = 0 as *mut libc::c_char;
        token_length = 0 as libc::c_int as size_t;
    }
    init_user_agent_once();
    seek = (strlen(hms_api)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    ap = args.clone();
    url = malloc(
        seek
            .wrapping_add(
                vsnprintf(
                    0 as *mut libc::c_char,
                    0 as libc::c_int as libc::c_ulong,
                    format,
                    ap.as_va_list(),
                ) as size_t,
            )
            .wrapping_add(token_length)
            .wrapping_add(1 as libc::c_int as size_t),
    ) as *mut libc::c_char;
    sprintf(url, b"%s/\0" as *const u8 as *const libc::c_char, hms_api);
    ap = args.clone();
    seek = seek
        .wrapping_add(
            vsprintf(&mut *url.offset(seek as isize), format, ap.as_va_list()) as size_t,
        );
    if !quack_token.is_null() {
        sprintf(
            &mut *url.offset(seek as isize) as *mut libc::c_char,
            b"?token=%s\0" as *const u8 as *const libc::c_char,
            quack_token,
        );
    }
    CONS_Printf(b"HMS: connecting '%s'...\n\0" as *const u8 as *const libc::c_char, url);
    buffer = malloc(::core::mem::size_of::<HMS_buffer>() as libc::c_ulong)
        as *mut HMS_buffer;
    (*buffer).curl = curl;
    (*buffer).end = 4096 as libc::c_int;
    (*buffer).buffer = malloc((*buffer).end as libc::c_ulong) as *mut libc::c_char;
    (*buffer).needle = 0 as libc::c_int;
    if cv_masterserver_debug.value != 0 {
        curl_easy_setopt(curl, CURLOPT_VERBOSE, 1 as libc::c_long);
        curl_easy_setopt(curl, CURLOPT_STDERR, logstream);
    }
    if M_CheckParm(b"-bindaddr\0" as *const u8 as *const libc::c_char) != 0
        && M_IsNextParm() != 0
    {
        curl_easy_setopt(curl, CURLOPT_INTERFACE, M_GetNextParm());
    }
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1 as libc::c_long);
    if M_CheckParm(b"-noipv6\0" as *const u8 as *const libc::c_char) != 0 {
        curl_easy_setopt(curl, CURLOPT_IPRESOLVE, 1 as libc::c_int);
    }
    curl_easy_setopt(curl, CURLOPT_TIMEOUT, cv_masterserver_timeout.value);
    curl_easy_setopt(
        curl,
        CURLOPT_WRITEFUNCTION,
        Some(
            HMS_on_read
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    size_t,
                    size_t,
                    *mut libc::c_void,
                ) -> size_t,
        ),
    );
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, buffer);
    curl_easy_setopt(curl, CURLOPT_USERAGENT, hms_useragent.as_mut_ptr());
    curl_free(quack_token as *mut libc::c_void);
    free(url as *mut libc::c_void);
    return buffer;
}
unsafe extern "C" fn HMS_do(mut buffer: *mut HMS_buffer) -> libc::c_int {
    let mut cc: CURLcode = CURLE_OK;
    let mut status: libc::c_long = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    cc = curl_easy_perform((*buffer).curl);
    if cc as libc::c_uint != CURLE_OK as libc::c_int as libc::c_uint {
        Contact_error();
        CONS_Printf(
            b"\x85From curl_easy_perform: %s\n\0" as *const u8 as *const libc::c_char,
            curl_easy_strerror(cc),
        );
        return 0 as libc::c_int;
    }
    *((*buffer).buffer).offset((*buffer).needle as isize) = '\0' as i32 as libc::c_char;
    curl_easy_getinfo(
        (*buffer).curl,
        CURLINFO_RESPONSE_CODE,
        &mut status as *mut libc::c_long,
    );
    if status != 200 as libc::c_int as libc::c_long {
        p = strchr((*buffer).buffer, '\n' as i32);
        if !p.is_null() {
            *p = '\0' as i32 as libc::c_char;
        }
        Contact_error();
        CONS_Printf(
            b"\x85Master server error %ld: %s%s\n\0" as *const u8 as *const libc::c_char,
            status,
            (*buffer).buffer,
            if !p.is_null() {
                b"\0" as *const u8 as *const libc::c_char
            } else {
                b" (malformed)\0" as *const u8 as *const libc::c_char
            },
        );
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn HMS_end(mut buffer: *mut HMS_buffer) {
    curl_easy_cleanup((*buffer).curl);
    free((*buffer).buffer as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn HMS_fetch_rooms(
    mut joining: libc::c_int,
    mut query_id: libc::c_int,
) -> libc::c_int {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut ok: libc::c_int = 0;
    let mut doing_shit: libc::c_int = 0;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut room_motd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id_no: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    hms = HMS_connect(b"rooms\0" as *const u8 as *const libc::c_char);
    if hms.is_null() {
        return 0 as libc::c_int;
    }
    if HMS_do(hms) != 0 {
        doing_shit = 1 as libc::c_int;
        p = (*hms).buffer;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int
            && {
                end = strstr(p, b"\n\n\n\0" as *const u8 as *const libc::c_char);
                !end.is_null()
            }
        {
            *end = '\0' as i32 as libc::c_char;
            id = strtok(p, b"\n\0" as *const u8 as *const libc::c_char);
            title = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
            room_motd = strtok(
                0 as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if !(!id.is_null() && !title.is_null() && !room_motd.is_null()) {
                break;
            }
            id_no = atoi(id);
            if joining != 0 || id_no != 0 as libc::c_int {
                room_list[i as usize]
                    .header
                    .buffer[0 as libc::c_int
                    as usize] = 1 as libc::c_int as libc::c_char;
                room_list[i as usize].id = id_no;
                strlcpy(
                    (room_list[i as usize].name).as_mut_ptr(),
                    title,
                    ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                );
                strlcpy(
                    (room_list[i as usize].motd).as_mut_ptr(),
                    room_motd,
                    ::core::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
                );
                i += 1;
                i;
            }
            p = end.offset(3 as libc::c_int as isize);
        }
        if doing_shit != 0 {
            room_list[i as usize]
                .header
                .buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        }
        ok = 1 as libc::c_int;
        if doing_shit != 0 {
            i = 0 as libc::c_int;
            while room_list[i as usize].header.buffer[0 as libc::c_int as usize] != 0 {
                if *(room_list[i as usize].name).as_mut_ptr() as libc::c_int
                    != '\0' as i32
                {
                    let ref mut fresh0 = (*MP_RoomMenu
                        .as_mut_ptr()
                        .offset((i + 1 as libc::c_int) as isize))
                        .text;
                    *fresh0 = (room_list[i as usize].name).as_mut_ptr();
                    roomIds[i as usize] = room_list[i as usize].id as uint32_t;
                    (*MP_RoomMenu.as_mut_ptr().offset((i + 1 as libc::c_int) as isize))
                        .status = (32 as libc::c_int | 0 as libc::c_int) as uint16_t;
                }
                i += 1;
                i;
            }
        }
    } else {
        ok = 0 as libc::c_int;
    }
    HMS_end(hms);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_register() -> libc::c_int {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut ok: libc::c_int = 0;
    let mut post: [libc::c_char; 256] = [0; 256];
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    hms = HMS_connect(
        b"rooms/%d/register\0" as *const u8 as *const libc::c_char,
        ms_RoomId as libc::c_int,
    );
    if hms.is_null() {
        return 0 as libc::c_int;
    }
    title = curl_easy_escape((*hms).curl, cv_servername.string, 0 as libc::c_int);
    snprintf(
        post.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"port=%d&title=%s&version=%s\0" as *const u8 as *const libc::c_char,
        current_port as libc::c_int,
        title,
        b"2.2.13\0" as *const u8 as *const libc::c_char,
    );
    curl_free(title as *mut libc::c_void);
    curl_easy_setopt((*hms).curl, CURLOPT_POSTFIELDS, post.as_mut_ptr());
    ok = HMS_do(hms);
    if ok != 0 {
        hms_server_token = strdup(
            strtok((*hms).buffer, b"\n\0" as *const u8 as *const libc::c_char),
        );
    }
    HMS_end(hms);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_unlist() -> libc::c_int {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut ok: libc::c_int = 0;
    hms = HMS_connect(
        b"servers/%s/unlist\0" as *const u8 as *const libc::c_char,
        hms_server_token,
    );
    if hms.is_null() {
        return 0 as libc::c_int;
    }
    curl_easy_setopt(
        (*hms).curl,
        CURLOPT_CUSTOMREQUEST,
        b"POST\0" as *const u8 as *const libc::c_char,
    );
    ok = HMS_do(hms);
    HMS_end(hms);
    free(hms_server_token as *mut libc::c_void);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_update() -> libc::c_int {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut ok: libc::c_int = 0;
    let mut post: [libc::c_char; 256] = [0; 256];
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    hms = HMS_connect(
        b"servers/%s/update\0" as *const u8 as *const libc::c_char,
        hms_server_token,
    );
    if hms.is_null() {
        return 0 as libc::c_int;
    }
    title = curl_easy_escape((*hms).curl, cv_servername.string, 0 as libc::c_int);
    snprintf(
        post.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"title=%s\0" as *const u8 as *const libc::c_char,
        title,
    );
    curl_free(title as *mut libc::c_void);
    curl_easy_setopt((*hms).curl, CURLOPT_POSTFIELDS, post.as_mut_ptr());
    ok = HMS_do(hms);
    HMS_end(hms);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_list_servers() {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    hms = HMS_connect(b"servers\0" as *const u8 as *const libc::c_char);
    if hms.is_null() {
        return;
    }
    if HMS_do(hms) != 0 {
        list = curl_easy_unescape(
            (*hms).curl,
            (*hms).buffer,
            0 as libc::c_int,
            0 as *mut libc::c_int,
        );
        p = strtok(list, b"\n\0" as *const u8 as *const libc::c_char);
        while !p.is_null() {
            CONS_Printf(b"\x80%s\n\0" as *const u8 as *const libc::c_char, p);
            p = strtok(
                0 as *mut libc::c_char,
                b"\n\0" as *const u8 as *const libc::c_char,
            );
        }
        curl_free(list as *mut libc::c_void);
    }
    HMS_end(hms);
}
#[no_mangle]
pub unsafe extern "C" fn HMS_fetch_servers(
    mut list: *mut msg_server_t,
    mut room_number: libc::c_int,
    mut query_id: libc::c_int,
) -> *mut msg_server_t {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut doing_shit: libc::c_int = 0;
    let mut local_version: [libc::c_char; 9] = [0; 9];
    let mut room: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut address: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut title: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut section_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if room_number > 0 as libc::c_int {
        hms = HMS_connect(
            b"rooms/%d/servers\0" as *const u8 as *const libc::c_char,
            room_number,
        );
    } else {
        hms = HMS_connect(b"servers\0" as *const u8 as *const libc::c_char);
    }
    if hms.is_null() {
        return 0 as *mut msg_server_t;
    }
    if HMS_do(hms) != 0 {
        doing_shit = 1 as libc::c_int;
        snprintf(
            local_version.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            b"2.2.13\0" as *const u8 as *const libc::c_char,
        );
        p = (*hms).buffer;
        i = 0 as libc::c_int;
        loop {
            section_end = strstr(p, b"\n\n\0" as *const u8 as *const libc::c_char);
            room = strtok(p, b"\n\0" as *const u8 as *const libc::c_char);
            p = strtok(
                0 as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char,
            );
            if p.is_null() {
                break;
            }
            while i < 127 as libc::c_int - 1 as libc::c_int
                && {
                    end = strchr(p, '\n' as i32);
                    !end.is_null()
                }
            {
                *end = '\0' as i32 as libc::c_char;
                address = strtok(p, b" \0" as *const u8 as *const libc::c_char);
                port = strtok(
                    0 as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                title = strtok(
                    0 as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                );
                version = strtok(
                    0 as *mut libc::c_char,
                    b"\0" as *const u8 as *const libc::c_char,
                );
                if !address.is_null() && !port.is_null() && !title.is_null()
                    && !version.is_null()
                {
                    if strcmp(version, local_version.as_mut_ptr()) == 0 as libc::c_int {
                        strlcpy(
                            ((*list.offset(i as isize)).ip).as_mut_ptr(),
                            address,
                            ::core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
                        );
                        strlcpy(
                            ((*list.offset(i as isize)).port).as_mut_ptr(),
                            port,
                            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                        );
                        strlcpy(
                            ((*list.offset(i as isize)).name).as_mut_ptr(),
                            title,
                            ::core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                        );
                        strlcpy(
                            ((*list.offset(i as isize)).version).as_mut_ptr(),
                            version,
                            ::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                        );
                        (*list.offset(i as isize)).room = atoi(room);
                        (*list.offset(i as isize))
                            .header
                            .buffer[0 as libc::c_int
                            as usize] = 1 as libc::c_int as libc::c_char;
                        i += 1;
                        i;
                    }
                    if end == section_end {
                        break;
                    }
                    p = end.offset(1 as libc::c_int as isize);
                } else {
                    section_end = 0 as *mut libc::c_char;
                    break;
                }
            }
            if doing_shit == 0 {
                break;
            }
            p = section_end.offset(2 as libc::c_int as isize);
            if section_end.is_null() {
                break;
            }
        }
        if doing_shit != 0 {
            (*list.offset(i as isize))
                .header
                .buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        }
    } else {
        list = 0 as *mut msg_server_t;
    }
    HMS_end(hms);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_compare_mod_version(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
) -> libc::c_int {
    let mut hms: *mut HMS_buffer = 0 as *mut HMS_buffer;
    let mut ok: libc::c_int = 0;
    let mut version: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version_name: *mut libc::c_char = 0 as *mut libc::c_char;
    hms = HMS_connect(
        b"versions/%d\0" as *const u8 as *const libc::c_char,
        18 as libc::c_int,
    );
    if hms.is_null() {
        return 0 as libc::c_int;
    }
    ok = 0 as libc::c_int;
    if HMS_do(hms) != 0 {
        version = strtok((*hms).buffer, b" \0" as *const u8 as *const libc::c_char);
        version_name = strtok(
            0 as *mut libc::c_char,
            b"\n\0" as *const u8 as *const libc::c_char,
        );
        if !version.is_null() && !version_name.is_null() {
            if atoi(version) != 54 as libc::c_int {
                strlcpy(buffer, version_name, buffer_size);
                ok = 1 as libc::c_int;
            } else {
                ok = -(1 as libc::c_int);
            }
        }
    }
    HMS_end(hms);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn HMS_set_api(mut api: *mut libc::c_char) {
    free(hms_api as *mut libc::c_void);
    hms_api = api;
}
unsafe extern "C" fn MasterServer_Debug_OnChange() {
    if cv_masterserver_debug.value != 0 {
        CONS_Printf(
            b"Master server debug messages will appear in log.txt\n\0" as *const u8
                as *const libc::c_char,
        );
    }
}
