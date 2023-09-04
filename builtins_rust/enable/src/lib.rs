use std::ffi::{CString,CStr};
extern crate rcmd;
use libc::c_char;
use std::path::Path;
use libloading::Library;
use rcmd::*;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
use rhelp::r_builtin_help;
/*
#define ENABLED  1
#define DISABLED 2
#define SPECIAL  4

#define AFLAG	0x01
#define DFLAG	0x02
#define FFLAG	0x04
#define NFLAG	0x08
#define PFLAG	0x10
#define SFLAG	0x20
 */
pub const ENABLED: i32 = 1;
pub const DISABLED: i32 = 2;
pub const SPECIAL: i32 = 4;

pub const AFLAG: i32 = 0x01;
pub const DFLAG: i32 = 0x02;
pub const FFLAG: i32 = 0x04;
pub const NFLAG: i32 = 0x08;
pub const PFLAG: i32 = 0x10;
pub const SFLAG: i32 = 0x20;


// Flags describing various things about a builtin. 
//#define BUILTIN_ENABLED 0x01	/* This builtin is enabled. */
//#define BUILTIN_DELETED 0x02	/* This has been deleted with enable -d. */
//#define STATIC_BUILTIN  0x04	/* This builtin is not dynamically loaded. */
//#define SPECIAL_BUILTIN 0x08	/* This is a Posix `special' builtin. */
//#define ASSIGNMENT_BUILTIN 0x10	/* This builtin takes assignment statements. */
//#define POSIX_BUILTIN	0x20	/* This builtins is special in the Posix command search order. */
//#define LOCALVAR_BUILTIN   0x40	/* This builtin creates local variables */
//#define REQUIRES_BUILTIN 0x80  /* This builtin requires other files. */
//#define BASE_INDENT	4

pub const BUILTIN_ENABLED :i32 = 0x01;
pub const BUILTIN_DELETED :i32 = 0x02;
pub const STATIC_BUILTIN :i32 = 0x04;
pub const SPECIAL_BUILTIN :i32 = 0x08;
pub const ASSIGNMENT_BUILTIN :i32 = 0x10;
pub const POSIX_BUILTIN :i32 = 0x20;
pub const LOCALVAR_BUILTIN :i32 = 0x40;
pub const REQUIRES_BUILTIN :i32 = 0x80;


/* The MODE argument to `dlopen' contains one of the following: */
// #define RTLD_LAZY	0x00001	/* Lazy function call binding.  */
// #define RTLD_NOW	0x00002	/* Immediate function call binding.  */
// #define	RTLD_BINDING_MASK   0x3	/* Mask of binding time value.  */
// #define RTLD_NOLOAD	0x00004	/* Do not load the object.  */
// #define RTLD_DEEPBIND	0x00008	/* Use deep binding.  */

/* If the following bit is set in the MODE argument to `dlopen',
   the symbols of the loaded object and its dependencies are made
   visible as if the object were linked directly into the program.  */
// #define RTLD_GLOBAL	0x00100

pub const RTLD_LAZY :i32 = 0x00001;
pub const RTLD_NOW :i32 = 0x00002;
pub const RTLD_BINDING_MASK :i32 = 0x3;
pub const RTLD_NOLOAD :i32 = 0x00004;
pub const RTLD_DEEPBIND :i32 = 0x00008;
pub const RTLD_GLOBAL :i32 = 0x00100;

pub const FS_NODIRS :i32 = 0x20;
pub const FS_EXEC_PREFERRED :i32 = 0x4;

extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn absolute_program(_: *const libc::c_char) -> libc::c_int;
    fn printable_filename(_: *mut libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn get_string_value(_: *const libc::c_char) -> *mut libc::c_char;
    static mut num_shell_builtins: libc::c_int;
    static mut static_shell_builtins: [builtin; 0];
    static mut shell_builtins: *mut builtin;
    static mut restricted: libc::c_int;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_warning(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_restricted(_: *mut libc::c_char);
    fn sh_notbuiltin(_: *mut libc::c_char);
    fn builtin_address_internal(_: *mut libc::c_char, _: libc::c_int) -> *mut builtin;
    fn initialize_shell_builtins();
    static mut list_optarg: *mut libc::c_char;
    static mut loptend: *mut WordList;
    fn internal_getopt(_: *const WordList, _: *const libc::c_char) -> i32;
    fn reset_internal_getopt();
    fn find_in_path(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: libc::c_int,
    ) -> *mut libc::c_char;
    static mut it_builtins: ITEMLIST;
    static mut it_disabled: ITEMLIST;
    static mut it_enabled: ITEMLIST;
    fn set_itemlist_dirty(_: *mut ITEMLIST);
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type sh_load_func_t = unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int;
pub type sh_unload_func_t = unsafe extern "C" fn(*mut libc::c_char) -> ();
pub type sh_builtin_func_t = unsafe extern "C" fn(*mut WordList) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_strings {
    pub list: *mut *mut libc::c_char,
    pub list_size: libc::c_int,
    pub list_len: libc::c_int,
}
pub type STRINGLIST = _list_of_strings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct builtin {
    pub name: *mut libc::c_char,
    pub function: Option::<sh_builtin_func_t>,
    pub flags: libc::c_int,
    pub long_doc: *const *mut libc::c_char,
    pub short_doc: *const libc::c_char,
    pub handle: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _list_of_items {
    pub flags: libc::c_int,
    pub list_getter: Option::<unsafe extern "C" fn(*mut _list_of_items) -> libc::c_int>,
    pub slist: *mut STRINGLIST,
    pub genlist: *mut STRINGLIST,
    pub genindex: libc::c_int,
}
pub type ITEMLIST = _list_of_items;
#[no_mangle]
pub unsafe extern "C" fn r_enable_builtin(mut list: *mut WordList) -> i32 {
    let mut result: i32 = 0;
    let mut flags: i32 = 0;
    let mut opt: i32 = 0;
    let mut filter: i32 = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    reset_internal_getopt();
    let adnpsf = CString::new("adnpsf").expect("CString::new failed");;
    loop {
        opt = internal_getopt(list, adnpsf.as_ptr() );
        if !(opt != -1) {
            break;
        }
        let opt_char = opt as u8 as char;
        match opt_char {
            'a' => {
                flags |= AFLAG;
            }
            'n' => {
                flags |= NFLAG;
            }
            'p' => {
                flags |= PFLAG;
            }
            's' => {
                flags |= SFLAG;
            }
            'f' => {
                flags |= FFLAG;
                filename = list_optarg;
            }
            'd' => {
                flags |= DFLAG;
            }
            _ => {
                if opt ==-99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
                builtin_usage();
                return EX_USAGE;
            }
        }
    }
    list = loptend;
    // 如果是严格模式，就直接返回EXECUTION_FAILURE，命令结束。
    if restricted != 0 && flags & (FFLAG|DFLAG) != 0 {
        sh_restricted (0 as *mut c_char);
        return EXECUTION_FAILURE!();
    }

    // 配置filter，通过flag和PFLAG，后者表示打印，
    // 如果命令传入不带选项，或者带-P
    // 则打印enable和disable的集合；
    // 否则判断是否带-N
    // 带则打印DISABLED的，不带—N则打印ENABLED的
    if list.is_null() || flags & PFLAG != 0 {
        filter = if flags & AFLAG != 0 { ENABLED | DISABLED }
        else if flags & NFLAG != 0 { 
            DISABLED
        } else {
            ENABLED 
        };

        if flags & SFLAG != 0 {
            filter |= SPECIAL;
        }

        list_some_builtins(filter);

    } else if flags & FFLAG != 0 {
    // 如果不带-N或者参数不为空,那么判断-F.(bash源文件中判断HAVE_DLSYM，HAVE_DLOPEN两个宏存在)
    // -F后面需要加文件名，载入so，作为内建命令

        //判断是ENABLED还是DISABLED
        filter = if flags & NFLAG != 0 {
            DISABLED
        } else {
            ENABLED
        };

        // 判断是否设置SPECIAL标志位
        if flags & SFLAG != 0 {
            filter |= SPECIAL;
        }

        //载入so
        result = dyn_load_builtin(list, filter, filename);
        
        // 设置完成，bash源代码中判断PROGRAMMABLE_COMPLETION
        set_itemlist_dirty(&mut it_builtins);

    } else if flags & DFLAG != 0 {
    // 否则判断-D,-D含义是删除以 -f 选项加载的内建
        while !list.is_null() {
            opt = dyn_unload_builtin((*(*list).word).word);
            if opt == EXECUTION_FAILURE!() {
                result = EXECUTION_FAILURE!();
            }
            list = (*list).next;
        }
        set_itemlist_dirty(&mut it_builtins);

    } else {
    // 不带-N -F -D，且选项不为空的其他
        while !list.is_null() {
            opt = enable_shell_command((*(*list).word).word, flags & NFLAG);
            if opt == EXECUTION_FAILURE!() {
                sh_notbuiltin((*(*list).word).word);
                result = EXECUTION_FAILURE!();
            }
            list = (*list).next;
        }
    }
    return result;
}

//仅仅-p的时候会调用，打印，filter决定是enable，disable
unsafe extern "C" fn list_some_builtins(mut filter: libc::c_int) {
    let mut i: i32 = 0;

    while i < num_shell_builtins {
        let tmpIter =*shell_builtins.offset(i as isize);
        if !(tmpIter.function.is_none()||tmpIter.flags & BUILTIN_DELETED != 0)
        {
            if !(filter & SPECIAL != 0
                && (*shell_builtins.offset(i as isize)).flags & SPECIAL_BUILTIN
                    == 0)
            {
                if filter & ENABLED != 0
                    && (*shell_builtins.offset(i as isize)).flags & BUILTIN_ENABLED
                        != 0
                {
                    let name= unsafe{CStr::from_ptr((*shell_builtins.offset(i as isize)).name)};
                    println!("enable {}", name.to_str().expect("name cannot trans"));
                } else if filter & DISABLED != 0
                        && (*shell_builtins.offset(i as isize)).flags
                            & BUILTIN_ENABLED == 0 as libc::c_int
                    {
                        let name= unsafe{CStr::from_ptr((*shell_builtins.offset(i as isize)).name)};
                        println!("enable -n {}", name.to_str().expect("name cannot trans"));
  
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn enable_shell_command(
    mut name: *mut libc::c_char,
    mut disable_p: libc::c_int,
) -> libc::c_int {
    let mut b: *mut builtin = 0 as *mut builtin;
    b = builtin_address_internal(name, 1);
    if b.is_null() {
        return EXECUTION_FAILURE!();
    }
    if disable_p != 0 {
        (*b).flags &= !(BUILTIN_ENABLED);
        if !set_cmd_enable(CStr::from_ptr(name).to_string_lossy().into_owned(), false) {
            insert_empty_cmd(CStr::from_ptr(name).to_string_lossy().into_owned());
            set_cmd_enable(CStr::from_ptr(name).to_string_lossy().into_owned(), false);
            //get_cmd_enable(CStr::from_ptr(name).to_string_lossy().into_owned());
        }
    } else if restricted != 0 && (*b).flags & BUILTIN_ENABLED == 0 {
        sh_restricted(0 as *mut libc::c_void as *mut libc::c_char);
        return EXECUTION_FAILURE!();
    } else {
        (*b).flags |= BUILTIN_ENABLED;
        if !set_cmd_enable(CStr::from_ptr(name).to_string_lossy().into_owned(), true) {
            insert_empty_cmd(CStr::from_ptr(name).to_string_lossy().into_owned());
            set_cmd_enable(CStr::from_ptr(name).to_string_lossy().into_owned(), true);
        }
    } 
    set_itemlist_dirty(&mut it_enabled);
    set_itemlist_dirty(&mut it_disabled);
    return EXECUTION_SUCCESS!();
}
unsafe extern "C" fn dyn_load_builtin(
    mut list: *mut WordList,
    mut flags: libc::c_int,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut l: *mut WordList = 0 as *mut WordList;
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut total: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut new: libc::c_int = 0;
    let mut replaced: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut struct_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut loadfunc: Option::<sh_load_func_t> = None;
    let mut new_builtins: *mut *mut builtin = 0 as *mut *mut builtin;
    let mut b: *mut builtin = 0 as *mut builtin;
    let mut new_shell_builtins: *mut builtin = 0 as *mut builtin;
    let mut old_builtin: *mut builtin = 0 as *mut builtin;
    let mut loadables_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut load_path: *mut libc::c_char = 0 as *mut libc::c_char;
    if list.is_null() {
        return 1 as libc::c_int;
    }
    handle = 0 as *mut libc::c_void;
    if absolute_program(filename) == 0 as libc::c_int {
        loadables_path = get_string_value(
            b"BASH_LOADABLES_PATH\0" as *const u8 as *const libc::c_char,
        );
        if !loadables_path.is_null() {
            load_path = find_in_path(
                filename,
                loadables_path,
                0x20 as libc::c_int | 0x4 as libc::c_int,
            );
            if !load_path.is_null() {
                handle = dlopen(load_path, 0x1 as libc::c_int);
                free(load_path as *mut libc::c_void);
            }
        }
    }
    if handle.is_null() {
        handle = dlopen(filename, 0x1 as libc::c_int);
    }
    if handle.is_null() {
        name = printable_filename(filename, 0 as libc::c_int);
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot open shared object %s: %s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
            dlerror(),
        );
        if name != filename {
            free(name as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    new = 0 as libc::c_int;
    l = list;
    while !l.is_null() {
        l = (*l).next;
        new += 1;
    }
    new_builtins = xmalloc(
        (new as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut builtin>() as libc::c_ulong),
    ) as *mut *mut builtin;
    let mut current_block_57: u64;
    new = 0 as libc::c_int;
    replaced = new;
    while !list.is_null() {
        name = (*(*list).word).word;
        size = strlen(name) as libc::c_int;
        struct_name = xmalloc((size + 8 as libc::c_int) as size_t) as *mut libc::c_char;
        strcpy(struct_name, name);
        strcpy(
            struct_name.offset(size as isize),
            b"_struct\0" as *const u8 as *const libc::c_char,
        );
        old_builtin = builtin_address_internal(name, 1 as libc::c_int);
        b = dlsym(handle, struct_name) as *mut builtin;
        if b.is_null() {
            name = printable_filename(filename, 0 as libc::c_int);
            builtin_error(
                dcgettext(
                    0 as *const libc::c_char,
                    b"cannot find %s in shared object %s: %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                struct_name,
                name,
                dlerror(),
            );
            if name != filename {
                free(name as *mut libc::c_void);
            }
            free(struct_name as *mut libc::c_void);
        } else {
            funcname = xrealloc(
                struct_name as *mut libc::c_void,
                (size as libc::c_ulong)
                    .wrapping_add(
                        ::std::mem::size_of::<[libc::c_char; 14]>() as libc::c_ulong,
                    )
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as *mut libc::c_char;
            strcpy(funcname, name);
            strcpy(
                funcname.offset(size as isize),
                b"_builtin_load\0" as *const u8 as *const libc::c_char,
            );
            loadfunc = ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<sh_load_func_t>,
            >(dlsym(handle, funcname));
            if loadfunc.is_some() {
                if !old_builtin.is_null()
                    && (*old_builtin).flags & 0x4 as libc::c_int == 0 as libc::c_int
                {
                    builtin_warning(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: dynamic builtin already loaded\0" as *const u8
                                as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                    );
                }
                r = (Some(loadfunc.expect("non-null function pointer")))
                    .expect("non-null function pointer")(name);
                if r == 0 as libc::c_int {
                    builtin_error(
                        dcgettext(
                            0 as *const libc::c_char,
                            b"load function for %s returns failure (%d): not loaded\0"
                                as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        name,
                        r,
                    );
                    free(funcname as *mut libc::c_void);
                    current_block_57 = 15345278821338558188;
                } else {
                    current_block_57 = 7990025728955927862;
                }
            } else {
                current_block_57 = 7990025728955927862;
            }
            match current_block_57 {
                15345278821338558188 => {}
                _ => {
                    free(funcname as *mut libc::c_void);
                    (*b).flags &= !(0x4 as libc::c_int);
                    if flags & 4 as libc::c_int != 0 {
                        (*b).flags |= 0x8 as libc::c_int;
                    }
                    let ref mut fresh0 = (*b).handle;
                    *fresh0 = handle as *mut libc::c_char;
                    if !old_builtin.is_null() {
                        replaced += 1;
                        libc::memcpy(
                            old_builtin as *mut libc::c_char as *mut libc::c_void,
                            b as *mut libc::c_char as *const libc::c_void,
                            ::std::mem::size_of::<builtin>() as libc::c_ulong
                                as libc::size_t,
                        );
                    } else {
                        let fresh1 = new;
                        new = new + 1;
                        let ref mut fresh2 = *new_builtins.offset(fresh1 as isize);
                        *fresh2 = b;
                    }
                }
            }
        }
        list = (*list).next;
    }
    if replaced == 0 as libc::c_int && new == 0 as libc::c_int {
        free(new_builtins as *mut libc::c_void);
        dlclose(handle);
        return 1 as libc::c_int;
    }
    if new != 0 {
        total = num_shell_builtins + new;
        size = ((total + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<builtin>() as libc::c_ulong)
            as libc::c_int;
        new_shell_builtins = xmalloc(size as size_t) as *mut builtin;
        libc::memcpy(
            new_shell_builtins as *mut libc::c_char as *mut libc::c_void,
            shell_builtins as *mut libc::c_char as *const libc::c_void,
            (num_shell_builtins as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<builtin>() as libc::c_ulong)
                as libc::size_t,
        );
        replaced = 0 as libc::c_int;
        while replaced < new {
            libc::memcpy(
                &mut *new_shell_builtins.offset((num_shell_builtins + replaced) as isize)
                    as *mut builtin as *mut libc::c_char as *mut libc::c_void,
                *new_builtins.offset(replaced as isize) as *mut libc::c_char
                    as *const libc::c_void,
                ::std::mem::size_of::<builtin>() as libc::c_ulong as libc::size_t,
            );
            replaced += 1;
        }
        let ref mut fresh3 = (*new_shell_builtins.offset(total as isize)).name;
        *fresh3 = 0 as *mut libc::c_char;
        let ref mut fresh4 = (*new_shell_builtins.offset(total as isize)).function;
        *fresh4 = None;
        (*new_shell_builtins.offset(total as isize)).flags = 0 as libc::c_int;
        if shell_builtins != static_shell_builtins.as_mut_ptr() {
            free(shell_builtins as *mut libc::c_void);
        }
        shell_builtins = new_shell_builtins;
        num_shell_builtins = total;
        initialize_shell_builtins();
    }
    free(new_builtins as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn delete_builtin(mut b: *mut builtin) {
    let mut ind: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut new_shell_builtins: *mut builtin = 0 as *mut builtin;
    ind = b.offset_from(shell_builtins) as libc::c_long as libc::c_int;
    size = (num_shell_builtins as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<builtin>() as libc::c_ulong) as libc::c_int;
    new_shell_builtins = xmalloc(size as size_t) as *mut builtin;
    if ind != 0 {
        libc::memcpy(
            new_shell_builtins as *mut libc::c_char as *mut libc::c_void,
            shell_builtins as *mut libc::c_char as *const libc::c_void,
            (ind as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<builtin>() as libc::c_ulong)
                as libc::size_t,
        );
    }
    libc::memcpy(
        &mut *new_shell_builtins.offset(ind as isize) as *mut builtin
            as *mut libc::c_char as *mut libc::c_void,
        &mut *shell_builtins.offset((ind + 1 as libc::c_int) as isize) as *mut builtin
            as *mut libc::c_char as *const libc::c_void,
        ((num_shell_builtins - ind) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<builtin>() as libc::c_ulong)
            as libc::size_t,
    );
    if shell_builtins != static_shell_builtins.as_mut_ptr() {
        free(shell_builtins as *mut libc::c_void);
    }
    num_shell_builtins -= 1;
    shell_builtins = new_shell_builtins;
}
unsafe extern "C" fn local_dlclose(mut handle: *mut libc::c_void) -> libc::c_int {
    return dlclose(handle);
}
unsafe extern "C" fn dyn_unload_builtin(mut name: *mut libc::c_char) -> libc::c_int {
    let mut b: *mut builtin = 0 as *mut builtin;
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut funcname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut unloadfunc: Option::<sh_unload_func_t> = None;
    let mut ref_0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    b = builtin_address_internal(name, 1 as libc::c_int);
    if b.is_null() {
        sh_notbuiltin(name);
        return 1 as libc::c_int;
    }
    if (*b).flags & 0x4 as libc::c_int != 0 {
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: not dynamically loaded\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
        );
        return 1 as libc::c_int;
    }
    handle = (*b).handle as *mut libc::c_void;
    i = 0 as libc::c_int;
    ref_0 = i;
    while i < num_shell_builtins {
        if (*shell_builtins.offset(i as isize)).handle == (*b).handle {
            ref_0 += 1;
        }
        i += 1;
    }
    size = strlen(name) as libc::c_int;
    funcname = xmalloc(
        (size as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    strcpy(funcname, name);
    strcpy(
        funcname.offset(size as isize),
        b"_builtin_unload\0" as *const u8 as *const libc::c_char,
    );
    unloadfunc = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<sh_unload_func_t>,
    >(dlsym(handle, funcname));
    if unloadfunc.is_some() {
        (Some(unloadfunc.expect("non-null function pointer")))
            .expect("no-null function pointer")(name);
    }
    free(funcname as *mut libc::c_void);
    if ref_0 == 1 as libc::c_int && local_dlclose(handle) != 0 as libc::c_int {
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"%s: cannot delete: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            name,
            dlerror(),
        );
        return 1 as libc::c_int;
    }
    delete_builtin(b);
    return 0 as libc::c_int;
}
