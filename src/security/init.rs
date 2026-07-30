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

use crate::security::{boot, crypto, hardening, module_db, monitoring, network, policy, quantum};

/* DEV NOTES eK@nonos.systems
   Complete security subsystem initialization sequence. This must be called during kernel boot
   after drivers are initialized but before scheduler starts. The initialization order matters:
   1. CPU mitigations (spectre) - must be first to protect subsequent code
   2. Memory sanitization - protects heap/stack operations
   3. Cryptographic primitives - needed by key management and storage
   4. Key management and secure boot - establishes trust chain
   5. Capability and policy systems - enables access control
   6. Audit and monitoring - tracks security events
   7. Network security - DNS privacy, ZK-IDS
   8. Quantum crypto engine - post-quantum readiness
*/
pub fn init_all_security() -> Result<(), &'static str> {
    hardening::speculation::init()?;
    hardening::memory_sanitization::init()?;
    crypto::constant_time::init()?;
    crypto::random::init()?;
    crypto::key_management::init().map_err(|_| "Key management init failed")?;
    crate::storage::crypto_storage::init().map_err(|_| "Crypto storage init failed")?;
    boot::secure_boot::init().map_err(|_| "Secure boot init failed")?;
    policy::capability::init_capabilities()?;
    policy::advanced::init_advanced_security()?;
    monitoring::audit::init()?;
    crate::monitor::nonos_monitor::init_security_monitor();
    boot::firmware::init()?;
    module_db::init()?;
    monitoring::monitor::set_enabled(true);
    monitoring::rootkit::init()?;
    crypto::trusted_hashes::init()?;
    crypto::trusted_keys::init()?;
    monitoring::leak_detection::add_sensitive_pattern("password");
    monitoring::leak_detection::add_sensitive_pattern("private_key");
    monitoring::leak_detection::add_sensitive_pattern("ssn");
    monitoring::leak_detection::add_sensitive_pattern("api_key");
    monitoring::leak_detection::add_sensitive_pattern("secret");
    monitoring::leak_detection::add_sensitive_pattern("token");
    network::dns_privacy::scan_dns_queries();
    network::zkids::init_zkids()?;
    let _ = quantum::pqc::QuantumSecurityEngine::new();
    policy::session::init()?;
    Ok(())
}
