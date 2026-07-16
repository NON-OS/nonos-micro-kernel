// NONOS Operating System (AGPL-3.0-or-later)
//! The plaintext 802.11 data frame both drivers exchange with the shared code.
//! `LinkPort::send_tx` receives one (from encap, before any CCMP) and
//! `LinkPort::poll_rx` returns one (after the chip or software has stripped
//! CCMP), so the shared decap is byte-identical for both chips.
//!
//! Layout: the 802.11 MAC header, then the LLC/SNAP header
//! (AA AA 03 00 00 00, then the two-byte ethertype), then the payload. No CCMP
//! header, no MIC, no FCS. This is what `dot11::build_data` produces and
//! `dot11::decap` consumes.

/// The RFC 1042 LLC/SNAP header that precedes an Ethernet ethertype inside an
/// 802.11 data frame.
pub const LLC_SNAP: [u8; 6] = [0xAA, 0xAA, 0x03, 0x00, 0x00, 0x00];

/// Offset of the LLC/SNAP header from the start of a plaintext data frame's
/// payload region (i.e. right after the MAC header).
pub const ETHERTYPE_OFFSET_IN_SNAP: usize = 6;
