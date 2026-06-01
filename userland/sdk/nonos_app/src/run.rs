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

use nonos_runtime::{
    exit, input_drain, yield_now, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP,
    INPUT_KIND_POINTER_ABS,
};
use nonos_ui::Control;
use nonos_window::Window;

use super::app::App;
use super::paint_all::paint_all;

impl App {
    pub fn run<C: Control>(self, mut root: C) -> ! {
        let mut win = match Window::new(self.width, self.height) {
            Some(w) => w,
            None => exit(1),
        };
        let (w, h) = (win.width(), win.height());
        paint_all(&mut win, w, h, self.background, &root);
        let (mut px, mut py) = (0i32, 0i32);
        let mut buf = [InputEvent::default(); 16];
        loop {
            let n = input_drain(&mut buf);
            if n > 0 {
                let count = (n as usize).min(buf.len());
                let mut repaint = false;
                for ev in buf.iter().take(count) {
                    let mut e = *ev;
                    if e.kind == INPUT_KIND_POINTER_ABS {
                        px = e.x;
                        py = e.y;
                    } else if e.kind == INPUT_KIND_BUTTON_DOWN || e.kind == INPUT_KIND_BUTTON_UP {
                        e.x = px;
                        e.y = py;
                    }
                    if root.on_event(&e) {
                        repaint = true;
                    }
                }
                if repaint {
                    paint_all(&mut win, w, h, self.background, &root);
                }
            }
            yield_now();
        }
    }
}
