# trust-chain report

| field | value |
|---|---|
| module | trust-chain |
| status | gap |
| blocking | true |
| commit | e6a31389c56c40e094c188264ab84b8b1479510a |
| toolchain | nightly-2026-01-16 |

## checks

| name | status | detail | artifact |
|---|---|---|---|
| policy-decode | pass | baked trust-anchor policy decoded |  |
| chain:keyring | pass | cert+manifest verified against baked anchor |  |
| chain:proof_io | pass | cert+manifest verified against baked anchor |  |
| chain:ramfs | pass | cert+manifest verified against baked anchor |  |

## gaps

| what | needs |
|---|---|
| per-capsule field assertions | expose DecodedManifest fields (caps, endpoints, target_triple) so they can be diffed against each Capsule.mk |
