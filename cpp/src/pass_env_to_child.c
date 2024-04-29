#include <sys/types.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>

extern char **environ;
static char *prog = "/bin/bash";
static char *env[] = {"NAME=TEST", NULL};
static char *const args[] = {"bash", "-c", "echo $0 && /usr/bin/env", NULL};
int main() {
    int pipe_fd[2];
    if (pipe(pipe_fd) == -1) {
        perror("pipe error");
    }
    pid_t child;
    int ret = -1;
    int status = 0;
    printf("parent env\n");
    for (char **env = environ; *env != NULL; env++) {
        printf("%s\n", *env);
    }
    ret = fork();
    if (ret == 0) {
        close(pipe_fd[0]);
        dup2(pipe_fd[1], STDOUT_FILENO);

        // execle 第二个参数是传给子进程的argv[0]，作为 process name
        // https://stackoverflow.com/questions/2050961/is-argv0-name-of-executable-an-accepted-standard-or-just-a-common-conventi

        // argv is an array of pointers to strings passed to the new program
        // as its command-line arguments.  By convention, the first of these
        // strings (i.e., argv[0]) should contain the filename associated
        // with the file being executed.  The argv array must be terminated
        // by a NULL pointer.  (Thus, in the new program, argv[argc] will be
        // NULL.)
        execle(prog, "foo", "-c", "echo $0 && /usr/bin/env", NULL, env);

        // 如果使用没有e后缀的exec家族函数，child env继承parent
        // execvp(prog, args);

        // exec 传递 environ 就能将parent的env原封不同传给child
        // execle(prog, "bash", "-c", "/usr/bin/env", NULL, environ);

        // execvpe(prog, args, env);
        perror("execle error");
        exit(EXIT_FAILURE);
        // child
    } else if (ret > 0) {
        child = ret;
        close(pipe_fd[1]);
        int r = waitpid(child, &status, 0);
        if (WIFEXITED(r)) {
            printf("bash exit successfully");
        }
        printf("\nchild output\n");
        char buffer[4096];
        ssize_t n;
        while ((n = read(pipe_fd[0], buffer, sizeof(buffer))) > 0) {
            write(STDOUT_FILENO, buffer, n);
        }
        close(pipe_fd[0]);
    } else {
        perror("fork syscall return");
        return -1;
    }
    return 0;
}