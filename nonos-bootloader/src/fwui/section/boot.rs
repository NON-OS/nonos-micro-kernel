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

use super::act::act;
use super::row::Row;
use crate::fwui::theme;
use crate::menu::{MenuAction, SecurityMode};
use alloc::vec;
use alloc::vec::Vec;

pub fn boot() -> Vec<Row> {
    #[allow(unused_mut)]
    let mut rows = vec![
        act(
            b"BOOT STANDARD",
            "signed",
            theme::OK,
            b"Boot the signed kernel with standard security enforced.",
            MenuAction::Boot(SecurityMode::Standard),
        ),
        act(
            b"BOOT HARDENED",
            "full chain",
            theme::OK,
            b"Require the full verification chain. Maximum security.",
            MenuAction::Boot(SecurityMode::Hardened),
        ),
        act(
            b"AIR-GAPPED",
            "no network",
            theme::TEXT,
            b"Boot with the network stack disabled for isolated operation.",
            MenuAction::NetworkIsolated,
        ),
        act(
            b"SAFE MODE",
            "minimal",
            theme::WARN,
            b"Minimal drivers and reduced features for diagnostics.",
            MenuAction::SafeMode,
        ),
        act(
            b"RECOVERY",
            "repair",
            theme::WARN,
            b"Boot the recovery environment for system repair.",
            MenuAction::Recovery,
        ),
    ];
    #[cfg(feature = "dev-mode")]
    rows.push(act(
        b"BOOT DEV",
        "INSECURE",
        theme::ERR,
        b"Unsigned kernel allowed, all checks disabled.",
        MenuAction::Boot(SecurityMode::Development),
    ));
    rows
}
