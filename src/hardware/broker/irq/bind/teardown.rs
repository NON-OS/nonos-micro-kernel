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

use super::super::msix_ops::current_ops;
use crate::hardware::broker::pci_index;

pub(crate) fn teardown_msix_vector(device_id: u64, device_vector: u16) {
    let Some(handle) = pci_index::lookup(device_id) else { return };
    let Some(msix) = handle.msix else { return };
    current_ops().teardown_vector(&handle.address, &msix, &handle.bars, device_vector);
}

pub(crate) fn disable_msix_for_device(device_id: u64) {
    let Some(handle) = pci_index::lookup(device_id) else { return };
    let Some(msix) = handle.msix else { return };
    current_ops().disable_for_device(&handle.address, &msix);
}
