import React from "react";
import { initContract, nftTokensForOwner, newLease } from "./NftContract";
import { BanknotesIcon, CubeIcon } from "@heroicons/react/24/outline";





export default function AccountPage() {
  const [name, setName] = React.useState("");
  const [frequency, setFrequency] = React.useState("Monthly");
  const [duration, setDuration] = React.useState(1);
  const [rate, setRate] = React.useState(0);

  // let onSubmit = async () => {
  //   let contract = await initContract(selectedContract.id);
  //   let expiration =
  //     Math.trunc(Date.now() / 1000) +
  //     durationDay * 24 * 3600 +
  //     durationHour * 3600 +
  //     durationMinute * 60;
  //   newLease(contract, selectedToken.id, borrower, expiration, rent);
  // };

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
                <p className="text-2xl font-semibold text-gray-900">122</p>
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
                <p className="text-2xl font-semibold text-gray-900">2</p>
              </dd>
            </div>
          </dl>
        </div>
        <div className="px-8 py-16 flex flex-row space-x-8 justify-end">
          <div
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-6 py-4 text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
          >
            Deposit
          </div>
          <div
            className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-6 py-4 text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
          >
            Withdraw
          </div>
        </div>

      </div>
    </>
  );
}
