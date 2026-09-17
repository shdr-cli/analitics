<div align="center">

# 📊 Media Metrics Scraper

**Высокопроизводительный асинхронный CLI инстурмент на Rust для автоматического мониторинга просмотров видео**

*ПРОЕКТ ПЕРЕПИСАН с python на Rust*

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=brown)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white)](https://www.docker.com/)
[![Tokio](https://img.shields.io/badge/Tokio-async-ff69b4?style=for-the-badge&logo=rust)](https://tokio.rs/)

<p align="center">
  <a href="#info">Информация</a> •
  <a href="#docker">Docker</a> •
  <a href="#local-dev">Локальная разработка</a> •
  <a href="#commands">Команды</a> •
  <a href="#roadmap">Будущие планы</a>
</p>

</div>

---
Проект частисно создан с использованием ии
---
<a id="info"></a>
## Информация

- ⚡ **Высокая скорость работы:** асинхронные сетевые вызовы `tokio` + `reqwest`.
- 🌐 **Поддержка трех платформ:**
  - 🎬 **YouTube:** сбор аналитики обычных видео и YouTube Shorts через официальный YouTube Data API v3.
  - 📸 **Instagram:** извлечение просмотров Reels через приватные мобильные API Meta.
  - 🎵 **TikTok:** парсинг открытых метрик просмотров видеороликов.
- 🐳 **Docker-контейнер:** двухэтапная сборка (*multi-stage build*) на базе минимального образа `debian:bookworm-slim`
- 🔍 **Автоматический поиск путей:** поиск правильных путей (`../`, `./`) путей при `cargo run` так и при запуске сборки (`rust.exe`)

---

### 2. Настройка переменных окружения

`.env` вставьте ваши ключи и куки авторизации:

```env
YOUTUBE_API="your_youtube_data_api_v3_key"
INST_SESSION_ID="your_instagram_session_cookie_id"

```

### 3. Подготовка очередей ссылок

Заполните текстовые файлы нужными URL-адресами (каждая ссылка с новой строки):

* `youtube.txt`:
```text
https://www.youtube.com/watch?v=gbHuP5cQilk&list=RDgbHuP5cQilk&start_radio=1
https://www.youtube.com/watch?v=gznUXH-o6lQ
https://youtube.com/shorts/xRrg_xGeWpM?si=HV7J39hr0h4xcMKa

```


* `tiktok.txt`:
```text
https://www.tiktok.com/@noela.mahmutaj/video/7674010376291306770?is_from_webapp=1&sender_device=pc
https://www.tiktok.com/@binanceuseer/video/7675028619512581383?is_from_webapp=1&sender_device=pc

```


* `instagram.txt`:
```text
https://www.instagram.com/reel/Dc3cTytstQ0/?stkn=MWFwbDd0Y3dwYmppbw==
https://www.instagram.com/reel/Dc8sJ8wKI1c/?stkn=MW1qeWM2MHYxd2p0eQ==

```

---
<a id="docker"></a>
## 🐳 Запуск через Docker

> Перед запуском убедитесь, что приложение **Docker Desktop** активен.

### Первый запуск и компиляция:

```bash
docker compose up --build

```

Docker скачает зависимости, скомпилирует Rust-код в оптимизированном режиме `--release` и запустит контейнер.

### Повторные запуски:

Если вы меняете ссылки в файлах `.txt` или токены в `.env`, повторная сборка не нужна — контейнер сразу прочитает обновлённые данные с вашего диска:

```bash
docker compose up

```

После завершения работы результат отобразится в терминале и запишется в файл `РЕЗУЛЬТАТ.txt` в корне проекта.

---
<a id="local-dev"></a>
## 🛠 Локальная разработка (без Docker)

Для сборки и запуска напрямую в операционной системе потребуется установленный язык [Rust](https://rust-lang.org/).

1. Перейдите в каталог с исходным кодом:
```bash
cd rust

```


2. Запуск в режиме debug:
```bash
cargo run

```


3. Сборка релизной версии:
```bash
cargo build --release

```



Готовый исполняемый файл будет по пути:

* **Windows:** `rust/target/release/rust.exe`

---
<a id="commands"></a>
## ⚙️ Полезные команды

| Действие | Команда | Описание |
| --- | --- | --- |
| 🔄 **Чистая пересборка** | `docker compose build --no-cache` | Игнорирует кэш и собирает контейнер заново |
| 🧹 **Очистка кэша сборщика** | `docker builder prune` | Удаляет временные файлы сборки Docker, освобождая место |
| ⏹️ **Остановка контейнеров** | `docker compose down` | Останавливает и удаляет созданные контейнеры |
| 🗑️ **Очистка артефактов Cargo** | `cargo clean` | Удаляет локальную тяжелую папку `target/` |

<a id="roadmap"></a>
## ☕ Будущие планы проекта
|План|Описание|Прогресс|
| --- | --- | --- |
|**Избавиться от ии кода**|Часть кода написана с использование ии (сложные функции и тд)|`80%`|
|**Обработка ссылок в едином файле**|Ссылки YouTube, TikTok, Instagram вынесе в отдельные файлы, что неудобно в использовании|`100%`|