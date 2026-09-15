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

//! The wire bytes of the resource commands, as the part saw them.

use crate::constants::{VG_CMD_RESOURCE_ATTACH_BACKING, VG_CMD_RESOURCE_CREATE_2D};
use crate::device::cmd::{attach_backing, create_resource_2d};
use crate::tests::harness::harness;
use crate::tests::part::{answering, Answer};
use crate::tests::wire::expected;

#[test]
fn resource_create_2d_carries_id_format_width_height() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    create_resource_2d(&h.queue, 7, 1, 2, 1024, 768).expect("created");
    assert_eq!(part.seen()[0], expected(VG_CMD_RESOURCE_CREATE_2D, 7, &[1, 2, 1024, 768]));
}

#[test]
fn attach_backing_carries_one_entry_with_the_address_split_low_high() {
    let h = harness();
    let part = answering(&h.region, Answer::Spec);
    attach_backing(&h.queue, 0, 1, 0x1_0000_2000, 4096).expect("attached");
    let body = [1, 1, 0x2000, 0x1, 4096, 0];
    assert_eq!(part.seen()[0], expected(VG_CMD_RESOURCE_ATTACH_BACKING, 0, &body));
}
