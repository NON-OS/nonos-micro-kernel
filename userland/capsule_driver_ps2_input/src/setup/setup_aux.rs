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

use super::claim::claim;
use super::irq::bind_raw as irq_bind_raw;
use crate::discover::find_ps2_aux;

pub(super) fn setup_aux() -> u64 {
    let Some(aux) = find_ps2_aux() else {
        return 0;
    };
    let Ok(epoch) = claim(aux.device_id) else {
        return 0;
    };
    match irq_bind_raw(aux, epoch) {
        Ok(out) => out.grant_id,
        Err(_) => {
            let _ = nonos_libc::mk_device_release(aux.device_id);
            0
        }
    }
}
