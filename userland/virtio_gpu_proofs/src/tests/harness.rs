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

//! A ring region and a register window to notify through.

use std::sync::Arc;

use nonos_devmodel::FakeBar;

use super::model::QUEUE_SIZE;
use crate::constants::VQ_REGION_SIZE;
use crate::device::virtqueue::{ControlQueue, QueueLayout};
use crate::regs::Regs;

/// A ring region and a legacy-shaped register window to notify through. The
/// region's device address is its host address, so the part reads the
/// descriptors' addresses directly.
pub struct Harness {
    pub region: Arc<FakeBar>,
    pub regs: Arc<FakeBar>,
    pub queue: ControlQueue,
}

pub fn harness() -> Harness {
    let region = Arc::new(FakeBar::new(VQ_REGION_SIZE as usize));
    let regs = Arc::new(FakeBar::new(0x40));
    let layout =
        QueueLayout::new(QUEUE_SIZE, region.base(), region.base()).expect("a valid layout");
    let queue = ControlQueue::new(layout, Regs::mmio(regs.base()));
    Harness { region, regs, queue }
}
