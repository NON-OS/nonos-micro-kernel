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

//! Kick the gen3 firmware self-load. Once the context-information structure and
//! the image loader are laid out in host memory, boot is four register writes:
//! hand the ROM the physical address of the context info, then the image
//! loader's address and length, then set the auto-boot bit. The ROM does the
//! rest and signals readiness with the ALIVE interrupt. The order mirrors
//! iwl_pcie_ctxt_info_gen3_init in Linux iwlwifi; a 64-bit register is two
//! 32-bit writes, low dword first, matching iwl_write64.

use super::csr::{
    CSR_AUTO_FUNC_BOOT_ENA, CSR_CTXT_INFO_ADDR, CSR_CTXT_INFO_BOOT_CTRL, CSR_IML_DATA_ADDR,
    CSR_IML_SIZE_ADDR,
};
use crate::regs::Mmio;

/// Start the self-load. `ctxt_info_addr` is the physical address of the gen3
/// context-information structure, `iml_addr`/`iml_len` the image loader the ROM
/// runs to fetch the firmware image. Returns once the auto-boot bit is set; the
/// caller then waits for ALIVE.
pub fn kick<M: Mmio>(mmio: &M, ctxt_info_addr: u64, iml_addr: u64, iml_len: u32) {
    write64(mmio, CSR_CTXT_INFO_ADDR, ctxt_info_addr);
    write64(mmio, CSR_IML_DATA_ADDR, iml_addr);
    mmio.write32(CSR_IML_SIZE_ADDR, iml_len);
    let cur = mmio.read32(CSR_CTXT_INFO_BOOT_CTRL);
    mmio.write32(CSR_CTXT_INFO_BOOT_CTRL, cur | CSR_AUTO_FUNC_BOOT_ENA);
}

fn write64<M: Mmio>(mmio: &M, off: usize, val: u64) {
    mmio.write32(off, val as u32);
    mmio.write32(off + 4, (val >> 32) as u32);
}
