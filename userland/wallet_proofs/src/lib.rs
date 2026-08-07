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

//! Proofs for the wallet's pure on-chain read helpers. The modules below are the
//! real files from `capsule_wallet_nonos`, included verbatim so the harnesses
//! check shipping code rather than a copy. Every property proved here has a
//! matching Lean theorem in `verification/lean/Nonos/Wallet*.lean`.

const WALLET: &str = "../capsule_wallet_nonos/src/wallet";

pub mod wallet;

// The real NOX read helpers. `apr_bps` reads `super::constants`, so `constants`
// is a sibling module under `nox` exactly as in the wallet.
#[allow(dead_code)]
pub mod nox;

#[allow(dead_code)]
#[path = "../../capsule_wallet_nonos/src/wallet/event/hex_digit.rs"]
pub mod hex_digit;

// Keep the doc string referenced so the path note is not dead.
#[allow(dead_code)]
pub fn source_root() -> &'static str {
    WALLET
}

#[cfg(test)]
mod mul_div_tests;
#[cfg(test)]
mod stakeable_tests;
#[cfg(test)]
mod swap_curve_tests;
#[cfg(test)]
mod swap_limits_tests;

#[cfg(kani)]
mod kani_proofs;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod probe_model;
