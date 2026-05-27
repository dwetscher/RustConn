---
inclusion: fileMatch
fileMatchPattern: "**/*.rs"
---

# Pragmatic Rust Guidelines (Microsoft) — RustConn Adaptation

Адаптація [Microsoft Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) для RustConn.
Доповнює `project-rules.md`, а не замінює. Перелічено лише пункти, які бракують в інших steering файлах.

## Universal

### M-LINT-OVERRIDE-EXPECT — `#[expect]` замість `#[allow]`

При локальному override clippy/compiler lint-у — використовуй `#[expect(..., reason = "...")]`.
`#[expect]` емітить warning, якщо lint не спрацював, що запобігає накопиченню застарілих override-ів.

```rust
#[expect(clippy::unused_async, reason = "API stable, I/O буде додано пізніше")]
pub async fn ping_server() { }
```

`#[allow]` лишається доречним лише в макросах і згенерованому коді.

### M-PANIC-IS-STOP / M-PANIC-ON-BUG — паніка = «програма має зупинитися»

Паніка не є винятком. `panic!()` означає «зупини програму зараз». Не використовуй паніку для:
- комунікації помилок наверх (це робить `Result`),
- обробки контрольованих умов (timeout, недоступний хост, неправильний пароль),
- припущення, що паніку зловлять (якщо `panic = "abort"` — програма впаде).

Валідні випадки: `expect("must never happen")` для programming bug, `unwrap()` на `OnceLock::get_or_init`, panic на отруєний lock.

Програмний баг → `panic!` / `unreachable!` / `debug_assert!`. Recoverable стан → `Result<T, ThisError>`. Не змішуй.

### M-DOCUMENTED-MAGIC — документуй магічні значення

Будь-яка магічна константа або поведінка-за-замовчуванням повинна мати коментар.
Особливо актуально для timeout-ів, retry бекоффів, лімітів буферів.

```rust
// Vault операції чекають 10 секунд — Bitwarden CLI може тригерити master-pw prompt.
const VAULT_OP_TIMEOUT: Duration = Duration::from_secs(10);
```

### M-LOG-STRUCTURED — структуроване логування

Ми вже використовуємо `tracing`. Додатково:
- передавай дані як поля, не як форматовану string: `tracing::info!(host = %h, port = p, "connecting")` замість `tracing::info!("connecting to {}:{}", h, p)`,
- ніколи не лонж `SecretString` (`expose_secret()` в `tracing::*` — заборонено).

## Applications (rustconn / rustconn-cli)

### M-MIMALLOC-APP — глобальний алокатор

[Не критично, опційно]. Apps можуть отримати ~10–25% прискорення на hot paths шляхом заміни алокатора на `mimalloc`. Якщо профілювання покаже, що allocation — bottleneck, додай:

```toml
[dependencies]
mimalloc = "0.1"
```

```rust
// rustconn/src/main.rs
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```

### M-APP-ERROR — `anyhow` дозволено в `rustconn` / `rustconn-cli`

Бінарні крейти можуть використовувати `anyhow` / `eyre` для зменшення boilerplate.
Бібліотечні функції в `rustconn-core` ВСЕ ОДНО мусять використовувати `thiserror::Error`
(M-ERRORS-CANONICAL-STRUCTS — щоб виклик з GUI/CLI міг паттерн-матчити варіанти).

## Safety

### M-UNSAFE — `unsafe_code = "forbid"` уже застосовано

Workspace `[lints.rust] unsafe_code = "forbid"`. Якщо колись знадобиться FFI — створюй
окремий маленький крейт `rustconn-*-sys` з документацією safety contract і Miri тестами.
Не дозволяй unsafe «розповзатися» по основних крейтах.

## Documentation

### M-CANONICAL-DOCS — секції в doc-коментарях

Public функції в `rustconn-core` мусять мати:

```rust
/// Резюме одне речення, до 15 слів. (M-FIRST-DOC-SENTENCE)
///
/// Розширений опис.
///
/// # Errors
/// Повертає `MyError::X`, якщо ...
///
/// # Panics
/// Панікує, якщо ... (тільки для programming bug, див. M-PANIC-ON-BUG)
pub fn foo() -> Result<(), MyError> { ... }
```

Не створюй таблицю параметрів — описуй їх у вступному реченні: `Copies a file from src to dst`.

### M-PUBLIC-DEBUG для типів із секретами

Якщо тип містить `SecretString` або credentials — `Debug` має бути ручний і покритий тестом.
`secrecy::SecretString` уже редагує себе в `Debug`, але обгортки навколо нього треба перевіряти.

```rust
#[test]
fn debug_does_not_leak_secret() {
    let creds = Credentials::new("user", SecretString::new("hunter2".into()));
    let rendered = format!("{creds:?}");
    assert!(!rendered.contains("hunter2"));
}
```

## Naming — компроміс M-CONCISE-NAMES

MS guideline радить уникати `Manager` / `Service` / `Factory`. У нас історично є
`ConnectionManager`, `SessionManager`, `SecretManager` — ці імена лишаємо для сумісності.
Для **нового** коду — обирай специфічніші імена: `ConnectionStore`, `SessionRouter`,
`CredentialResolver`, `SnippetCatalog`.

## Universal lints — recommended additions

Розглянути додавання до `[workspace.lints.rust]` (опційно, не блокуюче):

```toml
missing_debug_implementations = "warn"
unsafe_op_in_unsafe_fn = "warn"  # неактуально, у нас forbid
unused_lifetimes = "warn"
redundant_lifetimes = "warn"
```

І до `[workspace.lints.clippy]` з restriction групи:

```toml
allow_attributes_without_reason = "warn"  # форсує reason = "..." в #[allow] / #[expect]
clone_on_ref_ptr = "warn"                 # ловить .clone() на Rc/Arc — пиши Rc::clone()
empty_drop = "warn"
undocumented_unsafe_blocks = "warn"        # неактуально, у нас forbid
```

Перевір, що це не зламає білд: `cargo clippy --all-targets`.

## References

- Чекліст: <https://microsoft.github.io/rust-guidelines/guidelines/checklist/>
- Universal: <https://microsoft.github.io/rust-guidelines/guidelines/universal/>
- Apps: <https://microsoft.github.io/rust-guidelines/guidelines/apps/>
- Safety: <https://microsoft.github.io/rust-guidelines/guidelines/safety/>
- Docs: <https://microsoft.github.io/rust-guidelines/guidelines/docs/>
- Rust API Guidelines (upstream): <https://rust-lang.github.io/api-guidelines/>
