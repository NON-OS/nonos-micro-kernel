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

use alloc::vec;
use alloc::vec::Vec;

use crate::vfs_store::{Store, StoreError};

const PID: u32 = 1;

// Create or overwrite a file with `data` through the real open/write/close path.
fn put(store: &mut Store, path: &str, data: &[u8]) {
    let fd = store.open(path, PID, true, true, false, true).expect("open create");
    store.write(fd, PID, data).expect("write");
    store.close(fd, PID).expect("close");
}

// Read a file's whole content back.
fn get(store: &mut Store, path: &str) -> Vec<u8> {
    let fd = store.open(path, PID, false, false, false, true).expect("open read");
    let data = store.read(fd, PID, 1 << 20).expect("read");
    store.close(fd, PID).expect("close");
    data
}

#[test]
fn mkdir_p_creates_missing_ancestors() {
    let mut s = Store::new();
    s.mkdir("/a/b/c").unwrap();
    assert!(s.stat("/a").unwrap().1);
    assert!(s.stat("/a/b").unwrap().1);
    assert!(s.stat("/a/b/c").unwrap().1);
}

#[test]
fn mkdir_existing_is_rejected() {
    let mut s = Store::new();
    s.mkdir("/a").unwrap();
    assert_eq!(s.mkdir("/a"), Err(StoreError::Exists));
}

#[test]
fn write_then_read_roundtrips() {
    let mut s = Store::new();
    put(&mut s, "/f", b"hello world");
    assert_eq!(get(&mut s, "/f"), b"hello world");
    assert_eq!(s.stat("/f").unwrap().0, 11);
}

#[test]
fn default_modes_are_file_644_dir_755() {
    let mut s = Store::new();
    put(&mut s, "/f", b"");
    s.mkdir("/d").unwrap();
    assert_eq!(s.stat("/f").unwrap().3, 0o644);
    assert_eq!(s.stat("/d").unwrap().3, 0o755);
}

#[test]
fn chmod_readonly_blocks_writes_and_truncate() {
    let mut s = Store::new();
    put(&mut s, "/f", b"hello");
    s.chmod("/f", 0o444).unwrap();
    // A handle on a read-only file carries no write permission.
    let fd = s.open("/f", PID, false, false, false, true).unwrap();
    assert_eq!(s.write(fd, PID, b"x"), Err(StoreError::AccessDenied));
    // Opening with O_TRUNC on a read-only file is refused before clearing it.
    assert_eq!(s.open("/f", PID, false, true, false, true), Err(StoreError::AccessDenied));
    // The content is untouched.
    assert_eq!(get(&mut s, "/f"), b"hello");
}

#[test]
fn truncate_shrinks_and_zero_grows_without_stale_data() {
    let mut s = Store::new();
    put(&mut s, "/f", b"abcdef");
    s.truncate("/f", 3).unwrap();
    assert_eq!(s.stat("/f").unwrap().0, 3);
    assert_eq!(get(&mut s, "/f"), b"abc");
    // Growing back must zero-fill, never resurrect the dropped "def".
    s.truncate("/f", 6).unwrap();
    assert_eq!(get(&mut s, "/f"), b"abc\0\0\0");
}

#[test]
fn copy_file_duplicates_content() {
    let mut s = Store::new();
    put(&mut s, "/f", b"payload");
    s.copy("/f", "/g", false).unwrap();
    assert_eq!(get(&mut s, "/g"), b"payload");
    // Destination must not already exist.
    assert_eq!(s.copy("/f", "/g", false), Err(StoreError::Exists));
}

#[test]
fn copy_dir_recursive_rewrites_whole_subtree() {
    let mut s = Store::new();
    s.mkdir("/d").unwrap();
    put(&mut s, "/d/x", b"1");
    s.mkdir("/d/sub").unwrap();
    put(&mut s, "/d/sub/y", b"2");
    s.copy("/d", "/d2", true).unwrap();
    assert!(s.stat("/d2").unwrap().1);
    assert_eq!(get(&mut s, "/d2/x"), b"1");
    assert_eq!(get(&mut s, "/d2/sub/y"), b"2");
    // Original is left intact.
    assert_eq!(get(&mut s, "/d/x"), b"1");
}

#[test]
fn rmdir_refuses_nonempty_then_removes_recursively() {
    let mut s = Store::new();
    s.mkdir("/d").unwrap();
    put(&mut s, "/d/x", b"1");
    s.mkdir("/d/sub").unwrap();
    put(&mut s, "/d/sub/y", b"2");
    assert_eq!(s.rmdir("/d", false), Err(StoreError::NotEmpty));
    s.rmdir("/d", true).unwrap();
    assert_eq!(s.stat("/d"), Err(StoreError::NotFound));
    assert_eq!(s.stat("/d/x"), Err(StoreError::NotFound));
    assert_eq!(s.stat("/d/sub/y"), Err(StoreError::NotFound));
}

#[test]
fn unlink_reindexes_open_handles_to_the_right_file() {
    let mut s = Store::new();
    put(&mut s, "/a", b"aaa");
    put(&mut s, "/z", b"zzz");
    // Hold a handle on /z, then delete an earlier file; the handle must still
    // read /z, proving the fd file-index shift is correct.
    let fd = s.open("/z", PID, false, false, false, true).unwrap();
    s.unlink("/a").unwrap();
    assert_eq!(s.read(fd, PID, 1 << 20).unwrap(), b"zzz");
}

#[test]
fn directory_stat_reports_immediate_child_count() {
    let mut s = Store::new();
    s.mkdir("/c").unwrap();
    put(&mut s, "/c/a", b"");
    put(&mut s, "/c/b", b"");
    s.mkdir("/c/sub").unwrap();
    put(&mut s, "/c/sub/deep", b""); // not an immediate child of /c
    assert_eq!(s.stat("/c").unwrap().0, 3);
}

#[test]
fn usage_tracks_files_and_bytes() {
    let mut s = Store::new();
    assert_eq!(s.usage().0, 0);
    put(&mut s, "/f", b"hello");
    put(&mut s, "/g", b"world!");
    let (files, bytes, max) = s.usage();
    assert_eq!(files, 2);
    assert_eq!(bytes, 11);
    assert!(max >= 2048);
}

#[test]
fn sequential_writes_advance_position() {
    // The mechanism chunked client writes rely on: successive writes to one
    // handle append across the file rather than overwriting.
    let mut s = Store::new();
    let fd = s.open("/big", PID, true, true, false, true).unwrap();
    s.write(fd, PID, &[b'x'; 100]).unwrap();
    s.write(fd, PID, &[b'y'; 50]).unwrap();
    s.close(fd, PID).unwrap();
    assert_eq!(s.stat("/big").unwrap().0, 150);
    let data = get(&mut s, "/big");
    assert_eq!(&data[..100], &vec![b'x'; 100][..]);
    assert_eq!(&data[100..], &vec![b'y'; 50][..]);
}

#[test]
fn write_refreshes_mtime() {
    let mut s = Store::new();
    put(&mut s, "/f", b"a");
    let first = s.stat("/f").unwrap().2;
    put(&mut s, "/f", b"bb");
    let second = s.stat("/f").unwrap().2;
    assert!(second > first, "mtime should advance on write");
}

#[test]
fn rename_moves_a_file_and_rejects_existing_dest() {
    let mut s = Store::new();
    put(&mut s, "/a", b"data");
    s.rename("/a", "/b").unwrap();
    assert_eq!(s.stat("/a"), Err(StoreError::NotFound));
    assert_eq!(get(&mut s, "/b"), b"data");
    put(&mut s, "/c", b"x");
    assert_eq!(s.rename("/b", "/c"), Err(StoreError::Exists));
}

#[test]
fn rename_rewrites_a_directory_subtree() {
    let mut s = Store::new();
    s.mkdir("/d").unwrap();
    put(&mut s, "/d/x", b"1");
    s.mkdir("/d/sub").unwrap();
    put(&mut s, "/d/sub/y", b"2");
    s.rename("/d", "/e").unwrap();
    assert_eq!(s.stat("/d"), Err(StoreError::NotFound));
    assert!(s.stat("/e").unwrap().1);
    assert_eq!(get(&mut s, "/e/x"), b"1");
    assert_eq!(get(&mut s, "/e/sub/y"), b"2");
}

#[test]
fn chmod_can_re_enable_writes() {
    let mut s = Store::new();
    put(&mut s, "/f", b"a");
    s.chmod("/f", 0o444).unwrap();
    let fd = s.open("/f", PID, false, false, false, true).unwrap();
    assert_eq!(s.write(fd, PID, b"x"), Err(StoreError::AccessDenied));
    s.chmod("/f", 0o644).unwrap();
    let fd2 = s.open("/f", PID, false, false, false, true).unwrap();
    assert_eq!(s.write(fd2, PID, b"bb"), Ok(2));
}

#[test]
fn append_mode_writes_at_end() {
    let mut s = Store::new();
    put(&mut s, "/f", b"abc");
    let fd = s.open("/f", PID, false, false, true, true).unwrap();
    s.write(fd, PID, b"def").unwrap();
    s.close(fd, PID).unwrap();
    assert_eq!(get(&mut s, "/f"), b"abcdef");
}

#[test]
fn read_respects_max_bytes() {
    let mut s = Store::new();
    put(&mut s, "/f", b"0123456789");
    let fd = s.open("/f", PID, false, false, false, true).unwrap();
    assert_eq!(s.read(fd, PID, 4).unwrap(), b"0123");
}
