import { NavLink, Outlet } from "react-router-dom";

const items: { to: string; label: string; icon: string }[] = [
  { to: "/", label: "Главная", icon: "🏠" },
  { to: "/operations", label: "Операции", icon: "📋" },
  { to: "/accounts", label: "Счета", icon: "💳" },
  { to: "/categories", label: "Категории", icon: "🏷️" },
  { to: "/budgets", label: "Бюджеты", icon: "📊" },
  { to: "/recurring", label: "Регулярные", icon: "🔁" },
  { to: "/reports", label: "Отчёты", icon: "📈" },
  { to: "/settings", label: "Настройки", icon: "⚙️" },
];

export default function Layout() {
  return (
    <div className="flex h-screen w-screen bg-slate-50">
      <aside className="w-56 shrink-0 bg-slate-900 text-slate-100 flex flex-col">
        <div className="px-5 py-5 text-xl font-bold tracking-tight">HomeBuhg</div>
        <nav className="flex-1 px-2">
          {items.map((it) => (
            <NavLink
              key={it.to}
              to={it.to}
              end={it.to === "/"}
              className={({ isActive }) =>
                `flex items-center gap-3 px-4 py-2.5 rounded text-sm transition ${
                  isActive
                    ? "bg-slate-700 text-white"
                    : "text-slate-300 hover:bg-slate-800 hover:text-white"
                }`
              }
            >
              <span className="w-5 text-center">{it.icon}</span>
              <span>{it.label}</span>
            </NavLink>
          ))}
        </nav>
        <div className="px-5 py-3 text-xs text-slate-500 border-t border-slate-800">
          v1.0.0 · Desktop
        </div>
      </aside>
      <main className="flex-1 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}
