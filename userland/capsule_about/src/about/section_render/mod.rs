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

mod authority;
mod display;
mod identity;
mod license;
mod row;
mod uptime;

use nonos_app_skeleton::PaintBuffer;

use crate::about::section::Section;

pub fn render_section(section: Section, scroll: u32, visible: u32, top: u32, fb: &mut PaintBuffer) {
    match section {
        Section::Identity => identity::render(scroll, visible, top, fb),
        Section::Authority => authority::render(scroll, visible, top, fb),
        Section::Display => display::render(scroll, visible, top, fb),
        Section::Uptime => uptime::render(scroll, visible, top, fb),
        Section::License => license::render(scroll, visible, top, fb),
    }
}

pub fn section_line_count(section: Section) -> u32 {
    match section {
        Section::Identity => identity::LINE_COUNT,
        Section::Authority => authority::line_count(),
        Section::Display => display::LINE_COUNT,
        Section::Uptime => uptime::LINE_COUNT,
        Section::License => license::line_count(),
    }
}
