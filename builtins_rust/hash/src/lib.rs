
extern crate libc;
extern crate rread;
use libc::{c_char,c_int, strchr,free,c_void,strerror,EISDIR};
use std::ffi::{CStr,CString};
use std::io::{stdout, Write};
use rread::{SHELL_VAR};
use rcommon::{r_find_shell_builtin,r_builtin_usage};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
use rhelp::r_builtin_help;
use std::fs;
use std::os::linux::fs::MetadataExt;
type PTR_T=c_void;
#[repr (C)]
pub struct bucket_contents{
    pub next:*mut bucket_contents,
    pub key:*mut c_char,
    pub data:*mut PTR_T,     //void *  or char *
    pub khash:u32,
    pub times_found:i32,
}
type BUCKET_CONTENTS=bucket_contents;
#[repr (C)]
pub struct hash_table{
    pub bucket_array:*mut *mut BUCKET_CONTENTS,
    pub nbucjets:i32,
    pub nentries:i32,
}
type HASH_TABLE = hash_table;
#[repr (C)]
pub struct _pathdata{
    pub path:*mut c_char,
    pub flags:i32,
}
type PATH_DATA = _pathdata;
//enum
#[macro_export]
macro_rules! PARAMS {
    ($protos:expr) => {
        $protos
    };
}
pub unsafe fn hash_entries(ht: *mut HASH_TABLE) -> i32 {
    if ht != std::ptr::null_mut() {
        return (*ht).nentries;
    } else {
        return 0;
    }
}
#[macro_export]
macro_rules! HASH_ENTRIES {
    ($ht:expr) => {
        if ($ht) != std::ptr::null_mut(){
            return (*$ht).nentries;
        }
        else{
            return 0;
        }
    };
}
fn HASH_ENTRIES(ht:*mut HASH_TABLE)->i32{
    unsafe{
        if ht != std::ptr::null_mut(){
            return (*ht).nentries;
        }
        else{
            return 0;
        }
    }
}
#[macro_export]
macro_rules! pathdata {
    ($x:expr) => {
        (*$x).data as *mut PATH_DATA
    };
}
#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if $s != std::ptr::null_mut(){
            free($s as *mut c_void);
        }
    };
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alias {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub flags: libc::c_char,
}
pub type AliasT = alias;
// type i32 hash_efunc PARAMS(*mut BUCKET_CONTENTS);
type hash_wfunc = extern  fn(*mut BUCKET_CONTENTS)->i32;
type sh_builtin_func_t = extern fn (*mut WordList)->i32;
//extern c
extern "C"{
    static loptend:*mut WordList;
    static hashing_enabled:i32;
    static list_optarg:*mut c_char;
    static this_command_name:*mut c_char;
    static posixly_correct:i32;
    static restricted:i32;
    static shell_compatibility_level:i32;
    static hashed_filenames:*mut HASH_TABLE;
    static dot_found_in_search:i32;
    fn builtin_error(format:*const c_char,...);
    fn reset_internal_getopt();
    fn internal_getopt(list:*mut WordList,opts:*mut c_char)->i32;
    fn all_aliases() -> *mut *mut AliasT;
    fn sh_needarg(s:*mut c_char);
    fn phash_flush();
    fn sh_restricted(s:*mut c_char);
    fn absolute_program(string:*const c_char)->i32;
    fn find_user_command(name:*const c_char)->*mut c_char;
    fn executable_file(file:*const c_char)->i32;
    fn sh_notfound(s:*mut c_char);
    fn is_directory(file:*const c_char)->i32;
    fn phash_insert(filename:*mut c_char,full_path:*mut c_char,check_dot:i32,found:i32);
    fn phash_remove(filename:*const c_char)->i32;
    fn find_function(name:*const c_char)->*mut SHELL_VAR;
    fn find_shell_builtin(name:*mut c_char)->*mut sh_builtin_func_t;
    fn hash_walk(table:*mut HASH_TABLE,func:*mut hash_wfunc);
    fn phash_search(filename:*const c_char)->*mut c_char;
    fn printable_filename(f:*mut c_char,flage:i32)->*mut c_char;
}

static mut common_inode: c_int = 0;
//rust
/* Print statistics on the current state of hashed commands.  If LIST is
   not empty, then rehash (or hash in the first place) the specified
   commands. */
#[no_mangle]
pub extern "C" fn r_hash_builtin(mut list:*mut WordList)->i32{
    let mut expunge_hash_table:i32;
    let mut list_targets:i32;
    let mut list_portably:i32;
    let mut delete:i32;
    let mut opt:i32;
    let mut w:*mut c_char;
    let mut pathname:*mut c_char;
    unsafe{
        if hashing_enabled == 0{
            let c_str = CString::new("hashing disabled").unwrap();
            let c_str_ptr = c_str.as_ptr();
            builtin_error(c_str_ptr);
            return EXECUTION_FAILURE!();
        }
        expunge_hash_table = 0;
        list_targets = 0;
        list_portably = 0;
        delete = 0;
        pathname = std::ptr::null_mut();
        reset_internal_getopt();
        let opts = CString::new("dlp:rt").unwrap();
        opt = internal_getopt(list,opts.as_ptr() as *mut c_char);
        while opt != -1{
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);
            match opt_char{
                'd' => delete = 1,
                'l' => list_portably = 1,
                'p' => pathname = list_optarg,
                'r' => expunge_hash_table = 1,
                't' => list_targets = 1,
                 _  => {
                    if opt == -99 {
                        r_builtin_help();
                        return EX_USAGE;
                    }
                     r_builtin_usage();
                     return EX_USAGE;
                 }
            }
            opt = internal_getopt(list,opts.as_ptr() as *mut c_char);
        }
        list = loptend;
        /* hash -t requires at least one argument. */
        if list == std::ptr::null_mut() && (delete != 0 || list_targets != 0) {
            let temp:CString;
            let temp_ptr:*mut c_char;
            if delete != 0{
                temp = CString::new("-d").unwrap();
                temp_ptr = temp.as_ptr() as *mut c_char;
                sh_needarg(temp_ptr);
            }
            else{
                temp = CString::new("-t").unwrap();
                temp_ptr = temp.as_ptr() as *mut c_char;
                sh_needarg(temp_ptr);
            }
            return EXECUTION_FAILURE!();
        }
        
        /* We want hash -r to be silent, but hash -- to print hashing info, so
         we test expunge_hash_table. */
        if list==std::ptr::null_mut() && expunge_hash_table == 0{
            opt = r_print_hashed_commands(list_portably);
            if opt==0 && posixly_correct==0 && (list_portably==0 || shell_compatibility_level<=50){
                let s_cstr = CStr::from_ptr(this_command_name);
                let s_str = s_cstr.to_str().unwrap();
                let s_string = s_str.to_owned();
                println!("{}:hash table empty",s_string);
            }
            return EXECUTION_SUCCESS!();
        }
        if expunge_hash_table != 0{
            phash_flush();
        }
        /* If someone runs `hash -r -t xyz' he will be disappointed. */
        if list_targets != 0{
            return r_list_hashed_filename_targets(list,list_portably);
        }
        if restricted != 0 && pathname != std::ptr::null_mut(){
            if strchr(pathname,'/' as c_int) != std::ptr::null_mut(){
                sh_restricted(pathname);
                return EXECUTION_FAILURE!();
            }
            /* If we are changing the hash table in a restricted shell, make sure the
             target pathname can be found using a $PATH search. */
            w = find_user_command(pathname);
            if w==std::ptr::null_mut() || *w==0 || executable_file(w)==0{
                sh_notfound(pathname);
                free(w as *mut c_void);
                return EXECUTION_FAILURE!();
            }
            free(w as *mut c_void);
        }
        opt = EXECUTION_SUCCESS!();
        while list != std::ptr::null_mut(){
            /* Add, remove or rehash the specified commands. */
            w = (*(*list).word).word;
            if absolute_program(w as *const c_char) != 0{
                continue;
            }
            else if pathname != std::ptr::null_mut(){
                if is_directory(pathname) != 0{
                    let c_err = CString::new("%s:%s").unwrap();
                    let c_err_ptr = c_err.as_ptr();
                    builtin_error(c_err_ptr,pathname,strerror(EISDIR));
                    opt = EXECUTION_SUCCESS!();
                }
                else{
                    if legal_hash_rust(w,pathname) == 0{
                        phash_insert(w,pathname,0,0);
                    } 
                    
                }
            }
            else if delete != 0{
                if phash_remove(w) != 0{
                    sh_notfound(w);
                    opt = EXECUTION_FAILURE!();
                }
            }
            else if r_add_hashed_command(w,0) != 0{
                opt = EXECUTION_FAILURE!();
            }
            list = (*list).next;
        }
        stdout().flush();
        return opt;
    }//unsafe
}
extern "C" fn r_add_hashed_command(w:*mut c_char,quiet:i32)->i32{
    let mut rv:i32;
    let full_path:*mut c_char;
    rv = 0;
    unsafe{
        if find_function(w).is_null() && find_shell_builtin(w).is_null(){
        // if find_function(w).is_null() && r_find_shell_builtin(w).is_null(){
            phash_remove(w);
            full_path = find_user_command(w);
            if full_path != std::ptr::null_mut() && executable_file(full_path) != 0{
                phash_insert(w,full_path,dot_found_in_search,0)
            }
            else{
                if quiet == 0{
                    sh_notfound(w);
                }
                rv += 1;
            }
            FREE!(full_path);
        }
        return rv;
    }//unsafe
}
extern "C" fn r_print_hash_info(item:*mut BUCKET_CONTENTS)->i32{
    
    unsafe{
        let path_string = CStr::from_ptr((*pathdata!(item)).path).to_str().unwrap();//.to_owned()
        println!("{:04}\t{}",(*item).times_found,path_string);
        // println!("{:04}\t{}",(*item).times_found,*(*pathdata!(item)).path);
    }//unsafe
    0
}
#[no_mangle]
extern "C" fn r_print_portable_hash_info(item:*mut BUCKET_CONTENTS)->i32{
    let fp:*mut c_char;
    let f:*mut c_char;
    unsafe{
        fp = printable_filename((*pathdata!(item)).path,1);
        f = printable_filename((*item).key,1);
        let fp_string = CStr::from_ptr(fp).to_str().unwrap();//.to_owned()
        let f_string = CStr::from_ptr(f).to_str().unwrap();//.to_owned()
        println!("builtin hash -p {} {}",fp_string,f_string);
        if fp != (*pathdata!(item)).path{
            free(fp as *mut c_void);
        }
        if f != (*item).key{
            free(f as *mut c_void);
        }
        return 0;
    }//unsafe
}
#[no_mangle]
extern "C" fn r_print_hashed_commands(fmt:i32)->i32{
    unsafe{
        if hashed_filenames.is_null() || hash_entries(hashed_filenames) == 0 {
            return 0;
        }
        if fmt == 0{
            println!("hits\tcommand");
        }
        let fmt_t:hash_wfunc;
        if fmt != 0{
            fmt_t = r_print_portable_hash_info;
        }
        else{
            fmt_t = r_print_hash_info;
        }
        hash_walk(hashed_filenames,fmt_t as *mut hash_wfunc);
        return 1;
    }
}
#[no_mangle]
extern "C" fn r_list_hashed_filename_targets(list:*mut WordList,fmt:i32)->i32{
    let mut all_found:i32;
    let multiple:i32;
    let mut target:*mut c_char;
    let mut l:*mut WordList;
    all_found = 1;
  
    unsafe{
        if !(*list).next.is_null(){
            multiple = 1;
        }
        else{
            multiple = 0;
        }
        l = list;
        while !l.is_null(){
            target = phash_search((*(*l).word).word);
            if target.is_null(){
                all_found = 0;
                sh_notfound((*(*l).word).word);
                continue;
            }
            if fmt != 0{
                let target_string = CStr::from_ptr(target).to_str().unwrap();//.to_owned()
                let c_str = CStr::from_ptr((*(*l).word).word).to_str().unwrap();
                println!("builtin hash -p {} {}",target_string,c_str)
            }
            else{
                if multiple != 0{
                    let c_str = CStr::from_ptr((*(*l).word).word).to_str().unwrap();
                    print!("{}\t",c_str);
                }
                let target_str = CStr::from_ptr(target).to_str().unwrap();
                println!("{}",target_str);
            }
            free(target as *mut c_void);
            l = (*l).next;
        }
        
        if all_found != 0{
            return EXECUTION_SUCCESS!();
        }
        else{
            return EXECUTION_FAILURE!();
        }
    }
}
unsafe  fn legal_hash_rust(name :*mut libc::c_char,value :*mut libc::c_char ) -> libc::c_int {  
    let alias_list: *mut *mut AliasT  = all_aliases();
    let mut t: *mut AliasT;
    let mut offset;
    let mut name_w:*mut libc::c_char;
    let mut target:*mut c_char;
    offset = 0;
    if !alias_list.is_null() {
        t =  *alias_list.offset(offset as isize);
        while !t.is_null() {
            if !(*t).name.is_null() {
                if  libc::strcmp(name,(*t).name) == 0 {
                    println!("Prohibit setting existing variables that is already in alias");
                    println!("{} = {}" ,CStr::from_ptr((*t).name).to_string_lossy().into_owned(),CStr::from_ptr((*t).value).to_string_lossy().into_owned());
                    return 1;
                }
            }
            offset += 1;
            t =  *alias_list.offset(offset as isize);
         }
    }
    if find_shell_builtin(name) !=  std::ptr::null_mut() {
        println!("Prohibit setting existing variables {} is a shell builtin",CStr::from_ptr(name).to_string_lossy().into_owned());
        return 1;
    }
    else if find_function(name) !=  std::ptr::null_mut() {
        println!("Prohibit setting existing variables {} is a function",CStr::from_ptr(name).to_string_lossy().into_owned());
        return 1;
    }
    name_w = find_user_command(name) ;
    if name_w !=  std::ptr::null_mut() {
        file_inode(CStr::from_ptr(name_w).to_str().unwrap(),CStr::from_ptr(value).to_str().unwrap());
        if common_inode == 1 {
            return 1;
        }
    }
    target = phash_search(name);
    if target != std::ptr::null_mut() {
        println!("{} is already in hash", CStr::from_ptr(name).to_string_lossy().into_owned());
        return  1;
    }
    return 0;
}

unsafe fn file_inode(pathname : &str,pathname2 : &str) -> std::io::Result<()> {
    let meta = fs::metadata( pathname )?;
    let meta2 = fs::metadata( pathname2 )?;
    common_inode = 0;
    if (meta.st_ino() != meta2.st_ino()) {
        println!("The name and value point to different executable files");
        common_inode = 1;
    }
    Ok(())
}
