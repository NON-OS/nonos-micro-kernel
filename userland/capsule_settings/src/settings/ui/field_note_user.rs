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

use nonos_policy_proto::Field;

pub fn note(field: Field) -> Option<&'static str> {
    Some(match field {
        Field::AnonymousMode => "Route capsule traffic through the anonymity layer.",
        Field::NymEnabled => "Announce this machine on the Nym mixnet.",
        Field::AutoWipe => "Erase RAM-resident state when the machine powers down.",
        Field::ZkAttestation => "Prove capsule integrity without revealing the binary.",
        Field::HardwareCrypto => "Use CPU crypto instructions when the machine offers them.",
        Field::SystemKeysGenerated => "Identity keys were minted during first boot.",
        Field::WifiAutoconnect => "Automatically connect to known Wi-Fi networks.",
        Field::PreferIpv6 => "Use IPv6 when available on supported networks.",
        Field::MeteredConnection => "Hold background transfers on this connection.",
        Field::WifiAskToJoin => "Offer open networks when no known one is in range.",
        Field::AutoLockTimeout => "Minutes of inactivity before the session locks.",
        Field::ScreenTimeout => "Minutes before the display sleeps.",
        Field::DeveloperMode => "Unlock kernel diagnostics and unsigned tooling.",
        Field::AnimationsEnabled => "Animate window and launcher transitions.",
        Field::HighContrast => "Raise contrast across all system chrome.",
        Field::AlertSounds => "Play a tone for system alerts.",
        Field::StartupChime => "Play a tone when the machine finishes booting.",
        _ => return None,
    })
}
