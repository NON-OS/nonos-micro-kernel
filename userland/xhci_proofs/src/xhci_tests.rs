// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::{
    TRB_CYCLE, TRB_DIR_IN, TRB_IDT, TRB_IOC, TRB_TYPE_DATA_STAGE, TRB_TYPE_SETUP_STAGE,
    TRB_TYPE_STATUS_STAGE, TRT_IN_DATA,
};
use crate::protocol::{decode_request, encode_response_header, Request, HDR_LEN};
use crate::trb::builders::data_stage::data_stage_in;
use crate::trb::builders::setup_stage::setup_stage_get_descriptor_typed;
use crate::trb::builders::status_stage::status_stage_out;
use crate::trb::Trb;

extern crate alloc;
use alloc::vec::Vec;

// A TRB is the 16-byte unit everything crosses the xHCI rings in. The
// device writes event TRBs, so the extraction of completion code, slot id,
// type, and cycle must read exactly the spec fields; the driver writes
// control-transfer TRBs, so the setup, data, and status stages must encode
// exactly the spec layout. A wrong shift here addresses the wrong device
// slot or misreads a transfer result.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn event_field_extraction_reads_the_spec_bits() {
    for seed in 1..100_000u64 {
        let mut s = seed;
        let trb = Trb {
            d0: xorshift(&mut s) as u32,
            d1: xorshift(&mut s) as u32,
            d2: xorshift(&mut s) as u32,
            d3: xorshift(&mut s) as u32,
        };
        assert_eq!(trb.completion_code(), (trb.d2 >> 24) as u8);
        assert_eq!(trb.slot_id(), (trb.d3 >> 24) as u8);
        assert_eq!(trb.get_type(), (trb.d3 >> 10) & 0x3F);
        assert_eq!(trb.get_cycle(), trb.d3 & 1 != 0);
        assert_eq!(trb.get_pointer(), (trb.d0 as u64) | ((trb.d1 as u64) << 32));
    }
}

#[test]
fn setters_touch_only_their_field() {
    for seed in 1..100_000u64 {
        let mut s = seed;
        let base = Trb {
            d0: xorshift(&mut s) as u32,
            d1: xorshift(&mut s) as u32,
            d2: xorshift(&mut s) as u32,
            d3: xorshift(&mut s) as u32,
        };
        let ty = xorshift(&mut s) as u32;
        let mut t = base;
        t.set_type(ty);
        assert_eq!(t.get_type(), ty & 0x3F);
        assert_eq!(t.d3 & !(0x3F << 10), base.d3 & !(0x3F << 10), "type must not clobber d3");
        assert_eq!((t.d0, t.d1, t.d2), (base.d0, base.d1, base.d2));

        let len = xorshift(&mut s) as u32;
        let mut t = base;
        t.set_transfer_length(len);
        assert_eq!(t.d2 & 0x1_FFFF, len & 0x1_FFFF);
        assert_eq!(t.d2 >> 17, base.d2 >> 17, "length must not clobber the upper d2 bits");

        let ptr = xorshift(&mut s);
        let mut t = base;
        t.set_pointer(ptr);
        assert_eq!(t.get_pointer(), ptr);

        let mut t = base;
        t.set_cycle(true);
        assert!(t.get_cycle());
        t.set_cycle(false);
        assert!(!t.get_cycle());
        assert_eq!(t.d3 | 1, (base.d3 | 1), "cycle must touch only bit 0");
    }
}

// The control-transfer stages, against xHCI spec section 6.4.1 and the USB
// GET_DESCRIPTOR request layout.

#[test]
fn the_setup_stage_encodes_a_get_descriptor_request() {
    for (desc_type, desc_index, length) in
        [(1u8, 0u8, 18u16), (2, 0, 9), (2, 0, 512), (3, 2, 255)]
    {
        for cycle in [false, true] {
            let trb = setup_stage_get_descriptor_typed(desc_type, desc_index, length, cycle);
            // d0: bmRequestType 0x80 (device-to-host), bRequest 0x06
            // (GET_DESCRIPTOR), wValue = type << 8 | index.
            assert_eq!(trb.d0 & 0xFF, 0x80);
            assert_eq!((trb.d0 >> 8) & 0xFF, 0x06);
            assert_eq!((trb.d0 >> 16) & 0xFFFF, ((desc_type as u32) << 8) | desc_index as u32);
            // d1: wIndex 0, wLength in the upper half.
            assert_eq!(trb.d1 & 0xFFFF, 0);
            assert_eq!(trb.d1 >> 16, length as u32);
            // d2: a setup packet is always 8 bytes on the wire.
            assert_eq!(trb.d2, 8);
            // d3: immediate data, IN data transfer type, setup-stage type,
            // and the caller's cycle bit.
            assert_ne!(trb.d3 & TRB_IDT, 0);
            assert_eq!(trb.d3 & TRT_IN_DATA, TRT_IN_DATA);
            assert_eq!(trb.get_type(), TRB_TYPE_SETUP_STAGE);
            assert_eq!(trb.get_cycle(), cycle);
        }
    }
}

#[test]
fn the_data_stage_carries_the_buffer_and_direction() {
    for seed in 1..50_000u64 {
        let mut s = seed;
        let phys = xorshift(&mut s);
        let length = (xorshift(&mut s) & 0xffff) as u16;
        let cycle = seed % 2 == 0;
        let trb = data_stage_in(phys, length, cycle);
        assert_eq!(trb.get_pointer(), phys);
        assert_eq!(trb.d2 & 0x1_FFFF, length as u32);
        assert_eq!(trb.get_type(), TRB_TYPE_DATA_STAGE);
        assert_ne!(trb.d3 & TRB_DIR_IN, 0, "an IN data stage must set the direction bit");
        assert_eq!(trb.get_cycle(), cycle);
    }
}

#[test]
fn the_status_stage_completes_with_an_interrupt() {
    for cycle in [false, true] {
        let trb = status_stage_out(cycle);
        assert_eq!(trb.get_type(), TRB_TYPE_STATUS_STAGE);
        assert_ne!(trb.d3 & TRB_IOC, 0, "the status stage must interrupt on completion");
        assert_eq!(trb.d3 & TRB_DIR_IN, 0, "an OUT status stage carries no IN bit");
        assert_eq!(trb.get_cycle(), cycle);
        assert_eq!(trb.get_pointer(), 0);
    }
}

#[test]
fn the_cycle_bit_is_the_ownership_handshake() {
    // has_event compares the TRB cycle bit against the consumer cycle; the
    // extraction must therefore see exactly what a producer writes.
    let mut trb = Trb::zero();
    assert!(!trb.get_cycle());
    trb.d3 |= TRB_CYCLE;
    assert!(trb.get_cycle());
}

// The wire header.

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    const MAGIC: u32 = 0x4E58_4843; // the wire tag from protocol/header.rs
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..20_000u64 {
        let mut s = seed;
        let request = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &request, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, request.op);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}
