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

use super::class::ExceptionClass;

#[derive(Debug, Clone, Copy)]
pub struct EsrDecoded {
    pub class: ExceptionClass,
    pub il: bool,
    pub iss: u32,
}

pub fn decode_esr(esr: u64) -> EsrDecoded {
    let ec = ((esr >> 26) & 0x3F) as u8;
    let il = (esr & (1 << 25)) != 0;
    let iss = (esr & 0x01FF_FFFF) as u32;
    EsrDecoded { class: ExceptionClass::from(ec), il, iss }
}
