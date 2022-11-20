export function classNames(...classes) {
  return classes.filter(Boolean).join(" ");
}

export function formatNearAmount(amount) {
  return Number(window.nearApi.utils.format.formatNearAmount(BigInt(amount))).toFixed(2)
}

export function formatDuration(frequency) {
  if (frequency == 60) {
    return "1 minute"
  }
  if (frequency == 60 * 60) {
    return "1 hour"
  }
  if (frequency == 60 * 60 * 24) {
    return "1 day"
  }
  if (frequency == 60 * 60 * 24 * 7) {
    return " 1week"
  }
  if (frequency == 60 * 60 * 24 * 30) {
    return "30 days"
  }
  return frequency + " seconds";
}

export function subscriptionState(sub) {
  if (sub.state == "Invalid") {
    return "Invalid";
  }
  if ("Active" in sub.state) {
    return "Active";
  }
  if ("Canceled" in sub.state) {
    return "Canceled";
  }
}
