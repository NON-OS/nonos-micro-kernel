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

//! The envelope every service provider request and response travels in.
//!
//! An exit reads the interface envelope before it reads anything about
//! SOCKS5, and only then hands the rest to its own protocol. Sending the
//! SOCKS5 bytes bare is not a smaller version of the same message: the exit
//! reads the version byte as the envelope's, the request flag as the
//! envelope's tag, and a CONNECT arrives looking like a control request it
//! cannot parse. It is dropped without an answer, which is indistinguishable
//! from a request that never arrived.

/// Envelope version this speaks. Anything below three means an exit that
/// predates the envelope, and it reads such a message as bare provider data.
pub const INTERFACE_VERSION: u8 = 3;

/// The envelope carries a request for the provider itself.
pub const TAG_PROVIDER_DATA: u8 = 1;

/// Bytes the envelope adds in front of the provider's own message.
pub const ENVELOPE_BYTES: usize = 2;

/// Write the envelope, returning where the provider's message starts.
pub fn open_envelope(out: &mut [u8]) -> Option<usize> {
    if out.len() < ENVELOPE_BYTES {
        return None;
    }
    out[0] = INTERFACE_VERSION;
    out[1] = TAG_PROVIDER_DATA;
    Some(ENVELOPE_BYTES)
}

/// Strip the envelope from a response, or `None` if it is not one we speak.
///
/// An exit answers in the envelope it was addressed in, so a response without
/// one came from somewhere other than the request we sent.
pub fn inner_response(buf: &[u8]) -> Option<&[u8]> {
    if buf.len() < ENVELOPE_BYTES || buf[0] != INTERFACE_VERSION {
        return None;
    }
    if buf[1] != TAG_PROVIDER_DATA {
        return None;
    }
    Some(&buf[ENVELOPE_BYTES..])
}
