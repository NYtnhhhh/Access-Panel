// ---------- МУЛЬТИЯЗЫЧНОСТЬ ----------
//
// RU / EN. Все тексты — через функцию t().
// Хранение выбранного языка — в lang.txt.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lang {
    Ru,
    En,
}

impl Lang {
    pub fn code(&self) -> &'static str {
        match self {
            Lang::Ru => "ru",
            Lang::En => "en",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Lang::Ru => "RU",
            Lang::En => "EN",
        }
    }
}

const LANG_FILE: &str = "lang.txt";

/// Читает язык из файла. Если нет — RU.
pub fn load_lang() -> Lang {
    match std::fs::read_to_string(LANG_FILE) {
        Ok(s) => match s.trim() {
            "en" => Lang::En,
            _ => Lang::Ru,
        },
        Err(_) => Lang::Ru,
    }
}

/// Сохраняет язык в файл.
pub fn save_lang(lang: Lang) -> Result<(), String> {
    std::fs::write(LANG_FILE, lang.code())
        .map_err(|e| format!("Запись lang.txt: {}", e))
}

/// Перевод по ключу.
pub fn t(key: &str, lang: Lang) -> &'static str {
    match (key, lang) {
        // ---------- ОБЩИЕ ----------
        ("app_title", Lang::Ru) => "Панель Доступа",
        ("app_title", Lang::En) => "Access Panel",

        ("settings", Lang::Ru) => "Настройки",
        ("settings", Lang::En) => "Settings",

        ("theme", Lang::Ru) => "Тема",
        ("theme", Lang::En) => "Theme",

        ("theme_dark", Lang::Ru) => "Тёмная",
        ("theme_dark", Lang::En) => "Dark",

        ("theme_light", Lang::Ru) => "Светлая",
        ("theme_light", Lang::En) => "Light",

        ("language", Lang::Ru) => "Язык",
        ("language", Lang::En) => "Language",

        // ---------- ПОИСК ----------
        ("search_hint", Lang::Ru) => "Введите ник игрока...",
        ("search_hint", Lang::En) => "Enter player name...",

        ("search", Lang::Ru) => "Найти",
        ("search", Lang::En) => "Search",

        ("refresh", Lang::Ru) => "Обновить",
        ("refresh", Lang::En) => "Refresh",

        ("enter_nick", Lang::Ru) => "Введите ник игрока",
        ("enter_nick", Lang::En) => "Enter player name",

        ("no_connection", Lang::Ru) => "Нет подключения к базе",
        ("no_connection", Lang::En) => "No database connection",

        ("searching", Lang::Ru) => "Поиск",
        ("searching", Lang::En) => "Searching",

        ("found", Lang::Ru) => "Найдено",
        ("found", Lang::En) => "Found",

        ("ready", Lang::Ru) => "Готов к работе",
        ("ready", Lang::En) => "Ready",

        ("connected", Lang::Ru) => "Подключено",
        ("connected", Lang::En) => "Connected",

        ("disconnected", Lang::Ru) => "Отключено",
        ("disconnected", Lang::En) => "Disconnected",

        ("no_server", Lang::Ru) => "Без сервера",
        ("no_server", Lang::En) => "No server",

        ("empty_results", Lang::Ru) => "Здесь появятся результаты поиска",
        ("empty_results", Lang::En) => "Search results will appear here",

        ("uuid", Lang::Ru) => "UUID",
        ("uuid", Lang::En) => "UUID",

        ("nickname", Lang::Ru) => "НИКНЕЙМ",
        ("nickname", Lang::En) => "USERNAME",

        ("connected_to", Lang::Ru) => "Подключено к",
        ("connected_to", Lang::En) => "Connected to",

        // ---------- КАРТОЧКА ----------
        ("player_card", Lang::Ru) => "Карточка игрока",
        ("player_card", Lang::En) => "Player Card",

        ("group", Lang::Ru) => "Группа / Донат",
        ("group", Lang::En) => "Group / Donate",

        ("last_login", Lang::Ru) => "Последний вход",
        ("last_login", Lang::En) => "Last Login",

        ("registration", Lang::Ru) => "Регистрация",
        ("registration", Lang::En) => "Registered",

        ("issue_punishment", Lang::Ru) => "Выдать наказание",
        ("issue_punishment", Lang::En) => "Issue Punishment",

        ("reason", Lang::Ru) => "Причина:",
        ("reason", Lang::En) => "Reason:",

        ("reason_hint", Lang::Ru) => "Укажите причину",
        ("reason_hint", Lang::En) => "Enter reason",

        ("duration", Lang::Ru) => "Длительность:",
        ("duration", Lang::En) => "Duration:",

        ("minutes", Lang::Ru) => "мин.",
        ("minutes", Lang::En) => "min.",

        ("minutes_forever", Lang::Ru) => "мин.  (0 = навсегда)",
        ("minutes_forever", Lang::En) => "min.  (0 = forever)",

        ("active", Lang::Ru) => "Активные",
        ("active", Lang::En) => "Active",

        ("history", Lang::Ru) => "История",
        ("history", Lang::En) => "History",

        ("no_active", Lang::Ru) => "Активных наказаний нет",
        ("no_active", Lang::En) => "No active punishments",

        ("no_history", Lang::Ru) => "История пуста",
        ("no_history", Lang::En) => "History is empty",

        ("loading", Lang::Ru) => "Загрузка данных...",
        ("loading", Lang::En) => "Loading...",

        ("remove", Lang::Ru) => "Снять",
        ("remove", Lang::En) => "Remove",

        ("enter_reason", Lang::Ru) => "Укажите причину",
        ("enter_reason", Lang::En) => "Enter reason",

        ("issuing", Lang::Ru) => "Выдача",
        ("issuing", Lang::En) => "Issuing",

        ("for", Lang::Ru) => "для",
        ("for", Lang::En) => "for",

        ("removing_punish", Lang::Ru) => "Снятие наказания...",
        ("removing_punish", Lang::En) => "Removing punishment...",

        // ---------- WELCOME ----------
        ("welcome", Lang::Ru) => "Добро пожаловать!",
        ("welcome", Lang::En) => "Welcome!",

        ("welcome_desc", Lang::Ru) => "Введите данные вашего сервера для начала работы.",
        ("welcome_desc", Lang::En) => "Enter your server details to get started.",

        ("server_name", Lang::Ru) => "Название сервера",
        ("server_name", Lang::En) => "Server Name",

        ("server_name_hint", Lang::Ru) => "Мой сервер (необязательно)",
        ("server_name_hint", Lang::En) => "My server (optional)",

        ("host", Lang::Ru) => "Хост",
        ("host", Lang::En) => "Host",

        ("port", Lang::Ru) => "Порт",
        ("port", Lang::En) => "Port",

        ("user", Lang::Ru) => "Юзер",
        ("user", Lang::En) => "User",

        ("password", Lang::Ru) => "Пароль",
        ("password", Lang::En) => "Password",

        ("database", Lang::Ru) => "База",
        ("database", Lang::En) => "Database",

        ("remember", Lang::Ru) => "Запомнить данные",
        ("remember", Lang::En) => "Remember data",

        ("connect", Lang::Ru) => "Подключиться",
        ("connect", Lang::En) => "Connect",

        ("passwords_warning", Lang::Ru) =>
            "Файл servers.toml содержит пароли. Не передавайте его третьим лицам.",
        ("passwords_warning", Lang::En) =>
            "servers.toml contains passwords. Do not share it with third parties.",

        // ---------- ПАНЕЛЬ ----------
        ("servers_list", Lang::Ru) => "Список серверов",
        ("servers_list", Lang::En) => "Servers List",

        ("no_servers", Lang::Ru) => "Нет серверов",
        ("no_servers", Lang::En) => "No servers",

        ("add_server", Lang::Ru) => "+  Добавить сервер",
        ("add_server", Lang::En) => "+  Add Server",

        ("logout", Lang::Ru) => "Выйти из системы",
        ("logout", Lang::En) => "Log Out",

        // ---------- ЛОГАУТ ----------
        ("logout_confirm", Lang::Ru) => "Выйти из системы?",
        ("logout_confirm", Lang::En) => "Log out?",

        ("logout_question", Lang::Ru) => "Вы действительно хотите выйти?",
        ("logout_question", Lang::En) => "Are you sure you want to log out?",

        ("logout_warning", Lang::Ru) =>
            "Все сохранённые серверы будут удалены. Это действие нельзя отменить.",
        ("logout_warning", Lang::En) =>
            "All saved servers will be deleted. This action cannot be undone.",

        ("yes_logout", Lang::Ru) => "Да, выйти",
        ("yes_logout", Lang::En) => "Yes, log out",

        ("cancel", Lang::Ru) => "Отмена",
        ("cancel", Lang::En) => "Cancel",

        // ---------- МАКРОСЫ ----------
        ("macros", Lang::Ru) => "Кнопки-макросы",
        ("macros", Lang::En) => "Macros",

        ("macro_add", Lang::Ru) => "+  Добавить",
        ("macro_add", Lang::En) => "+  Add",

        ("macro_new", Lang::Ru) => "Новый макрос",
        ("macro_new", Lang::En) => "New Macro",

        ("macro_edit", Lang::Ru) => "Редактировать макрос",
        ("macro_edit", Lang::En) => "Edit Macro",

        ("macro_name", Lang::Ru) => "Название",
        ("macro_name", Lang::En) => "Name",

        ("macro_name_hint", Lang::Ru) => "Например: Heal",
        ("macro_name_hint", Lang::En) => "Example: Heal",

        ("macro_command", Lang::Ru) => "RCON-команда",
        ("macro_command", Lang::En) => "RCON command",

        ("macro_command_hint", Lang::Ru) => "Например: heal {player}",
        ("macro_command_hint", Lang::En) => "Example: heal {player}",

        ("macro_command_hint2", Lang::Ru) => "{player} заменится на ник игрока из карточки",
        ("macro_command_hint2", Lang::En) => "{player} will be replaced with the player's name",

        ("macro_color", Lang::Ru) => "Цвет",
        ("macro_color", Lang::En) => "Color",

        ("macro_custom", Lang::Ru) => "Свой:",
        ("macro_custom", Lang::En) => "Custom:",

        ("macro_save", Lang::Ru) => "Сохранить",
        ("macro_save", Lang::En) => "Save",

        ("macro_delete", Lang::Ru) => "Удалить",
        ("macro_delete", Lang::En) => "Delete",

        ("macro_fill", Lang::Ru) => "Заполните текст и команду",
        ("macro_fill", Lang::En) => "Fill in name and command",

        ("macro_saved", Lang::Ru) => "Макрос сохранён",
        ("macro_saved", Lang::En) => "Macro saved",

        ("macro_deleted", Lang::Ru) => "Макрос удалён",
        ("macro_deleted", Lang::En) => "Macro deleted",

        ("macro_no_player", Lang::Ru) => "Откройте карточку игрока для этой команды",
        ("macro_no_player", Lang::En) => "Open player card for this command",

        ("macro_no_rcon", Lang::Ru) => "Нет RCON-подключения",
        ("macro_no_rcon", Lang::En) => "No RCON connection",

        ("macro_done", Lang::Ru) => "Макрос",
        ("macro_done", Lang::En) => "Macro",

        // ---------- ОШИБКИ ----------
        ("err_mysql_host", Lang::Ru) => "Введите хост MySQL",
        ("err_mysql_host", Lang::En) => "Enter MySQL host",

        ("err_mysql_user", Lang::Ru) => "Введите пользователя MySQL",
        ("err_mysql_user", Lang::En) => "Enter MySQL user",

        ("err_mysql_db", Lang::Ru) => "Введите имя базы данных",
        ("err_mysql_db", Lang::En) => "Enter database name",

        // fallback
        (_, Lang::Ru) => "—",
        (_, Lang::En) => "—",
    }
}