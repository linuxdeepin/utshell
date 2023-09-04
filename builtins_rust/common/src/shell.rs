pub static EX_BADUSAGE:i32     = 2;

pub static EX_MISCERROR:i32 = 2;

pub static EX_RETRYFAIL:i32 = 124;
pub static EX_WEXPCOMSUB:i32 = 125;
pub static EX_BINARY_FILE :i32 = 126;
pub static EX_NOEXEC: i32 = 126;
pub static EX_NOINPUT:i32 = 126;
pub static EX_NOTFOUND:i32 = 127;

pub static EX_SHERRBASE:i32 = 256;     /* all special error values are > this. */

pub static EX_BADSYNTAX:i32 = 257;     /* shell syntax error */
pub static EX_REDIRFAIL:i32 =  259;    /* redirection failed */
pub static EX_BADASSIGN        :i32 = 260;     /* variable assignment error */
pub static EX_EXPFAIL:i32=     261;/* word expansion failed */
pub static EX_DISKFALLBACK:i32 = 262;  /* fall back to disk command from builtin */

