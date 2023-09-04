/* kill.c, created from kill.def. */
#line 22 "./kill.def"

#line 46 "./kill.def"

#include <config.h>

#include <stdio.h>
#include <errno.h>
#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include "../bashansi.h"
#include "../bashintl.h"

#include <signal.h>

#include "../shell.h"
#include "../trap.h"
#include "../jobs.h"
#include "common.h"

/* Not all systems declare ERRNO in errno.h... and some systems #define it! */
#if !defined (errno)
extern int errno;
#endif /* !errno */

static void kill_error PARAMS((pid_t, int));

#if !defined (CONTINUE_AFTER_KILL_ERROR)
#  define CONTINUE_OR_FAIL return (EXECUTION_FAILURE)
#else
#  define CONTINUE_OR_FAIL goto continue_killing
#endif /* CONTINUE_AFTER_KILL_ERROR */

/* Here is the kill builtin.  We only have it so that people can type
   kill -KILL %1?  No, if you fill up the process table this way you
   can still kill some. */

extern int r_kill_builtin(WORD_LIST *);

int
kill_builtin (list)
     WORD_LIST *list;
{
	r_kill_builtin(list);
}

static void
kill_error (pid, e)
     pid_t pid;
     int e;
{
  char *x;

  x = strerror (e);
  if (x == 0)
    x = _("Unknown error");
  builtin_error ("(%ld) - %s", (long)pid, x);
}
