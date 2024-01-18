use std::hash::Hash;
#[cfg(feature = "native")]
use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use p256::ecdsa::{signature::Verifier,
                  {Signature as p256Signature,
                   VerifyingKey}};

use sov_modules_core::{SigVerificationError, Signature};
use sov_rollup_interface::anyhow;

pub(crate) const PUBLIC_KEY_LENGTH: usize = 33; // Compressed public key length in bytes
const PRIVATE_KEY_LENGTH: usize = 32; // Private key length in bytes
const SIGNATURE_LENGTH: usize = 64; // Signature length in bytes


#[cfg(feature = "native")]
pub mod private_key {
    use std::ops::Deref;
    use p256::{
        ecdsa::{SigningKey,
                signature::Signer,
        }
    };
    use rand::rngs::OsRng;
    use sov_modules_core::{Address, PrivateKey, PublicKey};

    use serde::{Serialize, Deserialize, Serializer, Deserializer};
    use super::PRIVATE_KEY_LENGTH;

    use super::{Secp256r1PublicKey, Secp256r1Signature};

    #[derive(Clone)]
    struct RealSigningKey(SigningKey);

    /// A private key for the default signature scheme.
    /// This struct also stores the corresponding public key.
    #[derive(Clone, Serialize, Deserialize)]
    pub struct Secp256r1PrivateKey {
        key_pair: RealSigningKey,
    }

    impl Serialize for RealSigningKey {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let binding = self.0.to_bytes();
            let bytes: &[u8] = binding.deref();
            serializer.serialize_bytes(bytes)
        }
    }

    impl<'d> Deserialize<'d> for RealSigningKey {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'d>,
        {
            struct RealSigningKeyVisitor;

            impl<'de> serde::de::Visitor<'de> for RealSigningKeyVisitor {
                type Value = RealSigningKey;

                fn expecting(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(formatter, concat!("An secp256r1 signing (private) key"))
                }

                fn visit_borrowed_bytes<E: serde::de::Error>(
                    self,
                    bytes: &'de [u8],
                ) -> Result<Self::Value, E> {
                    let key_pair = SigningKey::from_slice(bytes.as_ref()).map_err(E::custom)?;
                    Ok(RealSigningKey(key_pair))
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                {
                    let mut bytes = [0u8; PRIVATE_KEY_LENGTH];
                    for i in 0..32 {
                        bytes[i] = seq
                            .next_element()?
                            .ok_or_else(|| serde::de::Error::invalid_length(i, &"expected 32 bytes"))?;
                    }

                    let remaining = (0..)
                        .map(|_| seq.next_element::<u8>())
                        .take_while(|el| matches!(el, Ok(Some(_))))
                        .count();

                    if remaining > 0 {
                        return Err(serde::de::Error::invalid_length(
                            32 + remaining,
                            &"expected 32 bytes",
                        ));
                    }

                    let key_pair = SigningKey::from_slice(&bytes).map_err(serde::de::Error::custom)?;
                    Ok(RealSigningKey(key_pair))
                }
            }

            deserializer.deserialize_bytes(RealSigningKeyVisitor)
        }
    }

    impl core::fmt::Debug for Secp256r1PrivateKey {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Secp256r1PrivateKey")
                .field("public_key", &self.key_pair.0.verifying_key())
                .field("private_key", &"***REDACTED***")
                .finish()
        }
    }

    impl TryFrom<&[u8]> for Secp256r1PrivateKey {
        type Error = anyhow::Error;

        fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            if value.len() == PRIVATE_KEY_LENGTH {
                let key_pair = SigningKey::from_slice(&value)?;
                Ok(Self { key_pair: RealSigningKey(key_pair) })
            } else {
                Err(anyhow::anyhow!("Invalid private key length"))
            }
        }
    }

    impl PrivateKey for Secp256r1PrivateKey {
        type PublicKey = Secp256r1PublicKey;
        type Signature = Secp256r1Signature;

        fn generate() -> Self {
            let mut csprng = OsRng;

            Self {
                key_pair: RealSigningKey(SigningKey::random(&mut csprng)),
            }
        }

        fn pub_key(&self) -> Self::PublicKey {
            Secp256r1PublicKey {
                pub_key: self.key_pair.0.verifying_key().clone(),
            }
        }

        fn sign(&self, msg: &[u8]) -> Self::Signature {
            Secp256r1Signature {
                msg_sig: self.key_pair.0.sign(msg),
            }
        }
    }

    impl Secp256r1PrivateKey {
        pub fn as_hex(&self) -> String {
            hex::encode(self.key_pair.0.to_bytes())
        }

        pub fn from_hex(hex: &str) -> anyhow::Result<Self> {
            let bytes = hex::decode(hex)?;
            Self::try_from(&bytes[..])
        }

        pub fn default_address(&self) -> Address {
            self.pub_key().to_address::<Address>()
        }
    }
}

#[cfg_attr(feature = "native", derive(schemars::JsonSchema))]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Secp256r1PublicKey {
    #[cfg_attr(
    feature = "native",
    schemars(with = "&[u8]", length(equal = "PUBLIC_KEY_LENGTH"))
    )]
    pub(crate) pub_key: VerifyingKey,
}

impl Hash for Secp256r1PublicKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pub_key.to_encoded_point(true).as_bytes().hash(state);
    }
}

impl BorshDeserialize for Secp256r1PublicKey {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0; PUBLIC_KEY_LENGTH];
        reader.read_exact(&mut buffer)?;
        let pub_key = VerifyingKey::from_sec1_bytes(&buffer)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Invalid public key"))?;
        Ok(Self { pub_key })
    }
}

// TODO: I'm a little suspicious here that from_sec1_bytes above and the to_encoded_point().as_bytes()
// below expect the same encoding. But gonna roll with this for now.
impl BorshSerialize for Secp256r1PublicKey {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.pub_key.to_encoded_point(true).as_bytes())
    }
}

impl TryFrom<&[u8]> for Secp256r1PublicKey {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
            if value.len() == PUBLIC_KEY_LENGTH {
                let pub_key = VerifyingKey::from_sec1_bytes(value)?;
                Ok(Self { pub_key })
            } else {
                Err(anyhow::anyhow!("Invalid public key length"))
            }
    }
}

#[cfg_attr(feature = "native", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Secp256r1Signature {
    #[cfg_attr(
    feature = "native",
    schemars(with = "&[u8]", length(equal = "SIGNATURE_LENGTH"))
    )]
    pub msg_sig: p256Signature,
}

impl BorshDeserialize for Secp256r1Signature {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0; SIGNATURE_LENGTH];
        reader.read_exact(&mut buffer)?;

        Ok(Self {
            msg_sig: p256Signature::from_bytes((&buffer).into())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Invalid signature"))?
        })
    }
}

impl BorshSerialize for Secp256r1Signature {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.msg_sig.to_bytes())
    }
}

impl TryFrom<&[u8]> for Secp256r1Signature {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            msg_sig: p256Signature::from_bytes(value.into()).map_err(anyhow::Error::msg)?,
        })
    }
}

impl Signature for Secp256r1Signature {
    type PublicKey = Secp256r1PublicKey;

    fn verify(&self, pub_key: &Self::PublicKey, msg: &[u8]) -> Result<(), SigVerificationError> {
        pub_key
            .pub_key
            .verify(msg, &self.msg_sig)
            .map_err(|e| SigVerificationError::BadSignature(e.to_string()))
    }
}

#[cfg(feature = "native")]
impl FromStr for Secp256r1PublicKey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pk_hex = &crate::pub_key_hex::PublicKeyHex::try_from(s)?;
        pk_hex.try_into()
    }
}

#[cfg(feature = "native")]
impl FromStr for Secp256r1Signature {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s)?;

        Ok(Secp256r1Signature {
            msg_sig: p256Signature::from_slice(&bytes)?
        })
    }
}
//
// #[test]
// #[cfg(feature = "native")]
// fn test_privatekey_serde_bincode() {
//     use self::private_key::Secp256r1PrivateKey;
//     use crate::PrivateKey;
//
//     let key_pair = Secp256r1PrivateKey::generate();
//     let serialized = bincode::serialize(&key_pair).expect("Serialization to vec is infallible");
//     let output = bincode::deserialize::<Secp256r1PrivateKey>(&serialized)
//         .expect("SigningKey is serialized correctly");
//
//     assert_eq!(key_pair.as_hex(), output.as_hex());
// }
//
// #[test]
// #[cfg(feature = "native")]
// fn test_privatekey_serde_json() {
//     use self::private_key::Secp256r1PrivateKey;
//     use crate::PrivateKey;
//
//     let key_pair = Secp256r1PrivateKey::generate();
//     let serialized = serde_json::to_vec(&key_pair).expect("Serialization to vec is infallible");
//     let output = serde_json::from_slice::<Secp256r1PrivateKey>(&serialized)
//         .expect("Keypair is serialized correctly");
//
//     assert_eq!(key_pair.as_hex(), output.as_hex());
// }
