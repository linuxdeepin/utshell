use rcommon::{WordList};
use rread::{ARRAY};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WordDesc = word_desc;

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
pub type SHELL_VAR = variable;

pub type pid_t = c_int;
pub type WAIT = c_int;
pub type sh_vptrfunc_t = *mut fn();
pub type JOB_STATE = c_int;

pub type command_type = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union REDIRECTEE {
    pub dest: c_int,
    pub filename: *mut WordDesc,
}

pub type r_instruction = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct redirect {
    pub next: *mut redirect,
    pub redirector: REDIRECTEE,
    pub rflags: c_int,
    pub flags: c_int,
    pub instruction: r_instruction,
    pub redirectee: REDIRECTEE,
    pub here_doc_eof: *mut c_char,
}

pub type REDIRECT = redirect;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct command {
    pub type_: command_type,
    pub flags: c_int,
    pub line: c_int,
    pub redirects: *mut REDIRECT,
    pub value: command__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union command__bindgen_ty_1 {
    pub For: *mut for_com,
    pub Case: *mut case_com,
    pub While: *mut while_com,
    pub If: *mut if_com,
    pub Connection: *mut connection,
    pub Simple: *mut simple_com,
    pub Function_def: *mut function_def,
    pub Group: *mut group_com,
    pub Select: *mut select_com,
    pub Arith: *mut arith_com,
    pub Cond: *mut cond_com,
    pub ArithFor: *mut arith_for_com,
    pub Subshell: *mut subshell_com,
    pub Coproc: *mut coproc_com,
}

pub type COMMAND = command;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pattern_list {
    pub next: *mut pattern_list,
    pub patterns: *mut WordList,
    pub action: *mut COMMAND,
    pub flags: c_int,
}

pub type PATTERN_LIST = pattern_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct case_com {
    pub flags: c_int,
    pub line: c_int,
    pub word: *mut WordDesc,
    pub clauses: *mut PATTERN_LIST,
}

pub type CASE_COM = case_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct for_com {
    pub flags: c_int,
    pub line: c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}

pub type FOR_COM = for_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arith_for_com {
    pub flags: c_int,
    pub line: c_int,
    pub init: *mut WordList,
    pub test: *mut WordList,
    pub step: *mut WordList,
    pub action: *mut COMMAND,
}

pub type ARITH_FOR_COM = arith_for_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct select_com {
    pub flags: c_int,
    pub line: c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}

pub type SELECT_COM = select_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct if_com {
    pub flags: c_int,
    pub test: *mut COMMAND,
    pub true_case: *mut COMMAND,
    pub false_case: *mut COMMAND,
}

pub type IF_COM = if_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct while_com {
    pub flags: c_int,
    pub test: *mut COMMAND,
    pub action: *mut COMMAND,
}

pub type WHILE_COM = while_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arith_com {
    pub flags: c_int,
    pub line: c_int,
    pub exp: *mut WordList,
}

pub type ARITH_COM = arith_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cond_com {
    pub flags: c_int,
    pub line: c_int,
    pub type_: c_int,
    pub op: *mut WordDesc,
    pub left: *mut cond_com,
    pub right: *mut cond_com,
}

pub type COND_COM = cond_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct simple_com {
    pub flags: c_int,
    pub line: c_int,
    pub words: *mut WordList,
    pub redirects: *mut REDIRECT,
}

pub type SIMPLE_COM = simple_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct function_def {
    pub flags: c_int,
    pub line: c_int,
    pub name: *mut WordDesc,
    pub command: *mut COMMAND,
    pub source_file: *mut c_char,
}

pub type FUNCTION_DEF = function_def;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct group_com {
    pub ignore: c_int,
    pub command: *mut COMMAND,
}

pub type GROUP_COM = group_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subshell_com {
    pub flags: c_int,
    pub line: c_int,
    pub command: *mut COMMAND,
}

pub type SUBSHELL_COM = subshell_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coproc {
    pub c_name: *mut c_char,
    pub c_pid: pid_t,
    pub c_rfd: c_int,
    pub c_wfd: c_int,
    pub c_rsave: c_int,
    pub c_wsave: c_int,
    pub c_flags: c_int,
    pub c_status: c_int,
    pub c_lock: c_int,
}

pub type Coproc = coproc;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coproc_com {
    pub flags: c_int,
    pub name: *mut c_char,
    pub command: *mut COMMAND,
}

pub type COPROC_COM = coproc_com;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct process {
    pub next: *mut process,
    pub pid: pid_t,
    pub status: WAIT,
    pub running: c_int,
    pub command: *mut c_char,
}

pub type PROCESS = process;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct job {
    pub wd: *mut c_char,
    pub pipe: *mut PROCESS,
    pub pgrp: pid_t,
    pub state: JOB_STATE,
    pub flags: c_int,
    pub deferred: *mut COMMAND,
    pub j_cleanup: sh_vptrfunc_t,
    pub cleanarg: *mut c_void,
}

pub type JOB = job;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct connection {
    pub _address: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jobstats {
    pub c_childmax: c_long,
    pub c_living: c_int,
    pub c_reaped: c_int,
    pub c_injobs: c_int,
    pub c_totforked: c_int,
    pub c_totreaped: c_int,
    pub j_jobslots: c_int,
    pub j_lastj: c_int,
    pub j_firstj: c_int,
    pub j_njobs: c_int,
    pub j_ndead: c_int,
    pub j_current: c_int,
    pub j_previous: c_int,
    pub j_lastmade: *mut JOB,
    pub j_lastasync: *mut JOB,
}

#[repr(C)]
pub struct BUCKET_CONTENTS {
	next:* mut BUCKET_CONTENTS,	/* Link to next hashed key in this bucket. */
	key:* mut c_char,		/* What we look up. */
	data:* mut c_void,			/* What we really want. */
	khash:u32,		/* What key hashes to */
	times_found:i32		/* Number of times this item has been found. */
}

#[repr(C)]
pub struct HASH_TABLE {
	bucket_array:*mut * mut BUCKET_CONTENTS,	/* Where the data is kept. */
	nbuckets:i32,			/* How many buckets does this table have. */
	nentries:i32			/* How many entries does this table have. */
}

pub type sh_builtin_func_t =
    unsafe extern "C" fn(arg1: *mut WordList) -> c_int;

pub const att_exported: c_int = 0x0000001;
pub const att_readonly: c_int = 0x0000002;
pub const att_array: c_int = 0x0000004;
pub const att_function: c_int = 0x0000008;
pub const att_integer: c_int = 0x0000010;
pub const att_local: c_int = 0x0000020;
pub const att_assoc: c_int = 0x0000040;
pub const att_trace: c_int = 0x0000080;
pub const att_uppercase: c_int = 0x0000100;
pub const att_lowercase: c_int = 0x0000200;
pub const att_capcase: c_int = 0x0000400;
pub const att_nameref: c_int = 0x0000800;

pub const att_invisible: c_int = 0x0001000;
pub const att_imported: c_int = 0x0008000;

pub const att_tempvar: c_int = 0x0100000;
pub const att_propagate: c_int = 0x0200000;

pub const EX_USAGE: c_int = 258;
pub const EX_BADASSIGN: c_int = 260;

pub const EXECUTION_SUCCESS: c_int = 0;
pub const EXECUTION_FAILURE: c_int = 1;

pub const ASS_APPEND: c_int = 0x0001;

pub const FUNC_MULTILINE: c_int = 0x01;
pub const FUNC_EXTERNAL: c_int = 0x02;

extern "C" {
    pub static mut loptend : *mut WordList;
    pub static mut array_needs_making: c_int;
    pub static mut this_shell_builtin: sh_builtin_func_t;
    pub static mut posixly_correct: c_int;
    pub static mut this_command_name: *mut c_char;
    pub static mut variable_context: c_int;
    pub static mut shell_compatibility_level: c_int;
    pub static mut nameref_invalid_value: SHELL_VAR;

    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;
    pub fn builtin_usage();
    pub fn builtin_error(arg1: *const c_char, ...);
    pub fn find_function(name: *const c_char) -> *mut SHELL_VAR;
    pub fn exportable_function_name(string: *const c_char) -> c_int;
    pub fn assignment(string: *const c_char, flags: c_int) -> c_int;
    pub fn legal_identifier(arg1: *const c_char) -> c_int;
    pub fn sh_invalidid(s: *mut c_char);
    pub fn make_word(string: *const c_char) -> *mut WordDesc;
    pub fn make_word_list(word: *mut WordDesc, wlink: *mut WordList) -> *mut WordList;
    pub fn declare_builtin(list:* mut WordList) -> c_int;
    pub fn dispose_word(w: *mut WordDesc);
    pub fn do_assignment_no_expand(string: *mut c_char) -> c_int;

    pub fn all_shell_functions() -> *mut *mut SHELL_VAR;
    pub fn all_shell_variables() -> *mut *mut SHELL_VAR;
    pub fn all_local_variables(visible_only: c_int) -> *mut *mut SHELL_VAR;    

    pub fn sh_chkwrite(s: c_int) -> c_int;
    pub fn sh_double_quote(string: *const c_char) -> *mut c_char;

    pub fn named_function_string (name: *mut c_char, command: *mut COMMAND, flags: c_int) -> *mut c_char;

    pub fn print_array_assignment (var: *mut SHELL_VAR, quoted: c_int);
    pub fn print_assoc_assignment (var: *mut SHELL_VAR, quoted: c_int);

    pub fn find_variable_noref(name: *const c_char) -> *mut SHELL_VAR;
    pub fn find_variable(name: *const c_char) -> *mut SHELL_VAR;
    pub fn find_global_variable(name: *const c_char) -> *mut SHELL_VAR;
    pub fn find_tempenv_variable(name: *const c_char) -> *mut SHELL_VAR;
    pub fn find_variable_notempenv(name: *const c_char) -> *mut SHELL_VAR;
    pub fn find_variable_nameref_for_create(name: *const c_char, flags: c_int) -> *mut SHELL_VAR;
    pub fn bind_variable(name: *const c_char, value: *mut c_char, flags: c_int) -> *mut SHELL_VAR;

    pub fn stupidly_hack_special_variables(name: *mut c_char);
    pub fn array_to_assign(a:*mut ARRAY,quote : c_int) -> *mut c_char;
    pub fn assoc_to_assign(a:*mut HASH_TABLE,quote : c_int) -> *mut c_char;
}
