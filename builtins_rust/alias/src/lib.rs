
use std::ffi::CStr;
//extern crate rcommon;
use rcommon::r_sh_notfound;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE,r_builtin_usage,SHELL_VAR};
use rhelp::r_builtin_help;
use std::ffi::CString;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn legal_alias_name(_: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn sh_single_quote(_: *const libc::c_char) -> *mut libc::c_char;
    static mut posixly_correct: libc::c_int;
    static mut aliases: *mut HashTable;
    fn find_alias(_: *mut libc::c_char) -> *mut AliasT;
    fn add_alias(_: *mut libc::c_char, _: *mut libc::c_char);
    fn remove_alias(_: *mut libc::c_char) -> libc::c_int;
    fn delete_all_aliases();
    fn all_aliases() -> *mut *mut AliasT;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_notfound(_: *mut libc::c_char);
    fn sh_chkwrite(_: libc::c_int) -> libc::c_int;
    static mut loptend: *mut WordList;
    fn internal_getopt(_: *mut WordList, _: *mut libc::c_char) -> libc::c_int;
    fn find_user_command(name:*const libc::c_char)->*mut libc::c_char;
    fn find_shell_builtin(builtin: *mut libc::c_char) -> *mut libc::c_char;
    fn find_function (name:* const libc::c_char)-> *mut SHELL_VAR;
    fn reset_internal_getopt();
}
pub type SizeT = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bucket_contents {
    pub next: *mut bucket_contents,
    pub key: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub khash: libc::c_uint,
    pub times_found: libc::c_int,
}
pub type BucketContents = bucket_contents;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub bucket_array: *mut *mut BucketContents,
    pub nbuckets: libc::c_int,
    pub nentries: libc::c_int,
}
pub type HashTable = hash_table;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type AliasT = alias;
pub static AL_REUSABLE:i32 = 0x01;
#[no_mangle]
pub unsafe extern "C" fn r_alias_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut any_failed;
    let mut offset;
    let mut pflag ;
    let mut dflags ;
    let  alias_list: *mut *mut AliasT;
    let mut t: *mut AliasT;
    let mut name: *mut libc::c_char;
    let mut value: *mut libc::c_char;
    dflags = if posixly_correct != 0 { 0 as libc::c_int } else { 0x1 as libc::c_int };
    pflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        offset = internal_getopt(
            list,
            b"p\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(offset != -(1 as libc::c_int)) {
            break;
        }
        match offset as u8 {
            b'p' => {
                pflag = 1 ;
                dflags |= AL_REUSABLE;
            }
            _ => {
                if offset == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
                r_builtin_usage();
                return EX_USAGE;
            }
        }
    }
    list = loptend;
    if list.is_null() || pflag != 0 {
        if aliases.is_null() {
            return EXECUTION_SUCCESS!();
        }
        alias_list = all_aliases();
        if alias_list.is_null() {
            return EXECUTION_SUCCESS!();
        }
        offset = 0;
        while !(*alias_list.offset(offset as isize)).is_null() {
            print_alias(*alias_list.offset(offset as isize), dflags);
            offset += 1;
        }
        free(alias_list as *mut libc::c_void);
        if list.is_null() {
            return sh_chkwrite(EXECUTION_SUCCESS!());
        }
    }
    any_failed = 0;
    while !list.is_null() {
        name = (*(*list).word).word;
        offset = 0;
        while *name.offset(offset as isize) as libc::c_int != 0
            && *name.offset(offset as isize) as libc::c_int != '=' as i32
        {
            offset += 1;
        }
        if offset != 0 && *name.offset(offset as isize) as libc::c_int == '=' as i32 {
            *name.offset(offset as isize) = '\u{0}' as i32 as libc::c_char;
            value = name.offset(offset as isize).offset(1 as libc::c_int as isize);
            if legal_alias_name(name, 0) == 0 {
                builtin_error(
                    dcgettext(
                        0 as *const libc::c_char,
                        b"`%s': invalid alias name\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    name,
                );
                any_failed += 1;
            } else {
                let slice= CStr::from_ptr(value);
                let mut r_str=slice.to_str().unwrap().to_owned();
                let new_str =  CString::new(r_str).unwrap();
                if legal_alias_rust(name,new_str.as_ptr() as *mut libc::c_char) == 0 {
                    add_alias(name, value);
                }
            }
        } else {
            t = find_alias(name);
            if !t.is_null() {
                print_alias(t, dflags);
            } else {
                sh_notfound(name);
                any_failed += 1;
            }
        }
        list = (*list).next;
    }
    return if any_failed != 0 {EXECUTION_FAILURE!()} else { EXECUTION_SUCCESS!()};
}
#[no_mangle]
pub unsafe extern "C" fn r_unalias_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut alias: *mut AliasT;
    let mut opt: libc::c_int;
    let mut aflag: libc::c_int;
    aflag = 0 as libc::c_int;
    reset_internal_getopt();
    loop {
        opt = internal_getopt(
            list,
            b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt as u8 {
            b'a' => {
                aflag = 1 as libc::c_int;
            }
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }
    }
    list = loptend;
    if aflag != 0 {
        delete_all_aliases();
        return 0;
    }
    if list.is_null() {
        builtin_usage();
        return EX_USAGE;
    }
    aflag = 0 as libc::c_int;
    while !list.is_null() {
        alias = find_alias((*(*list).word).word);
        if !alias.is_null() {
            remove_alias((*alias).name);
        } else {
            sh_notfound((*(*list).word).word);
            aflag += 1;
        }
        list = (*list).next;
    }
    return if aflag != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
unsafe extern "C" fn print_alias( alias: *mut AliasT,  flags: libc::c_int) {
    let value: *mut libc::c_char;
    value = sh_single_quote((*alias).value);
    if flags & 0x1 as libc::c_int != 0 {
        print!("alias ");
        //printf(
        //    b"alias %s\0" as *const u8 as *const libc::c_char,
            if !((*alias).name).is_null()
                && *((*alias).name).offset(0 as libc::c_int as isize) as libc::c_int
                    == '-' as i32
            {
               // b"-- \0" as *const u8 as *const libc::c_char
               print!("-- ");
            } else {
               // b"\0" as *const u8 as *const libc::c_char
               print!(" ");
            }
        //);
    }
    println!("{}={}", CStr::from_ptr((*alias).name).to_string_lossy().into_owned(), CStr::from_ptr(value).to_string_lossy().into_owned());
    free(value as *mut libc::c_void);
}
unsafe  fn legal_alias_rust(name :*mut libc::c_char,value  :*mut libc::c_char ) -> libc::c_int {  
    
    let mut name_w:*mut libc::c_char;
    let mut value_w:*mut libc::c_char;
    let mut new_value:*mut libc::c_char;
    let mut new_value_2:*mut libc::c_char;
    let mut shell_bui : *mut libc::c_char;
    let mut t: *mut AliasT;
    let mut dflags ;
    dflags = if posixly_correct != 0 { 0 as libc::c_int } else { 0x1 as libc::c_int };
    
    if libc::strstr(value,CString::new(";").unwrap().as_ptr() as *mut libc::c_char) != std::ptr::null_mut() {
        println!("; is not allow in alias");
        return  1;
    }
    t = find_alias(name);
    if !t.is_null() {
        println!("{} is already in alias", CStr::from_ptr(name).to_string_lossy().into_owned());
        print_alias(t, dflags);
        return  1;
    }
    name_w = find_user_command(name);
    new_value = sh_single_quote(value);
    // 按照空格区分
    new_value_2 = libc::strtok(value, CString::new(" ").unwrap().as_ptr() as *mut libc::c_char) ;
    t = find_alias(new_value_2);
    while t != std::ptr::null_mut() {
        new_value_2 = libc::strtok((*t).value, CString::new(" ").unwrap().as_ptr() as *mut libc::c_char) ;
        if libc::strcmp((*t).name,new_value_2) == 0 {
            break;
        }
        t = find_alias(new_value_2);
    }
    let arr:[ *mut libc::c_char;7] = [CString::new("exec").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new("eval").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new("builtin").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new("command").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new("function").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new("source").unwrap().into_raw() as *mut libc::c_char,
                                      CString::new(".").unwrap().into_raw() as *mut libc::c_char ];
    
    for index in 0..7 {
        if libc::strcmp(new_value_2, arr[index]) == 0 {
            println!("command {} will raise an unsafe operation",CStr::from_ptr(arr[index]).to_string_lossy().into_owned());
            return  1;
        }
    }
    value_w = find_user_command(new_value_2);
    if name_w != std::ptr::null_mut(){
        if value_w != std::ptr::null_mut() && libc::strcmp(name_w, value_w) == 0  {
            return 0;
        }
        else { 
            println!("The name and value point to different executable files");
            return 1 ;
        }
    }
    else {
        if find_shell_builtin(name) !=  std::ptr::null_mut() {
            println!("name {} is shell builtin",CStr::from_ptr(name).to_string_lossy().into_owned());
            return 1;
        }
        else if find_function(name) !=  std::ptr::null_mut() {
            println!("name {} is function",CStr::from_ptr(name).to_string_lossy().into_owned());
            return 1;
        }
    }
    return 0;
}
