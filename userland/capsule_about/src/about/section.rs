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

use nonos_toolkit::icons::IconId;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Section {
    Overview,
    System,
    Trust,
    Display,
    Licenses,
}

// Sidebar order. The nav painter and the click router both walk this slice, so
// the rail can never draw an entry the hit test does not know about.
pub const SECTIONS: [Section; 5] = [
    Section::Overview,
    Section::System,
    Section::Trust,
    Section::Display,
    Section::Licenses,
];

impl Section {
    pub fn nav_label(self) -> &'static [u8] {
        match self {
            Section::Overview => b"Overview",
            Section::System => b"System",
            Section::Trust => b"Trust",
            Section::Display => b"Display",
            Section::Licenses => b"Licenses",
        }
    }
    // The muted line on the right of the head band: what this screen is evidence
    // of, not a restatement of its name.
    pub fn head_meta(self) -> &'static [u8] {
        match self {
            Section::Overview => b"identity and terms",
            Section::System => b"build and address space",
            Section::Trust => b"signing chain and capabilities",
            Section::Display => b"surface and present path",
            Section::Licenses => b"this image and its third parties",
        }
    }
    pub fn icon(self) -> IconId {
        match self {
            Section::Overview => IconId::PmOverview,
            Section::System => IconId::SettingsDeveloper,
            Section::Trust => IconId::PmAuthority,
            Section::Display => IconId::SettingsAppearance,
            Section::Licenses => IconId::FsFile,
        }
    }
    pub fn index(self) -> usize {
        match self {
            Section::Overview => 0,
            Section::System => 1,
            Section::Trust => 2,
            Section::Display => 3,
            Section::Licenses => 4,
        }
    }
}
