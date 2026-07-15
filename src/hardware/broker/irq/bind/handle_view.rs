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

use super::super::validate::MsixHandleView;
use crate::drivers::pci::types::PciBar;
use crate::hardware::broker::pci_index::PciHandle;

pub(crate) fn handle_view(handle: &PciHandle) -> MsixHandleView {
    let msix = match handle.msix {
        Some(m) => m,
        None => return MsixHandleView::no_msix(),
    };
    let table_bar_idx = msix.table_bar as usize;
    let pba_bar_idx = msix.pba_bar as usize;
    let table_in = table_bar_idx < handle.bars.len();
    let pba_in = pba_bar_idx < handle.bars.len();
    let table_mmio = table_in && is_mmio_bar(&handle.bars[table_bar_idx]);
    let pba_mmio = pba_in && is_mmio_bar(&handle.bars[pba_bar_idx]);
    MsixHandleView {
        msix_present: true,
        msix_table_size: msix.vector_count(),
        table_bar_in_range: table_in,
        table_bar_is_mmio: table_mmio,
        pba_bar_in_range: pba_in,
        pba_bar_is_mmio: pba_mmio,
    }
}

fn is_mmio_bar(bar: &PciBar) -> bool {
    bar.is_present() && bar.is_memory() && bar.address().is_some()
}
