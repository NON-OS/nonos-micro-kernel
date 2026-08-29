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

//! Gen3 (AX210-class, including Alder Lake CNVi) firmware self-load. Where the
//! legacy FH path streams each ucode section over a DMA channel, gen3 devices
//! read a context-information structure from host memory and load their own
//! firmware through an on-ROM image loader. This module owns the pieces of that
//! handoff: the context-information structure (`ctxt_info`), the boot control
//! registers (`csr`), and the register sequence that starts the load (`boot`).
//!
//! The peripheral scratch layout, the firmware DRAM image assembly, the image
//! loader extraction, and the gen3 receive/transmit rings the context info
//! points at are the remaining layers; each is added and proven on top of this
//! one before the path is wired into device bring-up.

pub mod boot;
pub mod csr;
pub mod ctxt_info;
pub mod dram_map;
pub mod image;
pub mod prph_scratch;
pub mod rx_mpdu;
pub mod tx_cmd;

pub use boot::kick;
pub use ctxt_info::{CtxtInfoGen3, CTXT_INFO_GEN3_SIZE};
pub use dram_map::{classify, stage, DramPlacement, FwLayout};
pub use image::{find_iml, sections, Section};
pub use prph_scratch::{DramImage, PrphScratch, PRPH_SCRATCH_SIZE};
pub use rx_mpdu::extract_mpdu;
pub use tx_cmd::{TX_CMD, TX_CMD_GEN3_HDR};
