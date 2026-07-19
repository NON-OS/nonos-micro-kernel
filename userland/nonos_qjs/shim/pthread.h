#ifndef _NONOS_PTHREAD_H
#define _NONOS_PTHREAD_H
#include <stddef.h>
typedef struct { long _; } pthread_mutex_t;
typedef struct { long _; } pthread_cond_t;
typedef struct { long _; } pthread_once_t;
typedef struct { long _; } pthread_mutexattr_t;
typedef struct { long _; } pthread_condattr_t;
typedef struct { long _; } pthread_attr_t;
typedef unsigned long pthread_t;
struct timespec;
#define PTHREAD_MUTEX_INITIALIZER {0}
#define PTHREAD_COND_INITIALIZER {0}
#define PTHREAD_ONCE_INIT {0}
#define PTHREAD_CREATE_DETACHED 1
int pthread_mutex_init(pthread_mutex_t*, const pthread_mutexattr_t*);
int pthread_mutex_destroy(pthread_mutex_t*);
int pthread_mutex_lock(pthread_mutex_t*);
int pthread_mutex_unlock(pthread_mutex_t*);
int pthread_cond_init(pthread_cond_t*, const pthread_condattr_t*);
int pthread_cond_destroy(pthread_cond_t*);
int pthread_cond_signal(pthread_cond_t*);
int pthread_cond_broadcast(pthread_cond_t*);
int pthread_cond_wait(pthread_cond_t*, pthread_mutex_t*);
int pthread_cond_timedwait(pthread_cond_t*, pthread_mutex_t*, const struct timespec*);
int pthread_condattr_init(pthread_condattr_t*);
int pthread_condattr_destroy(pthread_condattr_t*);
int pthread_condattr_setclock(pthread_condattr_t*, int);
int pthread_once(pthread_once_t*, void(*)(void));
int pthread_attr_init(pthread_attr_t*);
int pthread_attr_destroy(pthread_attr_t*);
int pthread_attr_setstacksize(pthread_attr_t*, size_t);
int pthread_attr_setdetachstate(pthread_attr_t*, int);
int pthread_create(pthread_t*, const pthread_attr_t*, void*(*)(void*), void*);
int pthread_join(pthread_t, void**);
int pthread_detach(pthread_t);
int pthread_cond_timedwait_relative_np(pthread_cond_t*, pthread_mutex_t*, const struct timespec*);
#endif
