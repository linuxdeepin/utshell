/* suspend.c, created from suspend.def. */
#line 22 "./suspend.def"

#line 38 "./suspend.def"

#include <config.h>

#if defined (JOB_CONTROL)
#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include "../bashtypes.h"
#include <signal.h>
#include "../bashintl.h"
#include "../shell.h"
#include "../jobs.h"
#include "common.h"
#include "bashgetopt.h"

static sighandler suspend_continue PARAMS((int));

static SigHandler *old_cont;
#if 0
static SigHandler *old_stop;
#endif

/* Continue handler. */
static sighandler
suspend_continue (sig)
     int sig;
{
  set_signal_handler (SIGCONT, old_cont);
#if 0
  set_signal_handler (SIGSTOP, old_stop);
#endif
  SIGRETURN (0);
}

/* Suspending the shell.  If -f is the arg, then do the suspend
   no matter what.  Otherwise, complain if a login shell. */

extern int r_suspend_builtin(WORD_LIST *);

int
suspend_builtin (list)
     WORD_LIST *list;
{
	r_suspend_builtin(list);
}

#endif /* JOB_CONTROL */
