// TODO: Beneficier
// TODO: NFT contract

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::serde::ser::SerializeTuple;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

type SubscriptionPlanID = String; // ID for each subscrtion plan
type SubscriptionID = String;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Subscrtions,
    SubscriptionPlan,
    SubscrtionIDs,
    Deposit,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
enum SubscriptionState {
    Active,
    Canceled,
}

// Subscription template
pub struct SubscriptionPlan {
    provider_id: AccountId, // plan provider
    //TODO: beneficier: AccountId,
    payment_cycle: u64, // base payment cycle (e.g. hour, day, week) in the unit of seconds.
    payment_rate: u64,  // cost for 1 payment cycle
    payment_number: u64, // total number of paymens. 0 represent indefinte plan
    // allow_grace_period: u64,    // TODO: grace period in seconds
    metadata: string,
    prev_charge_ts: u64, // most recent charge of the plan
}

// Actual subscrtion instances based on SubscriptionPlan
pub struct Subscription {
    subscriber_id: AccountId,    // plan subscrtier
    plan_id: SubscriptionPlanID, // which plan is scubribed to
    // prev_charge_ts: u64, // ts of the previous charge. used for deciding whether the next payment is due.
    state: SubscriptionState, // state of the subscrtion
    start_ts: u64,            // start of this subscrtion
}

//Subscription Service Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId, // service owner
    subscription_plans_by_id: UnorderedMap<SubscriptionPlanID, SubscriptionPlan>,
    subscriptions_by_id: UnorderedMap<SubscriptionID, Subscription>,
    subscrtion_ids_by_plan_id: LookupMap<SubscrtionPlanID, UnorderedSet<SubscriptionID>>, // helper structure for viewing
    deposit_map: UnorderedMap<AccountId, u128>, // subscriber and their deposit
                                                //TODO: deposit_map_multi_token: UnorderedMap<AccountId, UnorderedMap<AccountId, u128>>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            owner: owner_id,
            subscription_plans_by_id: UnorderedMap::new(StorageKey::SubscriptionPlan),
            subscriptions_by_id: UnorderedMap::new(StorageKey::Subscrtions),
            subscrtion_ids_by_plan_id: LookupMap::new(StorageKey::SubscrtionIDs),
            deposit_map: UnorderedMap::new(StorageKey::Deposit),
        }
    }

    pub fn get_plan(&mut self, plan_id: SubscriptionPlanID) -> Option<Subscription> {
        self.subscription_plans_by_id.get(&plan_id)
    }

    // function to get all subscription of a given plan
    pub fn list_subscriptions_by_plan_id(
        &mut self,
        plan_id: SubscriptionPlanID,
    ) -> Option<Vector<(SubscriptionID, Subscription)>> {
        let mut results: Vec<(SubscriptionID, Subscription)> = vec![];
        for id in self.subscrtion_ids_by_plan_id.get(&plan_id).iter() {
            results.push((id, self.subscriptions_by_id.get(&id)))
        }
        results
    }

    // check if a subscriber has enough funds
    pub fn validate_subscription(&mut self, subscription_id: SubscriptionID) -> bool;
}

// functions related to to service provider
trait ProviderActions {
    pub fn create_plan(
        &mut self,
        provider_id: AccountId,       // account that creat a plan
        subscrtion_plan_info: string, // JSON string representing a plan details
    ) -> SubscriptionPlanID;

    // collect fees from a chosen plan.
    // return a list of tuple indicating the subscrtion and if the charge succeeds
    pub fn collect_fees(
        &mut self,
        plan_id: SubscriptionPlanID,
    ) -> Vector<SerializeTuple<Subscription, bool>>;
}

trait SubscriberActions {
    pub fn create_subscription(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionID;
    
    pub fn cancel_subscription(&mut self, subscription_id: SubscriptionID) -> bool;
    
    // function to deposit
    // TODO: multi FT
    pub fn deposit(&mut self, subscriber_id: AccountId, amount: u128) -> bool;
    
    pub fn withdraw(&mut self, amount: u128);
}

#[cfg(all(test, not(target_arch = "wasm32")))]
impl ProviderActions for Contract {
    pub fn create_plan(
            &mut self,
            provider_id: AccountId,       // account that creat a plan
            subscrtion_plan_info: string, // JSON string representing a plan details
        ) -> SubscriptionPlanID {
        
    }
}

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
