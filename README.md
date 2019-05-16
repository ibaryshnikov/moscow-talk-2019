# Дополнительные материалы к докладу

Moscow WebAssembly meetup №1, Rust и WebAssembly

## Ссылки

- https://www.rust-lang.org
- https://github.com/rustwasm
- https://www.arewewebyet.org/topics/webassembly
- https://crates.io/crates/wasm-tracing-allocator

## Примеры

- [nodejs](./nodejs)
- [es6 modules](./web)

## Настройка окружения для запуска примеров

Для начала нужно [установить компилятор](https://www.rust-lang.org/tools/install)

Добавить цель компиляции:

```bash
rustup target add wasm32-unknown-unknown
```

Установить `wasm-pack`:

```bash
cargo install wasm-pack
```
