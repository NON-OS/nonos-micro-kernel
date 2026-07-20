// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proof for the RTL8821CE firmware header parser, run against the
//! real firmware image (rtw8821c_fw.bin, redistributed by linux-firmware). The
//! parser's fields are asserted against the values the card's own driver
//! reports (firmware 24.11.0, H2C format 12), and the section sizes are checked
//! for consistency with the image length, so the parser is validated against
//! genuine hardware firmware rather than a hand-made vector.

use crate::fw::header::{parse, FW_HDR_LEN};

// The actual firmware image, embedded from the driver tree.
const FW: &[u8] = include_bytes!("../../capsule_driver_rtl8821ce/firmware/rtw8821c_fw.bin");

#[test]
fn header_matches_the_real_firmware() {
    let h = parse(FW).expect("the firmware has a full header");
    assert_eq!(h.signature, 0x8821, "RTL8821C firmware signature");
    assert_eq!(h.version, 24, "firmware version 24.x, as the card reports");
    assert_eq!(h.subversion, 11, "subversion 11, giving 24.11.0");
    assert_eq!(h.h2c_fmt_ver, 12, "H2C command format version 12");
}

#[test]
fn section_sizes_are_consistent_with_the_image() {
    let h = parse(FW).unwrap();
    // The card loads the DMEM, IMEM and EMEM sections to these on-chip
    // addresses; the values come from the image itself.
    assert_eq!(h.dmem_addr, 0x8020_0000);
    assert_eq!(h.imem_addr, 0x8003_0000);
    assert_eq!(h.emem_addr, 0x8010_0000);
    let sections = h.dmem_size as usize + h.imem_size as usize + h.emem_size as usize;
    // Header plus the three sections accounts for the whole image (the small
    // remainder is section alignment padding the download walks past).
    assert!(FW_HDR_LEN + sections <= FW.len(), "sections fit within the image");
    assert!(FW.len() - (FW_HDR_LEN + sections) < 64, "only alignment padding remains");
}

#[test]
fn a_truncated_image_has_no_header() {
    assert!(parse(&FW[..FW_HDR_LEN - 1]).is_none(), "a short image is rejected");
}
