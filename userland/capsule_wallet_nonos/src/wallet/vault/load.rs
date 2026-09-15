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

//! Reading the blob back.
//!
//! Three outcomes, not two. A machine that has never held a wallet, a machine
//! whose store has not answered yet, and a blob. The middle one is the reason
//! this is not an `Option`: the store is deaf for the first seconds of a boot
//! while it stages packages, and a window that asked in that window would
//! otherwise decide there is no wallet and stop asking.
//!
//! Anything that is not exactly one blob long is refused here rather than sent
//! to the keyring, which keeps a corrupt file from costing a TPM round trip.

use super::format::BLOB_LEN;
use super::load_ops::{open_existing, Opened};
use super::load_read::read_exact;
use super::path::VAULT_PATH;
use super::save_ops::close;

pub enum Stored {
    Blob([u8; BLOB_LEN]),
    /// The store answered and there is no vault, or what is there is not one.
    None,
    /// The store did not answer. Ask again later.
    Unknown,
}

pub fn load_blob() -> Stored {
    let fd = match open_existing(VAULT_PATH) {
        Opened::Fd(fd) => fd,
        Opened::Absent => return Stored::None,
        Opened::Silent => return Stored::Unknown,
    };
    let bytes = read_exact(fd);
    close(fd);
    match bytes {
        Some(b) => Stored::Blob(b),
        /*
         * The file opened, so the store is up and this is its content. A read
         * that comes back the wrong length is a corrupt vault, which is a
         * definite answer and not worth retrying.
         */
        None => Stored::None,
    }
}
