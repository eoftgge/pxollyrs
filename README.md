# pxollyrs
Webhook for @pxolly on Rust

### Найстрока 
```toml
auto_connect = true 
port = 1555

secret_key = "lolkek123"
pxolly_token = "84f..."
access_token = "bae..."
```
> auto_connect: автоматическая привязка с @pxolly (true - привязывать, false - не привязывать) <br/>
> port: порт на котором сервер будет запущен (любое число, но лучше не трогать) <br/>
> secret_key: секретный ключ (любой текст) <br/>
> pxolly_token: токен от бота @pxolly (можно получить токен от [@pxolly](https://vk.me/pxolly) командой: !токен новый навсегда) <br/>
> access_token: токен от вк (можно получить токен от вк по ссылке [vkhost.github.io](https://vkhost.github.io)) <br/>

### Установка
Установка компилятора Rust (Для установки компилятора нужен curl) и запуск бота

> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh <br/>
> cd pxollyrs/ <br/>
> cargo run --release <br/>
