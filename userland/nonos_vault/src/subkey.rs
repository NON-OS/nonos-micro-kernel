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

//! One key per record, derived from the machine key.
//!
//! Sealing everything under the machine key directly means one recovered key
//! opens every record on the machine: the wallet, the settings, whatever is
//! added next. The blast radius of a single mistake is the whole disk.
//!
//! So the machine key is never used to encrypt anything. It is a root that
//! each record derives its own key from, through HKDF-SHA256 with the record's
//! name as the info string. Recovering the key for `settings.theme` tells an
//! attacker nothing about `wallet.account`, because inverting HKDF is what
//! they would have to do to get back to the root.
//!
//! The root is fetched, used and wiped inside one call. It is never held.


/// The label a caller must derive the machine root under, published here so
/// every capsule asks for the same one and a record sealed by any of them is
/// openable by the rest. Versioned: changing it makes every existing record
/// unopenable, so a future change takes a new version rather than an edit.
pub const ROOT_LABEL: &[u8] = b"nonos.vault.root.v1";

/// HKDF's info prefix, so a vault subkey can never collide with a key derived
/// for some other purpose from the same root.
pub(crate) const INFO_PREFIX: &[u8] = b"nonos.vault.record.v1:";

/// Why a record's key could not be produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// The record name is empty or longer than the derivation accepts.
    BadRecord,
}

/// The longest record name. Names are identifiers like `wallet.account`, not
/// paths, and a bound keeps the derivation input a fixed shape.
pub const MAX_RECORD: usize = 64;
