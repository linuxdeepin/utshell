extern crate  libc;
extern crate nix;

use libc::c_void;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage,get_local_str};
use rhelp::r_builtin_help;

use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
use fluent_resmgr::resource_manager::ResourceManager;

#[macro_export]
macro_rules! CDESC_ALL{
    //print all descriptions of a command
   () => {0x001}
}

#[macro_export]
macro_rules! CDESC_SHORTDESC {
    //print the description for type and command -V
   () => {0x002}
}

#[macro_export]
macro_rules! CDESC_REUSABLE{
    //print in a format that may be reused as input
   () => {0x004}
}

#[macro_export]
macro_rules! CDESC_TYPE {
    //print the type for type -t
   () => {0x008}
}

#[macro_export]
macro_rules! CDESC_PATH_ONLY {
    //print the path for type -p
   () => {0x010}
}

#[macro_export]
macro_rules! CDESC_FORCE_PATH {
    //force a path search for type -P
   () => {0x020}
}

#[macro_export]
macro_rules! CDESC_NOFUNCS {
    //skip function lookup for type -f
   () => {0x040}
}

#[macro_export]
macro_rules!  CDESC_ABSPATH{
    //CDESC_ABSPATH
   () => {0x080}
}

#[macro_export]
macro_rules!  CDESC_STDPATH{
   () => {0x100}
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



#[macro_export]
macro_rules!  FS_EXECABLE{
   () => {0x2}
}
#[macro_export]
macro_rules!  FS_EXEC_PREFERRED{
   () => {0x4}
}

#[macro_export]
macro_rules!  FS_NODIRS{
   () => {0x20}
}

#[macro_export]
macro_rules!  MP_DOCWD{
   () => {0}
}

#[macro_export]
macro_rules!  MP_RMDOT{
   () => {1}
}

#[deny(missing_fragment_specifier)]
#[macro_export]
macro_rules!  STREQ{
   ($a:expr,$b:expr) =>{
       *$a as libc::c_char == *$b as libc::c_char && libc::strcmp($a,$b)==0
    }
}

#[macro_export]
macro_rules!  SIZEOFWORD{
    () => {
    std::mem::size_of::<WordDesc>()
    }  
}


#[repr(C)]
pub struct SHELL_VAR {
  name:*mut libc::c_char,
  value:*mut libc::c_char,
  exportstr:*mut libc::c_char,
  dynamic_value:*mut fn(v:* mut SHELL_VAR)->*mut SHELL_VAR,
  assign_func:* mut fn(v:* mut SHELL_VAR,str1:* mut libc::c_char,t:i64,str2:* mut libc::c_char)->*mut SHELL_VAR,
  attributes:i32,
  context:i32
}

#[repr (C)]
#[derive(Copy,Clone)]
pub struct alias {
    name :*mut libc::c_char,
    value :*mut libc::c_char ,
    flags:libc::c_char 
}

type sh_builtin_func_t = fn(WordList) -> i32;
type alias_t = alias;

pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
    /// 通过函数指针调用函数
    op(a, b)
}


#[repr(C)]
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND
}
#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
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
  redirector:REDIRECTEE,
  rflags: i32 ,
  flags: i32 ,
  instruction:r_instruction,
  redirectee:REDIRECTEE,
  here_doc_eof:*mut libc::c_char
}

#[repr(C)]
pub struct for_com {
    flags: i32 ,
    line: i32 ,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
#[derive(Copy,Clone)]
pub union REDIRECTEE {
    dest: i32 ,
    filename:* mut WordDesc
}

#[repr(C)]
pub struct case_com {
    flags: i32,
    line: i32,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
}
#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:i32
}
#[repr(C)]
pub struct while_com {
    flags: i32 ,
    test:*mut COMMAND,
    action:*mut COMMAND
}

#[repr(C)]
pub struct if_com {
    flags: i32,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}

#[repr(C)]
pub struct connection {
    ignore: i32 ,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector: i32 
}

#[repr(C)]
pub struct simple_com {
    flags: i32 ,
    line: i32 ,
    words:*mut WordList,
    redirects:*mut REDIRECT
}

#[repr(C)]
pub struct function_def {
    flags: i32 ,
    line: i32 ,
    name:*mut WordDesc,
    command:*mut COMMAND,
    source_file:*mut libc::c_char
}

#[repr(C)]
pub struct group_com {
    ignore: i32 ,
    command:*mut COMMAND,
    source_file:*mut libc::c_char
}

#[repr(C)]
pub struct select_com {
    flags: i32 ,
    line: i32 ,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct arith_com {
    flags: i32 ,
    line: i32 ,
    exp:*mut WordList
}

#[repr(C)]
pub struct cond_com {
    flags: i32 ,
    line: i32 ,
    type_c: i32 ,
    exp:*mut WordList
}

#[repr(C)]
pub struct arith_for_com {
    flags: i32 ,
    line: i32 ,
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
    name:*mut libc::c_char,
    command:*mut COMMAND
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
macro_rules! FS_EXEC_ONLY {
    () => {
       0x8
    }
}

#[macro_export]
macro_rules! ABSPATH {
    ($s :expr) => {
        unsafe {
            char::from(*($s as *mut libc::c_char) as u8) }== '/';
        
    // $x  == '/';
   }
 }
//define ABSPATH(x)	((x)[0] == '/')

extern "C" {
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList , opts:*mut libc::c_char)->i32;
    fn builtin_usage();
    fn sh_notfound (name:* mut libc::c_char);
    fn sh_chkwrite (ret:i32)->i32;
    fn find_alias(alia :*mut libc::c_char) ->alias_t;
    fn sh_single_quote(quote: *const libc::c_char) -> *mut libc::c_char;
    fn find_reserved_word(word: *mut libc::c_char)->i32;
    fn find_function (name:* const libc::c_char)-> *mut SHELL_VAR;
    fn named_function_string (name: *mut libc::c_char, cmd:* mut COMMAND, i:i32)->* mut libc::c_char;
    fn find_shell_builtin(builtin: *mut libc::c_char) -> *mut libc::c_char;
    fn find_special_builtin(builtins: *mut libc::c_char) -> *mut sh_builtin_func_t;
    fn absolute_program(program:*const libc::c_char) -> i32;
    fn file_status(status :*const libc::c_char) -> i32 ;
    fn phash_search(search:*const libc::c_char) -> *mut libc::c_char;
    fn conf_standard_path() -> *mut libc::c_char;
    fn find_in_path(path1:*const libc::c_char, path2:*mut libc::c_char, num: i32) -> *mut libc::c_char;
    fn find_user_command(cmd:*mut libc::c_char) -> *mut libc::c_char;
    fn user_command_matches(cmd:*const libc::c_char, num1:i32, num2:i32) -> *mut libc::c_char;
    fn sh_makepath(path:*const libc::c_char, path1:*const libc::c_char, i: i32) -> *mut libc::c_char;
    //fn find_alias(alia : *mut libc::c_char) -> *mut alias_t;
    static  expand_aliases : i32;
    static mut loptend:*mut WordList;
    static posixly_correct:i32;
}

unsafe fn function_cell(var:*mut SHELL_VAR) ->* mut COMMAND {
	return (*var).value as * mut COMMAND;
}

#[no_mangle]
pub unsafe extern "C" fn r_type_builtin (mut list :*mut WordList) -> i32 {
    //println!("rtype  is run");
    let  mut dflags : i32;
    let mut any_failed: i32 = 0 ;
    let  mut opt : i32  = 0;
    let mut this : *mut WordList;

    dflags = CDESC_SHORTDESC!();	/* default */
    unsafe{
    this = list;  
    while this != std::ptr::null_mut() && char::from((*(*(*this).word).word) as u8) == '-' {
         let mut flag  = (((*(*this).word).word) as usize + 1) as *mut libc::c_char;
         let mut c_str_type = CString::new("type").unwrap();
         let c_str_type1 = CString::new("-type").unwrap();
         let c_str_path = CString::new("path").unwrap();
         let c_str_path1 = CString::new("-path").unwrap();
         let c_str_all = CString::new("all").unwrap();
         let c_str_all1 = CString::new("-all").unwrap();
         if STREQ!(flag, c_str_type.as_ptr() as *mut libc::c_char ) || STREQ!(flag, c_str_type1.as_ptr() as *mut libc::c_char) {
           unsafe {
            *((*(*this).word).word).offset(1) = 't' as libc::c_char ;
            *((*(*this).word).word).offset(2) = '\0' as libc::c_char ;
            } 
        }
        else if STREQ!(flag, c_str_path.as_ptr() as *mut libc::c_char) || STREQ!(flag, c_str_path1.as_ptr() as *mut libc::c_char){
            *((*(*this).word).word).offset(1) = 'p' as libc::c_char ;
            *((*(*this).word).word).offset(2) = '\0' as libc::c_char ;
	     }
       
         else if STREQ!(flag, c_str_all.as_ptr() as *mut libc::c_char) || STREQ!(flag, c_str_all1.as_ptr() as *mut libc::c_char) {
            *((*(*this).word).word).offset(1) = 'a' as libc::c_char ;
            *((*(*this).word).word).offset(2) = '\0' as libc::c_char ;
        }

       if (*this).next != std::ptr::null_mut(){
        this = (*this).next; 
       }
       else  {
           break;
       }
    } 
}
    reset_internal_getopt();

   let c_str_afptP = CString::new("afptP").unwrap();
   let mut opt = unsafe {internal_getopt(list,c_str_afptP.as_ptr() as *mut libc::c_char) } ;
  while  opt != -1{
       let optu8:u8= opt as u8;
       let optChar:char=char::from(optu8);
       match optChar{
           'a'=> {dflags = dflags |CDESC_ALL!();}
           'f'=> {dflags = dflags | CDESC_NOFUNCS!(); }
           'p'=> {dflags = dflags | CDESC_PATH_ONLY!();
                  dflags  = dflags& !(CDESC_TYPE!()|CDESC_SHORTDESC!()); }
           't'=> {dflags = dflags | CDESC_TYPE!(); 
                  dflags = dflags& !(CDESC_PATH_ONLY!()|CDESC_SHORTDESC!());}
           'P'=> {dflags = dflags | CDESC_PATH_ONLY!()| CDESC_FORCE_PATH!(); 
                  dflags = dflags& !(CDESC_TYPE!()|CDESC_SHORTDESC!());
                }
            _ =>{
                 if opt == -99 {
                     r_builtin_help();
                     return EX_USAGE;
                 }
                unsafe {
                builtin_usage ();
                return EX_USAGE;
                }
            }
        } 
        opt = internal_getopt (list, c_str_afptP.as_ptr() as * mut libc::c_char);
   }
   list = loptend;
    while list !=  std::ptr::null_mut() {
        let found : i32;
        unsafe {
           found = describe_command ((*(*list).word).word, dflags);
        }
        if found ==0 && (dflags & (CDESC_PATH_ONLY!()|CDESC_TYPE!()))==0 {
            unsafe {
                sh_notfound((*(*list).word).word);
            }    
        }
        any_failed = found + any_failed;
        any_failed == 0;
     // (any_failed += found) == 0;
      unsafe {
        list = (*list).next;
      }
      
    }
    if any_failed == 0{
        EXECUTION_SUCCESS!();
    }
    else {
        EXECUTION_FAILURE!();
    }
    return unsafe{sh_chkwrite(opt)};
}


fn describe_command (command : *mut libc::c_char, dflags : i32) -> i32 {
    let mut found : i32 = 0;
    let mut i : i32;
    let mut found_file : i32 = 0;
    let mut f : i32;
    let mut all : i32;
    let mut full_path : *mut libc::c_char;
    let mut x : *mut libc::c_char;
    let mut pathlist : *mut libc::c_char;
    let mut func : *mut SHELL_VAR = 0 as  *mut SHELL_VAR; 
   // let mut alias : *mut alias_t;

    if (dflags & CDESC_ALL!()) != 0{
        all =1 ;     
    }
    else {
        all = 0;
    }
    unsafe {
        full_path = std::ptr::null_mut() ;
    }
/* 
    // #if defined (ALIAS)
    alias = find_alias(command);
    if (((dflags & CDESC_FORCE_PATH!()) == 0) && expand_aliases!=0 && alias != std::ptr::null_mut())
    {
      if (dflags & CDESC_TYPE!()) != 0{
          unsafe {
            libc::puts("alias" as *const libc::c_char );
          }
      }
      else if (dflags & CDESC_SHORTDESC!()) != 0 {
          unsafe{
            println!("{:?} is aliased to {:?}\n",CStr::from_ptr(command), CStr::from_ptr(alias.value));
          } 
      }
      else if dflags & CDESC_REUSABLE!(){
          unsafe {
            x = sh_single_quote((*alias).value);
            println!("alias {:?} = {:?}",CStr::from_ptr(command),CStr::from_ptr(x));
            libc::free(x);
          }  
	}
      found = 1;

      if all == 0 {
        return 1;
      }
    }
*/
    /* Command is a shell reserved word? */
    if ((dflags & CDESC_FORCE_PATH!()) == 0) && unsafe {find_reserved_word(command)} >=0 {
        if dflags & CDESC_TYPE!() != 0 {
            unsafe{ 
                let c_str_keyword = CString::new("keyword").unwrap();
                libc::puts(c_str_keyword.as_ptr());
            }
        }
        else if dflags & CDESC_SHORTDESC!()  != 0 {
            unsafe{
                let name = String::from("iskeyword");
                translation_fn(&name,command,std::ptr::null_mut());
            }
        }
        else if dflags & CDESC_REUSABLE!()  != 0 {
            unsafe {
                println! ("{:?}",CStr::from_ptr(command));
            }
        }

        found = 1;
        if all==0 {
            return 1;
        }
  }

  /* Command is a function? */
  if (dflags & (CDESC_FORCE_PATH!()|CDESC_NOFUNCS!()) == 0) && unsafe{find_function (command)}!=  std::ptr::null_mut()  {
      if dflags & CDESC_TYPE!()  != 0 {
          unsafe {
            let c_str_function = CString::new("function").unwrap();
            libc::puts(c_str_function.as_ptr());
          }
      }
      else if dflags & CDESC_SHORTDESC!() != 0 {
          let mut result : *mut libc::c_char;
          unsafe {
            let name = String::from("isfunction");
            translation_fn(&name,command,std::ptr::null_mut());
            result = named_function_string (command, function_cell(find_function (command)), FUNC_MULTILINE!()|FUNC_EXTERNAL!());
                println!("{:?}",CStr::from_ptr(result));
          }
          
      }
      else if dflags & CDESC_REUSABLE!() != 0{ 
         
            unsafe {
                println!("{:?}",CStr::from_ptr(command));
            }
      }

      found = 1;

      if all == 0{
        return 1; 
      }
    }

     /* Command is a builtin? */
  if ((dflags & CDESC_FORCE_PATH!()) == 0) && unsafe{find_shell_builtin (command)}!=  std::ptr::null_mut()  {
    if dflags  & CDESC_TYPE!() != 0{
        unsafe {
            let c_str_builtin = CString::new("builtin").unwrap();
            libc::puts(c_str_builtin.as_ptr());
        }
    }
    else if dflags & CDESC_SHORTDESC!() != 0{
        if unsafe {posixly_correct}!= 0 && unsafe {find_special_builtin (command)} != std::ptr::null_mut() {
            unsafe {
                 let name = String::from("special");
                 translation_fn(&name,command,std::ptr::null_mut());
            }

        }
        else {
            unsafe {
                let name = String::from("isbuiltin");
                translation_fn(&name,command,std::ptr::null_mut());
            }
        }
    }
    else if dflags & CDESC_REUSABLE!()  != 0 {
        unsafe {
            println!("{:?}",CStr::from_ptr(command));
        }
    }

    found = 1;
    if all == 0{
        return 1;
    }
  }

 /* Command is a disk file? */
  /* If the command name given is already an absolute command, just
     check to see if it is executable. */
     if unsafe {absolute_program (command)} != 0 {
  
       f = unsafe {file_status (command)};
       if f & FS_EXECABLE!()  != 0{
        if dflags & CDESC_TYPE!()  != 0{
            unsafe {
                let c_str_file = CString::new("file").unwrap();
                libc::puts(c_str_file.as_ptr());
            }
        }
       }
       else if dflags & CDESC_SHORTDESC!() != 0 {
           unsafe {
               let name = String::from("is");
               translation_fn(&name,command,command);
           }
       }
       else if dflags & (CDESC_REUSABLE!()|CDESC_PATH_ONLY!()) != 0{
        unsafe {
            println!("{:?}",CStr::from_ptr(command));
       }
 
       /* There's no use looking in the hash table or in $PATH,
          because they're not consulted when an absolute program
          name is supplied. */
       return 1;
     }
    }

    /* If the user isn't doing "-a", then we might care about
     whether the file is present in our hash table. */
  if all == 0 || (dflags & CDESC_FORCE_PATH!() != 0){

    full_path = unsafe{phash_search (command)};
    if full_path != std::ptr::null_mut(){

    if dflags & CDESC_TYPE!() != 0{
        unsafe{
            let c_str_file = CString::new("file").unwrap();
            libc::puts(c_str_file.as_ptr());
        }
    }
    else if dflags & CDESC_SHORTDESC!() != 0{
        unsafe{
            let name = String::from("hashed");
            translation_fn(&name,command,full_path);
        }
    }
    else if (dflags & (CDESC_REUSABLE!()|CDESC_PATH_ONLY!())) != 0{
        unsafe{
            println! ("{:?} ",CStr::from_ptr(full_path));
        }
    }
    unsafe{
        libc::free (full_path as *mut c_void);
    }
    return 1;
    }
  }

  /* Now search through $PATH. */
  #[warn(while_true)]
  while true{
    if dflags & CDESC_STDPATH!() != 0 {
          	/* command -p, all cannot be non-zero */
              unsafe{
                pathlist = conf_standard_path ();
                full_path = find_in_path (command, pathlist, FS_EXEC_PREFERRED!()|FS_NODIRS!());
                libc::free(pathlist as *mut c_void);
              }   
	  /* Will only go through this once, since all == 0 if STDPATH set */
	
    }
    else if all == 0{
        unsafe{
            full_path = find_user_command(command);
        }
        
    }
	else{
        unsafe {
            full_path = user_command_matches (command, FS_EXEC_ONLY!(), found_file);	/* XXX - should that be FS_EXEC_PREFERRED? */
        }
        
    }
      if full_path == std::ptr::null_mut(){
         // return 0;
        break;
      }

    /* If we found the command as itself by looking through $PATH, it
	 probably doesn't exist.  Check whether or not the command is an
	 executable file.  If it's not, don't report a match.  This is
	 the default posix mode behavior */
    if (unsafe {STREQ!(full_path, command)} || unsafe {posixly_correct}!=0){
        unsafe{
            f = file_status (full_path);
        }
        if (f & FS_EXECABLE!() == 0){
            unsafe {
                libc::free (full_path as *mut c_void);
	            full_path =  std::ptr::null_mut() ;
            }
            if all == 0{
                break;
            }	
	    }
	  else if ABSPATH!(full_path){
          ;
      }
	  /* placeholder; don't need to do anything yet */
	  else if dflags & (CDESC_REUSABLE!()|CDESC_PATH_ONLY!()|CDESC_SHORTDESC!()) != 0{
          if MP_DOCWD!()!=0 | (dflags & CDESC_ABSPATH!()) {
              f=MP_RMDOT!();
          }
          else {
              f=0;
          }
          unsafe {
            x = sh_makepath ( std::ptr::null_mut() , full_path, f);
            libc::free (full_path as *mut c_void);
          }
	     
	      full_path = x;
	    }
	}
      /* If we require a full path and don't have one, make one */
    else if ((dflags & CDESC_ABSPATH!())!= 0) && ABSPATH!(full_path) == false {
        unsafe {
            x = sh_makepath ( std::ptr::null_mut() , full_path, MP_DOCWD!()|MP_RMDOT!());
            libc::free (full_path as *mut c_void);
        }
	  full_path = x;
	}
    found_file += 1;
    found = 1;
    if dflags & CDESC_TYPE!() != 0{
          unsafe {
            let c_str_file = CString::new("file").unwrap();
            libc::puts(c_str_file.as_ptr());
          }
    }
    else if dflags & CDESC_SHORTDESC!() != 0{
        unsafe{
            let name = String::from("is");
            translation_fn(&name,command,full_path);
        }
       
    }
	else if dflags & (CDESC_REUSABLE!()|CDESC_PATH_ONLY!()) != 0{
        unsafe{
            println! ("{:?}", CStr::from_ptr(full_path));
        }
        
    }

    unsafe {
        libc::free (full_path as *mut c_void);
    } 
      full_path =  std::ptr::null_mut() ;
      if all == 0{
        break;
      }
	
    }
   found
}

unsafe fn translation_fn (command:&String,args1 : *mut libc::c_char,args2 : *mut libc::c_char) {
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec![ "message.ftl".into()];
    let mut args = FluentArgs::new();
    if args1 !=  std::ptr::null_mut(){
        args.set("str1",format!("{:?}",CStr::from_ptr(args1).to_str().unwrap()));
    }
    if args2 !=  std::ptr::null_mut(){
        args.set("str2",format!("{:?}",CStr::from_ptr(args2).to_str().unwrap()));
    }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let mut value = bundle.get_message(command).unwrap();
    let mut pattern = value.value().expect("partern err");
    let mut errors = vec![];
    if args1 !=  std::ptr::null_mut(){
        let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        println!("{msg1}");
    }
    else{
        let mut msg1 = bundle.format_pattern(&pattern, None, &mut errors);
        println!("{msg1}");
    } 
}
