#ifndef NONOS_SYS_H
#define NONOS_SYS_H

typedef unsigned int u32;
typedef int i32;
typedef unsigned long u64;
typedef long i64;

#define FOURCC(a, b, c, d) \
    ((i64)((unsigned char)(a) | ((unsigned char)(b) << 8) | \
           ((unsigned char)(c) << 16) | ((i64)(unsigned char)(d) << 24)))

#define SYS_MMAP             FOURCC('M', 'M', 'A', 'P')
#define SYS_SURFACE_REGISTER FOURCC('M', 'S', 'R', 'G')
#define SYS_SURFACE_SHARE    FOURCC('M', 'S', 'S', 'H')
#define SYS_SURFACE_RELEASE  FOURCC('M', 'S', 'R', 'L')
#define SYS_SERVICE_LOOKUP   FOURCC('M', 'S', 'V', 'L')
#define SYS_IPC_CALL         FOURCC('M', 'I', 'C', 'L')
#define SYS_YIELD            FOURCC('M', 'Y', 'L', 'D')

static inline i64 nonos_sys6(i64 num, u64 a1, u64 a2, u64 a3, u64 a4, u64 a5, u64 a6) {
    register i64 rax __asm__("rax") = num;
    register u64 rdi __asm__("rdi") = a1;
    register u64 rsi __asm__("rsi") = a2;
    register u64 rdx __asm__("rdx") = a3;
    register u64 r10 __asm__("r10") = a4;
    register u64 r8 __asm__("r8") = a5;
    register u64 r9 __asm__("r9") = a6;
    __asm__ volatile("syscall"
                     : "+r"(rax), "+r"(rdi), "+r"(rsi), "+r"(rdx),
                       "+r"(r10), "+r"(r8), "+r"(r9)
                     :
                     : "rcx", "r11", "memory");
    return rax;
}

static inline void nonos_yield(void) {
    nonos_sys6(SYS_YIELD, 0, 0, 0, 0, 0, 0);
}

static inline void put_u16(unsigned char *p, u32 off, u32 v) {
    p[off] = (unsigned char)(v & 0xFF);
    p[off + 1] = (unsigned char)((v >> 8) & 0xFF);
}

static inline void put_u32(unsigned char *p, u32 off, u32 v) {
    put_u16(p, off, v & 0xFFFF);
    put_u16(p, off + 2, (v >> 16) & 0xFFFF);
}

static inline void put_u64(unsigned char *p, u32 off, u64 v) {
    put_u32(p, off, (u32)v);
    put_u32(p, off + 4, (u32)(v >> 32));
}

static inline u32 get_u32(const unsigned char *p, u32 off) {
    return (u32)p[off] | ((u32)p[off + 1] << 8) |
           ((u32)p[off + 2] << 16) | ((u32)p[off + 3] << 24);
}

#endif
