use crate::db::DbState;
use crate::error::AppResult;
use crate::models::category::{Category, TYPE_EXPENSE, TYPE_INCOME};
use crate::models::household::{Household, HouseholdMember, User, ROLE_OWNER};
use crate::repository::{categories, households, now_ms};
use rusqlite::params;
use uuid::Uuid;

struct CategorySeed {
    name: &'static str,
    icon: &'static str,
    color: i64,
}

const EXPENSE_SEEDS: &[CategorySeed] = &[
    CategorySeed { name: "Продукты", icon: "shopping_cart", color: 0xFF4CAF50 },
    CategorySeed { name: "Транспорт", icon: "directions_car", color: 0xFF2196F3 },
    CategorySeed { name: "Кафе и рестораны", icon: "restaurant", color: 0xFFFF9800 },
    CategorySeed { name: "ЖКХ", icon: "home", color: 0xFF9C27B0 },
    CategorySeed { name: "Здоровье", icon: "local_hospital", color: 0xFFF44336 },
    CategorySeed { name: "Развлечения", icon: "theaters", color: 0xFFE91E63 },
    CategorySeed { name: "Одежда", icon: "checkroom", color: 0xFF00BCD4 },
    CategorySeed { name: "Связь", icon: "phone", color: 0xFF607D8B },
    CategorySeed { name: "Прочее", icon: "more_horiz", color: 0xFF9E9E9E },
];

const INCOME_SEEDS: &[CategorySeed] = &[
    CategorySeed { name: "Зарплата", icon: "work", color: 0xFF4CAF50 },
    CategorySeed { name: "Фриланс", icon: "laptop", color: 0xFF2196F3 },
    CategorySeed { name: "Кэшбэк", icon: "card_giftcard", color: 0xFFFF9800 },
    CategorySeed { name: "Прочее", icon: "more_horiz", color: 0xFF9E9E9E },
];

pub fn current_household_id(db: &DbState) -> AppResult<Option<String>> {
    let conn = db.lock();
    let mut stmt = conn.prepare("SELECT id FROM households LIMIT 1")?;
    let mut rows = stmt.query_map(params![], |row| row.get::<_, String>(0))?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn current_user_id(db: &DbState) -> AppResult<Option<String>> {
    let conn = db.lock();
    let mut stmt = conn.prepare(
        "SELECT owner_uid FROM households LIMIT 1",
    )?;
    let mut rows = stmt.query_map(params![], |row| row.get::<_, String>(0))?;
    if let Some(r) = rows.next() {
        return Ok(Some(r?));
    }
    Ok(None)
}

pub fn ensure_local_session(db: &DbState) -> AppResult<(String, String)> {
    if let Some(hid) = current_household_id(db)? {
        let uid = current_user_id(db)?.unwrap_or_else(|| "local".to_string());
        return Ok((uid, hid));
    }
    let conn = db.lock();
    let uid = "local".to_string();
    let hid = Uuid::new_v4().to_string();
    let now = now_ms();

    households::upsert_user(
        &conn,
        &User {
            uid: uid.clone(),
            display_name: "Я".into(),
            email: "".into(),
            photo_url: None,
        },
    )?;
    households::upsert_household(
        &conn,
        &Household {
            id: hid.clone(),
            name: "Мой кошелёк".into(),
            owner_uid: uid.clone(),
            base_currency: "RUB".into(),
        },
    )?;
    households::upsert_member(
        &conn,
        &HouseholdMember {
            household_id: hid.clone(),
            user_uid: uid.clone(),
            role: ROLE_OWNER.into(),
            joined_at: now,
        },
    )?;

    for (i, seed) in EXPENSE_SEEDS.iter().enumerate() {
        categories::upsert(
            &conn,
            &Category {
                id: Uuid::new_v4().to_string(),
                household_id: hid.clone(),
                name: seed.name.into(),
                category_type: TYPE_EXPENSE.into(),
                parent_id: None,
                color: seed.color,
                icon_key: seed.icon.into(),
                sort_order: i as i32,
                updated_at: now,
                is_deleted: false,
            },
        )?;
    }
    for (i, seed) in INCOME_SEEDS.iter().enumerate() {
        categories::upsert(
            &conn,
            &Category {
                id: Uuid::new_v4().to_string(),
                household_id: hid.clone(),
                name: seed.name.into(),
                category_type: TYPE_INCOME.into(),
                parent_id: None,
                color: seed.color,
                icon_key: seed.icon.into(),
                sort_order: i as i32,
                updated_at: now,
                is_deleted: false,
            },
        )?;
    }

    Ok((uid, hid))
}
