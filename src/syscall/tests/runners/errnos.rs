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

use crate::syscall::tests::errnos as t;
use crate::test::framework::{TestCase, TestSuite};

pub fn register(s: &mut TestSuite) {
    for case in CASES {
        s.add(TestCase::new(case.0, case.1));
    }
}

type ErrnoFn = fn();

const CASES: &[(&str, ErrnoFn)] = &[
    ("errnos::eperm", t::test_eperm),
    ("errnos::enoent", t::test_enoent),
    ("errnos::esrch", t::test_esrch),
    ("errnos::eintr", t::test_eintr),
    ("errnos::eio", t::test_eio),
    ("errnos::enxio", t::test_enxio),
    ("errnos::e2big", t::test_e2big),
    ("errnos::enoexec", t::test_enoexec),
    ("errnos::ebadf", t::test_ebadf),
    ("errnos::echild", t::test_echild),
    ("errnos::eagain", t::test_eagain),
    ("errnos::enomem", t::test_enomem),
    ("errnos::eacces", t::test_eacces),
    ("errnos::efault", t::test_efault),
    ("errnos::enotblk", t::test_enotblk),
    ("errnos::ebusy", t::test_ebusy),
    ("errnos::eexist", t::test_eexist),
    ("errnos::exdev", t::test_exdev),
    ("errnos::enodev", t::test_enodev),
    ("errnos::enotdir", t::test_enotdir),
    ("errnos::eisdir", t::test_eisdir),
    ("errnos::einval", t::test_einval),
    ("errnos::enfile", t::test_enfile),
    ("errnos::emfile", t::test_emfile),
    ("errnos::enotty", t::test_enotty),
    ("errnos::etxtbsy", t::test_etxtbsy),
    ("errnos::efbig", t::test_efbig),
    ("errnos::enospc", t::test_enospc),
    ("errnos::espipe", t::test_espipe),
    ("errnos::erofs", t::test_erofs),
    ("errnos::emlink", t::test_emlink),
    ("errnos::epipe", t::test_epipe),
    ("errnos::edom", t::test_edom),
    ("errnos::erange", t::test_erange),
    ("errnos::edeadlk", t::test_edeadlk),
    ("errnos::enametoolong", t::test_enametoolong),
    ("errnos::enolck", t::test_enolck),
    ("errnos::enosys", t::test_enosys),
    ("errnos::enotempty", t::test_enotempty),
    ("errnos::eloop", t::test_eloop),
    ("errnos::ewouldblock_equals_eagain", t::test_ewouldblock_equals_eagain),
    ("errnos::edeadlock_equals_edeadlk", t::test_edeadlock_equals_edeadlk),
    ("errnos::enotsock", t::test_enotsock),
    ("errnos::edestaddrreq", t::test_edestaddrreq),
    ("errnos::emsgsize", t::test_emsgsize),
    ("errnos::eprototype", t::test_eprototype),
    ("errnos::enoprotoopt", t::test_enoprotoopt),
    ("errnos::eprotonosupport", t::test_eprotonosupport),
    ("errnos::esocktnosupport", t::test_esocktnosupport),
    ("errnos::eopnotsupp", t::test_eopnotsupp),
    ("errnos::epfnosupport", t::test_epfnosupport),
    ("errnos::eafnosupport", t::test_eafnosupport),
    ("errnos::eaddrinuse", t::test_eaddrinuse),
    ("errnos::eaddrnotavail", t::test_eaddrnotavail),
    ("errnos::enetdown", t::test_enetdown),
    ("errnos::enetunreach", t::test_enetunreach),
    ("errnos::enetreset", t::test_enetreset),
    ("errnos::econnaborted", t::test_econnaborted),
    ("errnos::econnreset", t::test_econnreset),
    ("errnos::enobufs", t::test_enobufs),
    ("errnos::eisconn", t::test_eisconn),
    ("errnos::enotconn", t::test_enotconn),
    ("errnos::eshutdown", t::test_eshutdown),
    ("errnos::etoomanyrefs", t::test_etoomanyrefs),
    ("errnos::etimedout", t::test_etimedout),
    ("errnos::econnrefused", t::test_econnrefused),
    ("errnos::ehostdown", t::test_ehostdown),
    ("errnos::ehostunreach", t::test_ehostunreach),
    ("errnos::ealready", t::test_ealready),
    ("errnos::einprogress", t::test_einprogress),
    ("errnos::estale", t::test_estale),
    ("errnos::ecanceled", t::test_ecanceled),
    ("errnos::enokey", t::test_enokey),
    ("errnos::ekeyexpired", t::test_ekeyexpired),
    ("errnos::ekeyrevoked", t::test_ekeyrevoked),
    ("errnos::ekeyrejected", t::test_ekeyrejected),
    ("errnos::eownerdead", t::test_eownerdead),
    ("errnos::enotrecoverable", t::test_enotrecoverable),
    ("errnos::erfkill", t::test_erfkill),
    ("errnos::ehwpoison", t::test_ehwpoison),
    ("errnos::errno_values_are_positive", t::test_errno_values_are_positive),
    ("errnos::errno_values_unique", t::test_errno_values_unique),
    ("errnos::errno_range_basic", t::test_errno_range_basic),
    ("errnos::errno_range_network", t::test_errno_range_network),
];
