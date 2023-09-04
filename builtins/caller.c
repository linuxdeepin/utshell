/* caller.c, created from caller.def. */
#line 23 "./caller.def"

#line 41 "./caller.def"

#include <config.h>   
#include <stdio.h>
#include "chartypes.h"
#include "bashtypes.h"

#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include <errno.h>

#include "../bashintl.h"

#include "../shell.h"
#include "common.h"
#include "builtext.h"
#include "bashgetopt.h"

#ifdef LOADABLE_BUILTIN
#  include "builtins.h"
#endif

#if !defined (errno)
extern int errno;
#endif /* !errno */

extern int r_caller_builtin(WORD_LIST *);

int
caller_builtin (list)
     WORD_LIST *list;
{
	r_caller_builtin(list);
}

#ifdef LOADABLE_BUILTIN
static char *caller_doc[] = {
N_("Returns the context of the current subroutine call.\n\
    \n\
    Without EXPR, returns \"$line $filename\".  With EXPR, returns\n\
    \"$line $subroutine $filename\"; this extra information can be used to\n\
    provide a stack trace.\n\
    \n\
    The value of EXPR indicates how many call frames to go back before the\n\
    current one; the top frame is frame 0."),
  (char *)NULL
};

struct builtin caller_struct = {
	"caller",
	caller_builtin,
	BUILTIN_ENABLED,
	caller_doc,
	"caller [EXPR]",
	0
};

#endif /* LOADABLE_BUILTIN */
