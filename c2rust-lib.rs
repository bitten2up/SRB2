#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(asm)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate f128;#[macro_use]
extern crate num_traits;
extern crate libc;
pub mod src {
pub mod am_map;
pub mod b_bot;
pub mod blua {
pub mod lbaselib;
pub mod liolib;
} // mod blua
pub mod command;
pub mod comptime;
pub mod console;
pub mod d_clisrv;
pub mod d_main;
pub mod d_net;
pub mod d_netcmd;
pub mod d_netfil;
pub mod deh_lua;
pub mod deh_soc;
pub mod deh_tables;
pub mod dehacked;
pub mod dummy {
pub mod i_system;
} // mod dummy
pub mod f_finale;
pub mod f_wipe;
pub mod filesrch;
pub mod g_demo;
pub mod g_game;
pub mod g_input;
pub mod http_mserv;
pub mod hu_stuff;
pub mod i_tcp;
pub mod i_time;
pub mod info;
pub mod lua_baselib;
pub mod lua_blockmaplib;
pub mod lua_consolelib;
pub mod lua_hooklib;
pub mod lua_hudlib;
pub mod lua_hudlib_drawlist;
pub mod lua_infolib;
pub mod lua_inputlib;
pub mod lua_maplib;
pub mod lua_mathlib;
pub mod lua_mobjlib;
pub mod lua_playerlib;
pub mod lua_polyobjlib;
pub mod lua_script;
pub mod lua_skinlib;
pub mod lua_taglib;
pub mod lua_thinkerlib;
pub mod m_anigif;
pub mod m_cheat;
pub mod m_cond;
pub mod m_menu;
pub mod m_misc;
pub mod m_perfstats;
pub mod m_random;
pub mod mserv;
pub mod p_ceilng;
pub mod p_enemy;
pub mod p_floor;
pub mod p_inter;
pub mod p_lights;
pub mod p_map;
pub mod p_maputl;
pub mod p_mobj;
pub mod p_polyobj;
pub mod p_saveg;
pub mod p_setup;
pub mod p_sight;
pub mod p_slopes;
pub mod p_spec;
pub mod p_telept;
pub mod p_tick;
pub mod p_user;
pub mod r_bbox;
pub mod r_bsp;
pub mod r_data;
pub mod r_draw;
pub mod r_fps;
pub mod r_main;
pub mod r_patch;
pub mod r_patchrotation;
pub mod r_picformats;
pub mod r_plane;
pub mod r_portal;
pub mod r_segs;
pub mod r_skins;
pub mod r_sky;
pub mod r_splats;
pub mod r_textures;
pub mod r_things;
pub mod s_sound;
pub mod screen;
pub mod sounds;
pub mod st_stuff;
pub mod taglist;
pub mod v_video;
pub mod w_wad;
pub mod y_inter;
pub mod z_zone;
} // mod src
