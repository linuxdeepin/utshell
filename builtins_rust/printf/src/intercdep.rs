use rcommon::{WordList};
use rhelp::r_builtin_help;
pub type __intmax_t = c_long;
pub type intmax_t = __intmax_t;
pub type arrayind_t = intmax_t;
pub type sh_var_value_func_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: *mut variable) -> *mut variable>;

    pub type sh_var_assign_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut variable,
        arg2: *mut c_char,
        arg3: arrayind_t,
        arg4: *mut c_char,
    ) -> *mut variable,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct variable {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub exportstr: *mut c_char,
    pub dynamic_value: sh_var_value_func_t,
    pub assign_func: sh_var_assign_func_t,
    pub attributes: c_int,
    pub context: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
pub type mbstate_t = __mbstate_t;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub const VA_NOEXPAND: c_int = 0x001;
pub const VA_ONEWORD: c_int = 0x002;

pub const att_readonly: c_int = 0x0000002;
pub const att_noassign: c_int = 0x0004000;

extern "C" {
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);
    pub fn builtin_warning(format: *const c_char, ...);
    pub fn builtin_bind_variable(name: *mut c_char, value: *mut c_char, flags: c_int) -> *mut SHELL_VAR;
    pub fn stupidly_hack_special_variables(name: *const c_char) -> c_void;
    pub fn legal_identifier(arg1: *const c_char) -> c_int;
    pub fn legal_number (str1:*const c_char,result:* mut c_long) -> i32;
    pub fn sh_invalidid(arg1: *mut c_char);

    pub fn valid_array_reference (name: *const c_char, flags: c_int) -> c_int;
    pub fn bind_var_to_int(var: *mut c_char, val: c_long) -> *mut SHELL_VAR;

    pub fn mbtowc(pwc: *mut libc::wchar_t, s: *const c_char, n: size_t) -> size_t;

    pub fn sh_invalidnum(s: *mut c_char) -> c_void;

    pub fn xmalloc(bytes: size_t) -> *mut c_void;
    pub fn xrealloc(p: *mut c_void, bytes: size_t) -> *mut c_void;
    //pub fn savestring(s: *const c_char) -> *mut c_char;
    pub fn ansic_shouldquote(s: *const c_char) -> c_int;
    pub fn ansic_quote(str: *mut c_char, flags: c_int, rlen: *mut c_int) -> *mut c_char;
    pub fn sh_backslash_quote(string: *mut c_char, table: *mut c_char, flags: c_int) -> *mut c_char;

    pub fn u32cconv(c: c_ulong, s: *mut c_char) -> c_int;

    pub fn termsig_handler(arg1: c_int) -> c_void;
    pub fn throw_to_top_level() -> c_void;

    pub fn sv_tz(name: *mut c_char) -> c_void;
    pub fn strftime(s: *mut c_char, maxsize: size_t, format: *const c_char, timeptr: *const libc::tm) -> size_t;

    pub fn sh_wrerror() -> c_void;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;
    pub static mut assoc_expand_once: c_int;

    pub static terminating_signal : c_int;
    pub static interrupt_state : c_int;

    pub static stdout: *mut libc::FILE;

    pub static shell_start_time: libc::time_t;
}

