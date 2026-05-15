import { useEffect, useState } from "react";
import PageHeader from "../../components/PageHeader";
import { api } from "../../lib/api";
import { fromIsoDate } from "../../lib/format";
import type { ImportResult, SyncResult, SyncStatus } from "../../lib/types";
import { useSession } from "../../store/session";

function formatLastSync(ms: number): string {
  if (ms === 0) return "никогда";
  return new Date(ms).toLocaleString("ru-RU");
}

function thisYear(): { from: string; to: string } {
  const y = new Date().getFullYear();
  return { from: `${y}-01-01`, to: `${y}-12-31` };
}

export default function SettingsPage() {
  const householdId = useSession((s) => s.householdId)!;

  // ── Sync ──────────────────────────────────────────────────────────────────
  const [status, setStatus] = useState<SyncStatus | null>(null);
  const [syncLoading, setSyncLoading] = useState(false);
  const [syncError, setSyncError] = useState<string | null>(null);
  const [syncResult, setSyncResult] = useState<SyncResult | null>(null);

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [projectId, setProjectId] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [loginHouseholdId, setLoginHouseholdId] = useState("");

  useEffect(() => {
    api.sync.status().then(setStatus).catch(() => setStatus({ loggedIn: false, lastSyncMs: 0 }));
  }, []);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setSyncLoading(true);
    setSyncError(null);
    try {
      const s = await api.sync.login(email, password, projectId, apiKey, loginHouseholdId);
      setStatus(s);
      setSyncResult(null);
    } catch (err) {
      setSyncError(String(err));
    } finally {
      setSyncLoading(false);
    }
  }

  async function handleSync() {
    setSyncLoading(true);
    setSyncError(null);
    setSyncResult(null);
    try {
      const result = await api.sync.now();
      setSyncResult(result);
      const s = await api.sync.status();
      setStatus(s);
    } catch (err) {
      const msg = String(err);
      setSyncError(msg);
      if (msg.toLowerCase().includes("unauthorized") || msg.toLowerCase().includes("session expired")) {
        setStatus({ loggedIn: false, lastSyncMs: 0 });
      }
    } finally {
      setSyncLoading(false);
    }
  }

  async function handleLogout() {
    await api.sync.logout();
    setStatus({ loggedIn: false, lastSyncMs: 0 });
    setSyncResult(null);
    setSyncError(null);
  }

  // ── Export ────────────────────────────────────────────────────────────────
  const def = thisYear();
  const [fromDate, setFromDate] = useState(def.from);
  const [toDate, setToDate] = useState(def.to);
  const [csvLoading, setCsvLoading] = useState(false);
  const [xlsxLoading, setXlsxLoading] = useState(false);
  const [exportPath, setExportPath] = useState<string | null>(null);
  const [exportError, setExportError] = useState<string | null>(null);

  const [backupLoading, setBackupLoading] = useState(false);
  const [importLoading, setImportLoading] = useState(false);
  const [backupPath, setBackupPath] = useState<string | null>(null);
  const [importResult, setImportResult] = useState<ImportResult | null>(null);
  const [backupError, setBackupError] = useState<string | null>(null);

  function exportRange(): { fromMs: number; toMs: number } {
    const fromMs = fromIsoDate(fromDate);
    const toMs = fromIsoDate(toDate) + 86_400_000 - 1;
    return { fromMs, toMs };
  }

  async function handleExportCsv() {
    setCsvLoading(true);
    setExportPath(null);
    setExportError(null);
    try {
      const { fromMs, toMs } = exportRange();
      const path = await api.export.transactionsCsv(householdId, fromMs, toMs);
      setExportPath(path);
    } catch (err) {
      setExportError(String(err));
    } finally {
      setCsvLoading(false);
    }
  }

  async function handleExportXlsx() {
    setXlsxLoading(true);
    setExportPath(null);
    setExportError(null);
    try {
      const { fromMs, toMs } = exportRange();
      const path = await api.export.transactionsXlsx(householdId, fromMs, toMs);
      setExportPath(path);
    } catch (err) {
      setExportError(String(err));
    } finally {
      setXlsxLoading(false);
    }
  }

  async function handleBackupJson() {
    setBackupLoading(true);
    setBackupPath(null);
    setImportResult(null);
    setBackupError(null);
    try {
      const path = await api.export.backupJson(householdId);
      setBackupPath(path);
    } catch (err) {
      setBackupError(String(err));
    } finally {
      setBackupLoading(false);
    }
  }

  async function handleImportJson() {
    setImportLoading(true);
    setBackupPath(null);
    setImportResult(null);
    setBackupError(null);
    try {
      const result = await api.export.importJson(householdId);
      setImportResult(result);
    } catch (err) {
      setBackupError(String(err));
    } finally {
      setImportLoading(false);
    }
  }

  return (
    <div>
      <PageHeader title="Настройки" subtitle="Синхронизация и экспорт" />
      <div className="p-8 space-y-6">

        {/* ── Firebase синхронизация ────────────────────────────────────── */}
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">Firebase синхронизация</h2>

          {status === null && (
            <p className="text-sm text-slate-400">Загрузка...</p>
          )}

          {status !== null && !status.loggedIn && (
            <form onSubmit={handleLogin} className="space-y-3 max-w-sm">
              {syncError && (
                <p className="text-sm text-red-600 bg-red-50 rounded p-2">{syncError}</p>
              )}
              <div>
                <label className="block text-xs font-medium text-slate-600 mb-1">Email</label>
                <input
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="user@example.com"
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 mb-1">Пароль</label>
                <input
                  type="password"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 mb-1">Firebase Project ID</label>
                <input
                  type="text"
                  value={projectId}
                  onChange={(e) => setProjectId(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="my-project-12345"
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 mb-1">API Key (Web)</label>
                <input
                  type="text"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="AIzaSy..."
                />
              </div>
              <div>
                <label className="block text-xs font-medium text-slate-600 mb-1">Household ID</label>
                <input
                  type="text"
                  value={loginHouseholdId}
                  onChange={(e) => setLoginHouseholdId(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="uuid вашего домохозяйства"
                />
              </div>
              <button
                type="submit"
                disabled={syncLoading}
                className="w-full bg-blue-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {syncLoading ? "Входим..." : "Войти"}
              </button>
            </form>
          )}

          {status !== null && status.loggedIn && (
            <div className="space-y-4">
              <p className="text-sm text-slate-600">
                Последняя синхронизация:{" "}
                <span className="font-medium text-slate-900">
                  {formatLastSync(status.lastSyncMs)}
                </span>
              </p>

              {syncError && (
                <p className="text-sm text-red-600 bg-red-50 rounded p-2">{syncError}</p>
              )}

              {syncResult && (
                <p className="text-sm text-green-700 bg-green-50 rounded p-2">
                  Загружено: {syncResult.uploaded}, получено: {syncResult.downloaded}
                </p>
              )}

              <div className="flex gap-2">
                <button
                  onClick={handleSync}
                  disabled={syncLoading}
                  className="bg-blue-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {syncLoading ? "Синхронизация..." : "Синхронизировать сейчас"}
                </button>
                <button
                  onClick={handleLogout}
                  disabled={syncLoading}
                  className="bg-white border border-slate-300 text-slate-700 rounded px-4 py-2 text-sm font-medium hover:bg-slate-50 disabled:opacity-50"
                >
                  Выйти
                </button>
              </div>
            </div>
          )}
        </section>

        {/* ── Экспорт транзакций ────────────────────────────────────────── */}
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">Экспорт транзакций</h2>

          <div className="flex flex-wrap gap-4 items-end mb-4">
            <div>
              <label className="block text-xs font-medium text-slate-600 mb-1">С</label>
              <input
                type="date"
                value={fromDate}
                onChange={(e) => setFromDate(e.target.value)}
                className="border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div>
              <label className="block text-xs font-medium text-slate-600 mb-1">По</label>
              <input
                type="date"
                value={toDate}
                onChange={(e) => setToDate(e.target.value)}
                className="border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <button
              onClick={handleExportCsv}
              disabled={csvLoading || xlsxLoading}
              className="bg-emerald-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-emerald-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {csvLoading ? "Экспорт..." : "Скачать CSV"}
            </button>
            <button
              onClick={handleExportXlsx}
              disabled={csvLoading || xlsxLoading}
              className="bg-teal-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-teal-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {xlsxLoading ? "Экспорт..." : "Скачать XLSX"}
            </button>
          </div>

          {exportError && (
            <p className="text-sm text-red-600 bg-red-50 rounded p-2">{exportError}</p>
          )}
          {exportPath && (
            <p className="text-sm text-green-700 bg-green-50 rounded p-2">
              Сохранено: {exportPath}
            </p>
          )}
        </section>

        {/* ── Резервная копия ───────────────────────────────────────────── */}
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">Резервная копия</h2>

          <div className="flex flex-wrap gap-2 mb-4">
            <button
              onClick={handleBackupJson}
              disabled={backupLoading || importLoading}
              className="bg-indigo-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-indigo-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {backupLoading ? "Сохранение..." : "Скачать JSON"}
            </button>
            <button
              onClick={handleImportJson}
              disabled={backupLoading || importLoading}
              className="bg-white border border-slate-300 text-slate-700 rounded px-4 py-2 text-sm font-medium hover:bg-slate-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {importLoading ? "Импорт..." : "Импортировать JSON"}
            </button>
          </div>

          {backupError && (
            <p className="text-sm text-red-600 bg-red-50 rounded p-2">{backupError}</p>
          )}
          {backupPath && (
            <p className="text-sm text-green-700 bg-green-50 rounded p-2">
              Сохранено: {backupPath}
            </p>
          )}
          {importResult && (
            <p className="text-sm text-green-700 bg-green-50 rounded p-2">
              Импортировано: транзакций {importResult.transactions}, счетов {importResult.accounts},{" "}
              категорий {importResult.categories}, бюджетов {importResult.budgets}
            </p>
          )}
        </section>

      </div>
    </div>
  );
}
