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

mod api;
mod device;
mod init;
mod queue;

use core::sync::atomic::AtomicBool;
use device::VirtioRngDevice;
use spin::Mutex;

static VIRTIO_RNG: Mutex<Option<VirtioRngDevice>> = Mutex::new(None);
static VIRTIO_RNG_AVAILABLE: AtomicBool = AtomicBool::new(false);

pub const VIRTIO_VENDOR_ID: u16 = 0x1AF4;
pub const VIRTIO_RNG_DEVICE_ID_TRANSITIONAL: u16 = 0x1005;
pub const VIRTIO_RNG_DEVICE_ID_MODERN: u16 = 0x1044;

pub use api::{fill_random, get_random_bytes, is_available};
pub use init::init as init_virtio_rng;
