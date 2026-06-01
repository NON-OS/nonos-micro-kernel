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

#[derive(Debug, Clone, Copy, Default)]
pub struct ProtectionFlags {
    pub smep_enabled: bool,
    pub smap_enabled: bool,
    pub nx_enabled: bool,
    pub wp_enabled: bool,
    // UMIP is best-effort: it hardens against SGDT/SIDT/SLDT/SMSW/STR
    // leaks from userspace but is not present on every CPU, so it is
    // tracked here without gating `is_fully_protected`.
    pub umip_enabled: bool,
}

impl ProtectionFlags {
    pub const fn new() -> Self {
        Self {
            smep_enabled: false,
            smap_enabled: false,
            nx_enabled: false,
            wp_enabled: true,
            umip_enabled: false,
        }
    }

    pub const fn is_fully_protected(&self) -> bool {
        self.smep_enabled && self.smap_enabled && self.nx_enabled && self.wp_enabled
    }
}
