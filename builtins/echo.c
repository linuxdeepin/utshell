/* echo.c, created from echo.def. */
#line 22 "./echo.def"
#include <config.h>

#if defined (HAVE_UNISTD_H)
#  include <unistd.h>
#endif

#include "../bashansi.h"

#include <stdio.h>
#include "../shell.h"

#include "common.h"

#line 73 "./echo.def"

#line 88 "./echo.def"

#if defined (V9_ECHO)
#  define VALID_ECHO_OPTIONS "neE"
#else /* !V9_ECHO */
#  define VALID_ECHO_OPTIONS "n"
#endif /* !V9_ECHO */

/* System V machines already have a /bin/sh with a v9 behaviour.  We
   give Bash the identical behaviour for these machines so that the
   existing system shells won't barf.  Regrettably, the SUS v2 has
   standardized the Sys V echo behavior.  This variable is external
   so that we can have a `shopt' variable to control it at runtime. */
#if defined (DEFAULT_ECHO_TO_XPG) || defined (STRICT_POSIX)
int xpg_echo = 1;
#else
int xpg_echo = 0;
#endif /* DEFAULT_ECHO_TO_XPG */

/* Print the words in LIST to standard output.  If the first word is
   `-n', then don't print a trailing newline.  We also support the
   echo syntax from Version 9 Unix systems. */

extern int r_echo_builtin(WORD_LIST *);

int
echo_builtin (list)
     WORD_LIST *list;
{
	r_echo_builtin(list);
}
