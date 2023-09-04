use libc::{c_char, c_int };
use std::ffi::CStr;
use std::str;

#[repr(C)]
pub struct WORD_DESC {
    pub word : *mut c_char,
    pub flags : c_int
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct  WORD_LIST {
    next : *mut WORD_LIST,
    word : *mut WORD_DESC
}


//#[link(name = "")]
//extern {
 //   pub fn printf(
#[no_mangle]
pub extern "C" fn r_execute_cmd() {
    //println!("hello");
    //common::builtin_error("test error")
}

#[no_mangle]
pub extern "C" fn r_execute_cmd2(l : *mut WORD_LIST) -> i32 {
    unsafe {
        let mut it :  *mut WORD_LIST = l;
        while std::ptr::null()  !=  it {
            //let mut a = (&((* ((*l).word)).word) );
            let a :*mut c_char =( *(*it).word).word;
            let c_str: &CStr = CStr::from_ptr(a);
            let str_slice: &str = c_str.to_str().unwrap();
            println! ("word is {:?}", str_slice);
            it = (*it).next;
        }
    }
    0
}
