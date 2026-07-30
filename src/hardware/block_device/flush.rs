// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::backend::Backend;
use super::map_ahci::map_ahci_error;
use super::map_nvme::map_nvme_error;
use super::map_virtio::map_virtio_error;
use super::select::selected;
use super::BlockDeviceError;

pub fn flush() -> Result<(), BlockDeviceError> {
    match selected() {
        Backend::VirtioBlk => {
            crate::hardware::virtio_blk_capsule::flush().map_err(map_virtio_error)
        }
        Backend::Ahci => crate::hardware::ahci_capsule::flush().map_err(map_ahci_error),
        Backend::Nvme => crate::hardware::nvme_capsule::flush().map_err(map_nvme_error),
    }
}
