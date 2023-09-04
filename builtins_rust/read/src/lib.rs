use libc::{c_int, c_char, c_long, c_ulong, c_uint, size_t, c_void, PT_NULL, ssize_t};
use nix::errno::errno;
use std::{ffi::{CString, CStr}, ptr::null_mut,};
//use rcommon::{r_builtin_usage,r_sh_invalidid,r_builtin_bind_variable,SHELL_VAR};

include!(concat!("intercdep.rs"));

#[no_mangle]
pub static mut alrmbuf:sigjmp_buf = [__jmp_buf_tag{
    __jmpbuf:[0;8],
    __mask_was_saved:0,
    __saved_mask:__sigset_t{__val:[0;16]},
};1];

static mut old_alrm : *mut SigHandler = PT_NULL as *mut SigHandler;

// static mut sigalrm_seen : c_int = 0;
static mut reading : c_int = 0;
static mut tty_modified : c_int = 0;

static mut delim : c_char= b'\n' as c_char;
#[derive(Clone, Copy)]
pub struct tty_save {
    fd: i32,
    attrs: libc::termios,
}

// static mut termsave : Option<tty_save> =  None;
static mut termsave:tty_save = tty_save{
    fd:0,
    attrs:libc::termios {
         c_iflag: (0),
         c_oflag: (0),
         c_cflag: (0), 
         c_lflag: (0), 
         c_line: (0), 
         c_cc: [0;32], 
         c_ispeed: (0), 
         c_ospeed: (0) 
    }
};

static mut interactive : c_int = 0;
static mut default_buffered_input : c_int = -1;


#[no_mangle]
pub extern "C" fn r_read_builtin(mut list: *mut WordList) -> i32 {

    let mut varname :*mut c_char = libc::PT_NULL as *mut c_char;
    let mut size : c_int = 0;
    let mut nr : c_int = 0;
    let mut pass_next : c_int = 0;
    let mut saw_escape : c_int = 0;
    let mut eof : c_int;
    let mut opt : c_int;
    let mut retval : c_int;
    let mut code : c_int;
    let mut print_ps2 : c_int;
    let mut nflag : c_int = 0;

    let mut i : c_int = 0;

    let mut input_is_tty : c_int = 0;
    let mut input_is_pipe : c_int = 0;
    let mut unbuffered_read : c_int = 0;
    let mut skip_ctlesc : c_int;
    let mut skip_ctlnul : c_int;

    let mut raw : c_int = 0;
    let mut edit : c_int = 0;
    let mut nchars : c_int = 0;
    let mut silent : c_int = 0;
    let mut have_timeout : c_int = 0;
    let mut ignore_delim : c_int = 0;
    let mut fd : c_int = 0;

    let mut lastsig : c_int = 0;
    let mut t_errno : c_int;

    let mut mb_cur_max : c_int;

    let mut tmsec : c_uint = 0;
    let mut tmusec : c_uint = 0;

    let mut ival : c_long = 0;
    let mut uval : c_long = 0;
    let mut intval : c_long = 0;

    let mut c : c_char = 0;

    let mut input_string : *mut c_char;
    let mut orig_input_string : *mut c_char;
    let ifs_chars_null = CString::new("").unwrap();
    let mut ifs_chars : *mut c_char;
    let mut prompt : *mut c_char = PT_NULL as *mut c_char;
    let mut arrayname : *mut c_char = PT_NULL as *mut c_char;

    let mut e : *mut c_char;
    let t : *mut c_char;
    let t1 : *mut c_char;
    let mut ps2 : *mut c_char;
    let mut tofree : *mut c_char;

    let mut tsb : libc::stat;

    let mut var : *mut SHELL_VAR = PT_NULL as *mut SHELL_VAR;

    let mut ttattrs : libc::termios;
    let mut ttset : libc::termios;
    unsafe {
        ttattrs = std::mem::zeroed();
        ttset = std::mem::zeroed();
    }

    let mut alist : *mut WordList;

    let mut vflags : c_int;
    let mut rlbuf : *mut c_char = null_mut();
    let mut itext : *mut c_char = null_mut();

    let mut rlind : c_int = 0;


    let mut save_instream : *mut libc::FILE;

    let mut mb_cur_max : c_int =  1;

unsafe {

    // if termsave.is_none() {
    //     let tmp: tty_save  = std::mem::zeroed();
    //     termsave = Some(tmp);
    // }
    // ptermsave = std::mem::transmute(&termsave.unwrap());

    reset_internal_getopt();
    let opt_str = CString::new("ersa:d:i:n:p:t:u:N:").unwrap();
    opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'r' => raw = 1,
            'p' => prompt = list_optarg,
            's' => silent = 1,
            'e' => edit = 1,
            'i' => itext = list_optarg,
            'a' => arrayname = list_optarg,
            't' => {
                code = uconvert(list_optarg, &mut ival, &mut uval, PT_NULL as *mut *mut c_char);
                if code == 0 || ival < 0 || uval < 0 {
                    let c_err = CString::new("%s: invalid timeout specification").unwrap();
                    builtin_error( c_err.as_ptr(), list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    have_timeout = 1;
                    tmsec = ival as c_uint;
                    tmusec = uval as c_uint;
                }
            }
            'N' | 'n' => {
                if opt_char == 'N' {
                    ignore_delim = 1;
                    delim = 255 as u8 as libc::c_char;
                }
                nflag = 1;
                code = legal_number(list_optarg, &mut intval);
                if code == 0 || intval < 0 || intval != (intval as c_int) as c_long {
                    sh_invalidnum(list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    nchars = intval as c_int;
                }
            }
            'u' => {
                code = legal_number(list_optarg, &mut intval);
                if code == 0 || intval < 0 || intval != (intval as c_int) as c_long {
                    let c_err = CString::new("%s: invalid file descriptor specification").unwrap();
                    builtin_error(c_err.as_ptr(), list_optarg);
                    return EXECUTION_FAILURE;
                } else {
                    fd = intval as c_int;
                }
                if sh_validfd(fd) == 0 {
                    let c_err = CString::new("%d: invalid file descriptor: %s").unwrap();
                    builtin_error(c_err.as_ptr(), fd, libc::strerror(nix::errno::errno()));
                    return EXECUTION_FAILURE;
                }
            }
            'd' => {
                    delim = *list_optarg;
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

    //-t
    if have_timeout != 0 && tmsec == 0 && tmusec == 0 {
        return if input_avail(fd) != 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
    }

    vflags = if assoc_expand_once != 0 {(VA_NOEXPAND | VA_ONEWORD) as c_int} else {0};
    if !list.is_null() &&
        legal_identifier((*(*list).word).word) == 0 &&
        valid_array_reference((*(*list).word).word, vflags) == 0 {
            // sh_invalidid((*(*list).word).word);
            r_sh_invalidid((*(*list).word).word);
            return EXECUTION_FAILURE;
    }

    //忽略界定符
    if ignore_delim != 0{  //-N  ignore_delim = 1
        delim = 255 as u8 as libc::c_char;
    }

    ifs_chars = getifs(); //ifs_chars is "\n"
    if ifs_chars.is_null() {
        ifs_chars = ifs_chars_null.as_ptr() as *mut c_char;
    }

    if ignore_delim != 0 {
        ifs_chars = ifs_chars_null.as_ptr() as *mut c_char;
    }

    skip_ctlesc = 0;
    skip_ctlnul = 0;
    e = ifs_chars;

    loop {
        if *e == 0 {
            break;
        }
        skip_ctlesc |= (*e == 1) as c_int;
        skip_ctlnul |= (*e == 117) as c_int;
        e = ((e as usize) + 1) as *mut c_char;
    }

    input_string = xmalloc(112) as *mut c_char;
    *input_string = b'\0' as c_char;

'out_assig_vars: loop {
    if nflag == 1 && nchars == 0 {
        let mut gc : c_int = 0;
        retval = libc::read(fd, &mut gc as *mut i32 as *mut c_void, 0) as c_int;
        retval = if retval >= 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};

        break 'out_assig_vars;
    }

    //设置TMOUT后，TMOUT是默认读取时间
    let str_val = CString::new("TMOUT").unwrap();
    e = get_string_value(str_val.as_ptr());
    if have_timeout == 0 && !e.is_null() {
        code = uconvert(e, &mut ival, &mut uval, 0 as *mut *mut c_char);
        if code == 0 || ival < 0 || uval < 0 {
            tmsec = 0;
            tmusec = 0;
        } else {
            tmsec = ival as c_uint;
            tmusec = uval as c_uint;
        }
    }

    let frame_name = CString::new("r_read_builtin").unwrap();    //有没有可能是r_read_builtin?
    begin_unwind_frame(frame_name.as_ptr() as *mut c_char);

    
    if interactive == 0 && default_buffered_input >= 0 && fd_is_bash_input(fd) != 0 {
        sync_buffered_stream(default_buffered_input);
    }

    input_is_tty = libc::isatty(fd);
    if input_is_tty == 0 {
        input_is_pipe = (libc::lseek(fd, 0, libc::SEEK_CUR) < 0 && (errno() == libc::ESPIPE)) as c_int;
    }

    //如果设置 -p,-e,-s但输入不是终端，忽略
    if (!prompt.is_null() || edit != 0 || silent != 0) && input_is_tty == 0 {
        itext = PT_NULL as *mut c_char;
        edit = 0;
        silent = 0;
    }

    if edit != 0 {
        add_unwind_protect(xfree as *mut c_void, rlbuf);
    }

    tsb = std::mem::zeroed();
    if tmsec > 0 || tmusec > 0 {
        if (libc::fstat(fd, &mut tsb as *mut libc::stat) < 0) ||
            ((tsb.st_mode & __S_IFMT) == __S_IFREG) {
            tmsec = 0;
            tmusec = 0;
        }
    }

    if tmsec > 0 || tmusec > 0 {
        code = __sigsetjmp(&mut alrmbuf as *mut __jmp_buf_tag, 0);
        if code != 0 {
            sigalrm_seen = 0;
            orig_input_string = PT_NULL as *mut c_char;
            *input_string.offset(i as isize) = b'\0' as c_char;
            if i == 0 {
                t = libc::malloc(1) as *mut c_char;
                *t = b'\0' as c_char;
            } else {
                t = libc::strcpy( xmalloc(
                    (libc::strlen(input_string) + 1) as size_t) as *mut c_char, input_string);
            }
            run_unwind_frame(frame_name.as_ptr() as *mut c_char);
            input_string = t;
            retval = 128 + libc::SIGALRM;
            break 'out_assig_vars;
        }

        if interactive_shell == 0 {
            initialize_terminating_signals();
        }

        old_alrm = set_signal_handler(libc::SIGALRM, sigalrm as *mut SigHandler);
        add_unwind_protect(reset_alarm as *mut c_void, PT_NULL as *mut c_char);

        if edit != 0 {
            add_unwind_protect(reset_attempted_completion_function as *mut c_void,
                PT_NULL as *mut c_char);
            add_unwind_protect(bashline_reset_event_hook as *mut c_void,
                PT_NULL as *mut c_char);
        }
        falarm(tmsec, tmusec);
    }

    if nchars > 0 || delim != b'\n' as c_char { //-d -n
        if edit != 0 {
            if nchars > 0 {
                unwind_protect_mem(&mut rl_num_chars_to_read as *mut c_int, std::mem::size_of_val(&rl_num_chars_to_read) as c_int);
                rl_num_chars_to_read = nchars;
            }

            if delim != b'\n' as c_char {
                set_eol_delim(delim as c_int);
                add_unwind_protect(reset_eol_delim as *mut c_void, PT_NULL as *mut c_char);
            }
        } else if input_is_tty != 0 { //-d  -n
            // termsave.unwrap().fd = fd;
            termsave.fd = fd;
            ttgetattr(fd, &mut ttattrs as *mut libc::termios);
            // termsave.unwrap().attrs = ttattrs;
            termsave.attrs = ttattrs;

            ttset = ttattrs;
            if silent != 0 {
                i = ttfd_cbreak(fd, std::mem::transmute(&ttset));
            } else {
                i = ttfd_onechar(fd, std::mem::transmute(&ttset));
            }

            if i < 0 {
                sh_ttyerror(1);
            }
            tty_modified = 1;
            // add_unwind_protect(ttyrestore as *mut c_void, ptermsave);
            add_unwind_protect(ttyrestore as *mut c_void, &mut termsave);
            if interactive_shell == 0 {
                initialize_terminating_signals();
            }

        }
    } else if silent != 0 { //-s
        // termsave.unwrap().fd = fd;
        termsave.fd = fd;
        ttgetattr(fd, &mut ttattrs as *mut libc::termios);
        // termsave.unwrap().attrs = ttattrs;
        termsave.attrs = ttattrs;

        ttset = ttattrs;
        i = ttfd_noecho(fd, std::mem::transmute(&ttset));
        if i < 0 {
            sh_ttyerror(1);
        }

        tty_modified = 1;
        // add_unwind_protect(ttyrestore as *mut c_void, ptermsave);
        add_unwind_protect(ttyrestore as *mut c_void, &mut termsave );
        if interactive_shell == 0 {
            initialize_terminating_signals();
        }
    }

    save_instream = std::mem::zeroed();
    if edit != 0  && fd != 0 {

        if bash_readline_initialized == 0 {
            initialize_readline();
        }

        unwind_protect_mem(std::mem::transmute(rl_instream), std::mem::size_of_val(&rl_instream) as c_int);
        save_instream = rl_instream;
        rl_instream = libc::fdopen(fd, "r".as_ptr() as *const c_char);

    }

    add_unwind_protect(xfree as *mut c_void, input_string);
    check_alrm();
    if nchars > 0 && input_is_tty == 0 && ignore_delim != 0 {
        unbuffered_read = 2;
    } else if nchars > 0 || delim != b'\n' as c_char || input_is_pipe != 0 {
        unbuffered_read = 1;
    }

    if !prompt.is_null() && edit == 0 {  //-p no -e
        // eprintln!("{}", CStr::from_ptr(prompt).to_str().unwrap());
        eprint!("{}", CStr::from_ptr(prompt).to_str().unwrap());

    }

    ps2 = PT_NULL as *mut c_char;
    print_ps2 = 0;
    eof = 0;
    retval = 0;
    'get_input_string: loop {
        if sigalrm_seen != 0 {
            siglongjmp (std::mem::transmute(&alrmbuf), 1);
        }

        if edit != 0 {   //没有设置-e edit等于0
            if !rlbuf.is_null() &&
                *((rlbuf as usize + rlind as usize) as *mut c_char) == 0 &&
                delim != 0 {
                libc::free(rlbuf as *mut c_void);
                rlbuf = PT_NULL as *mut c_char;
            }
            if rlbuf.is_null() {
                reading = 1;
                rlbuf = if prompt.is_null() {
                            // edit_line("".as_ptr() as *mut c_char, itext)} 
                            // let c_str = b'\0';   //  b'\0'代表的是空字符串，和String::from("")不是同一个东西。
                            edit_line(b'\0' as *mut c_char,itext)    //  b'\0'代表的是空字符串，和String::from("")不是同一个东西。
                        }
                        else {
                            edit_line(prompt, itext)
                        };
                reading = 0;
                rlind = 0;
            }
            if rlbuf.is_null() {
                eof = 1;
                break 'get_input_string;
            }
            c = *((rlbuf as usize + rlind as usize) as *mut c_char);
            rlind += 1;
        } else {
            if print_ps2 != 0 {
                if ps2.is_null() {
                    ps2 = get_string_value("PS2".as_ptr() as *const c_char);
                }
                eprintln!("{}", CStr::from_ptr(prompt).to_str().unwrap());
                print_ps2 = 0;
            }

            reading = 1;
            check_alrm();
            *(libc::__errno_location()) = 0;
            if unbuffered_read == 2 {
                retval = if posixly_correct != 0 {
                            zreadintr(fd, &mut c as *mut c_char, 1) as c_int
                        }
                        else {
                            zreadn(fd, &mut c as *mut c_char, (nchars - nr) as usize) as c_int
                        };
            } else if unbuffered_read != 0 {
                retval = if posixly_correct != 0 {
                            zreadintr(fd, &mut c as *mut c_char, 1) as c_int}
                        else {
                            zread(fd, &mut c as *mut c_char, 1) as c_int};
            } 
            else {  
                retval = if posixly_correct != 0 {
                            zreadcintr(fd, &mut c as *mut c_char) as c_int
                        }
                        else { //-a  -t
                            zreadc(fd, &mut c as *mut c_char) as c_int
                        };
            }
            reading = 0;

            if retval <= 0 {
                let t = *libc::__errno_location();
                if retval < 0 && *libc::__errno_location() == libc::EINTR {
                    check_signals();
                    //lastsig = LASTSIG();
                    if terminating_signal != 0 {
                        lastsig = terminating_signal;
                    } else {
                        lastsig = if interrupt_state != 0 {libc::SIGINT} else { 0 };
                    }

                    if lastsig == 0 {
                        lastsig = trapped_signal_received;
                    }
                } else {
                    lastsig = 0;
                }
                if terminating_signal != 0 && tty_modified != 0 {
                    // ttyrestore();
                    ttyrestore(&mut termsave);
                }
                check_alrm();
                eof = 1;
                *libc::__errno_location() = t;
                break 'get_input_string;
            }

            quit();
        }

        if retval <= 0 {
            check_alrm();
        }

        if mb_cur_max <= 4 {
            mb_cur_max = 4;
        }
        if i + mb_cur_max >= size {
            size += 128;
            let t: *mut c_char= xrealloc(input_string as *mut c_void, size as usize) as *mut c_char;
            if t != input_string {
                input_string = t;
                remove_unwind_protect();
                add_unwind_protect(xfree as *mut c_void, input_string);
            }
        }
'out_add_char: loop {
        if pass_next != 0 {     
            pass_next = 0;
            if c == b'\n' as c_char {
                if skip_ctlesc == 0 && i > 0 {i -= 1;}
                if interactive != 0 && input_is_tty != 0 && raw == 0 {print_ps2 = 1;}
            } else {
                break 'out_add_char;
            }
            continue 'get_input_string;
        }

        if c == b'\\' as c_char && raw == 0 {
            pass_next += 1;
            if skip_ctlesc == 0 {
                saw_escape += 1;
                *((input_string as usize + i as usize) as *mut c_char) = CTLESC;
                i += 1;
            }

            continue 'get_input_string;
        }

        if ignore_delim == 0 && c == delim { //-a
            break 'get_input_string;
        }

        if c == b'\0' as c_char && delim != b'\0' as c_char {
            continue 'get_input_string;
        }

        if (skip_ctlesc == 0 && c == CTLESC) || (skip_ctlnul == 0 && c == CTLNUL) {
            saw_escape += 1;
            *((input_string as usize + i as usize) as *mut c_char) = CTLESC;
            i += 1;
        }
        break 'out_add_char;
    }//out_add_char

        *((input_string as usize + i as usize) as *mut c_char) = c;
        i += 1;
        check_alrm();

        if mb_cur_max > 1 && is_basic(c) == 0 {

            *((input_string as usize + i as usize) as *mut c_char) = b'\0' as c_char;

            if edit != 0 {
                let clen = mbrlen((rlbuf as usize + rlind as usize - 1) as *const c_char,
                mb_cur_max as usize,
                std::mem::transmute(&PT_NULL));
                if clen > 1 {
                    libc::memcpy( (input_string as usize + i as usize) as *mut c_void,
                    (rlbuf as usize + rlind as usize) as *mut c_void, (clen - 1) as size_t);
                    i += clen - 1;
                    rlind += clen - 1;
                }
            } else if locale_utf8locale == 0 || ((c as u8 & 0x80) != 0) {
                i += read_mbchar(fd, input_string, i, c as c_int, unbuffered_read);
            }
        }
        nr += 1;
        if nchars > 0 && nr >= nchars {
            break 'get_input_string;
        }
        
    } //get_input_string

    *((input_string as usize + i as usize) as *mut c_char) = b'\0' as c_char;
    check_alrm();

    if edit != 0 {
        libc::free(rlbuf as *mut c_void);
    }

    if retval < 0 {
        t_errno = *libc::__errno_location();
        if *libc::__errno_location() != EINTR {
            let c_err = CString::new("read error: %d: %s").unwrap();
            builtin_error( c_err.as_ptr(), fd, libc::strerror(*libc::__errno_location()));
        }

        run_unwind_frame(frame_name.as_ptr() as *mut c_char);
        return if t_errno != EINTR {EXECUTION_FAILURE} else { 128 + lastsig};
    }

    if tmsec > 0 || tmusec > 0 {
        reset_alarm();
    }

    if nchars > 0 || delim != b'\n' as c_char {
        if edit != 0 {
            if nchars > 0{
                rl_num_chars_to_read = 0;
            }
            if delim != b'\n' as c_char{
                reset_eol_delim(0 as *mut c_char);
            }
        } 
        else if input_is_tty != 0 {
            // ttyrestore();
            ttyrestore(&mut termsave);
        }
    } 
    else if silent != 0 {
        // ttyrestore();
        ttyrestore(&mut termsave);
    }

    // if unbuffered_read != 0 {
    if unbuffered_read == 0 {
        zsyncfd(fd);
    }

    if !save_instream.is_null() {
        rl_instream = save_instream;
    }

    discard_unwind_frame(frame_name.as_ptr() as *mut c_char);

    retval = if eof != 0 {EXECUTION_FAILURE} else {EXECUTION_SUCCESS};

    break 'out_assig_vars;
} //out_assig_vars   

    if !arrayname.is_null() {   //和-a有关
        if legal_identifier(arrayname) == 0 {   //标签不符合规范
            // sh_invalidid(arrayname);
			r_sh_invalidid(arrayname);
			libc::free(input_string as *mut c_void);
			return EXECUTION_FAILURE;
		}

		var = find_or_make_array_variable(arrayname, 1);
		if var.is_null() {
            libc::free(input_string as *mut c_void);
			return EXECUTION_FAILURE;	/* readonly or noassign */
		}
        if ((*var).attributes & 0x0000040) != 0  {
            let c_err = CString::new("%s: cannot convert associative to indexed array").unwrap();
			builtin_error(c_err.as_ptr(), arrayname);
            libc::free(input_string as *mut c_void);
			return EXECUTION_FAILURE;	/* existing associative array */
		} else if ((*var).attributes & 0x0001000) != 0  {
            (*var).attributes &= ((*var).attributes as u32 ^ 0xffffffff as u32) as i32;
        }

        array_flush(std::mem::transmute((*var).value));

		alist = list_string(input_string, ifs_chars, 0);
		if !alist.is_null() {
			if saw_escape != 0 {
				dequote_list(alist);
			} else {
				word_list_remove_quoted_nulls(alist);
            }
			assign_array_var_from_word_list(var, alist, 0);
			dispose_words(alist);
		}

        libc::free(input_string as *mut c_void);

		return retval;
    }

    if list.is_null() { //和-d相关  -n 0可以退出，有显示
        if saw_escape != 0 {
			let t = dequote_string(input_string);
			var = bind_variable("REPLY".as_ptr() as *const c_char, t, 0);
			libc::free(t as *mut c_void);
		} else {
			var = bind_variable("REPLY".as_ptr() as *const c_char, input_string, 0);
        }
        let cond = var.is_null() || ((*var).attributes & 0x0000002) != 0 ||  ((*var).attributes & 0x0004000) != 0; 
		if cond {
			retval = EXECUTION_FAILURE;
		} else {
            (*var).attributes &= ((*var).attributes as u32 ^ 0xffffffff as u32) as i32;
        }

        libc::free(input_string as *mut c_void);
		return retval;
    }

    orig_input_string = input_string;

    let mut t = input_string;
    while !ifs_chars.is_null() && *ifs_chars != 0 &&
        (*t == b' ' as c_char|| *t == b'\t' as c_char || *t == b'\n' as c_char) &&
        (ifs_cmap[*t as usize] != 0) {
        t = (t as usize + 1) as *mut c_char;
    }
    input_string = t;

    while !(*list).next.is_null() {
        varname = (*((*list).word)).word;

        if legal_identifier(varname) == 0 &&
            valid_array_reference(varname, vflags) == 0 {
            // sh_invalidid(varname);
            r_sh_invalidid(varname);
            libc::free(orig_input_string as *mut c_void);
			return EXECUTION_FAILURE;
        }

        if *input_string != 0 {
            t = get_word_from_string(std::mem::transmute(&input_string), ifs_chars, std::mem::transmute(&e));
            if !t.is_null() { *e = b'\0' as c_char;}

            if !t.is_null() && saw_escape != 0 {
                let t1 = dequote_string(t);
                var = bind_read_variable(varname, t1);
                libc::free(t1 as *mut c_void);
            } else {
                var = bind_read_variable(varname, if !t.is_null() {t} else {"".as_ptr() as *mut c_char});
            }
        } else {
            t = PT_NULL as *mut c_char;
			var = bind_read_variable(varname, "".as_ptr() as *mut c_char);
        }

        if !t.is_null() {
            libc::free(t as *mut c_void);
        }

        if var.is_null() {
            libc::free(orig_input_string as *mut c_void);
            return EXECUTION_FAILURE;
        }

        stupidly_hack_special_variables(varname);
        (*var).attributes &= ((*var).attributes as u32 ^ 0xffffffff as u32) as i32;

        list = (*list).next;
    }

    if legal_identifier((*((*list).word)).word) == 0 &&
        valid_array_reference((*((*list).word)).word, vflags) == 0 {
        // sh_invalidid((*((*list).word)).word);
        r_sh_invalidid((*((*list).word)).word);
        libc::free(orig_input_string as *mut c_void);
        return EXECUTION_FAILURE;
    }

    tofree = PT_NULL as *mut c_char;
    if *input_string != 0 {
        t1 = input_string;
        t = get_word_from_string(std::mem::transmute(&input_string), ifs_chars, std::mem::transmute(&e));
        if *input_string == 0 {
            input_string = t;
            tofree = input_string;
        } else {
            input_string = strip_trailing_ifs_whitespace(t1, ifs_chars, saw_escape);
            tofree = t;
        }
    }

    if saw_escape != 0 && !input_string.is_null() && *input_string != 0 {
        t = dequote_string(input_string);
        var = bind_read_variable((*((*list).word)).word, t);
        libc::free(t as *mut c_void);
    } else {
        var = bind_read_variable((*((*list).word)).word, if !input_string.is_null() {input_string} else {"".as_ptr() as *mut c_char});
    }

    if !var.is_null() {
        stupidly_hack_special_variables((*((*list).word)).word);
        (*var).attributes &= ((*var).attributes as u32 ^ 0xffffffff as u32) as i32;
    } else {
        retval = EXECUTION_FAILURE;
    }

    if !tofree.is_null() {
        libc::free(tofree as *mut c_void);
    }
    libc::free(orig_input_string as *mut c_void);
    return retval;
} //unsafe
}

/* ---------------------------------------------------------------------------------- */

// pub fn is_basic(c: i8) -> u32 {
//     let is_basic_table :[c_uint; 8] = [ 0x00001a00, 0xffffffef, 0xfffffffe, 0x7ffffffe, 0,0,0,0];

//     let index = (c >> 5) as usize;
//     return (is_basic_table[index] >> (c & 31) ) & 1;
// }

#[inline]
unsafe extern "C" fn is_basic(mut c:libc::c_char)->libc::c_int{
    return (*is_basic_table
        .as_ptr()
        .offset((c as libc::c_uchar as libc::c_int>>5 as libc::c_int)as isize)
        >>(c as libc::c_uchar as libc::c_int & 31 as libc::c_int)
        & 1 as libc::c_int as libc::c_uint) as libc::c_int;
}

pub fn bind_read_variable(name: *mut c_char, value: *mut c_char) -> * mut SHELL_VAR {
    let v: *mut SHELL_VAR;
unsafe {
    // v = builtin_bind_variable(name, value, 0);
	v = r_builtin_bind_variable(name, value, 0);

    if v.is_null() {
        return  v;
    } else {
        if ((*v).attributes & 0x0000002) != 0 || ((*v).attributes & 0x0004000) != 0 {
            return PT_NULL as *mut SHELL_VAR;
        } else {
            return v;
        }
    }
}
}

fn read_mbchar(fd: c_int, string: *mut c_char, ind: c_int, ch: c_int, unbuffered: c_int) -> c_int {
    let mut i: size_t = 1;
    let mut r: ssize_t;
	let mut c: c_char = 0;
	let mut ret: ssize_t;

unsafe {
    let mut mbchar: [c_char; MB_LEN_MAX as usize + 1] = std::mem::zeroed();
    let mut ps: mbstate_t = std::mem::zeroed();
    let mut ps_back: mbstate_t = std::mem::zeroed();
    let mut wc: libc::wchar_t =  std::mem::zeroed();

'out: loop {
	mbchar[0] = ch as c_char;
	for n in 0..= MB_LEN_MAX {
		ps_back = ps;
		ret = mbrtowc(std::mem::transmute(&wc), std::mem::transmute(&mbchar), i, std::mem::transmute(&ps)) as ssize_t;
		if ret == -2 {
			ps = ps_back;

			/* We don't want to be interrupted during a multibyte char read */
			if unbuffered == 2 {
				r = zreadn(fd, std::mem::transmute(&c), 1);
			} else if unbuffered != 0 {
				r = zread(fd, std::mem::transmute(&c), 1);
			} else {
				r = zreadc(fd, std::mem::transmute(&c));
            }
			if r <= 0 {
                break 'out;
            }
			mbchar[i] = c;
            i += 1;
			continue;
		} else if ret == -1 || ret == 0 || ret > 0 {
			break;
        }
	}
    break 'out;
}
	if i > 1 {
		r = 1;
		while r < i as isize {
            *((string as usize + ind as usize + r as usize -1) as *mut c_char) = mbchar[r as size_t];

            r += 1;
        }
    }
	return (i - 1) as c_int;
}
}

fn quit() {
unsafe {
    if terminating_signal != 0 {
        termsig_handler(terminating_signal);
    }

    if interrupt_state != 0 {
        throw_to_top_level();
    }
}
}

fn check_alrm() {
    unsafe {
        if sigalrm_seen != 0 {
            siglongjmp (std::mem::transmute(&alrmbuf), 1);
            // siglongjmp (&mut alrmbuf as *mut __jmp_buf_tag, 1);
        }
    }
}

static mut old_attempted_completion_function: usize = 0;

pub fn reset_attempted_completion_function(cp: *mut c_char)
{
unsafe {
    if rl_attempted_completion_function as usize == 0 &&
	    old_attempted_completion_function as usize != 0 {
        rl_attempted_completion_function = std::mem::transmute(old_attempted_completion_function);
    }
}
}

static mut old_startup_hook: usize = 0;
static mut deftext: *mut c_char = PT_NULL as *mut c_char;

fn set_itext() -> c_int
{
    let mut r1 = 0;
    let mut r2 = 0;

unsafe {
	if old_startup_hook != 0 {
        let fp: rl_hook_func_t = std::mem::transmute(old_startup_hook);
		r1 = fp();
    }
	if !deftext.is_null() {
		r2 = rl_insert_text(deftext as *const c_char);
		deftext = PT_NULL as *mut c_char;
		rl_startup_hook = std::mem::transmute(old_startup_hook);
		old_startup_hook = std::mem::transmute(0 as usize);
	}
}
	return (r1 != 0 || r2 != 0) as c_int;
}

// fn edit_line(p : *mut c_char, itext : *mut c_char) -> *mut c_char {
// unsafe {
// 	if bash_readline_initialized == 0 {
// 		initialize_readline();
//     }

// 	old_attempted_completion_function = std::mem::transmute(rl_attempted_completion_function);
//     rl_attempted_completion_function = std::mem::transmute(0 as usize);
// 	bashline_set_event_hook();
// 	if !itext.is_null() {
// 		old_startup_hook = std::mem::transmute(rl_startup_hook);
// 		rl_startup_hook = std::mem::transmute(set_itext as usize);
// 		deftext = itext;
// 	}

// 	let mut ret = readline(p);

// 	rl_attempted_completion_function = std::mem::transmute(old_attempted_completion_function);
// 	old_attempted_completion_function = std::mem::transmute(0 as usize);
// 	bashline_reset_event_hook();

// 	if ret.is_null() {
//         return ret;
//     }

//     let len:i32 = libc::strlen(ret) as i32;
// 	ret = xrealloc(ret as *mut c_void, (len + 2) as usize) as *mut c_char;
//     // *ret = delim;
//     *ret.offset(len as isize) = delim;
//     *((ret as usize + 1) as *mut c_char) = b'\0'  as c_char;

// 	return ret;
// }
// }


fn edit_line(p : *mut c_char, itext : *mut c_char) -> *mut c_char {
    let mut len:i32;
    unsafe {
        if bash_readline_initialized == 0 {
            initialize_readline();
        }
    
        old_attempted_completion_function = std::mem::transmute(rl_attempted_completion_function);
        rl_attempted_completion_function = std::mem::transmute(0 as usize);
        bashline_set_event_hook();
        if !itext.is_null() {
            old_startup_hook = std::mem::transmute(rl_startup_hook);
            rl_startup_hook = std::mem::transmute(set_itext as usize);
            deftext = itext;
        }
    
        let mut ret = readline(p);
    
        rl_attempted_completion_function = std::mem::transmute(old_attempted_completion_function);
        old_attempted_completion_function = std::mem::transmute(0 as usize);
        bashline_reset_event_hook();
    
        if ret.is_null() {
            return ret;
        }
        
        len = libc::strlen(ret) as i32;
        ret = xrealloc(ret as *mut c_void, (len + 2) as usize) as *mut c_char;
        *ret.offset(len as isize) = delim;
        len += 1;
        *ret.offset(len as isize) =  b'\0'  as c_char;
        return ret;
    }
}



fn sigalrm(s : c_int) {
unsafe {
    sigalrm_seen = 1;
}
}

fn reset_alarm()
{
unsafe {
    falarm(0, 0);
    set_signal_handler(libc::SIGALRM, old_alrm);
}
}

// fn ttyrestore()
// {
// unsafe {
//     if termsave.is_none() {
//         let tmp: tty_save  = std::mem::zeroed();
//         termsave = Some(tmp);
//     }

//     let ter = termsave.unwrap();
// 	ttsetattr(ter.fd, std::mem::transmute(&(ter.attrs)));
// 	tty_modified = 0;
// }
// }

unsafe extern "C" fn ttyrestore(mut ttp:*mut tty_save){
    ttsetattr((*ttp).fd,&mut (*ttp).attrs);
    tty_modified = 0 as libc::c_int;
}

#[no_mangle]
pub extern "C" fn read_tty_cleanup()
{
unsafe {
    if tty_modified != 0 {
        ttyrestore(&mut termsave);
    }
}
}

#[no_mangle]
pub extern "C" fn read_tty_modified() -> c_int
{
unsafe {
    return tty_modified;
    }
}

static mut old_delim_ctype: c_int = 0;
static mut old_delim_func: usize = 0;
static mut old_newline_ctype: c_int = 0;
static mut old_newline_func: usize = 0;

static mut delim_char: u8 = 0;
// fn set_eol_delim(c: c_int)
// {
// unsafe {
// 	if bash_readline_initialized == 0 {
//         initialize_readline();
//     }

// 	let cmap = rl_get_keymap();
//     let n = std::mem::size_of_val(&*cmap);
//     let ret_pos = (b'M' & 0x1f) as usize * n;
//     let c_pos = (c & 0x1f) as usize * n;

// 	/* Save the old delimiter char binding */
// 	old_newline_ctype = (*((cmap as usize + ret_pos) as Keymap)).tp as c_int;
// 	old_newline_func = (*((cmap as usize + ret_pos) as Keymap)).function as usize;
// 	old_delim_ctype =  (*((cmap as usize + c_pos) as Keymap)).tp as c_int;
// 	old_delim_func = (*((cmap as usize + c_pos) as Keymap)).function as usize;

// 	/* Change newline to self-insert */
// 	(*((cmap as usize + ret_pos) as Keymap)).tp = ISFUNC as c_char;
// 	(*((cmap as usize + ret_pos) as Keymap)).function = rl_insert;

// 	/* Bind the delimiter character to accept-line. */
// 	(*((cmap as usize + c_pos) as Keymap)).tp = ISFUNC as c_char;
// 	(*((cmap as usize + c_pos) as Keymap)).function = rl_newline;

// 	delim_char = c as u8;
// }
// }

fn set_eol_delim(c: c_int)
{
    let mut cmap:Keymap;
unsafe {
	if bash_readline_initialized == 0 {
        initialize_readline();
    }

	// let cmap = rl_get_keymap();
    cmap = rl_get_keymap();

    old_newline_ctype =  (*cmap.offset((b'M' as i32 & 0x1f) as isize)).tp as c_int;
    old_newline_func  =  (*cmap.offset((b'M' as i32 & 0x1f) as isize)).function as usize;
    old_delim_ctype   =  (*cmap.offset(c as isize)).tp as c_int;
    old_delim_func    =  (*cmap.offset(c as isize)).function as usize;

    /* Change newline to self-insert */
	(*cmap.offset((b'M' as i32 & 0x1f) as isize)).tp = ISFUNC as c_char;
	(*cmap.offset((b'M' as i32 & 0x1f) as isize)).function = rl_insert;

	/* Bind the delimiter character to accept-line. */
	(*cmap.offset(c as isize)).tp = ISFUNC as c_char;
	(*cmap.offset(c as isize)).function = rl_newline;

	delim_char = c as u8;
}
}

fn reset_eol_delim(cp: *mut c_char)
{
unsafe {
	let cmap = rl_get_keymap();
    let n = std::mem::size_of_val(&*cmap);
    let ret_pos = (b'M' & 0x1f) as usize * n;
    let delim_pos = (delim_char & 0x1f) as usize * n;

	(*((cmap as usize + ret_pos) as Keymap)).tp = old_newline_ctype as c_char;
	(*((cmap as usize + ret_pos) as Keymap)).function = std::mem::transmute(old_newline_func);

	(*((cmap as usize + delim_pos) as Keymap)).tp = old_delim_ctype as c_char;
	(*((cmap as usize + delim_pos) as Keymap)).function = std::mem::transmute(old_delim_func);
}
}
