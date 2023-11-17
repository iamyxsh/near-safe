use std::collections::HashMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, PanicOnDefault, require, log};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};


const INITIAL_BALANCE: Balance = 1_000_000_000_000_000_000_000_000; // 1NEAR

#[near_bindgen]
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize,)]
#[serde(crate = "near_sdk::serde")]
pub struct GetSafesByOwner {
    safes: Vec<AccountId>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Contract {
    owner: AccountId,
    safes_by_owner: HashMap<AccountId, Vec<AccountId>>,
}
#[near_bindgen]
impl Contract {
    #[init]
    pub fn init() -> Self {
            Self {
                owner: env::signer_account_id(),
                safes_by_owner: HashMap::new()
            }
    }

    pub fn create_safe(&mut self, name: String, code: Vec<u8>) -> (Vec<AccountId>, String) {
        assert_eq!(self.owner, env::signer_account_id(), "only owner can call this function");

        let subaccount_id = AccountId::new_unchecked(
            format!("{}.{}", name.as_str(), env::signer_account_id())
        );

        require!(
            env::is_valid_account_id(subaccount_id.as_bytes()),
            "Invalid subaccount"
        );

        Promise::new(subaccount_id.clone())
            .create_account()
            .transfer(INITIAL_BALANCE)
            .deploy_contract(code);
        let mut binding = Vec::<AccountId>::new();
        let safes: &mut Vec<AccountId> = match self.safes_by_owner.get_mut(&env::predecessor_account_id()) {
            None => &mut binding,
            Some(v) => v
        };

        safes.push(AccountId::new_unchecked(format!("{}.{}", name, env::current_account_id())));

        log!("{:?} {:?}", safes, env::signer_account_id());
        env::log_str(format!("{:?} {:?}", safes, env::signer_account_id()).as_str());

        return (safes.to_owned().to_vec() ,env::signer_account_id().to_string());
    }

    pub fn get_safes_by_owner(&self, owner_id: AccountId) -> GetSafesByOwner {
        let safes = self.safes_by_owner.get(&owner_id).unwrap().to_owned().to_vec();
        GetSafesByOwner{
            safes
        }

    }
}