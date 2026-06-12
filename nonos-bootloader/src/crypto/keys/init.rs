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

use super::state::{INIT_DONE, KEYSTORE, KEY_VERSION, NONOS_KEY_ID, NONOS_PUBLIC_KEY};
use super::util::{constant_time_eq, is_zero_key};
use crate::log::logger::{log_error, log_info, log_warn};
use core::sync::atomic::Ordering;

pub fn init_nonos_keys() -> Result<usize, &'static str> {
    if INIT_DONE.load(Ordering::SeqCst) {
        log_warn("crypto", "keystore already initialized");
        return Ok(KEYSTORE.lock().count);
    }
    log_info("crypto", "initializing NONOS keystore");
    if is_zero_key(&NONOS_PUBLIC_KEY) {
        log_error("crypto", "CRITICAL: signing key is zero");
        return Err("invalid signing key");
    }
    match super::add::add_key_versioned(&NONOS_PUBLIC_KEY, KEY_VERSION) {
        Ok(id) => {
            if !constant_time_eq(&id, &NONOS_KEY_ID) {
                log_error("crypto", "key ID mismatch");
                return Err("key verification failed");
            }
            log_info("crypto", "NONOS signing key loaded");
        }
        Err(e) => {
            log_error("crypto", "failed to load signing key");
            return Err(e);
        }
    }
    log_info("crypto", "keystore ready");
    Ok(1)
}

pub fn is_initialized() -> bool {
    INIT_DONE.load(Ordering::SeqCst)
}
pub use init_nonos_keys as init_production_keys;
