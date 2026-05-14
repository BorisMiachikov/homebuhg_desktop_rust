import { useState } from "react";
import { api } from "../../lib/api";
import { formatMoney, parseAmountToMinor } from "../../lib/format";
import type { Account, AccountType } from "../../lib/types";
import Modal from "../../components/Modal";
import Button from "../../components/Button";

interface Props {
  householdId: string;
  account: Account | null;
  onClose: () => void;
  onSaved: () => void;
}

const COLORS = [
  0xFF607D8B, 0xFF4CAF50, 0xFF2196F3, 0xFFFF9800, 0xFFF44336,
  0xFF9C27B0, 0xFFE91E63, 0xFF009688, 0xFF795548, 0xFF424242,
];

export default function AccountEditDialog({ householdId, account, onClose, onSaved }: Props) {
  const [name, setName] = useState(account?.name ?? "");
  const [type, setType] = useState<AccountType>(account?.type ?? "CARD_DEBIT");
  const [balance, setBalance] = useState<string>(
    account ? (account.balanceMinor / 100).toFixed(2).replace(".", ",") : "0,00"
  );
  const [currency, setCurrency] = useState(account?.currency ?? "RUB");
  const [color, setColor] = useState(account?.color ?? COLORS[0]);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function save() {
    setError(null);
    if (!name.trim()) {
      setError("Введите название");
      return;
    }
    const minor = parseAmountToMinor(balance);
    if (minor === null) {
      setError("Некорректный баланс");
      return;
    }
    setSaving(true);
    try {
      await api.accounts.upsert({
        id: account?.id,
        householdId,
        name: name.trim(),
        type,
        balanceMinor: minor,
        currency,
        color,
        iconKey: "credit_card",
      });
      onSaved();
    } catch (e: any) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  return (
    <Modal
      open
      title={account ? "Редактировать счёт" : "Новый счёт"}
      onClose={onClose}
      footer={
        <>
          <Button variant="secondary" onClick={onClose} disabled={saving}>Отмена</Button>
          <Button onClick={save} disabled={saving}>{saving ? "Сохранение..." : "Сохранить"}</Button>
        </>
      }
    >
      <div className="space-y-4">
        <Field label="Название">
          <input
            className="input"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="Карта Тинькофф"
          />
        </Field>
        <Field label="Тип">
          <select
            className="input"
            value={type}
            onChange={(e) => setType(e.target.value as AccountType)}
          >
            <option value="CARD_DEBIT">Дебетовая карта</option>
            <option value="CARD_CREDIT">Кредитная карта</option>
            <option value="CASH">Наличные</option>
          </select>
        </Field>
        <div className="grid grid-cols-2 gap-3">
          <Field label="Начальный баланс">
            <input
              className="input"
              value={balance}
              onChange={(e) => setBalance(e.target.value)}
              inputMode="decimal"
            />
          </Field>
          <Field label="Валюта">
            <select className="input" value={currency} onChange={(e) => setCurrency(e.target.value)}>
              <option value="RUB">RUB ₽</option>
              <option value="USD">USD $</option>
              <option value="EUR">EUR €</option>
            </select>
          </Field>
        </div>
        <Field label="Цвет">
          <div className="flex flex-wrap gap-2">
            {COLORS.map((c) => (
              <button
                key={c}
                type="button"
                onClick={() => setColor(c)}
                className={`w-8 h-8 rounded-full border-2 ${color === c ? "border-slate-900" : "border-transparent"}`}
                style={{ backgroundColor: cssColor(c) }}
              />
            ))}
          </div>
        </Field>
        {error && <div className="text-red-600 text-sm">{error}</div>}
        {account && (
          <div className="text-xs text-slate-500 pt-2 border-t">
            Текущий баланс счёта: {formatMoney(account.balanceMinor, account.currency)}
          </div>
        )}
      </div>
      <style>{`.input{display:block;width:100%;padding:.5rem .75rem;border:1px solid #cbd5e1;border-radius:.375rem;background:#fff;font-size:.875rem;outline:none}.input:focus{border-color:#0f172a;box-shadow:0 0 0 1px #0f172a}`}</style>
    </Modal>
  );
}

function Field({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <label className="block">
      <span className="text-sm text-slate-600 mb-1 block">{label}</span>
      {children}
    </label>
  );
}

function cssColor(c: number): string {
  const x = c >>> 0;
  const r = (x >> 16) & 0xff;
  const g = (x >> 8) & 0xff;
  const b = x & 0xff;
  return `rgb(${r},${g},${b})`;
}
