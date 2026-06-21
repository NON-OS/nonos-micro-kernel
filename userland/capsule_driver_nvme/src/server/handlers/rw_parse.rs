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

use crate::nvm::MAX_SECTORS;
use crate::protocol::{E_INVAL, E_MSGSIZE, E_NXIO, RW_HEADER_LEN};

pub(super) fn parse(body: &[u8], capacity: u64) -> Result<(u64, u32), i32> {
    if body.len() < RW_HEADER_LEN {
        return Err(E_MSGSIZE);
    }
    let lba = u64::from_le_bytes(body[0..8].try_into().map_err(|_| E_MSGSIZE)?);
    let nsectors = u32::from_le_bytes(body[8..12].try_into().map_err(|_| E_MSGSIZE)?);
    if nsectors == 0 || nsectors > MAX_SECTORS {
        return Err(E_INVAL);
    }
    let last = lba.checked_add(nsectors as u64).ok_or(E_INVAL)?;
    if last > capacity {
        return Err(E_NXIO);
    }
    Ok((lba, nsectors))
}
