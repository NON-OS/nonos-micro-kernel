#ifndef NONOS_COMPOSITOR_H
#define NONOS_COMPOSITOR_H

#include "nonos_sys.h"

#define NCMP_MAGIC       0x4E434D50u
#define NCMP_VERSION     1
#define NCMP_HDR_LEN     20
#define OP_SCENE_SUBMIT  0x0002
#define OP_DAMAGE_COMMIT 0x0003
#define OP_SCENE_REMOVE  0x0007
#define OP_DISPLAY_INFO  0x0008

struct display_info {
    u32 width;
    u32 height;
    u32 stride;
    u32 format;
};

static inline u32 nonos_lookup_port(const char *name, u64 len) {
    u32 port = 0;
    u32 pid = 0;
    i64 rc = nonos_sys6(SYS_SERVICE_LOOKUP, (u64)name, len, (u64)&port,
                        (u64)&pid, 0, 0);
    if (rc < 0 || pid == 0)
        return 0;
    return port;
}

static inline void ncmp_header(unsigned char *p, u32 op, u32 rid, u32 plen) {
    put_u32(p, 0, NCMP_MAGIC);
    put_u16(p, 4, NCMP_VERSION);
    put_u16(p, 6, op);
    put_u16(p, 8, 0);
    put_u16(p, 10, 0);
    put_u32(p, 12, rid);
    put_u32(p, 16, plen);
}

static inline i64 ncmp_call(u32 port, const unsigned char *tx, u64 txlen,
                            unsigned char *rx, u64 rxlen) {
    return nonos_sys6(SYS_IPC_CALL, (u64)port, (u64)tx, txlen, (u64)rx,
                      rxlen, 0);
}

static inline int ncmp_status_call(u32 port, const unsigned char *tx, u64 txlen) {
    unsigned char rx[NCMP_HDR_LEN + 4];
    i64 rc = ncmp_call(port, tx, txlen, rx, sizeof(rx));
    if (rc < (i64)sizeof(rx))
        return -1;
    if ((i32)get_u32(rx, NCMP_HDR_LEN) != 0)
        return -2;
    return 0;
}

static inline int ncmp_display_info(u32 port, u32 rid, struct display_info *out) {
    unsigned char tx[NCMP_HDR_LEN];
    unsigned char rx[NCMP_HDR_LEN + 4 + 16];
    ncmp_header(tx, OP_DISPLAY_INFO, rid, 0);
    i64 rc = ncmp_call(port, tx, sizeof(tx), rx, sizeof(rx));
    if (rc < (i64)sizeof(rx))
        return -1;
    if ((i32)get_u32(rx, NCMP_HDR_LEN) != 0)
        return -2;
    out->width = get_u32(rx, NCMP_HDR_LEN + 4);
    out->height = get_u32(rx, NCMP_HDR_LEN + 8);
    out->stride = get_u32(rx, NCMP_HDR_LEN + 12);
    out->format = get_u32(rx, NCMP_HDR_LEN + 16);
    if (out->format != 1u || !out->width || !out->height || !out->stride)
        return -3;
    return 0;
}

#endif
