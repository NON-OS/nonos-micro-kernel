#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include <string.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <poll.h>

#define EMIT(s) write(1, s, sizeof(s) - 1)

int main(void) {
    int fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) { EMIT("[C-NET] FAIL socket\n"); return 1; }
    if (fcntl(fd, F_SETFL, O_NONBLOCK) < 0) { EMIT("[C-NET] FAIL fcntl\n"); return 1; }

    struct sockaddr_in addr;
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(7);
    addr.sin_addr.s_addr = htonl(0x0A0002C8);

    int rc = -1;
    for (int attempt = 0; attempt < 60; attempt++) {
        rc = connect(fd, (struct sockaddr *)&addr, sizeof(addr));
        if (rc == 0 || errno == EINPROGRESS) break;
        poll(NULL, 0, 500);
    }
    if (rc < 0 && errno != EINPROGRESS) { EMIT("[C-NET] FAIL connect\n"); return 1; }

    struct pollfd pfd;
    pfd.fd = fd; pfd.events = POLLOUT; pfd.revents = 0;
    if (poll(&pfd, 1, 4000) <= 0 || !(pfd.revents & POLLOUT)) { EMIT("[C-NET] FAIL connect-wait\n"); return 1; }

    if (write(fd, "ping", 4) != 4) { EMIT("[C-NET] FAIL send\n"); return 1; }

    pfd.events = POLLIN; pfd.revents = 0;
    if (poll(&pfd, 1, 4000) <= 0 || !(pfd.revents & POLLIN)) { EMIT("[C-NET] FAIL recv-wait\n"); return 1; }

    char buf[8];
    for (int i = 0; i < 8; i++) buf[i] = 0;
    long n = read(fd, buf, 4);
    if (n != 4 || memcmp(buf, "ping", 4) != 0) { EMIT("[C-NET] FAIL echo\n"); return 1; }

    close(fd);
    EMIT("[C-NET] PASS\n");
    return 0;
}
