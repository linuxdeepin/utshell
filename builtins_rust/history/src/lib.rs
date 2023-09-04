use std::{ffi::{CString, CStr}, io::Write};

use libc::{size_t, c_int, c_char, c_long, c_void, PT_NULL};
use rcommon::{r_builtin_usage,r_sh_erange,r_sh_restricted,r_sh_chkwrite,r_get_numeric_arg,WordList};
use rhelp::r_builtin_help;
include!(concat!("intercdep.rs"));

pub const AFLAG: c_int = 0x01;
pub const RFLAG: c_int = 0x02;
pub const WFLAG: c_int = 0x04;
pub const NFLAG: c_int = 0x08;
pub const SFLAG: c_int = 0x10;
pub const PFLAG: c_int = 0x20;
pub const CFLAG: c_int = 0x40;
pub const DFLAG: c_int = 0x80;

#[no_mangle]
pub extern "C" fn r_history_builtin(mut list: *mut WordList) -> i32 {

    let mut flags: c_int = 0;
    let mut opt: c_int;
    let mut result: c_int;

    let mut filename: *mut c_char;
    let mut delete_arg: *mut c_char = PT_NULL as *mut c_char;
    let mut range: *mut c_char;

    let mut delete_offset: c_long = 0;

unsafe {
    reset_internal_getopt();
    let opt_str = CString::new("acd:npsrw").unwrap();
    opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'a' => flags |= AFLAG,
            'c' => flags |= CFLAG,
            'n' => flags |= NFLAG,
            'r' => flags |= RFLAG,
            'w' => flags |= WFLAG,
            's' => flags |= SFLAG,
            'd' => {
                flags |= DFLAG;
                delete_arg = list_optarg;
            }
            'p' => flags |= PFLAG,
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

    opt = flags & (AFLAG | RFLAG | WFLAG | NFLAG);
    if opt != 0 && opt != AFLAG && opt != RFLAG && opt != WFLAG && opt != NFLAG {
        let c_err = CString::new("cannot use more than one of -anrw").unwrap();
        builtin_error( c_err.as_ptr());
        return EXECUTION_FAILURE;
    }

    if (flags & CFLAG) != 0 {
        bash_clear_history();
        if list.is_null() {
            return EXECUTION_SUCCESS;
        }
    }

    if (flags & SFLAG) != 0 {
        if !list.is_null() {
            push_history(list);
        }
        return EXECUTION_SUCCESS;
    }
    else if (flags & PFLAG) != 0 {
        if !list.is_null() {
            return expand_and_print_history(list);
        }
        return r_sh_chkwrite(EXECUTION_SUCCESS);
    } 
    else if (flags & DFLAG) != 0 {
        let c_tmp = if *delete_arg == b'-' as c_char {delete_arg.offset(1 as isize ) as *mut c_char} else {delete_arg};
        range = libc::strchr(c_tmp, b'-' as c_int);
        if  !range.is_null() {
            let mut delete_start: c_long = 0;
            let mut delete_end: c_long = 0;

        *range = b'\0' as c_char;
        range = (range as usize + 1) as *mut c_char;
        if legal_number(delete_arg, std::mem::transmute(&delete_start)) == 0 ||
        legal_number(range, std::mem::transmute(&delete_end)) == 0 {
            *((range as usize - 1) as *mut c_char) = b'-' as c_char;
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *delete_arg == b'-' as c_char && delete_start < 0 {
            delete_start += history_length as c_long;
            if delete_start < history_base as c_long {
                r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_start > 0 {
            delete_start -= history_base as c_long;
        }
        if delete_start < 0 || delete_start >= history_length as c_long {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        if *range == b'-' as c_char && delete_end < 0 {
            delete_end += history_length as c_long;
            if delete_end < history_base as c_long {
                r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
        } else if delete_end > 0 {
            delete_end -= history_base as c_long;
        }

        if delete_end < 0 || delete_end >= history_length as c_long {
            r_sh_erange(range, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }
        result = bash_delete_history_range(delete_start as c_int, delete_end as c_int);
        if where_history() > history_length {
            history_set_pos(history_length);
        }

        return if result != 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
        }
     else if (flags & DFLAG) != 0 {
        if legal_number(delete_arg, &mut delete_offset) == 0 {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        }

        if *delete_arg == b'-' as c_char && delete_offset < 0 {
            let ind = history_length + delete_offset as c_int;
            if ind < history_base {
                r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
                return EXECUTION_FAILURE;
            }
            opt = ind + history_base;
        } else if delete_offset < history_base as c_long ||
            (delete_offset >= (history_base + history_length) as c_long) {
            r_sh_erange(delete_arg, "history position\0".as_ptr() as *mut c_char);
            return EXECUTION_FAILURE;
        } else {
            opt = delete_offset as c_int;
        }
        result = bash_delete_histent(opt - history_base);
        if where_history() > history_length {
            history_set_pos(history_length);
        }
        return if result != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
    }
}
    else if (flags & (AFLAG | RFLAG | NFLAG | WFLAG | CFLAG)) == 0 {
        result = display_history(list);
        return r_sh_chkwrite(result);
    }

    filename = if !list.is_null() {(*((*list).word)).word} else {get_string_value("HISTFILE\0".as_ptr() as *mut c_char)};
    result = EXECUTION_SUCCESS;

    if restricted != 0 && !(libc::strchr(filename, b'/' as c_int).is_null()) {
        r_sh_restricted(filename);
        return EXECUTION_FAILURE;
    }
    if (flags & AFLAG) != 0 {
        result = maybe_append_history(filename);
    } else if (flags & WFLAG) != 0 {
        result = write_history(filename);
    } else if (flags & RFLAG) != 0{
        result = read_history(filename);
        history_lines_in_file = history_lines_read_from_file;
    } else if (flags & NFLAG) != 0{
        let old_history_lines = history_lines_in_file;
        let obase = history_base;

        using_history();
        result = read_history_range(filename, history_lines_in_file, -1);
        using_history();

        history_lines_in_file = history_lines_read_from_file;
        if force_append_history == 0 {
            history_lines_this_session +=
            history_lines_in_file - old_history_lines + history_base - obase;
        }
    }
}

    return if result != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
}

fn histtime(hlist: *mut HIST_ENTRY, histtimefmt: *const c_char) -> *mut c_char
{
unsafe {
    static mut timestr: [c_char;128] = [0;128];

    let mut t = history_get_time(hlist);
    let tm = if t != 0 {libc::localtime(&t)} else {PT_NULL as *mut libc::tm};
    if t != 0 && !tm.is_null() {
        strftime(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr),
        histtimefmt,
        tm);
    } else if !(*hlist).timestamp.is_null() && (*(*hlist).timestamp) != 0 {
        let c_str = CString::new("%s: invalid timestamp").unwrap();
        libc::snprintf(std::mem::transmute(&timestr),
        std::mem::size_of_val(&timestr), c_str.as_ptr(),
        if *((*hlist).timestamp) == b'#' as c_char {((*hlist).timestamp as usize + 1) as *mut c_char} else {(*hlist).timestamp});
    } else {
        libc::strcpy(std::mem::transmute(&timestr), b"??\0".as_ptr() as *const c_char);
    }

    return timestr.as_mut_ptr();
}
}

unsafe fn quit()
{
    if terminating_signal != 0 {
        termsig_handler(terminating_signal);
    }
    if interrupt_state != 0 {
        throw_to_top_level();
    }
}

unsafe fn display_history(list: *mut WordList) -> c_int
{
    let mut limit:c_long = 0;
    let mut histtimefmt: *mut c_char;
    let mut timestr: *mut c_char;

    if !list.is_null() {
        if  r_get_numeric_arg(list, 0,&mut limit)== 0 {
            return EXECUTION_FAILURE;
        }
        if limit < 0 {
            limit = -limit;
        }
    } else {
        limit = -1;
    }
    let hlist = history_list();

    if !hlist.is_null() {
        let mut i: c_long = 0;
        while !(*hlist.offset(i as isize)).is_null() {
            i += 1;
        }

        i = if 0 <= limit && limit < i {i - limit} else {0};

        histtimefmt = get_string_value(b"HISTTIMEFORMAT\0" as *const u8 as *const c_char);

        while !(*hlist.offset(i as isize)).is_null() {
            if terminating_signal != 0 {
                termsig_handler(terminating_signal);
            }
            if interrupt_state != 0 {
                throw_to_top_level();
            }
            timestr = if !histtimefmt.is_null() && *histtimefmt as libc::c_int != 0 {
                histtime(*hlist.offset(i as isize), histtimefmt)
            } else {
                0 as *mut libc::c_void as *mut libc::c_char
            };
            printf(
                b"%5d%c %s%s\n\0" as *const u8 as *const libc::c_char,
                i + history_base as c_long,
                if !((**hlist.offset(i as isize)).data).is_null() {
                    '*' as i32
                } else {
                    ' ' as i32
                },
                if !timestr.is_null() && *timestr as libc::c_int != 0 {
                    timestr
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                (**hlist.offset(i as isize)).line,
            );
            i += 1;
        }
    }
    return EXECUTION_SUCCESS;
}

fn push_history(list: *mut WordList) {
unsafe {
    if remember_on_history != 0 && hist_last_line_pushed == 0 &&
        (hist_last_line_added != 0 || (current_command_line_count > 0 && current_command_first_line_saved != 0 && command_oriented_history != 0)) &&
        bash_delete_last_history() == 0 {
        return;
    }

    let s = string_list(list);
    check_add_history(s, 1);

    hist_last_line_pushed = 1;
    libc::free(s as *mut c_void);
}
}

fn expand_and_print_history(mut list: *mut WordList) -> c_int
{
unsafe {

    let mut s: *mut c_char = PT_NULL as *mut c_char;
    let mut result: c_int;

    if hist_last_line_pushed == 0 && hist_last_line_added != 0 && bash_delete_last_history() == 0 {
        return EXECUTION_FAILURE;
    }
    result = EXECUTION_SUCCESS;
    while !list.is_null() {
        let r = history_expand((*((*list).word)).word, std::mem::transmute(&s));
        if r < 0 {
            let c_err = CString::new("%s: history expansion failed").unwrap();
            builtin_error( c_err.as_ptr(), (*((*list).word)).word);
            result = EXECUTION_FAILURE;
        } else {
            println!("{}",CStr::from_ptr(s).to_str().unwrap());
            //println!("{}",String::from(CStr::from_ptr(s).to_str().unwrap()));
            //std::io::stdout().lock().write_all(CStr::from_ptr(s).to_bytes()).unwrap();
            //libc::putchar(b'\n' as c_int);
        }
        if !s.is_null() {
            libc::free(s as *mut c_void);
        }
        list = (*list).next;
    }
    std::io::stdout().lock().flush().unwrap();
    return result;
}
}
