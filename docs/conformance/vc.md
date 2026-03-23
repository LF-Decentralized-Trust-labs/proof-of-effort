[//]: # (SPDX-License-Identifier: Apache-2.0)

# W3C Verifiable Credentials Data Model 2.0 Conformance

> **Status**: Implemented — attestation results projected as VCs

## Credential Structure

CPoP Writers Authenticity Reports can be expressed as W3C Verifiable
Credentials 2.0:

```json
{
  "@context": [
    "https://www.w3.org/ns/credentials/v2",
    "https://writerslogic.com/ns/pop/v1"
  ],
  "type": ["VerifiableCredential", "ProcessAttestationCredential"],
  "issuer": "did:web:writersproof.com",
  "validFrom": "2026-03-18T20:00:00Z",
  "credentialSubject": {
    "id": "did:key:z6Mk...",
    "type": "Author",
    "processAttestation": {
      "status": "affirming",
      "trustVector": { ... },
      "attestationTier": "hardware_bound"
    }
  },
  "evidence": [{
    "type": "ProofOfProcessEvidence",
    "verifier": "cpop-engine/1.0.3"
  }],
  "proof": {
    "type": "DataIntegrityProof",
    "cryptosuite": "eddsa-rdfc-2022",
    "verificationMethod": "did:key:z6Mk...#key-1",
    "proofPurpose": "assertionMethod"
  }
}
```

## Field Mapping

| VC Field             | CPoP Source                                  |
| -------------------- | -------------------------------------------- |
| `issuer`             | Verifier DID (`did:web:writersproof.com`)    |
| `credentialSubject.id`| Author DID (`did:key:z...`)                 |
| `processAttestation` | EAR appraisal status + trust vector          |
| `evidence[].verifier`| `ear_verifier_id.build`                      |
| `proof.cryptosuite`  | `eddsa-rdfc-2022` (Ed25519)                  |

## Privacy

- **Selective disclosure**: Verifiable Presentations can reveal verdict only
  without behavioral telemetry
- **Unlinkability**: Per-session DIDs prevent cross-session correlation
- **Data minimization**: WAR abstracts raw evidence into scores and verdicts

## References

- [W3C VC Data Model 2.0](https://www.w3.org/TR/vc-data-model-2.0/)
- [Data Integrity EdDSA Cryptosuites](https://www.w3.org/TR/vc-di-eddsa/)
