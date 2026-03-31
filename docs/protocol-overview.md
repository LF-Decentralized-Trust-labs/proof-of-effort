[//]: # (SPDX-License-Identifier: Apache-2.0)

# CPoP Protocol Overview

This document provides a comprehensive technical overview of the
Cryptographic Proof of Process (CPoP) protocol: what it does, how it
works, and how the pieces fit together. For the normative specification,
see the [protocol draft](../draft-condrey-cpop-protocol.md) and
[appraisal draft](../draft-condrey-cpop-appraisal.md). For the RATS
architecture mapping, see [architecture.md](architecture.md).

## Purpose

The CPoP protocol produces tamper-evident evidence that a human
authored a document through a genuine creative process, rather than
generating it with AI and back-filling fake timing data. It works by
instrumenting the authoring environment to capture behavioral
telemetry as the author types, then binding that telemetry into a
cryptographic chain that is computationally expensive to forge.

## Evidence Collection

During composition, the Attester (the authoring application)
periodically creates checkpoints. Each checkpoint captures a SHA-256
hash of the document's current state, the inter-keystroke timing
intervals (jitter binding), and a Sequential Work Function (SWF) proof
that forces real wall-clock time to pass between checkpoints. The SWF
uses Argon2id (memory-hard) computation so that an adversary cannot
pre-compute or parallelize the proofs. In entangled mode (Mode 21),
each checkpoint's SWF depends on the previous checkpoint's output,
creating a strict inter-checkpoint sequential dependency that
eliminates parallel pre-computation. Every checkpoint's hash links to
the previous one, forming an append-only causality chain similar to a
blockchain, where tampering with any checkpoint invalidates all
subsequent ones.

## Behavioral Fingerprinting

The jitter binding captures the statistical fingerprint of human motor
control. When a person types, their inter-keystroke intervals follow
characteristic patterns: pink noise (1/f) in the frequency domain, a
Hurst exponent between 0.55 and 0.85 indicating long-range
dependence, and a coefficient of variation reflecting biological motor
variance. These patterns are fundamentally different from bot-generated
timing, which tends toward either mechanical regularity (low CV) or
white noise (high CV). The protocol optionally captures physical state
markers (thermal trajectories, kernel entropy deltas, and inertial
accelerometer data) that bind the evidence to non-reproducible
environmental conditions, making replay attacks harder.

## Wire Format

The evidence is packaged as a CBOR-encoded Evidence Packet tagged with
semantic tag 1129336656 ("CPOP"). The packet contains the document
reference (content hash, not content), the checkpoint chain,
attestation tier, and optional fields for presence challenges
(QR-based out-of-band human verification), channel binding (TLS
Exported Keying Material), hardware-anchored time (TPM attestation),
and behavioral baselines. The wire format uses integer keys for
compactness and is defined in CDDL ([`cddl/cpop.cddl`](../cddl/cpop.cddl))
with strict validation rules.

## Verification and Forensic Analysis

Verification is performed by a separate Verifier that produces a
Written Authorship Report (WAR, CBOR tag 1129791826 / "CWAR"). The
Verifier checks the hash chain integrity, recomputes sampled SWF
proofs via Merkle audit paths (probabilistic verification in
O(k log n) time), independently estimates entropy from the jitter
intervals, and runs a battery of forensic mechanisms:

| Mechanism | ID | What it detects |
|-----------|----|-----------------|
| SNR Analysis | SNR | Non-biological spectral patterns (white noise vs 1/f) |
| Cognitive Load Correlation | CLC | Timing uncorrelated with semantic complexity |
| Mechanical Turk Detection | MTD | Machine-clocked editing pace independent of content |
| Error Topology | ET | Correction patterns inconsistent with human cognition |
| QR Presence Challenge | OOB-PC | Absence of physical human at the device |
| Session Consistency | SC | Abrupt behavioral shifts indicating source switching |
| Perplexity Scoring | PPX | AI-generated text insertions (low perplexity + fast typing) |
| Biological Cadence | BCA | Mechanically regular or chaotically irregular timing |
| Inertial Coherence | ICA | Digital keystrokes without corresponding physical impulses |
| Distributional Conformance (KS) | KSD | IKI distribution shape deviates from empirical human reference |
| HID Device Provenance | HDP | Generic/injection HID descriptor or polling-rate saturation |
| Inhibition Response | IRT | Failure to cease input within human reaction time on stop signal |
| Dynamic Latency Injection | DLI | No typing adaptation when visual feedback is artificially delayed |

Each mechanism belongs to an independence class (spectral, temporal,
distributional, semantic, hardware, or out-of-band). Two or more flags
from independent classes trigger a suspicious verdict.

## Assurance Tiers

The protocol operates across four assurance tiers:

- **T1 (Self-Attested)**: Software-only with an honest-but-curious
  threat model. All evidence is self-reported.
- **T2 (Corroborated)**: Multiple software signals cross-checked.
  Defeats casual forgery.
- **T3 (Hardware-Bound)**: Evidence anchored to TPM/HSM via
  Hardware-Anchored Time (HAT) proofs. Prevents time-jump attacks.
- **T4 (Independent)**: External witness infrastructure provides
  independent corroboration.

The protocol explicitly acknowledges that T1/T2 cannot defeat a root
adversary who controls the kernel. Physical state markers at those
tiers provide dimensionality (increasing what an adversary must
fabricate consistently) rather than proof. The tier system lets relying
parties make risk-appropriate decisions about how much to trust a
given piece of evidence.

## Ecosystem Integration

CPoP is designed as an IETF protocol (experimental status) with two
companion Internet-Drafts: the protocol spec (wire format, checkpoint
chain, SWF, jitter binding, physical state) and the appraisal spec
(verifier behavior, forensic mechanisms, attestation result format).
It integrates with the IETF RATS architecture (RFC 9334) as a
specialized Evidence type, and maps to C2PA (content authenticity),
Verifiable Credentials, CAWG, and EU AI Act compliance frameworks.

## Reference Implementation

The Rust implementation provides two crates:

- **[cpop-jitter](../crates/cpop-jitter/)**: `no_std`-compatible
  entropy collection with HMAC-based (pure/deterministic) and
  hardware-based (TSC/CNTVCT timing) jitter engines, an append-only
  evidence chain with HMAC integrity, and a human-model validator.
- **[cpop-protocol](../crates/cpop-protocol/)**: Wire types matching
  the CDDL schema, CBOR/JSON codec with size-limited decoding,
  evidence builder with causality locks, forensic analysis engine,
  C2PA manifest generation, and Written Authorship Report encoding.
