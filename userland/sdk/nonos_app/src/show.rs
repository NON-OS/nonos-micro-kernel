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

use nonos_runtime::{log, yield_now};
use nonos_ui::Canvas;
use nonos_window::Window;

use super::app::App;

impl App {
    pub fn show(self) -> i64 {
        if !self.want_window {
            return -22;
        }
        let mut win = match Window::new(self.width, self.height) {
            Some(w) => w,
            None => return -1,
        };
        let (w, h) = (win.width(), win.height());
        let mut canvas = Canvas::new(win.framebuffer(), w, h);
        canvas.fill(self.background);
        win.present();
        let _ = log::log(self.title.as_bytes());
        loop {
            yield_now();
        }
    }
}
