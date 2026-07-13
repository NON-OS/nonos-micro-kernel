// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Receive-ring index arithmetic. The firmware advances a write index as it
//! posts packets; the driver processes from its read index up to that write
//! index, wrapping at the ring size. Pure math, checked by `iwlwifi_proofs`.

use crate::constants::RX_QUEUE_SIZE;

const MASK: usize = RX_QUEUE_SIZE - 1;

/// Advance the read index by one, wrapping at the ring size.
pub const fn advance(ptr: usize) -> usize {
    (ptr + 1) & MASK
}

/// Whether the ring has a packet for the driver: the read index has not caught
/// the firmware's write index.
pub const fn has_packet(read: usize, write: usize) -> bool {
    (read & MASK) != (write & MASK)
}
