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

use uefi::prelude::*;

use crate::boot::zk_challenge::read_pending_nonce;
use crate::entropy::collect_boot_entropy_64;
use crate::log::logger::{log_error, log_info};
use crate::zk::{init_boot_nonce, init_boot_nonce_value};

pub fn init_zk_nonce(st: &SystemTable<Boot>) -> Result<(), &'static str> {
    if let Some(nonce) = read_pending_nonce(st) {
        init_boot_nonce_value(nonce);
        log_info("zk_init", "Boot nonce restored from pending ZK challenge");
        return Ok(());
    }
    let entropy = collect_boot_entropy_64(st)?;
    init_boot_nonce(&entropy);
    log_info("zk_init", "Boot nonce initialized from hardware entropy");
    Ok(())
}

pub fn init_zk_nonce_required(st: &SystemTable<Boot>) {
    if let Err(e) = init_zk_nonce(st) {
        log_error("zk_init", e);
        log_error("zk_init", "FATAL: Cannot initialize ZK replay protection");
        crate::log::logger::log_critical("zk_init", "ZK nonce initialization failed - secure halt");
        loop {
            core::hint::spin_loop();
        }
    }
}
