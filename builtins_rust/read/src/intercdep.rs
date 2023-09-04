use rcommon::{r_builtin_usage,r_sh_invalidid,r_builtin_bind_variable,WordList,WordDesc};
pub type SHELL_VAR = rcommon::SHELL_VAR;
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

pub type __jmp_buf = [c_long; 8usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [c_ulong; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: c_int,
    pub __saved_mask: __sigset_t,
}

pub type sigjmp_buf = [__jmp_buf_tag; 1usize];

pub type rl_hook_func_t = fn() -> c_int;
pub type rl_completion_func_t = fn(args1 : *const c_char, args2 : c_int) -> *mut *mut c_char;

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
pub const EINTR : c_int = 4;
pub const EX_USAGE : c_int = 258;

pub const VA_NOEXPAND : c_int = 0x001;
pub const VA_ONEWORD : c_int = 0x002;

pub const ISFUNC: c_int = 0;

pub const MB_LEN_MAX: c_int = 16;

pub const CTLESC: c_char = b'\x01' as c_char;
pub const CTLNUL: c_char = b'\x4f' as c_char;

pub const __S_IFMT: u32 = 0o0170000;
pub const __S_IFREG: u32 = 0o0100000;

extern "C" { 

    pub fn reset_internal_getopt();

    pub fn internal_getopt(
        arg1: *mut WordList,
        arg2: *mut c_char,
    ) -> c_int;

    pub fn list_string(s: *mut c_char, t: *mut c_char, i: c_int) -> *mut WordList;
    pub fn dequote_string(s: *mut c_char) -> *mut c_char;
    pub fn dequote_list(s: *mut WordList) -> *mut WordList;
    pub fn word_list_remove_quoted_nulls(s: *mut WordList);
    pub fn dispose_words(s: *mut WordList);
    pub fn assign_array_var_from_word_list(var: *mut SHELL_VAR, list: *mut WordList, flags: c_int) -> *mut SHELL_VAR;

    pub fn builtin_usage();

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;
    pub static mut assoc_expand_once : c_int;
    pub static mut interactive_shell : c_int;

    pub static mut rl_num_chars_to_read: c_int;
    pub static mut rl_attempted_completion_function : rl_completion_func_t;

    pub static mut bash_readline_initialized : c_int;
    pub static mut rl_startup_hook: *mut rl_hook_func_t;

    pub static interrupt_state : c_int;
    pub static terminating_signal : c_int;
    pub static trapped_signal_received : c_int;

    // pub static mut alrmbuf: sigjmp_buf;

    pub static mut rl_instream: *mut libc::FILE;

    pub static posixly_correct: c_int;

    pub static locale_utf8locale: c_int;

    pub static mut ifs_cmap: [u8; 256];

    pub fn uconvert(
        s: *mut c_char,
        ip: *mut c_long,
        up: *mut c_long,
        ep: *mut *mut c_char,
    ) -> c_int;

    pub fn builtin_error(arg1: *const c_char, ...);

    pub fn legal_number(
        arg1: *const c_char,
        arg2: *mut std::os::raw::c_long,
    ) -> c_int;

    pub fn sh_invalidnum(arg1: *mut c_char);

    pub fn sh_validfd(arg1: c_int) -> c_int;

    pub fn input_avail(arg1: c_int) -> c_int;

    pub fn legal_identifier(arg1: *const c_char) -> c_int;

    pub fn valid_array_reference(
        arg1: *const c_char,
        arg2: c_int,
    ) -> c_int;

    pub fn sh_invalidid(arg1: *mut c_char);

    pub fn getifs() -> *mut c_char;

    pub fn xmalloc(arg1: libc::size_t) -> *mut c_void;

    pub fn xfree(arg1: *mut c_void);

    pub fn xrealloc(arg1: *mut c_void, arg2: libc::size_t)
        -> *mut c_void;

    pub fn get_string_value(arg1: *const c_char) -> *mut c_char;

    pub fn begin_unwind_frame(arg1: *mut c_char);
    pub fn run_unwind_frame(arg1: *mut c_char);
    pub fn discard_unwind_frame(arg1: *mut c_char);

    pub fn fd_is_bash_input(arg1: c_int) -> c_int;

    pub fn sync_buffered_stream(arg1: c_int) -> c_int;

    pub fn initialize_readline() -> c_void;
    pub fn readline(p : *const c_char) -> *mut c_char;
    pub fn rl_insert_text(p : *const c_char) -> c_int;

    pub fn bashline_set_event_hook() -> c_void;
    pub fn bashline_reset_event_hook() -> c_void;

    pub fn zreadintr(arg1: c_int, arg2: *mut c_char, arg3: size_t) -> libc::ssize_t;
    pub fn zreadcintr(arg1: c_int, arg2: *mut c_char) -> libc::ssize_t;

    pub fn zread(arg1: c_int, arg2: *mut c_char, arg3: size_t) -> libc::ssize_t;
    pub fn zreadn(arg1: c_int, arg2: *mut c_char, arg3: size_t) -> libc::ssize_t;
    pub fn zreadc(arg1: c_int, arg2: *mut c_char) -> libc::ssize_t;

    pub fn zsyncfd(fd: c_int) -> c_void;

    pub fn check_signals();
    pub fn termsig_handler(arg1: c_int) -> c_void;
    pub fn throw_to_top_level() -> c_void;

    pub fn builtin_bind_variable(name: *mut c_char, value: *mut c_char, flags: c_int) -> *mut SHELL_VAR;
    pub fn bind_variable(name: *const c_char, value: *mut c_char, flags: c_int) -> *mut SHELL_VAR;
    pub fn find_or_make_array_variable(name: *mut c_char, flags: c_int) -> *mut SHELL_VAR;

    pub fn array_flush(a: *mut ARRAY);

    pub fn get_word_from_string(stringp: *mut *mut c_char, separators: *mut c_char, endptr: *mut *mut c_char) -> *mut c_char;

    pub fn stupidly_hack_special_variables(name: *mut c_char);
    pub fn strip_trailing_ifs_whitespace(s: *mut c_char, sep: *mut c_char, es: c_int) -> *mut c_char;
}

extern "C" {
    pub fn initialize_terminating_signals();
}

pub type SigHandler = unsafe extern "C" fn(arg1: c_int);
extern "C" {
    pub fn set_signal_handler(arg1: c_int, arg2: *mut SigHandler) -> *mut SigHandler;
}

extern "C" {
    // todo: more f type
    pub fn add_unwind_protect(f : *mut c_void,...);
    pub fn unwind_protect_mem(var : *mut c_int, size : c_int);
    pub fn remove_unwind_protect() -> c_void;
}

extern "C" {
    pub fn falarm(
        arg1: c_uint,
        arg2: c_uint,
    ) -> c_uint;
}

extern "C" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: c_int,
    ) -> c_int;

    pub fn siglongjmp(__env: *mut __jmp_buf_tag, __val: c_int);

}

extern "C" {
    pub fn ttgetattr(arg1: c_int, arg2: *mut libc::termios) -> c_int;
    pub fn ttsetattr(arg1: c_int, arg2: *mut libc::termios) -> c_int;

    pub fn ttfd_noecho(
        arg1: c_int,
        arg2: *mut libc::termios,
    ) -> c_int;

}

extern "C" {
    pub fn sh_ttyerror(arg1: c_int);
    pub fn ttfd_cbreak(fd: c_int, ttp: *mut libc::termios) -> c_int;
    pub fn ttfd_onechar(fd: c_int, ttp: *mut libc::termios) -> c_int;
}


pub type rl_command_func_t = unsafe extern "C" fn(c_int, c_int) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _keymap_entry {
    pub tp: c_char,
    pub function: rl_command_func_t,
}
pub type KEYMAP_ENTRY = _keymap_entry;
pub type Keymap = *mut KEYMAP_ENTRY;

extern "C" {
    pub fn rl_get_keymap() -> Keymap;
    pub fn rl_insert(count: c_int, key: c_int) -> c_int;
    pub fn rl_newline(count: c_int, key: c_int) -> c_int;
}

extern "C" {
    pub fn mbrtowc(pwc: *mut libc::wchar_t, s: *mut c_char, n: libc::size_t, ps: *mut mbstate_t) -> libc::size_t;
    pub fn mbrlen(mbstr: *const c_char, count: size_t, mbstate: *mut mbstate_t) -> c_int;
}

pub const atype_array_indexed: atype = 0;
pub const atype_array_assoc: atype = 1;
pub type atype = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct array {
    pub type_: atype,
    pub max_index: arrayind_t,
    pub num_elements: c_int,
    pub head: *mut array_element,
    pub lastref: *mut array_element,
}
pub type ARRAY = array;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct array_element {
    pub ind: arrayind_t,
    pub value: *mut c_char,
    pub next: *mut array_element,
    pub prev: *mut array_element,
}
pub type ARRAY_ELEMENT = array_element;


extern "C" {
    static is_basic_table:[libc::c_uint;0];
    static mut sigalrm_seen:libc::c_int;
    // static mut tty_modified:libc::c_int ;
    // fn is_basic(c:libc::c_char)->libc::c_int;
}
