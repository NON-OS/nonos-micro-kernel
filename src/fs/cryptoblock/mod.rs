// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

mod constants;
mod error;
mod map_block;
mod open;
mod read;
mod seal;
mod write;

pub use constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
pub use error::CryptoBlockError;
pub use open::open;
pub use read::read;
pub use seal::seal;
pub use write::write;
