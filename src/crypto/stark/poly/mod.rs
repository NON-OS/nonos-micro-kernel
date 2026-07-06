// NONOS Operating System (AGPL-3.0-or-later)
//! Polynomials over the field: evaluation in coefficient form and Lagrange
//! evaluation of the low-degree extension.

mod eval;
mod lagrange;

pub use eval::eval;
pub use lagrange::eval_lagrange;
