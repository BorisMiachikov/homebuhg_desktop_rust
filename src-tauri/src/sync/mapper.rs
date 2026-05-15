use serde_json::{Map, Value, json};

use crate::error::{AppError, AppResult};
use crate::models::{
    account::Account,
    budget::Budget,
    category::Category,
    transaction::Transaction,
};

type Fields = Map<String, Value>;

// ─── Helpers: Rust → Firestore value wrappers ────────────────────────────────

fn str_val(s: &str) -> Value {
    json!({"stringValue": s})
}

fn int_val(n: i64) -> Value {
    json!({"integerValue": n.to_string()})
}

fn int_val_i32(n: i32) -> Value {
    json!({"integerValue": n.to_string()})
}

fn bool_val(b: bool) -> Value {
    json!({"booleanValue": b})
}

fn null_val() -> Value {
    json!({"nullValue": null})
}

fn opt_str(s: &Option<String>) -> Value {
    match s {
        Some(v) => str_val(v),
        None => null_val(),
    }
}

fn opt_int(n: &Option<i64>) -> Value {
    match n {
        Some(v) => int_val(*v),
        None => null_val(),
    }
}

fn opt_int_i32(n: &Option<i32>) -> Value {
    match n {
        Some(v) => int_val_i32(*v),
        None => null_val(),
    }
}

// ─── Helpers: Firestore value wrappers → Rust ────────────────────────────────

fn get_str(f: &Fields, key: &str) -> AppResult<String> {
    f.get(key)
        .and_then(|v| v["stringValue"].as_str())
        .map(|s| s.to_owned())
        .ok_or_else(|| AppError::InvalidArg(format!("missing string field: {key}")))
}

fn get_int(f: &Fields, key: &str) -> AppResult<i64> {
    f.get(key)
        .and_then(|v| v["integerValue"].as_str())
        .ok_or_else(|| AppError::InvalidArg(format!("missing int field: {key}")))?
        .parse::<i64>()
        .map_err(|e| AppError::InvalidArg(format!("bad int field {key}: {e}")))
}


fn get_opt_str(f: &Fields, key: &str) -> Option<String> {
    let v = f.get(key)?;
    if v.get("nullValue").is_some() {
        return None;
    }
    v["stringValue"].as_str().map(|s| s.to_owned())
}

fn get_opt_int(f: &Fields, key: &str) -> Option<i64> {
    let v = f.get(key)?;
    if v.get("nullValue").is_some() {
        return None;
    }
    v["integerValue"].as_str()?.parse::<i64>().ok()
}

fn get_opt_int_i32(f: &Fields, key: &str) -> Option<i32> {
    let v = f.get(key)?;
    if v.get("nullValue").is_some() {
        return None;
    }
    v["integerValue"].as_str()?.parse::<i32>().ok()
}

// ─── Transaction ─────────────────────────────────────────────────────────────

pub fn transaction_to_fields(t: &Transaction) -> Fields {
    let mut m = Map::new();
    m.insert("id".into(), str_val(&t.id));
    m.insert("householdId".into(), str_val(&t.household_id));
    m.insert("occurredAt".into(), int_val(t.occurred_at));
    m.insert("type".into(), str_val(&t.tx_type));
    m.insert("amountMinor".into(), int_val(t.amount_minor));
    m.insert("currency".into(), str_val(&t.currency));
    m.insert("accountId".into(), str_val(&t.account_id));
    m.insert("toAccountId".into(), opt_str(&t.to_account_id));
    m.insert("categoryId".into(), opt_str(&t.category_id));
    m.insert("merchantId".into(), opt_str(&t.merchant_id));
    m.insert("note".into(), opt_str(&t.note));
    m.insert("createdBy".into(), str_val(&t.created_by));
    m.insert("createdAt".into(), int_val(t.created_at));
    m.insert("updatedAt".into(), int_val(t.updated_at));
    m.insert("sourceType".into(), str_val(&t.source_type));
    m.insert("receiptId".into(), opt_str(&t.receipt_id));
    m.insert("isDeleted".into(), bool_val(t.is_deleted));
    m
}

pub fn fields_to_transaction(f: &Fields) -> AppResult<Transaction> {
    Ok(Transaction {
        id: get_str(f, "id")?,
        household_id: get_str(f, "householdId")?,
        occurred_at: get_int(f, "occurredAt")?,
        tx_type: get_str(f, "type")?,
        amount_minor: get_int(f, "amountMinor")?,
        currency: get_opt_str(f, "currency").unwrap_or_else(|| "RUB".into()),
        account_id: get_str(f, "accountId")?,
        to_account_id: get_opt_str(f, "toAccountId"),
        category_id: get_opt_str(f, "categoryId"),
        merchant_id: get_opt_str(f, "merchantId"),
        note: get_opt_str(f, "note"),
        created_by: get_opt_str(f, "createdBy").unwrap_or_default(),
        created_at: get_int(f, "createdAt")?,
        updated_at: get_int(f, "updatedAt")?,
        source_type: get_opt_str(f, "sourceType").unwrap_or_else(|| "MANUAL".into()),
        receipt_id: get_opt_str(f, "receiptId"),
        is_deleted: f
            .get("isDeleted")
            .and_then(|v| v["booleanValue"].as_bool())
            .unwrap_or(false),
    })
}

// ─── Account ──────────────────────────────────────────────────────────────────

pub fn account_to_fields(a: &Account) -> Fields {
    let mut m = Map::new();
    m.insert("id".into(), str_val(&a.id));
    m.insert("householdId".into(), str_val(&a.household_id));
    m.insert("name".into(), str_val(&a.name));
    m.insert("type".into(), str_val(&a.account_type));
    m.insert("currency".into(), str_val(&a.currency));
    m.insert("balanceMinor".into(), int_val(a.balance_minor));
    m.insert("creditLimitMinor".into(), opt_int(&a.credit_limit_minor));
    m.insert("gracePeriodDays".into(), opt_int_i32(&a.grace_period_days));
    m.insert("paymentDueDay".into(), opt_int_i32(&a.payment_due_day));
    m.insert("color".into(), int_val(a.color));
    m.insert("iconKey".into(), str_val(&a.icon_key));
    m.insert("isArchived".into(), bool_val(a.is_archived));
    m.insert("updatedAt".into(), int_val(a.updated_at));
    m
}

pub fn fields_to_account(f: &Fields) -> AppResult<Account> {
    Ok(Account {
        id: get_str(f, "id")?,
        household_id: get_str(f, "householdId")?,
        name: get_str(f, "name")?,
        account_type: get_str(f, "type")?,
        currency: get_opt_str(f, "currency").unwrap_or_else(|| "RUB".into()),
        balance_minor: get_int(f, "balanceMinor")?,
        credit_limit_minor: get_opt_int(f, "creditLimitMinor"),
        grace_period_days: get_opt_int_i32(f, "gracePeriodDays"),
        payment_due_day: get_opt_int_i32(f, "paymentDueDay"),
        color: get_int(f, "color")?,
        icon_key: get_str(f, "iconKey")?,
        is_archived: f
            .get("isArchived")
            .and_then(|v| v["booleanValue"].as_bool())
            .unwrap_or(false),
        updated_at: get_int(f, "updatedAt")?,
    })
}

// ─── Category ─────────────────────────────────────────────────────────────────

pub fn category_to_fields(c: &Category) -> Fields {
    let mut m = Map::new();
    m.insert("id".into(), str_val(&c.id));
    m.insert("householdId".into(), str_val(&c.household_id));
    m.insert("name".into(), str_val(&c.name));
    m.insert("type".into(), str_val(&c.category_type));
    m.insert("parentId".into(), opt_str(&c.parent_id));
    m.insert("color".into(), int_val(c.color));
    m.insert("iconKey".into(), str_val(&c.icon_key));
    m.insert("sortOrder".into(), int_val_i32(c.sort_order));
    m.insert("updatedAt".into(), int_val(c.updated_at));
    m.insert("isDeleted".into(), bool_val(c.is_deleted));
    m
}

pub fn fields_to_category(f: &Fields) -> AppResult<Category> {
    Ok(Category {
        id: get_str(f, "id")?,
        household_id: get_str(f, "householdId")?,
        name: get_str(f, "name")?,
        category_type: get_str(f, "type")?,
        parent_id: get_opt_str(f, "parentId"),
        color: get_int(f, "color")?,
        icon_key: get_str(f, "iconKey")?,
        sort_order: get_opt_int_i32(f, "sortOrder").unwrap_or(0),
        updated_at: get_int(f, "updatedAt")?,
        is_deleted: f
            .get("isDeleted")
            .and_then(|v| v["booleanValue"].as_bool())
            .unwrap_or(false),
    })
}

// ─── Budget ───────────────────────────────────────────────────────────────────

pub fn budget_to_fields(b: &Budget) -> Fields {
    let mut m = Map::new();
    m.insert("id".into(), str_val(&b.id));
    m.insert("householdId".into(), str_val(&b.household_id));
    m.insert("categoryId".into(), str_val(&b.category_id));
    m.insert("period".into(), str_val(&b.period));
    m.insert("limitMinor".into(), int_val(b.limit_minor));
    m.insert("currency".into(), str_val(&b.currency));
    m.insert("startDate".into(), int_val(b.start_date));
    m.insert("isRolling".into(), bool_val(b.is_rolling));
    m.insert("updatedAt".into(), int_val(b.updated_at));
    m.insert("isDeleted".into(), bool_val(b.is_deleted));
    m
}

pub fn fields_to_budget(f: &Fields) -> AppResult<Budget> {
    Ok(Budget {
        id: get_str(f, "id")?,
        household_id: get_str(f, "householdId")?,
        category_id: get_str(f, "categoryId")?,
        period: get_str(f, "period")?,
        limit_minor: get_int(f, "limitMinor")?,
        currency: get_opt_str(f, "currency").unwrap_or_else(|| "RUB".into()),
        start_date: get_int(f, "startDate")?,
        is_rolling: f
            .get("isRolling")
            .and_then(|v| v["booleanValue"].as_bool())
            .unwrap_or(false),
        updated_at: get_int(f, "updatedAt")?,
        is_deleted: f
            .get("isDeleted")
            .and_then(|v| v["booleanValue"].as_bool())
            .unwrap_or(false),
    })
}
