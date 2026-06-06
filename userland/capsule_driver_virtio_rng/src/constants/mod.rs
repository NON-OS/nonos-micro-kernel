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

mod pci;
mod queue;
mod regs;
mod status;

pub use pci::{VIRTIO_RNG_MODERN, VIRTIO_RNG_TRANSITIONAL, VIRTIO_VENDOR_ID};
pub use queue::{
    ENTROPY_BUF_LEN, QUEUE_SIZE, VQ_DESC_OFFSET, VQ_REGION_SIZE, VQ_USED_OFFSET, VRING_DESC_F_WRITE,
};
pub use regs::{
    LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_NOTIFY, LEG_QUEUE_NUM, LEG_QUEUE_PFN,
    LEG_QUEUE_SEL, LEG_STATUS,
};
pub use status::{
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};
