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

//! Host-runnable proofs for the pure filesystem logic. Each `#[path]` include
//! pulls in the real capsule source so the tests exercise production code, not
//! a copy of it. Run with `cargo test` from this directory.

extern crate alloc;

use alloc::string::String;

// Real capsule source under test, included verbatim.
#[path = "../../capsule_file_manager/src/fm/fmt_time.rs"]
mod fmt_time_src;
#[path = "../../capsule_file_manager/src/fm/human_size.rs"]
mod human_size_src;
#[path = "../../capsule_vfs/src/server/handlers/path/mod.rs"]
mod vfs_path;

// The real vfs store logic, assembled for the host with a clock shim.
pub mod vfs_store;

// Self-contained file-manager logic (listing parser, type classifier).
pub mod fm_logic;

// Real network wire parsers for untrusted-input coverage.
pub mod net;

// Machine-checked proof harnesses, compiled only under `cargo kani`.
#[cfg(kani)]
mod kani_proofs;

// The vfs wire protocol codec.
pub mod vfs_protocol;

// The block-layer error taxonomy the vfs handlers turn into errnos.
pub mod vfs_blk;

// Aliases so capsule source that uses crate-absolute paths (`crate::protocol`,
// `crate::store`, `crate::blk`) resolves when included here.
pub use vfs_blk as blk;
pub use vfs_protocol as protocol;
pub use vfs_store as store;

// Caller attestation and errno mapping, from the vfs request handlers.
#[path = "../../capsule_vfs/src/server/handlers/util.rs"]
mod vfs_util;

// Public surface for the attestation helpers so they are exercised as API.
pub fn split_caller(payload: &[u8], sender_pid: u32) -> Result<(u32, &[u8]), i32> {
    vfs_util::split_caller(payload, sender_pid)
}
pub fn map_blk_err(err: blk::error::BlkError) -> i32 {
    vfs_util::map_blk_err(err)
}
pub fn map_store_err(err: store::StoreError) -> i32 {
    vfs_util::map_store_err(err)
}

// Public surface so the included production functions are part of this crate's
// API and exercised as such, not flagged unused outside the test build.
pub fn normalize(path: &str) -> String {
    vfs_path::normalize(path)
}
pub fn normalize_to_buffer(src: &[u8], out: &mut [u8]) -> usize {
    vfs_path::normalize_to_buffer(src, out)
}
pub fn is_read_only(path: &str) -> bool {
    vfs_path::is_read_only(path)
}
pub use fmt_time_src::fmt_time;
pub use human_size_src::human_size;

// The desktop's direct-child classifier: the logic that keeps only top-level
// entries on the desktop and drops nested paths. Included from the shell capsule
// so the proof pins the real function, not a copy.
#[path = "../../capsule_desktop_shell/src/vfs_client/classify.rs"]
mod desktop_classify;

pub fn desktop_child(prefix: &str, raw: &str) -> Option<(String, bool)> {
    desktop_classify::classify(prefix, raw)
}

// The framing walk over a LIST reply, which reads bytes the vfs service sent.
// Included from the shell capsule so the bounds-safety proof pins the real code.
#[path = "../../capsule_desktop_shell/src/vfs_client/walk.rs"]
mod desktop_walk_src;

pub fn desktop_walk(rx: &[u8], start: usize, end: usize) -> alloc::vec::Vec<(String, bool)> {
    desktop_walk_src::walk(rx, start, end, |raw| desktop_classify::classify("/", raw))
}

#[cfg(test)]
mod desktop_tests;
#[cfg(test)]
mod fm_tests;
#[cfg(test)]
mod fmt_tests;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod net_tests;
#[cfg(test)]
mod protocol_tests;
#[cfg(test)]
mod store_tests;
#[cfg(test)]
mod util_tests;
#[cfg(test)]
mod vfs_path_tests;
