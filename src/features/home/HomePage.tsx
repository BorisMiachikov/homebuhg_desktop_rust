import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { api } from "../../lib/api";
import { colorToCss, formatDateTime, formatMoney } from "../../lib/format";
import type { Account, Category, Transaction } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";

export default function HomePage() {
  const householdId = useSession((s) => s.householdId)!;
  const navigate = useNavigate();
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [total, setTotal] = useState(0);
  const [recent, setRecent] = useState<Transaction[]>([]);
  const [cats, setCats] = useState<Map<string, Category>>(new Map());

  async function load() {
    const [acs, tot, txs, catList] = await Promise.all([
      api.accounts.list(householdId),
      api.accounts.total(householdId),
      api.operations.list(householdId, 10, 0),
      api.categories.list(householdId),
    ]);
    setAccounts(acs);
    setTotal(tot.totalMinor);
    setRecent(txs);
    setCats(new Map(catList.map((c) => [c.id, c])));
  }

  useEffect(() => {
    load();
  }, [householdId]);

  const accountsById = new Map(accounts.map((a) => [a.id, a]));

  return (
    <div>
      <PageHeader
        title="Главная"
        subtitle="Обзор вашего бюджета"
        actions={
          <>
            <Button variant="secondary" onClick={() => navigate("/operations?new=EXPENSE")}>+ Расход</Button>
            <Button onClick={() => navigate("/operations?new=INCOME")}>+ Доход</Button>
          </>
        }
      />
      <div className="p-8 space-y-6">
        <div className="bg-white rounded-lg p-6 shadow-sm">
          <div className="text-sm text-slate-500">Общий баланс</div>
          <div className={`text-4xl font-bold mt-2 ${total < 0 ? "text-expense" : "text-slate-900"}`}>
            {formatMoney(total)}
          </div>
        </div>

        <section>
          <h2 className="text-lg font-semibold text-slate-800 mb-3">Счета</h2>
          {accounts.length === 0 ? (
            <div className="text-slate-500">Счета ещё не созданы.</div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
              {accounts.map((a) => (
                <div
                  key={a.id}
                  className="bg-white rounded-lg p-4 shadow-sm flex items-center gap-3 cursor-pointer hover:shadow"
                  onClick={() => navigate("/accounts")}
                >
                  <div
                    className="w-10 h-10 rounded flex items-center justify-center text-white"
                    style={{ backgroundColor: colorToCss(a.color) }}
                  >
                    💳
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="font-medium text-slate-900 truncate">{a.name}</div>
                    <div className="text-xs text-slate-500">{accountTypeLabel(a.type)}</div>
                  </div>
                  <div className={`font-semibold ${a.balanceMinor < 0 ? "text-expense" : "text-slate-900"}`}>
                    {formatMoney(a.balanceMinor, a.currency)}
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>

        <section>
          <h2 className="text-lg font-semibold text-slate-800 mb-3">Последние операции</h2>
          {recent.length === 0 ? (
            <div className="text-slate-500">Операций пока нет.</div>
          ) : (
            <div className="bg-white rounded-lg shadow-sm divide-y divide-slate-100">
              {recent.map((t) => {
                const acc = accountsById.get(t.accountId);
                const cat = t.categoryId ? cats.get(t.categoryId) : undefined;
                const sign = t.type === "INCOME" ? "+" : t.type === "EXPENSE" ? "-" : "";
                const cls = t.type === "INCOME" ? "text-income" : t.type === "EXPENSE" ? "text-expense" : "text-transfer";
                return (
                  <div key={t.id} className="px-5 py-3 flex items-center gap-4">
                    <div className="flex-1 min-w-0">
                      <div className="font-medium text-slate-900">
                        {cat?.name ?? (t.type === "TRANSFER" ? "Перевод" : "Без категории")}
                      </div>
                      <div className="text-xs text-slate-500">
                        {formatDateTime(t.occurredAt)} · {acc?.name ?? "—"}
                        {t.note ? ` · ${t.note}` : ""}
                      </div>
                    </div>
                    <div className={`font-semibold ${cls}`}>
                      {sign}{formatMoney(t.amountMinor, t.currency)}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </section>
      </div>
    </div>
  );
}

function accountTypeLabel(t: string): string {
  switch (t) {
    case "CARD_DEBIT": return "Дебетовая карта";
    case "CARD_CREDIT": return "Кредитная карта";
    case "CASH": return "Наличные";
    default: return t;
  }
}
