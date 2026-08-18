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

use crate::ui::screen::Route;

pub const DEPTH: usize = 8;

pub struct Nav {
    stack: Vec<Route>,
}

impl Nav {
    pub fn new(root: Route) -> Nav {
        let mut stack = Vec::new();
        stack.push(root);
        Nav { stack }
    }

    pub fn current(&self) -> Route {
        *self.stack.last().unwrap_or(&Route::Home)
    }

    pub fn can_back(&self) -> bool {
        self.stack.len() > 1
    }

    pub fn go(&mut self, route: Route) -> bool {
        if self.current() == route {
            return false;
        }
        if route.in_nav() {
            self.stack.clear();
        } else if self.stack.len() >= DEPTH {
            self.stack.remove(0);
        }
        self.stack.push(route);
        true
    }

    pub fn back(&mut self) -> bool {
        if !self.can_back() {
            return false;
        }
        self.stack.pop();
        true
    }
}
