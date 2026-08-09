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

use crate::blk::error::BlkError;
use crate::store::StoreError;
use crate::{map_blk_err, map_store_err, split_caller};

// Errno values mirrored from the protocol header.
const ENOENT: i32 = -2;
const EBADF: i32 = -9;
const EACCES: i32 = -13;
const EEXIST: i32 = -17;
const EISDIR: i32 = -21;
const EINVAL: i32 = -22;
const ENOSPC: i32 = -28;
const ENOTEMPTY: i32 = -39;

#[test]
fn kernel_tcb_keeps_the_payload_pid() {
    // sender_pid == 0 is the kernel-side mirror (trusted); the attested pid is
    // whatever the payload carries.
    let payload = [5, 0, 0, 0, b'x', b'y'];
    let (pid, rest) = split_caller(&payload, 0).unwrap();
    assert_eq!(pid, 5);
    assert_eq!(rest, b"xy");
}

#[test]
fn userspace_pid_must_match_the_attested_sender() {
    let payload = [7, 0, 0, 0, b'z'];
    let (pid, rest) = split_caller(&payload, 7).unwrap();
    assert_eq!(pid, 7);
    assert_eq!(rest, b"z");
}

#[test]
fn userspace_impersonation_is_rejected() {
    // A CPL=3 sender claiming a different pid in the payload is denied.
    let payload = [7, 0, 0, 0];
    assert_eq!(split_caller(&payload, 9), Err(EACCES));
}

#[test]
fn short_payload_is_invalid() {
    assert_eq!(split_caller(&[1, 2, 3], 0), Err(EINVAL));
    assert_eq!(split_caller(&[], 5), Err(EINVAL));
}

#[test]
fn store_errors_map_to_expected_errnos() {
    assert_eq!(map_store_err(StoreError::NotFound), ENOENT);
    assert_eq!(map_store_err(StoreError::BadFd), EBADF);
    assert_eq!(map_store_err(StoreError::Full), ENOSPC);
    assert_eq!(map_store_err(StoreError::AccessDenied), EACCES);
    assert_eq!(map_store_err(StoreError::Exists), EEXIST);
    assert_eq!(map_store_err(StoreError::NotEmpty), ENOTEMPTY);
    assert_eq!(map_store_err(StoreError::IsDir), EISDIR);
}

#[test]
fn blk_errors_map_to_expected_errnos() {
    assert_eq!(map_blk_err(BlkError::Exists), EEXIST);
    assert_eq!(map_blk_err(BlkError::BadLength), ENOSPC);
    assert_eq!(map_blk_err(BlkError::NoService), EINVAL);
    assert_eq!(map_blk_err(BlkError::Transport(-11)), EINVAL);
    assert_eq!(map_blk_err(BlkError::ShortReply(3)), EINVAL);
    assert_eq!(map_blk_err(BlkError::BadHeader), EINVAL);
    assert_eq!(map_blk_err(BlkError::IdMismatch), EINVAL);
    assert_eq!(map_blk_err(BlkError::Status(-6)), EINVAL);
    assert_eq!(map_blk_err(BlkError::Inval), EINVAL);
    assert_eq!(map_blk_err(BlkError::BadContainer), EINVAL);
}
