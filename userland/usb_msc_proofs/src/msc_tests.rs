// NONOS Operating System (AGPL-3.0-or-later)
use crate::bot::{self, CommandBlockWrapper, CBW_FLAG_IN};
use crate::descriptors::{encode_probe, parse_config};
use crate::protocol::{
    parse, Request, E_INVAL, E_NO_MSC, E_OVERFLOW, E_PHASE, HDR_LEN, MAGIC, MAX_BINDINGS,
    MAX_TRANSFER_BLOCKS, VERSION,
};
use crate::scsi;

extern crate alloc;
use alloc::vec::Vec;

// The mass-storage driver parses three untrusted surfaces. The configuration
// descriptor and the 13-byte command status wrapper come from the device, so
// every byte is chosen by potentially hostile hardware. The wire header and
// the block-request body come from other capsules over IPC. The proofs below
// run the real parsers over all three: parsing never panics, the descriptor
// walk terminates and never yields more bindings than its cap, a status
// wrapper is accepted only in its exact shape, and a block request is bounded
// before it ever becomes a SCSI command.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

// Configuration descriptor walk: device-controlled record stream.

fn msc_config(total_override: Option<u16>) -> Vec<u8> {
    let mut raw = Vec::new();
    raw.extend_from_slice(&[9, 0x02, 0, 0, 1, 1, 0, 0x80, 50]);
    raw.extend_from_slice(&[9, 0x04, 3, 0, 2, 0x08, 0x06, 0x50, 0]);
    raw.extend_from_slice(&[7, 0x05, 0x81, 0x02, 0x00, 0x02, 0]);
    raw.extend_from_slice(&[7, 0x05, 0x02, 0x02, 0x00, 0x02, 0]);
    let total = total_override.unwrap_or(raw.len() as u16);
    raw[2..4].copy_from_slice(&total.to_le_bytes());
    raw
}

#[test]
fn a_bulk_only_scsi_configuration_yields_one_binding() {
    let raw = msc_config(None);
    let result = parse_config(&raw).expect("a well-formed MSC configuration parses");
    assert_eq!(result.count, 1);
    let b = result.bindings[0];
    assert_eq!(b.interface, 3);
    assert_eq!(b.bulk_in, 0x81);
    assert_eq!(b.bulk_out, 0x02);
    assert_eq!(b.max_packet_in, 512);
    assert_eq!(b.max_packet_out, 512);
}

#[test]
fn non_msc_interfaces_and_non_bulk_endpoints_yield_no_binding() {
    let mut raw = msc_config(None);
    raw[9 + 5] = 0x03; // interface class HID instead of mass storage
    assert_eq!(parse_config(&raw).err(), Some(E_NO_MSC));

    let mut raw = msc_config(None);
    raw[18 + 3] = 0x03; // IN endpoint attribute interrupt instead of bulk
    assert_eq!(parse_config(&raw).err(), Some(E_NO_MSC));
}

#[test]
fn malformed_configurations_are_rejected_not_looped() {
    // Too short, wrong descriptor type, absurd totals.
    assert_eq!(parse_config(&[]).err(), Some(E_INVAL));
    assert_eq!(parse_config(&[9, 0x04, 9, 0, 0, 0, 0, 0, 0]).err(), Some(E_INVAL));
    let raw = msc_config(Some(8));
    assert_eq!(parse_config(&raw).err(), Some(E_INVAL));
    let raw = msc_config(Some(4096));
    assert_eq!(parse_config(&raw).err(), Some(E_INVAL));

    // A zero-length record must be an error, not an infinite loop.
    let mut raw = msc_config(None);
    raw[9] = 0;
    assert_eq!(parse_config(&raw).err(), Some(E_INVAL));

    // A record running past the declared total is an error.
    let mut raw = msc_config(None);
    raw[9] = 60;
    assert_eq!(parse_config(&raw).err(), Some(E_INVAL));
}

#[test]
fn the_binding_count_never_exceeds_its_cap() {
    let mut raw = Vec::new();
    raw.extend_from_slice(&[9, 0x02, 0, 0, 1, 1, 0, 0x80, 50]);
    for i in 0..(MAX_BINDINGS + 4) as u8 {
        raw.extend_from_slice(&[9, 0x04, i, 0, 2, 0x08, 0x06, 0x50, 0]);
        raw.extend_from_slice(&[7, 0x05, 0x81, 0x02, 0x00, 0x02, 0]);
        raw.extend_from_slice(&[7, 0x05, 0x02, 0x02, 0x00, 0x02, 0]);
    }
    let total = raw.len() as u16;
    raw[2..4].copy_from_slice(&total.to_le_bytes());
    let result = parse_config(&raw).expect("a flood of interfaces still parses");
    assert_eq!(result.count, MAX_BINDINGS);
}

#[test]
fn config_parse_never_panics_and_bindings_are_directional() {
    for seed in 1..150_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 600) as usize;
        let mut raw: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        // Half the runs get a plausible prefix so the walk itself is exercised.
        if raw.len() >= 9 && seed % 2 == 0 {
            raw[1] = 0x02;
            let total = (raw.len() as u16).min(xorshift(&mut s) as u16);
            raw[2..4].copy_from_slice(&total.to_le_bytes());
        }
        if let Ok(result) = parse_config(&raw) {
            assert!(result.count >= 1 && result.count <= MAX_BINDINGS);
            for b in result.bindings.iter().take(result.count) {
                assert_ne!(b.bulk_in & 0x80, 0, "an IN endpoint must have the IN direction bit");
                assert_eq!(b.bulk_out & 0x80, 0, "an OUT endpoint must not have the IN bit");
                assert_ne!(b.bulk_out, 0, "a binding is complete only with a real OUT endpoint");
            }
        }
    }
}

#[test]
fn encode_probe_writes_exactly_the_bindings_it_parsed() {
    let raw = msc_config(None);
    let result = parse_config(&raw).expect("a well-formed MSC configuration parses");
    let mut out = [0u8; 4 + 8 * MAX_BINDINGS];
    let n = encode_probe(&result, &mut out);
    assert_eq!(n, 4 + 8 * result.count);
    assert_eq!(u32::from_le_bytes([out[0], out[1], out[2], out[3]]), result.count as u32);
    assert_eq!(out[4], 3);
    assert_eq!(out[5], 0x81);
    assert_eq!(out[6], 0x02);
    assert_eq!(u16::from_le_bytes([out[8], out[9]]), 512);
    assert_eq!(u16::from_le_bytes([out[10], out[11]]), 512);
}

// Command status wrapper: 13 device-controlled bytes ending every transfer.

#[test]
fn csw_parse_accepts_only_the_exact_shape() {
    let mut csw = [0u8; 13];
    csw[0..4].copy_from_slice(&0x5342_5355u32.to_le_bytes());
    csw[4..8].copy_from_slice(&0xdead_beefu32.to_le_bytes());
    csw[8..12].copy_from_slice(&64u32.to_le_bytes());
    csw[12] = 1;
    let parsed = bot::parse(&csw).expect("a well-formed CSW parses");
    assert_eq!(parsed.tag, 0xdead_beef);
    assert_eq!(parsed.residue, 64);
    assert_eq!(parsed.status, 1);

    for len in (0..13).chain(14..20) {
        assert_eq!(bot::parse(&vec![0u8; len]).err(), Some(E_INVAL), "length {len}");
    }
    let mut bad_sig = csw;
    bad_sig[0] ^= 1;
    assert_eq!(bot::parse(&bad_sig).err(), Some(E_INVAL));
    for status in 3..=255u8 {
        let mut bad_status = csw;
        bad_status[12] = status;
        assert_eq!(bot::parse(&bad_status).err(), Some(E_PHASE));
    }
}

#[test]
fn csw_parse_never_panics_on_hostile_bytes() {
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 20) as usize;
        let raw: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if let Ok(parsed) = bot::parse(&raw) {
            assert_eq!(raw.len(), 13);
            assert!(parsed.status <= 2);
        }
    }
}

// Block requests: attacker-controlled LBA and block count over IPC.

#[test]
fn block_request_is_bounded_and_faithful() {
    let body = |lba: u32, blocks: u16| {
        let mut b = [0u8; 6];
        b[0..4].copy_from_slice(&lba.to_le_bytes());
        b[4..6].copy_from_slice(&blocks.to_le_bytes());
        b
    };
    assert_eq!(scsi::block_request(&body(7, 1)), Ok((7, 1)));
    assert_eq!(
        scsi::block_request(&body(u32::MAX, MAX_TRANSFER_BLOCKS)),
        Ok((u32::MAX, MAX_TRANSFER_BLOCKS))
    );
    assert_eq!(scsi::block_request(&body(0, 0)).err(), Some(E_INVAL));
    assert_eq!(scsi::block_request(&body(0, MAX_TRANSFER_BLOCKS + 1)).err(), Some(E_OVERFLOW));
    for len in (0..6).chain(7..10) {
        assert_eq!(scsi::block_request(&vec![0u8; len]).err(), Some(E_INVAL), "length {len}");
    }
}

// The read path composition: a validated request becomes a CBW whose transfer
// length cannot overflow and whose CDB carries the fields big-endian, as
// SBC-2 specifies.

#[test]
fn a_validated_read_becomes_a_bounded_big_endian_cbw() {
    for seed in 1..50_000u64 {
        let mut s = seed;
        let mut body = [0u8; 6];
        for b in body.iter_mut() {
            *b = (xorshift(&mut s) & 0xff) as u8;
        }
        let Ok((lba, blocks)) = scsi::block_request(&body) else { continue };
        let data_len = u32::from(blocks) * 512;
        assert!(data_len <= u32::from(MAX_TRANSFER_BLOCKS) * 512);

        let (cdb, cdb_len) = scsi::read10(lba, blocks);
        assert_eq!(cdb_len, 10);
        assert_eq!(cdb[0], 0x28);
        assert_eq!(u32::from_be_bytes([cdb[2], cdb[3], cdb[4], cdb[5]]), lba);
        assert_eq!(u16::from_be_bytes([cdb[7], cdb[8]]), blocks);

        let cbw = CommandBlockWrapper {
            tag: 9,
            data_len,
            flags: CBW_FLAG_IN,
            lun: 0,
            cdb_len,
            cdb,
        };
        let mut out = [0u8; 31];
        cbw.write(&mut out);
        assert_eq!(u32::from_le_bytes([out[0], out[1], out[2], out[3]]), 0x4342_5355);
        assert_eq!(u32::from_le_bytes([out[8], out[9], out[10], out[11]]), data_len);
        assert_eq!(out[12], CBW_FLAG_IN);
        assert_eq!(out[14], 10);
        assert_eq!(&out[15..31], &cdb);
    }
}

#[test]
fn the_fixed_cdbs_match_their_spec_opcodes() {
    let (inq, inq_len) = scsi::inquiry();
    assert_eq!((inq[0], inq[4], inq_len), (0x12, 36, 6));
    let (cap, cap_len) = scsi::read_capacity10();
    assert_eq!((cap[0], cap_len), (0x25, 10));
    let (wr, wr_len) = scsi::write10(0x0102_0304, 0x0506);
    assert_eq!((wr[0], wr_len), (0x2A, 10));
    assert_eq!(&wr[2..6], &[0x01, 0x02, 0x03, 0x04]);
    assert_eq!(&wr[7..9], &[0x05, 0x06]);
}

// Wire protocol: the header must frame the payload exactly, and the returned
// payload slice is the region after the header, nothing more.

#[test]
fn wire_parse_demands_exact_framing() {
    let frame = |payload: &[u8]| {
        let mut buf = vec![0u8; HDR_LEN + payload.len()];
        buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        buf[4..6].copy_from_slice(&VERSION.to_le_bytes());
        buf[6..8].copy_from_slice(&5u16.to_le_bytes());
        buf[12..16].copy_from_slice(&77u32.to_le_bytes());
        buf[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        buf[HDR_LEN..].copy_from_slice(payload);
        buf
    };
    let buf = frame(&[1, 2, 3]);
    let (req, payload) = parse(&buf).expect("an exactly framed request parses");
    assert_eq!(req.op, 5);
    assert_eq!(req.request_id, 77);
    assert_eq!(payload, &[1, 2, 3]);

    // A trailing byte or a short payload breaks the frame.
    let mut long = frame(&[1, 2, 3]);
    long.push(0);
    assert!(parse(&long).is_none());
    let mut short = frame(&[1, 2, 3]);
    short.pop();
    assert!(parse(&short).is_none());
    let mut bad_magic = frame(&[]);
    bad_magic[0] ^= 1;
    assert!(parse(&bad_magic).is_none());
    let mut bad_version = frame(&[]);
    bad_version[4] ^= 1;
    assert!(parse(&bad_version).is_none());
    for len in 0..HDR_LEN {
        assert!(parse(&vec![0u8; len]).is_none());
    }
}

#[test]
fn wire_parse_never_panics_and_payloads_stay_in_bounds() {
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 64) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= HDR_LEN && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&VERSION.to_le_bytes());
        }
        if let Some((_, payload)) = parse(&buf) {
            assert_eq!(payload.len(), buf.len() - HDR_LEN);
            assert_eq!(payload, &buf[HDR_LEN..]);
        }
    }
}

#[test]
fn a_response_header_parses_back_when_framed() {
    let req = Request { op: 4, flags: 0, request_id: 31 };
    let mut out = vec![0u8; HDR_LEN + 4];
    crate::protocol::response_header(&mut out, &req, 4);
    crate::protocol::write_status(&mut out, -22);
    let (back, payload) = parse(&out).expect("a framed response parses");
    assert_eq!(back.op, 4);
    assert_eq!(back.request_id, 31);
    assert_eq!(i32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]), -22);
}
