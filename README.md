# pxollyrs
Webhook for @pxolly on Rust

### Преимущества вебхука
- ⚡ **МОЛНИЕНОСНО** БЫСТР ⚡ <br>
- 👀 **ПРОСТ** В НАСТРОЙКЕ ВЕБХУКА 👀 <br>
- 🚀 **БЕЗОПАСЕН** С ТОЧКИ ЗРЕНИЯ ПАМЯТИ 🚀 <br>
- А ГЛАВНОЕ НЕ СОДЕРЖИТ **НЕБЕЗОПАСНОЙ** КОДОВОЙ БАЗЫ

### Настройка
Для настройки вебхука: вам нужен конфигурационный файл `config.toml` (пример конфигурации config.example.toml) <br>
Для работы вебхука *жизненно необходимы* токены от @pxolly, и ВКонтакте. <br>
Можно получить от ВКонтакте - [тут](https://vkhost.github.io) <br>
А также от @pxolly в [личных сообщениях](https://vk.me/pxolly) командой `!токен новый` <br>

### Запуск
Для компиляции и запуска вебхука, требуется компилятор [Rust](https://www.rust-lang.org/ru/tools/install) (требуется версия не ниже 1.64.0)

Запуск бота:
```commandline
git clone https://github.com/eoftgge/pxollyrs pxollyrs 
cd pxollyrs/ 
cargo run --release 
```

Также имеется альтернативный способ запуска бота: <br>
- Для начала необходим бинарник. Его можно взять из [Github Releases](https://github.com/eoftgge/pxollyrs/releases) 
- Настраиваем конфигурационный файл ([пример](https://github.com/eoftgge/pxollyrs/blob/main/config.example.toml))
- И запускаем совершенно обычным способом бинарник 