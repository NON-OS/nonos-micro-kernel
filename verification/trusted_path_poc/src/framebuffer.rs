// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #404 present-blit channel order
//! (src/syscall/dispatch/router/graphics_present/blit.rs). Surfaces store
//! B,G,R,A. RGB firmware needs the red/blue swap once; the bug ran it twice, so
//! the swaps cancelled and RGB scanned out reversed.

/// Exchange byte 0 and byte 2 of a 4-byte pixel (the red/blue swap).
pub fn swap02(p: [u8; 4]) -> [u8; 4] {
    [p[2], p[1], p[0], p[3]]
}

/// The correct path: swap once for RGB firmware, not at all for BGR.
pub fn present(bgr: bool, p: [u8; 4]) -> [u8; 4] {
    if bgr {
        p
    } else {
        swap02(p)
    }
}
