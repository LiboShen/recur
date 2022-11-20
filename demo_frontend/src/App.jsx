import { Outlet } from "react-router-dom";
import UserWidget from "./UserWidget";
import {
  Bars3BottomLeftIcon,
  BellIcon,
  BuildingStorefrontIcon,
  ShoppingBagIcon,
  ChartBarIcon,
  HomeIcon,
  SparklesIcon,
  XMarkIcon,
} from "@heroicons/react/24/outline";

export default function App() {
  return (
    <main>
      <UserWidget />
      <Outlet />
    </main>
  );
}
