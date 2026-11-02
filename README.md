# world-map (мод для Horizon, .hmod)

Миникарта для Horizon. Команда `/map` включает и выключает её в правом верхнем углу.
Карта рисуется через `render.set-minimap`: цвета как у карты-предмета, поворот вместе с игроком, стрелка игрока в центре.

## Что внутри

- `horizon-mod.toml`: манифест (id, версия, api 0.1, capability `render` как experimental).
- `src/lib.rs`: логика мода, одна команда `/map`.
- `.github/workflows/build-hmod.yml`: автосборка в GitHub Actions.

## Сборка локально

1. `rustup target add wasm32-unknown-unknown`
2. `cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod`
3. `cargo horizon-mod new world-map`: сгенерирует `Cargo.toml` с зависимостью от `horizon-mod-sdk`. Его оставь, а `src/lib.rs` и `horizon-mod.toml` замени на эти.
4. `cargo horizon-mod build`: на выходе `target/horizon-mod/world_map.hmod`.

## Установка

Положи `.hmod` в папку `horizon-mods/` рядом с `options.txt` игры, затем включи мод в Options > Mods.

## Ограничения

- Полноэкранной карты мира в API пока нет, только миникарта. Чтение блоков (capability `world`) ещё не готово.
- Код не собирался и не запускался. Пути SDK и поля `Minimap` не сверены с исходниками, при первой сборке возможны правки в одну-две строки.
