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
use uefi::table::runtime::VariableVendor;

use super::attrs::variable_attrs;
use super::serialize::serialize_challenge;
use super::timestamp::current_timestamp_secs;
use super::write_nonce::write_pending_nonce;
use crate::log::logger::log_error;
use crate::zk::{get_boot_nonce, get_machine_id};

pub fn write_zk_challenge(st: &SystemTable<Boot>, kernel_hash: &[u8; 32]) {
    let nonce = match get_boot_nonce() {
        Ok(nonce) => nonce,
        Err(e) => {
            log_error("zk_challenge", e);
            return;
        }
    };
    let machine = match get_machine_id() {
        Ok(id) => id,
        Err(e) => {
            log_error("zk_challenge", e);
            return;
        }
    };
    let challenge = serialize_challenge(kernel_hash, &nonce, &machine, current_timestamp_secs(st));
    let _ = st.runtime_services().set_variable(
        uefi::cstr16!("NonosZkBootChallenge"),
        &VariableVendor::GLOBAL_VARIABLE,
        variable_attrs(),
        &challenge,
    );
    let _ = write_pending_nonce(st, &nonce);
}
