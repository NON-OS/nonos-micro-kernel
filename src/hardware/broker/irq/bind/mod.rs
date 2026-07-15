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

// x86-only: IO-APIC redirect (INTx) + PCI MSI-X capability program.
#![cfg(target_arch = "x86_64")]

//! `MkIrqBind` core. Two paths share the same syscall:
//!
//!   * INTx (default, `flags == 0`) — programs the IO-APIC, returns
//!     a single grant. Behaviour is bit-identical to the pre-MSI-X
//!     version of the broker.
//!   * MSI-X (`flags & BIND_MSIX != 0`) — kernel walks the
//!     capability list, validates the table BAR against the
//!     claimed device, allocates `vector_count` contiguous broker
//!     vectors, programs that many MSI-X table entries with the
//!     LAPIC redirect, enables MSI-X, then unmasks each entry. The
//!     capsule never sees the table address and never writes to
//!     it; it only receives the base grant id and base vector.
//!
//! All hardware-touching steps go through the `MsixOps` indirection.

mod bind;
mod handle_view;
mod intx;
mod msix;
mod teardown;

pub use bind::bind;
pub(super) use teardown::{disable_msix_for_device, teardown_msix_vector};
