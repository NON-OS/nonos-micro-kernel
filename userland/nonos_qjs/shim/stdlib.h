#ifndef _NONOS_STDLIB_H
#define _NONOS_STDLIB_H
#include <stddef.h>
void *malloc(size_t);
void *calloc(size_t, size_t);
void *realloc(void *, size_t);
void free(void *);
void abort(void) __attribute__((noreturn));
void exit(int) __attribute__((noreturn));
double strtod(const char *, char **);
long strtol(const char *, char **, int);
unsigned long strtoul(const char *, char **, int);
int abs(int);
#endif

#define PATH_MAX 4096
char *realpath(const char *, char *);
