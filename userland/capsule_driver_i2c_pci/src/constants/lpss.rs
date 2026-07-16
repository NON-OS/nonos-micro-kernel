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
// Intel LPSS wrapper (Skylake and later, drivers/mfd/intel-lpss.c). The
// DesignWare core sits at 0x000..0x100; the LPSS private register block is at
// offset 0x200 inside the same MMIO BAR. Out of reset the LPSS holds the
// DesignWare core asserted, so the RESETS register must be deasserted before
// any core register is touched or every access returns garbage.
pub const LPSS_PRIV: u64 = 0x200;
pub const LPSS_PRIV_RESETS: u64 = LPSS_PRIV + 0x04;
// Bit 2 (FUNC) releases the function reset; bits 1:0 (IDMA) release the
// integrated DMA reset. Writing 0x7 deasserts all three; writing 0 asserts
// them. This matches intel_lpss_deassert_reset().
pub const LPSS_PRIV_RESETS_FUNC: u32 = 1 << 2;
pub const LPSS_PRIV_RESETS_IDMA: u32 = 0x3;
pub const LPSS_PRIV_RESETS_DEASSERT: u32 = LPSS_PRIV_RESETS_FUNC | LPSS_PRIV_RESETS_IDMA;
