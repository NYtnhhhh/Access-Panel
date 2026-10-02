// ---------- СЕРВЕРЫ ----------
//
// Модель сервера + список серверов.
// Хранится в файле servers.toml рядом с .exe.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// ---------- MYSQL ----------

/// Данные подключения к MySQL.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MysqlConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl Default for MysqlConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 3306,
            user: String::new(),
            password: String::new(),
            database: String::new(),
        }
    }
}

// ---------- RCON ----------

/// Данные подключения к RCON.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RconData {
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl Default for RconData {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 25575,
            password: String::new(),
        }
    }
}

// ---------- СЕРВЕР ----------

/// Один сервер.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub id: String,
    pub name: String,
    pub remember: bool,
    pub mysql: MysqlConfig,
    pub rcon: RconData,
}

// ---------- СПИСОК СЕРВЕРОВ ----------

/// Список всех серверов + активный.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ServerList {
    pub active_id: Option<String>,
    pub servers: Vec<Server>,
}

impl ServerList {
    /// Загружает список из `servers.toml`.
    /// Если файла нет — возвращает пустой список.
    pub fn load() -> Self {
        let path = Path::new("servers.toml");
        if !path.exists() {
            return Self::default();
        }

        match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Сохраняет список в `servers.toml`.
    pub fn save(&self) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("TOML сериализация: {}", e))?;

        fs::write("servers.toml", content)
            .map_err(|e| format!("Запись servers.toml: {}", e))?;

        Ok(())
    }

    /// Добавляет новый сервер. Возвращает id.
    pub fn add(&mut self, mut server: Server) -> String {
        // Генерируем id, если его нет
        if server.id.is_empty() {
            let n = self.servers.len() + 1;
            server.id = format!("server_{}", n);
        }

        // Авто-имя, если пустое
        if server.name.trim().is_empty() {
            let n = self.servers.len() + 1;
            server.name = format!("Сервер {}", n);
        }

        let id = server.id.clone();

        // Если это первый сервер — делаем активным
        if self.servers.is_empty() {
            self.active_id = Some(id.clone());
        }

        self.servers.push(server);
        id
    }

    /// Удаляет сервер по id.
    pub fn remove(&mut self, id: &str) {
        self.servers.retain(|s| s.id != id);

        // Если удалили активный — переключаемся на первый доступный
        if self.active_id.as_deref() == Some(id) {
            self.active_id = self.servers.first().map(|s| s.id.clone());
        }
    }

    /// Устанавливает активный сервер.
    pub fn set_active(&mut self, id: &str) {
        if self.servers.iter().any(|s| s.id == id) {
            self.active_id = Some(id.to_string());
        }
    }

    /// Возвращает активный сервер.
    pub fn active(&self) -> Option<&Server> {
        let id = self.active_id.as_ref()?;
        self.servers.iter().find(|s| &s.id == id)
    }

    /// Есть ли серверы вообще.
    pub fn is_empty(&self) -> bool {
        self.servers.is_empty()
    }
}