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

use crate::usercopy::{accepted_range, accepts, MAX_COPY_SIZE, PAGE_SIZE, USER_SPACE_END};

// `check_range` guards every copy between the kernel and userspace. It must be
// total (never panic on any address/length) and must accept only ranges that
// lie strictly inside user space without wrapping.

#[test]
fn null_pointer_is_rejected() {
    assert!(!accepts(0, 1));
    assert!(!accepts(0, 0));
}

#[test]
fn oversized_lengths_are_rejected() {
    assert!(!accepts(0x1000, MAX_COPY_SIZE + 1));
    assert!(accepts(0x1000, MAX_COPY_SIZE));
}

#[test]
fn ranges_past_user_space_are_rejected() {
    // A single byte at the last user address is fine; two bytes cross the edge.
    assert!(accepts(USER_SPACE_END, 1));
    assert!(!accepts(USER_SPACE_END, 2));
    assert!(!accepts(USER_SPACE_END + 1, 1));
    assert!(accepts(0x1000, 0x1000));
}

#[test]
fn overflowing_ranges_are_rejected_not_panicked() {
    assert!(!accepts(u64::MAX, 2));
    assert!(!accepts(u64::MAX - 1, 100));
    // Totality: no address/length combination panics.
    for &addr in &[0u64, 1, 0x1000, USER_SPACE_END - 1, USER_SPACE_END, USER_SPACE_END + 1, u64::MAX] {
        for &len in &[0usize, 1, 2, 4096, MAX_COPY_SIZE, MAX_COPY_SIZE + 1, usize::MAX] {
            let _ = accepts(addr, len);
        }
    }
}

#[test]
fn accepted_range_is_page_aligned_and_within_user_space() {
    for &addr in &[1u64, 0x1234, 0x7FFF_0000, USER_SPACE_END - 8192] {
        for &len in &[1usize, 4096, 8192, MAX_COPY_SIZE] {
            if let Some((start, end)) = accepted_range(addr, len) {
                assert_eq!(start % PAGE_SIZE, 0, "start page is aligned");
                assert_eq!(end % PAGE_SIZE, 0, "end page is aligned");
                assert!(start <= addr, "start page does not exceed the address");
                assert!(end <= USER_SPACE_END, "end page stays within user space");
            }
        }
    }
}

#[test]
fn accepted_nonempty_ranges_stay_inside_user_space() {
    for &addr in &[1u64, 0x1000, 0x7FFF_0000, USER_SPACE_END - 4096, USER_SPACE_END] {
        for &len in &[1usize, 2, 4096, MAX_COPY_SIZE] {
            if accepts(addr, len) && len > 0 {
                assert!(addr != 0);
                assert!(addr <= USER_SPACE_END);
                // The last touched byte is within user space and does not wrap.
                assert!(len as u64 - 1 <= USER_SPACE_END - addr);
            }
        }
    }
}
