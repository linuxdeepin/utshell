use libc::{c_int, c_char, c_long, c_ulong};
use rcommon::{r_get_exitstat,WordList};
include!(concat!("intercdep.rs"));
use rhelp::r_builtin_help;

#[no_mangle]
pub extern "C" fn r_return_builtin(list: *mut WordList) -> i32 {

unsafe {
    if !list.is_null() && !(*list).word.is_null() &&
        libc::strcmp((*((*list).word)).word, "--help\0".as_ptr() as *const c_char) == 0 {
        r_builtin_help ();
        return EX_USAGE;
    }

    return_catch_value = r_get_exitstat(list);
    if return_catch_flag != 0 {
        siglongjmp(std::mem::transmute(&return_catch), 1);
    } else {
        builtin_error("can only `return' from a function or sourced script\0".as_ptr() as *const c_char);
        return EX_USAGE;
    }
}
    return EXECUTION_SUCCESS;
}
