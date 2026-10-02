// ---------- БАЗА ДАННЫХ ----------
//
// Всё, что связано с MySQL:
// - создание пула соединений
// - фоновые запросы (поиск, карточка, наказания)

use mysql::prelude::*;
use mysql::*;
use r2d2::Pool;
use r2d2_mysql::MySqlConnectionManager;
use std::sync::mpsc::Sender;

use crate::models::{BgMessage, MyPool, Player, PlayerInfo, Punishment};
use crate::servers::MysqlConfig;
use crate::utils::{format_ts, now_millis};

// ---------- СОЗДАНИЕ ПУЛА ----------

/// Создаёт пул соединений MySQL по переданной конфигурации.
/// Вызывается при выборе/переключении сервера.
pub fn create_pool_for(cfg: &MysqlConfig) -> Result<MyPool, String> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(cfg.host.as_str()))
        .tcp_port(cfg.port)
        .user(Some(cfg.user.as_str()))
        .pass(Some(cfg.password.as_str()))
        .db_name(Some(cfg.database.as_str()));

    let manager = MySqlConnectionManager::new(opts);
    Pool::builder()
        .max_size(5)
        .build(manager)
        .map_err(|e| format!("Ошибка создания пула: {}", e))
}

// ---------- ФОНОВЫЕ ФУНКЦИИ ----------

/// Поиск игроков по части ника.
/// Результат отправляется в канал (tx), а главный поток его обрабатывает.
pub fn bg_search(pool: MyPool, query: String, tx: Sender<BgMessage>) {
    let result = (|| -> Result<Vec<Player>, mysql::Error> {
        let mut conn = pool.get().unwrap();
        let stmt = format!(
            "SELECT uuid, username FROM luckperms_players \
             WHERE username LIKE '%{}%' LIMIT 200",
            query.replace('\'', "''")
        );
        let rows: Vec<(String, String)> = conn.query(stmt)?;
        Ok(rows.into_iter().map(|(uuid, username)| Player { uuid, username }).collect())
    })();

    match result {
        Ok(players) => { let _ = tx.send(BgMessage::SearchDone(players)); }
        Err(e) => { let _ = tx.send(BgMessage::Error(format!("Поиск: {}", e))); }
    }
}

/// Загружает карточку игрока:
/// - группа из LuckPerms
/// - даты входа/регистрации из AuthMe
/// - активные и завершённые наказания из AdvancedBan
pub fn bg_info(pool: MyPool, uuid: String, username: String, tx: Sender<BgMessage>) {
    let mut info = PlayerInfo {
        group: "—".into(),
        lastlogin: "—".into(),
        regdate: "—".into(),
        active_punishments: vec![],
        history_punishments: vec![],
    };

    let result = (|| -> Result<(), mysql::Error> {
        let mut conn = pool.get().unwrap();

        // Группа из LuckPerms
        let group: Option<String> = conn
            .exec_first("SELECT primary_group FROM luckperms_players WHERE uuid = ?", (&uuid,))
            .unwrap_or(None);
        if let Some(g) = group {
            if !g.is_empty() { info.group = g; }
        }

        // Даты из AuthMe
        let row: Option<(Option<i64>, Option<i64>)> = conn
            .exec_first("SELECT lastlogin, regdate FROM authme WHERE username = ?", (&username,))
            .unwrap_or(None);
        if let Some((ll, rd)) = row {
            if let Some(ts) = ll { info.lastlogin = format_ts(ts); }
            if let Some(ts) = rd { info.regdate = format_ts(ts); }
        }

        let now = now_millis();

        // Активные наказания
        let active_rows: Vec<(i64, String, String, i64, i64)> = conn
            .exec(
                "SELECT id, punishmentType, reason, start, end FROM Punishments \
                 WHERE uuid = ? ORDER BY start DESC",
                (&uuid,),
            )
            .unwrap_or_default();
        for (id, ptype, reason, start, end) in active_rows {
            info.active_punishments.push(Punishment {
                id, ptype, reason,
                start: format_ts(start),
                end: if end == 0 { "Навсегда".into() } else { format_ts(end) },
                active: true,
            });
        }

        // История наказаний
        let history_rows: Vec<(i64, String, String, i64, i64)> = conn
            .exec(
                "SELECT id, punishmentType, reason, start, end FROM PunishmentHistory \
                 WHERE uuid = ? ORDER BY start DESC LIMIT 200",
                (&uuid,),
            )
            .unwrap_or_default();
        for (id, ptype, reason, start, end) in history_rows {
            let is_active = end == 0 || end > now;
            info.history_punishments.push(Punishment {
                id, ptype, reason,
                start: format_ts(start),
                end: if end == 0 { "Навсегда".into() } else { format_ts(end) },
                active: is_active,
            });
        }
        Ok(())
    })();

    if let Err(e) = result {
        let _ = tx.send(BgMessage::Error(format!("Карточка: {}", e)));
        return;
    }
    let _ = tx.send(BgMessage::InfoDone(info));
}

/// Выдаёт наказание.
/// Пишет и в Punishments (активное), и в PunishmentHistory (история).
pub fn bg_punish(
    pool: MyPool,
    uuid: String,
    username: String,
    ptype: String,
    reason: String,
    duration_minutes: i64,
    tx: Sender<BgMessage>,
) {
    let result = (|| -> Result<(), mysql::Error> {
        let mut conn = pool.get().unwrap();

        let start = now_millis();
        let end = if duration_minutes <= 0 { 0 } else { start + duration_minutes * 60 * 1000 };
        let operator = "Console";

        conn.exec_drop(
            "INSERT INTO Punishments \
             (name, uuid, reason, operator, punishmentType, start, end, calculated) \
             VALUES (?, ?, ?, ?, ?, ?, ?, 1)",
            (&username, &uuid, &reason, operator, &ptype, start, end),
        )?;

        conn.exec_drop(
            "INSERT INTO PunishmentHistory \
             (name, uuid, reason, operator, punishmentType, start, end, calculated) \
             VALUES (?, ?, ?, ?, ?, ?, ?, 1)",
            (&username, &uuid, &reason, operator, &ptype, start, end),
        )?;

        Ok(())
    })();

    match result {
        Ok(_) => { let _ = tx.send(BgMessage::PunishDone(format!("Выдано: {}", ptype))); }
        Err(e) => { let _ = tx.send(BgMessage::Error(format!("Ошибка наказания: {}", e))); }
    }
}

/// Снимает активное наказание.
/// Удаляет из Punishments, но оставляет в PunishmentHistory.
pub fn bg_unpunish(pool: MyPool, punishment_id: i64, tx: Sender<BgMessage>) {
    let result = (|| -> Result<(), mysql::Error> {
        let mut conn = pool.get().unwrap();
        conn.exec_drop("DELETE FROM Punishments WHERE id = ?", (punishment_id,))?;
        Ok(())
    })();

    match result {
        Ok(_) => { let _ = tx.send(BgMessage::PunishDone("Наказание снято".into())); }
        Err(e) => { let _ = tx.send(BgMessage::Error(format!("Ошибка снятия: {}", e))); }
    }
}