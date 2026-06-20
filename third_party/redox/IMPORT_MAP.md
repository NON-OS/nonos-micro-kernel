# Redox donor — import map

Quarantined reference material from Redox OS. **Not a workspace member, never
compiled into the kernel.** Used only to translate register layouts, command
sequencing, and protocol state machines into NØNOS-native capsules.

The donor *source* is git-ignored (we do not redistribute it); only this
manifest is tracked. Anyone can reproduce the exact source by cloning each repo
at the pinned commit below and verifying the per-file SHA256.

License: every vendored repo is MIT; its `LICENSE` file is kept on disk.

Status legend: `vendored` (on disk, untouched) · `translating` (NØNOS port in
progress) · `translated` (port landed, donor no longer needed) · `reference`
(consulted for structure, not line-translated).

## Pinned commits

| repo | url | commit |
|------|-----|--------|
| ransid | https://github.com/redox-os/ransid | c73c3e9464b9c660ecdbb2fe1a078b9c3ce918f2 |
| orbterm | https://github.com/redox-os/orbterm | 8ac498177e0913034ab8e1879e1127c2034eed30 |

## Files

| donor file | sha256 | nonos target | status |
|------------|--------|--------------|--------|
| ransid/src/lib.rs | e4ce6348d3fa4161883b3869150c9ea57014da265ba05b6114f8f6b64bc275e9 | userland/capsule_terminal/src/term/vt/ | translating |
| ransid/src/color.rs | a04bf6c2e1ac71ad7f3e42e32bede785c7ce809d9484ed459f33cb2604d19cb4 | userland/capsule_terminal/src/term/vt/color.rs | translated |
| orbterm/src/console.rs | a7398fb40daae5b07a55473ccd766e71ba5514ce92a946be2f2ea2fc49d3a56e | userland/capsule_terminal (grid model) | reference |
| orbterm/src/handle.rs | 7432140a690e25d52f3f1f42ebff45fa37974250bd830ccf7f9663265dbcdd86 | userland/capsule_terminal (pty/output) | reference |
| orbterm/src/main.rs | bdf0cdb8213b5cfecbda8d26ea7d1e7d909f4d9910e285f2771687d43e7a39f0 | userland/capsule_terminal (loop) | reference |
