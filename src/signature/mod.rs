use crate::Error;
use algebra::{
    bytes::{
        ToBytes, FromBytes
    },
    Field,
};
use rand::Rng;
use std::hash::Hash;
use std::fmt::Debug;

pub mod schnorr;

pub trait SignatureScheme {
    type Parameters: Clone + Send + Sync;
    type PublicKey: ToBytes + Hash + Eq + Clone + Default + Send + Sync;
    type SecretKey: ToBytes + Clone + Default;
    type Signature: Clone + Default + Send + Sync;

    fn setup<R: Rng>(rng: &mut R) -> Result<Self::Parameters, Error>;

    fn keygen<R: Rng>(
        pp: &Self::Parameters,
        rng: &mut R,
    ) -> Result<(Self::PublicKey, Self::SecretKey), Error>;

    fn sign<R: Rng>(
        pp: &Self::Parameters,
        sk: &Self::SecretKey,
        message: &[u8],
        rng: &mut R,
    ) -> Result<Self::Signature, Error>;

    fn verify(
        pp: &Self::Parameters,
        pk: &Self::PublicKey,
        message: &[u8],
        signature: &Self::Signature,
    ) -> Result<bool, Error>;

    fn randomize_public_key(
        pp: &Self::Parameters,
        public_key: &Self::PublicKey,
        randomness: &[u8],
    ) -> Result<Self::PublicKey, Error>;

    fn randomize_signature(
        pp: &Self::Parameters,
        signature: &Self::Signature,
        randomness: &[u8],
    ) -> Result<Self::Signature, Error>;
}

pub trait FieldBasedSignatureScheme {

    type Data: Field;
    type PublicKey: ToBytes + Hash + Eq + Clone + Default + Debug + Send + Sync;
    type SecretKey: ToBytes + Clone + Default;
    type Signature: Copy + Clone + Default + Send + Sync + Debug + Eq + PartialEq + ToBytes + FromBytes;

    fn keygen<R: Rng>(
        rng: &mut R,
    ) -> (Self::PublicKey, Self::SecretKey);

    fn get_public_key(
        sk: &Self::SecretKey
    ) -> Self::PublicKey;

    fn sign<R: Rng>(
        rng: &mut R,
        pk: &Self::PublicKey,
        sk: &Self::SecretKey,
        message: &[Self::Data],
    ) -> Result<Self::Signature, Error>;

    fn verify(
        pk: &Self::PublicKey,
        message: &[Self::Data],
        signature: &Self::Signature,
    ) -> Result<bool, Error>;

    fn keyverify(
        pk: &Self::PublicKey,
    ) -> bool;
}