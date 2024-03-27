#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>

int main() {
    int pipe_fd[2];
    pid_t pid;
    
    // 创建管道
    if (pipe(pipe_fd) == -1) {
        perror("pipe");
        exit(EXIT_FAILURE);
    }
    
    // 创建子进程
    pid = fork();
    if (pid == -1) {
        perror("fork");
        exit(EXIT_FAILURE);
    }
    
    if (pid == 0) { // 子进程
        // 关闭管道的读取端
        close(pipe_fd[0]);
        
        // 将标准错误重定向到管道的写入端
        dup2(pipe_fd[1], STDERR_FILENO);
        // 执行子进程的命令，这里使用 ls 命令举例
        // https://man7.org/linux/man-pages/man2/execve.2.html#:~:text=By%20default%2C%20file%20descriptors%20remain%20open%20across%20an%20execve()
        // exec syscall 会保留之前打开的fd。
        // 1. fork
        // 2. 将stderr定向到pipe
        // 3. exec 运行真正的程序
        // 以上三步完成了对子进程stderr输出的收集
        execlp("ls", "ls", "/nonexistent_directory", NULL);
        
        // 如果 execlp 执行失败，则打印错误信息
        perror("execlp");
        exit(EXIT_FAILURE);
    } else { // 父进程
        int status;
        
        // 关闭管道的写入端
        close(pipe_fd[1]);
        
        // 等待子进程结束
        waitpid(pid, &status, 0);
        
        // 检查子进程是否正常退出
        if (WIFEXITED(status)) {
            printf("Child process exited with status: %d\n", WEXITSTATUS(status));
        } else {
            printf("Child process exited abnormally\n");
        }
        
        // 在父进程中读取子进程的标准错误输出
        char buffer[4096];
        ssize_t bytes_read;
        while ((bytes_read = read(pipe_fd[0], buffer, sizeof(buffer))) > 0) {
            write(STDOUT_FILENO, buffer, bytes_read);
        }
        
        // 关闭管道的读取端
        close(pipe_fd[0]);
    }
    
    return 0;
}