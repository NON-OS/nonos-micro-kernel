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

#[cfg(target_arch = "x86_64")]
use crate::arch::x86_64::vga::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Debug,
    Info,
    Warn,
    Err,
    Fatal,
}

impl Severity {
    /// VGA text-mode attribute for this severity. Text mode is x86 hardware,
    /// so the mapping only exists where a VGA console can consume it.
    #[cfg(target_arch = "x86_64")]
    pub fn color(self) -> Color {
        match self {
            Severity::Debug => Color::Cyan,
            Severity::Info => Color::LightGreen,
            Severity::Warn => Color::Yellow,
            Severity::Err => Color::LightRed,
            Severity::Fatal => Color::LightRed,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Debug => "DBG",
            Severity::Info => "INFO",
            Severity::Warn => "WARN",
            Severity::Err => "ERR",
            Severity::Fatal => "FATAL",
        }
    }
}
