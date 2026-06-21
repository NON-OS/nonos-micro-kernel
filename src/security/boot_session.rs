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

//! Per-boot 128-bit authority nonce.
//!
//! Latched once during boot after the RNG is ready and minted into
//! every authority-bearing capability token. The resolver compares
//! the token's bound nonce against the live value to reject any
//! token carried across a reboot. The API returns `Option` so mint
//! sites are forced to handle the not-yet-initialized case; the
//! production boot path halts if init fails (see `core_init.rs`).
//! Explicit empty tokens (`CapabilityToken::empty`,
//! `CapabilityToken::with_caps`) bind a literal zero nonce and do
//! not go through this API.

use spin::Once;

static BOOT_SESSION_NONCE: Once<[u8; 16]> = Once::new();

pub fn init_once_from_rng() -> Result<(), &'static str> {
    if BOOT_SESSION_NONCE.get().is_some() {
        return Err("boot session nonce already latched");
    }
    let mut buf = [0u8; 16];
    crate::crypto::random_api::get_bytes_secure(&mut buf)
        .map_err(|_| "boot session nonce: RNG not ready")?;
    BOOT_SESSION_NONCE.call_once(|| buf);
    Ok(())
}

#[inline]
pub fn nonce() -> Option<[u8; 16]> {
    BOOT_SESSION_NONCE.get().copied()
}

#[inline]
pub fn is_initialized() -> bool {
    BOOT_SESSION_NONCE.get().is_some()
}
