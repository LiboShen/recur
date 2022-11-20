import React from "react";
import { collectFees, getCollectableFeesForPlan, myPlans } from "./near-api";
import { formatDuration, formatNearAmount } from "./Utils"
import Modal from "./Modal";

export default function PlansPage() {
  const [plans, setPlans] = React.useState([]);
  const [success, setSuccess] = React.useState(false);

  async function fetchPlans() {
    let plans = await myPlans(window.accountId)
    plans = await Promise.all(plans.map(async ([key, plan]) => [key, { fees: await getCollectableFeesForPlan(key), ...plan }]))
    setPlans((_) => plans)
  }

  React.useEffect(() => {
    fetchPlans();
  }, [window.accountId]);

  let collect = (planId) => {
    collectFees(planId).then(_ => setSuccess(_ => true))
  };

  return (
    <>
      <div className="px-4 py-4 sm:px-6 lg:px-8">
        <div className="sm:flex sm:items-center">
          <div className="sm:flex-auto">
            <h1 className="text-xl font-semibold text-gray-900">My Subscription Plans</h1>
          </div>
          <div className="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
            <a
              className="inline-flex items-center justify-center rounded-md border border-transparent bg-indigo-600 px-4 py-2 text-sm font-medium text-white shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 sm:w-auto"
              href="/plans/new"
            >
              Create New Plan
            </a>
          </div>
        </div>
        <div className="mt-8 flex flex-col">
          <div className="-my-2 -mx-4 overflow-x-auto sm:-mx-6 lg:-mx-8">
            <div className="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8">
              <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
                <table className="min-w-full divide-y divide-gray-300">
                  <thead className="bg-gray-50">
                    <tr>
                      <th
                        scope="col"
                        className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6"
                      >
                        Name
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        Plan ID
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        Rate
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        Accrued fees
                      </th>
                      <th
                        scope="col"
                        className="relative py-3.5 pl-3 pr-4 sm:pr-6"
                      ></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200 bg-white">
                    {plans.map(([key, plan]) => (
                      <tr key={key}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                          {plan.plan_name}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {key}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {formatNearAmount(plan.payment_cycle_rate)}Ⓝ / {formatDuration(plan.payment_cycle_length)}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {formatNearAmount(plan.fees)}Ⓝ
                        </td>
                        <td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <button
                            className="text-indigo-600 hover:text-indigo-900"
                            onClick={(_) => collect(key)}
                          >
                            Collect fees
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>
      </div>
      <Modal
        open={success}
        setOpen={setSuccess}
        title="Subscription fee collected"
        description="The accrued fee has been transfered to your wallet"
        buttonText="OK"
        buttonUrl="/plans" />
    </>
  );
}
