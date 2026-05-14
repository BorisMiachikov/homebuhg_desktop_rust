import { useState } from "react";
import { api } from "../../lib/api";
import { parseAmountToMinor, toIsoDate, fromIsoDate, todayStartMs } from "../../lib/format";
import type { BudgetPeriod, BudgetProgress, Category } from "../../lib/types";
import Modal from "../../components/Modal";
import Button from "../../components/Button";

interface Props {
  householdId: string;
  progress: BudgetProgress | null;
  categories: Category[];
  onClose: () => void;
  onSaved: () => void;
}

export default function BudgetEditDialog({ householdId, progress, categories, onClose, onSaved }: Props) {
  const b = progress?.budget;
  const [categoryId, setCategoryId] = useState(b?.categoryId ?? categories[0]?.id ?? "");
  const [period, setPeriod] = useState<BudgetPeriod>(b?.period ?? "MONTH");
  const [limit, setLimit] = useState(
    b ? (b.limitMinor / 100).toFixed(2).replace(".", ",") : "10000,00"
  );
  const [startDate, setStartDate] = useState(b?.startDate ?? todayStartMs());
  const [isRolling, setIsRolling] = useState(b?.isRolling ?? false);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);

  async function save() {
    if (!categoryId) return;
    const minor = parseAmountToMinor(limit);
    if (minor === null || minor <= 0) return;
    setSaving(true);
    try {
      await api.budgets.upsert({
        id: b?.id,
        householdId,
        categoryId,
        period,
        limitMinor: minor,
        startDate,
        isRolling,
      });
      onSaved();
    } finally {
      setSaving(false);
    }
  }

  async function remove() {
    if (!b) return;
    if (!confirm("Удалить бюджет?")) return;
    setDeleting(true);
    try {
      await api.budgets.delete(b.id);
      onSaved();
    } finally {
      setDeleting(false);
    }
  }

  return (
    <Modal
      open
      title={b ? "Редактировать бюджет" : "Новый бюджет"}
      onClose={onClose}
      footer={
        <div className="flex w-full justify-between">
          <div>
            {b && <Button variant="danger" onClick={remove} disabled={deleting}>{deleting ? "..." : "Удалить"}</Button>}
          </div>
          <div className="flex gap-2">
            <Button variant="secondary" onClick={onClose}>Отмена</Button>
            <Button onClick={save} disabled={saving}>{saving ? "..." : "Сохранить"}</Button>
          </div>
        </div>
      }
    >
      <div className="space-y-3">
        <div>
          <label className="label">Категория</label>
          <select className="select" value={categoryId} onChange={(e) => setCategoryId(e.target.value)}>
            <option value="">— выберите —</option>
            {categories.map((c) => (
              <option key={c.id} value={c.id}>{c.name}</option>
            ))}
          </select>
        </div>
        <div className="grid grid-cols-2 gap-3">
          <div>
            <label className="label">Период</label>
            <select className="select" value={period} onChange={(e) => setPeriod(e.target.value as BudgetPeriod)}>
              <option value="WEEK">Неделя</option>
              <option value="MONTH">Месяц</option>
              <option value="YEAR">Год</option>
            </select>
          </div>
          <div>
            <label className="label">Лимит</label>
            <input
              className="input"
              value={limit}
              onChange={(e) => setLimit(e.target.value)}
              inputMode="decimal"
            />
          </div>
        </div>
        <div>
          <label className="label">Начало действия</label>
          <input
            type="date"
            className="input"
            value={toIsoDate(startDate)}
            onChange={(e) => setStartDate(fromIsoDate(e.target.value))}
          />
        </div>
        <label className="flex items-center gap-2 text-sm">
          <input type="checkbox" checked={isRolling} onChange={(e) => setIsRolling(e.target.checked)} />
          Скользящий период (последние N дней)
        </label>
      </div>
    </Modal>
  );
}
