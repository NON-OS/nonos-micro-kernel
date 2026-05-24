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

use crate::syscall::abi::{tag4, AbiDomain, AbiEntry, AbiStatus};
use crate::syscall::numbers::SyscallNumber;

pub(super) const ENTRIES: &[AbiEntry] = &[
    r(b"ARBT", SyscallNumber::AdminReboot, "AdminReboot"),
    r(b"ASDN", SyscallNumber::AdminShutdown, "AdminShutdown"),
    u(b"AMOD", SyscallNumber::AdminModLoad, "AdminModLoad"),
];

const fn r(tag: &[u8; 4], variant: SyscallNumber, name: &'static str) -> AbiEntry {
    AbiEntry {
        id: tag4(tag),
        variant,
        name,
        domain: AbiDomain::Admin,
        status: AbiStatus::Routed,
    }
}

const fn u(tag: &[u8; 4], variant: SyscallNumber, name: &'static str) -> AbiEntry {
    AbiEntry {
        id: tag4(tag),
        variant,
        name,
        domain: AbiDomain::Admin,
        status: AbiStatus::Unavailable,
    }
}
