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
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{
    read_u32_le, read_u64_le, Request, E_INVAL, E_MSGSIZE, E_NXIO, READ_REQ_LEN,
};
use crate::setup::Driver;

pub(super) struct ReadRequest {
    pub lba: u64,
    pub nsectors: u32,
    pub bytes_n: usize,
}

pub(super) fn read_request(
    driver: &Driver,
    req: &Request,
    body: &[u8],
) -> Result<ReadRequest, i32> {
    if body.len() < READ_REQ_LEN || req.payload_len != READ_REQ_LEN as u32 {
        return Err(E_MSGSIZE);
    }
    let lba = read_u64_le(body, 0).ok_or(E_MSGSIZE)?;
    let nsectors = read_u32_le(body, 8).ok_or(E_MSGSIZE)?;
    if nsectors == 0 || nsectors > MAX_SECTORS_PER_REQUEST {
        return Err(E_INVAL);
    }
    let last = lba.checked_add(nsectors as u64).ok_or(E_INVAL)?;
    if last > driver.capacity_sectors {
        return Err(E_NXIO);
    }
    Ok(ReadRequest { lba, nsectors, bytes_n: nsectors as usize * SECTOR_SIZE })
}
