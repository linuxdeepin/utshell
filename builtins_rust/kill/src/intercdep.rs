use rcommon::{r_builtin_usage,r_sh_invalidsig,r_sh_badpid,r_sh_badjob,r_get_job_spec,r_display_signal_list,WordList,WordDesc,err_translate_fn};
use rhelp::r_builtin_help;
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

pub const EINVAL: c_int = 22;
pub const NO_SIG: c_int = -1;

pub const DSIG_SIGPREFIX: c_int = 0x01;
pub const DSIG_NOCASE: c_int = 0x02;

pub const DUP_JOB: c_int = -2;

pub const J_JOBCONTROL: c_int = 0x04;

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

extern "C" {
    pub fn reset_internal_getopt();
    pub fn internal_getopt(list: *mut WordList, opts: *mut c_char) -> c_int;

    pub fn get_job_spec(list: *mut WordList) -> c_int;

    // pub fn builtin_usage();
    pub fn builtin_error(format: *const c_char, ...);

    pub fn legal_number(string: *mut c_char, result: c_long) -> c_int;
    pub fn display_signal_list (list: *mut WordList, forcecols: c_int) -> c_int;
    pub fn decode_signal (string: *mut c_char, flags: c_int) -> c_int;

    pub fn sh_needarg(s: *mut c_char) -> c_void;
    pub fn sh_invalidsig(s: *mut c_char) -> c_void;
    pub fn sh_badpid(s: *mut c_char) -> c_void;
    pub fn sh_badjob(s: *mut c_char) -> c_void;

    pub fn kill_pid(pid: libc::pid_t, sig: c_int, group: c_int) -> c_int;

    pub static mut list_optarg : *mut libc::c_char;
    pub static mut loptend : *mut WordList;

    pub static posixly_correct: c_int;

    pub static mut js: jobstats;
    pub static mut jobs: *mut *mut JOB;
}
