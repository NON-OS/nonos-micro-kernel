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
use super::grant_pio::grant_pio;
use super::map_mmio::map_mmio;
use super::map_modern::map_modern;
use super::state::RegisterGrant;
use crate::constants::VIRTIO_GPU_MODERN;
use crate::discover::Found;
use nonos_libc::{BAR_KIND_MMIO, BAR_KIND_PIO};
pub fn grant(dev: Found, claim_epoch: u64) -> Result<RegisterGrant, &'static str> {
    if dev.pci_device == VIRTIO_GPU_MODERN {
        return map_modern(dev, claim_epoch);
    }
    match dev.register_kind {
        BAR_KIND_MMIO => map_mmio(dev, claim_epoch),
        BAR_KIND_PIO => grant_pio(dev, claim_epoch),
        _ => Err("virtio-gpu: unsupported register bar"),
    }
}
