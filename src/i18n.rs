use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLanguage {
    #[default]
    Auto,
    Es,
    En,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    Es,
    En,
}

static ACTIVE_LANG: AtomicU8 = AtomicU8::new(0);

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
    let val = match resolve_language(pref) {
        Lang::Es => 0,
        Lang::En => 1,
    };
    ACTIVE_LANG.store(val, Ordering::Relaxed);
}

pub fn current_lang() -> Lang {
    if ACTIVE_LANG.load(Ordering::Relaxed) == 1 {
        Lang::En
    } else {
        Lang::Es
    }
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
    }
}
