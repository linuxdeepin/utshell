#[macro_use]
#[warn(temporary_cstring_as_ptr)]
extern crate  libc;
extern crate nix;

use std::ffi::CString;
use libc::c_long;

use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
use rhelp::r_builtin_help;


type intmax_t = c_long;
/*
#[macro_export]
macro_rules! ISHELP {
   ($s:expr) => {
    libc::strcmp($s as *const libc::c_char, CString::new("--help").unwrap().as_ptr())
    }
}

#[macro_export]
macro_rules! CHECK_HELPOPT {
  ($l:expr) => {
    if $l  !=std::ptr::null_mut() && (*$l).word !=std::ptr::null_mut() && ISHELP!((*(*$l).word).word) ==0 {
      builtin_help ();
      return EX_USAGE;
    }
  }
}
*/
fn checkhelp(l: *mut WordList) -> i32{
    unsafe {
    let tmp=CString::new("--help").unwrap();
    if l!=std::ptr::null_mut() && (*l).word !=std::ptr::null_mut() && 
        libc::strcmp((*((*l).word)).word, tmp.as_ptr()) == 0 {
            r_builtin_help();
        }
            return EX_USAGE;
    }
}

extern "C" {
    fn get_numeric_arg(list :*mut WordList, i: i32 , intmax :*mut intmax_t) -> i32;
   // fn get_loop_level() -> i32;
    //fn set_continuing(cont : i32);
    //fn set_breaking(breaking : i32);
    fn sh_erange (s:* mut libc::c_char, desc:* mut libc::c_char);
    //pub static  fn  check_loop_level () -> i64;
    /* Non-zero when a "break" instruction is encountered. */
    pub static  posixly_correct :i32;
    static mut breaking : i32;
    static mut continuing : i32;
    static mut loop_level : i32;
    fn builtin_error(err:*const libc::c_char,...);
}

#[no_mangle]
pub extern "C" fn r_break_builtin(mut list :*mut WordList) -> i32 {
    //println!("enter r_break_builtin");
    let  mut  newbreak : intmax_t = 1 as intmax_t;
    unsafe {
        checkhelp(list);
        //CHECK_HELPOPT! (list);
    if check_loop_level() == 0 {
        return EXECUTION_SUCCESS!();
    }
        get_numeric_arg(list, 1, &mut newbreak as *mut intmax_t);

    if newbreak <= 0{
        let mut tmp = CString::new("loop count ").unwrap();
        sh_erange ((*(*list).word).word, tmp.as_ptr() as * mut libc::c_char);
            //set_breaking (get_loop_level());
            breaking =  loop_level;
      return EXECUTION_FAILURE!();
    }

  if newbreak > loop_level as  libc::c_long{ 
    newbreak = loop_level as i64;
  }
  breaking =  newbreak as i32;
 // set_breaking(newbreak as i32);
  }
  return (EXECUTION_SUCCESS!());
}

#[no_mangle]
pub extern "C" fn r_continue_builtin (mut list :*mut WordList) -> i32 {
    let mut newcont : intmax_t = 0 as intmax_t;
    unsafe {
        //CHECK_HELPOPT! (list);
        checkhelp(list);
    }
    if check_loop_level() == 0 {
        return (EXECUTION_SUCCESS!());
    }
    unsafe {
        get_numeric_arg(list, 1, &mut newcont  as *mut intmax_t);
    }
    unsafe {
    if newcont <= 0{
        let mut tmp = CString::new("loop count ").unwrap();
        sh_erange ((*(*list).word).word, tmp.as_ptr() as * mut libc::c_char);
        //set_breaking(get_loop_level());
        breaking =  loop_level;
      return (EXECUTION_FAILURE!());
    }
   if newcont > loop_level.into(){
      newcont = loop_level as i64;
    }
    continuing = newcont as i32;
    //set_continuing(newcont as i32);

    }
    return (EXECUTION_SUCCESS!());
}

#[no_mangle]
pub extern "C" fn check_loop_level () -> i32 {
unsafe { 
  if loop_level == 0 &&  posixly_correct == 0 {
      builtin_error (b"only meaningful in a `for`, `while`, or until `loop` \0" as *const u8 as *const libc::c_char);
      return 0;
  }
   loop_level
}
}

