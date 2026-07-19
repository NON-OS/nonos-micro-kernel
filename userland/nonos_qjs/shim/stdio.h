#ifndef _NONOS_STDIO_H
#define _NONOS_STDIO_H
#include <stddef.h>
#include <stdarg.h>
typedef struct _NONOS_FILE FILE;
extern FILE *stdout;
extern FILE *stderr;
int printf(const char *, ...);
int fprintf(FILE *, const char *, ...);
int snprintf(char *, size_t, const char *, ...);
int vsnprintf(char *, size_t, const char *, va_list);
int fputc(int, FILE *);
int putchar(int);
size_t fwrite(const void *, size_t, size_t, FILE *);
int fflush(FILE *);
int vfprintf(FILE *, const char *, va_list);
#endif
