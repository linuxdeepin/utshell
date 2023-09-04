use std::{ffi::CString};
use libc::{size_t, c_int, c_uint, c_char, c_long, c_void, PT_NULL, c_ulong, strchr};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[no_mangle]
pub extern "C" fn r_colon_builtin(ignore: *mut WordList )->i32 {
    0
}

#[no_mangle]
pub extern "C" fn r_false_builtin(ignore: *mut  WordList) -> i32 {
    1
}
