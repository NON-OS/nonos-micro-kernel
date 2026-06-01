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

use crate::device::cmd;
use crate::device::cmd::transfer_to_host_2d::Rect;
use crate::device::virtqueue::ControlQueue;
use crate::state::{FenceCounter, Scanout};

pub fn display(
    q: &ControlQueue,
    fences: &FenceCounter,
    resource_id: u32,
    scanout: Scanout,
) -> Result<(), &'static str> {
    let rect = Rect { x: 0, y: 0, width: scanout.width, height: scanout.height };
    cmd::transfer_to_host_2d(q, fences.issue(), resource_id, rect, 0)?;
    cmd::set_scanout(q, fences.issue(), 0, resource_id, rect)?;
    cmd::resource_flush(q, fences.issue(), resource_id, rect)
}
