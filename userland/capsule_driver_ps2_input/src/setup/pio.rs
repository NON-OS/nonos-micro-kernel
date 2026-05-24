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
use nonos_libc::{mk_device_release, mk_pio_grant, PioGrantOut};
const BAR_INDEX: u8 = 0;
pub fn grant(device_id: u64, claim_epoch: u64) -> Result<PioGrantOut, &'static str> {
    let mut out = PioGrantOut { port_base: 0, port_count: 0, _pad: 0, grant_id: 0 };
    let r = mk_pio_grant(device_id, claim_epoch, BAR_INDEX, 0, &mut out);
    if r < 0 {
        let _ = mk_device_release(device_id);
        return Err("pio grant failed");
    }
    Ok(out)
}