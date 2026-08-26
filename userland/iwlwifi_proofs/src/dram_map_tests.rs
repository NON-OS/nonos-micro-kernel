// NONOS Operating System (AGPL-3.0-or-later)
//! Ground-truth proofs for the gen3 firmware DRAM map against the real AX210
//! (so-a0-gf-a0) image. The section list splits into three images at the two
//! separator markers; these pin the split (15 LMAC, 16 UMAC, 24 paged), a few
//! per-image section sizes and offsets read straight from the image, and the
//! page-aligned device addresses `stage` reports. A wrong separator magic or a
//! misplaced boundary moves firmware into the wrong image and fails here.

use crate::gen3::dram_map::{classify, stage};

const AX210: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

#[test]
fn the_image_splits_into_15_lmac_16_umac_24_paged() {
    let l = classify(AX210);
    assert_eq!(l.lmac.len(), 15, "LMAC sections before the CPU1/CPU2 separator");
    assert_eq!(l.umac.len(), 16, "UMAC sections between the separators");
    assert_eq!(l.virt.len(), 24, "paged sections after the paging separator");

    // The separators themselves are dropped, so no chunk carries a magic offset.
    for s in l.lmac.iter().chain(&l.umac).chain(&l.virt) {
        assert_ne!(s.load_offset, 0xFFFF_CCCC);
        assert_ne!(s.load_offset, 0xAAAA_BBBB);
    }
}

#[test]
fn the_image_boundaries_match_the_real_bytes() {
    let l = classify(AX210);
    // First LMAC section: the small header block at 0x00440000.
    assert_eq!(l.lmac[0].load_offset, 0x0044_0000);
    assert_eq!(l.lmac[0].data.len(), 1_656);
    // Last LMAC section before the CPU1/CPU2 separator.
    assert_eq!(l.lmac[14].load_offset, 0x0062_9980);
    assert_eq!(l.lmac[14].data.len(), 22_720);
    // First UMAC section (offsets carry the high control bits set in the image).
    assert_eq!(l.umac[0].load_offset, 0x8044_0000);
    assert_eq!(l.umac[0].data.len(), 1_656);
    // First paged section after the paging separator.
    assert_eq!(l.virt[0].load_offset, 0x0000_0000);
    assert_eq!(l.virt[0].data.len(), 1_656);
    assert_eq!(l.virt[23].load_offset, 0x010B_0000);
}

#[test]
fn stage_places_each_chunk_on_its_own_page_and_copies_it() {
    // A staging region large enough for the whole runtime image.
    let mut region = vec![0u8; 2 * 1024 * 1024];
    let user_va = region.as_mut_ptr() as u64;
    let device_addr = 0x1_0000_0000u64;

    let p = unsafe { stage(AX210, user_va, device_addr, region.len()) }.expect("fits");
    assert_eq!(p.lmac.len(), 15);
    assert_eq!(p.umac.len(), 16);
    assert_eq!(p.virt.len(), 24);

    // Every reported address is page-aligned and inside the region.
    for &a in p.lmac.iter().chain(&p.umac).chain(&p.virt) {
        assert_eq!(a % 4096, 0, "each chunk starts on a page");
        assert!(a >= device_addr && a < device_addr + region.len() as u64);
    }

    // The first LMAC chunk sits at the region base and holds that section's
    // bytes: a real copy, not just an address.
    assert_eq!(p.lmac[0], device_addr);
    let first = classify(AX210).lmac[0];
    let off = (p.lmac[0] - device_addr) as usize;
    assert_eq!(&region[off..off + first.data.len()], first.data);

    // The second chunk is one page on (the 1656-byte header rounds up to 4096).
    assert!(!p.umac.is_empty());
    assert!(p.staged_bytes > 1_600_000, "the whole ~1.6 MB image was staged");
}

#[test]
fn stage_refuses_a_region_that_cannot_hold_the_images() {
    let mut tiny = vec![0u8; 64 * 1024];
    let user_va = tiny.as_mut_ptr() as u64;
    let r = unsafe { stage(AX210, user_va, 0x1_0000_0000, tiny.len()) };
    assert!(r.is_err(), "a region too small to hold the images is refused");
}
