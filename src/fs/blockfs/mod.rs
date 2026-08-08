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

mod alloc_block;
mod child_node;
mod commit;
mod constants;
mod create;
mod deserialize;
mod deserialize_node;
mod digest;
mod dir_consts;
mod dir_link;
mod dir_lookup;
mod dir_name_match;
mod dir_record_alloc;
mod dir_unlink;
mod error;
mod file_consts;
mod file_write_data;
mod format;
mod header_lba;
mod mount;
mod new_superblock;
mod node_types;
mod path_create;
mod path_read;
mod path_unlink;
mod path_write;
mod read_file;
mod read_header;
mod read_node;
mod read_root;
mod read_u16;
mod read_u32;
mod read_u64;
mod resolve;
mod root_node;
mod select_newer;
mod serialize;
mod serialize_node;
mod split_parent;
mod types;
mod validate_geometry;
mod write_file;
mod write_node;
mod write_u16;
mod write_u32;
mod write_u64;

pub use alloc_block::alloc_block;
pub use commit::commit;
pub use constants::{MODE_DIR, MODE_FILE};
pub use create::create;
pub use dir_link::link;
pub use dir_lookup::lookup;
pub use error::BlockFsError;
pub(crate) use file_consts::MAX_FILE_BYTES;
pub use format::format;
pub use mount::mount;
pub use node_types::BlockFsNode;
pub use path_create::create_path;
pub use path_read::read_path;
pub use path_unlink::unlink_path;
pub use path_write::write_path;
pub use read_file::read_file;
pub use read_node::read_node;
pub use read_root::read_root;
pub use resolve::resolve;
pub use root_node::root_node;
pub use types::{BlockFsMount, BlockFsSuperblock};
pub use write_file::write_file;
