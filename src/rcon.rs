// ---------- RCON ----------
//
// Клиент для отправки команд на Minecraft-сервер через RCON.
// Использует синхронный крейт mc-rcon.

use mc_rcon::RconClient;

/// Настройки подключения к RCON.
#[derive(Clone, Debug)]
pub struct RconConfig {
    pub host: String,
    pub port: u16,
    pub password: String,
}

impl RconConfig {
    /// Создаёт конфиг с параметрами.
    pub fn new(host: String, port: u16, password: String) -> Self {
        Self { host, port, password }
    }
}

/// Отправляет команду на сервер через RCON.
/// Возвращает ответ сервера или ошибку.
pub fn send_command(cfg: &RconConfig, command: &str) -> Result<String, String> {
    let addr = format!("{}:{}", cfg.host, cfg.port);

    // 1. Подключаемся
    let client = RconClient::connect(&addr)
        .map_err(|e| format!("RCON подключение: {}", e))?;

    // 2. Авторизуемся
    client.log_in(&cfg.password)
        .map_err(|e| format!("RCON авторизация: {}", e))?;

    // 3. Отправляем команду
    client.send_command(command)
        .map_err(|e| format!("RCON команда: {}", e))
}

/// Проверяет подключение к RCON.
pub fn test_connection(cfg: &RconConfig) -> Result<(), String> {
    let addr = format!("{}:{}", cfg.host, cfg.port);

    let client = RconClient::connect(&addr)
        .map_err(|e| format!("RCON недоступен: {}", e))?;

    client.log_in(&cfg.password)
        .map_err(|e| format!("RCON неверный пароль: {}", e))?;

    Ok(())
}