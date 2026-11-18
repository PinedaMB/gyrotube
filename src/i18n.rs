use std::collections::HashMap;
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLanguage {
    #[default]
    Auto,
    Es,
    En,
}

impl AppLanguage {
    pub const ALL: &'static [AppLanguage] = &[
        AppLanguage::Auto,
        AppLanguage::Es,
        AppLanguage::En,
    ];

    pub fn display_name(self) -> &'static str {
        match self {
            AppLanguage::Auto => tr("Automático (sistema)", "Automatic (system)"),
            AppLanguage::Es => "Español",
            AppLanguage::En => "English",
        }
    }

    pub fn to_index(self) -> u32 {
        Self::ALL
            .iter()
            .position(|&lang| lang == self)
            .unwrap_or(0) as u32
    }

    pub fn from_index(index: u32) -> Self {
        Self::ALL
            .get(index as usize)
            .copied()
            .unwrap_or(AppLanguage::Auto)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Es,
    En,
}

impl Lang {
    fn to_u8(self) -> u8 {
        match self {
            Lang::Es => 0,
            Lang::En => 1,
        }
    }

    fn from_u8(val: u8) -> Self {
        match val {
            0 => Lang::Es,
            _ => Lang::En,
        }
    }
}

static ACTIVE_LANG: AtomicU8 = AtomicU8::new(0);

/// Parses a flat JSON translation catalog (`{ "English key": "Translated value" }`)
/// into a zero-allocation lookup table. Used when adding extra embedded locales in `locales/`.
#[allow(dead_code)]
fn parse_catalog(raw: &str) -> HashMap<String, &'static str> {
    let parsed: HashMap<String, String> = serde_json::from_str(raw).unwrap_or_default();
    parsed
        .into_iter()
        .map(|(k, v)| (k, Box::leak(v.into_boxed_str()) as &'static str))
        .collect()
}

/// Returns the catalog for additional embedded languages (see `locales/README.md`).
#[allow(dead_code)]
fn catalog_for(_lang: Lang) -> Option<&'static HashMap<String, &'static str>> {
    None
}

#[allow(dead_code)]
fn lookup_catalog(lang: Lang, en_key: &'static str) -> &'static str {
    catalog_for(lang)
        .and_then(|map| map.get(en_key).copied())
        .unwrap_or(en_key)
}

pub fn detect_lang_from_locale(locale: &str) -> Lang {
    let lower = locale.trim().to_ascii_lowercase();
    if lower.starts_with("es") {
        Lang::Es
    } else {
        Lang::En
    }
}

pub fn detect_system_lang() -> Lang {
    for var in ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"] {
        if let Ok(val) = std::env::var(var) {
            let first = val.split(':').next().unwrap_or("").trim();
            if !first.is_empty() && first != "C" && first != "POSIX" {
                return detect_lang_from_locale(first);
            }
        }
    }
    for name in gtk4::glib::language_names() {
        let s = name.as_str();
        if !s.is_empty() && s != "C" && s != "POSIX" {
            return detect_lang_from_locale(s);
        }
    }
    Lang::En
}

pub fn resolve_language(pref: AppLanguage) -> Lang {
    match pref {
        AppLanguage::Auto => detect_system_lang(),
        AppLanguage::Es => Lang::Es,
        AppLanguage::En => Lang::En,
    }
}

pub fn set_language_preference(pref: AppLanguage) {
    let val = resolve_language(pref).to_u8();
    ACTIVE_LANG.store(val, Ordering::Relaxed);
}

pub fn current_lang() -> Lang {
    Lang::from_u8(ACTIVE_LANG.load(Ordering::Relaxed))
}

#[inline]
pub fn tr(es: &'static str, en: &'static str) -> &'static str {
    match current_lang() {
        Lang::Es => es,
        Lang::En => en,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_spanish_and_english_locales() {
        assert_eq!(detect_lang_from_locale("es_MX.UTF-8"), Lang::Es);
        assert_eq!(detect_lang_from_locale("es-ES"), Lang::Es);
        assert_eq!(detect_lang_from_locale("en_US.UTF-8"), Lang::En);
        assert_eq!(detect_lang_from_locale("fr_FR.UTF-8"), Lang::En);
    }

    #[test]
    fn resolves_explicit_language_preferences() {
        assert_eq!(resolve_language(AppLanguage::Es), Lang::Es);
        assert_eq!(resolve_language(AppLanguage::En), Lang::En);
        for &lang in AppLanguage::ALL {
            assert_eq!(AppLanguage::from_index(lang.to_index()), lang);
        }
    }

    #[test]
    fn parses_json_catalog_and_falls_back_to_english() {
        let map = parse_catalog(r#"{"Collection": "Sammlung"}"#);
        assert_eq!(map.get("Collection").copied(), Some("Sammlung"));
        assert_eq!(
            lookup_catalog(Lang::En, "Untranslated fallback key"),
            "Untranslated fallback key"
        );
    }
}
