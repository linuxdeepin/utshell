use rcommon::{WordList};

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

extern "C" {
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn get_numeric_arg(list: *mut WordList, fatal: c_int, count: c_long) -> c_int;
    pub fn number_of_args() -> c_int;
    pub fn sh_erange(s: *mut c_char, desc: *mut c_char);
    pub fn clear_dollar_vars();
    pub fn shift_args(times: c_int);
    pub fn invalidate_cached_quoted_dollar_at();

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;
}
