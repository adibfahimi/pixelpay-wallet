use crypto::digest::Digest;
use crypto::sha2::Sha256;
use secp256k1::{Message, Secp256k1};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub sender: String,
    pub receiver: String,
    pub amount: i32,
    pub signature: String,
    pub hash: String,
    pub timestamp: i64,
}

impl Tx {
    pub fn new(
        sender: String,
        receiver: String,
        amount: i32,
        signature: String,
        hash: String,
        timestamp: i64,
    ) -> Tx {
        Tx {
            sender,
            receiver,
            amount,
            signature,
            hash,
            timestamp,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let data = format!(
            "{}{}{}{}",
            self.sender, self.receiver, self.amount, self.timestamp
        );
        hasher.input_str(&data);
        hasher.result_str()
    }

    pub fn sign(&mut self, enc_sk: String) {
        self.hash = self.calculate_hash();

        let secp = Secp256k1::new();
        let dec_sk = hex::decode(enc_sk).unwrap();
        let secret_key = secp256k1::SecretKey::from_slice(&dec_sk).unwrap();
        let message = Message::from_slice(&hex::decode(&self.hash).unwrap()).unwrap();
        let sig = secp.sign_ecdsa(&message, &secret_key);

        self.signature = hex::encode(sig.serialize_der());
    }
}
