use std::mem::size_of_val;

use libc::{c_int, c_uint, c_char, c_long, PT_NULL, c_void};
use rhelp::r_builtin_help;
include!(concat!("intercdep.rs"));
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn r_export_builtin(list: *mut WordList) -> c_int {
    return set_or_show_attributes(list, att_exported, 0);
}

#[no_mangle]
pub extern "C" fn r_readonly_builtin(list: *mut WordList) -> c_int {
    return set_or_show_attributes(list, att_readonly, 0);
}

#[no_mangle]
pub extern "C" fn set_or_show_attributes(mut list: *mut WordList, mut attribute: c_int, nodefs: c_int) -> c_int {
    let mut assign_error: c_int = 0;
    let mut any_failed: c_int = 0;
    let mut undo: c_int = 0;
    let mut functions_only: bool = false;
    let mut arrays_only: c_int = 0;
    let mut assoc_only: c_int = 0;
    let mut name: *mut c_char;
    let mut var: *mut SHELL_VAR;
    let mut assign: c_int;
    let mut aflags: c_int;
    let mut tlist: *mut WordList;
    let mut nlist: *mut WordList;
    let mut w: *mut WordDesc;
unsafe {
    reset_internal_getopt();
    let opt_str = std::ffi::CString::new("aAfnp").unwrap();
    let mut opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    while  opt != -1 {
        let opt_char:char=char::from(opt as u8);
        match opt_char {
            'n' => undo = 1,
            'f' => functions_only = true,
            'a' => arrays_only = 1,
            'A' => assoc_only = 1,
            'p' => break,
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
            builtin_usage ();
            return EX_USAGE;
            }
        }
        opt = internal_getopt (list, opt_str.as_ptr() as * mut c_char);
    }
    list = loptend;

    if !list.is_null() {
        if attribute & att_exported != 0 {
            array_needs_making = 1;
        }
        if undo != 0 && (attribute & att_readonly) != 0 {
            attribute &= !att_readonly;
        }
        while !list.is_null() {
            name = (*(*list).word).word;

            if functions_only {
                var = find_function(name);
                if var.is_null() {
                    builtin_error("%s: not a function\0".as_ptr() as *const c_char, name);
                    any_failed += 1;
                } else if (attribute & att_exported) != 0 && undo == 0 &&
                    exportable_function_name(name) == 0 {
                    builtin_error("%s: cannot export\0".as_ptr() as *const c_char, name);
                    any_failed += 1;
                } else {
                    if undo == 0 {
                        (*var).attributes |= attribute;
                     } else {
                        (*var).attributes &= !attribute;
                    }
                }
                list = (*list).next;
                continue;
            }
            assign = assignment(name, 0);
            aflags = 0;
            if assign != 0 {
                *(name.offset(assign  as isize ) ) = b'\0' as c_char;
                if *((name.offset((assign-1)  as isize))) == b'+' as c_char {
                    aflags |= ASS_APPEND;
                    *(name.offset((assign-1)  as isize))  = b'\0' as c_char;
                }
            }

            if legal_identifier(name) == 0 {
                sh_invalidid(name);
                if assign != 0 {
                    assign_error += 1;
                } else {
                    any_failed += 1;
                }
                list = (*list).next;
                continue;
            }

            if assign != 0 {
                *(name.offset(assign  as isize)) = b'=' as c_char;
                if (aflags & ASS_APPEND) != 0 {
                    *(name.offset((assign-1) as isize)) = b'+' as c_char;
                }

                if arrays_only != 0 || assoc_only != 0 {
                    tlist = (*list).next;
                    (*list).next = PT_NULL as *mut WordList;

                    let mut optw: [u8;8] = [0;8];
                    optw[0] = b'-';
                    optw[1] = b'g';
                    let mut opti = 2;
                    if (attribute & att_readonly) != 0 {
                        optw[opti] = b'r';
                        opti += 1;
                    }
                    if (attribute & att_exported) != 0 {
                        optw[opti] = b'x';
                        opti += 1;
                    }
                    if (attribute & arrays_only) != 0 {
                        optw[opti] = b'a';
                        opti += 1;
                    } else {
                        optw[opti] = b'A';
                        opti += 1;
                    }

                    w = make_word(optw.as_ptr() as *const c_char);
                    nlist = make_word_list(w, list);

                    opt = declare_builtin(nlist);
                    if opt != EXECUTION_SUCCESS {
                        assign_error += 1;
                    }
                    (*list).next = tlist;
                    dispose_word(w);
                    libc::free(nlist as *mut c_void);
                } else if do_assignment_no_expand(name) == 0 {
                    assign_error += 1;
                }
                *(name.offset(assign  as isize))  = b'\0' as c_char;
                if (aflags & ASS_APPEND) != 0 {
                    *(name.offset((assign-1)  as isize))  = b'\0' as c_char;
                }
            }

            set_var_attribute(name, attribute, undo);
            if assign != 0 {
                *(name.offset(assign  as isize))  = b'=' as c_char;
                // *((name as usize + assign as usize) as *mut c_char) = b'=' as c_char;
                if (aflags & ASS_APPEND) != 0 {
                    *(name.offset((assign-1)  as isize))  = b'+' as c_char;
                }
            }
            list = (*list).next;
        }
    } else {
        let mut variable_list: *mut *mut SHELL_VAR;
        if (attribute & att_function) != 0 || functions_only {
            variable_list = all_shell_functions();
            if attribute != att_function {
                attribute &= !att_function;
            }
        } else {
            variable_list = all_shell_variables();
        }

        if (attribute & att_array) != 0 {
            arrays_only += 1;
            if attribute != att_array {
                attribute &= !att_array;
            }
        } else if (attribute & att_assoc) != 0 {
            assoc_only += 1;
            if attribute != att_assoc {
                attribute &= !att_assoc;
            }
        }

        if !variable_list.is_null() {
            let mut i = 0;
            loop {
                var = *(variable_list.offset(i))  as  *mut SHELL_VAR;
                if var.is_null() {
                    break;
                }

                if arrays_only != 0 && (((*var).attributes & att_array) == 0) {
                    i += 1;
                    continue;
                } else if assoc_only != 0 && (((*var).attributes & att_assoc) == 0) {
                    i += 1;
                    continue;
                }

                if ((*var).attributes & (att_invisible | att_exported)) == (att_invisible | att_exported) {
                    i += 1;
                    continue;
                }

                if ((*var).attributes & attribute) != 0 {
                    let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
                        (this_shell_builtin as usize == r_export_builtin as usize);
                    if pattr {
                        show_var_attributes(var, 1, nodefs);
                    }
                    else {
                        show_var_attributes(var, 0, nodefs);
                    }
                    any_failed = sh_chkwrite(any_failed);
                    if any_failed != 0 {
                        break;
                    }
                }
                i += 1;
            }
            libc::free(variable_list as *mut c_void);
        }
    }
}
    return if assign_error != 0 {EX_BADASSIGN} else if any_failed == 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
}

#[no_mangle]
pub extern "C" fn show_all_var_attributes(v: c_int, nodefs: c_int) -> c_int {
    let mut i = 0;
    let mut any_failed = 0;
    let mut var: *mut SHELL_VAR;
    let mut variable_list: *mut *mut SHELL_VAR;
unsafe {
    variable_list =  if  v != 0 {all_shell_variables() } else {all_shell_functions()};
    if variable_list.is_null() {
        return EXECUTION_SUCCESS;
    }

    loop {
        var = *(variable_list.offset(i));
        if var.is_null() {
            break;
        }
        let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
            (this_shell_builtin as usize == r_export_builtin as usize);
        if pattr {
            show_var_attributes(var, 1, nodefs);
        }
        else {
            show_var_attributes(var, 0, nodefs);
        }
        any_failed = sh_chkwrite(any_failed);
        if any_failed != 0 {
            break;
        }
        i += 1;
    }

    libc::free(variable_list as *mut c_void);
}
    return if any_failed == 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
}

#[no_mangle]
pub extern "C" fn show_local_var_attributes(v: c_int, nodefs: c_int) -> c_int {
    let mut i = 0;
    let mut any_failed = 0;
    let mut var: *mut SHELL_VAR;
    let mut variable_list: *mut *mut SHELL_VAR;
unsafe {
    variable_list = all_local_variables(0);
    if variable_list.is_null() {
        return EXECUTION_SUCCESS;
    }

    loop {
        var = variable_list.offset(i) as *mut SHELL_VAR;
        if var.is_null() {
            break;
        }

        let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
            (this_shell_builtin as usize == r_export_builtin as usize);
        if pattr {
            show_var_attributes(var, 1, nodefs);
        }
        else {
            show_var_attributes(var, 0, nodefs);
        }
        any_failed = sh_chkwrite(any_failed);
        if any_failed != 0 {
            break;
        }

        i += 1;
    }

    libc::free(variable_list as *mut c_void);
}
    return if any_failed == 0 {EXECUTION_SUCCESS} else {EXECUTION_FAILURE};
}

#[no_mangle]
pub unsafe extern "C"   fn show_var_attributes(var: *mut SHELL_VAR, pattr: c_int, mut nodefs: c_int) -> c_int {
    let mut flags = [0;16];
    let i = var_attribute_string(var, pattr, flags.as_mut_ptr());
    if ((*var).attributes & att_function) != 0 && nodefs == 0 && (pattr == 0 || posixly_correct == 0) {
        println!("{}",CStr::from_ptr(named_function_string((*var).name, (*var).value as *mut COMMAND, FUNC_MULTILINE | FUNC_EXTERNAL)).to_str().unwrap());
        nodefs += 1;
        if pattr == 0 && i == 1 && flags[0] == b'f' as c_char {
            return 0;
        }
    }
    if pattr == 0 || posixly_correct == 0 {
        if i != 0 {
            print!("declare -{} ",CStr::from_ptr(flags.as_ptr()).to_str().unwrap());
        }
        else {
            print!("declare -- ");
        }
    } 
    else if i != 0 {
        print!("{} -{} ",CStr::from_ptr(this_command_name).to_str().unwrap(),CStr::from_ptr(flags.as_ptr()).to_str().unwrap());
    }
    else {
        print!("{} ",CStr::from_ptr(this_command_name).to_str().unwrap());
    }

    if ((*var).attributes & att_invisible) != 0 &&
       (((*var).attributes & att_array) != 0 ||
        ((*var).attributes & att_assoc) != 0 ){
            println!("{}",CStr::from_ptr((*var).name).to_str().unwrap());
    }
    else if ((*var).attributes & att_array) != 0 {
        r_print_array_assignment(var, 0);
    } 
    else if ((*var).attributes & att_assoc) != 0 {
        r_print_assoc_assignment(var, 0);
    } 
    else if nodefs != 0 || ((((*var).attributes & att_function) != 0 && pattr != 0 && posixly_correct != 0)) {
        println!("{}",CStr::from_ptr((*var).name).to_str().unwrap());
    } 
    else if ((*var).attributes & att_function) != 0 {
        println!("{}",CStr::from_ptr(named_function_string((*var).name, (*var).value as *mut COMMAND, FUNC_MULTILINE | FUNC_EXTERNAL)).to_str().unwrap());
    } 
    else if ((*var).attributes & att_invisible) != 0 || (*var).value == std::ptr::null_mut() {
        println!("{}",CStr::from_ptr((*var).name).to_str().unwrap());
    }
    else {
            let x = sh_double_quote (value_cell(var));
            println!("{}={}",CStr::from_ptr((*var).name).to_str().unwrap(),CStr::from_ptr(x).to_str().unwrap());
            libc::free(x as *mut c_void);
    }

    return 0;
}

unsafe fn  value_cell(var:*mut SHELL_VAR)->* mut c_char
{
  return (*var).value;
}

unsafe fn  array_cell(var:*mut SHELL_VAR)->* mut ARRAY
{
  return (*var).value as *mut ARRAY;
}

unsafe fn  assoc_cell(var:*mut SHELL_VAR)->* mut HASH_TABLE
{
  return (*var).value as *mut HASH_TABLE;
}

#[no_mangle]
pub extern "C" fn show_name_attributes(name: *mut c_char, nodefs: c_int) -> c_int {
unsafe {
    let var = find_variable_noref(name);
    if !var.is_null() {
        let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
            (this_shell_builtin as usize == r_export_builtin as usize);
        if pattr { 
            show_var_attributes(var, 1, nodefs);
        }
        else {
            show_var_attributes(var, 0, nodefs);
        }
        return 0;
    } else {
        return 1;
    }
}
}

#[no_mangle]
pub extern "C" fn show_localname_attributes(name: *mut c_char, nodefs: c_int) -> c_int {
unsafe {
    let var = find_variable_noref(name);
    let cond = var.is_null() && ((*var).attributes & att_local) != 0 && (*var).context == variable_context;
    if cond {
        let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
            (this_shell_builtin as usize == r_export_builtin as usize);
        if pattr {
            show_var_attributes(var, 1, nodefs);
        }
        else {
            show_var_attributes(var, 0, nodefs);
        }
        return 0;
    } else {
        return 1;
    }
}
}

#[no_mangle]
pub extern "C" fn show_func_attributes(name: *mut c_char, nodefs: c_int) -> c_int {
    unsafe {
        let var = find_function(name);
        if !var.is_null() {
            let pattr = (this_shell_builtin as usize == r_readonly_builtin as usize) ||
                (this_shell_builtin as usize == r_export_builtin as usize);
            if pattr {
                show_var_attributes(var, 1, nodefs);
            }
            else {
                show_var_attributes(var, 0, nodefs);
            }
            return 0;
        } else {
            return 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn set_var_attribute(name: *mut c_char, attribute: c_int, undo: c_int) {
    let mut var: *mut SHELL_VAR;
    let mut tvalue: *mut c_char;
unsafe {
    if undo != 0 {
        var = find_variable(name);
    } else {
        let tv = find_tempenv_variable(name);
        if !tv.is_null() && ((*tv).attributes & att_tempvar) != 0 {
            tvalue = if !(*tv).value.is_null() {
                libc::strdup((*tv).value)
            } else {"\0".as_ptr() as *mut c_char};

            var = bind_variable((*tv).name, tvalue, 0);
            if var.is_null() {
                libc::free(tvalue as *mut c_void);
                return;
            }
            (*var).attributes |= (*tv).attributes & (!att_tempvar);
            if posixly_correct != 0 || shell_compatibility_level <= 44 {
                if (*var).context == 0 && (attribute & att_readonly) != 0 {
                    let v = find_global_variable((*tv).name);
                    if v as usize != var as usize {
                        (*tv).attributes |= att_propagate;
                    }
                } else {
                    (*tv).attributes |= att_propagate;
                }

                if (*var).context != 0 {
                    (*var).attributes |= att_propagate;
                }
            }
            if undo == 0 {
                (*tv).attributes |= attribute;
            } else {
                (*tv).attributes &= !attribute;
            }

            stupidly_hack_special_variables((*tv).name);
            libc::free(tvalue as *mut c_void);
        } else {
            var = find_variable_notempenv(name);
            if var.is_null() {
                let refvar = find_variable_nameref_for_create(name, 0);
                if cmp_two(std::mem::transmute(refvar),std::mem::transmute(&nameref_invalid_value)) {
                    return;
                }
            }
            if var.is_null() {
                var = bind_variable(name, PT_NULL as *mut c_char, 0);
                if !var.is_null() {
                    (*var).attributes |= att_invisible;
                }
            } else if (*var).context != 0 {
                (*var).attributes |= att_propagate;
            }
        }
    }

    if !var.is_null() {
        if undo == 0 {
            (*var).attributes |= attribute;
        } else {
            (*var).attributes &= !attribute;
        }
    }

    if !var.is_null() && (((*var).attributes & att_exported) != 0 || (attribute & att_exported) != 0) {
        array_needs_making += 1;
    }
}
}

#[no_mangle]
pub extern "C" fn var_attribute_string(var: *mut SHELL_VAR, pattr: c_int, flags: *mut c_char) -> c_int {
    let mut i = 0;
unsafe {
    if pattr == 0 || posixly_correct == 0 {
        if ((*var).attributes & att_array) != 0 {
           *(flags.offset(i as isize)) = b'a' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_assoc) != 0 {
           *(flags.offset(i as isize)) = b'A' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_function) != 0 {
            *(flags.offset(i as isize)) = b'f' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_integer) != 0 {
            *(flags.offset(i as isize)) = b'i' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_nameref) != 0 {
            *(flags.offset(i as isize)) = b'n' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_readonly) != 0 {
            *(flags.offset(i as isize)) = b'r' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_trace) != 0 {
           *(flags.offset(i as isize)) = b't' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_exported) != 0 {
           *(flags.offset(i as isize)) = b'x' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_capcase) != 0 {
           *(flags.offset(i as isize)) = b'c' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_lowercase) != 0 {
           *(flags.offset(i as isize)) = b'l' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_uppercase) != 0 {
           *(flags.offset(i as isize)) = b'u' as c_char;
            i += 1;
        }
    } else {
        if ((*var).attributes & att_array) != 0 {
           *(flags.offset(i as isize)) = b'a' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_assoc) != 0 {
           *(flags.offset(i as isize)) = b'A' as c_char;
            i += 1;
        }
        if ((*var).attributes & att_function) != 0 {
           *(flags.offset(i as isize)) = b'f' as c_char;
            i += 1;
        }
    }

   *(flags.offset(i as isize)) = b'\0' as c_char;
}
    return i ;
}

unsafe fn cmp_two(a: usize, b: usize) -> bool {
    return a == b;
}

#[no_mangle]
pub unsafe extern "C" fn r_print_array_assignment(var: *mut SHELL_VAR, quote: c_int) {
    let mut vstr  = array_to_assign(array_cell(var) as *mut ARRAY ,quote); 

    if  vstr == std::ptr::null_mut() {
        if quote != 0 {
            println!("{}=\'()\'",CStr::from_ptr((*var).name).to_str().unwrap());
        }
        else {
            println!("{}=()",CStr::from_ptr((*var).name).to_str().unwrap());
        }
    }
    else {
        println!("{}={}",CStr::from_ptr((*var).name).to_str().unwrap(),CStr::from_ptr(vstr).to_str().unwrap());
        libc::free(vstr as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn r_print_assoc_assignment(var: *mut SHELL_VAR, quote: c_int) {
    let mut vstr  = assoc_to_assign(assoc_cell(var) as *mut HASH_TABLE ,quote); 

    if  vstr == std::ptr::null_mut() {
        if quote != 0 {
            println!("{}=\'()\'",CStr::from_ptr((*var).name).to_str().unwrap());
        }
        else {
            println!("{}=()",CStr::from_ptr((*var).name).to_str().unwrap());
        }
    }
    else {
        println!("{}={}",CStr::from_ptr((*var).name).to_str().unwrap(),CStr::from_ptr(vstr).to_str().unwrap());
        libc::free(vstr as *mut c_void);
    }
}
