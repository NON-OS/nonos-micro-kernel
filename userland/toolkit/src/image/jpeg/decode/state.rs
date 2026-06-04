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
use crate::image::jpeg::dht::{HuffmanTable, MAX_HT};
use crate::image::jpeg::dqt::{QuantTable, MAX_QT};
use crate::image::jpeg::sof0::FrameHeader;

pub struct DecodeState {
    pub pos: usize,
    pub frame_opt: Option<FrameHeader>,
    pub dqt: [QuantTable; MAX_QT],
    pub dc_tables: [HuffmanTable; MAX_HT],
    pub ac_tables: [HuffmanTable; MAX_HT],
    pub restart_interval: u32,
}
