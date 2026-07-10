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

pub const INTEL_VENDOR_ID: u16 = 0x8086;
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
// Runtime ucode images are hundreds of KB to ~1 MB; the old 64 KB staging
// buffer could not hold a single image, so staging truncated on real firmware.
pub const FW_STAGING_SIZE: u64 = 2 * 1024 * 1024;
pub const PAGE_MASK: u64 = 0xFFF;

// Flow Handler (FH) registers for the legacy (pre-8000) firmware-load DMA path.
// The FH service channel (9) transfers a staged section from host DRAM into the
// device's internal SRAM at a destination address. Offsets from iwl-fh.h.
pub const FH_TFDIB_CTRL0_REG: usize = 0x1948;
pub const FH_TFDIB_CTRL1_REG: usize = 0x194C;
pub const FH_SRVC_CHNL_SRAM_ADDR_REG: usize = 0x19C8;
pub const FH_TCSR_CHNL_TX_CONFIG_REG: usize = 0x1E20;
pub const FH_TCSR_CHNL_TX_BUF_STS_REG: usize = 0x1E28;
pub const FH_TFDIB_REG1_ADDR_BITSHIFT: u32 = 28;
pub const FH_TCSR_TX_CONFIG_DMA_PAUSE: u32 = 0x0000_0000;
pub const FH_TCSR_TX_CONFIG_DMA_ENABLE: u32 = 0x8000_0000;
pub const FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD: u32 = 0x0010_0000;
pub const FH_TCSR_TX_BUF_STS_TFDB_VALID: u32 = (1 << 20) | (1 << 12) | 0x1;
pub const FH_TX_POLL_ITERS: usize = 500_000;

// Peripheral (PRPH) indirect register access through the HBUS window, and the
// CPU-release register that starts the loaded firmware. After every ucode
// section is in SRAM, releasing the CPU reset boots the firmware, which then
// raises the ALIVE interrupt. Offsets from iwl-io.h / iwl-prph.h.
pub const HBUS_TARG_PRPH_WADDR: usize = 0x0444;
pub const HBUS_TARG_PRPH_WDAT: usize = 0x044C;
pub const PRPH_WADDR_ADDR_MASK: u32 = 0x000F_FFFF;
pub const PRPH_WADDR_WORD_ENABLE: u32 = 0x3 << 24;
pub const RELEASE_CPU_RESET: u32 = 0x300C;
pub const RELEASE_CPU_RESET_BIT: u32 = 0x0100_0000;
pub const CSR_INT_COALESCING: usize = 0x004;
pub const CSR_INT: usize = 0x008;
pub const CSR_INT_MASK: usize = 0x00C;
pub const CSR_FH_INT_STATUS: usize = 0x010;
pub const CSR_GP_CNTRL: usize = 0x024;
pub const CSR_HW_REV: usize = 0x028;
pub const GP_CNTRL_MAC_CLOCK_READY: u32 = 0x0000_0002;
pub const GP_CNTRL_INIT_DONE: u32 = 0x0000_0004;
pub const GP_CNTRL_MAC_ACCESS_REQ: u32 = 0x0000_0008;
pub const GP_CNTRL_XTAL_ON: u32 = 0x0000_0400;
pub const ALL_INTS_MASK: u32 = 0xFFFF_FFFF;
pub const INT_MASK_DISABLED: u32 = 0;
pub const INT_COALESCING_TIMEOUT: u32 = 64;
pub const APM_POLL_ITERS: usize = 250_000;
pub const INT_BIT_ALIVE: u32 = 1 << 0;
pub const ALIVE_POLL_ITERS: usize = 2_000_000;
pub const IWL_FW_MAGIC: u32 = 0x0A4C_5749;
pub const FW_API_VERSION_MASK: u32 = 0xFFFF;
pub const MIN_FW_API_VERSION: u16 = 22;
pub const MAX_FW_API_VERSION: u16 = 77;
