# pxollyrs
Webhook for @pxolly on Rust <br> <br>
⚡ **BLAZING** FAST ⚡, 👀 **SIMPLE** 👀, 🚀 **MEMORY** SAFE 🚀 <br>
😱 IT'S PROJECT HASN'T **UNSAFE** CODES 😱

### Настройка вебхука 
```toml
[server]
# type: bool
# description: автоматическая привязка сервера к @pxolly,
# т.е true - будет пытаться привязать вебхук к серверу, в ином случае ничего не произойдет 
is_bind = true
# type: int16
# description: порт сервера на котором будет размещен вебхук
port = 1488
# type: string
# description: url вебхука, в большинстве случаев не нужон 
# (в некоторых случаях возможно будет полезен для всяких ngrok, и т.д)
host = ""

[pxolly]
# type: string
# description: секретный ключ вебхука 
# обратите внимание, если сервер @pxolly отправил запрос с неверным ключом, то вебхук вернет ответ с `locked`
secret_key = "set_key_here"
# type: string
# description: токен от @pxolly
token = "v8x.CymULq...."

[vk]
# type: string
# description: версия VK API
version = "5.131"
# type: string
# description: токен полученный от VK API
token = "vk1.a.NJ52iO-4S..."
```

Для работы вебхука *жизненно необходимы* токены от @pxolly, и ВКонтакте. <br>
Можно получить от вк - [тут](https://vkhost.github.io) <br>
А также от @pxolly в [личных сообщениях](https://vk.me/pxolly) командой `!токен новый` <br>

### Установка
Для компиляции и запуска вебхука, требуется компилятор [Rust](https://www.rust-lang.org/ru/tools/install) (требуется версия не ниже 1.63.0)

Запуск бота:
> git clone https://github.com/eoftgge/pxollyrs pxollyrs <br/>
> cd pxollyrs/ <br/>
> cargo run --release <br/>
