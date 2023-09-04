use libc::{c_int, c_char, c_long, PT_NULL};
use rcommon::{r_no_args,WordList,r_builtin_usage};
use rhelp::r_builtin_help;
include!(concat!("intercdep.rs"));

pub static mut old_cont: *mut SigHandler = PT_NULL as *mut SigHandler;

#[no_mangle]
pub extern "C" fn r_suspend_builtin(mut list: *mut WordList) -> i32 {

    let mut opt: c_int;
    let mut force: c_int = 0;

unsafe {
    reset_internal_getopt();
    let opt_str = "f\0".as_ptr()  as *mut c_char;
    opt = internal_getopt (list, opt_str);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'f' => force += 1,
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
            r_builtin_usage ();
            return EX_USAGE;
            }
        }
        
        opt = internal_getopt (list, opt_str);
    }
    list = loptend;
    if job_control == 0 {
        sh_nojobs("cannot suspend\0".as_ptr() as *mut c_char);
        return EXECUTION_FAILURE;
    }
    if force == 0 {
        r_no_args(list);
        if login_shell != 0 {
            builtin_error("cannot suspend a login shell\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
    }

    old_cont = set_signal_handler(libc::SIGCONT, std::mem::transmute(suspend_continue as usize));
    killpg(shell_pgrp, libc::SIGSTOP);
}
    return EXECUTION_SUCCESS;
}

unsafe fn suspend_continue(sig: c_int)
{
    set_signal_handler(libc::SIGCONT, old_cont);
}
