import { useEffect, useMemo, useState } from "react";
import {
  Bar,
  BarChart,
  CartesianGrid,
  Legend,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { api } from "../../lib/api";
import { colorToCss, formatMoney } from "../../lib/format";
import type { CategorySpend, MonthlyPoint, ReportRange, ReportSummary } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";

type PeriodKey = "M1" | "M3" | "Y1" | "ALL";

export default function ReportsPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [periodKey, setPeriodKey] = useState<PeriodKey>("M3");
  const [tab, setTab] = useState<"dynamics" | "expenses">("dynamics");
  const [summary, setSummary] = useState<ReportSummary | null>(null);
  const [monthly, setMonthly] = useState<MonthlyPoint[]>([]);
  const [top, setTop] = useState<CategorySpend[]>([]);

  const range = useMemo<ReportRange>(() => {
    const now = Date.now();
    let start: number;
    switch (periodKey) {
      case "M1": start = now - 31 * 24 * 3600 * 1000; break;
      case "M3": start = now - 93 * 24 * 3600 * 1000; break;
      case "Y1": start = now - 366 * 24 * 3600 * 1000; break;
      case "ALL": start = 0; break;
    }
    return { householdId, startMs: start, endMs: now };
  }, [householdId, periodKey]);

  useEffect(() => {
    Promise.all([
      api.reports.summary(range),
      api.reports.monthly(range),
      api.reports.topCategories(range, 10),
    ]).then(([s, m, t]) => {
      setSummary(s);
      setMonthly(m);
      setTop(t);
    });
  }, [range]);

  const chartData = monthly.map((p) => ({
    bucket: p.bucket,
    "Доход": p.incomeMinor / 100,
    "Расход": p.expenseMinor / 100,
  }));

  const maxSpent = top.reduce((m, c) => Math.max(m, c.spentMinor), 0);

  return (
    <div>
      <PageHeader
        title="Отчёты"
        subtitle="Аналитика доходов и расходов"
        actions={
          <div className="flex gap-1 bg-slate-100 rounded p-1">
            <PeriodBtn active={periodKey === "M1"} onClick={() => setPeriodKey("M1")}>Месяц</PeriodBtn>
            <PeriodBtn active={periodKey === "M3"} onClick={() => setPeriodKey("M3")}>3 мес</PeriodBtn>
            <PeriodBtn active={periodKey === "Y1"} onClick={() => setPeriodKey("Y1")}>Год</PeriodBtn>
            <PeriodBtn active={periodKey === "ALL"} onClick={() => setPeriodKey("ALL")}>Всё</PeriodBtn>
          </div>
        }
      />
      <div className="p-8 space-y-6">
        <div className="grid grid-cols-3 gap-4">
          <SummaryCard label="Доходы" value={summary?.totalIncomeMinor ?? 0} cls="text-income" />
          <SummaryCard label="Расходы" value={summary?.totalExpenseMinor ?? 0} cls="text-expense" />
          <SummaryCard label="Баланс" value={summary?.balanceMinor ?? 0} cls={(summary?.balanceMinor ?? 0) >= 0 ? "text-income" : "text-expense"} />
        </div>

        <div className="flex gap-2">
          <TabBtn active={tab === "dynamics"} onClick={() => setTab("dynamics")}>Динамика</TabBtn>
          <TabBtn active={tab === "expenses"} onClick={() => setTab("expenses")}>Расходы по категориям</TabBtn>
        </div>

        {tab === "dynamics" && (
          <div className="bg-white rounded-lg p-4 shadow-sm" style={{ height: 360 }}>
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={chartData}>
                <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
                <XAxis dataKey="bucket" stroke="#64748b" />
                <YAxis stroke="#64748b" tickFormatter={(v) => formatShortRub(v)} />
                <Tooltip formatter={(v: any) => formatMoney(Math.round((v as number) * 100))} />
                <Legend />
                <Bar dataKey="Доход" fill="#16a34a" />
                <Bar dataKey="Расход" fill="#dc2626" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        )}

        {tab === "expenses" && (
          <div className="bg-white rounded-lg p-4 shadow-sm space-y-3">
            {top.length === 0 ? (
              <div className="text-slate-500">Нет данных за выбранный период.</div>
            ) : (
              top.map((c) => {
                const pct = maxSpent > 0 ? (c.spentMinor * 100) / maxSpent : 0;
                return (
                  <div key={c.categoryId}>
                    <div className="flex items-center justify-between text-sm mb-1">
                      <div className="flex items-center gap-2">
                        <div
                          className="w-4 h-4 rounded-full"
                          style={{ backgroundColor: colorToCss(c.color) }}
                        />
                        <span className="font-medium text-slate-900">{c.categoryName}</span>
                      </div>
                      <span className="text-slate-700 font-semibold">{formatMoney(c.spentMinor)}</span>
                    </div>
                    <div className="h-2 bg-slate-100 rounded overflow-hidden">
                      <div
                        className="h-full"
                        style={{ width: `${pct}%`, backgroundColor: colorToCss(c.color) }}
                      />
                    </div>
                  </div>
                );
              })
            )}
          </div>
        )}
      </div>
    </div>
  );
}

function SummaryCard({ label, value, cls }: { label: string; value: number; cls: string }) {
  return (
    <div className="bg-white rounded-lg p-5 shadow-sm">
      <div className="text-sm text-slate-500">{label}</div>
      <div className={`text-2xl font-bold mt-1 ${cls}`}>{formatMoney(value)}</div>
    </div>
  );
}

function PeriodBtn({ active, onClick, children }: { active: boolean; onClick: () => void; children: React.ReactNode }) {
  return (
    <button
      onClick={onClick}
      className={`px-3 py-1.5 rounded text-xs font-medium ${active ? "bg-white text-slate-900 shadow-sm" : "text-slate-600 hover:text-slate-900"}`}
    >
      {children}
    </button>
  );
}

function TabBtn({ active, onClick, children }: { active: boolean; onClick: () => void; children: React.ReactNode }) {
  return (
    <button
      onClick={onClick}
      className={`px-4 py-2 rounded text-sm font-medium ${active ? "bg-slate-900 text-white" : "bg-white text-slate-700 border border-slate-300"}`}
    >
      {children}
    </button>
  );
}

function formatShortRub(rub: number): string {
  const abs = Math.abs(rub);
  if (abs >= 1_000_000) return `${(rub / 1_000_000).toFixed(1)}M`;
  if (abs >= 1_000) return `${(rub / 1_000).toFixed(0)}k`;
  return rub.toFixed(0);
}
