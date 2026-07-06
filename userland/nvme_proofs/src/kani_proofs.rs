// NONOS Operating System (AGPL-3.0-or-later)
//! Kani harnesses: the NVMe parsers are total and faithful for every input.

use crate::admin::{ControllerIdentity, NamespaceIdentity, SmartHealth};
use crate::nvm::MAX_SECTORS;
use crate::protocol::{decode_request, HDR_LEN};
use crate::server::handlers::parse_rw;

// For every 12-byte request and every capacity: parsing is total, and an
// accepted request has a bounded sector count and never reaches past the disk.
#[kani::proof]
fn rw_parse_is_total_and_bounded() {
    let body: [u8; 12] = kani::any();
    let capacity: u64 = kani::any();

    if let Ok((lba, nsectors)) = parse_rw(&body, capacity) {
        assert!((1..=MAX_SECTORS).contains(&nsectors));
        let last = lba.checked_add(nsectors as u64);
        assert!(last.is_some());
        assert!(last.unwrap() <= capacity);
    }
}

// For every buffer up to one full header plus slack: decoding is total, an
// accepted header is at least HDR_LEN long, and every field comes from its
// wire offset in little-endian order.
#[kani::proof]
fn decode_is_total_and_header_faithful() {
    let buf: [u8; 24] = kani::any();
    let len: usize = kani::any();
    kani::assume(len <= buf.len());

    if let Some(req) = decode_request(&buf[..len]) {
        assert!(len >= HDR_LEN);
        assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
        assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
        assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
        assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
    }
}

// For every 4096-byte identify controller page: parsing never panics and the
// scalar fields are read from their spec offsets.
#[kani::proof]
fn identify_controller_parse_is_total() {
    let page: [u8; 4096] = kani::any();
    let id = ControllerIdentity::parse(&page);
    assert_eq!(id.vendor_id, u16::from_le_bytes([page[0x00], page[0x01]]));
    assert_eq!(id.mdts, page[0x4d]);
    assert_eq!(
        id.namespace_count,
        u32::from_le_bytes([page[0x204], page[0x205], page[0x206], page[0x207]])
    );
    assert_eq!(id.volatile_write_cache, page[0x20d]);
}

// For every 4096-byte identify namespace page and every nsid: parsing never
// panics for any of the 16 device-chosen LBA-format slots, the block size is
// zero or a power of two, and the formatted count never wraps to zero.
#[kani::proof]
fn identify_namespace_parse_is_total() {
    let page: [u8; 4096] = kani::any();
    let ns = NamespaceIdentity::parse(kani::any(), &page);
    assert!(ns.format_index <= 0x0f);
    assert!(ns.lba_size == 0 || ns.lba_size.is_power_of_two());
    assert!(ns.formatted_lba_count >= 1);
}

// For every 512-byte SMART log page: parsing never panics and the fields are
// read from their spec offsets.
#[kani::proof]
fn smart_health_parse_is_total() {
    let page: [u8; 512] = kani::any();
    let h = SmartHealth::parse(&page);
    assert_eq!(h.critical_warning, page[0]);
    assert_eq!(h.temperature_kelvin, u16::from_le_bytes([page[1], page[2]]));
    assert_eq!(
        h.critical_temp_time,
        u32::from_le_bytes([page[196], page[197], page[198], page[199]])
    );
}
