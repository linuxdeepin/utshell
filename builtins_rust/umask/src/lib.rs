extern crate libc;

use libc::{c_char,c_int};
use std::ffi::{CString};
use rcommon::{r_read_octal};
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage};

use rhelp::r_builtin_help;
//结构体


//枚举


//宏
#[macro_export]
macro_rules! mode_t {
    () => {
        i32
    };
}


#[macro_export]
macro_rules! S_IREAD{
    () => { 0o0400 }
}

#[macro_export]
macro_rules! S_IWRITE{
    () => { 0o0200 }
}

#[macro_export]
macro_rules! S_IEXEC {
    () => { 0o0100 };
}

#[macro_export]
macro_rules! S_IRUSR{           /* read, owner */
    () => { 
        S_IREAD!() 
    }
}

#[macro_export]
macro_rules! S_IWUSR{           /* write, owner */
    () => { 
        S_IWRITE!() 
    }
}

#[macro_export]
macro_rules! S_IXUSR {          /* execute, owner */
    () => { 
        S_IEXEC!() 
    };
}

#[macro_export]
macro_rules! S_IRGRP {          /* read, group */
    () => {
        S_IREAD!() >> 3
    }
}

#[macro_export]
macro_rules! S_IWGRP {          /* write, group */
    () => {
        S_IWRITE!() >> 3
    };
}

#[macro_export]
macro_rules! S_IXGRP {          /* execute, group */
    () => {
        S_IEXEC!() >> 3
    };
}

#[macro_export]
macro_rules! S_IROTH {          /* read, other */
    () => {
        S_IREAD!() >> 6
    };
}

#[macro_export]
macro_rules! S_IWOTH {          /* write, other */
    () => {
        S_IWRITE!() >> 6
    };
}

#[macro_export]
macro_rules! S_IXOTH {          /* execute, other */
    () => {
        S_IEXEC!() >> 6
    };
}

#[macro_export]
macro_rules! S_IRWXU{
    () => {
        S_IRUSR!() | S_IWUSR!() | S_IXUSR!()
    }
}



#[macro_export]
macro_rules! S_IRWXG {
    () => {
        S_IRGRP!() | S_IWGRP!() | S_IXGRP!()
    };
}

#[macro_export]
macro_rules! S_IRWXO {
    () => {
        S_IROTH!() | S_IWOTH!() | S_IXOTH!()
    };
}

#[macro_export]
macro_rules! S_IRUGO {
    () => {
        S_IRUSR!() | S_IRGRP!() | S_IROTH!()
    };
}

#[macro_export]
macro_rules! S_IWUGO {
    () => {
        S_IWUSR!() | S_IWGRP!() | S_IWOTH!()
    };
}

#[macro_export]
macro_rules! S_IXUGO {
    () => {
        S_IXUSR!() | S_IXGRP!() | S_IXOTH!()
    };
}


//C库
extern "C" {

    static mut loptend:*mut WordList;

    fn reset_internal_getopt();
    fn internal_getopt (list:*mut WordList,  opts:*mut c_char)->i32;
    fn builtin_usage();
    // fn read_octal(string:*mut c_char)->i32;
    fn sh_erange(s:*mut c_char,desc:*mut c_char);
    fn sh_chkwrite(s:i32)->i32;
    fn umask(__maks:mode_t!())->mode_t!();
}

//
unsafe fn DIGIT(c:c_char) -> bool{
    char::from(c as u8 ) >= '0' && char::from(c as u8) <= '9'
}
//有可能错误
unsafe fn member(c:*mut c_char,s:*mut c_char) -> bool{
    if c != std::ptr::null_mut(){
        let c  = c as c_int;
        let  ptr = libc::strchr(s,c);
        if ptr != std::ptr::null_mut(){
            true
        }
        else{
            false
        }
    }
    else{
        false
    }
}

//
#[no_mangle]
/* Set or display the mask used by the system when creating files.  Flag
   of -S means display the umask in a symbolic mode. */

pub extern "C" fn r_umask_builtin(mut list:*mut WordList) ->i32{
    let mut print_symbolically:i32;
    let mut opt:i32;
    let  umask_value:i32;
    let mut pflag:i32;
    let  umask_arg:mode_t!();
    unsafe{
        print_symbolically = 0;
        pflag = 0;
        reset_internal_getopt();

        let c_str_sp = CString::new("Sp").unwrap();
        opt = internal_getopt(list,c_str_sp.as_ptr() as *mut c_char);
        while opt != -1 { 
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);
            match opt_char {
                'S' => {print_symbolically = print_symbolically +1;}
                'p' => {pflag = pflag + 1;}
                _ => {
                    if opt == -99 {
                        r_builtin_help();
                        return EX_USAGE;
                    }
                     builtin_usage();
                    return EX_USAGE;
                }
            }

            opt = internal_getopt(list,c_str_sp.as_ptr() as *mut c_char);
        } 

        list = loptend;
        if list != std::ptr::null_mut(){
            if DIGIT( *(*(*list).word).word) != false {
                umask_value = r_read_octal((*(*list).word).word);
                 /* Note that other shells just let you set the umask to zero
                    by specifying a number out of range.  This is a problem
                    with those shells.  We don't change the umask if the input
                    is lousy. */
                if umask_value == -1{
                    let c_str = CString::new("octal number").unwrap();
                    let c_char_str:*mut c_char = c_str.into_raw();
                    sh_erange((*(*list).word).word,c_char_str);
                    return EXECUTION_FAILURE!();
                }
            }
            else{
                umask_value = r_symbolic_umask(list);
                if umask_value == -1{
                    return EXECUTION_FAILURE!();
                }                  
            }
            umask_arg = umask_value;
            umask(umask_arg);
            if print_symbolically != 0{
                r_print_symbolic_umask(umask_arg);
            }
        }
        else{            /* Display the UMASK for this user. */        
            umask_arg = umask(0o22);
            umask(umask_arg);
            if pflag != 0{
                if print_symbolically != 0{
                    println!("umask  -S");
                }
                else{
                    print!("umask ")
                }
            }
/*
            if pflag != 0{
                if print_symbolically != 0{
                    println!("umask \" -S\" ");
                }
                else{
                    println!("umask \"\" ")
                }
            }
*/
            if print_symbolically != 0{
                r_print_symbolic_umask(umask_arg);
            }
            else{
                println!("{:04o}",umask_arg);
            }
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
}


#[no_mangle]
/* Print the umask in a symbolic form.  In the output, a letter is
   printed if the corresponding bit is clear in the umask. */

extern "C"  fn r_print_symbolic_umask(um:mode_t!()){
    /* u=rwx,g=rwx,o=rwx */
    let mut ubits = String::new();
    let mut gbits =String::new();
    let mut obits = String::new();

    if um & S_IRUSR!() == 0{
        ubits.push('r');
    }
    if um & S_IWUSR!() == 0{
        ubits.push('w');
    }
    if um & S_IXUSR!() == 0{
        ubits.push('x');
    }

    if um & S_IRGRP!() == 0{
        gbits.push('r');
    }
    if um & S_IWGRP!() == 0{
        gbits.push('w');
    }
    if um & S_IXGRP!() == 0{
        gbits.push('x');
    }

    if um & S_IROTH!() == 0{
        obits.push('r');
    }
    if um & S_IWOTH!() == 0{
        obits.push('w');
    }
    if um & S_IXOTH!() == 0{
        obits.push('x');
    }

    println!{"u={},g={},o={}",ubits,gbits,obits};
}

#[no_mangle]
extern "C" fn r_parse_symbolic_mode(mode:*mut c_char,initial_bits:i32)->i32{
    let mut who:i32;
    let mut op:i32;
    let mut perm:i32;
    let mut bits:i32;
    let mut c:i32;
    let mut s:*mut c_char;

    s = mode;
    bits = initial_bits;

    unsafe{
        loop{
            who = 0;
            op = 0;
            perm = 0;
    
            /* Parse the `who' portion of the symbolic mode clause. */
            let c_str = CString::new("agou").unwrap();
            while member (s,c_str.as_ptr() as *mut c_char){
                c = *s as c_int;
                s = (s as usize + 1) as *mut c_char;
                let optu8 = c as u8;
                let opt_char = char::from(optu8);
                match opt_char{
                    'u' => {
                        who |= S_IRWXU!();
                        continue;
                    }
                    'g' => {
                        who |= S_IRWXG!();
                        continue;
                    } 
                    'o' => {
                        who |= S_IRWXO!();
                        continue;
                    }
                    'a' => {
                        who |= S_IRWXU!() | S_IRWXG!() | S_IRWXO!();
                        continue;
                    }
                    _ => {
                        
                    }
                }
            }

            /* The operation is now sitting in *s. */
            op = *s as c_int;
           // *s = *s + 1;
            s = (s as usize + 1) as *mut c_char;
            let opu8 = op as u8;
            let op_str = char::from(opu8);
            match op_str{
                '+' | '-' | '=' => {}
                _ => {
                    println!("{}:invalid symbolic mode operator",op_str);
                    return -1;
                }
            }

            /* Parse out the `perm' section of the symbolic mode clause. */
            let c_rwx_str = CString::new("rwx").unwrap();
            while member(s,c_rwx_str.as_ptr() as *mut c_char){
                c = s as c_int;
                //*s = *s + 1;
                s = (s as usize + 1) as *mut c_char;
                let optu8 = c as u8;
                let op_str = char::from(optu8);

                match op_str {
                    'r' => perm |= S_IRUGO!(),
                    'w' => perm |= S_IWUGO!(),
                    'x' => perm |= S_IXUGO!(),
                     _  => { }
                }
            }

            /* Now perform the operation or return an error for a
             bad permission string. */
            if *s != 0 || *s == ',' as libc::c_char{
                if who != 0{
                    perm &= who;
                }

                match op_str{
                    '+' => bits |= perm,
                    '-' => bits &= !perm,
                    '=' => {
                        if who == 0{
                            who = S_IRWXU!() | S_IRWXG!() | S_IRWXO!();
                            bits &= ! who;
                            bits |= perm;
                        }   
                    }
                    /* No other values are possible. */ 
                    _ => { }
                }
                if *s == '\0' as libc::c_char{
                    break;
                }
                else {
                    //*s = *s + 1;
                    s = (s as usize + 1) as *mut c_char;
                }
            }
            else {
                println!("{}:invalid symbolic mode character",*s as c_char);
                return -1;
            }
        }//loop
        return bits;
    }
}


#[no_mangle]
/* Set the umask from a symbolic mode string similar to that accepted
   by chmod.  If the -S argument is given, then print the umask in a
   symbolic form. */

extern "C" fn r_symbolic_umask(list:*mut WordList)->i32{
    let mut um:i32;
    let bits:i32;
    
    unsafe{
        /* Get the initial umask.  Don't change it yet. */
        um = umask(0o22);
        umask(um);

        /* All work is done with the complement of the umask -- it's
            more intuitive and easier to deal with.  It is complemented
            again before being returned. */
        bits = r_parse_symbolic_mode((*(*list).word).word, !um & 0777);
        if bits == -1 {
            return -1;
        }

        um = !bits & 0o777;
        return um;
    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
