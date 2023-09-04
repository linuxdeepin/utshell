use rcommon::{WordList};
use rhelp::r_builtin_help;
pub type arrayind_t = c_long;

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
#[derive(Debug)]
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
pub type SHELL_VAR = variable;

pub const atype_array_indexed: atype = 0;
pub const atype_array_assoc: atype = 1;
pub type atype = c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array {
    pub type_: atype,
    pub max_index: arrayind_t,
    pub num_elements: c_int,
    pub head: *mut array_element,
    pub lastref: *mut array_element,
}
pub type ARRAY = array;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_element {
    pub ind: arrayind_t,
    pub value: *mut c_char,
    pub next: *mut array_element,
    pub prev: *mut array_element,
}
pub type ARRAY_ELEMENT = array_element;

pub const ESPIPE: c_int = 29;
pub const SEEK_CUR: c_int = 1;

pub const EXECUTION_SUCCESS : c_int = 0;
pub const EXECUTION_FAILURE : c_int = 1;
pub const EX_USAGE: c_int = 258;

pub const SEVAL_NOHIST: c_int = 0x004;

pub const att_readonly: c_int = 0x0000002;
pub const att_array: c_int = 0x0000004;
pub const att_invisible: c_int = 0x0001000;
pub const att_noassign: c_int = 0x0004000;

extern "C" {
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn legal_identifier(arg1: *const c_char) -> c_int;
    pub fn legal_number (str1:*const c_char,result:* mut c_long) -> i32;
    pub fn sh_invalidid(arg1: *mut c_char);
    pub fn sh_validfd(arg1: c_int) -> c_int;

    pub fn sh_single_quote(s: *mut c_char) -> *mut c_char;

    pub fn evalstring(string: *mut c_char, from_file: *const c_char, flags: c_int) -> c_int;

    pub fn find_or_make_array_variable(name: *mut c_char, flags: c_int) -> *mut SHELL_VAR;

    pub fn bind_array_element(entry: *mut SHELL_VAR, ind: c_long, value: *mut c_char, flags: c_int) ->  *mut SHELL_VAR;

    pub fn array_flush(a: *mut ARRAY);

    pub fn err_readonly(s: *const c_char) -> c_void;

    pub fn zreset() -> c_void;
    pub fn zsyncfd(fd: c_int) -> c_void;
    pub fn zgetline (fd: c_int, lineptr: *mut *mut c_char, n: *mut size_t, delim: c_int, unbuffered_read: c_int) -> ssize_t;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;

}
