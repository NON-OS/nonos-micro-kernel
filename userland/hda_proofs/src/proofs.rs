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

//! The proofs, one module per property under test.
//!
//! They are gathered under a module rather than listed at the crate root only
//! so that no file in this crate runs past the line budget the tree holds
//! itself to. Each module below states, in its own header, which part of the
//! HD Audio specification it is holding the driver to and what goes wrong on a
//! real machine when the driver stops holding to it.

mod at_start;
mod bdl_tests;
mod codec_refusal_tests;
mod codec_tests;
mod corb_start_tests;
mod corb_tests;
mod graph_tests;
mod layout_tests;
mod order_corb_tests;
mod order_tests;
mod reset_tests;
mod setup_order_tests;
mod setup_tests;
mod stream_dma_tests;
mod stream_irq_tests;
mod stream_tests;
mod verb_payload_tests;
mod verb_ring_tests;
mod verb_tests;
mod verb_timeout_tests;
mod verb_word_tests;
