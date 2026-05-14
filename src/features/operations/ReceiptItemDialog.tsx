import { useEffect, useState } from "react";
import { api } from "../../lib/api";
import { formatMoney, parseAmountToMinor } from "../../lib/format";
import type { ReceiptItem } from "../../lib/types";
import { UNITS } from "../../lib/types";
import Modal from "../../components/Modal";
import Button from "../../components/Button";

interface Props {
  householdId: string;
  item: ReceiptItem | null;
  onClose: () => void;
  onSave: (item: ReceiptItem) => void;
}

export default function ReceiptItemDialog({ householdId, item, onClose, onSave }: Props) {
  const [name, setName] = useState(item?.name ?? "");
  const [price, setPrice] = useState(
    item ? (item.priceMinor / 100).toFixed(2).replace(".", ",") : "0,00"
  );
  const [qty, setQty] = useState(item ? String(item.qty) : "1");
  const [unit, setUnit] = useState(item?.unit ?? "шт");
  const [nameSuggestions, setNameSuggestions] = useState<string[]>([]);
  const [showSuggestions, setShowSuggestions] = useState(false);

  useEffect(() => {
    api.operations.itemNames(householdId).then(setNameSuggestions).catch(() => {});
  }, [householdId]);

  const filteredSuggestions = nameSuggestions
    .filter((s) => s.toLowerCase().includes(name.toLowerCase()) && s !== name)
    .slice(0, 8);

  async function pickName(n: string) {
    setName(n);
    setShowSuggestions(false);
    const last = await api.operations.lastPrice(householdId, n);
    if (last !== null) {
      setPrice((last / 100).toFixed(2).replace(".", ","));
    }
  }

  function save() {
    if (!name.trim()) return;
    const priceMinor = parseAmountToMinor(price);
    if (priceMinor === null || priceMinor < 0) return;
    const qtyNum = Number(qty.replace(",", "."));
    if (!Number.isFinite(qtyNum) || qtyNum <= 0) return;
    onSave({
      id: item?.id ?? crypto.randomUUID(),
      transactionId: item?.transactionId ?? "",
      name: name.trim(),
      priceMinor,
      qty: qtyNum,
      unit: unit.trim() || null,
      fnsRaw: item?.fnsRaw ?? null,
    });
  }

  const total = (() => {
    const p = parseAmountToMinor(price) ?? 0;
    const q = Number(qty.replace(",", ".")) || 0;
    return Math.round(p * q);
  })();

  return (
    <Modal
      open
      title={item ? "Редактировать позицию" : "Новая позиция"}
      onClose={onClose}
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>Отмена</Button>
          <Button onClick={save}>Сохранить</Button>
        </>
      }
    >
      <div className="space-y-3">
        <div className="relative">
          <label className="label">Товар</label>
          <input
            className="input"
            value={name}
            onChange={(e) => {
              setName(e.target.value);
              setShowSuggestions(true);
            }}
            onFocus={() => setShowSuggestions(true)}
            onBlur={() => setTimeout(() => setShowSuggestions(false), 150)}
            placeholder="Хлеб"
            autoFocus
          />
          {showSuggestions && filteredSuggestions.length > 0 && (
            <div className="absolute z-10 mt-1 w-full bg-white border border-slate-200 rounded shadow-lg max-h-48 overflow-auto">
              {filteredSuggestions.map((s) => (
                <button
                  key={s}
                  className="w-full text-left px-3 py-2 hover:bg-slate-100 text-sm"
                  onMouseDown={(e) => {
                    e.preventDefault();
                    pickName(s);
                  }}
                >
                  {s}
                </button>
              ))}
            </div>
          )}
        </div>
        <div className="grid grid-cols-3 gap-3">
          <div>
            <label className="label">Цена</label>
            <input
              className="input"
              value={price}
              onChange={(e) => setPrice(e.target.value)}
              inputMode="decimal"
            />
          </div>
          <div>
            <label className="label">Кол-во</label>
            <input
              className="input"
              value={qty}
              onChange={(e) => setQty(e.target.value)}
              inputMode="decimal"
            />
          </div>
          <div>
            <label className="label">Ед. изм.</label>
            <input
              className="input"
              value={unit}
              onChange={(e) => setUnit(e.target.value)}
              list="units-list"
            />
            <datalist id="units-list">
              {UNITS.map((u) => <option key={u} value={u} />)}
            </datalist>
          </div>
        </div>
        <div className="text-right text-sm text-slate-600">
          Сумма: <span className="font-semibold text-slate-900">{formatMoney(total)}</span>
        </div>
      </div>
    </Modal>
  );
}
