extern crate  libc;
extern crate nix;

use libc::{c_char, c_long, c_void};
use std::{ffi::CString};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, r_savestring};
use rhelp::r_builtin_help;
use rsetattr::{show_name_attributes,set_or_show_attributes,show_all_var_attributes};
use std::ffi::CStr;
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
pub struct SHELL_VAR {
  name:*mut c_char,			/* Symbol that the user types. */
  value:*mut c_char,			/* Value that is returned. */
  exportstr:*mut c_char,	/* String for the environment. */
  dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR,	/* Function called to return a `dynamic'
				   value for a variable, like $SECONDS
				   or $RANDOM. */
  assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut c_char,t:c_long,str2:* mut c_char)->*mut SHELL_VAR, /* Function called when this `special
				   variable' is assigned a value in
				   bind_variable. */
  attributes:i32,		/* export, readonly, array, invisible... */
  context:i32			/* Which context this variable belongs to. */
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

#[repr(C)]
pub struct VAR_CONTEXT {
	name:* mut c_char,/* empty or NULL means global context */
	scope:i32,	/* 0 means global context */
	flags:i32,
	up:* mut VAR_CONTEXT,	/* previous function calls */
	down:* mut VAR_CONTEXT,	/* down towards global context */
	table:* mut HASH_TABLE		/* variables at this scope */
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

#[macro_export]
macro_rules! att_local {
  () => {
    0x0000020
  }
}

#[macro_export]
macro_rules! att_array {
  () => {
    0x0000004 /* value is an array */
  }
}

#[macro_export]
macro_rules! att_assoc {
  () => {
    0x0000040	/* variable is an associative array */
  }
}

#[macro_export]
macro_rules! att_function {
  () => {
    0x0000008	/* value is a function */
  }
}

#[macro_export]
macro_rules! att_integer {
  () => {
    0x0000010	/* internal representation is int */
  }
}

#[macro_export]
macro_rules! att_nameref {
  () => {
    0x0000800	/* word is a name reference */
  }
}

#[macro_export]
macro_rules! att_readonly {
  () => {
    0x0000002	/* cannot change */
  }
}

#[macro_export]
macro_rules! att_trace {
  () => {
    0x0000080	/* function is traced with DEBUG trap */
  }
}

#[macro_export]
macro_rules! att_exported {
  () => {
    0x0000001	/* export to environment */
  }
}

#[macro_export]
macro_rules! att_capcase {
  () => {
    0x0000400	/* word capitalized on assignment */
  }
}

#[macro_export]
macro_rules! att_uppercase {
  () => {
    0x0000100	/* word converted to uppercase on assignment */
  }
}

#[macro_export]
macro_rules! att_lowercase {
  () => {
    0x0000200	/* word converted to lowercase on assignment */
  }
}

#[macro_export]
macro_rules! MKLOC_INHERIT {
  () => {
    0x04
  }
}


#[macro_export]
macro_rules! ASS_APPEND {
  () => {
    0x0001
  }
}

#[macro_export]
macro_rules! ASS_MKLOCAL {
  () => {
    0x0002
  }
}

#[macro_export]
macro_rules! MKLOC_ARRAYOK {
  () => {
    0x02
  }
}

#[macro_export]
macro_rules! MKLOC_ASSOCOK {
  () => {
    0x01
  }
}

#[macro_export]
macro_rules! FUNC_MULTILINE {
  () => {
    0x01
  }
}

#[macro_export]
macro_rules! FUNC_EXTERNAL {
  () => {
    0x02
  }
}

#[macro_export]
macro_rules! ASS_FORCE {
  () => {
    0x0020	/* force assignment even to readonly variable */
  }
}

#[macro_export]
macro_rules! W_COMPASSIGN {
  () => {
    1 << 15	/* Compound assignment */
  }
}

#[macro_export]
macro_rules! ASS_NOEXPAND {
  () => {
    0x0080	/* don't expand associative array subscripts */
  }
}

#[macro_export]
macro_rules! EX_BADASSIGN {
  () => {
    260	/* variable assignment error */
  }
}

#[macro_export]
macro_rules! att_tempvar {
  () => {
    0x0100000	/* variable came from the temp environment */
  }
}

#[macro_export]
macro_rules! att_propagate {
  () => {
    0x0200000	/* propagate to previous scope */
  }
}

#[macro_export]
macro_rules! ASS_NAMEREF {
  () => {
    0x0010	/* assigning to nameref variable */
  }
}

#[macro_export]
macro_rules! att_invisible {
  () => {
    0x0001000	/* cannot see */
  }
}

#[macro_export]
macro_rules! att_noassign {
  () => {
    0x0004000	/* assignment not allowed */
  }
}

pub union Functions {
  f_xfree:unsafe extern "C" fn(str1:* mut c_void),
  f_maybe_pop_dollar_vars: unsafe extern "C" fn(),
  f_maybe_set_debug_trap:unsafe extern "C" fn(* mut c_char)
}

extern "C" {
    static variable_context:i32;
    fn builtin_error(err:*const c_char,...);
	fn builtin_warning(err:*const c_char,...);
    fn find_variable (str:*const c_char)->* mut SHELL_VAR;
    fn find_global_variable (str:*const c_char)->* mut SHELL_VAR;
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
    static mut list_opttype:i32;
    static mut array_needs_making:i32;
    fn builtin_usage();
    static mut loptend:*mut WordList;
    fn show_local_var_attributes (v:i32, nodefs:i32)->i32;
    // fn show_all_var_attributes (v:i32, nodefs:i32)->i32;
    fn set_builtin (list:*mut WordList)->i32;
    fn sh_chkwrite (ret:i32)->i32;
    fn show_func_attributes (name:* mut c_char, nodefs:i32)->i32;
    fn show_localname_attributes (name:* mut c_char, nodefs:i32)->i32;
    fn sh_notfound (name:* mut c_char);
	static assoc_expand_once:i32;
	fn assignment (str1:* const c_char, flags:i32)->i32;
	fn make_local_variable (name:* const c_char, flags:i32)->* mut SHELL_VAR;
	fn get_current_options ()->* mut c_char;
	fn valid_array_reference (name:* const c_char, flags:i32)->i32;
	fn check_selfref (name:* const c_char, value:* mut c_char, flags:i32)->i32;
	fn valid_nameref_value (name:* const c_char, flags:i32)->i32;
	fn sh_invalidid (value:* mut c_char);
	static mut posixly_correct:i32;	
	fn nameref_transform_name (name:* mut c_char, flags:i32)->* mut c_char;
	fn find_variable_last_nameref (name:* const c_char, flags:i32)->* mut SHELL_VAR;
	fn make_local_assoc_variable (value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn make_local_array_variable (value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn find_global_variable_last_nameref (value:* const c_char, flags:i32)->* mut SHELL_VAR;
	fn find_global_variable_noref (value:* const c_char)->* mut SHELL_VAR;
	fn find_variable_noref (value:* const c_char)->* mut SHELL_VAR;
	fn sh_readonly (name:* const c_char);
	fn sh_invalidopt (value:* mut c_char);
	static mut debugging_mode:i32;
	fn find_function_def (name:* const c_char)->* mut function_def;
	fn named_function_string (name:* mut c_char, cmd:* mut COMMAND, i:i32)->* mut c_char;
	fn make_new_assoc_variable (name:* mut c_char)->* mut SHELL_VAR;
	fn make_new_array_variable (name:* mut c_char)->* mut SHELL_VAR;
	fn bind_global_variable (name:* const c_char,value:* mut c_char,flags:i32)->* mut SHELL_VAR;
	fn bind_variable (name:* const c_char,value:* mut c_char,flags:i32)->* mut SHELL_VAR;
	static mut shell_compatibility_level:i32;
	fn internal_warning (format:* const c_char, ...);
	fn assign_array_element (name:* mut c_char, value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn bind_assoc_variable (var:* mut SHELL_VAR, name:* mut c_char, key:* mut c_char, value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn bind_array_variable (name:* mut c_char, s:libc::c_long,  value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn bind_variable_value (var:* mut SHELL_VAR, name:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn delete_var (name:* const c_char, varc:* mut VAR_CONTEXT)->i32;
	static global_variables:* mut VAR_CONTEXT;
	static shell_variables:* mut VAR_CONTEXT;
	fn find_tempenv_variable (format:* const c_char)->* mut SHELL_VAR;
	fn stupidly_hack_special_variables (name:* mut c_char);
	fn assign_array_var_from_string (var:* mut SHELL_VAR, value:* mut c_char, flags:i32)->* mut SHELL_VAR;
	fn convert_var_to_array (var:* mut SHELL_VAR)->* mut SHELL_VAR;
	fn convert_var_to_assoc (var:* mut SHELL_VAR)->* mut SHELL_VAR;
	fn find_function (name:* const c_char)->* mut SHELL_VAR;
	fn legal_identifier (name:* const c_char)->i32;
}

#[no_mangle]
pub extern "C" fn r_declare_builtin (list:* mut WordList)->i32
{
  return r_declare_internal (list, 0);
}

unsafe fn STREQ( a:* const c_char, b:* const c_char)->bool {  
  return *a ==*b  && libc::strcmp(a, b) == 0; 
}

#[no_mangle]
pub extern "C" fn r_local_builtin (list:* mut WordList)->i32
{
  unsafe {
      /* Catch a straight `local --help' before checking function context */
      if list !=std::ptr::null_mut() && (*list).word != std::ptr::null_mut() && STREQ ((*(*list).word).word, CString::new("--help").unwrap().as_ptr()) {
        r_builtin_help ();
        return EX_USAGE;
      }

      if variable_context !=0 {
        return r_declare_internal (list, 1);
      } else {
        builtin_error (CString::new("can only be used in a function").unwrap().as_ptr());
        return EXECUTION_FAILURE!();
      }
  }

}

unsafe fn local_p( varr:* mut SHELL_VAR)->i32 {
  return (*varr).attributes & att_local!();
}

#[no_mangle]
pub extern "C" fn r_declare_find_variable (name:* const c_char, mkglobal:i32, chklocal:i32)->* mut SHELL_VAR
{
  let varr: * mut SHELL_VAR;
  unsafe {
    if mkglobal == 0 {
      return find_variable (name);
    } else if chklocal !=0 {
      varr = find_variable (name);
      if varr != std::ptr::null_mut() && local_p (varr) !=0 && (*varr).context == variable_context {
        return varr;
      }

      return find_global_variable (name);
    } else {
      return find_global_variable (name);
    }
  }
}

unsafe fn DECLARE_OPTS()-> CString
{
  return CString::new("+acfgilnprtuxAFGI").unwrap();
}

unsafe fn  value_cell(var:*mut SHELL_VAR)->* mut c_char
{
  return (*var).value;
}

unsafe fn  var_setvalue(var:*mut SHELL_VAR,str1:* mut c_char)
{
  (*var).value=str1;
}

unsafe fn VSETATTR(var:*mut SHELL_VAR, attr:i32) {
	(*var).attributes |= attr;
}

unsafe fn readonly_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_readonly!();
}

unsafe fn nameref_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_nameref!();
}

unsafe fn nameref_cell(var:*mut SHELL_VAR) ->* mut c_char {
	return (*var).value;/* so it can change later */
}

unsafe fn function_cell(var:*mut SHELL_VAR) ->* mut COMMAND {
	return (*var).value as * mut COMMAND;
}

unsafe fn VUNSETATTR(var:*mut SHELL_VAR,attr:i32) {
	(*var).attributes &= !attr;
}

unsafe fn array_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_array!();
}

unsafe fn assoc_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_assoc!();
}

unsafe fn var_isset(var:*mut SHELL_VAR) ->bool {
	return (*var).value !=std::ptr::null_mut();
}

unsafe fn tempvar_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_tempvar!();
}

unsafe fn noassign_p(var:*mut SHELL_VAR) ->i32 {
	return (*var).attributes & att_noassign!();
}

#[no_mangle]
pub extern "C" fn r_declare_internal (mut list:* mut WordList, local_var:i32)->i32
{
  let mut flags_on:i32=0;
  let mut flags_off:i32=0;
  let mut flags:* mut i32;
  let mut any_failed:i32=0;
  let mut assign_error:i32=0;
  let mut pflag:i32=0;
  let mut nodefs:i32=0;
  let mut opt:i32;
  let mut onref:i32;
  let mut offref:i32;
  let mut mkglobal:i32=0;
  let mut chklocal:i32=0;
  let mut inherit_flag:i32=0;

  let mut t: *mut c_char;
  let mut subscript_start: *mut c_char;
  let mut var:*mut SHELL_VAR;
  let mut refvar:*mut SHELL_VAR;
  let mut v:*mut SHELL_VAR;

  let mut shell_fn:*mut function_def;

  refvar = std::ptr::null_mut();
  unsafe {
  reset_internal_getopt ();
  let tmp = DECLARE_OPTS();
  opt = internal_getopt (list, tmp.as_ptr() as * mut c_char);
  while  opt != -1 {
      if list_opttype == '+' as i32 {
        flags= &mut flags_off;
      } else {
        flags= &mut flags_on;
      }
     
      let optu8:u8= opt as u8;
      let optChar:char=char::from(optu8);

      /* If you add options here, see whether or not they need to be added to
	 the loop in subst.c:shell_expand_word_list() */
      match optChar {
        'a'=>{ *flags |= att_array!();}
		'A'=>{ *flags |= att_assoc!();}
        'p'=>{ pflag+=1;}
        'F'=>{ nodefs+=1;
              *flags |= att_function!();
             }
        'f'=>{ *flags |= att_function!();}
        'G'=>{ 
              if flags == &mut flags_on {
                chklocal = 1;
              }
             }
        'g'=>{
          if flags == &mut flags_on {
            mkglobal = 1;
          }
        }
        'i'=>{ *flags |= att_integer!();}
        'n'=>{ *flags |= att_nameref!();}
        'r'=>{ *flags |= att_readonly!();}
        't'=>{ *flags |= att_trace!();}
        'x'=>{ *flags |= att_exported!();
               array_needs_making = 1;
             }
        'c'=>{ *flags |= att_capcase!();
              if flags == &mut flags_on {
                flags_off |= att_uppercase!() | att_lowercase!();
              }
             }
        'l'=>{ *flags |= att_lowercase!();
              if flags == &mut flags_on {
                flags_off |= att_capcase!()| att_uppercase!();
              }
             }
        'u'=>{ *flags |= att_uppercase!();
              if flags == &mut flags_on {
                flags_off |= att_capcase!()| att_lowercase!();
              }
             }
        'I'=>{ inherit_flag = MKLOC_INHERIT!();}
        _=>{
			if opt == -99 {
				r_builtin_help();
				return EX_USAGE;
			}
			 builtin_usage ();
             return EX_USAGE;
            }
	    }
		opt = internal_getopt (list, tmp.as_ptr() as * mut c_char);
  }
    list = loptend;
  /* If there are no more arguments left, then we just want to show
     some variables. */
  if list == std::ptr::null_mut() {	/* declare -[aAfFirtx] */
      /* Show local variables defined at this context level if this is
	 the `local' builtin. */
      if local_var != 0 {
        show_local_var_attributes (0, nodefs);	/* XXX - fix up args later */
      } else if pflag != 0 && (flags_on == 0 || flags_on == att_function!()) {
        let mut ret=0;
        if flags_on == 0 {
          ret=1;
        }
        show_all_var_attributes (ret, nodefs);
      } else if flags_on == 0 {
        return set_builtin (std::ptr::null_mut());
      } else {
        set_or_show_attributes (std::ptr::null_mut(), flags_on, nodefs);
      }
      return sh_chkwrite (EXECUTION_SUCCESS!());
  }

  if pflag !=0 {	/* declare -p [-aAfFirtx] name [name...] */
      any_failed=0;
      while  list != std::ptr::null_mut() {
        if (flags_on & att_function!()) != 0 {
          pflag = show_func_attributes ((*(*list).word).word, nodefs);
        } else if local_var !=0 {
          pflag = show_localname_attributes ((*(*list).word).word, nodefs);
        } else {
          pflag = show_name_attributes ((*(*list).word).word, nodefs);
        }
        if pflag !=0 {
            sh_notfound ((*(*list).word).word);
            any_failed += 1;
        }
         list = (*list).next;
	  }

      if any_failed !=0 {
        return EXECUTION_FAILURE!();
      } else {
        return EXECUTION_SUCCESS!();
      }
  }
  let tmpValue = CString::new("").unwrap();

  /* There are arguments left, so we are making variables. */
 'outter: while list !=std::ptr::null_mut() {		/* declare [-aAfFirx] name [name ...] */
      let mut value:* mut c_char;
	  let mut name:* mut c_char;
	  let mut oldname:* mut c_char;
      let mut offset:i32;
	  let mut aflags:i32;
	  let wflags:i32;
	  let mut created_var:i32;
	  let mut namelen:i32;
      let assoc_noexpand:bool;

      let mut making_array_special:i32;
	  let mut compound_array_assign:i32;
	  let mut simple_array_assign:i32;
      let mut var_exists:i32;
	  let mut array_exists:i32;
	  let mut creating_array:i32;
	  let mut array_subscript_assignment:bool;

      name = r_savestring ((*(*list).word).word);
      wflags = (*(*list).word).flags;

      assoc_noexpand = (assoc_expand_once !=0 && (wflags & (1 << 2)) !=0);
      //　分出=
      if assoc_noexpand {
		offset = assignment (name,  2);
	  } else {
		offset = assignment (name,  0);
	  }

      aflags = 0;
      created_var = 0;

      if local_var !=0 && variable_context !=0 && STREQ (name, CString::new("-").unwrap().as_ptr())	{
		var = make_local_variable (CString::new("-").unwrap().as_ptr(), 0);
        if value_cell(var) != std::ptr::null_mut() {
		  libc::free (value_cell (var) as * mut c_void);		/* just in case */
        }
		value = get_current_options ();
		var_setvalue (var, value);
		VSETATTR (var, att_invisible!());
		libc::free (name as * mut c_void);
		list = (*list).next;
		continue 'outter;
	  }

      if offset !=0 {	/* declare [-aAfFirx] name=value */
	  	*name.offset(offset as isize) = '\0' as c_char;
	  	value = name.offset((offset + 1) as isize) ; 
	  	if *(name.offset((offset - 1) as isize)) == '+' as c_char {
	      aflags |= ASS_APPEND!();
	      *(name.offset((offset - 1) as isize))= '\0' as c_char;
	    }
	  } else {
		value = tmpValue.as_ptr() as * mut c_char;
	  }
      /* Do some lexical error checking on the LHS and RHS of the assignment
	 that is specific to nameref variables. */
      if (flags_on & att_nameref!()) !=0 {
		if valid_array_reference (name, 0) !=0 {
			builtin_error (CString::new("%s: reference variable cannot be an array").unwrap().as_ptr(), name);
			assign_error+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		} else if check_selfref (name, value, 0) !=0 {/* disallow self references at global scope, warn at function scope */
	        if variable_context == 0 {
				builtin_error (CString::new("%s: nameref variable self references not allowed").unwrap().as_ptr(), name);
				assign_error+=1;
				libc::free (name as * mut c_void);
				list = (*list).next;
				continue 'outter;
			} else {
				builtin_warning (CString::new("%s: circular name reference").unwrap().as_ptr(), name);
			}
	    }

	    if value != std::ptr::null_mut() && (*value) !=0 && (aflags & ASS_APPEND!()) == 0 && valid_nameref_value (value, 1) == 0 {
	      builtin_error (CString::new("`nvalid %s': ivariable name for name reference").unwrap().as_ptr(), value);
	      assign_error+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter;
	    }
	 }
//restart_new_var_name:
	'inner: loop {
      var_exists = 0;
	  array_exists = 0;
	  creating_array = 0;
      compound_array_assign = 0;
	  simple_array_assign = 0;
      array_subscript_assignment = false;
      subscript_start = std::ptr::null_mut();
	  t = libc::strchr (name, '[' as libc::c_int);
      if t !=std::ptr::null_mut() && (flags_on & att_function!()) == 0	{/* ] */
	  /* If offset != 0 we have already validated any array reference
	     because assignment() calls skipsubscript() */
		if offset == 0 && valid_array_reference (name, 0) == 0 {
			sh_invalidid (name);
			assign_error+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter; 
		}

		subscript_start = t;
		*t = '\0' as c_char;
		making_array_special = 1;	/* XXX - should this check offset? */
		array_subscript_assignment = offset != 0;
	 } else {
		making_array_special = 0;
	 }
      /* If we're in posix mode or not looking for a shell function (since
	 shell function names don't have to be valid identifiers when the
	 shell's not in posix mode), check whether or not the argument is a
	 valid, well-formed shell identifier. */
	 if (posixly_correct !=0 || (flags_on & att_function!()) == 0) && legal_identifier(name) == 0 {
		sh_invalidid (name);
	  	assign_error+=1;
		libc::free (name as * mut c_void);
		list = (*list).next;
		continue 'outter; 
	 }
      /* If VARIABLE_CONTEXT has a non-zero value, then we are executing
	 inside of a function.  This means we should make local variables,
	 not global ones. */

      /* XXX - this has consequences when we're making a local copy of a
	       variable that was in the temporary environment.  Watch out
	       for this. */
     refvar = std::ptr::null_mut();
     if variable_context !=0 && mkglobal == 0 && ((flags_on & att_function!()) == 0) {
	  	 let newname: * mut c_char;

		 /* check name for validity here? */
		 var = find_variable (name);
	     if var == std::ptr::null_mut() {
			newname = nameref_transform_name (name, ASS_MKLOCAL!());
		 }
		 else if (flags_on & att_nameref!()) == 0 && (flags_off & att_nameref!()) == 0 {
	      /* Ok, we're following namerefs here, so let's make sure that if
		 we followed one, it was at the same context (see below for
		 more details). */
	      refvar = find_variable_last_nameref (name, 1);
		  if refvar != std::ptr::null_mut() && (*refvar).context != variable_context {
			newname =  name ;
		  }
		 else {
			newname =  (*var).name;
		}
	      refvar = std::ptr::null_mut();
	    } else {
			newname = name;	/* dealing with nameref attribute */
		}
// 至此，find_variable 返回var 没有被更新
	    /* Pass 1 as second argument to make_local_{assoc,array}_variable
	     return an existing {array,assoc} variable to be flagged as an
	     error below. */
		if (flags_on & att_assoc!()) !=0 {
			var = make_local_assoc_variable (newname, MKLOC_ARRAYOK!()|inherit_flag);
		} else if (flags_on & att_array!()) !=0 || making_array_special !=0 {
			var = make_local_array_variable (newname, MKLOC_ASSOCOK!()|inherit_flag);
		} else if offset == 0 && (flags_on & att_nameref!()) !=0 {
	      /* First look for refvar at current scope */
	      refvar = find_variable_last_nameref (name, 1);
	      /* VARIABLE_CONTEXT != 0, so we are attempting to create or modify
		 the attributes for a local variable at the same scope.  If we've
		 used a reference from a previous context to resolve VAR, we
		 want to throw REFVAR and VAR away and create a new local var. */
	      if refvar != std::ptr::null_mut() && (*refvar).context != variable_context {
		  	refvar = std::ptr::null_mut();
		  	var = make_local_variable (name, inherit_flag);
		  } else if refvar != std::ptr::null_mut() && (*refvar).context == variable_context {
			var = refvar;
		  } else if var == std::ptr::null_mut() || (*refvar).context != variable_context {/* Maybe we just want to create a new local variable */
			var = make_local_variable (name, inherit_flag);
		  }
	      /* otherwise we have a var at the right context */
	    } else {
			 /* XXX - check name for validity here with valid_nameref_value */
			 if flags_on & att_nameref!() !=0 {
				var = make_local_variable ( name , inherit_flag);
			 } else {
				var = make_local_variable ( newname, inherit_flag);	/* sets att_invisible for new vars */
			 }
		}

	    if var == std::ptr::null_mut() {
	      any_failed+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter; 
	    }

	  	if var != std::ptr::null_mut() && nameref_p (var) !=0 && readonly_p (var) != 0 && nameref_cell (var) != std::ptr::null_mut() && (flags_off & att_nameref!()) !=0 {
	      sh_readonly (name);
	      any_failed+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter; 
	    }
	} else {
		var = std::ptr::null_mut();
	}
      /* If we are declaring a function, then complain about it in some way.
	 We don't let people make functions by saying `typeset -f foo=bar'. */

      /* There should be a way, however, to let people look at a particular
	 function definition by saying `typeset -f foo'. */

    if (flags_on & att_function!()) !=0	{
	  if offset !=0 {	/* declare -f [-rix] foo=bar */   
	      builtin_error (CString::new("cannot use `-f' to make functions").unwrap().as_ptr());
	      libc::free (name as * mut c_void);
	      return EXECUTION_FAILURE!();
	  } else {/* declare -f [-rx] name [name...] */
	      var = find_function (name);
	      if var != std::ptr::null_mut() {
		  if readonly_p (var) !=0 && (flags_off & att_readonly!()) !=0 {
		      builtin_error (CString::new("%s: readonly function").unwrap().as_ptr(), name);
		      any_failed+=1;
		      libc::free (name as * mut c_void);
			  list = (*list).next;
			  continue 'outter; 
		  } else if (flags_on & (att_array!()|att_assoc!())) !=0 {
			  if (flags_on & att_array!()) !=0 {
				sh_invalidopt (CString::new("-a").unwrap().as_ptr() as * mut c_char);
			  } else {
				sh_invalidopt (CString::new("-A").unwrap().as_ptr() as * mut c_char);
			  }

		      any_failed+=1;
		      libc::free (name as * mut c_void);
			  list = (*list).next;
			  continue 'outter; 
		  }
		  /* declare -[Ff] name [name...] */
		  if flags_on == att_function!() && flags_off == 0 {
		      if nodefs !=0 && debugging_mode !=0 {
			  	shell_fn = find_function_def ((*var).name);
				if shell_fn !=std::ptr::null_mut() {
					println!("{} {} {}",CStr::from_ptr((*var).name).to_str().unwrap(),(*shell_fn).line,CStr::from_ptr((*shell_fn).source_file).to_str().unwrap());
				} else {
					println!("{}",CStr::from_ptr((*var).name).to_str().unwrap());
				}
			  } else {
				  if nodefs !=0 {
					t=(*var).name;
				  } else {
					t = named_function_string (name, function_cell (var), FUNC_MULTILINE!()|FUNC_EXTERNAL!());
				  }
				  println!("{}",CStr::from_ptr(t).to_str().unwrap());
			      any_failed = sh_chkwrite (any_failed);
			 }
		  } else {	/* declare -[fF] -[rx] name [name...] */ 
		      VSETATTR (var, flags_on);
		      flags_off &= ! att_function!(); 	/* makes no sense */
		      VUNSETATTR (var, flags_off);
		  }
		} else {
			any_failed+=1;
		}
		libc::free (name as * mut c_void);
		list = (*list).next;
		continue 'outter; 
	    }
	} else {
		/* declare -[aAinrx] name [name...] */
	 /* Non-null if we just created or fetched a local variable. */

	  /* Here's what ksh93 seems to do as of the 2012 version: if we are
	     using declare -n to modify the value of an existing nameref
	     variable, don't follow the nameref chain at all and just search
	     for a nameref at the current context.  If we have a nameref,
	     modify its value (changing which variable #define ASS_NAMEREF	0x0010	/* assigning to nameref variable */it references). */
	  if var == std::ptr::null_mut() && (flags_on & att_nameref!()) !=0 {
	      /* See if we are trying to modify an existing nameref variable,
		 but don't follow the nameref chain. */
		 if mkglobal !=0 {
			var = find_global_variable_noref (name);
		 } else {
			var = find_variable_noref (name);
		 }

	     if var != std::ptr::null_mut() && nameref_p (var) == 0 {
			var = std::ptr::null_mut();
		 }
	  } else if var == std::ptr::null_mut() && (flags_off & att_nameref!()) !=0 {
		  /* However, if we're turning off the nameref attribute on an existing
	     nameref variable, we first follow the nameref chain to the end,
	     modify the value of the variable this nameref variable references
	     if there is an assignment statement argument,
	     *CHANGING ITS VALUE AS A SIDE EFFECT*, then turn off the nameref
	     flag *LEAVING THE NAMEREF VARIABLE'S VALUE UNCHANGED* */
	      /* See if we are trying to modify an existing nameref variable */
		  if mkglobal !=0 {
			refvar=find_global_variable_last_nameref (name, 0);
		  } else {
			refvar=find_variable_last_nameref (name, 0);
		  }

	      if refvar != std::ptr::null_mut() && nameref_p (refvar) == 0 {
			refvar = std::ptr::null_mut();
		  }
	      /* If the nameref is readonly but doesn't have a value, ksh93
		 allows the nameref attribute to be removed.  If it's readonly
		 and has a value, even if the value doesn't reference an
		 existing variable, we disallow the modification */
	      if refvar != std::ptr::null_mut() && nameref_cell (refvar) != std::ptr::null_mut() && readonly_p (refvar) != 0 {
			sh_readonly (name);
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter; 
		  }
	      /* If all we're doing is turning off the nameref attribute, don't
		 bother with VAR at all, whether it exists or not. Just turn it
		 off and go on. */
	      if refvar != std::ptr::null_mut() && flags_on == 0 && offset == 0 && (flags_off & !att_nameref!()) == 0 {
		  	VUNSETATTR (refvar, att_nameref!());
		  	libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter; 
		  }

	      if refvar !=std::ptr::null_mut() {
			/* XXX - use declare_find_variable here? */
			if mkglobal !=0 {
				var = find_global_variable (nameref_cell (refvar));
			} else {
				var =find_variable (nameref_cell (refvar));
			}
		  }
	    } else if var == std::ptr::null_mut() && offset !=0 && array_subscript_assignment {
			/* If we have an array assignment to a nameref, remove the nameref
	     attribute and go on. */
		 if mkglobal !=0 {
			var = find_global_variable_noref (name);
		} else {
			var = find_variable_noref (name);
		}

	    if var !=std::ptr::null_mut() && nameref_p (var) !=0 {
		  internal_warning (CString::new("%s: removing nameref attribute").unwrap().as_ptr(), name);
		  libc::free (value_cell (var) as * mut c_void);	/* XXX - bash-4.3 compat */
		  var_setvalue (var, std::ptr::null_mut());
		  VUNSETATTR (var, att_nameref!());
		}
	  }

	  /* See if we are trying to set flags or value (or create) for an
	     existing nameref that points to a non-existent variable: e.g.,
		declare -n foo=bar
		unset foo	# unsets bar
		declare -i foo
		foo=4+4
		declare -p foo */
	  if var == std::ptr::null_mut() && (mkglobal !=0 || flags_on !=0 || flags_off !=0 || offset !=0) {
		  if mkglobal !=0 {
			refvar = find_global_variable_last_nameref (name, 0);
		  } else {
			refvar = find_variable_last_nameref (name, 0);
		  }

	      if refvar != std::ptr::null_mut() && nameref_p (refvar) == 0 {
			refvar = std::ptr::null_mut();
		  }

	      if refvar !=std::ptr::null_mut() {
			  /* XXX - use declare_find_variable here? */
			  if mkglobal !=0 {
				var = find_global_variable (nameref_cell (refvar)) ;
			  } else {
				var = find_variable (nameref_cell (refvar));
			  }
		  }

	      if refvar !=std::ptr::null_mut() && var == std::ptr::null_mut() {
		  	  oldname = name;	/* need to free this */
		      namelen = libc::strlen (nameref_cell (refvar)) as i32;

		      if subscript_start != std::ptr::null_mut() {
		      	*subscript_start = '[' as c_char;		/*]*/
		        namelen += libc::strlen (subscript_start) as i32;
		      }

		  name = libc::malloc (namelen as libc::size_t + 2 + libc::strlen (value) + 1 ) as * mut c_char ;
		  libc::strcpy (name, nameref_cell (refvar));

		  if subscript_start != std::ptr::null_mut() {
			libc::strcpy (name.offset(libc::strlen (nameref_cell (refvar)) as isize), subscript_start);
		  }

		  /* We are committed to using the new name, so reset */
		  if offset !=0 {
		      /* Rebuild assignment and restore offset and value */
		      if (aflags & ASS_APPEND!()) !=0 {
				*(name.offset( namelen as isize)  as * mut c_char) = '+' as c_char;

				namelen+=1;
			  }
              *(name.offset( namelen as isize)  as * mut c_char) = '=' as c_char;
			//   *((name as usize + namelen as usize) as * mut c_char) = '=' as c_char;
		      namelen+=1;

			  if value != std::ptr::null_mut() && (*value) !=0 {
				libc::strcpy (name.offset(namelen as isize), value);
			  } else {
				*(name.offset( namelen as isize)  as * mut c_char) = '\0' as c_char;
			  }

		      offset = assignment (name, 0);
		      /* if offset was valid previously, but the substituting
			 of the nameref value results in an invalid assignment,
			 throw an invalid identifier error */
		      if offset == 0 {
				libc::free (oldname as * mut c_void);
				sh_invalidid (name);
				assign_error+=1;
				libc::free (name as * mut c_void);
				list = (*list).next;
				continue 'outter; 
			  }
		        *(name.offset(offset as isize)) = '\0' as c_char;
			      	      
		      value = name.offset(namelen as isize) ;
		    }
		    libc::free (oldname as * mut c_void);

			/* OK, let's turn off the nameref attribute.
				Now everything else applies to VAR. */
		    if (flags_off & att_nameref!()) !=0 {
				VUNSETATTR (refvar, att_nameref!());
			}

			//goto restart_new_var_name;
			continue 'inner;
			/* NOTREACHED */
		  }
	    }
	    if var == std::ptr::null_mut() {
			var = r_declare_find_variable (name, mkglobal, chklocal);
		}

		var_exists = (var != std::ptr::null_mut()) as i32;
	    array_exists = (var != std::ptr::null_mut() && (array_p (var) !=0 || assoc_p (var) !=0 )) as i32;
	    creating_array = flags_on & (att_array!()|att_assoc!());

	    if var == std::ptr::null_mut() {
	      if (flags_on & att_assoc!()) !=0 {
		  	var = make_new_assoc_variable (name);
		    if var != std::ptr::null_mut() && offset == 0 {
			  VSETATTR (var, att_invisible!());
		    }
		  } else if (flags_on & att_array!()) !=0 || making_array_special !=0  {
		  	var = make_new_array_variable (name);
		  	if var != std::ptr::null_mut() && offset == 0 {
				VSETATTR (var, att_invisible!());
			}
		} else {
		  if mkglobal !=0 {
			var=bind_global_variable (name, std::ptr::null_mut(), ASS_FORCE!());
		  }	else {
			var= bind_variable (name, std::ptr::null_mut(), ASS_FORCE!());
		  }
		  if var != std::ptr::null_mut() && offset == 0 {
			VSETATTR (var, att_invisible!());
		  }
		}

		if var == std::ptr::null_mut() {
		  /* Has to appear in brackets */
		  libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter;
		}
	      created_var = 1;
	   } else if (array_p (var) !=0 || assoc_p (var) !=0 ) && (flags_on & att_nameref!()) !=0 {
			 /* Can't take an existing array variable and make it a nameref */
	      builtin_error (CString::new("%s: reference variable cannot be an array").unwrap().as_ptr(), name);
	      assign_error+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter;
	    } else if nameref_p (var) !=0 && (flags_on & att_nameref!()) == 0 && (flags_off & att_nameref!()) == 0 && offset !=0 && valid_nameref_value (value, 1) == 0 {
	      builtin_error (CString::new("`%s': invalid variable name for name reference").unwrap().as_ptr(), value);
	      any_failed+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter;
	    } else if (flags_on & att_nameref!()) !=0 {
	      /* Check of offset is to allow an assignment to a nameref var as
		 part of the declare word to override existing value */
	      if nameref_p (var) == 0 && var_isset (var) && offset == 0 && valid_nameref_value (value_cell (var), 0) == 0 {
			builtin_error (CString::new("`%s': invalid variable name for name reference").unwrap().as_ptr(), value_cell (var));
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		  }

	      if readonly_p (var) !=0 {
			sh_readonly (name);
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		  }
	      /* ksh93 compat: turning on nameref attribute turns off -ilu */
	      VUNSETATTR (var, att_integer!()|att_uppercase!()|att_lowercase!()|att_capcase!());
	    }

		/* Cannot use declare +r to turn off readonly attribute. */
		if readonly_p (var) !=0 && (flags_off & att_readonly!()) !=0 {
			sh_readonly ((*var).name);
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		}

		/* Cannot use declare to assign value to readonly or noassign
			variable. */
		if (readonly_p (var) !=0 || noassign_p (var)!=0 ) && offset !=0 {
			if readonly_p (var) !=0 {
				sh_readonly (name);
			}
			assign_error+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		}

		/* make declare a[2]=foo as similar to a[2]=foo as possible if
			a is already an array or assoc variable. */
		if array_subscript_assignment && array_exists !=0 && creating_array == 0 {
			simple_array_assign = 1;
		} else if (making_array_special !=0 || creating_array !=0 || array_exists !=0) && offset!=0 {
	      let mut vlen:i32;
	      vlen = libc::strlen (value) as i32;
		  /*itrace("declare_builtin: name = %s value = %s flags = %d", name, value, wflags);*/
	      if shell_compatibility_level > 43 && (wflags & W_COMPASSIGN!()) == 0 && *value == '(' as c_char && *(value.offset((vlen-1) as isize) as * mut c_char) == ')' as c_char {
		  /* The warning is only printed when using compound assignment
		     to an array variable that doesn't already exist.  We use
		     creating_array to allow things like
		     declare -a foo$bar='(abc)' to work. */
			if array_exists == 0 && creating_array == 0 {
				internal_warning (CString::new("%s: quoted compound array assignment deprecated").unwrap().as_ptr(), (*(*list).word).word);
			}
			compound_array_assign = (array_exists !=0 || creating_array !=0) as i32;
			simple_array_assign = making_array_special;
		 } else if *value == '(' as c_char && *(value.offset((vlen-1) as isize) as * mut c_char) == ')' as c_char && (shell_compatibility_level < 44 || (wflags & W_COMPASSIGN!()) !=0 ) {
			compound_array_assign = 1;
		 } else {
			simple_array_assign = 1;
		 }
	    }

		/* Cannot use declare +a name or declare +A name to remove an
			array variable. */
		if ((flags_off & att_array!()) !=0 && array_p (var) !=0) || ((flags_off & att_assoc!()) !=0 && assoc_p (var) !=0)	{
			builtin_error (CString::new("%s: cannot destroy array variables in this way").unwrap().as_ptr(), name);
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		}

		if (flags_on & att_array!()) !=0 && assoc_p (var) !=0 {
			builtin_error (CString::new("%s: cannot convert associative to indexed array").unwrap().as_ptr(), name);
			any_failed+=1;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		}

	    if (flags_on & att_assoc!()) !=0 && array_p (var) !=0 {
	      builtin_error (CString::new("%s: cannot convert indexed to associative array").unwrap().as_ptr(), name);
	      any_failed+=1;
	      libc::free (name as * mut c_void);
		  list = (*list).next;
		  continue 'outter;
	    }

	    /* declare -A name[[n]] makes name an associative array variable. */
	    if (flags_on & att_assoc!()) !=0 {
	      if assoc_p (var) == 0 {
			var = convert_var_to_assoc (var);
		  }
	    } else if (making_array_special !=0 || (flags_on & att_array!()) !=0 ) && array_p (var) == 0 && assoc_p (var) == 0 {
		  /* declare -a name[[n]] or declare name[n] makes name an indexed
			array variable. */
			var = convert_var_to_array (var);
	    }

		/* XXX - we note that we are turning on nameref attribute and defer
			setting it until the assignment has been made so we don't do an
			inadvertent nameref lookup.  Might have to do the same thing for
			flags_off&att_nameref. */
		/* XXX - ksh93 makes it an error to set a readonly nameref variable
			using a single typeset command. */
	    onref = flags_on & att_nameref!();
	    flags_on &= !att_nameref!();

		if array_p (var) !=0 || assoc_p (var) !=0 || (offset !=0 && compound_array_assign !=0) || simple_array_assign !=0 {
			onref = 0;		/* array variables may not be namerefs */
		}
		/* ksh93 seems to do this */
		offref = flags_off & att_nameref!();
		flags_off &= !att_nameref!();
        
		VSETATTR (var, flags_on);
		VUNSETATTR (var, flags_off);

	  if offset !=0 && compound_array_assign !=0 {
		assign_array_var_from_string (var, value, aflags|ASS_FORCE!());
	  } else if simple_array_assign !=0 && subscript_start !=std::ptr::null_mut() {
	      let mut local_aflags:i32;
	      /* declare [-aA] name[N]=value */
	      *subscript_start = '[' as c_char;	/* ] */
	      /* XXX - problem here with appending */
	      local_aflags = aflags&ASS_APPEND!();
		  if assoc_noexpand  {
			local_aflags |= ASS_NOEXPAND!();
		  } else {
			local_aflags |= 0;
		  }

	      var = assign_array_element (name, value, local_aflags);	/* XXX - not aflags */
	      *subscript_start = '\0' as c_char;
	      if var == std::ptr::null_mut() {/* some kind of assignment error */
			assign_error+=1;
			flags_on |= onref;
			flags_off |= offref;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		  }
	    } else if simple_array_assign !=0 {
	      /* let bind_{array,assoc}_variable take care of this. */
	      if assoc_p (var) !=0 {
			bind_assoc_variable (var, name, r_savestring (CString::new("0").unwrap().as_ptr()), value, aflags|ASS_FORCE!());
		  } else {
			bind_array_variable (name, 0, value, aflags|ASS_FORCE!());
		  }
	    } else if offset !=0 {
			/* XXX - no ASS_FORCE here */
	        /* bind_variable_value duplicates the essential internals of bind_variable() */
	      if onref !=0 || nameref_p (var) !=0 {
			aflags |= ASS_NAMEREF!();
		  }

	      v = bind_variable_value (var, value, aflags);
	      if v == std::ptr::null_mut() && (onref !=0 || nameref_p (var) !=0) {
			if valid_nameref_value (value, 1) == 0 {
				sh_invalidid (value);
			}

			assign_error+=1;
			/* XXX - unset this variable? or leave it as normal var? */
			if created_var !=0 {
				if mkglobal !=0 {
					delete_var ( (*var).name, global_variables);
				} else {
					delete_var ( (*var).name, shell_variables);
				}
			}

			flags_on |= onref;/* undo change from above */
			flags_off |= offref;
			libc::free (name as * mut c_void);
			list = (*list).next;
			continue 'outter;
		  }
	    }

	  /* If we found this variable in the temporary environment, as with
	     `var=value declare -x var', make sure it is treated identically
	     to `var=value export var'.  Do the same for `declare -r' and
	     `readonly'.  Preserve the attributes, except for att_tempvar. */
	  /* XXX -- should this create a variable in the global scope, or
	     modify the local variable flags?  ksh93 has it modify the
	     global scope.
	     Need to handle case like in set_var_attribute where a temporary
	     variable is in the same table as the function local vars. */
	  if (flags_on & (att_exported!()|att_readonly!()) !=0 ) && tempvar_p (var) !=0 {
		 let mut tv:* mut SHELL_VAR;
	     let mut tvalue:* mut c_char=std::ptr::null_mut();

	     tv = find_tempenv_variable ((*var).name);
	     if tv != std::ptr::null_mut() {
			  if var_isset(var) {
				tvalue = r_savestring (value_cell (var));
			  } else {
				tvalue = r_savestring (CString::new("").unwrap().as_ptr());
			  }
	          tv = bind_variable ((*var).name, tvalue, 0);

			  if tv != std::ptr::null_mut() {
		      	(*tv).attributes |= (*var).attributes & !att_tempvar!();
				if (*tv).context > 0 {
					VSETATTR (tv, att_propagate!());
				}
		      }
	          libc::free (tvalue as * mut c_void);
		 }
	     VSETATTR (var, att_propagate!());
	    }
	}

      /* Turn on nameref attribute we deferred above. */
      /* XXX - should we turn on the noassign attribute for consistency with
	  ksh93 when we turn on the nameref attribute? */
      VSETATTR (var, onref);
      flags_on |= onref;
      VUNSETATTR (var, offref);
      flags_off |= offref;
      /* Yuck.  ksh93 compatibility.  XXX - need to investigate more but
	  definitely happens when turning off nameref attribute on nameref
	  (see comments above).  Under no circumstances allow this to turn
	  off readonly attribute on readonly nameref variable. */
      if refvar !=std::ptr::null_mut() {
		if (flags_off & att_readonly!()) !=0 {
			flags_off &= !att_readonly!();
		}
		VUNSETATTR (refvar, flags_off);
	  }
      stupidly_hack_special_variables (name);
	  libc::free (name as * mut c_void);
	  list = (*list).next;
	  continue 'outter;
     }
	}
  if assign_error !=0 {
	return EX_BADASSIGN!();
  } else {
	  if any_failed == 0 {
		return EXECUTION_SUCCESS!();
	  } else {
		return EXECUTION_FAILURE!();
	  }
  }
}
}
