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

use crate::term::grid::types::Grid;
use crate::term::vt::parser::Parser;
use crate::term::vt::state::VtState;

impl Grid {
    pub fn feed(&mut self, bytes: &[u8]) {
        let mut parser = core::mem::replace(&mut self.parser, Parser::new());
        {
            let mut vt = VtState { g: self };
            for &b in bytes {
                parser.advance(&mut vt, b);
            }
        }
        self.parser = parser;
    }
}
