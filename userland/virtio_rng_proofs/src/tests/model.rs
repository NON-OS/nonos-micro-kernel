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

//! A virtio-rng device as the window shows it before the driver arrives.

use nonos_devmodel::FakeBar;

use crate::constants::*;

const WINDOW: usize = 256;

/// A device advertising a queue of `qmax` entries and nothing else.
pub fn device(qmax: u16) -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    bar.present16(LEG_QUEUE_NUM, qmax);
    bar
}
