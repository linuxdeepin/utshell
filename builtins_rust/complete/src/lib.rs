extern crate  libc;
extern crate nix;

use libc::{c_char, c_int, c_ulong, c_void};
use std::{ffi::CString, ffi::CStr};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, r_savestring} ;
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
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest:c_int,
    filename:* mut WordDesc
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,
  redirector:REDIRECTEE,
  rflags:c_int,
  flags:c_int,
  instruction:r_instruction,
  redirectee:REDIRECTEE,
  here_doc_eof:*mut c_char
}

/* FOR command. */
#[repr(C)]
pub struct for_com {
    flags:c_int,
    line:c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:c_int
}

#[repr(C)]
pub struct case_com {
    flags:c_int,
    line:c_int,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
}

#[repr(C)]
pub struct while_com {
    flags:c_int,
    test:*mut COMMAND,
    action:*mut COMMAND
}

#[repr(C)]
pub struct if_com {
    flags:c_int,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}

#[repr(C)]
pub struct connection {
    ignore:c_int,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector:c_int
}

#[repr(C)]
pub struct simple_com {
    flags:c_int,
    line:c_int,
    words:*mut WordList,
    redirects:*mut REDIRECT
}

#[repr(C)]
pub struct function_def {
    flags:c_int,
    line:c_int,
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
    flags:c_int,
    line:c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct arith_com {
    flags:c_int,
    line:c_int,
    exp:*mut WordList
}

#[repr(C)]
pub struct cond_com {
    flags:c_int,
    line:c_int,
    type_c:c_int,
    exp:*mut WordList
}

#[repr(C)]
pub struct arith_for_com {
    flags:c_int,
    line:c_int,
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

/* Structure containing all the non-action (binary) options; filled in by
   build_actions(). */
#[repr(C)]
#[derive(Copy,Clone)]
pub struct _optflags {
    pflag:c_int,
    rflag:c_int,
    Dflag:c_int,
    Eflag:c_int,
    Iflag:c_int
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct _compacts {
  actname:* const c_char,
  actflag:libc::c_ulong,
  actopt:c_int,
}

pub struct CompactsArray {
  compactsArr:[_compacts;25usize]
}

impl CompactsArray {
    pub fn new()->CompactsArray {
      CompactsArray{
        compactsArr:[
          _compacts{ actname:b"alias\0".as_ptr() as *const c_char, actflag: CA_ALIAS!(),actopt: 'a' as c_int },
          _compacts{ actname:b"arrayvar\0".as_ptr() as *const c_char, actflag: CA_ARRAYVAR!(),actopt: 0 as c_int },
          _compacts{ actname:b"binding\0".as_ptr() as *const c_char, actflag: CA_BINDING!(),actopt: 0 as c_int },
          _compacts{ actname:b"builtin\0".as_ptr() as *const c_char, actflag: CA_BUILTIN!(),actopt: 'b' as c_int },
          _compacts{ actname:b"command\0".as_ptr() as *const c_char, actflag: CA_COMMAND!(),actopt: 'c' as c_int },
          _compacts{ actname:b"directory\0".as_ptr() as *const c_char, actflag: CA_DIRECTORY!(),actopt: 'd' as c_int },
          _compacts{ actname:b"disabled\0".as_ptr() as *const c_char, actflag: CA_DISABLED!(),actopt: 0 as c_int },
          _compacts{ actname:b"enabled\0".as_ptr() as *const c_char, actflag: CA_ENABLED!(),actopt: 0 as c_int },
          _compacts{ actname:b"export\0".as_ptr() as *const c_char, actflag: CA_EXPORT!(),actopt: 'e' as c_int },
          _compacts{ actname:b"file\0".as_ptr() as *const c_char, actflag: CA_FILE!(),actopt: 'f' as c_int },
          _compacts{ actname:b"function\0".as_ptr() as *const c_char, actflag: CA_FUNCTION!(),actopt: 0 as c_int },
          _compacts{ actname:b"helptopic\0".as_ptr() as *const c_char, actflag: CA_HELPTOPIC!(),actopt: 0 as c_int },
          _compacts{ actname:b"hostname\0".as_ptr() as *const c_char, actflag: CA_HOSTNAME!(),actopt: 0 as c_int },
          _compacts{ actname:b"group\0".as_ptr() as *const c_char, actflag: CA_GROUP!(),actopt: 'g' as c_int },
          _compacts{ actname:b"job\0".as_ptr() as *const c_char, actflag: CA_JOB!(),actopt: 'j' as c_int },
          _compacts{ actname:b"keyword\0".as_ptr() as *const c_char, actflag: CA_KEYWORD!(),actopt: 'k' as c_int },
          _compacts{ actname:b"running\0".as_ptr() as *const c_char, actflag: CA_RUNNING!(),actopt: 0 as c_int },
          _compacts{ actname:b"service\0".as_ptr() as *const c_char, actflag: CA_SERVICE!(),actopt: 's' as c_int },
          _compacts{ actname:b"setopt\0".as_ptr() as *const c_char, actflag: CA_SETOPT!(),actopt: 0 as c_int },
          _compacts{ actname:b"shopt\0".as_ptr() as *const c_char, actflag: CA_SHOPT!(),actopt: 0 as c_int },
          _compacts{ actname:b"signal\0".as_ptr() as *const c_char, actflag: CA_SIGNAL!(),actopt: 0 as c_int },
          _compacts{ actname:b"stopped\0".as_ptr() as *const c_char, actflag: CA_STOPPED!(),actopt: 0 as c_int },
          _compacts{ actname:b"user\0".as_ptr() as *const c_char, actflag: CA_USER!(),actopt: 'u' as c_int },
          _compacts{ actname:b"variable\0".as_ptr() as *const c_char, actflag: CA_VARIABLE!(),actopt: 'v' as c_int },
          _compacts{ actname:std::ptr::null_mut(), actflag: 0,actopt: 0 as c_int },  
        ]
      }
    }
}

#[repr(C)]
#[derive(Copy,Clone)]
pub struct _compopt {
  optname:* const c_char,
  optflag:libc::c_ulong,
}

pub struct CompoptArray {
  compoptArr:[_compopt;9usize]
}

impl CompoptArray {
  pub fn new()->CompoptArray{
    CompoptArray{
      compoptArr:[
      _compopt{ optname:"bashdefault\0".as_ptr() as *const c_char, optflag:COPT_BASHDEFAULT!() },
      _compopt{ optname:"default\0".as_ptr() as *const c_char,	optflag:COPT_DEFAULT!() },
      _compopt{ optname:"dirnames\0".as_ptr() as *const c_char, optflag:COPT_DIRNAMES!() },
      _compopt{ optname:"filenames\0".as_ptr() as *const c_char,optflag:COPT_FILENAMES!()},
      _compopt{ optname:"noquote\0".as_ptr() as *const c_char, optflag:COPT_NOQUOTE!() },
      _compopt{ optname:"nosort\0".as_ptr() as *const c_char, optflag:COPT_NOSORT!() },
      _compopt{ optname:"nospace\0".as_ptr() as *const c_char,	optflag:COPT_NOSPACE!() },
      _compopt{ optname:"plusdirs\0".as_ptr() as *const c_char, optflag:COPT_PLUSDIRS!() },
      _compopt{ optname:std::ptr::null_mut(), optflag:0 },
      ]
    }
  }
}

#[repr(C)]
pub struct COMPSPEC {
  refcount:c_int,
  actions:c_ulong,
  options:c_ulong,
  globpat:* mut c_char,
  words:* mut c_char,
  prefix: * mut c_char,
  suffix: * mut c_char,
  funcname: * mut c_char,
  command:* mut c_char,
  lcommand:* mut c_char,
  filterpat:* mut c_char,
}
#[repr(C)]
pub struct BUCKET_CONTENTS {
  next:* mut BUCKET_CONTENTS,	/* Link to next hashed key in this bucket. */
  key:* mut c_char,			/* What we look up. */
  data:* mut libc::c_void,	/* What we really want. */
  khash:libc::c_uint,	/* What key hashes to */
  times_found:i32,		/* Number of times this item has been found. */
}

#[repr(C)]
pub struct STRINGLIST {
  list:* mut * mut c_char,
  list_size:c_int,
  list_len:c_int,
}


#[macro_export]
macro_rules! CA_ALIAS {
  () => {
    1<<0
  }
}

#[macro_export]
macro_rules! CA_ARRAYVAR {
  () => {
    1<<1
  }
}

#[macro_export]
macro_rules! CA_BINDING {
  () => {
    1<<2
  }
}

#[macro_export]
macro_rules! CA_BUILTIN {
  () => {
    1<<3
  }
}

#[macro_export]
macro_rules! CA_COMMAND {
  () => {
    1<<4
  }
}

#[macro_export]
macro_rules! CA_DIRECTORY {
  () => {
    1<<5
  }
}

#[macro_export]
macro_rules! CA_DISABLED {
  () => {
    1<<6
  }
}

#[macro_export]
macro_rules! CA_ENABLED {
  () => {
    1<<7
  }
}

#[macro_export]
macro_rules! CA_EXPORT {
  () => {
    1<<8
  }
}

#[macro_export]
macro_rules! CA_FILE {
  () => {
    1<<9
  }
}

#[macro_export]
macro_rules! CA_FUNCTION {
  () => {
    1<<10
  }
}

#[macro_export]
macro_rules! CA_GROUP {
  () => {
    1<<11
  }
}

#[macro_export]
macro_rules! CA_HELPTOPIC {
  () => {
    1<<12
  }
}

#[macro_export]
macro_rules! CA_HOSTNAME {
  () => {
    1<<13
  }
}

#[macro_export]
macro_rules! CA_JOB {
  () => {
    1<<14
  }
}

#[macro_export]
macro_rules! CA_KEYWORD {
  () => {
    1<<15
  }
}

#[macro_export]
macro_rules! CA_RUNNING {
  () => {
    1<<16
  }
}

#[macro_export]
macro_rules! CA_SERVICE {
  () => {
    1<<17
  }
}

#[macro_export]
macro_rules! CA_SETOPT {
  () => {
    1<<18
  }
}

#[macro_export]
macro_rules! CA_SHOPT {
  () => {
    1<<19
  }
}

#[macro_export]
macro_rules! CA_SIGNAL {
  () => {
    1<<20
  }
}

#[macro_export]
macro_rules! CA_STOPPED {
  () => {
    1<<21
  }
}

#[macro_export]
macro_rules! CA_USER {
  () => {
    1<<22
  }
}

#[macro_export]
macro_rules! CA_VARIABLE {
  () => {
    1<<23
  }
}

#[macro_export]
macro_rules! COPT_RESERVED {
  () => {
    1<<0
  }
}

#[macro_export]
macro_rules! COPT_DEFAULT {
  () => {
    1<<1
  }
}

#[macro_export]
macro_rules! COPT_FILENAMES {
  () => {
    1<<2
  }
}

#[macro_export]
macro_rules! COPT_DIRNAMES {
  () => {
    1<<3
  }
}

#[macro_export]
macro_rules! COPT_NOQUOTE {
  () => {
    1<<4
  }
}

#[macro_export]
macro_rules! COPT_NOSPACE {
  () => {
    1<<5
  }
}

#[macro_export]
macro_rules! COPT_BASHDEFAULT {
  () => {
    1<<6
  }
}

#[macro_export]
macro_rules! COPT_PLUSDIRS {
  () => {
    1<<7
  }
}

#[macro_export]
macro_rules! COPT_NOSORT {
  () => {
    1<<8
  }
}

#[macro_export]
macro_rules! RL_STATE_COMPLETING {
  () => {
    0x0004000	/* doing completion */
  }
}

extern "C" {
  fn reset_internal_getopt();
  fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
  fn sh_invalidopt (value:* mut c_char);
  fn sh_invalidid (value:* mut c_char);
  fn sh_invalidoptname (value:* mut c_char);
  fn builtin_usage();
  static list_optarg:* mut c_char;
  fn builtin_error(err:*const c_char,...);
  fn check_identifier (w:* mut WordDesc, f:i32)->i32;
  static mut posixly_correct:i32;
  static mut loptend:*mut WordList;
  fn make_word_list (w:* mut WordDesc, list:*mut WordList)->*mut WordList;
  fn make_bare_word (w:*const c_char)->* mut WordDesc;
  fn dispose_words (list:*mut WordList);
  fn progcomp_flush ();
  fn compspec_create ()->* mut COMPSPEC;
  fn progcomp_insert (str:* mut c_char, c:* mut COMPSPEC)->i32;
  fn progcomp_remove (str:* mut c_char)->i32;
  fn sh_single_quote (str:* mut c_char)->* mut c_char;
  fn progcomp_walk (func: unsafe extern "C" fn (item:* mut BUCKET_CONTENTS)->i32);
  fn sh_chkwrite (i:i32)->i32;
  fn progcomp_search (w:*const c_char)->* mut COMPSPEC;
  static mut pcomp_line:* mut c_char;
  static mut pcomp_ind:c_int;
  fn gen_compspec_completions (cs:* mut COMPSPEC, cmd:*const c_char, word:*const c_char, start:i32, end:i32, foundp:* mut i32)->* mut STRINGLIST;
  fn bash_default_completion (text:* const c_char, start:i32, end:i32, qc:i32, compflags:i32)->* mut * mut c_char;
  fn rl_filename_completion_function (text:* const c_char, state:i32)-> * mut c_char;
  fn rl_completion_matches (text:* const c_char, entry_function:unsafe extern "C" fn (text:* const c_char, state:i32)-> * mut c_char)->* mut * mut c_char;
  fn completions_to_stringlist (matches:* mut * mut c_char)->* mut STRINGLIST;
  fn strvec_dispose (matches:* mut * mut c_char);
  fn strlist_dispose (strlist:* mut STRINGLIST);
  fn strlist_print (strlist:* mut STRINGLIST, text:* mut c_char);
  fn compspec_dispose (com:* mut COMPSPEC);
  static mut list_opttype:i32;
  static mut rl_readline_state:c_ulong;
  static mut pcomp_curcs:* mut COMPSPEC;
  static pcomp_curcmd:* mut c_char;
  fn pcomp_set_compspec_options (cs:* mut COMPSPEC, flags:i32, set_or_unset:i32);
  fn pcomp_set_readline_variables (flags:i32, nval:i32);
}

pub static mut Garg:* mut c_char=std::ptr::null_mut();
pub static mut Warg:* mut c_char=std::ptr::null_mut();
pub static mut Parg:* mut c_char=std::ptr::null_mut();
pub static mut Sarg:* mut c_char=std::ptr::null_mut();
pub static mut Xarg:* mut c_char=std::ptr::null_mut();
pub static mut Farg:* mut c_char=std::ptr::null_mut();
pub static mut Carg:* mut c_char=std::ptr::null_mut();

unsafe fn STRDUP(x:* const c_char)->* mut c_char
{
  if x !=std::ptr::null_mut() {
      return r_savestring (x);
  } else {
      return std::ptr::null_mut();
  }
}

unsafe fn STREQ( a:* const c_char, b:* const c_char)->bool
{
	return *a ==*b  && libc::strcmp(a, b) == 0;
}

unsafe fn shell_break_chars()->* const c_char
{
  return b"()<>;&| \t\n\0".as_ptr() as *const c_char;
}

unsafe fn EMPTYCMD()->* const c_char
{
  return b"_EmptycmD_\0".as_ptr() as *const c_char;
}

unsafe fn DEFAULTCMD()->* const c_char
{
  return b"_DefaultCmD_\0".as_ptr() as *const c_char;
}

unsafe fn INITIALWORD()->* const c_char
{
  return b"_InitialWorD_\0".as_ptr() as *const c_char;
}

unsafe fn RL_ISSTATE(x:c_ulong)->c_ulong
{
  return rl_readline_state & x;
}

#[no_mangle]
pub extern "C" fn r_find_compact (name:* mut c_char)->i32
{
  let mut i:i32=0;
  unsafe {
    let compacts:CompactsArray=CompactsArray::new();
    while compacts.compactsArr[i as usize].actname != std::ptr::null_mut() {
      let tmp = CStr::from_ptr(compacts.compactsArr[i as usize].actname);
      if STREQ (name, compacts.compactsArr[i as usize].actname) {
        return i;
      }
      i+=1;
    }
    return -1;
  }
}

#[no_mangle]
pub extern "C" fn r_find_compopt (name:* mut c_char)->i32
{
  let mut i:i32=0;
  let compopts:CompoptArray=CompoptArray::new();
  unsafe {
    while compopts.compoptArr[i as usize].optname != std::ptr::null_mut() {
      if STREQ (name, compopts.compoptArr[i as usize].optname) {
        return i;
      }
      i+=1;
    }
    return -1;
  }
}

#[no_mangle]
pub extern "C" fn r_build_actions (mut list : *mut WordList, flagp:* mut _optflags, actp:* mut c_ulong, optp:* mut c_ulong)->i32
{
  let mut opt:i32;
  let mut ind:i32;
  let mut opt_given:i32=0;
  let mut acts:c_ulong=0;
  let mut copts:c_ulong=0;
  let mut w:WordDesc=WordDesc{word:std::ptr::null_mut(),flags:0};

  unsafe {
    reset_internal_getopt ();
    opt = internal_getopt(list, CString::new("abcdefgjko:prsuvA:G:W:P:S:X:F:C:DEI").unwrap().as_ptr() as * mut c_char);
    while opt != -1 {
        opt_given = 1;
        let optu8:u8= opt as u8;
        let optChar:char=char::from(optu8);
        match optChar{
            'r'=>{
              if flagp !=std::ptr::null_mut() {
                (*flagp).rflag = 1;
              } else {
                sh_invalidopt (CString::new("-r").unwrap().as_ptr() as * mut c_char);
                builtin_usage ();
                return EX_USAGE;
              }
            }
            'p'=>{
              if flagp !=std::ptr::null_mut() {
                (*flagp).pflag = 1;
              } else {
                sh_invalidopt (CString::new("-p").unwrap().as_ptr() as * mut c_char);
                builtin_usage ();
                return EX_USAGE;
              }
            }
            'a'=>{
              acts |= CA_ALIAS!();
            }
            'b'=>{
              acts |= CA_BUILTIN!();
            }
            'c'=>{
              acts |= CA_COMMAND!();
            }
            'd'=>{
              acts |= CA_DIRECTORY!();
            }
            'e'=>{
              acts |= CA_EXPORT!();
            }
            'f'=>{
              acts |= CA_FILE!();
            }
            'g'=>{
              acts |= CA_GROUP!();
            }
            'j'=>{
              acts |= CA_GROUP!();
            }
            'k'=>{
              acts |= CA_KEYWORD!();
            }
            's'=>{
              acts |= CA_SERVICE!();
            }
            'u'=>{
              acts |= CA_USER!();
            }
            'v'=>{
              acts |= CA_VARIABLE!();
            }
            'o'=>{
              ind = r_find_compopt (list_optarg);
              if ind < 0 {
                  sh_invalidoptname (list_optarg);
                  return EX_USAGE;
              }
              let compopts:CompoptArray=CompoptArray::new();
              copts |= compopts.compoptArr[ind as usize].optflag;
            }
            'A'=>{
              ind = r_find_compact (list_optarg);
              if ind < 0 {
                 builtin_error (CString::new("%s: invalid action name").unwrap().as_ptr(), list_optarg);
                 return EX_USAGE;
              }
              let compacts:CompactsArray=CompactsArray::new();
              acts |= compacts.compactsArr[ind as usize].actflag;
            }
           'C'=>{
              Carg = list_optarg;
           }
           'D'=>{
              if flagp !=std::ptr::null_mut() {
                (*flagp).Dflag = 1;
              } else {
                sh_invalidopt (CString::new("-D").unwrap().as_ptr() as * mut c_char);
                builtin_usage ();
                return EX_USAGE;
              }
           }
           'E'=>{
            if flagp !=std::ptr::null_mut() {
              (*flagp).Eflag = 1;
            } else {
              sh_invalidopt (CString::new("-E").unwrap().as_ptr() as * mut c_char);
              builtin_usage ();
              return EX_USAGE;
            }
           }
           'I'=>{
            if flagp !=std::ptr::null_mut() {
              (*flagp).Iflag = 1;
            } else {
              sh_invalidopt (CString::new("-I").unwrap().as_ptr() as * mut c_char);
              builtin_usage ();
              return EX_USAGE;
            }
           }
           'F'=>{
              w.word = list_optarg;
              Farg  = list_optarg;
              w.flags = 0;
              if check_identifier (&mut w, posixly_correct) == 0 || libc::strpbrk (Farg, shell_break_chars()) != std::ptr::null_mut() {
                  sh_invalidid (Farg);
                  return EX_USAGE;
              }
           }
           'G'=>{
            Garg = list_optarg;
           }
           'P'=>{
            Parg = list_optarg;
           }
           'S'=>{
            Sarg = list_optarg;
           }
           'W'=>{
            Warg = list_optarg;
           }
           'X'=>{
            Xarg = list_optarg;
           }
           _=>{
            if opt == -99 {
              r_builtin_help();
              return EX_USAGE;
          }
            builtin_usage ();
            return EX_USAGE;
          }
        }
        opt=internal_getopt(list, CString::new("abcdefgjko:prsuvA:G:W:P:S:X:F:C:DEI").unwrap().as_ptr() as * mut c_char);
      }
      *actp = acts;
      *optp = copts;
    list = loptend.clone();
      if opt_given !=0 {
        return EXECUTION_SUCCESS!();
      } else {
        return EXECUTION_FAILURE!();
      }
  }
}

/* Add, remove, and display completion specifiers. */
#[no_mangle]
pub extern "C" fn r_complete_builtin (listt: *mut WordList)->i32
{
  let mut opt_given:i32=0;
  let mut rval:i32;
  let mut acts:c_ulong=0;
  let mut copts:c_ulong=0;
  let mut cs:* mut COMPSPEC;
  let mut oflags:_optflags=_optflags{pflag:0,rflag:0,Dflag:0,Eflag:0,Iflag:0};
  let mut l: *mut WordList;
  let mut wl: *mut WordList;

  unsafe {
    let mut list:* mut WordList=listt.clone();
    if list == std::ptr::null_mut() {
        r_print_all_completions ();
        return EXECUTION_SUCCESS!();
    }

    oflags.pflag=0;
    oflags.rflag=0;
    oflags.Dflag=0;
    oflags.Eflag=0;
    oflags.Iflag=0;

    Garg=std::ptr::null_mut();
    Warg=std::ptr::null_mut();
    Parg=std::ptr::null_mut();
    Sarg=std::ptr::null_mut();
    Xarg=std::ptr::null_mut();
    Farg=std::ptr::null_mut();
    Carg=std::ptr::null_mut();

    cs=std::ptr::null_mut();

    /* Build the actions from the arguments.  Also sets the [A-Z]arg variables
      as a side effect if they are supplied as options. */
    rval = r_build_actions (list, &mut oflags, &mut acts, &mut copts);
    if rval == EX_USAGE {
      return rval;
    }

    opt_given = (rval != EXECUTION_FAILURE!()) as i32;

    list = loptend.clone();

    if oflags.Dflag !=0 {
        wl = make_word_list (make_bare_word (DEFAULTCMD()), std::ptr::null_mut());
    } else if oflags.Eflag !=0 {
        wl = make_word_list (make_bare_word (EMPTYCMD()), std::ptr::null_mut());
    } else if oflags.Iflag !=0 {
        wl = make_word_list (make_bare_word (INITIALWORD()), std::ptr::null_mut());
    } else {
        wl = std::ptr::null_mut();
    }

    /* -p overrides everything else */
    if oflags.pflag !=0 || (list == std::ptr::null_mut() && opt_given == 0) {
        if wl != std::ptr::null_mut() {
          rval = r_print_cmd_completions (wl);
          dispose_words (wl);
          return rval;
        } else if list == std::ptr::null_mut() {
            //给了P,但没给参数，直接打印全部并退出
          r_print_all_completions ();
          return EXECUTION_SUCCESS!();
        }
        return r_print_cmd_completions (list);
    }

    /* next, -r overrides everything else. */
    if oflags.rflag !=0 {
        if wl != std::ptr::null_mut() {
            rval = r_remove_cmd_completions (wl);
            dispose_words (wl);
            return rval;
        } else if list == std::ptr::null_mut() {
            progcomp_flush ();
            return EXECUTION_SUCCESS!();
        }
        return r_remove_cmd_completions (list);
    }

    if wl == std::ptr::null_mut() && list == std::ptr::null_mut() && opt_given !=0 {
        builtin_usage ();
        return EX_USAGE;
    }

    /* If we get here, we need to build a compspec and add it for each
      remaining argument. */
    cs = compspec_create ();
    (*cs).actions = acts;
    (*cs).options = copts;

    (*cs).globpat = STRDUP (Garg);
    (*cs).words = STRDUP (Warg);
    (*cs).prefix = STRDUP (Parg);
    (*cs).suffix = STRDUP (Sarg);
    (*cs).funcname = STRDUP (Farg);
    (*cs).command = STRDUP (Carg);
    (*cs).filterpat = STRDUP (Xarg);

    rval = EXECUTION_SUCCESS!();

    if wl != std::ptr::null_mut() {
      l= wl.clone();
    } else {
      l= list.clone();
    }

    while l != std::ptr::null_mut() {
      /* Add CS as the compspec for the specified commands. */
      if progcomp_insert ((*(*l).word).word, cs) == 0 {
          rval = EXECUTION_FAILURE!();
      }
      l = (*l).next;
    }

    dispose_words (wl);
    return rval;
  }
}

#[no_mangle]
pub extern "C" fn r_remove_cmd_completions (list: * mut WordList)->i32
{
  let mut l:* mut WordList;
  let mut ret:i32;
  unsafe {
    ret = EXECUTION_SUCCESS!();
    l = list.clone();
    while l!=std::ptr::null_mut() {
      if progcomp_remove ((*(*l).word).word) == 0	{
          builtin_error (CString::new("%s: no completion specification").unwrap().as_ptr(), (*(*l).word).word);
          ret = EXECUTION_FAILURE!();
      }
      l = (*l).next;
    }
    return ret;
  }
}

#[no_mangle]
pub extern "C" fn r_print_compoptions (copts:c_ulong, full:i32)
{
  unsafe {
    let compopts:CompoptArray=CompoptArray::new();
    for i in 0..compopts.compoptArr.len() {
      if (copts & compopts.compoptArr[i].optflag) !=0 {
        libc::printf (CString::new("-o %s ").unwrap().as_ptr(), compopts.compoptArr[i].optname);
      } else if full !=0 {
        libc::printf (CString::new("+o %s ").unwrap().as_ptr(), compopts.compoptArr[i].optname);
      }
    }
  }
}

#[no_mangle]
pub extern "C" fn r_print_compactions (acts:c_ulong)
{
  unsafe {
    let compacts:CompactsArray=CompactsArray::new();
    for i in 0..compacts.compactsArr.len() {
      if compacts.compactsArr[i].actopt !=0 && (acts & compacts.compactsArr[i].actflag) !=0 {
        libc::printf (CString::new("-%c ").unwrap().as_ptr(), compacts.compactsArr[i].actopt);
      }
    }

    for i in 0..compacts.compactsArr.len() {
      if compacts.compactsArr[i].actopt ==0 && (acts & compacts.compactsArr[i].actflag) !=0 {
        libc::printf (CString::new("-A %s ").unwrap().as_ptr(), compacts.compactsArr[i].actname);
      }
    }
  }
}

#[no_mangle]
pub extern "C" fn r_print_arg (arg:* const c_char, flag:* const c_char, quote:i32)
{
  let x:* mut c_char;
  unsafe {
    if arg != std::ptr::null_mut() {
        if quote !=0 {
            // 复制arg 增加单引号返给x
            x = sh_single_quote (arg as * mut c_char);
        } else {
            x= arg as * mut c_char;
        }
        libc::printf (CString::new("%s %s ").unwrap().as_ptr(), flag, x);
        if x != arg as * mut c_char {
          libc::free (x as * mut c_void);
        }
    }
  }
}

#[no_mangle]
pub extern "C" fn r_print_cmd_name (cmd:* const c_char)
{
  unsafe {
    if STREQ (cmd, DEFAULTCMD()) {
      libc::printf (CString::new("-D").unwrap().as_ptr());
    } else if STREQ (cmd, EMPTYCMD()) {
      libc::printf (CString::new("-E").unwrap().as_ptr());
    } else if STREQ (cmd, INITIALWORD()) {
      libc::printf (CString::new("-I").unwrap().as_ptr());
    } else if *cmd == 0	{ /* XXX - can this happen? */
      libc::printf (CString::new("''").unwrap().as_ptr());
    }	else {
      libc::printf (CString::new("%s").unwrap().as_ptr(),cmd);
    }
  }
}

#[no_mangle]
pub extern "C" fn r_print_one_completion (cmd: * mut c_char, cs:* mut COMPSPEC)->i32
{
  unsafe {
    libc::printf (CString::new("complete ").unwrap().as_ptr());

    r_print_compoptions ((*cs).options, 0);
    r_print_compactions ((*cs).actions);

    /* now the rest of the arguments */

    /* arguments that require quoting */
    r_print_arg ((*cs).globpat, CString::new("-G").unwrap().as_ptr(), 1);
    r_print_arg ((*cs).words, CString::new("-W").unwrap().as_ptr(), 1);
    r_print_arg ((*cs).prefix, CString::new("-P").unwrap().as_ptr(), 1);
    r_print_arg ((*cs).suffix, CString::new("-S").unwrap().as_ptr(), 1);
    r_print_arg ((*cs).filterpat, CString::new("-X").unwrap().as_ptr(), 1);

    r_print_arg ((*cs).command, CString::new("-C").unwrap().as_ptr(), 1);

    /* simple arguments that don't require quoting */
    r_print_arg ((*cs).funcname, CString::new("-F").unwrap().as_ptr(), 0);

    r_print_cmd_name (cmd);
    libc::printf (CString::new("\n").unwrap().as_ptr());

    return 0;
  }
}

#[no_mangle]
pub extern "C" fn r_print_compopts (cmd:* mut c_char, cs:* mut COMPSPEC, full:i32)
{
  unsafe {
    libc::printf (CString::new("compopt ").unwrap().as_ptr());

    r_print_compoptions ((*cs).options, full);
    r_print_cmd_name (cmd);

    libc::printf (CString::new("\n").unwrap().as_ptr());
  }
}

#[no_mangle]
pub extern "C" fn r_print_compitem (item:* mut BUCKET_CONTENTS)->i32
{
  let cs:* mut COMPSPEC;
  let cmd:* mut c_char;
  unsafe {
    cmd = (*item).key;
    cs = (*item).data as * mut COMPSPEC;
  }

  return r_print_one_completion (cmd, cs);
}

#[no_mangle]
pub extern "C" fn r_print_all_completions ()
{
  unsafe {
    progcomp_walk (r_print_compitem);
  }
}

#[no_mangle]
pub extern "C" fn r_print_cmd_completions (list:* mut WordList)->i32
{
  let mut l:* mut WordList;
  let mut cs:* mut COMPSPEC;
  let mut ret:i32;

  unsafe {
      ret = EXECUTION_SUCCESS!();
      l = list.clone();
      while l != std::ptr::null_mut() {
        cs = progcomp_search ((*(*l).word).word);
        if cs != std::ptr::null_mut() {
            r_print_one_completion ((*(*l).word).word, cs);
        } else {
            builtin_error (CString::new("%s: no completion specification").unwrap().as_ptr(),(*(*l).word).word);
            ret = EXECUTION_FAILURE!();
        }
        l = (*l).next;
      }
      return sh_chkwrite (ret);
  }
}

#[no_mangle]
pub extern "C" fn r_compgen_builtin (listt:* mut WordList)->i32
{
  let mut rval:i32;
  let mut acts:c_ulong=0;
  let mut copts:c_ulong=0;
  let mut cs: * mut COMPSPEC;
  let mut sl:* mut STRINGLIST;
  let word:* mut c_char;
  let mut matches:* mut * mut c_char;
  let old_line:* mut c_char;
  let old_ind:i32;
  unsafe {
    let mut list:* mut WordList=listt.clone();
    if list == std::ptr::null_mut() {
      return EXECUTION_SUCCESS!();
    }

    Garg=std::ptr::null_mut();
    Warg=std::ptr::null_mut();
    Parg=std::ptr::null_mut();
    Sarg=std::ptr::null_mut();
    Xarg=std::ptr::null_mut();
    Farg=std::ptr::null_mut();
    Carg=std::ptr::null_mut();

    cs = std::ptr::null_mut();

    /* Build the actions from the arguments.  Also sets the [A-Z]arg variables
      as a side effect if they are supplied as options. */
    rval = r_build_actions (list, std::ptr::null_mut(), &mut acts, &mut copts);
    if rval == EX_USAGE {
        return rval;
    }

    if rval == EXECUTION_FAILURE!() {
        return EXECUTION_SUCCESS!();
    }

    list = loptend.clone();

    let wordtmp=CString::new("").unwrap();
    if list !=std::ptr::null_mut() && (*list).word != std::ptr::null_mut() {
        word = (*((*list).word)).word; 
    } else {
        word = wordtmp.as_ptr() as * mut c_char;
    }

    if Farg != std::ptr::null_mut() {
      builtin_error (CString::new("warning: -F option may not work as you expect").unwrap().as_ptr());
    }

    if Carg != std::ptr::null_mut() {
      builtin_error (CString::new("warning: -C option may not work as you expect").unwrap().as_ptr());
    }

    /* If we get here, we need to build a compspec and evaluate it. */
    cs = compspec_create ();
    (*cs).actions = acts;
    (*cs).options = copts;
    (*cs).refcount = 1;

    (*cs).globpat = STRDUP (Garg);
    (*cs).words = STRDUP (Warg);
    (*cs).prefix = STRDUP (Parg);
    (*cs).suffix = STRDUP (Sarg);
    (*cs).funcname = STRDUP (Farg);
    (*cs).command = STRDUP (Carg);
    (*cs).filterpat = STRDUP (Xarg);

    rval = EXECUTION_FAILURE!();

    /* probably don't have to save these, just being safe */
    old_line = pcomp_line;
    old_ind = pcomp_ind;
    pcomp_line = std::ptr::null_mut();
    pcomp_ind = 0;
    let compgenStr=CString::new("compgen").unwrap();
    sl = gen_compspec_completions (cs, compgenStr.as_ptr(), word, 0, 0, std::ptr::null_mut());
    pcomp_line = old_line;
    pcomp_ind = old_ind;

    /* If the compspec wants the bash default completions, temporarily
      turn off programmable completion and call the bash completion code. */
    if (sl == std::ptr::null_mut() || (*sl).list_len == 0) && (copts & COPT_BASHDEFAULT!()) !=0 {
        matches = bash_default_completion (word, 0, 0, 0, 0);
        sl = completions_to_stringlist (matches);
        strvec_dispose (matches);
    }

    /* This isn't perfect, but it's the best we can do, given what readline
      exports from its set of completion utility functions. */
    if (sl == std::ptr::null_mut() || (*sl).list_len == 0) && (copts & COPT_DEFAULT!()) !=0 {
        matches = rl_completion_matches (word, rl_filename_completion_function);
        strlist_dispose (sl);
        sl = completions_to_stringlist (matches);
        strvec_dispose (matches);
      }

    if sl != std::ptr::null_mut() {
        if (*sl).list != std::ptr::null_mut() && (*sl).list_len !=0 {
            rval = EXECUTION_SUCCESS!();
            strlist_print (sl, std::ptr::null_mut());
        }
        strlist_dispose (sl);
      }

    compspec_dispose (cs);
    return rval;
  }
}

#[no_mangle]
pub extern "C" fn r_compopt_builtin (listt:* mut WordList)->i32
{
  let mut opts_on:i32=0;
  let mut opts_off:i32=0;
  let mut opts:* mut i32;
  let mut opt:i32;
  let mut oind:i32;
  let mut ret:i32;
  let mut Dflag:i32=0;
  let mut Eflag:i32=0;
  let mut Iflag:i32=0;
  let mut l:* mut WordList;
  let mut wl:* mut WordList;
  let mut cs:* mut COMPSPEC;

  ret = EXECUTION_SUCCESS!();
  unsafe {
    let mut list:* mut WordList=listt.clone();
    reset_internal_getopt ();

    opt = internal_getopt (list, CString::new("+o:DEI").unwrap().as_ptr() as * mut c_char);

    while opt != -1 {
        if list_opttype == '-' as i32 {
          opts = &mut opts_on;
        } else {
          opts = &mut opts_off;
        }

        let optu8:u8= opt as u8;
        let optChar:char=char::from(optu8);

        match optChar {
           'o'=>{
                oind = r_find_compopt (list_optarg);
                if oind < 0 {
                    sh_invalidoptname (list_optarg);
                    return EX_USAGE;
                }
                let compopts:CompoptArray=CompoptArray::new();
                *opts |= compopts.compoptArr[oind as usize].optflag as i32;
            }
            'D'=>{
              Dflag = 1;
            }
            'E'=>{
              Eflag = 1;
            }
            'I'=>{
              Iflag = 1;
            }
            _=>{
              builtin_usage ();
              return EX_USAGE;
            }
        }
        opt = internal_getopt (list, CString::new("+o:DEI").unwrap().as_ptr() as * mut c_char);
    }

    list = loptend.clone();

    if Dflag != 0 {
      wl = make_word_list (make_bare_word (DEFAULTCMD()), std::ptr::null_mut());
    } else if Eflag !=0 {
      wl = make_word_list (make_bare_word (EMPTYCMD()), std::ptr::null_mut());
    } else if Iflag !=0 {
      wl = make_word_list (make_bare_word (INITIALWORD()), std::ptr::null_mut());
    } else {
      wl = std::ptr::null_mut();
    }

    if list == std::ptr::null_mut() && wl == std::ptr::null_mut() {
        if RL_ISSTATE (RL_STATE_COMPLETING!()) == 0 || pcomp_curcs == std::ptr::null_mut() {
            builtin_error (CString::new("not currently executing completion function").unwrap().as_ptr());
            return EXECUTION_FAILURE!();
        }
        cs = pcomp_curcs.clone();

        if opts_on == 0 && opts_off == 0 {
            r_print_compopts (pcomp_curcmd, cs, 1);
            return sh_chkwrite (ret);
        }

        /* Set the compspec options */
        pcomp_set_compspec_options (cs, opts_on, 1);
        pcomp_set_compspec_options (cs, opts_off, 0);

        /* And change the readline variables the options control */
        pcomp_set_readline_variables (opts_on, 1);
        pcomp_set_readline_variables (opts_off, 0);

        return ret;
      }

      if wl != std::ptr::null_mut() {
          l = wl.clone();
      } else {
          l=list.clone();
      }

      while l != std::ptr::null_mut() {
          cs = progcomp_search ((*((*list).word)).word);
          if cs == std::ptr::null_mut() {
            builtin_error (CString::new("%s: no completion specification").unwrap().as_ptr(), (*((*list).word)).word);
            ret = EXECUTION_FAILURE!();
          l = (*l).next;
            continue;
          }
          if opts_on == 0 && opts_off == 0 {
            r_print_compopts ((*((*list).word)).word, cs, 1);
          l = (*l).next;
            continue;			/* XXX -- fill in later */
          }

          /* Set the compspec options */
          pcomp_set_compspec_options (cs, opts_on, 1);
          pcomp_set_compspec_options (cs, opts_off, 0);
          l = (*l).next;
        }

    if wl != std::ptr::null_mut() {
      dispose_words (wl);
    }

    return ret;
  }
}

