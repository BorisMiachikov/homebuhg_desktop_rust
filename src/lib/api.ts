import { invoke } from "@tauri-apps/api/core";
import type {
  Account,
  Budget,
  BudgetProgress,
  Category,
  CategorySpend,
  ImportResult,
  MonthlyPoint,
  OperationDetail,
  RecurringRule,
  ReportRange,
  ReportSummary,
  SessionInfo,
  SyncResult,
  SyncStatus,
  Transaction,
} from "./types";

export const api = {
  session: {
    bootstrap: (): Promise<SessionInfo> => invoke("session_bootstrap"),
  },
  accounts: {
    list: (householdId: string): Promise<Account[]> =>
      invoke("accounts_list", { householdId }),
    get: (id: string): Promise<Account | null> => invoke("accounts_get", { id }),
    upsert: (input: Partial<Account> & { householdId: string; name: string; type: string }): Promise<Account> =>
      invoke("accounts_upsert", { input }),
    archive: (id: string, archived: boolean): Promise<void> =>
      invoke("accounts_archive", { id, archived }),
    total: (householdId: string): Promise<{ totalMinor: number }> =>
      invoke("accounts_total", { householdId }),
  },
  categories: {
    list: (householdId: string): Promise<Category[]> =>
      invoke("categories_list", { householdId }),
    upsert: (input: Partial<Category> & { householdId: string; name: string; type: string }): Promise<Category> =>
      invoke("categories_upsert", { input }),
    delete: (id: string): Promise<void> => invoke("categories_delete", { id }),
  },
  operations: {
    list: (householdId: string, limit = 500, offset = 0): Promise<Transaction[]> =>
      invoke("operations_list", { householdId, limit, offset }),
    get: (id: string): Promise<OperationDetail | null> =>
      invoke("operations_get", { id }),
    upsert: (input: any): Promise<OperationDetail> =>
      invoke("operations_upsert", { input }),
    delete: (id: string): Promise<void> => invoke("operations_delete", { id }),
    itemNames: (householdId: string): Promise<string[]> =>
      invoke("operations_item_names", { householdId }),
    lastPrice: (householdId: string, name: string): Promise<number | null> =>
      invoke("operations_last_price", { householdId, name }),
  },
  budgets: {
    list: (householdId: string): Promise<BudgetProgress[]> =>
      invoke("budgets_list", { householdId }),
    upsert: (input: any): Promise<Budget> => invoke("budgets_upsert", { input }),
    delete: (id: string): Promise<void> => invoke("budgets_delete", { id }),
  },
  recurring: {
    list: (householdId: string): Promise<RecurringRule[]> =>
      invoke("recurring_list", { householdId }),
    upsert: (input: any): Promise<RecurringRule> =>
      invoke("recurring_upsert", { input }),
    delete: (id: string): Promise<void> => invoke("recurring_delete", { id }),
  },
  reports: {
    summary: (range: ReportRange): Promise<ReportSummary> =>
      invoke("reports_summary", { range }),
    monthly: (range: ReportRange): Promise<MonthlyPoint[]> =>
      invoke("reports_monthly", { range }),
    topCategories: (range: ReportRange, limit = 20): Promise<CategorySpend[]> =>
      invoke("reports_top_categories", { range, limit }),
  },
  sync: {
    login: (
      email: string,
      password: string,
      projectId: string,
      apiKey: string,
      householdId: string,
    ): Promise<SyncStatus> =>
      invoke("sync_login", { email, password, projectId, apiKey, householdId }),
    now: (): Promise<SyncResult> => invoke("sync_now"),
    status: (): Promise<SyncStatus> => invoke("sync_status"),
    logout: (): Promise<void> => invoke("sync_logout"),
  },
  export: {
    transactionsCsv: (householdId: string, fromMs: number, toMs: number): Promise<string | null> =>
      invoke("export_transactions_csv", { householdId, fromMs, toMs }),
    transactionsXlsx: (householdId: string, fromMs: number, toMs: number): Promise<string | null> =>
      invoke("export_transactions_xlsx", { householdId, fromMs, toMs }),
    backupJson: (householdId: string): Promise<string | null> =>
      invoke("export_backup_json", { householdId }),
    importJson: (householdId: string): Promise<ImportResult | null> =>
      invoke("import_backup_json", { householdId }),
  },
};
