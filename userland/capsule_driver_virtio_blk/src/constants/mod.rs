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
mod request;
mod status;
pub use pci::{VIRTIO_BLK_MODERN, VIRTIO_BLK_TRANSITIONAL, VIRTIO_VENDOR_ID};
pub use queue::{
    DATA_BUF_LEN, HEADER_BUF_LEN, HEADER_OFFSET, MAX_QUEUE_SIZE, MAX_SECTORS_PER_REQUEST,
    SECTOR_SIZE, STATUS_OFFSET, VQ_DESC_OFFSET, VQ_REGION_SIZE, VRING_DESC_F_NEXT,
    VRING_DESC_F_WRITE,
};
pub use regs::{
    LEG_CFG_CAPACITY, LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_NOTIFY, LEG_QUEUE_NUM,
    LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS,
};
pub use request::{
    VIRTIO_BLK_S_IOERR, VIRTIO_BLK_S_OK, VIRTIO_BLK_S_UNSUPP, VIRTIO_BLK_T_FLUSH, VIRTIO_BLK_T_IN,
    VIRTIO_BLK_T_OUT,
};
pub use status::{
    STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED, STATUS_FEATURES_OK,
};
