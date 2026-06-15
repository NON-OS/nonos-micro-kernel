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

#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SalStatus {
    Ok = 0,
    Locked = 1,
    InvalidInput = 2,
    NotSynced = 3,
    Network = 4,
    Crypto = 5,
    Storage = 6,
    Policy = 7,
}
