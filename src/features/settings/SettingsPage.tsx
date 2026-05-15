import { useEffect, useState } from "react";
import PageHeader from "../../components/PageHeader";
import { api } from "../../lib/api";
import type { SyncResult, SyncStatus } from "../../lib/types";

function formatLastSync(ms: number): string {
  if (ms === 0) return "никогда";
  return new Date(ms).toLocaleString("ru-RU");
}

export default function SettingsPage() {
  const [status, setStatus] = useState<SyncStatus | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [syncResult, setSyncResult] = useState<SyncResult | null>(null);

  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [projectId, setProjectId] = useState("");
  const [apiKey, setApiKey] = useState("");
  const [householdId, setHouseholdId] = useState("");

  useEffect(() => {
    api.sync.status().then(setStatus).catch(() => setStatus({ loggedIn: false, lastSyncMs: 0 }));
  }, []);

  async function handleLogin(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const s = await api.sync.login(email, password, projectId, apiKey, householdId);
      setStatus(s);
      setSyncResult(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  async function handleSync() {
    setLoading(true);
    setError(null);
    setSyncResult(null);
    try {
      const result = await api.sync.now();
      setSyncResult(result);
      const s = await api.sync.status();
      setStatus(s);
    } catch (err) {
      const msg = String(err);
      setError(msg);
      if (msg.toLowerCase().includes("unauthorized") || msg.toLowerCase().includes("session expired")) {
        setStatus({ loggedIn: false, lastSyncMs: 0 });
      }
    } finally {
      setLoading(false);
    }
  }

  async function handleLogout() {
    await api.sync.logout();
    setStatus({ loggedIn: false, lastSyncMs: 0 });
    setSyncResult(null);
    setError(null);
  }

  return (
    <div>
      <PageHeader title="Настройки" subtitle="Синхронизация с Firebase" />
      <div className="p-8 space-y-6">
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-4">Firebase синхронизация</h2>

          {status === null && (
            <p className="text-sm text-slate-400">Загрузка...</p>
          )}

          {status !== null && !status.loggedIn && (
            <form onSubmit={handleLogin} className="space-y-3 max-w-sm">
              {error && (
                <p className="text-sm text-red-600 bg-red-50 rounded p-2">{error}</p>
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
                  value={householdId}
                  onChange={(e) => setHouseholdId(e.target.value)}
                  required
                  className="w-full border border-slate-300 rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="uuid вашего домохозяйства"
                />
              </div>
              <button
                type="submit"
                disabled={loading}
                className="w-full bg-blue-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? "Входим..." : "Войти"}
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

              {error && (
                <p className="text-sm text-red-600 bg-red-50 rounded p-2">{error}</p>
              )}

              {syncResult && (
                <p className="text-sm text-green-700 bg-green-50 rounded p-2">
                  Загружено: {syncResult.uploaded}, получено: {syncResult.downloaded}
                </p>
              )}

              <div className="flex gap-2">
                <button
                  onClick={handleSync}
                  disabled={loading}
                  className="bg-blue-600 text-white rounded px-4 py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {loading ? "Синхронизация..." : "Синхронизировать сейчас"}
                </button>
                <button
                  onClick={handleLogout}
                  disabled={loading}
                  className="bg-white border border-slate-300 text-slate-700 rounded px-4 py-2 text-sm font-medium hover:bg-slate-50 disabled:opacity-50"
                >
                  Выйти
                </button>
              </div>
            </div>
          )}
        </section>

        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-2">Экспорт данных</h2>
          <p className="text-sm text-slate-500">
            Экспорт в CSV, XLSX, JSON-бэкап и импорт — этап 6.
          </p>
        </section>
      </div>
    </div>
  );
}
