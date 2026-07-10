// NONOS Operating System (AGPL-3.0-or-later)
use crate::admin::{ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::nvm::MAX_SECTORS;
use crate::protocol::{
    decode_request, encode_response_header, Request, E_INVAL, E_MSGSIZE, E_NXIO, HDR_LEN,
    RW_HEADER_LEN,
};
use crate::server::handlers::parse_rw;

extern crate alloc;
use alloc::vec::Vec;

// The NVMe driver has two untrusted surfaces. Block read/write requests and
// the wire header arrive from other capsules over IPC, so their fields are
// attacker-chosen. The identify and SMART pages arrive from the device over
// DMA into fixed-size buffers (4096 bytes for identify, 512 for the SMART log
// page), so their contents are device-chosen. The proofs below run the real
// parsers over both surfaces: parsing never panics, an accepted block request
// never reaches past the disk, and every decoded field comes from its
// specified offset in little-endian order.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn fill(buf: &mut [u8], seed: u64) {
    let mut s = seed | 1;
    for b in buf.iter_mut() {
        *b = (xorshift(&mut s) & 0xff) as u8;
    }
}

fn le16(d: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([d[o], d[o + 1]])
}

fn le32(d: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([d[o], d[o + 1], d[o + 2], d[o + 3]])
}

fn le64(d: &[u8], o: usize) -> u64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&d[o..o + 8]);
    u64::from_le_bytes(b)
}

fn le128(d: &[u8], o: usize) -> u128 {
    let mut b = [0u8; 16];
    b.copy_from_slice(&d[o..o + 16]);
    u128::from_le_bytes(b)
}

// Block I/O requests: same isolation property as the AHCI proof. An accepted
// request has a bounded sector count and lba + count neither overflows nor
// exceeds the device capacity.

#[test]
fn rw_parse_never_panics_and_requests_stay_within_the_disk() {
    for seed in 1..150_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 20) as usize;
        let body: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        let capacity = xorshift(&mut s);

        if let Ok((lba, nsectors)) = parse_rw(&body, capacity) {
            assert!((1..=MAX_SECTORS).contains(&nsectors), "sector count out of range");
            let last = lba.checked_add(nsectors as u64);
            assert!(last.is_some(), "lba + count overflowed");
            assert!(last.unwrap() <= capacity, "request reaches past the disk");
        }
    }
}

#[test]
fn rw_parse_short_bodies_are_rejected_not_panicked() {
    for len in 0..RW_HEADER_LEN {
        let body = vec![0xffu8; len];
        assert_eq!(parse_rw(&body, u64::MAX).unwrap_err(), E_MSGSIZE);
    }
}

#[test]
fn rw_parse_enforces_the_exact_boundaries() {
    let body = |lba: u64, n: u32| {
        let mut b = [0u8; 12];
        b[0..8].copy_from_slice(&lba.to_le_bytes());
        b[8..12].copy_from_slice(&n.to_le_bytes());
        b
    };
    // The last sector of the disk is reachable, one past it is not.
    let cap = 1_000_000u64;
    assert_eq!(parse_rw(&body(cap - 1, 1), cap), Ok((cap - 1, 1)));
    assert_eq!(parse_rw(&body(cap, 1), cap).unwrap_err(), E_NXIO);
    assert_eq!(
        parse_rw(&body(cap - MAX_SECTORS as u64, MAX_SECTORS), cap),
        Ok((cap - MAX_SECTORS as u64, MAX_SECTORS))
    );
    assert_eq!(
        parse_rw(&body(cap - MAX_SECTORS as u64 + 1, MAX_SECTORS), cap).unwrap_err(),
        E_NXIO
    );
    // Zero and oversized sector counts are invalid regardless of capacity.
    assert_eq!(parse_rw(&body(0, 0), cap).unwrap_err(), E_INVAL);
    assert_eq!(parse_rw(&body(0, MAX_SECTORS + 1), cap).unwrap_err(), E_INVAL);
    // lba + count that wraps a u64 is invalid, not wrapped.
    assert_eq!(parse_rw(&body(u64::MAX, 1), u64::MAX).unwrap_err(), E_INVAL);
}

// The IPC wire header: decoding never panics, short or mistagged headers are
// rejected, and an encoded response header decodes back to the same fields.

#[test]
fn decode_rejects_short_and_mistagged_headers() {
    for len in 0..HDR_LEN {
        assert!(decode_request(&vec![0xa5u8; len]).is_none());
    }
    let mut hdr = [0u8; HDR_LEN];
    hdr[0..4].copy_from_slice(&0x4e4e_564du32.to_le_bytes());
    hdr[4..6].copy_from_slice(&1u16.to_le_bytes());
    assert!(decode_request(&hdr).is_some());
    let mut bad_magic = hdr;
    bad_magic[0] ^= 1;
    assert!(decode_request(&bad_magic).is_none());
    let mut bad_version = hdr;
    bad_version[4] ^= 1;
    assert!(decode_request(&bad_version).is_none());
}

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        // Half the runs get a valid tag so the accepting path is exercised.
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&0x4e4e_564du32.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, le16(&buf, 6));
            assert_eq!(req.flags, le16(&buf, 8));
            assert_eq!(req.request_id, le32(&buf, 12));
            assert_eq!(req.payload_len, le32(&buf, 16));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..20_000u64 {
        let mut s = seed;
        let req = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &req, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, req.op);
        assert_eq!(back.flags, req.flags);
        assert_eq!(back.request_id, req.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}

// The identify controller page: 4096 device-controlled bytes. Every field is
// read from its NVMe 2.0 offset in little-endian order, and parsing never
// panics on any contents.

#[test]
fn identify_controller_reads_every_field_from_its_spec_offset() {
    let mut page = [0u8; 4096];
    for seed in 1..2_000u64 {
        fill(&mut page, seed);
        let id = ControllerIdentity::parse(&page);
        assert_eq!(id.vendor_id, le16(&page, 0x00));
        assert_eq!(id.subsystem_vendor_id, le16(&page, 0x02));
        assert_eq!(id.serial, page[0x04..0x18]);
        assert_eq!(id.model, page[0x18..0x40]);
        assert_eq!(id.firmware, page[0x40..0x48]);
        assert_eq!(id.mdts, page[0x4d]);
        assert_eq!(id.version, le32(&page, 0x50));
        assert_eq!(id.optional_admin, le16(&page, 0x100));
        assert_eq!(id.sq_entry_size, page[0x200]);
        assert_eq!(id.cq_entry_size, page[0x201]);
        assert_eq!(id.namespace_count, le32(&page, 0x204));
        assert_eq!(id.optional_nvm, le16(&page, 0x208));
        assert_eq!(id.volatile_write_cache, page[0x20d]);
    }
}

// The identify namespace page: the format index (FLBAS) selects one of 16
// LBA-format slots, so a hostile device steers which bytes are read. For every
// slot the reads stay at the slot's spec offset, the block size is a power of
// two or zero (an absurd shift yields zero rather than a wrapped shift), and
// the formatted-count increment saturates instead of wrapping.

#[test]
fn identify_namespace_is_faithful_for_every_lba_format_slot() {
    let mut page = [0u8; 4096];
    for seed in 1..2_000u64 {
        fill(&mut page, seed);
        for flbas in 0..16u8 {
            page[0x1a] = (page[0x1a] & 0xf0) | flbas;
            let ns = NamespaceIdentity::parse(7, &page);
            let slot = 0x80 + (flbas as usize) * 4;
            assert_eq!(ns.nsid, 7);
            assert_eq!(ns.size_lba, le64(&page, 0x00));
            assert_eq!(ns.capacity_lba, le64(&page, 0x08));
            assert_eq!(ns.used_lba, le64(&page, 0x10));
            assert_eq!(ns.format_index, flbas);
            assert_eq!(ns.metadata_size, le16(&page, slot));
            let shift = page[slot + 2];
            if shift < 32 {
                assert_eq!(ns.lba_size, 1u32 << shift);
            } else {
                assert_eq!(ns.lba_size, 0, "an out-of-range shift must not wrap");
            }
            assert!(ns.lba_size == 0 || ns.lba_size.is_power_of_two());
            assert_eq!(ns.formatted_lba_count, page[0x19].saturating_add(1));
            assert!(ns.formatted_lba_count >= 1);
        }
    }
}

// The SMART / health log page: 512 device-controlled bytes, fields up to
// 128 bits wide. Parsing never panics and every counter comes from its
// spec offset.

#[test]
fn smart_health_reads_every_field_from_its_spec_offset() {
    let mut page = [0u8; 512];
    for seed in 1..2_000u64 {
        fill(&mut page, seed);
        let h = SmartHealth::parse(&page);
        assert_eq!(h.critical_warning, page[0]);
        assert_eq!(h.temperature_kelvin, le16(&page, 1));
        assert_eq!(h.available_spare, page[3]);
        assert_eq!(h.available_spare_threshold, page[4]);
        assert_eq!(h.percentage_used, page[5]);
        assert_eq!(h.endurance_group_warning, page[6]);
        assert_eq!(h.data_units_read, le128(&page, 32));
        assert_eq!(h.data_units_written, le128(&page, 48));
        assert_eq!(h.host_read_commands, le128(&page, 64));
        assert_eq!(h.host_write_commands, le128(&page, 80));
        assert_eq!(h.controller_busy_time, le128(&page, 96));
        assert_eq!(h.power_cycles, le128(&page, 112));
        assert_eq!(h.power_on_hours, le128(&page, 128));
        assert_eq!(h.unsafe_shutdowns, le128(&page, 144));
        assert_eq!(h.media_errors, le128(&page, 160));
        assert_eq!(h.error_log_entries, le128(&page, 176));
        assert_eq!(h.warning_temp_time, le32(&page, 192));
        assert_eq!(h.critical_temp_time, le32(&page, 196));
    }
}

#[test]
fn temperature_conversion_is_the_kelvin_offset() {
    let mut page = [0u8; 512];
    page[1..3].copy_from_slice(&310u16.to_le_bytes());
    assert_eq!(SmartHealth::parse(&page).temperature_celsius(), 37);
    page[1..3].copy_from_slice(&0u16.to_le_bytes());
    assert_eq!(SmartHealth::parse(&page).temperature_celsius(), -273);
}
