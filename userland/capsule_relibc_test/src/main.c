#include <unistd.h>
#include <stdlib.h>
#include <time.h>
static void emit(const char *s, unsigned n) { write(1, s, n); }
int main(void) {
    char *p = (char *)malloc(1u << 20);
    if (!p) { emit("[RELIBC-TEST] FAIL malloc\n", 26); return 1; }
    for (unsigned i = 0; i < (1u << 20); i += 4096) p[i] = (char)i;
    char *q = (char *)malloc(4096);
    if (!q) { emit("[RELIBC-TEST] FAIL malloc2\n", 27); return 1; }
    free(q); free(p);
    struct timespec a, b;
    clock_gettime(CLOCK_MONOTONIC, &a);
    clock_gettime(CLOCK_MONOTONIC, &b);
    if (b.tv_sec < a.tv_sec) { emit("[RELIBC-TEST] FAIL clock\n", 25); return 1; }
    emit("[RELIBC-TEST] PASS\n", 19);
    return 0;
}
