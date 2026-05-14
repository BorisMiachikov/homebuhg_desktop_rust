import { useEffect } from "react";
import { Route, Routes } from "react-router-dom";
import Layout from "./components/Layout";
import { useSession } from "./store/session";
import HomePage from "./features/home/HomePage";
import OperationsPage from "./features/operations/OperationsPage";
import AccountsPage from "./features/accounts/AccountsPage";
import CategoriesPage from "./features/categories/CategoriesPage";
import BudgetsPage from "./features/budgets/BudgetsPage";
import RecurringPage from "./features/recurring/RecurringPage";
import ReportsPage from "./features/reports/ReportsPage";
import SettingsPage from "./features/settings/SettingsPage";

function App() {
  const { ready, bootstrap } = useSession();

  useEffect(() => {
    bootstrap().catch((e) => console.error("bootstrap failed", e));
  }, [bootstrap]);

  if (!ready) {
    return (
      <div className="h-screen w-screen flex items-center justify-center text-slate-500">
        Загрузка…
      </div>
    );
  }

  return (
    <Routes>
      <Route element={<Layout />}>
        <Route index element={<HomePage />} />
        <Route path="operations" element={<OperationsPage />} />
        <Route path="accounts" element={<AccountsPage />} />
        <Route path="categories" element={<CategoriesPage />} />
        <Route path="budgets" element={<BudgetsPage />} />
        <Route path="recurring" element={<RecurringPage />} />
        <Route path="reports" element={<ReportsPage />} />
        <Route path="settings" element={<SettingsPage />} />
      </Route>
    </Routes>
  );
}

export default App;
