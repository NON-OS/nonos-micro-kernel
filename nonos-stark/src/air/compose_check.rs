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

//! The out-of-domain composition gadget for the wired join-split: the last piece a
//! recursive verifier needs, proving the composition value `comp_z` the DEEP check
//! consumes was honestly formed from the out-of-domain frame. It arithmetizes the
//! join-split's own `transition_ext` at `z` (the fused conservation and range
//! transitions, selector weighted, plus the copy-constraint grand product), the
//! transition vanishing factor `exempt / (z^t - 1)`, and the boundary quotients,
//! then checks their batched sum equals `comp_z`. Every value is `Fp2` laid out as
//! base column pairs, its multiplication expanded with the `X^2 = 7` cross terms so
//! the single transition serves both the base composition and the extension
//! out-of-domain evaluation. Intermediates (the vanishing power tower, the
//! transition values, the quotients) are witnessed and each pinned by one
//! low-degree relation; the frame, the periodic values at `z`, the coefficients,
//! `z`, and `comp_z` are the public statement.

use super::super::field::{Felt, Fp, Fp2};
use super::spec::{Air, AirExt};
use alloc::vec::Vec;

const W: u64 = 7;
const BETA: u64 = 5;
const GAMMA: u64 = 7;

/// A pinned boundary of the inner AIR: which frame column it reads, the domain
/// point `g^row` it vanishes at, and the value it pins.
pub struct ComposeBoundary {
    pub col: usize,
    pub g_row: Fp,
    pub expected: Fp,
}

pub struct ComposeCheck {
    window: [Fp2; 6],
    periodic: [Fp2; 5],
    coeffs: [Fp2; 8],
    z: Fp2,
    comp_z: Fp2,
    g_tm1: Fp,
    t: u64,
    boundaries: Vec<ComposeBoundary>,
    witness: bool,
}

impl ComposeCheck {
    /// Build the check from the out-of-domain frame, the periodic values at `z`, the
    /// batching coefficients, the point, the claimed composition, the boundary data,
    /// and the trace length. The public statement (frame, periodic values, point,
    /// coefficients, composition) is pinned as boundaries: instance-specific.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        window: [Fp2; 6],
        periodic: [Fp2; 5],
        coeffs: [Fp2; 8],
        z: Fp2,
        comp_z: Fp2,
        g_tm1: Fp,
        t: u64,
        boundaries: Vec<ComposeBoundary>,
    ) -> ComposeCheck {
        ComposeCheck { window, periodic, coeffs, z, comp_z, g_tm1, t, boundaries, witness: false }
    }

    /// The production form: the frame, periodic values, point, coefficients, and
    /// composition ride the trace (they already do) but are not pinned, so the AIR is
    /// instance-independent and those cells are bound by the assembly grand product
    /// to their sources (the frame to the DEEP claims and the transcript, the point
    /// and coefficients to the transcript, the composition to the DEEP check).
    #[allow(clippy::too_many_arguments)]
    pub fn new_witness(
        window: [Fp2; 6],
        periodic: [Fp2; 5],
        coeffs: [Fp2; 8],
        z: Fp2,
        comp_z: Fp2,
        g_tm1: Fp,
        t: u64,
        boundaries: Vec<ComposeBoundary>,
    ) -> ComposeCheck {
        ComposeCheck { window, periodic, coeffs, z, comp_z, g_tm1, t, boundaries, witness: true }
    }

    fn out_values(&self) -> (Fp2, Fp2, Fp2, Fp2, Fp2) {
        let (w0, w1, w2, w3, w5) =
            (self.window[0], self.window[1], self.window[2], self.window[3], self.window[5]);
        let (sel0, sel1, id, sig, gp_sel) = (
            self.periodic[0],
            self.periodic[1],
            self.periodic[2],
            self.periodic[3],
            self.periodic[4],
        );
        let beta = Fp2::from_base(Fp::from_u64(BETA));
        let gamma = Fp2::from_base(Fp::from_u64(GAMMA));
        let two = Fp2::from_base(Fp::from_u64(2));
        let out0 = sel0 * (w3 - w0 - w1) + sel1 * (w0 - two * w3 - w1);
        let out1 = sel1 * (w1 * (w1 - Fp2::ONE));
        let num = w0 + beta * id + gamma;
        let den = w0 + beta * sig + gamma;
        let out2 = gp_sel * (w5 * den - w2 * num) + (Fp2::ONE - gp_sel) * (w5 - w2);
        (out0, out1, out2, num, den)
    }

    /// The witness: row 0 holds the public frame and the witnessed intermediates
    /// (the vanishing power tower, its inverse, the transition values and the grand
    /// product factors, the boundary quotients); row 1 is inert padding.
    pub fn trace(&self) -> Vec<Fp> {
        let width = self.trace_width();
        let mut tr = alloc::vec![Fp::ZERO; 2 * width];
        let mut put = |slot: usize, v: Fp2| {
            tr[2 * slot] = v.c0;
            tr[2 * slot + 1] = v.c1;
        };
        for (i, v) in self.window.iter().enumerate() {
            put(i, *v);
        }
        for (i, v) in self.periodic.iter().enumerate() {
            put(6 + i, *v);
        }
        put(11, self.z);
        for (i, v) in self.coeffs.iter().enumerate() {
            put(12 + i, *v);
        }

        let z = self.z;
        let z_h_inv = (z.pow(self.t) - Fp2::ONE).inv();
        put(20, z_h_inv);
        let exempt = z - Fp2::from_base(self.g_tm1);
        put(21, exempt * z_h_inv);

        let (out0, out1, out2, num, den) = self.out_values();
        put(22, out0);
        put(23, out1);
        put(24, out2);
        put(25, num);
        put(26, den);
        put(27, self.comp_z);

        for (j, b) in self.boundaries.iter().enumerate() {
            let q = (self.window[b.col] - Fp2::from_base(b.expected))
                * (z - Fp2::from_base(b.g_row)).inv();
            put(28 + j, q);
        }

        let mut zp = z;
        for k in 0..6 {
            zp = zp.square();
            put(33 + k, zp);
        }
        tr
    }

    fn transition_impl<F: Felt>(&self, window: &[F], _periodic: &[F]) -> Vec<F> {
        let w = F::from_base(Fp::from_u64(W));
        let rd = |slot: usize| -> (F, F) { (window[2 * slot], window[2 * slot + 1]) };
        let mul =
            |a: (F, F), b: (F, F)| -> (F, F) { (a.0 * b.0 + w * a.1 * b.1, a.0 * b.1 + a.1 * b.0) };
        let sub = |a: (F, F), b: (F, F)| -> (F, F) { (a.0 - b.0, a.1 - b.1) };
        let add = |a: (F, F), b: (F, F)| -> (F, F) { (a.0 + b.0, a.1 + b.1) };
        let base = |v: u64| -> (F, F) { (F::from_base(Fp::from_u64(v)), F::ZERO) };

        let mut res: Vec<(F, F)> = Vec::new();

        // The vanishing power tower: zpow[k] = z^(2^(k+1)).
        let z = rd(11);
        let mut prev = z;
        for k in 0..6 {
            let zp = rd(33 + k);
            res.push(sub(zp, mul(prev, prev)));
            prev = zp;
        }
        // z_h_inv * (z^t - 1) = 1.
        let z_h_inv = rd(20);
        let zt = rd(38);
        res.push(sub(mul(z_h_inv, sub(zt, base(1))), base(1)));
        // E = (z - g^(t-1)) * z_h_inv.
        let e = rd(21);
        let exempt = sub(z, (F::from_base(self.g_tm1), F::ZERO));
        res.push(sub(e, mul(exempt, z_h_inv)));

        // The fused transition values.
        let (w0, w1, w2, w3, w5) = (rd(0), rd(1), rd(2), rd(3), rd(5));
        let (sel0, sel1, id, sig, gp_sel) = (rd(6), rd(7), rd(8), rd(9), rd(10));
        let two = base(2);
        let acc_t = sub(w3, add(w0, w1));
        let rng_t = sub(sub(w0, mul(two, w3)), w1);
        let out0 = rd(22);
        res.push(sub(out0, add(mul(sel0, acc_t), mul(sel1, rng_t))));
        let out1 = rd(23);
        res.push(sub(out1, mul(sel1, mul(w1, sub(w1, base(1))))));
        // Grand product factors.
        let bt = base(BETA);
        let gm = base(GAMMA);
        let num = rd(25);
        let den = rd(26);
        res.push(sub(num, add(add(w0, mul(bt, id)), gm)));
        res.push(sub(den, add(add(w0, mul(bt, sig)), gm)));
        let out2 = rd(24);
        let prod = sub(mul(w5, den), mul(w2, num));
        let carry = sub(w5, w2);
        let gp = add(mul(gp_sel, prod), mul(sub(base(1), gp_sel), carry));
        res.push(sub(out2, gp));

        // Boundary quotients: q * (z - g^row) = window[col] - expected.
        for (j, b) in self.boundaries.iter().enumerate() {
            let q = rd(28 + j);
            let denom = sub(z, (F::from_base(b.g_row), F::ZERO));
            let numer = sub(rd(b.col), (F::from_base(b.expected), F::ZERO));
            res.push(sub(mul(q, denom), numer));
        }

        // comp_z = sum of coeff_i * out_i * E over the transitions, plus the
        // batched boundary quotients.
        let (c0, c1, c2) = (rd(12), rd(13), rd(14));
        let mut acc = mul(mul(c0, out0), e);
        acc = add(acc, mul(mul(c1, out1), e));
        acc = add(acc, mul(mul(c2, out2), e));
        for (j, _) in self.boundaries.iter().enumerate() {
            acc = add(acc, mul(rd(15 + j), rd(28 + j)));
        }
        res.push(sub(rd(27), acc));

        let mut out = Vec::with_capacity(res.len() * 2);
        for (a, b) in res {
            out.push(a);
            out.push(b);
        }
        out
    }
}

impl AirExt for ComposeCheck {
    fn transition_ext(&self, window: &[Fp2], periodic: &[Fp2]) -> Vec<Fp2> {
        self.transition_impl(window, periodic)
    }
}

impl Air for ComposeCheck {
    fn log_trace_len(&self) -> u32 {
        1
    }

    fn trace_width(&self) -> usize {
        // 6 frame + 5 periodic + z + 8 coeffs + z_h_inv + E + out0..2 + num + den +
        // comp_z + 5 quotients + 6 power tower = 39 extension values.
        78
    }

    fn window_size(&self) -> usize {
        2
    }

    fn constraint_degree(&self) -> usize {
        3
    }

    fn num_transition(&self) -> usize {
        // (6 tower + 1 inverse + 1 exempt + 2 out + 2 gp factors + 1 out2 + 5
        // boundary + 1 comp_z) extension constraints, two base residuals each.
        2 * (6 + 1 + 1 + 2 + 2 + 1 + 5 + 1)
    }

    fn transition(&self, window: &[Fp], periodic: &[Fp]) -> Vec<Fp> {
        self.transition_impl(window, periodic)
    }

    fn boundary(&self) -> Vec<(usize, usize, Fp)> {
        // Production form: nothing pinned. The frame, periodic values, point,
        // coefficients, and composition are witness, bound by the assembly.
        if self.witness {
            return Vec::new();
        }
        // Pin the public statement: the frame, the periodic values at z, the point,
        // the coefficients, and the claimed composition value.
        let mut b = Vec::new();
        let mut pin = |slot: usize, v: Fp2| {
            b.push((2 * slot, 0, v.c0));
            b.push((2 * slot + 1, 0, v.c1));
        };
        for (i, v) in self.window.iter().enumerate() {
            pin(i, *v);
        }
        for (i, v) in self.periodic.iter().enumerate() {
            pin(6 + i, *v);
        }
        pin(11, self.z);
        for (i, v) in self.coeffs.iter().enumerate() {
            pin(12 + i, *v);
        }
        pin(27, self.comp_z);
        b
    }
}
