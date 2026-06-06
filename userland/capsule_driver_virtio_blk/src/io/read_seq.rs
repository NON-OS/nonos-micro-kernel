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
use super::error::BlkError;
use nonos_libc::{mk_irq_poll, IrqPollOut};

pub(super) fn read_seq(grant: u64) -> Result<u64, BlkError> {
    let mut out = IrqPollOut { seq: 0, overflow: 0 };
    if mk_irq_poll(grant, &mut out as *mut _) < 0 {
        return Err(BlkError::Io);
    }
    Ok(out.seq)
}
