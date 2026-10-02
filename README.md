<div align="center">

# 🛡️ Access Panel

**Панель администратора для Minecraft-серверов**

Управление игроками, наказаниями и RCON — в одном красивом приложении.

[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)]()
[![Version](https://img.shields.io/badge/Version-0.1.0-success)]()

**🌍 Язык:** [Русский](README.md) · [English](README.en.md)

</div>

---

## ✨ Возможности

<table>
<tr>
<td width="50%">

### 👥 Игроки
- 🔍 Поиск по нику
- 📋 Сортировка по UUID / нику
- 🎫 Карточка игрока: группа, донат, даты
- 💾 Данные из MySQL

</td>
<td width="50%">

### ⚔️ Наказания
- 🚫 BAN / MUTE / KICK / WARN
- ⏱️ Длительность (0 = навсегда)
- 📜 История и активные
- ↩️ Снятие наказания

</td>
</tr>
<tr>
<td width="50%">

### 🔌 RCON
- 📡 Подключение к серверу
- ⚡ Мгновенные команды
- 🎯 Кастомные шаблоны наказаний
- 🔘 Кнопки-макросы

</td>
<td width="50%">

### 🎨 Интерфейс
- 🌙 Тёмная / ☀️ Светлая темы
- 🌍 Русский / English
- ✨ Плавные анимации
- 🖥️ Современный дизайн

</td>
</tr>
</table>

---

## 📸 Скриншоты

<div align="center">

### Главное окно
![Главное окно](docs/screenshots/main.png)

### Карточка игрока
![Карточка игрока](docs/screenshots/card.png)

### Настройки
![Настройки](docs/screenshots/settings.png)

</div>

> 💡 *Скриншоты скоро появятся.*

---

## 🚀 Установка

### Требования

- **Rust** 1.75+ — [установить](https://rustup.rs/)
- **Minecraft-сервер** с включённым **RCON**
- **MySQL** — для хранения данных плагинов

### Сборка

```bash
# Клонируй репозиторий
git clone https://github.com/NYtnhhhh/Access-Panel.git
cd Access-Panel

# Собери релиз
cargo build --release

# Запусти
cargo run --release
```

Готовый `.exe` появится в `target/release/access_panel.exe`.

---

## ⚙️ Настройка

### Первый запуск

При первом запуске откроется **экран «Добро пожаловать»**. Введи:

1. **Название сервера** — как хочешь (для удобства).
2. **MySQL** — данные от базы:
   - Хост (`localhost` или IP хостинга)
   - Порт (`3306` по умолчанию)
   - Юзер, пароль, имя базы
3. **RCON** — данные от сервера:
   - Хост (IP сервера)
   - Порт (`25575` по умолчанию)
   - Пароль

Нажми **«Подключиться»** — данные сохранятся в `servers.toml`.

### Настройка сервера Minecraft

В `server.properties`:

```properties
enable-rcon=true
rcon.port=25575
rcon.password=твой_пароль
```

Перезапусти сервер — RCON готов.

---

## 🎮 Использование

### 🔍 Поиск игрока

1. Введи ник в поле поиска.
2. Нажми **«Найти»** или **Enter**.
3. Кликни по нику → откроется карточка.

### ⚔️ Наказание

1. Открой карточку игрока.
2. Введи **причину** и **длительность**.
3. Нажми **BAN** / **MUTE** / **KICK** / **WARN**.

Команда уйдёт на сервер через RCON и запишется в базу.

### 🔘 Макросы

Под поиском — панель кнопок-макросов:

- **Левый клик** → выполнить команду.
- **Правый клик** → редактировать.
- **«+ Добавить»** → новый макрос.

Плейсхолдер `{player}` подставит ник из открытой карточки.

### 🎨 Смена темы и языка

Открой **⚙ Настройки** (справа вверху):

- **Тема:** Тёмная / Светлая
- **Язык:** RU / EN

Выбор сохраняется в `theme.txt` и `lang.txt`.

---

## 🔒 Безопасность

> ⚠️ **Файл `servers.toml` содержит пароли MySQL и RCON.**

Он **исключён** из Git через `.gitignore`. Никогда не коммить его!

Если случайно залил — **смени пароли** и удали файл из истории Git.

---

## 🛠️ Технологии

| Компонент | Технология |
|-----------|-----------|
| Язык | ![Rust](https://img.shields.io/badge/-Rust-000?logo=rust) |
| GUI | ![egui](https://img.shields.io/badge/-egui-8A2BE2) |
| База | ![MySQL](https://img.shields.io/badge/-MySQL-4479A1?logo=mysql&logoColor=white) |
| RCON | Собственная реализация |
| Формат | ![TOML](https://img.shields.io/badge/-TOML-9C4121) |

---

## 🗺️ Roadmap

- [x] Подключение к MySQL
- [x] Поиск игроков
- [x] Карточка игрока
- [x] Наказания через RCON
- [x] Темы (Dark / Light)
- [x] Мультиязычность (RU / EN)
- [x] Кнопки-макросы
- [x] Плавные анимации
- [ ] Кастомные шаблоны наказаний
- [ ] Статистика в шапке
- [ ] Логи действий
- [ ] Импорт / экспорт серверов
- [ ] Paper-плагин
- [ ] Веб-версия

---

## 🤝 Вклад

Pull request'ы приветствуются!

1. Fork репозитория.
2. Создай ветку (`git checkout -b feature/AmazingFeature`).
3. Коммит (`git commit -m 'Add some AmazingFeature'`).
4. Push (`git push origin feature/AmazingFeature`).
5. Открой Pull Request.

---

## 📄 Лицензия

Распространяется под лицензией **MIT**. Смотри [LICENSE](LICENSE).

---

<div align="center">

**Сделано с ❤️ для Minecraft-сообщества**

[⬆ Наверх](#-access-panel)

</div>