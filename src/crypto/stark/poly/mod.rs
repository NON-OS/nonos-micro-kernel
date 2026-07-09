// NONOS Operating System (AGPL-3.0-or-later)
//! Polynomials over the field: evaluation in coefficient form, Lagrange
//! evaluation of the low-degree extension, and the number-theoretic transform
//! with the coset low-degree extension built on it.

mod eval;
mod lagrange;
mod lde;
mod ntt;

pub use eval::eval;
pub use lagrange::eval_lagrange;
pub use lde::lde;
pub use ntt::{intt, ntt};
