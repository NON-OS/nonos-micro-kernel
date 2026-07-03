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

use super::affine::Affine;
use super::math::{cos, sin, PI};
use super::num::num_list;

// A transform attribute: translate/scale/rotate/matrix functions compose
// left to right. Unknown functions (skewX/Y) are ignored rather than
// guessed.
pub(super) fn parse_transform(v: &str) -> Affine {
    let mut m = Affine::identity();
    let mut rest = v;
    while let Some(open) = rest.find('(') {
        let name = rest[..open].trim().to_ascii_lowercase();
        let Some(close) = rest[open..].find(')') else { break };
        let args = num_list(&rest[open + 1..open + close]);
        let step = match (name.as_str(), args.len()) {
            ("translate", 1) => Affine::translate(args[0], 0.0),
            ("translate", _) if args.len() >= 2 => Affine::translate(args[0], args[1]),
            ("scale", 1) => Affine::scale(args[0], args[0]),
            ("scale", _) if args.len() >= 2 => Affine::scale(args[0], args[1]),
            ("rotate", 1) => rotate(args[0]),
            ("rotate", _) if args.len() >= 3 => Affine::translate(args[1], args[2])
                .then(&rotate(args[0]))
                .then(&Affine::translate(-args[1], -args[2])),
            ("matrix", _) if args.len() >= 6 => {
                Affine([args[0], args[1], args[2], args[3], args[4], args[5]])
            }
            _ => Affine::identity(),
        };
        m = m.then(&step);
        rest = &rest[open + close + 1..];
    }
    m
}

fn rotate(deg: f32) -> Affine {
    let a = deg * PI / 180.0;
    let (c, s) = (cos(a), sin(a));
    Affine([c, s, -s, c, 0.0, 0.0])
}
