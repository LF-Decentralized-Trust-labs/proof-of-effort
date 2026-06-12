# W3C Harmonization Layer for Cryptographic Proof of Effort

> **Audience**: W3C Credentials Community Group (CCG) members, Verifiable
> Credential implementers, and publishers seeking machine-readable authorship
> provenance.

## Overview

The IETF Cryptographic Proof of Effort (CPoE) protocol suite produces
hardware-anchored attestation evidence that a human authored a document over
real elapsed time.  This harmonization layer bridges that low-level evidence
into the W3C Verifiable Credentials (VC) ecosystem so that a publisher,
platform, or peer reviewer can consume authorship confidence as a standard
credential.

### Why Both Standards?

IETF RATS defines how to *produce and appraise* attestation evidence (binary
CBOR, TPM-anchored, formally verified).  W3C VCs define how to *present and
trust* claims on the web (JSON-LD, DID-resolved, browser-friendly).  Neither
replaces the other:

| Concern | IETF Layer | W3C Layer |
|---------|-----------|-----------|
| Wire format | CBOR (deterministic, compact) | JSON-LD (interoperable, web-native) |
| Trust anchor | TPM AIK certificate chain | DID document + Data Integrity proof |
| Verification | SWF recomputation, HMAC checks | Signature verification, status list |
| Audience | Verifier (cryptographic appraisal) | Relying Party (editorial/publishing decision) |

This harmonization layer sits between the two: the CPoE Verifier produces a
**Written Authorship Report** (WAR) from raw evidence, and the layer wraps
that WAR into a Verifiable Credential using JSON-LD terms defined in the
[`poe.jsonld`](w3c/contexts/poe.jsonld) context.

### Document Map

| File | Purpose |
|------|---------|
| [`w3c/baa-vc-profile.md`](w3c/baa-vc-profile.md) | Technical profile: Bayesian Appraisal to W3C Confidence Method v0.9/v1.0 |
| [`w3c/contexts/poe.jsonld`](w3c/contexts/poe.jsonld) | JSON-LD context defining CPoE terms for Linked Data consumption |
| [`draft-condrey-rats-hat.md`](draft-condrey-rats-hat.md) | IETF draft: Hardware Attestation of Time (TPM temporal binding) |
| [`draft-condrey-cpoe-appraisal.md`](draft-condrey-cpoe-appraisal.md) | IETF draft: CPoE Appraisal (Written Authorship Report) |
| [`draft-condrey-cpoe-protocol.md`](draft-condrey-cpoe-protocol.md) | IETF draft: CPoE Protocol (evidence packet structure) |

## Architecture

```
                         +-----------------+
                         |   CPoE Editor   |
                         | (checkpoints,   |
                         |  jitter, HAT)   |
                         +-------+---------+
                                 |
                        Evidence Packet (CBOR)
                        tag #6.1129336645
                                 |
                         +-------v---------+
                         |  CPoE Verifier  |
                         | (appraisal per  |
                         |  cpoe-appraisal)|
                         +-------+---------+
                                 |
                  Written Authorship Report (CBOR)
                  tag #6.1129791826
                                 |
              +------------------+------------------+
              |                                     |
    +---------v-----------+           +-------------v-----------+
    | JSON-LD VC Wrapper  |           |   COSE Envelope         |
    | (Data Integrity)    |           |   (VC-JOSE-COSE)        |
    | For web/browser     |           |   For CBOR-native       |
    | consumption         |           |   consumption           |
    +---------------------+           +-------------------------+
```

Two issuance paths exist because the CPoE evidence is CBOR-native:

1. **JSON-LD path** (this profile's primary focus): WAR fields are transcoded
   to JSON-LD using the `poe.jsonld` context, signed with Data Integrity.
   Suitable for web platforms, browsers, and JSON-based APIs.

2. **COSE envelope path**: The WAR CBOR is wrapped directly in a
   `VC-JOSE-COSE` envelope without JSON-LD transcoding.  Suitable for
   IoT, embedded systems, and CBOR-native verifiers.  See
   [VC-JOSE-COSE](https://www.w3.org/TR/vc-jose-cose/) for the envelope
   specification.

## IETF Attestation Result to W3C Credential Subject Mapping

The table below shows how each field in the IETF `attestation-result` (WAR,
CBOR tag `#6.1129791826`) maps to a property in the VC `credentialSubject`.
All JSON-LD terms are defined in `poe.jsonld`.

| CBOR Key | IETF Field | Type | W3C Property | Description |
|----------|-----------|------|-------------|-------------|
| 1 | `version` | uint | `schemaVersion` | WAR schema version (MUST be 1) |
| 2 | `evidence-ref` | hash-value | `evidenceDigest` | SHA-256 of the evidence packet |
| 3 | `verdict` | enum 1-4 | `evaluationResult` | `authentic` / `inconclusive` / `suspicious` / `invalid` |
| 4 | `attestation-tier` | enum 1-4 | `assuranceLevel` | T1 (software) through T4 (hardware-hardened) |
| 5 | `chain-length` | uint | `checkpointCount` | Number of checkpoints evaluated |
| 6 | `chain-duration` | uint | `sessionDuration` | Total authoring duration in seconds |
| 7 | `entropy-report` | map | `entropicEvidence` | Timing, revision, and pause entropy scores |
| 7.1 | `timing-entropy` | float | `.timingBitsPerSample` | Bits/sample of inter-keystroke interval entropy |
| 7.2 | `revision-entropy` | float | `.revisionBits` | Bits of edit-delta size distribution entropy |
| 7.3 | `pause-entropy` | float | `.pauseBits` | Bits of inter-checkpoint pause entropy |
| 7.4 | `meets-threshold` | bool | `.meetsThreshold` | True if timing >= 3.0, revision >= 3.0, pause >= 2.0 |
| 8 | `forgery-cost-estimate` | map | `forgeryCostEstimate` | Quantified cost to forge this evidence |
| 8.1 | `c-swf` | float | `.swfCost` | SWF recomputation cost |
| 8.2 | `c-entropy` | float | `.entropyCost` | Behavioral entropy synthesis cost |
| 8.3 | `c-hardware` | float | `.hardwareCost` | Hardware attestation attack cost |
| 8.4 | `c-total` | float | `.totalCost` | Aggregate forgery cost |
| 8.5 | `cost-unit` | enum | `.costUnit` | `USD` or `cpu-hours` |
| 9 | `absence-claim[]` | array | `absenceClaims` | Negative assertions ("no bulk paste detected") |
| 10 | `warnings[]` | array | `warnings` | Verifier-generated warning strings |
| 13 | `forensic-summary` | map | `forensicAnalysis` | Anomaly detection flags and coverage |
| 13.1 | `flags-triggered` | uint | `.flagsTriggered` | Count of forensic flags that fired |
| 13.2 | `flags-evaluated` | uint | `.flagsEvaluated` | Count of forensic tests run |
| 13.3 | `affected-checkpoints` | uint | `.affectedCheckpoints` | Checkpoints with at least one flag |
| 13.4 | `total-checkpoints` | uint | `.totalCheckpoints` | Total checkpoints analyzed |
| 13.5 | `forensic-flag[]` | array | `.mechanisms[]` | Per-mechanism detail (SNR, CLC, IKI, etc.) |
| 14 | `confidence-tier` | enum 1-4 | `baselineMaturity` | Baseline behavioral model maturity |
| 15 | `effort-attribution` | map | `authorshipConfidence` | Human vs. tool authorship split |
| 15.1 | `human-fraction` | float | `.humanFraction` | 0.0-1.0 fraction of human-authored content |
| 15.2 | `human-checkpoints` | uint | `.humanCheckpoints` | Checkpoints without tool receipts |
| 15.3 | `receipt-checkpoints` | uint | `.receiptCheckpoints` | Checkpoints containing tool receipts |
| 15.4 | `tool-attributed-chars` | uint | `.toolAttributedChars` | Character count from tool output |
| 15.5 | `total-chars` | uint | `.totalChars` | Total document character count |

**Derived fields** (not directly in the WAR but computed during VC issuance):

| W3C Property | Source | Description |
|-------------|--------|-------------|
| `processMonotonicity` | HAT `resetCount` consistency | True if no TPM reboot detected across the session |
| `jitterSeal.sealed` | All `jitter-binding` HMACs valid | True if every checkpoint's behavioral entropy seal verified |
| `jitterSeal.entropyEstimateCentibits` | Sum of `jitter-binding.entropy-estimate` | Aggregate behavioral entropy in centibits |
| `confidenceMethod.confidenceScore` | Composite formula (see profile) | Single 0.0-1.0 score for threshold decisions |

## Example Verifiable Credential

```json
{
  "@context": [
    "https://www.w3.org/ns/credentials/v2",
    "https://purl.org/poe/v1"
  ],
  "type": ["VerifiableCredential", "ProofOfEffortCredential"],
  "issuer": "did:web:verifier.example.com",
  "validFrom": "2026-05-12T14:30:00Z",
  "credentialSubject": {
    "id": "urn:uuid:a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "type": "AuthorshipAttestation",
    "documentDigest": "sha256:e3b0c44298fc1c149afbf4c8996fb924...",
    "evaluationResult": "authentic",
    "assuranceLevel": "T3-hardware-bound",
    "checkpointCount": 47,
    "sessionDuration": 7200,
    "authorshipConfidence": {
      "humanFraction": 0.92,
      "humanCheckpoints": 43,
      "receiptCheckpoints": 4
    },
    "entropicEvidence": {
      "timingBitsPerSample": 4.2,
      "revisionBits": 5.1,
      "pauseBits": 3.8,
      "meetsThreshold": true
    },
    "forgeryCostEstimate": {
      "totalCost": 14200.00,
      "costUnit": "USD",
      "swfCost": 8500.00,
      "entropyCost": 3200.00,
      "hardwareCost": 2500.00
    },
    "processMonotonicity": true,
    "jitterSeal": {
      "entropyEstimateCentibits": 420,
      "sealed": true
    },
    "forensicAnalysis": {
      "flagsTriggered": 0,
      "flagsEvaluated": 5,
      "affectedCheckpoints": 0,
      "totalCheckpoints": 47
    },
    "baselineMaturity": "established",
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
  },
  "evidence": [{
    "type": "CPoEEvidencePacket",
    "digestSRI": "sha256-47DEQpj8HBSa+/TImW+5JCeuQeRkm5NMpJWZG3hSuFU=",
    "encodingFormat": "application/cbor",
    "cborTag": 1129336645,
    "profile": "urn:ietf:params:cpoe:profile:enhanced"
  }],
  "proof": {
    "type": "DataIntegrityProof",
    "cryptosuite": "eddsa-rdfc-2022",
    "verificationMethod": "did:web:verifier.example.com#key-1",
    "proofPurpose": "assertionMethod",
    "created": "2026-05-12T14:30:00Z"
  }
}
```

**Verifying the example's `confidenceScore`:**
`(0.70 + 0.05 - 0.00 + 0.10) * 0.92 = 0.85 * 0.92 = 0.782`, clamped and
rounded to `0.78`.

## Integration Guide

### For Verifiers (Credential Issuers)

1. Accept a CPoE evidence packet (CBOR, tag `#6.1129336645`).
2. Run the appraisal algorithm per `draft-condrey-cpoe-appraisal`.
3. Produce a WAR (CBOR, tag `#6.1129791826`).
4. **Issuance decision**: If `verdict = invalid`, do NOT issue a credential.
   If `verdict = suspicious`, issue with the `warnings` array populated and
   `confidenceScore` reflecting the penalty.
5. Map WAR fields to VC properties using the table above.
6. Compute `confidenceScore` per the formula in
   [`baa-vc-profile.md` Section 5.1](w3c/baa-vc-profile.md#51-confidence-score-computation).
7. Sign the VC with a Data Integrity proof (or wrap in COSE for the
   CBOR-native path).
8. Return the VC to the author or publisher.

### For Relying Parties (Publishers, Platforms)

1. Receive a `ProofOfEffortCredential`.
2. Verify the Data Integrity proof against the issuer's DID.
3. Check `evaluationResult`: reject `invalid`, flag `suspicious`.
4. Compare `confidenceMethod.confidenceScore` against your threshold
   (see [suggested thresholds](w3c/baa-vc-profile.md#53-confidence-thresholds-informative)).
5. Use `authorshipConfidence.humanFraction` for editorial decisions about
   tool-assisted content.
6. Optionally inspect `forensicAnalysis.mechanisms` for anomaly details.
7. Store or present the credential alongside published content.

### For Authors

1. Use a CPoE-enabled editor that captures checkpoints during writing.
2. Submit evidence to a trusted CPoE Verifier.
3. Receive back a Verifiable Credential.
4. Attach the credential to manuscripts, submissions, or publications.
5. The `humanFraction` will reflect tool-assisted portions honestly;
   consider whether you want to disclose this level of detail.

## Relationship to Adjacent Standards

| Standard | Relationship |
|----------|-------------|
| [W3C Confidence Method](https://w3c-ccg.github.io/confidence-method/) | `confidenceMethod` object conforms to this spec; see [profile](w3c/baa-vc-profile.md) |
| [C2PA / CAI](https://c2pa.org/) | CPoE addresses *authorship process*; C2PA addresses *content provenance*. A publisher could attach both: C2PA for media integrity, CPoE VC for authorship confidence. |
| [IETF EAT (RFC 9711)](https://www.rfc-editor.org/rfc/rfc9711) | The WAR maps to an EAT with custom claims. The VC layer adds web-native presentation. |
| [VC-JOSE-COSE](https://www.w3.org/TR/vc-jose-cose/) | Alternative envelope for CBOR-native consumption without JSON-LD transcoding. |

## Status

This harmonization layer is a **draft** aligned with:

- W3C Verifiable Credentials Data Model v2.0 (CR)
- W3C Confidence Method v0.9/v1.0 (CCG Draft)
- IETF `draft-condrey-rats-hat` (Individual Draft)
- IETF `draft-condrey-cpoe-appraisal` (Individual Draft)
- IETF `draft-condrey-cpoe-protocol` (Individual Draft)

### Namespace Note

The JSON-LD context uses `https://purl.org/poe/v1` as a placeholder namespace.
Before production deployment, register the namespace with a persistent URL
service (purl.org, w3id.org) or host it at a domain you control with proper
CORS headers and `application/ld+json` content type.

Feedback is welcome via GitHub issues or the W3C CCG mailing list.
