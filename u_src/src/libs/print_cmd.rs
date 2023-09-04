use super::{ command_h};
use libc::{c_int,c_char,FILE};
use command_h::{COMMAND,REDIRECT};





pub static indentation:c_int = 0;
pub static indentation_amount:c_int = 4;


pub const PRINTED_COMMAND_INITIAL_SIZE:c_int = 64;
pub const PRINTED_COMMAND_CROW_SIZE:c_int = 128;

pub static mut the_printed_command: *mut c_char = std::ptr::null();
pub static mut the_printed_command_size:c_int = 0;
pub static mut command_string_index:c_int = 0;

pub static mut xtrace_fd:c_int = -1;
pub static mut xtrace_fp:*mut FILE = 0 as *mut FILE;


static mut inside_function_def:c_int = 0;
static mut skip_this_indent:c_int = 0;
static mut was_heredoc:c_int = 0;
static mut printing_connection:c_int = 0;
static mut deferred_heredocs:*mut REDIRECT = 0 as *mut REDIRECT;

static mut group_command_nesting:c_int = 0;

static mut indirection_string:c_int = 0;
static mut indirection_stringsiz:c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn print_command(command:*mut COMMAND)
{
        command_string_index = 0;
        println!("{}",make_command_string(command));
    
}

#[no_mangle]
pub unsafe extern "C" fn make_command_string(command:*mut COMMAND)
{
    command_string_index = 0;
    was_heredoc = 0;
    deferred_heredocs = 0;
    make_command_string_internal(command);
    return the_printed_command;
}

#[no_mangle]
unsafe fn make_command_string_internal(command:*mut COMMAND)
{
    let s:[c_char;3];

    if command == std::ptr::null{
        cprintf("");
    }
    else{
        if skip_this_indent != 0 {
            skip_this_indent = skip_this_indent - 1;
        }
        else{
            indent(indentation);
        } 
        
        if (*command).flags != 0 && CMD_TIME_PIPELINE != 0{
            cprintf("time ");
            if (*command).flags != 0 && CMD_TIME_POSIX != 0{
                cprintf("-p ");
            }
        }

        if (*command).flags != 0 && CMD_INVERT_RETURN != 0{
            cprintf("! ");
        }

        match (*command).type_ as libc::c_uint {
            cm_for => print_for_command((*command).value.For),
            cm_arith_for => print_arith_for_command((*command).value.ArithFor),
            cm_seletc => print_select_for_command((*command).value.Select),
            cm_case => print_case_for_command((*command).value.Case),
            cm_while => print_while_for_command((*command).value.While),
            cm_untile => print_untile_for_command((*command).value.While),
            cm_if => print_if_for_command((*command).value.If),
            cm_arith => print_airth_command((*(*command).value.Arith).exp),
            cm_cond => print_cond_command((*command).value.Cond),
            cm_simple => print_simple_command((*command).value.Simple),
            cm_connection => {
                skip_this_indent = skip_this_indent + 1;
                printing_connection = printing_connection + 1;
                make_command_string_internal((*(*command).value.Connection).first);

                match (*command).value.Connection.connector{
                    '&' | '|'=> {
                        let c:c_char = (*(*command).value.Connection).connector;
                        s[0] = ' ';
                        s[1] = c;
                        s[2] = '\0';

                        print_deferred_heredocs(s);

                        if c != '&' || (*(*command).value.Connection).second{
                            cprintf(" ");
                            skip_this_indent = skip_this_indent + 1;
                        }
                    }
                }
            }
        }
    }
}