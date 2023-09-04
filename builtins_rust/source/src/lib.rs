extern crate  libc;
extern crate nix;

use libc::{c_char, c_long, c_void};
use std::{ffi::CString};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage, r_savestring};
use rhelp::r_builtin_help;

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
pub union REDIRECT {
  next:*mut REDIRECT,
  redirector:libc::c_int,
  rflags:libc::c_int,
  flags:libc::c_int,
  instruction:r_instruction,
  redirectee:libc::c_int,
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


#[macro_export]
macro_rules! ARGS_SETBLTIN {
  () => {
    0x04
  }
}

#[macro_export]
macro_rules! EXITPROG {
  () => {
    3
  }
}
pub union Functions {
  f_xfree:unsafe extern "C" fn(str1:* mut c_void),
  f_maybe_pop_dollar_vars: unsafe extern "C" fn(),
  f_maybe_set_debug_trap:unsafe extern "C" fn(* mut c_char)
}

extern "C" {
    static variable_context:i32;
    fn dollar_vars_changed ()->i32;
    fn dispose_saved_dollar_vars ();
    fn pop_dollar_vars ();
    static mut debugging_mode:i32;
    fn pop_args ();
    fn set_dollar_vars_unchanged ();
    fn invalidate_cached_quoted_dollar_at ();
    fn no_options (list:* mut WordList)->i32;
    static mut loptend:*mut WordList;
    fn builtin_usage();
    fn builtin_error(err:*const c_char,...);
    static mut restricted:i32;
    fn sh_restricted (word:* mut c_char);
    static mut posixly_correct:i32;
    static mut source_searches_cwd:i32;
    fn printable_filename (path:* mut c_char, tab:i32)->* mut c_char;
    fn absolute_pathname (path:* const c_char)->i32;
    static source_uses_path:i32;
    fn find_path_file (path:* const c_char)->* mut c_char;
    static mut interactive_shell:i32;
    static mut executing_command_builtin:i32;
    static mut last_command_exit_value:i32;
    fn jump_to_top_level(level:i32);
    fn begin_unwind_frame (str: * mut c_char);
    fn add_unwind_protect(f:Functions,args:* mut c_char);
    fn maybe_set_debug_trap (str: * mut c_char);
    fn xfree(str1:* mut c_void);
    fn push_dollar_vars ();
    static shell_compatibility_level:i32;
    fn init_bash_argv();
    fn remember_args (list:* mut WordList, argc:i32);
    fn push_args (list:* mut WordList);
    static mut function_trace_mode:i32; 
    fn signal_is_trapped (sig:i32)->i32;
    fn signal_is_ignored (sig:i32)->i32;
    static trap_list:[* mut c_char;68];
    fn restore_default_signal (sig:i32);
    fn source_file (name:* const c_char, sflags:i32)->i32;
    fn run_unwind_frame (filename:* mut c_char);
}

#[no_mangle]
pub extern "C" fn r_maybe_pop_dollar_vars ()
{
  unsafe {
    if variable_context == 0 && (dollar_vars_changed () & ARGS_SETBLTIN!()) !=0 {
      dispose_saved_dollar_vars ();
    } else {
      pop_dollar_vars ();
    }
    if debugging_mode !=0 {
      pop_args ();	/* restore BASH_ARGC and BASH_ARGV */
    }

    set_dollar_vars_unchanged ();
    invalidate_cached_quoted_dollar_at ();	/* just invalidate to be safe */
  }
}


unsafe fn TRAP_STRING(s:i32)->* mut c_char {
  if signal_is_trapped (s) !=0 && signal_is_ignored (s) == 0 {
    return trap_list[s as usize];
  } else {
      return std::ptr::null_mut();
  }
}

unsafe fn DEBUG_TRAP()->i32
{
  return libc::SIGRTMAX() +1;
}

#[no_mangle]
pub extern "C" fn r_source_builtin (list:* mut WordList)->i32
{
  
  let mut result:i32;
  let mut filename:*mut c_char;
  let mut debug_trap:* mut c_char;
  let x:* mut c_char;
  unsafe {
  if no_options (list) !=0{
    return EX_USAGE;
  }

  let mut  llist:* mut WordList = loptend.clone();

  if list == std::ptr::null_mut() {
    builtin_error (b"filename argument required\0" as *const u8 as *const libc::c_char as *mut libc::c_char );
    builtin_usage ();
    return EX_USAGE;
  }

  if restricted !=0 && libc::strchr ((*(*llist).word).word, '/' as libc::c_int) != std::ptr::null_mut() {
      sh_restricted ((*(*llist).word).word);
      return EXECUTION_FAILURE!();
  }

  filename = std::ptr::null_mut();
  /* XXX -- should this be absolute_pathname? */
  if posixly_correct !=0 && libc::strchr ((*(*llist).word).word, '/' as libc::c_int) != std::ptr::null_mut() {
    filename = r_savestring ((*(*llist).word).word);
  } else if absolute_pathname ((*(*llist).word).word) !=0 {
    filename = r_savestring ((*(*llist).word).word);
  } else if source_uses_path !=0 {
    filename = find_path_file ((*(*llist).word).word);
  }

  if filename == std::ptr::null_mut() {
    if source_searches_cwd == 0 {
	    x = printable_filename ((*(*llist).word).word, 0);
      builtin_error (CString::new("%s: file not found").unwrap().as_ptr(), x);
      if x != (*(*llist).word).word {
        libc::free (x as * mut c_void);
      }

      if posixly_correct !=0 && interactive_shell == 0 && executing_command_builtin == 0  {
          last_command_exit_value = EXECUTION_FAILURE!();
          jump_to_top_level (EXITPROG!());
      }
      return EXECUTION_FAILURE!();
	  } else {
      filename = r_savestring ((*(*llist).word).word);
    }
  }

  begin_unwind_frame (b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
  let xf:Functions=Functions{f_xfree :xfree};
  add_unwind_protect (xf, filename);

  if (*list).next != std::ptr::null_mut() {
      push_dollar_vars ();
      let xvars:Functions=Functions{f_maybe_pop_dollar_vars:r_maybe_pop_dollar_vars};
      add_unwind_protect (xvars, std::ptr::null_mut());
      if debugging_mode !=0 || shell_compatibility_level <= 44 {
        init_bash_argv ();	/* Initialize BASH_ARGV and BASH_ARGC */
      }

      remember_args ((*list).next, 1);
      if debugging_mode !=0 {
        push_args ((*list).next);	/* Update BASH_ARGV and BASH_ARGC */
      }

  }
  set_dollar_vars_unchanged ();

  /* Don't inherit the DEBUG trap unless function_trace_mode (overloaded)
     is set.  XXX - should sourced files inherit the RETURN trap?  Functions
     don't. */
  debug_trap = TRAP_STRING (DEBUG_TRAP());
  if debug_trap != std::ptr::null_mut() && function_trace_mode == 0  {
      debug_trap = r_savestring (debug_trap);
      let xf1:Functions=Functions{f_xfree :xfree};
      add_unwind_protect (xf1, debug_trap);

      let xfmaybe_set_debug_trap:Functions=Functions{f_maybe_set_debug_trap :maybe_set_debug_trap};
      add_unwind_protect (xfmaybe_set_debug_trap, debug_trap);
      restore_default_signal (DEBUG_TRAP());
    }

  result = source_file (filename, (list !=std::ptr::null_mut() && (*list).next !=std::ptr::null_mut()) as i32);

  run_unwind_frame (b"source\0" as *const u8 as *const libc::c_char as *mut libc::c_char);

  return result;
  }
}

