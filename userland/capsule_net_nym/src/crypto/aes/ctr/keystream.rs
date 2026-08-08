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

use super::types::Ctr64Be;

impl Ctr64Be {
    /// Raw keystream. Sphinx needs it with no plaintext to combine it with,
    /// to build the padding it XORs the routing info against.
    pub fn keystream(&mut self, out: &mut [u8]) {
        for byte in out.iter_mut() {
            *byte = 0;
        }
        self.apply(out);
    }
}
