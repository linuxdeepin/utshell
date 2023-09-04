extern crate libc;

use libc::{c_char,c_int, strchr, putchar,clearerr,free,FILE, fprintf, c_void};
use std::ffi::{CString,CStr,};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
// use std::io::{stdout, Write};
use std::ptr::read_volatile;
use rhelp::r_builtin_help;
//结构体

//枚举

//宏
#[macro_export]
macro_rules!  VALID_ECHO_OPTIONS {
    () => {
        CString::new("neE").unwrap().as_ptr()
    };
}


#[macro_export]
macro_rules!  QUIT {
    () => {
        if read_volatile(&terminating_signal as *const i32) != 0{
            termsig_handler(read_volatile(&terminating_signal as *const i32));
        }
        if interrupt_state != 0{
            throw_to_top_level();
        }
    };
}



unsafe fn STRLEN (s:*const c_char) -> i32{

    if s!=std::ptr::null_mut(){
        let s_cstr = CStr::from_ptr(s);
        let s_str = s_cstr.to_str().unwrap();
        let s_string = s_str.to_owned();
    
        let len = s_string.len();
        return len as i32;
    }
    else{
        return 0;
    }

}


//
extern "C"{
    static terminating_signal:c_int;
    static interrupt_state:c_int;
    static stdout:*mut FILE;

    fn termsig_handler(sig:i32);
    fn throw_to_top_level();
    fn ansicstr(string:*mut c_char,len:i32,flags:i32,sawc:*mut c_int,rlen:*mut c_int)->*mut c_char;
    fn sh_chkwrite(s:i32)->i32;
}

/* System V machines already have a /bin/sh with a v9 behaviour.  We
   give Bash the identical behaviour for these machines so that the
   existing system shells won't barf.  Regrettably, the SUS v2 has
   standardized the Sys V echo behavior.  This variable is external
   so that we can have a `shopt' variable to control it at runtime. */

pub static mut xpg_echo:i32 = 0;    // 也有可能是1

/* Print the words in LIST to standard output.  If the first word is
   `-n', then don't print a trailing newline.  We also support the
   echo syntax from Version 9 Unix systems. */

#[no_mangle]
pub extern "C" fn r_echo_builtin(mut list:*mut WordList)->i32{
    let mut display_return:i32;
    let mut do_v9:i32;
    let mut i:i32;
    let mut len:i32;
    let mut temp:*mut c_char=std::ptr::null_mut();
    let mut s:*mut c_char;

    unsafe{
        do_v9 = xpg_echo;
        display_return = 1;

        // if posixly_correct!=0 && xpg_echo!=0{    //xpg_echo=0,所以这个可能不用翻译
            
        // }
        if  !list.is_null() && (*list).word != std::ptr::null_mut()  && (*(*list).word).word != std::ptr::null_mut(){
            temp = (*(*list).word).word;
        }
        while !list.is_null() && *temp=='-' as c_char{
        /* If it appears that we are handling options, then make sure that
         all of the options specified are actually valid.  Otherwise, the
         string should just be echoed. */

            temp = (temp as usize +1) as *mut c_char;
            let mut t = temp;
            i = 0;

            while *temp as i32 != 0{
                let s = *temp as i32;
                let su8 = s as u8;
                let s_opt = char::from(su8);

                if strchr(VALID_ECHO_OPTIONS!(), s_opt as c_int).is_null(){
                    break;
                }
                
                temp = (temp as usize + 1) as *mut c_char;
                i += 1;
            }
            // 
            /* echo - and echo -<nonopt> both mean to just echo the arguments. */
            if *t==0 || *((t as usize + i as usize) as *mut c_char) != 0{
                break;
            }

            /* All of the options in TEMP are valid options to ECHO.
            Handle them. */
            while !t.is_null(){               
                let optu8 = *t as u8;
                let opt_char = char::from(optu8);

                match opt_char{
                    'n' => {
                        display_return = 0;
                    }
                    'e' => {
                        do_v9 = 1;
                    }
                    'E' => {
                        do_v9 = 0;
                    }
                     _  => break,
                }
                t = (t as usize +1) as *mut c_char;
            }

            list = (*list).next;
            if !(*(*list).word).word.is_null(){
                temp = (*(*list).word).word;
            }
        }
  
        clearerr(stdout);       /* clear error before writing and testing success */
        
        while list != std::ptr::null_mut(){
            i = 0;
            len =0;

            if do_v9 !=0{
                temp = ansicstr((*(*list).word).word,STRLEN((*(*list).word).word), 1,&mut i,&mut len);
            }
            else{
                temp = (*(*list).word).word;
            }

            if temp != std::ptr::null_mut(){
                if do_v9 != 0{
                    s = temp;
                
                    for _ in 0..len{
                        putchar(*s as c_int );
                        s = (s as usize + 1) as *mut c_char;
                    }
                }
                else {
                    fprintf(stdout, temp);
                }
            }

            QUIT!();
            if do_v9 !=0 && temp != std::ptr::null_mut(){
                free(temp as *mut c_void);
            }

            list = (*list).next;
            if i != 0{
                display_return = 0;
                break;
            }

            if list!=std::ptr::null_mut(){
                putchar(' ' as i32);
                QUIT!();
            }
        }//while

        if display_return != 0{
            putchar('\n' as i32);
        }

        return sh_chkwrite(EXECUTION_SUCCESS!());

    }//unsafe
    
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
