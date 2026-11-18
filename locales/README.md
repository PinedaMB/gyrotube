# GyroTube Translations (`locales/`)

GyroTube uses an embedded JSON catalog architecture for internationalization (`src/i18n.rs`).

- **Spanish (`es`)** and **English (`en`)** are built directly into `tr(es, en)` call sites.
- **Additional languages** can be added as flat JSON key-value files keyed by the canonical English string (`locales/en.json`), embedded at compile time via `include_str!`, with automatic fallback to English for any missing key.

## How to Add a New Language (3 Steps)

1. **Generate/update the template** and copy it to your target ISO 639-1 code (e.g., `fr` for French, `de` for German):
   ```bash
   python3 scripts/extract_locales.py
   cp locales/en.json locales/fr.json
   ```
   Translate the values in `locales/fr.json` (keep the keys unchanged).

2. **Register the language in `src/i18n.rs`**:
   - Add the variant to `AppLanguage` (and `AppLanguage::ALL` + `display_name()`).
   - Add the variant to `Lang` (and `detect_lang_from_locale`, `from_u8` / `to_u8`, `resolve_language`).
   - Load its embedded JSON in `catalog_for(Lang::Fr)` using a `OnceLock` + `parse_catalog(include_str!("../locales/fr.json"))`, and route `other => lookup_catalog(other, en)` in `tr()`.

3. **Verify coverage & tests**:
   ```bash
   python3 scripts/extract_locales.py
   cargo test i18n
   ```
   The Settings language selector (`Configuración → Idioma`) updates automatically from `AppLanguage::ALL`.
