import { useEffect, useState } from "react";
import { api } from "../../lib/api";
import { colorToCss } from "../../lib/format";
import type { Category, CategoryType } from "../../lib/types";
import { useSession } from "../../store/session";
import PageHeader from "../../components/PageHeader";
import Button from "../../components/Button";
import Modal from "../../components/Modal";

const COLORS = [
  0xFF4CAF50, 0xFF2196F3, 0xFFFF9800, 0xFF9C27B0, 0xFFF44336,
  0xFFE91E63, 0xFF00BCD4, 0xFF607D8B, 0xFF795548, 0xFF9E9E9E,
];

export default function CategoriesPage() {
  const householdId = useSession((s) => s.householdId)!;
  const [tab, setTab] = useState<CategoryType>("EXPENSE");
  const [categories, setCategories] = useState<Category[]>([]);
  const [editing, setEditing] = useState<Category | null | undefined>(undefined);

  async function load() {
    setCategories(await api.categories.list(householdId));
  }
  useEffect(() => {
    load();
  }, [householdId]);

  const filtered = categories.filter((c) => c.type === tab);

  return (
    <div>
      <PageHeader
        title="Категории"
        subtitle="Группировка операций"
        actions={<Button onClick={() => setEditing(null)}>+ Добавить</Button>}
      />
      <div className="p-8">
        <div className="flex gap-2 mb-4">
          <TabBtn active={tab === "EXPENSE"} onClick={() => setTab("EXPENSE")}>Расходы</TabBtn>
          <TabBtn active={tab === "INCOME"} onClick={() => setTab("INCOME")}>Доходы</TabBtn>
        </div>
        <div className="bg-white rounded-lg shadow-sm divide-y divide-slate-100">
          {filtered.length === 0 ? (
            <div className="p-6 text-slate-500">Категорий нет</div>
          ) : (
            filtered.map((c) => (
              <div key={c.id} className="px-5 py-3 flex items-center gap-4">
                <div
                  className="w-8 h-8 rounded-full"
                  style={{ backgroundColor: colorToCss(c.color) }}
                />
                <div className="flex-1 font-medium text-slate-900">{c.name}</div>
                <button
                  className="text-slate-500 hover:text-slate-900 text-sm mr-2"
                  onClick={() => setEditing(c)}
                >
                  Изменить
                </button>
                <button
                  className="text-slate-400 hover:text-red-600 text-sm"
                  onClick={async () => {
                    if (confirm(`Удалить категорию "${c.name}"?`)) {
                      await api.categories.delete(c.id);
                      load();
                    }
                  }}
                >
                  Удалить
                </button>
              </div>
            ))
          )}
        </div>
      </div>

      {editing !== undefined && (
        <CategoryEditDialog
          householdId={householdId}
          category={editing}
          defaultType={tab}
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

function CategoryEditDialog({
  householdId,
  category,
  defaultType,
  onClose,
  onSaved,
}: {
  householdId: string;
  category: Category | null;
  defaultType: CategoryType;
  onClose: () => void;
  onSaved: () => void;
}) {
  const [name, setName] = useState(category?.name ?? "");
  const [type, setType] = useState<CategoryType>(category?.type ?? defaultType);
  const [color, setColor] = useState(category?.color ?? COLORS[0]);
  const [saving, setSaving] = useState(false);

  async function save() {
    if (!name.trim()) return;
    setSaving(true);
    try {
      await api.categories.upsert({
        id: category?.id,
        householdId,
        name: name.trim(),
        type,
        color,
        iconKey: "more_horiz",
        sortOrder: category?.sortOrder ?? 0,
      });
      onSaved();
    } finally {
      setSaving(false);
    }
  }

  return (
    <Modal
      open
      title={category ? "Редактировать категорию" : "Новая категория"}
      onClose={onClose}
      footer={
        <>
          <Button variant="secondary" onClick={onClose}>Отмена</Button>
          <Button onClick={save} disabled={saving}>{saving ? "..." : "Сохранить"}</Button>
        </>
      }
    >
      <div className="space-y-4">
        <label className="block">
          <span className="text-sm text-slate-600 mb-1 block">Название</span>
          <input
            className="w-full px-3 py-2 border border-slate-300 rounded text-sm"
            value={name}
            onChange={(e) => setName(e.target.value)}
            autoFocus
          />
        </label>
        <label className="block">
          <span className="text-sm text-slate-600 mb-1 block">Тип</span>
          <select
            className="w-full px-3 py-2 border border-slate-300 rounded text-sm"
            value={type}
            onChange={(e) => setType(e.target.value as CategoryType)}
          >
            <option value="EXPENSE">Расход</option>
            <option value="INCOME">Доход</option>
          </select>
        </label>
        <div>
          <span className="text-sm text-slate-600 mb-1 block">Цвет</span>
          <div className="flex flex-wrap gap-2">
            {COLORS.map((c) => (
              <button
                key={c}
                type="button"
                onClick={() => setColor(c)}
                className={`w-8 h-8 rounded-full border-2 ${color === c ? "border-slate-900" : "border-transparent"}`}
                style={{ backgroundColor: colorToCss(c) }}
              />
            ))}
          </div>
        </div>
      </div>
    </Modal>
  );
}
