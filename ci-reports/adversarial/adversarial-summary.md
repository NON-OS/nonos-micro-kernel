# adversarial report

| field | value |
|---|---|
| module | adversarial |
| status | pass |
| blocking | true |
| commit | 69b7f2f55445b52a5cd0b779e11f33785fb0a4b8 |
| toolchain | nightly-2026-01-16 |

## checks

| name | status | detail | artifact |
|---|---|---|---|
| baseline | pass | valid victim: about |  |
| attack:cert-bitflip | pass | DENIED (cert verify: VerifyTrustAnchorBadSig("mldsa65")) |  |
| attack:manifest-bitflip | pass | DENIED (manifest verify: VerifyPublisherBadSig("mldsa65")) |  |
| attack:policy-bitflip | pass | DENIED (cert verify: VerifyTrustAnchorBadSig("mldsa65")) |  |
| attack:cert-expired | pass | DENIED (cert verify: VerifyExpired) |  |
| attack:cert-premature | pass | DENIED (cert verify: VerifyNotYetValid) |  |
| attack:cert-truncated | pass | DENIED (cert decode: KeyFileShape("unexpected eof in decode")) |  |
| attack:cross-binding | pass | DENIED (manifest verify: VerifyNonosIdCertIdMismatch) |  |
