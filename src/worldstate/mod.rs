use super::account::Account;

// what kind of data will our world_state hold?
// in our case, we are moving around coins: add_coins and transfer_coins
// we also have account management: create_account, get_account, lock_account and unlock_account
// we will also use a merkle tree kind of thing like hyperledger sawtooth does to ensure data
// consistency

// would it make sense to create three seperate traits: coin_management, account_management and
// verfication?

pub trait WorldState {
    fn create_account(&mut self, account: &Account) -> Result<(), &str>;
}
