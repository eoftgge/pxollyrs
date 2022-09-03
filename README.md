# pxollyrs
Webhook for @pxolly on Rust

### Настройка 
```toml
[server]
is_bind = true
port = 1488
host = ""

[pxolly]
secret_key = "set_key_here"
token = "v8x.CymULq...."

[vk]
version = "5.131"
token = "vk1.a.NJ52iO-4S..."
```
> **SERVER:** <br>
> is_bind (bool) - автоматическая привязка сервера к API @pxolly <br>
> port (int16) - порт сервера <br>
> host (string) - url сервера, полезно для всяких ngrok <br>
> **PXOLLY:** <br>
> secret_key (string) - секретный ключ <br>
> token (string) - токен от бота [@pxolly](https://vk.me/pxolly) <br>
> **VK:** <br>
> version (string) - версия VKAPI <br>
> token (string) - токен от вк [жмяк](https://vkhost.github.io) <br>

### Установка
Для запуска бота, требуется компилятор Rust (https://www.rust-lang.org/ru/tools/install)

Запуск бота:
> git clone https://github.com/eoftgge/pxollyrs pxollyrs <br/>
> cd pxollyrs/ <br/>
> cargo run --release <br/>
