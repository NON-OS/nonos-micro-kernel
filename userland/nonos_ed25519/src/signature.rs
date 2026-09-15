// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;
use alloc::vec::Vec;
use core::ptr;

use nonos_hash::sha512;

use crate::field::ct_eq_32;
use crate::point::{
    ge_add, ge_has_large_order, ge_p1p1_to_p3, ge_pack, ge_scalarmult_base_ct,
    ge_to_cached, ge_unpack, scalarmult_vartime,
};
use crate::scalar::{clamp_scalar, sc_addmul_mod_l, sc_ge, sc_reduce_mod_l, L};

#[derive(Debug, Clone)]
pub struct KeyPair {
    pub public: [u8; 32],
    pub private: [u8; 32],
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        for b in &mut self.private {
            unsafe { ptr::write_volatile(b, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
}

impl Signature {
    #[inline]
    pub fn to_bytes(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[..32].copy_from_slice(&self.r);
        out[32..].copy_from_slice(&self.s);
        out
    }

    #[inline]
    pub fn from_bytes(b: &[u8; 64]) -> Self {
        let mut r = [0u8; 32];
        let mut s = [0u8; 32];
        r.copy_from_slice(&b[..32]);
        s.copy_from_slice(&b[32..]);
        Self { r, s }
    }
}

impl KeyPair {
    /*
     * `generate` is gone rather than ported. It reached for a global RNG,
     * which is ambient authority a signing library should not hold: a caller
     * that can influence that source can influence the key. The seed comes
     * from the caller now, which is the one that owns an entropy source and
     * knows what the key is for.
     */
    pub fn from_entropy(seed: [u8; 32]) -> Self {
        Self::from_seed(seed)
    }

    pub fn from_seed(seed: [u8; 32]) -> Self {
        let h = sha512(&seed);
        let mut a = [0u8; 32];
        a.copy_from_slice(&h[..32]);
        clamp_scalar(&mut a);
        let a_point = ge_scalarmult_base_ct(&a);
        let public = ge_pack(&a_point);
        Self { public, private: seed }
    }
}

pub fn sign(kp: &KeyPair, msg: &[u8]) -> Signature {
    let h = sha512(&kp.private);
    let mut a = [0u8; 32];
    a.copy_from_slice(&h[..32]);
    clamp_scalar(&mut a);
    let prefix = &h[32..64];

    let mut r_in = Vec::with_capacity(prefix.len() + msg.len());
    r_in.extend_from_slice(prefix);
    r_in.extend_from_slice(msg);
    let mut r64 = sha512(&r_in);
    let r = sc_reduce_mod_l(&mut r64);
    let rpt = ge_scalarmult_base_ct(&r);
    let r_bytes = ge_pack(&rpt);

    let mut kin = Vec::with_capacity(32 + 32 + msg.len());
    kin.extend_from_slice(&r_bytes);
    kin.extend_from_slice(&kp.public);
    kin.extend_from_slice(msg);
    let mut k64 = sha512(&kin);
    let k = sc_reduce_mod_l(&mut k64);

    let s = sc_addmul_mod_l(&r, &k, &a);

    Signature { r: r_bytes, s }
}

pub fn verify(public: &[u8; 32], msg: &[u8], sig: &Signature) -> bool {
    if sc_ge(&sig.s, &L) {
        return false;
    }

    let a_point = match ge_unpack(public) {
        Some(p) => p,
        None => return false,
    };
    let r_point = match ge_unpack(&sig.r) {
        Some(p) => p,
        None => return false,
    };

    if !ge_has_large_order(&a_point) {
        return false;
    }
    if !ge_has_large_order(&r_point) {
        return false;
    }

    let mut kin = Vec::with_capacity(32 + 32 + msg.len());
    kin.extend_from_slice(&sig.r);
    kin.extend_from_slice(public);
    kin.extend_from_slice(msg);
    let mut k64 = sha512(&kin);
    let k = sc_reduce_mod_l(&mut k64);

    let sb = ge_scalarmult_base_ct(&sig.s);
    let k_a = scalarmult_vartime(&a_point, &k);

    let rc = ge_to_cached(&k_a);
    let rp = ge_add(&r_point, &rc);
    let rp3 = ge_p1p1_to_p3(&rp);

    ct_eq_32(&ge_pack(&sb), &ge_pack(&rp3))
}
