# Todo
<!-- suggest | Updated: 2026-03-25 | Domain: mixed | Languages: rust | Content: specification, cicd, documentation, source | Files: 92 | Issues: 170+ -->

## Summary
| Severity | Open | Fixed | Skipped | Possibly Fixed |
|----------|------|-------|---------|----------------|
| CRITICAL | 0    | 5     | 0       | 0              |
| HIGH     | 2    | 17    | 0       | 0              |
| MEDIUM   | 41   | 25    | 0       | 0              |
| LOW      | 68   | 0     | 0       | 0              |

---

## Rust Source Code Audit (2026-03-25)

Audited 56 .rs files + 2 Cargo.toml + docs/config across cpop-jitter and cpop-protocol.

### HIGH

- [ ] [HIGH] crates/cpop-protocol/src/rfc/vdf.rs:102 - `validate()` skips duration consistency and spec bounds checks when iterations_per_second is zero; attacker can set it to 0 to bypass VDF timing validation entirely
- [ ] [HIGH] crates/cpop-protocol/src/rfc/wire_types/attestation.rs:228 - AttestationResultWire CDDL doc comment omits key 14 (confidence-tier) but the struct implements it; spec/code divergence

### MEDIUM — Security

- [ ] [MEDIUM] crates/cpop-jitter/src/traits.rs:23 - `inputs.len() as u32` silently truncates on inputs longer than 4 GiB, causing different inputs to produce the same HMAC length-prefix (collision)
- [ ] [MEDIUM] crates/cpop-jitter/src/lib.rs:230 - `EvidenceChain::with_secret` takes secret by value; the plaintext `[u8; 32]` copy remains on the caller's stack and is not zeroized
- [ ] [MEDIUM] crates/cpop-protocol/src/crypto.rs:65 - `compute_causality_lock_v2` silently falls back to v1 behavior when `phys_entropy` is empty; no warning or error
- [ ] [MEDIUM] crates/cpop-protocol/src/crypto.rs:116 - COSE sign error smuggling: empty `Vec<u8>` stored as signature before error is checked; fragile if panic-unwind occurs
- [ ] [MEDIUM] crates/cpop-protocol/src/evidence.rs:98 - `packet_id` (16 bytes) used as HMAC key is below 32-byte recommended minimum for HMAC-SHA256
- [ ] [MEDIUM] crates/cpop-protocol/src/identity.rs:150 - `IdentityManager` does not implement `Drop` with zeroization; key material in `SigningKey` may persist in memory
- [ ] [MEDIUM] crates/cpop-protocol/src/codec/cbor.rs:20 - `decode`/`decode_from` accept unbounded input with no size limit; malicious CBOR can cause OOM (DoS)
- [ ] [MEDIUM] crates/cpop-protocol/src/codec/json.rs:38 - `decode_from` via `serde_json::from_reader` accepts unbounded input; DoS vector
- [ ] [MEDIUM] crates/cpop-protocol/src/c2pa.rs:218 - COSE header label -1 is for algorithm params per RFC 9052, not for embedding public keys; should use x5chain (label 33) per C2PA spec
- [ ] [MEDIUM] crates/cpop-protocol/src/c2pa.rs:519 - Claim serialized to CBOR for signing, then re-serialized for manifest; re-serialization may differ, breaking signature verification

### MEDIUM — API & Correctness

- [ ] [MEDIUM] crates/cpop-jitter/src/pure.rs:8 - `PureJitter` fields `jmin` and `range` are `pub`; direct mutation can set `range = 0` bypassing constructor validation
- [ ] [MEDIUM] crates/cpop-jitter/src/phys.rs:11 - `PhysJitter` fields `min_entropy_bits`, `jmin`, `range` are `pub`; same bypass risk
- [ ] [MEDIUM] crates/cpop-jitter/src/phys.rs:143 - `estimate_entropy` uses variance proxy (`log2(std_dev)`), not min-entropy; can overestimate entropy for non-uniform distributions
- [ ] [MEDIUM] crates/cpop-jitter/src/evidence.rs:152 - `EvidenceChain.records` is `pub`; direct mutation bypasses append-only HMAC integrity
- [ ] [MEDIUM] crates/cpop-protocol/src/compact_ref.rs:90 - `signable_payload` relies on `serde_json` BTreeMap key ordering, which is an undocumented implementation detail
- [ ] [MEDIUM] crates/cpop-protocol/src/forensics/engine.rs:116 - `chain_duration_secs` field receives millisecond values (intervals are ms), not seconds; unit confusion
- [ ] [MEDIUM] crates/cpop-protocol/src/war/encoding.rs:148 - `signed` derived by comparing signature to all-zeros; a valid all-zero sig would be marked unsigned
- [ ] [MEDIUM] crates/cpop-protocol/src/war/profiles/c2pa.rs:140 - Hardware tier comparison uses raw i8 without `from_i8` normalization; non-standard values 3-31 incorrectly map to "hardware_bound"
- [ ] [MEDIUM] crates/cpop-protocol/src/war/profiles/vc.rs:131 - Same raw i8 comparison issue as c2pa.rs tier logic

### MEDIUM — CDDL Conformance

- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/packet.rs:72 - `error_topology`, `enclave_vise`, `zk_verdict` are Option but CDDL shows them as required (no ? prefix)
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/packet.rs:102 - `extensions` uses `serde_json::Value`; fails or produces wrong output when encoding to CBOR
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/packet.rs:162 - `entropy_millibits` is u32 but CDDL specifies uint (u64)
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/checkpoint.rs:59 - VDF proof, jitter binding, chain MAC are Option but CDDL says required
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/fixed_point.rs:48 - `from_float` casts to i32 before clamping; NaN/Infinity produce surprising results (0 or MAX)
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:730 - `StreamingStats` uses f64 but CDDL specifies float32; CBOR encoding mismatch
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:781 - `BaselineDigest.aggregate_iki_histogram` uses [f64; 9] but CDDL says [9* float32]
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:786 - `BaselineDigest.session_merkle_root` is Vec<u8> but CDDL says bstr .size 32
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:796 - `BaselineDigest.identity_fingerprint` is Vec<u8> but CDDL says bstr .size 32
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/attestation.rs:288 - `confidence_tier` at key 14 is not in the CDDL doc comment; spec/code divergence
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/attestation.rs:330 - validate() does not check `created != 0` or `chain_length != 0`

### MEDIUM — Validation & Bounds

- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:402 - `PhysicalState.thermal` unbounded Vec; DoS via crafted CBOR
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:429 - `PhysicalLiveness.thermal_trajectory` unbounded Vec; DoS
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:710 - `ProfileDeclarationWire.feature_flags` unbounded Vec; DoS
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/components.rs:231 - `MerkleProof.sibling_path` elements not validated for digest length
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/wire_types/checkpoint.rs:121 - `compute_hash` hardcodes SHA-256 but does not verify content_hash/prev_hash use same algorithm
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/jitter_binding.rs:458 - validate() does not check for NaN/infinity in float fields
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/time_evidence.rs:63 - TimeBindingTier::compute grants Enhanced for Roughtime alone; may not match CDDL intent
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/biology.rs:377 - Hurst scoring formula can yield negative values; not clamped to [0, 1]
- [ ] [MEDIUM] crates/cpop-protocol/src/rfc/biology.rs:401 - `error_topology.score` used in scoring without clamping to [0, 1]
- [ ] [MEDIUM] crates/cpop-protocol/src/c2pa.rs:671 - verify_jumbf_structure does not handle extended size boxes (box_len == 1)

### MEDIUM — Build & Dependencies

- [ ] [MEDIUM] crates/cpop-protocol/Cargo.toml:31 - getrandom "0.2" vs cpop-jitter "0.3"; inconsistent major versions
- [ ] [MEDIUM] crates/cpop-protocol/Cargo.toml:48 - cpop-jitter version "0.2.0" but Cargo.toml declares "0.2.1"
- [ ] [MEDIUM] ROADMAP.md:17 - cpop-protocol listed as v0.1.1 but Cargo.toml says v0.3.0; stale
- [ ] [MEDIUM] crates/cpop-protocol/src/codec/cbor.rs:14 - doc comment claims "deterministic CBOR" but ciborium does not guarantee RFC 8949 S4.2 key ordering
- [ ] [MEDIUM] crates/cpop-protocol/src/codec/mod.rs:69 - Format::detect does not recognize CBOR arrays (0x80-0x9F) or single-byte tags (0xC0-0xD8)
- [ ] [MEDIUM] crates/cpop-protocol/src/codec/mod.rs:103 - encode treats Cbor and CborWar identically; CborWar produces untagged CBOR

### LOW — Validation Gaps

- [ ] [LOW] crates/cpop-jitter/src/lib.rs:47 - `derive_session_secret` accepts empty/1-byte master keys without validation
- [ ] [LOW] crates/cpop-jitter/src/traits.rs:17 - `hmac_jitter` does not validate `range != 0`; callers guard this but function is pub(crate)
- [ ] [LOW] crates/cpop-jitter/src/model.rs:304 - `detect_repeating_pattern` requires > MIN_PATTERN_CHECKS (strict >), missing patterns at boundary
- [ ] [LOW] crates/cpop-jitter/src/model.rs:229 - `confidence > MIN_HUMAN_CONFIDENCE` check is redundant with `anomalies.is_empty()`
- [ ] [LOW] crates/cpop-jitter/src/evidence.rs:176 - TryFrom does not validate per-record sequence numbers match their indices
- [ ] [LOW] crates/cpop-jitter/src/evidence.rs:176 - TryFrom does not verify chain_mac against records; tampered MAC passes until verify_integrity called
- [ ] [LOW] crates/cpop-protocol/src/evidence.rs:247 - validate_structure does not enforce minimum 3 checkpoints; only finalize() does
- [ ] [LOW] crates/cpop-protocol/src/evidence.rs:343 - Temporal consistency allows equal timestamps; multiple checkpoints can share same millisecond
- [ ] [LOW] crates/cpop-protocol/src/baseline.rs:49 - iki_histogram not validated to sum to 1.0 or be non-negative
- [ ] [LOW] crates/cpop-protocol/src/baseline.rs:53 - hurst exponent not range-validated; NaN/Inf accepted
- [ ] [LOW] crates/cpop-protocol/src/rfc/mod.rs:133 - HashValue::validate does not reject all-zero digest
- [ ] [LOW] crates/cpop-protocol/src/rfc/mod.rs:162 - Checkpoint skips CBOR key 6; undocumented
- [ ] [LOW] crates/cpop-protocol/src/rfc/checkpoint.rs:149 - validate() does not check timestamp > 0 or sequence ordering
- [ ] [LOW] crates/cpop-protocol/src/rfc/packet.rs:419 - validate() does not check vdf.iterations > 0, jitter_seal.lang, or bucket_commitment
- [ ] [LOW] crates/cpop-protocol/src/rfc/packet.rs:189 - ContentHashTree accepts segment_count 0-19; CDDL says >= 20
- [ ] [LOW] crates/cpop-protocol/src/rfc/serde_helpers.rs:57 - hex_bytes_vec::deserialize has no maximum length; unbounded allocation
- [ ] [LOW] crates/cpop-protocol/src/rfc/jitter_binding.rs:92 - SourceDescriptor.weight is u16 but CDDL says 0-1000; values up to 65535 accepted
- [ ] [LOW] crates/cpop-protocol/src/rfc/time_evidence.rs:361 - validate() always passes has_vdf: true; cannot detect missing VDF
- [ ] [LOW] crates/cpop-protocol/src/rfc/vdf.rs:88 - is_duration_within_spec_bounds uses f64 division; imprecise at boundary
- [ ] [LOW] crates/cpop-protocol/src/rfc/biology.rs:505 - total_weight == 0.0 causes division by zero in compute_score; validate() does not catch
- [ ] [LOW] crates/cpop-protocol/src/rfc/biology.rs:523 - mean_iki_us <= 0.0 check does not catch NaN
- [ ] [LOW] crates/cpop-protocol/src/rfc/biology.rs:287 - AnomalyFlag.severity is u8 but CDDL defines only 1-3
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/checkpoint.rs:153 - validate() does not check checkpoint_id is non-zero or timestamp != 0
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/packet.rs:75 - presence_challenges permits empty vec but CDDL says [+ presence-challenge]
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/hash.rs:162 - CompactRef has no validate() method; truncated_digest size unchecked
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/attestation.rs:46 - EntropyReport meets_threshold field not cross-checked against computed result

### LOW — Missing Derives & API Quality

- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/components.rs:39 - DocumentRef, EditDelta, ProcessProof, JitterBindingWire, PhysicalState, PhysicalLiveness, PresenceChallenge, ChannelBinding, Receipt, StreamingStats, SessionBehavioralSummary, BaselineDigest, BaselineVerification, ProfileDeclarationWire all missing PartialEq derive
- [ ] [LOW] crates/cpop-protocol/src/rfc/mod.rs:148 - DocumentRef, Checkpoint, EvidencePacket, AttestationResult missing PartialEq/Eq
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/enums.rs:all - All enums missing Hash derive (useful for HashSet/HashMap)
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/enums.rs:all - All enums missing Display impl
- [ ] [LOW] crates/cpop-protocol/src/error.rs:8 - Error enum missing Clone, PartialEq derives
- [ ] [LOW] crates/cpop-jitter/src/lib.rs:80 - cpop_jitter::Error missing Clone, PartialEq derives
- [ ] [LOW] crates/cpop-protocol/src/war/profiles/vc.rs:166 - Issuer hardcoded to "did:web:writerslogic.com"; should be parameterized
- [ ] [LOW] Cargo.toml:8 - workspace.package defines metadata but crates redeclare locally instead of using `workspace = true`
- [ ] [LOW] crates/cpop-jitter/Cargo.toml:25 vs crates/cpop-protocol/Cargo.toml:26 - subtle version mismatch: "2.5" vs "2.6"

### LOW — Test Coverage Gaps

- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/tests.rs:1 - No negative validation tests (version != 1, too few checkpoints, oversized strings, zero IDs)
- [ ] [LOW] crates/cpop-protocol/src/rfc/wire_types/tests.rs:167 - Test sets revision_entropy: 2.8 with meets_threshold: true; inconsistent test data
- [ ] [LOW] crates/cpop-protocol/tests/e2e.rs:109 - make_checkpoint sets prev_hash to zeros for every checkpoint; does not test causality chain
- [ ] [LOW] crates/cpop-protocol/tests/integration_test.rs:1 - Only tests roundtrip with zero checkpoints; no populated checkpoint test
- [ ] [LOW] crates/cpop-protocol/src/forensics/engine.rs:156 - CV thresholds (0.15 and 0.80) have no cited empirical calibration reference
- [ ] [LOW] crates/cpop-protocol/src/forensics/transcription.rs:39 - linearity score can go negative if deletions + insertions > total_keystrokes; not clamped

### LOW — Miscellaneous

- [ ] [LOW] crates/cpop-jitter/src/phys.rs:84 - Non-x86/non-aarch64 hardware path reads Instant in tight loop; may produce low-entropy samples
- [ ] [LOW] crates/cpop-jitter/src/phys.rs:105 - Non-hardware path XORs single kernel_seed with sequential counter; minimal mixing
- [ ] [LOW] crates/cpop-protocol/src/wasm.rs:148 - Error classification uses string matching (msg.contains()); brittle if message changes
- [ ] [LOW] crates/cpop-protocol/src/c2pa.rs:34 - ProcessAssertion "jitter_seals" populated from checkpoint_hash, not jitter_hash; misleading
- [ ] [LOW] crates/cpop-protocol/src/c2pa.rs:556 - validate_manifest does not verify COSE_Sign1 signature; forged sigs pass
- [ ] [LOW] crates/cpop-protocol/src/war/encoding.rs:81 - Missing blank line in headers not explicitly detected; headers may bleed into statement
- [ ] [LOW] crates/cpop-protocol/src/crypto.rs:25 - debug_assert for field length compiled out in release; silent truncation possible
- [ ] [LOW] crates/cpop-protocol/src/identity.rs:142 - generate() creates Zeroizing seed but SigningKey itself is not wrapped in Zeroizing
- [ ] [LOW] crates/cpop-jitter/benches/benchmarks.rs:55 - chain.append() result silently ignored
- [ ] [LOW] crates/cpop-jitter/tests/e2e.rs:15 - Hardcoded weak secret [42u8; 32] appears in all examples; copy-paste risk
- [ ] [LOW] SECURITY.md:36 - Security team below LFDT-recommended minimum of three members; no resolution timeline
- [ ] [LOW] docs/architecture.md:101 - CBOR tag 1129336656 described as CPOP but not IANA-registered; could mislead readers
- [ ] [LOW] crates/cpop-protocol/Cargo.toml:38 - ciborium "0.2" does not enforce deterministic encoding per RFC 8949 S4.2

---

## Specifications & CI Audit (2026-03-18)

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
