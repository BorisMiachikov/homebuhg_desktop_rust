# HomeBuhg Desktop

Windows-приложение — десктопный клиент домашней бухгалтерии **HomeBuhg**.  
Синхронизируется с Android-версией через Firebase Firestore (те же коллекции, last-write-wins по `updatedAt`).

**Стек:** Tauri 2 · Rust · React 18 · TypeScript · SQLite (rusqlite) · Tailwind CSS 3 · Recharts

**Android-репозиторий:** https://github.com/BorisMiachikov/homebuhg

---

## Дорожная карта

| # | Этап | Статус | Описание |
|---|------|--------|----------|
| 0 | Setup | ✅ Done | Tauri 2 + React + TS + Tailwind, Cargo.toml, capabilities |
| 1 | Локальная БД и модели | ✅ Done | SQLite 10 таблиц, все Rust-модели, репозитории, domain (balance, session) |
| 2 | MVP UI | ✅ Done | Home, Operations + OperationEditDialog + ReceiptItemDialog, Accounts, Categories |
| 3 | Бюджеты и регулярные операции | ✅ Done | Budgets с прогресс-барами, RecurringRules + RecurringEditDialog |
| 4 | Отчёты | ✅ Done | Recharts BarChart динамики, топ категорий, SQL-агрегации |
| 5 | Firebase REST sync | 🔲 Todo | auth.rs, firestore.rs, mapper.rs, sync_service.rs; SettingsPage с формой входа |
| 6 | Экспорт и импорт | 🔲 Todo | CSV/XLSX/JSON export + import; кнопки в SettingsPage |
| 7 | Polish + MSI installer | 🔲 Todo | Тёмная/светлая тема, toast-уведомления, unit-тесты Rust, сборка MSI |

---

## Что уже реализовано (этапы 0–4)

### Rust backend (`src-tauri/src/`)
- `db/migrations.rs` — SQL V1: все 10 таблиц + индексы (parity с Android Room)
- `models/` — 9 файлов: account, category, transaction, receipt_item, budget, recurring, merchant, household, money (`format_minor` → "1 234,56 ₽")
- `repository/` — 8 файлов: CRUD + `modified_since()` для sync, `adjust_balance()`, `replace_all()` для позиций
- `domain/session.rs` — `ensure_local_session()`: user "local", household "Мой кошелёк", 13 категорий с теми же именами и ARGB-цветами, что в Android
- `domain/balance.rs` — `apply_new()` / `revert()`: пересчёт баланса счетов при создании/изменении/удалении операций
- `commands/` — 9 файлов, ~25 команд: accounts, categories, operations (с `OperationDetail {transaction, items}`), budgets (с `BudgetProgress`), recurring, reports (summary, monthly, top_categories), merchants, session
- `error.rs` — `AppError` с `impl Serialize` для возврата в JS

### React frontend (`src/`)
- `lib/types.ts` — TypeScript-интерфейсы для всех моделей + `UNITS` константа
- `lib/api.ts` — типизированные обёртки `invoke()` по всем командам
- `lib/format.ts` — `formatMoney`, `parseAmountToMinor`, `colorToCss`, `toIsoDate`, `fromIsoDate`
- `store/session.ts` — Zustand-стор с `householdId` и `bootstrap()`
- 8 страниц: HomePage, OperationsPage, AccountsPage, CategoriesPage, BudgetsPage, RecurringPage, ReportsPage, SettingsPage (заглушка)
- `OperationEditDialog` — табы Расход/Доход/Перевод, позиции с автосуммой, поле суммы блокируется при наличии позиций
- `ReceiptItemDialog` — autocomplete товаров, автозаполнение цены, datalist единиц измерения

---

## Следующий этап (5): Firebase REST sync

Файлы к созданию:
```
src-tauri/src/sync/
  auth.rs          # signInWithPassword, refreshIdToken, signUp
  firestore.rs     # run_query (WHERE updatedAt > lastSync), batch_commit
  mapper.rs        # to_firestore_value / from_firestore_value (parity с FirestoreMapper.kt)
  sync_service.rs  # upload_modified_since + download_updated_after (4 коллекции)
  mod.rs
src-tauri/src/commands/sync.rs  # tauri-команды: sync_login, sync_now, sync_status
src/features/settings/SettingsPage.tsx  # полная страница: загрузка google-services.json,
                                        # форма логина, last_sync, кнопка "Синхронизировать"
```

Ключевые детали mapper.rs:
- enum → строка через `.to_string()` / `.as_str()`
- `i64` → `{"integerValue": "число-как-строка"}` (REST Firestore format)
- `bool` → `{"booleanValue": true/false}`
- `f64` (qty) → `{"doubleValue": 1.0}`
- `null` → `{"nullValue": null}`
- Формат документа должен 1:1 повторять `FirestoreMapper.kt` в Android-репозитории

Конфиг хранится в `tauri-plugin-store` под ключом `firebase`:
```json
{ "project_id": "", "api_key": "", "id_token": "", "refresh_token": "",
  "local_id": "", "last_sync_ms": 0, "household_id": "" }
```

---

## Разработка

```bash
npm run tauri dev    # dev-режим с hot-reload
npm run tauri build  # сборка MSI/NSIS installer
```

Требования: Node 20+, Rust 1.80+, Visual Studio Build Tools 2022 (MSVC).
