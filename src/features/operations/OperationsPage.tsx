import { useEffect, useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";
import { api } from "../../lib/api";
import { formatDate, formatMoney } from "../../lib/format";
import type { Account, Category, Transaction, TransactionType } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";
import OperationEditDialog from "./OperationEditDialog";

export default function OperationsPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [params, setParams] = useSearchParams();
  const [list, setList] = useState<Transaction[]>([]);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [editingId, setEditingId] = useState<string | null | undefined>(undefined);
  const [defaultType, setDefaultType] = useState<TransactionType>("EXPENSE");

  async function load() {
    const [txs, acs, cats] = await Promise.all([
      api.operations.list(householdId, 500, 0),
      api.accounts.list(householdId),
      api.categories.list(householdId),
    ]);
    setList(txs);
    setAccounts(acs);
    setCategories(cats);
  }
  useEffect(() => {
    load();
  }, [householdId]);

  useEffect(() => {
    const newType = params.get("new");
    if (newType === "INCOME" || newType === "EXPENSE" || newType === "TRANSFER") {
      setDefaultType(newType);
      setEditingId(null);
      params.delete("new");
      setParams(params, { replace: true });
    }
  }, [params, setParams]);

  const accountsById = useMemo(() => new Map(accounts.map((a) => [a.id, a])), [accounts]);
  const categoriesById = useMemo(() => new Map(categories.map((c) => [c.id, c])), [categories]);

  const groups = useMemo(() => {
    const map = new Map<string, { date: string; items: Transaction[]; total: number }>();
    for (const t of list) {
      const d = formatDate(t.occurredAt);
      const sign = t.type === "INCOME" ? 1 : t.type === "EXPENSE" ? -1 : 0;
      const g = map.get(d) ?? { date: d, items: [], total: 0 };
      g.items.push(t);
      g.total += sign * t.amountMinor;
      map.set(d, g);
    }
    return Array.from(map.values());
  }, [list]);

  return (
    <div>
      <PageHeader
        title="Операции"
        subtitle={`Всего: ${list.length}`}
        actions={
          <>
            <Button variant="secondary" onClick={() => { setDefaultType("INCOME"); setEditingId(null); }}>+ Доход</Button>
            <Button variant="secondary" onClick={() => { setDefaultType("TRANSFER"); setEditingId(null); }}>+ Перевод</Button>
            <Button onClick={() => { setDefaultType("EXPENSE"); setEditingId(null); }}>+ Расход</Button>
          </>
        }
      />
      <div className="p-8">
        {groups.length === 0 ? (
          <div className="text-slate-500">Операций пока нет.</div>
        ) : (
          <div className="space-y-4">
            {groups.map((g) => (
              <div key={g.date}>
                <div className="flex items-center justify-between px-2 mb-2">
                  <div className="text-sm font-semibold text-slate-600">{g.date}</div>
                  <div className={`text-sm font-semibold ${g.total > 0 ? "text-income" : g.total < 0 ? "text-expense" : "text-slate-400"}`}>
                    {g.total > 0 ? "+" : ""}{formatMoney(g.total)}
                  </div>
                </div>
                <div className="bg-white rounded-lg shadow-sm divide-y divide-slate-100">
                  {g.items.map((t) => {
                    const acc = accountsById.get(t.accountId);
                    const cat = t.categoryId ? categoriesById.get(t.categoryId) : undefined;
                    const sign = t.type === "INCOME" ? "+" : t.type === "EXPENSE" ? "-" : "";
                    const cls = t.type === "INCOME" ? "text-income" : t.type === "EXPENSE" ? "text-expense" : "text-transfer";
                    return (
                      <div
                        key={t.id}
                        className="px-5 py-3 flex items-center gap-4 cursor-pointer hover:bg-slate-50"
                        onClick={() => setEditingId(t.id)}
                      >
                        <div className="flex-1 min-w-0">
                          <div className="font-medium text-slate-900">
                            {cat?.name ?? (t.type === "TRANSFER" ? "Перевод" : "Без категории")}
                          </div>
                          <div className="text-xs text-slate-500">
                            {acc?.name ?? "—"}
                            {t.type === "TRANSFER" && t.toAccountId
                              ? ` → ${accountsById.get(t.toAccountId)?.name ?? "—"}`
                              : ""}
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
              </div>
            ))}
          </div>
        )}
      </div>

      {editingId !== undefined && (
        <OperationEditDialog
          householdId={householdId}
          operationId={editingId}
          defaultType={defaultType}
          accounts={accounts}
          categories={categories}
          onClose={() => setEditingId(undefined)}
          onSaved={() => {
            setEditingId(undefined);
            load();
          }}
        />
      )}
    </div>
  );
}
