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

//! Finding the redistributor frame that belongs to the running CPU.
//!
//! GICv3 does not promise that frames appear in MPIDR order, or that a core's
//! position in the device tree is its position in the redistributor region.
//! Each frame states its own owner in `GICR_TYPER`, so the only correct way to
//! find yours is to walk the region comparing affinities until the frame that
//! claims you turns up, stopping at the one that marks itself last.

use super::device::GicRedistributor;
use crate::arch::aarch64::cpu::cpu_affinity;

/// `GICR_TYPER.Last`: this is the final frame in the region.
const TYPER_LAST: u64 = 1 << 4;
/// `GICR_TYPER.VLPIS`: the frame carries the two extra virtual-LPI pages.
const TYPER_VLPIS: u64 = 1 << 1;

/// Two 64 KiB pages per redistributor: RD_base and SGI_base.
const STRIDE_BASE: u64 = 0x2_0000;
/// Four when the implementation adds VLPI_base and its reserved neighbour.
const STRIDE_VLPI: u64 = 0x4_0000;

/// A hard stop, so a region that never reports `Last` cannot walk off into
/// unmapped MMIO forever.
const MAX_FRAMES: usize = 512;

/// The frame owning the calling CPU, or `None` if the region does not claim it.
pub fn for_this_cpu(region_base: u64) -> Option<GicRedistributor> {
    if region_base == 0 {
        return None;
    }
    let wanted = cpu_affinity();
    let mut base = region_base;

    for _ in 0..MAX_FRAMES {
        let frame = GicRedistributor::new(base);
        let typer = frame.typer();
        if frame.affinity() == wanted {
            return Some(frame);
        }
        if typer & TYPER_LAST != 0 {
            return None;
        }
        base += if typer & TYPER_VLPIS != 0 { STRIDE_VLPI } else { STRIDE_BASE };
    }
    None
}
