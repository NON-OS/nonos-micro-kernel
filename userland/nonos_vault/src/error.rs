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

//! Why a record could not be sealed or opened.

use crate::subkey::KeyError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    /// Not a vault record, or a version this build does not know.
    NotAVault,
    /// This machine will not produce the key: no TPM, or a boot state that is
    /// not the one the record belongs to.
    NoKey(KeyError),
    /// A key was produced and did not open these bytes. Either the record was
    /// sealed by another machine or the bytes were altered; the two are not
    /// distinguished, because telling an attacker which would tell them
    /// whether they have the right machine.
    Tampered,
    /// The caller's buffer is the wrong size for what it was asked to do.
    BadLength,
    /// No entropy for a nonce. Nothing is sealed rather than risk repeating
    /// one, which is the single failure this construction does not survive.
    NoEntropy,
}
