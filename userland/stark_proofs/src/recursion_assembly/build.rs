// NONOS Operating System (AGPL-3.0-or-later)
//! Assembling the wired recursive verifier: prove the inner join-split, build
//! the nine regions from it, lay them out, bind them, and place the witness.
//! The wired AIR is always the honest one; a tamper only alters the witness
//! it must reject.

use super::layout::{offsets, Layout};
use super::tamper::Tamper;
use super::{auth, compose, deep, fri, groups, inner, periodic, points, transcript};
use crate::crypto::stark::air::{AirExt, GpGroup, WiredMultiExt};
use crate::crypto::stark::field::Fp;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub(crate) struct Assembly {
    pub wired: WiredMultiExt,
    pub witness: Vec<Fp>,
    pub lay: Layout,
    pub publics: Vec<Fp>,
    pub n_groups: usize,
    /// Each region's first row in the stacked trace, in region order.
    pub region_offsets: Vec<usize>,
}

pub(crate) fn assemble(tamper: Tamper) -> Assembly {
    let h = inner::hasher();
    let inner = inner::join_split(&h);

    let (cregion, ctrace) = compose::compose_region(&inner);
    let (dregion, dtrace, n_terms) = deep::deep_region(&h, &inner, tamper);
    let ts = transcript::stark_transcript(&h, &inner, n_terms);
    let fs = fri::fri_side(&h, &inner);
    let au = auth::auth_side(&h, &inner, fs.i0, tamper);
    let pts = points::point_regions(&fs, &au);
    let (pzregion, pztrace) = periodic::periodic_region(&inner);

    let width_inner = au.ocells.len() - 3;
    let t_inner = inner.t as usize;
    let lay_parts = (
        ts.z_op,
        ts.deep_coeff_op,
        inner.publics.len(),
        inner.proof.trace_roots.len(),
        inner.ci.coeffs.len() * 2,
    );

    let regions: Vec<Box<dyn AirExt>> = alloc::vec![
        Box::new(ts.region) as Box<dyn AirExt>,
        Box::new(cregion),
        Box::new(dregion),
        Box::new(fs.transcript),
        Box::new(fs.fold),
        Box::new(au.region),
        Box::new(pts.ip),
        Box::new(pts.fp),
        Box::new(pzregion),
    ];
    let (off, span) = offsets(&regions);

    let lay = Layout {
        span,
        l: 4,
        c_off: off[1],
        d_off: off[2],
        ft_off: off[3],
        f_off: off[4],
        m_off: off[5],
        i_off: off[6],
        fp_off: off[7],
        pz_off: off[8],
        z_op: lay_parts.0,
        deep_coeff_op: lay_parts.1,
        pub_len: lay_parts.2,
        ntr: lay_parts.3,
        ncoeff2: lay_parts.4,
        n_terms,
        width_inner,
        window_inner: (n_terms - 1) / width_inner,
        ocells: au.ocells,
        depth: au.depth,
        n_open: au.n_open,
        n_folds: fs.n_folds,
        log_n: fs.log_n,
        pbits: pts.pbits,
        fbits: pts.fbits,
        t_inner,
        n_pz: 5,
        i0: fs.i0,
    };

    let mut gps: Vec<GpGroup> = Vec::new();
    groups::statement(&lay, &mut gps);
    groups::deep(&lay, &mut gps);
    groups::roots(&lay, &mut gps);
    groups::fold(&lay, &mut gps);
    groups::index(&lay, &mut gps);
    groups::periodic(&lay, &mut gps);

    let n_groups = gps.len();
    let wired = WiredMultiExt::new(regions, gps);
    let witness = wired.trace(&[
        ts.trace,
        ctrace,
        dtrace,
        fs.ttrace,
        fs.ftrace,
        au.trace,
        pts.itrace,
        pts.fptrace,
        pztrace,
    ]);
    Assembly { wired, witness, lay, publics: inner.publics, n_groups, region_offsets: off }
}
