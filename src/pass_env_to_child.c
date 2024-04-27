#include <unistd.h>
static char *prog = "/bin/bash";
int main() {
    pid_t child;
    int ret = -1;
    ret = fork();
    if (ret == 0) {
        // child
    } else if (ret > 0) {
        child = ret;
    } else {
        return -1;
    }
    return 0;
}