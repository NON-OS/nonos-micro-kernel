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
//! A TLS 1.3 client.
//!
//! Lifted out of the browser, which had one copy, while the wallet had
//! another that had already drifted from it in twenty one files. A
//! certificate check that only half the system gets is worse than no shared
//! code at all, so there is one of these now.
//!
//! Bulk crypto goes to the pool service rather than being implemented here.

#![no_std]

extern crate alloc;

mod aad_frame;
mod aes_gcm;
mod app_keys;
mod application_plaintext;
mod application_request;
mod application_write;
mod cert_at;
mod cert_count;
mod cert_dns_match;
mod cert_issuer;
mod cert_sig_alg;
mod cert_signature;
mod cert_spki;
mod cert_tbs;
mod cert_time_value;
mod cert_valid_now;
mod cert_verify_msg;
mod chain_walk;
mod client_finished;
mod client_flight;
mod client_hello;
mod constants;
mod crypto_port;
mod crypto_status;
mod der_tlv;
mod ecdsa_sig_raw;
mod expand_label;
mod ext_groups;
mod ext_keyshare;
mod ext_sigalgs;
mod ext_sni;
mod ext_versions;
mod finished_key;
mod finished_value;
mod finished_verify;
pub mod flight;
mod hash_sha256;
mod hash_sha384;
mod hkdf;
mod inner_plain;
mod nonce;
mod push;
mod read;
mod record;
mod record_open;
mod record_seal;
mod rtc_now;
mod roots;
mod scan_server_finished;
mod schedule;
mod server_complete;
mod server_context;
mod server_finished_flight_ready;
mod server_hello;
mod server_keys;
mod session;
mod spki_point;
mod traffic_keys;
mod verify_link;
mod verify_p256;
mod verify_p384;
mod verify_rsa;

pub use application_plaintext::{application_plaintext, application_plaintext_cached};
pub use application_request::application_request;
pub use application_write::application_write;
pub use client_flight::client_flight;
pub use server_complete::server_complete;
pub use server_finished_flight_ready::server_finished_flight_ready;
pub use rtc_now::rtc_now;
pub use session::{exchange, Io, SessionError};
pub use traffic_keys::TrafficKeys;
