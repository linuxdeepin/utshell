extern crate libc;
extern crate nix;
extern crate rcommon;

use libc::{c_char,c_int, strlen, strcpy, size_t, c_void, free};
use std::ffi::{CString,CStr};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage, r_savestring};
use nix::errno::errno;
use rcommon::r_sh_restricted;
use rhelp::r_builtin_help;

#[repr(C)]
struct redirect{
    next:*mut redirect,
    redirector:REDIRECTEE,
    rflags:i32,
    flags:i32,
    insturction:r_instruction,
    redirectee:REDIRECTEE,
    here_doc_eof:*mut c_char,
}
type REDIRECT = redirect;

#[repr (C)]
union REDIRECTEE {
    dest:i32,
    filename:*mut WordDesc,
}

pub type r_instruction = libc::c_uint;
pub const r_append_err_and_out: r_instruction = 19;
pub const r_move_output_word: r_instruction = 18;
pub const r_move_input_word: r_instruction = 17;
pub const r_move_output: r_instruction = 16;
pub const r_move_input: r_instruction = 15;
pub const r_duplicating_output_word: r_instruction = 14;
pub const r_duplicating_input_word: r_instruction = 13;
pub const r_output_force: r_instruction = 12;
pub const r_input_output: r_instruction = 11;
pub const r_err_and_out: r_instruction = 10;
pub const r_close_this: r_instruction = 9;
pub const r_deblank_reading_until: r_instruction = 8;
pub const r_duplicating_output: r_instruction = 7;
pub const r_duplicating_input: r_instruction = 6;
pub const r_reading_string: r_instruction = 5;
pub const r_reading_until: r_instruction = 4;
pub const r_appending_to: r_instruction = 3;
pub const r_inputa_direction: r_instruction = 2;
pub const r_input_direction: r_instruction = 1;
pub const r_output_direction: r_instruction = 0;


#[macro_export]
macro_rules! FREE {
    ($s:expr) => {
        if ($s) != std::ptr::null_mut(){
            free($s);
        }
    }
}

extern "C" {
    // static errno:i32;
    static mut exec_argv0:*mut c_char;
    static list_optarg:*mut c_char;
    static loptend:*mut WordList;
    static mut redirection_undo_list:*mut REDIRECT;
    static restricted:i32;
    // static comsub_ignore_return:i32;
    static export_env:*mut *mut c_char;
    static interactive_shell:i32;
    static subshell_environment:i32;
    static job_control:i32;
    static interactive:i32;
    static default_buffered_input:i32;
    static no_exit_on_failed_exec:i32;
    fn xmalloc(n:size_t)->*mut c_void;
    fn reset_internal_getopt();
    fn internal_getopt(list:*mut WordList,opts:*mut c_char)->i32;
    // fn builtin_usage();
    fn dispose_redirects(list:*mut REDIRECT);
    // fn sh_restricted(s:*mut c_char);
    fn strvec_from_word_list(list:*mut WordList,alloc:i32,starting_index:i32,ip:*mut i32)->*mut *mut c_char;
    fn absolute_program(string:*const c_char)->i32;
    fn search_for_command(pathname:*const c_char,flags:i32)->*mut c_char;
    fn file_isdir(f:*const c_char)->i32;
    fn builtin_error(format:*const c_char,...);
    fn strerror(e:i32)->*mut c_char;
    fn sh_notfound(s:*mut c_char);
    fn full_pathname(file:*mut c_char)->*mut c_char;
    fn adjust_shell_level(change:i32);
    fn strvec_create(n:i32)->*mut *mut c_char;
    fn maybe_make_export_env();
    fn maybe_save_shell_history()->i32;
    fn restore_original_signals();
    fn end_job_control();
    fn default_tty_job_signals();
    fn sync_buffered_stream(bfd:i32)->i32;
    fn shell_execve(command:*mut c_char,args:*mut *mut c_char,env:*mut *mut c_char)->i32;
    fn executable_file(file:*const c_char)->i32;
    fn file_error(filename:*mut c_char);
    fn exit_shell(s:i32);
    fn strvec_dispose(array:*mut *mut c_char);
    fn initialize_traps();
    fn initialize_signals(reinit:i32);
    fn restart_job_control();
}

/* If the user wants this to look like a login shell, then
   prepend a `-' onto NAME and return the new name. */
#[no_mangle]
extern "C" fn r_mkdashname(name:*mut c_char)->*mut c_char{
    let ret:*mut c_char;

    unsafe{
        ret = xmalloc(2 + strlen(name)) as *mut c_char;
        *ret.offset(0) = '-' as libc::c_char;
        strcpy(ret.offset(1), name);
        return ret;
    }
    
} 


#[no_mangle]
pub extern "C" fn r_exec_builtin(mut list:*mut WordList)->i32{
    let mut exit_value ;
    let mut cleanenv:i32 = 0;
    let mut login:i32 = 0;
    let mut opt:i32;
    let mut orig_job_control:i32 = 0;
    let mut argv0:*mut c_char = std::ptr::null_mut() as *mut c_char;
    let mut command:*mut c_char;
    let mut args:*mut *mut c_char;
    let mut env:*mut *mut c_char;
    let newname:*mut c_char;
    let com2:*mut c_char;

    unsafe{
        exec_argv0 = std::ptr::null_mut() as *mut c_char;

        reset_internal_getopt();
    
        loop{
            let c_str = CString::new("cla:").unwrap();
            opt = internal_getopt(list,c_str.as_ptr() as *mut c_char);
            while opt != -1{
                let optu8 = opt as u8;
                let opt_char = char::from(optu8);
                match opt_char{
                    'c' => cleanenv = 1,
                    'l' => login = 1,
                    'a' => argv0 = list_optarg,
                    _  => {
                        if opt == -99 {
                            r_builtin_help();
                            return EX_USAGE;
                        }
                        r_builtin_usage();
                        return EX_USAGE;
                    }
                }
    
                opt = internal_getopt(list,c_str.as_ptr() as *mut c_char);
            }
    
            list = loptend;
    
            /* First, let the redirections remain. */
            dispose_redirects(redirection_undo_list);
            redirection_undo_list = std::ptr::null_mut() as *mut REDIRECT;
    
            if list.is_null(){
                return EXECUTION_SUCCESS!();
            }

            if restricted != 0{     //限制性shell
                // sh_restricted(std::ptr::null_mut() as *mut c_char);
                r_sh_restricted(std::ptr::null_mut() as *mut c_char);
                return EXECUTION_FAILURE!();
            }
    
            args = strvec_from_word_list(list,1,0,0 as *mut c_int);     //这个指针这样写不清楚可不可以
            env = 0 as *mut *mut c_char;
    
            /* A command with a slash anywhere in its name is not looked up in $PATH. */
            if absolute_program(*args.offset(0)) != 0{  //命令给的绝对路径，或者执行脚本
                command = (*args).offset(0);
            }
            else {  //exec后直接给命令
                command = search_for_command(*args.offset(0),1);
                println!("command:{}",CStr::from_ptr(command).to_str().unwrap());
            }
    
            if command.is_null(){
                if file_isdir(*args.offset(0)) != 0{
                    let c_str = CString::new("%s: cannot execute: %s").unwrap();
                    let c_ptr = c_str.as_ptr();
                    builtin_error(c_ptr,*args.offset(0),strerror(errno()));
                    exit_value = EX_NOEXEC;
                }
                else{   
                    sh_notfound(*args.offset(0));
                    exit_value = EX_NOTFOUND;
                }
                //goto failed_exec;
                break;
            }
    
            com2 = full_pathname(command);
            if !com2.is_null(){
                if command != *args.offset(0){
                    free(command as *mut c_void);
                }
                command = com2;
            }
    
            if !argv0.is_null(){    //exec有-a参数
                free(*args.offset(0) as *mut c_void);
                if login != 0{
                    *args.offset(0) = r_mkdashname(argv0);
                }
                else {
                    *args.offset(0) = r_savestring(argv0);
                }
                exec_argv0 = r_savestring(*args.offset(0));
            }
            else if login != 0{
                newname = r_mkdashname(*args.offset(0));
                free(*args.offset(0) as *mut c_void);
                *args.offset(0) = newname;
            }
    
            /* Decrement SHLVL by 1 so a new shell started here has the same value,
             preserving the appearance.  After we do that, we need to change the
             exported environment to include the new value.  If we've already forked
             and are in a subshell, we don't want to decrement the shell level,
             since we are `increasing' the level */
            
            if cleanenv == 0 && (subshell_environment & SUBSHELL_PAREN!() == 0){
                adjust_shell_level(-1);
            }
    
            if cleanenv != 0{
                env = strvec_create(1);
                *env.offset(0) = 0 as *mut c_char;
            }
            else{
                maybe_make_export_env();
                env = export_env;
            }
    
            if interactive_shell !=0 && subshell_environment == 0{
                maybe_save_shell_history();
            }
    
            restore_original_signals();
    
            orig_job_control = job_control;
            if  subshell_environment == 0{
                end_job_control();
            }
            if interactive != 0 || job_control != 0{
                default_tty_job_signals();
            }
    
            if default_buffered_input >= 0{
                sync_buffered_stream(default_buffered_input);
            }
    
            exit_value = shell_execve(command,args,env);
    
            /* We have to set this to NULL because shell_execve has called realloc()
             to stuff more items at the front of the array, which may have caused
             the memory to be freed by realloc().  We don't want to free it twice. */
    
            args = std::ptr::null_mut() as *mut *mut c_char;
    
            if cleanenv == 0 {
                adjust_shell_level(1);
            }
    
            if exit_value == EX_NOTFOUND{
                //goto failed_exec;
                break;
            }
            else if executable_file(command) == 0{
                let c_str = CString::new("%s: cannot execute: %s").unwrap();
                let c_ptr = c_str.as_ptr();
                builtin_error(c_ptr,command,strerror(errno()));
                exit_value = EX_NOEXEC;
            }
            else{
                file_error(command);
            }            
            
            //跳出loop循环，只执行一次loop
            break;
        }


        //fialed_exec
        FREE!(command as *mut c_void);
        
        if subshell_environment != 0 || interactive == 0 && no_exit_on_failed_exec ==0{
            exit_shell(exit_value);
        }

        if !args.is_null(){
            strvec_dispose(args);
        }

        if !env.is_null() && env != export_env{
            strvec_dispose(env);
        } 

        initialize_traps();
        initialize_signals(1);

        if orig_job_control != 0{
            restart_job_control();
        }

        return exit_value;
    }
}


