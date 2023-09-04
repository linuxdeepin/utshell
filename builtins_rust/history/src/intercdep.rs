
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


pub type histdata_t = *mut libc::c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _hist_entry {
    pub line: *mut c_char,
    pub timestamp: *mut c_char,
    pub data: histdata_t,
}
pub type HIST_ENTRY = _hist_entry;

extern "C" {
    pub fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn bash_clear_history();
    pub fn bash_delete_last_history() -> c_int;
    pub fn bash_delete_history_range (first: c_int, last: c_int) -> c_int;
    pub fn bash_delete_histent(i: c_int) -> c_int;

    pub fn check_add_history (line: *mut c_char, force: c_int) -> c_int;
    pub fn history_expand (hstring: *mut c_char, output: *mut *mut c_char) -> c_int;
    pub fn history_get_time(hist: *mut HIST_ENTRY) -> libc::time_t;
    pub fn history_list() -> *mut *mut HIST_ENTRY;
    pub fn where_history() -> c_int;
    pub fn history_set_pos(pos: c_int) -> c_int;
    pub fn using_history();
    pub fn maybe_append_history(filename: *mut c_char) -> c_int;
    pub fn write_history(filename: *const c_char) -> c_int;
    pub fn read_history(filename: *const c_char) -> c_int;
    pub fn read_history_range(filename: *const c_char, from: c_int, to: c_int) -> c_int;
    pub fn strftime(s: *mut c_char, maxsize:size_t, format: *const c_char, timeptr: *const libc::tm) -> size_t;
    pub fn get_numeric_arg(list: *mut WordList, fatal: c_int, count: *mut c_long) -> c_int;

    pub fn string_list(list: *mut WordList) -> *mut c_char;

    pub fn sh_chkwrite(s: c_int) -> c_int;
    pub fn get_string_value(var_name: *const c_char) -> *mut c_char;

    pub fn termsig_handler(sig: c_int) -> c_int;
    pub fn throw_to_top_level() -> c_void;

    pub fn legal_number (str1:*const c_char,result:* mut c_long) -> i32;

    pub fn sh_erange(s: *mut c_char, desc: *mut c_char);
    pub fn sh_restricted(s: *mut c_char) -> c_void;
    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;

    pub static mut remember_on_history: c_int;
    pub static mut hist_last_line_pushed: c_int;
    pub static mut hist_last_line_added: c_int;
    pub static mut current_command_line_count: c_int;
    pub static mut current_command_first_line_saved: c_int;
    pub static mut command_oriented_history: c_int;

    pub static mut terminating_signal: c_int;
    pub static mut interrupt_state: c_int;

    pub static mut history_base: c_int;
    pub static mut history_length: c_int;
    pub static mut history_lines_in_file: c_int;
    pub static mut history_lines_read_from_file: c_int;
    pub static mut force_append_history: c_int;
    pub static mut history_lines_this_session: c_int;

    pub static mut restricted: c_int;
}
