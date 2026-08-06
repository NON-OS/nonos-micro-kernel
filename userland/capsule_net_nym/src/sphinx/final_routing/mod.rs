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

//! The last hop's routing block.

mod add_padding;
mod combine;
mod encrypt;
mod padded_len;
mod types;

pub use add_padding::add_padding;
pub use combine::combine_with_filler;
pub use encrypt::encrypt_final;
pub use padded_len::padded_len;
pub use types::FinalRoutingInformation;
