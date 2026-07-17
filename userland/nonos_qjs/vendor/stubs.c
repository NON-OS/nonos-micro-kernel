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

static void emit_str(char *buf, size_t size, size_t *pos, const char *s) {
    if (!s) s = "(null)";
    while (*s) emit(buf, size, pos, *s++);
}

static void emit_uint(char *buf, size_t size, size_t *pos, unsigned long long v, int base, int upper) {
    char tmp[24];
    const char *digits = upper ? "0123456789ABCDEF" : "0123456789abcdef";
    int n = 0;
    if (v == 0) tmp[n++] = '0';
    while (v) { tmp[n++] = digits[v % base]; v /= base; }
    while (n) emit(buf, size, pos, tmp[--n]);
}

static void emit_int(char *buf, size_t size, size_t *pos, long long v, int base) {
    if (v < 0) { emit(buf, size, pos, '-'); emit_uint(buf, size, pos, (unsigned long long)(-v), base, 0); }
    else emit_uint(buf, size, pos, (unsigned long long)v, base, 0);
}

int vsnprintf(char *buf, size_t size, const char *fmt, va_list ap) {
    size_t pos = 0;
    for (const char *f = fmt; *f; f++) {
        if (*f != '%') { emit(buf, size, &pos, *f); continue; }
        f++;
        while (*f == '-' || *f == '+' || *f == ' ' || *f == '#' || *f == '0') f++;
        while ((*f >= '0' && *f <= '9') || *f == '.' || *f == '*') { if (*f == '*') (void)va_arg(ap, int); f++; }
        int lng = 0;
        while (*f == 'l' || *f == 'z' || *f == 'j' || *f == 't' || *f == 'h') { if (*f == 'l') lng++; f++; }
        switch (*f) {
            case '%': emit(buf, size, &pos, '%'); break;
            case 'c': emit(buf, size, &pos, (char)va_arg(ap, int)); break;
            case 's': emit_str(buf, size, &pos, va_arg(ap, const char *)); break;
            case 'd': case 'i':
                emit_int(buf, size, &pos, lng >= 2 ? va_arg(ap, long long) : (long long)va_arg(ap, long), 10); break;
            case 'u':
                emit_uint(buf, size, &pos, lng >= 2 ? va_arg(ap, unsigned long long) : (unsigned long long)va_arg(ap, unsigned long), 10, 0); break;
            case 'x':
                emit_uint(buf, size, &pos, lng >= 2 ? va_arg(ap, unsigned long long) : (unsigned long long)va_arg(ap, unsigned long), 16, 0); break;
            case 'X':
                emit_uint(buf, size, &pos, lng >= 2 ? va_arg(ap, unsigned long long) : (unsigned long long)va_arg(ap, unsigned long), 16, 1); break;
            case 'o':
                emit_uint(buf, size, &pos, lng >= 2 ? va_arg(ap, unsigned long long) : (unsigned long long)va_arg(ap, unsigned long), 8, 0); break;
            case 'p':
                emit_str(buf, size, &pos, "0x");
                emit_uint(buf, size, &pos, (unsigned long long)(uintptr_t)va_arg(ap, void *), 16, 0); break;
            case 'f': case 'g': case 'e': {
                double d = va_arg(ap, double);
                long long ip = (long long)d;
                emit_int(buf, size, &pos, ip, 10);
                emit(buf, size, &pos, '.');
                double frac = d < 0 ? ip - d : d - ip;
                for (int k = 0; k < 6; k++) { frac *= 10; int dig = (int)frac; emit(buf, size, &pos, '0' + (dig % 10)); frac -= dig; }
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
