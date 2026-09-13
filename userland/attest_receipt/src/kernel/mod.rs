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

//! The kernel's format sources, compiled into the reader unchanged.
//!
//! A verifier that restates the format it verifies keeps a second copy of the
//! truth, and the two copies are guaranteed to agree only on the day they are
//! written. The failure when they drift is the worst shape available: both
//! sides keep running, receipts keep being produced and handed out, and every
//! one of them fails to verify with nothing to point at. Nobody would suspect
//! the format; they would suspect the machine.
//!
//! So the reader includes the files. `bit()` is the same table the kernel
//! enforces against, `DOMAIN` is the same prefix it folds under, and a change
//! to either breaks this build rather than the field.
//!
//! One thing cannot come along. `Authority` lives inside the kernel's dev-root
//! module and brings its storage with it, so [`crate::decode::Authority`]
//! restates only the two-line byte encoding, and `round_trip_tests` pins that
//! against a fold rather than against a number written down twice.

/// `Capability`, its bit table and its names, from the kernel.
pub mod capability;

/// The domain separator and the entry width, straight from the kernel file
/// that defines them. That file holds nothing else and imports nothing, which
/// is what keeps it includable from out here.
#[path = "../../../../src/security/attest_registry/format.rs"]
pub mod format;

pub use format::{DOMAIN, ENTRY_LEN};
