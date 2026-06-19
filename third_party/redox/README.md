# Redox donor quarantine

This directory holds vendored Redox OS source used as a porting donor. It is a
quarantine: nothing here is compiled into NØNOS. It is not a workspace member and
no NØNOS module may `use` it. Files keep their original MIT notices. Native NØNOS
code is written by translating donor logic into capsule + broker + capability
contracts, then the donor file is no longer needed and is deleted.

Redox is MIT (`src/drivers/LICENSE`, © 2017 Redox OS).
NØNOS is AGPL-3.0. MIT is one-way compatible into AGPL provided the MIT copyright
and permission notice is preserved on every copied file, which is why donor files
stay here verbatim with their headers and are referenced, not relicensed.
Translated native files under `src/` and `userland/` carry the NØNOS AGPL header
and contain no copied text.

## Vendored sources

| repo | upstream | pinned commit |
|------|----------|---------------|
| drivers | https://github.com/redox-os/drivers | 20ffe4d7f4a85b7cc1f59495d7e6e355fed4cb06 |
| redoxfs | https://gitlab.redox-os.org/redox-os/redoxfs | af493b9f4e1ee7086bc6e44a43f096d981936cfe |

Only the AHCI donor subset and the RedoxFS core layout subset are vendored (see
`IMPORT_MAP.md`). To refresh, re-clone at a new commit and update this table
plus `IMPORT_MAP.md`.

## Rules

- Never add `third_party/redox` to a workspace `members` list.
- Never `use` a donor module from NØNOS source.
- Every donor file actively translated gets a row in `IMPORT_MAP.md`.
- Delete a donor file once its NØNOS translation is native and reviewed.
