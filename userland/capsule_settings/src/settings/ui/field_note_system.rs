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

use nonos_policy_proto::Field;

pub fn note(field: Field) -> Option<&'static str> {
    Some(match field {
        Field::KernelAslr => "Randomise the kernel's virtual layout each boot.",
        Field::KernelStackGuard => "Trap on stack overflow with a guard page.",
        Field::KernelNxBit => "Refuse execution from writable pages.",
        Field::KernelSmep => "Block the kernel from running user-mode pages.",
        Field::KernelSmap => "Block stray kernel reads of user memory.",
        Field::KernelIommu => "Confine device DMA to granted pages.",
        Field::KernelSeccomp => "Restrict capsules to their declared syscalls.",
        Field::KernelWatchdog => "Reset the machine if the scheduler stalls.",
        Field::KernelDebug => "Emit kernel debug records on the serial line.",
        Field::KernelSerial => "Mirror kernel logging to the serial port.",
        Field::KernelPreempt => "Preempt kernel threads on the timer tick.",
        Field::KernelHugepages => "Back large mappings with 2 MiB pages.",
        Field::Hostname => "The name this machine announces on a network.",
        Field::DomainName => "The domain this machine reports itself under.",
        _ => return None,
    })
}
