use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Account {
    address: String,
    public_key: String,
    private_key: String,
    balance: u128
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
