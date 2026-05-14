import { useEffect, useMemo, useState } from "react";
import { api } from "../../lib/api";
import {
  formatMoney,
  fromIsoDate,
  parseAmountToMinor,
  toIsoDate,
  todayStartMs,
} from "../../lib/format";
import type {
  Account,
  Category,
  OperationDetail,
  ReceiptItem,
  TransactionType,
} from "../../lib/types";
import Modal from "../../components/Modal";
import Button from "../../components/Button";
import ReceiptItemDialog from "./ReceiptItemDialog";

interface Props {
  householdId: string;
  operationId: string | null;
  defaultType: TransactionType;
  accounts: Account[];
  categories: Category[];
  onClose: () => void;
  onSaved: () => void;
}

export default function OperationEditDialog({
  householdId,
  operationId,
  defaultType,
  accounts,
  categories,
  onClose,
  onSaved,
}: Props) {
  const [type, setType] = useState<TransactionType>(defaultType);
  const [occurredAt, setOccurredAt] = useState<number>(todayStartMs());
  const [amount, setAmount] = useState<string>("0,00");
  const [accountId, setAccountId] = useState<string>(accounts[0]?.id ?? "");
  const [toAccountId, setToAccountId] = useState<string>("");
  const [categoryId, setCategoryId] = useState<string>("");
  const [note, setNote] = useState<string>("");
  const [items, setItems] = useState<ReceiptItem[]>([]);
  const [itemDialog, setItemDialog] = useState<{ index: number; item: ReceiptItem | null } | null>(null);
  const [saving, setSaving] = useState(false);
  const [deleting, setDeleting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!operationId) return;
    (async () => {
      const detail = await api.operations.get(operationId);
      if (!detail) return;
      applyDetail(detail);
    })();
  }, [operationId]);

  function applyDetail(detail: OperationDetail) {
    const t = detail.transaction;
    setType(t.type);
    setOccurredAt(t.occurredAt);
    setAmount((t.amountMinor / 100).toFixed(2).replace(".", ","));
    setAccountId(t.accountId);
    setToAccountId(t.toAccountId ?? "");
    setCategoryId(t.categoryId ?? "");
    setNote(t.note ?? "");
    setItems(detail.items);
  }

  const filteredCats = useMemo(
    () => categories.filter((c) => c.type === (type === "INCOME" ? "INCOME" : "EXPENSE")),
    [categories, type]
  );

  // Автосумма из позиций
  useEffect(() => {
    if (items.length === 0) return;
    const sum = items.reduce((acc, it) => acc + Math.round(it.priceMinor * it.qty), 0);
    setAmount((sum / 100).toFixed(2).replace(".", ","));
  }, [items]);

  async function save() {
    setError(null);
    const minor = parseAmountToMinor(amount);
    if (minor === null || minor <= 0) {
      setError("Введите сумму больше нуля");
      return;
    }
    if (!accountId) {
      setError("Выберите счёт");
      return;
    }
    if (type === "TRANSFER" && (!toAccountId || toAccountId === accountId)) {
      setError("Выберите второй счёт для перевода");
      return;
    }
    setSaving(true);
    try {
      await api.operations.upsert({
        id: operationId ?? undefined,
        householdId,
        occurredAt,
        type,
        amountMinor: minor,
        currency: "RUB",
        accountId,
        toAccountId: type === "TRANSFER" ? toAccountId : null,
        categoryId: type === "TRANSFER" ? null : (categoryId || null),
        note: note.trim() || null,
        items: items.map((it) => ({
          id: it.id,
          name: it.name,
          priceMinor: it.priceMinor,
          qty: it.qty,
          unit: it.unit,
        })),
      });
      onSaved();
    } catch (e: any) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  async function remove() {
    if (!operationId) return;
    if (!confirm("Удалить операцию?")) return;
    setDeleting(true);
    try {
      await api.operations.delete(operationId);
      onSaved();
    } finally {
      setDeleting(false);
    }
  }

  return (
    <Modal
      open
      title={operationId ? "Редактировать операцию" : "Новая операция"}
      onClose={onClose}
      width="max-w-3xl"
      footer={
        <div className="flex w-full justify-between">
          <div>
            {operationId && (
              <Button variant="danger" onClick={remove} disabled={deleting || saving}>
                {deleting ? "..." : "Удалить"}
              </Button>
            )}
          </div>
          <div className="flex gap-2">
            <Button variant="secondary" onClick={onClose} disabled={saving}>Отмена</Button>
            <Button onClick={save} disabled={saving}>{saving ? "Сохранение..." : "Сохранить"}</Button>
          </div>
        </div>
      }
    >
      <div className="space-y-4">
        <div className="flex gap-2">
          <TypeTab active={type === "EXPENSE"} onClick={() => setType("EXPENSE")} color="bg-expense">Расход</TypeTab>
          <TypeTab active={type === "INCOME"} onClick={() => setType("INCOME")} color="bg-income">Доход</TypeTab>
          <TypeTab active={type === "TRANSFER"} onClick={() => setType("TRANSFER")} color="bg-transfer">Перевод</TypeTab>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="label">Сумма</label>
            <input
              className="input text-lg font-semibold"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              disabled={items.length > 0}
            />
            {items.length > 0 && (
              <div className="text-xs text-slate-500 mt-1">Сумма рассчитана из позиций</div>
            )}
          </div>
          <div>
            <label className="label">Дата</label>
            <input
              type="date"
              className="input"
              value={toIsoDate(occurredAt)}
              onChange={(e) => setOccurredAt(fromIsoDate(e.target.value))}
            />
          </div>
        </div>

        <div className="grid grid-cols-2 gap-4">
          <div>
            <label className="label">{type === "TRANSFER" ? "Со счёта" : "Счёт"}</label>
            <select
              className="select"
              value={accountId}
              onChange={(e) => setAccountId(e.target.value)}
            >
              <option value="">— выберите —</option>
              {accounts.map((a) => (
                <option key={a.id} value={a.id}>
                  {a.name} · {formatMoney(a.balanceMinor, a.currency)}
                </option>
              ))}
            </select>
          </div>
          {type === "TRANSFER" ? (
            <div>
              <label className="label">На счёт</label>
              <select
                className="select"
                value={toAccountId}
                onChange={(e) => setToAccountId(e.target.value)}
              >
                <option value="">— выберите —</option>
                {accounts.filter((a) => a.id !== accountId).map((a) => (
                  <option key={a.id} value={a.id}>{a.name}</option>
                ))}
              </select>
            </div>
          ) : (
            <div>
              <label className="label">Категория</label>
              <select
                className="select"
                value={categoryId}
                onChange={(e) => setCategoryId(e.target.value)}
              >
                <option value="">— без категории —</option>
                {filteredCats.map((c) => (
                  <option key={c.id} value={c.id}>{c.name}</option>
                ))}
              </select>
            </div>
          )}
        </div>

        <div>
          <label className="label">Заметка</label>
          <input
            className="input"
            value={note}
            onChange={(e) => setNote(e.target.value)}
            placeholder="Опционально"
          />
        </div>

        {type !== "TRANSFER" && (
          <div>
            <div className="flex items-center justify-between mb-2">
              <label className="label !mb-0">Позиции ({items.length})</label>
              <button
                onClick={() => setItemDialog({ index: -1, item: null })}
                className="text-sm text-slate-700 hover:text-slate-900 font-medium"
              >
                + Добавить позицию
              </button>
            </div>
            {items.length === 0 ? (
              <div className="text-xs text-slate-500 border border-dashed border-slate-300 rounded p-3 text-center">
                Позиций нет. Сумма вводится вручную.
              </div>
            ) : (
              <div className="border border-slate-200 rounded overflow-hidden">
                <table className="w-full text-sm">
                  <thead className="bg-slate-50 text-slate-600">
                    <tr>
                      <th className="text-left px-3 py-2 font-medium">Товар</th>
                      <th className="text-left px-3 py-2 font-medium w-20">Ед.</th>
                      <th className="text-right px-3 py-2 font-medium w-28">Цена</th>
                      <th className="text-right px-3 py-2 font-medium w-20">Кол-во</th>
                      <th className="text-right px-3 py-2 font-medium w-28">Сумма</th>
                      <th className="w-8"></th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-100">
                    {items.map((it, idx) => (
                      <tr key={it.id} className="hover:bg-slate-50 cursor-pointer" onClick={() => setItemDialog({ index: idx, item: it })}>
                        <td className="px-3 py-2">{it.name}</td>
                        <td className="px-3 py-2 text-slate-500">{it.unit ?? "—"}</td>
                        <td className="px-3 py-2 text-right">{formatMoney(it.priceMinor)}</td>
                        <td className="px-3 py-2 text-right">{it.qty}</td>
                        <td className="px-3 py-2 text-right font-medium">
                          {formatMoney(Math.round(it.priceMinor * it.qty))}
                        </td>
                        <td className="px-3 py-2 text-right">
                          <button
                            onClick={(e) => {
                              e.stopPropagation();
                              setItems(items.filter((_, i) => i !== idx));
                            }}
                            className="text-slate-400 hover:text-red-600"
                          >
                            ×
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        )}

        {error && <div className="text-red-600 text-sm">{error}</div>}
      </div>

      {itemDialog && (
        <ReceiptItemDialog
          householdId={householdId}
          item={itemDialog.item}
          onClose={() => setItemDialog(null)}
          onSave={(saved) => {
            if (itemDialog.index >= 0) {
              const next = items.slice();
              next[itemDialog.index] = saved;
              setItems(next);
            } else {
              setItems([...items, saved]);
            }
            setItemDialog(null);
          }}
        />
      )}
    </Modal>
  );
}

function TypeTab({
  active,
  onClick,
  color,
  children,
}: {
  active: boolean;
  onClick: () => void;
  color: string;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={`flex-1 px-4 py-2 rounded text-sm font-semibold transition ${
        active ? `${color} text-white` : "bg-white text-slate-700 border border-slate-300"
      }`}
    >
      {children}
    </button>
  );
}
