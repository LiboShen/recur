import "near-api-js/dist/near-api-js.min.js";
const { connect, Contract, keyStores, WalletConnection } = window.nearApi;
import { getConfig } from "./near-config";

export const nearConfig = getConfig(import.meta.env.MODE || "development");

export const PLAN_ID = "2et2iTwBw8Qq7AkokTsQdYuL3V4xbJC2QeiJ7xmJW2c";
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
      viewMethods: ["find_valid_subscription"],
      changeMethods: [],
    }
  );

  window.isMember = await findValidSubscription(PLAN_ID) !== null;
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

export async function findValidSubscription(planId) {
  if (!window.accountId) return null;
  let subId = await window.contract.find_valid_subscription({
    subscriber_id: window.accountId,
    plan_id: planId
  });
  return subId;
}

