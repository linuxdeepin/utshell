helplongdoc ={
$cmdName -> 
[command0]  
      Resume job in foreground.  
       
           Equivalent to the JOB_SPEC argument to the 'fg' command.  Resume a  
           stopped or background job.  JOB_SPEC can specify either a job name  
           or a job number.  Following JOB_SPEC with a '&' places the job in  
           the background, as if the job specification had been supplied as an  
           argument to 'bg'.  
       
           Exit Status:  
           Returns the status of the resumed job. 

[command1]
      Evaluate conditional expression.  
       
          This is a synonym for the  test  builtin, but the last argument must  
          be a literal ']', to match the opening '['. 
[command2]
      Execute commands from a file in the current shell.  
       
          Read and execute commands from FILENAME in the current shell.  The  
          entries in $PATH are used to find the directory containing FILENAME.  
          If any ARGUMENTS are supplied, they become the positional parameters  
          when FILENAME is executed.  
       
          Exit Status:  
          Returns the exit status of SHELL-BUILTIN, or false if SHELL-BUILTIN is  
          not a shell builtin. 

[command3] 
    Null command.  
       
          No effect; the command does nothing.  
       
          Exit Status:  
          Always succeeds. 

[command4] 
    Evaluate conditional expression.  
       
          This is a synonym for the  test  builtin, but the last argument must  
          be a literal ']', to match the opening '['. 
          
[command5] 
      Execute conditional command.  
       
           Returns a status of 0 or 1 depending on the evaluation of the  
           conditional  
           expression EXPRESSION.  Expressions are composed of the same primaries  
           used  
           by the 'test' builtin, and may be combined using the following  
           operators:  
       
           ( EXPRESSION ) Returns the value of EXPRESSION  
           ! EXPRESSION  True if EXPRESSION is false; else false  
           EXPR1 && EXPR2 True if both EXPR1 and EXPR2 are true; else false  
           EXPR1 || EXPR2 True if either EXPR1 or EXPR2 is true; else false  
       
           When the '==' and '!=' operators are used, the string to the right of  
          the operator is used as a pattern and pattern matching is performed.  
          When the '=~' operator is used, the string to the right of the operator  
          is matched as a regular expression.  
       
          The && and || operators do not evaluate EXPR2 if EXPR1 is sufficient to

[command6] 
      Arithmetic for loop.  
       
           Equivalent to  
             (( EXP1 ))  
             while (( EXP2 )); do  
                 COMMANDS  
                (( EXP3 ))  
             done  
            EXP1, EXP2, and EXP3 are arithmetic expressions.  If any expression is  
            omitted, it behaves as if it evaluates to 1.  
       
            Exit Status:  
            Returns the status of the last command executed. 

[command7]  
      Group commands as a unit.  
       
            Run a set of commands in a group.  This is one way to redirect an  
            entire set of commands.  
       
            Exit Status:  
            Returns the status of the last command executed.

[set] 
    Set or unset values of shell options and positional parameters.
    
        Change the value of shell attributes and positional parameters, or
        display tmes and values of shell variables.
                
        Options:
        -a  Mark variables which are modified or created for export.
        -b  Notify of job termination immediately.
        -e  Exit immediately if a command exits with a non-zero status.
        -f  Disable file name generation (globbing).
        -h  Remember the location of commands as they are looked up.
        -k  All assignment arguments are placed in the environment for a
            command, not just those that precede the command name.
        -m  Job control is enabled.
        -n  Read commands but do not execute them.
        -o option-name          
        Set the variable corresponding to option-name:              
            allexport    same as -a              
            braceexpand  same as -B              
            emacs        use an emacs-style line editing interface              
            errexit      same as -e              
            errtrace     same as -E              
            functrace    same as -T              
            hashall      same as -h              
            histexpand   same as -H              
            history      enable command history              
            ignoreeof    the shell will not exit upon reading EOF              
            interactive-comments                           
                         allow comments to appear in interactive commands              
            keyword      same as -k              
            monitor      same as -m              
            noclobber    same as -C              
            noexec       same as -n              
            noglob       same as -f              
            nolog        currently accepted but ignored              
            notify       same as -b              
            nounset      same as -u              
            onecmd       same as -t              
            physical     same as -P              
            pipefail     the return value of a pipeline is the status of                           
                         the last command to exit with a non-zero status,                          
                         or zero if no command exited with a non-zero status              
            posix        change the behavior of bash where the default                           
            operation differs from the Posix standard to                           
            match the standard              
            privileged   same as -p              
            verbose      same as -v              
            vi           use a vi-style line editing interface              
            xtrace       same as -x      
        -p  Turned on whenever the real and effective user ids do not match.          
            Disables processing of the $ENV file and importing of shell          
            functions.  Turning this option off causes the effective uid and          
            gid to be set to the real uid and gid.      
        -t  Exit after reading and executing one command.      
        -u  Treat unset variables as an error when substituting.      
        -v  Print shell input lines as they are read.      
        -x  Print commands and their arguments as they are executed.      
        -B  the shell will perform brace expansion      
        -C  If set, disallow existing regular files to be overwritten          
            by redirection of output.      
        -E  If set, the ERR trap is inherited by shell functions.      
        -H  Enable ! style history substitution.  This flag is on          
            by default when the shell is interactive.      
        -P  If set, do not resolve symbolic links when executing commands          
            such as cd which change the current directory.      
        -T  If set, the DEBUG and RETURN traps are inherited by shell functions.    
        --  Assign any remaining arguments to the positional parameters.          
            If there are no remaining arguments, the positional parameters          
            are unset.      
        -   Assign any remaining arguments to the positional parameters.         
            The -x and -v options are turned off.   
            
        Using + rather than - causes these flags to be turned off.  
        The flags can also be used upon invocation of the shell.  
        The current    set of flags may be found in $-.  
        The remaining n ARGs are positional   
        parameters and are assigned, in order, to $1, $2, .. $n.  If no  
        ARGs are given, all shell variables are printed.  
            
        Exit Status:
        Returns success unless an invalid option is given.
[read] 
    Read a line from the standard input and split it into fields.
    
        Reads a single line from the standard input, or from file descriptor FD
        if the -u option is supplied.  The line is split into fields as with 
        word splitting, and the first word is assigned to the first NAME, the 
        second word to the second NAME, and so on, with any leftover words 
        assigned to the last NAME.  Only the characters found in $IFS are recognized
        as  word delimiters.
        
        If no NAMEs are supplied, the line read is stored in the REPLY 
        variable.
        
        Options:
          -a array  assign the words read to sequential indices of the array
            variable ARRAY, starting at zero
          -d delim  continue until the first character of DELIM is read, rather
            than newline
          -e  use Readline to obtain the line
          -i text  use TEXT as the initial text for Readline
          -n nchars  return after reading NCHARS characters rather than waiting
            for a newline, but honor a delimiter if fewer than
            NCHARS characters are read before the delimiter
          -N nchars  return only after reading exactly NCHARS characters, 
             unless
            EOF is encountered or read times out, ignoring any
            delimiter
          -p prompt  output the string PROMPT without a trailing newline before
            attempting to read
          -r  do not allow backslashes to escape any characters
          -s  do not echo input coming from a terminal
          -t timeout  time out and return failure if a complete line of
            input is not read within TIMEOUT seconds.  The value of the
            TMOUT variable is the default timeout.  TIMEOUT may be a
            fractional number.  If TIMEOUT is 0, read returns
            immediately, without trying to read any data, returning
            success only if input is available on the specified
            file descriptor.  The exit status is greater than 128
            if the timeout is exceeded
          -u fd  read from file descriptor FD instead of the standard input
        
        Exit Status:
        The return code is zero, unless end-of-file is encountered, read times 
        out (in which case it's greater than 128), a variable assignment error 
        occurs, or an invalid file descriptor is supplied as the argument to -u.
    
[type] 
    Display information about command type.
    
        For each NAME, indicate how it would be interpreted if used as a
        command name.
       
        Options:
          -a  display all locations containing an executable named NAME;
            includes aliases, builtins, and functions, if and only if
            the '-p' option is not also used
          -f  suppress shell function lookup
          -P  force a PATH search for each NAME, even if it is an alias,
            builtin, or function, and returns the name of the disk file
            that would be executed
          -p  returns either the name of the disk file that would be executed,
            or nothing if 'type -t NAME' would not return 'file'
          -t  output a single word which is one of 'alias', 'keyword',
            'function', 'builtin', 'file' or '', if NAME is an alias,
            shell reserved word, shell function, shell builtin, disk file,
            or not found, respectively
       
        Arguments:
          NAME  Command name to be interpreted.
       
        Exit Status:
        Returns success if all of the NAMEs are found; fails if any are not 
        found.

[trap] 
    Trap signals and other events.
    
        Defines and activates handlers to be run when the shell receives 
        signals or other conditions.
        
        ARG is a command to be read and executed when the shell receives the
        signal(s) SIGNAL_SPEC.  If ARG is absent (and a single SIGNAL_SPEC
        is supplied) or '-', each specified signal is reset to its original
        value.  If ARG is the null string each SIGNAL_SPEC is ignored by the
        shell and by the commands it invokes.
        
        If a SIGNAL_SPEC is EXIT (0) ARG is executed on exit from the shell.    
        If a SIGNAL_SPEC is DEBUG, ARG is executed before every simple command.  
        If a SIGNAL_SPEC is RETURN, ARG is executed each time a shell function or 
        a script run by the . or source builtins finishes executing.  A 
        SIGNAL_SPEC of ERR means to execute ARG each time a command's failure 
        would cause the shell to exit when the -e option is enabled.
        
        If no arguments are supplied, trap prints the list of commands 
        associated with each signal.
        
        Options:
          -l  print a list of signal names and their corresponding numbers
          -p  display the trap commands associated with each SIGNAL_SPEC
        
        Each SIGNAL_SPEC is either a signal name in <signal.h> or a signal 
        number.
        Signal names are case insensitive and the SIG prefix is optional.  A
        signal may be sent to the shell with \kill -signal $$\.
        
        Exit Status:
        Returns success unless a SIGSPEC is invalid or an invalid option is 
        given.

[alias] 
    Define or display aliases.
    
        Without arguments, 'alias' prints the list of aliases in the reusable
        form 'alias NAME=VALUE' on standard output.
        
        Otherwise, an alias is defined for each NAME whose VALUE is given.
        A trailing space in VALUE causes the next word to be checked for
        alias substitution when the alias is expanded.
        
        Options:
          -p  print all defined aliases in a reusable format
        
        Exit Status:
        alias returns true unless a NAME is supplied for which no alias has 
        been defined.
    
[unalias] 
    Remove each NAME from the list of defined aliases.
    
         Options:
           -a  remove all alias definitions
         
         Return success unless a NAME is not an existing alias.
         
[break] 
    Exit for, while, or until loops.
    
        Exit a FOR, WHILE or UNTIL loop.  If N is specified, break N enclosing
        loops.
        
        Exit Status:
        The exit status is 0 unless N is not greater than or equal to 1.
     
[continue] 
    Resume for, while, or until loops.
    
        Resumes the next iteration of the enclosing FOR, WHILE or UNTIL loop.
        If N is specified, resumes the Nth enclosing loop.
        
        Exit Status:
        The exit status is 0 unless N is not greater than or equal to 1.
        
[builtin] 
    Execute shell builtins.
    
        Execute SHELL-BUILTIN with arguments ARGs without performing command
        lookup.  This is useful when you wish to reimplement a shell builtin
        as a shell function, but need to execute the builtin within the 
        function.
        
        Exit Status:
        Returns the exit status of SHELL-BUILTIN, or false if SHELL-BUILTIN is
        not a shell builtin.
    
[caller] 
    Returns the context of the current subroutine call.
    
        Without EXPR, returns \$line $filename\.  With EXPR, returns
        \$line $subroutine $filename\; this extra information can be used to
        provide a stack trace.
        
        The value of EXPR indicates how many call frames to go back before the
        current one; the top frame is frame 0.
    
[cd] 
    Change the shell working directory.
    
        Change the current directory to DIR.  The default DIR is the value of 
        the HOME shell variable.
        
        The variable CDPATH defines the search path for the directory 
        containing DIR.  Alternative directory names in CDPATH are 
        separated by a colon (:).
        A null directory name is the same as the current directory.  If DIR 
        begins with a slash (/), then CDPATH is not used.
        
        If the directory is not found, and the shell option 'cdable_vars' is 
        set,the word is assumed to be  a variable name.  If that variable has a 
        value,its value is used for DIR.
        
        Options:
          -L  force symbolic links to be followed: resolve symbolic
            links in DIR after processing instances of '..'
          -P  use the physical directory structure without following
            symbolic links: resolve symbolic links in DIR before
            processing instances of '..'
          -e  if the -P option is supplied, and the current working
            directory cannot be determined successfully, exit with
            a non-zero status
          -@  on systems that support it, present a file with extended
            attributes as a directory containing the file attributes
        
        The default is to follow symbolic links, as if '-L' were specified.
        '..' is processed by removing the immediately previous pathname 
        component back to a slash or the beginning of DIR.
        
        Exit Status:
        Returns 0 if the directory is changed, and if $PWD is set successfully 
        when -P is used; non-zero otherwise.
    
[pwd] 
    Print the name of the current working directory.
    
        Options:
          -L  print the value of $PWD if it names the current working
            directory
          -P  print the physical directory, without any symbolic links
        
        By default, 'pwd' behaves as if '-L' were specified.
        
        Exit Status:
        Returns 0 unless an invalid option is given or the current directory
        cannot be read.
    
[true] 
    Null command.
    
        No effect; the command does nothing.
        
        Exit Status:
        Always succeeds.
        
[shopt] 
    Set and unset shell options.
    
        Change the setting of each shell option OPTNAME.  Without any option
        arguments, list each supplied OPTNAME, or all shell options if no
        OPTNAMEs are given, with an indication of whether or not each is set.
        
        Options:
          -o  restrict OPTNAMEs to those defined for use with 'set -o'
          -p  print each shell option with an indication of its status
          -q  suppress output
          -s  enable (set) each OPTNAME
          -u  disable (unset) each OPTNAME
        
        Exit Status:
        Returns success if OPTNAME is enabled; fails if an invalid option is
        given or OPTNAME is disabled.
    
[false] 
    Return an unsuccessful result.
    
        Exit Status:
        Always fails.
        
[command]  
    Execute a simple command or display information about commands.
    
        Runs COMMAND with ARGS suppressing  shell function lookup, or display
        information about the specified COMMANDs.  Can be used to invoke 
        commands on disk when a function with the same name exists.
        
        Options:
          -p    use a default value for PATH that is guaranteed to find all of
                the standard utilities
          -v    print a description of COMMAND similar to the 'type' builtin
          -V    print a more verbose description of each COMMAND
        
        Exit Status:
        Returns exit status of COMMAND, or failure if COMMAND is not found.

[echo] 
    Write arguments to the standard output.
    
        Display the ARGs, separated by a single space character and followed by 
        a newline, on the standard output.
        
        Options:
          -n  do not append a newline
          -e  enable interpretation of the following backslash escapes
          -E  explicitly suppress interpretation of backslash escapes
        
        'echo' interprets the following backslash-escaped characters:
          \\a  alert (bell)
          \\b  backspace
          \\c  suppress further output
          \\e  escape character
          \\E  escape character
          \\f  form feed
          \  new line
          \\r  carriage return
          \    horizontal tab
          \\v  vertical tab
          \\\\  backslash
          \\0nnn  the character whose ASCII code is NNN (octal).  NNN can be
            0 to 3 octal digits
          \\xHH  the eight-bit character whose value is HH (hexadecimal).  HH
            can be one or two hex digits
          \\uHHHH  the Unicode character whose value is the hexadecimal value 
        HHHH.
            HHHH can be one to four hex digits.
          \\UHHHHHHHH the Unicode character whose value is the hexadecimal 
            value 
            HHHHHHHH. HHHHHHHH can be one to eight hex digits.
        
        Exit Status:
        Returns success unless a write error occurs.

[enable] 
    Enable and disable shell builtins.
    
        Enables and disables builtin shell commands.  Disabling allows you to
        execute a disk command which has the same name as a shell builtin
        without using a full pathname.
        
        Options:
          -a  print a list of builtins showing whether or not each is enabled
          -n  disable each NAME or display a list of disabled builtins
          -p  print the list of builtins in a reusable format
          -s  print only the names of Posix 'special' builtins
        
        Options controlling dynamic loading:
          -f  Load builtin NAME from shared object FILENAME
          -d  Remove a builtin loaded with -f
        
        Without options, each NAME is enabled.
        
        To use the 'test' found in $PATH instead of the shell builtin
        version, type 'enable -n test'.
        
        Exit Status:
        Returns success unless NAME is not a shell builtin or an error occurs.
    
[getopts] 
    Parse option arguments.
    
        Getopts is used by shell procedures to parse positional parameters
        as options.
        
        OPTSTRING contains the option letters to be recognized; if a letter
        is followed by a colon, the option is expected to have an argument,
        which should be separated from it by white space.
        
        Each time it is invoked, getopts will place the next option in the
        shell variable $name, initializing name if it does not exist, and
        the index of the next argument to be processed into the shell
        variable OPTIND.  OPTIND is initialized to 1 each time the shell or
        a shell script is invoked.  When an option requires an argument,
        getopts places that argument into the shell variable OPTARG.
        
        getopts reports errors in one of two ways.  If the first character
        of OPTSTRING is a colon, getopts uses silent error reporting.  In
        this mode, no error messages are printed.  If an invalid option is
        seen, getopts places the option character found into OPTARG.  If a
        required argument is not found, getopts places a ':' into NAME and
        sets OPTARG to the option character found.  If getopts is not in
        silent mode, and an invalid option is seen, getopts places '?' into
        NAME and unsets OPTARG.  If a required argument is not found, a '?'
        is placed in NAME, OPTARG is unset, and a diagnostic message is
        printed.
        
        If the shell variable OPTERR has the value 0, getopts disables the
        printing of error messages, even if the first character of
        OPTSTRING is not a colon.  OPTERR has the value 1 by default.
        
        Getopts normally parses the positional parameters, but if arguments
        are supplied as ARG values, they are parsed instead.
        
        Exit Status:
        Returns success if an option is found; fails if the end of options is
        encountered or an error occurs.

[exec]  
    Replace the shell with the given command.
    
        Execute COMMAND, replacing this shell with the specified program.
        ARGUMENTS become the arguments to COMMAND.  If COMMAND is not 
        specified,
        any redirections take effect in the current shell.
        
         Options:
          -a name  pass NAME as the zeroth argument to COMMAND
          -c  execute COMMAND with an empty environment
          -l  place a dash in the zeroth argument to COMMAND
        
        If the command cannot be executed, a non-interactive shell exits, 
        unless
        the shell option 'execfail' is set.
        
        Exit Status:
        Returns success unless COMMAND is not found or a redirection error 
        occurs.
        
[exit] 
    Exit the shell.
    
        Exits the shell with a status of N.  If N is omitted, the exit status
        is that of the last command executed.
        
[logout] 
    Exit a login shell.
    
        Exits a login shell with exit status N.  Returns an error if not 
        executed in a login shell.
        
[fc] 
    Display or execute commands from the history list.
    
        fc is used to list or edit and re-execute commands from the history 
        list.
        FIRST and LAST can be numbers specifying the range, or FIRST can be a
        string, which means the most recent command beginning with that
        string.
        
        Options:
          -e ENAME  select which editor to use.  Default is FCEDIT, then 
        EDITOR,
            then vi
          -l   list lines instead of editing
          -n  omit line numbers when listing
          -r  reverse the order of the lines (newest listed first)
        
        With the 'fc -s [pat=rep ...] [command]' format, COMMAND is
        re-executed after the substitution OLD=NEW is performed.
        
        A useful alias to use with this is r='fc -s', so that typing 'r cc'
        runs the last command beginning with 'cc' and typing 'r' re-executes
        the last command.
        
        Exit Status:
        Returns success or status of executed command; non-zero if an error 
        occurs.

[fg] 
    Resume job in foreground.
    
        Equivalent to the JOB_SPEC argument to the 'fg' command.  Resume a
        stopped or background job.  JOB_SPEC can specify either a job name
        or a job number.  Following JOB_SPEC with a '&' places the job in
        the background, as if the job specification had been supplied as an
        argument to 'bg'.
        
        Exit Status:
        Returns the status of the resumed job.
        
[bg] 
    Move jobs to the background.
        
        Place the jobs identified by each JOB_SPEC in the background, as if 
        they had been started with '&'.  If JOB_SPEC is not present, the shell's 
        notion of the current job is used.
        
        Exit Status:
        Returns success unless job control is not enabled or an error occurs.

[hash]  
    Remember or display program locations.
    
        Determine and remember the full pathname of each command NAME.  If
        no arguments are given, information about remembered commands is 
        displayed.
        
        Options:
          -d  forget the remembered location of each NAME
          -l  display in a format that may be reused as input
          -p pathname  use PATHNAME as the full pathname of NAME
          -r  forget all remembered locations
          -t  print the remembered location of each NAME, preceding
            each location with the corresponding NAME if multiple
            NAMEs are given
        Arguments:
          NAME  Each NAME is searched for in $PATH and added to the list
            of remembered commands.
    
    
[help] 
    Display information about builtin commands.
    
        Displays brief summaries of builtin commands.  If PATTERN is
        specified, gives detailed help on all commands matching PATTERN,
        otherwise the list of help topics is printed.
        
        Options:
          -d  output short description for each topic
          -m  display usage in pseudo-manpage format
          -s  output only a short usage synopsis for each topic matching
            PATTERN
        
        Arguments:
          PATTERN  Pattern specifying a help topic
        
        Exit Status:
        Returns success unless PATTERN is not found or an invalid option is 
        given.
        
[history] 
    Display or manipulate the history list.
    
        Display the history list with line numbers, prefixing each modified
        entry with a '*'.  An argument of N lists only the last N entries.
        
        Options:
          -c  clear the history list by deleting all of the entries
          -d offset  delete the history entry at position OFFSET. Negative
            offsets count back from the end of the history list
        
          -a  append history lines from this session to the history file
          -n  read all history lines not already read from the history file
            and append them to the history list
          -r  read the history file and append the contents to the history
            list
          -w  write the current history to the history file
        
          -p  perform history expansion on each ARG and display the result
            without storing it in the history list
          -s  append the ARGs to the history list as a single entry
        
        If FILENAME is given, it is used as the history file.  Otherwise,
        if HISTFILE has a value, that is used, else ~/.bash_history.
        
        If the HISTTIMEFORMAT variable is set and not null, its value is used
        as a format string for strftime(3) to print the time stamp associated
        with each displayed history entry.  No time stamps are printed 
        otherwise.
        
        Exit Status:
        Returns success unless an invalid option is given or an error occurs.
          
[jobs] 
    Display status of jobs.
    
        Lists the active jobs.  JOBSPEC restricts output to that job.
        Without options, the status of all active jobs is displayed.
        
        Options:
          -l  lists process IDs in addition to the normal information
          -n  lists only processes that have changed status since the last
            notification
          -p  lists process IDs only
          -r  restrict output to running jobs
          -s  restrict output to stopped jobs
        
        If -x is supplied, COMMAND is run after all job specifications that
        appear in ARGS have been replaced with the process ID of that job's
        process group leader.
        
        Exit Status:
        Returns success unless an invalid option is given or an error occurs.
        If -x is used, returns the exit status of COMMAND.
    
[disown] 
    Remove jobs from current shell.
    
        Removes each JOBSPEC argument from the table of active jobs.  Without
        any JOBSPECs, the shell uses its notion of the current job.
        
        Options:
          -a  remove all jobs if JOBSPEC is not supplied
          -h  mark each JOBSPEC so that SIGHUP is not sent to the job if the
            shell receives a SIGHUP
          -r  remove only running jobs
        
        Exit Status:
        Returns success unless an invalid option or JOBSPEC is given.
    
[kill] 
    Send a signal to a job.
    
        Send the processes identified by PID or JOBSPEC the signal named by
        SIGSPEC or SIGNUM.  If neither SIGSPEC nor SIGNUM is present, then
        SIGTERM is assumed.
        
        Options:
          -s sig  SIG is a signal name
          -n sig  SIG is a signal number
          -l  list the signal names; if arguments follow '-l' they are
            assumed to be signal numbers for which names should be listed
          -L  synonym for -l
        
        Kill is a shell builtin for two reasons: it allows job IDs to be used
        instead of process IDs, and allows processes to be killed if the limit
        on processes that you can create is reached.
        
        Exit Status:
        Returns success unless an invalid option is given or an error occurs.

[let] 
    Evaluate arithmetic expressions.
    
        Evaluate each ARG as an arithmetic expression.  Evaluation is done in
        fixed-width integers with no check for overflow, though division by 0
        is trapped and flagged as an error.  The following list of operators is
        grouped into levels of equal-precedence operators.  The levels are 
        listed in order of decreasing precedence.
        
            id++, id--       variable post-increment, post-decrement
            ++id, --id       variable pre-increment, pre-decrement
            -, +             unary minus, plus
            !, ~             logical and bitwise negation
            '**'             exponentiation
            '*', /, %        multiplication, division, remainder
            +, -             addition, subtraction
            <<, >>           left and right bitwise shifts
            <=, >=, <, >     comparison
            ==, !=           equality, inequality
            &                bitwise AND
            ^                tbitwise XOR
            |                bitwise OR
            &&               logical AND
            ||               logical OR
            expr ? expr : expr
                             conditional operator
            =, *=, /=, %=,
            +=, -=, <<=, >>=,
            &=, ^=, |=       assignment
        
        Shell variables are allowed as operands.  The name of the variable
        is replaced by its value (coerced to a fixed-width integer) within
        an expression.  The variable need not have its integer attribute
        turned on to be used in an expression.
        
        Operators are evaluated in order of precedence.  Sub-expressions in
        parentheses are evaluated first and may override the precedence
        rules above.
        
        Exit Status:
        If the last ARG evaluates to 0, let returns 1; let returns 0 otherwise.
    
[shift] 
    Shift positional parameters.
    
        Rename the positional parameters $N+1,$N+2 ... to $1,$2 ...  If N is
        not given, it is assumed to be 1.
        
        Exit Status:
        Returns success unless N is negative or greater than $#.
         
[suspend]  
    Suspend shell execution.
    
        Suspend the execution of this shell until it receives a SIGCONT signal.
        Unless forced, login shells cannot be suspended.
        
        Options:
          -f  force the suspend, even if the shell is a login shell
        
        Exit Status:
        Returns success unless job control is not enabled or an error occurs.

[eval] 
    Evaluate conditional expression.
    
        Exits with a status of 0 (true) or 1 (false) depending on
        the evaluation of EXPR.  Expressions may be unary or binary.  Unary
        expressions are often used to examine the status of a file.  There
        are string operators and numeric comparison operators as well.
        
        The behavior of test depends on the number of arguments.  Read the
        bash manual page for the complete specification.
        
        File operators:
        
          -a FILE        True if file exists.
          -b FILE        True if file is block special.
          -c FILE        True if file is character special.
          -d FILE        True if file is a directory.
          -e FILE        True if file exists.
          -f FILE        True if file exists and is a regular file.
          -g FILE        True if file is set-group-id.
          -h FILE        True if file is a symbolic link.
          -L FILE        True if file is a symbolic link.
          -k FILE        True if file has its 'sticky' bit set.
          -p FILE        True if file is a named pipe.
          -r FILE        True if file is readable by you.
          -s FILE        True if file exists and is not empty.
          -S FILE        True if file is a socket.
          -t FD          True if FD is opened on a terminal.
          -u FILE        True if the file is set-user-id.
          -w FILE        True if the file is writable by you.
          -x FILE        True if the file is executable by you.
          -O FILE        True if the file is effectively owned by you.
          -G FILE        True if the file is effectively owned by your group.
          -N FILE        True if the file has been modified since it was last 
                         read.
        
          FILE1 -nt FILE2  True if file1 is newer than file2 (according to
                           modification date).
        
          FILE1 -ot FILE2  True if file1 is older than file2.
        
          FILE1 -ef FILE2  True if file1 is a hard link to file2.
        
        String operators:
        
          -z STRING      True if string is empty.
        
          -n STRING
             STRING      True if string is not empty.
        
          STRING1 = STRING2
                         True if the strings are equal.
          STRING1 != STRING2
                         True if the strings are not equal.
          STRING1 < STRING2
                         True if STRING1 sorts before STRING2 
                         lexicographically.
          STRING1 > STRING2
                         True if STRING1 sorts after STRING2 lexicographically.
        
        Other operators:
        
          -o OPTION      True if the shell option OPTION is enabled.
          -v VAR         True if the shell variable VAR is set.
          -R VAR         True if the shell variable VAR is set and is a name
                         reference.
          ! EXPR         True if expr is false.
          EXPR1 -a EXPR2 True if both expr1 AND expr2 are true.
          EXPR1 -o EXPR2 True if either expr1 OR expr2 is true.
        
          arg1 OP arg2   Arithmetic tests.  OP is one of -eq, -ne,
                         -lt, -le, -gt, or -ge.
        
        Arithmetic binary operators return true if ARG1 is equal, not-equal,
        less-than, less-than-or-equal, greater-than, or greater-than-or-equal
        than ARG2.
        
        Exit Status:
        Returns success if EXPR evaluates to true; fails if EXPR evaluates to
        false or an invalid argument is given.

[times] 
    Display process times.
    
        Prints the accumulated user and system times for the shell and all of 
        its child processes.
        
        Exit Status:
        Always succeeds.

[ulimit] 
    Modify shell resource limits.
    
        Provides control over the resources available to the shell and 
        processes
        it creates, on systems that allow such control.
        
        Options:
          -S  use the 'soft' resource limit
          -H  use the 'hard' resource limit
          -a  all current limits are reported
          -b  the socket buffer size
          -c  the maximum size of core files created
          -d  the maximum size of a process's data segment
          -e  the maximum scheduling priority ('nice')
          -f  the maximum size of files written by the shell and its children
          -i  the maximum number of pending signals
          -k  the maximum number of kqueues allocated for this process
          -l  the maximum size a process may lock into memory
          -m  the maximum resident set size
          -n  the maximum number of open file descriptors
          -p  the pipe buffer size
          -q  the maximum number of bytes in POSIX message queues
          -r  the maximum real-time scheduling priority
          -s  the maximum stack size
          -t  the maximum amount of cpu time in seconds
          -u  the maximum number of user processes
          -v  the size of virtual memory
          -x  the maximum number of file locks
          -P  the maximum number of pseudoterminals
          -R  the maximum time a real-time process can run before blocking
          -T  the maximum number of threads
        
        Not all options are available on all platforms.
        
        If LIMIT is given, it is the new value of the specified resource; the
        special LIMIT values 'soft', 'hard', and 'unlimited' stand for the
        current soft limit, the current hard limit, and no limit, respectively.
        Otherwise, the current value of the specified resource is printed.  If
        no option is given, then -f is assumed.
        
        Values are in 1024-byte increments, except for -t, which is in seconds,
        -p, which is in increments of 512 bytes, and -u, which is an unscaled
        number of processes.
        
        Exit Status:
        Returns success unless an invalid option is supplied or an error occurs.
        
[umask] 
    Display or set file mode mask.
    
        Sets the user file-creation mask to MODE.  If MODE is omitted, prints
        the current value of the mask.
        
        If MODE begins with a digit, it is interpreted as an octal number;
        otherwise it is a symbolic mode string like that accepted by chmod(1).
        
        Options:
          -p  if MODE is omitted, output in a form that may be reused as input
          -S  makes the output symbolic; otherwise an octal number is output
        
        Exit Status:
        Returns success unless MODE is invalid or an invalid option is given.
    
[return] 
    Return from a shell function.

        Causes a function or sourced script to exit with the return value
        specified by N.  If N is omitted, the return status is that of the
        last command executed within the function or script.

        Exit Status:
        Returns N, or failure if the shell is not executing a function or script.
    
[wait] 
    Wait for job completion and return exit status.
    
        Waits for each process identified by an ID, which may be a process ID or 
        a job specification, and reports its termination status.  If ID is not
        given, waits for all currently active child processes, and the return
        status is zero.  If ID is a job specification, waits for all processes
        in that job's pipeline.
        
        If the -n option is supplied, waits for a single job from the list of IDs,
        or, if no IDs are supplied, for the next job to complete and returns its 
        exit status.
        
        If the -p option is supplied, the process or job identifier of the job
        for which the exit status is returned is assigned to the variable VAR
        named by the option argument. The variable will be unset initially, 
        before any assignment. This is useful only when the -n option is supplied.
        
        If the -f option is supplied, and job control is enabled, waits for the
        specified ID to terminate, instead of waiting for it to change status.
        
        Exit Status:
        Returns the status of the last ID; fails if ID is invalid or an invalid
        option is given, or if -n is supplied and the shell has no unwaited-for
        children.
    
[for] 
    Execute commands for each member in a list.
        
        The 'for' loop executes a sequence of commands for each member in a
        list of items.  If 'in WORDS ...;' is not present, then 'in \$@\' is
        assumed.  For each element in WORDS, NAME is set to that element, and
        the COMMANDS are executed.
        
        Exit Status:
        Returns the status of the last command executed.
    
[select] 
    Select words from a list and execute commands.
    
        The WORDS are expanded, generating a list of words.  The
        set of expanded words is printed on the standard error, each
        preceded by a number.  If 'in WORDS' is not present, 'in \$@\'
        is assumed.  The PS3 prompt is then displayed and a line read
        from the standard input.  If the line consists of the number
        corresponding to one of the displayed words, then NAME is set
        to that word.  If the line is empty, WORDS and the prompt are
        redisplayed.  If EOF is read, the command completes.  Any other
        value read causes NAME to be set to null.  The line read is saved
        in the variable REPLY.  COMMANDS are executed after each selection
        until a break command is executed.
        
        Exit Status:
        Returns the status of the last command executed.
        
[time] 
    Report time consumed by pipeline's execution.
        
        Execute PIPELINE and print a summary of the real time, user CPU time,
        and system CPU time spent executing PIPELINE when it terminates.
        
        Options:
          -p  print the timing summary in the portable Posix format
        
        The value of the TIMEFORMAT variable is used as the output format.
        
        Exit Status:
        The return status is the return status of PIPELINE.
        
[case] 
    Execute commands based on pattern matching.
        
        Selectively execute COMMANDS based upon WORD matching PATTERN.  The
        '|' is used to separate multiple patterns.
        
        Exit Status:
        Returns the status of the last command executed.
        
[if] 
    Execute commands based on conditional.
    
        The 'if COMMANDS' list is executed.  If its exit status is zero, then 
        the
        'then COMMANDS' list is executed.  Otherwise, each 'elif COMMANDS' list 
        is
        executed in turn, and if its exit status is zero, the corresponding
        'then COMMANDS' list is executed and the if command completes.  
        Otherwise,
        the 'else COMMANDS' list is executed, if present.  The exit status of 
        the
        entire construct is the exit status of the last command executed, or 
        zero
        if no condition tested true.
        
        Exit Status:
        Returns the status of the last command executed.
    
[while] 
    Execute commands as long as a test succeeds.
        
        Expand and execute COMMANDS as long as the final command in the
        'while' COMMANDS has an exit status of zero.
        
        Exit Status:
        Returns the status of the last command executed.
    
[until] 
    Execute commands as long as a test does not succeed.
    
        Expand and execute COMMANDS as long as the final command in the
        'until' COMMANDS has an exit status which is not zero.
        
        Exit Status:
        Returns the status of the last command executed.
        
[coproc] 
    Create a coprocess named NAME.
    
        Execute COMMAND asynchronously, with the standard output and standard
        input of the command connected via a pipe to file descriptors assigned
        to indices 0 and 1 of an array variable NAME in the executing shell.
        The default NAME is \COPROC\.
        
        Exit Status:
        The coproc command returns an exit status of 0.
        
[variables] 
    Common shell variable names and usage.
    
        BASH_VERSION  Version information for this Bash.
        CDPATH  A colon-separated list of directories to search
            for directories given as arguments to 'cd'.
        GLOBIGNORE  A colon-separated list of patterns describing filenames to
            be ignored by pathname expansion.
        HISTFILE  The name of the file where your command history is stored.
        HISTFILESIZE  The maximum number of lines this file can contain.
        HISTSIZE  The maximum number of history lines that a running
            shell can access.
        HOME  The complete pathname to your login directory.
        HOSTNAME  The name of the current host.
        HOSTTYPE  The type of CPU this version of Bash is running under.
        IGNOREEOF  Controls the action of the shell on receipt of an EOF
            character as the sole input.  If set, then the value
            of it is the number of EOF characters that can be seen
            in a row on an empty line before the shell will exit
            (default 10).  When unset, EOF signifies the end of input.
        MACHTYPE  A string describing the current system Bash is running on.
        MAILCHECK  How often, in seconds, Bash checks for new mail.
        MAILPATH  A colon-separated list of filenames which Bash checks
            for new mail.
        OSTYPE  The version of Unix this version of Bash is running on.
        PATH  A colon-separated list of directories to search when
            looking for commands.
        PROMPT_COMMAND  A command to be executed before the printing of each
            primary prompt.
        PS1    The primary prompt string.
        PS2    The secondary prompt string.
        PWD    The full pathname of the current directory.
        SHELLOPTS  A colon-separated list of enabled shell options.
        TERM  The name of the current terminal type.
        TIMEFORMAT  The output format for timing statistics displayed by the
            'time' reserved word.
        auto_resume  Non-null means a command word appearing on a line by
            itself is first looked for in the list of currently
            stopped jobs.  If found there, that job is foregrounded.
            A value of 'exact' means that the command word must
            exactly match a command in the list of stopped jobs.  A
            value of 'substring' means that the command word must
            match a substring of the job.  Any other value means that
            the command must be a prefix of a stopped job.
        histchars  Characters controlling history expansion and quick
            substitution.  The first character is the history
            substitution character, usually '!'.  The second is
            the 'quick substitution' character, usually '^'.  The
            third is the 'history comment' character, usually '#'.
        HISTIGNORE  A colon-separated list of patterns used to decide which
            commands should be saved on the history list.
   
[pushd] 
    Add directories to stack.
    
        Adds a directory to the top of the directory stack, or rotates
        the stack, making the new top of the stack the current working
        directory.  With no arguments, exchanges the top two directories.
        
        Options:
          -n Suppresses the normal change of directory when adding
          directories to the stack, so only the stack is manipulated.
        
        Arguments:
          +N Rotates the stack so that the Nth directory (counting
          from the left of the list shown by 'dirs', starting with
          zero) is at the top.
        
          -N Rotates the stack so that the Nth directory (counting
          from the right of the list shown by 'dirs', starting with
          zero) is at the top.
        
          dir Adds DIR to the directory stack at the top, making it the
          new current working directory.
        
        The 'dirs' builtin displays the directory stack.
        
        Exit Status:
        Returns success unless an invalid argument is supplied or the directory
        change fails.
    
[popd] 
    Removes entries from the directory stack.  With no arguments, removes
        the top directory from the stack, and changes to the new top directory.
        
        Options:
          -n  Suppresses the normal change of directory when removing
          directories from the stack, so only the stack is manipulated.
        
        Arguments:
          +N  Removes the Nth entry counting from the left of the list
          shown by 'dirs', starting with zero.  For example: 'popd +0'
          removes the first directory, 'popd +1' the second.
        
          -N  Removes the Nth entry counting from the right of the list
          shown by 'dirs', starting with zero.  For example: 'popd -0'
          removes the last directory, 'popd -1' the next to last.
        
        The 'dirs' builtin displays the directory stack.
        the stack, making the new top of the stack the current working
        directory.  With no arguments, exchanges the top two directories.
        
[dirs] 
    Display the list of currently remembered directories.  Directories
        find their way onto the list with the 'pushd' command; you can get
        back up through the list with the 'popd' command.
        
        Options:
          -c  clear the directory stack by deleting all of the elements
          -l  do not print tilde-prefixed versions of directories relative
          to your home directory
          -p  print the directory stack with one entry per line
          -v  print the directory stack with one entry per line prefixed
          with its position in the stack
        
        Arguments:
          +N  Displays the Nth entry counting from the left of the list shown 
        by dirs when invoked without options, starting with zero.
        
          -N  Displays the Nth entry counting from the right of the list shown 
        by dirs when invoked without options, starting with zero.
        
[printf] 
    Formats and prints ARGUMENTS under control of the FORMAT.
    
        Options:
          -v var assign the output to shell variable VAR rather than
          display it on the standard output
        
        FORMAT is a character string which contains three types of objects: 
        plain
        characters, which are simply copied to standard output; character 
        escape
        sequences, which are converted and copied to the standard output; and
        format specifications, each of which causes printing of the next 
        successive
        argument.
        
        In addition to the standard format specifications described in 
        printf(1),
        printf interprets:
        
          %b expand backslash escape sequences in the corresponding argument
          %q quote the argument in a way that can be reused as shell input
          %(fmt)T output the date-time string resulting from using FMT as a 
          format
                 string for strftime(3)
        
        The format is re-used as necessary to consume all of the arguments.  If
        there are fewer arguments than the format requires,  extra format
        specifications behave as if a zero value or null string, as 
        appropriate,
        had been supplied.
        
        Exit Status:
        Returns success unless an invalid option is given or a write or 
        assignment
        error occurs.
    
[complete] 
    Specify how arguments are to be completed by Readline.
    
        For each NAME, specify how arguments are to be completed.  If no 
        options are supplied, existing completion specifications are printed 
        in a way that allows them to be reused as input.
        
        Options:
          -p  print existing completion specifications in a reusable format
          -r  remove a completion specification for each NAME, or, if no
            NAMEs are supplied, all completion specifications
          -D  apply the completions and actions as the default for commands
            without any specific completion defined
          -E  apply the completions and actions to \empty\ commands --
            completion attempted on a blank line
          -I  apply the completions and actions to the initial (usually the
            command) word
        
        When completion is attempted, the actions are applied in the order the
        uppercase-letter options are listed above. If multiple options are 
        supplied,
        the -D option takes precedence over -E, and both take precedence over -
        I.
        
        Exit Status:
        Returns success unless an invalid option is supplied or an error occurs.

[compgen] 
    Display possible completions depending on the options.
    
        Intended to be used from within a shell function generating possible
        completions.  If the optional WORD argument is supplied, matches 
        against WORD are generated.
        
        Exit Status:
        Returns success unless an invalid option is supplied or an error occurs.
    
[compopt] 
    Modify or display completion options.
    
        Modify the completion options for each NAME, or, if no NAMEs are 
        supplied,
        the completion currently being executed.  If no OPTIONs are given, 
        print the completion options for each NAME or the current completion 
        specification.
        
        Options:
          -o option  Set completion option OPTION for each NAME
          -D    Change options for the \default\ command completion
          -E    Change options for the \empty\ command completion
          -I    Change options for completion on the initial word
        
        Using '+o' instead of '-o' turns off the specified option.
        
        Arguments:
        
        Each NAME refers to a command for which a completion specification must
        have previously been defined using the 'complete' builtin.  If no NAMEs
        are supplied, compopt must be called by a function currently generating
        completions, and the options for that currently-executing completion
        generator are modified.
        
        Exit Status:
        Returns success unless an invalid option is supplied or NAME does not
        have a completion specification defined.

[mapfile] 
    Read lines from the standard input into an indexed array variable.
    
        Read lines from the standard input into the indexed array variable 
        ARRAY, or from file descriptor FD if the -u option is supplied. The variable 
        MAPFILE is the default ARRAY.
        
        Options:
          -d delim  Use DELIM to terminate lines, instead of newline
          -n count  Copy at most COUNT lines.  If COUNT is 0, all lines are 
        copied
          -O origin  Begin assigning to ARRAY at index ORIGIN.  The default 
        index is 0
          -s count  Discard the first COUNT lines read
          -t  Remove a trailing DELIM from each line read (default newline)
          -u fd  Read lines from file descriptor FD instead of the standard 
        input
          -C callback  Evaluate CALLBACK each time QUANTUM lines are read
          -c quantum  Specify the number of lines read between each call to
              CALLBACK
        
        Arguments:
          ARRAY  Array variable name to use for file data
        
        If -C is supplied without -c, the default quantum is 5000.  When
        CALLBACK is evaluated, it is supplied the index of the next array
        element to be assigned and the line to be assigned to that element
        as additional arguments.
        
        If not supplied with an explicit origin, mapfile will clear ARRAY 
        before assigning to it.
        
        Exit Status:
        Returns success unless an invalid option is given or ARRAY is readonly 
        or not an indexed array.
        
[readarray] 
    Read lines from a file into an array variable.
    
        A synonym for 'mapfile'.

[unset] 
    Unset values and attributes of shell variables and functions.
    
        For each NAME, remove the corresponding variable or function.
        
        Options:
          -f  treat each NAME as a shell function
          -v  treat each NAME as a shell variable
          -n  treat each NAME as a name reference and unset the variable itself
            rather than the variable it references
        
        Without options, unset first tries to unset a variable, and if that 
        fails,
        tries to unset a function.
        
        Some variables cannot be unset; also see 'readonly'.
        
        Exit Status:
        Returns success unless an invalid option is given or a NAME is read-only.
    
[local] 
    Remember or display program locations.
    
        Determine and remember the full pathname of each command NAME.  If
        no arguments are given, information about remembered commands is 
        displayed.
        
        Options:
          -d  forget the remembered location of each NAME
          -l  display in a format that may be reused as input
          -p pathname  use PATHNAME as the full pathname of NAME
          -r  forget all remembered locations
          -t  print the remembered location of each NAME, preceding
            each location with the corresponding NAME if multiple
            NAMEs are given
        Arguments:
          NAME  Each NAME is searched for in $PATH and added to the list
            of remembered commands.
        
        Exit Status:
        Returns success unless NAME is not found or an invalid option is given.

[export] 
    Set export attribute for shell variables.
    
        Marks each NAME for automatic export to the environment of subsequently
        executed commands.  If VALUE is supplied, assign VALUE before 
        exporting.
        
        Options:
          -f refer to shell functions
          -n remove the export property from each NAME
          -p display a list of all exported variables and functions
        
        An argument of '--' disables further option processing.
        
        Exit Status:
        Returns success unless an invalid option is given or NAME is invalid.
        
[readonly] 
    Mark shell variables as unchangeable.
    
        Mark each NAME as read-only; the values of these NAMEs may not be
        changed by subsequent assignment.  If VALUE is supplied, assign VALUE
        before marking as read-only.
        
        Options:
          -a refer to indexed array variables
          -A refer to associative array variables
          -f refer to shell functions
          -p display a list of all readonly variables or functions,
          depending on whether or not the -f option is given
        
        An argument of '--' disables further option processing.
        
        Exit Status:
        Returns success unless an invalid option is given or NAME is invalid.

[declare]     
    Set variable values and attributes.
    
        Declare variables and give them attributes.  If no NAMEs are given,
        display the attributes and values of all variables.
    
         Options:
          -f\trestrict action or display to function names and definitions
          -F\trestrict display to function names only (plus line number and
             \t\tsource file when debugging)
          -g\tcreate global variables when used in a shell function; otherwise
             \t\tignored
          -I\tif creating a local variable, inherit the attributes and value
             \t\tof a variable with the same name at a previous scope
          -p\tdisplay the attributes and value of each NAME
    
         Options which set attributes:
          -a\tto make NAMEs indexed arrays (if supported)
          -A\tto make NAMEs associative arrays (if supported)
          -i\tto make NAMEs have the 'integer' attribute
          -l\tto convert the value of each NAME to lower case on assignment
          -n\tmake NAME a reference to the variable named by its value
          -r  to make NAMEs readonly
          -t  to make NAMEs have the 'trace' attribute
          -u  to convert the value of each NAME to upper case on assignment
          -x  to make NAMEs export
        
        Using '+' instead of '-' turns off the given attribute.
        
        Variables with the integer attribute have arithmetic evaluation (see
        the 'let' command) performed when the variable is assigned a value.
        
        When used in a function, 'declare' makes NAMEs local, as with the 
        'local'
        command.  The '-g' option suppresses this behavior.
        
        Exit Status:
        Returns success unless an invalid option is supplied or a variable
        assignment error occurs.

[function] 
    Define shell function.
    
        Create a shell function named NAME.  When invoked as a simple command,
        NAME runs COMMANDs in the calling shell's context.  When NAME is 
        invoked,
        the arguments are passed to the function as $1...$n, and the function's
        name is in $FUNCNAME.
        
        Exit Status:
        Returns success unless NAME is readonly.

[typeset] 
    Set variable values and attributes. 
    
        A synonym for 'declare'.  See 'help declare'.

[source] 
    Execute commands from a file in the current shell.
    
        Read and execute commands from FILENAME in the current shell.  The
        entries in $PATH are used to find the directory containing FILENAME.
        If any ARGUMENTS are supplied, they become the positional parameters
        when FILENAME is executed.
        
        Exit Status:
        Returns the status of the last command executed in FILENAME; fails if
        FILENAME cannot be read.
        
[bind]  
    Set Readline key bindings and variables.
        
        Bind a key sequence to a Readline function or a macro, or set a
        Readline variable.  The non-option argument syntax is equivalent to
        that found in ~/.inputrc, but must be passed as a single argument:
        e.g., bind '\\\C-x\\C-r\: re-read-init-file'.
        
        Options:
          -m  keymap         Use KEYMAP as the keymap for the duration of this
                             command.  Acceptable keymap names are emacs,
                             emacs-standard, emacs-meta, emacs-ctlx, vi, vi-
                             move,
                             vi-command, and vi-insert.
          -l                 List names of functions.
          -P                 List function names and bindings.
          -p                 List functions and bindings in a form that can be
                             reused as input.
          -S                 List key sequences that invoke macros and their 
                             values
          -s                 List key sequences that invoke macros and their 
                             values
                             in a form that can be reused as input.
          -V                 List variable names and values
          -v                 List variable names and values in a form that can
                             be reused as input.
          -q  function-name  Query about which keys invoke the named function.
          -u  function-name  Unbind all keys which are bound to the named 
                             function.
          -r  keyseq         Remove the binding for KEYSEQ.
          -f  filename       Read key bindings from FILENAME.
          -x  keyseq:shell-command  Cause SHELL-COMMAND to be executed when
                                    KEYSEQ is entered.
          -X                 List key sequences bound with -x and associated 
                             commands
                             in a form that can be reused as input.
        
        Exit Status:
        bind returns 0 unless an unrecognized option is given or an error occurs.

[test] 
    Evaluate arithmetic expressions.
    
        Evaluate each ARG as an arithmetic expression.  Evaluation is done in
        fixed-width integers with no check for overflow, though division by 0
        is trapped and flagged as an error.  The following list of operators is
        grouped into levels of equal-precedence operators.  The levels are 
        listed
        in order of decreasing precedence.
        
                
            id++, id--       variable post-increment, post-decrement
            ++id, --id       variable pre-increment, pre-decrement
            -, +             unary minus, plus
            !, ~             logical and bitwise negation
            '**'             exponentiation
            '*', /, %        multiplication, division, remainder
            +, -             addition, subtraction
            <<, >>           left and right bitwise shifts
            <=, >=, <, >     comparison
            ==, !=           equality, inequality
            &                bitwise AND
            ^                tbitwise XOR
            |                bitwise OR
            &&               logical AND
            ||               logical OR
            expr ? expr : expr
                             conditional operator
            =, *=, /=, %=,
            +=, -=, <<=, >>=,
            &=, ^=, |=       assignment
  

        Shell variables are allowed as operands.  The name of the variable
        is replaced by its value (coerced to a fixed-width integer) within
        an expression.  The variable need not have its integer attribute
        turned on to be used in an expression.
        
        Operators are evaluated in order of precedence.  Sub-expressions in
        parentheses are evaluated first and may override the precedence
        rules above.
        
        Exit Status:
        If the last ARG evaluates to 0, let returns 1; let returns 0 otherwise.

*[other] show other command help info.   
 }

helpname = {
$cmdName ->
[command0] {" % - Resume job in foreground."}
[command1] {"(( - Evaluate conditional expression. "}
[command2] {". - Execute commands from a file in the current shell. "}
[command3] {": - Null command."}
[command4] {"[ - Evaluate conditional expression."}
[command5] {"[[ - Execute conditional command. "}
[command6] {"for (( - Arithmetic for loop."}
[command7] Group commands as a unit. 
[set]  set -  Set or unset values of shell options and positional parameters.
[read] read - Read a line from the standard input and split it into fields.
[type] type - Display information about command type.
[trap] trap - Trap signals and other events.
[alias] alias - Define or display aliases.
[unalias] unalias - Remove each NAME from the list of defined aliases.
[break] break - Exit for, while, or until loops.
[continue] continue - Resume for, while, or until loops.
[builtin] builtin - Execute shell builtins.
[caller] caller - Returns the context of the current subroutine call.
[cd] cd - Change the shell working directory.
[pwd] pwd - Print the name of the current working directory.
[true] true - Null command.
[shopt] shopt - Set and unset shell options.
[false] false - Return an unsuccessful result.
[command] command - Execute a simple command or display information about commands.
[echo] echo - Write arguments to the standard output.
[enable] enable - Enable and disable shell builtins.
[getopts] getopts - Parse option arguments.
[exec] exec - Replace the shell with the given command.
[exit] exit - Exit the shell.
[logout] logout - Exit a login shell.
[fc] fc - Display or execute commands from the history list.
[fg] fg - Resume job in foreground.
[bg] bg - Move jobs to the background.
[hash] hash - Remember or display program locations.
[help] help - Display information about builtin commands.
[history] history - Display or manipulate the history list.
[jobs] jobs - Display status of jobs.
[disown] discow - Remove jobs from current shell.
[kill] kill - Send a signal to a job.
[let] let - Evaluate arithmetic expressions.
[shift] shift - Shift positional parameters.
[suspend] suspend - Suspend shell execution.
[eval] eval - Evaluate conditional expression.
[times] times - Display process times.
[ulimit] limit - Modify shell resource limits.
[umask] umask - Display or set file mode mask.
[return] return - Wait for job completion and return exit status.
[wait] wait -Wait for process completion and return exit status.
[for] for - Execute commands for each member in a list.
[select] select - Select words from a list and execute commands.
[time] time - Report time consumed by pipeline's execution.
[case] case - Execute commands based on pattern matching.
[if] if - Execute commands based on conditional.
[while] while - Execute commands as long as a test succeeds.
[until] until - Execute commands as long as a test does not succeed.
[coproc] corproc - Create a coprocess named NAME.
[variables] variables - Common shell variable names and usage.
[pushd] pushd - Add directories to stack.
[popd] popd - Removes entries from the directory stack.
[dirs] dirs - Display the list of currently remembered directories.
[printf] printf - Formats and prints ARGUMENTS under control of the FORMAT.
[complete] complete - Specify how arguments are to be completed by Readline.
[compgen] compgen - Display possible completions depending on the options.
[compopt] compopt - Modify or display completion options.
[mapfile] mapfile - Read lines from the standard input into an indexed array variable.
[unset] unset - Unset values and attributes of shell variables and functions.
[readarray] readarry - Read lines from a file into an array variable.
[local] local -Remember or display program locations.。
[export] export - Set export attribute for shell variables.
[readonly] readonly - Mark shell variables as unchangeable.
[function] function - Define shell function.
[typeset] typeset - Set variable values and attributes.
[source] source - Execute commands from a file in the current shell.
[bind] bind -  Set Readline key bindings and variables.。
[test] test - Evaluate arithmetic expressions.
[declare] declare - Set variable values and attributes.
*[other] show other command help info..
}

helpsynopsis = {
$cmdName ->
[command0] {"job_spec [&]"}
[command1] {"(( expression ))"}
[command2] {". filename [arguments]"}
[command3] {":"}
[command4] {"[ ... ]"}
[command5] {"[[ expression ]]"}
[command6] {"for (( exp1; exp2; exp33 )); do COMMANDS ; done"}
[command7] {"{ COMMAND ; }"}
[set]   {"set [-abefhkmnptuvxBCHP] [-o option-name] [--] [arg ...]"}
[read]  {"read [-ers] [-a array] [-d delim] [-i text] [-n nchars] [-N nchars] [-p prompt] [-t timeout] [-u fd] [name ...]"}
[type]  {"type [-afptP] name [name ...]"}
[trap]  {"trap [-lp] [[arg] signal_spec ...]"}
[alias] {"alias [-p] [name[=value] ... ]"}
[unalias] {"unalias [-a] name [name ...]"}
[break] {"break [n]"}
[continue] {"continue [n]"}
[builtin] {"unalias [-a] name [name ...]"}
[caller] {"caller [expr] "}
[cd] {"cd [-L|[-P [-e]] [-@]] [dir]"}
[pwd] {"pwd [-LP]"}
[true] {"true"}
[shopt] {"shopt [-pqsu] [-o] [optname ...]"}
[false] {"false"}
[command] {"command [-pVv] command [arg ...]"}
[echo] {"echo [-neE] [arg ...]"}
[enable] {"enable [-a] [-dnps] [-f filename] [name ...] "}
[getopts] {"getopts optstring name [arg ...]"}
[exec] {"exec [-cl] [-a name] [command [argument ...]] [redirection ...] "}
[exit] {"exit [n]"}
[logout] {"logout [n]"}
[fc] {"fc [-e ename] [-lnr] [first] [last] or fc -s [pat=rep] [command]"}
[fg] {"fg [job_spec] "}
[bg] {"bg [job_spec ...]"}
[hash] {"hash [-lr] [-p pathname] [-dt] [name ...]"}
[help] {"help [-dms] [pattren ...]"}
[history] {"history [-c] [-d offset] [n] or history -anrw [filename] or history -ps arg [arg...]"}
[jobs] {"jobs [-lnprs] [jobspec ...] or jobs -x command [args]"}
[disown] {"disown [-h] [-ar] [jobspec ... | pid ...]"}
[kill] {"kill [-s sigspec | -n signum | -sigspec] pid | jobspec ... or kill -l [sigspec]"}
[let] {"let arg [arg ...]"}
[shift] {"shift [n]"}
[suspend] {"suspend [-f]"}
[eval] {"eval [arg ...]"}
[times] {"times"}
[ulimit] {"ulimit [-SHabcdefiklmnpqrstuvxPT] [ulimit]"}
[umask] {"umask [-p] [-S] [pattern]"}
[return] {"return [n]"}
[wait] {"wait [-fn] [-p var] [id ...]"}
[for] {"for NAME [in WORDS ... ] ; do COMMANDS; done"}
[select] {"select NAME [in WORDS ... ;] do COMMANDS; done"}
[time] {"time [-p] pipeline"}
[case] {"case WORD in [pattern [| pattern]...) COMMANDS ;;]... esac"}
[if] {"if COMMANDS; then COMMANDS; [ elif COMMANDS; then COMMANDS; ]... [ else COMMANDS; ] fi"}
[while] {"while COMMANDS; do COMMANDS; done"}
[until] {"until COMMANDS; do COMMANDS; done"}
[coproc] {"coproc [NAME] COMMANDS [redirections]"}
[variables] {"variables - variables - Names and meanings of some shell variables"}
[pushd] {"pushd [-n] [+N | -N | dir]"}
[popd] {"popd [-n] [+N | -N]"}
[dirs] {"dirs [-clpv] [+N] [-N]"}
[printf] {"printf [-v var] format [arguments]"}
[complete] {"complete [-abcdefgjksuv] [-pr] [-DEI] [-o option] [-A action] [-G globpat] [-W wordlist] [-F function] [-C command] [-X filterpat] [-P prefix] [-S suffix] [name ...]"}
[compgen] {"compgen [-abcdefgjksuv] [-o option] [-A action] [-G globpat] [-W wordlist] [-F function] [-C command] [-X filterpat] [-P prefix] [-S suffix] [word]"}
[compopt] {"compopt [-o|+o option] [-DEI] [name ...]"}
[mapfile] {"mapfile [-d delim] [-n count] [-O origin] [-s count] [-t] [-u fd] [-C callback] [-c quantum] [array]]"}
[unset] {"unset [-f] [-v] [-n] [NAME ...]"}
[readarray] {"readarray [-d delim] [-n count] [-O origin] [-s count] [-t] [-u fd] [-C callback] [-c quantum] [array]"}
[local] {"local [option] name[=value] ... "}
[export] {"export [-fn] [name[=value] ...] or export -p"}
[readonly] {"readonly [-aAf] [name[=value] ...] or readonly -p"}
[function] {"function NAME { COMMANDS ; } 或 name () { COMMANDS ; } "}
[typeset] {"typeset [-aAfFgiIlnrtux] [-p] name[=value] ..."}
[source] {"source filename [arguments]"}
[bind] {"bind [-lpsvPSVX] [-m keymap] [-f filename] [-q name] [-u name] [-r keyseq] [-x keyseq:shell-command] [keyseq:readline-function or readline-command] "}
[test] {"test [expr]"}
[declare] {"declare [-aAfFgilnrtux] [-p] [name[=value] ...]"}
*[other]  {"show other command help info.."}
}

information =     These shell commands are defined internally.  Type 'help' to see this list.
  Type 'help name' to find out more about the function 'name'.
  Use 'info bash' to find out more about the shell in general.
  Use 'man -k' or 'info' to find out more about commands not in this list.
  A star (*) next to a name means that the command is disabled.

helperr = "no help topics match {$name} .Try 'help help' or 'man -k {$name}' or info {$name}"

is = {$str1} is {$str2}
special = {$str1} is a special shell builtin 
hashd = {$str1} is hashed {$str2} 
isfunction = {$str1} is a function 
iskeyword = {$str1} is a shell keyword
isalias = {$str1} is aliased to {$str2} 
isbuiltin = {$str1} is a shell builtin
killargerr = {$str1} : arguments must be process or job IDs
letwarn = utshell : let :  expression expected
bindvia = {$str1} can be invoked via 
bindnokeys = {$str1} is not bound to any keys.
unknowdfunction = {$str1} : unknown function name
unbindfaild = {$str1} : cannot unbind
invaildmap = {$str1} : invalid keymap name
logout = logout
nologinsh = not login shell: use 'exit'
stoppedjobs = There are stopped jobs.
runjobs = There are running jobs.
