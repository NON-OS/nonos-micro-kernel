#ifndef _NONOS_TIME_H
#define _NONOS_TIME_H
#include <stdint.h>
typedef long time_t;
struct timespec { time_t tv_sec; long tv_nsec; };
struct timeval { time_t tv_sec; long tv_usec; };
struct tm { int tm_sec,tm_min,tm_hour,tm_mday,tm_mon,tm_year,tm_wday,tm_yday,tm_isdst; long tm_gmtoff; const char *tm_zone; };
int clock_gettime(int, struct timespec *);
int gettimeofday(struct timeval *, void *);
struct tm *localtime_r(const time_t *, struct tm *);
struct tm *gmtime_r(const time_t *, struct tm *);
time_t time(time_t *);
#define CLOCK_MONOTONIC 1
#define CLOCK_REALTIME 0
#endif
