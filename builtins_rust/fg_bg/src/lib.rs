extern crate  libc;
extern crate nix;

use libc::{c_char, c_long};
use std::{ffi::CString, ops::Add};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE,r_builtin_usage};
use rhelp::r_builtin_help;

#[repr(i8)]
pub enum JOB_STATE {
    JNONE = -1,
    JRUNNING = 1,
    JSTOPPED = 2,
    JDEAD = 4,
    JMIXED = 8
}

#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
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
pub struct PROCESS {
    next: *mut PROCESS,
    pid:libc::c_int,
    status:libc::c_int,
    running:libc::c_int,
    command:*mut c_char
}



#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,	/* Next element, or NULL. */
  redirector:libc::c_int, 	/* Descriptor or varname to be redirected. */
  rflags:libc::c_int,			/* Private flags for this redirection */
  flags:libc::c_int,			/* Flag value for `open'. */
  instruction:r_instruction, /* What to do with the information. */
  redirectee:libc::c_int,	/* File descriptor or filename */
  here_doc_eof:*mut c_char		/* The word that appeared in <<foo. */
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
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:libc::c_int
}

#[repr(C)]
pub struct case_com {
    flags:libc::c_int,
    line:libc::c_int,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
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

#[repr(C)]
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND
}

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
pub struct jobstats {
    /* limits */
    c_childmax:libc::c_long,
    /* child process statistics */
    c_living:libc::c_int,		/* running or stopped child processes */
    c_reaped:libc::c_int,	/* exited child processes still in jobs list */
    c_injobs:libc::c_int,	/* total number of child processes in jobs list */
    /* child process totals */
    c_totforked:libc::c_int,	/* total number of children this shell has forked */
    c_totreaped:libc::c_int,	/* total number of children this shell has reaped */
    /* job counters and indices */
    j_jobslots:libc::c_int,/* total size of jobs array */
    j_lastj:libc::c_int,		/* last (newest) job allocated */
    j_firstj:libc::c_int,	/* first (oldest) job allocated */
    j_njobs:libc::c_int,		/* number of non-NULL jobs in jobs array */
    j_ndead:libc::c_int,		/* number of JDEAD jobs in jobs array */
    /* */
    j_current:libc::c_int,	/* current job */
    j_previous:libc::c_int,	/* previous job */
    /* */
    j_lastmade:* mut JOB,	/* last job allocated by stop_pipeline */
    j_lastasync:* mut JOB	/* last async job allocated by stop_pipeline */
}


#[macro_export]
macro_rules! BLOCK_SIGNAL {
   ($sig:expr, $nvar:expr, $ovar:expr) => {
        $nvar.unwrap().clear();
        $nvar.unwrap().add($sig);
        $nvar.unwrap().clear();
        nix::sys::signal::sigprocmask(nix::sys::signal::SigmaskHow::SIG_BLOCK,  $nvar, $ovar);   
   }
}

#[macro_export]
macro_rules! UNBLOCK_SIGNAL {
   ($ovar:expr) => {
        nix::sys::signal::sigprocmask(nix::sys::signal::SigmaskHow::SIG_SETMASK,  $ovar, None)
   }
}

#[macro_export]
macro_rules! UNBLOCK_CHILD {
   ($ovar:expr) => {
    UNBLOCK_SIGNAL!($ovar);   
   }
}

#[macro_export]
macro_rules! BLOCK_CHILD {
   ($nvar:expr,$ovar:expr) => {
    BLOCK_SIGNAL!(nix::sys::signal::SIGCHLD, $nvar, $ovar);  
   }
}

#[macro_export]
macro_rules! DUP_JOB {
   () => {-2}
}

#[macro_export]
macro_rules! get_job_by_jid {
   ($ind:expr) => {
    (*(((jobs as usize) + ($ind*8) as usize ) as *mut*mut JOB) as *mut JOB)
    }
}

#[macro_export]
macro_rules! J_JOBCONTROL {
   () => {0x04}
}

#[macro_export]
macro_rules! IS_JOBCONTROL {
   ($j:expr) => {
      ((*get_job_by_jid!($j)).flags & J_JOBCONTROL!()) != 0
    }
}

#[macro_export]
macro_rules! INVALID_JOB {
   ($j:expr) => {
         $j <0 || $j >=  js.j_jobslots || get_job_by_jid !($j) == std::ptr::null_mut()
    }
}

#[macro_export]
macro_rules! ISHELP {
   ($s:expr) => {
    libc::strcmp($s as *const c_char,CString::new("--help").unwrap().as_ptr())
    }
}

#[macro_export]
macro_rules! CHECK_HELPOPT {
  ($l:expr) => {
    if $l  !=std::ptr::null_mut() && (*$l).word !=std::ptr::null_mut() && ISHELP!((*(*$l).word).word) == 0 {
      r_builtin_help ();
      return EX_USAGE;
    }
  }
}

extern "C" {  
    fn builtin_error(err:*const c_char,...);  
    fn get_job_spec (list:*mut WordList)->i32;
    fn sh_badjob (str:*mut c_char);
    static jobs:*mut*mut JOB;    
    static  js:jobstats ;
    
    static mut loptend:*mut WordList; 
    fn sh_nojobs (str:*mut c_char);
    fn no_options (list:*mut WordList)->i32;
    static mut job_control:i32;
    static mut last_asynchronous_pid:i32;
    fn start_job (job:i32, foreground:i32)->i32;
}

/* How to bring a job into the foreground. */
#[no_mangle]
pub extern "C" fn  r_fg_builtin (list:*mut WordList)->i32 {
  let fg_bit:i32;
  unsafe {
    CHECK_HELPOPT! (list);

    if job_control == 0 {
        sh_nojobs (0 as *mut c_char);
        return EXECUTION_FAILURE!();
    }

    if no_options (list) !=0 {
      return EX_USAGE;
    } 
    
    /* If the last arg on the line is '&', then start this job in the
      background.  Else, fg the job. */
    
    if loptend  == std::ptr::null_mut() {
      return r_fg_bg (loptend, 1);
    } else {
      let mut t:WordList=*loptend;
      while  t.next !=std::ptr::null_mut() {
        t=*(t.next);
      }
      let cstr:&std::ffi::CStr=std::ffi::CStr::from_ptr((*(t.word)).word );
      let mut isfg:bool=char::from( cstr.to_bytes()[0] ) == '&';
      isfg =isfg && char::from( cstr.to_bytes()[1])  == '\0';
      isfg = isfg ==false;
      if isfg {
        fg_bit=1;
      } else {
        fg_bit=0;
      }    
      return r_fg_bg (loptend, fg_bit);
    }
  }
}

/* How to put a job into the background. */
#[no_mangle]
pub extern "C" fn  r_bg_builtin (list:*mut WordList)->i32 {
  let mut r:i32;
  unsafe {
  CHECK_HELPOPT !(list);

  if job_control == 0  {
      sh_nojobs (0 as *mut c_char);
      return EXECUTION_FAILURE!();
  }

  if no_options (list) !=0 {
    return EX_USAGE;
  }
    
  /* This relies on the fact that fg_bg() takes a WordList *, but only acts
     on the first member (if any) of that list. */
  r = EXECUTION_SUCCESS!();

  if r_fg_bg(loptend,0) == EXECUTION_FAILURE!() {
    r = EXECUTION_FAILURE!();
  }
  
  if loptend  !=std::ptr::null_mut() {
      let mut t:WordList=*loptend;
      while t.next !=std::ptr::null_mut() {
        if r_fg_bg (&mut t, 0) == EXECUTION_FAILURE!() {
          r = EXECUTION_FAILURE!();
        }
        t = *(t.next);
      }
      return r;
  } else {
    return r;
  }	
  }
}

/* How to put a job into the foreground/background. */
#[no_mangle]
pub extern "C" fn r_fg_bg (list:*mut WordList, foreground:i32)->i32{
  
  let mut set:nix::sys::signal::SigSet=nix::sys::signal::SigSet::empty();
  let mut oset:nix::sys::signal::SigSet =nix::sys::signal::SigSet::empty();
  let job:i32;
  let status:i32;
  let mut old_async_pid:i32=0;
  let j:*mut JOB;
  
  unsafe {
  BLOCK_CHILD !(Some(&mut set), Some(&mut oset));      
  job = get_job_spec (list);

  if INVALID_JOB !(job) {
    if job != DUP_JOB!() {
      if list != std::ptr::null_mut() {
        sh_badjob ( (*(*list).word).word );
      } else {
        let mut c_str_current = CString::new("current").unwrap(); // from a &str, creates a new allocation
        sh_badjob (c_str_current.as_ptr() as * mut c_char);
      }
    }

    UNBLOCK_CHILD !(Some(&oset));
    return EXECUTION_FAILURE!();
  }

  j = get_job_by_jid !(job);
  /* Or if j->pgrp == shell_pgrp. */
  if ! IS_JOBCONTROL !(job) {       
      let jobNum:i32=job + 1;
      builtin_error ( String::from("job ").add(&jobNum.to_string()).add(&String::from("started without job control").to_string()).as_ptr() as * const c_char);
      UNBLOCK_CHILD !(Some(&oset));
      return EXECUTION_FAILURE!();
  }

  if foreground == 0 {
      old_async_pid = i32::from(last_asynchronous_pid);
      last_asynchronous_pid = i32::from((*j).pgrp);	/* As per Posix.2 5.4.2 */
  }

  status = start_job (job, foreground);

  if status >= 0 {
    /* win: */
      UNBLOCK_CHILD !(Some(&oset));
      if foreground !=0 {
        return status;
      } else {
        return  EXECUTION_SUCCESS!();
      }
    } else {
      if foreground == 0 {
        last_asynchronous_pid = i32::from(old_async_pid);
      }

      UNBLOCK_CHILD !(Some(&oset));
      return EXECUTION_FAILURE!();
    }
  }
}

/*
#[no_mangle]
pub extern "C" fn cmd_name() ->*const u8 {
   return b"fg" as *const u8;
}
#[no_mangle]
pub extern "C" fn run(list : *mut WordList)->i32 {
  return r_fg_builtin(list);
}
*/
