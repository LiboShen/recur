import "near-api-js/dist/near-api-js.min.js";
const { connect, Contract, keyStores, WalletConnection } = window.nearApi;
import { getConfig } from "./near-config";

export const nearConfig = getConfig(import.meta.env.MODE || "development");

// Initialize contract & set global variables
export async function initContract() {
  // Initialize connection to the NEAR testnet
  const near = await connect(
    Object.assign(
      { deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } },
      nearConfig
    )
  );

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  window.walletConnection = new WalletConnection(near);

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId();

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(
    window.walletConnection.account(),
    nearConfig.contractName,
    {
      viewMethods: [
        "get_plan",
        "list_subscriptions_by_subscriber",
        "list_plans_by_provider",
        "get_withdrawable_deposit",
        "view_collectable_fees_per_plan",
        "view_collectable_fees_per_provider",
      ],
      changeMethods: [
        "create_subscription_plan",
        "create_subscription",
        "collect_fees",
        "cancel_subscription",
        "deposit",
        "withdraw",
      ],
    }
  );
}

export function signOutNearWallet() {
  window.walletConnection.signOut();
  // reload page
  window.location.replace(window.location.origin + window.location.pathname);
}

export function signInWithNearWallet() {
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName);
}

export async function mySubscriptions() {
  let subs = await window.contract.list_subscriptions_by_subscriber({
    subscriber_id: window.accountId,
  });
  return subs;
}

export async function myPlans() {
  let plans = await window.contract.list_plans_by_provider({
    provider_id: window.accountId,
  });
  return plans;
}

export async function myBalance() {
  let balance = await window.contract.get_withdrawable_deposit({
    account: window.accountId,
  });
  return balance;
}


export async function getPlan(planId) {
  if (planId === null) { return null }
  let plan = await window.contract.get_plan({
    plan_id: planId
  });
  return plan;
}

export async function getCollectableFeesForPlan(planId) {
  if (planId === null) { return null }
  let amount = await window.contract.view_collectable_fees_per_plan({
    plan_id: planId
  });
  return amount;
}


const BASE = BigInt("10000000000000000000000");
export async function createPlan(name, cycleLength, priceNear) {
  let priceYacto = BigInt(Number(priceNear * 100).toFixed(0)) * BASE
  let args = {
    provider_id: window.accountId,
    payment_cycle_length: cycleLength,
    payment_cycle_rate: priceYacto.toString(),
    payment_cycle_count: 1,
    plan_name: name,
  }
  let response = await window.contract.create_subscription_plan({
    args: args,
  });
  return response;
}

export async function createSubscription(planId) {
  let response = await window.contract.create_subscription({
    args: {
      plan_id: planId,
    },
  });
  return response;
}

export async function cancelSubscription(subscriptionId) {
  let response = await window.contract.cancel_subscription({
    args: {
      subscription_id: subscriptionId,
    },
  });
  return response;
}

export async function deposit(amountNear) {
  let amountYacto = BigInt(Number(amountNear * 100).toFixed(0)) * BASE;
  let response = await window.contract.deposit({
    args: {
      subscriber_id: window.accountId,
    },
    amount: amountYacto.toString()
  });
  return response;
}

export async function withdraw(amountNear) {
  let amountYacto;
  if (amountNear !== null) {
    amountYacto = (BigInt(Number(amountNear * 100).toFixed(0)) * BASE).toString();
  } else {
    amountYacto = null;
  }
  let response = await window.contract.withdraw({
    args: {
      amount: amountYacto,
    },
  });
  return response;
}


export async function collectFees(planId) {
  let response = await window.contract.collect_fees({
    args: {
      plan_id: planId,
    },
  });
  return response;
}
