[//]: # (SPDX-License-Identifier: Apache-2.0)

# CBOR / COSE Conformance

> **Status**: Normative — CPoE wire format uses RFC 8949 (CBOR) + RFC 9052 (COSE)

## Wire Format

All CPoE structures are encoded as CBOR (RFC 8949) using deterministic
encoding (Section 4.2.1) with integer-keyed maps.

| Structure           | CBOR Tag       | Hex        | Mnemonic | Media Type                                 |
| ------------------- | -------------- | ---------- | -------- | ------------------------------------------ |
| Evidence Packet     | `1129336645`   | `0x43504F45` | `CPoE` | `application/cpoe+cbor`                    |
| Compact Evidence Ref| `1129336658`   | `0x43504F52` | `CPoR` | —                                           |
| Attestation Result  | `1129791826`   | `0x43574152` | `CWAR` | `application/vnd.writersproof.cwar+cbor`   |

File extensions: `.cpoe` (evidence), `.cwar` (attestation result).

## Signature Algorithm

| Algorithm | RFC     | Curve / Parameters           | Usage                     |
| --------- | ------- | ---------------------------- | ------------------------- |
| EdDSA     | RFC 8032| Ed25519 (edwards25519)       | Evidence + WAR signing    |
| HMAC      | RFC 2104| SHA-256                      | Event integrity, jitter seals |
| HKDF      | RFC 5869| SHA-256                      | Key derivation (jitter, entangled binding) |

CPoE uses `COSE_Sign1` for packet signatures. Ed25519 is in the C2PA
allowed algorithm list, enabling cross-standard compatibility.

## Hash Algorithms

| Algorithm | Usage                                    |
| --------- | ---------------------------------------- |
| SHA-256   | Content hashing, checkpoint chain, VDF   |
| BLAKE3    | Internal caching (not on wire)           |

## CDDL Schema

The complete wire format is defined in [`cddl/cpoe.cddl`](../../cddl/cpoe.cddl)
using CDDL (RFC 8610).

## References

- [RFC 8949 — Concise Binary Object Representation (CBOR)](https://www.rfc-editor.org/rfc/rfc8949)
- [RFC 9052 — CBOR Object Signing and Encryption (COSE)](https://www.rfc-editor.org/rfc/rfc9052)
- [RFC 8032 — Edwards-Curve Digital Signature Algorithm (EdDSA)](https://www.rfc-editor.org/rfc/rfc8032)
- [RFC 8610 — Concise Data Definition Language (CDDL)](https://www.rfc-editor.org/rfc/rfc8610)
