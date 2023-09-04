use rcommon::WordList;
use ralias::{r_alias_builtin,r_unalias_builtin};
use rbind::r_bind_builtin;
use rbreak::{r_break_builtin,r_continue_builtin};
use rbuiltin::r_builtin_builtin;
use rcaller::r_caller_builtin;
use rcd::{r_cd_builtin,r_pwd_builtin};
//use rcmd::r_cmd_builtin;
use rcolon::{r_colon_builtin,r_false_builtin};
use command::r_command_builtin;
//use rcommon ::r__builtin;
use rcomplete::{r_complete_builtin,r_compgen_builtin,r_compopt_builtin};
use rdeclare::{r_declare_builtin,r_local_builtin};
use recho::r_echo_builtin;
use renable::r_enable_builtin;
use reval::r_eval_builtin;
use rexec::r_exec_builtin;
use rexit::{r_exit_builtin,r_logout_builtin};
use rfc::r_fc_builtin;
use rfg_bg::{r_fg_builtin,r_bg_builtin};
use rgetopts::r_getopts_builtin;
use rhash::r_hash_builtin;
use rhelp::r_help_builtin;
use rhistory::r_history_builtin;
use rjobs::r_jobs_builtin;
use rjobs::r_disown_builtin;
use rkill::r_kill_builtin;
use rmapfile::r_mapfile_builtin;
use rprintf::r_printf_builtin;
use rpushd::{r_pushd_builtin,r_dirs_builtin,r_popd_builtin};
use rread::r_read_builtin;
use rlet::r_let_builtin;
use rreturn::r_return_builtin;
use rset::{r_set_builtin,r_unset_builtin};
use rsetattr::{r_export_builtin,r_readonly_builtin};
use rshift::r_shift_builtin;
use rshopt::r_shopt_builtin;
use rsource::r_source_builtin;
use rsuspend::r_suspend_builtin;
use rtest::r_test_builtin;
use rtimes::r_times_builtin;
use rtrap::r_trap_builtin;
use rtype::r_type_builtin;
use rulimit::r_ulimit_builtin;
use rumask::r_umask_builtin;
use rwait::r_wait_builtin;
use std::ffi::CStr;
use std::ffi::CString;
use libc::{strcmp};

enum CMDType {
    AliasCmd,
    UnAliasCmd,
    BindCmd,
    BreakCmd,
    ContinueCmd,
    BuiltinCmd,
    CallerCmd,
    CdCmd,
    PwdCmd,
    ColonCmd,
    FalseCmd,
    CommandCmd,
    CommonCmd,
    CompleteCmd,
    CompoptCmd,
    CompgenCmd,
    DeclareCmd,
    LocalCmd,
    EchoCmd,
    EnableCmd,
    EvalCmd,
    ExecCmd,
    ExitCmd,
    LogoutCmd,
    FcCmd,
    FgCmd,
    BgCmd,
    GetoptsCmd,
    HashCmd,
    HelpCmd,
    HistoryCmd,
    JobsCmd,
    KillCmd,
    LetCmd,
    MapfileCmd,
    PrintfCmd,
    PushdCmd,
    DirsCmd,
    PopdCmd,
    ReadCmd,
    ReservedCmd,
    ReturnCmd,
    SetattrCmd,
    ExportCmd,
    ReadonlyCmd,
    SetCmd,
    UnSetCmd,
    ShiftCmd,
    ShoptCmd,
    SourceCmd,
    SuspendCmd,
    TestCmd,
    TimesCmd,
    TrapCmd,
    TypeCmd,
    UlimitCmd,
    UmaskCmd,
    WaitCmd,
    DisownCmd
}

  struct AliasComand ;
  impl CommandExec for AliasComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_alias_builtin(list)
        }
    }
  }
  struct UnAliasComand ;
  impl CommandExec for UnAliasComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_unalias_builtin(list)
        }
    }
  }

  struct BindComand;
  impl CommandExec for BindComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_bind_builtin(list)
    }
}
  struct BreakComand;
  impl CommandExec for BreakComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_break_builtin(list)
    }
}
struct ContinueComand;
  impl CommandExec for ContinueComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_continue_builtin(list)
    }
}
  struct BuiltinComand;
  impl CommandExec for BuiltinComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_builtin_builtin(list)
    }
}
  struct CallerComand;
  impl CommandExec for CallerComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_caller_builtin(list)
    }
}
  struct CdComand;
  impl CommandExec for CdComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_cd_builtin(list)
    }
}

struct PwdComand;
impl CommandExec for PwdComand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_pwd_builtin(list)
  }
}
  struct ColonComand;
  impl CommandExec for ColonComand{
    fn  excute(&self,list :*mut WordList)-> i32{
        r_colon_builtin(list)
      // 0
    }
}
struct FalseComand;
impl CommandExec for FalseComand{
  fn  excute(&self,list :*mut WordList)-> i32{
      r_false_builtin(list)
    // 0
  }
}
  struct CommandComand;
  impl CommandExec for CommandComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_command_builtin(list)
        }

    }
}
  struct CommonComand;
  impl CommandExec for CommonComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        0
    }
}
  struct CompleteComand; 
  impl CommandExec for CompleteComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_complete_builtin(list)
    }
}


struct CompgenCommand; 
impl CommandExec for CompgenCommand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_compgen_builtin(list)
  }
}

struct CompoptCommand; 
impl CommandExec for CompoptCommand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_compopt_builtin(list)
  }
}
  struct DeclareComand;
  impl CommandExec for DeclareComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_declare_builtin(list)
    }
}
struct LocalComand;
  impl CommandExec for LocalComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_local_builtin(list)
    }
}

  struct EchoComand;
  impl CommandExec for EchoComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_echo_builtin(list)
    }
}
  struct EnableComand;
  impl CommandExec for EnableComand{
    fn  excute(&self,list : *mut WordList)-> i32{
      unsafe {
        r_enable_builtin(list)
      }  
    }
}
  struct EvalComand;
  impl CommandExec for EvalComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_eval_builtin(list)
    }
}
  struct ExecComand;
  impl CommandExec for ExecComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_exec_builtin(list)
    }
}
  struct ExitComand;
  impl CommandExec for ExitComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_exit_builtin(list)
    }
}

struct LogoutCommand;
impl CommandExec for LogoutCommand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_logout_builtin(list)
  }
}
  struct FcComand;
  impl CommandExec for FcComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_fc_builtin(list)
    }
}
  struct FgComand;
  impl CommandExec for FgComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_fg_builtin(list)
    }
}
struct BgComand;
impl CommandExec for BgComand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_bg_builtin(list)
  }
}
  struct GetoptsComand;
  impl CommandExec for GetoptsComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_getopts_builtin(list)
    }
}
  struct HashComand;
  impl CommandExec for HashComand{
    fn  excute(&self,list : *mut WordList)-> i32{
       r_hash_builtin(list)
    }
}
  struct HelpComand;
  impl CommandExec for HelpComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_help_builtin(list)
    }
}
  struct HistoryComand;
  impl CommandExec for HistoryComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_history_builtin(list)
    }
}
  struct JobsComand;
  impl CommandExec for JobsComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_jobs_builtin(list)
    }
}
  struct KillComand;
  impl CommandExec for KillComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_kill_builtin(list)
    }
}
  struct LetComand;
  impl CommandExec for LetComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_let_builtin(list)
    }
}
  struct MapfileComand;
  impl CommandExec for MapfileComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_mapfile_builtin(list)
    }
}
  struct PrintfComand;
  impl CommandExec for PrintfComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_printf_builtin(list)
    }
}
  struct PushdCommand;
  impl CommandExec for PushdCommand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_pushd_builtin(list)
    }
}

struct PopdComand;
impl CommandExec for PopdComand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_popd_builtin(list)
  }
}
struct DirsCommand;
impl CommandExec for DirsCommand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_dirs_builtin(list)
  }
}
  struct ReadComand;
  impl CommandExec for ReadComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_read_builtin(list)
    }
}
  struct ReservedComand;
  impl CommandExec for ReservedComand{
    fn  excute(&self,list : *mut WordList)-> i32{
       // r_reserve_builtin(list)
       0
    }
}
  struct ReturnComand;
  impl CommandExec for ReturnComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_return_builtin(list)
    }
}
  struct SetattrComand;
  impl CommandExec for SetattrComand{
    fn  excute(&self,list : *mut WordList)-> i32{
       //r_setattr_builtin(list);
       /*unkown enter which func */
       0
    }  

}
struct ExportComand;
impl CommandExec for ExportComand{
  fn  excute(&self,list : *mut WordList)-> i32{
     r_export_builtin(list)
  }  

}

struct ReadonlyComand;
impl CommandExec for ReadonlyComand{
  fn  excute(&self,list : *mut WordList)-> i32{
     r_readonly_builtin(list)
  }  

}

  struct SetComand;
  impl CommandExec for SetComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_set_builtin(list)
    }

}

struct UnSetComand;
impl CommandExec for UnSetComand{
  fn  excute(&self,list : *mut WordList)-> i32{
      r_unset_builtin(list)
  }

}
  struct ShiftComand;
  impl CommandExec for ShiftComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_shift_builtin(list)
    }
}    
  struct ShoptComand;
  impl CommandExec for ShoptComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
         r_shopt_builtin(list)
        } 
   }
}
  struct SourceComand;
  impl CommandExec for SourceComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_source_builtin(list)
    }
}
  struct SuspendComand;
  impl CommandExec for SuspendComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_suspend_builtin(list)
    }
}
  struct TestComand;
  impl CommandExec for TestComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_test_builtin(list)
    }
}
  struct TimesComand;
  impl CommandExec for TimesComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_times_builtin(list)
    }
}
  struct TrapComand;
  impl CommandExec for TrapComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_trap_builtin(list)
    }
}
  struct TypeComand;
  impl CommandExec for TypeComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_type_builtin(list)
        }
       
    }

}
  struct UlimitComand;
  impl CommandExec for UlimitComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_ulimit_builtin(list)
        }
    }

}
  struct UmaskComand;
  impl CommandExec for UmaskComand{
    fn  excute(&self,list : *mut WordList)-> i32{
        r_umask_builtin(list)
    }
}
  struct WaitComand;
  impl CommandExec for WaitComand{
     fn  excute(&self,list : *mut WordList)-> i32{
        r_wait_builtin(list)
    }
}

  struct DisownCommand ;
  impl CommandExec for DisownCommand{
    fn  excute(&self,list : *mut WordList)-> i32{
        unsafe {
            r_disown_builtin(list)
        }
    }
  }

// 定义接口
pub trait CommandExec {
    fn excute(&self,list : *mut WordList) -> i32;
}


// 工厂模式
trait Factory {
    fn make_product(&self, product_type : CMDType) ->Box<dyn CommandExec>;
}

struct SimpleFactory;
impl SimpleFactory {
    fn new() -> Self {
        Self
    }
}

impl Factory for SimpleFactory {
    fn make_product(&self, cmd_type : CMDType) -> Box<dyn CommandExec> {
        match cmd_type {
            CMDType::AliasCmd => Box::new(
                AliasComand{}
            ) ,
            CMDType::UnAliasCmd => Box::new(
                UnAliasComand{}
            ) ,
            CMDType::BindCmd => Box::new(
                BindComand{}
            ),
            CMDType::BreakCmd => Box::new(
                BreakComand{}
            ) ,
            CMDType::ContinueCmd => Box::new(
                ContinueComand{}
            ) ,
            CMDType::BuiltinCmd  => Box::new(
                BuiltinComand{}
            ),
            CMDType::CallerCmd  => Box::new(
                CallerComand{}
            ),
            CMDType::CdCmd  => Box::new(
                CdComand{}
            ),
            CMDType::PwdCmd  => Box::new(
                PwdComand{}
            ),
            CMDType::ColonCmd  => Box::new(
                ColonComand{}
            ),
            CMDType::FalseCmd  => Box::new(
                FalseComand{}
            ),
            CMDType::CommandCmd => Box::new(
                CommandComand{}
            ),
            CMDType::CommonCmd => Box::new(
                CommonComand{}
            ),
            CMDType::CompleteCmd => Box::new(
                CompleteComand{}
            ),
            CMDType::CompoptCmd => Box::new(
                CompoptCommand{}
            ),
            CMDType::CompgenCmd => Box::new(
                CompgenCommand{}
            ),
            CMDType::DeclareCmd => Box::new(
                DeclareComand{}
            ),
            CMDType::LocalCmd => Box::new(
                LocalComand{}
            ),
            CMDType::EchoCmd => Box::new(
                EchoComand{}
            ),
            CMDType::EnableCmd => Box::new(
                EnableComand{}
            ),
            CMDType::EvalCmd => Box::new(
                EvalComand{}
            ),
            CMDType::ExecCmd => Box::new(
                ExecComand{}
            ),
            CMDType::ExitCmd => Box::new(
                ExitComand{}
            ),
            CMDType::LogoutCmd => Box::new(
                LogoutCommand{}
            ),
            CMDType::FcCmd  => Box::new(
                FcComand{}
            ),
            CMDType::FgCmd  => Box::new(
                FgComand{}
            ),
            CMDType::BgCmd  => Box::new(
                BgComand{}
            ),
            CMDType::GetoptsCmd  => Box::new(
                GetoptsComand{}
            ),
            CMDType::HashCmd  => Box::new(
                HashComand{}
            ),
            CMDType::HelpCmd => Box::new(
                HelpComand{}
            ),
            CMDType::HistoryCmd => Box::new(
                HistoryComand{}
            ),
            CMDType::JobsCmd => Box::new(
                JobsComand{}
            ),
            CMDType::KillCmd => Box::new(
                KillComand{}
            ),
            CMDType::LetCmd => Box::new(
                LetComand{}
            ),
            CMDType::MapfileCmd => Box::new(
                MapfileComand{}
            ),
            CMDType::PrintfCmd => Box::new(
                PrintfComand{}
            ),
            CMDType::PushdCmd => Box::new(
                PushdCommand{}
            ),
            CMDType::DirsCmd => Box::new(
                DirsCommand{}
            ),
            CMDType::PopdCmd => Box::new(
                PopdComand{}
            ),
            CMDType::ReadCmd => Box::new(
                ReadComand{}
            ),
            CMDType::ReservedCmd => Box::new(
                ReservedComand{}
            ),
            CMDType::ReturnCmd => Box::new(
                ReturnComand{}
            ),
            CMDType::SetattrCmd => Box::new(
                SetattrComand{}
            ),
            CMDType::ExportCmd => Box::new(
                ExportComand{}
            ),
            CMDType::ReadonlyCmd => Box::new(
                ReadonlyComand{}
            ),
            CMDType::SetCmd => Box::new(
                SetComand{}
            ),
            CMDType::UnSetCmd => Box::new(
                UnSetComand{}
            ),
            CMDType::ShiftCmd => Box::new(
                ShiftComand{}
            ),
            CMDType::ShoptCmd => Box::new(
                ShoptComand{}
            ),
            CMDType::SourceCmd => Box::new(
                SourceComand{}
            ),
            CMDType::SuspendCmd => Box::new(
                SuspendComand{}
            ),
            CMDType::TestCmd => Box::new(
                TestComand{}
            ),
            CMDType::TimesCmd => Box::new(
                TimesComand{}
            ),
            CMDType::TrapCmd  => Box::new(
                TrapComand{}
            ),
            CMDType::TypeCmd => Box::new(
                TypeComand{}
            ),
            CMDType::UlimitCmd => Box::new(
                UlimitComand{}
            ),
            CMDType::UmaskCmd => Box::new(
                UmaskComand{}
            ),
            CMDType::WaitCmd  => Box::new(
                WaitComand{}
            ),
            CMDType::DisownCmd => Box::new(
                DisownCommand{}
            )
        }
    }
}

unsafe fn get_cmd_type (command : *mut libc::c_char) -> CMDType{
    let mut types = CMDType::HelpCmd;
    if libc::strcmp(command, b"alias\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0{
        types = CMDType::AliasCmd;
    }
    if libc::strcmp(command, b"unalias\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0{
        types = CMDType::UnAliasCmd;
    }
    else if libc::strcmp(command, b"bind\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::BindCmd;
    }
    else if libc::strcmp(command, b"break\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::BreakCmd;
    }
    else if libc::strcmp(command, b"continue\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::ContinueCmd;
    }
    else if libc::strcmp(command, b"builtin\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::BuiltinCmd;
    }
    else if libc::strcmp(command, b"caller\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CallerCmd;
    }
    else if libc::strcmp(command, b"cd\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CdCmd;
    }
    else if libc::strcmp(command, b"pwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::PwdCmd;
    }
    else if libc::strcmp(command,  b":\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0
    || libc::strcmp(command,  b"true\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ColonCmd;
    }
    else if libc::strcmp(command,  b"false\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::FalseCmd;
    }
    else if libc::strcmp(command,  b"command\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CommandCmd;
    }
    else if libc::strcmp(command, b"common\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CommonCmd;
    }
    else if libc::strcmp(command, b"complete\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CompleteCmd;
    }
    else if libc::strcmp(command, b"compopt\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CompoptCmd;
    }
    else if libc::strcmp(command, b"compgen\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::CompgenCmd;
    }
    else if libc::strcmp(command,b"declare\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0 || libc::strcmp(command,b"typeset\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::DeclareCmd;
    }
    else if libc::strcmp(command,b"local\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::LocalCmd;
    }
    else if libc::strcmp(command,b"echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::EchoCmd;
    }
   
    else if libc::strcmp(command,b"enable\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::EnableCmd;
    }
    else if libc::strcmp(command,b"eval\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::EvalCmd;
    }

    else if libc::strcmp(command,b"exec\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ExecCmd;
    }
  
    else if libc::strcmp(command,b"exit\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ExitCmd;
    }
    else if libc::strcmp(command,b"logout\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::LogoutCmd;
    }
    else if libc::strcmp(command,b"fc\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::FcCmd;
    }
   
    else if libc::strcmp(command,b"fg\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::FgCmd;
    }
    else if libc::strcmp(command,b"bg\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::BgCmd;
    }
    
    else if libc::strcmp(command,b"getopts\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::GetoptsCmd;
    }

    else if libc::strcmp(command, b"hash\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::HashCmd;
    }
    else if libc::strcmp(command,b"help\0" as *const u8 as *const libc::c_char as * mut libc::c_char) == 0 {
        types = CMDType::HelpCmd;
    }
    else if libc::strcmp(command,b"history\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::HistoryCmd;
    }
    else if libc::strcmp(command,b"jobs\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::JobsCmd;
    }
    else if libc::strcmp(command, b"kill\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::KillCmd;
    }
    else if libc::strcmp(command, b"mapfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0 ||
    libc::strcmp(command, b"readarray\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::MapfileCmd;;
    }
    else if libc::strcmp(command,b"printf\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::PrintfCmd;
    }
    else if libc::strcmp(command,b"pushd\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::PushdCmd;
    }
    else if libc::strcmp(command,b"dirs\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::DirsCmd;
    }
    else if libc::strcmp(command,b"popd\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::PopdCmd;
    }
    else if libc::strcmp(command, b"read\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ReadCmd;
    }

    else if libc::strcmp(command, b"let\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::LetCmd;
    }
   
    else if libc::strcmp(command,b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::ReturnCmd;
    }
    else if libc::strcmp(command,b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::SetCmd;
    }
    else if libc::strcmp(command,b"unset\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::UnSetCmd;
    }
    else if libc::strcmp(command,b"setattr\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::SetattrCmd;
    }

    else if libc::strcmp(command,b"readonly\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ReadonlyCmd;
    }
    else if libc::strcmp(command,b"export\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ExportCmd;
    }
    else if libc::strcmp(command,b"shift\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ShiftCmd;
    }
 
    else if libc::strcmp(command,b"shopt\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::ShoptCmd;
    }
  
    else if libc::strcmp(command,b"source\0"  as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 
     || libc::strcmp(command,b".\0"  as *const u8 as *const libc::c_char as *mut libc::c_char)== 0  {
        types = CMDType::SourceCmd;
    }
 
    else if libc::strcmp(command, b"suspend\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::SuspendCmd;
    }

    else if libc::strcmp(command,b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0 
    || libc::strcmp(command,b"[\0" as *const u8 as *const libc::c_char as *mut libc::c_char)== 0 {
        types = CMDType::TestCmd;
    }
 
 
    else if libc::strcmp(command ,b"times\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::TimesCmd;
    }
    else if libc::strcmp(command ,b"trap\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::TrapCmd;
    }
    
    else if libc::strcmp(command ,b"type\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::TypeCmd;
    }

    else if libc::strcmp(command ,b"ulimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char)  == 0{
        types = CMDType::UlimitCmd;
    }
  
    else if libc::strcmp(command ,b"umask\0" as *const u8 as *const libc::c_char as *mut libc::c_char ) == 0{ 
        types = CMDType::UmaskCmd;
    }
    
    else if libc::strcmp(command , b"wait\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::WaitCmd;
    }
    else if libc::strcmp(command , b"disown\0" as *const u8 as *const libc::c_char as *mut libc::c_char) == 0 {
        types = CMDType::DisownCmd;
    }
    
   types
}

#[no_mangle]
pub extern "C" fn r_exec_cmd(command : *mut libc::c_char, mut list :*mut WordList) -> i32 {

    // println!("enter r_exec_cmd");
    // unsafe {
    //    println!("command is {:?}",CStr::from_ptr(command));
    // }
    let commandType = unsafe {get_cmd_type(command)};
    let  factory = SimpleFactory::new();
    let cmdCall = factory.make_product(commandType);
    cmdCall.excute(list)
}
