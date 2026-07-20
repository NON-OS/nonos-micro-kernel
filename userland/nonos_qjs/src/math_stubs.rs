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

//! The libm C ABI QuickJS links against, forwarded to the pure-Rust libm crate.

macro_rules! unary {
    ($($name:ident),* $(,)?) => {
        $(
            #[no_mangle]
            pub extern "C" fn $name(x: f64) -> f64 { libm::$name(x) }
        )*
    };
}

unary!(
    acos, acosh, asin, asinh, atan, atanh, cbrt, ceil, cos, cosh, exp, expm1, fabs, floor, log,
    log10, log1p, log2, round, sin, sinh, sqrt, tan, tanh, trunc
);

#[no_mangle]
pub extern "C" fn atan2(y: f64, x: f64) -> f64 {
    libm::atan2(y, x)
}
#[no_mangle]
pub extern "C" fn pow(x: f64, y: f64) -> f64 {
    libm::pow(x, y)
}
#[no_mangle]
pub extern "C" fn fmod(x: f64, y: f64) -> f64 {
    libm::fmod(x, y)
}
#[no_mangle]
pub extern "C" fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}
#[no_mangle]
pub extern "C" fn copysign(x: f64, y: f64) -> f64 {
    libm::copysign(x, y)
}
#[no_mangle]
pub extern "C" fn scalbn(x: f64, n: i32) -> f64 {
    libm::scalbn(x, n)
}
#[no_mangle]
pub extern "C" fn lrint(x: f64) -> i64 {
    libm::round(x) as i64
}
#[no_mangle]
pub unsafe extern "C" fn frexp(x: f64, e: *mut i32) -> f64 {
    let (m, ee) = libm::frexp(x);
    *e = ee;
    m
}
#[no_mangle]
pub unsafe extern "C" fn modf(x: f64, i: *mut f64) -> f64 {
    let (f, ii) = libm::modf(x);
    *i = ii;
    f
}
