use std::ops::Deref;
use ::hash::{ Hash, GenericHash };
use super::{ Mac, NonceMac };


/// HMAC, nonce variant.
///
/// # Definition:
/// `H(nonce, (K xor opad) || H(nonce, (K xor ipad) || text))`
///
/// # Example(result)
/// ```
/// use sarkara::auth::{ HMAC, Mac };
/// use sarkara::hash::Blake2b;
///
/// assert_eq!(
///     HMAC::<Blake2b>::new().result(&[5; 16], &[]),
///     [
///         103, 94, 237, 110, 44, 95, 234, 140,
///         231, 34, 21, 54, 134, 161, 118, 37,
///         36, 117, 44, 209, 164, 126, 32, 1,
///         117, 64, 234, 107, 194, 131, 210, 93,
///         95, 127, 126, 222, 45, 114, 152, 82,
///         129, 175, 78, 62, 31, 20, 128, 255,
///         47, 203, 122, 70, 202, 200, 33, 75,
///         253, 132, 234, 116, 220, 81, 39, 182
///     ][..]
/// );
/// ```
///
/// # Example(with_size/with_nonce)
/// ```
/// use sarkara::auth::{ HMAC, Mac, NonceMac };
/// use sarkara::hash::Blake2b;
///
/// assert_eq!(
///     HMAC::<Blake2b>::new()
///         .with_size(16)
///         .with_nonce(&[1; 8])
///         .result(&[5; 16], &[]),
///     [
///         156, 249, 9, 142, 32, 148, 190, 61,
///         50, 43, 151, 147, 161, 103, 56, 10
///     ][..]
/// );
/// ```
#[derive(Clone, Debug)]
pub struct HMAC<H> {
    ih: H,
    oh: H
}

impl<H: Default + Hash> Default for HMAC<H> {
    fn default() -> Self {
        HMAC {
            ih: H::default(),
            oh: H::default()
        }
    }
}

impl<H: Default + Hash> HMAC<H> {
    /// Create a new HMAC.
    pub fn new() -> HMAC<H> {
        HMAC::default()
    }
}

impl<B, H> Mac for HMAC<H> where
    B: Deref<Target=[u8]> + PartialEq<[u8]>,
    H: Hash<Digest=B>
{
    type Tag = H::Digest;

    fn result(&self, key: &[u8], data: &[u8]) -> Self::Tag {
        let mut ipad = vec![0x36; 64];
        let mut opad = vec![0x5c; 64];

        for i in 0..key.len() {
            ipad[i] ^= key[i];
            opad[i] ^= key[i];
        }

        ipad.extend_from_slice(data);
        opad.extend_from_slice(&self.ih.hash(&ipad));

        self.oh.hash(&opad)
    }

    fn verify(&self, key: &[u8], data: &[u8], tag: &[u8]) -> bool {
        self.result(key, data) == tag[..]
    }
}

impl<B, H> NonceMac for HMAC<H> where
    B: Deref<Target=[u8]> + PartialEq<[u8]>,
    H: GenericHash<Digest=B>
{
    fn with_nonce(&mut self, nonce: &[u8]) -> &mut Self {
        self.ih.with_key(nonce);
        self.oh.with_key(nonce);
        self
    }

    fn with_size(&mut self, len: usize) -> &mut Self {
        self.oh.with_size(len);
        self
    }
}