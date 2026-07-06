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

use nonos_app_skeleton::PaintBuffer;

use crate::clock::fmt;
use crate::clock::state::State;
use crate::clock::theme;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(theme::BG);
    let r = &state.rtc;
    let hms = fmt::hms(r.hour, r.minute, r.second);
    fb.text_scaled(36, 56, &hms, theme::FG, 4);
    let wd = fmt::weekday(r.year, r.month, r.day);
    fb.text(40, 150, wd.as_bytes(), theme::DIM);
    let mon = fmt::month_name(r.month);
    fb.text(40, 172, mon.as_bytes(), theme::DIM);
    let dd = fmt::two(r.day);
    fb.text(84, 172, &dd, theme::DIM);
    let yr = fmt::year4(r.year);
    fb.text(120, 172, &yr, theme::DIM);
}
