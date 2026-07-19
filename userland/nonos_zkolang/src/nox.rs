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

//! Pay-to-prove metering in NOX.
//!
//! A proof is a service: a buyer wants a computation proven, a prover spends real
//! work to produce the STARK, and the network wants to capture value from the
//! traffic it carries. This module turns a proving run into a price. The price is
//! deterministic in the size of the work, so both sides can agree on it before a
//! prover is engaged, and it is denominated in NOX so the token is the settlement
//! rail for the whole market.
//!
//! The model, in one line: a small anti-spam floor plus a rate on the trace area,
//! split so most of the fee pays the prover and a protocol cut accrues to the NOX
//! treasury. The trace area, rows times width, is the honest cost driver: the
//! prover's field arithmetic, commitments, and low-degree test all scale with it,
//! so charging on it aligns price with work and cannot be gamed by padding, which
//! only raises the bill.
//!
//! Every rate here is a governance-tunable constant, not a fixed law. They are set
//! to sensible starting values; the shape of the model, floor plus area rate with
//! a basis-point protocol cut, is what matters.

use crate::driver::Report;

/// microNOX per whole NOX. Fees are computed in microNOX so small proofs still
/// carry a meaningful, non-zero price.
pub const MICRONOX_PER_NOX: u64 = 1_000_000;

// A flat floor on every proof, so submitting work is never free and the market is
// not a spam vector. 0.001 NOX.
const BASE_MICRONOX: u64 = 1_000;

// The rate on proving work: microNOX per thousand trace cells. 0.00005 NOX per
// thousand cells. A cell is one field element of the committed trace.
const MICRONOX_PER_KCELL: u64 = 50;

// The protocol's cut of each fee, in basis points. Accrues to the NOX treasury as
// the network's revenue on proving traffic. 5%.
const PROTOCOL_FEE_BPS: u64 = 500;

/// A priced quote for proving one run: what it costs and how the fee splits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Quote {
    /// The proving work, trace rows times width, the fee's cost driver.
    pub cells: u64,
    /// The flat floor component.
    pub base_micronox: u64,
    /// The work-proportional component.
    pub compute_micronox: u64,
    /// The full price the buyer pays, base plus compute.
    pub total_micronox: u64,
    /// The protocol cut, the network's revenue on this proof.
    pub protocol_fee_micronox: u64,
    /// What the prover is paid, the remainder after the protocol cut.
    pub prover_micronox: u64,
}

/// Price a proving run from its report. The report's trace shape is the whole
/// input: price follows work, so the quote is agreed before a prover commits.
pub fn quote(report: &Report) -> Quote {
    let cells = report.trace_len as u64 * report.trace_width as u64;
    let compute_micronox = cells.saturating_mul(MICRONOX_PER_KCELL) / 1_000;
    let total_micronox = BASE_MICRONOX.saturating_add(compute_micronox);
    let protocol_fee_micronox = total_micronox.saturating_mul(PROTOCOL_FEE_BPS) / 10_000;
    let prover_micronox = total_micronox - protocol_fee_micronox;
    Quote {
        cells,
        base_micronox: BASE_MICRONOX,
        compute_micronox,
        total_micronox,
        protocol_fee_micronox,
        prover_micronox,
    }
}
