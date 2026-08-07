// NONOS Operating System (AGPL-3.0-or-later)
//! The wallet's wide arithmetic, included verbatim so the proofs run over the
//! shipping files. `mul_div` reads its two halves as siblings exactly as it
//! does in the capsule, so the module shape has to match.

#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/num/div_wide.rs"]
pub mod div_wide;
#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/num/mul_div.rs"]
pub mod mul_div;
#[allow(dead_code)]
#[path = "../../../../capsule_wallet_nonos/src/wallet/num/mul_wide.rs"]
pub mod mul_wide;

pub use mul_div::mul_div;
