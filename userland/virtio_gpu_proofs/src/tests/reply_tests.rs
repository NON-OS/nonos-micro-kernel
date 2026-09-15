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

//! What the driver makes of the part's answers: bodies parsed as the
//! specification lays them out.

use crate::device::cmd::{get_capset_info, get_display_info, get_edid};
use crate::tests::harness::harness;
use crate::tests::part::answers::{CAPSET, PANEL, PANEL_CM};
use crate::tests::part::{answering, Answer};

#[test]
fn display_info_yields_the_one_enabled_scanout_at_its_size() {
    let h = harness();
    let _part = answering(&h.region, Answer::Spec);
    let info = get_display_info(&h.queue, 0).expect("answered");
    assert_eq!(info.count, 1);
    assert_eq!((info.scanouts[0].width, info.scanouts[0].height), PANEL);
    assert_eq!(info.scanouts[1].enabled, 0);
}

#[test]
fn edid_yields_the_panel_size_in_centimetres_from_bytes_21_and_22() {
    let h = harness();
    let _part = answering(&h.region, Answer::Spec);
    let size = get_edid(&h.queue, 0).expect("answered").expect("a size");
    assert_eq!((size.width_cm, size.height_cm), (PANEL_CM.0 as u32, PANEL_CM.1 as u32));
}

#[test]
fn an_edid_block_without_the_magic_is_not_believed() {
    let h = harness();
    let _part = answering(&h.region, Answer::BadEdidMagic);
    assert_eq!(get_edid(&h.queue, 0).err(), Some("virtio-gpu: edid magic mismatch"));
}

#[test]
fn capset_info_yields_id_version_and_size() {
    let h = harness();
    let _part = answering(&h.region, Answer::Spec);
    let info = get_capset_info(&h.queue, 0).expect("answered");
    assert_eq!((info.capset_id, info.max_version, info.max_size), CAPSET);
}
