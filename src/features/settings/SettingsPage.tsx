import PageHeader from "../../components/PageHeader";

export default function SettingsPage() {
  return (
    <div>
      <PageHeader
        title="Настройки"
        subtitle="Синхронизация с Firebase, экспорт, импорт"
      />
      <div className="p-8 space-y-6">
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-2">Firebase синхронизация</h2>
          <p className="text-sm text-slate-500">
            Синхронизация с Android-приложением будет настроена в следующем этапе. Здесь появятся:
            подключение google-services.json, вход по email/паролю, ручная и автоматическая синхронизация.
          </p>
        </section>
        <section className="bg-white rounded-lg p-6 shadow-sm">
          <h2 className="text-lg font-semibold text-slate-900 mb-2">Экспорт данных</h2>
          <p className="text-sm text-slate-500">
            Здесь появятся кнопки экспорта в CSV, XLSX, JSON-бэкап и импорт.
          </p>
        </section>
      </div>
    </div>
  );
}
