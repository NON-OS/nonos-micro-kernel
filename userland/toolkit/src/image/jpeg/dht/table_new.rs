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
use super::huffman_table::HuffmanTable;

impl HuffmanTable {
    pub const fn new() -> Self {
        Self {
            present: false,
            bits: [0; 17],
            huffval: [0; 256],
            mincode: [0; 17],
            maxcode: [-1; 18],
            valptr: [0; 17],
            total: 0,
        }
    }
}
