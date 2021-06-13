use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// this struct has the following properties:
/// 1. address
/// 2. public key
/// 3. private key
/// 4. balance
#[derive(Debug, Clone)]
pub struct Account {
    pub address: String,
    pub public_key: String,
    private_key: String,
    pub balance: u128
}

impl Account {
    pub fn new(pubkey: &str, prvkey: &str) -> Self {
        // combine private and public key to create address
        let mut hash = DefaultHasher::new();
        let mixed_key = pubkey.to_string().to_owned().push_str(prvkey);
        mixed_key.hash(&mut hash);

        Account {
            address: hash.finish().to_string(),
            public_key: pubkey.to_string(),
            private_key: prvkey.to_string(),
            balance: 0
        }
    }
}
