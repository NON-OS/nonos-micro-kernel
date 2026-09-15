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

//! What the device understood the host to have asked, in order. A proof
//! reads the driver's wake sequence off this rather than off the bytes.

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Command {
    /// SET_POWER (opcode 8). `sleep` is the power state in the command's low
    /// bits: zero is ON.
    SetPower { sleep: bool },
    /// RESET (opcode 1). The device answers with a zero-length input report
    /// the host has to read before anything else arrives.
    Reset,
    /// GET_REPORT (opcode 2) of report `id`, type in `ty` (3 is Feature).
    GetReport { ty: u8, id: u8 },
    /// SET_REPORT (opcode 3). `data` is the report as sent, id first.
    SetReport { ty: u8, id: u8, data: Vec<u8> },
    /// A register-addressed read of something other than the command path:
    /// the HID descriptor, the report descriptor, or the input register.
    RegisterRead(u16),
    /// A plain read with no register addressed, which the specification
    /// defines as a read of the input register.
    InputRead,
    /// A register this device does not have.
    UnknownRegister(u16),
}
