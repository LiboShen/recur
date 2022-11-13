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
use std::cmp::min;

type SubscriptionPlanID = String; // ID for each subscrtion plan
type SubscriptionID = String;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    SubscrtionById,
    SubscriptionPlanById,
    SubscrtionIdsByPlan,
    SubscrtionIdsByPlanInner { account_id_hash: CryptoHash },
    SubscriptionsPerSubscriber,
    SubscriptionsPerSubscriberInner { account_id_hash: CryptoHash },
    DepositByAccount,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
enum SubscriptionState {
    Active { ts: u64 },   // subscription activated time
    Canceled { ts: u64 }, // subscription canceld time
    Invalid, // When canceld subscrtion passed one more payment cycle, it is ready to be removed
}

// Subscription template
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SubscriptionPlan {
    provider_id: AccountId, // plan provider
    //TODO: beneficier: AccountId,
    payment_cycle_length: u64, // base payment cycle (e.g. hour, day, week) in the unit of seconds.
    payment_cycle_rate: u128,  // cost for 1 payment cycle
    payment_cycle_count: u64,  // total number of payments. 0 represents indefinte plan
    // allow_grace_period: u64,    // TODO: grace period in seconds
    plan_name: Option<String>, // name of the plan
                               // prev_charge_ts: Option<u64>, // most recent charge of the plan - used for calculating payment amount
                               //                              // set to 0 at initialisation
}
// Actual subscription instance based on SubscriptionPlan
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    subscriber_id: AccountId,    // plan subscriber
    plan_id: SubscriptionPlanID, // the subscribed plan
    // prev_charge_ts: u64, // ts of the previous charge. used for deciding whether the next payment is due.
    state: SubscriptionState, // state of the subscripion
    prev_charge_ts: u64,      // most recent charge of the subsciption, initilise to 0
                              // this will be only updated when funds are actually moved from subscriber's account
                              // Charge occurs upfront at the start of a cycle.
}

// helper structure for sorting subscrtions by next payment date
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct SortResultSubscription {
    subscription_id: SubscriptionID,
    next_payment_due_ts: u64,
    incurred_fees: u128,
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

        let deposit = self.get_valid_deposit(&subscription.subscriber_id);
        let cost = self.calcuate_subscription_incurred_cost(subscription_id, charge_ts);

        return deposit >= cost;
    }

    // check the depost after removing occured fees
    pub fn get_valid_deposit(&mut self, account: &AccountId) -> u128 {
        let balance = self
            .deposit_by_account
            .get(&account)
            .expect("The account has no deposit!");

        let total_fees = self.calculate_total_fees_for_subscriber(&account);

        return max(0, balance - total_fees);
    }

    // function to calculate the cost of one subscription. This should cover all states.
    pub fn calcuate_subscription_incurred_cost(
        &mut self,
        subscription_id: &SubscriptionID,
        end_ts: Option<u64>,
    ) -> u128 {
        // end_ts represents the charge period stop ts. if not given, default to current ts
        // 1. Active Subscrtion: check the actual charge_start_time
        // 2. Canceled Subscription: use canceled ts as the charge_end_time,
        //    charge_start_time is the calculated the same way as Active subscription
        // 3. Invalid subscription: no cost. Waiting to be removed from record

        let subscription = self
            .subscription_by_id
            .get(&subscription_id)
            .expect("No such subscription!");

        // get subscription start time
        let mut start_ts: u64 = 0;
        // if end_ts is not given, using the current ts
        let mut charge_end_ts = end_ts.unwrap_or_else(env::block_timestamp);

        match subscription.state {
            SubscriptionState::Active { ts } => {
                start_ts = ts;
            }
            SubscriptionState::Canceled { ts } => {
                charge_end_ts = ts;
            }
            SubscriptionState::Invalid => {
                // invalid subscrtion incurrs no cost and is ready to be removed
                return 0;
            }
        }

        assert!(
            charge_end_ts <= env::block_timestamp(),
            "Charge end time can't be in the furture!"
        );

        // get the plan details
        let plan = self
            .subscription_plan_by_id
            .get(&subscription.plan_id)
            .unwrap();

        // decide the charge period duration
        let prev_charge_ts = subscription.prev_charge_ts;

        // if no charge has been taken before, treat start_ts as one cycle earlier to achive upfront payment
        let charge_start_ts = max(prev_charge_ts, start_ts - &plan.payment_cycle_length);
        let duration = charge_end_ts - charge_start_ts;

        let count_cycle = duration / &plan.payment_cycle_length;
        let cost = (count_cycle as u128) * &plan.payment_cycle_rate;

        return cost;
    }

    // function to calcuate all subscrtions cost for a subscriber
    // This function will be used when calculating withdraw amount of a subscriber
    pub fn calculate_total_fees_for_subscriber(&mut self, subscriber_id: &AccountId) -> u128 {
        //1. get all subscritons of one user
        //2. accumulate fees from all subscriptions

        let mut total_fees: u128 = 0;
        let subscription_ids = self
            .subscriptions_per_subscriber
            .get(&subscriber_id)
            .expect("No subscriptions to charge!");

        for subscription_id in subscription_ids.iter() {
            let sub = self
                .subscription_by_id
                .get(&subscription_id)
                .expect("Invalid subscrtion!");

            total_fees += self.calcuate_subscription_incurred_cost(&subscription_id, None);
        }

        return total_fees;
    }

    // hellper function: tranfer FT to account
    // TODO: support Multi FT
    fn transfer(&self, to: AccountId, amount: Balance) {
        // helper function to perform FT transfer
        Promise::new(to).transfer(amount);
    }

    // Provider or Subscriber can choose to cancel a service
    pub fn cancel_subscription(&mut self, subscription_id: &SubscriptionID) {
        // updating subscription state
        // insert the subscription back
        // cancled subscpriton should incurr cost until current cycle end,
        // which can be derived from prev_payment_ts and cancelation_ts

        let mut subscription = self
            .subscription_by_id
            .get(subscription_id)
            .expect("No such subscription!");

        let plan = self
            .subscription_plan_by_id
            .get(&subscription.plan_id)
            .unwrap(); // A plan must exist if the subscription exists

        assert!(
            plan.provider_id == env::predecessor_account_id()
                || subscription.subscriber_id == env::predecessor_account_id(),
            "Only the subscriber or service provider can cancel a subscription!"
        );

        if let SubscriptionState::Active { ts: start_ts } = subscription.state {
            // when Active subscription is canceled before a charge has even occured,
            // mark the prev charge ts to be one cycle earlier than active start_ts to enforce upfront payment
            if subsciption.prev_charge_ts == 0 {
                subscription.prev_charge_ts = start_ts - plan.payment_cycle_length;
            }
        }

        // update subscription state to Canceled
        subscription.state = SubscriptionState::Canceled {
            ts: env::block_timestamp(),
        };

        // insert back to the index
        self.subscription_by_id
            .insert(subscription_id, &subscription);
    }

    // function to remove invalid subscriptions
    pub fn prune_subscriptions(&self) {
        todo!()
    }

    fn get_next_payment_due_ts(&self, sub: &Subscription) -> u64 {
        let plan = self.subscription_plan_by_id.get(&sub.plan_id).unwrap();

        if let SubscriptionState::Invalid = sub.state {
            return u64::MAX; // invalid subscriton can doesn't has a due date
        }

        let mut due_ts: u64 = 0;
        if sub.prev_charge_ts > 0 {
            due_ts = sub.prev_charge_ts + plan.payment_cycle_length;
        } else if let SubscriptionState::Active { ts: start_ts } = sub.state {
            due_ts = start_ts;
        }

        return due_ts;
    }

    // helper function to check available fund for one subscrition
    fn get_available_fund_for_subscription(&self, subscription_id: &SubscriptionID) -> (u128, u128) {
        /*
        This function takes into consider the timely order of next due payment date.
        Return a tuple (available_fund, incurred_fees)

        Assumption: the number of subscriptions from one account is relatively small.
        So iteration/sort won't be expensive.

        1. Get all subs from the same subscriber
        2. Sort the subs based on their next payment date.
        3. For each sub in sorted_subs:
            Calculate pseudo_available_fund by deducting from account deposit the incurred fees
            Early stop if pseudo_available_fund becomes 0
        4. return (pseudo_available_fund, incurred_fees)
        */
        let mut target_sub = self
            .subscription_by_id
            .get(subscription_id)
            .expect("No such subscription!");

        let sub_ids = self
            .subscriptions_per_subscriber
            .get(&target_sub.subscriber_id)
            .expect("Error when getting subscriptions from subscriber");

        let mut sorted_subs: Vec<SortResultSubscription> = Vec::new();
        for sub_id in sub_ids.iter() {
            let sub = self.subscription_by_id.get(&sub_id).unwrap();
            let plan = self.subscription_plan_by_id.get(&sub.plan_id).unwrap();
            let due_payment_ts = self.get_next_payment_due_ts(&sub);
            let fee = self.calcuate_subscription_incurred_cost(&sub_id, None);

            let sort_sub_result = SortResultSubscription{
                subscription_id: sub_id,
                next_payment_due_ts: due_payment_ts,
                incurred_fees: fee,
            };

            sorted_subs.push(sort_sub_result);
        }
        // sort the result based on payment due ts
        sorted_subs.sort_by_key(|k| k.next_payment_due_ts);

        let pseudo_deposit = self.deposit_by_account.get(&target_sub.subscriber_id).unwrap();
        for sub_result in sorted_subs.iter(){
            pseudo_deposit = max(0, pseudo_deposit - sub_result.incurred_fees);
            if sub_result.subscription_id.eq(subscription_id){
                let fund_for_sub = min(pseudo_deposit, sub_result.incurred_fees);
                return (fund_for_sub, sub_result.incurred_fees);
            }
            if pseudo_deposit <= 0 {
                return (0, sub_result.incurred_fees);
            }
        };        

        // if no sub result return. give a dummy return.
        return(0,0)
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
        //prev_charge_ts: Option<u64>,
    ) -> SubscriptionPlanID;

    // collect fees from a chosen plan.
    // return a list of tuple representing the subscription and the collected fees
    fn collect_fees(
        &mut self,
        plan_id: SubscriptionPlanID,
        charge_ts: Option<u64>,
    ) -> Vec<(SubscriptionID, u128)>;
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
        //prev_charge_ts: Option<u64>,
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
            //prev_charge_ts: prev_charge_ts,
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
    ) -> Vec<(SubscriptionID, u128)> {
        /*
        Collect fees from active & canceled subscrtions. Return a vector of <(sub_id, charged_fee)>
        Move canceled subscrtions to Invalid, when current payment cycle ends
        For each subscriber, fees are charged following time order of payment_due_date

        1. get all subscritions of a plan
        2. for each subscription:
            2.1 If state is Invalid: no fee, continue
            2.2 (available_fund, incurred_fee) = get_available_fund_for_sub(): handles the order by due_payment_time
                2.2.1 if available_fund < incurred_fee: cancel the subscription
                2.2.2 internal_transfer_amount = min(available_fund, incurred_fee)
                      total_fee += internal_transfer_amount
                      internal_transfer(internal_transfer_amount): update deposit table
            2.3 update sub details and insert back to indices
                2.3.1. if State is canceled, check if the current payment cycle has ended,
                        if so, change state to Invalid and continue
                2.3.2 update pre_charge_time
                2.3.3 insert back to indices
        3. transfer total_fee to provider
        */

        let mut charge_ts = charge_ts.unwrap_or_else(env::block_timestamp);
        assert!(
            charge_ts <= env::block_timestamp(),
            "You can't charge for future time!"
        );

        let mut plan = self
            .subscription_plan_by_id
            .get(&plan_id)
            .expect("No such plan!");

        let subscription_ids = self
            .subscription_ids_by_plan_id
            .get(&plan_id)
            .expect("No existing subscriptions!");

        // prepare result
        let mut total_fees: u128 = 0;
        let mut result: Vec<(SubscriptionID, u128)> = vec![];

        for subscription_id in subscription_ids.iter() {
            let subscription = self.subscription_by_id.get(&subscription_id).unwrap();

            // if subscription Invalid, no fees, skip
            if let SubscriptionState::Invalid = subscription.state {
                result.push((subscription_id, 0));
                continue;
            }

            //2.2
            let (available_fund, incurred_fees) =
                self.get_available_fund_for_subscription(&subscription_id);

            
//////////////////////////////TODO Above ^^^/////////////////
            // // if Canceled, check if it has become invalid
            // let mut cancled_ts: u64 = 0;
            // if let SubscriptionState::Canceled { ts: _ } = subscription.state {
            //     continue;
            // }

            // if deposit is not enough, mark false for the result
            // TODO: charge max available amount from deposit
            if !self.validate_subscription(&subscription_id, charge_ts) {
                result.push((subscription_id.clone(), false));
                continue;
            }

            let fee = self.calcuate_subscription_incurred_cost(&subscription_id, charge_ts);
            // udpate deposit
            let mut deposit = self
                .deposit_by_account
                .get(&subscription.subscriber_id)
                .unwrap();
            deposit -= fee;
            self.deposit_by_account
                .insert(&subscription.subscriber_id, &deposit);
            // build result
            result.push((subscription_id.clone(), true));

            // accumulate total fee
            total_fees += fee;
        }

        // update plan details & insert back to index
        plan.prev_charge_ts = charge_ts;
        self.subscription_plan_by_id.insert(&plan_id, &plan);

        self.transfer(plan.provider_id, total_fees);

        return result;
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

        // check validate deposit : deposit should cover at least the 1st paymen
        let valid_deposit = self.get_valid_deposit(&subscriber);
        assert!(
            valid_deposit >= plan.payment_cycle_rate,
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
        let new_subscription = Subscription {
            subscriber_id: subscriber.clone(),
            plan_id: plan_id.clone(),
            state: SubscriptionState::Active {
                ts: env::block_timestamp(),
            },
            prev_charge_ts: 0,
        };

        //update relevant indices
        self.subscription_by_id
            .insert(&subscription_id, &new_subscription);

        //update relevant indices
        // TODO: check if all new unordered sets of different plans from the same provider will be put at the same memory
        let mut subscriptions_ids_set = self
            .subscription_ids_by_plan_id
            .get(&plan_id)
            .unwrap_or_else(|| {
                // if the plan doesn't have any subscription, we create a new unordered set
                UnorderedSet::new(
                    StorageKey::SubscrtionIdsByPlanInner {
                        account_id_hash: hash_account_id(&plan.provider_id),
                    }
                    .try_to_vec()
                    .unwrap(),
                )
            });
        subscriptions_ids_set.insert(&subscription_id);
        self.subscription_ids_by_plan_id
            .insert(&plan_id, &subscriptions_ids_set);

        //update relevant indices
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

        // update index
        self.deposit_by_account.insert(&subscriber_id, &balance);
    }

    // function to withdraw unlocked deposit
    #[payable]
    fn withdraw(&mut self, amount: Option<u128>) {
        // 1. get valid deposit
        // 2. when no input amount is given, set asking_amount to available_fund
        // if asking_amount < available_fund:
        //          update the deposit table
        //          transfer token
        // else: panic
        let user_id = env::predecessor_account_id();

        // withdrawable amount
        let valid_deposit = self.get_valid_deposit(&user_id);

        // if no input amount is given, withdarw all available fund
        let asking_amount = amount.unwrap_or(valid_deposit);

        // panic if not enough fund!
        assert!(valid_deposit >= asking_amount, "Not enough fund!");

        // update deposit index
        let balance = self
            .deposit_by_account
            .get(&user_id)
            .expect("No such account!");
        let new_balance = max(0, balance - asking_amount);
        self.deposit_by_account.insert(&user_id, &new_balance);

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
