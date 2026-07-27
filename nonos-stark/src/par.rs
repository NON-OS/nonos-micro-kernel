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

//! One dispatch point for the prover's data-parallel maps. With the `parallel`
//! feature the maps run across every core; without it they are the ordinary
//! serial iterators the kernel builds. Both forms are indexed and
//! order-preserving, so the output is identical either way — the callers pass
//! pure functions of the index or element, and a bit-exact gate proves the two
//! prover forms emit the same proof bytes.

use alloc::vec::Vec;

/// Map a pure function over `0..n`, collecting in index order.
#[cfg(feature = "parallel")]
pub(crate) fn map_index<T, F>(n: usize, f: F) -> Vec<T>
where
    T: Send,
    F: Fn(usize) -> T + Send + Sync,
{
    use rayon::prelude::*;
    (0..n).into_par_iter().map(f).collect()
}

#[cfg(not(feature = "parallel"))]
pub(crate) fn map_index<T, F>(n: usize, f: F) -> Vec<T>
where
    F: Fn(usize) -> T,
{
    (0..n).map(f).collect()
}

/// Map a pure function over a slice, collecting in element order.
#[cfg(feature = "parallel")]
pub(crate) fn map_slice<A, T, F>(items: &[A], f: F) -> Vec<T>
where
    A: Sync,
    T: Send,
    F: Fn(&A) -> T + Send + Sync,
{
    use rayon::prelude::*;
    items.par_iter().map(f).collect()
}

#[cfg(not(feature = "parallel"))]
pub(crate) fn map_slice<A, T, F>(items: &[A], f: F) -> Vec<T>
where
    F: Fn(&A) -> T,
{
    items.iter().map(f).collect()
}
