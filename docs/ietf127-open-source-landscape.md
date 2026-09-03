# IETF 127 CPOE Implementation Landscape

Research date: 2026-09-01

This note distinguishes implementations of the proposed CPoE wire formats from
projects that address adjacent provenance or process-evidence problems. A public
product, an open standard, and an open-source implementation are not treated as
synonyms. Repository and license claims below should be rechecked immediately
before the BoF request is submitted.

## Direct CPoE Work

| Project | Public evidence | Assessment |
| --- | --- | --- |
| [LF Decentralized Trust Proof of Effort Lab](https://github.com/LF-Decentralized-Trust-labs/proof-of-effort) | CPoE Internet-Draft sources, CDDL, examples, integration documents, and a minimal sequential-memory implementation | Primary specification repository and proponent-controlled reference work; not a complete independent implementation |
| [WritersProof](https://writerslogic.com/about/) | Public browser extension, desktop integration, `.cpoe` export, and [web verifier](https://verify.writersproof.com/); WritersLogic states that the protocol/reference implementation is Apache-2.0 | A direct implementation, but controlled by the proponent. The public GitHub link currently identifies the WritersLogic organization rather than the application's source repository, so the application source and license should be linked before submission |

No independently controlled implementation of the CPoE wire formats was found.

## Independent Process-Evidence and Attribution Projects

| Project | License/status | Relevant capability | CPoE relationship |
| --- | --- | --- | --- |
| [Tracked Writing File Format (TWFF)](https://github.com/Functional-Intelligence-Research-Lab/twff) | Apache-2.0; specification, schemas, validators, and Python Glass Box reference editor | ZIP container holding a document and deterministic log of AI interactions, paste events, revisions, and timing | Strong independent evidence of the same writing-process interoperability problem; no claimed CPoE support |
| [KeyWitness / typed.by](https://github.com/magicseth/keywitness) | README declares MIT, but the repository has no LICENSE file and GitHub reports no license; working TypeScript and Swift implementation with a live JSON-LD context at `keywitness.io/ns/v1` | iOS custom keyboard that captures per-keystroke timing, touch coordinates, contact radius, and force, hashes them, and seals the typed text as a W3C VC 2.0 credential signed with Ed25519 (`eddsa-jcs-2022`, `did:key`), with an Apple App Attest assertion and optional Face ID signature | The closest independent work to CPoE found so far, and the only one already building on W3C VC, Data Integrity, JCS, and EAT, with C2PA embedding on its roadmap. It has no trusted timestamp or transparency log, so time is self-asserted inside the signed credential; its own tracker records the Secure Enclave claim as false and the device and biometric bindings as forgeable. No claimed CPoE support |
| [Proof SDK](https://github.com/EveryInc/proof-sdk) | MIT; working TypeScript editor/server/SDK | Collaborative document editing with provenance tracking for human and agent contributions | Strong authoring and contribution-provenance use case; not a cryptographic CPoE implementation |
| [humanshipd](https://github.com/Flagrare/humanshipd) | MIT; early Rust proof of concept | Metadata-only writing record, signing, RFC 3161 anchoring, CLI verification, and planned capture adapters | Close technical overlap and an explicit threat model; its own schema and no claimed CPoE support |
| [Certified Human-Made](https://github.com/dabhunt/krita-certified-human-made) | Repository states GPL-3.0; Python/Rust Krita plug-in | Records strokes, layers, timing, imports, and tool use and produces process certificates | Demonstrates the problem outside text; no claimed CPoE support |
| [OpenFab](https://github.com/open-fab-ai/openfab) | Apache-2.0; active Rust implementation | Human/AI attribution for software, signed in-toto/SLSA provenance, and human approval gates | Adjacent software-creation domain; useful source of requirements and participants, not CPoE |
| [Art Provenance Vault](https://github.com/0thernes/art-provenance-vault) | MIT; Python proof of concept | Hash-to-manifest-to-git provenance loop for AI-assisted art | Working basic provenance loop, but signatures, watermarking, and C2PA serialization remain planned; not CPoE |
| [ScholarScribe](https://github.com/waleedmandour/scholarscribe) | MIT; Rust/Svelte desktop application | Local timestamped writing snapshots exported as JSON | Useful user need, but no cryptographic tamper evidence or CPoE support identified |

## Reusable Open-Source Provenance Infrastructure

| Project | What it supplies | Relevance and limit |
| --- | --- | --- |
| [Content Authenticity Initiative SDKs](https://opensource.contentauthenticity.org/docs/introduction/) | Rust, JavaScript, Python, C/C++, Node.js, and mobile implementations for C2PA manifests, signing, embedding, parsing, and validation | A likely carriage and verification layer; C2PA does not define CPoE's content-creation event or appraisal semantics |
| [SLSA](https://slsa.dev/) and in-toto | Software-build provenance predicates, attestations, producer/consumer roles, and assurance levels | Valuable architecture and terminology for verifiable processes; scoped to software supply chains rather than human content creation |
| [Cisco Model Provenance Kit](https://github.com/cisco-ai-defense/model-provenance-kit) | Apache-2.0 model-lineage comparison using metadata, tokenizer, and model-weight fingerprints | Demonstrates evidence-based provenance appraisal, but infers model lineage rather than recording a creation process |

## Related Systems Not Counted as Open-Source Implementations

| System | Public evidence | Why it is not counted |
| --- | --- | --- |
| [Genotone](https://genotone.com/) | Site describes a working internal C2PA-based audio registration, watermarking, fingerprinting, signing, and verification prototype, with a public registry still planned | No public, licensed source implementation was identified. It is relevant independent prototype evidence, but neither open-source nor CPoE-compatible on current evidence |
| [Khepri Proof of Effort](https://github.com/SeekingProof/Proof-of-Effort-) | Public documentation describes a LibreOffice/Word process-evidence product and sealed `.poe` artifacts | The public repository contains documentation rather than source code and declares the method patent-pending/all rights reserved |
| [punchsig](https://github.com/oreparaz/punchsig) | Public firmware and a static HTML verifier for a USB keyboard interposer that Ed25519-signs typed text; two Raspberry Pi Pico boards joined by a unidirectional serial link | The repository carries no LICENSE file, so the source is published rather than open-source. The published firmware also derives its signing key from a hardcoded seed, so current signatures have no unforgeability. It signs only the final typed buffer, with no timing, edit history, timestamp, or replay protection, and references no provenance standard |
| [NetRise Provenance](https://www.netrise.io/products/provenance) | Commercial product mapping software packages to repositories, maintainers, dependency relationships, and risk signals | No relevant public-source implementation identified; it evaluates software-component origin rather than content-creation effort |

## Relevant Practice and Relying-Party Evidence

punchsig is worth separating from the other uncounted systems because its
contribution is a trust root rather than a product. It constrains the bytes
reaching the signer to have crossed a hardware diode from a physical keyboard,
so a compromised host cannot request a signature over text the user never typed.
CPoE does not define that boundary and would benefit from a hardware
human-presence primitive underneath its capture layer. Its author is a
hardware-security researcher, which makes him a plausible reviewer of the
appraisal and capture threat model even though the project is not an
implementation of anything CPoE defines.

The [Berkeley Protocol on Digital Open Source Investigations](https://humanrights.berkeley.edu/projects/developing-the-berkeley-protocol-on-digital-open-source-investigations/)
is not a software implementation or an IETF protocol. It is important prior
practice because it establishes minimum professional standards for identifying,
collecting, preserving, verifying, and analyzing digital open-source evidence.
Those practices can inform CPoE requirements for preservation, chain of custody,
validation reporting, minimization, and ethical use.

The banking sector has also articulated a need to trace where information
originated, how it changed, which model used it, what action followed, and who
authorized it. The linked
[International Banker article](https://internationalbanker.com/technology/digital-provenance-the-chain-of-evidence-banks-need-in-the-ai-era/)
is useful relying-party context, but not implementation evidence. Its scope is
broader than the current content-creation proposal and should not expand the BoF
without confirmed financial-sector participants.

## Recommended Outreach Priority

1. KeyWitness/typed.by: an independent implementation that already emits signed
   W3C Verifiable Credentials over captured keystroke evidence, and whose
   missing pieces (trusted time, revocation enforcement, binding of device and
   biometric claims to content) are exactly the layers CPoE specifies.
2. TWFF/Glass Box and Proof SDK: strongest independent evidence of writing and
   human/agent contribution-provenance requirements.
3. humanshipd and Certified Human-Made: closest independent process-capture and
   appraisal prototypes in text and visual art.
4. OpenFab: an implementer familiar with portable signed process attestations
   and human/AI attribution in an adjacent domain.
5. Genotone: an independent audio-provenance prototype, if its founder will
   review the scope, publish technical material, or participate in the BoF.
6. CAI/C2PA, SLSA/in-toto, and Cisco Model Provenance Kit maintainers: reviewers
   of reuse boundaries rather than claimed CPoE implementers.

For the BoF case, a written statement from even two of these independent teams
that they share the interoperability problem and intend to participate is more
valuable than listing many projects without engagement.
