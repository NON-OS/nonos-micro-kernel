// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use crate::display::{log_hex, log_ok, log_size, log_u32};
use crate::loader::KernelImage;

pub fn display_loaded_image(image: &KernelImage, gop: bool) {
    if !gop {
        return;
    }
    log_ok(b"ELF64 parsed successfully");
    log_hex(b"entry   ", image.entry_point as u64);
    log_hex(b"base    ", image.address as u64);
    log_size(b"size    ", image.size);
    log_u32(b"segments ", image.segment_count as u32);
}
