#include "nonos_marker.h"
#include "nonos_surface.h"
#include "nonos_scene.h"

#define SCENE_Z 1000u
#define ACQUIRE_ATTEMPTS 60u
#define ACQUIRE_YIELDS 256u

static void hang(void) {
    for (;;)
        nonos_yield();
}

static void acquire_delay(void) {
    for (u32 i = 0; i < ACQUIRE_YIELDS; i++)
        nonos_yield();
}

static u32 acquire_compositor(struct display_info *di, i64 *last_rc) {
    *last_rc = 0;
    for (u32 attempt = 0; attempt < ACQUIRE_ATTEMPTS; attempt++) {
        u32 port = nonos_lookup_port("compositor", 10);
        if (port) {
            i64 rc = ncmp_display_info(port, 1, di);
            if (rc == 0)
                return port;
            *last_rc = rc;
        }
        acquire_delay();
    }
    return 0;
}

static void paint_gradient(u64 base, u32 w, u32 h, u32 stride) {
    volatile u32 *px = (volatile u32 *)base;
    u32 spx = stride / 4u;
    for (u32 y = 0; y < h; y++) {
        for (u32 x = 0; x < w; x++) {
            u32 r = (x * 255u) / w;
            u32 g = (y * 255u) / h;
            px[(u64)y * spx + x] = 0xFF000000u | (r << 16) | (g << 8);
        }
    }
}

int main(void) {
    struct display_info di;
    i64 rc = 0;
    u32 port = acquire_compositor(&di, &rc);
    if (!port) {
        if (rc == 0)
            emit_fail("lookup", 0);
        else
            emit_fail("display_info", rc);
        hang();
    }

    u64 byte_len = (u64)di.stride * (u64)di.height;
    u64 base = nonos_mmap(byte_len);
    if (!base || (base & 0xFFFu)) { emit_fail("mmap", (i64)base); hang(); }

    struct surface_descriptor desc;
    desc.width = di.width;
    desc.height = di.height;
    desc.stride = di.stride;
    desc.format = SURFACE_FORMAT_ARGB8888;
    desc.byte_len = byte_len;
    desc.base_va = base;
    desc.flags = 0;

    i64 sid = nonos_surface_register(&desc);
    if (sid < 0) { emit_fail("register", sid); hang(); }

    i64 handle = nonos_surface_share((u64)sid);
    if (handle <= 0) { emit_fail("share", handle); hang(); }

    paint_gradient(base, di.width, di.height, di.stride);

    rc = ncmp_scene_submit(port, 2, (u64)handle, 0, 0, di.width, di.height, SCENE_Z);
    if (rc != 0) { emit_fail("submit", rc); hang(); }

    rc = ncmp_damage_commit(port, 3, 0, 0, di.width, di.height);
    if (rc != 0) { emit_fail("damage", rc); hang(); }

    emit("[NSFB-PROBE] PASS\n");
    hang();
    return 0;
}
