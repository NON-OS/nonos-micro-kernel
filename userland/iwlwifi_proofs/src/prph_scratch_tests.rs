// NONOS Operating System (AGPL-3.0-or-later)
//! Ground-truth proofs for the gen3 peripheral-scratch structure. The layout is
//! pinned to `struct iwl_prph_scratch` in Linux iwlwifi (pcie/iwl-context-info-v2.h)
//! and the DRAM image map in pcie/iwl-context-info.h: a wrong field offset, a
//! wrong struct size, or a byte written into the reserved gaps fails here. This
//! is the block the context-information structure points at; if it is malformed
//! the boot ROM reads garbage and the device never ALIVEs.

use crate::gen3::prph_scratch::flags::CTRL_RB_SIZE_4K;
use crate::gen3::prph_scratch::{DramImage, PrphScratch, MAX_DRAM_ENTRY, PRPH_SCRATCH_SIZE};

fn rd16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn rd32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}
fn rd64(b: &[u8], o: usize) -> u64 {
    let mut v = [0u8; 8];
    v.copy_from_slice(&b[o..o + 8]);
    u64::from_le_bytes(v)
}

#[test]
fn the_structure_is_1724_bytes() {
    // ctrl_cfg (84) + fseq_override (4) + step_analog_params (4)
    // + reserved[8] (32) + dram: umac[64] + lmac[64] + virtual[64] + fseq[8]
    // = 84 + 40 + (512 + 512 + 512 + 64) = 1724.
    assert_eq!(PRPH_SCRATCH_SIZE, 1724);
    assert_eq!(MAX_DRAM_ENTRY, 64);
}

#[test]
fn the_addresses_land_at_their_documented_offsets() {
    let umac = [0x1000u64, 0x2000, 0x3000];
    let lmac = [0xA000u64, 0xB000];
    let virt = [0xC000u64];
    let s = PrphScratch {
        mac_id: 0x0034,
        version: 0x0001,
        control_flags: CTRL_RB_SIZE_4K,
        control_flags_ext: 0,
        pnvm_base: 0x1_2345_0000,
        pnvm_size: 0x4000,
        reduce_power_base: 0x9_8765_0000,
        reduce_power_size: 0x0800,
        free_rbd_addr: 0x5_5555_0000,
        dram: DramImage { umac: &umac, lmac: &lmac, virt: &virt },
    };
    let mut buf = [0xEEu8; PRPH_SCRATCH_SIZE];
    assert!(s.write(&mut buf));

    // version @0: mac_id, version, size-in-dwords (84/4 = 21).
    assert_eq!(rd16(&buf, 0), 0x0034);
    assert_eq!(rd16(&buf, 2), 0x0001);
    assert_eq!(rd16(&buf, 4), 21);

    // control @8, control_ext @12.
    assert_eq!(rd32(&buf, 8), CTRL_RB_SIZE_4K);
    assert_eq!(rd32(&buf, 12), 0);

    // pnvm_cfg @16: base then size.
    assert_eq!(rd64(&buf, 16), 0x1_2345_0000);
    assert_eq!(rd32(&buf, 24), 0x4000);

    // rbd_cfg.free_rbd_addr @48.
    assert_eq!(rd64(&buf, 48), 0x5_5555_0000);

    // reduce_power_cfg @60: base then size.
    assert_eq!(rd64(&buf, 60), 0x9_8765_0000);
    assert_eq!(rd32(&buf, 68), 0x0800);

    // dram image arrays: umac @124, lmac @124+512=636, virtual @636+512=1148.
    assert_eq!(rd64(&buf, 124), 0x1000);
    assert_eq!(rd64(&buf, 124 + 8), 0x2000);
    assert_eq!(rd64(&buf, 124 + 16), 0x3000);
    assert_eq!(rd64(&buf, 636), 0xA000);
    assert_eq!(rd64(&buf, 636 + 8), 0xB000);
    assert_eq!(rd64(&buf, 1148), 0xC000);
}

#[test]
fn unused_dram_entries_and_the_hwm_gap_stay_zero() {
    let umac = [0x1000u64];
    let s = PrphScratch {
        mac_id: 0,
        version: 0,
        control_flags: 0,
        control_flags_ext: 0,
        pnvm_base: 0,
        pnvm_size: 0,
        reduce_power_base: 0,
        reduce_power_size: 0,
        free_rbd_addr: 0,
        dram: DramImage { umac: &umac, lmac: &[], virt: &[] },
    };
    let mut buf = [0xEEu8; PRPH_SCRATCH_SIZE];
    assert!(s.write(&mut buf));
    // The second umac slot was not supplied, so it must be zero, not 0xEE.
    assert_eq!(rd64(&buf, 124 + 8), 0);
    // The hwm_cfg block (@32..48) is not programmed and must be clear.
    assert_eq!(rd64(&buf, 32), 0);
    assert_eq!(rd32(&buf, 40), 0);
    // The fseq_img array (last 64 bytes) stays zero: no external FSEQ image.
    assert_eq!(rd64(&buf, PRPH_SCRATCH_SIZE - 8), 0);
}

#[test]
fn an_oversized_image_or_short_buffer_is_refused() {
    let too_many = [0u64; MAX_DRAM_ENTRY + 1];
    let s = PrphScratch {
        mac_id: 0,
        version: 0,
        control_flags: 0,
        control_flags_ext: 0,
        pnvm_base: 0,
        pnvm_size: 0,
        reduce_power_base: 0,
        reduce_power_size: 0,
        free_rbd_addr: 0,
        dram: DramImage { umac: &too_many, lmac: &[], virt: &[] },
    };
    let mut buf = [0u8; PRPH_SCRATCH_SIZE];
    assert!(!s.write(&mut buf), "an image with more than 64 chunks is refused");

    let ok = PrphScratch { dram: DramImage { umac: &[], lmac: &[], virt: &[] }, ..s };
    let mut small = [0u8; PRPH_SCRATCH_SIZE - 1];
    assert!(!ok.write(&mut small), "a buffer below the struct size is refused");
}
