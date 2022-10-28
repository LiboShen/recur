use near_contract_standards::non_fungible_token::TokenId;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftOnTransferJson {
    lease_id: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Dummy,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self { owner: owner_id }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    const MINT_COST: u128 = 1000000000000000000000000;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    // TODO: Add tests
    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new(accounts(1).into());

        testing_env!(context
            .storage_usage(env::storage_usage())
            .attached_deposit(MINT_COST)
            .predecessor_account_id(accounts(0))
            .build());
    }
}
