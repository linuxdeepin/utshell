extern crate rcommon;
use std::ffi::*;
use rset::r_set_shellopts;
use libc::*;

use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage};
use rhelp::r_builtin_help;
/*
/* First, the user-visible attributes */
#define att_exported	0x0000001	/* export to environment */
#define att_readonly	0x0000002	/* cannot change */
#define att_array	0x0000004	/* value is an array */
#define att_function	0x0000008	/* value is a function */
#define att_integer	0x0000010	/* internal representation is int */
#define att_local	0x0000020	/* variable is local to a function */
#define att_assoc	0x0000040	/* variable is an associative array */
#define att_trace	0x0000080	/* function is traced with DEBUG trap */
#define att_uppercase	0x0000100	/* word converted to uppercase on assignment */
#define att_lowercase	0x0000200	/* word converted to lowercase on assignment */
#define att_capcase	0x0000400	/* word capitalized on assignment */
#define att_nameref	0x0000800	/* word is a name reference */

#define user_attrs	(att_exported|att_readonly|att_integer|att_local|att_trace|att_uppercase|att_lowercase|att_capcase|att_nameref)

#define attmask_user	0x0000fff

/* Internal attributes used for bookkeeping */
#define att_invisible	0x0001000	/* cannot see */
#define att_nounset	0x0002000	/* cannot unset */
#define att_noassign	0x0004000	/* assignment not allowed */
#define att_imported	0x0008000	/* came from environment */
#define att_special	0x0010000	/* requires special handling */
#define att_nofree	0x0020000	/* do not free value on unset */
#define att_regenerate	0x0040000	/* regenerate when exported */

#define	attmask_int	0x00ff000

/* Internal attributes used for variable scoping. */
#define att_tempvar	0x0100000	/* variable came from the temp environment */
#define att_propagate	0x0200000	/* propagate to previous scope */

#define attmask_scope	0x0f00000
*/

pub static att_exported:i32= 0x0000001;	/* export to environment */
pub static att_readonly:i32= 0x0000002;	/* cannot change */
pub static att_array:i32= 0x0000004;	/* value is an array */
pub static att_function:i32= 0x0000008;	/* value is a function */
pub static att_integer:i32= 0x0000010;	/* internal representation is int */
pub static att_local:i32= 0x0000020;	/* variable is local to a function */
pub static att_assoc:i32= 0x0000040;	/* variable is an associative array */
pub static att_trace:i32= 0x0000080;	/* function is traced with DEBUG trap */
pub static att_uppercase:i32= 0x0000100;	/* word converted to uppercase on assignment */
pub static att_lowercase:i32= 0x0000200;	/* word converted to lowercase on assignment */
pub static att_capcase:i32= 0x0000400;	/* word capitalized on assignment */
pub static att_nameref:i32= 0x0000800;	/* word is a name reference */

pub static user_attrs:i32=att_exported|att_readonly|att_integer|att_local|att_trace|att_uppercase|att_lowercase|att_capcase|att_nameref;

pub static attmask_user:i32= 0x0000fff;

/* Internal attributes used for bookkeeping */
pub static att_invisible:i32= 0x0001000;	/* cannot see */
pub static att_nounset:i32= 0x0002000;	/* cannot unset */
pub static att_noassign:i32= 0x0004000;	/* assignment not allowed */
pub static att_imported:i32= 0x0008000;	/* came from environment */
pub static att_special:i32= 0x0010000;	/* requires special handling */
pub static att_nofree:i32= 0x0020000;	/* do not free value on unset */
pub static att_regenerate:i32= 0x0040000;	/* regenerate when exported */

pub static	attmask_int:i32= 0x00ff000;

/* Internal attributes used for variable scoping. */
pub static att_tempvar:i32=0x0100000;	/* variable came from the temp environment */
pub static att_propagate:i32= 0x0200000;	/* propagate to previous scope */

pub static attmask_scope:i32= 0x0f00000;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: i32,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn free(__ptr: *mut libc::c_void);
    fn xmalloc(_: SizeT) -> *mut libc::c_void;
    fn extract_colon_unit(
        _: *mut libc::c_char,
        _: *mut i32,
    ) -> *mut libc::c_char;
    static mut localvar_inherit: i32;
    fn find_variable(_: *const libc::c_char) -> *mut ShellVar;
    fn bind_variable(
        _: *const libc::c_char,
        _: *mut libc::c_char,
        _: i32,
    ) -> *mut ShellVar;
    fn init_bash_argv();
    static mut assoc_expand_once: i32;
    fn dispose_words(_: *mut WordList);
    fn make_word(_: *const libc::c_char) -> *mut WordDesc;
    fn make_word_list(_: *mut WordDesc, _: *mut WordList) -> *mut WordList;
    static mut inherit_errexit: i32;
    fn shell_is_restricted(_: *mut libc::c_char) -> i32;
    fn itos(_: intmax_t) -> *mut libc::c_char;
    fn strvec_create(_: i32) -> *mut *mut libc::c_char;
    static mut debugging_mode: i32;
    static mut login_shell: i32;
    static mut shell_compatibility_level: i32;
    static mut shell_name: *mut libc::c_char;
    static mut hup_on_exit: i32;
    static mut check_jobs_at_exit: i32;
    static mut autocd: i32;
    static mut check_window_size: i32;
    static mut mark_modified_vars: i32;
    static mut interactive_comments: i32;
    static mut function_trace_mode: i32;
    static mut error_trace_mode: i32;
    static mut restricted_shell: i32;
    fn builtin_error(_: *const libc::c_char, _: ...);
    fn builtin_usage();
    fn sh_invalidoptname(_: *mut libc::c_char);
    fn sh_chkwrite(_: i32) -> libc::c_int;
    fn minus_o_option_value(_: *mut libc::c_char) -> i32;
    fn list_minus_o_opts(_: i32, _: i32);
    fn set_minus_o_option(_: i32, _: *mut libc::c_char) -> libc::c_int;
    //fn r_set_shellopts();
    static mut print_shift_error: i32;
    static mut source_uses_path: i32;
    static mut loptend: *mut WordList;
    fn internal_getopt(_: *mut WordList, _: *mut libc::c_char) -> i32;
    fn reset_internal_getopt();
    static mut perform_hostname_completion: i32;
    static mut no_empty_command_completion: i32;
    static mut force_fignore: i32;
    static mut dircomplete_spelling: i32;
    static mut dircomplete_expand: i32;
    static mut complete_fullquote: i32;
    fn enable_hostname_completion(_: i32) -> libc::c_int;
    fn set_directory_hook();
    static mut literal_history: i32;
    static mut force_append_history: i32;
    static mut command_oriented_history: i32;
    static mut hist_verify: i32;
    static mut allow_null_glob_expansion: i32;
    static mut fail_glob_expansion: i32;
    static mut glob_dot_filenames: i32;
    static mut cdable_vars: i32;
    static mut mail_warning: i32;
    static mut no_exit_on_failed_exec: i32;
    static mut check_hashed_filenames: i32;
    static mut promptvars: i32;
    static mut cdspelling: i32;
    static mut expand_aliases: i32;
    static mut extended_quote: i32;
    static mut glob_ignore_case: i32;
    static mut match_ignore_case: i32;
    static mut xpg_echo: i32;
    static mut gnu_error_format: i32;
    static mut glob_star: i32;
    static mut glob_asciirange: i32;
    static mut lastpipe_opt: i32;
    static mut localvar_unset: i32;
    static mut extended_glob: i32;
    static mut history_reediting: i32;
    static mut prog_completion_enabled: i32;
    static mut progcomp_alias: i32;
}
pub type SizeT = libc::c_ulong;
pub type IntmaxT = libc::c_long;

pub type ArrayindT = intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub exportstr: *mut libc::c_char,
    pub dynamic_value: Option::<ShVarValueFuncT>,
    pub assign_func: Option::<ShVarAssignFuncT>,
    pub attributes: i32,
    pub context: i32,
}
pub type ShVarAssignFuncT = unsafe extern "C" fn(
    *mut variable,
    *mut libc::c_char,
    ArrayindT,
    *mut libc::c_char,
) -> *mut variable;
pub type ShVarValueFuncT = unsafe extern "C" fn(*mut variable) -> *mut variable;
pub type ShellVar = variable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RShoptVars {
    pub name: *mut libc::c_char,
    pub value: *mut i32,
    pub set_func: Option::<ShoptSetFuncT>,
}
pub type ShoptSetFuncT = unsafe extern "C" fn(
    *mut libc::c_char,
    i32,
) -> i32;
static mut SHOPT_LOGIN_SHELL: i32 = 0;
static mut SHOPT_COMPAT31: i32 = 0;
static mut SHOPT_COMPAT32: i32 = 0;
static mut SHOPT_COMPAT40: i32 = 0;
static mut SHOPT_COMPAT41: i32 = 0;
static mut SHOPT_COMPAT42: i32 = 0;
static mut SHOPT_COMPAT43: i32 = 0;
static mut SHOPT_COMPAT44: i32 = 0;
static mut SHOPT_VARS: [RShoptVars; 54] = unsafe {
    [
        {
            let init = RShoptVars {
                name: b"autocd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &autocd as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"assoc_expand_once\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &assoc_expand_once as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cdable_vars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &cdable_vars as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cdspell\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &cdspelling as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkhash\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &check_hashed_filenames as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkjobs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &check_jobs_at_exit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"checkwinsize\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &check_window_size as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"cmdhist\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &command_oriented_history as *const i32
                    as *mut i32,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat31\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT31 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat32\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT32 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat40\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT40 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat41\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT41 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat42\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT42 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat43\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT43 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"compat44\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_COMPAT44 as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_compatibility_level
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"complete_fullquote\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &complete_fullquote as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"direxpand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &dircomplete_expand as *const i32 as *mut libc::c_int,
                set_func: Some(
                    shopt_set_complete_direxpand
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"dirspell\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &dircomplete_spelling as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"dotglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &glob_dot_filenames as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"execfail\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &no_exit_on_failed_exec as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"expand_aliases\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &expand_aliases as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extdebug\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &debugging_mode as *const i32 as *mut libc::c_int,
                set_func: Some(
                    shopt_set_debug_mode
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &extended_glob as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"extquote\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &extended_quote as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"failglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &fail_glob_expansion as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"force_fignore\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &force_fignore as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"globasciiranges\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &glob_asciirange as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"globstar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &glob_star as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"gnu_errfmt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &gnu_error_format as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histappend\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &force_append_history as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histreedit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &history_reediting as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"histverify\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &hist_verify as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"hostcomplete\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &perform_hostname_completion as *const i32
                    as *mut i32,
                set_func: Some(
                    shopt_enable_hostname_completion
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"huponexit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &hup_on_exit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"inherit_errexit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &inherit_errexit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"interactive_comments\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &interactive_comments as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_shellopts_after_change
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"lastpipe\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &lastpipe_opt as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"lithist\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &literal_history as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"localvar_inherit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &localvar_inherit as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"localvar_unset\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &localvar_unset as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"login_shell\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &SHOPT_LOGIN_SHELL as *const i32 as *mut i32,
                set_func: Some(
                    r_set_login_shell
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"mailwarn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &mail_warning as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"no_empty_cmd_completion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &no_empty_command_completion as *const i32
                    as *mut i32,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nocaseglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &glob_ignore_case as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nocasematch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &match_ignore_case as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"nullglob\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &allow_null_glob_expansion as *const i32
                    as *mut i32,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"progcomp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &prog_completion_enabled as *const i32
                    as *mut i32,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"progcomp_alias\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &progcomp_alias as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"promptvars\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &promptvars as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"restricted_shell\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &restricted_shell as *const i32 as *mut libc::c_int,
                set_func: Some(
                    set_restricted_shell
                        as unsafe extern "C" fn(
                            *mut libc::c_char,
                            i32,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"shift_verbose\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &print_shift_error as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"sourcepath\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &source_uses_path as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: b"xpg_echo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                value: &xpg_echo as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let init = RShoptVars {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                value: 0 as *const i32 as *mut libc::c_int,
                set_func: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<ShoptSetFuncT>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};

static SFLAG:i32 = 0x01;
static UFLAG:i32 = 0x02;
static QFLAG:i32 = 0x04;
static OFLAG:i32 = 0x08;
static PFLAG:i32 = 0x10;
static SETOPT:i32 = 1;
static UNSETOPT:i32 = 0;

static mut ON: *const libc::c_char = b"on\0" as *const u8 as *const libc::c_char;
static mut OFF: *const libc::c_char = b"off\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn r_shopt_builtin(mut list: *mut WordList) -> i32 {
    let mut opt: i32;
    let mut flags: i32 = 0;
    let mut rval: i32 = 0;

    reset_internal_getopt();
    let psuoq = CString::new("psuoq").expect("CString::new failed");
    loop {
        opt = internal_getopt( list, psuoq.as_ptr() as *mut libc::c_char);
        if !(opt != -(1 as i32)) {
            break;
        }
        let opt_char = opt as u8 as char;
        match opt_char {
            's' => {
                flags |= SFLAG;
            }
            'u' => {
                flags |= UFLAG
            }
            'q' => {
                flags |= QFLAG;
            }
            'o' => {
                flags |= OFLAG;
            }
            'p' => {
                flags |= PFLAG;
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
    if flags & (SFLAG | UFLAG) == SFLAG | UFLAG {
        builtin_error(
            dcgettext(
                0 as *const libc::c_char,
                b"cannot set and unset shell options simultaneously\0" as *const u8
                    as *const libc::c_char,
                5 as i32,
            ),
        );
        return EXECUTION_FAILURE!();
    }

    if (flags & OFLAG != 0) &&( (flags & (SFLAG | UFLAG)) == 0) // shopt -o
    {//设置了o-flag，并没设s或u-flag
        rval = r_list_shopt_o_options(list, flags);
    } else if !list.is_null() && flags & OFLAG != 0 {  //设置-o了   //shopt -so args , shopt -u args
        rval = set_shopt_o_options(
            if flags & SFLAG != 0 { '-' as i32 /*on*/} else { '+' as i32 /*off*/},
            list,
            flags & QFLAG,//是否沉默?
        );
    } else if flags & OFLAG != 0 {                        // shopt -so
        rval = list_some_o_options(
            if flags & SFLAG != 0 { 1 } else { 0 },
            flags,
        );
    } else if !list.is_null() && flags & (SFLAG | UFLAG) != 0 {    // shopt -s/u args
        rval = toggle_shopts(
            if flags & SFLAG != 0 { 1 } else { 0 },
            list,
            flags & QFLAG,
        );
    } else if flags & (SFLAG | UFLAG) == 0 {                        // shopt [args]
        //println!("shopt   ===list all ");
        rval = r_list_shopts(list, flags);
    } else {                                                        // shopt -su
        rval = list_some_shopts(
            if flags & SFLAG != 0 { SETOPT } else { UNSETOPT },
            flags,
        );
    }
    return rval;
}


// 把环境变量置0
#[no_mangle]
pub unsafe extern "C" fn r_reset_shopt_options() {
    cdspelling = 0;
    cdable_vars = 0;
    autocd = 0;
    check_hashed_filenames = 0;
    check_window_size = 1;
    glob_dot_filenames = 0;
    allow_null_glob_expansion = glob_dot_filenames;
    no_exit_on_failed_exec = 0;
    expand_aliases = 0;
    extended_quote = 1;
    fail_glob_expansion = 0;
    glob_asciirange = 1;
    glob_star = 0;
    gnu_error_format = 0;
    hup_on_exit = 0;
    inherit_errexit = 0;
    interactive_comments = 1;
    lastpipe_opt = 0;
    localvar_unset = 0;
    localvar_inherit = localvar_unset;
    mail_warning = 0;
    match_ignore_case = 0;
    glob_ignore_case = match_ignore_case;
    print_shift_error = 0;
    promptvars = 1;
    source_uses_path = promptvars;
    check_jobs_at_exit = 0;
    extended_glob = 0;
    assoc_expand_once = 0;
    literal_history = 0;
    force_append_history = 0;
    command_oriented_history = 1;
    complete_fullquote = 1;
    force_fignore = 1;
    history_reediting = 0;
    hist_verify = history_reediting;
    perform_hostname_completion = 1;
    dircomplete_expand = 0;
    dircomplete_spelling = 0;
    no_empty_command_completion = 0;
    prog_completion_enabled = 1;
    progcomp_alias = 0;
    xpg_echo = 0;
    SHOPT_LOGIN_SHELL = login_shell;
}

unsafe extern "C" fn find_shopt( name: *mut libc::c_char) -> i32 {
    let mut i = 0;
    for item in SHOPT_VARS {
        i += 1;
        if item.name.is_null() {return -1;}
    if strcmp(name, SHOPT_VARS[i as usize].name) == 0 {
        return i; 
    }

    }
    return -1;
}

unsafe extern "C" fn shopt_error( s: *mut libc::c_char) {
    builtin_error(
        dcgettext(
            0 as *const libc::c_char,
            b"%s: invalid shell option name\0" as *const u8 as *const libc::c_char,
            5 as i32,
        ),
        s,
    );
}

unsafe extern "C" fn toggle_shopts(
     mode: i32,
     list: *mut WordList,
     _quiet: i32,
) -> i32 {
    // printf(CString::new(" set command: %s mode=%d").expect("").as_ptr() ,(*(*list).word).word, mode);
    let mut l: *mut WordList;
    let mut ind:i32;
    let mut rval: i32;
    let  v: *mut ShellVar;
    l = list;
    rval = EXECUTION_SUCCESS!();
    while !l.is_null() {
        ind = find_shopt((*(*l).word).word);
        if ind < 0 {
            shopt_error((*(*l).word).word);
            rval = EXECUTION_FAILURE!();
        } else {
            *SHOPT_VARS[ind as usize].value = mode;
            if (SHOPT_VARS[ind as usize].set_func).is_some() {
                       SHOPT_VARS[ind as usize].set_func.expect("") (SHOPT_VARS[ind as usize].name, mode);
                /*
                (Some(
                    ((*SHOPT_VARS.as_mut_ptr().offset(ind as isize)).set_func)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(SHOPT_VARS[ind as usize].name, mode);
                */
            }
        }
        l = (*l).next;
    }
    v = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);
    if !v.is_null() {
        r_set_bashopts();
    }
    return rval;
}
unsafe extern "C" fn print_shopt(
    name: *mut libc::c_char,
    val: i32,
    flags: i32,
) {

    let msg: CString = CString::new("shopt %s %s\n").expect("CString new faild");
    let s: CString = CString::new("-s").expect("CString new faild");
    let u: CString = CString::new("-u").expect("CString new faild");
    let optfmt: CString = CString::new("%-15s\t%s\n").expect("CString new faild");
    if flags & PFLAG != 0 {
            printf(
            msg.as_ptr(),
            if val != 0 {
                s.as_ptr()
            } else {
                u.as_ptr()
            },
            name,
        );
    } else {
        printf(
            optfmt.as_ptr(),
            name,
            if val != 0 { ON } else { OFF },
        );
    };
}
unsafe extern "C" fn r_list_shopts(
     list: *mut WordList,
     flags: i32,
) -> i32 {
    let mut l:*mut WordList;
    let mut i;
    let mut val = 0;
    let mut rval =0;
    rval = EXECUTION_SUCCESS!();
    if (flags & QFLAG) ==0 {
        if list.is_null() {
            for item in SHOPT_VARS {
              if  item.value != std::ptr::null_mut()  {
                val = *item.value;
                    print_shopt(item.name, val, flags);
              }
            }
            return sh_chkwrite(EXECUTION_SUCCESS!());
        }
        l = list;
        while !l.is_null() {
            i = find_shopt((*(*l).word).word);
            if i < 0 {
                shopt_error((*(*l).word).word);
                rval = EXECUTION_FAILURE!();
            } else {
                val = *SHOPT_VARS[i as usize].value;
                if val == 0 {
                    rval = EXECUTION_FAILURE!();
                }
                print_shopt((*(*l).word).word, val, flags);
            }
            l = (*l).next;
        }

    }
    return sh_chkwrite(rval);
}

unsafe extern "C" fn list_some_shopts(
    mode: i32,
    flags: i32,
) -> i32 {
    for item in SHOPT_VARS {
        //if !item.name.is_null()  {
        //printf(b"===name=%s, value=%d\n\0" as *const u8 as *const libc::c_char, item.name as *const u8 as *const libc::c_char, *item.value);
        //}
        if ((flags & QFLAG) == 0 )&& item.value != std::ptr::null_mut() && mode==*item.value {
            print_shopt(item.name, *item.value, flags);
        }
    }
    return sh_chkwrite(EXECUTION_SUCCESS!());
}

unsafe extern "C" fn r_list_shopt_o_options(
    list: *mut WordList,
    flags: i32,
) -> i32 {
    let mut l: *mut WordList = 0 as *mut WordList;
    let mut val: i32 = 0;
    let mut rval: i32 = EXECUTION_SUCCESS!();
    if list.is_null() {
        if flags & QFLAG == 0 {
            list_minus_o_opts(-1, flags & PFLAG);
        }
        return sh_chkwrite(EXECUTION_SUCCESS!());
    }
    l = list;

    while !l.is_null() {
        val = minus_o_option_value((*(*l).word).word);
        if val == -1 {
            sh_invalidoptname((*(*l).word).word);
            rval = EXECUTION_FAILURE!();
        } else {
            if val == 0 {
                rval = EXECUTION_FAILURE!();
            }
            if flags & QFLAG == 0  {
                if flags & PFLAG != 0 {
                    printf(
                        b"set %co %s\n\0" as *const u8 as *const libc::c_char,
                        if val != 0 { '-' as i32 } else { '+' as i32 },
                        (*(*l).word).word,
                    );
                    println!("set {:?}o %{:?}",if val !=0 {b'-'} else {b'+'}, (*(*l).word).word)
                } else {
                    printf(
                        b"%-15s\t%s\n\0" as *const u8 as *const libc::c_char,
                        (*(*l).word).word,
                        if val != 0 { ON } else { OFF },
                    );
                }
            }
        }
        l = (*l).next;
    }
    return sh_chkwrite(rval);
}
////0701
unsafe extern "C" fn list_some_o_options(
    mode: i32,
    flags: i32,
) -> i32 {
    if flags & QFLAG == 0 {
        list_minus_o_opts(mode, flags & PFLAG);
    }
    return sh_chkwrite(EXECUTION_SUCCESS!());
}
unsafe extern "C" fn set_shopt_o_options(
    mode: i32,
    list: *mut WordList,
    quiet: i32,
) -> i32 {
    //let mut l: *mut WordList =0 as *mut WordList;
    let mut l: *mut WordList;
    let mut rval: i32 ;
    l = list;
    rval = EXECUTION_SUCCESS!();
    while !l.is_null() {
        if set_minus_o_option(mode, (*(*l).word).word) == 1 as i32 {
            rval = 1 as i32;
        }
        l = (*l).next;
    }
    r_set_shellopts();
    return rval;
}
unsafe extern "C" fn set_shellopts_after_change(
     option_name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    r_set_shellopts();
    return 0;
}
unsafe extern "C" fn shopt_set_debug_mode(
     option_name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    function_trace_mode = debugging_mode;
    error_trace_mode = function_trace_mode;
    r_set_shellopts();
    if debugging_mode != 0 {
        init_bash_argv();
    }
    return 0;
}
unsafe extern "C" fn shopt_enable_hostname_completion(
    option_name: *mut libc::c_char,
    mode: i32,
) -> i32 {
    return enable_hostname_completion(mode);
}
unsafe extern "C" fn set_compatibility_level(
    option_name: *mut libc::c_char,
    mode: i32,
) -> i32 {
    let mut ind: i32 = 0;
    let mut rhs: *mut libc::c_char = 0 as *mut libc::c_char;
    if mode != 0 {
        SHOPT_COMPAT32 = 0 as i32;
        SHOPT_COMPAT31 = SHOPT_COMPAT32;
        SHOPT_COMPAT43 = 0 as i32;
        SHOPT_COMPAT42 = SHOPT_COMPAT43;
        SHOPT_COMPAT41 = SHOPT_COMPAT42;
        SHOPT_COMPAT40 = SHOPT_COMPAT41;
        SHOPT_COMPAT44 = 0 as i32;
        ind = find_shopt(option_name);
        *SHOPT_VARS[ind as usize].value = mode;
    }
    if SHOPT_COMPAT31 != 0 {
        shell_compatibility_level = 31 as i32;
    } else if SHOPT_COMPAT32 != 0 {
        shell_compatibility_level = 32 as i32;
    } else if SHOPT_COMPAT40 != 0 {
        shell_compatibility_level = 40 as i32;
    } else if SHOPT_COMPAT41 != 0 {
        shell_compatibility_level = 41 as i32;
    } else if SHOPT_COMPAT42 != 0 {
        shell_compatibility_level = 42 as i32;
    } else if SHOPT_COMPAT43 != 0 {
        shell_compatibility_level = 43 as i32;
    } else if SHOPT_COMPAT44 != 0 {
        shell_compatibility_level = 44 as i32;
    } else {
        shell_compatibility_level = 51 as i32;
    }
    rhs = itos(shell_compatibility_level as intmax_t);
    bind_variable(
        b"BASH_COMPAT\0" as *const u8 as *const libc::c_char,
        rhs,
        0 as i32,
    );
    free(rhs as *mut libc::c_void);
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn r_set_compatibility_opts() {
    SHOPT_COMPAT32 = 0;
    SHOPT_COMPAT31 = 0;
    SHOPT_COMPAT43 = 0;
    SHOPT_COMPAT42 = 0;
    SHOPT_COMPAT41 = 0;
    SHOPT_COMPAT40 = 0;
    SHOPT_COMPAT44 = 0;
    match shell_compatibility_level {
        44 => {
            SHOPT_COMPAT44 = 1;
        }
        43 => {
            SHOPT_COMPAT43 = 1;
        }
        42 => {
            SHOPT_COMPAT42 = 1;
        }
        41 => {
            SHOPT_COMPAT41 = 1;
        }
        40 => {
            SHOPT_COMPAT40 = 1;
        }
        32 => {
            SHOPT_COMPAT32 = 1;
        }
        31 => {
            SHOPT_COMPAT31 = 1;
        }
        _ => {}
    };
}
unsafe extern "C" fn shopt_set_complete_direxpand(
     option_name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    set_directory_hook();
    return 0;
}
unsafe extern "C" fn set_restricted_shell(
     option_name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    static mut SAVE_RESTRICTED:i32 = -1;
    if SAVE_RESTRICTED == -1 {
        SAVE_RESTRICTED = shell_is_restricted(shell_name);
    }
    restricted_shell = SAVE_RESTRICTED;
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn r_set_login_shell(
     option_name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    SHOPT_LOGIN_SHELL = if login_shell != 0 {1} else{0};
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn r_get_shopt_options() -> *mut *mut libc::c_char {
    let mut ret: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    n = (::std::mem::size_of::<[RShoptVars; 54]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<RShoptVars>() as libc::c_ulong)
        as i32;
    ret = strvec_create(n + 1 as i32);
    i = 0 as i32;
    while !(SHOPT_VARS[i as usize].name).is_null() {
        let ref mut fresh0 = *ret.offset(i as isize);
        *fresh0 = strcpy(
            xmalloc(
                (1 as i32 as libc::c_ulong)
                    .wrapping_add(strlen(SHOPT_VARS[i as usize].name)),
            ) as *mut libc::c_char,
            SHOPT_VARS[i as usize].name,
        );
        i += 1;
    }
    let ref mut fresh1 = *ret.offset(i as isize);
    *fresh1 = 0 as *mut libc::c_void as *mut libc::c_char;
    return ret;
}

///0707begin  
#[no_mangle]
pub unsafe extern "C" fn r_shopt_setopt(
     name: *mut libc::c_char,
     mode: i32,
) -> i32 {
    let  wl: *mut WordList;
    let  r: i32;
    wl = make_word_list(make_word(name), std::ptr::null_mut());
    //wl = make_word_list(make_word(name), 0 as *mut libc::c_void as *mut WordList);
    r = toggle_shopts(mode, wl, 0);
    dispose_words(wl);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn r_shopt_listopt(
     name: *mut libc::c_char,
     reusable: i32,
) -> i32 {
    let mut i: i32 = 0;
    if name.is_null() {
        return r_list_shopts(
           // 0 as *mut libc::c_void as *mut WordList,
           std::ptr::null_mut(),
            if reusable != 0 { PFLAG } else { 0 },
        );
    }
    i = find_shopt(name);
    if i < 0 {
        shopt_error(name);
        return 1;
    }
    print_shopt(
        name,
        *SHOPT_VARS[i as usize].value,
        if reusable != 0 { PFLAG } else { 0 },
    );
    return sh_chkwrite(EXECUTION_SUCCESS!());
}
#[no_mangle]
pub unsafe extern "C" fn r_set_bashopts() {
    let value: *mut libc::c_char ;
    let mut tflag: [libc::c_char; 54] = [0; 54];
    let mut vsize: i32;
    let mut i: i32;
    let mut vptr: i32;
    /*
    let mut ip: *mut i32;
    */
    let  exported: i32;
    let mut v: *mut ShellVar;
    i = 0;
    vsize = 0;
    while !(SHOPT_VARS[i as usize].name).is_null() {
        tflag[i as usize] = 0 as libc::c_char;
        if *SHOPT_VARS[i as usize].value != 0 {
            vsize += strlen(SHOPT_VARS[i as usize].name) as i32;
            vsize += 1;
            /*
            vsize = (vsize as libc::c_ulong)
                .wrapping_add(
                    (strlen(SHOPT_VARS[i as usize].name))
                        .wrapping_add(1 as i32 as libc::c_ulong),
                ) as i32 as libc::c_int;
            */
            tflag[i as usize] = 1 as libc::c_char;
        }
        i += 1;
    }
    value = libc::malloc((vsize + 1 ) as usize) as *mut libc::c_char;
    vptr = 0;
    i = 0;
    while !(SHOPT_VARS[i as usize].name).is_null() {
        if tflag[i as usize] != 0 {
            strcpy(value.offset(vptr as isize), SHOPT_VARS[i as usize].name);
            vptr = (vptr as libc::c_ulong)
                .wrapping_add(strlen(SHOPT_VARS[i as usize].name)) as i32
                as i32;
            let fresh2 = vptr;
            vptr = vptr + 1;
            *value.offset(fresh2 as isize) = ':' as i32 as libc::c_char;
        }
        i += 1;
    }
    //printf(b"the values=%s" as *const u8 as *const libc::c_char, value);
    if vptr != 0 {
        vptr -= 1;
    }
    *value.offset(vptr as isize) = '\u{0}' as i32 as libc::c_char;
    v = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);
    if !v.is_null() {
        (*v).attributes &= !(0x2 as i32);
        exported = (*v).attributes & 0x1 as i32;
    } else {
        exported = 0;
    }
    v = bind_variable(
        b"BASHOPTS\0" as *const u8 as *const libc::c_char,
        value,
        0,
    );
    (*v).attributes |= 0x2;
    if mark_modified_vars != 0 && exported == 0 as i32
        && (*v).attributes & 0x1 != 0
    {
        (*v).attributes &= !(0x1);
    }
    libc::free(value as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn r_parse_bashopts( value: *mut libc::c_char) {
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vptr: i32 = 0;
    let mut ind: i32 = 0;
    vptr = 0 as i32;
    loop {
        vname = extract_colon_unit(value, &mut vptr);
        if vname.is_null() {
            break;
        }
        ind = find_shopt(vname);
        if ind >= 0 as i32 {
            *SHOPT_VARS[ind as usize].value = 1 as i32;
            if (SHOPT_VARS[ind as usize].set_func).is_some() {
                (Some(
                    ((*SHOPT_VARS.as_mut_ptr().offset(ind as isize)).set_func)
                        .expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(SHOPT_VARS[ind as usize].name, 1 as i32);
            }
        }
        free(vname as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn r_initialize_bashopts(no_bashopts: i32) {
    let  temp: *mut libc::c_char;
    let  var: *mut ShellVar;
    if no_bashopts == 0 {
        var = find_variable(b"BASHOPTS\0" as *const u8 as *const libc::c_char);
        if !var.is_null() && (*var).attributes & att_imported != 0 {
            temp = if (*var).attributes & att_array != 0
                || (*var).attributes & att_assoc != 0
            {
                std::ptr::null_mut()
            } else {
                strcpy(
                    xmalloc(
                        (1 as libc::c_ulong)
                            .wrapping_add(strlen((*var).value)),
                    ) as *mut libc::c_char,
                    (*var).value,
                )
            };
            if !temp.is_null() {
                r_parse_bashopts(temp);
                free(temp as *mut libc::c_void);
            }
        }
    }
    r_set_bashopts();
}

