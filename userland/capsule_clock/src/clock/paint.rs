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

use crate::clock::state::State;
use crate::clock::tabs::Tab;
use crate::clock::{fmt, manifest, paint_analog, paint_stopwatch, paint_tabbar, theme};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(theme::BG);
    match state.tab {
        Tab::Clock => paint_clock(state, fb),
        Tab::Stopwatch => paint_stopwatch::paint(state, fb),
        Tab::Timer => {}
    }
    paint_tabbar::paint(state.tab, fb, manifest::WIDTH);
}

fn paint_clock(state: &State, fb: &mut PaintBuffer) {
    let r = &state.rtc;
    let hms = fmt::hms(r.hour, r.minute, r.second);
    fb.text_scaled(36, 56, &hms, theme::FG, 4);
    let wd = fmt::weekday(r.year, r.month, r.day);
    fb.text(40, 122, wd.as_bytes(), theme::DIM);
    let mon = fmt::month_name(r.month);
    fb.text(40, 142, mon.as_bytes(), theme::DIM);
    let dd = fmt::two(r.day);
    fb.text(84, 142, &dd, theme::DIM);
    let yr = fmt::year4(r.year);
    fb.text(120, 142, &yr, theme::DIM);
    paint_analog::paint(state, fb, 180, 300, 84);
}
