/* umask.c, created from umask.def. */
#line 22 "./umask.def"

#line 41 "./umask.def"

#include <config.h>

#include "../bashtypes.h"
#include "filecntl.h"
#if ! defined(_MINIX) && defined (HAVE_SYS_FILE_H)
#  include <sys/file.h>
#endif

#if defined (HAVE_UNISTD_H)
#include <unistd.h>
#endif

#include <stdio.h>
#include <chartypes.h>

#include "../bashintl.h"

#include "../shell.h"
#include "posixstat.h"
#include "common.h"
#include "bashgetopt.h"

/* **************************************************************** */
/*                                                                  */
/*                     UMASK Builtin and Helpers                    */
/*                                                                  */
/* **************************************************************** */

static void print_symbolic_umask PARAMS((mode_t));
static int symbolic_umask PARAMS((WORD_LIST *));

extern int  r_umask_builtin(WORD_LIST *);

/* Set or display the mask used by the system when creating files.  Flag
   of -S means display the umask in a symbolic mode. */
int
umask_builtin (list)
     WORD_LIST *list;
{
	r_umask_builtin(list);

}





/* Print the umask in a symbolic form.  In the output, a letter is
   printed if the corresponding bit is clear in the umask. */
static void
#if defined (__STDC__)
print_symbolic_umask (mode_t um)
#else
print_symbolic_umask (um)
     mode_t um;
#endif
{
}

int
parse_symbolic_mode (mode, initial_bits)
     char *mode;
     int initial_bits;
{
}

/* Set the umask from a symbolic mode string similar to that accepted
   by chmod.  If the -S argument is given, then print the umask in a
   symbolic form. */
static int
symbolic_umask (list)
     WORD_LIST *list;
{
}

