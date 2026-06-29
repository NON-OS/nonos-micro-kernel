#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define EMIT(s) write(1, s, sizeof(s) - 1)

int main(void) {
    char *p = (char *)malloc(1u << 20);
    if (!p) { EMIT("[RELIBC-TEST] FAIL malloc\n"); return 1; }
    for (unsigned i = 0; i < (1u << 20); i += 4096) p[i] = (char)i;
    char *q = (char *)malloc(4096);
    if (!q) { EMIT("[RELIBC-TEST] FAIL malloc2\n"); return 1; }
    free(q);
    free(p);

    struct timespec a, b;
    clock_gettime(CLOCK_MONOTONIC, &a);
    clock_gettime(CLOCK_MONOTONIC, &b);
    if (b.tv_sec < a.tv_sec) { EMIT("[RELIBC-TEST] FAIL clock\n"); return 1; }

    FILE *f = fopen("/relibctest.txt", "w");
    if (!f) { EMIT("[RELIBC-TEST] FAIL fopen-w\n"); return 1; }
    if (fwrite("hello", 1, 5, f) != 5) { EMIT("[RELIBC-TEST] FAIL fwrite\n"); return 1; }
    fclose(f);

    f = fopen("/relibctest.txt", "r");
    if (!f) { EMIT("[RELIBC-TEST] FAIL fopen-r\n"); return 1; }
    char buf[8];
    for (int i = 0; i < 8; i++) buf[i] = 0;
    size_t n = fread(buf, 1, 5, f);
    fclose(f);
    if (n != 5 || buf[0] != 'h' || buf[4] != 'o') { EMIT("[RELIBC-TEST] FAIL fread\n"); return 1; }

    EMIT("[RELIBC-TEST] PASS\n");
    return 0;
}
