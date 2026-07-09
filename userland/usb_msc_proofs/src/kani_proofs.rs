// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the mass-storage parsers are total, strict, and bounded
//! for every input.

use crate::bot::{self, CommandBlockWrapper};
use crate::descriptors::parse_config;
use crate::protocol::{parse, HDR_LEN, MAX_BINDINGS, MAX_TRANSFER_BLOCKS};
use crate::scsi;

// For every buffer up to 64 bytes: the descriptor walk never panics, always
// terminates, and an accepted result holds between one and MAX_BINDINGS
// bindings whose endpoints carry the right direction bits.
#[kani::proof]
#[kani::unwind(40)]
fn config_parse_is_total_and_bounded() {
    let raw: [u8; 64] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= raw.len());

    if let Ok(result) = parse_config(&raw[..len]) {
        assert!(result.count >= 1);
        assert!(result.count <= MAX_BINDINGS);
        for b in result.bindings.iter().take(result.count) {
            assert!(b.bulk_in & 0x80 != 0);
            assert!(b.bulk_out & 0x80 == 0);
            assert!(b.bulk_out != 0);
        }
    }
}

// For every buffer up to a CSW plus slack: parsing never panics, and an
// accepted status wrapper is exactly 13 bytes with a valid signature, a
// status of at most 2, and fields read little-endian from their offsets.
#[kani::proof]
fn csw_parse_is_total_and_strict() {
    let raw: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= raw.len());

    if let Ok(csw) = bot::parse(&raw[..len]) {
        assert!(len == 13);
        assert!(u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) == 0x5342_5355);
        assert!(csw.status <= 2);
        assert_eq!(csw.tag, u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]));
        assert_eq!(csw.residue, u32::from_le_bytes([raw[8], raw[9], raw[10], raw[11]]));
    }
}

// For every request body up to 8 bytes: validation never panics, and an
// accepted block request is exactly 6 bytes with a block count between 1 and
// MAX_TRANSFER_BLOCKS, so the derived transfer size cannot overflow a u32.
#[kani::proof]
fn block_request_is_total_and_bounded() {
    let body: [u8; 8] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= body.len());

    if let Ok((lba, blocks)) = scsi::block_request(&body[..len]) {
        assert!(len == 6);
        assert!(blocks >= 1 && blocks <= MAX_TRANSFER_BLOCKS);
        assert_eq!(lba, u32::from_le_bytes([body[0], body[1], body[2], body[3]]));
        let bytes = u32::from(blocks).checked_mul(512);
        assert!(bytes.is_some());
    }
}

// For every buffer up to a header plus slack: wire parsing never panics, and
// an accepted frame is exact: the payload length field equals the bytes that
// follow the header and the returned slice is that region.
#[kani::proof]
fn wire_parse_is_total_and_exactly_framed() {
    let buf: [u8; 32] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some((req, payload)) = parse(&buf[..len]) {
        assert!(len >= HDR_LEN);
        let declared = u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as usize;
        assert_eq!(declared, len - HDR_LEN);
        assert_eq!(payload.len(), declared);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
    }
}

// For every field assignment: a written CBW carries the signature, fields,
// and CDB at their Bulk-Only Transport offsets.
#[kani::proof]
fn cbw_write_is_faithful() {
    let cbw = CommandBlockWrapper {
        tag: kani::any(),
        data_len: kani::any(),
        flags: kani::any(),
        lun: kani::any(),
        cdb_len: kani::any(),
        cdb: kani::any(),
    };
    let mut out = [0u8; 31];
    cbw.write(&mut out);
    assert_eq!(u32::from_le_bytes([out[0], out[1], out[2], out[3]]), 0x4342_5355);
    assert_eq!(u32::from_le_bytes([out[4], out[5], out[6], out[7]]), cbw.tag);
    assert_eq!(u32::from_le_bytes([out[8], out[9], out[10], out[11]]), cbw.data_len);
    assert_eq!(out[12], cbw.flags);
    assert_eq!(out[13], cbw.lun);
    assert_eq!(out[14], cbw.cdb_len);
    assert_eq!(out[15..31], cbw.cdb);
}
