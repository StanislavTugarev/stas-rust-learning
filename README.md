# stas-rust-learning

Учебный проект по Rust. Каждый файл — самостоятельный пример, который можно
запустить отдельной командой и посмотреть, как выполняется код.

## Как запустить любой пример

```bash
cargo run --example <имя>
```

Например:

```bash
cargo run --example s05_enums
cargo run --example s10_closures
cargo run --example p07_bank_demo
```

Показать список всех примеров:

```bash
cargo run --example
```

Имена: `sXX_*` — из папки `examples/section_XX/`, `pXX_*` — практические
задания (`examples/practitions_XX/`).

## Структура

```
src/
  lib.rs           — библиотека `rust_test` (реэкспорты для тестов/бенчей)
  section_07/      — код раздела 7 (нужен только тестам и бенчмаркам)
    sorting.rs     — алгоритмы сортировки
    bank.rs        — мини-банк Account/Bank (+ юнит-тесты)
    shapes.rs      — демо юнит-тестирования (Circle)
examples/
  section_04/  — владение и заимствования
  section_05/  — структуры, enum'ы, сопоставление с образцом
  section_09/  — трейты и обобщения
  section_10/  — замыкания и итераторы
  section_11/  — времена жизни (lifetimes)
  practitions_07/  — практика: демо банка
  practitions_09/  — практика: Notifier, валидация
  practitions_10/  — практика: заказы (dyn Order)
tests/
  s07_bank_stress.rs — интеграционный тест банка
benches/
  sorting.rs     — бенчмарк сортировки (Criterion)
  bank.rs        — бенчмарк банка (Criterion)
```

## Тесты и бенчмарки (раздел 7)

```bash
cargo test              # все юнит- и интеграционные тесты
cargo test --lib        # только юнит-тесты внутри src/
cargo bench             # бенчмарки Criterion
```
