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

use super::Layout;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Control {
    PlayPause,
    Stop,
    SeekBackSecs(u32),
    SeekFwdSecs(u32),
    SeekPermille(u32),
    VolumePermille(u32),
}

fn track_permille(x: i32, base: u32, w: u32) -> u32 {
    if w == 0 {
        return 0;
    }
    let raw = (x as i64 - base as i64) * 1000 / w as i64;
    raw.clamp(0, 1000) as u32
}

pub fn control_at(l: &Layout, x: i32, y: i32) -> Option<Control> {
    if l.back.contains(x, y) {
        return Some(Control::SeekBackSecs(10));
    }
    if l.play.contains(x, y) {
        return Some(Control::PlayPause);
    }
    if l.stop.contains(x, y) {
        return Some(Control::Stop);
    }
    if l.fwd.contains(x, y) {
        return Some(Control::SeekFwdSecs(10));
    }
    if l.progress.contains(x, y) {
        return Some(Control::SeekPermille(track_permille(x, l.progress.x, l.progress.w)));
    }
    if l.volume.contains(x, y) {
        return Some(Control::VolumePermille(track_permille(x, l.volume.x, l.volume.w)));
    }
    None
}
