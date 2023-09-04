/* bind.c, created from bind.def. */
#line 22 "./bind.def"

#include <config.h>

#line 63 "./bind.def"

#if defined (READLINE)

#if defined (HAVE_UNISTD_H)
#  ifdef _MINIX
#    include <sys/types.h>
#  endif
#  include <unistd.h>
#endif

#include <stdio.h>
#include <errno.h>
#if !defined (errno)
extern int errno;
#endif /* !errno */

#include <readline/readline.h>
#include <readline/history.h>

#include "../bashintl.h"

#include "../shell.h"
#include "../bashline.h"
#include "bashgetopt.h"
#include "common.h"

static int query_bindings PARAMS((char *));
static int unbind_command PARAMS((char *));
static int unbind_keyseq PARAMS((char *));

#define BIND_RETURN(x)  do { return_code = x; goto bind_exit; } while (0)

#define LFLAG	0x0001
#define PFLAG	0x0002
#define FFLAG	0x0004
#define VFLAG	0x0008
#define QFLAG	0x0010
#define MFLAG	0x0020
#define RFLAG	0x0040
#define PPFLAG	0x0080
#define VVFLAG	0x0100
#define SFLAG   0x0200
#define SSFLAG  0x0400
#define UFLAG	0x0800
#define XFLAG	0x1000
#define XXFLAG	0x2000

extern int r_bind_builtin(WORD_LIST *);
extern int r_unbind_builtin(WORD_LIST *);

int
bind_builtin (list)
     WORD_LIST *list;
{
	r_bind_builtin(list);
}

static int
query_bindings (name)
     char *name;
{
  rl_command_func_t *function;
  char **keyseqs;
  int j;

  function = rl_named_function (name);
  if (function == 0)
    {
      builtin_error (_("`%s': unknown function name"), name);
      return EXECUTION_FAILURE;
    }

  keyseqs = rl_invoking_keyseqs (function);

  if (!keyseqs)
    {
      printf (_("%s is not bound to any keys.\n"), name);
      return EXECUTION_FAILURE;
    }

  printf (_("%s can be invoked via "), name);
  for (j = 0; j < 5 && keyseqs[j]; j++)
    printf ("\"%s\"%s", keyseqs[j], keyseqs[j + 1] ? ", " : ".\n");
  if (keyseqs[j])
    printf ("...\n");
  strvec_dispose (keyseqs);
  return EXECUTION_SUCCESS;
}

static int
unbind_command (name)
     char *name;
{
//return r_unbind_builtin(name);
  rl_command_func_t *function;

  function = rl_named_function (name);
  if (function == 0)
    {
      builtin_error (_("`%s': unknown function name"), name);
      return EXECUTION_FAILURE;
    }

  rl_unbind_function_in_map (function, rl_get_keymap ());
  return EXECUTION_SUCCESS;
}

static int
unbind_keyseq (seq)
     char *seq;
{
  char *kseq;
  int kslen, type;
  rl_command_func_t *f;

  kseq = (char *)xmalloc ((2 * strlen (seq)) + 1);
  if (rl_translate_keyseq (seq, kseq, &kslen))
    {
      free (kseq);
      builtin_error (_("`%s': cannot unbind"), seq);
      return EXECUTION_FAILURE;
    }
  if ((f = rl_function_of_keyseq_len (kseq, kslen, (Keymap)0, &type)) == 0)
    {
      free (kseq);
      return (EXECUTION_SUCCESS);
    }
  if (type == ISKMAP)
    f = ((Keymap) f)[ANYOTHERKEY].function;

  /* I wish this didn't have to translate the key sequence again, but readline
     doesn't have a binding function that takes a translated key sequence as
     an argument. */
  if (rl_bind_keyseq (seq, (rl_command_func_t *)NULL) != 0)
    {
      free (kseq);
      builtin_error (_("`%s': cannot unbind"), seq);
      return (EXECUTION_FAILURE);
    }

  if (f == bash_execute_unix_command)
    unbind_unix_command (seq);

  free (kseq);
  return (EXECUTION_SUCCESS);
}
#endif /* READLINE */
