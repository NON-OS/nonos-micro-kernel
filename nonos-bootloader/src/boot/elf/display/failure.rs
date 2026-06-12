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

extern crate alloc;
use crate::display::{log_error as panel_error, log_size, show_error_screen};
use crate::kernel_verify::CryptoVerifyResult;
use crate::loader::errors::LoaderError;
use alloc::format;

pub fn display_load_failure(
    elf: &[u8],
    crypto: &CryptoVerifyResult,
    full: &[u8],
    gop: bool,
    e: &LoaderError,
) {
    if !gop {
        return;
    }
    log_size(b"FAIL len  ", elf.len());
    log_size(b"FAIL code ", crypto.kernel_code_size);
    log_size(b"FAIL full ", full.len());
    panel_error(format!("{}", e).as_bytes());
    panel_error(b"ELF parse failed");
    show_error_screen(b"Kernel ELF parsing failed");
}
