import { useEffect, useState } from "react";
import { api } from "../../lib/api";
import { colorToCss, formatMoney } from "../../lib/format";
import type { Account } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";
import AccountEditDialog from "./AccountEditDialog";

export default function AccountsPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [editing, setEditing] = useState<Account | null | undefined>(undefined);

  async function load() {
    setAccounts(await api.accounts.list(householdId));
  }
  useEffect(() => {
    load();
  }, [householdId]);

  return (
    <div>
      <PageHeader
        title="Счета"
        subtitle="Управление кошельками и картами"
        actions={<Button onClick={() => setEditing(null)}>+ Добавить счёт</Button>}
      />
      <div className="p-8">
        {accounts.length === 0 ? (
          <div className="text-slate-500">Счета ещё не созданы. Добавьте первый счёт.</div>
        ) : (
          <div className="bg-white rounded-lg shadow-sm overflow-hidden">
            <table className="w-full">
              <thead className="bg-slate-50 text-slate-600 text-sm">
                <tr>
                  <th className="text-left px-4 py-3 font-medium">Название</th>
                  <th className="text-left px-4 py-3 font-medium">Тип</th>
                  <th className="text-right px-4 py-3 font-medium">Баланс</th>
                  <th className="px-4 py-3 w-32"></th>
                </tr>
              </thead>
              <tbody className="divide-y divide-slate-100">
                {accounts.map((a) => (
                  <tr key={a.id} className="hover:bg-slate-50">
                    <td className="px-4 py-3">
                      <div className="flex items-center gap-3">
                        <div
                          className="w-8 h-8 rounded flex items-center justify-center text-white text-sm"
                          style={{ backgroundColor: colorToCss(a.color) }}
                        >
                          💳
                        </div>
                        <span className="font-medium text-slate-900">{a.name}</span>
                      </div>
                    </td>
                    <td className="px-4 py-3 text-slate-600">{typeLabel(a.type)}</td>
                    <td className={`px-4 py-3 text-right font-semibold ${a.balanceMinor < 0 ? "text-expense" : "text-slate-900"}`}>
                      {formatMoney(a.balanceMinor, a.currency)}
                    </td>
                    <td className="px-4 py-3 text-right">
                      <button
                        className="text-slate-500 hover:text-slate-900 text-sm mr-3"
                        onClick={() => setEditing(a)}
                      >
                        Изменить
                      </button>
                      <button
                        className="text-slate-400 hover:text-red-600 text-sm"
                        onClick={async () => {
                          if (confirm(`Архивировать счёт "${a.name}"?`)) {
                            await api.accounts.archive(a.id, true);
                            load();
                          }
                        }}
                      >
                        Архив
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      {editing !== undefined && (
        <AccountEditDialog
          householdId={householdId}
          account={editing}
          onClose={() => setEditing(undefined)}
          onSaved={() => {
            setEditing(undefined);
            load();
          }}
        />
      )}
    </div>
  );
}

function typeLabel(t: string): string {
  switch (t) {
    case "CARD_DEBIT": return "Дебетовая карта";
    case "CARD_CREDIT": return "Кредитная карта";
    case "CASH": return "Наличные";
    default: return t;
  }
}
