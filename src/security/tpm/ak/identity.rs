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


//! The attestation key's public point, kept from the moment the key is loaded.
//!
//! A verifier needs it to check a quote's signature, and the machine is the
//! only place it exists. It is not secret: it is what the document carries so
//! a counterparty can pin this machine across boots.

use spin::Mutex;

static AK_PUBLIC: Mutex<Option<[u8; 64]>> = Mutex::new(None);

pub(super) fn remember(point: [u8; 64]) {
    *AK_PUBLIC.lock() = Some(point);
}

/// The loaded key's uncompressed P-256 point, or `None` before bring-up.
pub fn ak_public() -> Option<[u8; 64]> {
    *AK_PUBLIC.lock()
}
