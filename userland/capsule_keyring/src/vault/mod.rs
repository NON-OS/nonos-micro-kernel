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

//! Wallet secrets that survive a reboot without being stored in the clear.
//!
//! The sealing itself is `nonos_vault`, shared with every other capsule that
//! keeps something. This module is the keyring's half of it: fetching the
//! machine root, naming the record, and drawing the nonce from the entropy
//! this capsule already gathers.
//!
//! It used to be its own implementation, sealing under the machine key
//! directly. Two differences came out of the move. There is now one sealing
//! format on the machine rather than two that could drift apart. And the key
//! is derived per record, so recovering the wallet's key does not open the
//! settings, which sealing under the machine key directly would have allowed.

mod error;
mod record;
mod root;
mod wipe;

pub use error::VaultError;
pub use record::{open_secret, seal_secret, BLOB_LEN};
