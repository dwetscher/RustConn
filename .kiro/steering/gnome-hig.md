---
inclusion: fileMatch
fileMatchPattern: "rustconn/src/**/*.rs"
---

# GNOME HIG — RustConn Adaptation

Адаптація [GNOME Human Interface Guidelines](https://developer.gnome.org/hig/) для RustConn (GTK4 + libadwaita).
Доповнює `dialogs-guide.md` і `window-guide.md`. Перелічено лише пункти, які бракують в інших steering файлах.

## Writing Style — мова в UI

GNOME HIG: коротко, людяно, без жаргону. Українська локалізація — див. `po/uk.po` style guide
(`uk-translation-reviewer` agent). Загальні правила:

- **Sentence case** для всього: кнопки, заголовки, меню, toggles. Заголовок діалогу: «Properties of connection», НЕ «Properties Of Connection».
- **Звертайся до користувача напряму** через imperative («Save», «Connect»), не «Please save».
- **Не вживай знаки оклику** «!» в normal UI — звучить тривожно. Винятки: критичні помилки.
- **Уникай скорочень** на кшталт «info», «config» — пиши повні слова.
- **Назви кнопок — дієслова дії**: «Connect», «Save», «Delete» — не «OK», коли можна точніше.
- **Помилки** — пояснюй що сталося + що зробити. Не «Error 0x80070005», а «Connection refused. Check that the host is reachable.»

Усе ще обгортай в `i18n()` / `i18n_f()`.

## UI Styling — CSS classes від libadwaita

Кнопки несуть семантику через CSS class:

```rust
let connect_button = gtk4::Button::with_label(&i18n("Connect"));
connect_button.add_css_class("suggested-action");   // primary action — синя

let delete_button = gtk4::Button::with_label(&i18n("Delete"));
delete_button.add_css_class("destructive-action");  // червона
```

Інші семантичні класи (libadwaita 1.5+):
- `flat` — кнопка без рамки (icon-only в header bar),
- `pill` — округла кнопка (welcome screens),
- `circular` — кругла кнопка (close, add),
- `accent` — на банерах і стилях.

**Правило**: один `suggested-action` на діалог (primary дія). `destructive-action` — лише для незворотніх операцій (delete, revoke).

## Dialogs — використовуй `adw::AlertDialog`

Для confirm/alert (так/ні, OK) — `adw::AlertDialog`, НЕ `gtk::MessageDialog` (deprecated):

```rust
let dialog = adw::AlertDialog::new(
    Some(&i18n("Delete connection?")),
    Some(&i18n_f("This will permanently remove '{}'.", &[&conn.name])),
);
dialog.add_response("cancel", &i18n("Cancel"));
dialog.add_response("delete", &i18n("Delete"));
dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
dialog.set_default_response(Some("cancel"));
dialog.set_close_response("cancel");
```

- `set_response_appearance` → `Suggested` або `Destructive`.
- Default response — найбезпечніша дія (зазвичай Cancel).
- Close response (Escape) — теж Cancel.

Для більших форм — `adw::Dialog` із власним контентом (Properties, Connection editor).

## Header bars

- `adw::HeaderBar` — стандарт; не використовуй `gtk::HeaderBar` напряму в нових віджетах.
- Title widget → `adw::WindowTitle` із title + subtitle, або `adw::ViewSwitcher` для tabs.
- Primary action в headerbar — зліва (наприклад New connection); secondary/menu — справа.
- Burger menu (☰) — `gtk::MenuButton` з `adw::PopoverMenu`, відкривається F10.

## Toasts vs Banners vs Dialogs — коли що

| Patterns | Коли |
|----------|------|
| `adw::Toast` (через `adw::ToastOverlay`) | Транзиєнтні повідомлення про результат («Connected», «Saved»). Не блокує. |
| `adw::Banner` | Постійний стан, що вимагає уваги: «You are offline», «Update available». Інтегрований у вікно. |
| `adw::AlertDialog` | Підтвердження дії або модальне рішення. Блокує. |

Не показуй toast для критичних помилок — використовуй banner або alert dialog.

## Boxed lists — настройки і списки

Будь-який список settings → `adw::PreferencesGroup` з `adw::ActionRow` / `adw::EntryRow` /
`adw::SwitchRow` / `adw::ComboRow` / `adw::SpinRow`. Не комбінуй з raw `gtk::ListBox`.

```rust
let group = adw::PreferencesGroup::new();
group.set_title(&i18n("Connection details"));

let host_row = adw::EntryRow::new();
host_row.set_title(&i18n("Host"));
group.add(&host_row);
```

## Keyboard — обов'язкові shortcuts

Кожен GTK4 додаток мусить підтримувати:

| Shortcut | Дія |
|----------|-----|
| `Ctrl+W` | Закрити поточне вікно/tab |
| `Ctrl+Q` | Вийти з програми |
| `Ctrl+,` | Open Preferences (якщо є) |
| `F10` | Відкрити primary menu |
| `Ctrl+?` або `F1` | Show shortcuts window |
| `Escape` | Закрити dialog / popover / cancel mode |
| `Ctrl+F` | Search (де релевантно) |

Реєструй через `gtk::Application::set_accels_for_action()`.

Shortcuts window → `gtk::ShortcutsWindow` з `.ui` файла або `gtk::Builder`.

## Adaptive design — Wayland-first, mobile-friendly

- Мінімальний розмір вікна — підтримуй 360×294px (phone size). Перевіряй через `adw::WindowResizable`.
- Sidebar → `adw::OverlaySplitView` (auto-collapse), не `gtk::Paned`.
- Toolbar → `adw::ToolbarView` замість ручного `gtk::Box`.

## Pointer & Touch

- Мінімальний tap target: 44×44px (через `set_size_request` для icon-only buttons).
- Long-press для context menu — додавай через `gtk::GestureLongPress` поряд із right-click.
- Hover state — лише декорація; не покладайся на hover для важливої функціональності (touch screens не мають hover).

## Accessibility

- Кожен icon-only button → `set_tooltip_text(Some(&i18n("...")))` І
  `update_property(&[gtk4::accessible::Property::Label(&i18n("..."))])`. Уже задокументовано в `dialogs-guide.md`.
- Усі form widgets → `set_accessible_role(Role::TextBox)` (зазвичай встановлюється автоматично, перевіряй Inspector).
- Test з high-contrast і large-text — `gsettings set org.gnome.desktop.a11y.interface high-contrast true`.
- Min contrast ratio 4.5:1 для тексту, 3:1 для UI elements (WCAG AA).
- Не передавай інформацію лише кольором (статус conn — кольор + іконка).

## Icons

- Symbolic icons (`*-symbolic`) для inline UI (toolbar, lists). Кольорові — лише для app icon і decorative.
- Розмір: 16px для inline, 24px для toolbar, 32px для grid items.
- Перевіряй наявність в Adwaita icon theme: <https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/named-icons.html>

## Spacing — quick reference

| Контекст | Spacing |
|----------|---------|
| Margin вікна / `adw::Clamp` | 12px |
| Між пов'язаними елементами (label + entry) | 6px |
| Між групами | 18–24px |
| Header bar internal padding | автоматично |
| Boxed list rows | автоматично через AdwListBox |

Кламп ширини: 600px для preferences, 800px для content (повідомлення).

## Anti-patterns (не роби так)

- ❌ `gtk::MessageDialog` — deprecated, використовуй `adw::AlertDialog`.
- ❌ `gtk::Notebook` для main UI — використовуй `adw::TabView` + `adw::TabBar`.
- ❌ `gtk::Statusbar` — використовуй `adw::Toast` або `adw::Banner`.
- ❌ `gtk::Dialog` без `set_modal(true)` — на Wayland виглядає як окреме вікно.
- ❌ Hardcoded RGB кольори в коді — використовуй CSS classes (suggested-action, error, success).
- ❌ Власні розміри вікна через `set_default_size` без `adw::WindowResizable`.

## References

- HIG entry: <https://developer.gnome.org/hig/>
- Patterns: <https://developer.gnome.org/hig/patterns.html>
- Accessibility: <https://developer.gnome.org/hig/guidelines/accessibility.html>
- Writing style: <https://developer.gnome.org/hig/guidelines/writing-style.html>
- libadwaita docs: <https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/>
- libadwaita named icons: <https://gnome.pages.gitlab.gnome.org/libadwaita/doc/main/named-icons.html>
