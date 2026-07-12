// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Transmit Frame Descriptor (TFD) ring index arithmetic for a command or data
//! queue. The legacy ring holds `TFD_QUEUE_SIZE` descriptors (a power of two);
//! the write and read pointers advance modulo that size and one slot is kept
//! free, so the queue is full when advancing the write pointer would catch the
//! read pointer. Pure math, checked by `iwlwifi_proofs`.

use crate::constants::TFD_QUEUE_SIZE;

const MASK: usize = TFD_QUEUE_SIZE - 1;

/// Advance a ring pointer by one, wrapping at the queue size.
pub const fn advance(ptr: usize) -> usize {
    (ptr + 1) & MASK
}

/// The number of descriptors outstanding between the read and write pointers.
pub const fn used(write: usize, read: usize) -> usize {
    write.wrapping_sub(read) & MASK
}

/// Whether the queue is full: one more enqueue would catch the read pointer.
pub const fn is_full(write: usize, read: usize) -> bool {
    advance(write) == read
}
