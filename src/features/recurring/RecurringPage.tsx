import { useEffect, useState } from "react";
import { api } from "../../lib/api";
import { formatDateTime, formatMoney } from "../../lib/format";
import type { Account, Category, RecurringRule } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";
import RecurringEditDialog from "./RecurringEditDialog";

export default function RecurringPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [rules, setRules] = useState<RecurringRule[]>([]);
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [categories, setCategories] = useState<Category[]>([]);
  const [editing, setEditing] = useState<RecurringRule | null | undefined>(undefined);

  async function load() {
    const [rs, as, cs] = await Promise.all([
      api.recurring.list(householdId),
      api.accounts.list(householdId),
      api.categories.list(householdId),
    ]);
    setRules(rs);
    setAccounts(as);
    setCategories(cs);
  }
  useEffect(() => {
    load();
  }, [householdId]);

  return (
    <div>
      <PageHeader
        title="Регулярные операции"
        subtitle="Автоматические повторяющиеся транзакции"
        actions={<Button onClick={() => setEditing(null)}>+ Добавить правило</Button>}
      />
      <div className="p-8">
        {rules.length === 0 ? (
          <div className="text-slate-500">Регулярных правил ещё нет.</div>
        ) : (
          <div className="bg-white rounded-lg shadow-sm divide-y divide-slate-100">
            {rules.map((r) => {
              const tmpl = parseTemplate(r.templateJson);
              const acc = accounts.find((a) => a.id === tmpl?.accountId);
              const cat = categories.find((c) => c.id === tmpl?.categoryId);
              return (
                <div
                  key={r.id}
                  className="px-5 py-4 flex items-center gap-4 cursor-pointer hover:bg-slate-50"
                  onClick={() => setEditing(r)}
                >
                  <div className="flex-1 min-w-0">
                    <div className="font-medium text-slate-900">
                      {cat?.name ?? "Без категории"} · {acc?.name ?? "—"}
                    </div>
                    <div className="text-xs text-slate-500 mt-0.5">
                      RRULE: {r.rrule} · след. запуск: {formatDateTime(r.nextRunAt)}
                      {!r.isActive && <span className="ml-2 text-amber-600">⏸ неактивно</span>}
                    </div>
                  </div>
                  <div className="font-semibold text-slate-900">
                    {tmpl ? formatMoney(tmpl.amountMinor, tmpl.currency || "RUB") : "—"}
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {editing !== undefined && (
        <RecurringEditDialog
          householdId={householdId}
          rule={editing}
          accounts={accounts}
          categories={categories}
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

function parseTemplate(json: string): any | null {
  try {
    return JSON.parse(json);
  } catch {
    return null;
  }
}
