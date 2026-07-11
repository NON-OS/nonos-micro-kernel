# Contributor Rewards (NOX)

NONOS rewards contributions with **NOX**, the project's token, in proportion to
how much a contribution moves the system forward and how good it is. This is a
discretionary rewards program run by the maintainers: you build something that
lands and matters, and a share of NOX comes back to you.

It is not a bounty auction and not a salary. It is a way to share the upside of
the project with the people who make it better.

## How it works

1. **Contribute.** Open a pull request. Land real work: a fix, a feature, a
   driver, a hardening, a proof, documentation.
2. **It gets reviewed and merged.** A contribution earns a reward only once it
   is accepted on `main` through normal review.
3. **The maintainers assess it.** They weigh the impact and the quality (below)
   and place it in a reward tier.
4. **NOX is sent to your address.** You provide a NOX payout address; the reward
   is transferred from the project rewards pool after the work lands.

## What earns more

The size of a reward is driven by the contribution, not the line count. What
raises it:

- **Impact and scope.** A new subsystem or driver, a real capability, or a fix
  that unblocks the roadmap counts far more than churn.
- **The trusted path.** Work on the security-critical core, the crypto, the
  capability enforcement, the boot and capsule-load and attestation chain, is
  weighted highest, because it is the hardest to get right and the most
  valuable to get right.
- **Verification.** Contributions that come with tests, proofs, or a runnable
  demonstration are rewarded above the same change without them.
- **Documentation.** Work that leaves the wiki and the code better explained is
  rewarded, both alongside a feature and on its own.
- **Craft.** Code that matches the project's discipline (modular, no panics in
  kernel code, `cargo fmt` and `clippy` clean, honest about its limits) is worth
  more than code that works but has to be cleaned up.

## Tiers

Contributions fall into tiers by impact and quality. The maintainers set the NOX
amount for each tier per reward cycle, so the bands below are relative, not fixed
figures:

| Tier | What it looks like |
|------|--------------------|
| Trivial | A typo, a small doc correction, a one-line obvious fix |
| Minor | A small bug fix or a small self-contained feature |
| Substantial | A meaningful feature, a subsystem improvement, a real driver capability |
| Major | A new subsystem or driver, a significant hardening, a broad correctness fix |
| Critical | A verified security fix or hardening on the trusted path |

The verification and documentation dimensions above act as multipliers within a
tier: the same substantial feature earns more when it ships with tests and docs.

## Claiming a reward

- Put a **NOX payout address** in your pull request description, or send it to
  `team@nonos.systems` after your work is merged.
- Rewards are paid after the contribution is merged and reviewed, in batches.
- One reward per accepted contribution; a series of related PRs may be assessed
  together.

## Not eligible

- Spam, machine-generated slop, or trivial churn made to farm rewards.
- Plagiarized work, or work that is not yours to license.
- Self-merged or unreviewed changes.
- Reformatting, dependency bumps, or renames with no substance behind them.

All contributions must be your own original work, offered under the project's
AGPL-3.0 license.

## The honest part

This is a discretionary community rewards program. Amounts are set by the
maintainers, the program can change or end, and a reward is not a wage, a
contract, or an offer of investment. Nothing here is financial advice, and
participation is subject to applicable law in your jurisdiction. If you are
unsure whether receiving NOX is appropriate for you, do not claim a reward.

Questions: `team@nonos.systems`.
