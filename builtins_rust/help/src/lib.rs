extern crate  libc;
extern crate nix;
extern crate std;
use libc::{c_char,  c_void ,putchar, free};
use std::{ffi::{CString,CStr}, i32, io::{Read, stdout, Write}, mem, string, u32};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, 
  EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,
  r_builtin_usage,get_local_str};

use fluent_bundle::{FluentBundle, FluentResource, FluentValue, FluentArgs};
use fluent_resmgr::resource_manager::ResourceManager;
pub enum Option<T> {
    None,
    Some(T),
}

#[repr (C)]
#[derive(Copy,Clone)]
pub struct builtin {
   name : *mut libc::c_char,
   function :*mut sh_builtin_func_t,
   flags : bool,
   long_doc :*mut *mut c_char,
   short_doc :*mut  libc::c_char,
   handle :*mut libc::c_char
}
type sh_builtin_func_t = fn(WordList) -> i32;

#[repr(C)]
struct FieldStruct {
    name : *mut  c_char,
}

#[macro_export]
macro_rules! FNMATCH_EXTFLAG {
    () => {0}
}

#[macro_export]
macro_rules! EX_USAGE {
   () => {258}
}

#[macro_export]
macro_rules! MB_CUR_MAX	{
     () => {6}
 }

#[macro_export]
macro_rules! BASE_INDENT{
    () => {4}
}

#[macro_export]
macro_rules! BUILTIN_ENABLED{
    () => {1}
}

#[macro_export]
macro_rules! FNM_NOMATCH{
    () => {1}
}

#[macro_export]
macro_rules! BUILTIN_SIZEOF{
    () => {48}
}

#[macro_export]
macro_rules! EXIT_FAILURE{
  () => {1}
}

extern "C"{
    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList , opts:*mut c_char)->i32;
    //fn builtin_error(err:*const c_char,...);
    fn builtin_usage();
    fn show_shell_version(ver:i32);
    fn glob_pattern_p(pattern:*const c_char) -> i32;    
    fn zcatfd(fd : i32 ,id : i32, nn :*mut c_char) -> i32;
    fn zmapfd(fd : i32, name :*mut *mut libc::c_char, nn: *mut libc::c_char) -> i32;
    fn sh_builtin_func_t(list :*mut WordList) -> i32;
    fn  builtin_address_internal(comand_name:*mut c_char, i:i32) -> *mut  builtin;
    fn termsig_handler (sig:i32); 
    fn throw_to_top_level();
    fn default_columns() -> usize;
    fn wcsnwidth (chaa : * mut libc::wchar_t, size :i32, i: i32) -> i32;
    fn xstrmatch (string1 : * mut libc::c_char, string2 : * mut libc::c_char, i : libc::c_char) -> libc::c_char;
    fn open(pathname : *const libc::c_char, oflag : i32) -> i32;
    fn wcwidth( c :libc::wchar_t) -> i32;
    static mut loptend:*mut WordList;
    static bash_copyright : *const c_char;
    static bash_license : *const c_char;
    static mut terminating_signal:i32;
    static this_command_name:*mut libc::c_char;
    static mut interrupt_state:i32;
    static mut num_shell_builtins : i32;
    static mut static_shell_builtin : [builtin ; 100];
    static  shell_builtins:*mut  builtin;
    static  mut current_builtin :*mut builtin;
}

#[no_mangle]
pub extern "C" fn r_help_builtin(mut list:*mut WordList)->i32 {
   // let mut i:i32;
    let mut plen:usize;
    let mut match_found:i32;
    let mut sflag :i32 =  0;
    let mut dflag : i32 = 0;
    let mut mflag : i32 = 0;
    let mut m: bool;
    let  pass:i32 = 0;
    let mut this_found:i32;
    let mut pattern:*mut c_char;
    let mut name:*mut c_char; 
    let  l:*mut WordList= list;
    let  mut  i : i32;
    unsafe {
        reset_internal_getopt();
    }
    let c_str_dms = CString::new("dms").unwrap(); // from a &str, creates a new allocation
     unsafe {
       i = internal_getopt (list, c_str_dms.as_ptr() as * mut c_char);
     }
    while i != -1 {
        let optu8:u8= i as u8;
        let optChar:char=char::from(optu8);
        match optChar{
           'd'=> {dflag = 1; }
           'm'=> {mflag = 1; }
           's'=> {sflag = 1; }
            _=>{
                unsafe {
                  if i == -99 {
                    r_builtin_help();
                    return EX_USAGE;
                  }
                  builtin_usage ();
                  return EX_USAGE;
                }
            }
        } 
        unsafe{
          i = internal_getopt (list, c_str_dms.as_ptr() as * mut c_char);    
        }
    }
    if list == std::ptr::null_mut(){
      unsafe {
        show_shell_version (0);
      }
      show_builtin_command_help ();
      return EXECUTION_SUCCESS!();
    }
   unsafe {
    let mut  pattern = 0;
   pattern =  glob_pattern_p ((*(*list).word).word);
   if pattern == 1 {
       println!("Shell commands matching keyword, Shell commands matching keyword");
       if  list != std::ptr::null_mut() && (*list).next !=std::ptr::null_mut() {
            println!("Shell commands matching keywords");
       }
       else {
             println!("Shell commands matching keyword");
       }
       println!("{:?} ,",list);
   }
  //  let mut  loptendt=*list;

   let mut match_found = 0;
   let mut pattern:*mut c_char =  0 as *mut libc::c_char;
   while list !=std::ptr::null_mut() {
       pattern = (*(*list).word).word;
       plen = libc::strlen (pattern);
       let mut  this_found = 0;
       let mut v : Vec<*mut libc::c_char> = Vec::new();
       for val in 0..=75 {
           //let nname = &shell_builtins[val].name;
           let  builtin1  = unsafe{&(*((shell_builtins as usize + (val*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
           if  builtin1.name != std::ptr::null_mut(){
               v.push(builtin1.name);
           }
       }
       for val in 1..3 {
           //for &mut namee in &mut v {
           for  i in  0..v.len(){
                QUIT();
               /* First val: look for exact string or pattern matches.
                 Second val: look for prefix matches like bash-4.2 */
              if val == 1{
                  m = (libc::strcmp (pattern,v[i]) == 0)||
                    (strmatch (pattern,v[i], FNMATCH_EXTFLAG!()) != FNM_NOMATCH!());
              }
              else{
                 m = libc::strncmp (pattern, v[i], plen) == 0;
              }
              if m {
                  this_found = 1;
                  match_found = match_found +1 ;
                  if dflag == 1{
                      show_desc (i as i32);
                      continue;
                  }
                  else if mflag ==1{
                    show_manpage (v[i], i as  i32); 
                    continue;
                    }
                    let  builtin1 = unsafe{&(*((shell_builtins as usize + (i*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
                    print!("{:?}:",CStr::from_ptr(builtin1.name));
                    show_helpsynopsis(i as i32);
                    if sflag == 0{
                      show_longdoc(i as i32);    
                  }
                }
              }
              if val == 1 && this_found == 1{
                 break;
              }
       }
      if (*list).next != std::ptr::null_mut(){
       list = (*list).next;

      }
      else {

        break;
      }
   }
  if match_found == 0{
        let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
        let resources = vec![ "message.ftl".into()];
        let mut args = FluentArgs::new();
        let s1 = String::from("command");
        args.set("name",format!("{:?}",CStr::from_ptr(pattern)));
        let bundle = mgr.get_bundle(get_local_str(), resources);
        let mut value = bundle.get_message("helperr").unwrap();
        let mut pattern = value.value().expect("partern err");
        let mut errors = vec![];
        let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
        println!("utshell: help: {}", msg1);
        return EXECUTION_FAILURE!();
    }
   }
   unsafe {
       std::io::stdout().flush();
   }
  return EXECUTION_SUCCESS!();
}


#[no_mangle]
pub extern "C" fn  r_help_null_builtin (mut list:*mut WordList) -> i32{
  unsafe {
    show_shell_version(0);
  }
  show_builtin_command_help (); 
  return EXECUTION_SUCCESS!();
}

unsafe fn QUIT ()
{
  if terminating_signal !=0 {
    termsig_handler (terminating_signal);
  }

  if interrupt_state !=0{
    throw_to_top_level();
  }
}

pub  extern "C"  fn r_builtin_help (){
    // print  all  command usage
    let mut ind: i32 = 5;
    let d: i32;
    unsafe {
        current_builtin = builtin_address_internal(this_command_name, 0);
            if current_builtin == 0 as *mut  builtin{
                return ;
        }   

        d = (current_builtin as usize  - shell_builtins as usize) as i32;
    }
    ind = d/BUILTIN_SIZEOF!() ;
    unsafe {
       print!("{:?} : ",CStr::from_ptr(this_command_name));
    }
    show_helpsynopsis(ind);
    show_longdoc (ind);
}

fn open_helpfile(name :*mut c_char) -> i32{
  
    let mut  fd  : i32;
    unsafe {
    fd = open (name, 0);

    }
    if fd == -1 {
        return -1;
    }
    else {
      fd
    }
}

fn show_longdoc(i : i32){
  let  builtin1 = unsafe{&(*((shell_builtins as usize + (i*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
  let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
  let resources = vec![ "message.ftl".into()];
  let mut args = FluentArgs::new();
  let c_str: &CStr = unsafe { CStr::from_ptr(builtin1.name) };
  let s1 = String::from("command");
  match i {
      0|1|2|3|4|5 => {
                args.set("cmdName",format!("{}{}",s1,i));}
      33 => {
              args.set("cmdName",format!("{}{}",s1,6))}
      75 => {
            args.set("cmdName",format!("{}{}",s1,7))}
      _ => {
        let msg: &str = c_str.to_str().unwrap();
        args.set("cmdName",msg);}
  }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let mut value = bundle.get_message("helplongdoc").unwrap();
    let mut pattern = value.value().expect("partern err");
    let mut errors = vec![];
    let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
     println!("    {}", msg1);
}

fn show_helpsynopsis( i : i32) 
{
    let  builtin1 = unsafe{&(*((shell_builtins as usize + (i*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec![ "message.ftl".into()];
    let mut args = FluentArgs::new();
    let c_str: &CStr = unsafe { CStr::from_ptr(builtin1.name) };
    let s1 = String::from("command");
    match i {
      0|1|2|3|4|5 => {
                args.set("cmdName",format!("{}{}",s1,i));}
      33 => {
              args.set("cmdName",format!("{}{}",s1,6))}
      75 => {
            args.set("cmdName",format!("{}{}",s1,7))}
      _ => {
        let msg: &str = c_str.to_str().unwrap();
        args.set("cmdName",msg);}
   }
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let mut value = bundle.get_message("helpsynopsis").unwrap();
    let mut pattern = value.value().expect("partern err");
    let mut errors = vec![];
    let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
     println!(" {}", msg1);
}

fn show_desc (i :i32){
  let  builtin1 = unsafe{&(*((shell_builtins as usize + (i*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
  let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
  let resources = vec![ "message.ftl".into()];
  let mut args = FluentArgs::new();
  let c_str: &CStr = unsafe { CStr::from_ptr(builtin1.name) };
  let s1 = String::from("command");
  match i {
    0|1|2|3|4|5 => {
              args.set("cmdName",format!("{}{}",s1,i));}
    33 => {
            args.set("cmdName",format!("{}{}",s1,6))}
    75 => {
          args.set("cmdName",format!("{}{}",s1,7))}
    _ => {
      let msg: &str = c_str.to_str().unwrap();
      args.set("cmdName",msg);}
 }
  let bundle = mgr.get_bundle(get_local_str(), resources);
  let mut value = bundle.get_message("helpname").unwrap();
  let mut pattern = value.value().expect("partern err");
  let mut errors = vec![];
  let mut msg1 = bundle.format_pattern(&pattern, Some(&args), &mut errors);
   println!("    {}", msg1);
}

fn show_manpage (name : *mut c_char, i : i32){
  /* NAME */
  println! ("NAME\n");
  show_desc(i);

  /* SYNOPSIS */
  println! ("SYNOPSIS\n");
  show_helpsynopsis(i);
  println! ("DESCRIPTION\n");
  show_longdoc(i);
  /* SEE ALSO */
  println! ("SEE ALSO\n");
  println! ("    utshell(1) {} \n\n"," ");

  /* IMPLEMENTATION */
  println! ("IMPLEMENTATION\n");
  println! ("    ");
  unsafe {
    show_shell_version (0);
  }
  println! ("    ");
  unsafe {
    println! ("{:?}", CStr::from_ptr(bash_copyright));
  }
  println! ("    ");
  unsafe {
     println! ("{:?}", CStr::from_ptr(bash_license));
  }
}

#[no_mangle]
pub extern "C" fn  dispcolumn (i : i32, buf : *mut c_char, bufsize :libc::c_int, width : usize, height : i32){
    let mut j : i32;
    let mut dispcols : usize;
    let mut helpdoc :*mut  libc::c_char;
     /* first column */
    let mut builtin1 = unsafe{&(*((shell_builtins as usize + (i*BUILTIN_SIZEOF!()) as usize) as *mut builtin))};
    helpdoc = builtin1.short_doc;
    unsafe {
    libc::strncpy (((buf as usize + 4 as usize ) as * mut c_char), helpdoc, width - 2);
     *((buf as usize + (width - 2) as usize) as * mut c_char)='>' as c_char;
     *((buf as usize+(width - 1) as usize) as * mut c_char)='\0' as c_char;
     }
    /* indicate truncation */
    println! ("{:?}", buf);
    unsafe {
    if ((i << 1) >= num_shell_builtins) || (i+height >= num_shell_builtins){
        println! ("\n");
        return;
    }
    }
    dispcols = unsafe {libc::strlen(buf)};
    /* two spaces */
    for j in  dispcols .. width{
         std::io::stdout().write(b" ");
  }
  /* second column */
  builtin1 = unsafe{&(*((shell_builtins as usize + (((i+height)*BUILTIN_SIZEOF!()) as usize)) as *mut builtin))};
  helpdoc = builtin1.short_doc as *mut libc::c_char;
  unsafe {
  if  builtin1.flags && BUILTIN_ENABLED!()==1 {
       *((buf as usize) as * mut c_char)=' ' as c_char;
  }
  else{
      *((buf as usize) as * mut c_char)='*' as c_char;
  }
  libc::strncpy (((buf as usize + 4 as usize ) as * mut c_char), helpdoc, width - 3);
  *((buf as usize + (width - 3) as usize) as * mut c_char)='>' as c_char;
  *((buf as usize+(width - 2) as usize) as * mut c_char)='\0' as c_char;
  }
   println! ("{:?}\n", buf);
}

pub fn  wdispcolumn (i : i32, buf :*mut c_char, bufsize : i32, width : i32, height : i32){
    let  mut j : i32;
    show_helpsynopsis(i);
}

fn show_builtin_command_help (){
    let mut i : i32;
    let mut j : i32;
    let  height : i32 = 76;
    let mut width : usize;
    let mut t :*mut libc::c_char;
    let mut blurb:[libc::c_char;128] = ['0' as  libc::c_char;128];
    let mgr = ResourceManager::new("/usr/share/utshell/resources/{locale}/{res_id}".into());
    let resources = vec!["message.ftl".into()];
    let bundle = mgr.get_bundle(get_local_str(), resources);
    let value = bundle.get_message("information").unwrap();
    let  pattern = value.value().expect("partern err");
    let mut errors = vec![];
    let msg1 = bundle.format_pattern(&pattern, None, &mut errors);
    println!("{}\n", msg1);
    //println!("{}",("These shell commands are defined internally.  Type `help' to see this list.\n Type `help name' to find out more about the function `name'.\n Use `info bash' to find out more about the shell in general.\n Use `man -k' or `info' to find out more about commands not in this list.\n A star (*) next to a name means that the command is disabled.\n"));

    let ref2: &mut libc::c_char= &mut blurb[0];

    unsafe {
    width = default_columns();
  }
  width /= 2;
  if width > (std::mem::size_of::<libc::c_char>()*128) {
    width = std::mem::size_of::<libc::c_char>()*128;
  }
  if width <= 3{
    width = 40;
  }
  for i in 0..height{
      unsafe {
        QUIT();
      }
      if MB_CUR_MAX!() > 1 {
       let ptr2: *mut libc::c_char = ref2 as *mut libc::c_char;
       wdispcolumn (i,  ptr2,128, width as i32, height);
  }
}
}
//#endif /* HELP_BUILTIN */
fn strmatch (pattern : *mut libc::c_char, string : *mut libc::c_char, flags : libc::c_char) -> libc::c_char
{
  if ((string as usize)as * mut c_char != std::ptr::null_mut()) || ((pattern as usize)as * mut c_char != std::ptr::null_mut()){
     return FNM_NOMATCH!();
  }
  return unsafe {xstrmatch (pattern, string, flags)};
}

struct Thing {
  pointer_to_self: *mut Thing,
}

fn xmalloc (size:usize) ->*mut c_void  {
	let ret: *mut c_void;
unsafe {
	ret = libc::malloc(size);
}
// 	if (ret == 0) {
//     println!("man2html: out of memory");
// //		fprintf(stderr, "man2html: out of memory");
// 		（1）
// 	}
	ret
} 

// fn wcswidth(pwcs : *mut libc::wchar_t , n : i32) -> i32{
//   let mut wc : libc::wchar_t;
//   let mut len : i32 = 0;
//   let mut l : i32;
			
//   while n-1 > 0 && *(pwcs as usize + 1 as usize) != '\0' as libc::wchar_t{
//     wc = *(pwcs  += 1);
//       if wcwidth(wc) < 0 {
//         return -1;
//       }
//       len += l;
//     }
//   len
// }
