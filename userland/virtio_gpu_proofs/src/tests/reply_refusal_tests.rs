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

//! Refusals honoured and a fence held to.

use crate::constants::VG_FLAG_FENCE;
use crate::device::cmd::{create_resource_2d, ctx_create, submit_3d};
use crate::tests::harness::harness;
use crate::tests::part::{answering, Answer};

#[test]
fn a_rejected_command_is_an_error_not_a_success_with_a_strange_header() {
    let h = harness();
    let _part = answering(&h.region, Answer::Reject);
    let err = create_resource_2d(&h.queue, 0, 1, 2, 64, 64).err();
    assert_eq!(err, Some("virtio-gpu: create_resource_2d rejected"));
    assert_eq!(ctx_create(&h.queue, 1, b"nonos").err(), Some("virtio-gpu: ctx create rejected"));
}

#[test]
fn a_3d_submission_asks_for_a_fence_in_its_context() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    submit_3d(&h.queue, 1, 9, &[0u8; 8]).expect("fenced");
    let seen = part.seen();
    let flags = u32::from_le_bytes([seen[0][4], seen[0][5], seen[0][6], seen[0][7]]);
    assert_eq!(flags, VG_FLAG_FENCE);
    assert_eq!(&seen[0][8..16], &9u64.to_le_bytes(), "fence nine");
    assert_eq!(&seen[0][16..20], &1u32.to_le_bytes(), "in context one");
}

#[test]
fn a_part_echoing_the_wrong_fence_is_refused() {
    let h = harness();
    let _part = answering(&h.region, Answer::WrongFence);
    let err = submit_3d(&h.queue, 1, 9, &[0u8; 8]).err();
    assert_eq!(err, Some("virtio-gpu: submit 3d fence mismatch"));
}
