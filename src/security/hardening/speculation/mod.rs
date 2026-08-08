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

//! Speculative-execution mitigations, as the rest of the kernel asks for them.
//!
//! What has to be done is completely different per architecture. x86_64 sets
//! IBRS, STIBP and SSBD through model-specific registers and routes indirect
//! branches through retpoline thunks; aarch64 sets SSBS in PSTATE, relies on
//! CSV2 and CSV3 to say whether the core needs anything at all, and separates
//! contexts with `SB`. Nothing about those two vocabularies maps onto each
//! other, so this boundary carries only the three moments the shared kernel
//! actually cares about: set the machine up at boot, and cross the privilege
//! boundary in either direction.

mod boundary;
mod init;

pub use boundary::{kernel_entry, kernel_exit};
pub use init::init;
