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

//! Keeping a wallet, and finding it again next boot.
//!
//! Two calls, at the two moments that matter: after a wallet comes into
//! existence, and when a fresh window has none. Everything under them is
//! best-effort by design. A machine with no TPM, or a store still staging
//! packages, must not stop the wallet working for this session; it only means
//! the wallet is for this session.

use nonos_libc::mk_time_millis;

use super::keyring::{open_vault, seal_vault};
use super::load::{load_blob, Stored};
use super::save::save_blob;

/// A year, matching the expiry every other keyring entry is stored with.
const LIFETIME_MS: u64 = 31_536_000_000;

/// Seal the wallet to this machine and put it on the disk. False when there
/// was no machine key to seal under, or the store would not take it, which
/// the caller reports without treating the wallet as broken.
pub fn remember(port: u32, owner_pid: u32, wallet_id: u32) -> bool {
    match seal_vault(port, owner_pid, wallet_id) {
        Ok(blob) => save_blob(&blob),
        Err(_) => false,
    }
}

/// What a restore found.
pub enum Recall {
    /// No vault on this disk, which is every machine that has never held a
    /// wallet and must not read as a failure.
    Nothing,
    /// The store has not answered yet. It stages packages for the first
    /// seconds of a boot and is deaf while it does, so this is the ordinary
    /// state of an early window and the caller must ask again rather than
    /// conclude the machine has no wallet.
    NotYet,
    /// Opened, and this is the wallet.
    Wallet(u32),
    /// A vault is there and this machine will not open it. The keyring says
    /// which: ENOENT when it could not derive a key, so the machine changed
    /// or has no TPM, and anything else when a key was derived and did not
    /// fit, so the vault belongs to another machine.
    Sealed { machine_changed: bool },
}

pub fn recall(port: u32, owner_pid: u32) -> Recall {
    let blob = match load_blob() {
        Stored::Blob(b) => b,
        Stored::None => return Recall::Nothing,
        Stored::Unknown => return Recall::NotYet,
    };
    let now = mk_time_millis().max(0) as u64;
    match open_vault(port, owner_pid, now, now.saturating_add(LIFETIME_MS), &blob) {
        Ok(id) => Recall::Wallet(id),
        Err(-2) => Recall::Sealed { machine_changed: true },
        Err(_) => Recall::Sealed { machine_changed: false },
    }
}
