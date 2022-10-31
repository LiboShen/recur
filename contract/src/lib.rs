// TODO: Beneficier
// TODO: NFT contract

use near_contract_standards::non_fungible_token::TokenId;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, UnorderedMap};
use near_sdk::serde::ser::SerializeTuple;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NftOnTransferJson {
    lease_id: String,
}

type SubscriptionPlanID = String; // ID for each subscrtion plan
type SubscriptionID = String; 

pub struct SubscrtionPlan {
    provider_id: AccountId,   // plan provider
    //TODO: beneficier: AccountId,    
    payment_cycle: u64,       // base payment cycle (e.g. hour, day, week) in the unit of seconds.
    payment_rate: u64,        // cost for 1 payment cycle
    payment_number: u64,       // total number of paymens. 0 represent indefinte plan
    // allow_grace_period: u64,    // TODO: grace period in seconds
    metadata: string,           // context info for meta
    prev_charge_ts: u64,        // most resent charge of the plan
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
enum SubscrtionState {
    Active,
    Canceled,
}

pub struct SubscrtionInfo {
    subscriber_id: AccountId,    // plan subscrtier
    plan_id: SubscriptionPlanID, // which plan is scubribed to
    // prev_charge_ts: u64, // ts of the previous charge. used for deciding whether the next payment is due.
    state: SubscrtionState, // state of the subscrtion
    start_ts: u64,          // start of this subscrtion
}

//Subscrtion Service contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId, // serverice owner
    subscrtion_plans: UnorderedMap<SubscriptionPlanID, SubscrtionPlan>,
    subscrtions: UnorderedMap<SubscriptionID, SubscrtionInfo>,
    // TODO: subscriton_ids_by_plan_id: UnorderedMap<SubscriptionPlanID. SubscriptionId>,
    deposit_map: UnorderedMap<AccountId, u128>, // subscriber and their deposit
    //TODO: deposit_map_multi_token: UnorderedMap<AccountId, UnorderedMap<AccountId, u128>> 
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    SubscrtionsKey,
}

// functions related to to service provider
trait ProviderActions {
    pub fn create_a_plan(
        &mut self,
        provider_id: AccountId,       // account that creat a plan
        subscrtion_plan_info: string, // JSON string representing a plan details
    ) -> SubscriptionPlanID;

    // collect fees from a chosen plan. 
    // return a list of tuple indicating the subscrtion and if the charge succeeds
    pub fn collect_fees(
        &mut self,
        plan_id: SubscriptionPlanID,
    ) -> Vector<SerializeTuple<SubscrtionInfo, bool>>;

    
}

trait SubscriberActions {
    pub fn create_subscription(
        &mut self,
        plan_id: SubscriptionPlanID,
    ) -> SubscriptionID;

    pub fn cancel_subscription(
        &mut self, subscription_id: SubscriptionID
    ) -> bool;

    // function to deposit 
    // TODO: multi FT
    pub fn deposit(
        &mut self,
        subscriber_id: AccountId,
        amount: u128
    ) -> bool;


    pub fn withdraw(
        &mut self,
        amount: u128,
    );
}

trait ServiceOperations {
    pub fn get_plan(&mut self, plan_id: SubscriptionPlanID) -> SubscrtionInfo;

    // function to check all subscribers of a plan
    pub fn list_subscrtions_by_plan_id(&mut self, plan_id: SubscriptionPlanID) -> Vector<SubscrtionInfo>;

    // check if subscriber has enough funds
    pub fn valid_subscrtion(
        &mut self,
        subscription_id: SubscriptionID,
    ) -> bool;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: owner_id,
            subscrtion_plans: UnorderedMap::new(StorageKey::SubscrtionsKey),
            subscrtions: UnorderedMap::new(StorageKey::SubscrtionsKey),
            // TODO: subscriton_ids_by_plan_id: UnorderedMap<SubscriptionPlanID. SubscriptionId>,
            deposit_map: UnorderedMap::new(StorageKey::SubscrtionsKey), // subscriber and their deposit
        }
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
