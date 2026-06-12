// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

extern crate alloc;

use crate::crypto::get_keystore_fingerprint;
use crate::crypto::sig::{
    get_build_timestamp, get_key_fingerprint, get_nonos_key_id, init_production_keys,
};
use crate::log::logger::{log_error, log_info};
use crate::security::types::SecurityContext;
use alloc::format;

pub fn load_production_keys(ctx: &mut SecurityContext) -> bool {
    match init_production_keys() {
        Ok(count) => {
            ctx.key_count = count;
            log_key_info();
            true
        }
        Err(_) => {
            ctx.key_count = 0;
            log_error("security", "Failed to load production keys");
            false
        }
    }
}

fn log_key_info() {
    log_info("security", &format!("Key fingerprint: {}", get_key_fingerprint()));
    log_info("security", &format!("Build timestamp: {}", get_build_timestamp()));
    let kid = get_nonos_key_id();
    log_info(
        "security",
        &format!("Key ID: {:02x}{:02x}{:02x}{:02x}", kid[0], kid[1], kid[2], kid[3]),
    );
    log_info("security", &format!("Keystore: {}", get_keystore_fingerprint()));
}
