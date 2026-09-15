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

//! Host proofs for the TPM machine-key derivation. Includes the kernel's own
//! wire code via `#[path]` so the bytes under test are the bytes that ship.
//!
//! The derivation is six commands whose framing has to be right to the byte,
//! and the TPM's answer to a wrong byte is a response code with no further
//! help. These pin each command against the specification, and, where a
//! software TPM is installed, run the sequence for real.

extern crate alloc;

/// The kernel's module tree, enough of it for `crate::security::tpm::...` to
/// resolve to the shipping files.
#[cfg(test)]
pub mod security;
#[cfg(test)]
pub use security::tpm::machine_key;
