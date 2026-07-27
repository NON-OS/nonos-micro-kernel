// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #400 physical frame allocator (src/memory/frame_alloc/types/ops.rs). The old
//! allocator fell back to a bump range the phys bitmap already owned, so once
//! the bitmap filled it re-handed live frames. The fix makes phys the sole
//! source: exhaustion returns None, no fallback.

/// Fixed allocator: whatever the bitmap reports is the whole answer.
pub fn fixed_alloc(free: Option<u64>) -> Option<u64> {
    free
}

/// Old allocator: on an empty bitmap it fell through to a bump frame that
/// overlapped the bitmap's own pool.
pub fn old_alloc(free: Option<u64>, bump: u64) -> Option<u64> {
    free.or(Some(bump))
}

/// A bump frame can collide with an already-allocated frame (aliasing).
pub fn bump_aliases(used: &[u64], base: u64, counter: u64) -> bool {
    used.contains(&(base + counter))
}
