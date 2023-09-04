use std::ffi::CStr;
use std::{ffi::CString};
use libc::{size_t, c_int, c_uint, c_char, c_long, c_void, PT_NULL, c_ulong, strchr, };

use rcommon::{r_builtin_usage,r_sh_invalidid,r_sh_wrerror,r_builtin_bind_variable,SHELL_VAR, r_savestring};

include!(concat!("intercdep.rs"));

macro_rules! IS_DIGITAL {
    ($x: expr) => {
        $x >= b'0' as c_char && $x <= b'9' as c_char
    };
}

unsafe fn QUIT()
{
    if terminating_signal != 0 {
        termsig_handler(terminating_signal);
    }

    if interrupt_state != 0 {
        throw_to_top_level();
    }
}

unsafe fn PC(c: u8)
{
    let mut b: [c_char; 2] = [0; 2];
    tw += 1;
    b[0] = c as c_char;
    if vflag != 0 {
        vbadd(b.as_ptr() as *mut c_char, 1);
    } else {
        libc::putchar(c as c_int );
    }
    QUIT();
}

static mut conversion_error: c_int = 0;
static mut conv_buf: *mut c_char = PT_NULL as *mut c_char;
static mut conv_bufsize: size_t = 0;

static mut vbuf: *mut c_char = PT_NULL as *mut c_char;
static mut vname: *mut c_char = PT_NULL as *mut c_char;
static mut vflag: c_int = 0;
static mut vbsize: size_t = 0;
static mut vblen: c_int = 0;

static mut retval: c_int = 0;
static mut tw: c_long = 0;

static mut garglist: *mut WordList = PT_NULL as *mut WordList;
static mut orig_arglist: *mut WordList = PT_NULL as *mut WordList;


#[no_mangle]
pub extern "C" fn r_printf_builtin(mut list: *mut WordList) -> i32 {

    let mut ch: c_int;
    let mut fieldwidth: c_int;
    let mut have_fieldwidth: c_int;
    let mut precision: c_int;
    let mut have_precision: c_int;
    let mut arrayflags: c_int;

    let mut fmt;
    let mut start;
    let mut modstart;
    let mut convch;
    let mut thisch;
    let mut nextch;
unsafe {

    let PRETURN = |out_val: c_int| {
        QUIT();
        if vflag != 0 {
            let v = r_builtin_bind_variable(vname, vbuf, 0);
            stupidly_hack_special_variables(vname);
            if v.is_null() || ((*v).attributes & att_readonly) != 0 || ((*v).attributes & att_noassign) != 0 {
                return EXECUTION_FAILURE;
            }
        }
        if conv_bufsize > 4096 {
            libc::free(conv_buf as *mut c_void);
            conv_bufsize = 0;
            conv_buf = PT_NULL as *mut c_char;
        }

        if vbsize > 4096 {
            libc::free(vbuf as *mut c_void);
            vbsize = 0;
            vbuf = PT_NULL as *mut c_char;
        } else if !vbuf.is_null() {
            *vbuf = 0;
        }

        if libc::ferror(stdout) == 0 {
            libc::fflush(stdout);
        }
        QUIT();
        if libc::ferror(stdout) != 0 {
            r_sh_wrerror();
            libc::clearerr(stdout);
            return EXECUTION_FAILURE;
        }

        return out_val;
    };

    vflag = 0;
    reset_internal_getopt();
    let opt_str = CString::new("v:").unwrap();
    let mut opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'v' => {
                vname = list_optarg;
                arrayflags = if assoc_expand_once != 0 {VA_NOEXPAND | VA_ONEWORD} else {0};
                if legal_identifier(vname) != 0 || valid_array_reference(vname, arrayflags) != 0 {
                    vflag = 1;
                    if vbsize == 0 {
                        vbsize = 16;
                        vbuf = xmalloc(16) as *mut c_char;
                    }
                    vblen = 0;
                    if !vbuf.is_null() {
                        *vbuf = 0;
                    }
                } else {
                    r_sh_invalidid(vname);
                    return EX_USAGE;
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
        r_builtin_usage();
        return EX_USAGE;
    }

    if vflag != 0 && !((*(*list).word).word.is_null()) && *(*(*list).word).word == b'\0' as c_char {
        let v = r_builtin_bind_variable(vname, "\0".as_ptr() as *mut c_char, 0);
        stupidly_hack_special_variables(vname);
        return if v.is_null() || ((*v).attributes & att_readonly) != 0 || ((*v).attributes & att_noassign) != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};
    }

    if (*(*list).word).word.is_null() || *(*(*list).word).word == b'\0' as c_char {
        return EXECUTION_SUCCESS;
    }

    let format = (*(*list).word).word;
    tw = 0;

    garglist = (*list).next;
    orig_arglist =  (*list).next;

    if format.is_null() || *format == 0 {
        return EXECUTION_SUCCESS;
    }

    'outer: loop {
        tw = 0;
        fmt = format;

        while *fmt != 0 {
            precision = 0;
            fieldwidth = 0;
            have_fieldwidth = 0;
            have_precision = 0;

            if *fmt == b'\\' as c_char {               
                fmt = (fmt as usize + 1) as *mut c_char;
                
                let mut mbch: [libc::c_char;25] = [0; 25];
                let mut mblen: c_int = 0;
                fmt = (fmt as usize + tescape(fmt, mbch.as_ptr() as *mut c_char, std::mem::transmute(&mblen), PT_NULL as *mut c_int) as usize) as *mut c_char;
                let mut mbind = 0;
               
                while mbind < mblen {
                    PC(mbch[mbind as usize] as u8);
                    mbind += 1;
                }
                fmt = (fmt as usize - 1) as *mut c_char;
            
                fmt = (fmt as usize + 1) as *mut c_char;
                continue;
            }

            if *fmt != b'%' as c_char {
                PC(*fmt as u8);

                fmt = (fmt as usize + 1) as *mut c_char;
                continue;
            }

            start = fmt;
            fmt = (fmt as usize + 1) as *mut c_char;
            if *fmt == b'%' as c_char {
                PC(b'%');

                fmt = (fmt as usize + 1) as *mut c_char;
                continue;
            }

            while *fmt != 0 && !(strchr("#'-+ 0\0".as_ptr() as *const c_char, *fmt as c_int).is_null()) {
                fmt = (fmt as usize + 1) as *mut c_char;
            }

            if *fmt == b'*' as c_char {
                fmt = (fmt as usize + 1) as *mut c_char;
                have_fieldwidth = 1;
                fieldwidth = getint();
            } else {
                while IS_DIGITAL!(*fmt) {
                    fmt = (fmt as usize + 1) as *mut c_char;
                }
            }

            if *fmt == b'.' as c_char {
                fmt = (fmt as usize + 1) as *mut c_char;
                if *fmt == b'*' as c_char {
                    fmt = (fmt as usize + 1) as *mut c_char;
                    have_precision = 1;
                    precision = getint();
                } else {
                    if *fmt == b'-' as c_char {
                        fmt = (fmt as usize + 1) as *mut c_char;
                    }
                    while IS_DIGITAL!(*fmt) {
                        fmt = (fmt as usize + 1) as *mut c_char;
                    }
                }
            }

            modstart = fmt;
            while *fmt != 0 && !(strchr("hjlLtz\0".as_ptr() as *const c_char, *fmt as c_int).is_null()) {
                fmt = (fmt as usize + 1) as *mut c_char;
            }

            if *fmt == 0 {
                builtin_error("`%s': missing format character\0".as_ptr() as *const c_char, start);
                return PRETURN(EXECUTION_FAILURE);
            }

            convch = *fmt;
            thisch = *modstart;
            nextch = *((modstart as usize + 1) as *mut c_char);
            *modstart = convch;
            *((modstart as usize + 1) as *mut c_char) = b'\0' as c_char;

            QUIT();
            let format_type = convch as u8;
            match format_type {
                b'c' => {
                    let p = getchr();
                    let f = start;
                    libc::clearerr(stdout);
                    let PF = || {
                        let nw: c_int;
                        if vflag == 0 {
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = libc::printf(f, fieldwidth, precision, p);
                            } else if have_fieldwidth != 0 {
                                nw = libc::printf(f, fieldwidth, p);
                            } else if have_precision != 0 {
                                nw = libc::printf(f, precision, p);
                            } else {
                                nw = libc::printf(f, p);
                            }
                        } else {
                            let vbsnprintf2 = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf1 = |x: c_int| {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = vbsnprintf2();
                            } else if have_fieldwidth != 0 {
                                nw = vbsnprintf1(fieldwidth);
                            } else if have_precision != 0 {
                                nw = vbsnprintf1(precision);
                            } else {
                                nw = vbsnprintf();
                            }
                        }
                        tw += nw as c_long;
                    };
                    PF();
                    QUIT();
                    if libc::ferror(stdout) != 0 {
                        r_sh_wrerror();
                        libc::clearerr(stdout);
                        return EXECUTION_FAILURE;
                    }
                }
                b's' => {
                    let p = getstr();
                    let f = start;
                    libc::clearerr(stdout);
                    let PF = || {
                        let nw: c_int;
                        if vflag == 0 {
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = libc::printf(f, fieldwidth, precision, p);
                            } else if have_fieldwidth != 0 {
                                nw = libc::printf(f, fieldwidth, p);
                            } else if have_precision != 0 {
                                nw = libc::printf(f, precision, p);
                            } else {
                                nw = libc::printf(f, p);
                            }
                        } else {
                            let vbsnprintf2 = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf1 = |x: c_int| {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = vbsnprintf2();
                            } else if have_fieldwidth != 0 {
                                nw = vbsnprintf1(fieldwidth);
                            } else if have_precision != 0 {
                                nw = vbsnprintf1(precision);
                            } else {
                                nw = vbsnprintf();
                            }
                        }
                        tw += nw as c_long;
                    };
                    PF();
                    QUIT();
                    if libc::ferror(stdout) != 0 {
                        r_sh_wrerror();
                        libc::clearerr(stdout);
                        return EXECUTION_FAILURE;
                    }
                }
                b'(' => {
                    *((modstart as usize + 1) as *mut c_char) = nextch;
                    let timefmt = xmalloc(libc::strlen(fmt) + 3) as *mut c_char;
                    fmt = (fmt as usize + 1) as *mut c_char;
                    let mut t = timefmt;
                    let mut n = 1;
                    while *fmt != 0 {
                        if *fmt == b'(' as c_char {
                            n += 1;
                        } else if *fmt == b')' as c_char {
                            n -= 1;
                        }
                        if n == 0 {
                            break;
                        }
                        *t = *fmt;
                        t = (t as usize + 1) as *mut c_char;
                        fmt = (fmt as usize + 1) as *mut c_char;
                    }
                    *t = b'\0' as c_char;
                    fmt = (fmt as usize + 1) as *mut c_char;
                    if *fmt != b'T' as c_char {
                        builtin_warning("`%c': invalid time format specification\0".as_ptr() as *const c_char, *fmt as c_int);
                        fmt = start;
                        libc::free(timefmt as *mut c_void);
                        PC(*fmt as u8);

                        fmt = (fmt as usize + 1) as *mut c_char;
                        continue;
                    }
                    if *timefmt == b'\0' as c_char {
                        *timefmt = b'%' as c_char;
                        *((timefmt as usize + 1) as *mut c_char) = b'X' as c_char;
                        *((timefmt as usize + 2) as *mut c_char) = b'\0' as c_char;
                    }

                    let arg = if !garglist.is_null() {getintmax()} else {-1};
                    let mut secs: libc::time_t;
                    if arg == -1 {
                        secs = libc::time(0 as *mut libc::time_t);
                    } else if arg == -2 {
                        secs = shell_start_time;
                    } else {
                        secs = arg;
                    }

                    sv_tz("TZ\0".as_ptr() as *mut c_char);
                    let mut tm = libc::localtime(std::mem::transmute(&secs));
                    if tm.is_null() {
                        secs = 0;
                        tm = libc::localtime(std::mem::transmute(&secs));
                    }
                    let mut timebuf:[c_char; 128] = [0; 128];
                    let mut n: c_int = if !tm.is_null() {strftime(timebuf.as_ptr() as *mut c_char, 128, timefmt, tm) as c_int} else {0};
                    libc::free(timefmt as *mut c_void);
                    if n == 0 {
                        timebuf[0] = b'\0' as c_char;
                    } else {
                        timebuf[127] = b'\0' as c_char;
                    }

                    *modstart = b's' as c_char;
                    *((modstart as usize + 1) as *mut c_char) = b'\0' as c_char;
                    n = printstr(start, timebuf.as_ptr() as *mut c_char, libc::strlen(timebuf.as_ptr()) as c_int, fieldwidth, precision);
                    if n < 0 {
                        if libc::ferror(stdout) == 0 {
                            r_sh_wrerror();
                            libc::clearerr(stdout);
                        }
                        return PRETURN(EXECUTION_FAILURE);
                    }
                }
                b'n' => {
                    let var = getstr();
                    if !var.is_null() && *var != 0 {
                        if legal_identifier(var) != 0 {
                            bind_var_to_int(var, tw);
                        } else {
                            r_sh_invalidid(var);
                            return PRETURN(EXECUTION_FAILURE);
                        }
                    }
                }
                b'b' => {
                    let mut rlen: c_int = 0;
                    let mut r: c_int = 0;
                    ch = 0;
                    let p = getstr();
                    let xp = bexpand(p, libc::strlen(p) as c_int, std::mem::transmute(&ch), std::mem::transmute(&rlen));
                    if !xp.is_null() {
                        r = printstr(start, xp, rlen, fieldwidth, precision);
                        if r < 0 {
                            if libc::ferror(stdout) == 0 {
                                r_sh_wrerror();
                                libc::clearerr(stdout);
                            }
                            retval = EXECUTION_FAILURE;
                        }
                        libc::free(xp as *mut c_void);
                    }
                    if ch != 0 || r < 0 {
                        return PRETURN(retval);
                    }
                }
                b'q' => {
                    let mut r: c_int = 0;
                    let xp: *mut c_char;
                    let p = getstr();
                    if !p.is_null() && *p == 0 {
                        xp = r_savestring(b"''\0".as_ptr() as *mut c_char);
                        //xp = savestring(b"''\0".as_ptr() as *const c_char);
                    } else if ansic_shouldquote(p) != 0 {
                        xp = ansic_quote(p, 0, PT_NULL as *mut c_int);
                    } else {
                        xp = sh_backslash_quote(p, PT_NULL as *mut c_char, 3);
                    }
                    if !xp.is_null() {
                        r = printstr(start, xp, libc::strlen(xp) as c_int, fieldwidth, precision);
                        if r < 0 {
                            if libc::ferror(stdout) == 0 {
                                r_sh_wrerror();
                                libc::clearerr(stdout);
                            }
                            libc::free(xp as *mut c_void);
                        }
                    }
                    if r < 0 {
                        return PRETURN(EXECUTION_FAILURE);
                    }
                }
                b'd' | b'i' => {
                    let f = mklong(start, "l\0".as_ptr() as *mut c_char, 1);
                    let p = getintmax();
                    libc::clearerr(stdout);
                    let PF = || {
                        let nw: c_int;
                        if vflag == 0 {
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = libc::printf(f, fieldwidth, precision, p);
                            } else if have_fieldwidth != 0 {
                                nw = libc::printf(f, fieldwidth, p);
                            } else if have_precision != 0 {
                                nw = libc::printf(f, precision, p);
                            } else {
                                nw = libc::printf(f, p);
                            }
                        } else {
                            let vbsnprintf2 = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf1 = |x: c_int| {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = vbsnprintf2();
                            } else if have_fieldwidth != 0 {
                                nw = vbsnprintf1(fieldwidth);
                            } else if have_precision != 0 {
                                nw = vbsnprintf1(precision);
                            } else {
                                nw = vbsnprintf();
                            }
                        }
                        tw += nw as c_long;
                    };
                    PF();
                    QUIT();
                    if libc::ferror(stdout) != 0 {
                        r_sh_wrerror();
                        libc::clearerr(stdout);
                        return EXECUTION_FAILURE;
                    }
                }
                b'o' | b'u' | b'x' | b'X' => {
                    let f = mklong(start, "l\0".as_ptr() as *mut c_char, 1);
                    let p = getuintmax();
                    libc::clearerr(stdout);
                    let PF = || {
                        let nw: c_int;
                        if vflag == 0 {
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = libc::printf(f, fieldwidth, precision, p);
                            } else if have_fieldwidth != 0 {
                                nw = libc::printf(f, fieldwidth, p);
                            } else if have_precision != 0 {
                                nw = libc::printf(f, precision, p);
                            } else {
                                nw = libc::printf(f, p);
                            }
                        } else {
                            let vbsnprintf2 = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf1 = |x: c_int| {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = vbsnprintf2();
                            } else if have_fieldwidth != 0 {
                                nw = vbsnprintf1(fieldwidth);
                            } else if have_precision != 0 {
                                nw = vbsnprintf1(precision);
                            } else {
                                nw = vbsnprintf();
                            }
                        }
                        tw += nw as c_long;
                    };
                    PF();
                    QUIT();
                    if libc::ferror(stdout) != 0 {
                        r_sh_wrerror();
                        libc::clearerr(stdout);
                        return EXECUTION_FAILURE;
                    }
                }
                b'e' | b'E' | b'f' | b'F' | b'g' | b'G' | b'a' | b'A' => {
                    let p = getfloatmax();
                    let f = mklong(start, "l\0".as_ptr() as *mut c_char, 1);
                    libc::clearerr(stdout);
                    let PF = || {
                        let nw: c_int;
                        if vflag == 0 {
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = libc::printf(f, fieldwidth, precision, p);
                            } else if have_fieldwidth != 0 {
                                nw = libc::printf(f, fieldwidth, p);
                            } else if have_precision != 0 {
                                nw = libc::printf(f, precision, p);
                            } else {
                                nw = libc::printf(f, p);
                            }
                        } else {
                            let vbsnprintf2 = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, have_fieldwidth, have_precision, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf1 = |x: c_int| {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, x, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            let vbsnprintf = || {
                                let mut blen: c_int;
                                blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                let nlen: size_t = vblen as size_t + blen as size_t + 1;
                                if nlen > vbsize {
                                    vbsize = ((nlen as size_t + 63) >> 6) << 6;
                                    vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
                                    blen = libc::snprintf((vbuf as usize + vblen as usize) as *mut c_char, vbsize - vblen as usize, f, p);
                                }
                                vblen += blen;
                                *((vbuf as usize + vblen as usize) as *mut c_char) = b'0' as c_char;
                                blen
                            };
                            if have_fieldwidth != 0 && have_precision != 0 {
                                nw = vbsnprintf2();
                            } else if have_fieldwidth != 0 {
                                nw = vbsnprintf1(fieldwidth);
                            } else if have_precision != 0 {
                                nw = vbsnprintf1(precision);
                            } else {
                                nw = vbsnprintf();
                            }
                        }
                        tw += nw as c_long;
                    };
                    PF();
                    QUIT();
                    if libc::ferror(stdout) != 0 {
                        r_sh_wrerror();
                        libc::clearerr(stdout);
                        return EXECUTION_FAILURE;
                    }
                }
                _ => {
                    builtin_error("`%c': invalid format character\0".as_ptr() as *const c_char, convch as c_int);
                    return PRETURN(EXECUTION_FAILURE);
                }
            }

            *modstart = thisch;
            *((modstart as usize + 1) as *mut c_char) = nextch;

            fmt = (fmt as usize + 1) as *mut c_char;
        }

        if libc::ferror(stdout) != 0 {
            return PRETURN(EXECUTION_FAILURE);
        }

        if !garglist.is_null() && garglist != (*list).next {
            continue 'outer;
        } else {
            break 'outer;
        }
    }

    if conversion_error != 0 {
        retval = EXECUTION_FAILURE;
    }

    return PRETURN(retval);
}
}

fn hexvalue(c: u8) -> c_int
{
    return (
    if (c) >= b'a' && (c) <= b'f' {
        (c) - b'a' + 10
    } else if (c) >= b'A' && (c) <= b'F' {
            (c) - b'A' + 10
    } else {
        (c) - b'0'
    }) as i32
}

unsafe fn printstr(mut fmt: *mut c_char, mut string: *mut c_char, len: c_int, fieldwidth: c_int, precision: c_int) -> c_int
{
    if string.is_null() {
        string = "\0".as_ptr() as *mut c_char;
    }

    if *fmt == b'%' as c_char {
        fmt = (fmt as usize + 1) as *mut c_char;
    }

    let mut ljust: c_int = 0;
    let mut fw: c_int = 0;
    let mut mfw: c_long = 0;
    let mut pr: c_int = -1;
    let mut mpr: c_long = -1;

    while !(strchr("#'-+ 0\0".as_ptr() as *const c_char, *fmt as c_int).is_null()) {
        if *fmt == b'-' as c_char {
            ljust = 1;
        }
        fmt = (fmt as usize + 1) as *mut c_char;
    }

    if *fmt == b'*' as c_char {
        fmt = (fmt as usize + 1) as *mut c_char;
        fw = fieldwidth;
        if fw < 0 {
            fw = -fw;
            ljust = 1;
        }
    } else if IS_DIGITAL!(*fmt) {
        mfw = (*fmt - b'0' as c_char) as c_long;
        fmt = (fmt as usize + 1) as *mut c_char;
        while IS_DIGITAL!(*fmt) {
            mfw = mfw * 10 + (*fmt - b'0' as c_char) as c_long;
            fmt = (fmt as usize + 1) as *mut c_char;
        }
        fw = if mfw < 0 || mfw > (libc::INT_MAX as c_long) {libc::INT_MAX} else {mfw as c_int};
    }

    if *fmt == b'.' as c_char {
        fmt = (fmt as usize + 1) as *mut c_char;
        if *fmt == b'*' as c_char {
            fmt = (fmt as usize + 1) as *mut c_char;
            pr = precision;
        } else if IS_DIGITAL!(*fmt) {
            mpr = (*fmt - b'0' as c_char) as c_long;
            fmt = (fmt as usize + 1) as *mut c_char;
            while IS_DIGITAL!(*fmt) {
                mpr = mpr * 10 + (*fmt - b'0' as c_char) as c_long;
                fmt = (fmt as usize + 1) as *mut c_char;
            }
            pr = if mpr < 0 || mpr > (libc::INT_MAX as c_long) {libc::INT_MAX} else {mpr as c_int};
        } else {
            pr = 0;
        }
    }

    let nc = if pr >= 0 && pr <= len {pr} else {len};
    let mut padlen = fw - nc;
    if padlen < 0 {
        padlen = 0;
    }
    if ljust != 0 {
        padlen = -padlen;
    }

    while padlen > 0 {
        PC(b' ');
        padlen -= 1;
    }

    for i in 0..nc {
        PC(*((string as usize + i as usize) as *mut c_char) as u8);
    }

    return 0;
}

unsafe fn tescape(estart: *mut c_char, cp:*mut c_char, lenp: *mut c_int, sawc: *mut c_int) -> c_int
{

    let mut p: *mut c_char = estart;
    let mut evalue: c_int;
    let mut temp: c_int;
    let mut uvalue: c_ulong;
    if !lenp.is_null() {
        *lenp = 1;
    }

    let c = *p as u8;
    p = (p as usize + 1) as *mut c_char;
    match c {
        b'a' => *cp = 7,
        b'b' => *cp = 8,
        b'e' | b'E' => *cp = 27,
        b'f' => *cp = 12,
        b'n' => *cp = 10,
        b'r' => *cp = 13,
        b't' => *cp = 9,
        b'v' => *cp = 11,
        b'0'..=b'7' => {
            evalue = (c - b'0') as c_int;
            temp = 2 + (evalue == 0 && !sawc.is_null()) as c_int;
            while *p >= b'0' as c_char && *p <= b'7' as c_char && temp != 0 {
                temp -= 1;
                evalue = (evalue * 8) + (*p - b'0' as c_char) as c_int;
                p = (p as usize + 1) as *mut c_char;
            }

            *cp = (evalue & 0xff) as c_char;
        }
        b'x' => {
            temp = 2;
            evalue = 0;
            while libc::isdigit(*p as c_int) != 0 && temp != 0 {
                temp -= 1;
                evalue = (evalue * 16) + hexvalue(*p as u8);
                p = (p as usize + 1) as *mut c_char;
            }

            if p as usize == (estart as usize + 1) {
                builtin_error("missing hex digit for \\x\0".as_ptr() as *const c_char);
                *cp = b'\\' as c_char;
                return 0;
            }
            *cp = (evalue & 0xff) as c_char;
        }
        b'u' | b'U' => {
            temp = if c == b'u' {4} else {8};
            uvalue = 0;
            while libc::isdigit(*p as c_int) != 0 && temp != 0 {
                temp -= 1;
                uvalue = (uvalue * 16) + hexvalue(*p as u8) as c_ulong;
                p = (p as usize + 1) as *mut c_char;
            }

            if p as usize == (estart as usize + 1) {
                builtin_error("missing unicode digit for \\%c\0".as_ptr() as *const c_char, c as c_int);
                *cp = b'\\' as c_char;
                return 0;
            }
            if uvalue <= 0x7f {
                *cp = uvalue as c_char;
            } else {
                temp = u32cconv(uvalue, cp);
                *((cp as usize + temp as usize) as *mut c_char) = b'\0' as c_char;
                if !lenp.is_null() {
                    *lenp = temp;
                }
            }
        }
        b'\\' => *cp = c as c_char,
        b'\'' | b'"' | b'?' => {
            if sawc.is_null() {
                *cp = c as c_char;
            } else {
                *cp = b'\\' as c_char;
                return 0;
            }
        }
        b'c' => {
            if sawc.is_null() {
                *sawc = 1;
            } else {
                *cp = b'\\' as c_char;
                return 0;
            }
        }
        _ => {
            *cp = b'\\' as c_char;
            return 0;
        }
    }
    return (p as usize - estart as usize) as c_int;
}

unsafe fn bexpand(string: *mut c_char, len: c_int, sawc: *mut c_int, lenp: *mut c_int) -> *mut c_char
{
    let mut mbch:[c_char; 25];
    let mut mblen: c_int = 0;

    let mut ret: *mut c_char;
    let mut r: *mut c_char;
    let mut s: *mut c_char;
    let mut c: c_char;

    if string.is_null() || len == 0 {
        if !sawc.is_null() {
            *sawc = 0;
        }
        if !lenp.is_null() {
            *lenp = 0;
        }
        ret = xmalloc(1) as *mut c_char;
        *ret = b'\0' as c_char;
        return ret;
    }

    ret = xmalloc(len as size_t + 1) as *mut c_char;
    r = ret;
    s = string;
    while !s.is_null() && *s != 0 {
        c = *s as c_char;
        s = (s as usize + 1) as *mut c_char;
        if c != b'\\' as c_char || *s == b'\0' as c_char {
            *r = c;
            r = (r as usize + 1) as *mut c_char;
            continue;
        }

        let mut temp: c_int = 0;
        mbch = [0; 25];
        let n = tescape(s, mbch.as_mut_ptr() as *mut c_char,
        std::mem::transmute(&mblen), std::mem::transmute(&temp));
        s = (s as usize + n as size_t) as *mut c_char;

        if temp != 0 {
            if !sawc.is_null() {
                *sawc = 1;
            }
            break;
        }

        for mbind in 0..mblen {
            *r = mbch[mbind as usize];
            r = (r as usize + 1) as *mut c_char;
        }
    }

    *r = b'\0' as c_char;
    if !lenp.is_null() {
        *lenp = (r as usize - ret as usize) as c_int;
    }

    return ret;
}

unsafe fn vbadd(buf: *mut c_char, blen: c_int) -> *mut c_char
{
    let nlen: size_t = vblen as size_t + blen as size_t + 1;
    if nlen >= vbsize {
        vbsize = ((nlen + 63) >> 6) << 6;
        vbuf = xrealloc(vbuf as *mut c_void, vbsize) as *mut c_char;
    }

    if blen == 1 {
        *((vbuf as usize + vblen as usize) as *mut c_char) = *buf;
        vblen += 1;
    } else if blen > 1 {
        libc::memcpy((vbuf as usize + vblen as usize) as *mut c_void,
        buf as *mut c_void, blen as size_t);
        vblen += blen;
    }
    *((vbuf as usize + vblen as usize) as *mut c_char) = b'\0' as c_char;

    return vbuf;
}

unsafe fn printf_erange(s: *mut c_char)
{
    builtin_error("warning: %s: %s\0".as_ptr() as *const c_char, s, libc::strerror(libc::ERANGE));
}

unsafe fn mklong(str: *mut c_char, modifiers: *mut c_char, mlen: size_t) -> *mut c_char
{
    let slen = libc::strlen(str);
    let len = slen + mlen + 1;

    if len > conv_bufsize {
        conv_bufsize = ((len + 1023) >> 10) << 10;
        conv_buf = libc::realloc(conv_buf as *mut c_void, conv_bufsize) as *mut c_char;
    }

    libc::memcpy(conv_buf as *mut c_void, str as *mut c_void, slen - 1);
    libc::memcpy((conv_buf as usize + slen - 1) as *mut c_void, modifiers as *mut c_void, mlen);

    *((conv_buf as usize + len - 2) as *mut c_char) = *((str as usize + slen - 1) as *mut c_char);
    *((conv_buf as usize + len - 1) as *mut c_char) = b'\0' as c_char;

    return conv_buf;
}

unsafe fn getchr() -> c_int
{
    if garglist.is_null() {
        return b'\0' as c_int;
    }

    let ret = *(*(*garglist).word).word as c_int;
    garglist = (*garglist).next;
    return ret;
}

unsafe fn getstr() -> *mut c_char
{
    if garglist.is_null() {
        return "\0".as_ptr() as *mut c_char;
    }

    let ret = (*(*garglist).word).word;
    garglist = (*garglist).next;
    return ret;
}

unsafe fn getint() -> c_int
{
    let mut ret = getintmax();

    if garglist.is_null() {
        return ret as c_int;
    }

    if ret > libc::INT_MAX as c_long{
        printf_erange((*(*garglist).word).word);
        ret = libc::INT_MAX as c_long;
    } else if ret < libc::INT_MIN as c_long{
        printf_erange((*(*garglist).word).word);
        ret = libc::INT_MIN as c_long;
    }

    return ret as c_int;
}

unsafe fn getintmax() -> c_long
{
    if garglist.is_null() {
        return 0;
    }

    if *(*(*garglist).word).word == b'\'' as c_char ||
       *(*(*garglist).word).word == b'"' as c_char {
        return asciicode();
    }

    let mut ep: *mut c_char = PT_NULL as *mut c_char;
    *libc::__errno_location() = 0;
    let ret = libc::strtol((*(*garglist).word).word, std::mem::transmute(&ep), 0);
    if *ep != 0 {
        sh_invalidnum((*(*garglist).word).word);
        conversion_error = 1;
    } else if *libc::__errno_location() == libc::ERANGE {
         printf_erange((*(*garglist).word).word);
    }
    garglist = (*garglist).next;
    return ret;
}

unsafe fn getuintmax() -> c_ulong
{
    if garglist.is_null() {
        return 0;
    }

    if *(*(*garglist).word).word == b'\'' as c_char ||
       *(*(*garglist).word).word == b'"' as c_char {
        return asciicode() as c_ulong;
    }

    *libc::__errno_location() = 0;
    let mut ep: *mut c_char = PT_NULL as *mut c_char;
    let ret = libc::strtoul((*(*garglist).word).word, std::mem::transmute(&ep), 0);
    if *ep != 0 {
        sh_invalidnum((*(*garglist).word).word);
        conversion_error = 1;
    } else if *libc::__errno_location() == libc::ERANGE {
         printf_erange((*(*garglist).word).word);
    }
    garglist = (*garglist).next;
    return ret;
}

unsafe fn getfloatmax() -> f64
{
    let ep: *mut c_char = PT_NULL as *mut c_char;

    if garglist.is_null() {
        return 0.0;
    }

    if *(*(*garglist).word).word == b'\'' as c_char ||
       *(*(*garglist).word).word == b'\"' as c_char {
           return asciicode() as f64;
    }

    *libc::__errno_location() = 0;
    let ret = libc::strtod((*(*garglist).word).word, std::mem::transmute(&ep));
    if *ep != 0 {
        sh_invalidnum((*(*garglist).word).word);
        conversion_error = 1;
    } else if *libc::__errno_location() == libc::ERANGE {
        printf_erange((*(*garglist).word).word);
    }

    garglist = (*garglist).next;

    return ret;
}

unsafe fn asciicode() -> c_long
{
    let ch: c_long;
    //let state: mbstate_t = std::mem::zeroed();
    let slen = libc::strlen((*(*garglist).word).word);
    let wc: libc::wchar_t = 0;
    let mblength = mbtowc(std::mem::transmute(&wc), ((*(*garglist).word).word as usize + 1) as *mut c_char, slen);
    if mblength > 0 {
        ch = wc as c_long;
    } else {
        ch = *(((*(*garglist).word).word as usize + 1) as *mut c_char) as c_long;
    }

    garglist = (*garglist).next;

    return ch;
}

