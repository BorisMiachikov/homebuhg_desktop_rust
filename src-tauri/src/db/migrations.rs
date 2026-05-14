use crate::error::AppResult;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};

pub fn run(conn: &mut Connection) -> AppResult<()> {
    let migrations = Migrations::new(vec![M::up(V1)]);
    migrations.to_latest(conn)?;
    Ok(())
}

const V1: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    uid TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL,
    email TEXT NOT NULL,
    photo_url TEXT
);

CREATE TABLE IF NOT EXISTS households (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner_uid TEXT NOT NULL,
    base_currency TEXT NOT NULL DEFAULT 'RUB'
);

CREATE TABLE IF NOT EXISTS household_members (
    household_id TEXT NOT NULL,
    user_uid TEXT NOT NULL,
    role TEXT NOT NULL,
    joined_at INTEGER NOT NULL,
    PRIMARY KEY (household_id, user_uid)
);

CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    currency TEXT NOT NULL DEFAULT 'RUB',
    balance_minor INTEGER NOT NULL DEFAULT 0,
    credit_limit_minor INTEGER,
    grace_period_days INTEGER,
    payment_due_day INTEGER,
    color INTEGER NOT NULL,
    icon_key TEXT NOT NULL,
    is_archived INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_accounts_hid ON accounts(household_id);
CREATE INDEX IF NOT EXISTS idx_accounts_hid_updated ON accounts(household_id, updated_at);

CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    parent_id TEXT,
    color INTEGER NOT NULL,
    icon_key TEXT NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL,
    is_deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_categories_hid ON categories(household_id);
CREATE INDEX IF NOT EXISTS idx_categories_parent ON categories(parent_id);
CREATE INDEX IF NOT EXISTS idx_categories_hid_updated ON categories(household_id, updated_at);

CREATE TABLE IF NOT EXISTS merchants (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    name TEXT NOT NULL,
    default_category_id TEXT,
    last_used_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    is_deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_merchants_hid ON merchants(household_id);

CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    occurred_at INTEGER NOT NULL,
    type TEXT NOT NULL,
    amount_minor INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'RUB',
    account_id TEXT NOT NULL,
    to_account_id TEXT,
    category_id TEXT,
    merchant_id TEXT,
    note TEXT,
    created_by TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    source_type TEXT NOT NULL DEFAULT 'MANUAL',
    receipt_id TEXT,
    is_deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_tx_hid ON transactions(household_id);
CREATE INDEX IF NOT EXISTS idx_tx_occurred ON transactions(occurred_at);
CREATE INDEX IF NOT EXISTS idx_tx_account ON transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_tx_category ON transactions(category_id);
CREATE INDEX IF NOT EXISTS idx_tx_hid_updated ON transactions(household_id, updated_at);

CREATE TABLE IF NOT EXISTS receipt_items (
    id TEXT PRIMARY KEY NOT NULL,
    transaction_id TEXT NOT NULL,
    name TEXT NOT NULL,
    price_minor INTEGER NOT NULL,
    qty REAL NOT NULL,
    unit TEXT,
    fns_raw TEXT
);
CREATE INDEX IF NOT EXISTS idx_ritems_tx ON receipt_items(transaction_id);

CREATE TABLE IF NOT EXISTS budgets (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    category_id TEXT NOT NULL,
    period TEXT NOT NULL,
    limit_minor INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'RUB',
    start_date INTEGER NOT NULL,
    is_rolling INTEGER NOT NULL DEFAULT 0,
    updated_at INTEGER NOT NULL,
    is_deleted INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_budgets_hid ON budgets(household_id);
CREATE INDEX IF NOT EXISTS idx_budgets_cat ON budgets(category_id);
CREATE INDEX IF NOT EXISTS idx_budgets_hid_updated ON budgets(household_id, updated_at);

CREATE TABLE IF NOT EXISTS recurring_rules (
    id TEXT PRIMARY KEY NOT NULL,
    household_id TEXT NOT NULL,
    template_json TEXT NOT NULL,
    rrule TEXT NOT NULL,
    next_run_at INTEGER NOT NULL,
    last_run_at INTEGER,
    is_active INTEGER NOT NULL DEFAULT 1,
    updated_at INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_rrules_hid ON recurring_rules(household_id);
"#;
