/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The one hand-provided external definition for the policy extraction, filled
in from FunsExternal_Template.lean as Aeneas prescribes. The kernel's
check_range uses the `?` operator on `checked_add(..).ok_or(..)`, and Aeneas
treats core::option::Option::ok_or as external, so its four-line semantics
is given here and enters the trusted base. It is the documented behavior of
ok_or: Some maps to Ok, None maps to the provided error.
-/

import Aeneas
import Policy.Types
open Aeneas Aeneas.Std Result

/-- [core::option::{core::option::Option<T>}::ok_or]:
    Source: '/rustc/library/core/src/option.rs', lines 1334:4-1334:73
    Name pattern: [core::option::{core::option::Option<@T>}::ok_or] -/
@[rust_fun "core::option::{core::option::Option<@T>}::ok_or"]
def core.option.Option.ok_or
    {T : Type} {E : Type} (o : Option T) (e : E) :
    Result (core.result.Result T E) :=
  match o with
  | some x => ok (core.result.Result.Ok x)
  | none => ok (core.result.Result.Err e)
