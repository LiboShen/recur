import React from "react";
import { createPlan } from "./near-api";
import Modal from "./Modal";


const FREQUENCIES = {
  "Every minute": { quantifier: "Minute(s)", seconds: 60 },
  "Daily": { quantifier: "Day(s)", seconds: 24 * 3600 },
  "Weekly": { quantifier: "Week(s)", seconds: 7 * 24 * 3600 },
  "Monthly": { quantifier: "Month(s)", seconds: 30 * 24 * 3600 },
  "Annually": { quantifier: "Year(s)", seconds: 365 * 24 * 3600 },
}



export default function NewLendingPage() {
  const [name, setName] = React.useState("");
  const [frequency, setFrequency] = React.useState("Monthly");
  const [price, setPrice] = React.useState(0);
  const [success, setSuccess] = React.useState(false)

  let onSubmit = async () => {
    let cycleLength = FREQUENCIES[frequency].seconds
    createPlan(name, cycleLength, price).then(_ => setSuccess(_ => true))
  };

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
                  The subscription payment frequency
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
                    value={price}
                    onChange={(e) => setPrice(e.target.value)}
                  />
                  <span>{frequency}</span>
                </div>
                <p className="mt-2 text-sm text-gray-500">
                  How much the subscriber should pay in cycle (in NEAR)
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
      <Modal
        open={success}
        setOpen={setSuccess}
        title="New plan has been created 🎉"
        description="The new plan is ready for recieving recuring payment!"
        buttonText="Go back to Plans page"
        buttonUrl="/plans" />
    </>
  );
}
