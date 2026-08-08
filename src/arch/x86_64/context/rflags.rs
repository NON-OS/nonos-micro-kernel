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

//! What a saved RFLAGS is allowed to carry back into execution.
//!
//! A context comes off memory a process could have influenced, so the flags
//! are not trusted. IOPL is the reason this matters most: resuming a capsule
//! with IOPL 3 hands it every I/O port on the machine, silently and for good.
//!
//! Both restore paths used to carry their own copy of the mask and the same
//! masking expression. They share this one, and `mechanism_proofs` includes
//! this file directly so what is checked is what runs.

/// Bits a restored context must not choose for itself: TF (8), DF (10),
/// IOPL (12, 13), NT (14), RF (16), VM (17), AC (18), VIF (19), VIP (20).
pub const RFLAGS_PRIVILEGED_MASK: u64 = 0x0000_0000_001F_7500;

/// Bit 1 is reserved and reads as one; a context that cleared it is malformed.
pub const RFLAGS_RESERVED_SET: u64 = 0x0000_0000_0000_0002;

/// IF (9). Set on the way back to user mode, never on a CPL=0 continuation:
/// a yield or preempt saved under interrupt-gate discipline carries IF=0 and
/// must keep it, or the timer can land on a path already holding a scheduler
/// lock and deadlock the core.
pub const RFLAGS_IF: u64 = 0x0000_0000_0000_0200;

/// Drop every privileged bit and restore the reserved one. Keeps IF as saved.
pub const fn sanitize(rflags: u64) -> u64 {
    (rflags & !RFLAGS_PRIVILEGED_MASK) | RFLAGS_RESERVED_SET
}

/// The same, for a resume that lands in user mode with interrupts on.
pub const fn sanitize_user(rflags: u64) -> u64 {
    sanitize(rflags) | RFLAGS_IF
}
