use libc::{c_int,c_char,c_uint};



#[driver(Clone,Copy)]
#[repr(C)]
pub struct word_list{
    pub next: *mut world_list,
    pub word: *mut WORD_DESC,
}
pub type WORD_LIST = word_list;


#[driver(Clone,Copy)]
#[repr(C)]
pub struct word_desc{
    pub word: *mut c_char,
    pub flags: c_int,
}
pub type WORD_DESC = word_desc;


#[driver(Clone,Copy)]
#[repr(C)]
pub union REDIRECTEE{
    pub dest: c_int,
    pub filename: *mut WORD_DESC,
}

pub type r_instruction = c_uint;
pub const r_output_direction: r_instruction = 0;
pub const r_input_direction : r_instruction = 1;
pub const r_inputs_direction: r_instruction = 2;
pub const r_appending_to: r_instruction = 3;
pub const r_reading_until: r_instruction = 4;
pub const r_reading_string: r_instruction = 5;
pub const r_duplicating_input: r_instruction = 6;
pub const r_duplicating_output: r_instruction = 7;
pub const r_deblank_reading_until: r_instruction = 8;
pub const r_close_this: r_instruction = 9;
pub const r_err_and_out: r_instruction = 10;
pub const r_input_output: r_instruction = 11;
pub const r_output_force: r_instruction = 12;
pub const r_duplicating_input_word: r_instruction = 13;
pub const r_duplicating_output_word: r_instruction = 14;
pub const r_move_input: r_instruction = 15;
pub const r_move_output: r_instruction = 16;
pub const r_move_input_word: r_instruction = 17;
pub const r_move_output_word: r_instruction = 18;
pub const r_append_err_and_out: r_instruction = 19;


#[derive(Clone,Copy)]
#[repr(C)]
pub struct redirect{
    pub next: *mut redirect,
    pub redirector: REDIRECTEE,
    pub rflags: c_int,
    pub flags: c_int,
    pub instruction: r_instruction,
    pub redirectee: REDIRECTEE,
    pub here_doc_eof: *mut char,
}
pub type REDIRECT = redirect;

#[derive(Clone, Copy)]
#[repr (C)]
pub struct command {
    pub type:command_type;0000
    pub flags: c_int,
    pub line: c_int,
    pub redirects: *mut REDIRECT,
    pub value: command_union,
}
pub type COMMAND = command;
#[derive(Clone, Copy)]
#[repr (C)]
pub union command_union{
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
    pub ArithFor: *mut airth_for_com,
    pub Subshell: *mut subshell_com,
    pub Coproc: *mut coproc_com, 
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct for_com{
    pub falgs: c_int,
    pub line: c_int,
    pub name: *mut WORD_DESC,
    pub map_list: *mut WORD_DESC,
    pub action: *mut COMMAND,
}
pub type FOR_COM = for_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct case_com{
    pub flags: c_int,
    pub line: c_int,
    pub word: WORD_DESC,
    pub clauses: *mut PATTERN_LIST,
}
pub type CASE_COM = case_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct pattern_list{
    pub next: *mut pattern_list,
    pub patterns: *mut WORD_LIST,
    pub action: *mut COMMAND,
    pub flags: c_int,
} 
pub type PATTERN_LIST = pattern_list;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct while_com{
    pub flags:c_int,
    pub test:*mut COMMAND,
    pub action:*mut COMMAND,
}
pub type WHILE_COM = while_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct if_com{
    pub flags:c_int,
    pub test:*mut COMMAND,
    pub true_case:*mut COMMAND,
    pub false_case:*mut COMMAND,
}
pub type IF_COM = if_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection{
    pub ignore:c_int,
    pub first:*mut COMMAND,
    pub second:*mut COMMAND,
    pub connector: c_int,
}
pub type CONNECTION = connection;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct simple_com{
    pub flags:c_int,
    pub line: c_int,
    pub words: *mut WORD_LIST,
    pub redirects: *mut REDIRECT,
}
pub type SIMPLE_COM = simple_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct function_def{
    pub flags:c_int,
    pub line: c_int,
    pub name: *mut WORD_DESC,
    pub command: *mut COMMAND,
    pub source_file: *mut c_char,
}
pub type FUNCTION_DEF = function_def;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_com{
    pub ignore: c_int,
    pub command: *mut COMMAND,
}
pub type GROUP_COM = group_com;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct select_com{
    pub flags:c_int,
    pub line: c_int,
    pub name: *mut WORD_DESC,
    pub map_llist: *mut WORD_LIST,
    pub action: *mut COMMAND,
}
pub type SELECT_COM = select_com;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct arith_com{
    pub flags:c_int,
    pub line: c_int,
    pub exp: *mut WORD_LIST,
}
pub type ARITH_COM = arith_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cond_com{
    pub flags:c_int,
    pub line: c_int,
    pub type_0: c_int,
    pub op: *mut WORD_DESC,
    pub left: *mut cond_com,
    pub right: *mut cond_com, 
}
pub type COND_COM = cond_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct airth_for_com{
    pub flags:c_int,
    pub line: c_int,
    pub init: *mut WORD_LIST,
    pub test: *mut WORD_LIST,
    pub step: *mut WORD_LIST,
    pub action: *mut COMMAND,
}
pub type AIRTH_FOR_COM = airth_for_com;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct subshell_com{
    pub flags:c_int,
    pub line: c_int,
    pub command: *mut COMMAND,
}
pub type SUBSHELL_COM = subshell_com;


#[derive(Copy, Clone)]
#[repr(C)]
pub struct coproc_com{
    pub flags:c_int,
    pub name: *mut c_char,
    pub command: *mut COMMAND,
}
pub type COPROC_COM = coproc_com;



#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
