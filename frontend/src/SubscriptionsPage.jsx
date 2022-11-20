import React from "react";
import { getPlan, mySubscriptions, cancelSubscription } from "./near-api";
import { formatDuration, formatNearAmount, subscriptionState } from "./Utils"
import Modal from "./Modal";

export default function SubscriptionsPage() {
  const [subscriptions, setSubscriptions] = React.useState([]);
  const [success, setSuccess] = React.useState(false);

  async function fetchSubscriptions() {
    let subs = await mySubscriptions(window.accountId)
    subs = await Promise.all(subs.map(async ([key, sub]) => [key, { plan: await getPlan(sub.plan_id), ...sub }]))
    console.log(subs)
    setSubscriptions((_) => subs)
  };

  React.useEffect(() => {
    fetchSubscriptions();
  }, [window.accountId]);

  let cancel = (subscriptionId) => {
    cancelSubscription(subscriptionId).then(_ => setSuccess(_ => true))
  };

  return (
    <>
      <div className="px-4 py-4 sm:px-6 lg:px-8">
        <div className="sm:flex sm:items-center">
          <div className="sm:flex-auto">
            <h1 className="text-xl font-semibold text-gray-900">My Subscriptions </h1>
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
                        Plan Name
                      </th>
                      <th
                        scope="col"
                        className="px-3 py-3.5 text-left text-sm font-semibold text-gray-900"
                      >
                        State
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
                        Provider
                      </th>
                      <th
                        scope="col"
                        className="relative py-3.5 pl-3 pr-4 sm:pr-6"
                      ></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-gray-200 bg-white">
                    {subscriptions.map(([key, subscription]) => (
                      <tr key={key}>
                        <td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                          {subscription.plan.plan_name}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {subscriptionState(subscription)}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {formatNearAmount(subscription.plan.payment_cycle_rate)}Ⓝ / {formatDuration(subscription.plan.payment_cycle_length)}
                        </td>
                        <td className="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                          {subscription.plan.provider_id}
                        </td>
                        <td className="space-x-2 relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                          <a
                            href={"/subscriptions/" + key}
                            className="text-indigo-600 hover:text-indigo-900"
                          >
                            Details
                          </a>
                          <button
                            className={subscriptionState(subscription) === "Active" ? "text-indigo-600 hover:text-indigo-900" : "text-gray-500"}
                            disabled={subscriptionState(subscription) !== "Active"}
                            onClick={(_) => cancel(key)}
                          >
                            Stop
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
      </div >
      <Modal
        open={success}
        setOpen={setSuccess}
        title="Subscription has been canceled"
        description=""
        buttonText="OK"
        buttonUrl="/subscriptions" />
    </>
  );
}
