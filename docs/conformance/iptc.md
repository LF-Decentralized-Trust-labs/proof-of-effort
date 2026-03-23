[//]: # (SPDX-License-Identifier: Apache-2.0)

# IPTC Digital Source Type Conformance

> **Status**: Implemented — vocabulary mapping in reference implementation

## Mapping

CPoP declarations include an `AiExtent` field that maps directly to the
[IPTC Digital Source Type vocabulary](http://cv.iptc.org/newscodes/digitalsourcetype/),
used by C2PA, NIST AI 100-4, and the EU AI Act.

| CPoP AiExtent   | IPTC Digital Source Type URI                                      | W3C ai-disclosure |
| ---------------- | ----------------------------------------------------------------- | ----------------- |
| `None`           | `http://cv.iptc.org/newscodes/digitalsourcetype/humanCreation`    | `none`            |
| `Minimal`        | `http://cv.iptc.org/newscodes/digitalsourcetype/compositeWithTrainedAlgorithmicMedia` | `ai-assisted` |
| `Moderate`       | `http://cv.iptc.org/newscodes/digitalsourcetype/compositeWithTrainedAlgorithmicMedia` | `ai-assisted` |
| `Substantial`    | `http://cv.iptc.org/newscodes/digitalsourcetype/trainedAlgorithmicMedia` | `ai-generated` |

## Usage in C2PA

When projecting a CPoP attestation as a C2PA assertion, the
`digitalSourceType` field in `c2pa.actions.v2` uses the IPTC URI
from this mapping.

## References

- [IPTC Digital Source Type NewsCodes](http://cv.iptc.org/newscodes/digitalsourcetype/)
- [C2PA Specification — Actions Assertion](https://c2pa.org/specifications/specifications/2.3/specs/C2PA_Specification.html)
