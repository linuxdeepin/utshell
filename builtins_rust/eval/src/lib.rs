extern crate libc;
extern crate rcommon;

use libc::{c_char,c_int};
use std::ffi::{CString};
use rcommon::{r_no_options};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
use rhelp::r_builtin_help;

#[macro_export]
macro_rules! SEVAL_NOHIST {
    () => {0x004}
}


extern "C" {
    static loptend:*mut WordList;

    // fn no_options(list:*mut WordList)->i32;
    fn evalstring(string:*mut c_char,from_file:*const c_char,flag:i32)->i32;
    fn string_list(list:*mut WordList)->*mut c_char;
}

#[no_mangle]
pub extern "C" fn r_eval_builtin(mut list:*mut WordList)->i32{
    
    unsafe{
        if r_no_options(list) != 0{
            return EX_USAGE;
        }

        list = loptend;

        if !list.is_null() {
            let c_str = CString::new("eval").unwrap();
            let c_ptr = c_str.as_ptr();
            return evalstring(string_list(list),c_ptr,SEVAL_NOHIST!());
        }
        else{
            return EXECUTION_SUCCESS!();
        }
    }
  
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
