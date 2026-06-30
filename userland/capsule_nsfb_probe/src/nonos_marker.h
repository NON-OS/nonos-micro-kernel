#ifndef NONOS_MARKER_H
#define NONOS_MARKER_H

#include <unistd.h>

#include "nonos_sys.h"

static inline u64 cstr_len(const char *s) {
    u64 n = 0;
    while (s[n])
        n++;
    return n;
}

static inline void emit(const char *s) {
    write(1, s, cstr_len(s));
}

static inline u64 fmt_i64(char *buf, i64 v) {
    char tmp[24];
    u64 i = 0;
    u64 neg = v < 0;
    u64 u = neg ? (u64)(-v) : (u64)v;
    do {
        tmp[i++] = (char)('0' + (u % 10));
        u /= 10;
    } while (u);
    u64 n = 0;
    if (neg)
        buf[n++] = '-';
    while (i)
        buf[n++] = tmp[--i];
    buf[n] = 0;
    return n;
}

static inline void emit_fail(const char *step, i64 rc) {
    char num[24];
    fmt_i64(num, rc);
    emit("[NSFB-PROBE] FAIL:");
    emit(step);
    emit(":");
    emit(num);
    emit("\n");
}

#endif
