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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Main,
    Boot,
    Setup,
    Security,
    Monitor,
    Advanced,
    Tool,
}

impl Section {
    pub const ALL: [Section; 7] = [
        Section::Main,
        Section::Boot,
        Section::Setup,
        Section::Security,
        Section::Monitor,
        Section::Advanced,
        Section::Tool,
    ];

    pub const fn nav_label(self) -> &'static [u8] {
        match self {
            Section::Main => b"MAIN",
            Section::Boot => b"BOOT",
            Section::Setup => b"SETUP",
            Section::Security => b"SECURITY",
            Section::Monitor => b"MONITOR",
            Section::Advanced => b"ADVANCED",
            Section::Tool => b"TOOL",
        }
    }

    pub const fn title(self) -> &'static [u8] {
        match self {
            Section::Main => b"SYSTEM OVERVIEW",
            Section::Boot => b"BOOT OPTIONS",
            Section::Setup => b"SETUP",
            Section::Security => b"SECURITY POSTURE",
            Section::Monitor => b"HARDWARE MONITOR",
            Section::Advanced => b"CPU FEATURES",
            Section::Tool => b"TOOLS",
        }
    }
}
