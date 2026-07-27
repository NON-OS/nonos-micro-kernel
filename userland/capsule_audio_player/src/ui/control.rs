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
    Prev,
    Next,
    Shuffle,
    Repeat,
    Mute,
    Seek(u32),
    Volume(u32),
    SeekBackSecs(u32),
    SeekFwdSecs(u32),
}

fn track_permille(x: i32, base: u32, w: u32) -> u32 {
    if w == 0 {
        return 0;
    }
    let raw = (x as i64 - base as i64) * 1000 / w as i64;
    raw.clamp(0, 1000) as u32
}

pub fn control_at(l: &Layout, x: i32, y: i32) -> Option<Control> {
    if l.shuffle.contains(x, y) {
        return Some(Control::Shuffle);
    }
    if l.prev.contains(x, y) {
        return Some(Control::Prev);
    }
    if l.play.contains(x, y) {
        return Some(Control::PlayPause);
    }
    if l.next.contains(x, y) {
        return Some(Control::Next);
    }
    if l.repeat.contains(x, y) {
        return Some(Control::Repeat);
    }
    if l.speaker.contains(x, y) {
        return Some(Control::Mute);
    }
    if l.waveform.contains(x, y) {
        return Some(Control::Seek(track_permille(x, l.waveform.x, l.waveform.w)));
    }
    if l.volume.contains(x, y) {
        return Some(Control::Volume(track_permille(x, l.volume.x, l.volume.w)));
    }
    None
}
