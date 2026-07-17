/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
 * SPDX-License-Identifier: AGPL-3.0
 *
 * C ABI symbols QuickJS links against that are natural to write in C:
 * single-threaded pthread no-ops, zeroed time sources, and a minimal
 * integer/string vsnprintf. The engine runs one context per capsule, so the
 * locking primitives never contend; Date is driven from host time later.
 */
#include <stddef.h>
#include <stdint.h>
#include <stdarg.h>

int errno = 0;
void *stdout = 0;
void *stderr = 0;

/* pthread: single-threaded no-ops */
int pthread_mutex_init(void *a, const void *b) { (void)a; (void)b; return 0; }
int pthread_mutex_destroy(void *a) { (void)a; return 0; }
int pthread_mutex_lock(void *a) { (void)a; return 0; }
int pthread_mutex_unlock(void *a) { (void)a; return 0; }
int pthread_cond_init(void *a, const void *b) { (void)a; (void)b; return 0; }
int pthread_cond_destroy(void *a) { (void)a; return 0; }
int pthread_cond_signal(void *a) { (void)a; return 0; }
int pthread_cond_broadcast(void *a) { (void)a; return 0; }
int pthread_cond_wait(void *a, void *b) { (void)a; (void)b; return 0; }
int pthread_cond_timedwait(void *a, void *b, const void *c) { (void)a; (void)b; (void)c; return 0; }
int pthread_condattr_init(void *a) { (void)a; return 0; }
int pthread_condattr_destroy(void *a) { (void)a; return 0; }
int pthread_condattr_setclock(void *a, int b) { (void)a; (void)b; return 0; }
int pthread_once(void *a, void (*f)(void)) { (void)a; if (f) f(); return 0; }

/* time: zeroed sources; a host clock feeds Date later */
struct timespec { long tv_sec; long tv_nsec; };
struct timeval { long tv_sec; long tv_usec; };
struct tm { int s, m, h, md, mo, y, wd, yd, dst; long gmtoff; const char *zone; };
int clock_gettime(int id, struct timespec *t) { (void)id; if (t) { t->tv_sec = 0; t->tv_nsec = 0; } return 0; }
int gettimeofday(struct timeval *t, void *tz) { (void)tz; if (t) { t->tv_sec = 0; t->tv_usec = 0; } return 0; }
struct tm *localtime_r(const long *t, struct tm *r) { (void)t; if (r) { for (unsigned i = 0; i < sizeof(*r); i++) ((char *)r)[i] = 0; } return r; }

/* output: only formatted-to-buffer is functional; stream output is a sink */
static void emit(char *buf, size_t size, size_t *pos, char c) {
    if (*pos + 1 < size) buf[*pos] = c;
    (*pos)++;
}

/* Emit at most `prec` characters of `s` (prec < 0 means until NUL). Honouring
 * precision is essential: QuickJS passes %.*s with bounded, non-NUL-terminated
 * buffers, so ignoring it reads off the end of the allocation. */
static void emit_str_n(char *buf, size_t size, size_t *pos, const char *s, int prec) {
    if (!s) s = "(null)";
    for (int i = 0; s[i] && (prec < 0 || i < prec); i++) emit(buf, size, pos, s[i]);
}

static void emit_uint_w(char *buf, size_t size, size_t *pos, unsigned long long v,
                        int base, int upper, int width, int zero) {
    char tmp[24];
    const char *digits = upper ? "0123456789ABCDEF" : "0123456789abcdef";
    int n = 0;
    if (v == 0) tmp[n++] = '0';
    while (v) { tmp[n++] = digits[v % base]; v /= base; }
    for (int p = width - n; p > 0; p--) emit(buf, size, pos, zero ? '0' : ' ');
    while (n) emit(buf, size, pos, tmp[--n]);
}

static void emit_int_w(char *buf, size_t size, size_t *pos, long long v, int width, int zero) {
    if (v < 0) {
        emit(buf, size, pos, '-');
        emit_uint_w(buf, size, pos, (unsigned long long)(-v), 10, 0, width > 0 ? width - 1 : 0, zero);
    } else {
        emit_uint_w(buf, size, pos, (unsigned long long)v, 10, 0, width, zero);
    }
}

int vsnprintf(char *buf, size_t size, const char *fmt, va_list ap) {
    size_t pos = 0;
    for (const char *f = fmt; *f; f++) {
        if (*f != '%') { emit(buf, size, &pos, *f); continue; }
        f++;
        int zero = 0;
        while (*f == '-' || *f == '+' || *f == ' ' || *f == '#' || *f == '0') {
            if (*f == '0') zero = 1;
            f++;
        }
        int width = 0;
        if (*f == '*') { width = va_arg(ap, int); f++; }
        else while (*f >= '0' && *f <= '9') { width = width * 10 + (*f - '0'); f++; }
        int prec = -1;
        if (*f == '.') {
            f++;
            prec = 0;
            if (*f == '*') { prec = va_arg(ap, int); f++; }
            else while (*f >= '0' && *f <= '9') { prec = prec * 10 + (*f - '0'); f++; }
        }
        int lng = 0;
        while (*f == 'l' || *f == 'z' || *f == 'j' || *f == 't' || *f == 'h') {
            if (*f == 'l') lng++;
            f++;
        }
        long long sv;
        unsigned long long uv;
        switch (*f) {
            case '%': emit(buf, size, &pos, '%'); break;
            case 'c': emit(buf, size, &pos, (char)va_arg(ap, int)); break;
            case 's': emit_str_n(buf, size, &pos, va_arg(ap, const char *), prec); break;
            case 'd': case 'i':
                sv = lng >= 2 ? va_arg(ap, long long)
                             : (lng == 1 ? (long long)va_arg(ap, long) : (long long)va_arg(ap, int));
                emit_int_w(buf, size, &pos, sv, width, zero);
                break;
            case 'u': case 'x': case 'X': case 'o':
                uv = lng >= 2 ? va_arg(ap, unsigned long long)
                             : (lng == 1 ? (unsigned long long)va_arg(ap, unsigned long)
                                         : (unsigned long long)va_arg(ap, unsigned int));
                emit_uint_w(buf, size, &pos, uv, *f == 'o' ? 8 : (*f == 'u' ? 10 : 16), *f == 'X', width, zero);
                break;
            case 'p':
                emit_str_n(buf, size, &pos, "0x", -1);
                emit_uint_w(buf, size, &pos, (unsigned long long)(uintptr_t)va_arg(ap, void *), 16, 0, 0, 0);
                break;
            case 'f': case 'g': case 'e': {
                double d = va_arg(ap, double);
                long long ip = (long long)d;
                emit_int_w(buf, size, &pos, ip, 0, 0);
                emit(buf, size, &pos, '.');
                double frac = d < 0 ? (double)ip - d : d - (double)ip;
                int after = prec >= 0 ? prec : 6;
                for (int k = 0; k < after; k++) { frac *= 10; int dig = (int)frac; emit(buf, size, &pos, '0' + (dig % 10)); frac -= dig; }
                break;
            }
            default: emit(buf, size, &pos, '%'); if (*f) emit(buf, size, &pos, *f); break;
        }
    }
    if (size) buf[pos < size ? pos : size - 1] = 0;
    return (int)pos;
}

int snprintf(char *buf, size_t size, const char *fmt, ...) {
    va_list ap; va_start(ap, fmt);
    int n = vsnprintf(buf, size, fmt, ap);
    va_end(ap);
    return n;
}

int printf(const char *f, ...) { (void)f; return 0; }
int fprintf(void *s, const char *f, ...) { (void)s; (void)f; return 0; }
int vfprintf(void *s, const char *f, va_list ap) { (void)s; (void)f; (void)ap; return 0; }
int fputc(int c, void *s) { (void)s; return c; }
size_t fwrite(const void *p, size_t a, size_t b, void *s) { (void)p; (void)s; return a * b; }
int fflush(void *s) { (void)s; return 0; }
