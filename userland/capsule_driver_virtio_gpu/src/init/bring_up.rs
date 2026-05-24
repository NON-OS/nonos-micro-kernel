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

use super::legacy::bring_up_legacy;
use super::modern::bring_up_modern;
use super::types::InitOut;
use crate::constants::VIRTIO_GPU_MODERN;
use crate::regs::Regs;

pub fn bring_up(regs: Regs, queue_phys: u64, pci_device: u16) -> Result<InitOut, &'static str> {
    if pci_device == VIRTIO_GPU_MODERN {
        bring_up_modern(regs, queue_phys)
    } else {
        bring_up_legacy(regs, queue_phys)
    }
}
