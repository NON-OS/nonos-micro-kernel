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

use crate::crypto::kernel_keys;
use crate::crypto::util::rng;

pub fn init_crypto_subsystem() -> Result<(), &'static str> {
    if rng::init_rng().is_err() {
        return Err("crypto: init_rng failed, entropy unavailable");
    }
    kernel_keys::init();
    Ok(())
}

pub fn init() {
    if rng::init_rng().is_err() {
        crate::sys::serial::println(b"[FATAL] crypto: init_rng failed, entropy unavailable");
        crate::arch::halt_loop();
    }
    kernel_keys::init();
}

pub fn feature_summary() -> &'static str {
    #[cfg(feature = "mlkem512")]
    return "kyber=512";
    #[cfg(all(feature = "mlkem768", not(feature = "mlkem512")))]
    return "kyber=768";
    #[cfg(all(feature = "mlkem1024", not(any(feature = "mlkem512", feature = "mlkem768"))))]
    return "kyber=1024";
    #[cfg(not(any(feature = "mlkem512", feature = "mlkem768", feature = "mlkem1024")))]
    "kyber=off"
}
