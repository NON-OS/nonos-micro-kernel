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

pub mod all_fields;
pub mod blocks;
pub mod blocks_for;
pub mod coverage;
pub mod read_only;
pub mod rows;
pub mod section_fields;

pub use all_fields::ALL_FIELDS;
pub use blocks_for::blocks_for;
pub use read_only::read_only;
pub use section_fields::{field_at, field_count};
