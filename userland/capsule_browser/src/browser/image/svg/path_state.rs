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

use alloc::vec::Vec;

type P = [f32; 2];

// Interpreter state while walking path data: the pen, the open subpath, and
// the reflection anchors the smooth curve commands need.
pub(super) struct PathState {
    pub cur: P,
    pub start: P,
    pub sub: Vec<P>,
    pub out: Vec<Vec<P>>,
    pub last_c2: Option<P>,
    pub last_q: Option<P>,
}

impl PathState {
    pub fn new() -> Self {
        PathState {
            cur: [0.0, 0.0],
            start: [0.0, 0.0],
            sub: Vec::new(),
            out: Vec::new(),
            last_c2: None,
            last_q: None,
        }
    }

    pub fn move_to(&mut self, p: P) {
        self.flush();
        self.cur = p;
        self.start = p;
        self.sub.push(p);
    }

    pub fn close(&mut self) {
        if !self.sub.is_empty() {
            self.sub.push(self.start);
            self.cur = self.start;
        }
        self.flush();
    }

    fn flush(&mut self) {
        if self.sub.len() > 1 {
            let done = core::mem::take(&mut self.sub);
            self.out.push(done);
        } else {
            self.sub.clear();
        }
    }

    pub fn finish(mut self) -> Vec<Vec<P>> {
        self.flush();
        self.out
    }
}
