
use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE};

extern "C" {
    static mut loption :*mut WordList;
    pub fn no_options(list: *mut WordList) -> c_int;
    pub fn builtin_address(command: *const c_char) -> extern "C" fn(w:*mut WordList) ->i32;

    static mut this_command_name: *mut libc::c_char;

    fn sh_notbuiltin(_: *mut libc::c_char);
    fn find_shell_builtin(_: *mut libc::c_char) -> Option::<sh_builtin_func_t>;
    static mut this_shell_builtin: Option::<sh_builtin_func_t>;
    static mut loptend: *mut WordList;
}

pub type sh_builtin_func_t = unsafe extern "C" fn(*mut WordList) -> i32;
