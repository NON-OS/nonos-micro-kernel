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

use crate::display::{log_hash, log_size};
use crate::kernel_verify::CryptoVerifyResult;

pub fn display_elf_info(elf: &[u8], crypto: &CryptoVerifyResult, gop: bool) {
    if !gop {
        return;
    }
    log_size(b"ELF len   ", elf.len());
    log_size(b"code_size ", crypto.kernel_code_size);
    if elf.len() >= 8 {
        let mut hdr = [0u8; 8];
        hdr.copy_from_slice(&elf[..8]);
        log_hash(b"ELF hdr   ", &hdr);
    }
}
