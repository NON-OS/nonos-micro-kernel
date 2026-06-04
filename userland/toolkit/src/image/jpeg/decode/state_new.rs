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
use crate::image::jpeg::dht::HuffmanTable;
use crate::image::jpeg::dqt::QuantTable;

use super::state::DecodeState;

impl DecodeState {
    pub fn new() -> Self {
        Self {
            pos: 2,
            frame_opt: None,
            dqt: [QuantTable::new(); 4],
            dc_tables: [
                HuffmanTable::new(),
                HuffmanTable::new(),
                HuffmanTable::new(),
                HuffmanTable::new(),
            ],
            ac_tables: [
                HuffmanTable::new(),
                HuffmanTable::new(),
                HuffmanTable::new(),
                HuffmanTable::new(),
            ],
            restart_interval: 0,
        }
    }
}
