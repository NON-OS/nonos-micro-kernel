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

use core::arch::asm;

use super::extension::Extension;

pub unsafe fn has_extension_from_misa(ext: Extension) -> bool {
    let misa = read_misa();
    if ext == Extension::G {
        return has_extension_from_misa(Extension::I)
            && has_extension_from_misa(Extension::M)
            && has_extension_from_misa(Extension::A)
            && has_extension_from_misa(Extension::F)
            && has_extension_from_misa(Extension::D);
    }
    match ext.bit() {
        Some(bit) => (misa >> bit) & 1 != 0,
        None => false,
    }
}

unsafe fn read_misa() -> usize {
    let misa: usize;
    asm!("csrr {}, misa", out(reg) misa, options(nostack));
    misa
}

pub unsafe fn mxl() -> usize {
    (read_misa() >> 62) & 0x3
}

pub unsafe fn is_rv64() -> bool {
    mxl() == 2
}

pub unsafe fn is_rv32() -> bool {
    mxl() == 1
}
