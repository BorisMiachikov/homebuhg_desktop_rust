import { useState } from "react";
import { api } from "../../lib/api";
import { fromIsoDate, parseAmountToMinor, toIsoDate, todayStartMs } from "../../lib/format";
import type { Account, Category, RecurringRule, TransactionType } from "../../lib/types";
import Modal from "../../components/Modal";
import Button from "../../components/Button";

interface Props {
  householdId: string;
  rule: RecurringRule | null;
  accounts: Account[];
  categories: Category[];
  onClose: () => void;
  onSaved: () => void;
}

type Freq = "DAILY" | "WEEKLY" | "MONTHLY" | "YEARLY";

export default function RecurringEditDialog({
  householdId,
  rule,
  accounts,
  categories,
  onClose,
  onSaved,
}: Props) {
  const tmpl = rule ? safeParse(rule.templateJson) : null;
  const initialRrule = parseRrule(rule?.rrule);

  const [type, setType] = useState<TransactionType>(tmpl?.type ?? "EXPENSE");
  const [accountId, setAccountId] = useState(tmpl?.accountId ?? accounts[0]?.id ?? "");
  const [categoryId, setCategoryId] = useState(tmpl?.categoryId ?? "");
  const [amount, setAmount] = useState(
    tmpl ? (tmpl.amountMinor / 100).toFixed(2).replace(".", ",") : "0,00"
  );
  const [note, setNote] = useState(tmpl?.note ?? "");
  const [freq, setFreq] = useState<Freq>(initialRrule.freq);
  const [interval, setInterval] = useState<number>(initialRrule.interval);
  const [nextRunAt, setNextRunAt] = useState(rule?.nextRunAt ?? todayStartMs());
  const [isActive, setIsActive] = useState(rule?.isActive ?? true);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);

  const filteredCats = categories.filter((c) => c.type === (type === "INCOME" ? "INCOME" : "EXPENSE"));

  async function save() {
    const minor = parseAmountToMinor(amount);
    if (minor === null || minor <= 0) return;
    if (!accountId) return;
    const template = {
      type,
      amountMinor: minor,
      currency: "RUB",
      accountId,
      toAccountId: null,
      categoryId: type === "TRANSFER" ? null : (categoryId || null),
      note: note.trim() || null,
    };
    const rrule = `FREQ=${freq};INTERVAL=${interval}`;
    setSaving(true);
    try {
      await api.recurring.upsert({
        id: rule?.id,
        householdId,
        templateJson: JSON.stringify(template),
        rrule,
        nextRunAt,
        isActive,
      });
      onSaved();
    } finally {
      setSaving(false);
    }
  }

  async function remove() {
    if (!rule) return;
    if (!confirm("Удалить правило?")) return;
    setDeleting(true);
    try {
      await api.recurring.delete(rule.id);
      onSaved();
    } finally {
      setDeleting(false);
    }
  }

  return (
    <Modal
      open
      title={rule ? "Редактировать правило" : "Новое правило"}
      onClose={onClose}
      width="max-w-2xl"
      footer={
        <div className="flex w-full justify-between">
          <div>
            {rule && <Button variant="danger" onClick={remove} disabled={deleting}>{deleting ? "..." : "Удалить"}</Button>}
          </div>
          <div className="flex gap-2">
            <Button variant="secondary" onClick={onClose}>Отмена</Button>
            <Button onClick={save} disabled={saving}>{saving ? "..." : "Сохранить"}</Button>
          </div>
        </div>
      }
    >
      <div className="space-y-3">
        <div className="grid grid-cols-3 gap-3">
          <div>
            <label className="label">Тип</label>
            <select className="select" value={type} onChange={(e) => setType(e.target.value as TransactionType)}>
              <option value="EXPENSE">Расход</option>
              <option value="INCOME">Доход</option>
            </select>
          </div>
          <div>
            <label className="label">Сумма</label>
            <input className="input" value={amount} onChange={(e) => setAmount(e.target.value)} inputMode="decimal" />
          </div>
          <div>
            <label className="label">Счёт</label>
            <select className="select" value={accountId} onChange={(e) => setAccountId(e.target.value)}>
              <option value="">— выберите —</option>
              {accounts.map((a) => (
                <option key={a.id} value={a.id}>{a.name}</option>
              ))}
            </select>
          </div>
        </div>
        <div>
          <label className="label">Категория</label>
          <select className="select" value={categoryId} onChange={(e) => setCategoryId(e.target.value)}>
            <option value="">— без категории —</option>
            {filteredCats.map((c) => (
              <option key={c.id} value={c.id}>{c.name}</option>
            ))}
          </select>
        </div>
        <div>
          <label className="label">Заметка</label>
          <input className="input" value={note} onChange={(e) => setNote(e.target.value)} />
        </div>
        <div className="grid grid-cols-3 gap-3 pt-2 border-t border-slate-200">
          <div>
            <label className="label">Частота</label>
            <select className="select" value={freq} onChange={(e) => setFreq(e.target.value as Freq)}>
              <option value="DAILY">Ежедневно</option>
              <option value="WEEKLY">Еженедельно</option>
              <option value="MONTHLY">Ежемесячно</option>
              <option value="YEARLY">Ежегодно</option>
            </select>
          </div>
          <div>
            <label className="label">Интервал</label>
            <input
              className="input"
              type="number"
              min={1}
              value={interval}
              onChange={(e) => setInterval(Math.max(1, parseInt(e.target.value || "1", 10)))}
            />
          </div>
          <div>
            <label className="label">Следующий запуск</label>
            <input
              type="date"
              className="input"
              value={toIsoDate(nextRunAt)}
              onChange={(e) => setNextRunAt(fromIsoDate(e.target.value))}
            />
          </div>
        </div>
        <label className="flex items-center gap-2 text-sm">
          <input type="checkbox" checked={isActive} onChange={(e) => setIsActive(e.target.checked)} />
          Правило активно
        </label>
      </div>
    </Modal>
  );
}

function safeParse(s: string): any | null {
  try { return JSON.parse(s); } catch { return null; }
}

function parseRrule(s?: string): { freq: Freq; interval: number } {
  const def = { freq: "MONTHLY" as Freq, interval: 1 };
  if (!s) return def;
  const parts = Object.fromEntries(s.split(";").map((p) => p.split("=")));
  const freq = (parts.FREQ as Freq) || "MONTHLY";
  const interval = Math.max(1, parseInt(parts.INTERVAL ?? "1", 10) || 1);
  return { freq, interval };
}
