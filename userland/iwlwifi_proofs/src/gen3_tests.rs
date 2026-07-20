// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the gen3 (AX210-class) firmware self-load handoff. The context
//! information structure is byte-checked field by field against the layout the
//! boot ROM reads (iwl-context-info-gen3.h), and the boot register sequence is
//! captured against a modeled device and checked write for write.

use crate::gen3::boot::kick;
use crate::gen3::csr::{
    CSR_AUTO_FUNC_BOOT_ENA, CSR_CTXT_INFO_ADDR, CSR_CTXT_INFO_BOOT_CTRL, CSR_IML_DATA_ADDR,
    CSR_IML_SIZE_ADDR,
};
use crate::gen3::ctxt_info::{CtxtInfoGen3, CTXT_INFO_GEN3_SIZE};
use crate::regs::Mmio;
use core::cell::RefCell;

fn rd64(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
}
fn rd32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}
fn rd16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes(buf[off..off + 2].try_into().unwrap())
}

// Distinct sentinels so a field written at the wrong offset shows up as a
// mismatch rather than aliasing another field's value.
fn sample() -> CtxtInfoGen3 {
    CtxtInfoGen3 {
        prph_info_base: 0x1111_1111_0000_0008,
        cr_head_idx_arr_base: 0x2222_2222_0000_0010,
        tr_tail_idx_arr_base: 0x3333_3333_0000_0018,
        cr_tail_idx_arr_base: 0x4444_4444_0000_0020,
        cr_idx_arr_size: 0x0055,
        tr_idx_arr_size: 0x0066,
        mtr_base: 0x7777_7777_0000_0034,
        mcr_base: 0x8888_8888_0000_003C,
        mtr_size: 0x0099,
        mcr_size: 0x00AA,
        prph_scratch_base: 0xBBBB_BBBB_0000_0058,
        prph_scratch_size: 0x0000_00CC,
    }
}

#[test]
fn context_info_is_104_bytes_and_every_field_lands_at_its_offset() {
    let mut buf = [0xFFu8; 128];
    assert!(sample().write(&mut buf));
    // The structure is exactly 104 bytes; the writer must not touch beyond it.
    assert_eq!(CTXT_INFO_GEN3_SIZE, 104);
    assert_eq!(buf[CTXT_INFO_GEN3_SIZE], 0xFF, "byte past the structure is untouched");

    assert_eq!(rd64(&buf, 8), 0x1111_1111_0000_0008, "prph_info_base_addr @8");
    assert_eq!(rd64(&buf, 16), 0x2222_2222_0000_0010, "cr_head_idx_arr_base_addr @16");
    assert_eq!(rd64(&buf, 24), 0x3333_3333_0000_0018, "tr_tail_idx_arr_base_addr @24");
    assert_eq!(rd64(&buf, 32), 0x4444_4444_0000_0020, "cr_tail_idx_arr_base_addr @32");
    assert_eq!(rd16(&buf, 48), 0x0055, "cr_idx_arr_size @48");
    assert_eq!(rd16(&buf, 50), 0x0066, "tr_idx_arr_size @50");
    assert_eq!(rd64(&buf, 52), 0x7777_7777_0000_0034, "mtr_base_addr @52");
    assert_eq!(rd64(&buf, 60), 0x8888_8888_0000_003C, "mcr_base_addr @60");
    assert_eq!(rd16(&buf, 68), 0x0099, "mtr_size @68");
    assert_eq!(rd16(&buf, 70), 0x00AA, "mcr_size @70");
    assert_eq!(rd64(&buf, 88), 0xBBBB_BBBB_0000_0058, "prph_scratch_base_addr @88");
    assert_eq!(rd32(&buf, 96), 0x0000_00CC, "prph_scratch_size @96");
}

#[test]
fn context_info_leaves_the_reserved_and_version_words_zero() {
    let mut buf = [0u8; CTXT_INFO_GEN3_SIZE];
    assert!(sample().write(&mut buf));
    // version/size/config (bytes 0..8) and the trailing reserved word (100..104)
    // are not set by the driver; the ROM reads them from the zeroed structure.
    assert_eq!(rd64(&buf, 0), 0, "version/size/config stay zero");
    assert_eq!(rd32(&buf, 100), 0, "trailing reserved word stays zero");
}

#[test]
fn write_rejects_a_short_buffer() {
    let mut small = [0u8; CTXT_INFO_GEN3_SIZE - 1];
    assert!(!sample().write(&mut small));
}

struct MockMmio {
    writes: RefCell<Vec<(usize, u32)>>,
}
impl Mmio for MockMmio {
    fn read32(&self, _off: usize) -> u32 {
        0
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

#[test]
fn boot_writes_the_addresses_then_sets_the_auto_boot_bit_in_order() {
    let m = MockMmio { writes: RefCell::new(Vec::new()) };
    let ctxt = 0xDEAD_BEEF_1234_5678u64;
    let iml = 0x0000_00A5_9ABC_DEF0u64;
    let iml_len = 0x4321u32;
    kick(&m, ctxt, iml, iml_len);

    let w = m.writes.borrow();
    assert_eq!(
        *w,
        vec![
            // context info physical address, low dword then high dword
            (CSR_CTXT_INFO_ADDR, 0x1234_5678),
            (CSR_CTXT_INFO_ADDR + 4, 0xDEAD_BEEF),
            // image loader physical address, low then high
            (CSR_IML_DATA_ADDR, 0x9ABC_DEF0),
            (CSR_IML_DATA_ADDR + 4, 0x0000_00A5),
            // image loader length
            (CSR_IML_SIZE_ADDR, 0x4321),
            // auto-boot enable (read-modify-write from a zero register)
            (CSR_CTXT_INFO_BOOT_CTRL, CSR_AUTO_FUNC_BOOT_ENA),
        ],
        "gen3 boot programs ctxt-info, IML, size, then auto-boot, in that order"
    );
}
