// NONOS Operating System (AGPL-3.0-or-later)
//! Ground-truth proofs for the gen3 firmware parser against the real AX210
//! (so-a0-gf-a0) image: the runtime section count and byte totals, the section
//! load offsets, and the image-loader length are pinned to the actual bytes.
//! A wrong section type or a wrong framing offset fails here.

use crate::gen3::image::{find_iml, sections};

const AX210: &[u8] =
    include_bytes!("../../../nonos-bootloader/firmware/intel/iwlwifi-so-a0-gf-a0-86.ucode");

#[test]
fn the_image_loader_is_found_at_its_real_length() {
    let iml = find_iml(AX210).expect("AX210 image carries an IML (type 52)");
    assert_eq!(iml.len(), 13_944, "IML length in the real so-a0-gf-a0 image");
}

#[test]
fn the_runtime_sections_match_the_real_image() {
    let secs: Vec<_> = sections(AX210).collect();
    assert_eq!(secs.len(), 57, "runtime section count in the real image");

    // Total section bytes are the type-19 payload total (1_628_696) minus the
    // four-byte load-offset prefix stripped from each of the 57 sections.
    let data_total: usize = secs.iter().map(|s| s.data.len()).sum();
    assert_eq!(data_total, 1_628_696 - 57 * 4, "assembled runtime bytes");

    // The first few sections and their device load offsets, read straight from
    // the image: a small header section, then 32 KiB code blocks.
    assert_eq!(secs[0].load_offset, 0x0044_0000);
    assert_eq!(secs[0].data.len(), 1_656);
    assert_eq!(secs[1].load_offset, 0x0080_0000);
    assert_eq!(secs[1].data.len(), 32_768);
    assert_eq!(secs[2].load_offset, 0x0000_0000);
    assert_eq!(secs[3].load_offset, 0x0000_8000);
}

#[test]
fn a_truncated_image_yields_no_sections_and_no_loader() {
    // Below the 88-byte header there is nothing to parse; the walk stays empty
    // instead of reading out of bounds.
    assert!(find_iml(&AX210[..80]).is_none());
    assert_eq!(sections(&AX210[..80]).count(), 0);
}

// gen3 TX command wrapper proofs.
mod tx {
    use crate::gen3::tx_cmd::{build, IWL_TX_FLAGS_CMD_RATE, TX_CMD_GEN3_HDR};

    #[test]
    fn wraps_the_frame_after_a_28_byte_header() {
        let frame = [0xAAu8; 40];
        let mut out = [0u8; 128];
        let n = build(&frame, 0x0080_0003, &mut out).unwrap();
        assert_eq!(TX_CMD_GEN3_HDR, 28);
        assert_eq!(n, 28 + 40);
        // len @0 = frame length
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), 40);
        // flags @2 = use-command-rate
        assert_eq!(u16::from_le_bytes([out[2], out[3]]), IWL_TX_FLAGS_CMD_RATE);
        // rate_n_flags @16
        assert_eq!(u32::from_le_bytes([out[16], out[17], out[18], out[19]]), 0x0080_0003);
        // dram_info @8 and reserved @20 left zero for firmware
        assert_eq!(&out[8..16], &[0u8; 8]);
        assert_eq!(&out[20..28], &[0u8; 8]);
        // the frame follows the header verbatim
        assert_eq!(&out[28..28 + 40], &frame);
    }

    #[test]
    fn rejects_an_output_buffer_that_is_too_small() {
        let frame = [0u8; 40];
        let mut small = [0u8; 28 + 39];
        assert!(build(&frame, 0, &mut small).is_none());
    }
}

// gen3 RX-MPDU frame extraction proofs (descriptor size is a runtime input).
mod rx {
    use crate::gen3::rx_mpdu::extract_mpdu;

    // Build a notification: a `desc_len`-byte descriptor whose first two bytes
    // are mpdu_len, then the frame bytes.
    fn notif(desc_len: usize, frame: &[u8]) -> Vec<u8> {
        let mut v = vec![0u8; desc_len + frame.len()];
        v[0..2].copy_from_slice(&(frame.len() as u16).to_le_bytes());
        v[desc_len..].copy_from_slice(frame);
        v
    }

    #[test]
    fn extracts_the_frame_after_the_descriptor() {
        let frame = [0xDE, 0xAD, 0xBE, 0xEF, 0x11, 0x22, 0x33];
        for desc_len in [32usize, 40, 48] {
            let n = notif(desc_len, &frame);
            assert_eq!(extract_mpdu(&n, desc_len), Some(&frame[..]), "desc_len={desc_len}");
        }
    }

    #[test]
    fn rejects_a_payload_that_cannot_hold_the_claimed_frame() {
        // Claims a 100-byte frame but the buffer is far too short.
        let mut n = vec![0u8; 40 + 8];
        n[0..2].copy_from_slice(&100u16.to_le_bytes());
        assert!(extract_mpdu(&n, 40).is_none());
        // Too short to even hold mpdu_len.
        assert!(extract_mpdu(&[0u8; 1], 40).is_none());
    }
}
