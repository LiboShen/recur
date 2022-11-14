import React from "react";
import { initContract, nftTokensForOwner, newLease } from "./NftContract";


const FREQUENCIES = {
  "Daily": { quantifier: "Day(s)" },
  "Weekly": { quantifier: "Week(s)" },
  "Monthly": { quantifier: "Month(s)" },
  "Annually": { quantifier: "Year(s)" },
}



export default function NewLendingPage() {
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
            New Plan
          </h1>
        </div>
        <div className="mx-auto px-4 sm:px-6 md:px-8">
          <div className="flex-auto space-y-6 sm:space-y-5">
            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label
                className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
              >
                Plan Name
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <input
                  type="text"
                  className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                />
                <p className="mt-2 text-sm text-gray-500">
                  The name of the subscription plan you want to create
                </p>
              </div>
            </div>

            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label
                className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2"
              >
                Frequency
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <select
                  className="mt-1 block w-full rounded-md border-gray-300 py-2 pl-3 pr-10 text-base focus:border-indigo-500 focus:outline-none focus:ring-indigo-500 sm:text-sm"
                  value={frequency}
                  onChange={(e) => setFrequency(e.target.value)}
                >
                  {Object.keys(FREQUENCIES).map((k, i) => <option key={i}>{k}</option>)}
                </select>
                <p className="mt-2 text-sm text-gray-500">
                  Choose the token you want to lend
                </p>
              </div>
            </div>

            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2">
                Duration
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <div className="flex flex-row items-center space-x-2">
                  <input
                    type="text"
                    className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    value={duration}
                    onChange={(e) => setDuration(e.target.value)}
                  />
                  <span>{FREQUENCIES[frequency].quantifier}</span>
                </div>
                <p className="mt-2 text-sm text-gray-500">
                  The maximum duration of the subscription
                </p>
              </div>
            </div>

            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label className="block text-sm font-medium text-gray-700 sm:mt-px sm:pt-2">
                Price
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <div className="flex flex-row items-center space-x-2">
                  <input
                    type="number"
                    className="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    value={rate}
                    onChange={(e) => setRate(e.target.value)}
                  />
                  <span>{frequency}</span>
                </div>
                <p className="mt-2 text-sm text-gray-500">
                  The maximum duration of the subscription
                </p>
              </div>
            </div>
          </div>

          <div className="pt-5">
            <div className="flex justify-end">
              <a
                className="rounded-md border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                href="/"
              >
                Cancel
              </a>
              <button
                className="ml-3 inline-flex justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
                onClick={(_) => onSubmit()}
              >
                Submit
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
