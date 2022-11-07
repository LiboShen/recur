// TODO: Beneficier
// TODO: NFT contract

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::bs58;
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet, Vector};
use near_sdk::serde::ser::SerializeTuple;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

type SubscriptionPlanID = String; // ID for each subscrtion plan
type SubscriptionID = String;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Subscrtion,
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
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SubscriptionPlan {
    provider_id: AccountId, // plan provider
    //TODO: beneficier: AccountId,
    payment_cycle_length: u64, // base payment cycle (e.g. hour, day, week) in the unit of seconds.
    payment_cycle_rate: u128,  // cost for 1 payment cycle
    payment_cycle_count: u64,  // total number of paymens. 0 represents indefinte plan
    // allow_grace_period: u64,    // TODO: grace period in seconds
    plan_name: Option<String>, // name of the plan
    prev_charge_ts: u64, // most recent charge of the plan - used for calculating payment amount
                         // set to 0 at initialisation
}
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
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
    subscription_plan_by_id: UnorderedMap<SubscriptionPlanID, SubscriptionPlan>,
    subscription_by_id: UnorderedMap<SubscriptionID, Subscription>,
    subscrtion_ids_by_plan_id: LookupMap<SubscriptionPlanID, UnorderedSet<SubscriptionID>>, // helper structure for viewing
    deposit_by_account: UnorderedMap<AccountId, u128>, // subscriber and her deposit
                                                       //TODO: deposit_map_multi_token: UnorderedMap<AccountId, UnorderedMap<AccountId, u128>>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            owner: owner_id,
            subscription_plan_by_id: UnorderedMap::new(StorageKey::SubscriptionPlan),
            subscription_by_id: UnorderedMap::new(StorageKey::Subscrtion),
            subscrtion_ids_by_plan_id: LookupMap::new(
                StorageKey::SubscrtionIDs.try_to_vec().unwrap(),
            ),
            deposit_by_account: UnorderedMap::new(StorageKey::Deposit),
        };
        this
    }

    pub fn get_plan(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionPlan {
        let plan = self
            .subscription_plan_by_id
            .get(&plan_id)
            .expect("No such plan!");
        return plan;
    }

    // get all subscriptions of a given plan
    pub fn list_subscriptions_by_plan_id(
        &mut self,
        plan_id: SubscriptionPlanID,
    ) -> Vec<(SubscriptionID, Subscription)> {
        let mut results: Vec<(SubscriptionID, Subscription)> = vec![];

        let ids = self.subscrtion_ids_by_plan_id.get(&plan_id).unwrap();
        for id in ids.iter() {
            let sub = self.subscription_by_id.get(&id).unwrap();
            results.push(id, sub);
        }
        return results;
    }

    // check if a subscriber has enough funds
    pub fn validate_subscription(&mut self, subscription_id: SubscriptionID) {
        todo!()
    }

    // check the depostive amount of a given account
    pub fn get_deposit(&mut self, account: AccountId) -> u128 {
        let mut balance = self
            .deposit_by_account
            .get(&account)
            .expect(format!("No such account {}!", account.to_string()));

        return &balance;
    }

    fn calcuate_subscription_cost(&mut self, subscription_id: SubscriptionID) -> u128 {
        let subscription = self
            .subscription_by_id
            .get(&subscription_id)
            .expect("No such subscription!");

        let mut cost: u128 = 0;
        let plan = self
            .subscription_plan_by_id
            .get(&subscription.plan_id)
            .unwrap();
        let curr_ts = env::block_timestamp();
        let duration = (curr_ts - subscription.start_ts);
        let count_cycle = 1 + duration / &plan.payment_cycle_length;

        cost = (count_cycle as u128) * &plan.payment_cycle_rate;
        return &cost;
    }
}

// functions related to to service provider
pub trait ProviderActions {
    fn create_subscription_plan(
        &mut self,
        provider_id: Option<AccountId>, // if none, use the caller accountid
        payment_cycle_length: u64,
        payment_cycle_rate: u128,
        payment_cycle_count: u64,
        plan_name: Option<String>,
    ) -> SubscriptionPlanID;

    // collect fees from a chosen plan.
    // return a list of tuple representing the subscription and if the charge succeeds
    fn collect_fees(&mut self, plan_id: SubscriptionPlanID) -> Vec<(Subscription, bool)>;
}

pub trait SubscriberActions {
    fn create_subscription(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionID;

    fn cancel_subscription(&mut self, subscription_id: SubscriptionID);

    // function to deposit fund
    // TODO: support multi FTs
    fn deposit(&mut self, subscriber_id: AccountId, amount: u128);

    fn withdraw(&mut self, amount: u128);
}

#[near_bindgen]
impl ProviderActions for Contract {
    fn create_subscription_plan(
        &mut self,
        provider_id: Option<AccountId>, // if none, use the sending account id
        payment_cycle_length: u64,
        payment_cycle_rate: u128,
        payment_cycle_count: u64,
        plan_name: Option<String>,
    ) -> SubscriptionPlanID {
        // if no provider is given, using the sender's account id
        let a_provider_id = provider_id
            // convert the valid provider ID into an account ID
            .map(|a| a.into())
            // if no provider id is given, simply use the caller's ID
            .unwrap_or_else(env::predecessor_account_id());

        assert!(
            payment_cycle_length >= 60,
            "Payment cycle needs to be not less than 1 min!"
        );

        assert!(payment_cycle_rate > 0, "Rate needs to be a postive number!");

        assert!(
            payment_cycle_count >= 0,
            "Payment count needs to be non-negative! "
        );

        // create plan ID
        let curr_ts_string = env::block_timestamp().to_string();
        let mut seed = a_provider_id.as_str().to_owned();
        seed.push_str(&curr_ts_string);

        let plan_id = bs58::encode(seed.into_bytes())
            .with_alphabet(bs58::Alphabet::BITCOIN)
            .into_string();

        // initiate the struct and return
        let a_plan = SubscriptionPlan {
            provider_id: &a_provider_id,
            payment_cycle_length: &payment_cycle_length,
            payment_cycle_rate: &payment_cycle_rate,
            payment_cycle_count: &payment_cycle_count,
            plan_name: plan_name,
            prev_charge_ts: 0,
        };

        // insert the plan into map
        self.subscription_plan_by_id.insert(&plan_id, &a_plan);

        return plan_id;
    }

    // TODO: support multi FTs
    fn collect_fees(&mut self, plan_id: SubscriptionPlanID) -> Vec<(Subscription, bool)> {
        /* collect fees from all valid subscrtion of a given plan:
        For each subscrtion of a plan:
            1. check if the subscription is active
            2. check if payments number exceeds count
            3. calcuate the payment and check if the deposit is enough
            4. record the valid amount and the correct charge state

        transfer the total amount to provider

         */
        todo!()
    }
}

#[near_bindgen]
impl SubscriberActions for Contract {
    fn create_subscription(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionID {
        // get the plan
        let plan = self
            .subscription_plan_by_id
            .get(&plan_id)
            .expect("No such plan!");

        // validate deposit : deposit should cover at least the 1st payment
        let balance = self
            .deposit_by_account
            .get(&subscriber)
            .expect("Deposit first before creating subscrptions!");
        assert!(
            balance >= &plan.payment_cycle_rate,
            format!(
                "Deposit is not enough for first payment {}",
                &plan.payment_cycle_rate
            )
        );

        // subscription can only be created by own account
        let subscriber: AccountId = env::predecessor_account_id();

        // generate an id
        let curr_ts_string = env::block_timestamp().to_string();
        let mut seed = subscriber.as_str().to_owned();
        seed.push_str(&curr_ts_string);

        let subscription_id: SubscriptionID = bs58::encode(seed.into_bytes())
            .with_alphabet(bs58::Alphabet::BITCOIN)
            .into_string();

        // create the subscription
        let a_subscription = Subscription {
            subscriber_id: subscriber,
            plan_id: &plan_id,
            state: SubscriptionState::Active,
            start_ts: env::block_timestamp(),
        };

        //record the new subscription
        self.subscription_by_id
            .insert(&subscription_id, &a_subscription);

        return subscription_id;
    }

    fn cancel_subscription(&mut self, subscription_id: SubscriptionID) {
        todo!()
    }

    // function to top up deposit
    fn deposit(&mut self, subscriber_id: AccountId, amount: u128) {
        assert!(amount > 0, "Deposit must be positive!");

        // get balance of the account, if the account is not in the map, default the balance to 0
        let mut balance: u128 = self.deposit_by_account.get(&subscriber_id).unwrap_or(0);
        balance += deposit;
        self.deposit_by_account.insert(&subscriber_id, &balance);
    }

    fn withdraw(&mut self, amount: u128) {
        todo!()
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
