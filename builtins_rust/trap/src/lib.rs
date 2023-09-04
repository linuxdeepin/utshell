use std::{ffi::CString};

use libc::{c_int, c_char, c_void, PT_NULL};
use rcommon::{r_builtin_usage,r_display_signal_list,WordList,r_sh_invalidsig,r_sh_chkwrite};
use rhelp::r_builtin_help;
include!(concat!("intercdep.rs"));


#[no_mangle]
pub extern "C" fn r_trap_builtin(mut list: *mut WordList) -> i32 {

    let mut list_signal_names: c_int = 0;
    let mut display: c_int = 0;
    let mut result: c_int = EXECUTION_SUCCESS;

unsafe {
    reset_internal_getopt();
    let opt_str = CString::new("lp").unwrap();
    let mut opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'l' => list_signal_names += 1,
            'p' => display += 1,
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
                r_builtin_usage ();
                return EX_USAGE;
            }
        }
        opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    }
    list = loptend;

    opt = DSIG_NOCASE | DSIG_SIGPREFIX;

    if list_signal_names != 0 {
        return r_sh_chkwrite(r_display_signal_list(PT_NULL as *mut WordList, 1));
    } else if display != 0 || list.is_null() {
        initialize_terminating_signals();
        get_all_original_signals();
        return r_sh_chkwrite(display_traps(list, (display != 0 && posixly_correct != 0) as c_int));
    } else {
        let mut operation = SET;
        let first_arg = (*(*list).word).word;
        let first_signal = !first_arg.is_null() && *first_arg != 0 &&
            all_digits(first_arg) != 0 && decode_signal (first_arg,opt) != NO_SIG;
        if first_signal {
            operation = REVERT;
        } else if posixly_correct == 0 && !first_arg.is_null() && *first_arg != 0 &&
            (*first_arg != b'-' as c_char || *((first_arg as usize + 1) as *mut c_char) != 0 ) &&
            decode_signal (first_arg,opt) != NO_SIG && (*list).next.is_null() {
            operation = REVERT;
        } else {
            list = (*list).next;
            if list.is_null() {
                r_builtin_usage();
                return EX_USAGE;
            } else if *first_arg == b'\0' as c_char {
                operation = IGNORE;
            } else if *first_arg == b'-' as c_char && *((first_arg as usize + 1) as *mut c_char) == 0 {
                operation = REVERT;
            }
        }

        if subshell_environment & SUBSHELL_RESETTRAP != 0 {
            free_trap_strings();
            subshell_environment &= !SUBSHELL_RESETTRAP;
        }

        let mut sig: c_int;
        while !list.is_null() {
            sig = decode_signal((*(*list).word).word, opt);
            if sig == NO_SIG {
                r_sh_invalidsig((*(*list).word).word);
                result = EXECUTION_FAILURE;
            } else {
                match operation {
                    SET => set_signal(sig, first_arg),
                    IGNORE => ignore_signal(sig),
                    REVERT => {
                        restore_default_signal(sig);
                        match sig {
                            libc::SIGINT => {
                                if interactive != 0 {
                                    set_signal_handler(libc::SIGINT, sigint_sighandler as *mut SigHandler);
                                } else if interactive_shell != 0 &&
                                    (sourcelevel != 0 || running_trap != 0 || parse_and_execute_level != 0) {
                                        set_signal_handler(libc::SIGINT, sigint_sighandler as *mut SigHandler);
                                } else {
                                    set_signal_handler(libc::SIGINT, termsig_sighandler as *mut SigHandler);
                                }
                            }
                            libc::SIGQUIT => {
                                set_signal_handler(libc::SIGQUIT, std::mem::transmute(1_usize));
                            }
                            libc::SIGTERM | libc::SIGTTIN | libc::SIGTTOU | libc::SIGTSTP => {
                                if interactive != 0 {
                                    set_signal_handler(sig, std::mem::transmute(1_usize));
                                }
                            }
                            _ => (),
                        }
                        break;
                    }
                    _ => (),
                }
            }

            list = (*list).next;
        }
    }
}
    return result;
}

unsafe fn showtrap(i: c_int, show_default: c_int)
{
    let t: *mut c_char;

    let p = trap_list[i as usize];
    if (p == libc::SIG_DFL as *mut c_char) && signal_is_hard_ignored(i) == 0 {
        if show_default != 0 {
            t = "-\0".as_ptr() as *mut c_char;
        } else {
            return;
        }
    } else if signal_is_hard_ignored(i) != 0 {
        t = PT_NULL as *mut c_char;
    } else {
        t = if p == libc::SIG_IGN as *mut c_char {PT_NULL as *mut c_char} else {sh_single_quote(p)}
    }

    let sn = signal_name(i);
    if libc::strncmp(sn, "SIGJUNK\0".as_ptr() as *const c_char, 7) == 0 ||
    libc::strncmp(sn, "unknown\0".as_ptr() as *const c_char, 7) == 0 {
        libc::printf("trap -- %s %d\n\0".as_ptr() as *const c_char, if t.is_null() {"''\0".as_ptr() as *mut c_char} else {t}, i);
    } else if posixly_correct != 0 {
        if libc::strncmp(sn, "SIG\0".as_ptr() as *const c_char, 3) == 0 {
            libc::printf("trap -- %s %s\n\0".as_ptr() as *const c_char, if t.is_null() {"''\0".as_ptr() as *mut c_char} else {t}, (sn as usize + 3) as *mut c_char);
        } else {
            libc::printf("trap -- %s %s\n\0".as_ptr() as *const c_char, if t.is_null() {"''\0".as_ptr() as *mut c_char} else {t}, sn);
        }
    } else {
        libc::printf("trap -- %s %s\n\0".as_ptr() as *const c_char, if t.is_null() {"''\0".as_ptr() as *mut c_char} else {t}, sn);
    }

    if show_default == 0 {
        if !t.is_null() {
            libc::free(t as *mut c_void);
        }
    }
}

unsafe fn display_traps(mut list: *mut WordList, show_all: c_int) -> c_int
{
    if list.is_null() {
        for i in 0..BASH_NSIG {
            showtrap(i, show_all);
        }
        return EXECUTION_SUCCESS;
    }

    let mut result = EXECUTION_SUCCESS;
    let mut i: c_int;
    while !list.is_null() {
        i = decode_signal((*(*list).word).word, DSIG_NOCASE | DSIG_SIGPREFIX);
        if i == NO_SIG {
            r_sh_invalidsig((*(*list).word).word);
            result = EXECUTION_FAILURE;
        } else {
            showtrap(i, show_all);
        }

        list = (*list).next;
    }

    return result;
}
