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

use super::mode::SecurityMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    Boot(SecurityMode),
    Recovery,
    Diagnostics,
    SecurityStatus,
    SafeMode,
    MemoryTest,
    NetworkIsolated,
    UefiShell,
    Continue,
    Timeout,
    Shutdown,
}

impl MenuAction {
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Boot(SecurityMode::Development) => "Boot (Dev - INSECURE)",
            Self::Boot(SecurityMode::Standard) => "Boot (Standard Mode)",
            Self::Boot(SecurityMode::Hardened) => "Boot (Hardened Mode)",
            Self::Boot(SecurityMode::SafeMode) => "Boot (Safe Mode)",
            Self::Boot(SecurityMode::NetworkIsolated) => "Boot (Air-Gapped)",
            Self::Boot(SecurityMode::Recovery) => "Boot (Recovery)",
            Self::Recovery => "Recovery Mode",
            Self::Diagnostics => "Hardware Diagnostics",
            Self::SecurityStatus => "Security Status",
            Self::SafeMode => "Safe Mode (Minimal)",
            Self::MemoryTest => "Memory Test",
            Self::NetworkIsolated => "Air-Gapped Mode",
            Self::UefiShell => "UEFI Shell",
            Self::Continue => "Continue Boot",
            Self::Timeout => "Boot (Default)",
            Self::Shutdown => "Shutdown",
        }
    }
    pub const fn requires_verification(&self) -> bool {
        matches!(self, Self::Boot(SecurityMode::Standard) | Self::Boot(SecurityMode::Hardened))
    }
}
