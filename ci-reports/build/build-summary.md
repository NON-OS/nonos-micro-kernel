# build report

| field | value |
|---|---|
| module | build |
| status | fail |
| blocking | true |
| commit | c8dd9f4744b20d51492d8650ec0cd16240d8d3c0 |
| toolchain | nightly-2026-01-16 |

## checks

| name | status | detail | artifact |
|---|---|---|---|
| rustfmt | pass | cargo fmt --check (nonos-verify) |  |
| clippy-nonos-sign | pass | clippy -D warnings (nonos-sign) |  |
| build-x86_64-capsules | fail | make nonos-mk-capsules |  |
| section-size | pass | size + readelf -S captured on release ELF |  |
| symbol-scan | pass | binary symbol scan (nonos-ci/scan-microkernel-symbols.sh) |  |

## gaps

| what | needs |
|---|---|
| kernel build lane for aarch64 | a aarch64-nonos.json kernel target + a make build target (only userland/aarch64-nonos-user.json exists today) |
| kernel build lane for riscv64 | a riscv64-nonos.json kernel target + a make build target (only userland/riscv64-nonos-user.json exists today) |
