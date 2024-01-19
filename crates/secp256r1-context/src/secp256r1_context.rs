use sha2::Digest;

use sov_modules_api::default_context::ZkDefaultContext;
use sov_modules_core::{Address, Context, PublicKey, Spec, TupleGasUnit};

#[cfg(feature = "native")]
use crate::secp256r1_signature::private_key::Secp256r1PrivateKey;
use crate::secp256r1_signature::{Secp256r1PublicKey, Secp256r1Signature};


#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "native")]
use sov_modules_api::default_context::DefaultContext;
use sov_modules_api::RollupAddress;

#[cfg(feature = "native")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NativeSecp256r1Context {
    pub sender: Address,
    pub sequencer: Address,
    /// The height to report. This is set by the kernel when the context is created
    visible_height: u64,
}

#[cfg(feature = "native")]
impl Spec for NativeSecp256r1Context {
    type Address = <DefaultContext as Spec>::Address;
    type Storage = <DefaultContext as Spec>::Storage;
    type PrivateKey = Secp256r1PrivateKey;
    type PublicKey = Secp256r1PublicKey;
    type Hasher = <DefaultContext as Spec>::Hasher;
    type Signature = Secp256r1Signature;
    type Witness = <DefaultContext as Spec>::Witness;
}

#[cfg(feature = "native")]
impl Context for NativeSecp256r1Context {
    type GasUnit = TupleGasUnit<2>;

    fn sender(&self) -> &Self::Address {
        &self.sender
    }

    fn sequencer(&self) -> &Self::Address {
        &self.sequencer
    }

    fn new(sender: Self::Address, sequencer: Self::Address, height: u64) -> Self {
        Self {
            sender,
            sequencer,
            visible_height: height,
        }
    }

    fn slot_height(&self) -> u64 {
        self.visible_height
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ZkSecp256r1Context {
    pub sender: Address,
    pub sequencer: Address,
    /// The height to report. This is set by the kernel when the context is created
    visible_height: u64,
}

impl Spec for ZkSecp256r1Context {
    type Address = <ZkDefaultContext as Spec>::Address;
    type Storage = <ZkDefaultContext as Spec>::Storage;
    #[cfg(feature = "native")]
    type PrivateKey = Secp256r1PrivateKey;
    type PublicKey = Secp256r1PublicKey;
    type Hasher = <ZkDefaultContext as Spec>::Hasher;
    type Signature = Secp256r1Signature;
    type Witness = <ZkDefaultContext as Spec>::Witness;
}

impl Context for ZkSecp256r1Context {
    type GasUnit = TupleGasUnit<2>;

    fn sender(&self) -> &Self::Address {
        &self.sender
    }

    fn sequencer(&self) -> &Self::Address {
        &self.sequencer
    }

    fn new(sender: Self::Address, sequencer: Self::Address, height: u64) -> Self {
        Self {
            sender,
            sequencer,
            visible_height: height,
        }
    }

    fn slot_height(&self) -> u64 {
        self.visible_height
    }
}

impl PublicKey for Secp256r1PublicKey {
    fn to_address<A: RollupAddress>(&self) -> A {
        let pub_key_hash = {
            let mut hasher = <ZkDefaultContext as Spec>::Hasher::new();
            hasher.update(self.pub_key.to_encoded_point(true).as_bytes());
            hasher.finalize().into()
        };
        A::from(pub_key_hash)
    }
}
