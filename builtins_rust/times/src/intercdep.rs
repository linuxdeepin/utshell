use rcommon::WordList;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

extern "C" {
    pub fn no_options(list: *mut WordList) -> c_int;
    pub fn print_timeval(fp: *mut libc::FILE, tvp: *mut libc::timeval);
    pub fn sh_chkwrite(s: c_int) -> c_int;

    pub static stdout: *mut libc::FILE;
}
