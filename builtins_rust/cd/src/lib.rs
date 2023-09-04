extern crate  libc;
extern crate nix;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};

use libc::{c_char, c_long, c_void};
use std::{ffi::{CString, CStr}};
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


#[macro_export]
macro_rules! DUP_JOB {
   () => {-2}
}

#[macro_export]
macro_rules! readonly_p {
  ($var:expr) => {
    (*$var).attributes & 0x0000002
  }
}

#[macro_export]
macro_rules! exported_p {
  ($var:expr) => {
    (*$var).attributes & 0x0000001
  }
}

#[macro_export]
macro_rules! LCD_DOVARS {
  () => {
    0x001
  }
}
#[macro_export]
macro_rules! LCD_DOSPELL {
  () => {
    0x002
  }
}
#[macro_export]
macro_rules! LCD_PRINTPATH {
  () => {
    0x004
  }
}

#[macro_export]
macro_rules! LCD_FREEDIRNAME {
  () => {
    0x008
  }
}

#[macro_export]
macro_rules! MP_DOTILDE {
  () => {
    0x01
  }
}

#[macro_export]
macro_rules! PATH_CHECKDOTDOT {
  () => {
    0x0001
  }
}

#[macro_export]
macro_rules! PATH_CHECKEXISTS {
  () => {
    0x0002
  }
}

#[macro_export]
macro_rules! errno {
  () => {
    *libc::__errno_location()
  }
}

extern "C" {
    fn builtin_error(err:*const c_char,...);
    
    static mut loptend:*mut WordList;

    static mut array_needs_making:i32;
    fn bind_variable ( lhs:*const c_char, rhs:* mut c_char, i:i32)->* mut SHELL_VAR;
    fn update_export_env_inplace (env_prefix:* mut c_char, preflen:i32, value:* mut c_char);
    static mut the_current_working_directory:* mut c_char;
    fn get_working_directory (for_whom:* mut c_char)->* mut c_char;
    fn sh_physpath (path:*mut c_char, flags:i32)->* mut c_char;
    fn sh_chkwrite (s:i32)->i32;
    fn get_string_value (var_name:*const c_char)->* mut c_char;   
    
    static mut restricted:i32;
    fn sh_restricted (s:*mut c_char);
    static no_symbolic_links:i32;
    
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
    fn builtin_usage();
    static cdable_vars:i32;
    static  interactive:i32;
    static cdspelling:i32;
    fn absolute_pathname (str:*const c_char)->i32;
    fn extract_colon_unit (string:*mut c_char, p_index:*mut i32)->* mut c_char;
    fn sh_makepath (path:*const c_char, dir:*const c_char, flags:i32)->* mut c_char;
    static privileged_mode:i32;
    fn dirspell (dirname:* mut c_char)->* mut c_char;
    fn printable_filename (fnc:* mut c_char, flags:i32)->* mut c_char;
    
    static posixly_correct:i32;
    fn same_file (path1:*const c_char, path2:*const c_char, stp1:*mut libc::stat, stp2:*mut libc::stat)->i32;
    fn make_absolute (str1:*const c_char, dot_path:*const c_char)->* mut c_char;
    fn sh_canonpath (path:* mut c_char, flags:i32)->* mut c_char;
    fn set_working_directory (path:* mut c_char);   
    
}

pub static mut xattrfd:i32=-1;
pub static  mut xattrflag:i32=0;
pub static  mut verbatim_pwd:i32=0;
pub static  mut eflag:i32=0;

/* How to bring a job into the foreground. */
#[no_mangle]
pub extern "C" fn r_setpwd (dirname:* mut c_char)->i32
{
  let old_anm:i32;
  let tvar:* mut SHELL_VAR;
  unsafe {
  old_anm = array_needs_making;
  
  let c_str_pwd = CString::new("PWD").unwrap();
  if dirname ==std::ptr::null_mut() {
    tvar=bind_variable (c_str_pwd.as_ptr(), CString::new("").unwrap().as_ptr() as * mut c_char, 0);    
  } else {
    tvar = bind_variable (c_str_pwd.as_ptr(), dirname , 0);
  }
  
  if tvar !=std::ptr::null_mut() && readonly_p! (tvar) !=0 {
    return EXECUTION_FAILURE!();
  }
  
  if tvar  !=std::ptr::null_mut() && old_anm == 0 && array_needs_making !=0 && exported_p !(tvar) !=0 {
    if dirname ==std::ptr::null_mut() {
      update_export_env_inplace (c_str_pwd.as_ptr() as * mut c_char, 4, CString::new("").unwrap().as_ptr() as * mut c_char);      
    } else {
      update_export_env_inplace (c_str_pwd.as_ptr() as * mut c_char, 4, dirname);
    }
    array_needs_making = 0;
  }
  return EXECUTION_SUCCESS!();
  }
}

#[no_mangle]
pub extern "C" fn r_bindpwd (no_symlinks:i32)->i32 {
  let mut dirname:*mut c_char;
  let pwdvar:*mut c_char;
  let old_anm:i32;
  let mut r:i32;
  let mut canon_failed:i32;
  let tvar:* mut SHELL_VAR;
  unsafe {
  r = sh_chkwrite (EXECUTION_SUCCESS!());
  if the_current_working_directory !=std::ptr::null_mut() {
    if no_symlinks !=0{
      dirname=sh_physpath (the_current_working_directory, 0);
    }else {
      dirname =the_current_working_directory;
    }
  } else {
    let c_str_cd = CString::new("cd").unwrap();
    dirname=get_working_directory(c_str_cd.as_ptr() as * mut c_char);
  }

  /* If canonicalization fails, reset dirname to the_current_working_directory */
  canon_failed = 0;
  if dirname == std::ptr::null_mut() {
      canon_failed = 1;
      dirname = the_current_working_directory;
  }

  old_anm = array_needs_making;
  let c_str_pwd = CString::new("PWD").unwrap();
  pwdvar = get_string_value (c_str_pwd.as_ptr());

  tvar = bind_variable (CString::new("OLDPWD").unwrap().as_ptr(), pwdvar, 0);
  if tvar !=std::ptr::null_mut() && readonly_p! (tvar) !=0{
      r = EXECUTION_FAILURE!();
  }

  if old_anm == 0 && array_needs_making !=0 && exported_p! (tvar) !=0 {
      update_export_env_inplace (CString::new("OLDPWD").unwrap().as_ptr() as * mut c_char, 7, pwdvar);
      array_needs_making = 0;
  }

  if r_setpwd (dirname) == EXECUTION_FAILURE!() {
    r = EXECUTION_FAILURE!();
  }

  if canon_failed !=0 && eflag !=0 {
    r = EXECUTION_FAILURE!();
  }

  if dirname !=std::ptr::null_mut() && dirname != the_current_working_directory {
    libc::free(dirname as * mut libc::c_void);
  }
    return r;
  }
}

/* Call get_working_directory to reset the value of
   the_current_working_directory () */
#[no_mangle]
pub extern "C" fn r_resetpwd (caller:*mut c_char)->*mut c_char {
  let tdir:*mut c_char;
  unsafe {
    libc::free(the_current_working_directory as * mut libc::c_void);
    the_current_working_directory = 0 as * mut c_char;
    tdir = get_working_directory (caller);
    return tdir;
  }
}

#[no_mangle]
pub extern "C" fn r_cdxattr (dir: *mut c_char, ndirp:*mut c_char)->i32 {
  return -1;
}

#[no_mangle]
pub extern "C" fn r_resetxattr () {
  unsafe {
    xattrfd = -1;		/* not strictly necessary */
  }
}

#[no_mangle]
pub extern "C" fn r_cd_builtin (mut list:*mut WordList)->i32 {
  let mut dirname:*mut c_char=std::ptr::null_mut();
  let cdpath:*mut c_char;
  let mut path:*mut c_char;
  let mut temp:*mut c_char;
  let mut path_index:i32;
  let mut no_symlinks:i32;
  let mut opt:i32;
  let mut lflag:i32;
  let e:i32;

  unsafe {
    if restricted !=0 {
        sh_restricted (0 as * mut c_char);
        return EXECUTION_FAILURE!();
    }

    eflag = 0;
    no_symlinks = no_symbolic_links;
    xattrflag = 0;
    reset_internal_getopt ();

    let c_str_elp = CString::new("eLP").unwrap(); // from a &str, creates a new allocation     
    opt = internal_getopt (list, c_str_elp.as_ptr() as * mut c_char);
    while  opt != -1 {
      let optu8:u8= opt as u8;
      let optChar:char=char::from(optu8);
      match optChar {
        'P'=>{no_symlinks = 1;}  
        'L'=>{no_symlinks = 0;} 
        'e'=>{eflag = 1;}
          _=>{
            if opt == -99 {
              r_builtin_help();
              return EX_USAGE;
          }
              builtin_usage ();
              return EX_USAGE;
            }
      }
      opt =internal_getopt (list, c_str_elp.as_ptr() as * mut c_char);
    }

    // list = loptend;     //后加的

    if cdable_vars != 0 {
      lflag=LCD_DOVARS!();
    } else {
      lflag=0;
    }

    if interactive !=0 && cdspelling !=0 {
      lflag=lflag | LCD_DOSPELL!();
    } else {
      lflag=lflag | 0;
    }

    if eflag !=0 && no_symlinks == 0{
      eflag = 0;
    }

    if loptend == std::ptr::null_mut()  {
      /* `cd' without arguments is equivalent to `cd $HOME' */
      dirname = get_string_value (CString::new("HOME").unwrap().as_ptr());

      if dirname == std::ptr::null_mut() {
        builtin_error (CString::new("HOME not set").unwrap().as_ptr());
        return EXECUTION_FAILURE!();
      }
      lflag = 0;
    }
    else if (*loptend).next != std::ptr::null_mut() {
        builtin_error (CString::new("too many arguments").unwrap().as_ptr());
        return EXECUTION_FAILURE!();
    }
    else if char::from((*(*(*loptend).word).word) as u8) == '-' && char::from(*((((*(*loptend).word).word) as usize +4) as *mut c_char) as u8) == '\0' {
      /* This is `cd -', equivalent to `cd $OLDPWD' */
      dirname = get_string_value (CString::new("OLDPWD").unwrap().as_ptr());
      if dirname == std::ptr::null_mut() {
        builtin_error (CString::new("OLDPWD not set").unwrap().as_ptr());
        return EXECUTION_FAILURE!();
      }
      lflag = LCD_PRINTPATH!();		/* According to SUSv3 */
    } 
    else if absolute_pathname ((*(*loptend).word).word) !=0 {
      dirname = (*(*loptend).word).word;
    }
    else if privileged_mode == 0 && get_string_value (CString::new("CDPATH").unwrap().as_ptr() ) != std::ptr::null_mut(){
      cdpath = get_string_value (CString::new("CDPATH").unwrap().as_ptr() );
      dirname = (*(*loptend).word).word;

      /* Find directory in $CDPATH. */
      path_index = 0;
      path = extract_colon_unit (cdpath, & mut path_index);

      while path  != std::ptr::null_mut()	{
        /* OPT is 1 if the path element is non-empty */
        opt = (char::from(*path as u8 )!= '\0') as i32 ;
        temp = sh_makepath (path, dirname, MP_DOTILDE!());
        libc::free (path as * mut c_void);

        if r_change_to_directory (temp, no_symlinks, xattrflag) !=0 {
          /* POSIX.2 says that if a nonempty directory from CDPATH
          is used to find the directory to change to, the new
          directory name is echoed to stdout, whether or not
          the shell is interactive. */
          if opt !=0 {
            if no_symlinks !=0 {
              path=temp;
            } 
            else {
              path=the_current_working_directory;
            }

            if path !=std::ptr::null_mut() {
              libc::printf(CString::new("%s\n").unwrap().as_ptr() as * const c_char,path);
            }
          } 

          libc::free (temp as * mut c_void);
          return r_bindpwd (no_symlinks);
        }	
        else {
          libc::free (temp as * mut c_void);
        }
        
        path = extract_colon_unit (cdpath, &mut path_index);
      }
    }  
    else{
        dirname = (*(*loptend).word).word;
    }


    /* When we get here, DIRNAME is the directory to change to.  If we
      chdir successfully, just return. */
    if 0 != r_change_to_directory (dirname, no_symlinks, xattrflag) {
      if (lflag  & LCD_PRINTPATH!()) !=0 {
          libc::printf(CString::new("%s\n").unwrap().as_ptr() as * const c_char,dirname);
      }
      return r_bindpwd (no_symlinks);
    }

    /* If the user requests it, then perhaps this is the name of
      a shell variable, whose value contains the directory to
      change to. */
    if (lflag & LCD_DOVARS!()) !=0 {
      temp = get_string_value (dirname);
      if temp != std::ptr::null_mut() && r_change_to_directory (temp, no_symlinks, xattrflag) !=0 {
        libc::printf(CString::new("%s\n").unwrap().as_ptr() as * const c_char,temp);
        return r_bindpwd (no_symlinks);
      }
    }

    /* If the user requests it, try to find a directory name similar in
      spelling to the one requested, in case the user made a simple
      typo.  This is similar to the UNIX 8th and 9th Edition shells. */
    if (lflag & LCD_DOSPELL!()) !=0 {
        temp = dirspell (dirname);
        if temp !=std::ptr::null_mut() && r_change_to_directory (temp, no_symlinks, xattrflag) !=0  {
          println!("{:?}", temp);
          libc::free (temp as * mut c_void);
          return r_bindpwd (no_symlinks);
        }  else {
          libc::free (temp as * mut c_void);
        }
    }

    e =errno!();
    temp = printable_filename (dirname, 0);
    builtin_error (CString::new("%s: %s").unwrap().as_ptr(), temp, libc::strerror (e));

    if temp != dirname {
      libc::free (temp as * mut c_void);
    }  
    return EXECUTION_FAILURE!();
  }
}

#[no_mangle]
pub extern "C" fn r_pwd_builtin (list:* mut WordList)->i32 {
  let mut directory:* mut c_char;
  let mut opt:i32;
  let mut pflag:i32;
  unsafe {
  verbatim_pwd = no_symbolic_links;
  pflag = 0;
  reset_internal_getopt ();
  let c_str_lp = CString::new("LP").unwrap(); // from a &str, creates a new allocation      
  opt = internal_getopt (list, c_str_lp.as_ptr() as * mut c_char);
  while  opt != -1 {
    let optu8:u8= opt as u8;
    let optChar:char=char::from(optu8);
    match optChar{      	 
      'P'=>{verbatim_pwd =1;
          pflag = 1;}
	    'L'=>{verbatim_pwd = 0;}
      _=>{
        if opt == -99 {
          r_builtin_help();
          return EX_USAGE;
      }
      builtin_usage ();
          return EX_USAGE;
        }
	  }
    opt = internal_getopt (list, c_str_lp.as_ptr() as * mut c_char);
  }
  if the_current_working_directory != std::ptr::null_mut() {
      if verbatim_pwd != 0 {
        directory=sh_physpath (the_current_working_directory, 0);
      } else {
        directory=the_current_working_directory;
      }
  } else {
    directory=get_working_directory(CString::new("pwd").unwrap().as_ptr() as * mut c_char);
  }

  /* Try again using getcwd() if canonicalization fails (for instance, if
     the file system has changed state underneath bash). */
  if (the_current_working_directory != std::ptr::null_mut() && directory == std::ptr::null_mut()) ||
      (posixly_correct !=0 && same_file (CString::new(".").unwrap().as_ptr(), the_current_working_directory, std::ptr::null_mut() , std::ptr::null_mut()) == 0) {
      if directory !=std::ptr::null_mut() && directory != the_current_working_directory {
        libc::free (directory as * mut c_void);
      }
      directory = r_resetpwd (CString::new("pwd").unwrap().as_ptr() as * mut c_char);
    }

  if directory != std::ptr::null_mut() {
      opt = EXECUTION_SUCCESS!();
      libc::printf(CString::new("%s\n").unwrap().as_ptr() as * const c_char,directory);
      /* This is dumb but posix-mandated. */
      if posixly_correct !=0 && pflag !=0 {
        opt = r_setpwd (directory);
      }
	
      if directory != the_current_working_directory {
        libc::free (directory as * mut c_void);
      }
      return sh_chkwrite (opt);
    } else {
      return EXECUTION_FAILURE!();
    }    
  }
}

/* Do the work of changing to the directory NEWDIR.  Handle symbolic
   link following, etc.  This function *must* return with
   the_current_working_directory either set to NULL (in which case
   getcwd() will eventually be called), or set to a string corresponding
   to the working directory.  Return 1 on success, 0 on failure. */
#[no_mangle]
pub extern "C" fn r_change_to_directory (newdir:* mut c_char, nolinks:i32, xattr:i32)->i32 {
  unsafe {
  let mut t:*mut c_char;
  let mut tdir:*mut c_char;
  let mut ndir:*mut c_char;
  let err:i32;
  let mut canon_failed:i32;
  let mut r:i32;
  let ndlen:i32;
  
  tdir = std::ptr::null_mut();

  if the_current_working_directory == std::ptr::null_mut() {
      t = get_working_directory (CString::new("chdir").unwrap().as_ptr() as * mut c_char);
      libc::free (t as * mut c_void);
  }

  t = make_absolute (newdir, the_current_working_directory);

  /* TDIR is either the canonicalized absolute pathname of NEWDIR
     (nolinks == 0) or the absolute physical pathname of NEWDIR
     (nolinks != 0). */
  if nolinks !=0 {
    tdir=sh_physpath (t, 0);
  } else {
    tdir=sh_canonpath (t, PATH_CHECKDOTDOT!()|PATH_CHECKEXISTS!());
  }

  ndlen = libc::strlen (newdir ) as i32;

  /* Use the canonicalized version of NEWDIR, or, if canonicalization
     failed, use the non-canonical form. */
  canon_failed = 0;
  if tdir !=std::ptr::null_mut() && *tdir !=0 {
    libc::free (t as * mut c_void);
  } else {
    libc::free (tdir as * mut c_void);
    tdir = t;
    canon_failed = 1;
  }

  /* In POSIX mode, if we're resolving symlinks logically and sh_canonpath
     returns NULL (because it checks the path, it will return NULL if the
     resolved path doesn't exist), fail immediately. */
  if posixly_correct !=0 && nolinks == 0 && canon_failed !=0 && (errno!() != libc::ENAMETOOLONG || ndlen > libc::PATH_MAX) {
      if errno!() != libc::ENOENT && errno!() != libc::ENAMETOOLONG {
        errno!() = libc::ENOTDIR;
      }
      libc::free (tdir as * mut c_void);
      return 0;
  }

  {
    if nolinks !=0 {
      r = libc::chdir (newdir);
    } else {
      r = libc::chdir (tdir);
    }

    if r >= 0 {
      r_resetxattr ();
    }

  }

  /* If the chdir succeeds, update the_current_working_directory. */
  if r == 0 {
      /* If canonicalization failed, but the chdir succeeded, reset the
	 shell's idea of the_current_working_directory. */
      if canon_failed !=0	{
	      t = r_resetpwd (CString::new("cd").unwrap().as_ptr() as * mut c_char);
        if t == std::ptr::null_mut(){
          set_working_directory (tdir);
        } else {
          libc::free (t as * mut c_void);         
        }
      } else {
        set_working_directory (tdir);
      }

      libc::free (tdir as * mut c_void);  
      return 1;
  }

  /* We failed to change to the appropriate directory name.  If we tried
     what the user passed (nolinks != 0), punt now. */
  if nolinks !=0 {
      libc::free (tdir as * mut c_void);
      return 0;
  }

  err = errno!();

  /* We're not in physical mode (nolinks == 0), but we failed to change to
     the canonicalized directory name (TDIR).  Try what the user passed
     verbatim. If we succeed, reinitialize the_current_working_directory.
     POSIX requires that we just fail here, so we do in posix mode. */
  if posixly_correct == 0 && libc::chdir (newdir) == 0  {
      t = r_resetpwd (CString::new("cd").unwrap().as_ptr() as * mut c_char);
      if t == std::ptr::null_mut() {
        set_working_directory (tdir);
      } else {
        libc::free (t as * mut c_void);  
      }
      r = 1;
  } else {
    errno!()= err;
    r = 0;
  }

  libc::free (tdir as * mut c_void);
  return r;
  }
}

