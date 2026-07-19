// NONOS Operating System (AGPL-3.0-or-later)
//! Regions 6 and 7: the two index-to-point product chains. Region 6 walks the
//! consistency index bits to shift * omega^p, the x the DEEP check divides
//! by; region 7 walks the FRI query index bits to shift * omega^i0, the
//! layer-zero point the fold chain descends from.

use super::auth::AuthSide;
use super::fri::FriSide;
use crate::crypto::stark::air::IndexPoint;
use crate::crypto::stark::field::Fp;
use crate::crypto::stark::fri::root_of_unity;
use alloc::vec::Vec;

pub(crate) struct PointSide {
    pub ip: IndexPoint,
    pub itrace: Vec<Fp>,
    pub pbits: usize,
    pub fp: IndexPoint,
    pub fptrace: Vec<Fp>,
    pub fbits: usize,
}

pub(crate) fn point_regions(fs: &FriSide, au: &AuthSide) -> PointSide {
    let bo = root_of_unity(fs.log_n);
    let shift = Fp::from_u64(7);
    let pbits = au.cons_dirs.len();
    let pidx: usize = au.cons_dirs.iter().enumerate().map(|(k, &b)| (b as usize) << k).sum();
    let ip = IndexPoint::new(bo, shift, pbits, pidx);
    let itrace = ip.trace();
    let fbits = (fs.log_n - 1) as usize;
    let fp = IndexPoint::new(bo, shift, fbits, fs.i0);
    let fptrace = fp.trace();
    PointSide { ip, itrace, pbits, fp, fptrace, fbits }
}
