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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        let _ = $crate::io::stdout().write_fmt(::core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{ use $crate::io::Write; let _ = $crate::io::stdout().write_all(b"\n"); }};
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        let _ = $crate::io::stdout().write_fmt(::core::format_args!($($arg)*));
        let _ = $crate::io::stdout().write_all(b"\n");
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        let _ = $crate::io::stderr().write_fmt(::core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {{ use $crate::io::Write; let _ = $crate::io::stderr().write_all(b"\n"); }};
    ($($arg:tt)*) => {{
        use $crate::io::Write;
        let _ = $crate::io::stderr().write_fmt(::core::format_args!($($arg)*));
        let _ = $crate::io::stderr().write_all(b"\n");
    }};
}
