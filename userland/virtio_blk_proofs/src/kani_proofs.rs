// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the virtio-blk parsers are total, bounded, and exactly
//! framed for every input. The Driver value carries null queue pointers, so
//! these proofs also establish that parsing never touches device memory.

use crate::constants::{MAX_SECTORS_PER_REQUEST, SECTOR_SIZE};
use crate::protocol::{decode_request, read_u32_le, read_u64_le, Request, RW_HEADER_LEN};
use crate::queue::Queue;
use crate::regs::Regs;
use crate::server::{read::parse_read, write::parse_write};
use crate::setup::Driver;

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

fn any_request() -> Request {
    Request {
        op: kani::any(),
        flags: kani::any(),
        request_id: kani::any(),
        payload_len: kani::any(),
    }
}

// For every body up to 16 bytes, every capacity, and every declared payload
// length: read parsing is total, and an accepted read stays on the disk with
// an exactly declared header length.
#[kani::proof]
fn read_parse_is_total_and_bounded() {
    let body: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= body.len());
    let capacity: u64 = kani::any();
    let req = any_request();
    let d = driver(capacity);

    if let Ok((lba, nsectors, bytes_n)) = parse_read(&d, &req, &body[..len]) {
        assert!(nsectors >= 1 && nsectors <= MAX_SECTORS_PER_REQUEST);
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
        assert!(bytes_n == nsectors as usize * SECTOR_SIZE);
        assert!(req.payload_len == RW_HEADER_LEN as u32);
    }
}

// For every body up to one sector plus header, every capacity, and every
// declared payload length: write parsing is total, and an accepted write is
// exactly framed: body and declared length both equal header plus data, the
// sector count is bounded, and the request stays on the disk.
#[kani::proof]
fn write_parse_is_total_and_exactly_framed() {
    let body: [u8; RW_HEADER_LEN + SECTOR_SIZE] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= body.len());
    let capacity: u64 = kani::any();
    let req = any_request();
    let d = driver(capacity);

    if let Ok((lba, nsectors, bytes_n)) = parse_write(&d, &req, &body[..len]) {
        assert!(nsectors >= 1 && nsectors <= MAX_SECTORS_PER_REQUEST);
        assert!(bytes_n == nsectors as usize * SECTOR_SIZE);
        assert!(len == RW_HEADER_LEN + bytes_n);
        assert!(req.payload_len as usize == len);
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
    }
}

// For every buffer up to 16 bytes and every offset: the field readers never
// panic, and a successful read lies entirely inside the buffer even when
// offset plus width would overflow a usize.
#[kani::proof]
fn field_readers_are_total_and_inbounds() {
    let buf: [u8; 16] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());
    let offset: usize = kani::any();

    if let Some(v) = read_u32_le(&buf[..len], offset) {
        assert!(offset + 4 <= len);
        assert_eq!(
            v,
            u32::from_le_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]])
        );
    }
    if read_u64_le(&buf[..len], offset).is_some() {
        assert!(offset + 8 <= len);
    }
}

// For every buffer up to a header plus slack: header decoding is total and
// every accepted field comes from its wire offset in little-endian order.
#[kani::proof]
fn decode_is_total_and_header_faithful() {
    let buf: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some(req) = decode_request(&buf[..len]) {
        assert!(len >= 20);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
        assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
    }
}

// For every op, sender and attestation verdict: a mutating operation passes
// only for the kernel client or the attested installer, and nothing else is
// ever refused.
#[kani::proof]
fn write_authority_is_exactly_the_kernel_or_the_installer() {
    use crate::protocol::{OP_FLUSH, OP_WRITE_BLOCKS};
    use crate::server::acl::rule::allows;
    let op: u16 = kani::any();
    let pid: u32 = kani::any();
    let attested: bool = kani::any();
    let verdict = allows(op, pid, attested);
    if op == OP_WRITE_BLOCKS || op == OP_FLUSH {
        assert_eq!(verdict, pid == 0 || attested);
    } else {
        assert!(verdict);
    }
}
