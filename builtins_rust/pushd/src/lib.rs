extern crate  libc;
extern crate nix;

use libc::{c_char, c_long, c_void};
use std::{ffi::{CString,CStr}};

use rcommon::{WordList, WordDesc, EX_USAGE, EXECUTION_SUCCESS, EXECUTION_FAILURE, EX_NOTFOUND, EX_NOEXEC, SUBSHELL_PAREN,r_builtin_usage, r_savestring};
use rhelp::r_builtin_help;
#[repr(u8)]
enum command_type { cm_for, cm_case, cm_while, cm_if, cm_simple, cm_select,
    cm_connection, cm_function_def, cm_until, cm_group,
    cm_arith, cm_cond, cm_arith_for, cm_subshell, cm_coproc
}

#[repr(u8)]
#[derive(Copy,Clone)]
enum r_instruction {
    r_output_direction, r_input_direction, r_inputa_direction,
    r_appending_to, r_reading_until, r_reading_string,
    r_duplicating_input, r_duplicating_output, r_deblank_reading_until,
    r_close_this, r_err_and_out, r_input_output, r_output_force,
    r_duplicating_input_word, r_duplicating_output_word,
    r_move_input, r_move_output, r_move_input_word, r_move_output_word,
    r_append_err_and_out
}

#[repr(C)]
pub union REDIRECT {
  next:*mut REDIRECT,
  redirector:libc::c_int,
  rflags:libc::c_int,
  flags:libc::c_int,
  instruction:r_instruction,
  redirectee:libc::c_int,
  here_doc_eof:*mut c_char
}

/* FOR command. */
#[repr(C)]
pub struct for_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct PATTERN_LIST {
    next:* mut PATTERN_LIST,
    patterns:* mut WordList,
    action:*mut COMMAND,
    flags:libc::c_int
}

#[repr(C)]
pub struct case_com {
    flags:libc::c_int,
    line:libc::c_int,
    word:*mut WordDesc,
    clauses:*mut PATTERN_LIST
}

#[repr(C)]
pub struct while_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    action:*mut COMMAND
}

#[repr(C)]
pub struct if_com {
    flags:libc::c_int,
    test:*mut COMMAND,
    true_case:*mut COMMAND,
    false_case:*mut COMMAND
}

#[repr(C)]
pub struct connection {
    ignore:libc::c_int,
    first:*mut COMMAND,
    second:*mut COMMAND,
    connector:libc::c_int
}

#[repr(C)]
pub struct simple_com {
    flags:libc::c_int,
    line:libc::c_int,
    words:*mut WordList,
    redirects:*mut REDIRECT
}

#[repr(C)]
pub struct function_def {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    command:*mut COMMAND,
    source_file:*mut c_char
}

#[repr(C)]
pub struct group_com {
    ignore:libc::c_int,
    command:*mut COMMAND,
    source_file:*mut c_char
}

#[repr(C)]
pub struct select_com {
    flags:libc::c_int,
    line:libc::c_int,
    name:*mut WordDesc,
    map_list:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct arith_com {
    flags:libc::c_int,
    line:libc::c_int,
    exp:*mut WordList
}

#[repr(C)]
pub struct cond_com {
    flags:libc::c_int,
    line:libc::c_int,
    type_c:libc::c_int,
    exp:*mut WordList
}

#[repr(C)]
pub struct arith_for_com {
    flags:libc::c_int,
    line:libc::c_int,
    init:*mut WordList,
    test:*mut WordList,
    step:*mut WordList,
    action:*mut COMMAND
}

#[repr(C)]
pub struct subshell_com {
    flags:i32,
    line:i32,
    command:*mut COMMAND
}

#[repr(C)]
pub struct coproc_com {
    flags:i32,
    name:*mut c_char,
    command:*mut COMMAND
}

#[repr(C)]
pub union VALUE_COMMAND {
    For:*mut for_com,
    Case:*mut case_com,
    While:*mut while_com,
    If:*mut if_com,
    Connection:*mut connection,
    Simple:*mut simple_com,
    Function_def:*mut function_def,
    Group:*mut group_com,
    Select:*mut select_com,
    Arith:*mut arith_com,
    Cond:*mut cond_com,
    ArithFor:*mut arith_for_com,
    Subshell:*mut subshell_com,
    Coproc:*mut coproc_com
}

#[repr(C)]
pub struct COMMAND {
    type_c:command_type,
    flags:i32,
    line:i32,
    redirects:*mut REDIRECT,
    value:VALUE_COMMAND
}


#[macro_export]
macro_rules! NOCD {
  () => {
    0x01
  }
}

#[macro_export]
macro_rules! ROTATE {
  () => {
    0x02
  }
}

#[macro_export]
macro_rules! LONGFORM {
  () => {
    0x04
  }
}

#[macro_export]
macro_rules! CLEARSTAK {
  () => {
    0x08
  }
}

extern "C" {
  fn builtin_error(err:*const c_char,...);
  fn get_working_directory (path:* mut c_char)-> * mut c_char;
  fn sh_invalidopt (value:* mut c_char);
  fn builtin_usage();
  fn sh_invalidnum (value:* mut c_char);
  fn legal_number (str1:* const c_char, num:* mut libc::c_long)->i32;
  fn cd_builtin (list:*mut WordList)->i32;
  fn polite_directory_format (path:* mut c_char)->* mut c_char;
  fn sh_erange (str1:* mut c_char, str2:* mut c_char);
  fn make_word_list (w: * mut WordDesc , l: * mut WordList)->* mut WordList;
  fn make_word (w:*const c_char)->* mut WordDesc;
  fn dispose_words (l: * mut WordList);
  fn strvec_resize (c:* mut * mut c_char, s:i32)->* mut * mut c_char;
  fn get_string_value (w:*const c_char)-> * mut c_char;
  fn sh_chkwrite (i:i32)->i32;
}
pub static  mut pushd_directory_list:* mut * mut c_char=std::ptr::null_mut();
pub static  mut directory_list_offset:i32=0;
pub static  mut directory_list_size:i32=0;

unsafe fn STREQ( a:* const c_char, b:* const c_char)->bool
{
	return *a ==*b  && libc::strcmp(a, b) == 0;
}

unsafe fn ISHELP(s:* const c_char)->bool
{
	return STREQ (s,  CString::new("--help").unwrap().as_ptr());
}

unsafe fn ISOPTION(s:* const c_char, c:c_char)->bool
{
	return *s == '-' as c_char && *(s.offset(1)) == c && *(s.offset(2)) == 0;
}

#[no_mangle]
pub extern "C" fn r_pushd_builtin (listt:* mut WordList)->i32
{
	let orig_list:* mut WordList;
	let mut temp:* mut c_char;
	let current_directory:* mut c_char;
	let mut top:* mut c_char;
	let mut j:i32;
	let mut flags:i32;
	let skipopt:i32;
	let mut num:libc::c_long=0;
	let mut direction:c_char;

	unsafe {
  let mut list:* mut WordList=listt.clone();
	orig_list = list.clone();

	if list != std::ptr::null_mut() &&  (*list).word != std::ptr::null_mut() && ISHELP((*((*list).word)).word) {
		r_builtin_help ();
		return EX_USAGE;
	}

	if list != std::ptr::null_mut() &&  (*list).word != std::ptr::null_mut() && ISOPTION ((*((*list).word)).word, '-' as c_char) {
		list = (*list).next;
		skipopt = 1;
  } else {
    skipopt = 0;
  }

	/* If there is no argument list then switch current and
	   top of list. */
	if list == std::ptr::null_mut() {
		if directory_list_offset == 0 {
		  builtin_error (CString::new("no other directory").unwrap().as_ptr());
		  return EXECUTION_FAILURE!();
	  }

		current_directory = get_working_directory (CString::new("pushd").unwrap().as_ptr() as * mut c_char);
		if current_directory == std::ptr::null_mut() {
      return EXECUTION_FAILURE!();
    }

		j = directory_list_offset - 1;
		temp = *((pushd_directory_list as usize + (j*8) as usize) as * mut *mut c_char);
		*((pushd_directory_list as usize + (j*8) as usize) as * mut *mut c_char) = current_directory;
		j = r_change_to_temp (temp);
		libc::free (temp as * mut c_void);
		return j;
	}

  flags = 0;

	while skipopt == 0 && list !=std::ptr::null_mut() {
		if ISOPTION ((*((*list).word)).word, 'n' as c_char) {
		    flags |= NOCD!();
	  }	else if ISOPTION ((*((*list).word)).word, '-' as c_char) {
		    list = (*list).next;
		    break;
	  }	else if *((*((*list).word)).word) == '-' as c_char && *(((*((*list).word)).word as usize +1) as * mut c_char) == '\0' as c_char {
        /* Let `pushd -' work like it used to. */
	      break;
    }	else {
      direction = *((*((*list).word)).word);
      if direction == '+' as c_char || direction == '-' as c_char {
        if legal_number (((*((*list).word)).word as usize +1) as * mut c_char, &mut num) == 0 {
          sh_invalidnum ((*((*list).word)).word);
          builtin_usage ();
          return EX_USAGE;
        }

        if direction == '-' as c_char {
          num = directory_list_offset as libc::c_long - num;
        }

        if num > directory_list_offset as libc::c_long || num < 0 {
          r_pushd_error (directory_list_offset, (*((*list).word)).word);
          return EXECUTION_FAILURE!();
        }
        flags |= ROTATE!();
      } else if *((*((*list).word)).word)== '-' as c_char {
          sh_invalidopt ((*((*list).word)).word);
          builtin_usage ();
          return EX_USAGE;
      } else {
          break;
      }
    }
    list = (*list).next;
	}

	if (flags & ROTATE!()) != 0 {
		/* Rotate the stack num times.  Remember, the current
	   directory acts like it is part of the stack. */
		temp = get_working_directory (CString::new("pushd").unwrap().as_ptr() as * mut c_char);

		if num == 0 {
        if (flags & NOCD!()) == 0 {
          j=r_change_to_temp (temp);
        } else {
          j=EXECUTION_SUCCESS!();
        }

        libc::free (temp as * mut c_void);
        return j;
	  }

	  {
		  top = *((pushd_directory_list as usize + ((directory_list_offset - 1)*8) as usize ) as * mut * mut c_char);
      j = directory_list_offset - 2;

      while j > -1 {
        *((pushd_directory_list as usize + ((j +1)*8) as usize ) as * mut * mut c_char) =*((pushd_directory_list as usize + (j*8) as usize ) as * mut * mut c_char);
        j-=1;
      }

      *((pushd_directory_list as usize + ((j +1)*8) as usize ) as * mut * mut c_char)=temp;

		  temp = top;
		  num-=1;
	  }

		while num != 0 {
      top = *((pushd_directory_list as usize + ((directory_list_offset - 1)*8) as usize ) as * mut * mut c_char);
      j = directory_list_offset - 2;

      while j > -1 {
        *((pushd_directory_list as usize + ((j +1)*8) as usize ) as * mut * mut c_char) =*((pushd_directory_list as usize + (j*8) as usize ) as * mut * mut c_char);
        j-=1;
      }

      *((pushd_directory_list as usize + ((j +1)*8) as usize ) as * mut * mut c_char)=temp;

		  temp = top;
		  num-=1;
    }

    if (flags & NOCD!()) == 0 {
      j=r_change_to_temp (temp);
    } else {
      j=EXECUTION_SUCCESS!();
    }

    libc::free (temp as * mut c_void);
    return j;
	}

	if list == std::ptr::null_mut() {
    return EXECUTION_SUCCESS!();
  }

	/* Change to the directory in list->word->word.  Save the current
	   directory on the top of the stack. */
	current_directory = get_working_directory (CString::new("pushd").unwrap().as_ptr() as * mut c_char);
	if current_directory == std::ptr::null_mut() {
    return EXECUTION_FAILURE!();
  }

  if (flags & NOCD!()) == 0 {
    if skipopt !=0 {
      j=cd_builtin(orig_list);
    } else {
      j=cd_builtin(list);
    }
  } else {
    j=EXECUTION_SUCCESS!();
  }

	if j == EXECUTION_SUCCESS!() {
    if (flags & NOCD!()) !=0 {
      r_add_dirstack_element(r_savestring ((*((*list).word)).word));
    } else {
      r_add_dirstack_element(current_directory);
    }

		r_dirs_builtin (std::ptr::null_mut());
		if (flags & NOCD!()) != 0 {
      libc::free (current_directory as * mut c_void);
    }
		return EXECUTION_SUCCESS!();
	}	else {
		libc::free (current_directory as * mut c_void);
		return EXECUTION_FAILURE!();
	}
  }
}

/* Pop the directory stack, and then change to the new top of the stack.
   If LIST is non-null it should consist of a word +N or -N, which says
   what element to delete from the stack.  The default is the top one. */
#[no_mangle]
pub extern "C" fn r_popd_builtin (listt:* mut WordList)->i32 {
let mut i:i32;
let mut which:libc::c_long;
let mut flags:i32;
let mut direction:c_char;
let mut which_word:* mut c_char;

unsafe {
let mut list:* mut WordList=listt.clone();
if list != std::ptr::null_mut() &&  (*list).word != std::ptr::null_mut() && ISHELP((*((*list).word)).word) { 
  r_builtin_help ();
  return EX_USAGE;
}

which_word = std::ptr::null_mut();
flags = 0;
which = 0;
direction = '+' as c_char;
while list != std::ptr::null_mut() {
    if ISOPTION ((*((*list).word)).word, 'n' as c_char){
      flags |= NOCD!();
    } else if ISOPTION ((*((*list).word)).word, '-' as c_char) {
      list = (*list).next;
      break;
    } else {
      direction = *((*((*list).word)).word);
      if direction == '+' as c_char || direction == '-' as c_char {

        if legal_number ((((*((*list).word)).word as usize + 1) as * mut c_char), & mut which) == 0 {
          sh_invalidnum ((*((*list).word)).word);
          builtin_usage ();
          return EX_USAGE;
        }
        which_word = (*((*list).word)).word;
      } else if *((*((*list).word)).word) == '-' as c_char {
        sh_invalidopt ((*((*list).word)).word);
        builtin_usage ();
        return EX_USAGE;
      } else if (*((*list).word)).word != std::ptr::null_mut() {
        builtin_error (CString::new("%s: invalid argument").unwrap().as_ptr() as * mut c_char, (*((*list).word)).word);
        builtin_usage ();
        return EX_USAGE;
    } else {
      break;
    }
   }
   list = (*list).next;
}

if which > directory_list_offset as libc::c_long || (which < -directory_list_offset as libc::c_long) || (directory_list_offset == 0 && which == 0) {
    if which_word !=std::ptr::null_mut() {
      r_pushd_error (directory_list_offset, which_word);
    } else {
      r_pushd_error (directory_list_offset, CString::new("").unwrap().as_ptr() as * mut c_char);
    }
    return EXECUTION_FAILURE!();
}

/* Handle case of no specification, or top of stack specification. */
if (direction == '+' as c_char && which == 0) ||
    (direction == '-' as c_char && which == directory_list_offset as libc::c_long) {
      if (flags & NOCD!()) == 0 {
        i=r_cd_to_string (*((pushd_directory_list as usize + ((directory_list_offset - 1)*8) as usize) as *mut *mut c_char));
      } else {
        i=EXECUTION_SUCCESS!();
      }

      if i != EXECUTION_SUCCESS!() {
        return i;
      }

      directory_list_offset-=1;

      libc::free ((*((pushd_directory_list as usize + (directory_list_offset *8) as usize) as *mut *mut c_char)) as * mut c_void);
  } else {
        /* Since an offset other than the top directory was specified,
    remove that directory from the list and shift the remainder
    of the list into place. */
    if direction == '+' as c_char{
      i= directory_list_offset - which as i32;
    } else {
      i=which as i32;
    }

    if i < 0 || i > directory_list_offset {
      if which_word !=std::ptr::null_mut() {
        r_pushd_error (directory_list_offset, which_word);
      } else {
        r_pushd_error (directory_list_offset, CString::new("").unwrap().as_ptr() as * mut c_char);
      }

      return EXECUTION_FAILURE!();
    }
    libc::free ((*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)) as * mut c_void);
    directory_list_offset-=1;

    /* Shift the remainder of the list into place. */
    while i < directory_list_offset {
      *((pushd_directory_list as usize +(i*8) as usize) as * mut * mut c_char)=*((pushd_directory_list as usize +((i+1)*8) as usize) as * mut * mut c_char);
      i+=1;
    }
  }

  r_dirs_builtin (std::ptr::null_mut());
  return EXECUTION_SUCCESS!();
}
}

/* Print the current list of directories on the directory stack. */
#[no_mangle]
pub extern "C" fn r_dirs_builtin (listt:* mut WordList)->i32
{
  let mut flags:i32=0;
  let mut desired_index:i32=-1;
  let mut index_flag:i32=0;
  let mut vflag:i32=0;
  let mut i:libc::c_long=0;
  let mut temp:*mut c_char;
  let mut w:* mut c_char=CString::new("").unwrap().as_ptr() as * mut c_char;

  unsafe {
    let mut list:* mut WordList=listt.clone();
    if list != std::ptr::null_mut() &&  (*list).word != std::ptr::null_mut() && ISHELP((*((*list).word)).word) {
      r_builtin_help ();
      return EX_USAGE;
    }

    while  list != std::ptr::null_mut() {
      if ISOPTION ((*((*list).word)).word, 'l' as c_char) {
	      flags |= LONGFORM!();
	    } else if ISOPTION ((*((*list).word)).word, 'c' as c_char) {
	      flags |= CLEARSTAK!();
	    } else if ISOPTION ((*((*list).word)).word, 'v' as c_char) {
	      vflag |= 2;
	    } else if ISOPTION ((*((*list).word)).word, 'p' as c_char) {
	      vflag |= 1;
	    } else if ISOPTION ((*((*list).word)).word, '-' as c_char) {
	      list = (*list).next;
	      break;
	    } else if *((*((*list).word)).word) == '+' as c_char || *((*((*list).word)).word) == '-' as c_char {
	      let sign:i32;
	      w = (*(*list).word).word.offset(1);
        if legal_number (w, &mut i) == 0 {
          sh_invalidnum ((*((*list).word)).word);
          builtin_usage ();
          return EX_USAGE;
        }

        if *((*(*list).word).word) == '+' as c_char{  
          sign = 1;
        } else {
          sign = -1;
        }

	      desired_index = r_get_dirstack_index (i, sign, &mut index_flag);
	    } else	{
        sh_invalidopt ((*((*list).word)).word);
        builtin_usage ();
        return EX_USAGE;
	    }
      list=(*list).next
    }

    if (flags & CLEARSTAK!()) !=0 {
      r_clear_directory_stack ();
      return EXECUTION_SUCCESS!();
    }

    if index_flag !=0 && (desired_index < 0 || desired_index > directory_list_offset) {
      r_pushd_error (directory_list_offset, w);
      return EXECUTION_FAILURE!();
    }

    /* The first directory printed is always the current working directory. */
    if index_flag == 0 || (index_flag == 1 && desired_index == 0) {
      temp = get_working_directory (CString::new("dirs").unwrap().as_ptr() as * mut c_char);
      if temp == std::ptr::null_mut() {
        temp = r_savestring (CString::new("<no current directory>").unwrap().as_ptr() as * mut c_char);
      }

      if (vflag & 2) !=0 {
        if (flags & LONGFORM!()) !=0 {
          libc::printf (CString::new("%2d  %s").unwrap().as_ptr(), 0, temp);
        } else {
          libc::printf (CString::new("%2d  %s").unwrap().as_ptr(), 0, polite_directory_format (temp));
        }
      } else {
        if (flags & LONGFORM!()) !=0 {
          libc::printf (CString::new("%s").unwrap().as_ptr(), temp);
        } else {
          libc::printf (CString::new("%s").unwrap().as_ptr(), polite_directory_format (temp));
        }
      }

      libc::free (temp as * mut c_void);
      if index_flag !=0 {
        libc::putchar ('\n' as libc::c_int);
        return sh_chkwrite (EXECUTION_SUCCESS!());
      }
    }

  /* Now print the requested directory stack entries. */
  if index_flag !=0 {
      if (vflag & 2) !=0 {
        if (flags & LONGFORM!()) !=0 {
          libc::printf (CString::new("%2d  %s").unwrap().as_ptr(), directory_list_offset - desired_index,
          *((pushd_directory_list as usize + (desired_index*8) as usize) as * mut * mut c_char) );
        } else {
          libc::printf (CString::new("%2d  %s").unwrap().as_ptr(), directory_list_offset - desired_index,
          polite_directory_format (*((pushd_directory_list as usize + (desired_index*8) as usize) as * mut * mut c_char)));
        }
      } else {
        if (flags & LONGFORM!()) !=0 {
          libc::printf (CString::new("%s").unwrap().as_ptr(), *((pushd_directory_list as usize + (desired_index*8) as usize) as * mut * mut c_char) );
        } else {
          libc::printf (CString::new("%s").unwrap().as_ptr(), polite_directory_format (*((pushd_directory_list as usize + (desired_index*8) as usize) as * mut * mut c_char)));
        }
      }
  } else {
    i = (directory_list_offset - 1) as libc::c_long;
    while i >= 0 {
      if vflag >= 2 {
        if (flags & LONGFORM!()) !=0 {
          libc::printf (CString::new("\n%2d  %s").unwrap().as_ptr(), directory_list_offset - i as i32 ,*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char) );
        } else {
          libc::printf (CString::new("\n%2d  %s").unwrap().as_ptr(),directory_list_offset - i as i32 ,polite_directory_format (*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)));
        }
      } else {
        if (flags & LONGFORM!()) !=0 {
          if (vflag & 1) !=0 {
            libc::printf (CString::new("%s%s").unwrap().as_ptr(),CString::new("\n").unwrap().as_ptr() as * mut c_char,*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char) );
          } else {
            libc::printf (CString::new("%s%s").unwrap().as_ptr() , CString::new(" ").unwrap().as_ptr() as * mut c_char,*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char) );
          }

        } else {
          if (vflag & 1) !=0 {
            libc::printf (CString::new("%s%s").unwrap().as_ptr(), CString::new("\n").unwrap().as_ptr() as * mut c_char,polite_directory_format (*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)));
          } else {
            libc::printf (CString::new("%s%s").unwrap().as_ptr() , CString::new(" ").unwrap().as_ptr() as * mut c_char,polite_directory_format (*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)));
          }
        }

      }
      i-=1;
    }
  }

  libc::putchar ('\n' as libc::c_int);
  return sh_chkwrite (EXECUTION_SUCCESS!());
  }
}

#[no_mangle]
pub extern "C" fn r_pushd_error (offset:i32, arg:* mut c_char)
{
  unsafe {
    if offset == 0 {
      builtin_error (CString::new("directory stack empty").unwrap().as_ptr());
    } else{
      sh_erange (arg, CString::new("directory stack index").unwrap().as_ptr() as * mut c_char);
    }
  }
}

#[no_mangle]
pub extern "C" fn r_clear_directory_stack ()
{
  let mut i:i32=0;
  unsafe {
    while  i < directory_list_offset {
      libc::free (*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char) as * mut c_void);
      i+=1;
    }

    directory_list_offset = 0;
  }
}

/* Switch to the directory in NAME.  This uses the cd_builtin to do the work,
   so if the result is EXECUTION_FAILURE then an error message has already
   been printed. */
#[no_mangle]
pub extern "C" fn r_cd_to_string (name:* mut c_char)->i32
{
  unsafe {
    let tlist:* mut WordList;
    let dir:* mut WordList;
    let result:i32;

    dir = make_word_list (make_word (name), std::ptr::null_mut());
    tlist = make_word_list (make_word (CString::new("--").unwrap().as_ptr()), dir);
    result = cd_builtin (tlist);
    dispose_words (tlist);
    return result;
  }
}

#[no_mangle]
pub extern "C" fn r_change_to_temp (temp: * mut c_char)->i32
{
  let tt:i32;

  if temp !=std::ptr::null_mut() {
    tt = r_cd_to_string (temp);
  } else {
    tt= EXECUTION_FAILURE!();
  }

  if tt == EXECUTION_SUCCESS!() {
    r_dirs_builtin (std::ptr::null_mut());
  }

  return tt;
}

#[no_mangle]
pub extern "C" fn r_add_dirstack_element (dir:* mut c_char)
{
  unsafe {
    if directory_list_offset == directory_list_size {
      directory_list_size += 10;
      pushd_directory_list = strvec_resize (pushd_directory_list, directory_list_size);
    }

    *((pushd_directory_list as usize +  (directory_list_offset*8) as usize ) as * mut * mut c_char) = dir;
    directory_list_offset+=1;
  }
}

#[no_mangle]
pub extern "C" fn r_get_dirstack_index (ind:libc::c_long, sign:i32, indexp:* mut i32)->i32
{
  unsafe {
    if indexp !=std::ptr::null_mut(){
      if sign > 0 {
        *indexp=1;
      } else {
        *indexp=2;
      }
    }
    /* dirs +0 prints the current working directory. */
    /* dirs -0 prints last element in directory stack */
    if ind == 0 && sign > 0 {
      return 0;
    } else if ind == directory_list_offset as libc::c_long {
        if indexp !=std::ptr::null_mut() {
          if sign > 0 {
            *indexp = 2;
          } else {
            *indexp = 1;
          }
        }
        return 0;
    } else if ind >= 0 && ind <= directory_list_offset as libc::c_long {
      if sign > 0 {
        return directory_list_offset - ind as i32;
      } else {
        return ind as i32;
      }
    } else {
      return -1;
    }
  }
}

/* Used by the tilde expansion code. */
#[no_mangle]
pub extern "C" fn r_get_dirstack_from_string (strt:* mut c_char)-> * mut c_char
{
  let ind:i32;
  let mut sign:i32;
  let mut index_flag:i32;
  let mut i:libc::c_long=0;

  sign = 1;
  let mut str1=strt.clone();
  unsafe {

    if *str1 == '-' as c_char || *str1 == '+' as c_char {
        if *str1 == '-' as c_char {
          sign=-1;
        } else {
          sign=1;
        }
        str1=(str1 as usize + 1 ) as * mut c_char;
    }

    if legal_number (str1, &mut i) == 0 {
      return std::ptr::null_mut();
    }

    index_flag = 0;
    ind = r_get_dirstack_index (i, sign, &mut index_flag);
    if index_flag !=0 && (ind < 0 || ind > directory_list_offset) {
      return std::ptr::null_mut();
    }

    if index_flag == 0 || (index_flag == 1 && ind == 0) {
      return get_string_value (CString::new("PWD").unwrap().as_ptr());
    } else {
      return *((pushd_directory_list as usize + (ind*8) as usize) as * mut * mut c_char);
    }
  }
}

#[no_mangle]
pub extern "C" fn r_get_dirstack_element (ind:libc::c_long, sign:i32)-> * mut c_char
{
  let mut i:i32;
  unsafe {
    i = r_get_dirstack_index (ind, sign, std::ptr::null_mut());
    if i < 0 || i > directory_list_offset {
      return std::ptr::null_mut();
    } else {
      return *((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char);
    }
  }
}

#[no_mangle]
pub extern "C" fn r_set_dirstack_element (ind:libc::c_long, sign:i32, value:* mut c_char)
{
  let i:i32;
  unsafe {
    i = r_get_dirstack_index (ind, sign, std::ptr::null_mut());
    if ind == 0 || i < 0 || i > directory_list_offset {
      return;
    }
    libc::free ((*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)) as * mut c_void);
    *((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char) = r_savestring (value);
  }
}

#[no_mangle]
pub extern "C" fn r_get_directory_stack (flags:i32)->* mut WordList
{
  let mut i:i32;
  let mut ret:* mut WordList;
  let mut d:* mut c_char;
  let t:* mut c_char;
  unsafe {
  ret = std::ptr::null_mut();
  i = 0;
  while i < directory_list_offset {
      if (flags&1) !=0 {
        d=polite_directory_format (*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char));
      } else {
        d=*((pushd_directory_list as usize + (i*8) as usize) as * mut * mut c_char)
      }
      ret = make_word_list (make_word (d), ret);
      i+=1;
  }
  /* Now the current directory. */
  d = get_working_directory (CString::new("dirstack").unwrap().as_ptr() as * mut c_char);
  i = 0;	/* sentinel to decide whether or not to free d */
  if d == std::ptr::null_mut() {
    d = CString::new(".").unwrap().as_ptr() as * mut c_char;
  } else {
      if (flags&1) !=0 {
        t=polite_directory_format(d);
      } else {
          t=d;
      }
      /* polite_directory_format sometimes returns its argument unchanged.
	 If it does not, we can free d right away.  If it does, we need to
	 mark d to be deleted later. */
      if t != d	{
	      libc::free (d as * mut c_void);
	      d = t;
	    } else { /* t == d, so d is what we want */
	      i = 1;
      }
  }
  ret = make_word_list (make_word (d), ret);
  if i !=0 {
    libc::free (d as * mut c_void);
  }
  return ret;	/* was (REVERSE_LIST (ret, (WordList *)); */
  }
}

