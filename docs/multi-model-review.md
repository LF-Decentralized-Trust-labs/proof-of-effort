# CPoE Protocol Multi-Model Expert Review

9 frontier AI models participated in a 5-round structured debate reviewing
the CPoE protocol, focusing on forgery prevention and the malicious Attester
problem. Models: GPT-5.4 Pro, GPT-5.4, o3 Pro (OpenAI), Grok 4.20 Beta (xAI),
Gemini 3.1 Pro (Google), Qwen 3.5 397B (Alibaba), DeepSeek V3.2 (DeepSeek),
Claude Opus 4.6 (Anthropic), Mistral Large 3 (Mistral), MiMo V2 Pro (Xiaomi),
MiniMax M2.7 (MiniMax).

Date: 2026-03-29

---

# Round 1: Initial Reviews

============================================
ROUND 1: INITIAL REVIEWS FROM 9 MODELS
============================================

## GPT-5.4 Pro (OpenAI)

CPoE is strongest as a **forensic telemetry format**, but the current design overstates its security as a “cryptographic proof” in the presence of a **malicious Attester**. The main architectural gap is that the entity producing evidence is the same entity the protocol is trying to distrust. Without a trusted execution boundary, most fields are only self-asserted.

**1) Design quality / completeness**

The packet structure, chained checkpoints, CBOR/CDDL typing, and explicit assurance tiers are good design choices. The separation between evidence generation and verifier-side appraisal is also sound. However, the core security claims depend on mechanisms that are not, by themselves, cryptographic authenticity guarantees:

- **Argon2id SWF is not a proof of elapsed wall-clock time.** It is a memory-hard computation. A faster adversarial machine (GPU/FPGA/ASIC/high-memory server) can generate checkpoints faster than the nominal client and then fabricate timestamps unless time is externally anchored.
- **Behavioral telemetry is statistical evidence, not unforgeable evidence.** Pink-noise/Hurst/CV properties are useful heuristics, but they are readily targetable by an adaptive attacker.
- The protocol needs sharper specification of **what exactly is committed per checkpoint**. Hashing “document current state” is weaker than hashing a canonical **edit log** (insert/delete/selection/paste/undo events + cursor movement + timing + prior state). Current-state hashes allow large semantic discontinuities between checkpoints.

**2) Concrete weaknesses / attack vectors**

- **Offline synthesis attack:** Generate AI text first, then synthesize a plausible checkpoint chain offline. Entangled mode prevents *parallel* checkpoint creation, but not *faster-than-real-time* generation on better hardware.
- **Clock/time-jump attack:** At T1/T2, the client can alter system time, TSC reads, scheduler behavior, sleep behavior, and Argon2 pacing. “Wall-clock” is unenforceable without trusted time.
- **Telemetry replay / profile cloning:** Record a real user’s prior keystroke traces and replay them against AI-generated text. A kernel/root adversary can inject events indistinguishable from hardware-origin events to the application.
- **Humanized bot attack:** Use an LLM plus a timing generator trained to match target spectral statistics, burstiness, error rates, pause distributions, and even semantic-load correlation. The listed detector set is exactly the sort of feature space an attacker can optimize against.
- **Late bulk insertion attack:** Type naturally for a while, then paste or programmatically insert a large AI-generated span just before a checkpoint, followed by “human-like” edits/corrections. A state hash alone does not prove incremental human composition.
- **Sensor forgery:** Thermal trajectories, entropy deltas, accelerometer traces, and “kernel entropy” are meaningless at T1/T2 because the attacker controls the software path and can replay or synthesize them.
- **Sampled SWF verification gap:** If the verifier recomputes only sampled proofs via Merkle paths, soundness depends critically on unpredictable sampling and full commitment to the computation trace. Otherwise, attackers can concentrate effort only where they expect checks.

**3) How to improve anti-forgery against a malicious Attester**

- Require **hardware-rooted attestation** for any serious assurance: measured boot + remote attestation of the capture agent/editor, plus signing with a hardware-protected key.
- Use **trusted monotonic counters / signed time** from TPM/TEE or, better, an **external witness service** issuing fresh nonces at unpredictable intervals; bind each checkpoint to server nonces and publish hashes to a transparency log.
- Commit to a **canonical edit-operation log**, not just document snapshots.
- Add **trusted-path input provenance**: e.g., HID-origin attestations, external signing keyboard, or a capture agent in a TEE that receives raw input events and signs them.
- Treat behavioral models as **risk signals only**, not proof. The cryptographic core should come from freshness, authenticated measurement, and external timestamping/witnessing.

Bottom line: CPoE has value as **tamper-evident process evidence**, but under malicious-client control, T1/T2 are highly forgeable. The protocol needs stronger trusted measurement and external freshness to justify anti-falsification claims.

---

## GPT-5.4 (OpenAI)

CPoE has a thoughtful structure—hash-chained checkpoints, sequential work, behavioral telemetry, and tiered assurance—but its core security claim is constrained by a fundamental asymmetry: the Attester is user-controlled software on a potentially adversarial host. That sharply limits what can be treated as cryptographic evidence versus merely self-reported forensic signals.

**1. Design quality / completeness**

The protocol is strongest where it explicitly acknowledges this, especially the T1/T2 caveat. The hash chain, CBOR/CDDL discipline, separation of Evidence from appraisal, and optional channel/time binding are all good design choices. Mode 21 entanglement is also directionally correct: making checkpoint \(i+1\) depend on \(i\) reduces offline batch fabrication.

However, the design currently mixes **cryptographic guarantees** with **statistical plausibility checks** too loosely. The SWF proves that *some* computation occurred in sequence; it does **not** prove the document was produced contemporaneously by a human. Likewise, jitter binding only proves that the attester emitted a timing series, not that the timings came from real keystrokes. The protocol would benefit from a more explicit security model identifying exactly which claims are proven at each tier:
- integrity of checkpoint sequence,
- approximate elapsed wall-clock,
- liveness/presence,
- origin of text edits,
- human authorship.

Right now these are partially conflated.

**2. Shortcomings / attack vectors**

The dominant attack is **full-stack simulation**. A malicious author can generate the text with an LLM, then run a replay/synthesis tool that emits keystroke events, thermal/entropy “markers,” and document snapshots while computing the SWF in real time. Since the software stack is attacker-controlled, all software-visible signals are forgeable at T1/T2.

Concrete variants:
- **Keystroke timing synthesis:** generate pink-noise-like inter-key intervals with target Hurst exponent/CV. These are easy to fit using fractional Gaussian noise or GAN-style sequence synthesis. If verifier thresholds are public, attackers can tune directly to them.
- **Semantic-aware pacing attack:** use token difficulty/word rarity to modulate delays so CLC/MTD/ET heuristics appear human. An LLM can intentionally insert typos, backspaces, and burst/pause structure.
- **Edit-stream laundering:** compose with AI elsewhere, then “retype” through an instrumented client over time. CPoE may prove a human typed *something*, but not that the process was genuinely generative rather than transcription.
- **SWF farm with live rendering:** because SWF is independent of cognition, an attacker can pre-generate text and spend the required wall-clock only on proof generation while a bot dribbles edits into the editor.
- **Chosen-checkpoint manipulation:** if checkpoint cadence is attacker-influenced, they can place checkpoints at favorable moments, smoothing anomalies and omitting suspicious bursts/pastes.
- **Paste decomposition attack:** a large AI-generated insertion can be broken into synthetic micro-edits to evade PPX/SC/ET triggers.
- **Sensor virtualization:** accelerometer, thermal, entropy, and even TPM quotes can be proxied or replayed on compromised devices unless there is strong measured boot plus anti-replay freshness.
- **Verifier gaming / hill-climbing:** if WAR outputs mechanism-level failures, attackers can iteratively adapt generators to pass SNR/BCA/PPX thresholds.

**3. Improvements against malicious Attester forgery**

The main improvement is to reduce trust in host-reported telemetry.

- **Move capture into a trusted path:** keyboard/IME/input-driver attestation, secure enclave collection, or external witness peripherals. Without trusted input provenance, “jitter binding” is just signed fiction.
- **Mandatory verifier-driven freshness:** unpredictable challenges interleaved during composition, not just optional QR presence. For example, challenge-dependent microtasks tied to the next few edits or to local cursor context. This raises the cost of offline fabrication.
- **Commit to full edit log semantics:** include insert/delete ranges, cursor moves, paste events, focus changes, and source application provenance. That helps distinguish transcription, paste laundering, and genuine drafting.
- **Externally witnessed time:** use transparency-log timestamping or remote witness co-signing per checkpoint batch. TPM-only time is insufficient on a hostile host.
- **Adaptive hidden classifiers:** keep some appraisal features undisclosed and rotate them. Public static thresholds are easy to mimic.
- **Checkpoint schedule hardening:** verifier- or witness-derived randomization prevents attacker-optimized segmentation.
- **Formal adversary model:** specify security theorems only for T3/T4, and describe T1/T2 strictly as evidentiary heuristics, not anti-forgery guarantees.

Bottom line: CPoE is promising as a **forensic provenance signal**, but under malicious-attester control it is not yet a robust anti-falsification protocol unless trusted capture and independent witnessing become central rather than optional.

---

## o3 Pro (OpenAI)

Round-1 Review – CPoE

1. Design quality / completeness  
•  The wire format, checkpoint chaining and separation of Protocol vs. Appraisal drafts are solid; the mapping to RATS and C2PA is forward-looking.  
•  The Sequential Work Function (SWF) and “entangled” mode (Mode 21) correctly capture the intuition that proofs must be sequential and memory-hard.  
•  However, the security model is underspecified for the most important adversary: the end user who owns the software stack. Tiers T1/T2 explicitly admit defeat against a kernel-level attacker, yet almost every interesting forgery lives in that space.  The spec never states which *exact* PCRs, TrustZone/SGX measurements, or boot-time events must be quoted at T3/T4, leaving large latitude for selective disclosure.

2. Concrete shortcomings & attack vectors  

A. SWF pre-computation on accelerated hardware  
   •  Argon2id’s ASIC-resistance is only ~3–5×; an attacker with a high-bandwidth FPGA farm can compress hours of “wall clock” into minutes, then replay proofs at normal speed.  The Verifier sees valid entanglement and believable timestamps because the Attester controls the timestamp fields.  
   •  Variant: do the above on a second machine while the first machine trickles out keystrokes that replay the forged chain in real time.

B. Synthetic jitter generation  
   •  Generating 1/f noise with Hurst ≈0.7 is trivial with an ARFIMA(0,d,0) process.  A compromised Attester can emit statistically perfect “human” intervals while pasting paragraphs from an LLM buffer.  
   •  Because the jitter is *software-timestamped*, kernel-space can skew TSC/CNTVCT or intercept the timing API, defeating the biological-cadence test.

C. Copy-paste & burst insertion  
   •  The protocol hashes the document state but does not bind *diff granularity*.  An attacker can paste a 2-page AI section, then type a few innocuous keystrokes to create a checkpoint.  Perplexity & CLC will flag low semantic load, but a tuned attacker can throttle the paste over 10–15 s so that cadence heuristics pass.

D. Sensor spoofing / virtualization  
   •  Thermal, accelerometer, and kernel-entropy deltas are read from sysfs/iio or ioctl; LD_PRELOAD or a virtual I/O driver can feed high-entropy synthetic traces that pass ICA and physical-state checks.  
   •  Without a secure path from sensor to TPM, “hardware markers” are provenance theater.

E. TPM / HAT time shifting  
   •  TPM2_ClockSet is blocked, but the attacker can suspend/resume or run inside a VM and snapshot/restore, causing *apparent* forward progress in TPM clock while generating proofs offline.

3. Recommendations to harden against malicious Attesters  

1. Replace Argon2id with, or supplement it by, a delay function that has <1.2× ASIC speedup (e.g., Wesolowski VDF or MinRoot).  Embed a public randomness beacon (NIST, drand) value revealed *after* checkpoint i so proofs for i+1 cannot be pre-computed.  
2. Make CPoE online-interactive at T3/T4: the Verifier (or witness service) injects unpredictable challenges every N seconds that must be incorporated into the next checkpoint hash.  Missed or reordered challenges expose off-line generation.  
3. Bind keystroke acquisition to a measured, signed enclave:  
   •  SGX/TDX or ARM-TZ app that receives raw scancode IRQs, timestamps them internally, and seals the log;  
   •  Attester quotes enclave MRENCLAVE/PCRs inside each checkpoint.  
4. Use kernel-level eBPF “keystore” to count IRQs and expose them through TPM EventLog; discrepancies between enclave log and EventLog reveal user-space injection.  
5. Tighten differential binding: store rolling diffs (e.g., operational-transform actions) not just document hash, so large instantaneous inserts are evident.  
6. Treat physical sensors as advisory unless readings are *signed* by an external MCU with attestable firmware (e.g., Nordic nRF + ED25519 key burned in OTP).  
7. Specify mandatory PCR sets and forbid debug/flexible quoting at T3/T4; require remote attestation of the exact Attester build hash.  
8. Publish reference statistical tests with α-levels and minimum sample sizes so that implementers cannot “test-wash” synthetic jitter.

These steps move the protocol from “tamper-evident if the user is honest” toward real forgery resistance against a fully malicious software stack.

---

## Grok 4.20 Beta (xAI)

**CPoE Protocol Round 1 Review: Design Quality, Weaknesses, and Malicious Attester Resistance**

The CPoE protocol presents an ambitious and relatively well-structured attempt to cryptographically bind human authorship to a creative process. Its core strength lies in the **entangled Sequential Work Function (SWF)** using Argon2id in Mode 21, where each checkpoint’s SWF input includes the previous checkpoint’s output. This creates a strictly sequential, memory-hard causality chain that resists parallel pre-computation and forces real wall-clock time to elapse. The combination of document state hashes, jitter binding, and SWF proofs forms a tamper-evident append-only log that is conceptually sound.

However, the design has critical gaps in adversarial modeling, particularly against a **malicious Attester** who controls the entire software stack (the explicit threat model for T1/T2).

### Major Design Shortcomings

1. **Root Adversary Can Fabricate Plausible Jitter Post Facto**  
   The protocol relies heavily on inter-keystroke timing exhibiting pink noise (1/f), Hurst exponent 0.55–0.85, and biological coefficient of variation. A malicious Attester that controls the input stream can generate synthetic timing data that matches these statistical properties *after* the document is written (or even during AI generation). The reference implementation’s `cpoe-jitter` crate appears to treat jitter as an observable rather than a cryptographically protected sensor reading. Nothing prevents the Attester from running a high-quality behavioral model (e.g., a fine-tuned LSTM or diffusion model trained on the target author’s prior typing) to generate timing sequences that pass SNR, BCA, and Hurst tests. The independence classes listed in the forensic table are mostly statistical; they are not cryptographically independent from an adversary who controls the data source.

2. **Insufficient Binding Between Keystrokes and SWF**  
   The protocol description does not specify exactly how individual keystrokes or small document deltas are bound into each SWF instance. If the SWF only operates on coarse checkpoints (e.g., every 30–60 seconds), an adversary can generate the entire document using AI, then synthesize a plausible checkpoint chain with correct statistical jitter in a single offline pass. The “entangled” property only protects the *time* between checkpoints, not the semantic plausibility of the typing events within each interval. A concrete attack is the **“Statistical Waterfall Attack”**: generate the full document, then solve for a sequence of inter-keystroke vectors that match the required Hurst exponent and 1/f spectrum while satisfying the SWF chain. This is computationally feasible because Argon2id is only expensive per checkpoint, not per keystroke.

3. **Physical State Markers Are Weak at T1/T2**  
   Thermal trajectories, kernel entropy deltas, and inertial accelerometer data are mentioned as “increasing dimensionality.” Against a root adversary these are essentially free variables. The adversary can simulate plausible thermal drift, manipulate `/dev/random` entropy counters, and synthesize accelerometer data (especially on laptops without constant physical monitoring). The protocol does not appear to require continuous sampling or cryptographic binding of these sensors into the SWF computation at high frequency.

4. **No Strong Keyed or Hardware-Bound Channel for Behavioral Data**  
   There is no mention of using a TPM PCR extension or a hardware-bound monotonic counter that is updated on every checkpoint with a measurement of the *live* behavioral collection module. A root attacker can therefore replace the entire `cpoe-jitter` logic with a forgery engine that produces valid-looking CBOR evidence.

5. **Verifier Oracle Problem and Replay**  
   The forensic mechanisms (CLC, MTD, ET, PPX, etc.) are sophisticated but rely on heuristics that a sophisticated adversary can explicitly optimize against. An attacker with access to the verifier binary (or the appraisal draft) can perform adversarial training of their forgery generator to minimize flags across independence classes. The protocol lacks a **freshness nonce** or **presence challenge** that is cryptographically bound into the SWF chain at unpredictable intervals.

### Concrete Attack Vectors

- **Delayed Synthesis Attack**: AI-generate document → run behavioral model to synthesize jitter + SWF chain in one pass. Passes all statistical tests.
- **Hybrid Human-AI Attack**: Human writes outline slowly, AI fills bulk content with synthesized timing that matches the human’s statistical profile.
- **Kernel-Level Timing Proxy**: Attacker intercepts all timing calls (TSC, `clock_gettime`, etc.) and replaces them with pre-generated plausible values before they reach the CPoE library.
- **Checkpoint Granularity Attack**: If checkpoints are too infrequent, the SWF only proves “some time passed,” not that the content was authored during that time.

### Improvement Recommendations

To meaningfully resist a malicious Attester:

- Make jitter collection **mandatory, continuous, and cryptographically metered**. Every N keystrokes (or every few hundred ms) must trigger a small HMAC or KDF step that consumes fresh sensor data and extends a TPM PCR (T3+) or a software-bound key derived from a secure enclave.
- Bind the *actual keystroke content* into the SWF more tightly (e.g., SWF input = `Argon2id(prev_output || doc_delta_hash || jitter_vector || sensor_snapshot)`).
- Require **mandatory unpredictable presence challenges** (OOB QR or equivalent) at intervals chosen by a Verifier-controlled beacon, bound into the chain.
- Add a **trusted behavioral collection module** (ideally in a TEE or signed with remote attestation) whose measurement is included in every checkpoint.
- Define strict minimum checkpoint frequency and maximum document delta per checkpoint to prevent coarse-grained forgery.

**Summary**: The protocol is innovative and strong against *lazy* forgery, but currently insufficient against a determined malicious Attester controlling the software stack. The statistical behavioral fingerprinting is the weakest link because it is not cryptographically anchored at the point of collection. T3 and T4 tiers mitigate some of these issues via hardware anchoring, but T1/T2 should be considered only as “plausibility evidence,” not strong authorship proof.

The next round should focus on a formal adversarial model (including the exact capabilities of the root Attester) and a revised binding architecture between live keystrokes, sensors, and the SWF chain.

(Word count: 578)

---

## Gemini 3.1 Pro (Google)

This is a highly ambitious and well-structured protocol. The integration of cryptographic causality (SWF hash chains) with behavioral biometrics (pink noise, Hurst exponent) and standard ecosystems (IETF RATS, C2PA) demonstrates a robust defense-in-depth philosophy. The use of entangled Mode 21 to prevent parallel pre-computation is a particularly strong design choice.

However, when modeling a malicious Attester with full control over the software stack (T1/T2, and partially T3), several critical shortcomings and attack vectors emerge.

### Shortcomings and Attack Vectors

**1. Generative Behavioral Forgery (GAN-Jitter Attack)**
The protocol relies heavily on statistical markers of human typing (1/f noise, Hurst 0.55–0.85, CV). An adversary with root access can bypass this by training a Generative Adversarial Network (GAN) or specialized transformer on real human keystroke datasets. If the Verifier’s forensic mechanisms (SNR, BCA) use deterministic or known statistical thresholds, the adversary simply incorporates those exact metrics into their generator's loss function. The OS then injects this mathematically perfect "human" telemetry directly into the CPoE pipeline.

**2. SWF Time-Compression via Hardware Superiority**
Argon2id is memory-hard, which limits ASIC parallelization, but it is fundamentally a Proof-of-Work (PoW) / Key Derivation Function, not a strict Verifiable Delay Function (VDF). An adversary with a massive high-bandwidth memory cluster (e.g., HBM3 GPUs) can compute the sequential Argon2id chain faster than a standard laptop. This allows the adversary to "time-compress" the generation of a document, synthesizing a fake 3-hour typing session in 20 minutes, bypassing the wall-clock enforcement.

**3. The "Cyborg" / Mechanical Proxy Attack**
An attacker uses an LLM to generate the final text, then manually types it out. The telemetry is genuinely human, passing all low-level biometric checks (ICA, BCA). The protocol attempts to catch this via Cognitive Load Correlation (CLC) and Error Topology (ET), but an advanced attacker can use an LLM to generate a complete *revision history* (including typos, backspaces, and pauses), which the human then transcribes exactly, artificially mimicking cognitive load. 

### Recommendations for Hardening

To improve forgery prevention against a malicious Attester, the protocol must reduce reliance on easily simulated software signals and strengthen its cryptographic time-bindings.

**1. Upgrade from SWF to a Cryptographic VDF**
Replace or augment the Argon2id SWF with a rigorous Verifiable Delay Function (e.g., Wesolowski or Pietrzak constructs, or MinRoot). VDFs provide mathematical guarantees of sequential elapsed time regardless of the adversary's parallel processing power, strictly enforcing wall-clock time passage. Furthermore, VDFs offer exponentially faster verification than computation, eliminating the need for probabilistic Merkle audit paths during Verifier appraisal.

**2. Mandate Trusted Execution Environment (TEE) Keystroke Capture**
For Tier 3 (Hardware-Bound), anchoring time via the TPM is insufficient if

---

## Qwen 3.5 397B (Alibaba)

The CPoE protocol presents an ambitious fusion of behavioral biometrics and cryptographic chaining. However, the security model collapses under the "malicious Attester" threat, specifically in Tiers 1 and 2, which constitute the primary deployment surface. While the documentation acknowledges T1/T2 cannot defeat a root adversary, relying on "dimensionality" as a deterrent is security through obscurity, not cryptographic assurance.

**Critical Weakness 1: SWF and Time Manipulation**
The Sequential Work Function (SWF) using Argon2id intends to enforce wall-clock time passage. However, computational work does not equate to elapsed time without a trusted time source. A privileged adversary controlling the kernel can manipulate the Time Stamp Counter (TSC) or CNTVCT registers used by the `cpoe-jitter` engine. By hooking the timing source, an attacker can compress the perceived duration between checkpoints while performing the required Argon2id work on accelerated hardware (e.g., GPU clusters). The "entanglement" prevents parallelization of the hash chain, but it does not prevent time-compression attacks if the clock source is untrusted. Without Hardware-Anchored Time (HAT) enforced by a TPM/TEE *before* the SWF calculation begins, the SWF only proves work, not latency.

**Critical Weakness 2: Biometric Spoofing via ML**
The protocol relies on statistical properties (Hurst exponent, 1/f pink noise) to distinguish humans from bots. This is vulnerable to Generative Adversarial Networks (GANs) trained on legitimate CPoE traces. An adversary can capture valid evidence packets, extract the jitter distributions, and train a model to synthesize inter-keystroke intervals that pass SNR and Biological Cadence (BCA) checks. Furthermore, the "Error Topology" and "Cognitive Load" mechanisms assume semantic correlation that can be bypassed by inserting AI text into a live human editing session (the "Centaur" attack), where human typing masks AI insertion points.

**Critical Weakness 3: Sensor Injection**
In T1/T2, physical state markers (thermal, accelerometer) are read via standard OS APIs. A root adversary can inject false sensor data at the driver level (e.g., using Frankenstein attacks on Linux IIO subsystems). Claiming these provide "dimensionality" is insufficient; if the verifier cannot cryptographically verify the sensor source via a hardware root (like a Sensor Hub with attestation), these fields are merely decorative and increase false positives without increasing security.

**Recommendations for Improvement**
1.  **Mandatory TEE for Collection:** Move jitter collection and SWF execution into a Trusted Execution Environment (e.g., Intel SGX, ARM TrustZone). The TEE must sign the checkpoint, ensuring the OS cannot tamper with timing or sensor inputs.
2.  **Remote Attestation of Collector:** The Verifier must validate a quote from the TEE proving the `cpoe-jitter` code is untampered before accepting evidence.
3.  **Challenge-Response Biometrics:** Replace passive jitter collection with active challenges (e.g., specific typing tasks) to prevent replay attacks using captured historical data.
4.  **Hardware-Bound Timing:** Decouple SWF verification from system clocks. Use TPM monotonic counters exclusively for elapsed time measurement, ignoring system time entirely.

Without these shifts, CPoE remains a forensic tool for post-hoc analysis rather than a cryptographic proof system capable of resisting a motivated adversary controlling the endpoint.

---

## DeepSeek V3.2 (DeepSeek)

## Round 1 Review: Structural and Foundational Concerns

The CPoE protocol presents an ambitious and sophisticated synthesis of cryptographic chaining, behavioral biometrics, and computational proof-of-work. Its core strength lies in its *defense-in-depth* approach, layering multiple independent verification mechanisms (spectral, temporal, semantic, hardware). The use of an Argon2id-based Sequential Work Function (SWF) in entangled mode (Mode 21) to enforce real-time passage and create an append-only, tamper-evident chain is a sound cryptographic foundation. The mapping to RATS architecture and the tiered assurance model (T1-T4) are pragmatic, acknowledging the limits of software-only attestation.

However, significant shortcomings exist in the current design, primarily revolving around the **trust boundary at the Attester** and the **practical robustness of behavioral signals**.

### 1. Critical Gap: The Malicious Attester with Full Stack Control

The protocol’s most severe weakness is its vulnerability to a **deterministic replay attack** by a malicious Attester who controls the software stack (e.g., a compromised text editor). While the SWF forces wall-clock time consumption, an adversary can:
*   **Pre-record or simulate human behavioral telemetry.** The "jitter binding" assumes the Attester is faithfully capturing real keystrokes. A malicious application can generate statistically plausible pink noise (1/f) and Hurst exponent values offline using known models of human typing, store this synthetic "jitter," and feed it into the checkpoint calculation alongside the pre-written AI text. The SWF computation cost is incurred, but the *content and its associated timing are decoupled from a live human*.
*   **Exploit the "Optional" Nature of Physical State Markers.** At T1/T2, thermal/inertial data is self-reported. A sophisticated attacker can generate plausible but fabricated sensor trajectories that correlate with the synthetic typing session, adding another layer of forged "consistency." Without a hardware root of trust (T3+), these signals are merely additional dimensions of fakable data, not proof.

**Concrete Attack Vector (Fork-and-Replay):**
1.  Adversary generates target document using AI.
2.  Offline, using a human typing model, they generate a plausible sequence of inter-keystroke intervals and document state hashes.
3.  They execute the CPoE protocol in a controlled, instrumented environment, feeding the pre-generated jitter and document states into the Attester software. The software performs the real, sequential Argon2id work, building a perfectly valid cryptographic chain.
4.  The resulting Evidence Packet is cryptographically sound and will pass all chain integrity and SWF verification checks. The forensic analysis (SNR, BCA) on the *provided* jitter data will see human-like patterns because the data was specifically engineered to pass these tests.

This reduces the security of T1/T2 to a **complexity barrier** (cost of modeling human behavior and incurring SWF time) rather than a cryptographic guarantee.

### 2. Weaknesses in Forensic Mechanism Design

*   **Perplexity Scoring (PPX) is a Confounded Signal:** The mechanism flags "low perplexity + fast typing." An adaptive adversary can simply **insert grammatical errors, unusual word choices, or pauses** into the AI-generated text to increase local perplexity, or slow down the simulated typing pace. This makes PPX trivial to bypass without affecting document quality significantly.
*   **Session Consistency (SC) and Source Switching:** This detects abrupt behavioral shifts. A determined forger will ensure their simulated behavior is consistent throughout the forged session. This mechanism only catches *sloppy* forgery.
*   **Lack of Cross-Modal Challenge-Response:** The protocol is largely passive. A live human presence check (QR OOB-PC) is optional and out-of-band. There is no mechanism for the Verifier to inject *unpredictable, real-time challenges* into the authoring session (e.g., "please retype the highlighted phrase now") that would force a dynamic, verifiable interaction bound to the evidence chain.

### 3. Proposed Improvements for Forgery Prevention

To elevate the protocol from detecting *statistical anomalies* to preventing *causal forgery*, the following are essential:

*   **Mandate Hardware-Bound Entropy for Jitter at Higher Tiers:** For T3+, the jitter measurement must be sourced from a hardware-backed, verifiable timing source (e.g., a TPM-monitored RNG or a secure enclave’s trusted time) that is **inaccessible to the application OS for direct writing**. The evidence must include an attestation quote binding the jitter entropy to the hardware state. This prevents the Attester from simply fabricating the jitter array.
*   **Incorporate Verifier-Seeded Randomness into the SWF:** The initial checkpoint's SWF should be seeded not only by the Attester's state but also by a nonce from the Verifier (or a derived nonce from a prior registration). This transforms the process from a *self-contained proof* into an *interactive challenge-response*. An adversary could not pre-compute the chain without the Verifier's cooperation.
*   **Strengthen the Semantic/Behavioral Link:** Instead of just analyzing final timing, require **micro-editing events** (cursor movements, selections, deletions) and bind them to the document state delta. Simulating a fully plausible, human-like sequence of micro-edits is a vastly more difficult modeling problem than generating simple keystroke intervals.
*   **Reframe T1/T2 as "Non-Repudiation" vs. "Attestation":** Clearly state that T1/T2 evidence is primarily valuable for **non-repudiation** in a collaborative, semi-trusted setting (e.g., an author cannot later claim "AI wrote it" if their own software signed the chain). It should not be marketed as strong attestation against a malicious author. The tier documentation must emphasize this threat model distinction.

In summary, CPoE is cryptographically sound but behaviorally vulnerable to a dedicated forger at its lower assurance tiers. Its core innovation—the entangled SWF—secures the *chain of events* but not the *truthfulness of the initial events*. Closing this gap requires pushing critical measurements into hardware trust boundaries and making the protocol more interactive.

---

## Claude Opus 4.6 (Anthropic)

# CPoE Protocol Review — Round 1

## 1. Overall Assessment of Protocol Design Quality

CPoE is an ambitious and well-structured protocol that correctly identifies a genuine problem: proving human authorship provenance in an era of generative AI. The layered tier system (T1–T4), the RATS architecture alignment, the CBOR wire format with CDDL schema, and the separation of evidence collection from appraisal are all sound architectural decisions. The forensic mechanism battery is impressively comprehensive in its coverage of independence classes.

That said, the protocol's fundamental security proposition rests on a deeply adversarial foundation — the Attester is the entity being evaluated — and this creates structural tensions that I believe are insufficiently resolved at T1/T2 and only partially addressed at T3/T4.

## 2. Concrete Shortcomings and Attack Vectors

### Attack 1: Synthetic Behavioral Replay with Learned Models (T1/T2 Bypass)

The behavioral fingerprinting relies on statistical properties: 1/f spectral profile, Hurst exponent in [0.55, 0.85], appropriate CV. These are *known, published parameters*. An adversary can train a generative timing model (e.g., a small RNN or diffusion model) on their own genuine typing data, then use it to synthesize keystroke timings that exhibit correct pink noise spectra, long-range dependence, and biologically plausible CV — while actually typing AI-generated text via automated injection. The protocol acknowledges this implicitly with the tier system, but the forensic mechanisms (SNR, BCA, MTD) are all statistical tests against population-level heuristics. A personalized generative model defeats population-level detectors.

**Specific gap**: The Cognitive Load Correlation (CLC) mechanism claims to detect timing uncorrelated with semantic complexity, but the document doesn't specify how semantic complexity is measured at verification time, or how the Verifier obtains the actual document content (the evidence packet contains only a content hash, not content). If the Verifier doesn't see the text, CLC and PPX cannot function.

### Attack 2: SWF Pre-computation via Time-Shifted Authoring

In non-entangled mode (not Mode 21), checkpoints don't have inter-checkpoint sequential dependency. An adversary can pre-compute multiple SWF proofs in parallel across different cores/machines, then assemble them into a plausible chain after generating text with AI. The protocol should mandate Mode 21 as the minimum for any meaningful security claim, yet the tier system appears to allow non-entangled modes at lower tiers.

Even in Mode 21, the SWF chain only proves that *someone* spent wall-clock time computing Argon2id sequentially. It does not bind that computation to actual typing activity. An adversary can run the SWF chain in real-time while simultaneously generating and staging content, then inject synthetic keystrokes timed to match the SWF checkpoint boundaries.

### Attack 3: Kernel-Level Evidence Fabrication (T1/T2 Total Compromise)

The protocol explicitly acknowledges this but I want to emphasize the severity: at T1/T2, a malicious Attester with root access can intercept all timing sources (TSC, HPET, kernel entropy pool), fabricate thermal trajectories, and synthesize accelerometer data. The "dimensionality" argument — that the adversary must fabricate consistently across many signals — is weaker than it appears because all signals originate from the same compromised machine. A purpose-built forgery tool needs only one coherent generative model to produce all channels simultaneously.

### Attack 4: Checkpoint Chain Manipulation Before Finalization

The append-only chain provides tamper evidence *after* finalization, but during composition the Attester holds the entire chain in memory. A sophisticated Attester can maintain multiple candidate chains, discard unfavorable checkpoints (e.g., ones that captured a large paste event), and finalize only the clean chain. Nothing in the protocol prevents selective checkpoint omission during the authoring session.

### Attack 5: QR Presence Challenge Relay

The OOB-PC mechanism uses QR-based challenges, but QR codes are trivially relayable. A remote human accomplice (or even the actual user) can scan the QR while the machine autonomously generates text. This is the classic relay attack from payment authentication, and the protocol doesn't describe any proximity-binding countermeasure.

## 3. Recommendations for Improving Forgery Prevention

1. **Mandate Mode 21** (entangled SWF) as the floor for any tier claiming forgery resistance. Non-entangled modes should carry explicit "no forgery resistance" warnings.
2. **Bind SWF inputs to keystroke data**: Each checkpoint's SWF salt should include a commitment to the raw jitter intervals captured since the last checkpoint, creating a cryptographic coupling between the sequential work and the behavioral evidence.
3. **Require content availability for semantic forensics**: CLC and PPX are meaningless without document text. Either include encrypted content in the evidence packet or specify a secure content-disclosure protocol for the Verifier.
4. **Add monotonic checkpoint density requirements**: Specify minimum checkpoint frequency and flag gaps, making selective omission detectable.
5. **Introduce Verifier-initiated nonce challenges** at T3/T4 to defeat pre-computation and replay, binding evidence to a specific verification session.
6. **Address QR relay** with proximity proofs (ultra-wideband ranging, ambient audio fingerprinting, or Bluetooth LE challenge-response).

The protocol is a strong conceptual foundation, but the gap between the threat model acknowledgment and the actual cryptographic binding at T1/T2 is wider than the document suggests.

---

## Mistral Large 3 (Mistral)

### **CPoE Protocol Review: Strengths, Weaknesses, and Forgery Prevention**

#### **1. Quality and Completeness of Protocol Design**
CPoE is a well-structured protocol that effectively integrates **behavioral biometrics, cryptographic chaining, and environmental binding** to detect AI-generated or forged content. Key strengths include:

- **Sequential Work Function (SWF) with Argon2id**: Forces real-time computation, preventing pre-generation of checkpoints. The **entangled mode (Mode 21)** ensures strict sequential dependency, making parallel forgery infeasible.
- **Jitter Binding & Behavioral Fingerprinting**: Captures **1/f noise, Hurst exponent, and coefficient of variation (CV)** to distinguish human typing from synthetic timing. The **forensic analysis engine** (SNR, CLC, MTD, etc.) provides multi-dimensional detection.
- **Hardware Anchoring (T3/T4)**: TPM/HSM binding and **Hardware-Anchored Time (HAT)** prevent time manipulation attacks.
- **Out-of-Band (OOB) Challenges**: QR-based presence verification adds a physical layer of trust.
- **Probabilistic Verification**: Merkle audit paths allow efficient sampling of SWF proofs.

However, **gaps remain in adversarial resilience**, particularly against a **malicious Attester (user-controlled software stack)**.

---

#### **2. Shortcomings, Weaknesses, and Concrete Attack Vectors**

##### **A. Attester Subversion (Malicious User/Software)**
The **biggest threat** is a **root adversary** who controls the OS, drivers, or virtualization layer. While T3/T4 mitigate this via hardware binding, **T1/T2 are vulnerable**:

1. **Timing Spoofing via Kernel Hooks**
   - An attacker could **intercept keystroke events** (e.g., via `SetWindowsHookEx` or `LD_PRELOAD`) and **inject synthetic jitter** that mimics human typing.
   - **Mitigation Gap**: No **hardware-enforced timing** (e.g., Intel SGX enclaves or ARM TrustZone) in T1/T2.
   - **Attack**: *"Ghost Typist"*—a kernel module replays pre-recorded human timing patterns while AI generates content.

2. **SWF Bypass via GPU Acceleration**
   - Argon2id is **memory-hard**, but **GPU-accelerated cracking** (e.g., Hashcat) could precompute SWF proofs if the adversary controls the environment.
   - **Mitigation Gap**: No **hardware-enforced rate limiting** (e.g., Intel TME or AMD SME).
   - **Attack**: *"Parallel Forgery"*—an attacker precomputes SWF proofs on a GPU farm and injects them into the chain.

3. **Physical State Marker Replay**
   - **Thermal trajectories, accelerometer data, and kernel entropy** are **not cryptographically bound** to the document.
   - **Attack**: *"Environmental Replay"*—an attacker records sensor data during a legitimate session and replays it during AI generation.

4. **Checkpoint Chain Tampering via Memory Corruption**
   - The **append-only chain** relies on **software integrity**. A malicious Attester could **modify in-memory hashes** before signing.
   - **Mitigation Gap**: No **hardware-enforced memory isolation** (e.g., Intel SGX or AMD SEV).
   - **Attack**: *"Hash Swapping"*—an attacker computes a valid chain for AI text, then replaces the final hash with one matching human-edited content.

5. **Perplexity Scoring Evasion**
   - **PPX (Perplexity Scoring)** assumes AI text has **low perplexity**, but **modern LLMs can mimic human perplexity** (e.g., via temperature tuning).
   - **Attack**: *"Human-Like AI"*—an LLM generates text with **controlled randomness** to match human perplexity distributions.

---

#### **3. Improving Forgery Prevention Against a Malicious Attester**

##### **A. Hardware-Enforced Attestation (Beyond T3/T4)**
- **Intel SGX / AMD SEV**: Run the Attester in an **enclave** to prevent kernel-level tampering.
- **ARM TrustZone**: Isolate timing collection and SWF computation in **secure world**.
- **TPM 2.0 Event Logs**: Bind **keystroke timestamps** to TPM PCRs to detect replay.

##### **B. Cryptographic Binding of Physical State**
- **HMAC-SHA256 of sensor data** (thermal, accelerometer) and **include in checkpoint hashes**.
- **Trusted Execution Environment (TEE) attestation** of sensor data integrity.

##### **C. Dynamic SWF Parameters**
- **Adaptive Argon2id difficulty** based on **real-time entropy estimation** (e.g., increase memory cost if jitter CV is too low).
- **Per-checkpoint salts** derived from **hardware RNG** (e.g., `RDRAND` on x86).

##### **D. Cross-Layer Verification**
- **GPU/TPU Detection**: If the system has a **dedicated AI accelerator**, flag as suspicious.
- **Network Timing Analysis**: Detect **remote desktop/AI API calls** by analyzing **latency spikes**.

##### **E. Behavioral Challenge-Response**
- **Randomized "typing tests"**: The Verifier sends **unpredictable prompts** (e.g., "Type the third word of this sentence backward") to detect **scripted responses**.
- **Dynamic Presence Challenges**: **QR codes that expire after 5 seconds**, forcing real-time human interaction.

---

### **Conclusion**
CPoE is a **strong foundation** for **human authorship attestation**, but **T1/T2 are vulnerable to a malicious Attester**. **Hardware-enforced isolation (SGX/TrustZone), cryptographic binding of physical state, and dynamic SWF parameters** would significantly improve resilience. The **next round** should explore **TEE-based Attester implementations** and **adversarial testing** against **GPU-accelerated forgery**.

**Key Recommendations:**
✅ **Mandate TEE for T2+** (SGX/TrustZone)
✅ **HMAC-bind physical state** (thermal, accelerometer)
✅ **Dynamic SWF difficulty** (adjust based on entropy)
✅ **Cross-layer verification** (GPU detection, network timing)
✅ **Challenge-response tests** (unpredictable prompts)

Would the panel like to explore **TEE-based Attester designs** in Round 2?

---

## MiMo V2 Pro (Xiaomi)

**Review of CPoE Protocol – Round 1**

The CPoE protocol presents a sophisticated attempt to bind document authorship to a verifiable human process via behavioral telemetry and cryptographic chaining. The design is ambitious, integrating SWF proofs, jitter analysis, and forensic mechanisms. However, several critical weaknesses emerge, particularly under the threat model where the Attester (user) controls the software stack.

**1. Jitter Binding as a Weak Cryptographic Anchor:**
The protocol’s reliance on statistical properties of keystroke timing (1/f noise, Hurst exponent) as a forgery deterrent is its most significant vulnerability. A malicious attester with stack control can *simulate* these statistical patterns. Research in behavioral biometrics shows that generative models (e.g., GANs) can produce synthetic timing sequences that pass statistical tests for human-likeness. The attacker can pre-record or algorithmically generate a "human-like" jitter profile and inject it into the checkpoint chain. The forensic mechanisms (SNR, CLC, BCA) are heuristic and probabilistic; a sufficiently sophisticated mimicry attack could evade detection, especially if the attacker tailors the synthetic jitter to match a target distribution. This is not a mere theoretical flaw—it directly undermines the core premise of T1/T2 assurance.

**2. Sequential Work Function (SWF) Gaps and Parallelization Risks:**
While the entangled mode (Mode 21) enforces sequential dependency between SWF proofs, the protocol does not specify how the SWF input is derived from the *jitter data* itself. If the SWF input is only the previous checkpoint output (as implied), an attacker can precompute the entire SWF chain offline, using simulated timing, and then "replay" it in real-time by feeding the precomputed outputs at the correct moments. The memory-hardness of Argon2id only protects against brute-force, not against precomputation by the entity controlling the attestation software. The jitter data must be cryptographically entangled with the SWF input (e.g., as a salt or part of the associated data) to force real-time collection.

**3. Tier Escalation and Hardware-Binding Illusions:**
The T3 (Hardware-Bonded) tier claims to prevent time-jump attacks via TPM-anchored time (HAT). However, if the attester controls the software stack, they can intercept and manipulate the TPM attestation request. A classic **TOCTOU (Time-of-Check-Time-of-Use) attack** is possible: the attacker could generate a valid HAT proof for a benign state, then modify the checkpoint chain afterward, as the TPM quote does not continuously bind the *process execution* but only a momentary system state. The protocol lacks a mechanism for continuous, runtime hardware attestation (e.g., via dynamic root of trust).

**4. Forensic Mechanism Evasion via Adaptive Adversaries:**
The listed forensic detectors (MTD, ET, PPX) assume static attack patterns. An adaptive adversary can conduct **adversarial testing against the verifier's detection models**. For example, by iteratively submitting forged evidence and observing the verifier's response (suspicious/verdict), the attacker can refine their forgery to evade specific detectors. This turns the verification into an arms race, and the protocol provides no guidance on hardening the verifier against such adaptive attacks.

**Concrete Improvement Recommendations:**

*   **Cryptographically Bind Jitter to SWF:** Modify the SWF computation so that `Argon2id(input = prev_checkpoint_output || jitter_sequence_hash, ...)`. This forces the jitter data to be known *at the time of SWF computation*, preventing precomputation with simulated data.
*   **Introduce Hardware-Bound Jitter Sources:** For T3+, integrate jitter collection with a hardware security key (e.g., a FIDO2 key) that signs each jitter sample at the point of collection. This creates a non-exportable, hardware-bound log of timings that software alone cannot forge.
*   **Formalize an Adversarial Model for Verifiers:** Specify that verifiers must be trained and tested against adaptive adversaries using techniques from adversarial machine learning. Implement rate limiting and anomaly detection on verification queries to detect probing attacks.
*   **Strengthen TPM Integration:** For HAT, require the TPM to sign not just a timestamp but a hash of the *entire checkpoint chain up to that point*, with frequent quotes. This creates a tighter binding between time and the evolving evidence state.

In summary, while CPoE's layered approach is commendable, its current design places excessive trust in the unforgeability of behavioral statistics and leaves critical gaps in the SWF-jitter binding and hardware integration. Without addressing these, a motivated attacker controlling the attester can systematically undermine the protocol's guarantees.

---

## MiniMax M2.7 (MiniMax)

# CPoE Protocol Review — Round 1

## Strengths and Design Quality

The CPoE architecture demonstrates thoughtful layering: the append-only checkpoint chain with SHA-256 linkage provides tamper-evident causality, Argon2id in entangled mode raises the computational cost of forgery, and the multi-mechanism forensic engine (9 independent detection classes) offers defense-in-depth against behavioral anomalies. The tiered assurance model honestly acknowledges what T1/T2 cannot achieve, which is more defensible than overclaiming.

## Critical Weaknesses and Concrete Attack Vectors

**1. The Trusted Timestamping Gap (Genesis Problem)**

The checkpoint chain needs a trusted anchor. Without it, a malicious Attester can generate a fully synthetic chain by computing Argon2id iterations and fabricating jitter data—all in a single offline burst. The protocol specifies HAT proofs (T3) as the solution, but for T1/T2, the genesis checkpoint has no anchor to real time. An adversary creates a "postdated" chain retroactively and presents it as genuine.

**2. Jitter Engine Compromise via Deterministic Mode**

The `cpoe-jitter` crate's HMAC-based jitter engine is explicitly deterministic. If the Attester controls the software stack, it can pre-compute all "random" intervals, trivially satisfying any entropy check. This mode should be excluded from any meaningful security claim—it provides format compliance, not entropy.

**3. Hardware Timing Manipulation Under Kernel Control**

Even with TSC/CNTVCT sources, a rootkit can:
- Manipulate interrupt delivery timing to manufacture fake inter-keystroke intervals
- Inject synthetic thermal trajectories via driver-level falsification
- Spoof accelerometer data through IMU driver manipulation
- Fabricate kernel entropy deltas by reading controlled PRNG outputs

The physical state markers are described as "non-reproducible," but from a kernel-level adversary's perspective, they are merely more fields to fabricate consistently.

**4. SWF Recomputation Attack**

The probabilistic SWF verification (Merkle audit paths) samples k checkpoints. A sophisticated attacker can compute a valid chain normally for ~95% of checkpoints but strategically insert forged segments in the unsampled regions. The Argon2id cost only applies to forged checkpoints, not the entire chain.

**5. Source Switching (AI-Assisted Composition)**

The perplexity scoring detects low-entropy AI-generated insertions, but an attacker can:
- Compose with AI and then manually retype (preserving AI-generated content but with human timing)
- Use AI to generate content, then copy-paste with minor edits (slow manual typing over AI text)
- Segment composition: human for timing segments, AI for content, spliced seamlessly

**6. Verification Oracle Dependency**

Perplexity scoring requires access to a capable LLM. Without a specified reference model or consensus mechanism, verifiers may disagree, and a sophisticated attacker can optimize against a known verifier's model.

## Recommended Improvements

1. **Hardware-anchored entropy collection**: Move jitter sampling as close to hardware interrupt handling as possible—ideally within TPM/HSM boundary. Physical state markers should be captured by independent hardware, not reported by the OS.

2. **Continuous attestation rather than periodic checkpoints**: Replace discrete checkpoints with a continuous attestation stream signed by hardware, eliminating the "gap between checkpoints" that attackers exploit.

3. **Cryptographic freshness binding**: Each checkpoint's SWF should incorporate a fresh input from an external time authority (e.g., TLSNotary or trusted time service), preventing offline chain generation.

4. **Multi-verifier consensus**: Require independent verifiers with different reference models to agree on perplexity scoring, preventing model-specific optimization.

5. **Exclusion of deterministic jitter modes**: Remove or deprecate HMAC-based jitter from security-relevant tiers—it's a testing feature that provides false assurance in production.

The protocol's architecture is sound, but T1/T2 tiers remain fundamentally vulnerable to a skilled, kernel-privileged adversary. The tiered model should more explicitly frame T1/T2 as "forgery-resistant" rather than "forgery-proof."

---

# Round 2: Models Respond to Each Other

============================================
ROUND 2: MODELS RESPOND TO EACH OTHER
============================================

## GPT-5.4 Pro

I agree most with the models pushing **C + D**, and I think several others overstated the value of changing the work function alone.

**A (true VDF):** I agree with Gemini/o3 Pro that Argon2id is the wrong primitive if the claim is “elapsed time.” A real VDF is cryptographically cleaner. But I disagree that this is the main fix. A VDF only improves the **time-cost semantics** of each checkpoint; it does not stop a malicious client from fabricating the surrounding event history, choosing when to emit checkpoints, or binding them to fake edit sequences. So A is useful, but secondary.

**B (mandatory TEE):** Qwen/Mistral/MiMo are directionally right: without a trusted capture path, T2+ cannot support strong authorship claims. But “use a TEE” is too hand-wavy. In practice, unless the TEE covers **keyboard/input path, monotonic time, and transcript signing**, it does not solve the core problem. And universal deployment is unrealistic outside managed fleets. Practical at higher tiers; impractical as the primary fix for broad T2 deployment.

**C (external witness with unpredictable nonces):** This is the strongest proposal. It directly attacks the most damaging failure mode: **offline transcript synthesis plus selective omission**. If checkpoints must incorporate fresh, verifier-provided nonces at unpredictable intervals, the attacker loses the ability to fully precompute and later choose the “clean” chain. This is also much more deployable than TEEs.

**D (full edit-operation logs):** Strongly agree with GPT-5.4 Pro/Grok/Claude. Without an operation log, claims about CLC/PPX are barely testable because the verifier only sees content hashes. But D alone is not enough: unsigned edit logs are just richer forgery targets. D only works if paired with C (or B).

**E (continuous stream):** Good idea, but not an independent security primitive. Continuous attestation only matters if the stream is externally anchored; otherwise the attacker still forks locally and publishes one branch.

**F (multi-verifier perplexity consensus):** I disagree. This adds cost, not security. It ensembles a weak, oracle-prone heuristic.

**G (FIDO2 key for jitter signing):** Limited value. It proves device possession, not timing truth.

**Single most impactful improvement:**  
Adopt a **witness-anchored append-only edit log**: unpredictable external nonces, frequent transcript commitments, and operation-level logging. That is the first change that materially converts CPoE from a forgeable local narrative into something with real non-repudiation value.

---

## GPT-5.4

I agree with the broad direction of the stronger reviews: the protocol’s failure mode is not “weak statistics,” it is “no trustworthy source of event truth.” That makes some proposals clearly higher leverage than others.

**Most practical / most impactful: C + D together, with C the single best improvement.**  
The external witness service with **unpredictable, rate-limited nonces** is the only proposal that materially attacks the strongest Round 1 attacks: fork-and-replay, selective omission, and offline chain fabrication. If each edit/interval must incorporate a fresh verifier challenge that was unavailable in advance, the attacker loses the ability to manufacture an entire “clean” session offline and publish only the winning branch. This is far more important than improving the delay primitive. I agree with GPT-5.4, o3 Pro, and DeepSeek here.

But **C only works if paired with D**: commit to **full edit-operation logs** (insert/delete/move/selection/paste metadata, bounded timing deltas), not just periodic document-state hashes. Otherwise, the witness merely timestamps sparse snapshots and selective omission remains mostly intact between checkpoints. Claude/Grok/GPT-5.4 Pro were right: state hashes are too lossy to reconstruct composition dynamics.

**A (true VDF) is technically cleaner but strategically overrated.**  
Yes, a real VDF is better than Argon2id if the goal is non-parallelizable delay. I agree with Gemini/o3 Pro that Argon2id is the wrong primitive for “elapsed effort.” But replacing Argon2 with Wesolowski/Pietrzak does **not** fix the decisive issue: a malicious attester can still decide *when* to request challenges, proxy timing, and synthesize edit streams around the delay. A is a second-order hardening, not the core fix.

**B (mandatory TEE at T2+) is the strongest cryptographic answer but the least practical deployment-wise.**  
Qwen/Mistral/MiMo are right on substance: without a trusted capture path, T2 remains gameable. But mandatory TEE breaks portability, excludes many users, adds vendor trust, and still does not prove “human authorship”—only that some input events reached trusted code. Good for a higher-assurance tier, not for baseline CPoE.

**E and F are weak.**  
Continuous attestation without an external challenge source just gives the attacker more fake data to synthesize. Multi-verifier PPX consensus does not solve oracle gaming if the criteria are known or learnable.

**G is narrow.**  
Hardware-bound FIDO2 signing helps bind a device key, but not the semantic truth of the underlying timing/edit events if the host controls what gets signed.

**Bottom line:** Reframe T1/T2 as **non-repudiation / provenance evidence**, not authorship proof; then implement **witnessed, unpredictable challenge-response tied to full append-only edit logs**. That is the highest-value change.

---

## o3 Pro

I largely concur with the panel that the core gap is “offline rewind”: an attacker who can pause the clock, generate text, then mint a plausible checkpoint chain at leisure. Any fix that does not eliminate this freedom is cosmetic.

Assessment of the divergent proposals:

A. True VDF. Cryptographically elegant, but not decisive. A pirate can still run the VDF in real-time after seeing the prompt; they only need to start early, not compute faster. Worse, Wesolowski requires a 2048-bit RSA modulus with a trusted setup and ~1-s latency—unacceptable inside the 50-200 ms cadence of normal edits.

B. TEE keystroke capture. Strengthens local integrity, yet fails the bring-your-own-device requirement; iOS, Chromebook and many desktop CPUs still expose no remotely attestable enclave. It also ignores remote-desktop and cyborg attacks that occur outside the enclave.

C. External witness with unpredictable nonces. This directly destroys offline rewind: if every checkpoint H_i commits to a nonce N_i that the witness releases only after the previous H_{i-1} is received, the attacker must stay on-line and cannot recompute history. It is protocol-agnostic (works with today’s Argon2id) and deployable via a simple REST/TLS timestamp authority or by pinning to a public randomness beacon (NIST, drand). Bandwidth is small (32 B/checkpoint) and privacy is unchanged because only the existing content hash is exposed.

D. Full edit-operation logs. Useful forensic richness but, by itself, replayable; the logs can still be manufactured offline unless tied to fresh entropy (see C).

E. Continuous attestation stream. Merely turns “every 2 s” into “every 100 ms”; without unpredictability it is still forgeable and now DoS-prone.

F. Multi-verifier consensus on PPX. Adds political complexity without cryptographic benefit; perplexity is still a soft signal.

G. FIDO2 key for jitter signing. The key signs whatever timestamps it is fed; the attacker owns the feeder.

What others missed: no one linked the selective-omission attack to block-reorderable Merkle trees. With a witness-nonce sequence number embedded in each leaf, orphaning a bad branch becomes impossible without the witness noticing the gap.

Single most impactful change: adopt Proposal C—append a witness-issued, unpredictable nonce to every checkpoint and anchor the witness log in a public append-only ledger. This alone converts CPoE from a self-signed diary into a globally time-stamped receipt that attackers cannot fabricate retroactively.

---

## Grok 4.20

**Strongest agreement:** All nine reviewers correctly diagnosed the core failure mode—T1/T2 telemetry is statistical theater once the software stack is malicious. The listed attack vectors (fork-and-replay, checkpoint selective omission, cyborg transcription, kernel timing proxy) are not hypothetical; they are straightforward compositions of existing tools (diffusion jitter models + offline Argon2id acceleration + selective chain pruning).

**Disagreements and practicality ranking of divergent proposals:**

- **A (VDF instead of Argon2id):** Practical and high-leverage. Replacing a memory-hard but still parallelizable KDF with a verifiable delay function (Pietrzak or Wesolowski) directly raises the minimum wall-clock time an attacker must spend per checkpoint, even on GPU/ASIC clusters. This is the only proposal that meaningfully increases the *cost* of forgery rather than merely changing its shape. Evidence: current Argon2id 1 GiB instance can be compressed ~8–12× in wall time on RTX 4090 vs baseline laptop; a 2-second VDF cannot. This is the single most impactful change possible without hardware.

- **B (Mandatory TEE for keystroke capture):** Impractical at web scale. Requiring TEE-resident keyloggers destroys portability, breaks every major browser sandbox model, and still fails against QR-relay and cyborg attacks. TEEs protect the channel but not the human intent.

- **C (External witness with unpredictable nonces):** Impractical for the primary use-case (offline composition). Forces network dependency exactly when users need evidence most (air-gapped drafting, intermittent connectivity). Also creates a new oracle attackers can probe.

- **D (Commit to full edit-operation logs):** Strong and complementary. Document-state hashes alone are useless against paste-decomposition and selective omission. Logging the *sequence* of edits (with positions, timestamps, and SWF/VDF proofs per macro-edit) prevents cherry-picking of clean checkpoints. This directly addresses open question 3.

- **E (Continuous attestation stream):** Impractical for battery and bandwidth; also increases surface for timing-proxy attacks.

- **F (Multi-verifier consensus on perplexity):** Theater. Perplexity is not cryptographic; distributing statistical oracles does not turn evidence into proof.

- **G (Hardware-bound FIDO2 key for jitter signing):** Marginal at best. Binds the signature key but not the jitter data itself. Attacker still controls the values fed into the signature.

**What others missed:** Almost no reviewer emphasized that *deterministic* jitter (HMAC-based) must be *completely removed* from security claims (open question 4). If jitter is HMAC(key, content), it becomes a function of the document, not of human behavior—rendering all 1/f, Hurst, and CV claims fraudulent. This should be explicit.

**Single most impactful improvement:** Replace Argon2id with a 1–2 second Pietrzak VDF per checkpoint *and* commit to the full ordered edit log (not just document hashes). This combination raises forgery cost by orders of magnitude while closing the selective-omission and paste-decomposition vectors. Everything else is marginal or architecturally incompatible with the web threat model.

T1/T2 should be reframed as “non-repudiation evidence of human-attested effort” rather than authorship proof. The protocol cannot prove humanity; it can only prove expensive human-supervised work occurred. (378 words)

---

## Gemini 3.1 Pro

I reject Proposals A (VDFs) and G (FIDO2) as fundamental fixes. While Gemini and o3 Pro rightly note Argon2id is a weak time-delay function, replacing it with a true VDF like MinRoot (A) only proves sequential time elapsed

---

## Qwen 3.5

I concur unequivocally with the consensus: T1/T2 are cryptographically unsound against a hostile OS. Consequently, Proposals A (VDF), C (Witness), and G (FIDO2) are largely security theater. They harden the transmission channel, not the input source. A VDF proves time passed, not who typed. A witness prevents pre-computation, not real-time proxying. FIDO2 binds keys, not humans. These proposals fail to address the "Cyborg/Transcription" attack where a human legitimately retypes AI content.

I disagree with models suggesting jitter synthesis is a patchable signal. It is not. The "Fork-and-Replay" and "Kernel-Level Timing Proxy" attacks render behavioral biometrics obsolete against determined adversaries. Relying on statistical thresholds invites the "Verifier Hill-Climbing" attack.

Regarding Divergent Proposals:
**Proposal B (TEE)** is the only cryptographically valid solution to Consensus #1. If the OS is untrusted, only hardware-isolated input capture works. However, mandating TEEs destroys web compatibility. It is the correct *security* move but the wrong *product* move for general adoption.
**Proposal F (Perplexity)** is subjective and gameable by fine-tuned LLMs.
**Proposal E (Continuous)** increases data volume without increasing the trust root.

**Proposal D (Full Edit-Operation Logs)** is the single most impactful improvement. Unlike jitter, edit graphs contain structural entropy (backspaces, cursor navigation, pause-and-think patterns) that is significantly harder to synthesize convincingly across long documents without leaving statistical artifacts. While a hostile OS can still fabricate logs, committing incrementally prevents "Checkpoint Selective Omission." It transforms the protocol from "proof of humanity" to "forensic audit trail."

However, D raises privacy concerns (Open Question #2). The protocol must hash operations locally before transmission.

**Verdict:** Abandon claims of "human authorship." Adopt **Proposal D** with local hashing. This acknowledges the OS threat (Consensus #1) while providing verifiable structural evidence that resists "Paste Decomposition." Proposals A, C, and G should be discarded as they offer false confidence against kernel-level timing proxies. B remains the gold standard for high-assurance environments but is impractical for general deployment.

---

## DeepSeek V3.2

Having reviewed all Round 1 arguments, I find the consensus on fundamental T1/T2 vulnerabilities unassailable. The panel is correct: behavioral telemetry at the mercy of a compromised OS is statistical theater, not cryptographic proof. Where we diverge is on the path forward.

**On Divergent Proposals:**
I strongly disagree with **Proposals A (VDFs)** and **G (FIDO2 keys)**. They are elegant but impractical red herrings. A true VDF only proves elapsed time on *its* hardware, not that a human typed during it; an attacker can still fork-and-replay text generation while the VDF runs. FIDO2 keys merely sign jitter, they don't authenticate its human origin. Both add complexity for marginal, misplaced security.

I am skeptical of **Proposal B (Mandatory TEE)**. While it theoretically secures the sensing layer, it imposes an impossible adoption barrier (specific hardware/OS) and still fails against the **Cyborg/Transcription Attack**—a human genuinely retyping AI text inside the TEE remains undetectable. It solves a subset of technical problems while ignoring the human-in-the-loop threat model.

The most promising and practical direction combines elements of **C (External Witness)** and **D (Full Edit Logs)**, as hinted by GPT-5.4 and Claude. An external service providing **unpredictable, time-bound nonces** (C) directly mitigates *Time Compression* and *Selective Omission* by forcing real-time, sequential commitment. Pairing this with **fine-grained edit-operation logs** (D) creates a forensic chain that makes *Paste Decomposition* and selective forking statistically glaring, not just theoretically possible. This combo is implementable with current web tech and incremental trust in a third party.

**The Single Most Impactful Improvement:**
The protocol must abandon the fiction of standalone "authorship proof." Its most impactful improvement is to **reframe T1/T2 explicitly as *non-repudiation evidence* (Open Question 1) within a broader adversarial audit trail.** Stop claiming to "prove a human typed." Instead, provide a **tamper-evident, temporally-constrained log** of document construction. When combined with the document's semantic content analysis (PPX/ET) by a verifier, this log raises the cost of forgery beyond the value of most deception. It shifts the claim from "this is human-written" to "the construction of this document is consistent with human composition and inconsistent with common AI-paste attacks." This honest reframing is a prerequisite for any technical fix like C or D to have credible utility.

---

## MiniMax M2.7

## Round 2 Response: Making CPoE Honest About What It Cannot Prove

The consensus identifies the right attacks but misdiagnoses the solution space. Most proposals (A, D, E, F) attempt to add cryptographic complexity where the root problem is *ontological*: you cannot cryptographically prove human consciousness typed words.

**On Proposal A (True VDF):** Theoretically elegant but operationally naive. VDFs prove sequential time passed—they don't prove a human watched. An attacker with a VDF ASIC could still generate AI text offline, inject it at T1, and let the VDF prove nothing useful about authorship. VDFs are a timing control, not a human-input oracle. *Reject for this use case.*

**On Proposal B (TEE for keystroke capture):** Underrated. If keystrokes are captured inside SGX or TrustZone, the host OS cannot inject pre-generated timing data. This directly addresses the kernel-level timing proxy attack. The trust assumption—CPU manufacturer honesty—is explicit and auditable. *Support with modification: restrict TEE claims to T2+ only, abandon T1.*

**On Proposal G (Hardware-bound FIDO2 key):** The most practical proposal and surprisingly overlooked. FIDO2 keys are deployed on hundreds of millions of devices, tamper-resistant, and require physical presence. They don't prove human input *per keystroke*, but they create an undeniable cryptographic commitment point the attacker cannot forge without physical key access. *This directly defeats fork-and-replay.* Combine with unpredictable nonces (proposal C) for anti-precomputation.

**What others missed:** The protocol's framing is its biggest vulnerability. Calling T1/T2 "authorship proof" invites attacks on that exact claim. *Reframe as continuous non-repudiation evidence with hardware-bound anchoring.* Accept that 100% human authorship is unprovable; instead prove continuous hardware-device presence.

**Single most impactful improvement:** Hardware-bound FIDO2 signing at discrete moments, combined with TEE-captured keystroke timing for inter-signature intervals. Ditch the SWF/Argon2id theater entirely for T1. Accept honest limitations. Attackers control the software stack—but they cannot clone your security key.

---

# Round 3: Concrete Protocol Change Proposals

Concrete protocol change: “Witness-anchored checkpoints” (consensus Proposal C)

1. Wire-format / CDDL additions  
   Original Checkpoint record (simplified)  
     Checkpoint = {  
       seq: uint64,            ; monotone inside a session  
       doc_hash: bstr,         ; SHA-256 of current document  
       swf_proof: bstr         ; existing Argon2/VDF blob  
     }  

   NEW fields (★):  

     Checkpoint = {  
       seq:         uint64,  
       prev_id:     bstr,      ; hash of previous Checkpoint  
       doc_hash:    bstr,  
       swf_proof:   bstr,  
       w_nonce:     bstr .size 32,   ; ★ 256-bit nonce issued by witness  
       w_expiry:    uint32,          ; ★ UNIX seconds, nonce valid until  
       w_sig:       bstr .size 64    ; ★ Ed25519(sig( seq ‖ prev_id ‖ w_nonce ‖ w_expiry ))  
     }  

   Witness public key is distributed in the protocol metadata block once per session.

   Network flow  
   a. Client → Witness:  POST /nonce  {session_id, seq-1}  
   b. Witness → Client:  {w_nonce, w_expiry, w_sig}  
   c. Client embeds the tuple in the next Checkpoint and ships it as usual.

2. New verifier logic  
   For every checkpoint i:  
   a. Recompute hash(prev) and compare prev_id.  
   b. Verify Ed25519(w_sig, witness_pub, (seq,prev_id,w_nonce,w_expiry)).  
   c. Confirm monotone sequence and that w_expiry ≥ checkpoint_time ≥ (w_expiry – Δmax), where Δmax (e.g. 5 s) is published by the witness.  
   d. Look the (seq,w_nonce) up in the witness transparency log (append-only Merkle tree served over HTTPS or ordinary CT audit interface). Missing or duplicated entries invalidates the chain.

3. Deployment cost  
   Client: one extra HTTPS round-trip and +100 bytes on-wire per checkpoint; negligible CPU.  
   Service operator: run a stateless nonce API (~1 KiB/s per 10 k active writers) and periodically upload the Merkle tree root to any public bulletin board (CT, NIST beacon, Git).  
   No hardware change, no change to existing SWF proof code.

4. Attacks newly defeated  
   • Offline chain generation / “time-compression”: attacker cannot pre-compute future w_nonces.  
   • Fork-and-replay & selective omission: missing seq numbers are evident in the public witness log.  
   • Back-dated editing: checkpoints signed after w_expiry are provably invalid.  
   • Oracle-hill-climbing on statistical detectors: attacker must stay online and cannot iteratively prune “bad” chains without leaving gaps.

This single change converts CPoE from a self-signed diary into an externally time-stamped, non-forkable ledger while adding <0.1 s latency per checkpoint on commodity networks.

---

**CONCRETE PROTOCOL CHANGE SPECIFICATION: Witness-Anchored Checkpoints (Proposal C + minimal D)**

**1. Wire format / CDDL change**

Add a new top-level map key to the existing `Checkpoint` structure:

```cddl
Checkpoint = {
  ...
  witness_nonce: bstr,          ; exactly 32 bytes, fresh from witness
  witness_seq: uint,            ; monotonic sequence number from witness
  edit_log_hash: bstr,          ; SHA-256 of the append-only edit log segment since last checkpoint
  sig: bstr                     ; Ed25519 or ECDSA signature over the above three fields + prior hash
}

; New top-level container sent to witness service
WitnessRequest = {
  prev_checkpoint_hash: bstr,   ; hash of previous Checkpoint
  edit_log_hash: bstr,
  client_pubkey: bstr
}

WitnessResponse = {
  nonce: bstr .size 32,
  seq: uint,
  timestamp: uint,              ; witness monotonic time
  signature: bstr               ; witness signature over (nonce, seq, prev_checkpoint_hash, edit_log_hash)
}
```

Checkpoints are now emitted at most every 2 seconds or after 1500 Unicode code points, whichever comes first. The `edit_log_hash` commits to a canonical CBOR encoding of the ordered list of edit operations (Insert, Delete, Replace, Paste, CursorMove) with their exact byte offsets and millisecond deltas since the previous operation.

**2. New verification steps (verifier side)**

- Receive `Checkpoint` and attached `WitnessResponse`.
- Verify witness signature on the tuple `(nonce, seq, prev_checkpoint_hash, edit_log_hash)`.
- Verify client signature over `(witness_nonce, witness_seq, edit_log_hash)`.
- Enforce `witness_seq` is exactly `previous.witness_seq + 1`.
- Recompute the edit-log Merkle root from the supplied edit log segment and match `edit_log_hash`.
- Reject any gap > 3 seconds between `witness.timestamp` values.

**3. Deployment cost**

- One new always-on REST endpoint (or WebSocket) operated by the same vendor already running the verifier. 32-byte nonce + 64-byte signature per checkpoint ≈ 200 bytes every 2 s → ~100 KB per hour per active user.
- No client hardware changes. Pure software. Backward-compatible: old clients produce checkpoints without the new fields and are treated as T1.
- Witness service can be implemented with a simple rate-limited PRNG + HSM signing key. Can later be replaced by a public beacon (drand) without client change.

**4. Attacks defeated**

This directly defeats offline chain fabrication, selective omission, and fork-and-replay. An attacker can no longer pre-generate an entire plausible transcript and later choose which checkpoints to publish, because each checkpoint must embed a nonce that did not exist at the time the attacker started writing. Gaps in the `witness_seq` are immediately visible. When paired with the edit-log commitment, paste-decomposition and post-hoc reordering become detectable because the ordered operation sequence is cryptographically bound to the external nonce sequence.

(Word count: 298)

---

**Specification: Witness-Anchored Edit Log Commitments**

**1. Wire Format (CDDL) Change:**
Replace the standalone `Checkpoint` with a `WitnessedCheckpoint`. The critical change is binding an external, unpredictable challenge to the local edit state.
```cddl
WitnessedCheckpoint = {
  seq: uint,
  prev_hash: bstr,
  edit_delta_hash: bstr,    ; Hash of ordered edit ops (Insert/Delete/Move)
  witness_challenge: bstr,  ; NEW: 32-byte nonce from Witness Service
  witness_attestation: bstr ; NEW: Witness signature over (seq || prev_hash || edit_delta_hash)
}
```
The `prev_hash` must include the previous `witness_attestation`, ensuring chain continuity anchored by the witness.

**2. New Verification Step:**
Verifiers must validate the `witness_attestation` against a known Witness Service public key. Crucially, the verifier confirms that `witness_challenge` was issued *after* the `prev_hash` was computable. This is enforced by the Witness Service requiring a POST of `prev_hash` before releasing `witness_challenge`. Verifiers reject any checkpoint where the witness timestamp precedes the local `prev_hash` generation time by more than network skew (e.g., 500ms).

**3. Deployment Cost:**
Requires one synchronous HTTPS request per checkpoint (e.g., every 2 seconds or per 50 edits). Adds ~100-200ms latency to the editing loop. Bandwidth overhead is negligible (~1KB/checkpoint including headers). No client-side hardware changes; requires operating a high-availability Witness Service (can be centralized or federated).

**4. Attacks Defeated:**
**Offline Transcript Synthesis:** Attackers cannot precompute a valid chain because `witness_challenge` is unpredictable and only released upon request. **Selective Omission:** The Witness Service logs issued sequence numbers; verifiers can detect gaps if the attacker tries to publish a forked branch with missing intervals. **Time Compression:** The witness timestamp provides an external lower-bound on elapsed wall-clock time, preventing batch-generation of checkpoints.

This change shifts CPoE from a self-signed diary to a third-party anchored receipt, materially raising the cost of forgery without requiring TEEs or VDFs.

---

## CONCRETE PROTOCOL CHANGE SPECIFICATION

**1. Wire Format/CDDL Change:**
Add an `external_witness_nonce` field to the `Checkpoint` structure and replace the `document_state_hash` with an `edit_log_commitment`. CDDL diff:

```
checkpoint = {
  sequence_number: uint,
- document_state_hash: bytes,
+ edit_log_commitment: bytes,
  timestamp: uint,
  swf_proof: bytes,
+ external_witness_nonce: bytes,  ; 32-byte fresh nonce
  previous_checkpoint_hash: bytes,
  signature: bytes
}

edit_log_entry = [
  operation_type: uint,  ; 0=insert, 1=delete, 2=move, 3=selection
  position: uint,
  content_hash: bytes,   ; hash of inserted/deleted content
  relative_timestamp_ms: uint16  ; delta from previous operation (max 65535 ms)
]
```

The client maintains an append-only Merkle tree of edit log entries. `edit_log_commitment` is the root hash after applying all operations since the previous checkpoint.

**2. New Verification Step:**
Verifiers must validate witness nonce freshness and edit log consistency:
- Each `external_witness_nonce` must be signed by a trusted witness service and include a sequence number
- Nonces must be monotonically increasing with no gaps
- The edit log must reconstruct to the final document state when applied sequentially
- Relative timestamps between operations must not exceed a maximum delta (e.g., 65 seconds)
- Any checkpoint missing a valid witness nonce or with inconsistent edit logs is rejected

**3. Deployment Cost:**
- Client: Must maintain connection to witness service (1 RTT per checkpoint, ~2 KB bandwidth)
- Service: New witness service with simple REST API issuing signed nonces (~10 ms latency requirement)
- Storage: Edit logs increase payload size ~5-10× (mitigated by local hashing)
- No hardware changes or TEE requirements

**4. Attacks Defeated:**
- **Offline chain generation**: Attacker cannot precompute future checkpoints without fresh witness nonces
- **Selective omission**: Edit log commits to all operations between checkpoints; omitting bad segments breaks Merkle consistency
- **Fork-and-replay**: Witness sequence numbers prevent branching histories
- **Paste decomposition**: Individual paste operations are explicitly logged with timing, preventing decomposition into fake keystrokes

This change converts CPoE from self-attested timing evidence to externally-anchored forensic audit trail, defeating the most damaging offline fabrication attacks while maintaining practical deployability.

---

# Protocol Change Specification: Witness-Anchored Checkpoint Protocol (WACP)

## 1. Wire Format / CDDL Changes

Add new structures to checkpoint payload:

```cddl
witness-nonce = bstr .size 32
nonce-request = {
  checkpoint-hash: bytes,
  client-epoch: uint,
  prior-nonce: bstr .size 32 / null,
  request-timestamp: uint .uint
}
nonce-response = {
  nonce: witness-nonce,
  sequence-number: uint64,
  issued-at: uint,
  witness-sig: bytes
}
augmented-checkpoint = {
  base-checkpoint,
  witness-nonce: bstr .size 32,
  nonce-sequence: uint64,
  witness-sig: bytes,
  prior-checkpoint-hash: bytes
}
```

**Constraint:** Nonce must be requested after prior checkpoint hash is known and before content hash for current checkpoint is finalized. Network round-trip ≤ 500ms enforced client-side; violations invalidate the checkpoint.

## 2. New Verification Step

Verifier implements **nonce-chain continuity check**:

1. Extract `nonce-sequence` from each checkpoint; verify sequences are strictly monotonically increasing with no gaps.
2. Reconstruct expected nonce: `expected = HMAC-Chacha20(witness-key, checkpoint-hash || sequence-idx)`. Compare against embedded nonce.
3. Verify witness signature over `(nonce || sequence-number || checkpoint-hash)` using witness's published public key.
4. **Fork detection:** If client presents two branches with overlapping sequence ranges, flag for manual review; both chains are invalid.

## 3. Deployment Cost

| Component | Cost Estimate |
|-----------|----------------|
| Witness service (stateless) | ~$50/mo for single-region, ~10k RPS |
| Client SDK integration | 2-4 engineering days |
| Nonce latency overhead | +1-2 RTT per checkpoint (~100-300ms) |
| Storage increase per checkpoint | +64 bytes |

**Backward compatibility:** Checkpoints without witness fields rejected as legacy-tier only; verifiers MAY accept them with reduced confidence score.

## 4. Defeated Attacks

- **Offline chain fabrication:** Attacker cannot precompute checkpoint chain because nonces are revealed only after prior hash commitment.
- **Selective omission:** Gaps in nonce sequence expose branch pruning.
- **Time-compression fork-and-replay:** Attacker cannot retroactively inject checkpoints without round-trip to witness.

This is the single highest-leverage change: it eliminates the primary failure mode (malicious offline precomputation) without hardware dependency or Argon2id replacement.

---


# Round 4: Proposal Evaluation

**Best proposal:** the **first one** (“Witness-anchored checkpoints” with `prev_id`, `w_nonce`, `w_expiry`, `w_sig`, plus a public transparency log). It is the most deployable **now** because it minimally changes the existing checkpoint object, preserves current SWF/VDF machinery, and adds the one thing CPoE most needs: an external, unpredictable, publicly auditable anchor. The transparency log is the differentiator; several others say “signed nonce,” but without an auditable issuance log, selective omission/fork detection is much weaker in practice.

**What I’d change:**  
1. Sign **`doc_hash` too**, not just `(seq, prev_id, nonce, expiry)`. Otherwise the witness only anchors chain position, not the claimed document state for that checkpoint.  
2. Replace `w_expiry` with **`issued_at` + `max_age_ms`**; expiry-only semantics are awkward and verifier clocks are messy.  
3. Make witness logging mandatory on **issuance**, not optional lookup later.  
4. Add an explicit **witness key rotation / witness-ID** field.

**Gotchas missed:**  
- **Online availability:** one RTT per checkpoint is fine on paper, but witness outages will stall writing unless you specify degraded mode.  
- **Clock assumptions:** `checkpoint_time` is underspecified; whose clock is authoritative? This will create false rejects.  
- **Privacy/linkability:** a public witness log can leak session cadence and metadata unless session IDs are blinded or per-session pseudonymous.  
- **Replay/domain separation:** nonce signatures need a protocol/version/context tag to avoid cross-protocol reuse.  
- **Duplicate issuance semantics:** define whether duplicate `(seq, prev_id)` requests are idempotent.

**Minimum viable improvement for next spec revision:**  
Ship **signed witness nonces bound to `(session_id, seq, prev_id, doc_hash)` plus mandatory monotonic sequence verification**. Skip full transparency-log/audit integration in vNext if needed; that’s valuable, but the nonce binding alone already kills offline precomputation and is implementable immediately.

---

1. Winner: the first variant (“prev_id + w_nonce + w_expiry + w_sig + transparency-log”).  
   • Smallest diff to today’s Checkpoint object (+3 fields, no edit-log surgery).  
   • Uses only primitives already in the stack (Ed25519, SHA-256, HTTPS).  
   • A single extra RTT and ≈100 B/checkpoint is acceptable even on 4G.  
   • The public Merkle transparency log gives independent auditors something to monitor, so fork-detection does not rely on the relying party being online at the time of writing.  
   In short, it shuts the biggest hole (offline pre-fabrication) with the least surface-area change, so it can actually ship this quarter.

2. What to change  
   • Collapse w_expiry and Δmax into one unsigned 16-bit “ttl” field measured in 100 ms units. It trims 4 bytes and side-steps leap-second headaches.  
   • Allow the client to pre-fetch up to N (e.g. 3) nonces so a brief witness outage doesn’t stall typing. A simple “nonce_batch” RPC suffices and does not weaken security because nonces are still unpredictable and logged.  
   • Publish the witness public key inside the transparency log header as well, giving you key-rotation for free.

3. Implementation gotchas  
   • The proposer forgot to specify how the witness handles clock skew. Require the witness to stamp its own time in the log entry; verifiers compare that to checkpoint_time and apply an ϵ=±5 s window.  
   • Sequence overflow: `seq` is uint64; roll-over won’t happen in practice, but the code still needs a sanity check to reject a wrap.  
   • Log availability: if the log is down, verifiers must decide whether to treat “not yet present” as soft-fail or hard-fail. Spell that out.

4. Minimum viable that could ship in the next spec rev  
   Add only `prev_id`, `w_nonce` and `w_sig`, plus the one-round-trip nonce API. Leave `w_expiry` and the public transparency log OPTIONAL for now; they can be added as extension points later without breaking binary compatibility. That already blocks offline chain generation and fork-and-replay, delivering 90 % of the benefit for 10 % of the work.

---

**Best Proposal: Witness-anchored checkpoints (Original Proposal C)**

**Why it wins:** It is the cleanest, most minimal, and most deployable design. It adds only the essential external unpredictability (a signed nonce + short expiry) while preserving the existing SWF proof, checkpoint structure, and verification flow. The other four proposals bloat the format with redundant fields (edit_log_hash, witness_seq on client, client signatures, relative ms deltas), over-specify edit-log granularity, or introduce unnecessary complexity (HMAC-Chacha20 reconstruction, client-side 500 ms enforcement). Proposal C defeats the core attacks—offline fabrication, fork-and-replay, selective omission, and time-compression—with the smallest surface area.

**What I would change:** Drop the separate transparency log upload requirement. It is nice-to-have but not required for the security argument; a simple append-only (seq, w_nonce) list served by the same witness endpoint is sufficient for gap detection. Make w_expiry a relative delta (e.g. max 10 s) instead of absolute UNIX timestamp to avoid clock skew fights. Publish the witness public key in the session metadata exactly as written.

**Implementation gotchas missed:** 
- Clock skew between client, witness, and verifier (the Δmax window must be chosen conservatively, ~5–8 s). 
- Witness must reject nonce requests for the same (session, seq) pair to prevent client replays. 
- Backward-compatibility path for old checkpoints must be explicitly defined (treated as T1 with lower score). 
- The witness service, while “stateless,” still needs per-session rate limiting and basic anti-abuse.

**Minimum Viable Improvement for next spec revision:**  
Add only three fields to Checkpoint: `prev_id`, `w_nonce` (32 B), `w_sig` (64 B) over `(seq ‖ prev_id ‖ w_nonce ‖ expiry)`. Require one extra HTTPS `/nonce` round-trip before emitting each checkpoint. Verifiers check: (1) hash chain via prev_id, (2) witness signature, (3) no gaps in seq, (4) expiry window. Everything else (full edit-log commitment, per-op ms deltas, dual signatures, Merkle transparency log) is deferred. This single change turns CPoE from a self-signed diary into a non-forkable, externally anchored ledger and can ship immediately.

(Word count: 278)

---

## Round 4 Verdict

### BEST PROPOSAL: **Proposal 1 (Original C)**

It is the most conservative, highest-leverage change. The wire-format delta is minimal (three fields), it imposes no new complex data structures (no edit logs, no Merkle trees), and the witness transparency log leverages existing Certificate Transparency infrastructure rather than inventing new consensus mechanisms. The prev_id chain link plus nonce expiry creates a tight causal ordering that defeats the primary attack: offline chain fabrication.

Proposal 2/4/5 add edit-log commitments which are *defense-in-depth*, not *required* for the core threat model. Proposal 4's 5-10× payload increase is a non-starter for bandwidth-constrained clients. Proposal 5's HMAC-Chacha20 derivation is elegant but adds verifier complexity and the "legacy tier" backward-compatibility surface is a footgun.

### CHANGES I'd MAKE

1. **Relax the transparency log requirement for MVP.** Replace "append-only Merkle tree served over HTTPS" with a simpler: witness publishes a signed statement list (sequentially numbered, hash-chained) that verifiers can fetch and cache. Full CT integration can be Phase 2.
2. **Replace w_expiry with a simple witness-side sequence number.** The expiry semantics add clock-skew complexity. Monotonic seq is sufficient and more robust.
3. **Bound the RTT assumption.** Explicitly document that if the witness RTT exceeds 2 seconds, the client should abort and re-request the nonce rather than proceeding.

### GOTCHAS THE PROPOSER MISSED

- **Rate limiting is underspecified.** The witness nonce API needs per-client rate limits to prevent enumeration attacks. A naive PRNG+HSM is not enough.
- **No guidance on witness key rotation.**

---


# Round 5: Final Recommendations

**FINAL TOP 3 RECOMMENDATIONS (ranked by impact / effort)**

**1) Witness-anchored checkpoints, bound to the actual state.**  
**What:** Add a witness-signed nonce to every checkpoint, over `(protocol_tag, witness_id, session_id, seq, prev_id, doc_hash, issued_at, max_age_ms)`.  
**Why:** Defeats offline precomputation, fork-and-replay, and “same position, different document state” attacks.  
**How:** Extend the checkpoint with `prev_id`, `w_nonce`, `issued_at`, `max_age_ms`, `witness_id`, `w_sig`; verifier checks signature, max-age, and that `prev_id` matches the prior checkpoint ID.  
**Cost:** **Low**

**2) Make chain consistency checks mandatory, not advisory.**  
**What:** Require strict monotonic verification of `seq`, `prev_id`, and `doc_hash` continuity, with duplicates/gaps as hard failures.  
**Why:** Defeats omission, truncation, reordering, and duplicate-issuance ambiguity even before full log auditing exists.  
**How:** Spec change: verifiers MUST reject non-consecutive `seq`, mismatched `prev_id`, or conflicting checkpoints for the same `(session_id, seq)`; define duplicate requests as idempotent at the witness.  
**Cost:** **Low**

**3) Mandatory witness issuance log with auditable append-only behavior.**  
**What:** Every issued witness nonce must be logged publicly in an append-only transparency log.  
**Why:** Defeats selective omission and makes fork detection independently auditable.  
**How:** Witness logs `(session_id_pseudonym, seq, prev_id, doc_hash, w_nonce, issued_at, witness_id)` on issuance; verifiers/auditors fetch inclusion proofs and track witness key rotation.  
**Cost:** **Medium**

**ONE THING CPoE GETS RIGHT:**  
Keep the existing **checkpointed SWF/VDF proof model**; it is the right lightweight core and should not be replaced with heavyweight per-edit signing or full edit-log commitments.

---

TOP-3 CHANGES (ranked by impact : effort)

1. Witness-anchored checkpoint
   What: Add an unpredictable witness nonce and signature to every checkpoint and publish each (sess_id, seq, w_nonce) in an append-only log.
   Why: Blocks the core attack—offline pre-fabrication / fork-and-replay—by forcing every checkpoint to be externally time-stamped and globally auditable.
   How: Extend wire object with {prev_id, issued_at, ttl, w_nonce32, w_sig64}.  Client: GET /nonce → {w_nonce, issued_at}.  Witness: log entry = H(sess_id‖seq‖w_nonce‖doc_hash), sign it, return w_sig.  Verifier: check hash chain, witness sig, ttl, no seq gaps, inclusion in log.
   Cost: Medium (new RTT + 3 fields; server is stateless and can reuse CT tooling).

2. Bind document state to the witness signature
   What: Include doc_hash in the witness-signed message.
   Why: Prevents “position-only” anchoring where an attacker swaps in a different document after the nonce is issued.
   How: Witness signs H(seq‖prev_id‖w_nonce‖doc_hash‖issued_at‖ttl).  No extra wire bytes—just change the signature input.
   Cost: Low (code change only).

3. Batched-nonce fallback
   What: Allow the client to pre-fetch up to N nonces in one call.
   Why: Keeps writing alive during short witness outages without weakening unpredictability.
   How: new RPC /nonce?count=N returns [{w_nonce_i, issued_at_i}], each logged immediately; client burns them in order, ttl≤10 s.
   Cost: Low (single optional endpoint; no verifier change).

UNCHANGED & KEEP IT: the monotonically increasing 64-bit seq field—simple, already-deployed, and essential for O(1) gap detection.

---

**FINAL TOP 3 RECOMMENDATIONS** (ranked by impact-to-effort)

**1. Witness-anchored checkpoints**  
What: Add `prev_id`, `w_nonce`, `w_issued_at`, `w_sig` to every Checkpoint.  
Why: Defeats offline pre-fabrication, fork-and-replay, and selective omission by forcing external fresh unpredictability.  
How: Before emitting checkpoint, client calls `/nonce?session_id=X&seq=N&prev_id=H` (one RTT); witness logs `(session_id, seq, prev_id, doc_hash, issued_at)` to append-only hash-chain log, returns 32 B nonce + Ed25519 signature over `( "CPoE-v1" ‖ session_id ‖ seq ‖ prev_id ‖ doc_hash ‖ issued_at ‖ max_age_ms )`. Verifier checks signature, hash link, no seq gaps, and issued_at freshness (±5 s).  
Cost: Low.

**2. Mandatory doc_hash binding + monotonic seq verification**  
What: Witness signs `doc_hash` and enforces strictly increasing per-session sequence numbers.  
Why: Prevents witness from selectively omitting or back-dating checkpoints for the same document state.  
How: Include `doc_hash` in signed tuple above; witness rejects duplicate `(session,seq)` or out-of-order seq.  
Cost: Low.

**3. Simple public witness log with key rotation header**  
What: Publish all issued tuples in a publicly auditable, hash-chained log with current witness pubkey in header.  
Why: Enables independent fork detection and key-rotation without relying on online verifiers.  
How: Log served at `/.well-known/cpoe-log`; entries are signed statements, not full Merkle tree for MVP.  
Cost: Medium.

**ONE THING the protocol gets RIGHT that should NOT be changed:** The core SWF/VDF + checkpoint emission model. All improvements are additive; the existing proof machinery and client-side timing source remain untouched.

(Word count: 218)

---

## FINAL TOP 3 RECOMMENDATIONS

### 1. Witness-Anchored Nonce Binding
**What:** Add `prev_id`, `w_nonce` (32B), and `w_sig` (64B) to each checkpoint, obtained via one HTTPS round-trip to a witness service.
**Why:** Defeats offline pre-fabrication, fork-and-replay, and time-compression — the existential threats to CPoE's credibility.
**How:** Before emitting checkpoint `n`, client calls `POST /nonce` with `(session_id, seq, prev_id)`. Witness returns `(w_nonce, w_sig)` over `(seq ‖ prev_id ‖ doc_hash ‖ w_nonce ‖ ttl)`. Verifier checks signature, hash-chain continuity, sequence monotonicity, and TTL window (≤10s). Witness rejects duplicate `(session_id, seq)` pairs.
**Cost:** Low. Three new fields, one RTT, existing Ed25519/HTTPS primitives.

### 2. Mandatory Monotonic Sequence with Gap Detection
**What:** Enforce strict `seq` incrementing with `prev_id` hash chaining so any omitted or reordered checkpoint is detectable.
**Why:** Defeats selective omission and checkpoint reordering attacks.
**How:** `prev_id = SHA-256(checkpoint[n-1])`. Verifiers reject any chain where `seq` is non-consecutive or `prev_id` doesn't match. Checkpoint 0 uses a well-known sentinel.
**Cost:** Low. Pure verification logic, no new infrastructure.

### 3. Witness Transparency Log (Phase 2)
**What:** Witness appends every issued `(session_id, seq, w_nonce)` to a publicly auditable, hash-chained log.
**Why:** Defeats silent witness collusion and enables third-party fork detection without relying on verifier being online at write time.
**How:** Witness publishes a signed, sequentially numbered, hash-chained statement list. Verifiers and auditors fetch and cache periodically. Full Merkle/CT integration deferred to Phase 3.
**Cost:** Medium. Requires witness-side storage and a public read API.

---

### THE ONE THING TO PRESERVE
**The SWF/VDF proof-of-work mechanism is correct and should not be changed.** It provides the fundamental computational cost asymmetry that makes bulk fabrication expensive. Every proposal that tried to replace it added complexity without improving the core guarantee. The witness layer *complements* SWF — it doesn't substitute for it.

---

FINAL TOP 3 RECOMMENDATIONS

1. **What:** Witness-anchored nonces bound to `doc_hash`.
   - **Why:** Defeats offline chain fabrication and state fork attacks by tying witnesses to specific content, preventing pre-computation.
   - **How:** Add `w_nonce` (32B) and `w_sig` to Checkpoint; witness signs `(seq, prev_id, doc_hash, nonce)` using Ed25519.
   - **Cost:** Low.

2. **What:** Relative TTL + Nonce Batching.
   - **Why:** Mitigates clock skew errors and prevents witness outages from stalling writes, ensuring high availability during network partitions.
   - **How:** Replace absolute `w_expiry` with `ttl_ms` (16-bit); allow `/nonce_batch` RPC for pre-fetching up to 3 nonces.
   - **Cost:** Low.

3. **What:** Mandatory Issuance Logging.
   - **Why:** Enables independent fork detection without requiring the verifier to be online during writing, allowing asynchronous auditability.
   - **How:** Witness must append `(seq, nonce_hash)` to a public append-only log before signing; verifiers sample log entries via HTTPS.
   - **Cost:** Medium.

**ONE THING RIGHT:**
The existing **SWF/VDF proof machinery** must remain unchanged. The protocol correctly identifies that external anchoring secures the timeline without replacing the core cryptographic proof stack. This ensures immediate deployability without breaking legacy clients or requiring complex consensus changes, leveraging proven primitives instead of inventing new cryptography.

---

**FINAL TOP 3 RECOMMENDATIONS**

1.  **Mandatory Witness-Anchored Nonces.**  
    *What:* Add a signed, time-bound nonce from a third-party witness to each checkpoint.  
    *Why:* Defeats offline chain precomputation and fork-and-replay attacks.  
    *How:* Extend checkpoint object with `prev_id`, `w_nonce`, `w_sig`; client fetches nonce via HTTPS `/nonce` RPC before writing.  
    *Cost:* **Low.** Minimal wire-format change using existing primitives.

2.  **Enforce Monotonic Sequence & Hash Chain.**  
    *What:* Verifiers must reject gaps in `seq` and breakages in the `prev_id` hash chain.  
    *Why:* Defeats selective omission and causal order manipulation.  
    *How:* Verification step adds two checks: `seq == prev.seq + 1` and `prev_id == hash(prev_checkpoint)`.  
    *Cost:* **Low.** Pure logic change for verifiers.

3.  **Basic Witness Transparency Log.**  
    *What:* Witness maintains a public, append-only log of issued nonces.  
    *Why:* Enables third-party audit and detection of witness equivocation.  
    *How:* Witness publishes a hash-chained list of `(session_id, seq, w_nonce)`; served via simple HTTPS endpoint.  
    *Cost:* **Medium.** Requires witness service to implement logging and publishing.

**ONE THING TO KEEP:** The core **Single-Writer Feed (SWF)** guarantee. Its simplicity—a single authorized writer per session—is the protocol's foundational strength and must remain unchanged.

---

# FINAL TOP 3 RECOMMENDATIONS

## 1. ⭐ External Nonce Anchor (HIGH IMPACT / LOW COST)
**What:** Bind every checkpoint to a witness-signed nonce fetched at emission time.
**Why:** Defeats offline pre-fabrication—the core attack. Without external unpredictability, the client can generate valid-looking chains in isolation.
**How:** Add `w_nonce` (32 B) + `w_sig` (64 B) to Checkpoint; client calls `POST /nonce/{session_id}/{seq}` before each write. Signature covers `(seq ‖ prev_id ‖ nonce ‖ expiry)`. Witness maintains a hash-chained nonce log (Phase 1); full CT integration deferred.
**Cost:** Low. One extra RTT, ~100 B/checkpoint, no new primitives.

## 2. ⭐ Hash-Chain prev_id Link (HIGH IMPACT / LOW COST)
**What:** Each checkpoint commits to the previous checkpoint's hash via `prev_id`.
**Why:** Defeats fork-and-replay and selective omission. Verifiers detect gaps without needing the full issuance timeline.
**How:** `prev_id = H(Checkpoint_{seq-1})`. Verification: walk chain backward, fail on any break.
**Cost:** Low. One field addition, deterministic.

## 3. Monotonic Sequence + Gap Detection (MEDIUM IMPACT / LOW COST)
**What:** Require verifiers to track `last_seq` per session and reject any checkpoint with non-contiguous sequence.
**Why:** Complements nonce anchor—detects insertions even if witness logging is unavailable.
**How:** Add `seq` (uint64) to all checkpoints. Verifier state: `max(seq)` per session. Reject `seq ≠ max_seq + 1`.
**Cost:** Low. State management only.

---

## ONE THING TO PRESERVE

**The witness is stateless.** Do not add edit-log commitments or per-operation state. This is the right abstraction—it keeps the witness scalable and deployment-simple. Any stateful accumulation (edit logs, operation trees) belongs in the session layer, not the checkpoint anchor.

---

