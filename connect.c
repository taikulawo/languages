#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <memory.h>
#include <stdio.h>
#include <pthread.h>
#include <semaphore.h>

char *host = "127.0.0.1";
char *listen_at = "0.0.0.0";
int port = 3000;
char *msg = "hello\n";
char *resp = "hi\n";

int start_connect();
void *start_listen(void *p);

int main() {
    sem_t sem;
    sem_init(&sem,0, 0);
    pthread_t id;
    int j = 1;
    pthread_create(&id, NULL, start_listen, &sem);
    sem_wait(&sem);
    int c = start_connect();
    return c;
}

void* start_listen(void *p) {
    sem_t *sem = p;
    char buf[64];
    int fd = -1;
    if ((fd = socket(AF_INET, SOCK_STREAM, 0)) == -1) {

    }
    struct sockaddr_in sin;
    sin.sin_family = AF_INET;
    sin.sin_port = port;
    inet_pton(AF_INET, listen_at,&sin.sin_addr);
    if((bind(fd, (struct sockaddr *)&sin, sizeof(sin))) < 0) {
        printf("server Bind failed\n");
        return NULL;
    }
    if (listen(fd, 1) != 0) {
        printf("server Listen failed\n");
        return NULL;
    }
    // 信号量+1，client解除block
    sem_post(sem);
    int addr_len = sizeof(sin);
    while(1) {
        int server_fd = accept(fd, (struct sockaddr *)&sin, (socklen_t *)&addr_len);
        if(recv(server_fd, buf, sizeof(buf), 0) == -1) {
            printf("server Recv failed\n");
            return NULL;
        }
        printf("recv from client %s", buf);
        int n = -1;
        if ((n = send(server_fd, resp, strlen(resp),0)) == -1) {
            printf("server send failed\n");
            return NULL;
        }
    }
}

int start_connect() {
    char buf[64];
    memset(&buf,0, sizeof(buf));
    int fd = -1;
    if((fd = socket(AF_INET, SOCK_STREAM, 0)) == -1) {

    }
    struct sockaddr_in sin;
    sin.sin_family = AF_INET;
    sin.sin_port = port;
    inet_pton(AF_INET, host, &sin.sin_addr);
    int errno = -1;
    if ((errno = connect(fd, (struct sockaddr *)&sin,sizeof(sin))) < 0) {
        printf("client connect failed\n");
        return -1;
    }
    send(fd, msg, strlen(msg), 0);
    int n = 0;
    while((n = recv(fd, buf, sizeof(buf),0)) > 0) {
        for (int i = 0; i < n; i ++) {
            printf("%c", buf[i]);
        }
    }
}