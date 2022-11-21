import React from "react";
import { useNavigate } from "react-router-dom";
import { CheckIcon } from '@heroicons/react/24/outline'

const RECUR_HOSTNAME = "https://recur-near.netlify.app"
const PLAN_ID = "2et2iTwBw8Qq7AkokTsQdYnN2rkTrow46v59oDwdKtb"

const tiers = [
  {
    name: 'Basic',
    href: RECUR_HOSTNAME + "/plans/" + PLAN_ID + "/subscribe?redirect_to=" + location.href,
    priceDaily: 1,
    features: [
      'Access to all exclusive research report',
      'Join the community chat group',
    ],
  },
  {
    name: 'Pro',
    href: RECUR_HOSTNAME + "/plans/" + PLAN_ID + "/subscribe?redirect_to=" + location.href,
    priceDaily: 3,
    features: [
      'Access to all exclusive research report',
      'Join the community chat group',
      'Special badge in the community chat',
      'Vote for the new research topics to be covered',
    ],
  },
]

export default function PricingPage() {
  const navigate = useNavigate();
  React.useEffect(() => {
    if (window.isMember === true) {
      navigate("/posts")
    }
  }, [window.isMember]);
  return window.isMember === false ? (
    <div className="bg-gray-900">
      <div className="pt-12 sm:pt-16 lg:pt-24">
        <div className="mx-auto max-w-7xl px-4 text-center sm:px-6 lg:px-8">
          <div className="mx-auto max-w-3xl space-y-2 lg:max-w-none">
            <p className="text-3xl font-bold tracking-tight text-white sm:text-4xl lg:text-5xl">
              Choose your membership plan
            </p>
          </div>
        </div>
      </div>
      <div className="mt-8 bg-gray-50 pb-12 sm:mt-12 sm:pb-16 lg:mt-16 lg:pb-24">
        <div className="relative">
          <div className="absolute inset-0 h-3/4 bg-gray-900" />
          <div className="relative z-10 mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="mx-auto max-w-md space-y-4 lg:grid lg:max-w-5xl lg:grid-cols-2 lg:gap-5 lg:space-y-0">
              {tiers.map((tier) => (
                <div key={tier.name} className="flex flex-col overflow-hidden rounded-lg shadow-lg">
                  <div className="bg-white px-6 py-8 sm:p-10 sm:pb-6">
                    <div>
                      <h3
                        className="inline-flex rounded-full bg-indigo-100 px-4 py-1 text-base font-semibold text-indigo-600"
                        id="tier-standard"
                      >
                        {tier.name}
                      </h3>
                    </div>
                    <div className="mt-4 flex items-baseline text-6xl font-bold tracking-tight">
                      {tier.priceDaily}
                      <span className="ml-1 text-2xl font-medium tracking-normal text-gray-500">Ⓝ/minute</span>
                    </div>
                  </div>
                  <div className="flex flex-1 flex-col justify-between space-y-6 bg-gray-50 px-6 pt-6 pb-8 sm:p-10 sm:pt-6">
                    <ul role="list" className="space-y-4">
                      {tier.features.map((feature) => (
                        <li key={feature} className="flex items-start">
                          <div className="flex-shrink-0">
                            <CheckIcon className="h-6 w-6 text-green-500" aria-hidden="true" />
                          </div>
                          <p className="ml-3 text-base text-gray-700">{feature}</p>
                        </li>
                      ))}
                    </ul>
                    <div className="rounded-md shadow">
                      <a
                        href={tier.href}
                        className="flex items-center justify-center rounded-md border border-transparent bg-gray-800 px-5 py-3 text-base font-medium text-white hover:bg-gray-900"
                        aria-describedby="tier-standard"
                      >
                        Get started
                      </a>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  ) : "loading"
}
