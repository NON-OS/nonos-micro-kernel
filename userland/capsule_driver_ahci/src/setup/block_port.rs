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

use crate::constants::PORT_KIND_SATA;
use crate::controller::PortInfo;
use crate::engine::{init_port, Port};
use crate::regs::Regs;

pub(super) fn bring_up(
    device_id: u64,
    claim_epoch: u64,
    regs: Regs,
    ports: &[PortInfo],
) -> Option<Port> {
    for p in ports {
        if p.present == 1 && p.kind == PORT_KIND_SATA {
            if let Ok(port) = init_port(device_id, claim_epoch, regs, p.index) {
                return Some(port);
            }
        }
    }
    None
}
