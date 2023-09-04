extern crate  libc;
extern crate nix;

use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::format;
use std::ptr;
use std::mem;
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage};
use rhelp::r_builtin_help;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}


#[repr (C)]
#[derive(Copy,Clone)]
pub struct RESOURCE_LIMITS{
     option : i32,           	/* The ulimit option for this limit. */
     parameter : i32,            /* Parameter to pass to get_limit (). */
     block_factor :  i32,         /* Blocking factor for specific limit. */
     description : *const libc::c_char,    /* Descriptive string to output. */
     units : *const libc::c_char           /* scale */
}

#[repr (C)]
#[derive(Copy,Clone)]
pub struct _cmd {
     cmd :  i32,
    arg : *mut libc::c_char
} 

#[repr (C)]
#[derive(Copy,Clone)]
/* Information about the current user. */
pub struct user_info {
    uid : uid_t,
    euid : uid_t,
    gid : gid_t,
    egid : gid_t,
    user_name : *mut libc::c_char,
    shell :*mut libc::c_char,
    home_dir : *mut  libc::c_char
}

#[macro_export]
macro_rules!  SIZEOFLIMIT{
    () => {
    std::mem::size_of::<RESOURCE_LIMITS>() as usize
    }  
}

#[macro_export]
macro_rules!  SIZEOFLIMITS{
    () => {
        SIZEOFLIMIT!() *18
    }  
}

#[macro_export]
macro_rules! SIZEOFULCMD{
    () => {
        std::mem::size_of::<cmdlist>()
    }
}

#[macro_export]
macro_rules!  LIMIT_HARD{
    () => {
        0x01  
    }
}

#[macro_export]
macro_rules!  LIMIT_SOFT{
    () => {
        0x02  
    }
}


#[macro_export]
macro_rules! POSIXBLK{
    () => {
      -2
    }
}

#[macro_export]
macro_rules! BLOCKSIZE{
    ($s:expr) => {
        if $s == POSIXBLK!() {
            if unsafe{posixly_correct}!= 0 {
                512
            }
            else {
                1024
            }
        }
        else {
            $s
        }     
    }
}

#[macro_export]
macro_rules! RLIM_SAVED_MAX{
    () => {
        RLIM_INFINITY!();
    }
}


#[deny(missing_fragment_specifier)]
#[macro_export]
macro_rules!  STREQ{
   ($a:expr,$b:expr) =>{
       $a==$b && libc::strcmp($a,$b)==0
    }
}

#[macro_export]
macro_rules!  NCMDS {
    () => {
        SIZEOFLIMITS!() / SIZEOFLIMIT!() 
    }
}

#[macro_export]
macro_rules!  RLIMIT_FILESIZE {
    () => {
        1
    }
}

#[macro_export]
macro_rules! RLIMIT_PIPESIZE {
    () =>  {
        257
    }
}

#[macro_export]
macro_rules! PIPESIZE {
    () =>  {
        4096
    }
}

#[macro_export]
macro_rules! PIPE_BUF {
    () =>  {
        PIPESIZE!()
    }
}

#[macro_export]
macro_rules! RLIMIT_OPENFILES{
    () => {
        7
    }
}

#[macro_export]
macro_rules!  RLIMIT_VIRTMEM{
    () => {
        9
    }
}

#[macro_export]
macro_rules! RLIMIT_MAXUPROC{
    () => {
        6
    }
}

#[macro_export]
macro_rules! RLIM_INFINITY {
    () => {
        -1
        //0x7fffffff
    }
}

#[macro_export]
macro_rules!  RLIM_SAVED_CUR{
    () => {
        RLIM_INFINITY!()
    }
}

type  RLIMTYPE = i64;
type  RESOURCE_LIMITS_T= RESOURCE_LIMITS;
type  ULCMD =  _cmd;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type __rlim_t = u64;
pub type rlim_t = __rlim_t;
pub type __rlimit_resource_t = __rlimit_resource;
pub type __uid_t = i32;
pub type __gid_t = i32;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;

static mut cmdlistsz : i32  = 0;

const  limits: [ RESOURCE_LIMITS_T;18] =[

    {   RESOURCE_LIMITS {
        option: 'R' as i32,
        parameter: __RLIMIT_RTTIME as i32,
        block_factor: 1 as i32,
        description: b"real-time non-blocking time\0" as *const u8
            as *const libc::c_char,
        units: b"microseconds\0" as *const u8 as *const libc::c_char,
    }},
   
    {   RESOURCE_LIMITS {
        option: 'c' as i32,
        parameter: RLIMIT_CORE as i32,
        block_factor: -(2 as i32),
        description: b"core file size\0" as *const u8 as *const libc::c_char,
        units: b"blocks\0" as *const u8 as *const libc::c_char,
    }},
   
    {   RESOURCE_LIMITS {
        option: 'd' as i32,
        parameter: RLIMIT_DATA as i32,
        block_factor: 1024 as i32,
        description: b"data seg size\0" as *const u8 as *const libc::c_char,
        units: b"kbytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'e' as i32,
        parameter: __RLIMIT_NICE as i32,
        block_factor: 1 as i32,
        description: b"scheduling priority\0" as *const u8 as *const libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'f' as i32,
        parameter: RLIMIT_FSIZE as i32,
        block_factor: -(2 as i32),
        description: b"file size\0" as *const u8 as *const libc::c_char,
        units: b"blocks\0" as *const u8 as *const libc::c_char,
    }},
    
    {
        RESOURCE_LIMITS {
            option: 'i' as i32,
            parameter: __RLIMIT_SIGPENDING as i32,
            block_factor: 1 as i32,
            description: b"pending signals\0" as *const u8 as *const libc::c_char,
            units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        }
    },

    {   RESOURCE_LIMITS {
        option: 'l' as i32,
        parameter: __RLIMIT_MEMLOCK as i32,
        block_factor: 1024 as i32,
        description: b"max locked memory\0" as *const u8 as *const libc::c_char,
        units: b"kbytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'm' as i32,
        parameter: __RLIMIT_RSS as i32,
        block_factor: 1024 as i32,
        description: b"max memory size\0" as *const u8 as *const libc::c_char,
        units: b"kbytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'n' as i32,
        parameter: RLIMIT_NOFILE as i32,
        block_factor: 1 as i32,
        description: b"open files\0" as *const u8 as *const libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'p' as i32,
        parameter: 257 as i32,
        block_factor: 512 as i32,
        description: b"pipe size\0" as *const u8 as *const libc::c_char,
        units: b"512 bytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'q' as i32,
        parameter: __RLIMIT_MSGQUEUE as i32,
        block_factor: 1 as i32,
        description: b"POSIX message queues\0" as *const u8 as *const libc::c_char,
        units: b"bytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'r' as i32,
        parameter: __RLIMIT_RTPRIO as i32,
        block_factor: 1 as i32,
        description: b"real-time priority\0" as *const u8 as *const libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 's' as i32,
        parameter: RLIMIT_STACK as i32,
        block_factor: 1024 as i32,
        description: b"stack size\0" as *const u8 as *const libc::c_char,
        units: b"kbytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 't' as i32,
        parameter: RLIMIT_CPU as i32,
        block_factor: 1 as i32,
        description: b"cpu time\0" as *const u8 as *const libc::c_char,
        units: b"seconds\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'u' as i32,
        parameter: __RLIMIT_NPROC as i32,
        block_factor: 1 as i32,
        description: b"max user processes\0" as *const u8 as *const libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'v' as i32,
        parameter: RLIMIT_AS as i32,
        block_factor: 1024 as i32,
        description: b"virtual memory\0" as *const u8 as *const libc::c_char,
        units: b"kbytes\0" as *const u8 as *const libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: 'x' as i32,
        parameter: __RLIMIT_LOCKS as i32,
        block_factor: 1 as i32,
        description: b"file locks\0" as *const u8 as *const libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }},

    {   RESOURCE_LIMITS {
        option: -1 ,
        parameter: -1,
        block_factor:-1,
        description: 0 as *const libc::c_void as *mut libc::c_void
            as *mut libc::c_char,
        units: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
    }}
    ];

extern "C" {
    fn reset_internal_getopt();
    fn xmalloc(_: u64) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn all_digits(_: *const libc::c_char) -> i32;
    fn sh_chkwrite(_: i32) -> i32;
    fn internal_getopt(_: *mut WordList, _: *mut libc::c_char) -> i32;
    fn strerror(_: i32) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> i32;
    fn string_to_rlimtype(_: *mut libc::c_char ) -> rlim_t;
    fn getdtablesize() -> i32;
    fn builtin_usage();
    fn sh_erange (s:* mut libc::c_char, desc:* mut libc::c_char);
    fn sh_invalidnum(arg1: *mut libc::c_char);
    fn __errno_location() -> *mut i32;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> i32;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> i32;

    fn builtin_error(_: *const libc::c_char, _: ...);
    fn getmaxchild() -> i64;
    
    static mut loptend: *mut WordList;
    static mut list_optarg: *mut libc::c_char;
    static mut posixly_correct:i32 ;
    static mut current_user: user_info;
}

static mut optstring:[ libc::c_char;4 + 2 * NCMDS!() as usize] = [0;4 + 2 * NCMDS!() as usize];
static mut cmdlist : *mut ULCMD = 0 as *const ULCMD as *mut ULCMD;
static mut ncmd : i32  = 0;

fn _findlim (opt:i32) -> i32{
  //  let mut register : i32;
    //let i : i32 = 0;

    for i in 0..17 {
        if limits[i].option > 0{
            if limits[i].option  == opt {
                return i  as i32;
            }
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn r_ulimit_builtin(mut list: *mut WordList) -> i32{

    let mut  s : *mut libc::c_char;
    let mut c : i32 ;
    let mut limind : i32 ;
    let mut mode : i32 = 0 ;
    let mut opt : i32 = 0 ;
    let  mut all_limits : i32 = 0 ;
    if optstring[0] == 0 {
         s = optstring.as_mut_ptr();
         s = s.offset(0);
        *s = 'a' as libc::c_char;  
         s = s.offset(1);
        *s  = 'S' as libc::c_char;
         s =  s.offset(1);
        *s   = 'H' as libc::c_char;
         s =  s.offset(1);
         c = 0 ;
        for i in 0..17 {
            if limits[i].option > 0{
                *s = limits[i].option as libc::c_char;
                 s =  s.offset(1);
                *s = ';' as  libc::c_char;
                 s =  s.offset(1);
            } 
        }
        *s = '\0' as  libc::c_char;
    }

    if cmdlistsz == 0{
       cmdlistsz = 16;
        unsafe {
            cmdlist = 
             xmalloc ((cmdlistsz as u64)*(std::mem::size_of::<ULCMD>() as libc::c_ulong) ) as *mut ULCMD;
        }
    }
    ncmd = 0;
    reset_internal_getopt ();
    opt = internal_getopt(list, optstring.as_ptr() as *mut libc::c_char);
    while opt != -1 {
        let optu8:u8= opt as u8;
        let optChar:char=char::from(optu8);
        match optChar {
            'a' => { all_limits  = all_limits + 1 ;}
            'S' => { mode = mode | LIMIT_SOFT!() ; }
            'H' => { mode = mode | LIMIT_HARD!();}
            '?'=> {   
                 builtin_usage();
                 return  EX_USAGE;
            }
            _ => {
                if opt == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                }
                if ncmd >= cmdlistsz {
                    cmdlistsz = cmdlistsz * 2 ;
                    cmdlist = xrealloc(
                        cmdlist as *mut libc::c_void,(cmdlistsz as u64) * std::mem::size_of::<ULCMD>() as u64 ) as *mut ULCMD;
                }
                unsafe {
                    (*cmdlist.offset(ncmd as isize)).cmd = opt;
                    let fresh5 = ncmd;
                    //ncmd = ncmd + 1;
                    let ref mut fresh6 = (*cmdlist.offset(fresh5 as isize)).arg;
                    *fresh6 = list_optarg;
                    // let mut cmm =&mut  (*((cmdlist as usize + 
                    //                     (ncmd as usize)*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD);
                    // cmm.cmd = opt;
                    // cmm.arg = list_optarg;
                    // (*((cmdlist as usize + (ncmd as usize)*std::mem::size_of::<ULCMD>())
                    // as *mut ULCMD) as ULCMD).cmd = opt ;
                    //  (*((cmdlist as usize + (ncmd as usize) * std::mem::size_of::<ULCMD>())
                    //  as *mut ULCMD) as ULCMD).arg = list_optarg;
                    ncmd = ncmd+1;
                }   

            }
        }
        opt = internal_getopt (list, optstring.as_ptr() as * mut libc::c_char);
  }

    //  as *mut ULCMD) as ULCMD).cmd );
    list = loptend;

   if  all_limits != 0 {
      if mode == 0  {
        print_all_limits (LIMIT_SOFT!());
      }
      else {
        print_all_limits (mode);
      }
    return sh_chkwrite(EXECUTION_SUCCESS!());
  }
 
   if ncmd == 0 {
      unsafe {
        (*cmdlist.offset(ncmd as isize)).cmd = 'f' as i32;
        //   let mut cmm =  *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
        //   cmm.cmd = 'f' as i32;
      }
    /* `ulimit something' is same as `ulimit -f something' */
      if !list.is_null() {
          unsafe { 
            (*cmdlist.offset(ncmd as isize)).arg =  (*(*list).word).word;
            // let mut cmm =  *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
            // cmm.arg =  (*(*list).word).word;
            ncmd = ncmd+1;
          }
        }
       else {
            unsafe {
            (*cmdlist.offset(ncmd as isize)).arg =  std::ptr::null_mut();  
            // let mut cmm = *((cmdlist as usize + (ncmd as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
            // cmm.arg  =  std::ptr::null_mut();    
            ncmd = ncmd+1;        
        }

    }
    if !list.is_null() {
        list = (*list).next;
    }
  }

  for d in 0..ncmd {
    //as *mut ULCMD) as ULCMD).cmd);
      let cmm = *((cmdlist as usize + (d as usize )*std::mem::size_of::<ULCMD>())as *mut ULCMD) as ULCMD;
      let dmd = cmm.cmd;

      limind = _findlim ((*cmdlist.offset(d as isize)).cmd);
      if limind == -1 {
        unsafe {
            builtin_error(b"%s: bad command : %s\0" as *const u8 as  *const libc::c_char, 
            (*cmdlist.offset(d as isize)).cmd, 
            strerror(*__errno_location()) as *const libc::c_char);
        }
        return EX_USAGE;
      }
  }

  for d in 0..ncmd {
    let dmd = (*cmdlist.offset(d as isize)).cmd;
    let  drg = (*cmdlist.offset(d as isize)).arg;
    // let dmd =   (*((cmdlist as usize + (d as usize )*std::mem::size_of::<ULCMD>())
    // as *mut ULCMD) as ULCMD).cmd;
    // let drg =  (*((cmdlist as usize + (d as usize )*std::mem::size_of::<ULCMD>())
    // as *mut ULCMD) as ULCMD).arg;
    if (ulimit_internal (dmd,drg, mode, d-1))  == EXECUTION_FAILURE!() {
        return EXECUTION_FAILURE!();
       }
    }
      return EXECUTION_SUCCESS!();
  
}

unsafe fn ulimit_internal (cmd : i32 , cmdarg :*mut libc::c_char,mut  mode : i32, multiple : i32) -> i32 {
    let mut opt : i32 ;
    let mut limind : i32 ;
    let mut setting : i32 ;
    let mut block_factor : i32 ;
    let mut soft_limit : RLIMTYPE = 0;
    let mut  hard_limit : RLIMTYPE  =0;
    let mut real_limit : RLIMTYPE = 0;
    let mut limit : RLIMTYPE;
   
    if cmdarg != std::ptr::null_mut() {
        setting = 1;
    }
    else {
        setting = 0;
    }
    limind = _findlim(cmd);
    if mode == 0 {
        if setting != 0 {
            mode = LIMIT_HARD!()|LIMIT_SOFT!();
        }
        else {
            mode = LIMIT_SOFT!();
        }
    }
  opt = get_limit (limind, &mut soft_limit, &mut hard_limit);

  if opt < 0 {
    unsafe {
        builtin_error(b"%s: cannot get limit : %s\0" as *const u8 as  *const libc::c_char,  limits[limind as usize].description, 
         strerror(*__errno_location()) as *const libc::c_char);
    }
 
    return EXECUTION_FAILURE!();
  }

  if setting == 0 {
      if (mode & LIMIT_SOFT!()) != 0 {   
        printone (limind,soft_limit,multiple);
      }
      else {
        printone (limind,hard_limit,multiple);
      }
    return EXECUTION_SUCCESS!();
  }

  let mut c_str_hard = CString::new("hard").unwrap();
  let mut c_str_soft = CString::new("soft").unwrap();
  let mut c_str_unlimited = CString::new("unlimited").unwrap();
  if unsafe{STREQ!(cmdarg,c_str_hard.as_ptr() as *mut libc::c_char )}{
    real_limit = hard_limit;
  }

  else if unsafe{STREQ!(cmdarg, c_str_soft.as_ptr() as *mut libc::c_char)}{
    real_limit = soft_limit;
  }
  else if unsafe{STREQ!(cmdarg, c_str_unlimited.as_ptr() as *mut libc::c_char)}{
    real_limit = RLIM_INFINITY!();
  }

  else if unsafe {all_digits(cmdarg)} !=0 {
    limit = unsafe {string_to_rlimtype (cmdarg) as i64};
    block_factor =  BLOCKSIZE!(limits[limind as usize].block_factor);
    real_limit = limit * block_factor as i64;

    if (real_limit / block_factor as i64) != limit {
        let c_str_limit =CString::new("limit").unwrap();
	    unsafe {sh_erange (cmdarg,c_str_limit.as_ptr() as *mut libc::c_char)};
	    return EXECUTION_FAILURE!();
	}
  }

  else {
    sh_invalidnum (cmdarg);
    return EXECUTION_FAILURE!();
  }
    if set_limit (limind, real_limit, mode) < 0 {
            builtin_error(b"%s: cannot modify limit : %s\0" as *const u8 as  *const libc::c_char,  limits[limind as usize].description, 
             strerror(*__errno_location()) as *const libc::c_char);
    return EXECUTION_FAILURE!();
    }
    return EXECUTION_SUCCESS!();

}

fn get_limit (mut ind : i32, softlim : *mut RLIMTYPE, hardlim : *mut RLIMTYPE ) -> i32 { 
    let mut value :  RLIMTYPE = 0 ;
    let mut limit: rlimit = rlimit { rlim_cur: 1, rlim_max: 1 };

    if limits[ind as usize].parameter >= 256 {
        match limits[ind as usize].parameter {
            RLIMIT_FILESIZE!() => {
                if filesize (((&mut value)  as *mut i64) as *mut u64) < 0 {
                    return -1;
                }
            }
            RLIMIT_PIPESIZE!() => {
                if unsafe {
                    pipesize (((&mut value)  as *mut i64) as *mut u64)} < 0 {
                        return -1;
                    }
                
            }
            RLIMIT_OPENFILES!() => {
                value = unsafe {getdtablesize()} as RLIMTYPE ;
                
            }
            RLIMIT_VIRTMEM!() => {
                return unsafe {getmaxvm(softlim, hardlim as *mut libc::c_char) };
            }
            RLIMIT_MAXUPROC!() => {
                if getmaxuprc ((value as usize) as *mut u64) < 0 {
                    return -1;
                }
              
            }
            _ => {
                unsafe {
                    *__errno_location() = libc::EINVAL;
                }
            }
        }
        unsafe {
            *softlim =  value;
            *hardlim = value;
        }
        return 0;
      }
    else{
        unsafe {
        let ii = getrlimit(limits[ind as u32 as usize ].parameter as __rlimit_resource_t,
              &mut limit);
        if  ii < 0 {
            return -1;
        } 
        }
        unsafe {
           // limit.rlim_max as i64);
            *softlim = limit.rlim_cur as i64;
            *hardlim = limit.rlim_max as i64;
        }
        return 0;
    }
}

fn  set_limit (ind : i32, newlim : RLIMTYPE, mode : i32) -> i32{
    let  mut limit  : rlimit  = rlimit { rlim_cur: 0, rlim_max: 0 };
    let  mut val :  RLIMTYPE = 0;
     
    if limits[ind as usize].parameter >= 256 {
        match limits[ind as usize].parameter {
            RLIMIT_FILESIZE!() => {
                unsafe {
                    *__errno_location() = libc::EINVAL;
                }
	            return -1;
            }
            RLIMIT_OPENFILES!() | RLIMIT_PIPESIZE !() |
            RLIMIT_VIRTMEM!() | RLIMIT_MAXUPROC !() |
             _ => {
                unsafe {
                    *__errno_location() = libc::EINVAL;
                }
                return -1;
            }
        }
    }
    else{
        if unsafe {
            getrlimit (limits[ind as usize].parameter 
                as __rlimit_resource_t,&mut limit )
            } < 0 {
                return  -1;
        }
        let b =  unsafe {current_user.euid }!= 0  && newlim == RLIM_INFINITY!() 
                 && (mode & LIMIT_HARD!()) == 0  && limit.rlim_cur <= limit.rlim_max;
        if b {
            val = limit.rlim_max as i64; 
        }
        else {
            val =  newlim;
        }
        if mode & LIMIT_SOFT!() != 0 {
            limit.rlim_cur = val as u64;
        }
        if mode & LIMIT_HARD!() != 0 {
            limit.rlim_max = val as u64;
        }
        return 
        unsafe {setrlimit(limits[ind as usize].parameter as __rlimit_resource_t, &mut limit)};
    }  
}

unsafe fn getmaxvm(softlim : *mut RLIMTYPE , hardlim : *mut libc::c_char) -> i32 {
    let  mut  datalim :  rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    let  mut  stacklim : rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    
    if getrlimit(RLIMIT_DATA, &mut datalim) < 0 {
        return -1;
    }
    if getrlimit (RLIMIT_STACK, &mut  stacklim) < 0 {
        return -1;
    }    
        *softlim = (datalim.rlim_cur as i64 / 1024 as i64) + (stacklim.rlim_cur as i64/1024 as i64);
        *hardlim = ((datalim.rlim_max as i64) /1024 as i64) as libc::c_char + (stacklim.rlim_max as i64/1024 as i64) as libc::c_char;
    return 0;
}
 
fn filesize(mut valuep: *mut rlim_t) -> i32 {
    unsafe {
        *__errno_location() = libc::EINVAL;
    }
    return -1;
}

unsafe fn pipesize(mut valuep: *mut rlim_t) -> i32 {
    *((valuep as usize) as *mut rlim_t) =  PIPE_BUF!() as rlim_t;
    return 0 ;
}

fn getmaxuprc(mut valuep: *mut rlim_t) -> i32 {
    let mut maxchild: i64 = 0;
    maxchild = unsafe{getmaxchild()};
    if maxchild < 0 as i32 as libc::c_long {
        unsafe {
            *__errno_location() = libc::EINVAL;
        }
        return -1;
    } else {
        unsafe {
            *valuep = maxchild as rlim_t;
        }        
        return 0 ;
    };
}

fn print_all_limits (mut mode : i32) {
    let mut  i : i32 ;
    let mut softlim : RLIMTYPE = 0;
    let mut hardlim : RLIMTYPE = 0;
  
    if mode == 0
    {
        mode = mode | LIMIT_SOFT!();
    }
    i = 0;
    while limits[i as usize].option >0 {

        if get_limit(i, &mut softlim, &mut hardlim) == 0 {
            if mode & LIMIT_SOFT!() != 0 {
                printone(i,softlim,1);
            }
            else {
                printone(i, hardlim,1);
            }
        }
        else if unsafe {
            *__errno_location() != libc::EINVAL } {
                unsafe {
                    builtin_error(b"%s: cannot get limit : %s\0" as *const u8 as  *const libc::c_char,  limits[i as usize].description, 
                     strerror(*__errno_location()) as *const libc::c_char);
                }
        }
     i = i+1;
    }
}

fn  printone (limind : i32, curlim :RLIMTYPE  , pdesc : i32){
    let mut unitstr :[ libc::c_char; 64] = [0 ; 64];
    let mut factor : i32 ;

    factor = BLOCKSIZE!(limits[limind as usize].block_factor);
    if pdesc > 0 {
        if !limits[limind as usize].units.is_null(){
            unsafe {
                sprintf (unitstr.as_mut_ptr(), b"(%s, -%c) \0" as *const u8 as *const libc::c_char,
                limits[limind as usize].units, 
                limits[limind as usize].option);
            }
         
        }
        else {
            unsafe {
                sprintf (unitstr.as_mut_ptr(),b"(-%c) \0" as *const u8 as *const libc::c_char,
                limits[limind as usize].option);
            }
        }
        print!("{:<20} {:>20}", unsafe {
            CStr::from_ptr(limits[limind as usize].description).to_str().unwrap()}
        , unsafe {CStr::from_ptr(unitstr.as_mut_ptr()).to_str().unwrap()});
    }
    if curlim == RLIM_INFINITY!() {
        let c_str_unlimited = b"unlimited" as *const u8 as *const libc::c_char;
        println!("{}",unsafe {CStr::from_ptr(c_str_unlimited).to_str().unwrap()});
    }
  
    else if curlim == RLIM_SAVED_MAX!() {
        //println!("hard");
        let c_str_hard = b"hard" as *const u8 as *const libc::c_char;
        println!("{}",unsafe {CStr::from_ptr(c_str_hard).to_str().unwrap()});
    }
    else if curlim == RLIM_SAVED_CUR!() {
        //println!("soft");
        let c_str_soft = b"soft" as *const u8 as *const libc::c_char;
        println!("{}",unsafe {CStr::from_ptr(c_str_soft).to_str().unwrap()});
    }  
    else{
        print_rlimtype ((curlim / factor as i64) as u64 , 1); 
    }
   
}

/* Set all limits to NEWLIM.  NEWLIM currently must be RLIM_INFINITY, which
   causes all limits to be set as high as possible depending on mode (like
   csh `unlimit').  Returns -1 if NEWLIM is invalid, 0 if all limits
   were set successfully, and 1 if at least one limit could not be set.

   To raise all soft limits to their corresponding hard limits, use
	ulimit -S -a unlimited
   To attempt to raise all hard limits to infinity (superuser-only), use
	ulimit -H -a unlimited
   To attempt to raise all soft and hard limits to infinity, use
	ulimit -a unlimited
*/

fn  print_rlimtype(num : u64, nl : i32) 
{
    if nl > 0{
        println!("{num}");
    }
    else {
        print!("{num}");
    }
}


fn  set_all_limits (mut mode : i32 , newlim : RLIMTYPE) -> i32 {
    let mut  i : i32 ;
    let mut retval : i32  = 0;
  
    if newlim != RLIM_INFINITY!() {
        unsafe {
            *__errno_location() = libc::EINVAL;
        }
        return -1;
    }

    if mode == 0 {
        mode = LIMIT_SOFT!()|LIMIT_HARD!();
    }
    retval = 0 ;
    i = 0;

    while limits[i as usize].option > 0 {
        if set_limit (i, newlim, mode) < 0 {
            unsafe {
                builtin_error(b"%s: cannot modify limit : %s\0" as *const u8 as  *const libc::c_char,  limits[i as usize].description, 
                 strerror(*__errno_location()) as *const libc::c_char);
            }
	        retval = 1;
            i = i +1;
        }
    }
  return retval;
}


