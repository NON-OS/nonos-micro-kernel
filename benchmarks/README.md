# NØNOS Benchmarks

This directory is the public benchmark evidence root.

Generated benchmark runs are written under `benchmarks/runs/<run-id>/` by the
benchmark workflow and uploaded as GitHub Actions artifacts. Generated run data
is not committed by default; every uploaded run carries its own manifest,
host metadata, raw logs, parsed JSON, CSV metrics, and summary.

Pull-request CI runs the strict host/build/boot benchmark tier. The benchmark
lane provisions scratch trust for the desktop GUI capsule fleet before the boot
harness runs. A manual workflow dispatch can disable QEMU boot evidence for
host/build-only diagnosis, but the default benchmark lane is expected to
produce boot artifacts.

The benchmark contract is:

- `host.json`: host, toolchain, git, CPU, memory, and QEMU metadata
- `build-verify-fast.json`: build verification wall time and resource usage
- `boot-evidence.json`: bounded boot evidence harness timing
- `boot-log.json`: parsed serial markers, boot phases, and ZK/capsule status
- `results.jsonl`: normalized benchmark records
- `metrics.csv`: tabular metrics and parsed boot phase samples
- `manifest.json`: SHA-256 manifest for every artifact in the run directory
- `summary.md`: human-readable run summary

Boot latency claims must use parsed `[BENCH]` phase metrics from `boot-log.json`
and `metrics.csv`, not harness wall time alone.
