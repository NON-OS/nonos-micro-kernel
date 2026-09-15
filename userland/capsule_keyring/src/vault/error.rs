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

//! What can go wrong keeping a wallet, told apart by what the user should do.
//!
//! Three cases rather than one because the answers differ: a machine that
//! cannot derive a key needs its boot state looked at, a record that will not
//! open needs the seed phrase, and no entropy needs a retry. Collapsing them
//! into one error would tell somebody with a recoverable problem that their
//! wallet is gone.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultError {
    /// This machine will not produce a key: no TPM, or a boot state that is
    /// not the one the record belongs to.
    NoKey,
    /// A key was produced and did not open these bytes: another machine's
    /// record, or altered bytes.
    Tampered,
    /// No entropy for a nonce, so nothing was sealed.
    NoEntropy,
}

impl From<nonos_vault::VaultError> for VaultError {
    /// Everything that is not a missing key is reported as tampering. The
    /// library distinguishes a bad magic from a failed tag, and that
    /// distinction is not the caller's business: both mean these bytes are
    /// not this machine's record, and saying which would tell somebody
    /// feeding in records how far each one got.
    fn from(e: nonos_vault::VaultError) -> Self {
        match e {
            nonos_vault::VaultError::NoKey(_) => VaultError::NoKey,
            nonos_vault::VaultError::NoEntropy => VaultError::NoEntropy,
            _ => VaultError::Tampered,
        }
    }
}
