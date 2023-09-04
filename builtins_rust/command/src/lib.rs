use std::ffi::*;

use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};
use rhelp::r_builtin_help;
extern "C" {
    fn copy_word_list(_: *mut WordList) -> *mut WordList;
    fn begin_unwind_frame(_: *mut libc::c_char);
    fn run_unwind_frame(_: *mut libc::c_char);
    fn add_unwind_protect();
    fn dispose_command(_: *mut COMMAND);
    fn make_bare_simple_command() -> *mut COMMAND;
    fn execute_command(_: *mut COMMAND) -> libc::c_int;
    static mut restricted: libc::c_int;
    static mut loptend: *mut WordList;
    fn internal_getopt(_: *mut WordList, _: *const libc::c_char) -> libc::c_int;
    fn reset_internal_getopt();
    fn builtin_usage();
    fn sh_notfound(_: *mut libc::c_char);
    fn sh_restricted(_: *mut libc::c_char);
    fn describe_command(_: *mut libc::c_char, _: libc::c_int) -> libc::c_int;
    //type Function = fn ()->i32;
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
pub type command_type = libc::c_uint;
pub const cm_coproc: command_type = 14;
pub const cm_subshell: command_type = 13;
pub const cm_arith_for: command_type = 12;
pub const cm_cond: command_type = 11;
pub const cm_arith: command_type = 10;
pub const cm_group: command_type = 9;
pub const cm_until: command_type = 8;
pub const cm_function_def: command_type = 7;
pub const cm_connection: command_type = 6;
pub const cm_select: command_type = 5;
pub const cm_simple: command_type = 4;
pub const cm_if: command_type = 3;
pub const cm_while: command_type = 2;
pub const cm_case: command_type = 1;
pub const cm_for: command_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union REDIRECTEE {
    pub dest: libc::c_int,
    pub filename: *mut WordDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redirect {
    pub next: *mut redirect,
    pub redirector: REDIRECTEE,
    pub rflags: libc::c_int,
    pub flags: libc::c_int,
    pub instruction: r_instruction,
    pub redirectee: REDIRECTEE,
    pub here_doc_eof: *mut libc::c_char,
}
pub type REDIRECT = redirect;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct command {
    pub type_0: command_type,
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub redirects: *mut REDIRECT,
    pub value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub For: *mut for_com,
    pub Case: *mut case_com,
    pub While: *mut while_com,
    pub If: *mut if_com,
    pub Connection: *mut connection,
    pub Simple: *mut simple_com,
    pub Function_def: *mut function_def,
    pub Group: *mut group_com,
    pub Select: *mut select_com,
    pub Arith: *mut arith_com,
    pub Cond: *mut cond_com,
    pub ArithFor: *mut arith_for_com,
    pub Subshell: *mut subshell_com,
    pub Coproc: *mut coproc_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coproc_com {
    pub flags: libc::c_int,
    pub name: *mut libc::c_char,
    pub command: *mut COMMAND,
}
pub type COMMAND = command;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct subshell_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub init: *mut WordList,
    pub test: *mut WordList,
    pub step: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub type_0: libc::c_int,
    pub op: *mut WordDesc,
    pub left: *mut cond_com,
    pub right: *mut cond_com,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub exp: *mut WordList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct select_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_com {
    pub ignore: libc::c_int,
    pub command: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct function_def {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub command: *mut COMMAND,
    pub source_file: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub words: *mut WordList,
    pub redirects: *mut REDIRECT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub ignore: libc::c_int,
    pub first: *mut COMMAND,
    pub second: *mut COMMAND,
    pub connector: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub true_case: *mut COMMAND,
    pub false_case: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct while_com {
    pub flags: libc::c_int,
    pub test: *mut COMMAND,
    pub action: *mut COMMAND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub word: *mut WordDesc,
    pub clauses: *mut PATTERN_LIST,
}
pub type PATTERN_LIST = pattern_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_list {
    pub next: *mut pattern_list,
    pub patterns: *mut WordList,
    pub action: *mut COMMAND,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct for_com {
    pub flags: libc::c_int,
    pub line: libc::c_int,
    pub name: *mut WordDesc,
    pub map_list: *mut WordList,
    pub action: *mut COMMAND,
}

pub const CDESC_ALL:i32 = 0x001;
pub const CDESC_SHORTDESC: i32 = 0x002;
pub const CDESC_REUSABLE:i32 = 0x004;
pub const CDESC_TYPE: i32 = 0x008;
pub const CDESC_PATH_ONLY:i32 = 0x010;
pub const CDESC_FORCE_PATH: i32 = 0x020;
pub const CDESC_NOFUNCS:i32 = 0x040;
pub const CDESC_ABSPATH: i32 = 0x080;
pub const CDESC_STDPATH: i32 = 0x100;

//#define CDESC_ALL		0x001	/* type -a */
//#define CDESC_SHORTDESC		0x002	/* command -V */
//#define CDESC_REUSABLE		0x004	/* command -v */
//#define CDESC_TYPE		0x008	/* type -t */
//#define CDESC_PATH_ONLY		0x010	/* type -p */
//#define CDESC_FORCE_PATH	0x020	/* type -ap or type -P */
//#define CDESC_NOFUNCS		0x040	/* type -f */
//#define CDESC_ABSPATH		0x080	/* convert to absolute path, no ./ */
//#define CDESC_STDPATH		0x100	/* command -p */


pub const const_command_builtin:*mut libc::c_char = b"command_builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char;//.unwrap();
//#define COMMAND_BUILTIN_FLAGS (CMD_NO_FUNCTIONS | CMD_INHIBIT_EXPANSION | CMD_COMMAND_BUILTIN | (use_standard_path ? CMD_STDPATH : 0))
//#define CMD_WANT_SUBSHELL  0x01	/* User wants a subshell: ( command ) */
//#define CMD_FORCE_SUBSHELL 0x02	/* Shell needs to force a subshell. */
//#define CMD_INVERT_RETURN  0x04	/* Invert the exit value. */
//#define CMD_IGNORE_RETURN  0x08	/* Ignore the exit value.  For set -e. */
//#define CMD_NO_FUNCTIONS   0x10 /* Ignore functions during command lookup. */
//#define CMD_INHIBIT_EXPANSION 0x20 /* Do not expand the command words. */
//#define CMD_NO_FORK	   0x40	/* Don't fork; just call execve */
//#define CMD_TIME_PIPELINE  0x80 /* Time a pipeline */
//#define CMD_TIME_POSIX	   0x100 /* time -p; use POSIX.2 time output spec. */
//#define CMD_AMPERSAND	   0x200 /* command & */
//#define CMD_STDIN_REDIR	   0x400 /* async command needs implicit </dev/null */
//#define CMD_COMMAND_BUILTIN 0x0800 /* command executed by `command' builtin */
//#define CMD_COPROC_SUBSHELL 0x1000
//#define CMD_LASTPIPE	    0x2000
//#define CMD_STDPATH	    0x4000	/* use standard path for command lookup */
//#define CMD_TRY_OPTIMIZING  0x8000	/* try to optimize this simple command */

pub const CMD_WANT_SUBSHELL :i32 = 0x01;
pub const CMD_FORCE_SUBSHELL :i32 = 0x02;
pub const CMD_INVERT_RETURN :i32 = 0x04;
pub const CMD_IGNORE_RETURN :i32 = 0x08;
pub const CMD_NO_FUNCTIONS :i32 = 0x10;
pub const CMD_INHIBIT_EXPANSION :i32 = 0x20;
pub const CMD_NO_FORK :i32 = 0x40;
pub const CMD_TIME_PIPELINE :i32 = 0x80;
pub const CMD_TIME_POSIX :i32 = 0x100;
pub const CMD_AMPERSAND :i32 = 0x200;
pub const CMD_STDIN_REDIR :i32 = 0x400;
pub const CMD_COMMAND_BUILTIN :i32 = 0x0800;
pub const CMD_COPROC_SUBSHELL :i32 = 0x1000;
pub const CMD_LASTPIPE :i32 = 0x2000;
pub const CMD_STDPATH :i32 = 0x4000;
pub const CMD_TRY_OPTIMIZING :i32 = 0x8000;

#[no_mangle]
pub unsafe extern "C" fn r_command_builtin(mut list: *mut WordList) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut verbose: libc::c_int = 0;
    let mut use_standard_path: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut command: *mut COMMAND = 0 as *mut COMMAND;
    use_standard_path = 0 as libc::c_int;
    verbose = use_standard_path;

    reset_internal_getopt();
    let adnpsf = CString::new("pvV").expect("CString::new failed");
    loop {
        opt = internal_getopt(list, adnpsf.as_ptr() );
        if !(opt != -1) {
            break;
        }
        let opt_char = opt as u8 as char;
        match opt_char {
            'p' => {
                use_standard_path = CDESC_STDPATH;
            }
            'V' => {
                verbose = CDESC_SHORTDESC | CDESC_ABSPATH;
            }
            'v' => {
                verbose = CDESC_REUSABLE; // ditto
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
    if list.is_null() {
        return EXECUTION_SUCCESS!();
    }
    if use_standard_path != 0 && restricted != 0 {
        sh_restricted(b"-p\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return EXECUTION_FAILURE!();
    }
    if verbose != 0 {
        let mut found: libc::c_int = 0;
        let mut any_found: libc::c_int = 0;
        any_found = 0 as libc::c_int;
        while !list.is_null() {
            found = describe_command((*(*list).word).word, verbose | use_standard_path);
            if found == 0 as libc::c_int && verbose != CDESC_REUSABLE {
                sh_notfound((*(*list).word).word);
            }
            any_found += found;
            list = (*list).next;
        }
        return if any_found != 0 { EXECUTION_SUCCESS!() } else { EXECUTION_FAILURE!() };
    }
    begin_unwind_frame(
        const_command_builtin
    );
    command = make_bare_simple_command();
    // let ref mut fresh0 = (*(*command).value.Simple).words;
    //*fresh0 = copy_word_list(list);
    (*(*command).value.Simple).words = copy_word_list(list);
    //let ref mut fresh1 = (*(*command).value.Simple).redirects;
    //*fresh1 = 0 as *mut libc::c_void as *mut REDIRECT;
    (*(*command).value.Simple).redirects = 0  as *mut libc::c_void as *mut REDIRECT;

    (*command).flags
        |= CMD_NO_FUNCTIONS | CMD_INHIBIT_EXPANSION | CMD_COMMAND_BUILTIN
            | (if use_standard_path != 0 {
                CMD_STDPATH
            } else {
                0 as libc::c_int
            });
    (*(*command).value.Simple).flags
        |= CMD_NO_FUNCTIONS | CMD_INHIBIT_EXPANSION | CMD_COMMAND_BUILTIN
        | (if use_standard_path != 0 {
            CMD_STDPATH
        } else {
            0 as libc::c_int
        });
    /*add_unwind_protect(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut COMMAND) -> ()>,
            *mut libc::c_char,
        >(Some(dispose_command as unsafe extern "C" fn(*mut COMMAND) -> ())),
        command,
    );*/
    result = execute_command(command);
    run_unwind_frame(
        b"command_builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return result;
}
