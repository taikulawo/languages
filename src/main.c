#include <stdio.h>
#include <string.h>

int main() {
    char *url = "https://127.0.0.1:443/v1/lookup/name?name=a.b.c";
    char *p = strchr(url, ':');
    int port = -1;
    sscanf(p, "%d", &port);
    printf("%d", port);
    return 0;
}