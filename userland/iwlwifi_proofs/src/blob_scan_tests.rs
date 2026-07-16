// NONOS Operating System (AGPL-3.0-or-later)
//! Ground-truth proofs against the real embedded 7265 firmware. These exercise
//! the driver's own header parser and TLV constants against the actual bytes,
//! not a synthetic fixture, so a wrong header offset or a wrong section-type
//! number fails here even when a self-consistent unit test would pass. This is
//! the guard the earlier off-by-one and 20-vs-88 header bugs slipped past.

use crate::tlv::{parse_header, le32, TLV_HEADER_LEN, TLV_SEC_INIT, TLV_SEC_RT};

const BLOB: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-7265D-29.ucode");

// Walk the TLV chain and total the payload bytes carried under `want`.
fn total_bytes_of_type(want: u32) -> (usize, usize) {
    let mut off = TLV_HEADER_LEN;
    let (mut count, mut bytes) = (0usize, 0usize);
    while off + 8 <= BLOB.len() {
        let ty = le32(BLOB, off).unwrap();
        let len = le32(BLOB, off + 4).unwrap() as usize;
        off += 8;
        if off + len > BLOB.len() {
            break;
        }
        if ty == want {
            count += 1;
            bytes += len;
        }
        off += (len + 3) & !3;
    }
    (count, bytes)
}

#[test]
fn the_real_firmware_header_parses_with_the_driver_parser() {
    let h = parse_header(BLOB).expect("real 7265 firmware must parse");
    // The 7265D-29 image reports api 29; reading the version at the wrong
    // offset (inside the human-readable name) would not land on 29.
    assert_eq!(h.api, 29, "firmware api version read from offset 72");
    // A header truncated below its real 88-byte length must be rejected.
    assert!(parse_header(&BLOB[..TLV_HEADER_LEN - 1]).is_none());
}

#[test]
fn the_runtime_sections_live_under_the_section_type_the_driver_uses() {
    // The driver stages sections of type TLV_SEC_RT; in the real image those
    // are the four runtime sections. If TLV_SEC_RT were off by one it would
    // match the init sections instead, and this count/size would not hold.
    let (rt_count, rt_bytes) = total_bytes_of_type(TLV_SEC_RT);
    assert_eq!(rt_count, 4, "four runtime sections under TLV_SEC_RT");
    assert_eq!(rt_bytes, 364_400, "runtime section bytes in the real image");

    // Sanity that init sections are a distinct, separately-sized set, so the
    // two constants are not accidentally aliased.
    let (init_count, init_bytes) = total_bytes_of_type(TLV_SEC_INIT);
    assert_eq!(init_count, 4);
    assert_ne!(init_bytes, rt_bytes, "init and runtime are different images");
}
