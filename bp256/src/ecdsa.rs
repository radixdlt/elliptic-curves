//! Elliptic Curve Digital Signature Algorithm (ECDSA)

pub use crate::BrainpoolP256;

/// ECDSA/Brainpool P-256 signature (fixed-size)
pub type Signature = ecdsa::Signature<BrainpoolP256>;

/// ECDSA/Brainpool P-256 signature (ASN.1 DER encoded)
pub type Asn1Signature = ecdsa::der::Signature<BrainpoolP256>;

impl ecdsa::CheckSignatureBytes for BrainpoolP256 {}

#[cfg(feature = "sha256")]
#[cfg_attr(docsrs, doc(cfg(feature = "sha256")))]
impl ecdsa::hazmat::DigestPrimitive for BrainpoolP256 {
    type Digest = sha2::Sha256;
}