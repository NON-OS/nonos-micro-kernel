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

use crate::settings::schema::rows::{Block, Pill, Tone};
use crate::settings::state::{cached_value, FieldValue, State};

use super::live_net::link_state;
use super::valbuf::ValBuf;

/// The badge a card header shows, resolved against live state. `Radio` reports
/// the stored Wi-Fi power setting; `Net` reports what the DHCP client says.
pub fn block_pill(state: &State, b: &Block) -> Option<(ValBuf, Tone)> {
    match b.pill {
        Pill::None => None,
        Pill::Net => Some(link_state(state)),
        Pill::Radio => Some(radio(state)),
        Pill::Fixed(label, tone) => {
            let mut v = ValBuf::new();
            v.push_str(label);
            Some((v, tone))
        }
    }
}

fn radio(state: &State) -> (ValBuf, Tone) {
    let mut v = ValBuf::new();
    let tone = match cached_value(state, Field::WifiRadio) {
        FieldValue::Bool(true) => {
            v.push_str("On");
            Tone::Ok
        }
        FieldValue::Bool(false) => {
            v.push_str("Off");
            Tone::Idle
        }
        _ => {
            v.push_str("Unknown");
            Tone::Warn
        }
    };
    (v, tone)
}
