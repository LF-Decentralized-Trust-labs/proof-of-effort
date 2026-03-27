// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt;
use subtle::ConstantTimeEq;

/// CBOR tag for CPoP evidence packets (0x43504F50 = "CPOP").
pub const CBOR_TAG_EVIDENCE_PACKET: u64 = 1129336656;
/// CBOR tag for attestation results (0x43574152 = "CWAR").
pub const CBOR_TAG_ATTESTATION_RESULT: u64 = 1129791826;

/// IANA Private Enterprise Number for CPOP Inc.
/// Registered under SMI Network Management Private Enterprise Codes.
pub const IANA_PEN: u32 = 65074;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u64)]
pub enum HashAlgorithm {
    /// SHA-256 (32-byte digest).
    Sha256 = 1,
    /// SHA-384 (48-byte digest).
    Sha384 = 2,
    /// SHA-512 (64-byte digest).
    Sha512 = 3,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha256 => write!(f, "SHA-256"),
            HashAlgorithm::Sha384 => write!(f, "SHA-384"),
            HashAlgorithm::Sha512 => write!(f, "SHA-512"),
        }
    }
}

/// Hardware attestation strength tier per draft-condrey-rats-pop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u64)]
pub enum AttestationTier {
    /// Pure software signing, no hardware root of trust.
    SoftwareOnly = 1,
    /// Software key with remote attestation evidence.
    AttestedSoftware = 2,
    /// Key bound to TPM/Secure Enclave.
    HardwareBound = 3,
    /// Hardware-hardened with anti-tamper protections.
    HardwareHardened = 4,
}

impl fmt::Display for AttestationTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttestationTier::SoftwareOnly => write!(f, "Software Only"),
            AttestationTier::AttestedSoftware => write!(f, "Attested Software"),
            AttestationTier::HardwareBound => write!(f, "Hardware Bound"),
            AttestationTier::HardwareHardened => write!(f, "Hardware Hardened"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u64)]
pub enum ContentTier {
    /// Minimal evidence (checkpoints and hashes only).
    Core = 1,
    /// Additional behavioral metrics included.
    Enhanced = 2,
    /// Full forensic payload with jitter and HID data.
    Maximum = 3,
}

impl fmt::Display for ContentTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentTier::Core => write!(f, "Core"),
            ContentTier::Enhanced => write!(f, "Enhanced"),
            ContentTier::Maximum => write!(f, "Maximum"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u64)]
pub enum ProofAlgorithm {
    SwfSha256 = 10,
    SwfArgon2id = 20,
    /// Argon2id with cross-checkpoint entanglement.
    SwfArgon2idEntangled = 21,
}

impl fmt::Display for ProofAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProofAlgorithm::SwfSha256 => write!(f, "SWF-SHA256"),
            ProofAlgorithm::SwfArgon2id => write!(f, "SWF-Argon2id"),
            ProofAlgorithm::SwfArgon2idEntangled => write!(f, "SWF-Argon2id-Entangled"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u64)]
pub enum Verdict {
    /// Evidence verified as authentic human authorship.
    Authentic = 1,
    /// Insufficient data to reach a definitive conclusion.
    Inconclusive = 2,
    /// Anomalies detected; manual review recommended.
    Suspicious = 3,
    /// Evidence is structurally or cryptographically invalid.
    Invalid = 4,
}

impl fmt::Display for Verdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Verdict::Authentic => write!(f, "Authentic"),
            Verdict::Inconclusive => write!(f, "Inconclusive"),
            Verdict::Suspicious => write!(f, "Suspicious"),
            Verdict::Invalid => write!(f, "Invalid"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashValue {
    #[serde(rename = "1")]
    pub algorithm: HashAlgorithm,
    #[serde(rename = "2", with = "serde_bytes")]
    pub digest: Vec<u8>,
}

impl HashValue {
    /// Constant-time comparison to prevent timing side-channels on HMAC outputs.
    pub fn ct_eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm && self.digest.ct_eq(&other.digest).into()
    }

    pub fn expected_digest_len(&self) -> usize {
        match self.algorithm {
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha384 => 48,
            HashAlgorithm::Sha512 => 64,
        }
    }

    pub fn validate(&self) -> bool {
        self.digest.len() == self.expected_digest_len()
    }
}

// PartialEq/Eq for non-security-critical uses (serialization, tests).
// Security-critical verification must use ct_eq().
impl PartialEq for HashValue {
    fn eq(&self, other: &Self) -> bool {
        self.algorithm == other.algorithm && self.digest == other.digest
    }
}

impl Eq for HashValue {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRef {
    #[serde(rename = "1")]
    pub content_hash: HashValue,
    #[serde(rename = "2", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "3")]
    pub byte_length: u64,
    #[serde(rename = "4")]
    pub char_count: u64,
}

/// Single checkpoint in the causality chain, binding content state to a timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    #[serde(rename = "1")]
    pub sequence: u64,
    #[serde(rename = "2", with = "serde_bytes")]
    pub checkpoint_id: Vec<u8>,
    #[serde(rename = "3")]
    pub timestamp: u64,
    #[serde(rename = "4")]
    pub content_hash: HashValue,
    #[serde(rename = "5")]
    pub char_count: u64,
    #[serde(rename = "7")]
    pub prev_hash: HashValue,
    #[serde(rename = "8")]
    pub checkpoint_hash: HashValue,
    #[serde(rename = "9", skip_serializing_if = "Option::is_none")]
    pub jitter_hash: Option<HashValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidencePacket {
    #[serde(rename = "1")]
    pub version: u32,
    #[serde(rename = "2")]
    pub profile_uri: String,
    #[serde(rename = "3", with = "serde_bytes")]
    pub packet_id: Vec<u8>,
    #[serde(rename = "4")]
    pub created: u64,
    #[serde(rename = "5")]
    pub document: DocumentRef,
    #[serde(rename = "6")]
    pub checkpoints: Vec<Checkpoint>,
    #[serde(rename = "7", skip_serializing_if = "Option::is_none")]
    pub attestation_tier: Option<AttestationTier>,
    #[serde(rename = "19", skip_serializing_if = "Option::is_none")]
    pub baseline_verification: Option<crate::baseline::BaselineVerification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResult {
    #[serde(rename = "1")]
    pub version: u32,
    #[serde(rename = "2")]
    pub evidence_ref: HashValue,
    #[serde(rename = "3")]
    pub verdict: Verdict,
    #[serde(rename = "4")]
    pub attestation_tier: AttestationTier,
    #[serde(rename = "5")]
    pub chain_length: u64,
    #[serde(rename = "6")]
    pub chain_duration: u64,
    #[serde(rename = "12")]
    pub created: u64,
    /// None if no baseline verification was present in the evidence.
    #[serde(rename = "14", skip_serializing_if = "Option::is_none")]
    pub confidence_tier: Option<crate::baseline::ConfidenceTier>,
}
