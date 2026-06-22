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

use super::machine::init_zk_machine_id;
use super::nonce::init_zk_nonce_required;
use crate::log::logger::log_info;
use crate::zk::{is_machine_id_initialized, is_nonce_initialized};

pub fn initialize_zk_replay_protection(st: &SystemTable<Boot>) {
    log_info("zk_init", "Initializing ZK replay protection bindings");

    init_zk_nonce_required(st);

    if let Err(e) = init_zk_machine_id(st) {
        crate::log::logger::log_warn("zk_init", e);
    }

    verify_initialization();
}

fn verify_initialization() {
    if !is_nonce_initialized() {
        crate::log::logger::log_critical("zk_init", "ZK boot nonce not initialized - secure halt");
        loop {
            core::hint::spin_loop();
        }
    }

    if !is_machine_id_initialized() {
        crate::log::logger::log_critical("zk_init", "ZK machine ID not initialized - secure halt");
        loop {
            core::hint::spin_loop();
        }
    }

    log_info("zk_init", "ZK replay protection READY");
}
