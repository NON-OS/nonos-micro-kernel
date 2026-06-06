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
use crate::image::jpeg::sos::ScanHeader;

pub struct ScanContext<'a> {
    pub frame: &'a FrameHeader,
    pub scan: &'a ScanHeader,
    pub dc_tables: &'a [HuffmanTable; MAX_HT],
    pub ac_tables: &'a [HuffmanTable; MAX_HT],
    pub qt: &'a [QuantTable; MAX_QT],
    pub restart_interval: u32,
}

pub struct McuScratch {
    pub pred: [i32; 3],
    pub y_blocks: [[u8; 64]; 4],
    pub cb_blk: [u8; 64],
    pub cr_blk: [u8; 64],
}
