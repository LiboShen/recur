import React from "react";
import { initContract, nftTokensForOwner, newLease } from "./NftContract";
import { useParams } from "react-router-dom";



const TEST_PLAN = {
  name: "Web3 Book Club Membership",
  frequency: 60,
  rate: "100000000000000000000",
  count: 100
}

function formatDuration(frequency) {
  if (frequency == 60) {
    return "1 minute"
  }
  if (frequency == 60 * 60) {
    return "1 hour"
  }
  if (frequency == 60 * 60 * 24) {
    return "1 day"
  }
  if (frequency == 60 * 60 * 7) {
    return " 1week"
  }
  if (frequency == 60 * 60 * 30) {
    return "30 days"
  }
  return frequency + " seconds";
}



export default function ReviewSubscriptionPage() {
  let { planId } = useParams();

  const [plan, setPlan] = React.useState(null);

  React.useEffect(() => {
    async function fetchPlan() {
      setPlan((_) => TEST_PLAN)
    }
    fetchPlan();
  });

  return plan ? (
    <>
      <div className="py-6 max-w-6xl">
        <div className="mx-auto px-4 sm:px-6 md:px-8">
          <h1 className="text-2xl mb-8 font-semibold text-gray-900">
            Subscribe
          </h1>
        </div>
        <div className="mx-auto px-4 sm:px-6 md:px-8">
          <div className="flex-auto space-y-6 sm:space-y-5">
            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label
                className="block text-sm font-medium text-gray-700"
              >
                Plan Name
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <div
                  className="block w-full "
                >{plan.name}</div>
              </div>
            </div>

            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label
                className="block text-sm font-medium text-gray-700"
              >
                Rate
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <div
                  className="block w-full "
                >{window.nearApi.utils.format.formatNearAmount(
                  BigInt(plan.rate).toString()
                )} every {formatDuration(plan.frequency)}</div>
              </div>
            </div>

            <div className="sm:grid sm:grid-cols-3 sm:items-start sm:gap-4 sm:border-t sm:border-gray-200 sm:pt-5">
              <label
                className="block text-sm font-medium text-gray-700"
              >
                Duration
              </label>
              <div className="mt-1 sm:col-span-2 sm:mt-0">
                <div
                  className="block w-full "
                >{formatDuration(plan.frequency * plan.count)}</div>
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
                Subscribe
              </button>
            </div>
          </div>
        </div>
      </div>
    </>) : "Loading";
}
