/* exec.c, created from exec.def. */
#line 22 "./exec.def"

#line 43 "./exec.def"

#include <config.h>

#include "../bashtypes.h"
#include "posixstat.h"
#include <signal.h>
#include <errno.h>

#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include <stdio.h>

#include "../bashansi.h"
#include "../bashintl.h"

#include "../shell.h"
#include "../execute_cmd.h"
#include "../findcmd.h"
#if defined (JOB_CONTROL)
#  include "../jobs.h"
#endif
#include "../flags.h"
#include "../trap.h"
#if defined (HISTORY)
#  include "../bashhist.h"
#endif
#include "common.h"
#include "bashgetopt.h"
#include "input.h"

/* Not all systems declare ERRNO in errno.h... and some systems #define it! */
#if !defined (errno)
extern int errno;
#endif /* !errno */

extern REDIRECT *redirection_undo_list;
extern char *exec_argv0;

int no_exit_on_failed_exec;

/* If the user wants this to look like a login shell, then
   prepend a `-' onto NAME and return the new name. */

extern int r_exec_builtin(WORD_LIST *);

int
exec_builtin (list)
     WORD_LIST *list;
{
	r_exec_builtin(list);
}
