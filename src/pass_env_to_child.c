#include <sys/types.h>
#include <sys/wait.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>

static char *prog = "/bin/bash";
static char *env[] = {"NAME=TEST", NULL};
int main() {
    int pipe_fd[2];
    if (pipe(pipe_fd) == -1) {
        perror("pipe error");
    }
    pid_t child;
    int ret = -1;
    int status = 0;
    printf("parent env\n");
    for (char **env = __environ; *env != NULL; env++) {
        printf("%s\n", *env);
    }
    ret = fork();
    if (ret == 0) {
        close(pipe_fd[0]);
        dup2(pipe_fd[1], STDOUT_FILENO);

        execle(prog, "bash", "-c", "/usr/bin/env", NULL, env);
        // exec 传递 __environ 就能将parent的env原封不同传给child
        // execle(prog, "bash", "-c", "/usr/bin/env", NULL, __environ);

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