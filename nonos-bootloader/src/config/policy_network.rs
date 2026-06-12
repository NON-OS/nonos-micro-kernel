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

use uefi::cstr16;
use uefi::prelude::*;

use crate::log::logger::{log_info, log_warn};
use crate::network::NetworkBootContext;

use super::types::{BootloaderConfig, NetworkPolicy};

pub fn apply_network_policy(
    config: &BootloaderConfig,
    system_table: &mut SystemTable<Boot>,
    network: &NetworkBootContext,
) -> bool {
    match config.network_policy {
        NetworkPolicy::Disabled => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Network boot disabled by policy\r\n"))
                .unwrap_or(());
            log_info("config", "Network boot disabled by policy");
        }
        NetworkPolicy::Secured => {
            if !network.http_client_available {
                system_table
                    .stdout()
                    .output_string(cstr16!(
                        "   [WARN] Secured network policy requires HTTPS support\r\n"
                    ))
                    .unwrap_or(());
                log_warn("config", "Secured network policy requirements not fully met");
                return false;
            }
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Secured network policy enforced\r\n"))
                .unwrap_or(());
        }
        NetworkPolicy::Standard => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Standard network policy enforced\r\n"))
                .unwrap_or(());
        }
        NetworkPolicy::Unrestricted => {
            system_table
                .stdout()
                .output_string(cstr16!("   [INFO] Unrestricted network policy enforced\r\n"))
                .unwrap_or(());
        }
    }

    true
}
