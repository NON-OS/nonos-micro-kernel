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

use super::constants::{GICR_WAKER, WAKER_CHILDREN_ASLEEP, WAKER_PROCESSOR_SLEEP};
use super::device::GicRedistributor;

impl GicRedistributor {
    pub(super) fn wake(&self) {
        let mut waker = self.read_reg(GICR_WAKER);
        if waker & WAKER_PROCESSOR_SLEEP == 0 {
            return;
        }
        waker &= !WAKER_PROCESSOR_SLEEP;
        self.write_reg(GICR_WAKER, waker);
        while self.read_reg(GICR_WAKER) & WAKER_CHILDREN_ASLEEP != 0 {
            core::hint::spin_loop();
        }
    }
}
