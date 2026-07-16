// NONOS Operating System (AGPL-3.0-or-later)
//! Known-answer proof for the firmware section split, run against the real
//! RTL8821CE image. It asserts the three sections land at the on-chip DMEM,
//! IMEM and EMEM addresses the firmware declares, that together with the header
//! they cover the whole image exactly (each section carrying its 8-byte
//! checksum), that the slices are contiguous and in range, and that a truncated
//! image is rejected before any transfer.

use crate::fw::header::{parse, FW_HDR_LEN};
use crate::fw::sections::{sections, SECTION_CHKSUM_LEN};

const FW: &[u8] = include_bytes!("../../capsule_driver_rtl8821ce/firmware/rtw8821c_fw.bin");

#[test]
fn the_real_firmware_splits_into_three_addressed_sections() {
    let h = parse(FW).unwrap();
    let (secs, n) = sections(&h, FW.len()).expect("sections computed");
    assert_eq!(n, 3, "this image carries DMEM, IMEM and EMEM");
    // The image declares 0x8020_0000 etc.; bit 31 is an image-format marker the
    // download clears, so the sections land at the real on-chip OCPBASE
    // addresses (DMEM 0x0020_0000, IMEM 0x0003_0000, EMEM 0x0010_0000).
    assert_eq!(secs[0].dst, 0x0020_0000, "DMEM on-chip address");
    assert_eq!(secs[1].dst, 0x0003_0000, "IMEM on-chip address");
    assert_eq!(secs[2].dst, 0x0010_0000, "EMEM on-chip address");
}

#[test]
fn sections_are_contiguous_and_cover_the_whole_image() {
    let h = parse(FW).unwrap();
    let (secs, n) = sections(&h, FW.len()).unwrap();
    // First section begins right after the header.
    assert_eq!(secs[0].offset, FW_HDR_LEN);
    // Each section abuts the next.
    for i in 1..n {
        assert_eq!(secs[i].offset, secs[i - 1].offset + secs[i - 1].len, "no gap or overlap");
    }
    // Header plus every section byte accounts for the entire image.
    let last = secs[n - 1];
    assert_eq!(last.offset + last.len, FW.len(), "the sections end exactly at the image end");
    // Each declared size carries its checksum.
    assert_eq!(secs[0].len, h.dmem_size as usize + SECTION_CHKSUM_LEN);
    assert_eq!(secs[1].len, h.imem_size as usize + SECTION_CHKSUM_LEN);
    assert_eq!(secs[2].len, h.emem_size as usize + SECTION_CHKSUM_LEN);
}

#[test]
fn a_truncated_image_is_rejected() {
    let h = parse(FW).unwrap();
    // Claim the image is shorter than the sections need.
    assert!(sections(&h, FW_HDR_LEN + 10).is_none(), "sections past the image end are rejected");
}
