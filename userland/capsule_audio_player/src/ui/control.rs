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

use crate::transport::{State, Transport};

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

pub fn playing(tp: &Transport) -> bool {
    tp.state() == State::Playing
}
