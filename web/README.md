## Пример использования wasm-bindgen и wasm-pack c модулями ES6

Сборка

```bash
wasm-pack build --target web
```

Чтобы раздать статику можно использовать пакет [https](https://crates.io/crates/https),
устанавливается через `cargo install https`, или любой другой сервер

```bash
# раздать статику через https
http

# через python
python -m SimpleHTTPServer
```
