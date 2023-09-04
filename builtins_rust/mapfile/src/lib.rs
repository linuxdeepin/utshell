use std::{ffi::CString};

use libc::{size_t, ssize_t, c_int, c_uint, c_char, c_uchar, c_long, c_void, PT_NULL};
use rcommon::{r_builtin_usage,r_sh_invalidid};

include!(concat!("intercdep.rs"));

pub const DEFAULT_QUANTUM: c_long = 5000;
pub const MAPF_CLEARARRAY: c_int = 0x01;
pub const MAPF_CHOP: c_int = 0x02;

static mut delim: c_int = 0;

#[no_mangle]
pub extern "C" fn r_mapfile_builtin(mut list: *mut WordList) -> i32 {

    let mut opt: c_int;
    let mut code: c_int;
    let mut fd: c_int = 0;
    let mut flags: c_int = MAPF_CLEARARRAY;
    let intval: c_long = 0;
    let mut lines: c_long = 0;
    let mut origin: c_long = 0;
    let mut nskip: c_long = 0;
    let mut callback_quantum: c_long = DEFAULT_QUANTUM;

    let array_name: *mut c_char;
    let mut callback: *mut c_char = PT_NULL as *mut c_char;

unsafe {

    delim = b'\n' as c_int;

    reset_internal_getopt();
    let opt_str = CString::new("d:u:n:O:tC:c:s:").unwrap();
    opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'd' => delim = *list_optarg as c_int,
            'u' => {
                code = legal_number(list_optarg, std::mem::transmute(&intval));
                if code == 0 || intval < 0 || intval != (intval as c_int) as c_long{
                    builtin_error("%s: invalid file descriptor specification\0".as_ptr() as *const c_char, list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    fd = intval as c_int;
                }
                if sh_validfd(fd) == 0 {
                    builtin_error("%d: invalid file descriptor: %s\0".as_ptr() as *const c_char,
                        fd, libc::strerror(*libc::__errno_location()));
                    return EXECUTION_FAILURE;
                }
            }
            'n' => {
                code = legal_number(list_optarg, std::mem::transmute(&intval));
                if code == 0 || intval < 0 || intval != (intval as c_uint) as c_long {
                    builtin_error("%s: invalid line count\0".as_ptr() as *const c_char, list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    lines = intval;
                }
            }
            'O' => {
                code = legal_number(list_optarg, std::mem::transmute(&intval));
                if code == 0 || intval < 0 || intval != (intval as c_uint) as c_long {
                    builtin_error("%s: invalid array origin\0".as_ptr() as *const c_char,
                              list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    origin = intval;
                }
                flags &= (MAPF_CLEARARRAY as c_uint ^ 0xffffffff) as c_int;
            }
            't' => flags |= MAPF_CHOP,
            'C' => callback = list_optarg,
            'c' => {
                code = legal_number(list_optarg, std::mem::transmute(&intval));
                if code == 0 || intval < 0 || intval != (intval as c_uint) as c_long {
                    builtin_error("%s: invalid callback quantum\0".as_ptr() as *const c_char, list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    callback_quantum = intval;
                }
            }
            's' => {
                code = legal_number(list_optarg, std::mem::transmute(&intval));
                if code == 0 || intval < 0 || intval != (intval as c_uint) as c_long {
                    builtin_error("%s: invalid line count\0".as_ptr() as *const c_char, list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    nskip = intval;
                }
            }
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

    if list.is_null() {
        array_name = "MAPFILE".as_ptr() as *mut c_char;
    } else if (*list).word.is_null() || (*(*list).word).word.is_null() {
        builtin_error("internal error: getting variable name\0".as_ptr() as *const c_char);
        return EXECUTION_FAILURE;
    } else if *(*(*list).word).word == b'\0' as c_char {
        builtin_error("empty array variable name\0".as_ptr() as *const c_char);
        return EX_USAGE;
    } else {
        array_name = (*(*list).word).word;
    }
    if legal_identifier(array_name) == 0 {
        r_sh_invalidid(array_name);
        return EXECUTION_FAILURE;
    }

    return mapfile(fd, lines, origin, nskip, callback_quantum, callback,
        array_name, delim, flags);
}
}

unsafe fn run_callback(callback: *const c_char, curindex: c_uint, curline: *mut c_char) -> c_int
{
    let qline = sh_single_quote(curline);
    let execlen = libc::strlen(callback) + libc::strlen(qline) + 10 + 3;
    let execstr = libc::malloc(execlen);

    let flags = SEVAL_NOHIST;

    libc::snprintf(execstr as *mut c_char, execlen, "%s %d %s\0".as_ptr() as *const c_char,
        callback, curindex, qline);
    libc::free(qline as *mut c_void);
    return evalstring(execstr as *mut c_char, PT_NULL as *const c_char, flags);
}

unsafe fn do_chop(line: *mut c_char, d: c_uchar)
{
    let length = libc::strlen(line);
    if length != 0 && *((line as usize + length - 1) as *mut c_char) == d as c_char {
        *((line as usize + length - 1) as *mut c_char) = b'\0' as c_char;
    }
}

unsafe fn mapfile(fd: c_int, line_count_goal: c_long, origin: c_long, nskip: c_long, callback_quantum: c_long,
    callback: *mut c_char, array_name: *mut c_char, dlm: c_int, flags: c_int) -> c_int
{
    let mut line: *mut c_char = PT_NULL as *mut c_char;
    let mut line_length: size_t = 0;
    let mut unbuffered_read: c_int;

    let entry = find_or_make_array_variable(array_name, 1);
    // let entry_test = *entry;
    // println!("entry:{:#?}",entry_test);

    if entry.is_null() || ((*entry).attributes & att_readonly) != 0 || ((*entry).attributes & att_noassign) != 0 {
        if !entry.is_null() && ((*entry).attributes & att_readonly) != 0 {
            err_readonly(array_name);
        }

        return EXECUTION_FAILURE;
    } else if ((*entry).attributes & att_array) == 0 {
        builtin_error("%s: not an indexed array\0".as_ptr() as *const c_char, array_name);
        return EXECUTION_FAILURE;
    } else if ((*entry).attributes & att_array) != 0 {
        (*entry).attributes &= (att_invisible as c_uint ^ 0xffffffff) as c_int;
    }

    if (flags & MAPF_CLEARARRAY) != 0 {
        array_flush(std::mem::transmute((*entry).value));
    }
    unbuffered_read = ((libc::lseek(fd, 0, SEEK_CUR) < 0) && (*libc::__errno_location() == ESPIPE)) as c_int;

    if dlm != b'\n' as c_int {
        unbuffered_read = 1;
    }
    zreset();

    let mut line_count: c_uint = 0;
    while (line_count as c_long) < nskip {
        if zgetline(fd, std::mem::transmute(&line), std::mem::transmute(&line_length), dlm, unbuffered_read) < 0 {
            break;
        }
        line_count += 1;
    }

    line = PT_NULL as *mut c_char;
    line_length = 0;
    let mut array_index: c_uint = origin as c_uint;
    line_count = 1;
    while zgetline(fd, std::mem::transmute(&line), std::mem::transmute(&line_length), dlm, unbuffered_read) != -1 {
        if (flags & MAPF_CHOP) != 0 {
            do_chop(line, dlm as c_uchar);
        }

        if !callback.is_null() && line_count != 0 && (line_count as c_long % callback_quantum) == 0 {
            run_callback(callback, array_index, line);

            if unbuffered_read == 0 {
                zsyncfd(fd);
            }
        }

        bind_array_element(entry, array_index as c_long, line, 0);

        line_count += 1;
        if line_count_goal != 0 && (line_count as c_long) > line_count_goal {
            break;
        }

        array_index += 1;
    }

    libc::free(line as *mut c_void);

    if unbuffered_read == 0 {
        zsyncfd(fd);
    }

    return EXECUTION_SUCCESS;
}
