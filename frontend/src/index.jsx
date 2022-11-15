import "regenerator-runtime/runtime";
import React from "react";
import { createRoot } from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import Home from "./Home";
import NewPlanPage from "./NewPlanPage";
import PlansPage from "./PlansPage";
import SubscriptionsPage from "./SubscriptionsPage";
import SubscribePage from "./SubscribePage";
import { initContract } from "./near-api";

const reactRoot = createRoot(document.querySelector("#root"));

window.nearInitPromise = initContract()
  .then(() => {
    reactRoot.render(
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<App />}>
            <Route index element={<Home />} />
            <Route path="plans/new" element={<NewPlanPage />} />
            <Route path="plans" element={<PlansPage />} />
            <Route path="subscriptions" element={<SubscriptionsPage />} />
            <Route path="plans/:planId/subscribe" element={<SubscribePage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    );
  })
  .catch((e) => {
    reactRoot.render(
      <div style={{ color: "red" }}>
        Error: <code>{e.message}</code>
      </div>
    );
    console.error(e);
  });

