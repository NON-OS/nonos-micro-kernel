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

//! The attestation registry, as the kernel runs it.
//!
//! Included from its own `mod.rs`, so the table, the ordering, the fold and
//! the receipt bytes are the kernel's files with their real module shape.
//! `attest_receipt` proves the reader's half against a fold restated in the
//! test; this proves the kernel's half against the kernel's own code.

/// The one kernel type the registry names that cannot come along: the real
/// `Authority` drags in the developer-root table. Only the enum is restated.
pub mod dev_roots;

#[path = "../../../../src/security/attest_registry/mod.rs"]
pub mod attest_registry;
