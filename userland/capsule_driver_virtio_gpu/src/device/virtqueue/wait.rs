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

use nonos_libc::mk_yield;

use super::{layout::QueueLayout, used};

const SPINS_PER_YIELD: u32 = 1024;
const YIELD_LIMIT: u32 = 8192;

pub fn used_changed(layout: QueueLayout, pre_used_idx: u16) -> Result<(), &'static str> {
    for _ in 0..YIELD_LIMIT {
        for _ in 0..SPINS_PER_YIELD {
            if used::read_idx(layout) != pre_used_idx {
                return Ok(());
            }
            core::hint::spin_loop();
        }
        if mk_yield() < 0 {
            return Err("virtio-gpu: yield failed");
        }
    }
    Err("virtio-gpu: device timeout")
}
