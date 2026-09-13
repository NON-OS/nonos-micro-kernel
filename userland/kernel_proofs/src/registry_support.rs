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

//! Shared by the registry tests. The table is one global, so every test
//! holds `TABLE_IN_USE` and leaves the table as it found it.

use std::sync::Mutex;

use crate::security::attest_registry::{record_attested, DOMAIN, ENTRY_LEN};
use crate::security::dev_roots::Authority;

pub(crate) static TABLE_IN_USE: Mutex<()> = Mutex::new(());

/// The fold as the receipt reader does it, from one byte string.
pub(crate) fn refold(entries: &[u8]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(DOMAIN);
    h.update(&((entries.len() / ENTRY_LEN) as u32).to_be_bytes());
    h.update(entries);
    *h.finalize().as_bytes()
}

pub(crate) fn record(pid: u32, fill: u8, caps: u64, authority: Authority) {
    record_attested(pid, [fill; 32], caps, authority);
}
