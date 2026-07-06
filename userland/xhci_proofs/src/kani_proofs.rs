// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the TRB field algebra is exact for every value.

use crate::constants::{TRB_DIR_IN, TRB_TYPE_DATA_STAGE};
use crate::protocol::decode_request;
use crate::trb::builders::data_stage::data_stage_in;
use crate::trb::Trb;

// For every TRB contents and every setter argument: a setter writes exactly
// its field and leaves every other bit untouched, and the getters invert
// the setters.
#[kani::proof]
fn setters_and_getters_are_exact_inverses() {
    let base = Trb { d0: kani::any(), d1: kani::any(), d2: kani::any(), d3: kani::any() };

    let ty: u32 = kani::any();
    let mut t = base;
    t.set_type(ty);
    assert!(t.get_type() == ty & 0x3F);
    assert!(t.d3 & !(0x3F << 10) == base.d3 & !(0x3F << 10));

    let len: u32 = kani::any();
    let mut t = base;
    t.set_transfer_length(len);
    assert!(t.d2 & 0x1_FFFF == len & 0x1_FFFF);
    assert!(t.d2 >> 17 == base.d2 >> 17);

    let ptr: u64 = kani::any();
    let mut t = base;
    t.set_pointer(ptr);
    assert!(t.get_pointer() == ptr);

    let cycle: bool = kani::any();
    let mut t = base;
    t.set_cycle(cycle);
    assert!(t.get_cycle() == cycle);
    assert!(t.d3 >> 1 == base.d3 >> 1);

    assert!(base.completion_code() as u32 == base.d2 >> 24);
    assert!(base.slot_id() as u32 == base.d3 >> 24);
}

// For every buffer address, length, and cycle: the data stage TRB carries
// the address split across d0/d1, the length in the 17-bit field, the data
// stage type, and the IN direction.
#[kani::proof]
fn the_data_stage_is_faithful_for_every_argument() {
    let phys: u64 = kani::any();
    let length: u16 = kani::any();
    let cycle: bool = kani::any();
    let trb = data_stage_in(phys, length, cycle);
    assert!(trb.get_pointer() == phys);
    assert!(trb.d2 & 0x1_FFFF == length as u32);
    assert!(trb.get_type() == TRB_TYPE_DATA_STAGE);
    assert!(trb.d3 & TRB_DIR_IN != 0);
    assert!(trb.get_cycle() == cycle);
}

// For every buffer up to a header plus slack: wire decoding is total and
// every accepted field comes from its wire offset in little-endian order.
#[kani::proof]
fn decode_is_total_and_header_faithful() {
    let buf: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some(req) = decode_request(&buf[..len]) {
        assert!(len >= 20);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
        assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
    }
}
