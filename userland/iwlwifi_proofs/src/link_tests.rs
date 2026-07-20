// NONOS Operating System (AGPL-3.0-or-later)
//! Integration proof for the iwlwifi data path: the shared `LinkStation` framing
//! engine composed with the gen3 transmit command and RX-MPDU extraction. The
//! unit proofs check each piece; this checks they fit together, so an ethernet
//! frame becomes a firmware TX command on transmit and a received notification
//! becomes an ethernet frame on receive. Intel does CCMP in firmware, so the
//! station frames plaintext (`Ccmp::Hardware`) and the radio encrypts. The only
//! part not exercised here is the DMA ring itself, which is the on-silicon step.

use crate::gen3::extract_mpdu;
use crate::gen3::tx_cmd;
use crate::station::{Ccmp, LinkStation};

const OUR: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const BSSID: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const RATE_N_FLAGS: u32 = 0x0000_0003;
// An example RX descriptor size; the running firmware reports the real one.
const RX_DESC_LEN: usize = 32;
const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];
const FC_PROTECTED: u16 = 0x4000;

fn ethernet() -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x33, 0x33, 0x00, 0x00, 0x00, 0x16]); // destination
    f.extend_from_slice(&OUR); // source
    f.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    f.extend_from_slice(b"the ip stack rides the radio");
    f
}

#[test]
fn transmit_frames_a_plaintext_mpdu_into_a_gen3_command() {
    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    let eth = ethernet();
    let mpdu = sta.tx_frame(&eth).expect("frame the ethernet payload");

    // The radio encrypts, so the frame handed to the TX command is plaintext.
    let fc = u16::from_le_bytes([mpdu[0], mpdu[1]]);
    assert_eq!(fc & FC_PROTECTED, 0, "hardware CCMP: the framed MPDU is not encrypted");

    let mut cmd = [0u8; tx_cmd::TX_CMD_GEN3_HDR + 1600];
    let n = tx_cmd::build(&mpdu, RATE_N_FLAGS, &mut cmd).expect("wrap in a gen3 TX command");
    assert_eq!(n, tx_cmd::TX_CMD_GEN3_HDR + mpdu.len(), "command is the header plus the frame");
    assert_eq!(&cmd[0..2], &(mpdu.len() as u16).to_le_bytes(), "length field is the frame length");
    assert_eq!(&cmd[tx_cmd::TX_CMD_GEN3_HDR..n], &mpdu[..], "the frame follows the 28-byte header");
}

#[test]
fn receive_extracts_the_mpdu_and_recovers_the_ethernet_frame() {
    // A hardware-CCMP chip delivers an already-decrypted MPDU after a descriptor.
    // Build the FromDS data frame the AP sends the station.
    let src: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x09];
    let payload = b"a reply from the access point";
    let fc: u16 = 0x0208; // data frame, FromDS
    let mut mpdu = Vec::new();
    mpdu.extend_from_slice(&fc.to_le_bytes());
    mpdu.extend_from_slice(&[0, 0]); // duration
    mpdu.extend_from_slice(&OUR); // addr1 = receiver (station)
    mpdu.extend_from_slice(&BSSID); // addr2 = transmitter (AP)
    mpdu.extend_from_slice(&src); // addr3 = source
    mpdu.extend_from_slice(&[0, 0]); // sequence control
    mpdu.extend_from_slice(&LLC_SNAP);
    mpdu.extend_from_slice(&[0x08, 0x00]); // ethertype IPv4
    mpdu.extend_from_slice(payload);

    // The firmware posts the frame after a descriptor whose first field is the
    // MPDU length.
    let mut notif = vec![0u8; RX_DESC_LEN];
    notif[0..2].copy_from_slice(&(mpdu.len() as u16).to_le_bytes());
    notif.extend_from_slice(&mpdu);

    let extracted = extract_mpdu(&notif, RX_DESC_LEN).expect("extract the MPDU after the descriptor");
    assert_eq!(extracted, &mpdu[..], "the MPDU is recovered from the notification");

    let mut sta = LinkStation::new(OUR);
    sta.associate(BSSID, Ccmp::Hardware);
    let eth = sta.rx_frame(extracted).expect("recover the ethernet frame");
    assert_eq!(&eth[0..6], &OUR, "destination is the station (FromDS addr1)");
    assert_eq!(&eth[6..12], &src, "source is addr3");
    assert_eq!(&eth[12..14], &[0x08, 0x00], "ethertype preserved");
    assert_eq!(&eth[14..], payload, "payload preserved end to end");
}
