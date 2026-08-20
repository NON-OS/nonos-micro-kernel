// NONOS Operating System (AGPL-3.0-or-later)
//! Polynomials over the field: evaluation in coefficient form, Lagrange
//! evaluation of the low-degree extension, and the number-theoretic transform
//! with the coset low-degree extension built on it.

mod barycentric;
mod eval;
mod lagrange;
mod lde;
mod ntt;

pub use barycentric::{eval_subgroup_ext, eval_subgroup_or_lagrange_ext};
pub use eval::{eval, eval_ext};
pub use lagrange::{eval_lagrange, eval_lagrange_ext};
pub use lde::lde;
pub use ntt::{intt, ntt};
