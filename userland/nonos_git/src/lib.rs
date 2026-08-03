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

//! Git's object model for the NONOS terminal, from scratch and `no_std`.
//!
//! A git repository is a content-addressed store: every blob, tree and commit
//! is named by the SHA-1 of its framed bytes, `<type> <size>\0<content>`. This
//! crate is that model, packfiles both ways, the wire protocol, and clone and
//! push on top of a transport it does not implement. It has to agree with real
//! git bit for bit, so it is checked against real git rather than against
//! itself.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod commit;
pub(crate) mod config;
mod index;
mod object;
mod odb;
mod oid;
mod pack;
mod refs;
mod remote;
mod repo;
mod sha1;
mod storage;
mod transport;
mod tree;
mod wire;
mod zlib;

pub use commit::{parse as parse_commit, Commit, CommitError, Signature};
pub use config::{remote_url, set_remote};
pub use index::{IndexEntry, IndexError};
pub use object::{frame, unframe, ObjectKind};
pub use odb::{read_object, write_object, OdbError};
pub use oid::ObjectId;
pub use pack::{read_pack, write_pack, write_pack_index, PackError, PackObject};
pub use refs::{is_valid_ref_name, read_head, resolve_head, set_head_branch, update_ref, Head};
pub use remote::{clone, discover, fetch, push};
pub use repo::{
    add, checkout, clone_into, commit, init, log, objects_to_send, read_index, store_pack,
    write_index, write_tree, CloneRequest, CommitRequest, LogEntry, RepoError,
};
pub use sha1::Sha1;
pub use storage::{Storage, StorageError};
pub use transport::{Transport, TransportError};
pub use tree::{parse as parse_tree, Mode, TreeEntry, TreeError};
pub use wire::{
    encode_pkt, parse_advertisement, push_request, read_pkt, want_request, Pkt, RefUpdate,
    RemoteRef, WireError,
};
pub use zlib::{compress, decompress, InflateError};

/// Encode a commit's content bytes.
pub use commit::encode as encode_commit;
/// Encode a tree's content bytes, sorting the entries into git's order.
pub use tree::encode as encode_tree;
