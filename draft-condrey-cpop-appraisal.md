---
v: 3
docname: draft-condrey-cpop-appraisal-latest
title: "Cryptographic Proof of Process (CPoP): Forensic Appraisal and Security Model"
abbrev: CPoP Appraisal
category: exp
ipr: trust200902
submissiontype: independent
area: Security
workgroup: Individual Submission
keyword:
  - attestation
  - forensics
  - biometrics
  - security economics

stand_alone: yes
pi:
  toc: yes
  tocdepth: "4"
  sortrefs: yes
  symrefs: yes

author:
  - fullname: David Condrey
    initials: D.
    surname: Condrey
    organization: WritersLogic Inc
    abbrev: WritersLogic
    city: San Diego
    region: California
    country: United States
    email: david@writerslogic.com

normative:
  RFC8610:
  RFC8949:
  RFC9052:
  CPoP-Protocol:
    title: "Cryptographic Proof of Process (CPoP): Architecture and Evidence Format"
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026-02
    seriesinfo:
      Internet-Draft: draft-condrey-cpop-protocol-06

informative:
  RFC9106:
  RFC9334:
  Monrose2000:
    title: Keystroke dynamics as a biometric for authentication
    target: https://doi.org/10.1016/S0167-739X(99)00059-X
    author:
      - fullname: F. Monrose
        initials: F.
        surname: Monrose
      - fullname: A. Rubin
        initials: A.
        surname: Rubin
    date: 2000
  Monaco2018:
    title: "SoK: Keylogging Side Channels"
    target: https://doi.org/10.1109/SP.2018.00026
    author:
      - fullname: John V. Monaco
        initials: J.V.
        surname: Monaco
    date: 2018
    seriesinfo:
      "IEEE Symposium on Security and Privacy (SP)": "pp. 211-228"
  Dhakal2018:
    title: "Observations on Typing from 136 Million Keystrokes"
    target: https://doi.org/10.1145/3173574.3174220
    author:
      - fullname: Vivek Dhakal
        initials: V.
        surname: Dhakal
      - fullname: Anna Maria Feit
        initials: A.M.
        surname: Feit
      - fullname: Per Ola Kristensson
        initials: P.O.
        surname: Kristensson
      - fullname: Antti Oulasvirta
        initials: A.
        surname: Oulasvirta
    date: 2018
    seriesinfo:
      "ACM CHI": "2018"
  ScholaWrite:
    title: "ScholaWrite: A Dataset of End-to-End Scholarly Writing Process"
    target: https://arxiv.org/abs/2502.02904
    author:
      - fullname: Linghe Wang
        initials: L.
        surname: Wang
      - fullname: Minhwa Lee
        initials: M.
        surname: Lee
      - fullname: Ross Volkov
        initials: R.
        surname: Volkov
      - fullname: Luan Tuyen Chau
        initials: L.T.
        surname: Chau
      - fullname: Dongyeop Kang
        initials: D.
        surname: Kang
    date: 2025
    seriesinfo:
      arXiv: "2502.02904"
  ScholaWriteAugmented:
    title: "ScholaWrite-Augmented: Revision-Tracked Scholarly Writing Dataset with Annotated External Insertion Events"
    target: https://github.com/writerslogic/scholawrite-augmented
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      GitHub: "writerslogic/scholawrite-augmented"
  HaberStornetta1991:
    title: "How to Time-Stamp a Digital Document"
    target: https://doi.org/10.1007/BF00196791
    author:
      - fullname: Stuart Haber
        initials: S.
        surname: Haber
      - fullname: W. Scott Stornetta
        initials: W.S.
        surname: Stornetta
    date: 1991
    seriesinfo:
      "Journal of Cryptology": "3(2), 99-111"
  Condrey2026Attack:
    title: "On the Insecurity of Keystroke-Based AI Authorship Detection: Timing-Forgery Attacks Against Motor-Signal Verification"
    target: https://arxiv.org/abs/2601.17280
    author:
      - fullname: David Condrey
        initials: D.
        surname: Condrey
    date: 2026
    seriesinfo:
      arXiv: "2601.17280"
  Liang2009:
    title: "Effects of Input Methods on Inter-Key Press Intervals During Continuous Typing"
    target: https://doi.org/10.1080/00140130902919121
    author:
      - fullname: Hsieh-Wen Liang
        initials: H.-W.
        surname: Liang
      - fullname: Yu-Hung Hwang
        initials: Y.-H.
        surname: Hwang
      - fullname: Fang-Hsin Chang
        initials: F.-H.
        surname: Chang
    date: 2009
    seriesinfo:
      Ergonomics: "52(9), 1153-1161"
  WangZhai2001:
    title: "Chinese Input with Keyboard and Eye-Tracking: An Anatomical Study"
    target: https://doi.org/10.1145/365024.365298
    author:
      - fullname: Jingtao Wang
        initials: J.
        surname: Wang
      - fullname: Shumin Zhai
        initials: S.
        surname: Zhai
      - fullname: Hui Su
        initials: H.
        surname: Su
    date: 2001
    seriesinfo:
      "ACM CHI": "2001"
  Shangguan2021:
    title: "Dissecting User-Perceived Latency of On-Device E2E Speech Recognition"
    target: https://doi.org/10.21437/Interspeech.2021-81
    author:
      - fullname: Yuan Shangguan
        initials: Y.
        surname: Shangguan
      - fullname: Kate Knister
        initials: K.
        surname: Knister
      - fullname: Yanzhang He
        initials: Y.
        surname: He
      - fullname: Ian McGraw
        initials: I.
        surname: McGraw
    date: 2021
    seriesinfo:
      INTERSPEECH: "2021"
  Gilden2001:
    title: "Cognitive Emissions of 1/f Noise"
    target: https://doi.org/10.1037/0033-295X.108.1.33
    author:
      - fullname: David L. Gilden
        initials: D.L.
        surname: Gilden
    date: 2001
    seriesinfo:
      "Psychological Review": "108(1), 33-56"
  Kello2007:
    title: "The Emergent Coordination of Cognitive Function"
    target: https://doi.org/10.1037/0096-3445.136.4.551
    author:
      - fullname: Christopher T. Kello
        initials: C.T.
        surname: Kello
      - fullname: Brandon C. Beltz
        initials: B.C.
        surname: Beltz
      - fullname: John G. Holden
        initials: J.G.
        surname: Holden
      - fullname: Guy C. Van Orden
        initials: G.C.
        surname: Van Orden
    date: 2007
    seriesinfo:
      "Journal of Experimental Psychology: General": "136(4), 551-568"
  Adams1961:
    title: "The Second Facet of Forgetting: A Review of Warmup Decrement"
    target: https://doi.org/10.1037/h0044798
    author:
      - fullname: Jack A. Adams
        initials: J.A.
        surname: Adams
    date: 1961
    seriesinfo:
      "Psychological Bulletin": "58(4), 257-273"
  Gentner1983:
    title: "Keystroke Timing in Transcription Typing"
    target: https://doi.org/10.1007/978-1-4612-5470-6_5
    author:
      - fullname: Donald R. Gentner
        initials: D.R.
        surname: Gentner
    date: 1983
    seriesinfo:
      "Cognitive Aspects of Skilled Typewriting": "pp. 95-120"
  Majaranta2009:
    title: "Fast Gaze Typing with an Adjustable Dwell Time"
    target: https://doi.org/10.1145/1518701.1518758
    author:
      - fullname: Paiivi Majaranta
        initials: P.
        surname: Majaranta
      - fullname: Ulla-Kaija Ahola
        initials: U.-K.
        surname: Ahola
      - fullname: Oleg Spakov
        initials: O.
        surname: Spakov
    date: 2009
    seriesinfo:
      "ACM CHI": "2009"
  Karat1999:
    title: "Patterns of Entry and Correction in Large Vocabulary Continuous Speech Recognition Systems"
    target: https://doi.org/10.1145/302979.303160
    author:
      - fullname: Clare-Marie Karat
        initials: C.-M.
        surname: Karat
      - fullname: Christine Halverson
        initials: C.
        surname: Halverson
      - fullname: Daniel Horn
        initials: D.
        surname: Horn
      - fullname: John Karat
        initials: J.
        surname: Karat
    date: 1999
    seriesinfo:
      "ACM CHI": "1999"
  Takens1981:
    title: "Detecting Strange Attractors in Turbulence"
    target: https://doi.org/10.1007/BFb0091924
    author:
      - fullname: Floris Takens
        initials: F.
        surname: Takens
    date: 1981
    seriesinfo:
      "Lecture Notes in Mathematics": "898, 366-381"
  Orden2003:
    title: "Self-Organization of Cognitive Performance"
    target: https://doi.org/10.1037/0096-3445.132.3.331
    author:
      - fullname: Guy C. Van Orden
        initials: G.C.
        surname: Van Orden
      - fullname: John G. Holden
        initials: J.G.
        surname: Holden
      - fullname: Michael T. Turvey
        initials: M.T.
        surname: Turvey
    date: 2003
    seriesinfo:
      "Journal of Experimental Psychology: General": "132(3), 331-350"
  Salthouse1986:
    title: "Perceptual, Cognitive, and Motoric Aspects of Transcription Typing"
    target: https://doi.org/10.1037/0033-2909.99.3.303
    author:
      - fullname: Timothy A. Salthouse
        initials: T.A.
        surname: Salthouse
    date: 1986
    seriesinfo:
      "Psychological Bulletin": "99(3), 303-319"
  Sardar-RATS:
    title: Security Considerations for Remote ATtestation procedureS (RATS)
    target: https://datatracker.ietf.org/doc/html/draft-sardar-rats-sec-cons-02
    author:
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
    date: 2026-02
    seriesinfo:
      Internet-Draft: draft-sardar-rats-sec-cons-02
  RFC9711:
  EAR:
    title: "EAT Attestation Results"
    target: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ear-02
    author:
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Eric Voit
        initials: E.
        surname: Voit
      - fullname: Sergei Trofimov
        initials: S.
        surname: Trofimov
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-ietf-rats-ear-02
  AR4SI:
    title: "Attestation Results for Secure Interactions"
    target: https://datatracker.ietf.org/doc/html/draft-ietf-rats-ar4si-09
    author:
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Eric Voit
        initials: E.
        surname: Voit
      - fullname: Henk Birkholz
        initials: H.
        surname: Birkholz
    date: 2025-08
    seriesinfo:
      Internet-Draft: draft-ietf-rats-ar4si-09
  SEAT-EXPAT:
    title: Remote Attestation with Exported Authenticators
    target: https://datatracker.ietf.org/doc/html/draft-fossati-seat-expat-01
    author:
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Tirumaleswar Reddy
        initials: T.
        surname: Reddy
      - fullname: Yaron Sheffer
        initials: Y.
        surname: Sheffer
      - fullname: Hannes Tschofenig
        initials: H.
        surname: Tschofenig
      - fullname: Ionut Mihalcea
        initials: I.
        surname: Mihalcea
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-fossati-seat-expat-01
  SEAT-UseCases:
    title: Use Cases and Properties for Integrating Remote Attestation with Secure Channel Protocols
    target: https://datatracker.ietf.org/doc/html/draft-mihalcea-seat-use-cases-01
    author:
      - fullname: Ionut Mihalcea
        initials: I.
        surname: Mihalcea
      - fullname: Muhammad Usama Sardar
        initials: M.U.
        surname: Sardar
      - fullname: Thomas Fossati
        initials: T.
        surname: Fossati
      - fullname: Tirumaleswar Reddy
        initials: T.
        surname: Reddy
      - fullname: Yuning Jiang
        initials: Y.
        surname: Jiang
      - fullname: Meiling Chen
        initials: M.
        surname: Chen
    date: 2026-01
    seriesinfo:
      Internet-Draft: draft-mihalcea-seat-use-cases-01

--- abstract

This document specifies the forensic appraisal methodology and quantitative security model for the Cryptographic Proof of Process (CPoP) framework defined in the companion protocol document. It defines the Verifier's evaluation of behavioral entropy, forgery cost bounds, and the Cryptographic Writers Authenticity Report (WAR) wire format. It is intended for implementers of CPoP Verifier components.

--- to_be_removed_note_Discussion_Venues

Source for this draft and an issue tracker can be found at
<https://github.com/writerslogic/draft-condrey-cpop>.

--- middle

# Introduction {#introduction}

The value of Cryptographic Proof of Process (CPoP) evidence lies in the Verifier's ability to distinguish biological effort from algorithmic simulation. While traditional RATS {{RFC9334}} appraisals verify system state, CPoP appraisal verifies a continuous physical process. This document provides the normative framework for forensic appraisal, defining the logic required to generate a Cryptographic Writers Authenticity Report (WAR).

This document is a companion to {{CPoP-Protocol}},
which defines the Evidence Packet wire format and Attester
procedures. The present document specifies the Verifier's appraisal
logic, Attestation Result (WAR) wire format, and forensic
methodology. Implementers of Verifier components require both
documents.

At T3/T4 attestation tiers, platform integrity verification as described in the SEAT use cases {{SEAT-UseCases}} provides the trust anchor for CPoP's hardware-bound claims. When CPoP Evidence is delivered over an attested TLS channel {{SEAT-EXPAT}}, the Verifier gains assurance that the Attesting Environment's platform was trustworthy during evidence generation.

# Terminology {#terminology}

This document uses the following terms in addition to those defined
in {{RFC9334}} and {{CPoP-Protocol}}:

Synthetic Authoring:
: Content generated by AI or automated tools that is subsequently attributed to a human author.

Evidence Quantization:
: The process of reducing timing resolution in behavioral data to protect author privacy while maintaining forensic utility.

IKI (Inter-Keystroke Interval):
: The time elapsed between consecutive keystrokes, measured in milliseconds.

C_intra:
: Pearson correlation between pause duration and subsequent edit complexity within a single checkpoint interval. Values near 0.0 indicate robotic pacing; values above 0.3 indicate human-like variable effort.

CLC (Cognitive Load Correlation):
: Statistical correlation between content semantic complexity and typing cadence, used to distinguish original composition from retyping.

SNR (Signal-to-Noise Ratio) Analysis:
: Spectral analysis of jitter intervals to distinguish biological motor noise patterns from synthetic injection.

Forensic Flag:
: An indicator triggered when a specific forensic assessment mechanism detects anomalous behavior in the Evidence Packet. Multiple flags inform the composite verdict assignment.

Verdict:
: The appraisal outcome assigned by the Verifier to an Evidence Packet: authentic (1), inconclusive (2), suspicious (3), or invalid (4). Defined in {{verdict-assignment}}.

Effort Attribution:
: A quantitative breakdown of human versus tool-assisted content contribution within an Evidence Packet, expressed as a human-fraction and per-checkpoint counts. Defined in {{effort-attribution}}.

Composite Assessment:
: The multi-flag evaluation procedure that determines whether forensic anomalies warrant elevation to a suspicious verdict. Defined in {{forensic-assessment}}.

# Requirements Language {#requirements-language}

{::boilerplate bcp14-tagged}

# Step-by-Step Verification Procedure {#verification-procedure}

A Verifier MUST perform the following procedure to appraise a CPoP Evidence Packet:

1. *Structural Validation:* The Verifier MUST reject with verdict invalid (4) any Evidence Packet that: (a) fails CBOR decoding, (b) lacks CBOR tag 1129336656, (c) has version != 1, (d) is missing mandatory fields (keys 1-6 in evidence-packet, keys 1-9 in each checkpoint), or (e) contains CBOR types that do not match the CDDL schema.

2. *Chain Integrity:* Verify the hash link (using the Evidence Packet's selected hash function H, as defined in {{CPoP-Protocol}}) between all checkpoints. For continuation packets (previous-packet-ref present), additionally verify that the first checkpoint's prev-hash equals the final checkpoint-hash of the preceding Evidence Packet and that checkpoint sequence numbers are globally monotonic across the series. Any break invalidates the entire Evidence Packet (or series). The Verifier MUST set the verdict to invalid (4). The warnings field SHOULD include the checkpoint sequence number where the break was detected.

3. *Temporal Order:* For each process-proof, recompute Argon2id from the declared seed to obtain state_0, then verify sampled Merkle proofs against the committed root (process-proof key 4, merkle-root). Verify that claimed-duration is within \[0.5x, 3.0x\] of the expected wall-clock time for the declared proof-params on reference hardware. Reference hardware is defined as a system with DDR4-2400 memory providing 25 GB/s peak bandwidth at 100% utilization with no OS scheduling contention. Verifiers operating on different hardware MUST apply a correction factor proportional to measured memory bandwidth. The \[0.5x, 3.0x\] tolerance range accommodates typical hardware variation. Expected times are defined in {{CPoP-Protocol}}, Mandatory SWF Parameters section. When comparing timestamps across devices or against external time sources, the clock skew tolerances defined in {{clock-skew-tolerance}} MUST be applied.

4. *Entropy Threshold:* Independently estimate entropy from the jitter-binding intervals array using a standard entropy estimator (e.g., NIST SP 800-90B most common value estimator). Verify the independent estimate meets or exceeds 3.0 bits per inter-keystroke interval {{Dhakal2018}}. The Attester's self-reported entropy-estimate field MUST NOT be relied upon. Low-entropy segments (below threshold) MUST be flagged as "Non-Biological."

5. *Entanglement:* Verify the HMAC value (entangled-binding) over the combined document, jitter, and physical state.

6. *State Matching:* Verify that the final checkpoint's content-hash matches the document-ref content-hash. Verify that the cumulative char-count from edit-deltas is consistent with the document-ref char-count.

7. *Channel Binding:* If the Evidence Packet contains a channel-binding field and was received over TLS, verify that the binding-value matches the locally-computed TLS Exported Keying Material. The Verifier MUST reject the Evidence Packet on mismatch. If channel binding verification fails, the Verifier MUST assign the verdict invalid (4).

Steps 4 and 5 apply only when jitter-binding and entangled-binding fields
are present (ENHANCED and MAXIMUM profiles). For CORE Evidence Packets
lacking these fields, the Verifier MUST skip Steps 4 and 5 and note
in the WAR warnings that behavioral analysis was not performed.

8. *Profile Matching:* If the Verifier's policy requires ENHANCED or MAXIMUM evidence but the Evidence Packet provides only CORE content tier, the Verifier MUST assign the verdict inconclusive (2) and include a warning indicating the content tier mismatch.

When appraising a series of continuation packets (linked via
previous-packet-ref as defined in {{CPoP-Protocol}}),
the Verifier MUST apply the above steps to each packet
individually and additionally verify cross-packet linkage:
the previous-packet-ref hash MUST match the complete
CBOR-encoded preceding packet, document-ref MUST be identical
across all packets, and checkpoint sequence numbers MUST be
globally monotonic. The chain-length and chain-duration fields
in the Attestation Result SHOULD reflect the aggregate totals
across the entire series. Forensic assessment (Session
Consistency Analysis in particular) SHOULD be evaluated across
packet boundaries to detect behavioral discontinuities at
rollover points.

## Clock Skew Tolerance {#clock-skew-tolerance}

Distributed CPoP deployments involve timestamp comparisons across
independently-clocked devices (Attester, Verifier, OOB presence
challenge device). Because no two clocks are perfectly
synchronized, Verifiers MUST apply the following tolerances
when evaluating temporal evidence:

Intra-chain monotonicity (same device):
: Checkpoint timestamps within a single Evidence Packet
  originate from the same local clock. The Verifier MUST
  require strict monotonic ordering (each timestamp strictly
  greater than its predecessor). No skew tolerance is applied;
  any non-monotonic timestamp indicates tampering or a broken
  implementation and MUST result in verdict invalid (4).

Cross-device validation (OOB-PC):
: When evaluating presence-challenge response-time values
  against the corresponding checkpoint's time window, the
  Verifier MUST allow a maximum clock skew tolerance of
  ±5000 milliseconds (±5 seconds) between the Attester's
  local clock and the secondary device's clock. A
  response-time that falls outside the checkpoint's time
  window but within the ±5000 ms tolerance MUST NOT be
  treated as a failure. Deployments using tightly
  synchronized time sources (e.g., GPS-disciplined clocks,
  PTP) MAY reduce this tolerance to ±1000 milliseconds by
  local policy.

Attester-to-Verifier comparison:
: The Verifier MUST NOT reject Evidence solely because
  Attester-generated pop-timestamp values differ from the
  Verifier's own wall clock. Temporal validation is primarily
  relative: the Verifier checks intra-chain ordering,
  duration plausibility, and cross-checkpoint consistency. If
  the Verifier independently compares Evidence timestamps
  against an external time source (e.g., a trusted timestamp
  authority or the Verifier's NTP-synchronized clock), a
  maximum skew tolerance of ±5000 milliseconds SHOULD be
  applied. Evidence exceeding this tolerance SHOULD generate
  a warning in the Attestation Result but MUST NOT
  automatically invalidate the Evidence.

The ±5000 ms default accommodates NTP variance across consumer
devices, including VM clock steering and mobile sleep/wake
cycles. Implementations targeting environments with known poor
clock discipline SHOULD document their tolerance requirements
in the profile-uri metadata.

# Forensic Assessment Mechanisms {#forensic-assessment}

The appraisal logic is designed to detect "Synthetic Authoring" -- content generated by AI and subsequently "back-filled" with timing and hardware attestation.

NOTE: The specific numeric thresholds in this section (e.g.,
spectral flatness 0.9, CLC metric r < 0.2, CoV bounds)
are initial calibration values derived from empirical
observation of human authoring behavior {{ScholaWrite}}. Refinement of these
values based on implementation experience is an explicit goal
of the Experimental status of this specification (see
{{experimental-status-rationale}}). The MUST-level flagging
requirements ensure interoperable Verifier behavior for
initial deployments; individual flags do not determine the
verdict (see the composite assessment procedure below).

SNR (Signal-to-Noise Ratio) Analysis:
: Verifiers MUST compute the power spectral density of jitter intervals. Human motor signals exhibit characteristic noise patterns consistent with biological motor control {{Monrose2000}}. Evidence exhibiting spectral flatness greater than 0.9 (indicating white noise rather than biological 1/f-like noise {{Gilden2001}}{{Kello2007}}) MUST be flagged as potentially synthetic. Spectral flatness is computed as the ratio of the geometric mean to the arithmetic mean of the power spectral density (PSD) of the inter-keystroke interval sequence: SF = exp(mean(ln(PSD))) / mean(PSD). The PSD MUST be estimated using Welch's method with a minimum window of 64 samples and 50% overlap. A spectral flatness value approaching 1.0 indicates white noise (synthetic); values below 0.5 are characteristic of natural 1/f-like human timing.

Cognitive Load Correlation (CLC):
: Verifiers MUST correlate timing patterns with semantic complexity. Human authors exhibit increased inter-keystroke intervals (IKI) and pause frequency during composition of semantically complex segments compared to simple connective text {{Dhakal2018}}. Verifiers MUST compute the Pearson correlation between segment semantic complexity and mean IKI. Evidence with r < 0.2 (or r < 0.1 in assistive mode) MUST be flagged as a Semantic Mismatch. Semantic complexity per checkpoint is estimated as the normalized compression ratio of the inserted text: complexity = 1 - (compressed_length / raw_length), where compression uses the DEFLATE algorithm (RFC 1951) on the UTF-8 encoded insertion. Higher values indicate more complex, less repetitive content. The CLC metric coefficient r is computed as Pearson's correlation between per-checkpoint semantic complexity and per-checkpoint mean inter-keystroke interval across all checkpoints in the session.

Mechanical Turk Detection:
: Verifiers MUST compute C_intra (Pearson correlation between pause duration and subsequent edit complexity within each checkpoint). C_intra is computed per checkpoint as: C_intra = standard_deviation(IKI) / mean(IKI), where IKI is the set of inter-keystroke intervals within that checkpoint. Pause duration refers to inter-keystroke intervals exceeding 500 milliseconds. Edit complexity is the character-level Levenshtein distance between the document state at checkpoint start and checkpoint end, normalized by checkpoint character count. C_intra values below 0.15 MUST be flagged as indicating robotic pacing, where an automated system maintains a machine-clocked editing rate independent of content demands.{{Monrose2000}}{{Monaco2018}} Checkpoints containing receipt structures (key 13) MUST have their associated paste events excluded from C_intra computation.

Error Topology Analysis:
: Verifiers SHOULD analyze error patterns for consistency with human cognitive processing {{Salthouse1986}}: localized corrections near recent insertions, fractal self-similarity in revision patterns, and deletion-to-insertion ratios consistent with natural composition. Evidence exhibiting unnaturally low error rates (below 1 correction per 500 characters {{Dhakal2018}}) or randomly distributed errors lacking positional correlation SHOULD be flagged.{{ScholaWriteAugmented}}

QR Presence Challenge (OOB-PC):
: When presence-challenge structures are present in the Evidence Packet, Verifiers MUST verify that the response-time is within the corresponding checkpoint's time window (subject to the cross-device clock skew tolerance defined in {{clock-skew-tolerance}}) and MUST validate the device-signature. NOTE: The Attester-side procedure for issuing presence challenges is specified in {{CPoP-Protocol}}.

Session Consistency Analysis:
: Verifiers MUST analyze cross-checkpoint behavioral trends. IKI distributions should exhibit gradual drift consistent with fatigue effects {{Dhakal2018}}. An abrupt change is defined as a shift in mean IKI between consecutive checkpoints exceeding 2 standard deviations of the session-wide IKI distribution. Verifiers MUST flag transitions exceeding this threshold as potential data source switching. Jitter-binding intervals across consecutive checkpoints MUST be checked for statistical independence (cross-checkpoint correlation below 0.3). Edit-delta patterns SHOULD be checked for non-stationarity consistent with human creative flow.

Spatial-Temporal Divergence (Informative):
: Advanced Verifiers MAY project the 1D `jitter-binding.intervals` array into an estimated 8-zone spatial keyboard model. By binning inter-keystroke intervals (IKI) into histograms representing spatial travel distances, Verifiers can compute the Kullback-Leibler (KL) divergence against biological baseline distributions. High KL divergence indicates structural timing anomalies invisible to raw entropy checks.

Perplexity Scoring:
: Verifiers that support a reference language model MUST compute per-checkpoint perplexity over characters inserted during that checkpoint only. Verifiers without access to a reference language model MUST omit the perplexity flag from forensic-summary and MUST NOT treat its absence as a verification failure. Verifiers SHOULD compute the character-level perplexity of text inserted during each checkpoint interval using a reference language model. Sudden drops in perplexity (highly predictable text) that correlate with rapid insertion rates (characters per second exceeding the session's 95th percentile) MUST be flagged as potential AI injection. A perplexity drop exceeding 50% relative to the session median, sustained for more than 100 consecutive inserted characters, constitutes a flag. Checkpoints containing receipt structures (key 13) MUST have their associated paste events excluded from perplexity scoring.

Biological Cadence Analysis:
: Verifiers MUST compute the Coefficient of Variation (CoV = standard deviation / mean) of inter-keystroke intervals within each checkpoint. Human typing exhibits characteristic CoV values reflecting biological motor variance. Evidence with per-checkpoint CoV consistently below 0.15 (mechanically regular) MUST be flagged as potentially non-biological.{{Monrose2000}}{{Dhakal2018}} Evidence with per-checkpoint CoV consistently above 0.90 (chaotically irregular) SHOULD be flagged as potentially injected random noise. The session-wide CoV trend SHOULD exhibit gradual drift consistent with fatigue and warm-up effects {{Adams1961}}.

A conforming Verifier MUST evaluate all forensic mechanisms for
which the Evidence Packet contains sufficient data. Verifiers
MAY implement additional analysis mechanisms beyond those
defined in this specification.

Individual forensic flags MUST be recorded in the
forensic-summary structure and reported as warnings in the
Attestation Result, but a single triggered flag alone is not
sufficient to assign the suspicious verdict. To account for
legitimate human variance (e.g., pauses for thought, fatigue,
environmental interruptions), the Verifier MUST apply the
following composite assessment before elevating to a suspicious
verdict:

1. *Multi-flag threshold:* If two or more independent
   forensic mechanisms trigger flags, the Verifier MUST assign
   the suspicious verdict regardless of checkpoint coverage.
   A contradiction arises when two or more forensic mechanisms
   produce opposing assessments of the same behavioral evidence
   (e.g., one mechanism flags synthetic timing while another
   confirms natural revision patterns). When contradictory
   results occur, the Verifier MUST assign the more conservative
   verdict. Specifically: if any mechanism triggers a flag, the
   flag stands regardless of non-triggering mechanisms.
   Non-triggering mechanisms do not cancel triggered flags.

2. *Sustained single-flag threshold:* If exactly one
   forensic mechanism triggers, the Verifier MUST assign the
   suspicious verdict only when the flag is sustained across
   more than 30% of evaluated checkpoints. A flag affecting
   30% or fewer of checkpoints MUST be reported as a warning
   but MUST NOT elevate the verdict beyond authentic.

The 30% threshold reflects the observation that transient
anomalies (single low-entropy windows, brief mechanical pacing
during copy-paste sequences) are common in legitimate authoring
sessions, while systematic anomalies spanning a substantial
portion of the session are strongly indicative of synthetic
generation.

## State Space Reconstruction (Informative) {#state-space-reconstruction}

Advanced Verifiers MAY apply Takens' embedding theorem to
reconstruct the dynamical attractor of the author's motor
noise from the scalar time series
(`jitter-binding.intervals`). By embedding the 1D
interval sequence into a higher-dimensional phase space using
time-delay coordinates, the Verifier can estimate the
correlation dimension of the resulting trajectory. Biological
motor control processes typically produce a low-dimensional
attractor (dimension 2-5), reflecting the finite degrees of
freedom in the neuromuscular system {{Takens1981}}{{Orden2003}}. Synthetic or AI-injected
timing noise will typically fail to produce a low-dimensional
biological attractor in the embedded space, instead exhibiting
high or non-convergent correlation dimension estimates. This
provides a high-confidence signal for adversarial probing that
is complementary to the spectral (SNR) and distributional (KL
divergence) checks.

## SNR Computation (Informative) {#snr-computation}

The signal-to-noise ratio measures productive editing activity versus
idle or mechanical noise within each evidence window:

~~~ artwork
SNR = 10 * log10(P_signal / P_noise)

where:
  P_signal = (keystroke_count + revision_count) / window_duration
  P_noise  = (pause_total_ms + idle_intervals) / window_duration
~~~

Typical ranges observed in human authorship:

* Human sessions: -3 dB to +12 dB, with variation reflecting
  cognitive processing cycles.
* Automated input (copy-paste, scripted typing): consistently
  above +15 dB due to minimal pause behavior.
* Sessions above +20 dB across all windows should be flagged
  as potentially non-human.
* Sessions below -10 dB across all windows indicate predominantly
  idle behavior and should be flagged as potentially fabricated
  padding.

The Verifier should compute per-window SNR and session-wide SNR
statistics (mean, variance, trend) as forensic indicators.

## Compositional Divergence and IKI Computation (Informative) {#clc-computation}

The Compositional Divergence Rate (CDR) measures the rate at
which writing complexity evolves over the session, analogous to
Lyapunov exponents in dynamical systems:

~~~ artwork
CDR = (1/n) * sum_{i=1}^{n} ln(|delta_IKI[i]| / |delta_IKI[i-1]|)

where:
  delta_IKI[i] = IKI_mean[i] - IKI_mean[i-1]
  n = number of consecutive window pairs
~~~

The Incremental Kolmogorov Information (IKI) measures informational
complexity added per window:

~~~ artwork
IKI[i] ~= compressed_size(delta_content[i]) / raw_size(delta_content[i])
~~~

Typical ranges: human authorship exhibits positive CLC values
(0.01 to 0.5) reflecting natural creative divergence. CLC near
zero indicates mechanical regularity. IKI values for human writing
typically range from 0.3 to 0.8; values consistently near 1.0
suggest random content insertion, values near 0.0 suggest verbatim
copying.

## Mechanical Turk Scoring (Informative) {#turk-scoring}

Indicators of mechanical turk behavior include:

* Paste-to-keystroke ratio exceeding 0.7 across a session.
* Burst insertion: more than 200 characters appearing in under
  2 seconds, characteristic of clipboard paste operations.
* Low IKI variance: pasted content with uniformly high
  compressibility (IKI below 0.2), consistent with LLM-generated
  prose.
* Absence of cognitive pause patterns before and after complex
  sentences.
* Temporal clustering: paste events at regular intervals suggesting
  a prompt-copy-paste workflow.

Not all paste events indicate mechanical turk behavior.
Authors who compose content in one application and transfer
it to another (e.g., drafting in a text editor and pasting
into a typesetting or markup system) produce a distinct
pattern: one or few paste events concentrated at session
start, followed by sustained editing (reformatting, adding
markup, inserting citations). Verifiers should distinguish
this cross-tool composition pattern from prompt-copy-paste
workflows by evaluating whether: (1) paste events are
concentrated at session start rather than distributed
throughout; (2) post-paste editing exhibits normal behavioral
signals (IKI variance, CLC metric, error topology
consistent with human editing); and (3) the session's
paste-to-keystroke ratio reflects a single transfer event
rather than repeated injections. When cross-tool composition
is indicated, Verifiers should exclude the initial transfer
from the mechanical turk score. When the paste event is
accompanied by a receipt structure (checkpoint key 13),
the Verifier can cryptographically verify the provenance of
the pasted content as described in
{{tool-receipt-protocol}}.

Verifiers should compute a mechanical turk probability score
from 0.0 (no indicators) to 1.0 (all indicators present). A
score exceeding 0.6 should trigger a recommendation for tool
receipt documentation.

## Error Topology Model (Informative) {#error-topology-model}

Error topology analysis constructs a directed graph of error and
correction patterns. The error graph G = (V, E) has vertices V
representing edit operations and edges E representing temporal
succession. Human error topology exhibits:

* Power-law distribution of error cluster sizes.
* Short-range temporal locality (errors corrected within 5
  seconds).
* Increasing error rates at cognitive load boundaries (end of
  paragraphs, section transitions).
* Fractal self-similarity in revision patterns.

Simulated error injection produces uniform error distribution,
regular correction intervals, and no correlation between error
rates and structural boundaries. A graph clustering coefficient
below 0.1 combined with uniform correction latency is flagged
as potentially synthetic.

## Perplexity Scoring (Informative) {#perplexity-scoring}

Character-level perplexity measures how predictable inserted
text is under a reference language model. Lower perplexity
indicates more predictable (template-like or AI-generated)
text:

~~~ artwork
PPL = exp( -(1/N) * sum_{i=1}^{N} ln P(c_i | c_1..c_{i-1}) )

where:
  N = number of characters in the insertion segment
  P(c_i | ...) = conditional probability from reference model
~~~

The Verifier computes per-checkpoint perplexity for inserted
text segments and compares against the session median. A
detection event is triggered when:

* The checkpoint's perplexity drops below 50% of the
  session median perplexity, AND
* The insertion rate exceeds the session's 95th percentile
  (characters per second), AND
* The low-perplexity segment spans more than 100
  consecutive inserted characters.

All three conditions must hold simultaneously to trigger
the flag. Paste events accompanied by receipt structures
(self-receipt or tool-receipt) are excluded from this
analysis, as they represent documented composition rather
than undisclosed AI injection.

The choice of reference language model is a Verifier
implementation decision and is not specified by this
document. Verifiers should use a model trained on general
prose to avoid domain-specific bias. The reference model
should not be disclosed to the Attester to prevent adversarial
tuning.

## Biological Cadence Analysis (Informative) {#cadence-analysis}

The Coefficient of Variation (CoV) of inter-keystroke intervals
provides a scalar measure of typing rhythm regularity:

~~~ artwork
CoV = sigma_IKI / mu_IKI

where:
  sigma_IKI = standard deviation of IKI within the window
  mu_IKI    = mean IKI within the window
~~~

Observed CoV ranges for different input sources:

* Human typing (composition): CoV 0.25-0.70, reflecting
  natural motor variance, micro-pauses for word selection,
  and variable key travel distances {{Dhakal2018}}{{Salthouse1986}}.
* Human typing (transcription): CoV 0.15-0.40 {{Gentner1983}}, more
  regular due to reduced cognitive load but still
  biologically variable.
* Automated injection (scripted replay): CoV below 0.10,
  mechanically uniform timing from software-driven
  keystroke injection.
* Synthetic noise (random jitter): CoV above 0.90,
  lacking the structured variability of biological motor
  control.

The Verifier should compute per-checkpoint CoV and track
the session-wide CoV trajectory. A monotonically decreasing
CoV trend across the session is consistent with motor
warm-up effects in human typing. Abrupt CoV shifts between
consecutive checkpoints (exceeding 0.20) should be
evaluated in conjunction with Session Consistency Analysis
for potential data source switching.

# Forgery Cost Bounds (Quantified Security) {#economic-model}

Forgery cost bounds provide a Verifier with a lower bound on the computational resources required to forge an Evidence Packet. The cost (*C_total*) is computed as:

~~~ artwork
C_total = C_swf + C_entropy + C_hardware
~~~

## Sequential Work Function Cost (C_swf) {#cost-swf}

The SWF cost component provides a lower bound on the computational
time an adversary must expend:

~~~ artwork
C_swf >= n * t_checkpoint

where:
  n = number of checkpoints in the Evidence chain
  t_checkpoint = wall-clock time for one SWF computation
~~~

The memory-hard nature of Argon2id ensures that reducing memory
per evaluation forces a disproportionate increase in computation
time; the best known time-memory tradeoff (TMTO) attacks achieve at most a ~2x
reduction in time-area product for single-pass Argon2id, decreasing
to ~1.33x with multiple passes ({{RFC9106}}, Section 7). The raw
memory-bandwidth advantage of custom hardware (HBM3 at ~800 GB/s
versus consumer DDR4 at ~25 GB/s) provides an additional 3-4x
speedup when amortized over device cost. The combined ASIC advantage
is bounded at approximately 8-16x; Verifiers SHOULD use a
conservative factor of 10x when computing c-swf. Beyond
approximately 8 parallel threads, memory bandwidth saturation on
commodity hardware yields diminishing returns. The minimum forgery
time equals the
sum of SWF claimed-durations across all checkpoints. At T1 tier
without hardware binding, C_swf represents an economic cost only
(the adversary must spend real time, but has no hardware constraint).

### Adaptive SWF Calibration (Informative) {#adaptive-swf-calibration}

Attesters should dynamically calibrate the `steps`
parameter during the `initial-jitter-sample` phase to
consistently hit the target duty cycle, compensating for
local hardware variance. By running a short calibration pass
before the first checkpoint, the Attester measures actual
Argon2id throughput on the local platform and adjusts the
step count so that the iterated memory-hard computation fills
the target fraction of each checkpoint interval (30% for
CORE, 50% for ENHANCED, 70% for MAXIMUM). This ensures that
the economic security bounds documented in this section hold
uniformly across diverse deployment hardware, rather than
relying on fixed parameters tuned to a single reference
platform.

## Behavioral Evidence Synthesis Cost (C_entropy) {#cost-entropy}

The entropy cost component estimates the resources required to
synthesize behavioral noise satisfying all forensic constraints:

~~~ artwork
C_entropy = O(d * n * log(1/epsilon))

where:
  d = number of independent forensic dimensions
  n = number of checkpoints
  epsilon = target false-negative rate
~~~

At T1/T2, only basic entropy and timing are checked (d = 2). For
T3/T4, the full forensic assessment applies (d >= 7, including CLC,
IKI, error topology, SNR dynamics, session consistency, and
cross-checkpoint correlation), making synthesis exponentially more
expensive in the number of correlated dimensions the adversary must
simultaneously satisfy.

The cost of synthesizing behavioral noise that satisfies all
forensic constraints is inherently uncertain and depends on
adversary capability. Verifiers SHOULD set C_entropy conservatively.
When the Verifier cannot independently assess AI synthesis costs,
C_entropy SHOULD be set to 0 and the WAR warnings field SHOULD
note that entropy cost was not estimated.

## Hardware Attestation Cost (C_hardware) {#cost-hardware}

* *T1/T2:* C_hardware = 0. No hardware root of trust;
  keys are software-managed.
* *T3 (Hardware-Bound):* Requires compromise of TPM or
  platform Secure Element. Estimated cost: USD 10,000-100,000
  per device class, depending on the specific hardware and
  attack methodology.
* *T4 (Hardware-Hardened):* Requires invasive hardware
  attacks, manufacturer collusion, or firmware exploits targeting
  PUF-bound keys. Estimated cost: USD 100,000 or more.

Verifiers MUST include these estimates in the WAR to allow Relying Parties to set trust thresholds based on objective economic risk.

The c-total field in the forgery-cost-estimate MUST equal the sum
of c-swf, c-entropy, and c-hardware. All component costs within a
single forgery-cost-estimate MUST be expressed in the same
cost-unit.

# Absence Proofs: Negative Evidence Taxonomy {#absence-proofs}

Absence proofs are assertions that supplement but do not override forensic analysis. Forensic flags take precedence over contradictory absence claims when determining the verdict. Type 1 (computationally-bound) claims may strengthen an authentic verdict. Type 2 (monitoring-dependent) and Type 3 (environmental) claims are informational and weighted by attestation tier.

Absence proofs assert that certain events did not occur during the monitored session. They are divided into categories based on verifiability:

Type 1: Computationally-Bound Claims
: Verifiable from the Evidence Packet alone (e.g., "Max single delta size < 500 bytes" or "No checkpoint timestamps out of order").

Type 2: Monitoring-Dependent Claims
: Require trust in the AE's event monitoring (e.g., "No paste from unauthorized AI tool" or "No clipboard activity detected"). Trust in these claims MUST be weighted by the declared Attestation Tier (T1-T4).

Type 3: Environmental Claims
: Assertions about the execution environment (e.g., "No debugger attached" or "Hardware temperature remained within stable physical bounds").

Type 1 (Computationally-Bound) claims MUST be verified computationally
by the Verifier from the Evidence Packet data alone. Type 3
(Environmental) claims SHOULD be evaluated against physical-state
markers when present, and MUST be treated as unverifiable when
physical-state is absent.

# Attestation Result Wire Format {#war-wire-format}

The Cryptographic Writers Authenticity Report (WAR) is a CBOR-encoded
{{RFC8949}} Attestation Result identified by semantic
tag 1129791826 (encoding ASCII "CWAR"). The CDDL notation
{{RFC8610}} defines the wire format:

~~~ cddl
pop-war = #6.1129791826(attestation-result)

attestation-result = {
    1 => uint,                    ; version (must be 1)
    2 => hash-value,              ; evidence-ref
    3 => verdict,                 ; appraisal verdict
    4 => attestation-tier,        ; assessed assurance level
    5 => uint,                    ; chain-length
    6 => uint,                    ; chain-duration (seconds)
    ? 7 => entropy-report,        ; entropy assessment (omit for CORE)
    ? 8 => forgery-cost-estimate, ; quantified forgery cost
    ? 9 => [+ absence-claim],     ; absence claims (1+ when present)
    ? 10 => [* tstr],             ; warnings
    11 => bstr .cbor COSE_Sign1,   ; verifier-signature
    12 => pop-timestamp,          ; created (appraisal timestamp)
    ? 13 => forensic-summary,     ; forensic assessment summary
    ? 14 => confidence-tier,      ; baseline confidence level
    ? 15 => effort-attribution,   ; human-to-tool attribution
    * int => any,                 ; extension fields
}

effort-attribution = {
    1 => float32,                 ; human-fraction (0.0 to 1.0)
    2 => uint,                    ; human-checkpoints
    3 => uint,                    ; receipt-checkpoints
    ? 4 => uint,                  ; tool-attributed-chars
    ? 5 => uint,                  ; total-chars
}

verdict = &(
    authentic: 1,                 ; consistent with human authorship
    inconclusive: 2,              ; insufficient evidence
    suspicious: 3,                ; anomalies detected
    invalid: 4,                   ; chain broken or forged
)

forensic-summary = {
    1 => uint,                    ; flags-triggered
    2 => uint,                    ; flags-evaluated
    3 => uint,                    ; affected-checkpoints
    4 => uint,                    ; total-checkpoints
    ? 5 => [+ forensic-flag],    ; per-flag detail
}

forensic-flag = {
    1 => tstr,                    ; mechanism (e.g., "SNR", "CLC")
    2 => bool,                    ; triggered
    3 => uint,                    ; affected-windows
    4 => uint,                    ; total-windows
}

; In the forensic-flag structure, "windows" refers to individual
; checkpoints. The affected-windows field is the count of checkpoints
; where the forensic mechanism triggered; total-windows is the total
; number of checkpoints evaluated by that mechanism.

entropy-report = {
    1 => float32,                 ; timing-entropy (bits/sample)
    2 => float32,                 ; revision-entropy (bits)
    3 => float32,                 ; pause-entropy (bits)
    4 => bool,                    ; meets-threshold
}

forgery-cost-estimate = {
    1 => float32,                 ; c-swf
    2 => float32,                 ; c-entropy
    3 => float32,                 ; c-hardware
    4 => float32,                 ; c-total
    5 => cost-unit,               ; currency
}

cost-unit = &(
    usd: 1,
    cpu-hours: 2,
)

absence-claim = {
    1 => absence-type,            ; proof category
    2 => time-window,             ; claimed window
    3 => tstr,                    ; claim-id
    ? 4 => any,                   ; threshold/parameter
    5 => bool,                    ; assertion
}

absence-type = &(
    computationally-bound: 1,     ; verifiable from Evidence alone
    monitoring-dependent: 2,      ; requires trust in AE monitoring
    environmental: 3,             ; environmental assertions
)

time-window = {
    1 => pop-timestamp,           ; start
    2 => pop-timestamp,           ; end
}

; Behavioral Baseline Verification
baseline-verification = {
    ? 1 => baseline-digest,       ; omitted during enrollment
    2 => session-behavioral-summary, ; current session metrics
    ? 3 => bstr .cbor COSE_Sign1,  ; digest-signature
}

baseline-digest = {
    1 => uint,                    ; version (must be 1)
    2 => uint,                    ; session-count
    3 => uint,                    ; total-keystrokes
    4 => streaming-stats,         ; iki-stats
    5 => streaming-stats,         ; cv-stats
    6 => streaming-stats,         ; hurst-stats
    7 => [9* float32],            ; aggregate-iki-histogram
    8 => streaming-stats,         ; pause-stats
    9 => bstr .size 32,           ; session-merkle-root (MMR)
    10 => confidence-tier,        ; baseline maturity
    11 => pop-timestamp,          ; computed-at
    12 => bstr .size 32,          ; identity-fingerprint
}

session-behavioral-summary = {
    1 => [9* float32],            ; iki-histogram
    2 => float32,                 ; iki-cv
    3 => float32,                 ; hurst
    4 => float32,                 ; pause-frequency
    5 => uint,                    ; duration-secs
    6 => uint,                    ; keystroke-count
}

streaming-stats = {
    1 => uint,                    ; count
    2 => float32,                 ; mean
    3 => float32,                 ; m2 (Welford's sum of squared diffs)
    4 => float32,                 ; min
    5 => float32,                 ; max
}

; Receipts (Composition and Tool Attribution)
receipt = self-receipt / tool-receipt

self-receipt = {
    1 => tstr,                    ; tool-id (source environment)
    2 => hash-value / compact-ref, ; output-commit
    3 => hash-value / compact-ref, ; evidence-ref (source packet)
    4 => pop-timestamp,           ; transfer-time
}

tool-receipt = {
    1 => tstr,                    ; tool-id (provider URI)
    2 => hash-value,              ; output-commit
    ? 3 => hash-value,            ; input-ref (prompt hash)
    4 => pop-timestamp,           ; issued-at
    5 => bstr .cbor COSE_Sign1,   ; tool-signature
    ? 6 => uint,                  ; output-char-count
}

; Shared type definitions reproduced from [CPoP-Protocol] for reader
; convenience. In case of conflict, [CPoP-Protocol] is authoritative.
pop-timestamp = uint                ; epoch milliseconds (no tag 1; see [CPoP-Protocol])
hash-value = {
    1 => hash-algorithm,
    2 => hash-digest,              ; length must match algorithm output
}
hash-digest = bstr .size 32 /        ; SHA-256
              bstr .size 48 /        ; SHA-384
              bstr .size 64          ; SHA-512
hash-algorithm = &(
    sha256: 1,
    sha384: 2,
    sha512: 3,
)
compact-ref = {
    1 => hash-algorithm,          ; algorithm used for full hash
    2 => bstr .size (8..32),      ; truncated-digest (8-32 bytes)
    3 => uint,                    ; prefix-length (bytes in digest)
}
confidence-tier = &(
    population-reference: 1,      ; 0-4 sessions
    emerging: 2,                  ; 5-9 sessions
    established: 3,               ; 10-19 sessions
    mature: 4,                    ; 20+ sessions
)
attestation-tier = &(
    software-only: 1,             ; T1: AAL1
    attested-software: 2,         ; T2: AAL2
    hardware-bound: 3,            ; T3: AAL3
    hardware-hardened: 4,         ; T4: LoA4
)
~~~

The evidence-ref field MUST contain a hash-value computed as
SHA-256 over the CBOR-encoded evidence-packet structure
(including CBOR tag 1129336656), excluding any COSE_Sign1
wrapper. This binds the Attestation Result to a specific
Evidence Packet.

In the absence-claim structure, claim-id is a unique textual
identifier for the claim (e.g., "no-paste-event",
"max-delta-below-500"). The assertion field is true if the claim
holds and false if the Verifier determined it does not hold. The
time-window specifies the temporal scope of the claim within the
Evidence Packet's session.

When appraising CORE Evidence Packets that lack jitter-binding data,
the Verifier SHOULD omit the entropy-report field from the
Attestation Result and include a warning indicating that behavioral
entropy analysis was not performed.

The verifier-signature field (key 11) MUST contain a COSE_Sign1
structure {{RFC9052}}. The protected header MUST include an algorithm
identifier; ES256 or EdDSA is RECOMMENDED. The payload is the CBOR
encoding of the attestation-result map with key 11 set to a
zero-length bstr (h''). The Verifier's signing key MUST be
identifiable via the COSE_Sign1 unprotected header (e.g., kid or x5chain).

The created field (key 12) MUST contain the timestamp at which the
Verifier completed the appraisal. Relying Parties use this field
to evaluate the freshness of the Attestation Result.

## Entropy Report Computation {#entropy-report-computation}

The Verifier MUST compute entropy-report fields as follows:

timing-entropy:
: Shannon entropy of quantized jitter intervals across all
  checkpoints. Timing-entropy is measured in bits per inter-keystroke sample.

revision-entropy:
: Shannon entropy of edit-delta sizes (chars-added values)
  across all checkpoints. Revision-entropy is measured in bits over the full session edit-delta distribution.

pause-entropy:
: Shannon entropy of inter-checkpoint pause durations.
  Pause-entropy is measured in bits over the session pause-duration distribution.

meets-threshold:
: True if and only if timing-entropy is at or above the
  minimum threshold (3.0 bits per sample) AND revision-entropy
  is at or above 3.0 bits AND pause-entropy is at or above 2.0
  bits. These thresholds are calibrated for the NIST SP 800-90B
  most common value estimator. Implementations using alternative
  entropy estimators MUST provide equivalent assurance levels.

## Verdict Assignment {#verdict-assignment}

The Verifier MUST assign the verdict based on the composite
forensic assessment defined in
{{forensic-assessment}}:

authentic (1):
: All verification steps passed. Either no forensic flags
  were triggered, or a single flag was triggered across 30%
  or fewer of evaluated checkpoints. In the latter case, the
  triggered flag MUST be reported in the warnings field and
  forensic-summary.

inconclusive (2):
: Verification steps passed but insufficient behavioral data
  available for forensic assessment (e.g., CORE profile without
  jitter-binding).

suspicious (3):
: Two or more independent forensic flags triggered, OR a
  single flag sustained across more than 30% of evaluated
  checkpoints. Chain integrity is intact. The forensic-summary
  MUST detail which mechanisms triggered and the checkpoint
  coverage of each flag. When multiple forensic checks produce
  contradictory results, the Verifier MUST assign the more
  conservative verdict (suspicious over authentic).

invalid (4):
: Chain integrity broken, SWF verification failed, or
  structural validation error. Evidence cannot be trusted.

## Baseline Appraisal {#baseline-appraisal}

When the Evidence Packet includes a baseline-verification
structure (evidence-packet key 19, defined in
{{CPoP-Protocol}}), the Verifier MUST perform
the following checks before incorporating baseline data
into the verdict:

1. If baseline-digest is present, verify the
   digest-signature (COSE_Sign1) over the CBOR-encoded
   baseline-digest. Reject the baseline data if signature
   verification fails.

2. Verify that the identity-fingerprint in the
   baseline-digest equals SHA-256 of the Evidence Packet's
   signing key. This binds the baseline to the author
   identity.

3. Compare the session-behavioral-summary against the
   baseline-digest using Gaussian similarity on the
   streaming-stats fields (iki-stats, cv-stats, hurst-stats,
   pause-stats). For each metric, compute the z-score of the
   session value against the baseline mean and variance.
   A metric with |z| > 3.0 MUST trigger a baseline-deviation
   warning. Metrics with |z| between 2.0 and 3.0 MAY be
   reported as informational warnings. Baseline deviations do
   not constitute forensic flags and do not contribute to the
   flags-triggered count in forensic-summary.

The Verifier SHOULD weight baseline comparison results by
the confidence-tier: population-reference baselines provide
weak evidence, while mature baselines provide strong
per-author discrimination. The Verifier MUST include the
assessed confidence-tier in the Attestation Result (key 14)
when baseline-verification data was present and valid.

When the baseline-digest is absent (enrollment phase),
the Verifier SHOULD still evaluate the
session-behavioral-summary against population-level
distributions for human-vs-machine discrimination.
The confidence-tier in the Attestation Result MUST be
set to population-reference (1) in this case.

The Verifier SHOULD compute the baseline similarity score
as a weighted combination:

| Metric | Weight | Method | Flag Threshold |
|--------|--------|--------|----------------|
| IKI histogram | 0.4 | Bhattacharyya coefficient (session vs. baseline 9-bin) | coefficient < 0.5 |
| IKI CV | 0.2 | z-score of session iki-cv vs. baseline cv-stats | \|z\| > 3.0 |
| Hurst exponent | 0.2 | z-score of session hurst vs. baseline hurst-stats | \|z\| > 3.0 |
| Pause frequency | 0.2 | z-score of session pause-frequency vs. baseline pause-stats | \|z\| > 3.0 |

The baseline similarity thresholds in this section are preliminary values derived from limited empirical observation. Verifiers MAY adjust these thresholds based on deployment-specific population statistics. Future revisions will include sensitivity analysis and calibration methodology.

When the
baseline has fewer than 5 sessions (population-reference
tier), variance estimates are unreliable and z-score
comparisons SHOULD be treated as informational only.

Baseline comparison does not constitute a forensic flag. Baseline deviations are reported in the warnings field (key 10) of the Writers Authenticity Report but do not contribute to flags-triggered in forensic-summary. This separation allows Relying Parties to weight baseline context independently from forensic mechanism anomalies. A mature baseline match strengthens an authentic
verdict; a significant baseline deviation SHOULD be
reported as a warning but does not by itself trigger a
suspicious verdict.

# Appraisal Policy Architecture {#appraisal-policy}

RFC 9334 Section 2.2 defines two types of Appraisal Policy:
Appraisal Policy for Evidence (used by the Verifier) and
Appraisal Policy for Attestation Results (used by the Relying
Party). This section maps the CPoP appraisal framework to these
two policy types.

## Appraisal Policy for Evidence {#policy-for-evidence}

The Verifier's Appraisal Policy for Evidence is defined by
the verification procedure in {{verification-procedure}}, the
forensic assessment mechanisms in {{forensic-assessment}}, and
the per-tier verification constraints in
{{per-tier-constraints}}. Collectively, these specify how the
Verifier evaluates an Evidence Packet to produce an Attestation
Result (WAR).

The policy inputs are:

* The Evidence Packet itself (CBOR tag 1129336656)
* The CPoP specification (this document and {{CPoP-Protocol}}),
  which defines SWF parameters, forensic thresholds, and
  profile requirements
* Endorsements: TPM endorsement certificates and platform
  attestation credentials (T3/T4 tiers only), supplied by
  hardware manufacturers acting as Endorsers per {{RFC9334}}
* Reference Values: specification-defined forensic thresholds
  and behavioral baselines as described in
  {{CPoP-Protocol}}, Section "Reference Value Trust Model"
* Deployment-specific configuration: clock skew tolerances
  ({{clock-skew-tolerance}}), entropy estimator selection, and
  optional forensic mechanism enablement per Evidence Content
  Tier

## Appraisal Policy for Attestation Results {#policy-for-results}

The Relying Party's Appraisal Policy for Attestation Results
determines how WAR verdicts inform trust decisions. This policy
is deployment-specific, but implementations SHOULD consider the
following framework:

Verdict interpretation:

* authentic (1): The Relying Party MAY accept the claimed
  authorship provenance. The attestation-tier and
  confidence-tier fields indicate the strength of the
  underlying evidence.
* inconclusive (2): Insufficient behavioral data was available
  (typically CORE profile Evidence). The Relying Party SHOULD
  NOT treat this as evidence of forgery. Deployments requiring
  behavioral assurance SHOULD require ENHANCED or MAXIMUM
  Evidence Content Tiers.
* suspicious (3): Forensic anomalies were detected. The Relying
  Party SHOULD request additional evidence or apply enhanced
  scrutiny. The forensic-summary field identifies which
  mechanisms triggered and the affected checkpoint coverage.
* invalid (4): The Evidence Packet is structurally or
  cryptographically broken. The Relying Party MUST NOT rely on
  any claims from the associated content.

Tier-based acceptance:

* Relying Parties SHOULD define minimum acceptable attestation
  tiers and Evidence Content Tiers for their use case. For
  example, a legal evidentiary context might require T3
  ENHANCED or higher, while a blogging platform might accept
  T1 CORE.
* The attestation-tier in the WAR reflects the Verifier's
  assessed assurance level, which may differ from the
  Attester's claimed tier if hardware attestation validation
  failed.

Forgery cost evaluation:

* The forgery-cost-estimate field provides a quantified lower
  bound on the resources required to forge the Evidence. Relying
  Parties MAY compare this against the economic value of the
  content to assess whether forgery is economically rational.

Relying Parties MUST NOT make trust decisions based solely on
the verdict enum. The attestation-tier, confidence-tier,
forensic-summary, and forgery-cost-estimate fields provide
essential context for informed trust decisions.

# Effort Attribution {#effort-attribution}

When an Evidence Packet contains receipt structures (checkpoint
key 13), the Verifier MUST compute an effort-attribution and
include it in the Attestation Result (key 15). When no receipts
are present in any checkpoint, the Verifier MUST omit
effort-attribution entirely; the absence of receipts does not
imply the absence of tool use.

The Verifier computes effort-attribution fields as follows:

human-checkpoints:
: The count of checkpoints that contain no receipt structures.

receipt-checkpoints:
: The count of checkpoints that contain one or more receipt
  structures (either self-receipt or tool-receipt).

tool-attributed-chars:
: The sum of output-char-count values from all verified
  tool-receipts. For tool-receipts lacking output-char-count,
  the Verifier SHOULD estimate the contribution from the
  enclosing checkpoint's edit-delta chars-added field.
  Self-receipt contributions are excluded from this count
  because their content is already attested by the referenced
  Evidence Packet.

total-chars:
: MUST equal the char-count field from the Evidence Packet's
  document-ref.

human-fraction:
: Computed as:

      human-fraction = 1.0 - (tool-attributed-chars / total-chars)

  The result MUST be clamped to the range \[0.0, 1.0\]. When
  total-chars is zero, human-fraction MUST be 1.0.

When a self-receipt references an Evidence Packet that is not
available to the Verifier, the Verifier MUST still count the
checkpoint as a receipt-checkpoint but MUST NOT include
self-receipt content in tool-attributed-chars. The Verifier
SHOULD emit a warning indicating that the referenced Evidence
Packet could not be verified.

When computing human-fraction from Evidence Packets that reference other Evidence via self-receipts, the referenced packet's attestation tier determines the confidence weight. CORE-tier references contribute with a 0.5x weight discount to human-fraction calculations. ENHANCED and MAXIMUM references contribute at full weight.

# Tool Receipt Protocol {#tool-receipt-protocol}

Checkpoint key 13 carries receipt structures that attribute
content to external sources. Two receipt types are defined:
tool-receipt for external AI tool contributions and self-receipt
for cross-tool CPoP composition. Verifiers MUST process both
types when present.

## AI Tool Receipt Verification {#ai-tool-receipt-verification}

When external tools (e.g., large language models) contribute
content, the tool-receipt structure provides cryptographic
attribution. The Attester records the tool-generated content as
a paste event and binds the corresponding tool-receipt into the
next checkpoint.

Upon encountering a tool-receipt in checkpoint key 13, the
Verifier MUST perform the following steps:

1. Retrieve the tool provider's public key corresponding to the
   tool-id URI. The mechanism for key discovery and trust
   establishment will be specified in a companion document.
   Until a companion specification defines tool provider key
   discovery and trust establishment, Verifiers MUST NOT assign
   an authentic verdict to Evidence containing tool-receipts
   whose tool-id URIs cannot be resolved to a trusted public
   key. Unresolvable tool-receipts MUST be flagged in the
   warnings field of the Writers Authenticity Report. The
   Verifier MAY still evaluate all other forensic mechanisms
   and assign a verdict based on non-receipt evidence.

2. Verify the COSE_Sign1 structure in the tool-signature field
   (key 5) using the retrieved public key. The COSE_Sign1
   payload MUST be the CBOR encoding of the map
   `{1: tool-id, 2: output-commit, 4: issued-at}`. If
   signature verification fails, the Verifier MUST mark the
   checkpoint as invalid.

3. Verify that the output-commit hash (key 2) matches the
   content attributed to the tool in the checkpoint's
   content-hash chain. The hash algorithm is identified by the
   hash-algorithm field within the output-commit hash-value
   structure.

4. If input-ref (key 3) is present, record the prompt hash for
   inclusion in the effort-attribution computation. The
   Verifier MAY use input-ref to correlate tool invocations
   across checkpoints.

5. If output-char-count (key 6) is present, the Verifier MUST
   use this value when computing tool-attributed-chars in the
   effort-attribution structure. When absent, the Verifier
   SHOULD estimate the tool contribution from the edit-delta
   of the enclosing checkpoint.

Checkpoints containing a verified tool-receipt are excluded from
forensic mechanisms that assume continuous human authorship (see
{{forensic-assessment}}).

## Cross-Tool Composition {#cross-tool-composition}

The same receipt mechanism supports cross-tool composition
workflows where an author drafts content in one application and
transfers it to another for formatting or publication. When the
first authoring environment runs CPoP, it produces an Evidence
Packet covering the drafting phase. The second environment
records the paste event with a self-receipt: a receipt whose
tool-id identifies the first authoring environment and whose
output-commit matches the content-hash from the first Evidence
Packet's final checkpoint. The self-receipt additionally
includes an evidence-ref field containing the hash of the first
Evidence Packet. The Verifier chains the two packets: the first
Evidence Packet proves the drafting process, the second proves
the editing and formatting process, and the self-receipt's
output-commit binds the paste to the first packet's terminal
content state. When cross-tool composition without a
self-receipt is detected, Verifiers SHOULD apply the cross-tool
composition guidance defined in the Mechanical Turk Scoring
section.

# Adversary Model {#adversary-model}

This document inherits the adversary model defined in the Threat Model section of {{CPoP-Protocol}}. The appraisal-specific defenses at each tier are:

Tier 1 (Casual):
: SWF time-binding provides the primary defense. The T1 appraisal policy accepts the risk of basic retype attacks.
External timestamping via append-only ledgers {{HaberStornetta1991}} is complementary infrastructure that establishes a lower bound on when content was created but does not substitute for SWF-based minimum elapsed computation.

Tier 2 (Motivated):
: Multi-dimensional behavioral analysis (SNR + CLC + mechanical turk detection).

Tier 3 (Professional):
: HAT cross-validation and advanced forensic metrics (CLC, IKI, error topology, and SNR dynamics).

Tier 4 (Nation-State):
: Combined cost of SWF sequentiality, multi-dimensional behavioral evidence synthesis (d >= 7 correlated dimensions), and hardware attestation integrity. Minimum forgery cost equals the claimed authorship duration plus the hardware compromise cost.

# Privacy Considerations {#privacy-considerations}

## Evidence and Attestation Result Privacy {#privacy}

High-resolution behavioral data poses a stylometric de-anonymization
risk {{Monaco2018}}. Implementations SHOULD support
Evidence Quantization, reducing timing resolution to a level that
maintains forensic confidence while breaking unique author fingerprints.

The entropy-report in Attestation Results (timing-entropy,
revision-entropy, pause-entropy) may enable cross-document author
identification by Relying Parties. Verifiers SHOULD quantize
entropy-report values to reduce fingerprinting precision while
preserving forensic utility. Relying Parties MUST NOT correlate
entropy reports across multiple Attestation Results to identify
or track authors.

## Evidence Quantization Requirements {#privacy-quantization}

Attestation Results MUST quantize forensic indicator values to
the following resolutions:

* Cadence (IKI) values: millisecond resolution. Sub-millisecond
  data MUST NOT be included.
* Entropy values: 0.01 bit resolution (two decimal places).
* SNR values: 0.5 dB resolution.
* CLC and IKI metric values: two decimal places.
* KL divergence values (Spatial-Temporal Divergence): two
  decimal places.

These quantization levels are calibrated to preserve the
forensic utility of all assessment mechanisms defined in
{{forensic-assessment}} while limiting the
precision available for stylometric fingerprinting.
Spatial-temporal binning and zone-model projections MUST
operate on the already-quantized millisecond IKI values;
Verifiers MUST NOT reconstruct sub-millisecond timing from
spatial models.

## Data Retention and Behavioral Profiles {#privacy-retention}

Verifiers MUST NOT maintain per-author behavioral profile
databases. Attestation Results SHOULD NOT include raw forensic
indicator values; tier-level pass/fail determinations are
sufficient for Relying Parties. Evidence retention SHOULD NOT
exceed 90 days (the default validity period). Implementations
SHOULD support anonymous Evidence submission to prevent
linking authorship sessions to real-world identities.

# Accessibility and Alternative Input Modes {#accessibility}

Verifiers MUST NOT automatically reject evidence based solely on atypical timing patterns. Implementations MUST support input mode declarations that adjust SNR and CLC metric thresholds for authors using assistive technologies (eye-tracking, dictation, switch-access) or alternative input methods (input method editors for CJK and other non-Latin scripts).

To signal input mode usage, the Attester SHOULD include an assistive-mode indicator in the profile-declaration structure of the Evidence Packet. When this indicator is present, Verifiers MUST apply adjusted thresholds as follows:

## Eye-Tracking Mode {#assistive-eye-tracking}

Eye-tracking input produces IKI ranges of 500-3000 ms {{Majaranta2009}} (versus
100-300 ms for keyboard). Adjusted thresholds:

* Entropy: 2.0 to 4.0 bits/sample (reduced from 3.0 minimum)
* SNR: -5 dB to +5 dB (narrower than keyboard range). SNR
  anomaly threshold: +15 dB.
* CLC metric: r > 0.1 (reduced from r > 0.2)
* Error topology: Adjusted for gaze drift corrections, which
  produce characteristic error patterns distinct from keyboard
  errors.

## Dictation Mode {#assistive-dictation}

Dictation input produces burst patterns with higher cadence
variance than keyboard {{Karat1999}}. Adjusted thresholds:

* SNR: -8 dB to +8 dB (wider range reflecting speech pauses {{Shangguan2021}})
* CLC metric: r > 0.1 (range 0.1 to 0.8)
* Paste-to-keystroke ratio threshold: disabled (dictation
  engines produce burst insertions by design)
* Error topology: waived (dictation corrections follow
  speech-recognition patterns, not typing patterns)

## Input Method Editor (IME) Mode {#ime-input}

CJK and other non-Latin input methods produce a
composition-commit cycle (rapid phonetic keystrokes, candidate
selection pause, commit) with timing patterns distinct from
both direct keyboard input and clipboard paste. Adjusted
thresholds:

* IKI analysis: Verifiers MUST account for the bimodal
  distribution of IME input. Composition-phase keystrokes
  exhibit intervals of 50-200 ms {{Liang2009}}; inter-sequence pauses
  during candidate selection range from 300-2000 ms {{WangZhai2001}}.
  Verifiers MUST segment IKI analysis by composition phase
  rather than applying a single distribution model.
* SNR: -5 dB to +8 dB (wider than keyboard due to
  candidate selection pauses).
* CLC metric: r > 0.15 (candidate selection
  timing adds variance uncorrelated with content
  complexity).
* Paste-to-keystroke ratio: IME commit events insert
  one or more characters per candidate selection, which
  Verifiers MUST distinguish from clipboard paste by the
  presence of preceding composition keystrokes. Commit
  events MUST be excluded from paste detection.
* Error topology: Adapted for candidate reselection.
  The characteristic IME error pattern is commit, delete,
  recompose, recommit, which differs from keyboard typo
  corrections (character, backspace, retype) and dictation
  corrections (word-level replacement).

## Additional Accommodations {#assistive-accommodations}

* Switch-access input: minimum event count per checkpoint
  reduced to 1 (from default of 5).
* Head-tracking and mouth-stick input: apply eye-tracking
  thresholds.
* When assistive mode thresholds produce anomalous results,
  the Verifier SHOULD flag the inconsistency in the WAR
  warnings rather than reject the Evidence.

The WAR MUST indicate when assistive mode thresholds were applied.
Assistive mode is signaled through the profile-declaration
structure in the Evidence Packet. Implementations MAY include
an assistive-mode feature flag (value 60) in the feature-flags
array. The following values are defined: 0 (none), 1
(motor-disability), 2 (eye-tracking), 3 (dictation), 4
(ime-input). A future
revision of {{CPoP-Protocol}} will formalize this
signaling mechanism.

# EAR Compatibility Mapping (Informative) {#ear-compatibility}

The RATS working group is developing a standard Attestation Result
format called EAT Attestation Results (EAR) {{EAR}}, which profiles
the Entity Attestation Token (EAT) {{RFC9711}} for Verifier output.
The WAR format defined in this document is designed to be compatible
with future EAR profiling. This section documents the mapping
between WAR fields and EAR claims to facilitate interoperability
and future convergence.

## Verdict to ear.status Mapping {#ear-status-mapping}

The WAR verdict enum maps to EAR's ear.status as follows:

| WAR verdict | Value | ear.status | Semantics |
|-------------|-------|------------|-----------|
| authentic | 1 | affirming | Evidence consistent with human authorship |
| inconclusive | 2 | none | Insufficient evidence for appraisal |
| suspicious | 3 | warning | Anomalies detected; not definitively synthetic |
| invalid | 4 | contraindicated | Chain broken, forged, or structurally invalid |

## WAR Fields as EAR Extensions {#ear-extensions}

When the WAR is expressed as an EAR, the attestation-result fields
map as follows:

Top-level EAR claims:

- `eat_profile`: "tag:ietf.org,2025-07:ear" (per EAR requirement)
- `iat`: WAR created field (key 12)
- `ear.verifier-id`: derived from the Verifier's signing key
  identity in the verifier-signature COSE_Sign1 header (per
  {{AR4SI}})
- `submods`: single entry keyed by the evidence-ref hash

Per-appraisal EAR claims (within the submods entry):

- `ear.status`: mapped from WAR verdict per {{ear-status-mapping}}

CPoP-specific extension claims (registered via $$ear-appraisal-extension):

| WAR field (key) | Extension claim | Type |
|-----------------|----------------|------|
| evidence-ref (2) | ear.pop.evidence-ref | hash-value |
| attestation-tier (4) | ear.pop.attestation-tier | uint (1-4) |
| chain-length (5) | ear.pop.chain-length | uint |
| chain-duration (6) | ear.pop.chain-duration | uint (seconds) |
| entropy-report (7) | ear.pop.entropy-report | map |
| forgery-cost-estimate (8) | ear.pop.forgery-cost-estimate | map |
| absence-claims (9) | ear.pop.absence-claims | array |
| warnings (10) | ear.pop.warnings | array of tstr |
| forensic-summary (13) | ear.pop.forensic-summary | map |
| confidence-tier (14) | ear.pop.confidence-tier | uint (1-4) |
| effort-attribution (15) | ear.pop.effort-attribution | map |

## Migration Path {#ear-migration}

A future revision of this document will adopt EAR as the native
Attestation Result format when {{EAR}} reaches Working Group Last
Call. The migration path is:

1. Replace the WAR CBOR tag (1129791826) with CWT or JWT wrapping
   per EAR requirements. The verdict field maps to ear.status per
   {{ear-status-mapping}}.

2. Register the CPoP-specific extension claims listed in
   {{ear-extensions}} via the EAR extension point
   ($$ear-appraisal-extension). The CDDL definitions for these
   claims are provided below to facilitate early implementation:

   ~~~ cddl
   $$ear-appraisal-extension //= (
     "ear.pop.evidence-ref" => hash-value,
     "ear.pop.attestation-tier" => uint .within (1..4),
     "ear.pop.chain-length" => uint,
     "ear.pop.chain-duration" => uint,
     ? "ear.pop.entropy-report" => entropy-report,
     ? "ear.pop.forgery-cost-estimate" => forgery-cost-estimate,
     ? "ear.pop.absence-claims" => [+ absence-claim],
     ? "ear.pop.warnings" => [* tstr],
     ? "ear.pop.forensic-summary" => forensic-summary,
     ? "ear.pop.confidence-tier" => uint .within (1..4),
     ? "ear.pop.effort-attribution" => effort-attribution,
   )
   ~~~

   The attestation-tier and confidence-tier claims use `uint .within
   (1..4)` rather than the named enums defined in
   {{war-wire-format}}. The numeric values are identical;
   implementers should consult the attestation-tier and
   confidence-tier definitions in the WAR format for the
   corresponding symbolic names.

3. Adopt the ear.verifier-id structure from {{AR4SI}} in place of
   the current verifier-signature field.

4. Retain the CPoP profile URI
   "urn:ietf:params:rats:eat:profile:pop:1.0" to distinguish CPoP
   Attestation Results from other EAR-conformant results.

Until EAR stabilizes, the WAR format defined in
{{war-wire-format}} remains the normative format for this
specification. Implementers are encouraged to support both the
WAR format and the EAR extension claims defined above to
facilitate a smooth transition.

# Experimental Status Rationale {#experimental-status-rationale}

This document is published with Experimental status for the
following reasons:

1. *Novel application of RATS:* CPoP extends the RATS
   architecture from device state attestation to physical
   process attestation, with a trust inversion (adversarial
   Attester) that has no established precedent in IETF
   standards. Implementation experience is needed to validate
   this model.

2. *Empirically-derived thresholds:* The forensic assessment
   thresholds specified in {{forensic-assessment}} (spectral
   flatness, CLC metric coefficients, CoV bounds, entropy
   minimums) are calibrated from published behavioral research
   but have not been validated across the diversity of
   authoring workflows, languages, input methods, and
   assistive technologies that a deployed system would
   encounter. These values are expected to be refined based on
   implementation experience.

3. *Evolving forensic landscape:* The adversarial capabilities
   of AI systems for synthesizing behavioral data are rapidly
   advancing. The specific forensic mechanisms and their
   relative effectiveness may need revision as the threat
   landscape evolves.

4. *Companion specification dependency:* This document depends
   on the companion protocol specification {{CPoP-Protocol}},
   which is also Experimental. Both documents should advance
   together based on coordinated implementation feedback.

Implementers are encouraged to report experience with threshold
calibration, false positive/negative rates, and adversarial
evasion attempts to inform future revisions.

# IANA Considerations {#iana-considerations}

This document has no IANA actions. All IANA registrations for the CPoP framework are defined in {{CPoP-Protocol}}.

# Security Considerations {#security-considerations}

This document defines forensic appraisal procedures that inherit and extend the security model from {{CPoP-Protocol}}. The broader RATS security considerations {{Sardar-RATS}} also apply. Implementers should consider the following security aspects:

## Entropy Manipulation Attacks {#sec-entropy-manipulation}

An adversary may attempt to inject synthetic jitter patterns that satisfy entropy thresholds while lacking biological origin {{Condrey2026Attack}}. The use of multi-dimensional analysis (SNR, CLC, Error Topology) rather than single metrics provides defense-in-depth against high-fidelity simulation.

## Verifier Trust Model {#sec-verifier-trust}

The forensic assessments defined in this document produce probabilistic confidence scores, not binary determinations. Relying Parties need to understand that forgery cost bounds represent economic estimates, not cryptographic guarantees. Trust decisions SHOULD incorporate the declared Attestation Tier (T1-T4) and the specific absence proof types claimed.

## Stylometric De-anonymization {#sec-stylometric-risk}

High-resolution behavioral data (keystroke timing, pause patterns) can enable author identification even when document content is not disclosed. Implementations SHOULD support Evidence Quantization to reduce timing resolution while maintaining forensic utility. The trade-off between forensic confidence and privacy SHOULD be documented for Relying Parties.

## Baseline Biometric Re-identification {#sec-baseline-privacy}

The baseline-digest accumulates a persistent behavioral
biometric profile (IKI histogram, coefficient of variation,
Hurst exponent, pause frequency statistics) bound to a stable
identity-fingerprint. This profile can enable author
re-identification across sessions even when document content
is not disclosed. The 9-bin IKI histogram alone provides
sufficient discriminative power to distinguish authors within
populations of moderate size.

Implementations SHOULD offer authors the option to omit the
baseline-digest from exported Evidence Packets. When baseline
data is included, the identity-fingerprint (SHA-256 of the
signing key) acts as a persistent pseudonym; compromise of
this pseudonym linkage enables correlation of all Evidence
Packets by the same author.

Verifiers that store baseline data for longitudinal analysis
MUST apply the same data protection requirements as other
biometric data under applicable regulations. Baseline data
MUST NOT be shared between Verifiers without explicit author
consent.

## Attestation Tier Downgrade Detection {#sec-tier-downgrade}

A Verifier that maintains state across sessions SHOULD detect
tier downgrades — where an Attesting Environment that previously
submitted Evidence at tier T(n) submits Evidence at tier T(n-k)
where k > 0. Tier downgrades MAY indicate key compromise,
environment tampering, or an attempt to evade hardware-bound
verification. A Verifier that detects a tier downgrade SHOULD
include a warning in the Attestation Result warnings field
(key 10) and MAY assign a more conservative verdict.

## Assistive Mode Abuse {#sec-assistive-bypass}

Adversaries may falsely claim assistive technology usage to bypass behavioral entropy checks. Verifiers SHOULD require consistent assistive mode declarations across sessions and MAY request additional out-of-band verification for mode changes. The WAR SHOULD indicate when assistive modes were active, as specified in the accessibility section above. Assistive mode declarations at T1 and T2 attestation tiers are self-reported by the Attester and cannot be independently verified. Verifiers accepting assistive mode at these tiers accept an elevated forgery risk, as an adversary may claim assistive mode to bypass behavioral consistency checks. For T3 and T4, assistive technology usage SHOULD be corroborated by hardware attestation of the input device.

--- back

# Verification Constraint Summary {#verification-checklist}
{:numbered="false"}

This appendix is normative. The following constraints summarize the verification requirements defined in the preceding sections:

## Structural Integrity {#structural-checks}
{:numbered="false"}

1. Chain Integrity: SHA-256 hash chain is unbroken from genesis to final checkpoint.
2. Temporal Monotonicity: All checkpoint timestamps strictly exceed their predecessors.
3. SWF Continuity: Recompute Argon2id from seed; verify sampled Merkle proofs.
4. Content Binding: Final document hash matches document-ref in Evidence Packet.

## Behavioral Analysis (ENHANCED/MAXIMUM profiles) {#behavioral-checks}
{:numbered="false"}

1. Entropy Threshold: Independent entropy estimate >= 3.0 bits per inter-keystroke interval per checkpoint.
2. SNR Analysis: Jitter exhibits characteristic biological noise patterns, not periodic or spectrally flat patterns.
3. CLC Metric: Semantic complexity correlates with timing (r > 0.2, or r > 0.1 for assistive mode).
4. Error Topology: Correction patterns consistent with human cognitive processing.
5. Mechanical Turk Detection: No robotic pacing (machine-clocked editing rate).

## Absence Proof Validation {#absence-checks}
{:numbered="false"}

1. Type 1 Claims: Verify computationally from Evidence Packet (delta sizes, timestamp ordering).
2. Type 2 Claims: Weight by Attestation Tier (T1-T4).
3. Type 3 Claims: Evaluate environmental assertions against physical-state markers.

## Tool Receipt Validation {#tool-receipt-checks}
{:numbered="false"}

When checkpoint key 13 contains receipt structures, the
Verifier MUST validate them as follows. Paste events
accompanied by a verified receipt (either type) MUST be
excluded from C_intra, perplexity scoring, and Mechanical
Turk detection analysis.

AI Tool Receipts (tool-receipt):

1. Retrieve the tool provider's public key for the tool-id
   URI.
2. Verify the COSE_Sign1 structure in the tool-signature
   field (key 5). The payload MUST be the CBOR encoding of
   `{1: tool-id, 2: output-commit, 4: issued-at}`.
3. Verify that output-commit (key 2) is consistent with
   the checkpoint's content-hash chain.
4. If output-char-count (key 6) is present, include it in
   the effort-attribution computation.

Self-Receipts (self-receipt):

1. Verify that evidence-ref (key 3) identifies a valid
   Evidence Packet.
2. Verify that output-commit (key 2) matches the
   content-hash of the referenced Evidence Packet's final
   checkpoint.
3. If the referenced Evidence Packet is unavailable, emit
   a warning and exclude the self-receipt from
   effort-attribution character counts.

# Per-Tier Verification Constraints {#per-tier-constraints}
{:numbered="false"}

This appendix is normative. It summarizes the verification
thresholds and constraints for each Attestation Tier. These
values are the normative defaults; deployment profiles MAY
adjust them within the ranges specified.

## T1 (Software-Only) Constraints {#constraints-t1}
{:numbered="false"}

* Chain integrity: prev-hash linkage required.
* Temporal ordering: monotonic timestamps required; SWF
  claimed-duration within \[0.5x, 3.0x\] of expected time.
* Entropy: minimum 3.0 bits/sample when jitter-binding is
  present. No upper bound enforced.
* Entanglement: jitter tag presence required when
  jitter-binding is present; HMAC verification SHOULD be
  performed (MAC key derivable from public merkle-root).
* State matching: final content hash match required.
* Forensic assessment: SNR computation OPTIONAL; CLC, error
  topology, and mechanical turk detection RECOMMENDED for
  ENHANCED+ profiles.
* Forgery cost bound: C_total = C_swf + C_entropy (no
  hardware component). Physical-state fields are self-reported
  and provide no additional assurance.

## T2 (Attested Software) Constraints {#constraints-t2}
{:numbered="false"}

* Chain integrity: all T1 requirements.
* Temporal ordering: all T1 requirements; SWF
  claimed-duration within \[0.5x, 3.0x\] of expected time on
  reference hardware.
* Entropy: 3.0 to 6.0 bits/sample per checkpoint.
  Values above 6.0 suggest injected randomness and SHOULD
  be flagged.
* Entanglement: jitter tag and entangled-binding presence
  required for ENHANCED+ profiles. HMAC verification
  SHOULD be performed.
* State matching: final content hash match required;
  intermediate content hash progression SHOULD be
  verified for monotonic growth.
* Forensic assessment: SNR, CLC, and mechanical turk
  detection required. Error topology OPTIONAL.
* Forgery cost bound: C_total = C_swf + C_entropy.
  Minimum forgery time equals the sum of SWF
  claimed-durations.

## T3 (Hardware-Bound) Constraints {#constraints-t3}
{:numbered="false"}

* Chain integrity: all T2 requirements. COSE_Sign1
  signature MUST verify against hardware-bound key.
* Temporal ordering: all T2 requirements; HAT delta
  cross-validation SHOULD be performed when TPM monotonic
  counter data is available.
* Entropy: 3.0 to 5.5 bits/sample, reflecting tighter
  calibration against verified human authorship baselines.
* Entanglement: HMAC verification MUST be performed.
  Device attestation certificate chain SHOULD be validated
  against known Endorser roots.
* State matching: all T2 requirements; intermediate
  content hash progression MUST be verified for monotonic
  growth. Non-monotonic changes (document size decreasing
  by more than 50% between consecutive checkpoints) MUST
  be flagged.
* Forensic assessment: all T2 requirements plus error
  topology analysis required. QR presence challenge
  OPTIONAL.
* Forgery cost bound: C_total = C_swf + C_entropy +
  C_hardware. Hardware compromise cost estimated at
  USD 10,000-100,000.

## T4 (Hardware-Hardened) Constraints {#constraints-t4}
{:numbered="false"}

* Chain integrity: all T3 requirements.
* Temporal ordering: all T3 requirements; HAT delta
  cross-validation MUST be performed; HAT-SWF agreement
  within 5% tolerance required.
* Entropy: 3.0 to 5.0 bits/sample; entropy trajectory
  standard deviation MUST exceed 0.1 bits across the
  session. Entropy trajectory standard deviation is computed
  as: SD = standard_deviation(\[H_1, H_2, ..., H_n\]) where
  H_i is the timing-entropy computed independently for
  checkpoint i. A constant-entropy session (SD < 0.1 bits)
  indicates that behavioral entropy does not vary naturally
  across the session, which is characteristic of synthetic
  generation.
* Entanglement: all T3 requirements; timing vector
  entropy consistency check required (within 0.5 bits of
  reported entropy-estimate).
* State matching: all T3 requirements.
* Forensic assessment: all T3 requirements;
  cross-correlation analysis between entropy and SNR
  required. QR presence challenge RECOMMENDED.
* Forgery cost bound: C_total = C_swf + C_entropy +
  C_hardware. Hardware compromise cost estimated at
  USD 100,000 or more. Total minimum forgery cost exceeds
  sum of claimed-durations plus hardware procurement.

# Composite Trust Score (Informative) {#composite-trust-score}
{:numbered="false"}

Relying Parties may wish to derive a single scalar trust
metric from the Attestation Result. The following formula
provides one example weighting:

~~~ pseudocode
process-score = w_r * regularity +
                w_s * swf-strength +
                w_b * behavioral-consistency
~~~

Where:

* regularity reflects checkpoint spacing uniformity
  (derived from timestamp intervals)
* swf-strength reflects the sequential work function's
  forgery cost relative to claimed duration
* behavioral-consistency reflects the forensic assessment
  outcome (fraction of flags not triggered)
* w_r, w_s, w_b are Relying Party-configured weights
  that must sum to 1.0 (example: 0.3, 0.3, 0.4)

This formula is non-normative. Relying Parties should
adjust weights based on their threat model and the
Attestation Tier of the Evidence. For example, a Relying
Party that trusts hardware-bound attestation (T3/T4) may
increase w_s, while one focused on behavioral authenticity
may increase w_b.

# Acknowledgements {#acknowledgements}
{:numbered="false"}

The author thanks the participants of the RATS working group for
their ongoing work on remote attestation architecture and security
considerations that informed this specification.
