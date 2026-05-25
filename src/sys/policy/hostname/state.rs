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

use crate::sys::sync::IrqMutex;

pub(super) const NAME_MAX: usize = 64;

pub(super) struct HostnameState {
    pub(super) hostname: [u8; NAME_MAX],
    pub(super) hostname_len: usize,
    pub(super) domainname: [u8; NAME_MAX],
    pub(super) domainname_len: usize,
}

pub(super) static STATE: IrqMutex<HostnameState> = IrqMutex::new(HostnameState {
    hostname: [0; NAME_MAX],
    hostname_len: 0,
    domainname: [0; NAME_MAX],
    domainname_len: 0,
});
