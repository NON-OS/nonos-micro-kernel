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

mod builder;
mod call;
mod call_payload;
mod constants;
mod read_i32;
mod read_u32;
mod read_u64;

pub use builder::build_request;
pub use call::call_status;
pub use call_payload::call_payload;
pub use constants::{HDR_LEN, NCLP_MAGIC, NCMP_MAGIC, NINP_MAGIC, NIRS_MAGIC, NWMP_MAGIC, VERSION};
pub use read_i32::read_i32;
pub use read_u32::read_u32;
pub use read_u64::read_u64;
