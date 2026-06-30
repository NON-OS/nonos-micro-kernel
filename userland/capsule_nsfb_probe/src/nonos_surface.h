#ifndef NONOS_SURFACE_H
#define NONOS_SURFACE_H

#include <stddef.h>
#include "nonos_sys.h"

#define PROT_RW                 0x3
#define MAP_PRIVATE_ANON        0x22
#define SURFACE_FORMAT_ARGB8888 1u

struct surface_descriptor {
    u32 width;
    u32 height;
    u32 stride;
    u32 format;
    u64 byte_len;
    u64 base_va;
    u64 flags;
};
_Static_assert(sizeof(struct surface_descriptor) == 40,
               "surface_descriptor size must be 40 bytes");
_Static_assert(offsetof(struct surface_descriptor, byte_len) == 16,
               "surface_descriptor byte_len must be at offset 16");
_Static_assert(offsetof(struct surface_descriptor, base_va) == 24,
               "surface_descriptor base_va must be at offset 24");
_Static_assert(offsetof(struct surface_descriptor, flags) == 32,
               "surface_descriptor flags must be at offset 32");

static inline u64 nonos_mmap(u64 len) {
    return (u64)nonos_sys6(SYS_MMAP, 0, len, PROT_RW, MAP_PRIVATE_ANON,
                           (u64)(-1), 0);
}

static inline i64 nonos_surface_register(const struct surface_descriptor *d) {
    return nonos_sys6(SYS_SURFACE_REGISTER, (u64)d, 0, 0, 0, 0, 0);
}

static inline i64 nonos_surface_share(u64 sid) {
    return nonos_sys6(SYS_SURFACE_SHARE, sid, 0, 0, 0, 0, 0);
}

static inline i64 nonos_surface_release(u64 handle) {
    return nonos_sys6(SYS_SURFACE_RELEASE, handle, 0, 0, 0, 0, 0);
}

#endif
