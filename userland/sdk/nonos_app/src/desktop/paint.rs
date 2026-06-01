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

use nonos_ui::{Canvas, Color, Control};

use super::types::DesktopWindow;

pub(crate) fn paint_present<C: Control>(win: &mut DesktopWindow, bg: Color, root: &C) {
    let (w, h) = (win.width, win.height);
    {
        let mut canvas = Canvas::new(win.framebuffer(), w, h);
        canvas.fill(bg);
        root.paint(&mut canvas);
    }
    win.present();
}
