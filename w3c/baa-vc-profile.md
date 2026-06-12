# Bayesian Attestation Appraisal -- Verifiable Credential Profile

> **Specification Status**: Draft
>
> **Profile of**: W3C Verifiable Credentials Data Model v2.0, W3C Confidence
> Method v0.9/v1.0
>
> **Companion to**: IETF `draft-condrey-cpoe-appraisal` (Written Authorship
> Report)

## 1. Introduction

This profile defines how the output of a CPoE Bayesian Attestation Appraisal
(the Written Authorship Report, or WAR) is expressed as a W3C Verifiable
Credential with confidence scores conforming to the W3C Confidence Method.

The WAR is a multi-signal appraisal that evaluates temporal binding, behavioral
entropy, content-latency correlation, forensic anomalies, and baseline
consistency.  This profile maps those heterogeneous signals into a single
credential with a composite confidence value that relying parties can consume
without understanding the underlying cryptographic machinery.

## 2. Conformance

A credential conforming to this profile:

- MUST use the `ProofOfEffortCredential` type.
- MUST include the JSON-LD context `https://purl.org/poe/v1`.
- MUST include an `authorshipConfidence` object in the credential subject.
- MUST include an `evaluationResult` reflecting the WAR verdict.
- MUST include a `confidenceMethod` object computed per Section 5.
- SHOULD include the `evidence` array referencing the CPoE evidence packet.
- MUST NOT be issued when the WAR verdict is `invalid` (see Section 3.1).

## 3. Credential Type

```json
{
  "type": ["VerifiableCredential", "ProofOfEffortCredential"]
}
```

### 3.1 Issuance Policy

Not every WAR should result in a credential.  The issuer MUST follow this
policy:

| WAR Verdict | Issuance Decision |
|-------------|------------------|
| `authentic` (1) | Issue. All checks passed. |
| `inconclusive` (2) | Issue with `warnings`. Insufficient behavioral data, but no structural failure. |
| `suspicious` (3) | Issue with `warnings` populated and `confidenceScore` reflecting penalty. The credential is honest about its own doubt. |
| `invalid` (4) | Do NOT issue. Chain broken, SWF failed, or structural error. No credential should assert confidence in evidence that failed structural verification. |

Rationale: Issuing a credential for `invalid` evidence would undermine trust
in the credential type.  `Suspicious` evidence still gets a credential because
the anomaly may be legitimate (e.g., assistive technology, unusual typing
patterns), and the confidence score already reflects the penalty.

## 4. Credential Subject Properties

### 4.1 Core Properties

| Property | Type | Required | Source (WAR CBOR Key) | Description |
|----------|------|----------|----------------------|-------------|
| `evaluationResult` | string | MUST | 3 (verdict) | One of: `authentic`, `inconclusive`, `suspicious` |
| `assuranceLevel` | string | MUST | 4 (attestation-tier) | `T1-software`, `T2-attested`, `T3-hardware-bound`, `T4-hardware-hardened` |
| `authorshipConfidence` | object | MUST | 15 (effort-attribution) | Human authorship attribution; see Section 4.2 |
| `documentDigest` | string | MUST | (evidence-packet.5) | `algorithm:hex` digest of the attested document |
| `evidenceDigest` | string | MUST | 2 (evidence-ref) | `sha256:hex` digest of the evidence packet |

### 4.2 authorshipConfidence

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `humanFraction` | number | MUST | 15.1 | 0.0 to 1.0; fraction of content attributed to human authorship |
| `humanCheckpoints` | integer | SHOULD | 15.2 | Checkpoints without tool receipts |
| `receiptCheckpoints` | integer | SHOULD | 15.3 | Checkpoints containing tool receipts |
| `toolAttributedChars` | integer | MAY | 15.4 | Character count attributed to tool output |
| `totalChars` | integer | MAY | 15.5 | Total character count of the document |

### 4.3 entropicEvidence

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `timingBitsPerSample` | number | SHOULD | 7.1 | Shannon entropy of IKI distribution (bits/sample) |
| `revisionBits` | number | SHOULD | 7.2 | Shannon entropy of edit-delta size distribution |
| `pauseBits` | number | SHOULD | 7.3 | Shannon entropy of inter-checkpoint pause distribution |
| `meetsThreshold` | boolean | MUST | 7.4 | True if timing >= 3.0, revision >= 3.0, pause >= 2.0 |

### 4.4 jitterSeal

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `entropyEstimateCentibits` | integer | SHOULD | checkpoint.10.2 | Aggregate entropy across checkpoints (1/100th bit) |
| `sealed` | boolean | MUST | (derived) | True if all checkpoints with jitter-binding have valid HMAC seals |

### 4.5 processMonotonicity

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `processMonotonicity` | boolean | MUST | (derived from HAT) | True if all TPM resetCount values are consistent across the session |

The name "process monotonicity" comes from the IETF HAT draft: the TPM's
`resetCount` is a monotonic counter that increments on owner-clear.  If it
changes between the `time-before` and `time-after` readings in any HAT proof,
the platform rebooted during computation, breaking the temporal binding.  A
`true` value means no reboot was detected.

### 4.6 forensicAnalysis

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `flagsTriggered` | integer | SHOULD | 13.1 | Number of forensic mechanisms that flagged anomalies |
| `flagsEvaluated` | integer | SHOULD | 13.2 | Total forensic mechanisms evaluated |
| `affectedCheckpoints` | integer | MAY | 13.3 | Checkpoints with at least one triggered flag |
| `totalCheckpoints` | integer | MAY | 13.4 | Total checkpoints analyzed |
| `mechanisms` | array | MAY | 13.5 | Per-mechanism detail; see Section 4.6.1 |

#### 4.6.1 Forensic Mechanism Detail

Each entry in the `mechanisms` array:

| Property | Type | Description |
|----------|------|-------------|
| `mechanismId` | string | Identifier: `SNR` (spectral noise), `CLC` (content-latency correlation), `IKI` (inter-keystroke interval), `ERR-TOPO` (error topology), `MTURK` (mechanical pacing) |
| `triggered` | boolean | Whether this mechanism flagged an anomaly |
| `coverage` | number | Fraction of checkpoints where this mechanism was evaluated (0.0-1.0) |

### 4.7 forgeryCostEstimate

| Property | Type | Required | Source | Description |
|----------|------|----------|--------|-------------|
| `totalCost` | number | SHOULD | 8.4 | Aggregate forgery cost |
| `costUnit` | string | SHOULD | 8.5 | `USD` or `cpu-hours` |
| `swfCost` | number | MAY | 8.1 | Sequential work function recomputation cost |
| `entropyCost` | number | MAY | 8.2 | Behavioral entropy synthesis cost |
| `hardwareCost` | number | MAY | 8.3 | Hardware attack cost |

## 5. Confidence Method Integration

This section defines how the WAR's multi-signal appraisal maps to the W3C
Confidence Method specification.

### 5.1 Confidence Score Computation

The composite `confidenceScore` is a value in `[0.0, 1.0]` derived from the
WAR's signals.  Relying parties MAY use this single number for threshold-based
decisions.

```
   base        = tier_to_base(confidence-tier)
   e_boost     = meets_threshold ? 0.05 : 0.0
   f_penalty   = min(flags_triggered * 0.05, 0.25)
   v_adjust    = verdict_adjustment(verdict)
   h_fraction  = effort-attribution.human-fraction

   confidenceScore = clamp(
     (base + e_boost - f_penalty + v_adjust) * h_fraction,
     0.0,
     1.0
   )
```

Where:

| Function | Input | Output | Rationale |
|----------|-------|--------|-----------|
| `tier_to_base` | `population-reference (1)` | 0.30 | Minimal baseline data (0-4 sessions); score reflects statistical uncertainty |
| | `emerging (2)` | 0.50 | 5-9 sessions; IKI histogram is forming but not yet stable |
| | `established (3)` | 0.70 | 10-19 sessions; Bhattacharyya coefficient against baseline is meaningful |
| | `mature (4)` | 0.85 | 20+ sessions; baseline is robust; ceiling below 1.0 reserves room for perfect evidence |
| `verdict_adjustment` | `authentic (1)` | +0.10 | All structural and forensic checks passed |
| | `inconclusive (2)` | 0.00 | No adjustment; insufficient data is neutral, not negative |
| | `suspicious (3)` | -0.15 | 2+ forensic flags or single flag >30% checkpoints |

**Design constraints on the weights:**

- `tier_to_base` values are chosen so that a `mature` baseline + `authentic`
  verdict + entropy pass = 0.95 (not 1.0), because 1.0 would imply
  mathematical certainty, which no behavioral system can provide.
- `f_penalty` is capped at 0.25 to prevent a single noisy forensic channel
  from zeroing out an otherwise strong signal.
- The `h_fraction` multiplier is applied last so that a fully tool-generated
  document with `humanFraction = 0.0` yields `confidenceScore = 0.0`
  regardless of other signals.
- `invalid` verdict does not appear in the table because `invalid` WARs
  MUST NOT produce a credential (Section 3.1).

If `confidence-tier` is absent (no baseline enrollment), use `base = 0.30`.

If `effort-attribution` is absent, use `h_fraction = 1.0` (assume fully
human; this is conservative toward the issuer's trust claim).

### 5.2 Confidence Method Expression

Per the W3C Confidence Method v0.9 specification, confidence is expressed
as a `confidenceMethod` object in the credential subject:

```json
{
  "confidenceMethod": {
    "type": "BayesianAttestationAppraisal",
    "confidenceScore": 0.78,
    "confidenceComponents": {
      "baselineMaturity": "established",
      "baseScore": 0.70,
      "entropyBoost": 0.05,
      "forensicPenalty": 0.00,
      "verdictAdjustment": 0.10,
      "humanFraction": 0.92
    },
    "evaluationCriteria": [
      {
        "criterion": "temporal-binding",
        "method": "TPM-HAT",
        "result": "pass",
        "weight": "structural"
      },
      {
        "criterion": "behavioral-entropy",
        "method": "IKI-Shannon",
        "result": "pass",
        "bitsPerSample": 4.2,
        "threshold": 3.0
      },
      {
        "criterion": "content-latency-correlation",
        "method": "Pearson-CLC",
        "result": "pass",
        "correlation": 0.34,
        "threshold": 0.2
      },
      {
        "criterion": "forensic-analysis",
        "method": "multi-mechanism",
        "result": "pass",
        "flagsTriggered": 0,
        "flagsEvaluated": 5
      },
      {
        "criterion": "effort-attribution",
        "method": "receipt-partitioning",
        "result": "pass",
        "humanFraction": 0.92
      }
    ]
  }
}
```

**Verify**: `(0.70 + 0.05 - 0.00 + 0.10) * 0.92 = 0.782`, rounded to 0.78.

The `evaluationCriteria` array provides transparency: a relying party can see
*which signals* contributed to the score and audit each independently. The
`weight: "structural"` on temporal-binding indicates it is a pass/fail gate
(if TPM binding fails, the evidence is `invalid`) rather than a continuous
contributor.

### 5.3 Confidence Thresholds (Informative)

Relying parties may adopt thresholds based on their risk tolerance:

| Use Case | Suggested Threshold | Rationale |
|----------|-------------------|-----------|
| Academic journal submission | >= 0.70 | High confidence in human authorship; reputational risk |
| News/editorial attribution | >= 0.60 | Moderate confidence; editorial judgment supplements |
| Content marketplace listing | >= 0.50 | Baseline authenticity for commerce |
| Internal draft tracking | >= 0.30 | Low bar; primarily for provenance chain, not gatekeeping |

These thresholds are informative.  Each relying party should calibrate against
their own false-positive/false-negative tolerance.

### 5.4 Signal Independence

The confidence components are derived from independent physical phenomena:

| Signal | Physical Basis | Attack Surface |
|--------|---------------|---------------|
| Temporal binding | TPM hardware clock (monotonic, tamper-resistant) | Hardware fault injection |
| Behavioral entropy | Human motor noise (inter-keystroke jitter) | Robotic keystroke synthesis |
| Content-latency correlation | Cognitive processing time | Semantic-aware timing injection |
| Forensic analysis | Statistical anomaly detection across 5 mechanisms | Simultaneous multi-channel spoofing |
| Effort attribution | Tool receipt accounting (COSE-signed) | Receipt forgery (requires tool's signing key) |

This independence means a forger must simultaneously defeat all channels.
The `forgeryCostEstimate.totalCost` quantifies that aggregate difficulty as
the sum of per-channel attack costs.

## 6. Evidence Attachment

The VC SHOULD include an `evidence` array referencing the original CPoE
evidence packet so that a relying party can independently re-verify.

```json
{
  "evidence": [{
    "type": "CPoEEvidencePacket",
    "digestSRI": "sha256-<base64>",
    "encodingFormat": "application/cbor",
    "cborTag": 1129336645,
    "profile": "urn:ietf:params:cpoe:profile:enhanced"
  }]
}
```

The evidence packet itself is not embedded in the VC (it can exceed 100 KB
for long authoring sessions with thousands of checkpoints).  The `digestSRI`
field provides a Subresource Integrity hash for retrieval verification.  The
`profile` field indicates which content tier (CORE, ENHANCED, MAXIMUM) was
used, so the relying party knows what signals are available for re-appraisal.

## 7. Proof Requirements

Credentials conforming to this profile MUST use one of:

- `DataIntegrityProof` with `eddsa-rdfc-2022` cryptosuite (RECOMMENDED for
  JSON-LD path)
- `DataIntegrityProof` with `ecdsa-rdfc-2019` cryptosuite
- COSE envelope per VC-JOSE-COSE (RECOMMENDED for CBOR-native path)

The proof `verificationMethod` MUST resolve to a key controlled by the
CPoE Verifier that produced the WAR.  The DID method used for the verifier
is not constrained by this profile, but `did:web` and `did:key` are common
choices.

## 8. Privacy Considerations

### 8.1 Tool-Assistance Disclosure

The `authorshipConfidence.humanFraction` reveals how much of a document was
tool-assisted.  This has implications:

- **Author consent**: The author MUST consent to disclosure of tool-assistance
  ratios before the credential is issued.  A verifier SHOULD present the
  `humanFraction` to the author and allow them to decline credential issuance.
- **Selective disclosure**: Implementers MAY support BBS+ selective disclosure
  to allow authors to prove `humanFraction > threshold` without revealing the
  exact value.

### 8.2 Forensic Detail Granularity

The `forensicAnalysis` object supports three levels of detail:

| Level | Properties Included | Use Case |
|-------|-------------------|----------|
| Summary | `flagsTriggered`, `flagsEvaluated` | Pass/fail decisions |
| Coverage | + `affectedCheckpoints`, `totalCheckpoints` | Audit trail |
| Full | + `mechanisms[]` | Forensic investigation |

Issuers SHOULD default to the Summary level unless the relying party's policy
requires more.

### 8.3 No Raw Biometric Leakage

The credential does not contain raw keystroke timings, thermal readings,
accelerometer samples, or IKI histograms.  The evidence packet digest is
one-way.  A relying party that receives only the VC cannot reconstruct
behavioral biometrics.

### 8.4 Correlation Risk

Multiple credentials from the same author (same baseline fingerprint) could
be correlated across documents.  The `evidenceDigest` is per-document, but
if `baselineMaturity` progresses from `emerging` to `established` in a
predictable pattern, a colluding set of relying parties could link authorship.
Mitigations:

- Use pairwise DIDs for author identifiers.
- Omit `baselineMaturity` when not needed for the relying party's decision.

## 9. Security Considerations

### 9.1 Verifier Trust

Confidence scores are only as trustworthy as the issuing verifier.  Relying
parties:

- MUST verify the credential proof.
- SHOULD maintain an allow-list of trusted verifier DIDs.
- SHOULD periodically audit verifier behavior (e.g., do their scores
  correlate with independent human review?).

### 9.2 Score Manipulation

A malicious verifier could inflate `confidenceScore` by:

- Claiming `authentic` for `suspicious` evidence.
- Omitting forensic flags.
- Inflating `humanFraction`.

Mitigation: The `evidence` array includes the evidence packet digest.  A
relying party can re-run the appraisal independently and compare scores.
Persistent divergence between a verifier's scores and independent re-appraisal
is grounds for distrust.

### 9.3 Replay Attacks

Replay is mitigated by three bindings:

1. `evidenceDigest` binds the WAR to a specific evidence packet.
2. The evidence packet's `document-ref` binds to a specific document hash.
3. The evidence packet's `created` timestamp and `packet-id` provide temporal
   and identity uniqueness.

A replayed credential for a different document will fail the digest chain.

### 9.4 Forgery Cost Caveats

The `forgeryCostEstimate` is model-dependent and informative.  It:

- Assumes current hardware costs (which decrease over time).
- Does not account for novel attack techniques.
- Should not be the sole basis for high-stakes decisions.

Relying parties SHOULD treat `forgeryCostEstimate` as one input among many,
not as a guarantee.
