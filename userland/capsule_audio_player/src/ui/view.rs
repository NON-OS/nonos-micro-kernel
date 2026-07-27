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

use crate::library::{Library, Queue};
use crate::model::PlayerView;
use crate::waveform::Waveform;

use super::draw::{paint_header, paint_list, paint_panel, paint_transport, BG};
use super::geometry::Layout;

pub fn paint_player(fb: &mut PaintBuffer, v: &PlayerView, wf: &Waveform, l: &Layout) {
    fb.clear(BG);
    paint_header(fb, l, false);
    paint_panel(fb, v, wf, l);
    paint_transport(fb, v, l);
}

pub fn paint_library(fb: &mut PaintBuffer, lib: &Library, q: &Queue, current: Option<usize>, l: &Layout) {
    fb.clear(BG);
    paint_header(fb, l, true);
    paint_list(fb, lib, q, current, &l.list);
}
