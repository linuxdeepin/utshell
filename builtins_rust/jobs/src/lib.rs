extern crate  libc;
extern crate nix;

use libc::{c_char, c_long};
use std::ffi::CString;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
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
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest:libc::c_int,			/* Place to redirect REDIRECTOR to, or ... */
    filename:* mut WordDesc 		/* filename to redirect to. */
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,	/* Next element, or NULL. */
  redirector:REDIRECTEE, 	/* Descriptor or varname to be redirected. */
  rflags:libc::c_int,			/* Private flags for this redirection */
  flags:libc::c_int,			/* Flag value for `open'. */
  instruction:r_instruction, /* What to do with the information. */
  redirectee:REDIRECTEE,	/* File descriptor or filename */
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
macro_rules! JLIST_STANDARD {
   () => {0}
}

#[macro_export]
macro_rules! JSTATE_ANY {
   () => {0x0}
}

#[macro_export]
macro_rules! JLIST_LONG {
   () => {1}
}

#[macro_export]
macro_rules! JLIST_PID_ONLY {
   () => {2}
}

#[macro_export]
macro_rules! JLIST_CHANGED_ONLY {
   () => {3}
}


#[macro_export]
macro_rules! JSTATE_RUNNING {
   () => {0x1}
}

#[macro_export]
macro_rules! JSTATE_STOPPED {
   () => {0x2}
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
macro_rules! NO_JOB {
   () => {-1}
}

#[macro_export]
macro_rules! DUP_JOB {
   () => {-2}
}

#[macro_export]
macro_rules! CMD_INHIBIT_EXPANSION {/* Do not expand the command words. */
   () => {0x20}
}

#[macro_export]
macro_rules! get_job_by_jid {
   ($ind:expr) => {
    (*((jobs as usize + ($ind*8) as usize ) as *mut*mut JOB) as *mut JOB)
    }
}

#[macro_export]
macro_rules! INVALID_JOB {
   ($j:expr) => {
         $j <0 || $j >=  js.j_jobslots || get_job_by_jid!($j) == std::ptr::null_mut()
    }
}

extern "C" {
    fn list_running_jobs(format:i32);
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
    fn builtin_error(err:*const c_char,...);
    fn builtin_usage();
    static mut loptend:*mut WordList;
    fn list_all_jobs(form:i32);
    fn list_stopped_jobs(form:i32);
    fn list_one_job (jjob:*mut JOB, format:i32, ignore:i32, job_index:i32);
    fn get_job_spec (list:*mut WordList)->i32;
    fn sh_badjob (str:*mut c_char);
    static jobs:*mut*mut JOB;
    fn discard_unwind_frame (str: * mut c_char);  
    fn begin_unwind_frame (str: * mut c_char);
    fn execute_command (command:* mut COMMAND)->i32;
    fn dispose_command (command:* mut COMMAND);
    fn make_bare_simple_command ()->* mut COMMAND;
    fn copy_word_list (list:*mut WordList)->* mut WordList;
    static  js:jobstats ;
    fn add_unwind_protect(_:unsafe extern "C" fn(command:* mut COMMAND),...);
    fn legal_number(str:* const c_char, result:* mut c_long)->i32;
    fn get_job_by_pid (pid:i32, block:i32, ignore:*mut*mut PROCESS)->i32;
    fn delete_job (job_index:i32, dflags:i32);
    fn nohup_job (job_index:i32);
    fn nohup_all_jobs (running_only:i32);
    fn delete_all_jobs(running_only:i32);
 }

 #[no_mangle]
 pub extern "C" fn r_execute_list_with_replacements (list:*mut WordList)->i32{
  unsafe{
  let mut l:*mut WordList=list;
  let mut job:i32;
  let result:i32;
  let command:*mut COMMAND;
   
  /* First do the replacement of job specifications with pids. */  
  while l !=std::ptr::null_mut() {
      let lchar:char=char::from((*(*(*l).word).word) as u8);
      if  lchar== '%'	/* we have a winner */	{
	  job = get_job_spec ( l);
	  /* A bad job spec is not really a job spec! Pass it through. */
	  if INVALID_JOB!(job){
        continue;
      }
	    
	  libc::free((*(*l).word).word as * mut libc::c_void);
	  (*(*(*l).word).word) = (*get_job_by_jid!(job)).pgrp as libc::c_char;
	  }
      l=(*l).next;
  }
   
  let mut c_str_jobs_builtin = CString::new("jobs_builtin").unwrap();
  /* Next make a new simple command and execute it. */
  begin_unwind_frame (c_str_jobs_builtin.as_ptr() as * mut c_char);

  command = make_bare_simple_command ();
  (*((*command).value.Simple)).words= copy_word_list (list);
  (*((*command).value.Simple)).redirects = std::ptr::null_mut();
  (*command).flags |= CMD_INHIBIT_EXPANSION!();
  (*((*command).value.Simple)).flags |= CMD_INHIBIT_EXPANSION!();

  add_unwind_protect(dispose_command, command);
  result = execute_command (command);
  dispose_command (command);
  discard_unwind_frame (c_str_jobs_builtin.as_ptr() as * mut c_char);
  return result;
  }
}

#[no_mangle]
pub extern "C" fn r_jobs_builtin(mut list:*mut WordList)->i32 {
    let mut form:i32;
    let mut execute:i32=0;
    let mut state:i32;
    let mut opt:i32;
    let mut any_failed:i32=0;
    let mut job:i32;
    
    let mut set:nix::sys::signal::SigSet=nix::sys::signal::SigSet::empty();
    let mut oset:nix::sys::signal::SigSet =nix::sys::signal::SigSet::empty();

    form = JLIST_STANDARD!();
    state = JSTATE_ANY!();

    unsafe {
    reset_internal_getopt();
    
    let mut c_str_lpnxrs = CString::new("lpnxrs").unwrap(); // from a &str, creates a new allocation

    opt = internal_getopt (list, c_str_lpnxrs.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_har:char= opt as u8 as char;
        match opt_har{
            'l'=>{form = JLIST_LONG!();}
            'p'=>{form = JLIST_PID_ONLY!();}
            'n'=>{form = JLIST_CHANGED_ONLY!();}
            'x'=>{
                if form != JLIST_STANDARD!() {
                    let mut c_str_err = CString::new("no other options allowed with `-x'").unwrap(); // from a &str, creates a new allocation
                    builtin_error (c_str_err.as_ptr());
                    return EXECUTION_FAILURE!();
                }
                execute+=1;
            }
            'r'=>{state = JSTATE_RUNNING!();}
            's'=>{state = JSTATE_STOPPED!();}
        _=>{
            if opt == -99 {
                r_builtin_help();
                return EX_USAGE;
            }
            builtin_usage ();
            return EX_USAGE;
        }              
        }
        opt = internal_getopt (list, c_str_lpnxrs.as_ptr() as * mut c_char);
    }

    list = loptend;

    if execute != 0 {  
        return r_execute_list_with_replacements (loptend);
    }

    if loptend  ==std::ptr::null_mut() {
        if state == JSTATE_ANY!() {
            list_all_jobs (form);
        } else if state == JSTATE_RUNNING!() {
            list_running_jobs (form);
        }else if state == JSTATE_STOPPED!() {
            list_stopped_jobs (form);
        }
        return EXECUTION_SUCCESS!();
    }


    while list !=std::ptr::null_mut() {
        BLOCK_CHILD !(Some(&mut set), Some(&mut oset));
        job = get_job_spec (list);

        if (job == NO_JOB!()) || jobs  == std::ptr::null_mut() || get_job_by_jid!(job)  == std::ptr::null_mut() {
            sh_badjob ((*((*list).word)).word);                 
            any_failed+=1;
        } else if job != DUP_JOB!() {
            list_one_job (0 as * mut JOB, form, 0, job);
        }

        UNBLOCK_CHILD !(Some(&oset));

        list = (*list).next;
    }
        if any_failed !=0 {
            return EXECUTION_FAILURE!();
        } else {
            return EXECUTION_SUCCESS!();
        }
    }
}

#[no_mangle]
pub extern "C" fn r_disown_builtin (list:* mut WordList)->libc::c_int {
  let mut opt:i32;
  let mut job:i32=0;
  let mut retval:i32;
  let mut nohup_only:i32=0;
  let mut running_jobs:i32=0;
  let mut all_jobs:i32=0;
  
  let mut set:nix::sys::signal::SigSet=nix::sys::signal::SigSet::empty();
  let mut oset:nix::sys::signal::SigSet =nix::sys::signal::SigSet::empty();
  let mut pid_value:c_long=0;
  unsafe {
  reset_internal_getopt ();
  let mut c_str_ahr = CString::new("ahr").unwrap(); // from a &str, creates a new allocation
  opt = internal_getopt (list, c_str_ahr.as_ptr() as * mut c_char);
  while  opt != -1 {
    let opt_char:char= opt as u8 as char;
    match opt_char{
        'a'=>{all_jobs = 1;}
        'h'=>{nohup_only = 1;}
        'r'=>{running_jobs = 1;}
        _=>{
            if opt == -99 {
                r_builtin_help();
                return EX_USAGE;
            }
            builtin_usage ();
            return EX_USAGE;
        }
	}
  opt = internal_getopt (list, c_str_ahr.as_ptr() as * mut c_char);
  }

  retval = EXECUTION_SUCCESS!();

  /* `disown -a' or `disown -r' */
  if loptend == std::ptr::null_mut() && (all_jobs !=0 || running_jobs != 0) {
      if nohup_only!=0{
        nohup_all_jobs (running_jobs);
      } else {
        delete_all_jobs (running_jobs);
      }
	    return EXECUTION_SUCCESS!();
  }
  BLOCK_CHILD !(Some(&mut set), Some(&mut oset));
  if (loptend !=std::ptr::null_mut() && legal_number ((*(*loptend).word).word, &mut pid_value) !=0 && pid_value ==  pid_value) {
     job=get_job_by_pid ( pid_value as i32, 0, 0 as *mut*mut PROCESS);
  }else {
    job = get_job_spec (loptend);
  }
  if (job == NO_JOB!()) ||( jobs ==std::ptr::null_mut()) || (INVALID_JOB!(job)) {
    if loptend !=std::ptr::null_mut() {
            sh_badjob ((*(*loptend).word).word);
    } else {
            sh_badjob (CString::new("current").unwrap().as_ptr() as * mut c_char);
    }          
    retval = EXECUTION_FAILURE!();
    } else if nohup_only !=0{
        nohup_job (job);
    } else {
        delete_job (job, 1);
    }

    UNBLOCK_CHILD !(Some(&oset));   

    if loptend  != std::ptr::null_mut() {
        let mut loptendt=*loptend;
        while  loptendt.next !=std::ptr::null_mut() {
            loptendt = *loptendt.next;
            BLOCK_CHILD !(Some(&mut set), Some(&mut oset));
            if legal_number ((*loptendt.word).word, &mut pid_value) !=0 && pid_value ==  pid_value {
            job=get_job_by_pid ( pid_value as i32, 0, 0 as *mut*mut PROCESS);
            } else {
            get_job_spec (&mut loptendt);
            }
        if job == NO_JOB!() || jobs !=std::ptr::null_mut() || INVALID_JOB!(job) {                     
            sh_badjob ((*loptendt.word).word);                
            retval = EXECUTION_FAILURE!();
        }  else if nohup_only !=0{
            nohup_job (job);
        } else {
            delete_job (job, 1);
        }
        UNBLOCK_CHILD !(Some(&oset));
        }
    }
    return retval;
    }
}

fn cmd_name() ->*const u8 {
   return b"jobs" as *const u8;
}
