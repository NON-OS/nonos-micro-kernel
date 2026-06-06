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

mod cache;
mod header;
mod name;
mod query;
mod response;
mod types;

pub use cache::Cache;
pub use header::{Header, HDR_LEN, RCODE_NO_ERROR, RCODE_NXDOMAIN};
pub use name::{skip, NameError};
pub use query::{build_a_query, build_aaaa_query};
pub use response::{first_address, Answer};
pub use types::{LABEL_MAX, NAME_MAX, POINTER_MASK, TYPE_A, TYPE_AAAA};
