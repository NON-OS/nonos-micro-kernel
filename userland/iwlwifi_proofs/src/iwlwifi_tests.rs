// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::{
    CSR_FH_INT_STATUS, FH_SRVC_CHNL_SRAM_ADDR_REG, FH_TCSR_CHNL_TX_BUF_STS_REG,
    FH_TCSR_CHNL_TX_CONFIG_REG, FH_TCSR_TX_BUF_STS_TFDB_VALID, FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD,
    FH_TCSR_TX_CONFIG_DMA_ENABLE, FH_TCSR_TX_CONFIG_DMA_PAUSE, FH_TFDIB_CTRL0_REG,
    FH_TFDIB_CTRL1_REG, FH_TFDIB_REG1_ADDR_BITSHIFT, HBUS_TARG_PRPH_WADDR, HBUS_TARG_PRPH_WDAT,
    PRPH_WADDR_ADDR_MASK, PRPH_WADDR_WORD_ENABLE, RELEASE_CPU_RESET, RELEASE_CPU_RESET_BIT,
};
use crate::load::{load_sections, release_cpu};
use crate::regs::Mmio;
use core::cell::RefCell;

// Writes per section: 6 FH config writes + 1 completion acknowledge.
const WRITES_PER_SECTION: usize = 7;

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

// A real chip sets the FH interrupt status once the DMA finishes; the model
// reports it as already set so the driver's completion poll returns on the
// first read. The sequence under test is the driver's own register program.
const FH_DONE: u32 = 0x8000_0000;

struct MockMmio {
    writes: RefCell<Vec<(usize, u32)>>,
}

impl MockMmio {
    fn new() -> Self {
        Self { writes: RefCell::new(Vec::new()) }
    }
}

impl Mmio for MockMmio {
    fn read32(&self, off: usize) -> u32 {
        if off == CSR_FH_INT_STATUS {
            FH_DONE
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        self.writes.borrow_mut().push((off, val));
    }
}

// Build a staged buffer of `[dest:u32][len:u32][pad:u32][payload]` blocks, the
// exact layout `stage_section` produces, and return the per-section metadata
// (dest, payload_offset, len) so the expected physical addresses can be checked.
fn staged(sections: &[(u32, usize)]) -> (Vec<u8>, Vec<(u32, usize, usize)>) {
    let mut buf = Vec::new();
    let mut meta = Vec::new();
    for &(dest, len) in sections {
        buf.extend_from_slice(&dest.to_le_bytes());
        buf.extend_from_slice(&(len as u32).to_le_bytes());
        buf.extend_from_slice(&0u32.to_le_bytes());
        let payload_off = buf.len();
        buf.extend(core::iter::repeat(0xABu8).take(len));
        meta.push((dest, payload_off, len));
    }
    (buf, meta)
}

fn expected_chunk_writes(dest: u32, phys: u64, len: usize) -> Vec<(usize, u32)> {
    vec![
        (FH_TCSR_CHNL_TX_CONFIG_REG, FH_TCSR_TX_CONFIG_DMA_PAUSE),
        (FH_SRVC_CHNL_SRAM_ADDR_REG, dest),
        (FH_TFDIB_CTRL0_REG, (phys & 0xFFFF_FFFF) as u32),
        (
            FH_TFDIB_CTRL1_REG,
            (((phys >> 32) as u32) << FH_TFDIB_REG1_ADDR_BITSHIFT) | len as u32,
        ),
        (FH_TCSR_CHNL_TX_BUF_STS_REG, FH_TCSR_TX_BUF_STS_TFDB_VALID),
        (
            FH_TCSR_CHNL_TX_CONFIG_REG,
            FH_TCSR_TX_CONFIG_DMA_ENABLE | FH_TCSR_TX_CONFIG_CIRQ_HOST_ENDTFD,
        ),
        (CSR_FH_INT_STATUS, FH_DONE), // completion acknowledge
    ]
}

#[test]
fn load_programs_exact_fh_sequence_per_section() {
    // Physical base above 4 GiB so the high-address bits in TFDIB_CTRL1 matter.
    let dev_base: u64 = 0x2_0000_0000;
    let (buf, meta) = staged(&[(0x0000_0000, 16), (0x0080_0000, 40), (0x0010_0000, 4)]);
    let mmio = MockMmio::new();

    let n = load_sections(&mmio, buf.as_ptr() as u64, dev_base, buf.len() as u32)
        .expect("all sections load");
    assert_eq!(n, meta.len() as u32, "every staged section is transferred");

    let mut expected: Vec<(usize, u32)> = Vec::new();
    for &(dest, payload_off, len) in &meta {
        expected.extend(expected_chunk_writes(dest, dev_base + payload_off as u64, len));
    }
    assert_eq!(
        *mmio.writes.borrow(),
        expected,
        "the FH transfer must program the exact documented register sequence and values"
    );
}

#[test]
fn empty_staging_transfers_nothing() {
    let mmio = MockMmio::new();
    let n = load_sections(&mmio, 0, 0, 0).expect("empty is ok");
    assert_eq!(n, 0);
    assert!(mmio.writes.borrow().is_empty(), "no register writes for empty staging");
}

#[test]
fn overrunning_section_is_rejected() {
    // A block claims a payload length that runs past the staged region.
    let mut buf = Vec::new();
    buf.extend_from_slice(&0x1234u32.to_le_bytes());
    buf.extend_from_slice(&64u32.to_le_bytes()); // 64, but only 4 payload bytes follow
    buf.extend_from_slice(&0u32.to_le_bytes());
    buf.extend_from_slice(&[0u8; 4]);
    let mmio = MockMmio::new();
    let r = load_sections(&mmio, buf.as_ptr() as u64, 0x1000, buf.len() as u32);
    assert!(r.is_err(), "a section overrunning the buffer must be refused");
    assert!(
        mmio.writes.borrow().is_empty(),
        "no chunk may be programmed for an overrunning section"
    );
}

#[test]
fn zero_length_section_is_rejected() {
    let (buf, _) = staged(&[(0x2000, 0)]);
    let mmio = MockMmio::new();
    let r = load_sections(&mmio, buf.as_ptr() as u64, 0x1000, buf.len() as u32);
    assert!(r.is_err(), "a zero-length section is malformed");
}

#[test]
fn release_cpu_programs_prph_write_of_the_release_bit() {
    let mmio = MockMmio::new();
    release_cpu(&mmio);
    assert_eq!(
        *mmio.writes.borrow(),
        vec![
            (
                HBUS_TARG_PRPH_WADDR,
                (RELEASE_CPU_RESET & PRPH_WADDR_ADDR_MASK) | PRPH_WADDR_WORD_ENABLE
            ),
            (HBUS_TARG_PRPH_WDAT, RELEASE_CPU_RESET_BIT),
        ],
        "CPU release must be a PRPH indirect write of the release bit"
    );
}

// The full firmware start is the transfer of every section followed by the CPU
// release, in that order. This is what Driver::load_firmware performs.
#[test]
fn full_start_is_all_sections_then_cpu_release() {
    let dev_base: u64 = 0x1_0000_0000;
    let (buf, meta) = staged(&[(0x10, 8), (0x20, 12)]);
    let mmio = MockMmio::new();

    let n = load_sections(&mmio, buf.as_ptr() as u64, dev_base, buf.len() as u32).unwrap();
    release_cpu(&mmio);

    let writes = mmio.writes.borrow();
    assert_eq!(n as usize, meta.len());
    // All the per-section writes come first, then exactly the two PRPH writes.
    assert_eq!(writes.len(), meta.len() * WRITES_PER_SECTION + 2);
    assert_eq!(writes[writes.len() - 2].0, HBUS_TARG_PRPH_WADDR);
    assert_eq!(writes[writes.len() - 1], (HBUS_TARG_PRPH_WDAT, RELEASE_CPU_RESET_BIT));
}

// Fuzz over many random well-formed staging layouts: every section must be
// transferred with correct addressing and no panic, and the write count must
// be exact.
#[test]
fn fuzz_load_transfers_every_section_correctly() {
    let dev_base: u64 = 0x1_0000_0000;
    for seed in 1..5000u64 {
        let mut s = seed;
        let count = (xorshift(&mut s) % 6) as usize;
        let mut specs = Vec::new();
        for _ in 0..count {
            let dest = (xorshift(&mut s) & 0x00FF_FFFF) as u32;
            let len = 1 + (xorshift(&mut s) % 256) as usize;
            specs.push((dest, len));
        }
        let (buf, meta) = staged(&specs);
        let mmio = MockMmio::new();
        let n = load_sections(&mmio, buf.as_ptr() as u64, dev_base, buf.len() as u32)
            .expect("well-formed staging always loads");
        assert_eq!(n as usize, meta.len());

        let writes = mmio.writes.borrow();
        assert_eq!(writes.len(), meta.len() * WRITES_PER_SECTION);
        for (i, &(dest, payload_off, len)) in meta.iter().enumerate() {
            let base = i * WRITES_PER_SECTION;
            assert_eq!(writes[base], (FH_TCSR_CHNL_TX_CONFIG_REG, FH_TCSR_TX_CONFIG_DMA_PAUSE));
            assert_eq!(writes[base + 1], (FH_SRVC_CHNL_SRAM_ADDR_REG, dest));
            let phys = dev_base + payload_off as u64;
            assert_eq!(writes[base + 2], (FH_TFDIB_CTRL0_REG, (phys & 0xFFFF_FFFF) as u32));
            assert_eq!(writes[base + 3].0, FH_TFDIB_CTRL1_REG);
            assert_eq!(writes[base + 3].1 & 0x0FFF_FFFF, len as u32, "byte count field");
            assert_eq!(writes[base + 4], (FH_TCSR_CHNL_TX_BUF_STS_REG, FH_TCSR_TX_BUF_STS_TFDB_VALID));
        }
    }
}
