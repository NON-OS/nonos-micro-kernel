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

use nonos_desktop::drain_input;
use nonos_runtime::{
    exit, InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS,
};
use nonos_ui::Control;

use super::app::App;
use super::desktop::{paint_present, DesktopWindow};

const WINDOW_ID: u32 = 1;

impl App {
    pub fn run<C: Control>(self, mut root: C) -> ! {
        let mut win = match DesktopWindow::open(self.width, self.height, WINDOW_ID) {
            Some(w) => w,
            None => exit(1),
        };
        paint_present(&mut win, self.background, &root);
        let (mut px, mut py) = (0i32, 0i32);
        let mut buf = [InputEvent::default(); 16];
        loop {
            let n = drain_input(&mut buf);
            if n > 0 {
                let mut repaint = false;
                for ev in buf.iter().take(n) {
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
                if root.wants_close() {
                    win.close();
                    exit(0);
                }
                if repaint {
                    paint_present(&mut win, self.background, &root);
                }
            }
        }
    }
}
