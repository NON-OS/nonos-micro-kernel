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

use core::fmt::{self, Write};

use super::ops::write_str_to_port;
use super::state::primary_port_index;

pub struct SerialWriter {
    port_index: usize,
}

impl SerialWriter {
    pub fn new() -> Self {
        Self { port_index: primary_port_index() }
    }

    pub fn for_port(port_index: usize) -> Self {
        Self { port_index }
    }

    pub fn port_index(&self) -> usize {
        self.port_index
    }
}

impl Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = write_str_to_port(self.port_index, s);
        Ok(())
    }
}

impl Default for SerialWriter {
    fn default() -> Self {
        Self::new()
    }
}
