//! brainpoolP256t1 elliptic curve: twisted variant

#[cfg(feature = "ecdsa")]
#[cfg_attr(docsrs, doc(cfg(feature = "ecdsa")))]
pub mod ecdsa;

use elliptic_curve::bigint::U256;

#[cfg(feature = "pkcs8")]
use crate::pkcs8;

/// brainpoolP256t1 elliptic curve: twisted variant
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct BrainpoolP256t1;

impl elliptic_curve::Curve for BrainpoolP256t1 {
    /// 256-bit field modulus
    type UInt = U256;

    /// Curve order
    const ORDER: U256 =
        U256::from_be_hex("a9fb57dba1eea9bc3e660a909d838d718c397aa3b561a6f7901e0e82974856a7");
}

impl elliptic_curve::PrimeCurve for BrainpoolP256t1 {}

impl elliptic_curve::PointCompression for BrainpoolP256t1 {
    const COMPRESS_POINTS: bool = false;
}

#[cfg(feature = "pkcs8")]
impl elliptic_curve::AlgorithmParameters for BrainpoolP256t1 {
    const OID: pkcs8::ObjectIdentifier = pkcs8::ObjectIdentifier::new("1.3.36.3.3.2.8.1.1.8");
}

/// brainpoolP256t1 field element serialized as bytes.
///
/// Byte array containing a serialized field element value (base field or scalar).
pub type FieldBytes = elliptic_curve::FieldBytes<BrainpoolP256t1>;

/// brainpoolP256t1 SEC1 encoded point.
pub type EncodedPoint = elliptic_curve::sec1::EncodedPoint<BrainpoolP256t1>;

/// brainpoolP256t1 secret key.
pub type SecretKey = elliptic_curve::SecretKey<BrainpoolP256t1>;

impl elliptic_curve::sec1::ValidatePublicKey for BrainpoolP256t1 {}
