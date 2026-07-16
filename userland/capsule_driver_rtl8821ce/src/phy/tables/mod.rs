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

//! The RTL8821CE baseband, AGC, RF and PHY-MAC register tables, carried verbatim
//! from rtw88 in the PHY-condition format the parser in `phy::cond` walks. Each
//! is an opaque address/value stream extracted mechanically from
//! rtw8821c_table.c, so the values are exactly the vendor's.

mod agc;
mod bb;
mod mac;
mod rf_a;

pub use agc::RTL8821C_AGC;
pub use bb::RTL8821C_BB;
pub use mac::RTL8821C_MAC;
pub use rf_a::RTL8821C_RF_A;
