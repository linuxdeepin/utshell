
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

pub type SigHandler = unsafe extern "C" fn(arg1: c_int);

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub const DSIG_SIGPREFIX: c_int = 0x01;
pub const DSIG_NOCASE: c_int = 0x02;

pub const BASH_NSIG: c_int = (64 + 1) + 3;
pub const NO_SIG: c_int = -1;

pub const SET: c_int = 0;
pub const REVERT: c_int = 1;
pub const IGNORE: c_int = 2;

pub const SUBSHELL_RESETTRAP: c_int = 0x80;

extern "C" {
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn sh_chkwrite(s: c_int) -> c_int;
    pub fn display_signal_list(list: *mut WordList, forcecols: c_int) -> c_int;
    pub fn initialize_terminating_signals();
    pub fn get_all_original_signals();
    pub fn free_trap_strings();
    pub fn ignore_signal(sig: c_int);
    pub fn set_signal(sig: c_int, s: *mut c_char);
    pub fn restore_default_signal(sig: c_int);
    pub fn sigint_sighandler(sig: c_int);
    pub fn termsig_sighandler(sig: c_int);
    pub fn set_signal_handler(arg1: c_int, arg2: *mut SigHandler) -> *mut SigHandler;

    pub fn all_digits(s: *const c_char) -> c_int;
    pub fn decode_signal(s: *mut c_char, flags: c_int) -> c_int;

    pub fn signal_is_hard_ignored(sig: c_int) -> c_int;
    pub fn sh_single_quote(s: *const c_char) -> *mut c_char;
    pub fn signal_name(sig: c_int) -> *mut c_char;

    pub fn sh_invalidsig(s: *mut c_char);

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;

    pub static trap_list: [*mut c_char; BASH_NSIG as usize];
    pub static posixly_correct: c_int;
    pub static mut subshell_environment: c_int;
    pub static interactive: c_int;
    pub static interactive_shell: c_int;
    pub static sourcelevel: c_int;
    pub static running_trap: c_int;
    pub static parse_and_execute_level: c_int;
}