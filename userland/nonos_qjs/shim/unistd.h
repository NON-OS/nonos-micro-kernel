#ifndef _NONOS_UNISTD_H
#define _NONOS_UNISTD_H
#include <stddef.h>
typedef long ssize_t;
ssize_t write(int, const void *, size_t);
int getpid(void);
#endif
