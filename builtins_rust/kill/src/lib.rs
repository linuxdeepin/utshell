use std::{ffi::{CString, CStr}};
use libc::{c_int, c_uint, c_char, c_long, c_void, PT_NULL};

// use rcommon::{r_builtin_usage,r_sh_invalidsig,r_sh_badpid,r_sh_badjob,r_get_job_spec,r_display_signal_list,WordList};

include!(concat!("intercdep.rs"));


#[no_mangle]
pub extern "C" fn r_kill_builtin(mut list: *mut WordList) -> i32 {

unsafe {
    let mut word: *mut c_char;
	let mut pid:libc::pid_t;
	let mut pid_value: c_long = 0;

	if list.is_null() {
		r_builtin_usage();
		return EX_USAGE;
	}

	if !list.is_null() && !(*list).word.is_null() &&
		libc::strcmp((*((*list).word)).word, "--help\0".as_ptr() as *const c_char) == 0 {
        r_builtin_help ();
        return EX_USAGE;
    }

	let mut any_succeeded: c_int = 0;
    let mut listing: c_int = 0;
    let mut saw_signal: c_int = 0;
	let mut sig = libc::SIGTERM;
	let mut sigspec = "TERM\0".as_ptr() as *mut c_char;

	let dflags = DSIG_NOCASE | if posixly_correct == 0 {DSIG_SIGPREFIX} else {0};
	while !list.is_null() {
		word = (*((*list).word)).word;
		if is_option(word, b'l') || is_option(word, b'L') {
			listing += 1;
			list = (*list).next;
		} else if is_option(word, b's') || is_option(word, b'n') {
            list = (*list).next;
			if !list.is_null() {
				sigspec = (*((*list).word)).word;
				if *sigspec == b'0' as c_char &&
					*((sigspec as usize + 1) as *mut c_char) == b'\0' as c_char {
					sig = 0;
				} else {
					sig = decode_signal(sigspec, dflags);
                }
				list = (*list).next;
				saw_signal += 1;
			} else {
				sh_needarg(word);
				return EXECUTION_FAILURE;
			}
		} else if *word == b'-' as c_char &&
				  *((word as usize + 1) as *mut c_char) == b's' as c_char &&
				  libc::isalpha(*((word as usize + 2) as *mut c_char) as c_int) != 0 {
			sigspec = (word as usize + 2) as *mut c_char;
			if *sigspec == b'0' as c_char &&
				*((sigspec as usize + 1) as *mut c_char) == b'\0' as c_char {
				sig = 0;
			} else {
				sig = decode_signal(sigspec, dflags);
            }
			list = (*list).next;
			saw_signal += 1;
		} else if *word == b'-' as c_char &&
		          *((word as usize + 1) as *mut c_char) == b'n' as c_char &&
				  libc::isdigit(*((word as usize + 2) as *mut c_char) as c_int) != 0 {
			sigspec = (word as usize + 2) as *mut c_char;
			if *sigspec == b'0' as c_char &&
				*((sigspec as usize + 1) as *mut c_char) == b'\0' as c_char {
				sig = 0;
			} else {
				sig = decode_signal(sigspec, dflags);
            }
			list = (*list).next;
			saw_signal += 1;
		} else if is_option(word, b'-') {
			list = (*list).next;
			break;
		} else if is_option(word, b'?') {
			r_builtin_usage();
			return EX_USAGE;
		} else if *word == b'-' as c_char && saw_signal == 0 {
			sigspec = (word as usize + 1) as *mut c_char;
			sig = decode_signal(sigspec, dflags);
			saw_signal += 1;
			list = (*list).next;
		} else {
			break;
        }
	}

	if listing != 0 {
		return r_display_signal_list(list, 0);
	}

	if sig == NO_SIG {
		r_sh_invalidsig(sigspec);
		return EXECUTION_FAILURE;
	}

	if list.is_null() {
		r_builtin_usage();
		return EX_USAGE;
	}

	while !list.is_null() {
		word = (*((*list).word)).word;

		if *word == b'-' as c_char {
			word = (word as usize + 1) as *mut c_char;
		}

		if *word != 0 && legal_number((*((*list).word)).word, std::mem::transmute(&pid_value)) != 0
		    && (pid_value == (pid_value as c_int) as c_long) {
			pid = pid_value as libc::pid_t;

			if kill_pid(pid, sig, (pid < -1) as c_int) < 0 {
				if *(libc::__errno_location()) == EINVAL {
					r_sh_invalidsig(sigspec);
				} else {
					kill_error(pid, *(libc::__errno_location()));
				}
				list = (*list).next;
				continue;
			} else {
				any_succeeded += 1;
			}
		}
		else if *((*((*list).word)).word) != 0 && *((*((*list).word)).word) != b'%' as c_char {
			eprint!("utshell : kill :");
            let names = String::from("killargerr");
            err_translate_fn(&names,(*((*list).word)).word);
			println!();
			list = (*list).next;
			continue;
		} else if *word != 0 {
			let set: libc::sigset_t = std::mem::zeroed();
			let oset: libc::sigset_t = std::mem::zeroed();
			let j: *mut JOB;

			libc::sigemptyset(std::mem::transmute(&set));
			libc::sigaddset(std::mem::transmute(&set), libc::SIGCHLD);
			libc::sigemptyset(std::mem::transmute(&oset));
			libc::sigprocmask(libc::SIG_BLOCK, std::mem::transmute(&set), std::mem::transmute(&oset));
			
		    let job = r_get_job_spec(list);

			if job < 0 || job > js.j_jobslots || ((jobs as usize + job as usize * 8) as *mut JOB).is_null() {
				if job != DUP_JOB {
					r_sh_badjob((*((*list).word)).word);
				}
				libc::sigprocmask(libc::SIG_SETMASK, std::mem::transmute(&oset), PT_NULL as *mut libc::sigset_t);
				list = (*list).next;
				continue;
			}

			j = (jobs as usize + job as usize * 8) as *mut JOB;
			if (*j).flags & J_JOBCONTROL != 0 {
				pid = (*j).pgrp;
			} else {
				pid = (*((*j).pipe)).pid;
			}

			libc::sigprocmask(libc::SIG_SETMASK, std::mem::transmute(&oset), PT_NULL as *mut libc::sigset_t);

			if kill_pid(pid, sig, 1) < 0 {
				if *(libc::__errno_location()) == EINVAL {
					r_sh_invalidsig(sigspec);
				} else {
					kill_error(pid, *(libc::__errno_location()));
				}
				list = (*list).next;
				continue;
			} else {
				any_succeeded += 1;
			}
		}
		else {
			r_sh_badpid((*((*list).word)).word);
			list = (*list).next;
			continue;
		}
		list = (*list).next;
	}

	return if any_succeeded != 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
}
}

unsafe fn is_option(s: *mut c_char, c: u8) -> bool
{
    let str = CStr::from_ptr(s).to_bytes_with_nul();
    return str[0] == b'-' && str[1] == c && str[2] == 0
}

unsafe fn kill_error(pid: libc::pid_t, e: c_int)
{
    let mut x = libc::strerror(e);
    if x.is_null() {
        x = "Unknown error".as_ptr() as *mut c_char;
    }

    builtin_error("(%ld) - %s".as_ptr() as *const c_char, pid, x);
}
