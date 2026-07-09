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

//! Host-runnable proofs for the bootloader's anti-rollback logic. The real
//! `security::anti_rollback` decision code is pulled in via `#[path]` and run
//! directly; only the TPM/NVRAM write is shimmed, so the invariants are proven
//! about the code that actually gates a kernel boot.

pub mod image_format;
pub mod security;

#[cfg(test)]
mod antirollback_tests;
#[cfg(test)]
mod image_format_tests;

#[cfg(kani)]
mod kani_proofs;
