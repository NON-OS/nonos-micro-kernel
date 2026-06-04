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
use crate::constants::IMAN_IP;
use crate::regs::runtime::{iman_read, iman_write};
use nonos_libc::mk_irq_ack;
pub fn ack_irq(intr_base: u64, irq_grant_id: u64) {
    let cur = iman_read(intr_base);
    if cur & IMAN_IP != 0 {
        iman_write(intr_base, cur | IMAN_IP);
    }
    let _ = mk_irq_ack(irq_grant_id);
}
