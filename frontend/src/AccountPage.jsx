import React from "react";
import { deposit, withdraw, myBalance, mySubscriptions } from "./near-api";
import { BanknotesIcon, CubeIcon } from "@heroicons/react/24/outline";
import ModalInput from "./ModalInput";
import { formatDuration, formatNearAmount, subscriptionState } from "./Utils"

export default function AccountPage() {
  const [balance, setBalance] = React.useState(0);
  const [subscriptionCount, setSubscriptionCount] = React.useState(null);
  const [confirmDeposit, setConfirmDeposit] = React.useState(false)
  const [confirmWithdraw, setConfirmWithdraw] = React.useState(false)

  async function fetchBalance() {
    myBalance().then((v) =>
      setBalance((_) => v)
    );
  }

  React.useEffect(() => {
    fetchBalance();
    setInterval(fetchBalance, 60 * 1000)
  }, [window.accountId]);

  React.useEffect(() => {
    async function fetch() {
      let subs = await mySubscriptions()
      console.log(subs)
      subs = subs.filter(([_, sub]) => subscriptionState(sub) === "Active");

      setSubscriptionCount((_) => subs.length)
    }
    fetch();
  }, [window.accountId]);

  let onDeposit = (amount) => {
    if (amount === 0) {
      setConfirmDeposit(false);
      return;
    }
    deposit(amount);
  };

  let onWithdraw = async (amount) => {
    if (amount === 0) {
      setConfirmWithdraw(false);
      return;
    }
    await withdraw(amount);
    await fetchBalance();
    setConfirmWithdraw(false);
  };

  return (
    <>
      <div className="py-6 max-w-6xl">
        <div className="mx-auto px-4 sm:px-6 md:px-8">
          <h1 className="text-2xl mb-8 font-semibold text-gray-900">
            Account
          </h1>
        </div>
        <div className="mx-auto px-4 sm:px-6 md:px-8">

          <dl className="mt-5 grid grid-cols-1 gap-5 lg:grid-cols-2">
            <div
              className="relative overflow-hidden rounded-lg bg-white px-4 pt-5 pb-12 shadow sm:px-6 sm:pt-6"
            >
              <dt>
                <div className="absolute rounded-md bg-indigo-500 p-3">
                  <BanknotesIcon className="h-6 w-6 text-white" aria-hidden="true" />
                </div>
                <p className="ml-16 truncate text-sm font-medium text-gray-500">Balance</p>
              </dt>
              <dd className="ml-16 flex items-baseline pb-6 sm:pb-7">
                <p className="text-2xl font-semibold text-gray-900">{
                  formatNearAmount(balance)}Ⓝ</p>
              </dd>
            </div>
            <div
              className="relative overflow-hidden rounded-lg bg-white px-4 pt-5 pb-12 shadow sm:px-6 sm:pt-6"
            >
              <dt>
                <div className="absolute rounded-md bg-indigo-500 p-3">
                  <CubeIcon className="h-6 w-6 text-white" aria-hidden="true" />
                </div>
                <p className="ml-16 truncate text-sm font-medium text-gray-500">Active Subscriptions</p>
              </dt>
              <dd className="ml-16 flex items-baseline pb-6 sm:pb-7">
                <p className="text-2xl font-semibold text-gray-900">{subscriptionCount}</p>
              </dd>
            </div>
          </dl>
        </div>
        <div className="px-8 py-16 flex flex-row space-x-8 justify-end">
          <div
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-6 py-4 text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto cursor-pointer"
            onClick={_ => setConfirmDeposit(true)}
          >
            Deposit
          </div>
          <div
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-6 py-4 text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto cursor-pointer"
            onClick={_ => setConfirmWithdraw(true)}
          >
            Withdraw
          </div>
        </div>
      </div>
      <ModalInput
        open={confirmDeposit}
        title="Deposit amount"
        description="Please input the amount in NEAR"
        buttonText="Deposit"
        onClose={v => onDeposit(v)}
      />
      <ModalInput
        open={confirmWithdraw}
        title="Withdraw amount"
        description="Please input the amount in NEAR"
        buttonText="Withdraw"
        onClose={v => onWithdraw(v)}
      />
    </>
  );
}
