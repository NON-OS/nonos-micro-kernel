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

use crate::hardware::tpm::get_tpm_ek_public;
use crate::log::logger::{log_info, log_warn};
use crate::zk::init_machine_id;

#[cfg(not(feature = "production"))]
use super::machine_fallback_dev::init_fallback_machine_id;

#[cfg(feature = "production")]
use super::machine_fallback_production::init_fallback_machine_id;

pub fn init_zk_machine_id(st: &SystemTable<Boot>) -> Result<(), &'static str> {
    match get_tpm_ek_public(st) {
        Ok(ek_public) => {
            init_machine_id(&ek_public);
            log_info("zk_init", "Machine ID initialized from TPM EK");
            Ok(())
        }
        Err(e) => {
            log_warn("zk_init", "TPM EK unavailable");
            log_warn("zk_init", e);
            init_fallback_machine_id(st)
        }
    }
}
