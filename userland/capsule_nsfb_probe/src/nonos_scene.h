#ifndef NONOS_SCENE_H
#define NONOS_SCENE_H

#include "nonos_compositor.h"

static inline int ncmp_scene_submit(u32 port, u32 rid, u64 handle, u32 x,
                                    u32 y, u32 w, u32 h, u32 z) {
    unsigned char tx[NCMP_HDR_LEN + 32];
    ncmp_header(tx, OP_SCENE_SUBMIT, rid, 32);
    put_u64(tx, 20, handle);
    put_u32(tx, 28, x);
    put_u32(tx, 32, y);
    put_u32(tx, 36, w);
    put_u32(tx, 40, h);
    put_u32(tx, 44, z);
    put_u32(tx, 48, 0);
    return ncmp_status_call(port, tx, sizeof(tx));
}

static inline int ncmp_damage_commit(u32 port, u32 rid, u32 x, u32 y, u32 w, u32 h) {
    unsigned char tx[NCMP_HDR_LEN + 16];
    ncmp_header(tx, OP_DAMAGE_COMMIT, rid, 16);
    put_u32(tx, 20, x);
    put_u32(tx, 24, y);
    put_u32(tx, 28, w);
    put_u32(tx, 32, h);
    return ncmp_status_call(port, tx, sizeof(tx));
}

static inline int ncmp_scene_remove(u32 port, u32 rid) {
    unsigned char tx[NCMP_HDR_LEN + 8];
    ncmp_header(tx, OP_SCENE_REMOVE, rid, 8);
    put_u32(tx, 20, 0);
    put_u32(tx, 24, 0);
    return ncmp_status_call(port, tx, sizeof(tx));
}

#endif
