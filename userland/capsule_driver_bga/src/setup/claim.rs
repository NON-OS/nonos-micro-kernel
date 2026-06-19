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

use nonos_libc::mk_device_claim;

use crate::error::{BgaError, BgaResult};

pub fn claim(device_id: u64) -> BgaResult<u64> {
    let epoch = mk_device_claim(device_id);
    if epoch < 0 {
        return Err(BgaError::BrokerCallFailed(epoch));
    }
    Ok(epoch as u64)
}
