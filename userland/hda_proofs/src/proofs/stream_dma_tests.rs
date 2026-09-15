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

//! The descriptor list as the controller will read it out of memory.
//!
//! This structure is not a register write, it is a record the driver leaves in
//! RAM for a bus master to fetch. Its layout is fixed by the specification at
//! a 64-bit address, a 32-bit length and a 32-bit flags word per entry, and
//! nothing validates it: an entry laid out wrongly is read as an address the
//! controller then plays from, at full rate, with no bound but the length
//! field that was also read out of the same wrong place.

use nonos_devmodel::FakeBar;

use crate::constants::{SD_CTL, SD_FMT, STREAM_FMT_48K16S};
use crate::controller::bdl::{BDL_IOC, N_PERIODS, PERIOD_BYTES};
use crate::model::WINDOW;
use crate::proofs::stream_tests::{play, BDL_BYTES, OFF, SAMPLE_DEV, TAG};

#[test]
fn each_descriptor_entry_names_its_period_in_the_layout_a_controller_fetches() {
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    play(&bar, &bdl);
    for i in 0..N_PERIODS {
        let at = i * 16;
        let addr = SAMPLE_DEV + i as u64 * PERIOD_BYTES;
        assert_eq!(bdl.wrote32(at), addr as u32, "entry {i} has the wrong address");
        assert_eq!(bdl.wrote32(at + 4), (addr >> 32) as u32, "entry {i} lost its upper half");
        assert_eq!(bdl.wrote32(at + 8) as u64, PERIOD_BYTES, "entry {i} has the wrong length");
        assert_eq!(bdl.wrote32(at + 12) & BDL_IOC, BDL_IOC, "entry {i} will not interrupt");
    }
}

#[test]
fn the_descriptor_list_is_written_where_the_engine_was_told_to_look_and_no_further() {
    /*
     * The entries are 16 bytes each and the engine is told there are four of
     * them. Writing a fifth over the end of the allocation is a stray write
     * into whatever the DMA allocator handed out next.
     */
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    play(&bar, &bdl);
    for at in (N_PERIODS * 16)..BDL_BYTES {
        assert_eq!(bdl.wrote8(at), 0, "byte {at} lies past the descriptor list");
    }
}

#[test]
fn the_stream_tag_and_sample_format_are_the_ones_the_codec_was_configured_for() {
    /*
     * The tag is what joins this engine to the codec's converter: they are
     * configured separately and matched only by that number. A mismatch leaves
     * the engine reading the ring at full speed into a link nobody decodes.
     */
    let (bar, bdl) = (FakeBar::new(WINDOW), FakeBar::new(BDL_BYTES));
    play(&bar, &bdl);
    assert_eq!(bar.wrote8((OFF + SD_CTL) as usize + 2), TAG << 4, "the tag is misplaced");
    assert_eq!(bar.wrote16((OFF + SD_FMT) as usize), STREAM_FMT_48K16S);
}
