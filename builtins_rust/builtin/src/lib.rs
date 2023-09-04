use libc::{c_int, c_char};
use std::ffi::CStr;
include!(concat!("intercdep.rs"));

#[no_mangle]
pub extern "C" fn r_builtin_builtin(mut list: *mut WordList) -> i32 {
    unsafe{
        let mut function: Option::<sh_builtin_func_t> = None;
        if no_options(list) != 0 {
            return EX_USAGE;
        }
        list = loptend;
        if list == std::ptr::null_mut() {
            return EXECUTION_SUCCESS!();
        }
        let mut command: &CStr = CStr::from_ptr((*(*list).word).word as *mut c_char);
        function = find_shell_builtin(command.as_ptr() as *mut c_char);
        if function.is_none() {
            sh_notbuiltin(command.as_ptr() as *mut c_char);
            return EXECUTION_FAILURE!();
        } else {
            this_command_name = command.as_ptr() as *mut c_char;
            this_shell_builtin = function;
            list = (*list).next;
            return (Some(function.expect("non-null function pointer")))
                .expect("non-null function pointer")(list);
        };
    }
}

