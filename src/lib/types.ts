export type AccountType = "CARD_DEBIT" | "CARD_CREDIT" | "CASH";
export type CategoryType = "INCOME" | "EXPENSE";
export type TransactionType = "INCOME" | "EXPENSE" | "TRANSFER";
export type SourceType = "MANUAL" | "SMS" | "QR" | "IMPORT";
export type BudgetPeriod = "WEEK" | "MONTH" | "YEAR";

export interface Account {
  id: string;
  householdId: string;
  name: string;
  type: AccountType;
  currency: string;
  balanceMinor: number;
  creditLimitMinor: number | null;
  gracePeriodDays: number | null;
  paymentDueDay: number | null;
  color: number;
  iconKey: string;
  isArchived: boolean;
  updatedAt: number;
}

export interface Category {
  id: string;
  householdId: string;
  name: string;
  type: CategoryType;
  parentId: string | null;
  color: number;
  iconKey: string;
  sortOrder: number;
  updatedAt: number;
  isDeleted: boolean;
}

export interface Transaction {
  id: string;
  householdId: string;
  occurredAt: number;
  type: TransactionType;
  amountMinor: number;
  currency: string;
  accountId: string;
  toAccountId: string | null;
  categoryId: string | null;
  merchantId: string | null;
  note: string | null;
  createdBy: string;
  createdAt: number;
  updatedAt: number;
  sourceType: SourceType;
  receiptId: string | null;
  isDeleted: boolean;
}

export interface ReceiptItem {
  id: string;
  transactionId: string;
  name: string;
  priceMinor: number;
  qty: number;
  unit: string | null;
  fnsRaw: string | null;
}

export interface OperationDetail {
  transaction: Transaction;
  items: ReceiptItem[];
}

export interface Budget {
  id: string;
  householdId: string;
  categoryId: string;
  period: BudgetPeriod;
  limitMinor: number;
  currency: string;
  startDate: number;
  isRolling: boolean;
  updatedAt: number;
  isDeleted: boolean;
}

export interface BudgetProgress {
  budget: Budget;
  spentMinor: number;
  periodStart: number;
  periodEnd: number;
}

export interface RecurringRule {
  id: string;
  householdId: string;
  templateJson: string;
  rrule: string;
  nextRunAt: number;
  lastRunAt: number | null;
  isActive: boolean;
  updatedAt: number;
}

export interface SessionInfo {
  userId: string;
  householdId: string;
}

export interface MonthlyPoint {
  bucket: string;
  incomeMinor: number;
  expenseMinor: number;
}

export interface CategorySpend {
  categoryId: string;
  categoryName: string;
  color: number;
  spentMinor: number;
}

export interface ReportSummary {
  totalIncomeMinor: number;
  totalExpenseMinor: number;
  balanceMinor: number;
}

export interface ReportRange {
  householdId: string;
  startMs: number;
  endMs: number;
}

export const UNITS = ["шт", "кг", "г", "л", "мл", "м", "упак", "пачка", "пара", "рул"] as const;

export interface SyncStatus {
  loggedIn: boolean;
  lastSyncMs: number;
}

export interface SyncResult {
  uploaded: number;
  downloaded: number;
}

export interface ImportResult {
  transactions: number;
  accounts: number;
  categories: number;
  budgets: number;
}
