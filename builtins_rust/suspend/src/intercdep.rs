
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_desc {
//     pub word: *mut c_char,
//     pub flags: c_int,
// }
// pub type WordDesc = word_desc;

// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct word_list {
//     pub next: *mut word_list,
//     pub word: *mut WordDesc,
// }
// pub type WordList = word_list;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub type SigHandler = unsafe extern "C" fn(arg1: c_int);

extern "C" {
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn sh_nojobs(s: *mut c_char);
    pub fn no_args(list: *mut WordList);
    pub fn killpg(pgrp: libc::pid_t, sig: c_int) -> c_int;

    pub fn set_signal_handler(arg1: c_int, arg2: *mut SigHandler) -> *mut SigHandler;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;
    pub static mut job_control: c_int;
    pub static mut login_shell: c_int;
    pub static mut shell_pgrp: libc::pid_t;
}
