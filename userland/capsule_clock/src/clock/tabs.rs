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

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    Clock,
    Stopwatch,
    Timer,
    Set,
}

pub const BAR_H: i32 = 32;
pub const BAR_TOP: i32 = 28;

pub fn all() -> [Tab; 4] {
    [Tab::Clock, Tab::Stopwatch, Tab::Timer, Tab::Set]
}

pub fn label(t: Tab) -> &'static [u8] {
    match t {
        Tab::Clock => b"Clock",
        Tab::Stopwatch => b"Stopwatch",
        Tab::Timer => b"Timer",
        Tab::Set => b"Set",
    }
}

pub fn hit(width: i32, x: i32, y: i32) -> Option<Tab> {
    if y < BAR_TOP || y >= BAR_TOP + BAR_H {
        return None;
    }
    let w = width / 4;
    match x / w {
        0 => Some(Tab::Clock),
        1 => Some(Tab::Stopwatch),
        2 => Some(Tab::Timer),
        _ => Some(Tab::Set),
    }
}
