// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{
    decode_request, encode_response_header, read_u32_le, read_u64_le, Request, E_INVAL, E_MSGSIZE,
    E_NXIO, HDR_LEN, RW_HEADER_LEN,
};

const MAGIC: u32 = 0x4E42_4C4B; // "NBLK", the wire tag from protocol/header.rs
const VERSION: u16 = 1;
use crate::queue::Queue;
use crate::regs::Regs;
use crate::server::{read::parse_read, write::parse_write};
use crate::setup::Driver;

extern crate alloc;
use alloc::vec::Vec;

// The virtio-blk request parsers receive attacker-controlled bytes over IPC:
// an LBA, a sector count, and for writes the data itself. The proofs run the
// real parsers against a real Driver value whose queue pointers are null, so
// any dereference of device memory during parsing would crash immediately.
// An accepted request must stay on the disk and, for writes, must be framed
// exactly: header length, body length, and the declared payload length all
// agreeing before a byte is copied toward the device.

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn driver(capacity: u64) -> Driver {
    Driver {
        irq_grant: 0,
        queue: Queue {
            region_va: core::ptr::null_mut(),
            queue_size: 8,
            avail_offset: 0,
            used_offset: 0,
            header_va: core::ptr::null_mut(),
            header_phys: 0,
            data_va: core::ptr::null_mut(),
            data_phys: 0,
            data_len: 0,
            last_used: 0,
        },
        regs: Regs::mmio(0),
        capacity_sectors: capacity,
    }
}

fn req(payload_len: u32) -> Request {
    Request { op: 3, flags: 0, request_id: 1, payload_len }
}

fn rw_body(lba: u64, nsectors: u32, data: usize) -> Vec<u8> {
    let mut b = vec![0u8; RW_HEADER_LEN + data];
    b[0..8].copy_from_slice(&lba.to_le_bytes());
    b[8..12].copy_from_slice(&nsectors.to_le_bytes());
    b
}

// Read requests.

#[test]
fn read_parse_never_panics_and_requests_stay_within_the_disk() {
    for seed in 1..150_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 20) as usize;
        let body: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        let capacity = xorshift(&mut s);
        let payload_len =
            if seed % 2 == 0 { RW_HEADER_LEN as u32 } else { xorshift(&mut s) as u32 };
        let d = driver(capacity);

        if let Ok((lba, nsectors, bytes_n)) = parse_read(&d, &req(payload_len), &body) {
            assert!((1..=MAX_SECTORS_PER_REQUEST).contains(&nsectors));
            let last = lba.checked_add(nsectors as u64);
            assert!(last.is_some(), "lba + count overflowed");
            assert!(last.unwrap() <= capacity, "request reaches past the disk");
            assert_eq!(bytes_n, nsectors as usize * SECTOR_SIZE);
            assert_eq!(payload_len, RW_HEADER_LEN as u32);
        }
    }
}

#[test]
fn read_parse_enforces_the_exact_boundaries() {
    let cap = 1_000_000u64;
    let d = driver(cap);
    let hdr = req(RW_HEADER_LEN as u32);
    let ok = |lba, n| parse_read(&d, &hdr, &rw_body(lba, n, 0));
    assert_eq!(ok(cap - 1, 1), Ok((cap - 1, 1, SECTOR_SIZE)));
    assert_eq!(ok(cap, 1).err(), Some(E_NXIO));
    let m = MAX_SECTORS_PER_REQUEST;
    assert!(ok(cap - m as u64, m).is_ok());
    assert_eq!(ok(cap - m as u64 + 1, m).err(), Some(E_NXIO));
    assert_eq!(ok(0, 0).err(), Some(E_INVAL));
    assert_eq!(ok(0, m + 1).err(), Some(E_INVAL));
    assert_eq!(ok(u64::MAX, 1).err(), Some(E_INVAL));
    // A short body or a mismatched declared payload length is a framing error.
    for len in 0..RW_HEADER_LEN {
        assert_eq!(parse_read(&d, &hdr, &vec![0u8; len]).err(), Some(E_MSGSIZE));
    }
    assert_eq!(parse_read(&d, &req(11), &rw_body(0, 1, 0)).err(), Some(E_MSGSIZE));
    assert_eq!(parse_read(&d, &req(13), &rw_body(0, 1, 0)).err(), Some(E_MSGSIZE));
}

// Write requests: the body carries the data, so the framing must be exact.

#[test]
fn write_parse_never_panics_and_accepts_only_exact_frames() {
    for seed in 1..40_000u64 {
        let mut s = seed;
        let n = (xorshift(&mut s) % 70) as u32;
        let lba = xorshift(&mut s);
        let capacity = xorshift(&mut s);
        let mut body = rw_body(lba, n, n as usize * SECTOR_SIZE);
        // A third of the runs tamper with the frame.
        match seed % 3 {
            1 => body.push(0),
            2 => {
                body.pop();
            }
            _ => {}
        }
        let payload_len = if seed % 2 == 0 { body.len() as u32 } else { xorshift(&mut s) as u32 };
        let d = driver(capacity);

        if let Ok((plba, pn, bytes_n)) = parse_write(&d, &req(payload_len), &body) {
            assert!((1..=MAX_SECTORS_PER_REQUEST).contains(&pn));
            assert_eq!(plba, lba);
            assert_eq!(bytes_n, pn as usize * SECTOR_SIZE);
            assert_eq!(body.len(), RW_HEADER_LEN + bytes_n, "accepted frame must be exact");
            assert_eq!(payload_len as usize, body.len());
            let last = plba.checked_add(pn as u64);
            assert!(last.is_some());
            assert!(last.unwrap() <= capacity);
        }
    }
}

#[test]
fn write_parse_enforces_the_exact_boundaries() {
    let cap = 1_000u64;
    let d = driver(cap);
    let n1 = SECTOR_SIZE;
    let good = rw_body(cap - 1, 1, n1);
    let hdr = req((RW_HEADER_LEN + n1) as u32);
    assert_eq!(parse_write(&d, &hdr, &good), Ok((cap - 1, 1, n1)));
    assert_eq!(parse_write(&d, &req(RW_HEADER_LEN as u32 + 1), &good).err(), Some(E_MSGSIZE));
    let mut long = good.clone();
    long.push(0);
    assert_eq!(parse_write(&d, &hdr, &long).err(), Some(E_MSGSIZE));
    let mut short = good.clone();
    short.pop();
    assert_eq!(parse_write(&d, &hdr, &short).err(), Some(E_MSGSIZE));
    assert_eq!(parse_write(&d, &hdr, &rw_body(cap, 1, n1)).err(), Some(E_NXIO));
    assert_eq!(parse_write(&d, &req(RW_HEADER_LEN as u32), &rw_body(0, 0, 0)).err(), Some(E_INVAL));
    let m = MAX_SECTORS_PER_REQUEST;
    let over = rw_body(0, m + 1, (m + 1) as usize * SECTOR_SIZE);
    assert_eq!(parse_write(&d, &req(over.len() as u32), &over).err(), Some(E_INVAL));
    let wrap = rw_body(u64::MAX, 1, n1);
    assert_eq!(parse_write(&d, &req(wrap.len() as u32), &wrap).err(), Some(E_INVAL));
}

// The bounds-checked field readers behind both parsers.

#[test]
fn field_readers_are_inbounds_or_none_never_panicking() {
    let buf: Vec<u8> = (0u8..16).collect();
    assert_eq!(read_u32_le(&buf, 0), Some(u32::from_le_bytes([0, 1, 2, 3])));
    assert_eq!(read_u64_le(&buf, 8), Some(u64::from_le_bytes([8, 9, 10, 11, 12, 13, 14, 15])));
    assert_eq!(read_u32_le(&buf, 13), None);
    assert_eq!(read_u64_le(&buf, 9), None);
    assert_eq!(read_u32_le(&buf, usize::MAX), None, "offset overflow must be None");
    assert_eq!(read_u64_le(&buf, usize::MAX - 3), None);
    assert_eq!(read_u32_le(&[], 0), None);
}

// The wire header.

#[test]
fn decode_rejects_short_and_mistagged_headers_and_reads_fields_faithfully() {
    for len in 0..HDR_LEN {
        assert!(decode_request(&vec![0xa5u8; len]).is_none());
    }
    let mut hdr = [0u8; HDR_LEN];
    hdr[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    hdr[4..6].copy_from_slice(&VERSION.to_le_bytes());
    hdr[6..8].copy_from_slice(&4u16.to_le_bytes());
    hdr[16..20].copy_from_slice(&512u32.to_le_bytes());
    let r = decode_request(&hdr).expect("a tagged header decodes");
    assert_eq!((r.op, r.payload_len), (4, 512));
    let mut bad_magic = hdr;
    bad_magic[0] ^= 1;
    assert!(decode_request(&bad_magic).is_none());
    let mut bad_version = hdr;
    bad_version[4] ^= 1;
    assert!(decode_request(&bad_version).is_none());
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
        assert_eq!(back.flags, request.flags);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}

// Write authority.

#[test]
fn mutating_ops_answer_only_the_kernel_or_the_attested_installer() {
    use crate::protocol::{OP_CAPACITY, OP_FLUSH, OP_HEALTHCHECK, OP_READ_BLOCKS, OP_WRITE_BLOCKS};
    use crate::server::acl::rule::allows;
    for op in [OP_WRITE_BLOCKS, OP_FLUSH] {
        assert!(allows(op, 0, false), "the kernel client must never be refused");
        assert!(!allows(op, 7, false), "an unattested capsule reached a mutating op");
        assert!(allows(op, 7, true), "the attested installer was refused");
    }
    for op in [OP_CAPACITY, OP_READ_BLOCKS, OP_HEALTHCHECK, 0xffff] {
        assert!(allows(op, 7, false), "the read side must stay open as before");
    }
}

#[test]
fn the_installer_name_matches_exactly_or_not_at_all() {
    use crate::server::acl::rule::{entry_names_installer, INSTALLER_NAME};
    let want = INSTALLER_NAME.len() as u8;
    let mut name = [0u8; 24];
    name[..INSTALLER_NAME.len()].copy_from_slice(INSTALLER_NAME);
    assert!(entry_names_installer(&name, want));

    let mut longer = name;
    longer[INSTALLER_NAME.len()] = b'x';
    assert!(!entry_names_installer(&longer, want + 1), "a prefix extension passed");
    assert!(!entry_names_installer(&name, want - 1), "a truncation passed");

    let mut off = name;
    off[0] ^= 1;
    assert!(!entry_names_installer(&off, want), "a one-bit name change passed");

    assert!(!entry_names_installer(&name[..4], want), "a length past the buffer passed");
}
