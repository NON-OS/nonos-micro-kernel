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

//! Trading one asset for another.

mod quote;
mod scale;
mod text;
mod token;

pub use quote::{
    amount_out, apply_slippage, impact_bps, is_dangerous, is_warning, quote, Quote, Reserves, BPS,
};
pub use scale::scaled;
pub use text::{
    amount_text, bps_text, gas_text, min_out_text, rate_text, route_text, slippage_text,
};
pub use token::{count, token, Token};
