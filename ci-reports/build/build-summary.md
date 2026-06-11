# build report

| field | value |
|---|---|
| module | build |
| status | gap |
| blocking | true |
| commit | 5a44935e326ff4ebf36a5d3564a911d164eb8ccf |
| toolchain | nightly-2026-01-16 |

## checks

| name | status | detail | artifact |
|---|---|---|---|
| rustfmt | pass | cargo fmt --check (nonos-verify) |  |
| clippy-nonos-sign | pass | clippy -D warnings (nonos-sign) |  |
| build-x86_64-capsules | pass | make nonos-mk-capsules |  |
| section-size | pass | size + readelf -S captured on release ELF |  |
| symbol-scan | pass | binary symbol scan (nonos-ci/scan-microkernel-symbols.sh) |  |

## gaps

| what | needs |
|---|---|
| kernel build lane for aarch64 | a aarch64-nonos.json kernel target + a make build target (only userland/aarch64-nonos-user.json exists today) |
| kernel build lane for riscv64 | a riscv64-nonos.json kernel target + a make build target (only userland/riscv64-nonos-user.json exists today) |
