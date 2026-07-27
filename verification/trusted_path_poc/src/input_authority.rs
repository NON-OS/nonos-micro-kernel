// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! #401 input authority (src/capabilities/token/types/authority_broker.rs, gated
//! in src/syscall/contract/cap_table/mk.rs). Posting accepts an interrupt owner;
//! draining must not, or any driver could keylog the focused app.

/// The input-relevant rights a capability token may hold.
#[derive(Clone, Copy)]
pub struct Rights {
    pub input_source: bool,
    pub irq: bool,
    pub admin: bool,
}

/// can_input_source: may POST an input event (input source, interrupt, or admin).
pub fn can_post(r: Rights) -> bool {
    r.input_source || r.irq || r.admin
}

/// can_input_consumer: may DRAIN or WAIT on the input ring. No bare interrupt
/// owner, so a driver holding only Irq cannot read keystrokes.
pub fn can_drain(r: Rights) -> bool {
    r.input_source || r.admin
}
