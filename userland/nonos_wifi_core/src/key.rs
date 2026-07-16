// NONOS Operating System (AGPL-3.0-or-later)
//! Key installation: the one place chips diverge on CCMP. Software chips stash
//! the key and encrypt per frame; hardware chips (Realtek's security CAM, and
//! Intel firmware's key table) program a slot and let the radio encrypt. The
//! shared supplicant installs keys through this trait after deriving them and
//! never names a hardware slot; the implementation chooses one.

pub trait KeyStore {
    /// Install the pairwise temporal key for `peer`. `key_id` is the 802.11 key
    /// index; the implementation picks the hardware slot. False if it could not
    /// be installed.
    fn install_ptk(&mut self, key: &[u8; 16], key_id: u8, peer: &[u8; 6]) -> bool;
    /// Install the group temporal key at `key_id`.
    fn install_gtk(&mut self, key: &[u8; 16], key_id: u8) -> bool;
    /// Remove a key: a pairwise key names its peer, a group key passes None.
    fn remove_key(&mut self, key_id: u8, peer: Option<&[u8; 6]>);
}
