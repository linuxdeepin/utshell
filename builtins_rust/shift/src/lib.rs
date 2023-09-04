use libc::{c_int, c_char, c_long, PT_NULL};
use rcommon::{r_sh_erange,};
use rhelp::r_builtin_help;

include!(concat!("intercdep.rs"));

pub static print_shift_error: c_int = 0;

#[no_mangle]
pub extern "C" fn r_shift_builtin(list: *mut WordList) -> i32 {

unsafe {
    if !list.is_null() && !(*list).word.is_null() &&
        libc::strcmp((*((*list).word)).word, "--help\0".as_ptr() as *const c_char) == 0 {
        r_builtin_help ();
        return EX_USAGE;
    }

    let mut times: c_int = 0;
    if get_numeric_arg(list, 0, std::mem::transmute(&times)) == 0 {
        return EXECUTION_FAILURE;
    }

    if times == 0 {
        return EXECUTION_SUCCESS;
    } else if times < 0 {
        let s = if list.is_null() {PT_NULL as *mut c_char} else {(*(*list).word).word};
        r_sh_erange(s,"shift count\0".as_ptr() as *mut c_char);
        return EXECUTION_FAILURE;
    }

    let nargs = number_of_args();
    if times > nargs {
        if print_shift_error != 0 {
            let s = if list.is_null() {PT_NULL as *mut c_char} else {(*(*list).word).word};
            r_sh_erange(s,"shift count\0".as_ptr() as *mut c_char);  
        }
        return EXECUTION_FAILURE;
    } else if times == nargs {
        clear_dollar_vars();
    } else {
        shift_args(times);
    }

    invalidate_cached_quoted_dollar_at();
}
    return EXECUTION_SUCCESS;
}
