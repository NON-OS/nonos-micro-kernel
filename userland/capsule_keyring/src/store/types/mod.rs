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
mod constants;
mod key_entry;
mod key_metadata;
mod key_type;
mod key_type_from_u8;
mod key_type_to_u8;
mod store;
mod store_error;

pub use constants::{MAX_KEYS, MAX_KEY_SIZE};
pub(in crate::store) use key_entry::KeyEntry;
pub use key_metadata::KeyMetadata;
pub use key_type::KeyType;
pub use store::Store;
pub use store_error::StoreError;
