use libc::{c_int, c_char, c_void};
use std::ffi::{CStr, CString};
use rcommon::{r_make_builtin_argv,WordList};
include!(concat!("intercdep.rs"));
use rhelp::r_builtin_help;

#[no_mangle]
pub extern "C" fn r_test_builtin(list: *mut WordList) -> i32 {

    let result: c_int;
    let mut argc: c_int = 0;
unsafe {
    if list.is_null() {
        if *this_command_name == b'[' as c_char &&
            *((this_command_name as usize + 1) as *mut c_char) == 0 {
            builtin_error("missing `]'\0".as_ptr() as *mut c_char);
            return EX_BADUSAGE;
        }
        return EXECUTION_FAILURE;
    }
    let argv = r_make_builtin_argv(list, std::mem::transmute(&argc));
    /*
    let mut i = 0;
    let argv = r_make_builtin_argv(list, &argc as *const i32 as*mut i32);
    while  i<(argc)  {
        let tmp = CStr::from_ptr(argv as *mut c_char);
        //println!("test argv={}", tmp.to_str().unwrap());
        libc::printf(CString::new("test:i=%d, argv=%s=\n").unwrap().as_ptr(), i , *argv.offset(i as isize) as *mut c_char);
        i=i+1;
    }
    */
    result = test_command(argc, argv);
    libc::free(argv as *mut c_void);
}
    return result;
}
