extern crate libc;
extern crate rjobs;
extern crate rread;
extern crate nix;

include!("./signal.rs");

pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
extern "C" {
    pub fn setjmp(__env: *mut __jmp_buf_tag) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}

use libc::{c_ulong, c_char, intmax_t,  c_short,c_int, c_long};
use std::ffi::{CString,};
use nix::sys::signal::{SigSet, Signal};
use rjobs::{PROCESS,COMMAND, BLOCK_CHILD, UNBLOCK_CHILD};
use rread::{SHELL_VAR, sh_var_value_func_t, sh_var_assign_func_t};
use rcommon::{r_builtin_unbind_variable,r_builtin_usage,r_get_job_spec,WordList};
use rcommon::{ WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN};

use rhelp::r_builtin_help;
// 结构体
#[repr(C)]
pub struct procstat{
    pub pid:pid_t,
    pub status:c_short,
}

#[repr(C)]
pub struct JOB {
    wd: *mut c_char,
    pipe: *mut PROCESS,
    pgrp:i32,
    state:JOB_STATE,
    flags:i32,
    deferred:*mut COMMAND,
    j_cleanup:*mut fn(),
    cleanarg:* mut fn()
}


#[repr(C)]
pub struct jobstats {
    /* limits */
    c_childmax:libc::c_long,
    /* child process statistics */
    c_living:libc::c_int,       /* running or stopped child processes */
    c_reaped:libc::c_int,   /* exited child processes still in jobs list */
    c_injobs:libc::c_int,   /* total number of child processes in jobs list */
    /* child process totals */
    c_totforked:libc::c_int,    /* total number of children this shell has forked */
    c_totreaped:libc::c_int,    /* total number of children this shell has reaped */
    /* job counters and indices */
    j_jobslots:libc::c_int,/* total size of jobs array */
    j_lastj:libc::c_int,        /* last (newest) job allocated */
    j_firstj:libc::c_int,   /* first (oldest) job allocated */
    j_njobs:libc::c_int,        /* number of non-NULL jobs in jobs array */
    j_ndead:libc::c_int,        /* number of JDEAD jobs in jobs array */
    /* */
    j_current:libc::c_int,  /* current job */
    j_previous:libc::c_int, /* previous job */
    /* */
    j_lastmade:* mut JOB,   /* last job allocated by stop_pipeline */
    j_lastasync:* mut JOB   /* last async job allocated by stop_pipeline */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct variable {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub exportstr: *mut c_char,
    pub dynamic_value: sh_var_value_func_t,
    pub assign_func: sh_var_assign_func_t,
    pub attributes: c_int,
    pub context: c_int,
}

pub type arrayind_t = intmax_t;
//枚举
#[repr(i8)]
#[derive(PartialEq)]
pub enum JOB_STATE {
    JNONE = -1,
    JRUNNING = 1,
    JSTOPPED = 2,
    JDEAD = 4,
    JMIXED = 8
}


//宏

#[macro_export]
macro_rules! J_WAITING {
    () => { 0x08 }
}

#[macro_export]
macro_rules! JWAIT_FORCE {
    () => { 1 << 1 }
}

#[macro_export]
macro_rules! NO_JOB {
    () => { -1 }
}

#[macro_export]
macro_rules! DUP_JOB{
    () => { -2 }
}

#[macro_export]
macro_rules! VA_NOEXPAND {
    () => { 0x001 }
}

#[macro_export]
macro_rules! VA_ONEWORD {
    () => { 0x002 };
}

#[macro_export]
macro_rules! JWAIT_WAITING {
    () => { 1 << 3 };
}

#[macro_export]
macro_rules! JWAIT_PEROOR {
    () => { 1 << 0};
}

#[macro_export]
macro_rules! NO_PID {
    () => { -1 as pid_t }
}

#[macro_export]
macro_rules! get_job_by_jid {
    ($ind:expr) => {
        (*((jobs as usize + ($ind*8) as usize) as *mut *mut JOB) as *mut JOB)
    };
}

#[macro_export]
macro_rules! INVALID_JOB {
    ($j:expr) => {
        ($j)<0 || ($j)>= js.j_jobslots || get_job_by_jid!($j) == std::ptr::null_mut()
    };
}

#[macro_export]
macro_rules! BLOCK_SIGNAL{
    ($sig:expr,$nvar:expr,$ovar:expr) => {
        $nvar.unwrap().clear();
        $nvar.unwrap().add($sig);
        $nvar.unwrap().clear();
        nix::sys::signal::sigprocmask(nix::sys::signal::SigmaskHow::SIG_BLOCK,$nvar,$ovar);
    }
}

#[macro_export]
macro_rules! UNBLOCK_SIGNAL {
    ($ovar:expr) => {
        nix::sys::signal::sigprocmask(nix::sys::signal::SigmaskHow::SIG_SETMASK,$ovar,None)
    };
}

pub type procenv_t=__jmp_buf_tag;

//C库
extern "C" {
    static mut wait_intr_buf: procenv_t;
    static mut wait_signal_received:i32;
    static mut wait_intr_flag:i32;
    static mut loptend:*mut WordList;
    static  js:jobstats ;
    static mut jobs:*mut*mut JOB;
    static list_optarg:*mut c_char;
    static assoc_expand_once:i32;
    static mut last_command_exit_signal:i32;
    static posixly_correct:i32;
    fn internal_getopt (list:*mut WordList,  opts:*mut c_char)->i32;
    fn legal_number(string:*const c_char,result:*mut c_long)->i32;
    fn get_job_by_pid(pid:pid_t,block:i32,procp:*mut *mut PROCESS)->i32;
    fn sh_badjob(str:*mut c_char);
    fn reset_internal_getopt();
    fn legal_identifier(name:*const c_char)->i32;
    fn valid_array_reference(name:*const c_char,flage:i32)->i32;
    fn sh_invalidid(s:*mut c_char);
    fn wait_sigint_cleanup();
    fn first_pending_trap()->i32;
    fn next_pending_trap(start:i32)->i32;
    fn wait_for_any_job(flags:i32,ps:*mut procstat)->i32;
    fn bind_var_to_int(var:*mut c_char,val:intmax_t)->*mut SHELL_VAR;
    fn wait_for_background_pids(ps:*mut procstat);
    fn wait_for_single_pid(pid:pid_t,flags:i32)->i32;
    fn wait_for_job(job:i32,flags:i32,ps:*mut procstat)->i32;
}

unsafe fn DIGIT(c:c_char)->bool{
    char::from(c as u8) >= '0' && char::from(c as u8) <= '9'
}

#[macro_export]
macro_rules! WAIT_RETURN  {
    ($s:expr) => {
        {
            wait_signal_received = 0;
            wait_intr_flag = 0;
            $s
        }
    };
} 

//rust
#[no_mangle]
pub extern  "C" fn r_wait_builtin(mut list:*mut WordList)->i32{
    let mut status:i32;
    let code:i32;
    let mut opt:i32;
    let mut nflag:i32;
    let mut wflags:i32;
    let mut vname:*mut c_char;
    let pidvar:*mut SHELL_VAR;
    let mut pstat:procstat = procstat{   
        pid:0,
        status:0,};

    unsafe{
        // USE_VAR(list);
        nflag = 0;
        wflags = 0;
        vname = std::ptr::null_mut();
        pidvar = std::ptr::null_mut();

        reset_internal_getopt();
        let c_fnp = CString::new("fnp:").unwrap();
        
        loop{
            opt = internal_getopt(list,c_fnp.as_ptr() as *mut c_char);
            if opt == -1{
                break;
            }
            let optu8 = opt as u8;
            let opt_char = char::from(optu8);

            match opt_char{
                'n' => nflag = 1,
                'f' => wflags |= JWAIT_FORCE!(),
                'p' => vname = list_optarg,
                 _  => {
                    if opt == -99 {
                        r_builtin_help();
                        return EX_USAGE;
                    }
                     r_builtin_usage();
                     return EX_USAGE;
                 }
            }
        }

        list = loptend;
        /* Sanity-check variable name if -p supplied. */
        if vname != std::ptr::null_mut(){

            //这里有个条件编译，确定是否需要
            let  arrayflags:i32;
            if assoc_expand_once != 0{
                arrayflags = VA_NOEXPAND!() | VA_ONEWORD!();
            }
            else{
                arrayflags = 0;
            }

            if legal_identifier(vname) == 0 && valid_array_reference(vname,arrayflags) == 0{
                sh_invalidid(vname);
                return WAIT_RETURN!(EXECUTION_FAILURE!());
            }

            if r_builtin_unbind_variable(vname) == -2{
                return WAIT_RETURN!(EXECUTION_FAILURE!());
            }
        }
        /* POSIX.2 says:  When the shell is waiting (by means of the wait utility)
            for asynchronous commands to complete, the reception of a signal for
            which a trap has been set shall cause the wait utility to return
            immediately with an exit status greater than 128, after which the trap
            associated with the signal shall be taken.

            We handle SIGINT here; it's the only one that needs to be treated
            specially (I think), since it's handled specially in {no,}jobs.c. */

        wait_intr_flag = 1;

        code = __sigsetjmp(&mut wait_intr_buf, 1);//*mut [__jmp_buf_tag; 1]

        if code != 0{
            last_command_exit_signal = wait_signal_received;
            status = 128 + wait_signal_received;
            wait_sigint_cleanup();
            return WAIT_RETURN!(status);
        }

        opt = first_pending_trap();

        //#if define (SIGCHLD)

        /* We special case SIGCHLD when not in posix mode because we don't break
            out of the wait even when the signal is trapped; we run the trap after
            the wait completes. See how it's handled in jobs.c:waitchld(). */

        if opt==(SIGCHLD as i32) && posixly_correct==0{
            opt = next_pending_trap(opt+1);
        }
        if opt != -1{
            last_command_exit_signal = opt; 
            wait_signal_received = opt;
            status = opt +128;
            return WAIT_RETURN!(status);
        }

        //if define JB_CONTROL
            /* We support jobs or pids.
            wait <pid-or-job> [pid-or-job ...] */
        if nflag != 0{
            if list != std::ptr::null_mut(){
                opt = r_set_waitlist(list);
                if opt == 0{
                    return WAIT_RETURN!(127);
                }
                wflags |= JWAIT_WAITING!();
            }

            status = wait_for_any_job(wflags,&mut pstat);
            if vname!=std::ptr::null_mut() && status>=0{
                bind_var_to_int(vname,pstat.pid as intmax_t);
            }
            
            if status < 0{
                status = 127;
            }
            if list != std::ptr::null_mut(){
                r_unset_waitlist();
            }
            return WAIT_RETURN!(status);
        }
        //endif

        /* But wait without any arguments means to wait for all of the shell's
            currently active background processes. */
        if list == std::ptr::null_mut(){
            wait_for_background_pids(&mut pstat);
            if vname != std::ptr::null_mut() {
                bind_var_to_int(vname,pstat.pid as intmax_t);
            }
            return WAIT_RETURN!(EXECUTION_SUCCESS!());
            // WAIT_RETURN!()
        }

        status = EXECUTION_SUCCESS!();
        while list != std::ptr::null_mut(){
            let pid:pid_t;
            let w:*mut c_char;
            let mut pid_value:intmax_t = 0;

            w = (*(*list).word).word;
            if DIGIT(*w){
                if legal_number(w, &mut pid_value) !=0 && pid_value == (pid_value as pid_t) as i64 {
                    pid =pid_value as pid_t;
                    status = wait_for_single_pid(pid,wflags|JWAIT_PEROOR!());
                    pstat.pid = pid;
                    pstat.status = status as c_short;
                }
                else {
                    sh_badjob(w);
                    pstat.pid = NO_PID!();
                    pstat.status = 127;
                    return WAIT_RETURN!(EXECUTION_FAILURE!());
                }
            }

            //if defined (JOB_CONTROL)
            //else if  w != std::ptr::null_mut() && (w as u8)as char == '%' {
            else if *w != 0 && *w == '%' as libc::c_char {
                /* Must be a job spec.  Check it out. */
                let job:i32;
                let mut set:SigSet = SigSet::empty();
                let mut oset:SigSet = SigSet::empty();

                BLOCK_CHILD!(Some(&mut set),Some(&mut oset));
                job = r_get_job_spec(list);

                if INVALID_JOB!(job) == true{
                    if job != DUP_JOB!(){
                        sh_badjob( (*(*list).word).word);
                    }
                    UNBLOCK_CHILD!(Some(&mut oset));
                    status = 127;       /* As per Posix.2, section 4.70.2 */
                    pstat.pid = NO_PID!();
                    pstat.status = status as c_short;
                    list = (*list).next;
                    continue;
                }

                /* Job spec used.  Wait for the last pid in the pipeline. */
                UNBLOCK_CHILD!(Some(&mut oset));
                status = wait_for_job(job,wflags,&mut pstat)
            }
            else {
                sh_badjob(w);
                pstat.pid = NO_PID!();
                pstat.status = 127;
                status = EXECUTION_FAILURE!();
            }

            /* Don't waste time with a longjmp. */
            if wait_signal_received != 0{
                last_command_exit_signal = wait_signal_received;
                status = 128 + wait_signal_received;
                wait_sigint_cleanup();
                return WAIT_RETURN!(status);
            }

            list = (*list).next;
        }

        return WAIT_RETURN!(status);
            
    } //unsafe
}



#[no_mangle]
extern "C" fn r_set_waitlist(list:*mut WordList) -> i32{
    let mut set:SigSet = SigSet::empty();
    let mut oset:SigSet = SigSet::empty();
    let mut job:i32;
    let mut r:i32;
    let mut njob:i32;
    let mut pid:intmax_t=0;
    let mut l:*mut WordList;

    unsafe{
        BLOCK_CHILD!(Some(&mut set),Some(&mut oset));
        njob = 0;

        l = list;      
        while l != std::ptr::null_mut(){   //如何换成for
            job = NO_JOB!();
            
            if l!=std::ptr::null_mut() && legal_number( (*(*l).word).word, &mut pid ) != 0  && pid == (pid as pid_t) as i64{
                job = get_job_by_pid(pid as pid_t,0,std::ptr::null_mut());
            }
            else{
                r_get_job_spec(l);
            }

            if job == NO_JOB!() || jobs == std::ptr::null_mut() || INVALID_JOB!(job) {
                sh_badjob( (*(*l).word).word );
                continue;
            }

            /* We don't check yet to see if one of the desired jobs has already
             terminated, but we could. We wait until wait_for_any_job(). This
             has the advantage of validating all the arguments. */
            if (*get_job_by_jid!(job)).flags & J_WAITING!() == 0{
                njob = njob + 1;
                (*get_job_by_jid!(job)).flags |= J_WAITING!();
            }

            l = (*l).next;
        }
        UNBLOCK_CHILD!(Some(&mut oset));

        return njob;
    }//unsafe
    
}


// #[macro_export]
// macro_rules! get_job_by_jid {          //研究下jobs[i] 
//    ($ind:expr) => {
//     (*((jobs as usize + ($ind*8) as usize ) as *mut*mut JOB) as *mut JOB)
//     }
// }
/* Clean up after a call to wait -n jobs */
#[no_mangle]
extern "C" fn r_unset_waitlist(){
    // let mut i:i32;
    let mut set:SigSet = SigSet::empty();
    let mut oset:SigSet = SigSet::empty();

    unsafe{
        BLOCK_CHILD!(Some(&mut set),Some(&mut oset));
        for i in 0..js.j_jobslots{
            if get_job_by_jid!(i) != std::ptr::null_mut() && (*get_job_by_jid!(i)).flags & J_WAITING!(

            ) != 0{
                (*get_job_by_jid!(i)).flags &= !J_WAITING!();
            }
        }
    
        UNBLOCK_CHILD!(Some(&mut oset));
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
