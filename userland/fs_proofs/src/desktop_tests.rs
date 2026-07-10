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

//! Proofs for the desktop's direct-child classifier. This is the logic that
//! decides which filesystem entries become icons on the desktop, so getting it
//! wrong shows up as an empty or wrongly-populated desktop. The cases below pin
//! it against the real seeded root and the paths the server actually emits.

use crate::{desktop_child, desktop_walk};
use alloc::vec::Vec;

// Build a length-prefixed name buffer the way the vfs service frames a reply.
fn framed(names: &[&str]) -> Vec<u8> {
    let mut out = Vec::new();
    for n in names {
        out.push(n.len() as u8);
        out.extend_from_slice(n.as_bytes());
    }
    out
}

// A file directly under the root keeps its name and reads as a file.
#[test]
fn root_file_is_kept() {
    assert_eq!(desktop_child("/", "/readme.txt"), Some(("readme.txt".into(), false)));
}

// A directory arrives with a trailing slash; we strip it and flag it a dir.
#[test]
fn root_dir_strips_slash_and_flags_dir() {
    assert_eq!(desktop_child("/", "/docs/"), Some(("docs".into(), true)));
}

// Nested entries belong to a subfolder, not the desktop, so they drop out.
#[test]
fn nested_entries_drop() {
    assert_eq!(desktop_child("/", "/docs/about.txt"), None);
    assert_eq!(desktop_child("/", "/capsules/std_proof.elf"), None);
}

// The root itself is not one of its own children.
#[test]
fn root_itself_drops() {
    assert_eq!(desktop_child("/", "/"), None);
}

// The exact seeded root must yield exactly three icons: readme, docs, capsules.
// This is the case that would have caught the desktop going blank.
#[test]
fn seeded_root_yields_three_icons() {
    let reply = [
        "/readme.txt",
        "/docs/",
        "/capsules/",
        "/docs/about.txt",
        "/docs/demo.txt",
        "/capsules/std_proof.elf",
    ];
    let kept: alloc::vec::Vec<_> = reply.iter().filter_map(|p| desktop_child("/", p)).collect();
    assert_eq!(kept.len(), 3);
    assert_eq!(kept[0], ("readme.txt".into(), false));
    assert_eq!(kept[1], ("docs".into(), true));
    assert_eq!(kept[2], ("capsules".into(), true));
}

// The classifier must also cope with a server that answers relative names.
#[test]
fn relative_names_still_classify() {
    assert_eq!(desktop_child("/", "docs/"), Some(("docs".into(), true)));
    assert_eq!(desktop_child("/", "readme.txt"), Some(("readme.txt".into(), false)));
}

// A non-root prefix lists that folder's own children by basename.
#[test]
fn subfolder_prefix_lists_children() {
    assert_eq!(desktop_child("/docs/", "/docs/about.txt"), Some(("about.txt".into(), false)));
    assert_eq!(desktop_child("/docs/", "/docs/"), None);
}

// A freshly created "New Folder" keeps the space in its name.
#[test]
fn new_folder_name_with_space() {
    assert_eq!(desktop_child("/", "/New Folder/"), Some(("New Folder".into(), true)));
}

// A well-formed reply yields exactly its entries.
#[test]
fn walk_reads_valid_entries() {
    let buf = framed(&["/a", "/b/"]);
    let out = desktop_walk(&buf, 0, buf.len());
    assert_eq!(out, [("a".into(), false), ("b".into(), true)]);
}

// A length that runs past the reply stops the walk instead of reading over the
// end. The entries seen before the bad length are still returned.
#[test]
fn walk_stops_on_length_past_end() {
    let mut buf = framed(&["/a"]);
    buf.push(200); // claims a 200-byte name with nothing behind it
    buf.push(b'x');
    let out = desktop_walk(&buf, 0, buf.len());
    assert_eq!(out, [("a".into(), false)]);
}

// Zero padding, which fills the tail of the real reply buffer, produces nothing.
#[test]
fn walk_ignores_zero_padding() {
    let buf = [0u8; 32];
    assert!(desktop_walk(&buf, 0, buf.len()).is_empty());
}

// An end index past the buffer is clamped, never read out of range.
#[test]
fn walk_clamps_end_past_buffer() {
    let buf = framed(&["/a"]);
    let out = desktop_walk(&buf, 0, 9999);
    assert_eq!(out, [("a".into(), false)]);
}

// A start beyond the end yields nothing rather than underflowing.
#[test]
fn walk_empty_when_start_past_end() {
    let buf = framed(&["/a", "/b"]);
    assert!(desktop_walk(&buf, 10, 4).is_empty());
}

// A flood of entries is capped so a hostile reply cannot exhaust memory.
#[test]
fn walk_caps_entry_count() {
    let names: Vec<&str> = core::iter::repeat("/a").take(5000).collect();
    let buf = framed(&names);
    assert_eq!(desktop_walk(&buf, 0, buf.len()).len(), 4096);
}
