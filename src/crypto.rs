
//! The crypto module acts as a facade for any direct interaction with cryptography.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CryptoAlgorithm {
    DUMMY,
    SECP256K1, // TODO Implement SECP256K1
    CURVE25519, // TODO Implement CURVE25519
    RSA, // TODO Implement RSA
    EXPERIMENTAL, // Implement EXPERIMENTAL
}

#[derive(Clone, PartialEq)]
pub struct SignedMsg {
    algo: CryptoAlgorithm,
    sig: Vec<u8>,
    msg: String,
}

pub struct EncryptedMsg {
    destination: PubKey,
    msg: String,
}

pub struct SignedEncryptedMsg {
    algo: CryptoAlgorithm,
    msg: String,
    sig: Vec<u8>,
    signer: PubKey,
    destination: PubKey,
}

#[derive(Debug)]
pub struct DecryptedMsg {
    success: bool,
    msg: String,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PubKey {
    pub algo: CryptoAlgorithm,
    pub pubk: Vec<u8>,
}

pub struct PrivKey {
    algo: CryptoAlgorithm,
    prik: Vec<u8>,
}

impl SignedMsg {
    pub fn get_msg(&self) -> &String {
        &self.msg
    }
}

impl PubKey {
    // TODO Test
    pub fn verify(&self, signedmsg: &SignedMsg) -> bool {
        if self.algo != signedmsg.algo {
            return false;
        }
        match self.algo {
            CryptoAlgorithm::DUMMY => {
                // TODO DUMMY the signature could be just a xor or the pubk and the last msg characters.
                // It will actually work as expected for shor enough messages.
                signedmsg.msg.len() > 0 && signedmsg.sig.len() > 0
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }

    pub fn encrypt(&self, msg: String) -> EncryptedMsg {
        match self.algo {
            CryptoAlgorithm::DUMMY => {
                // TODO DUMMY the encryption could be just XOR with the pubkey.
                EncryptedMsg{msg, destination: self.clone()}
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }

    pub fn encrypt_signed(&self, msg_signed: SignedMsg, signer: PubKey) -> SignedEncryptedMsg {
        assert!(self.algo == msg_signed.algo);

        match self.algo {
            CryptoAlgorithm::DUMMY => {
                // TODO DUMMY the encryption could be just XOR with the pubkey.
                let enc_msg = self.encrypt(msg_signed.msg);

                SignedEncryptedMsg{
                    algo: self.algo,
                    sig: msg_signed.sig,
                    signer,
                    destination: enc_msg.destination,
                    msg: enc_msg.msg,
                }
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }
}

impl PrivKey {
    pub fn new(algo: CryptoAlgorithm) -> Option<PrivKey> {
        // TODO use real cryptography
        let prik = Vec::<u8>::new();
        match algo {
            CryptoAlgorithm::DUMMY => {
                Some(PrivKey{algo: CryptoAlgorithm::DUMMY, prik})
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }

    pub fn get_pub(&self) -> PubKey {
        match self.algo {
            CryptoAlgorithm::DUMMY => {
                // TODO use real cryptography
                PubKey{algo: self.algo, pubk: self.prik.clone()}
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }

    pub fn sign(&self, msg: String) -> SignedMsg {
        match self.algo {
            CryptoAlgorithm::DUMMY => {
                // TODO use real cryptography
                let sig = vec![1; 8];
                SignedMsg{algo: self.algo, msg, sig}
            },
            CryptoAlgorithm::SECP256K1 => {
                unimplemented!("TODO ? Nobody cares about SECP256K1 anymore anyway");
            },
            CryptoAlgorithm::CURVE25519 => {
                unimplemented!("TODO ? Nobody cares about CURVE25519 anymore anyway");
            },
            CryptoAlgorithm::RSA => {
                unimplemented!("TODO ? Nobody cares about RSA anymore anyway");
            },
            CryptoAlgorithm::EXPERIMENTAL => {
                unimplemented!("TODO ? Nobody cares about new cryptographic algorithms anymore anyway");
            }
        }
    }

    pub fn decrypt(&self, encryptedmsg: EncryptedMsg) -> DecryptedMsg {
        if encryptedmsg.destination != self.get_pub() {
            return DecryptedMsg{msg: "Can't decrypt with this private key".to_string(), success: false}
        }
        // TODO use real cryptography
        DecryptedMsg{msg: encryptedmsg.msg, success: true}
    }

    pub fn decrypt_verify(&self, signed_encrypted_msg: SignedEncryptedMsg) -> DecryptedMsg {
        assert!(self.algo == signed_encrypted_msg.algo);

        if signed_encrypted_msg.destination != self.get_pub() {
            return DecryptedMsg{msg: "Can't decrypt with this private key".to_string(), success: false}
        }

        // TODO use real cryptography
        let msg_dec = signed_encrypted_msg.msg;
        let signedmsg = SignedMsg{msg: msg_dec.clone(), algo: signed_encrypted_msg.algo, sig: signed_encrypted_msg.sig};
        if !signed_encrypted_msg.signer.verify(&signedmsg) {
            return DecryptedMsg{msg: "The signature doesn't match the message and pubkey".to_string(), success: false}
        }

        DecryptedMsg{msg: msg_dec, success: true}
    }
}
