/* wait.c, created from wait.def. */
#line 51 "./wait.def"

#line 66 "./wait.def"

#include <config.h>

#include "../bashtypes.h"
#include <signal.h>

#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include <chartypes.h>

#include "../bashansi.h"

#include "../shell.h"
#include "../execute_cmd.h"
#include "../jobs.h"
#include "../trap.h"
#include "../sig.h"
#include "common.h"
#include "bashgetopt.h"

extern int wait_signal_received;

extern procenv_t wait_intr_buf;
extern int wait_intr_flag;

static int set_waitlist PARAMS((WORD_LIST *));
static void unset_waitlist PARAMS((void));

/* Wait for the pid in LIST to stop or die.  If no arguments are given, then
   wait for all of the active background processes of the shell and return
   0.  If a list of pids or job specs are given, return the exit status of
   the last one waited for. */

#define WAIT_RETURN(s) \
  do \
    { \
      wait_signal_received = 0; \
      wait_intr_flag = 0; \
      return (s);\
    } \
  while (0)

extern int r_wait_builtin(WORD_LIST *);

int
wait_builtin (list)
     WORD_LIST *list;
{
	r_wait_builtin(list);
}

#if defined (JOB_CONTROL)
/* Take each valid pid or jobspec in LIST and mark the corresponding job as
   J_WAITING, so wait -n knows which jobs to wait for. Return the number of
   jobs we found. */
static int
set_waitlist (list)
     WORD_LIST *list;
{
}

/* Clean up after a call to wait -n jobs */
static void
unset_waitlist ()
{
}
#endif
