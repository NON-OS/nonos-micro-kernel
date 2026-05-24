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

use super::args::Args;
use crate::syscall::microkernel::debug::sys_mk_debug;
use crate::syscall::microkernel::numbers::*;
use crate::syscall::microkernel::pci::{sys_pci_config_read, sys_pci_config_write};

pub(super) fn handle(nr: u64, a: Args) -> Option<i64> {
    Some(match nr {
        SYS_MK_DEBUG => sys_mk_debug(a.a0, a.a1),
        SYS_PCI_CONFIG_READ => sys_pci_config_read(a.a0, a.a1, a.a2 as u32, a.a3 as u32),
        SYS_PCI_CONFIG_WRITE => sys_pci_config_write(a.a0, a.a1, a.a2 as u32, a.a3 as u32),
        _ => return None,
    })
}
