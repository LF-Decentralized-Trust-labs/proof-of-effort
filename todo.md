# Todo
<!-- suggest | Updated: 2026-03-18 | Domain: mixed | Languages: rust | Content: specification, cicd, documentation | Files: 36 | Issues: 48 -->

## Summary
| Severity | Open | Fixed | Skipped | Possibly Fixed |
|----------|------|-------|---------|----------------|
| CRITICAL | 0    | 5     | 0       | 0              |
| HIGH     | 1    | 17    | 0       | 0              |
| MEDIUM   | 0    | 25    | 0       | 0              |

## Systemic Issues
- [x] **SYS-001** `unpinned_action` — 6 workflows — HIGH
  <!-- pid:unpinned_action | verified:true | first:2026-03-17 | last:2026-03-17 -->
  All GitHub Actions use mutable version tags (`@v1`, `@v6`, `@stable`) instead of pinned commit SHAs.
  Files: `.github/workflows/ghpages.yml`, `publish.yml`, `rust.yml`, `update.yml`, `cddl.yml`, `release.yml`
  Fix: Pin all actions to commit SHAs. Use `pin-github-action` tool or Dependabot's `versioning-strategy: increase-if-necessary` with SHA pinning.
  Effort: small (mechanical)

- [x] **SYS-002** `draft_naming_stale` — 5 files — HIGH
  <!-- pid:draft_naming_stale | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Stale "rats-pop" naming convention not updated to "cpop" after draft rename.
  Files: `.github/ISSUE_TEMPLATE/bug_report.yml:56-57`, `.github/copilot-instructions.md:11-12`, `.github/instructions/kramdown-rfc.instructions.md:22`, `crates/cpop-protocol/CHANGELOG.md:11,27`, `crates/cpop-protocol/src/rfc.rs:26`
  Fix: Replace all `rats-pop` references with `cpop`. Effort: small

- [x] **SYS-003** `missing_permissions_block` — 4 workflows — MEDIUM
  <!-- pid:missing_permissions_block | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Workflows `publish.yml`, `rust.yml`, `cddl.yml`, `update.yml` lack explicit `permissions:` blocks, granting implicit full permissions.
  Fix: Add minimal permissions block to each workflow. Effort: small

## Critical
- [x] **C-001** `[correctness]` `draft-condrey-cpop-appraisal.md:1452` — Tool receipt PKI deferred to unspecified companion document
  <!-- pid:tool_receipt_pki_missing | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Verifiers cannot securely validate tool-receipts; no key discovery, trust model, or fallback defined. Security hole.
  Fix: Add MUST NOT for unverifiable receipts, define fallback behavior, sketch PKI mechanism. Effort: large

- [x] **C-002** `[correctness]` `draft-condrey-cpop-appraisal.md:617` — Verdict contradiction resolution undefined
  <!-- pid:verdict_contradiction_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: "Contradictory" forensic results not formally defined; no resolution procedure for conflicting flags. Verdict becomes non-deterministic.
  Fix: Define contradiction formally with decision tree or matrix. Effort: large

- [x] **C-003** `[correctness]` `draft-condrey-cpop-appraisal.md:598` — Perplexity scoring has conflicting normative language (SHOULD compute vs MUST flag)
  <!-- pid:perplexity_ambiguous | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Verifiers will diverge on whether perplexity is mandatory. Scope of "per-checkpoint perplexity" undefined.
  Fix: Resolve SHOULD/MUST conflict; define checkpoint scope; specify fallback when model unavailable. Effort: medium

- [x] **C-004** `[correctness]` `draft-condrey-cpop-protocol.md:1337` — attestation-tier is optional (?) but Verifier MUST assess tier from it
  <!-- pid:attestation_tier_optional_must | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Undefined behavior when attestation-tier field is absent. Interoperability failure.
  Fix: Either make field required or specify deterministic tier derivation algorithm when absent. Effort: small

- [x] **C-005** `[correctness]` `examples/writers-authenticity-report.cddl-diag:76` — COSE_Sign1 placeholder (4 bytes) invalid against schema
  <!-- pid:cose_sign1_invalid | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Example contradicts schema requirement for valid COSE_Sign1 structure.
  Fix: Use larger placeholder with explicit documentation, or minimal valid COSE_Sign1. Effort: small

## High
- [x] **H-001** `[correctness]` `draft-condrey-cpop-appraisal.md:577` — SNR spectral flatness algorithm undefined
  <!-- pid:spectral_flatness_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: No interoperable spectral flatness estimator specified. Normative flagging requirement has no computable definition.
  Fix: Define spectral flatness algorithm (Wiener entropy), specify PSD estimator and parameters. Effort: large

- [x] **H-002** `[correctness]` `draft-condrey-cpop-appraisal.md:580` — CLC "semantic complexity" undefined and non-deterministic
  <!-- pid:semantic_complexity_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Verifiers cannot implement CLC deterministically. No algorithm for computing segment-level complexity.
  Fix: Define concrete algorithm (vocabulary rarity, compression ratio, syntactic depth). Effort: large

- [x] **H-003** `[correctness]` `draft-condrey-cpop-appraisal.md:583` — C_intra (Mechanical Turk) "pause duration" and "edit complexity" undefined
  <!-- pid:cintra_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: C_intra values non-interoperable across implementations.
  Fix: Define pause-duration scope, edit-complexity computation algorithm. Effort: medium

- [x] **H-004** `[correctness]` `draft-condrey-cpop-appraisal.md:475` — Step 7 (Channel Binding) verdict assignment missing
  <!-- pid:step7_verdict_missing | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Ambiguous whether channel binding failure = invalid or suspicious.
  Fix: Add explicit verdict for Step 7 failure. Effort: small

- [x] **H-005** `[completeness]` `draft-condrey-cpop-appraisal.md:493` — Profile mismatch fallback missing
  <!-- pid:profile_fallback_missing | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: No interoperable behavior when Verifier requires ENHANCED but receives CORE.
  Fix: Add profile matching step with explicit verdict assignment. Effort: small

- [x] **H-006** `[completeness]` `draft-condrey-cpop-appraisal.md:1573` — Assistive mode spoofing risk
  <!-- pid:assistive_mode_spoofing | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Adversary can claim assistive mode to bypass behavioral checks at T1/T2.
  Fix: Clarify validation mechanism; add security note about unverified assistive claims. Effort: medium

- [x] **H-007** `[completeness]` `draft-condrey-cpop-appraisal.md:1387` — Effort attribution for CORE cross-references undefined
  <!-- pid:effort_attribution_cross_ref | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Human-fraction may be misleading when referencing low-assurance Evidence.
  Fix: Define confidence discount for CORE-tier cross-references. Effort: medium

- [x] **H-008** `[structure]` `draft-condrey-cpop-appraisal.md:1010` — "windows" in forensic-flag CDDL never defined
  <!-- pid:window_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: affected-windows and total-windows fields uninterpretable.
  Fix: Define "window" (checkpoint-duration interval or 1-second interval). Effort: small

- [x] **H-009** `[completeness]` `draft-condrey-cpop-protocol.md:1630` — Tool receipt key discovery mechanism unspecified
  <!-- pid:tool_receipt_key_discovery | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Tool receipt verification normative but key discovery is undefined.
  Fix: Specify minimum PKI requirements or explicit draft reference. Effort: medium

- [x] **H-010** `[completeness]` `draft-condrey-cpop-protocol.md:1150` — Attester state machine section is stub
  <!-- pid:missing_state_machine | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: No normative state machine definition for checkpoint generation.
  Fix: Populate with state diagram, transition table, error/recovery states. Effort: large

- [x] **H-011** `[correctness]` `draft-condrey-cpop-protocol.md:944` — identity-fingerprint input undefined
  <!-- pid:identity_fingerprint_input | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Two Attesters with different identity representations produce incomparable fingerprints.
  Fix: Define explicit input (canonical UTF-8 email, COSE Key ID, etc.). Effort: medium

- [x] **H-012** `[correctness]` `draft-condrey-cpop-protocol.md:2791` — Threat model incomplete (Dolev-Yao defined but not used)
  <!-- pid:incomplete_threat_model | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Security analysis lacks formal adversary capability statement; attack taxonomy not mapped to threat model.
  Fix: Expand with per-attack formalization. Effort: large

- [x] **H-013** `[correctness]` `.github/workflows/ghpages.yml:21` — Missing `pages: write` permission
  <!-- pid:missing_pages_perm | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: GitHub Pages deployment step may fail due to insufficient permissions.
  Fix: Add `pages: write` to permissions block. Effort: small

- [x] **H-014** `[correctness]` `.github/workflows/publish.yml:28` — Unquoted `github.ref` in git fetch command
  <!-- pid:unquoted_git_ref | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Potential command injection if ref contains special characters.
  Fix: Quote variable: `"${{ github.ref }}"`. Effort: small

- [x] **H-015** `[best_practices]` `.github/workflows/ghpages.yml:33` — Timestamp-based cache key rebuilds daily
  <!-- pid:inefficient_cache | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Cache invalidates daily instead of on content change. Same in `publish.yml:34`.
  Fix: Use `hashFiles` strategy. Effort: small

- [x] **H-016** `[completeness]` `draft-condrey-cpop-appraisal.md:1997` — T4 entropy trajectory SD undefined
  <!-- pid:entropy_trajectory_undefined | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Verifiers cannot implement T4 entropy constraint. No algorithm provided.
  Fix: Define: `SD = standard_deviation([entropy(cp_1), ..., entropy(cp_n)])`. Effort: small

- [x] **H-017** `[correctness]` `draft-condrey-cpop-appraisal.md:483` — Reference hardware definition imprecise
  <!-- pid:reference_hardware_imprecise | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: SWF temporal verification non-deterministic; "25 GB/s" is ambiguous.
  Fix: Tabulate expected times per (steps, memory) pair; define correction factors. Effort: medium

- [x] **H-018** `[completeness]` `draft-condrey-cpop-appraisal.md:1259` — Baseline z-score threshold only in table, not normative text
  <!-- pid:baseline_zscore_unclear | verified:true | first:2026-03-17 | last:2026-03-17 -->
  Impact: Inconsistent z-score thresholds across implementations.
  Fix: Add |z| > 3.0 threshold to procedure text. Effort: small

## Medium
- [x] **M-001** `[correctness]` `draft-condrey-cpop-protocol.md:870` — Checkpoint interval SHOULD vs MUST ambiguity
- [x] **M-002** `[correctness]` `draft-condrey-cpop-protocol.md:1547` — pop-timestamp allows zero (should be `.gt 0`)
- [x] **M-003** `[correctness]` `draft-condrey-cpop-protocol.md:1250` — Unknown field handling: ignore-all vs strict-reserved-range
- [x] **M-004** `[correctness]` `draft-condrey-cpop-protocol.md:877` — Packet rollover trigger conditions undefined
- [x] **M-005** `[correctness]` `draft-condrey-cpop-protocol.md:1678` — Edit-graph array truncation "recent" ambiguous
- [x] **M-006** `[correctness]` `draft-condrey-cpop-protocol.md:2087` — Nonce vs beacon ordering in seed undefined
- [x] **M-007** `[correctness]` `draft-condrey-cpop-protocol.md:3003` — Key rotation grace period unspecified
- [x] **M-008** `[correctness]` `draft-condrey-cpop-protocol.md:2314` — Beacon fetch timeout unspecified
- [x] **M-009** `[correctness]` `draft-condrey-cpop-protocol.md:936` — Baseline identity-fingerprint mismatch not checked
- [x] **M-010** `[correctness]` `draft-condrey-cpop-protocol.md:3100` — Jitter quantization default (5ms) not enforced
- [x] **M-011** `[consistency]` `draft-condrey-cpop-appraisal.md:454` — CLC terminology inconsistent across document
- [x] **M-012** `[completeness]` `draft-condrey-cpop-appraisal.md:1203` — Entropy units ambiguous (per-sample vs aggregate)
- [x] **M-013** `[completeness]` `draft-condrey-cpop-appraisal.md:1279` — Baseline similarity thresholds arbitrary, no sensitivity analysis
- [x] **M-014** `[completeness]` `draft-condrey-cpop-appraisal.md:949` — Absence proof priority vs forensic flags undefined
- [x] **M-015** `[completeness]` `draft-condrey-cpop-appraisal.md:1268` — Baseline flag status unclear (forensic flag or not?)
- [x] **M-016** `[structure]` `draft-condrey-cpop-appraisal.md:1856` — Normative content in unnumbered appendix
- [x] **M-017** `[correctness]` `ROADMAP.md:16` — Crate versions stale (v0.2.0→0.2.1, v0.1.0→0.1.1)
- [x] **M-018** `[consistency]` `cddl/cpop.cddl:224` — presence-challenge COSE_Sign1 lacks `.cbor` constraint
- [x] **M-019** `[consistency]` `cddl/cpop.cddl:1` — COSE_Sign1 external dependency not declared
- [x] **M-020** `[completeness]` `examples/evidence-packet.cddl-diag:4` — 6 optional fields omitted from example
- [x] **M-021** `[consistency]` `README.md:162` — Mixed license file references (LICENSE vs LICENSE.md)
- [x] **M-022** `[security]` `.github/workflows/release.yml:27` — Redundant GITHUB_TOKEN injection
- [x] **M-023** `[consistency]` `.github/workflows/rust.yml:28` — Inconsistent cache key strategy across workflows
- [x] **M-024** `[completeness]` `CONTRIBUTING.md:53` — Missing Rust toolchain setup for crate contributors
- [x] **M-025** `[correctness]` `draft-condrey-cpop-protocol.md:2530` — Security bound per-checkpoint vs per-session ambiguous

## Quick Wins
| ID | Sev | File:Line | Issue | Effort |
|----|-----|-----------|-------|--------|
| SYS-002 | HIGH | 5 files | Fix stale "rats-pop" → "cpop" naming | small |
| C-004 | CRITICAL | protocol:1337 | Make attestation-tier required or add derivation | small |
| C-005 | CRITICAL | examples/war:76 | Fix COSE_Sign1 placeholder | small |
| H-004 | HIGH | appraisal:475 | Add Step 7 verdict assignment | small |
| H-005 | HIGH | appraisal:493 | Add profile mismatch fallback | small |
| H-013 | HIGH | ghpages.yml:21 | Add pages: write permission | small |
| H-014 | HIGH | publish.yml:28 | Quote github.ref variable | small |
| H-015 | HIGH | ghpages/publish | Fix timestamp cache keys | small |
| H-016 | HIGH | appraisal:1997 | Define entropy trajectory SD | small |
| H-018 | HIGH | appraisal:1259 | Add z-score threshold to text | small |

## Coverage
<!-- reviewed:draft-condrey-cpop-protocol.md:2026-03-17 -->
<!-- reviewed:draft-condrey-cpop-appraisal.md:2026-03-17 -->
<!-- reviewed:cddl/cpop.cddl:2026-03-17 -->
<!-- reviewed:examples/evidence-packet.cddl-diag:2026-03-17 -->
<!-- reviewed:examples/writers-authenticity-report.cddl-diag:2026-03-17 -->
<!-- reviewed:examples/README.md:2026-03-17 -->
<!-- reviewed:.github/workflows/ghpages.yml:2026-03-17 -->
<!-- reviewed:.github/workflows/publish.yml:2026-03-17 -->
<!-- reviewed:.github/workflows/rust.yml:2026-03-17 -->
<!-- reviewed:.github/workflows/update.yml:2026-03-17 -->
<!-- reviewed:.github/workflows/cddl.yml:2026-03-17 -->
<!-- reviewed:.github/workflows/release.yml:2026-03-17 -->
<!-- reviewed:.github/dependabot.yml:2026-03-17 -->
<!-- reviewed:.github/release.yml:2026-03-17 -->
<!-- reviewed:Cargo.toml:2026-03-17 -->
<!-- reviewed:rust-toolchain.toml:2026-03-17 -->
<!-- reviewed:requirements.txt:2026-03-17 -->
<!-- reviewed:README.md:2026-03-17 -->
<!-- reviewed:ROADMAP.md:2026-03-17 -->
<!-- reviewed:CHANGELOG.md:2026-03-17 -->
<!-- reviewed:SECURITY.md:2026-03-17 -->
<!-- reviewed:MAINTAINERS.md:2026-03-17 -->
<!-- reviewed:CONTRIBUTING.md:2026-03-17 -->
<!-- reviewed:CODE_OF_CONDUCT.md:2026-03-17 -->
<!-- reviewed:LICENSE.md:2026-03-17 -->
<!-- reviewed:docs/architecture.md:2026-03-17 -->
<!-- reviewed:docs/integration/c2pa.md:2026-03-17 -->
<!-- reviewed:docs/integration/cawg.md:2026-03-17 -->
<!-- reviewed:docs/integration/did.md:2026-03-17 -->
<!-- reviewed:.github/pull_request_template.md:2026-03-17 -->
<!-- reviewed:.github/ISSUE_TEMPLATE/bug_report.yml:2026-03-17 -->
<!-- reviewed:.github/ISSUE_TEMPLATE/feature_request.yml:2026-03-17 -->
<!-- reviewed:.github/copilot-instructions.md:2026-03-17 -->
<!-- reviewed:.github/instructions/kramdown-rfc.instructions.md:2026-03-17 -->
<!-- reviewed:.github/instructions/cddl.instructions.md:2026-03-17 -->
<!-- reviewed:.github/instructions/integration.instructions.md:2026-03-17 -->
