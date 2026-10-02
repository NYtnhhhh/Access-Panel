// ---------- МОДЕЛИ ДАННЫХ ----------
//
// Здесь все структуры и типы, которые используются в проекте.
// Никакой логики — только определения.

use r2d2::Pool;
use r2d2_mysql::MySqlConnectionManager;

// ---------- ТИП ПУЛА СОЕДИНЕНИЙ ----------

/// Пул соединений MySQL. Один на всё приложение.
pub type MyPool = Pool<MySqlConnectionManager>;

// ---------- ИГРОК ----------

/// Игрок сервера — короткая запись (UUID + ник).
#[derive(Clone)]
pub struct Player {
    pub uuid: String,
    pub username: String,
}

// ---------- ИНФОРМАЦИЯ ОБ ИГРОКЕ ----------

/// Полная информация об игроке (карточка).
#[derive(Clone, Default)]
pub struct PlayerInfo {
    pub group: String,
    pub lastlogin: String,
    pub regdate: String,
    pub active_punishments: Vec<Punishment>,
    pub history_punishments: Vec<Punishment>,
}

// ---------- НАКАЗАНИЕ ----------

/// Одно наказание игрока.
#[derive(Clone)]
pub struct Punishment {
    pub id: i64,
    pub ptype: String,
    pub reason: String,
    pub start: String,
    pub end: String,
    pub active: bool,
}

// ---------- СООБЩЕНИЯ ИЗ ФОНОВЫХ ПОТОКОВ ----------

/// Сообщение от фонового потока в главный.
/// Фоновые потоки не могут напрямую менять UI — они отправляют
/// эти сообщения через канал, а главный поток их обрабатывает.
pub enum BgMessage {
    /// Поиск завершён — список игроков.
    SearchDone(Vec<Player>),
    /// Карточка игрока загружена.
    InfoDone(PlayerInfo),
    /// Ошибка в фоне.
    Error(String),
    /// Наказание выдано или снято.
    PunishDone(String),
}

// ---------- СОРТИРОВКА ----------

/// По какому столбцу сортируем.
#[derive(Clone, Copy, PartialEq)]
pub enum SortColumn {
    Uuid,
    Username,
}

/// Направление сортировки.
#[derive(Clone, Copy, PartialEq)]
pub enum SortDir {
    Asc,
    Desc,
}