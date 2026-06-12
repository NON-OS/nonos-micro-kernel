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

use super::row::Row;
use super::set::set;
use crate::fwui::settings::{mode_name, Edit, Settings};
use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

pub fn setup(s: &Settings) -> Vec<Row> {
    let timeout = if s.timeout_s == 0 { "wait".to_string() } else { format!("{}s", s.timeout_s) };
    vec![
        set(
            b"DEFAULT BOOT MODE",
            mode_name(s.default_mode).to_string(),
            b"Boot mode applied when the auto-boot timeout elapses.",
            Edit::Mode,
        ),
        set(
            b"AUTO-BOOT TIMEOUT",
            timeout,
            b"Seconds before the default mode boots. wait disables auto-boot.",
            Edit::Timeout,
        ),
        set(
            b"SECURE-BOOT ENFORCE",
            if s.enforce_sb { "[on]".to_string() } else { "[off]".to_string() },
            b"Require UEFI Secure Boot to be active before kernel handoff.",
            Edit::Enforce,
        ),
    ]
}
