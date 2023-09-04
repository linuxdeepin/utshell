extern crate libc;

use libc::{c_char,c_int, c_void, FILE, size_t, intmax_t,c_long, strcmp};
use libc::{isdigit,strerror, __errno_location, fflush, ferror,clearerr, free,strcpy,strlen,strncmp,atoi,qsort};
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr::read_volatile;
use nix::errno::errno;
use std::env::var;
use unic_langid::LanguageIdentifier;
include!(concat!("lib_readline_keymaps.rs"));

include!(concat!("command.rs"));

use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
use fluent_resmgr::resource_manager::ResourceManager;

//struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_desc {
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WordDesc = word_desc;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct word_list {
    pub next: *mut word_list,
    pub word: *mut WordDesc,
}
pub type WordList = word_list;

#[repr (C)]
pub struct builtin{
    pub name:*mut c_char,
    pub function:*mut sh_builtin_func_t,
    pub flags:i32,
    pub long_doc: *const *mut c_char ,
    pub short_doc:*const c_char,
    pub handle:*mut c_char,
}

#[repr (C)]
pub struct g_list{
    pub next:*mut g_list,
}
type GENERIC_LIST = g_list;

#[repr (C)]
pub struct process{
    pub next:*mut process,
    pub pid:pid_t,
    pub status:WAIT,
    pub running:i32,
    pub command:*mut c_char,
}
type WAIT = i32;
type pid_t = c_int;
type PROCESS = process;

#[repr(C)]
pub struct JOB {
    wd: *mut c_char,
    pipe: *mut PROCESS,
    pgrp:i32,
    state:JOB_STATE,
    flags:i32,
    deferred:*mut COMMAND,
    j_cleanup:*mut fn(),
    cleanarg:* mut fn()
}

#[repr(C)]
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND,
}
#[repr(C)]
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest:libc::c_int,           /* Place to redirect REDIRECTOR to, or ... */
    filename:* mut WordDesc        /* filename to redirect to. */
}

#[repr(u8)]
#[derive(Copy,Clone)]
enum r_instruction {
    r_output_direction, r_input_direction, r_inputa_direction,
    r_appending_to, r_reading_until, r_reading_string,
    r_duplicating_input, r_duplicating_output, r_deblank_reading_until,
    r_close_this, r_err_and_out, r_input_output, r_output_force,
    r_duplicating_input_word, r_duplicating_output_word,
    r_move_input, r_move_output, r_move_input_word, r_move_output_word,
    r_append_err_and_out
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,   /* Next element, or NULL. */
  redirector:REDIRECTEE,    /* Descriptor or varname to be redirected. */
  rflags:libc::c_int,           /* Private flags for this redirection */
  flags:libc::c_int,            /* Flag value for `open'. */
  instruction:r_instruction, /* What to do with the information. */
  redirectee:REDIRECTEE,    /* File descriptor or filename */
  here_doc_eof:*mut c_char      /* The word that appeared in <<foo. */
}

/* FOR command. */
#[repr(C)]
pub struct for_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct case_com {
    flags:libc::c_int,
    line:libc::c_int,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
}

#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:libc::c_int
}

#[repr(C)]
pub struct while_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    action:*mut COMMAND
}
#[repr(C)]
pub struct if_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}

#[repr(C)]
pub struct connection {
    ignore:libc::c_int,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector:libc::c_int
}
#[repr(C)]
pub struct simple_com {
    flags:libc::c_int,
    line:libc::c_int,
    words:*mut WordList,
    redirects:*mut REDIRECT
}
#[repr(C)]
pub struct function_def {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    command:*mut COMMAND,
    source_file:*mut c_char
}
#[repr(C)]
pub struct group_com {
    ignore:libc::c_int,
    command:*mut COMMAND,
    source_file:*mut c_char
}
#[repr(C)]
pub struct select_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct arith_com {
    flags:libc::c_int,
    line:libc::c_int,
    exp:*mut WordList
}

#[repr(C)]
pub struct cond_com {
    flags:libc::c_int,
    line:libc::c_int,
    type_c:libc::c_int,
    exp:*mut WordList
}
#[repr(C)]
pub struct arith_for_com {
    flags:libc::c_int,
    line:libc::c_int,
    init:*mut WordList,
    test:*mut WordList,
    step:*mut WordList,
    action:*mut COMMAND
}
#[repr(C)]
pub struct subshell_com {
    flags:i32,
    line:i32,
    command:*mut COMMAND
}
#[repr(C)]
pub struct coproc_com {
    flags:i32,
    name:*mut c_char,
    command:*mut COMMAND
}

#[repr(C)]
pub union VALUE_COMMAND {
    For:*mut for_com,
    Case:*mut case_com,
    While:*mut while_com,
    If:*mut if_com,
    Connection:*mut connection,
    Simple:*mut simple_com,
    Function_def:*mut function_def,
    Group:*mut group_com,
    Select:*mut select_com,
    Arith:*mut arith_com,
    Cond:*mut cond_com,
    ArithFor:*mut arith_for_com,
    Subshell:*mut subshell_com,
    Coproc:*mut coproc_com
}


#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
}

#[repr(C)]
pub struct jobstats {
    /* limits */
    c_childmax:libc::c_long,
    /* child process statistics */
    c_living:libc::c_int,       /* running or stopped child processes */
    c_reaped:libc::c_int,   /* exited child processes still in jobs list */
    c_injobs:libc::c_int,   /* total number of child processes in jobs list */
    /* child process totals */
    c_totforked:libc::c_int,    /* total number of children this shell has forked */
    c_totreaped:libc::c_int,    /* total number of children this shell has reaped */
    /* job counters and indices */
    j_jobslots:libc::c_int,/* total size of jobs array */
    j_lastj:libc::c_int,        /* last (newest) job allocated */
    j_firstj:libc::c_int,   /* first (oldest) job allocated */
    j_njobs:libc::c_int,        /* number of non-NULL jobs in jobs array */
    j_ndead:libc::c_int,        /* number of JDEAD jobs in jobs array */
    /* */
    j_current:libc::c_int,  /* current job */
    j_previous:libc::c_int, /* previous job */
    /* */
    j_lastmade:* mut JOB,   /* last job allocated by stop_pipeline */
    j_lastasync:* mut JOB   /* last async job allocated by stop_pipeline */
}

#[repr(C)]
pub struct SHELL_VAR {
  pub name:*mut c_char,         /* Symbol that the user types. */
  pub value:*mut c_char,            /* Value that is returned. */
  pub exportstr:*mut c_char,    /* String for the environment. */
  pub dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR, /* Function called to return a `dynamic'
                   value for a variable, like $SECONDS
                   or $RANDOM. */
  pub assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut c_char,t:c_long,str2:* mut c_char)->*mut SHELL_VAR, /* Function called when this `special
                   variable' is assigned a value in
                   bind_variable. */
  pub attributes:i32,       /* export, readonly, array, invisible... */
  pub context:i32           /* Which context this variable belongs to. */
}

//macro
#[macro_export]
macro_rules! EXECUTION_FAILURE {
    () => {
        1
    };
}

#[macro_export]
macro_rules! EXECUTION_SUCCESS {
    () => {
        0
    };
}

#[macro_export]
macro_rules! DISCARD {
    () => {
        2
    };
}

#[macro_export]
macro_rules! GETOPT_HELP {
    () => {
        -99
    };
}

#[macro_export]
macro_rules! ARGS_INVOC {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! ARGS_FUNC {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! ARGS_SETBLTIN {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! EX_BADUSAGE {
    () => {
        2
    };
}

#[macro_export]
macro_rules! DEBUG_TRAP {
    () => {
        NSIG!()
    };
}

#[macro_export]
macro_rules! NSIG {
    () => {
        65
    };
}

#[macro_export]
macro_rules! NO_JOB {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! DUP_JOB {
    () => {
        -2
    };
}

#[macro_export]
macro_rules! JM_SUBSTRING {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! JM_EXACT {
    () => {
        0x04
    };
}

#[macro_export]
macro_rules! JM_STOPPED {
    () => {
        0x08
    };
}

#[macro_export]
macro_rules! JM_FIRSTMATCH {
    () => {
        0x10
    };
}

#[macro_export]
macro_rules! VA_NOEXPAND {
    () => {
        0x001
    };
}

#[macro_export]
macro_rules! VA_ONEWORD {
    () => {
        0x002
    };
}

#[macro_export]
macro_rules! ASS_NOEXPAND {
    () => {
        0x0080
    };
}

#[macro_export]
macro_rules! att_readonly {
    () => {
        0x0000002
    };
}

#[macro_export]
macro_rules! att_invisible {
    () => {
        0x0001000
    };
}

#[macro_export]
macro_rules! att_nounset {
    () => {
        0x0002000
    };
}

#[macro_export]
macro_rules! att_noassign {
    () => {
        0x0004000
    };
}

#[macro_export]
macro_rules! SPECIAL_BUILTIN {
    () => {
        0x08
    };
}

#[macro_export]
macro_rules! BUILTIN_ENABLED {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! BUILTIN_DELETED {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! DSIG_SIGPREFIX {
    () => {
        0x01
    };
}

#[macro_export]
macro_rules! DSIG_NOCASE {
    () => {
        0x02
    };
}

#[macro_export]
macro_rules! NO_SIG {
    () => {
        -1
    };
}

#[macro_export]
macro_rules! readonly_p {
    ($var:expr) => {
        (*$var).attributes & att_readonly!()
    };
}

#[macro_export]
macro_rules! noassign_p {
    ($var:expr) => {
        (*$var).attributes & att_noassign!()
    };
}

#[macro_export]
macro_rules! non_unsettable_p {
    ($var:expr) => {
        (*$var).attributes & att_nounset!()
    };
}

#[macro_export]
macro_rules! VUNSETATTR {
    ($var:expr,$attr:expr) => {
        (*$var).attributes &=  !($attr)
    };
}



#[macro_export]
macro_rules! ISOCTAL {
    ($c:expr) => {
        ($c) >= b'0' as libc::c_char  && ($c) <= b'7' as libc::c_char
    };
}

#[macro_export]
macro_rules! DIGIT {
    ($c:expr) => {
        ($c) >= b'0' as libc::c_char  && ($c) <= b'9' as libc::c_char
    };
}

#[macro_export]
macro_rules!  QUIT {
    () => {
        if read_volatile(&terminating_signal as *const i32) != 0{
            termsig_handler(read_volatile(&terminating_signal as *const i32));
        }
        if interrupt_state != 0{
            throw_to_top_level();
        }
    };
}


#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut(){
            free($s);
        }
    }
}

#[macro_export]
macro_rules! STREQN {
    ($a:expr,$b:expr,$n:expr) => {
        if $n == 0 {
            1
        }
        else{
            (*$a == *$b && strncmp($a,$b,$n) == 0) as i32
        }
    }
}

#[macro_export]
macro_rules! get_job_by_jid {
   ($ind:expr) => {
        (*((jobs as usize + ($ind*8) as usize ) as *mut*mut JOB) as *mut JOB)
    }
}

#[macro_export]
macro_rules! J_JOBSTATE {
   ($j:expr) => {
        (*$j).state
    }
}

//enum
#[repr(i8)]
#[derive(PartialEq)]
pub enum JOB_STATE {
    JNONE = -1,
    JRUNNING = 1,
    JSTOPPED = 2,
    JDEAD = 4,
    JMIXED = 8
}

//type
pub type sh_builtin_func_t = fn (*mut WordList)->i32;
pub type QSFUNC = unsafe extern "C" fn(*const c_void,*const c_void)->i32;


pub static EX_SUCCESS:i32 = 0;
pub static EX_USAGE:i32 = 258;
include!("./shell.rs");

//extern C
extern "C"{
    static interactive_shell:i32;
    static this_command_name:*mut c_char;
    static mut current_builtin:*mut builtin;
    static terminating_signal:c_int;
    static interrupt_state:c_int;
    static stdout:*mut FILE;
    static mut posparam_count:i32;
    static mut dollar_vars:[*mut c_char;10];
    static mut rest_of_args:*mut WordList;
    static variable_context:i32;
    static running_trap:i32;
    static trap_saved_exit_value:i32;
    static last_command_exit_value:i32;
    static no_symbolic_links:i32;
    // static bsah_getcwd_errstr:*const c_char;
    static js:jobstats;
    static jobs:*mut*mut JOB;
    static assoc_expand_once:i32;
    static shell_builtins:*mut builtin;
    static mut num_shell_builtins:i32;
    static posixly_correct:i32;


    fn get_name_for_error()->*mut c_char;
    fn executing_line_number()->i32;
    fn top_level_cleanup();
    fn jump_to_top_level(value:i32);
    fn internal_getopt(list:*mut WordList,opts:*mut c_char)->i32;
    fn reset_internal_getopt();
    fn termsig_handler(sig:i32);
    fn throw_to_top_level();
    fn fpurge(stream:*mut FILE) ->i32;
    fn strvec_from_word_list(list:*mut WordList,alloc:i32,starting_index:i32,ip:*mut i32)->*mut *mut c_char;
    fn xmalloc(n:size_t)->*mut c_void;
    fn dispose_words(list:*mut WordList);
    fn copy_word_list(list:*mut WordList)->*mut WordList;
    fn list_length(list:*mut GENERIC_LIST)->i32;
    fn invalidate_cached_quoted_dollar_at();
    fn set_builtin(list:*mut WordList)->i32;
    fn legal_number(string:*mut c_char,result:*mut c_long)->i32;
    fn return_builtin(list:*mut WordList)->i32;
    fn getcwd(buf:*mut c_char,size:size_t)->*mut c_char;
    fn internal_error(format:*const c_char,...);
    fn strcasestr(s1:*const c_char,s2:*const c_char)->*mut c_char;
    fn all_digits(string:*const c_char)->i32;
    fn valid_array_reference(name:*const c_char,flags:i32)->i32;
    fn bind_variable (name:* const c_char,value:* mut c_char,flags:i32)->* mut SHELL_VAR;
    fn assign_array_element(name:*mut c_char,value:*mut c_char,flags:i32)->*mut SHELL_VAR;
    fn find_variable(_:*const c_char)->*mut SHELL_VAR;
    fn unbind_variable(name:*const c_char)->i32;
    fn signal_name(sig:i32)->*mut c_char;
    fn kill_builtin(list:*mut WordList)->i32;
    fn decode_signal(string:*mut c_char,flags:i32)->i32;
    fn builtin_help();
    
    fn builtin_error(format:*const c_char,...);
}

unsafe fn ISOPTION(s:* const c_char, c:c_char)->bool
{
    // return *s == '-' as c_char && *((s as usize + 1)as * mut c_char) == c && *((s as usize + 8)as * mut c_char) != 0;
    return *s == '-' as c_char && *s.offset(1) == c && *s.offset(2) != 0;
}




/* Used by some builtins and the mainline code. */
pub static mut last_shell_builtin:*mut sh_builtin_func_t = std::ptr::null_mut();
pub static mut this_shell_builtin:*mut sh_builtin_func_t = std::ptr::null_mut();
 
/* **************************************************************** */
/*                                                                  */
/*           Error reporting, usage, and option processing          */
/*                                                                  */
/* **************************************************************** */

/* This is a lot like report_error (), but it is for shell builtins
   instead of shell control structures, and it won't ever exit the
   shell. */

#[no_mangle]
fn r_builitin_error_prolog(){
    let name:*mut c_char;

    unsafe{
        name = get_name_for_error();
        eprint!("{}: ",CStr::from_ptr(name).to_str().unwrap());

        if interactive_shell == 0{
            eprint!("line {}: ",executing_line_number())
        }

        if !this_command_name.is_null()  && *this_command_name!=0{
            eprint!("{}:",CStr::from_ptr(name).to_str().unwrap());
        }
    }
}

//builtin_error builtin_waring是可变参函数，先跳过


/* Print a usage summary for the currently-executing builtin command. */
#[no_mangle]
pub extern "C" fn r_builtin_usage(){
    unsafe{
        if !this_command_name.is_null() && *this_command_name != 0{
            eprint!("{}: usage: ",CStr::from_ptr(this_command_name).to_str().unwrap());
            eprintln!("{}",CStr::from_ptr((*current_builtin).short_doc).to_str().unwrap() );
            // stderr().flush();
        }
    }
    
}

/* Return if LIST is NULL else barf and jump to top_level.  Used by some
   builtins that do not accept arguments. */
#[no_mangle]
pub extern "C" fn r_no_args(list:*mut WordList){
    unsafe{
        if !list.is_null(){
            let c_str = CString::new("too many arguments").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
            top_level_cleanup();
            jump_to_top_level(DISCARD!());
        } 
    }
}

/* Check that no options were given to the currently-executing builtin,
   and return 0 if there were options. */
#[no_mangle]
pub extern "C" fn r_no_options(list:*mut WordList)->i32{
    let opt:i32;

    unsafe{
        reset_internal_getopt();
        let c_str = CString::new("").unwrap();
        let c_ptr = c_str.as_ptr(); 
        opt = internal_getopt(list,c_ptr as *mut libc::c_char);
        if opt != -1{
            if opt == GETOPT_HELP!(){
                builtin_help();
                return 2;
            }
            r_builtin_usage();
            return 1;
        }
        return 0;
    } 
}

#[no_mangle]
pub extern "C" fn r_sh_needarg(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: option requires an argument").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_neednumarg(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: numeric argument requited").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}


#[no_mangle]
pub extern "C" fn r_sh_notfound(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: not found").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }

}

/* Function called when one of the builtin commands detects an invalid
   option. */
#[no_mangle]
pub extern "C" fn r_sh_invalidopt(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: invalid option").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_invalidoptname(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: invalid option name").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_invalidid(s:*mut c_char){
    unsafe{
        let c_str = CString::new("`%s': not a valid identifier").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_invalidnum(s:*mut c_char){
    unsafe{
        // let msg:*mut c_char;
        let mut msg = String::new();
        let mut mag_ptr:*const c_char = std::ptr::null_mut();

        if *s == b'0' as libc::c_char && isdigit(*s.offset(1) as c_int) != 0{
            msg.push_str("invalid octal number");
            mag_ptr = msg.as_ptr() as *mut c_char;
        }
        else if *s == b'0' as libc::c_char && *s.offset(1) == b'x' as libc::c_char{
            msg.push_str("invalid hex number");
            mag_ptr = msg.as_ptr() as *mut c_char;
        }
        else {
            msg.push_str("invalid number");
            mag_ptr = msg.as_ptr() as *mut c_char;
        }

        let c_str = CString::new("%s: %s").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s,mag_ptr);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_invalidsig(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: invalid signal specification").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_badpid(s:*mut c_char){
    unsafe{
        let c_str = CString::new("`%s': not a pid or valid job spec").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_readonly(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: readonly variable").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_erange(s:*mut c_char,desc:*mut c_char){
    unsafe{
        if !s.is_null(){
            let c_str = CString::new("%s: %s out of range").unwrap();
            let c_ptr = c_str.as_ptr();
            if !desc.is_null(){
                builtin_error(c_ptr, s,desc);
            }
            else{
                let desc_str = CString::new("argument").unwrap();
                let desc_ptr = desc_str.as_ptr();
                builtin_error(c_ptr, s,desc_ptr);
            }
        }
        else{
            let c_str = CString::new("%s out of range").unwrap();
            let c_ptr = c_str.as_ptr();
            let desc_str = CString::new("argument").unwrap();
            let desc_ptr = desc_str.as_ptr();
            builtin_error(c_ptr,desc_ptr)
        }
    }
}

#[no_mangle]
pub extern "C" fn r_sh_badjob(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: no job control").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_nojobs(s:*mut c_char){
    unsafe{
        if !s.is_null(){
            let c_str = CString::new("%s: no job control").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr,s);
        }
        else{
            let c_str = CString::new("no job control").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn r_sh_restricted(s:*mut c_char){
    unsafe{
        if !s.is_null(){
            let c_str = CString::new("%s: restricted").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr,s);
        }
        else{
            let c_str = CString::new("restricted").unwrap();
            let c_ptr = c_str.as_ptr();
            builtin_error(c_ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn r_sh_notbuiltin(s:*mut c_char){
    unsafe{
        let c_str = CString::new("%s: not a shell builtin").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,s);
    }
}

#[no_mangle]
pub extern "C" fn r_sh_wrerror(){
    unsafe{
        let c_str = CString::new("write error: %s").unwrap();
        let c_ptr = c_str.as_ptr();
        builtin_error(c_ptr,strerror(*__errno_location()));
    }
}

#[no_mangle]
pub extern "C" fn r_sh_ttyerror(set:i32){
    unsafe{
        if set != 0{
            let c_str = CString::new("error setting terminal attributes: %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr,strerror(*__errno_location()));
        }
        else{
            let c_str = CString::new("error getting terminal attributes: %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr,strerror(*__errno_location()));
        }
    }
}

#[no_mangle]
pub extern "C" fn r_sh_chkwrite(s:i32)->i32{
    unsafe{
        QUIT!();
        fflush(stdout);
        QUIT!();

        if ferror(stdout) != 0{
            r_sh_wrerror();
            fpurge(stdout);
            clearerr(stdout);
            return EXECUTION_FAILURE!();
        }
        return s;
    }
}

/* **************************************************************** */
/*                                                                  */
/*           Shell positional parameter manipulation                */
/*                                                                  */
/* **************************************************************** */

/* Convert a WordList into a C-style argv.  Return the number of elements
   in the list in *IP, if IP is non-null.  A convenience function for
   loadable builtins; also used by `test'. */
#[no_mangle]
pub extern "C" fn r_make_builtin_argv(list:*mut WordList,ip:*mut i32)->*mut *mut c_char{
    let argv:*mut *mut c_char;
    unsafe{
        argv = strvec_from_word_list(list,0,1,ip);
        *argv.offset(0) = this_command_name;
        return argv;
    }
}

/* Remember LIST in $1 ... $9, and REST_OF_ARGS.  If DESTRUCTIVE is
   non-zero, then discard whatever the existing arguments are, else
   only discard the ones that are to be replaced.  Set POSPARAM_COUNT
   to the number of args assigned (length of LIST). */
#[no_mangle]
pub extern "C" fn r_remember_args(mut list:*mut WordList,destructive:i32){
    let mut i:i32;
    
    unsafe{
        posparam_count = 0;
        i = 1;
        while i<10{
            if (destructive !=0 || list != std::ptr::null_mut()) && *dollar_vars.as_ptr().offset(i as isize) != std::ptr::null_mut(){
                free(*dollar_vars.as_ptr().offset(i as isize) as *mut c_void);
                *dollar_vars.as_mut_ptr().offset(i as isize) = std::ptr::null_mut();
            }

            if !list.is_null(){
                posparam_count = i;
                *dollar_vars.as_mut_ptr().offset(posparam_count as isize) = r_savestring((*(*list).word).word);
                list = (*list).next;
            }
            i += 1;
        }

        /* If arguments remain, assign them to REST_OF_ARGS.
            Note that copy_word_list (NULL) returns NULL, and
            that dispose_words (NULL) does nothing. */
        if destructive !=0 || !list.is_null(){
            dispose_words(rest_of_args);
            rest_of_args = copy_word_list(list);
            posparam_count += list_length(list as *mut GENERIC_LIST);//there is may be problems
        }

        if destructive != 0{
            r_set_dollar_vars_changed();
        }
        invalidate_cached_quoted_dollar_at();
    }
}
 
#[no_mangle]
pub extern "C" fn r_shift_args(mut times:i32){
    let mut temp:*mut WordList;
    // let mut count:i32;

    unsafe{
        if times <= 0{
            return;
        }

        times -= 1;     //可能存在问题
        while times > 0 {
            if *dollar_vars.as_ptr().offset(1) != std::ptr::null_mut(){
                free(*dollar_vars.as_ptr().offset(1) as *mut c_void);
            }

            for count in 1..9{
                *dollar_vars.as_mut_ptr().offset(count) = *dollar_vars.as_ptr().offset(count+1)
            }

            if !rest_of_args.is_null(){
                temp = rest_of_args;
                *dollar_vars.as_mut_ptr().offset(9) = r_savestring((*(*temp).word).word);
                rest_of_args = (*rest_of_args).next;
                (*temp).next = std::ptr::null_mut();
                dispose_words(temp);
            }
            else{
                *dollar_vars.as_mut_ptr().offset(9) = std::ptr::null_mut();
            }

            posparam_count -= 1;

            times -= 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn r_number_of_args()->i32{
    unsafe{
        return posparam_count;
    }
}

static mut changed_dollar_vars:i32 = 0;

/* Have the dollar variables been reset to new values since we last
   checked? */

#[no_mangle]
pub extern "C" fn r_dollar_vars_changed()->i32{
    unsafe{
        return changed_dollar_vars;
    }
}

#[no_mangle]
pub extern "C" fn r_set_dollar_vars_unchanged(){
    unsafe{
        changed_dollar_vars = 0;
    } 
}

#[no_mangle]
pub extern "C" fn r_set_dollar_vars_changed(){
    unsafe{
        if variable_context != 0{
            changed_dollar_vars |= ARGS_FUNC!();
        }
        else if this_shell_builtin == set_builtin as *mut sh_builtin_func_t{     //there may be problems
            changed_dollar_vars |= ARGS_SETBLTIN!();
        }
        else{
            changed_dollar_vars |= ARGS_INVOC!();
        }
    }
}


/* **************************************************************** */
/*                                                                  */
/*              Validating numeric input and arguments              */
/*                                                                  */
/* **************************************************************** */

/* Read a numeric arg for this_command_name, the name of the shell builtin
   that wants it.  LIST is the word list that the arg is to come from.
   Accept only the numeric argument; report an error if other arguments
   follow.  If FATAL is 1, call throw_to_top_level, which exits the
   shell; if it's 2, call jump_to_top_level (DISCARD), which aborts the
   current command; if FATAL is 0, return an indication of an invalid
   number by setting *NUMOK == 0 and return -1. */
#[no_mangle]
pub extern "C" fn r_get_numeric_arg(mut list:*mut WordList,fatal:i32,count:*mut intmax_t)->i32{
    let arg:*mut c_char;
    unsafe{
        if !count.is_null(){
            *count = 1;
        }

        if !list.is_null() && !(*list).word.is_null() && ISOPTION((*(*list).word).word,b'-' as libc::c_char){
            list = (*list).next;
        }

        if !list.is_null(){
            arg = (*(*list).word).word;
            if arg.is_null() || legal_number(arg,count) == 0{
                if !(*(*list).word).word.is_null(){
                    r_sh_neednumarg((*(*list).word).word);
                }
                else {
                    r_sh_neednumarg(String::from("`'").as_ptr() as *mut libc::c_char);
                }
                
                if fatal == 0{
                    return 0;
                }
                else if fatal == 1{             /* fatal == 1; abort */
                    throw_to_top_level();
                }
                else{                           /* fatal == 2; discard current command */
                    top_level_cleanup();
                    jump_to_top_level(DISCARD!());
                }
            }
            r_no_args((*list).next);
        }
        return 1;
    }
}

/* Get an eight-bit status value from LIST */
#[no_mangle]
pub extern "C" fn r_get_exitstat(mut list:*mut WordList)->i32{
    let status:i32;
    let mut sval:intmax_t = 0;
    let arg:*mut c_char;

    unsafe{
        if !list.is_null() && !(*list).word.is_null() && ISOPTION((*(*list).word).word,b'-' as libc::c_char){
            list = (*list).next;
        }

        if list.is_null(){
             /* If we're not running the DEBUG trap, the return builtin, when not
                given any arguments, uses the value of $? before the trap ran.  If
                given an argument, return uses it.  This means that the trap can't
                change $?.  The DEBUG trap gets to change $?, though, since that is
                part of its reason for existing, and because the extended debug mode
                does things with the return value. */
            if this_shell_builtin == return_builtin as *mut sh_builtin_func_t  && running_trap > 0 && running_trap != DEBUG_TRAP!()+1{
                return trap_saved_exit_value;
            }
            return last_command_exit_value;
        }

        arg = (*(*list).word).word;
        if arg.is_null() || legal_number(arg,&mut sval) == 0{
            if !(*(*list).word).word.is_null(){
                r_sh_neednumarg((*(*list).word).word);
            }
            else {
                r_sh_neednumarg(String::from("`'").as_ptr() as *mut libc::c_char);
            }

            return EX_BADUSAGE!();
        }

        r_no_args((*list).next);

        status = (sval & 255) as i32;
        return status;
    }
} 

/* Return the octal number parsed from STRING, or -1 to indicate
   that the string contained a bad number. */
#[no_mangle]
pub extern "C" fn r_read_octal(mut string:*mut c_char)->i32{
    let mut result:i32 = 0;
    let mut digits:i32 = 0;

    unsafe{
        while *string!=0 && ISOCTAL!(*string){
            digits += 1;
            result = (result * 8) + (*string - b'0' as libc::c_char) as i32;
            string = (string as usize + 1 ) as *mut c_char;
            if result > 0o7777{
                return -1;
            }
        }

        if digits == 0 || *string != 0{
            result = -1;
        }

        return result;
    }
}


/* **************************************************************** */
/*                                                                  */
/*           Manipulating the current working directory             */
/*                                                                  */
/* **************************************************************** */

/* Return a consed string which is the current working directory.
   FOR_WHOM is the name of the caller for error printing.  */

pub static mut the_current_working_directory:*mut c_char = std::ptr::null_mut();

#[no_mangle]
pub extern "C" fn r_get_working_directory(for_whom:*mut c_char)->*mut c_char{
    unsafe{
        if no_symbolic_links != 0{
            FREE!(the_current_working_directory as *mut c_void);
            the_current_working_directory = std::ptr::null_mut();
        }

        if the_current_working_directory.is_null(){
            the_current_working_directory = getcwd(0 as *mut c_char,0);
            
            if the_current_working_directory.is_null(){
                let strerror_str = CStr::from_ptr(strerror(errno()));
                let strerror_string = strerror_str.to_str().unwrap().to_owned();
                let bash_getcwd_errstr = String::from("getcwd: cannot access parent directories");
                if !for_whom.is_null() && *for_whom!=0{
                    let for_whom_str = CStr::from_ptr(for_whom);
                    let for_whom_string = for_whom_str.to_str().unwrap().to_owned();
                    eprintln!("{}: error retrieving current directory: {}: {}",for_whom_string,bash_getcwd_errstr,strerror_string);
                }
                else{
                    let for_whom_str = CStr::from_ptr(get_name_for_error());
                    let for_whom_string = for_whom_str.to_str().unwrap().to_owned();
                    eprintln!("{}: error retrieving current directory: {}: {}",for_whom_string,bash_getcwd_errstr,strerror_string);
                }
                return std::ptr::null_mut();
            }
        }
        return r_savestring(the_current_working_directory);
    }
}

#[no_mangle]
pub extern "C" fn r_set_working_dierctory(name:*mut c_char){
    unsafe{
        FREE!(the_current_working_directory as *mut c_void);
        the_current_working_directory = r_savestring(name);
    } 
}

/* **************************************************************** */
/*                                                                  */
/*              Job control support functions                       */
/*                                                                  */
/* **************************************************************** */
#[no_mangle]
pub extern "C" fn r_get_job_by_name(name:*const c_char,flags:i32)->i32{
    let mut i:i32;
    let wl:i32;
    let mut cl:i32;
    let mut match_0:i32;
    let mut job:i32;
    let mut p:*mut PROCESS;
    let mut j:*mut JOB;

    unsafe{
        job = NO_JOB!();
        wl = strlen(name) as i32;

        i = js.j_jobslots - 1;
        while i >= 0{
            j = get_job_by_jid!(i);
            if j.is_null() || ( flags & JM_STOPPED!() != 0 && J_JOBSTATE!(j) != JOB_STATE::JSTOPPED){
                continue;
            }

            p = (*j).pipe;

            loop{
                if (flags & JM_EXACT!()) != 0{
                    cl = strlen((*p).command) as i32;
                    match_0 = STREQN!((*p).command,name,cl as usize);
                }
                else if (flags & JM_SUBSTRING!()) != 0{
                    match_0 = (strcasestr((*p).command,name) != 0 as *mut c_char) as i32;
                }
                else{
                    match_0 = STREQN!((*p).command,name,wl as usize);
                }

                if match_0 == 0{
                    p = (*p).next;
                    continue;
                }
                else if flags & JM_FIRSTMATCH!() != 0{
                    return i;
                }
                else if job != NO_JOB!(){
                    if !this_shell_builtin.is_null(){
                        let c_str = CString::new("%s: ambiguous job spece").unwrap();
                        let c_str_ptr = c_str.as_ptr();
                        builtin_error(c_str_ptr,name);
                    } 
                    else{
                        let c_str = CString::new("%s: ambiguous job spece").unwrap();
                        let c_str_ptr = c_str.as_ptr();
                        internal_error(c_str_ptr,name)
                    }
                    return DUP_JOB!();
                }
                else{
                    job = i;
                }

                if p == (*j).pipe{
                    break;
                }
            }

            i -= 1;
        }
        return job;
    }
}

/* Return the job spec found in LIST. */
#[no_mangle]
pub extern "C" fn r_get_job_spec(list:*mut WordList)->i32{
    let mut word:*mut c_char;
    let job:i32;
    let mut jflags:i32;

    unsafe{
        if list.is_null(){
            return js.j_current;
        }

        word = (*(*list).word).word;

        if *word.offset(0) == '\0' as libc::c_char {
            return NO_JOB!();
        }

        if *word.offset(0) == '%' as libc::c_char {
            word = word.offset(1);
        }

        if DIGIT!(*word) && all_digits(word) != 0{
           job = atoi(word);
           if job<0 || job>js.j_jobslots{
               return NO_JOB!();
           } 
           else{
               return job -1;
           }
        }

        jflags = 0;
        let opt = word.offset(0) as u8;
        let opt_char = char::from(opt);
        match opt_char{
            '\0'|'%' | '+' => return js.j_current,
            '-' => return js.j_previous,
            '?' => {
                jflags |= JM_SUBSTRING!();
                word = word.offset(1);
            }
            _ => {},
        }
        return r_get_job_by_name(word, jflags);
    }
}

/*
 * NOTE:  `kill' calls this function with forcecols == 0
 */
pub extern "C" fn r_display_signal_list(mut list:*mut WordList,forcecols:i32)->i32{
    let mut i:i32;
    let mut column:i32;
    let mut name:*mut c_char;
    let mut result:i32;
    let mut signum:i32;
    let mut dflags:i32;
    let mut lsignum:intmax_t = 0;

    unsafe{
        result = EXECUTION_SUCCESS!();
        if list.is_null(){
            column = 0;
            for i in 1..NSIG!() {
                name = signal_name(i);
                if STREQN!(name,String::from("SIGJUNK").as_ptr() as *mut c_char,7) != 0|| 
                   STREQN!(name,String::from("Unknown").as_ptr() as *mut c_char,7) != 0 {
                    continue;
                }

                if posixly_correct != 0 && forcecols != 0{
                    /* This is for the kill builtin.  POSIX.2 says the signal names
                     are displayed without the `SIG' prefix. */
                    if STREQN!(name,String::from("SIG").as_ptr() as *mut c_char,3) != 0{
                        name = name.offset(3);
                        if i == NSIG!() - 1{
                            print!("{}",CStr::from_ptr(name).to_str().unwrap().to_owned());
                        }
                        else {
                            print!("{} ",CStr::from_ptr(name).to_str().unwrap().to_owned());
                        }
                    }
                }
                else{
                    print!("{:02} {}",i,CStr::from_ptr(name).to_str().unwrap().to_owned());
                    
                    column += 1;
                    if column < 5{
                        print!{"\t"};
                    }
                    else{
                        print!("\n");
                        column = 0;
                    }
                }
            }

            if posixly_correct != 0 && forcecols != 0 || column != 0{
                print!("\n");
            }
            return result;
        }   //if list.is_null()

        /* List individual signal names or numbers. */
        while !list.is_null(){
            if legal_number((*(*list).word).word,&mut lsignum) != 0{
                /* This is specified by Posix.2 so that exit statuses can be
                 mapped into signal numbers. */
                if lsignum > 128{
                    lsignum -= 128;
                }
                if lsignum<0 || lsignum >= NSIG!(){
                    r_sh_invalidsig((*(*list).word).word);
                    result = EXECUTION_FAILURE!();
                    list = (*list).next;
                    continue;
                }

                signum = lsignum as i32;
                name = signal_name(signum);
                if STREQN!(name,String::from("SIGJUNK").as_ptr() as *mut c_char,7) != 0||
                   STREQN!(name,String::from("Unknow").as_ptr() as *mut c_char,7) != 0{
                    list = (*list).next;
                    continue;
                }
                /* POSIX.2 says that `kill -l signum' prints the signal name without
                 the `SIG' prefix. */ 
                if this_shell_builtin == kill_builtin as *mut sh_builtin_func_t && signum > 0{
                    // name = name.offset(3);
                    println!("{}",CStr::from_ptr(name.offset(3)).to_str().unwrap().to_owned());
                }
                else {
                    println!("{}",CStr::from_ptr(name).to_str().unwrap().to_owned());
                }
            }
            else{
                dflags = DSIG_NOCASE!();
                if posixly_correct == 0 || this_shell_builtin != kill_builtin as *mut sh_builtin_func_t{
                    dflags |= DSIG_SIGPREFIX!();
                }
                signum = decode_signal((*(*list).word).word,dflags);
                if signum == NO_SIG!(){
                    r_sh_invalidsig((*(*list).word).word);
                    result = EXECUTION_FAILURE!();
                    list = (*list).next;
                    continue;
                }
                println!("{}",signum);
            }
            list = (*list).next;
        }//while
        return result;
    }
}


/* **************************************************************** */
/*                                                                  */
/*          Finding builtin commands and their functions            */
/*                                                                  */
/* **************************************************************** */

/* Perform a binary search and return the address of the builtin function
   whose name is NAME.  If the function couldn't be found, or the builtin
   is disabled or has no function associated with it, return NULL.
   Return the address of the builtin.
   DISABLED_OKAY means find it even if the builtin is disabled. */

extern "C" fn r_print_builtin_name() {
    let mut hi:i32;
    let mut lo:i32;
    let mut mid:i32 = 0;
    let mut j:i32;

    unsafe{
        hi = num_shell_builtins -1;
        lo = 0;

        while lo <= hi {
            //printf(b" builtin command name is :%s\n", (*shell_builtins.offset(mid as isize)).name);
        }
    }
}

extern "C" fn r_builtin_address_internal(name:*mut c_char,disabled_okay:i32)->*mut builtin{
    let mut hi:i32;
    let mut lo:i32;
    let mut mid:i32 = 0;
    let mut j:i32;

    unsafe{
        hi = num_shell_builtins -1;
        lo = 0;

        while lo <= hi {
            mid = (lo + hi) / 2;
            
            j = (*((*shell_builtins.offset(mid as isize)).name).offset(0) - *name.offset(0)) as i32;
            
            if j==0{
                j = strcmp((*shell_builtins.offset(mid as isize)).name,name);
            }

            if j==0{
                /* It must have a function pointer.  It must be enabled, or we
                 must have explicitly allowed disabled functions to be found,
                 and it must not have been deleted. */
                if !(*shell_builtins.offset(mid as isize)).function.is_null() &&
                    (*shell_builtins.offset(mid as isize)).flags & BUILTIN_DELETED!() == 0 &&
                    (*shell_builtins.offset(mid as isize)).flags & BUILTIN_ENABLED!() != 1 ||
                    disabled_okay != 0{
                        return &mut *shell_builtins.offset(mid as isize);
                }
                else {
                    return 0 as *mut builtin;
                }
            }

            if j > 0{
                hi = mid -1;
            }
            else{
                lo = mid + 1;
            }
        }

        return 0 as *mut builtin;
    }
}

/* Return the pointer to the function implementing builtin command NAME. */
pub extern "C" fn r_find_shell_builtin(name:*mut c_char)->*mut sh_builtin_func_t{
    unsafe{
        // println!("222");
        current_builtin = r_builtin_address_internal(name,0);
        if !current_builtin.is_null(){
            return (*current_builtin).function;
        }
        else{
            return 0 as *mut sh_builtin_func_t;
        }
    }
}

/* Return the address of builtin with NAME, whether it is enabled or not. */
pub extern "C" fn r_builtin_address(name:*mut c_char)->*mut sh_builtin_func_t{
    unsafe{
        current_builtin = r_builtin_address_internal(name,1);
        if !current_builtin.is_null(){
            return (*current_builtin).function;
        }
        else{
            return 0 as *mut sh_builtin_func_t;
        }
    }
}

/* Return the function implementing the builtin NAME, but only if it is a
   POSIX.2 special builtin. */
#[no_mangle]
pub extern "C" fn r_find_special_builtin(name:*mut c_char)->*mut sh_builtin_func_t{
    unsafe{
        current_builtin = r_builtin_address_internal(name,0);

        if !current_builtin.is_null() && (*current_builtin).flags & SPECIAL_BUILTIN!() != 0{
            return (*current_builtin).function;
        }
        else {
            return 0 as *mut sh_builtin_func_t;
        }
    }
}

#[no_mangle]
extern "C" fn r_shell_builtin_compare(sbp1:*mut builtin,sbp2:*mut builtin)->i32{
    let mut result:i32;
    
    unsafe{
        result = (*((*sbp1).name).offset(0) - *((*sbp2).name).offset(0)) as i32;
        if result == 0{
            result = strcmp((*sbp1).name, (*sbp2).name);
        }
        return result;
    }
}

/* Sort the table of shell builtins so that the binary search will work
   in find_shell_builtin. */
#[no_mangle]
pub extern "C" fn r_initialize_shell_builtins(){
    unsafe{
        qsort(shell_builtins as *mut c_void,
              num_shell_builtins as usize,
              size_of::<builtin>(),
              std::mem::transmute::<
                  Option::<unsafe extern "C" fn() -> libc::c_int>,
                  Option::<QSFUNC>
                >(
                    Some(
                        std::mem::transmute::<
                            unsafe extern "C" fn(*mut builtin, *mut builtin) -> c_int,
                            unsafe extern "C" fn() -> c_int,
                        >(r_shell_builtin_compare),
                    ),
                ),
            );
    }
}


/* **************************************************************** */
/*                                                                  */
/*          Variable assignments during builtin commands            */
/*                                                                  */
/* **************************************************************** */
#[no_mangle]
pub extern "C" fn r_builtin_bind_variable(name:*mut c_char,value:*mut c_char,flags:i32)->*mut SHELL_VAR{
    let mut v:*mut SHELL_VAR;

    unsafe{
        let opt:i32;
        if assoc_expand_once != 0{
            opt = VA_NOEXPAND!() | VA_ONEWORD!();
        }
        else{
            opt = 0;
        }

        if valid_array_reference(name,opt) == 0{
            v = bind_variable(name,value,flags);
        }
        else{
            v = assign_array_element(
                    name,
                    value,
                    flags 
                        | (if assoc_expand_once != 0{
                            ASS_NOEXPAND!()
                        } else{
                            0
                        }),  
            );
        }

        if !v.is_null() && readonly_p!(v)==0 && noassign_p!(v) ==0 {
            VUNSETATTR!(v,att_invisible!());
        }
        return v;
    }
}

/* Like check_unbind_variable, but for use by builtins (only matters for
   error messages). */
pub extern "C" fn r_builtin_unbind_variable(vname:*const c_char)->i32{
    let v:*mut SHELL_VAR;

    unsafe{
        v = find_variable(vname);
        if !v.is_null() && readonly_p!(v) != 0{  
            let c_str = CString::new("%s: cannot unset: readonly %s").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr,vname,"variable");
            return -2;
        }
        
        else if !v.is_null() && non_unsettable_p!(v) != 0{
            let c_str = CString::new("%s: cannot unset").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr,vname);
            return -2
        }

        return unbind_variable(vname);
    }
}

pub extern "C" fn get_local_str()-> Vec<LanguageIdentifier>{

    let  lang : String;
    match var("LANG") {
        Ok(v) => lang = v ,
        Err(e) => 
        {
            lang = String::from("en-US");
            println!("err is {e:?}")
        },
    }
   // println!("now language is {:?}",lang);
    //parse() 用于类型转换
    let v: Vec<_> = lang.split('.').collect();
    let langid : LanguageIdentifier = v[0].parse().expect("wrong language");
    let locales = vec![langid.into()];
    return locales; 
  }

pub unsafe fn r_savestring(x:* const c_char)->* mut c_char
{
    let len = 1+libc::strlen(x);
  let str1:* mut c_char=libc::malloc(len) as * mut c_char;
  libc::memset(str1 as *mut libc::c_void, 0, len);
  return libc::strcpy(str1 as *mut c_char ,x);
}


pub unsafe fn err_translate_fn (command:&String , args1 : *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec![ "message.ftl".into() ];
    let mut args = FluentArgs::new();
    if args1 !=  std::ptr::null_mut(){
        args.set("str1",format!("{:?}",CStr::from_ptr(args1).to_str().unwrap()));
    }

    let bundle = mgr.get_bundle(get_local_str(), resources);
    let mut value = bundle.get_message(command).unwrap();
    let mut pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 !=  std::ptr::null_mut(){
        let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        eprint!("{msg1}");
    }
    else {
        let mut msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        eprint!("{msg1}");
    } 
}

pub unsafe fn translate_fn (command:&String , args1 : *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec![ "message.ftl".into() ];
    let mut args = FluentArgs::new();
    if args1 !=  std::ptr::null_mut(){
        args.set("str1",format!("{:?}",CStr::from_ptr(args1).to_str().unwrap()));
    }

    let bundle = mgr.get_bundle(get_local_str(), resources);
    let mut value = bundle.get_message(command).unwrap();
    let mut pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 !=  std::ptr::null_mut(){
        let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        print!("{msg1}");
    }
    else {
        let mut msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        print!("{msg1}");
    } 
}
