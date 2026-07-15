// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the completed Bulk-Only Transport status validation and the SCSI
//! command and response codec. The status wrapper and the INQUIRY, READ
//! CAPACITY(10) and REQUEST SENSE payloads are all device-controlled, so these
//! run the real codec over spec-shaped and hostile bytes: a status wrapper is
//! trusted only when it echoes the command tag and its residue fits the
//! request, and every response decoder is byte-faithful and never reads past
//! its input.

use crate::bot::{self, validate, CommandBlockWrapper, TransferOutcome, ValidateError, CBW_FLAG_IN};
use crate::protocol::CBW_LEN;
use crate::scsi;

extern crate alloc;
use alloc::vec::Vec;

fn make_csw(tag: u32, residue: u32, status: u8) -> [u8; 13] {
    let mut b = [0u8; 13];
    b[0..4].copy_from_slice(&0x5342_5355u32.to_le_bytes());
    b[4..8].copy_from_slice(&tag.to_le_bytes());
    b[8..12].copy_from_slice(&residue.to_le_bytes());
    b[12] = status;
    b
}

// Bulk-Only Transport status validation (§6.3).

#[test]
fn a_status_wrapper_is_rejected_unless_it_echoes_the_command_tag() {
    let csw = bot::parse(&make_csw(0x1234_5678, 0, 0)).unwrap();
    assert_eq!(validate(csw, 0x1234_5678, 512), Ok(TransferOutcome::Passed { transferred: 512 }));
    assert_eq!(validate(csw, 0x1234_5679, 512), Err(ValidateError::TagMismatch));
}

#[test]
fn a_residue_beyond_the_request_or_a_phase_status_is_a_phase_error() {
    let over = bot::parse(&make_csw(1, 600, 0)).unwrap();
    assert_eq!(validate(over, 1, 512), Err(ValidateError::PhaseError));
    let phase = bot::parse(&make_csw(1, 0, 2)).unwrap();
    assert_eq!(validate(phase, 1, 512), Err(ValidateError::PhaseError));
}

#[test]
fn validation_reports_the_bytes_actually_transferred() {
    let short = bot::parse(&make_csw(5, 100, 0)).unwrap();
    assert_eq!(validate(short, 5, 512), Ok(TransferOutcome::Passed { transferred: 412 }));
    let failed = bot::parse(&make_csw(5, 0, 1)).unwrap();
    assert_eq!(validate(failed, 5, 512), Ok(TransferOutcome::Failed { transferred: 512 }));
    let full = bot::parse(&make_csw(5, 0, 0)).unwrap();
    assert_eq!(validate(full, 5, 512), Ok(TransferOutcome::Passed { transferred: 512 }));
}

#[test]
fn a_read_command_round_trips_through_its_status_wrapper() {
    let (cdb, cdb_len) = scsi::read10(0x00A0_B0C0, 8);
    let data_len = 8 * 512;
    let cbw =
        CommandBlockWrapper { tag: 0xABCD, data_len, flags: CBW_FLAG_IN, lun: 0, cdb_len, cdb };
    let mut wire = [0u8; CBW_LEN];
    cbw.write(&mut wire);
    let echoed = u32::from_le_bytes([wire[4], wire[5], wire[6], wire[7]]);
    let csw = bot::parse(&make_csw(echoed, 0, 0)).unwrap();
    assert_eq!(
        validate(csw, 0xABCD, data_len),
        Ok(TransferOutcome::Passed { transferred: data_len })
    );
}

// The remaining command blocks, byte-for-byte against SPC.

#[test]
fn test_unit_ready_is_an_all_zero_six_byte_cdb() {
    assert_eq!(scsi::test_unit_ready(), ([0u8; 16], 6));
}

#[test]
fn request_sense_carries_its_allocation_length() {
    let (cdb, len) = scsi::request_sense(18);
    assert_eq!(len, 6);
    assert_eq!(cdb[0], 0x03);
    assert_eq!(cdb[4], 18);
}

// Device-controlled response decoders.

#[test]
fn inquiry_data_decodes_the_device_identity() {
    let mut raw = [0u8; 36];
    raw[0] = 0x00; // direct-access block device
    raw[1] = 0x80; // removable medium
    raw[2] = 0x06; // SPC-4
    raw[8..16].copy_from_slice(b"NONOS   ");
    raw[16..32].copy_from_slice(b"MASS STORAGE    ");
    raw[32..36].copy_from_slice(b"0001");
    let d = scsi::parse_inquiry(&raw).unwrap();
    assert_eq!(d.peripheral_type, 0x00);
    assert!(d.removable);
    assert_eq!(d.version, 0x06);
    assert_eq!(&d.vendor, b"NONOS   ");
    assert_eq!(&d.product, b"MASS STORAGE    ");
    assert_eq!(&d.revision, b"0001");
    raw[1] = 0x00;
    assert!(!scsi::parse_inquiry(&raw).unwrap().removable);
    assert!(scsi::parse_inquiry(&raw[..20]).is_none());
}

#[test]
fn read_capacity_decodes_a_4k_geometry() {
    let mut raw = [0u8; 8];
    raw[0..4].copy_from_slice(&0x0003_A97Fu32.to_be_bytes()); // last LBA, big-endian
    raw[4..8].copy_from_slice(&4096u32.to_be_bytes()); // 4Kn block length
    let cap = scsi::parse_capacity(&raw).unwrap();
    assert_eq!(cap.last_lba, 0x0003_A97F);
    assert_eq!(cap.block_len, 4096);
    assert_eq!(cap.block_count(), 0x0003_A97F + 1);
    assert_eq!(cap.capacity_bytes(), (0x0003_A97Fu64 + 1) * 4096);
    assert!(scsi::parse_capacity(&raw[..4]).is_none());
}

#[test]
fn sense_data_decodes_fixed_format_and_refuses_the_rest() {
    let mut raw = [0u8; 18];
    raw[0] = 0x70; // current, fixed format
    raw[2] = 0x06; // UNIT ATTENTION
    raw[12] = 0x28; // NOT READY TO READY CHANGE
    raw[13] = 0x00;
    let s = scsi::parse_sense(&raw).unwrap();
    assert_eq!((s.sense_key, s.asc, s.ascq), (0x06, 0x28, 0x00));
    let mut desc = raw;
    desc[0] = 0x72; // descriptor-format sense is not decoded here
    assert!(scsi::parse_sense(&desc).is_none());
    assert!(scsi::parse_sense(&raw[..10]).is_none());
}

#[test]
fn response_decoders_never_panic_on_hostile_bytes() {
    fn xs(s: &mut u64) -> u64 {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    }
    for seed in 1..200_000u64 {
        let mut s = seed;
        let len = (xs(&mut s) % 40) as usize;
        let raw: Vec<u8> = (0..len).map(|_| (xs(&mut s) & 0xff) as u8).collect();
        let _ = scsi::parse_inquiry(&raw);
        let _ = scsi::parse_capacity(&raw);
        if let Some(sense) = scsi::parse_sense(&raw) {
            assert!(sense.sense_key <= 0x0f);
        }
    }
}
