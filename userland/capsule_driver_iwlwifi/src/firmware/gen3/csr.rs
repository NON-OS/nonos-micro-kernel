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

//! Control registers for the gen3 (AX210-class) firmware self-load. On these
//! devices the host does not stream firmware over a DMA channel as the legacy
//! FH path does; instead it hands the boot ROM the physical address of a
//! context-information structure and an image loader, then sets one bit and the
//! device loads its own firmware. The register offsets and the boot-enable bit
//! mirror the Linux iwlwifi CSR map (iwl-context-info-gen3.h, iwl-csr.h),
//! reimplemented here as plain constants.

/// Physical address of the `iwl_context_info_gen3` structure. Written as a
/// 64-bit value (low dword at this offset, high dword four bytes on).
pub const CSR_CTXT_INFO_ADDR: usize = 0x118;

/// Physical address of the image-loader (IML) blob, written 64-bit.
pub const CSR_IML_DATA_ADDR: usize = 0x120;

/// Length in bytes of the image-loader blob.
pub const CSR_IML_SIZE_ADDR: usize = 0x128;

/// The boot-control register; setting the auto-boot bit starts the self-load.
pub const CSR_CTXT_INFO_BOOT_CTRL: usize = 0x000;

/// Auto function boot enable: the bit in the boot-control register that tells
/// the ROM to read the context information and load firmware on its own.
pub const CSR_AUTO_FUNC_BOOT_ENA: u32 = 1 << 1;
