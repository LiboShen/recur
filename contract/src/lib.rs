// TODO: Beneficier
// TODO: NFT contract

use near_contract_standards::non_fungible_token::hash_account_id;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::bs58;
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::Serialize;
use near_sdk::{
    env, near_bindgen, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault, Promise,
};

use std::cmp::max;

type SubscriptionPlanID = String; // ID for each subscrtion plan
type SubscriptionID = String;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    SubscrtionById,
    SubscriptionPlanById,
    SubscrtionIdsByPlan,
    SubscriptionsPerSubscriber,
    SubscriptionsPerSubscriberInner { account_id_hash: CryptoHash },
    DepositByAccount,
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
    prev_charge_ts: Option<u64>, // most recent charge of the plan - used for calculating payment amount
                                 // set to 0 at initialisation
}
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
// Actual subscription instance based on SubscriptionPlan
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
    subscription_ids_by_plan_id: LookupMap<SubscriptionPlanID, UnorderedSet<SubscriptionID>>, // helper structure for viewing
    subscriptions_per_subscriber: LookupMap<AccountId, UnorderedSet<SubscriptionID>>, // heper structure to group all subscrtions under one user
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
            subscription_plan_by_id: UnorderedMap::new(StorageKey::SubscriptionPlanById),
            subscription_by_id: UnorderedMap::new(StorageKey::SubscrtionById),
            subscription_ids_by_plan_id: LookupMap::new(
                StorageKey::SubscrtionIdsByPlan.try_to_vec().unwrap(),
            ),
            subscriptions_per_subscriber: LookupMap::new(
                StorageKey::SubscriptionsPerSubscriber.try_to_vec().unwrap(),
            ),
            deposit_by_account: UnorderedMap::new(StorageKey::DepositByAccount),
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

        let ids = self.subscription_ids_by_plan_id.get(&plan_id).unwrap();
        for id in ids.iter() {
            let sub = self.subscription_by_id.get(&id).unwrap();
            results.push((id, sub));
        }
        return results;
    }

    // check if a subscriber has enough funds
    // this can be used by providers to decide if service should be suspended
    pub fn validate_subscription(
        &mut self,
        subscription_id: &SubscriptionID,
        charge_ts: Option<u64>,
    ) -> bool {
        //check deposit
        //check currrent cost
        //compare

        let subscription = self
            .subscription_by_id
            .get(subscription_id)
            .expect("No such subscription!");

        let deposit = self.get_deposit(&subscription.subscriber_id);
        let cost = self.calcuate_subscription_cost(subscription_id, charge_ts);

        return deposit >= cost;
    }

    // check the depostive amount of a given account
    pub fn get_deposit(&mut self, account: &AccountId) -> u128 {
        let balance = self
            .deposit_by_account
            .get(&account)
            .expect("No such account!");

        return balance;
    }

    // function to calculate the cost of one subscription
    fn calcuate_subscription_cost(
        &mut self,
        subscription_id: &SubscriptionID,
        end_ts: Option<u64>,
    ) -> u128 {
        // end_ts represents the charge period stop ts. if not given, default to current ts

        let subscription = self
            .subscription_by_id
            .get(&subscription_id)
            .expect("No such subscription!");

        // get the plan details
        let plan = self
            .subscription_plan_by_id
            .get(&subscription.plan_id)
            .unwrap();

        // decide the charge period duration
        let charge_end_ts = end_ts.unwrap_or_else(env::block_timestamp); // if end_ts is not given, using the current ts
        let prev_charge_ts = plan.prev_charge_ts.unwrap_or(0);

        assert!(
            charge_end_ts <= env::block_timestamp(),
            "Charge end time can't be in the furture"
        );


        // if the plan has been charged previously, calcualte using updated time
        // treat start_ts as one cycle earlier to achive upfront payment
        let charge_start_ts = max(prev_charge_ts, subscription.start_ts - &plan.payment_cycle_length);
        
        let duration = charge_end_ts - charge_start_ts;

        // calcuate cost. Subscriber will always be charged upfront for 1 cycle.
        let count_cycle = 1 + duration / &plan.payment_cycle_length;
        let cost = (count_cycle as u128) * &plan.payment_cycle_rate;

        return cost;
    }

    // function to calcuate all subscrtions cost from a subscriber
    // This function will be used when calculating withdraw amount of a subscriber
    fn calculate_total_cost_of_subscriber(&mut self, subscriber_id: &AccountId) -> u128 {
        //1. get all subscritons of one user
        //2. accumulate cost from all active subscriptions

        let mut total_cost: u128 = 0;
        let subscription_ids = self
            .subscriptions_per_subscriber
            .get(&subscriber_id)
            .expect("No subscriptions to charge!");

        for sub_id in subscription_ids.iter() {
            let sub = self
                .subscription_by_id
                .get(&sub_id)
                .expect("Invalid subscrtion!");
            // skip cancled subscrtion
            if let SubscriptionState::Canceled = sub.state {
                continue;
            }

            total_cost += self.calcuate_subscription_cost(&sub_id, None);
        }

        return total_cost;
    }

    // hellper function: tranfer FT to account
    // TODO: support Multi FT
    fn transfer(&self, to: AccountId, amount: Balance) {
        // helper function to perform FT transfer
        Promise::new(to).transfer(amount);
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
        prev_charge_ts: Option<u64>,
    ) -> SubscriptionPlanID;

    // collect fees from a chosen plan.
    // return a list of tuple representing the subscription and if the charge succeeds
    fn collect_fees(
        &mut self,
        plan_id: SubscriptionPlanID,
        charge_ts: Option<u64>,
    ) -> Vec<(SubscriptionID, bool)>;

    // A provider can choose to stop a service e.g. when a subscription is overdue.
    fn stop_subscription(&mut self, subscription_id: &SubscriptionID);
}

pub trait SubscriberActions {
    fn create_subscription(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionID;

    // function to deposit fund
    // TODO: support multi FTs
    fn deposit(&mut self, subscriber_id: AccountId);

    fn withdraw(&mut self, amount: Option<u128>);
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
        prev_charge_ts: Option<u64>,
    ) -> SubscriptionPlanID {
        // if no provider is given, using the sender's account id
        let provider_id = provider_id
            // convert the valid provider ID into an account ID
            .map(|a| a.into())
            // if no provider id is given, simply use the caller's ID
            .unwrap_or_else(env::predecessor_account_id);

        assert!(
            payment_cycle_length >= 60,
            "Payment cycle needs to be not less than 1 min!"
        );

        assert!(payment_cycle_rate > 0, "Rate needs to be a postive number!");

        assert!(
            payment_cycle_count > 0,
            "Payment count needs to be positive! "
        );

        // create plan ID
        let curr_ts_string = env::block_timestamp().to_string();
        let mut seed = provider_id.as_str().to_owned();
        seed.push_str(&curr_ts_string);

        let plan_id = bs58::encode(seed.into_bytes())
            .with_alphabet(bs58::Alphabet::BITCOIN)
            .into_string();

        // initiate the struct and return
        let a_plan = SubscriptionPlan {
            provider_id: provider_id,
            payment_cycle_length: payment_cycle_length,
            payment_cycle_rate: payment_cycle_rate,
            payment_cycle_count: payment_cycle_count,
            plan_name: plan_name,
            prev_charge_ts: prev_charge_ts,
        };

        // insert the plan into map
        self.subscription_plan_by_id.insert(&plan_id, &a_plan);

        return plan_id;
    }

    // TODO: support multi FTs
    #[payable]
    fn collect_fees(
        &mut self,
        plan_id: SubscriptionPlanID,
        charge_ts: Option<u64>,
    ) -> Vec<(SubscriptionID, bool)> {
        /* collect fees from all valid subscrtions of a given plan:
        For each subscrtion of a plan:
            1. check if the subscription is active
            2. calculate the correct payment
            3. validate if deposit is enough
            3. accumulate total fees
            4. record charge result & update deposit table

        transfer the total fees to provider
        update plan prev_charge_ts

        transfer the total fees to provider
        */

        // let charge_ts = charge_ts.unwrap_or_else(env::block_timestamp);
        if charge_ts.is_some() {
            assert!(
                charge_ts.unwrap() <= env::block_timestamp(),
                "You can't charge for future time!"
            );
        }

        let mut plan = self
            .subscription_plan_by_id
            .get(&plan_id)
            .expect("No such plan!");

        let subscription_ids = self
            .subscription_ids_by_plan_id
            .get(&plan_id)
            .expect("No existing subscrtions!");

        let mut total_fees: u128 = 0;
        let mut result: Vec<(SubscriptionID, bool)> = vec![];

        for sub_id in subscription_ids.iter() {
            let subscription = self.subscription_by_id.get(&sub_id).unwrap();

            // if subscription is not active, skip
            if subscription.state == SubscriptionState::Canceled {
                continue;
            }

            // if deposit is not enough mark false for the result
            // TODO: charge max available amount from deposit
            if !self.validate_subscription(&sub_id, charge_ts) {
                result.push((sub_id.clone(), false));
                continue;
            }

            let fee = self.calcuate_subscription_cost(&sub_id, charge_ts);
            // udpate deposit
            let mut deposit = self
                .deposit_by_account
                .get(&subscription.subscriber_id)
                .unwrap();
            deposit -= fee;
            self.deposit_by_account
                .insert(&subscription.subscriber_id, &deposit);
            // build result
            result.push((sub_id.clone(), true));

            // accumulate total fee
            total_fees += fee;
        }

        // update plan details & insert back to index
        plan.prev_charge_ts = charge_ts;
        self.subscription_plan_by_id.insert(&plan_id, &plan);

        self.transfer(plan.provider_id, total_fees);

        return result;
    }

    fn stop_subscription(&mut self, subscription_id: &SubscriptionID) {
        // only the service provider can stop the service
        // stop by updating subscription state
        // insert the subscription back

        let mut subscription = self
            .subscription_by_id
            .get(subscription_id)
            .expect("No such subscription!");

        // A plan must exist if a subscrtion exists
        let plan = self
            .subscription_plan_by_id
            .get(&subscription.plan_id)
            .unwrap();

        assert!(
            plan.provider_id == env::predecessor_account_id(),
            "Only the service provider can cancel the subscrtion!"
        );

        // update state
        subscription.state = SubscriptionState::Canceled;

        // insert back to the index
        self.subscription_by_id
            .insert(subscription_id, &subscription);
    }
}

#[near_bindgen]
impl SubscriberActions for Contract {
    fn create_subscription(&mut self, plan_id: SubscriptionPlanID) -> SubscriptionID {
        // subscription can only be created by own account
        let subscriber: AccountId = env::predecessor_account_id();

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
            balance >= plan.payment_cycle_rate,
            "Deposit is not enough for first payment {rate}",
            rate = &plan.payment_cycle_rate
        );

        // generate an id
        let curr_ts_string = env::block_timestamp().to_string();
        let mut seed = subscriber.as_str().to_owned();
        seed.push_str(&curr_ts_string);

        let subscription_id: SubscriptionID = bs58::encode(seed.into_bytes())
            .with_alphabet(bs58::Alphabet::BITCOIN)
            .into_string();

        // create the subscription
        let a_subscription = Subscription {
            subscriber_id: subscriber.clone(),
            plan_id: plan_id.clone(),
            state: SubscriptionState::Active,
            start_ts: env::block_timestamp(),
        };

        //record the new subscription in relevant indices
        self.subscription_by_id
            .insert(&subscription_id, &a_subscription);

        let mut subscriptions_ids_set = self
            .subscription_ids_by_plan_id
            .get(&plan_id)
            .unwrap_or_else(|| {
                // if the plan doesn't have any subscriptions, we create a new unordered set
                UnorderedSet::new(StorageKey::SubscrtionIdsByPlan.try_to_vec().unwrap())
            });
        subscriptions_ids_set.insert(&subscription_id);
        self.subscription_ids_by_plan_id
            .insert(&plan_id, &subscriptions_ids_set);

        let mut subscriptions_ids_set_2 = self
            .subscriptions_per_subscriber
            .get(&subscriber)
            .unwrap_or_else(|| {
                UnorderedSet::new(
                    StorageKey::SubscriptionsPerSubscriberInner {
                        //get a new unique prefix for the set
                        account_id_hash: hash_account_id(&subscriber),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        subscriptions_ids_set_2.insert(&subscription_id);
        self.subscriptions_per_subscriber
            .insert(&subscriber, &subscriptions_ids_set_2);

        return subscription_id;
    }

    // function to top up deposit
    #[payable]
    fn deposit(&mut self, subscriber_id: AccountId) {
        // 1. transfer fund to current contract
        // 2. update the deposit table
        // one should be able to deposit to other's ccount too

        let amount = env::attached_deposit();
        assert!(amount > 0, "Deposit must be positive!");

        // get balance of the account, if the account is not in the map, default the balance to 0
        let mut balance: u128 = self.deposit_by_account.get(&subscriber_id).unwrap_or(0);
        balance += &amount;
        self.deposit_by_account.insert(&subscriber_id, &balance);
    }

    // function to withdraw unlocked deposit
    #[payable]
    fn withdraw(&mut self, amount: Option<u128>) {
        // 1. get total cost from all subscrtions
        // 2. find available_fund = deposit - total_cost
        // 3. when not input amount is given, set asking_amount to available_fund
        // if asking_amount < available_fund:
        //          transfer token
        //          update the deposit table
        // else: panic
        let user_id = env::predecessor_account_id();

        // find withdrawable amount
        let deposit = self.get_deposit(&user_id);
        let total_cost = self.calculate_total_cost_of_subscriber(&user_id);
        assert!(
            deposit >= total_cost,
            "No available fund! Account: {}",
            &user_id
        );

        let available_fund = deposit - total_cost;

        // if no input amount is given, withdarw all available fund
        let asking_amount = amount.unwrap_or(available_fund);
        assert!(available_fund >= asking_amount, "Not enough fund!");

        // transfer token
        self.transfer(user_id, asking_amount);
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
