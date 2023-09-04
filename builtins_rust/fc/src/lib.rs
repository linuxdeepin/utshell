extern crate  libc;
extern crate nix;

use libc::{c_char, c_long, c_void, c_int};
use nix::sys::termios::SpecialCharacterIndices;
use std::{ffi::{CString,CStr}, i32, io::{Write, stdout}, ops::Add, string, u32};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE,r_builtin_usage,r_savestring};
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
    dest:libc::c_int,
    filename:* mut WordDesc
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,
  redirector:REDIRECTEE,
  rflags:libc::c_int,
  flags:libc::c_int,
  instruction:r_instruction,
  redirectee:REDIRECTEE,
  here_doc_eof:*mut c_char
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
pub struct SHELL_VAR {
  name:*mut c_char,
  value:*mut c_char,
  exportstr:*mut c_char,
  dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR,
  assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut c_char,t:c_long,str2:* mut c_char)->*mut SHELL_VAR,
  attributes:i32,
  context:i32
}

#[repr(C)]
pub struct GENERIC_LIST {
  next: * mut GENERIC_LIST
}

#[repr(C)]
pub struct REPL {
  next: *mut REPL,
  pat:*mut c_char,
  rep:*mut c_char
}

#[repr(C)]
pub struct HIST_ENTRY {
    line:*mut c_char,
    timestamp:*mut c_char,
    data:*mut fn()
}


#[macro_export]
macro_rules! ISHELP {
   ($s:expr) => {
    libc::strcmp($s as *const c_char,CString::new("--help").unwrap().as_ptr())
    }
}

#[macro_export]
macro_rules! errno {
  () => {
    *libc::__errno_location()
  }
}

#[macro_export]
macro_rules! HN_LISTING {
  () => {
    0x01
  }
}

#[macro_export]
macro_rules! SUBSHELL_COMSUB {
  () => {
    0x04
  }
}

#[macro_export]
macro_rules! HN_FIRST {
  () => {
    0x02
  }
}

#[macro_export]
macro_rules! HIST_INVALID {
  () => {
    std::i32::MIN
  }
}

#[macro_export]
macro_rules! HIST_ERANGE {
  () => {
    std::i32::MIN +1
  }
}

#[macro_export]
macro_rules! HIST_NOTFOUND {
  () => {
    std::i32::MIN +2
  }
}

#[macro_export]
macro_rules! MT_USETMPDIR {
  () => {
    0x0001
  }
}

#[macro_export]
macro_rules! MT_READWRITE {
  () => {
    0x0002
  }
}

#[macro_export]
macro_rules! MT_USERANDOM {
  () => {
    0x0004
  }
}

#[macro_export]
macro_rules! MT_TEMPLATE {
  () => {
    0x0008
  }
}

#[macro_export]
macro_rules! SEVAL_NOHIST {
  () => {
    0x004
  }
}

pub union Functions {
    f_unlink:unsafe extern "C" fn(t: * const c_char)->i32,
    f_xfree:unsafe extern "C" fn(str1:* mut c_void),
    f_set_verbose: unsafe extern "C" fn(),
}

extern "C" {
    fn reset_internal_getopt();
    static mut loptend:*mut WordList;
    static mut lcurrent:*mut WordList;
    fn legal_number (str1:*const c_char,result:* mut c_long)->i32;
    fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
    static list_optarg:*mut c_char;
    fn builtin_usage();
    fn history_list ()->*mut * mut HIST_ENTRY;
    static subshell_environment:i32;
    fn sh_erange (s:* mut c_char, desc:* mut c_char);
    fn builtin_error(err:*const c_char,...);
    fn bash_delete_last_history ()->i32;
    fn list_reverse (list:* mut GENERIC_LIST)->* mut GENERIC_LIST;
    fn strsub (str1:*mut c_char, pat:*mut c_char, rep:*mut c_char, global:i32)->* mut c_char;
    fn maybe_add_history (line:* mut c_char);
    static mut hist_last_line_added:i32;
    static mut remember_on_history:i32;
    static mut enable_history_list:i32;
    fn sh_mktmpfp (nameroot:* mut c_char, flags:i32, namep:&mut * mut c_char)->* mut libc::FILE;
    fn strerror(e:i32)->* mut c_char;
    static mut interrupt_state:i32;
    static mut terminating_signal:i32;
    fn termsig_handler (sig:i32);
    fn throw_to_top_level();
    static mut history_base:i32;
    static mut posixly_correct:i32;
    fn sh_chkwrite (s:i32)->i32;
    fn sh_wrerror ();
    fn parse_and_execute (str1:* mut c_char, from_file:* const c_char, flags:i32)->i32;
    fn unlink (path:* const c_char)->i32;
    fn begin_unwind_frame (be:* mut c_char);
    fn xfree (str1:* mut c_void);
    fn add_unwind_protect(f:Functions,args:* mut c_char);
    fn unwind_protect_mem (x:* mut c_char, s:i32);
    static mut suppress_debug_trap_verbose:i32;
    fn fc_execute_file (filename:*const c_char)->i32;
    fn run_unwind_frame (filename:* mut c_char);
    static mut echo_input_at_read:i32;
    static mut verbose_flag:i32;
}

#[no_mangle]
pub extern "C" fn r_set_verbose_flag (){
  unsafe {
    echo_input_at_read = verbose_flag;
  }
}

#[no_mangle]
pub extern "C" fn r_fc_number(list:*mut WordList)->i32
{
  let mut s:*mut c_char;
  unsafe{
    if list.is_null(){
      return 0;
    }
    s = (*(*list).word).word;
    if *s as c_int == '-' as i32{
      s = s.offset(1);
    }
    let res = legal_number(s,std::ptr::null_mut());
    return res;
  }
}

unsafe fn REVERSE_LIST(list:* mut GENERIC_LIST)->* mut REPL
{
  if list != std::ptr::null_mut() && (*list).next != std::ptr::null_mut(){
    list_reverse(list) as * mut REPL
  } else {
    list as * mut REPL
  }
}

unsafe fn printToStderr(str:* mut c_char) -> std::io::Result<()> {
  let stderr = std::io::stderr();
  let mut handle = stderr.lock();
  handle.write_all(std::ffi::CStr::from_ptr(str).to_bytes())?;
  Ok(())
}

unsafe fn printToStdout(str:* mut c_char) -> std::io::Result<()> {
  let stdout = std::io::stdout();
  let mut handle = stdout.lock();
  handle.write_all(std::ffi::CStr::from_ptr(str).to_bytes())?;
  Ok(())
}

unsafe fn printToStdoutflush() -> std::io::Result<()> {
  let stdout = std::io::stdout();
  let mut handle = stdout.lock();
  handle.flush()?;
  Ok(())
}

unsafe fn QUIT ()
{
  if terminating_signal !=0 {
    termsig_handler (terminating_signal);
  }

  if interrupt_state !=0{
    throw_to_top_level ();
  }
}

unsafe fn DIGIT ( c: c_char)->bool {
  char::from(c as u8) >= '0' && char::from(c as u8) <= '9'
}

unsafe fn STREQN ( a:* const c_char, b:* const c_char, n:i32)->bool {
  if n==0 {
    return true;
  } else {
    return *a == *b   && libc::strncmp(a, b, n as libc::size_t) == 0
  }
}

#[no_mangle]
pub extern "C" fn r_fc_builtin (mut list:* mut WordList)->i32
{
  let mut i:i32;
  let mut sep:*mut c_char;
  let mut numbering:i32;
  let mut reverse:i32;
  let mut listing:i32;
  let mut execute:i32;
  let mut histbeg:i32;
  let mut histend:i32;
  let mut last_hist:i32;
  let mut retval:i32;
  let mut opt:i32;
  let rh:i32;
  let mut real_last:i32;
  let stream:* mut libc::FILE;
  let mut rlist:*mut REPL;
  let mut rl:*mut REPL;
  let mut ename:*mut c_char;
  let mut command:*mut c_char;
  let newcom:*mut c_char;
  let fcedit: std::ffi::CString;
  let hlist:*mut*mut HIST_ENTRY;
  let mut fnc:*mut c_char=std::ptr::null_mut();

  numbering = 1;
  reverse = 0;
  listing = 0;
  execute = 0;
  ename = std::ptr::null_mut();
  unsafe {  
  reset_internal_getopt ();
  lcurrent = list;

  loptend = lcurrent;
  while  r_fc_number (loptend) ==0{
    opt = internal_getopt (list, CString::new(":e:lnrs").unwrap().as_ptr() as * mut c_char);
    if opt != -1 {
      let optu8:u8= opt as u8;
      let optChar:char=char::from(optu8);
      match optChar{
        'n' => numbering=0,
        'l' => listing = HN_LISTING!(),
        'r' => reverse = 1,
        's' => execute = 1,
        'e' => ename = list_optarg,
        _ => {
          if opt == -99 {
            r_builtin_help();
            return EX_USAGE;
          }
          r_builtin_usage();
          return EX_USAGE;
        }
      }
    }
    else{
      break;
    }   
    loptend = lcurrent;
  }

  list = loptend;

  if ename != std::ptr::null_mut() && char::from(*ename as u8 ) == '-' && char::from(*((ename as usize +4) as * mut c_char) as u8 )== '\0'{
    execute = 1;
  }

  if execute != 0 {
      rlist = std::ptr::null_mut();

      let mut ret:bool=loptend !=std::ptr::null_mut();
      sep= libc::strchr((*(*list).word).word, char::from('=') as libc::c_int);
      ret=ret && sep != std::ptr::null_mut();
      while ret	{
        sep= (sep as usize + 4) as * mut c_char;
	      *sep = char::from('\0') as c_char ;
	      rl = libc::malloc (  std::mem::size_of::<& REPL>() ) as * mut REPL;
	      (*rl).next = std::ptr::null_mut();
	      (*rl).pat = r_savestring ((*(*list).word).word);
        (*rl).rep = r_savestring (sep);

        if rlist == std::ptr::null_mut(){
          rlist = rl;
        } else {
	        (*rl).next = rlist;
	        rlist = rl;
	      }
	      list = (*list).next;
	    }

      rlist = REVERSE_LIST (rlist as * mut GENERIC_LIST);
      hlist = history_list ();      
      if list != std::ptr::null_mut() {
        command=r_fc_gethist((*(*list).word).word, hlist, 0);
      } else {
        command=r_fc_gethist(std::ptr::null_mut(), hlist, 0);
      }
      
      if command == std::ptr::null_mut() {
	      builtin_error (CString::new("no command found").unwrap().as_ptr());
        if rlist !=std::ptr::null_mut()  {
          rl = rlist;
          while rl != std::ptr::null_mut() {
            let r:*mut REPL;
            r = (*rl).next;
            if (*rl).pat !=std::ptr::null_mut() {
              libc::free((*rl).pat as * mut c_void);
            }
            
            if (*rl).rep !=std::ptr::null_mut() {
              libc::free((*rl).rep as * mut c_void);
            }

            libc::free(rl as * mut c_void);
            rl = r;
          }
        }
	      return EXECUTION_FAILURE!();
	    }

  if rlist !=std::ptr::null_mut()	{
	  newcom = r_fc_dosubs (command, rlist);
	  libc::free (command as * mut c_void);
	  rl = rlist;
    while rl != std::ptr::null_mut() {
      let r:* mut REPL;
      r = (*rl).next;
      if (*rl).pat !=std::ptr::null_mut() {
        libc::free((*rl).pat as * mut c_void);
      }
      
      if (*rl).rep !=std::ptr::null_mut() {
        libc::free((*rl).rep as * mut c_void);
      }

      libc::free(rl as * mut c_void);
      rl = r;
    }
	  command = newcom;
	}
    printToStderr(command);
    r_fc_replhist (command);
    return parse_and_execute (command, CString::new("fc").unwrap().as_ptr(), SEVAL_NOHIST!());
  }
  
  hlist = history_list ();
  
  if hlist == std::ptr::null_mut(){
    return EXECUTION_SUCCESS!();
  }
  i=0;
  
  while !(*hlist.offset(i as isize)).is_null(){  
    i+=1;
  }

  rh = (remember_on_history !=0 || ((subshell_environment  & SUBSHELL_COMSUB!()) !=0 && enable_history_list !=0)) as i32;
  last_hist = i - rh - hist_last_line_added;

  real_last = i;
  while (*hlist.offset(real_last as isize)).is_null() && real_last > 0
  {
    real_last-=1;
  }
  
  if i == last_hist && (*hlist.offset(last_hist as isize)).is_null() {
    while last_hist >= 0 && (*hlist.offset(last_hist as isize)).is_null() 
    {
      last_hist-=1;
    }
  }

  if last_hist < 0{
    last_hist = 0;
  }

  if !(list.is_null()) {
    histbeg = r_fc_gethnum ((*(*list).word).word, hlist, listing| HN_FIRST!());
    list = (*list).next;

    if list != std::ptr::null_mut(){
      histend = r_fc_gethnum ((*(*list).word).word, hlist, listing);
    } else if histbeg == real_last {
      histend = if listing != 0 { real_last } else { histbeg };      
    } else {
      histend = if listing != 0 { last_hist } else { histbeg }
    }
  } else {
      if listing != 0	{
	      histend = last_hist;
	      histbeg = histend - 16 + 1;
	      if histbeg < 0{
          histbeg = 0;
        }
      } else{
        histbeg =last_hist;
        histend =last_hist;
      }
  }

  if histbeg == HIST_INVALID!() || histend == HIST_INVALID!() {
      sh_erange (std::ptr::null_mut(), CString::new("history specification").unwrap().as_ptr() as * mut c_char); 
      return EXECUTION_FAILURE!();
  } else if histbeg == HIST_ERANGE!() || histend == HIST_ERANGE!() {
      sh_erange (std::ptr::null_mut(), CString::new("history specification").unwrap().as_ptr() as * mut c_char);
      return EXECUTION_FAILURE!();
  } else if histbeg == HIST_NOTFOUND!() || histend == HIST_NOTFOUND!() {
      builtin_error (CString::new("no command found").unwrap().as_ptr() as * mut c_char);
      return EXECUTION_FAILURE!();
  }

  if histbeg < 0 {
    histbeg = 0;
  }
    
  if histend < 0 {
    histend = 0;
  }

  if listing == 0 && hist_last_line_added !=0 {
      bash_delete_last_history ();

      if histbeg == histend && histend == last_hist && *((hlist as usize + (8*last_hist) as usize) as  * mut * mut HIST_ENTRY)  == std::ptr::null_mut() {
        histend-=1;
        last_hist = histend;
        histbeg = histend;
      }

      if *((hlist as usize + (8*last_hist) as usize) as  * mut * mut HIST_ENTRY)  == std::ptr::null_mut() {
        last_hist-=1;
      }

      if histend >= last_hist {
        histend = last_hist;
      } else if histbeg >= last_hist {
        histbeg = last_hist;
      }
  }

  if histbeg == HIST_INVALID!() || histend == HIST_INVALID!() {
      sh_erange (std::ptr::null_mut(), CString::new ("history specification").unwrap().as_ptr() as * mut c_char);
      return EXECUTION_FAILURE!();
  } else if histbeg == HIST_ERANGE!() || histend == HIST_ERANGE!() {
      sh_erange (std::ptr::null_mut(), CString::new ("history specification").unwrap().as_ptr() as * mut c_char);
      return EXECUTION_FAILURE!();
  } else if histbeg == HIST_NOTFOUND!() || histend == HIST_NOTFOUND!() {
      builtin_error (CString::new ("no command found").unwrap().as_ptr());
      return EXECUTION_FAILURE!();
  }

  if histbeg < 0 {
    histbeg = 0;
  }

  if histend < 0{
    histend = 0;
  }

  if histend < histbeg {
    i = histend;
    histend = histbeg;
    histbeg = i;
    reverse = 1;
  }

  if listing !=0{
    stream = std::ptr::null_mut();
  } else  {
      numbering = 0;
      stream = sh_mktmpfp (CString::new ("bash-fc").unwrap().as_ptr() as * mut c_char, MT_USERANDOM!()|MT_USETMPDIR!(),  &mut fnc);

      if stream == std::ptr::null_mut()	{
        if fnc != std::ptr::null_mut() {
          builtin_error (CString::new ("%s: cannot open temp file: %s").unwrap().as_ptr(),  fnc , strerror (errno!()));
        } else {
          builtin_error (CString::new ("%s: cannot open temp file: %s").unwrap().as_ptr(), CString::new ("").unwrap().as_ptr(), strerror (errno!()));
        }

        libc::free(fnc as * mut c_void);
        return EXECUTION_FAILURE!();
	    }
  }

  if reverse !=0{
    i=histend;
  }else {
    i=histbeg;
  }
  
  let mut ret:bool=reverse !=0;
  
  if ret {
    ret =i >= histbeg ;
  } else {
    ret = i <= histend;
  }

  while ret {
      QUIT();
      if numbering != 0 {
        if stream != std::ptr::null_mut(){
          libc::fprintf (stream, CString::new ("%d").unwrap().as_ptr(), i + history_base);
        }else {
          let diff =i + history_base;   
          printToStdout(CString::new (diff.to_string()).unwrap().as_ptr() as * mut c_char );
        }
      }

      if listing !=0 {  
        if posixly_correct !=0 {
          if stream != std::ptr::null_mut(){
            libc::fputs (CString::new ("\t").unwrap().as_ptr(), stream);
          } else {
            printToStdout(CString::new ("\t").unwrap().as_ptr() as * mut c_char );
          }
        } else {
          let mut ch:char;
          if (**((hlist as usize + (8*i) as usize) as *mut*mut HIST_ENTRY)).data != std::ptr::null_mut() {
            ch='*';
          } else {
            ch=' ';
          }

          if stream != std::ptr::null_mut() {
            libc::fprintf (stream, CString::new ("\t%c").unwrap().as_ptr(), & mut ch);
          }else {           
            let mut th=vec!['\t' as c_char];
            th.push(ch as c_char);
            th.push(0);
            printToStdout(  th.as_ptr() as * mut c_char);
          }
        }
      }
            
      if stream != std::ptr::null_mut() {
        libc::fprintf (stream, CString::new ("%s\n").unwrap().as_ptr(), (**((hlist as usize + (i*8) as usize) as *mut*mut HIST_ENTRY)).line);
      }else {   
        printToStdout((**((hlist as usize + (i*8) as usize) as *mut*mut HIST_ENTRY)).line);
        printToStdout(CString::new ("\n").unwrap().as_ptr() as * mut c_char );
      }
      //printToStdoutflush();

      ret=reverse !=0;
      if ret {
        i-=1;
      } else {
        i+=1;
      }

      if ret {
        ret =i >= histbeg;
      } else {
        ret =i <= histend;
      }
  }

  if listing !=0 {
    return sh_chkwrite (EXECUTION_SUCCESS!());
  }
    
  if stream != std::ptr::null_mut() {
    libc::fflush (stream);
    if libc::ferror (stream) !=0 {
      sh_wrerror ();
      libc::fclose (stream);
      libc::free(fnc as * mut c_void);
      return EXECUTION_FAILURE!();
    }
    libc::fclose (stream);
  } else {
    if printToStdoutflush().is_err() {
      sh_wrerror ();
      libc::free(fnc as * mut c_void);
      return EXECUTION_FAILURE!();
    }
  }
  
  /* Now edit the file of commands. */
  if ename != std::ptr::null_mut() {
     command=libc::malloc (  libc::strlen (ename) + libc::strlen (fnc) + 2 ) as * mut c_char;
     libc::sprintf(command,CString::new("").unwrap().as_ptr());
     libc::strcpy(command,ename);
     libc::strcat(command,CString::new(" ").unwrap().as_ptr());
     libc::strcat(command,fnc);
  } else {
     if posixly_correct !=0 {
      fcedit=CString::new("${FCEDIT:-${EDITOR:-ed}}").unwrap();
     } else {
      fcedit=CString::new("${FCEDIT:-${EDITOR:-vi}}").unwrap();
     }
     
     command=libc::malloc (  3 + libc::strlen(fcedit.as_ptr())  as libc::size_t + libc::strlen (fnc) ) as * mut c_char;
     libc::sprintf(command,CString::new("").unwrap().as_ptr());
     libc::strcpy(command,fcedit.as_ptr());
     libc::strcat(command,CString::new(" ").unwrap().as_ptr());
     libc::strcat(command,fnc);
  }
 
  retval = parse_and_execute (command, CString::new("fc").unwrap().as_ptr(), SEVAL_NOHIST!());
  if retval != EXECUTION_SUCCESS!() {
      unlink (fnc);
      libc::free (fnc as * mut c_void);
      return EXECUTION_FAILURE!();
  }

  /* Make sure parse_and_execute doesn't turn this off, even though a
     call to parse_and_execute farther up the function call stack (e.g.,
     if this is called by vi_edit_and_execute_command) may have already
     called bash_history_disable. */
  remember_on_history = 1;

  /* Turn on the `v' flag while fc_execute_file runs so the commands
     will be echoed as they are read by the parser. */
  //begin_unwind_frame (CString::new ("fc builtin").unwrap().as_ptr() as * mut c_char);
  let xf:Functions=Functions{f_xfree :xfree};
  let uk:Functions=Functions{f_unlink :unlink};
  let r_flag:Functions=Functions{f_set_verbose :r_set_verbose_flag};
  add_unwind_protect (xf, fnc);
  add_unwind_protect (uk, fnc);
  add_unwind_protect (r_flag, std::ptr::null_mut());
  unwind_protect_mem ((& mut (suppress_debug_trap_verbose as c_char) ) as * mut c_char,4);
  echo_input_at_read = 1;
  suppress_debug_trap_verbose = 1;

  retval = fc_execute_file (fnc);
  //run_unwind_frame (CString::new ("fc builtin").unwrap().as_ptr() as * mut c_char);

  return retval;
}
}

#[no_mangle]
pub extern "C" fn r_fc_gethist (command:* mut c_char, hlist:* mut * mut HIST_ENTRY, mode:i32)->* mut c_char
{
  let mut i:i32;

  if hlist == std::ptr::null_mut() {
    return std::ptr::null_mut();
  }

  i = r_fc_gethnum (command, hlist, mode);
  unsafe {
    if i >= 0 {
      return r_savestring ((*(*((hlist as usize + (8*i) as usize) as * mut * mut HIST_ENTRY))).line );
    }  else {
      return std::ptr::null_mut();
    }
  }
}

#[no_mangle]
pub extern "C" fn r_fc_gethnum (command:* mut c_char, hlist:* mut * mut HIST_ENTRY, mode:i32)->i32
{
  let mut sign:i32;
  let mut n:i32;
  let mut clen:i32;
  let mut rh:i32;
  let mut i:i32=0;
  let mut j:i32;
  let mut last_hist:i32;
  let mut real_last:i32;
  let mut listing:i32;

  let mut s:* mut c_char;
  
  unsafe {
  listing = mode & HN_LISTING!();
  sign = 1;
  /* Count history elements. */
  while !(*hlist.offset(i as isize)).is_null() {    
    i+=1;
  }
  /* With the Bash implementation of history, the current command line
     ("fc blah..." and so on) is already part of the history list by
     the time we get to this point.  This just skips over that command
     and makes the last command that this deals with be the last command
     the user entered before the fc.  We need to check whether the
     line was actually added (HISTIGNORE may have caused it to not be),
     so we check hist_last_line_added.  This needs to agree with the
     calculation of last_hist in fc_builtin above. */
  /* Even though command substitution through parse_and_execute turns off
     remember_on_history, command substitution in a shell when set -o history
     has been enabled (interactive or not) should use it in the last_hist
     calculation as if it were on. */
  rh = (remember_on_history !=0 || ((subshell_environment  & SUBSHELL_COMSUB!()) !=0 && enable_history_list !=0)) as i32;
  last_hist = i - rh - hist_last_line_added;

  if i == last_hist && (*hlist.offset(last_hist as isize)).is_null() {
    while last_hist >= 0 && (*hlist.offset(last_hist as isize)).is_null(){
      last_hist-=1;
    }    
  }
    
  if last_hist < 0 {
    return -1;
  }

  real_last = i;
  i = last_hist;

  /* No specification defaults to most recent command. */
  if command == std::ptr::null_mut(){
    return i;
  }

  /* back up from the end to the last non-null history entry */
  while (*hlist.offset(real_last as isize)).is_null() && real_last > 0 {
    real_last-=1;
  }

  /* Otherwise, there is a specification.  It can be a number relative to
     the current position, or an absolute history number. */
  s = command;

  /* Handle possible leading minus sign. */
  if s != std::ptr::null_mut() && ( char::from(*s as u8) == '-') {
      sign = -1;
      s = s.offset(1)
  }

  if s != std::ptr::null_mut() && DIGIT( *s ) {
    n = libc::atoi(s);
    n *= sign;

      /* We want to return something that is an offset to HISTORY_BASE. */
      /* If the value is negative or zero, then it is an offset from
	 the current history item. */
      /* We don't use HN_FIRST here, so we don't return different values
	 depending on whether we're looking for the first or last in a
	 pair of range arguments, but nobody else does, either. */
    if n < 0 {
	    n += i + 1;
      if n < 0 {
        return 0;
      } else {
        return n;
      }
	  } else if n == 0 {
      if sign == -1  {
        if listing != 0 {
          return real_last;
        } else {
          return HIST_INVALID!();
        }
      } else {
        return i;
      }
    }  else	{
	  /* If we're out of range (greater than I (last history entry) or
	     less than HISTORY_BASE, we want to return different values
	     based on whether or not we are looking for the first or last
	     value in a desired range of history entries. */
      n -= history_base;
      if n < 0 {
        if mode & HN_FIRST!() !=0 {
          return 0;
        } else{
          return i;
        }
      } else if n >= i {
        if mode & HN_FIRST!() !=0 {
          return 0;
        } else{
          return i;
        }
      } else {
        return n;
      }
	  }
  }

  clen = libc::strlen (command as * const c_char) as i32;
  j = i;
  while j >= 0 {
    if STREQN (command, (*(*hlist.offset(j as isize))).line, clen) {
    //if STREQN (command, (*(*((hlist as usize + (8*j) as usize ) as  * mut * mut HIST_ENTRY))).line, clen) {
      return j;
    }
    j-=1;
  }
  return HIST_NOTFOUND!();
  }
}

#[no_mangle]
pub extern "C" fn r_fc_dosubs (command:* mut c_char, subs:* mut REPL)->* mut c_char
{
  let mut new:* mut c_char;
  let mut t:* mut c_char;
  let mut r:* mut REPL;
  unsafe {
    new = r_savestring (command);
    while subs !=std::ptr::null_mut() {
      r = subs;
      t = strsub (new, (*r).pat, (*r).rep, 1);
      r = (*r).next;
      libc::free(new as * mut c_void);
      new = t;
    }
    return new;
  }
}

#[no_mangle]
pub extern "C" fn r_fc_replhist (command:* mut c_char)
{
  let  n:i32;
  unsafe {
    if command == std::ptr::null_mut() || char::from(*command as u8)== '\0' {
      return;
    }

    n = libc::strlen (command as * const c_char) as i32;
    if char::from(*((command as usize + 4*(n-1) as usize) as *mut c_char) as u8)   == '\n' {
      *((command as usize + 4*(n-1) as usize ) as *mut c_char)= 0 as c_char;
    }

    if command != std::ptr::null_mut() && (*command) != 0 {
        bash_delete_last_history ();
        maybe_add_history (command);
    }
  }
}

#[no_mangle]
pub extern "C" fn fc_addhist (line: * mut c_char)
{
  let n:i32;
  unsafe {
    if line == std::ptr::null_mut() || *line == 0 {
      return;
    }

    n = libc::strlen (line) as i32;

    if *((line as usize + (n-1) as usize) as * mut c_char) == '\n' as c_char {
      *((line as usize + (n-1) as usize) as * mut c_char) = '\0' as c_char;
    }

    if line != std::ptr::null_mut() && *line != 0 {
      maybe_add_history (line);
    }
  }
}

#[no_mangle]
pub extern "C" fn fc_readline (stream:* mut libc::FILE)->* mut c_char
{
  let mut c:i32;
  let mut line_len:i32 = 0;
  let mut lindex:i32 = 0;
  let mut line:* mut c_char = std::ptr::null_mut();
  unsafe {
    c = libc::fgetc (stream);
    while c != libc::EOF {
      if (lindex + 2) >= line_len {
        line_len += 128;
        line = libc::malloc ( line_len as libc::size_t) as  * mut c_char;
      }

      if c == '\n' as i32  {
        *((line as usize + (4*lindex) as usize) as * mut c_char)='\n' as c_char;
        lindex+=1;
        *((line as usize + (4*lindex) as usize) as * mut c_char)='\0' as c_char;
        lindex+=1;
        return line;
      } else {
        *((line as usize + (4*lindex) as usize) as * mut c_char)=c as c_char;
        lindex+=1;
      }

      c = libc::fgetc (stream);
    }

  if lindex ==0 {
      if line != std::ptr::null_mut() {
        libc::free(line as * mut c_void);
      }
      return std::ptr::null_mut();
  }

  if lindex + 2 >= line_len {
    line = libc::malloc((lindex + 3) as libc::size_t) as * mut c_char;
  }

  *((line as usize + (4*lindex) as usize) as * mut c_char)='\n' as c_char;
  lindex+=1;
  *((line as usize + (4*lindex) as usize) as * mut c_char)='\0' as c_char;
  lindex+=1;

  return line;
  }
}

/*
#[no_mangle]
pub extern "C" fn cmd_name() ->*const u8 {
   return b"fc" as *const u8;
}
#[no_mangle]
pub extern "C" fn run(list : *mut WordList)->i32 {
  return r_fc_builtin(list);
}
*/
