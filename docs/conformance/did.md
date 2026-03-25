[//]: # (SPDX-License-Identifier: Apache-2.0)

# W3C DID Core 1.0 Conformance

> **Status**: Implemented — `did:key` for self-sovereign identity, `did:web` for organizational

## DID Methods

| Method     | Usage                                    | Resolution     | Key Rotation |
| ---------- | ---------------------------------------- | -------------- | ------------ |
| `did:key`  | Author identity (Ed25519 public key)     | Deterministic  | No (new DID) |
| `did:web`  | Organizational identity (WritersProof)   | HTTPS          | Yes          |

### did:key Encoding

CPoP uses the multicodec-prefixed multibase encoding per the
[did:key Method Specification](https://w3c-ccg.github.io/did-key-spec/):

```
did:key:z{base58btc(0xed01 || raw_ed25519_pubkey)}
```

Where `0xed01` is the multicodec prefix for Ed25519 public keys.

### did:web Identifiers

- Verifier/Issuer: `did:web:writersproof.com`
- Author (API-anchored): `did:web:writersproof.com:authors:{id}`

## Verification Relationships

| Relationship           | CPoP Usage                                    |
| ---------------------- | --------------------------------------------- |
| `assertionMethod`      | Signing evidence packets and VCs (primary)    |
| `authentication`       | Session binding via key hierarchy             |
| `capabilityDelegation` | Co-author / editor delegation chains          |
| `keyAgreement`         | IPC secure channel (X25519)                   |

## Key Hierarchy

CPoP implements a three-level key hierarchy that maps to DID verification
methods:

```
Master Key (did:key:z...) ─── assertionMethod, authentication
  └── Session Key          ─── per-session, time-bounded
       └── Ratchet Key     ─── per-checkpoint, forward-secret
```

Each level derives from the previous via HKDF-SHA256.

## References

- [W3C DID Core 1.0](https://www.w3.org/TR/did-1.0/)
- [did:key Method Specification](https://w3c-ccg.github.io/did-key-spec/)
- [did:web Method Specification](https://w3c-ccg.github.io/did-method-web/)
