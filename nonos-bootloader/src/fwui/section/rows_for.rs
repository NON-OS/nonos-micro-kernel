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
use crate::fwui::data::Sys;
use crate::fwui::settings::Settings;
use crate::fwui::state::Section;
use alloc::vec::Vec;
use uefi::table::runtime::Time;

pub fn rows_for(section: Section, sys: &Sys, settings: &Settings, time: &Time) -> Vec<Row> {
    match section {
        Section::Main => super::main::main(sys, time),
        Section::Boot => super::boot::boot(),
        Section::Setup => super::setup::setup(settings),
        Section::Security => super::security::security(sys),
        Section::Monitor => super::monitor::monitor(sys),
        Section::Advanced => super::advanced::advanced(sys),
        Section::Tool => super::tool::tool(),
    }
}
