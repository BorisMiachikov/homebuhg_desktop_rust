import { useEffect, useState } from "react";
import { api } from "../../lib/api";
import { colorToCss, formatMoney } from "../../lib/format";
import type { BudgetProgress, Category } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";
import BudgetEditDialog from "./BudgetEditDialog";

export default function BudgetsPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [items, setItems] = useState<BudgetProgress[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [editing, setEditing] = useState<BudgetProgress | null | undefined>(undefined);

  async function load() {
    const [bs, cs] = await Promise.all([
      api.budgets.list(householdId),
      api.categories.list(householdId),
    ]);
    setItems(bs);
    setCategories(cs);
  }
  useEffect(() => {
    load();
  }, [householdId]);

  const catsById = new Map(categories.map((c) => [c.id, c]));

  return (
    <div>
      <PageHeader
        title="Бюджеты"
        subtitle="Лимиты по категориям"
        actions={<Button onClick={() => setEditing(null)}>+ Добавить бюджет</Button>}
      />
      <div className="p-8 space-y-3">
        {items.length === 0 ? (
          <div className="text-slate-500">Бюджеты ещё не созданы.</div>
        ) : (
          items.map((it) => {
            const cat = catsById.get(it.budget.categoryId);
            const pct = it.budget.limitMinor > 0 ? Math.min(100, (it.spentMinor * 100) / it.budget.limitMinor) : 0;
            const over = it.spentMinor > it.budget.limitMinor;
            const warn = pct >= 90 && !over;
            const barColor = over ? "bg-expense" : warn ? "bg-yellow-500" : "bg-income";
            return (
              <div
                key={it.budget.id}
                className="bg-white rounded-lg p-4 shadow-sm cursor-pointer hover:shadow"
                onClick={() => setEditing(it)}
              >
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center gap-3">
                    <div
                      className="w-6 h-6 rounded-full"
                      style={{ backgroundColor: colorToCss(cat?.color ?? 0xFF9E9E9E) }}
                    />
                    <span className="font-semibold text-slate-900">{cat?.name ?? "—"}</span>
                    <span className="text-xs text-slate-500">{periodLabel(it.budget.period)}</span>
                  </div>
                  <div className={`text-sm font-semibold ${over ? "text-expense" : warn ? "text-yellow-700" : "text-slate-700"}`}>
                    {formatMoney(it.spentMinor)} / {formatMoney(it.budget.limitMinor)}
                  </div>
                </div>
                <div className="h-2 bg-slate-100 rounded overflow-hidden">
                  <div
                    className={`h-full ${barColor} transition-all`}
                    style={{ width: `${Math.min(100, pct)}%` }}
                  />
                </div>
                {over && (
                  <div className="text-xs text-expense mt-1">
                    Превышение: {formatMoney(it.spentMinor - it.budget.limitMinor)}
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>

      {editing !== undefined && (
        <BudgetEditDialog
          householdId={householdId}
          progress={editing}
          categories={categories.filter((c) => c.type === "EXPENSE")}
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

function periodLabel(p: string): string {
  switch (p) {
    case "WEEK": return "Неделя";
    case "MONTH": return "Месяц";
    case "YEAR": return "Год";
    default: return p;
  }
}
